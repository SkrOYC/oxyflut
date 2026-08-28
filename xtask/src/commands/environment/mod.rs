//! Reference-environment inspection with live and fixture-backed platform sources.

#[cfg(test)]
mod fixtures;
mod linux;
mod macos;
mod wayland;
mod windows;
mod x11;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use oxyflut_qualification::environment::EnvironmentInventory;
use oxyflut_qualification::evidence::{EvidenceError, EvidenceRef, write_canonical_json_to_path};
use oxyflut_qualification::identifiers::{EnvironmentId, RepositoryPath};
use oxyflut_qualification::schema::SchemaError;
use thiserror::Error;

use super::super::{CommandError, CommandOutcome};

const LOCK_PATH: &str = ".constitution/tech-spec/contracts/qualification-lock.json";
const LOCK_SCHEMA: &str = "urn:oxyflut:schema:qualification-lock:5";

/// Supplies one candidate-neutral reference-environment inventory.
pub(crate) trait PlatformSource {
    /// Returns the Tier 1 environment represented by this source.
    fn environment(&self) -> EnvironmentId;

    /// Collects the inventory without inspecting candidate code or changing readiness.
    fn collect(&self) -> Result<EnvironmentInventory, EnvironmentCommandError>;
}

/// Inspects one live reference environment and writes its immutable evidence pair.
pub(crate) fn run(arguments: &[String]) -> CommandOutcome {
    let (environment, output) = match parse_arguments(arguments) {
        Ok(arguments) => arguments,
        Err(()) => {
            return CommandOutcome::failed(CommandError::InvalidInput {
                code: "environment-inspect-arguments",
            });
        }
    };
    let root = match workspace_root() {
        Ok(root) => root,
        Err(()) => {
            return CommandOutcome::failed(CommandError::Execution {
                code: "workspace-root",
                hint: "rerun: environment inspect --environment ENVIRONMENT --output PATH",
            });
        }
    };

    let source = live_source(environment);
    match inspect_with_source(&root, source.as_ref(), &output) {
        Ok(references) => {
            println!(
                "environment inspect: ok ({}; {})",
                references.projection.path.as_str(),
                references.inventory.path.as_str()
            );
            CommandOutcome::Success
        }
        Err(error) => CommandOutcome::failed(CommandError::ValidationFailed {
            code: error.code(),
            hint: "rerun: environment inspect --environment ENVIRONMENT --output PATH",
        }),
    }
}

fn parse_arguments(arguments: &[String]) -> Result<(EnvironmentId, RepositoryPath), ()> {
    let [environment_flag, environment, output_flag, output] = arguments else {
        return Err(());
    };
    if environment_flag != "--environment" || output_flag != "--output" {
        return Err(());
    }
    let environment = environment.parse::<EnvironmentId>().map_err(|_| ())?;
    let output = RepositoryPath::parse(output).map_err(|_| ())?;
    Ok((environment, output))
}

fn live_source(environment: EnvironmentId) -> Box<dyn PlatformSource> {
    match environment {
        EnvironmentId::Macos => Box::new(macos::MacosSource),
        EnvironmentId::Windows => Box::new(windows::WindowsSource),
        EnvironmentId::Wayland => Box::new(wayland::WaylandSource),
        EnvironmentId::X11 => Box::new(x11::X11Source),
    }
}

struct EnvironmentEvidence {
    projection: EvidenceRef,
    inventory: EvidenceRef,
}

fn inspect_with_source(
    root: &Path,
    source: &dyn PlatformSource,
    output: &RepositoryPath,
) -> Result<EnvironmentEvidence, EnvironmentCommandError> {
    let inventory = source.collect()?;
    if inventory.environment() != source.environment() {
        return Err(EnvironmentCommandError::SourceEnvironment);
    }
    validate_reference_environment(source.environment(), &inventory)?;
    let projection = inventory.lock_environment_value();
    validate_lock_environment_projection(root, source.environment(), &projection)?;
    let inventory_path = companion_inventory_path(output)?;
    validate_artifact_pair(root, output, &inventory_path)?;

    let projection_reference = write_canonical_json_to_path(root, output, &projection)
        .map_err(EnvironmentCommandError::Evidence)?;
    let complete_inventory =
        inventory.inventory_value(&projection_reference.path, &projection_reference.sha256);
    let inventory_reference =
        write_canonical_json_to_path(root, &inventory_path, &complete_inventory)
            .map_err(EnvironmentCommandError::Evidence)?;
    Ok(EnvironmentEvidence {
        projection: projection_reference,
        inventory: inventory_reference,
    })
}

fn companion_inventory_path(
    output: &RepositoryPath,
) -> Result<RepositoryPath, EnvironmentCommandError> {
    let stem = match output.as_str().strip_suffix(".json") {
        Some(stem) => stem,
        None => output.as_str(),
    };
    RepositoryPath::parse(&format!("{stem}.inventory.json"))
        .map_err(|_| EnvironmentCommandError::InventoryPath)
}

fn validate_reference_environment(
    environment: EnvironmentId,
    inventory: &EnvironmentInventory,
) -> Result<(), EnvironmentCommandError> {
    validate_architecture(environment, inventory)?;
    validate_operating_system(environment, inventory)
}

fn validate_architecture(
    environment: EnvironmentId,
    inventory: &EnvironmentInventory,
) -> Result<(), EnvironmentCommandError> {
    let expected = match environment {
        EnvironmentId::Macos => "aarch64",
        EnvironmentId::Windows | EnvironmentId::Wayland | EnvironmentId::X11 => "x86_64",
    };
    match inventory.fields().architecture.observed_value() {
        Some(actual) if actual == expected => Ok(()),
        Some(_) | None => Err(EnvironmentCommandError::EnvironmentMismatch),
    }
}

fn validate_operating_system(
    environment: EnvironmentId,
    inventory: &EnvironmentInventory,
) -> Result<(), EnvironmentCommandError> {
    let expected = match environment {
        EnvironmentId::Macos => "macos-26.5",
        EnvironmentId::Windows => "windows-11-25H2",
        EnvironmentId::Wayland | EnvironmentId::X11 => "ubuntu-26.04",
    };
    match inventory.operating_system().observed_value() {
        Some(actual) if actual == expected => Ok(()),
        Some(_) | None => Err(EnvironmentCommandError::EnvironmentMismatch),
    }
}

fn validate_artifact_pair(
    root: &Path,
    projection: &RepositoryPath,
    inventory: &RepositoryPath,
) -> Result<(), EnvironmentCommandError> {
    let projection_exists = artifact_exists(root, projection)?;
    let inventory_exists = artifact_exists(root, inventory)?;
    match (projection_exists, inventory_exists) {
        (true, false) => Err(EnvironmentCommandError::ArtifactPair {
            orphaned: projection.clone(),
        }),
        (false, true) => Err(EnvironmentCommandError::ArtifactPair {
            orphaned: inventory.clone(),
        }),
        (false, false) | (true, true) => Ok(()),
    }
}

fn artifact_exists(root: &Path, path: &RepositoryPath) -> Result<bool, EnvironmentCommandError> {
    let destination = root.join(path.as_str());
    match fs::symlink_metadata(&destination) {
        Ok(_) => Ok(true),
        Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(source) => Err(EnvironmentCommandError::Io {
            path: destination,
            source,
        }),
    }
}

fn validate_lock_environment_projection(
    root: &Path,
    environment: EnvironmentId,
    projection: &serde_json::Value,
) -> Result<(), EnvironmentCommandError> {
    let path = root.join(LOCK_PATH);
    let bytes = fs::read(&path).map_err(|source| EnvironmentCommandError::Io { path, source })?;
    let mut lock = serde_json::from_slice::<serde_json::Value>(&bytes)
        .map_err(EnvironmentCommandError::LockJson)?;
    let key = match environment {
        EnvironmentId::Macos => "macos-arm64",
        EnvironmentId::Windows => "windows-x86_64",
        EnvironmentId::Wayland => "wayland-linux-x86_64",
        EnvironmentId::X11 => "x11-linux-x86_64",
    };
    let environments = lock
        .get_mut("referenceEnvironments")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or(EnvironmentCommandError::LockShape)?;
    environments.insert(key.to_owned(), projection.clone());
    let registry = crate::contracts::schema::compile_workspace(root)?;
    registry.validate(LOCK_SCHEMA, &lock)?;
    Ok(())
}

fn workspace_root() -> Result<PathBuf, ()> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or(())
}

/// Classifies environment collection and publication failures without exposing collected content.
#[derive(Debug, Error)]
pub(crate) enum EnvironmentCommandError {
    /// A source was asked to collect a nonmatching environment.
    #[error("environment source does not match its requested environment")]
    SourceEnvironment,
    /// The host session, operating system, or architecture cannot satisfy the requested lock key.
    #[error("environment does not match the requested qualification lock key")]
    EnvironmentMismatch,
    /// The host does not run the requested operating system.
    #[error("environment source is unavailable on this host")]
    UnsupportedHost {
        /// The explicit inventory reason that prevents host collection.
        reason: oxyflut_qualification::environment::MissingReason,
    },
    /// A fixture response could not be read.
    #[cfg(test)]
    #[error("environment fixture could not be read")]
    FixtureIo {
        /// The local fixture path.
        path: PathBuf,
        /// The local I/O cause.
        #[source]
        source: io::Error,
    },
    /// A fixture response was not valid JSON.
    #[cfg(test)]
    #[error("environment fixture response JSON is invalid")]
    FixtureJson(#[source] serde_json::Error),
    /// A collected inventory was invalid.
    #[error("environment inventory is invalid")]
    Inventory(#[source] oxyflut_qualification::environment::EnvironmentError),
    /// The output path could not derive a distinct companion inventory path.
    #[error("environment companion inventory path is invalid")]
    InventoryPath,
    /// Exactly one immutable environment artifact already exists.
    #[error("environment artifact pair is partial; orphaned artifact: {orphaned}")]
    ArtifactPair {
        /// The projection or inventory artifact that has no companion.
        orphaned: RepositoryPath,
    },
    /// The active qualification lock could not be read.
    #[error("qualification lock could not be read")]
    Io {
        /// The local lock path.
        path: PathBuf,
        /// The local I/O cause.
        #[source]
        source: io::Error,
    },
    /// The active qualification lock was not valid JSON.
    #[error("qualification lock is invalid JSON")]
    LockJson(#[source] serde_json::Error),
    /// The active qualification lock lacked its required environment map.
    #[error("qualification lock environment map is invalid")]
    LockShape,
    /// The local schema registry could not compile.
    #[error("environment schema registry failed")]
    SchemaRegistry(#[from] crate::contracts::schema::ContractSchemaError),
    /// The lock-compatible environment projection failed schema validation.
    #[error("environment lock projection failed schema validation")]
    Schema(#[from] SchemaError),
    /// The immutable evidence writer could not publish the validated projection or inventory.
    #[error("environment evidence publication failed")]
    Evidence(#[source] EvidenceError),
}

impl EnvironmentCommandError {
    fn code(&self) -> &'static str {
        match self {
            Self::EnvironmentMismatch => "environment-mismatch",
            Self::SourceEnvironment => "environment-source-mismatch",
            Self::UnsupportedHost { .. } => "environment-unsupported-host",
            #[cfg(test)]
            Self::FixtureIo { .. } => "environment-fixture-io",
            #[cfg(test)]
            Self::FixtureJson(_) => "environment-fixture-json",
            Self::Inventory(_) => "environment-inventory",
            Self::InventoryPath => "environment-inventory-path",
            Self::ArtifactPair { .. } => "environment-artifact-pair",
            Self::Io { .. } => "environment-lock-io",
            Self::LockJson(_) => "environment-lock-json",
            Self::LockShape => "environment-lock-shape",
            Self::SchemaRegistry(_) => "environment-schema-registry",
            Self::Schema(_) => "environment-lock-schema",
            Self::Evidence(error) if error.code() == "content-address-collision" => {
                "evidence-destination-exists"
            }
            Self::Evidence(_) => "environment-evidence",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::error::Error;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use oxyflut_qualification::environment::{EnvironmentInventory, InventoryValue, MissingReason};
    use oxyflut_qualification::evidence::{MediaType, canonical_json_bytes, verify_file};
    use oxyflut_qualification::identifiers::{EnvironmentId, RepositoryPath};

    use super::{
        EnvironmentCommandError, PlatformSource, companion_inventory_path, inspect_with_source,
        parse_arguments, validate_reference_environment, workspace_root,
    };
    use crate::CommandOutcome;
    use crate::commands::environment::fixtures::FixturePlatformSource;

    #[test]
    fn raw_fixture_collectors_emit_one_candidate_neutral_inventory_shape()
    -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        for environment in EnvironmentId::tier_one() {
            let source = FixturePlatformSource::new(&root, environment);
            let inventory = source.collect()?;
            assert_eq!(inventory.environment(), environment);
            assert_eq!(EnvironmentInventory::field_names().len(), 13);
            assert!(inventory.fields().architecture.observed_value().is_some());
            assert!(
                inventory
                    .fields()
                    .compiler_identity
                    .observed_value()
                    .is_some()
            );
            assert!(inventory.fields().sdk_identity.observed_value().is_some());
            assert!(inventory.fields().rust_toolchain.observed_value().is_some());
            assert!(
                inventory.system_package_lock().packages().len()
                    <= oxyflut_qualification::environment::MAXIMUM_SYSTEM_PACKAGES
            );
            match environment {
                EnvironmentId::Macos | EnvironmentId::Wayland | EnvironmentId::X11 => {
                    assert!(inventory.fields().session.observed_value().is_some());
                }
                EnvironmentId::Windows => {
                    assert!(matches!(
                        inventory.fields().session,
                        InventoryValue::Missing {
                            reason: MissingReason::ManualCapture
                        }
                    ));
                }
            }
            match environment {
                EnvironmentId::Wayland => assert_eq!(
                    inventory.fields().protocol_version.observed_value(),
                    Some("wayland-wl_compositor-6-xdg_wm_base-6")
                ),
                EnvironmentId::X11 => assert_eq!(
                    inventory.fields().protocol_version.observed_value(),
                    Some("x11-11.0")
                ),
                EnvironmentId::Macos | EnvironmentId::Windows => assert!(matches!(
                    inventory.fields().protocol_version,
                    InventoryValue::Missing {
                        reason: MissingReason::ManualCapture
                    }
                )),
            }
            assert_eq!(
                inventory
                    .lock_environment_value()
                    .as_object()
                    .map(|value| value.len()),
                Some(6)
            );
        }
        Ok(())
    }

    #[test]
    fn raw_fixture_collectors_keep_missing_values_explicit_and_never_default_them()
    -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        let source = FixturePlatformSource::new(&root, EnvironmentId::X11);
        let inventory = source.collect()?;
        assert!(inventory.minimum_version().is_missing());
        assert!(inventory.driver_version().is_missing());
        assert_eq!(
            inventory
                .lock_environment_value()
                .pointer("/minimumVersion"),
            Some(&serde_json::Value::Null)
        );
        assert_eq!(
            inventory.lock_environment_value().pointer("/driverVersion"),
            Some(&serde_json::Value::Null)
        );
        Ok(())
    }

    #[test]
    fn raw_fixture_adapters_are_deterministic_and_schema_valid_for_every_lock_fragment()
    -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        for environment in EnvironmentId::tier_one() {
            let source = FixturePlatformSource::new(&root, environment);
            let first = source.collect()?;
            let second = source.collect()?;
            assert_eq!(first, second);
            super::validate_lock_environment_projection(
                &root,
                environment,
                &first.lock_environment_value(),
            )?;
        }
        Ok(())
    }

    #[test]
    fn inspect_writes_bound_projection_and_complete_inventory_through_the_evidence_writer()
    -> Result<(), Box<dyn Error>> {
        let workspace = temporary_directory("inspect")?;
        let root = workspace.path();
        let output = "qualification/evidence/output.json".parse::<RepositoryPath>()?;
        let output_path = root.join(output.as_str());
        let inventory_path = root.join(companion_inventory_path(&output)?.as_str());

        let source = FixturePlatformSource::new(root, EnvironmentId::Wayland);
        let references = inspect_with_source(root, &source, &output)?;
        let projection = verify_file(root, &output, &MediaType::application_json())?;
        let companion_path = companion_inventory_path(&output)?;
        let companion = verify_file(root, &companion_path, &MediaType::application_json())?;
        let projection_value: serde_json::Value = serde_json::from_slice(&fs::read(&output_path)?)?;
        let companion_value: serde_json::Value =
            serde_json::from_slice(&fs::read(&inventory_path)?)?;

        let projection_keys = projection_value
            .as_object()
            .ok_or("inventory projection must be an object")?
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        assert_eq!(
            projection_keys,
            BTreeSet::from([
                "driverVersion".to_owned(),
                "gpuId".to_owned(),
                "hardwareId".to_owned(),
                "minimumVersion".to_owned(),
                "operatingSystem".to_owned(),
                "systemPackageLockDigest".to_owned(),
            ])
        );
        assert_eq!(references.projection.sha256, projection.sha256());
        assert_eq!(references.inventory.sha256, companion.sha256());
        assert_eq!(
            fs::read(&output_path)?,
            canonical_json_bytes(&projection_value)?
        );
        assert_eq!(
            fs::read(&inventory_path)?,
            canonical_json_bytes(&companion_value)?
        );
        assert_eq!(
            companion_value
                .pointer("/lockProjection/path")
                .and_then(serde_json::Value::as_str),
            Some(references.projection.path.as_str())
        );
        let projection_digest = references.projection.sha256.to_string();
        assert_eq!(
            companion_value
                .pointer("/lockProjection/sha256")
                .and_then(serde_json::Value::as_str),
            Some(projection_digest.as_str())
        );
        assert_eq!(
            companion_value
                .pointer("/architecture/status")
                .and_then(serde_json::Value::as_str),
            Some("observed")
        );
        assert!(companion_value.pointer("/compilerIdentity").is_some());
        assert!(companion_value.pointer("/sdkIdentity").is_some());
        assert!(companion_value.pointer("/rustToolchain").is_some());
        assert!(companion_value.pointer("/compositor").is_some());
        assert!(companion_value.pointer("/session").is_some());
        assert!(companion_value.pointer("/protocolVersion").is_some());
        assert_eq!(
            companion_value
                .pointer("/systemPackageLock/digest/status")
                .and_then(serde_json::Value::as_str),
            Some("observed")
        );
        assert!(
            companion_value
                .pointer("/systemPackageLock/packages")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|packages| !packages.is_empty())
        );

        Ok(())
    }

    #[test]
    fn inactive_session_fails_closed_without_writing_evidence() -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        let output = "qualification/fixtures/environments/inactive-session-test.json"
            .parse::<RepositoryPath>()?;
        assert_no_evidence(&root, &output)?;
        let source =
            FixturePlatformSource::with_fixture(&root, EnvironmentId::X11, "x11-on-wayland");
        let result = inspect_with_source(&root, &source, &output);
        assert!(matches!(
            result,
            Err(EnvironmentCommandError::EnvironmentMismatch)
        ));
        assert_no_evidence(&root, &output)?;
        Ok(())
    }

    #[test]
    fn wrong_architecture_fails_closed_without_writing_evidence() -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        let output = "qualification/fixtures/environments/architecture-test.json"
            .parse::<RepositoryPath>()?;
        assert_no_evidence(&root, &output)?;
        let source =
            FixturePlatformSource::with_fixture(&root, EnvironmentId::Wayland, "wayland-arm64");
        let result = inspect_with_source(&root, &source, &output);
        assert!(matches!(
            result,
            Err(EnvironmentCommandError::EnvironmentMismatch)
        ));
        assert_no_evidence(&root, &output)?;
        Ok(())
    }

    #[test]
    fn reference_operating_systems_accept_pinned_releases_and_reject_nixos_without_writing()
    -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        let ubuntu = FixturePlatformSource::new(&root, EnvironmentId::Wayland).collect()?;
        validate_reference_environment(EnvironmentId::Wayland, &ubuntu)?;

        let output =
            "qualification/fixtures/environments/nixos-test.json".parse::<RepositoryPath>()?;
        assert_no_evidence(&root, &output)?;
        let source =
            FixturePlatformSource::with_fixture(&root, EnvironmentId::Wayland, "wayland-nixos");
        let result = inspect_with_source(&root, &source, &output);
        assert!(matches!(
            result,
            Err(EnvironmentCommandError::EnvironmentMismatch)
        ));
        assert_no_evidence(&root, &output)?;
        Ok(())
    }

    #[test]
    fn partial_environment_artifact_pair_names_the_orphaned_artifact() -> Result<(), Box<dyn Error>>
    {
        let workspace = temporary_directory("partial-pair")?;
        let root = workspace.path();
        let output = "qualification/evidence/environment.json".parse::<RepositoryPath>()?;
        let output_path = root.join(output.as_str());
        let parent = output_path
            .parent()
            .ok_or("environment projection must have a parent")?;
        fs::create_dir_all(parent)?;
        fs::write(&output_path, b"{}")?;
        let source = FixturePlatformSource::new(root, EnvironmentId::Wayland);

        let result = inspect_with_source(root, &source, &output);
        assert!(matches!(
            &result,
            Err(EnvironmentCommandError::ArtifactPair { orphaned }) if orphaned == &output
        ));
        assert_eq!(
            result.err().map(|error| error.to_string()).as_deref(),
            Some(
                "environment artifact pair is partial; orphaned artifact: qualification/evidence/environment.json"
            )
        );
        assert!(
            !root
                .join(companion_inventory_path(&output)?.as_str())
                .exists()
        );
        Ok(())
    }

    #[test]
    fn linux_collector_requires_the_complete_ubuntu_binary_package_set()
    -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        let source = FixturePlatformSource::new(&root, EnvironmentId::Wayland);
        let inventory = source.collect()?;
        let names = inventory
            .system_package_lock()
            .packages()
            .iter()
            .map(|package| package.name())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            names,
            BTreeSet::from([
                "binutils",
                "clang",
                "libc6",
                "libc6-dev",
                "libglib2.0-0t64",
                "libgtk-4-1",
                "libwayland-client0",
                "libwayland-server0",
                "libx11-6",
                "libxcb1",
                "lld",
                "rustc",
                "xserver-xorg-core",
            ])
        );
        assert!(
            inventory
                .system_package_lock()
                .digest()
                .observed_value()
                .is_some()
        );
        Ok(())
    }

    #[test]
    fn linux_driver_identity_binds_the_kernel_driver_and_userspace_package()
    -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        let source = FixturePlatformSource::new(&root, EnvironmentId::Wayland);
        let inventory = source.collect()?;
        assert_eq!(
            inventory.fields().gpu_id.observed_value(),
            Some("pci:1002:73bf")
        );
        assert_eq!(
            inventory.driver_version().observed_value(),
            Some("amdgpu/libgl1-mesa-dri=25.0.0-1ubuntu1")
        );
        Ok(())
    }

    #[test]
    fn missing_required_linux_package_is_typed_and_never_hashes_a_partial_set()
    -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        let source = FixturePlatformSource::with_fixture(
            &root,
            EnvironmentId::Wayland,
            "wayland-missing-package",
        );
        let inventory = source.collect()?;
        assert!(inventory.system_package_lock().digest().is_missing());
        let missing = inventory
            .system_package_lock()
            .packages()
            .iter()
            .find(|package| package.name() == "libwayland-server0")
            .ok_or("required package must remain in the inventory")?;
        assert!(matches!(
            missing.version(),
            InventoryValue::Missing {
                reason: MissingReason::NotInstalled
            }
        ));
        Ok(())
    }

    #[test]
    fn nvidia_driver_packages_are_bound_to_one_observed_kernel_driver() -> Result<(), Box<dyn Error>>
    {
        let root = test_workspace_root()?;
        let source = FixturePlatformSource::with_fixture(
            &root,
            EnvironmentId::Wayland,
            "wayland-nvidia-ambiguous",
        );
        let inventory = source.collect()?;

        assert_eq!(
            inventory.fields().gpu_id.observed_value(),
            Some("pci:10de:2684")
        );
        assert!(matches!(
            inventory.driver_version(),
            InventoryValue::Missing {
                reason: MissingReason::AmbiguousSource
            }
        ));
        Ok(())
    }

    #[test]
    fn multiple_qualifying_linux_gpu_cards_leave_both_gpu_fields_explicitly_missing()
    -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        let source = FixturePlatformSource::with_fixture(
            &root,
            EnvironmentId::Wayland,
            "wayland-multiple-gpus",
        );
        let inventory = source.collect()?;

        assert!(matches!(
            inventory.fields().gpu_id,
            InventoryValue::Missing {
                reason: MissingReason::UnsupportedBySource
            }
        ));
        assert!(matches!(
            inventory.fields().driver_version,
            InventoryValue::Missing {
                reason: MissingReason::UnsupportedBySource
            }
        ));
        Ok(())
    }

    #[test]
    fn platform_package_locks_hash_only_the_required_pinned_entries() -> Result<(), Box<dyn Error>>
    {
        let root = test_workspace_root()?;
        for (environment, expected) in [
            (
                EnvironmentId::Macos,
                BTreeSet::from([
                    "com.apple.pkg.CLTools_Executables",
                    "com.apple.pkg.CLTools_SDK_macOS",
                    "com.apple.pkg.Xcode",
                ]),
            ),
            (
                EnvironmentId::Windows,
                BTreeSet::from(["Microsoft.VisualStudio.BuildTools", "Microsoft.WindowsSDK"]),
            ),
        ] {
            let inventory = FixturePlatformSource::new(&root, environment).collect()?;
            let names = inventory
                .system_package_lock()
                .packages()
                .iter()
                .map(|package| package.name())
                .collect::<BTreeSet<_>>();
            assert_eq!(names, expected);
            assert!(
                inventory
                    .system_package_lock()
                    .digest()
                    .observed_value()
                    .is_some()
            );
        }
        Ok(())
    }

    #[test]
    fn missing_required_macos_and_windows_packages_keep_their_locks_null()
    -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        for (environment, fixture, missing_name) in [
            (
                EnvironmentId::Macos,
                "macos-missing-receipt",
                "com.apple.pkg.CLTools_SDK_macOS",
            ),
            (
                EnvironmentId::Windows,
                "windows-missing-package",
                "Microsoft.WindowsSDK",
            ),
        ] {
            let inventory =
                FixturePlatformSource::with_fixture(&root, environment, fixture).collect()?;
            assert!(inventory.system_package_lock().digest().is_missing());
            let missing = inventory
                .system_package_lock()
                .packages()
                .iter()
                .find(|package| package.name() == missing_name)
                .ok_or("required package must remain in the inventory")?;
            assert!(matches!(
                missing.version(),
                InventoryValue::Missing {
                    reason: MissingReason::NotInstalled
                }
            ));
        }
        Ok(())
    }

    #[test]
    fn windows_pci_pnp_identifier_strips_the_bus_prefix_before_parsing()
    -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        let source = FixturePlatformSource::new(&root, EnvironmentId::Windows);
        let inventory = source.collect()?;
        assert_eq!(
            inventory.fields().gpu_id.observed_value(),
            Some("pci:10de:2684")
        );
        Ok(())
    }

    #[test]
    fn windows_driver_requires_the_matching_display_device_raw_fixture()
    -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        let source = FixturePlatformSource::with_fixture(
            &root,
            EnvironmentId::Windows,
            "windows-mismatched-pnp-driver",
        );
        let inventory = source.collect()?;
        assert!(matches!(
            inventory.driver_version(),
            InventoryValue::Missing {
                reason: MissingReason::UnsupportedBySource
            }
        ));
        Ok(())
    }

    #[test]
    fn windows_compiler_raw_fixtures_accept_the_banner_and_fall_back_to_pinned_sources()
    -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        for (fixture, expected) in [
            ("windows", "msvc-19.44.35207"),
            ("windows-compiler-env", "msvc-14.44.35207"),
            ("windows-compiler-vswhere", "msvc-17.14.39"),
        ] {
            let inventory =
                FixturePlatformSource::with_fixture(&root, EnvironmentId::Windows, fixture)
                    .collect()?;
            assert_eq!(
                inventory.fields().compiler_identity.observed_value(),
                Some(expected)
            );
        }
        Ok(())
    }

    #[test]
    fn observed_inventory_values_reject_unbounded_content() {
        assert!(
            InventoryValue::observed("user content is not an inventory value".to_owned()).is_err()
        );
    }

    #[test]
    fn command_arguments_require_one_closed_environment_and_evidence_output_path() {
        assert!(
            parse_arguments(&[
                "--environment".to_owned(),
                "wayland".to_owned(),
                "--output".to_owned(),
                "qualification/evidence/environment.json".to_owned(),
            ])
            .is_ok()
        );
        assert!(parse_arguments(&[]).is_err());
        assert!(
            parse_arguments(&[
                "--environment".to_owned(),
                "unknown".to_owned(),
                "--output".to_owned(),
                "qualification/evidence/environment.json".to_owned(),
            ])
            .is_err()
        );
    }

    #[test]
    fn sources_that_report_the_wrong_environment_fail_before_writing() -> Result<(), Box<dyn Error>>
    {
        let root = test_workspace_root()?;
        let output = "qualification/fixtures/environments/wrong-source-test.json"
            .parse::<RepositoryPath>()?;
        let result = inspect_with_source(&root, &WrongEnvironmentSource, &output);
        assert!(matches!(
            result,
            Err(EnvironmentCommandError::SourceEnvironment)
        ));
        Ok(())
    }

    fn test_workspace_root() -> Result<PathBuf, Box<dyn Error>> {
        workspace_root().map_err(|_| "xtask must remain directly below the workspace root".into())
    }

    fn assert_no_evidence(
        root: &std::path::Path,
        output: &RepositoryPath,
    ) -> Result<(), Box<dyn Error>> {
        let projection = root.join(output.as_str());
        let inventory = root.join(companion_inventory_path(output)?.as_str());
        assert!(!projection.exists());
        assert!(!inventory.exists());
        Ok(())
    }

    struct WrongEnvironmentSource;

    impl PlatformSource for WrongEnvironmentSource {
        fn environment(&self) -> EnvironmentId {
            EnvironmentId::Wayland
        }

        fn collect(&self) -> Result<EnvironmentInventory, EnvironmentCommandError> {
            let root = workspace_root().map_err(|_| EnvironmentCommandError::UnsupportedHost {
                reason: MissingReason::UnavailableOnHost,
            })?;
            FixturePlatformSource::new(&root, EnvironmentId::X11).collect()
        }
    }

    #[test]
    fn live_sources_are_host_gated() {
        #[cfg(not(target_os = "macos"))]
        assert!(matches!(
            super::macos::MacosSource.collect(),
            Err(EnvironmentCommandError::UnsupportedHost {
                reason: MissingReason::UnavailableOnHost
            })
        ));
        #[cfg(not(target_os = "windows"))]
        assert!(matches!(
            super::windows::WindowsSource.collect(),
            Err(EnvironmentCommandError::UnsupportedHost {
                reason: MissingReason::UnavailableOnHost
            })
        ));
        #[cfg(not(target_os = "linux"))]
        assert!(matches!(
            super::linux::collect_linux(EnvironmentId::Wayland),
            Err(EnvironmentCommandError::UnsupportedHost {
                reason: MissingReason::UnavailableOnHost
            })
        ));
    }

    #[test]
    fn existing_output_without_its_inventory_reports_a_partial_pair() -> Result<(), Box<dyn Error>>
    {
        let workspace = temporary_directory("collision")?;
        let root = workspace.path();
        let output = "qualification/evidence/environment.json".parse::<RepositoryPath>()?;
        fs::create_dir_all(root.join("qualification/evidence"))?;
        fs::write(root.join(output.as_str()), b"{}")?;
        let source = FixturePlatformSource::new(root, EnvironmentId::Wayland);

        let error = inspect_with_source(root, &source, &output)
            .err()
            .ok_or("existing output must fail")?;
        assert_eq!(error.code(), "environment-artifact-pair");
        Ok(())
    }

    #[test]
    fn environment_command_stays_registered_and_returns_an_outcome() {
        let outcome = super::run(&[]);
        assert!(matches!(outcome, CommandOutcome::Failed(_)));
    }

    static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    struct TemporaryDirectory {
        path: PathBuf,
    }

    impl TemporaryDirectory {
        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TemporaryDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn temporary_directory(name: &str) -> Result<TemporaryDirectory, Box<dyn Error>> {
        let source = test_workspace_root()?;
        for _ in 0..128 {
            let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "oxyflut-environment-{name}-{}-{sequence}",
                std::process::id()
            ));
            match fs::create_dir(&path) {
                Ok(()) => {
                    copy_directory(&source.join(".constitution"), &path.join(".constitution"))?;
                    copy_directory(&source.join("qualification"), &path.join("qualification"))?;
                    return Ok(TemporaryDirectory { path });
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error.into()),
            }
        }
        Err("could not create a temporary environment workspace".into())
    }

    fn copy_directory(source: &Path, destination: &Path) -> Result<(), std::io::Error> {
        fs::create_dir_all(destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let destination_path = destination.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                copy_directory(&entry.path(), &destination_path)?;
            } else {
                fs::copy(entry.path(), destination_path)?;
            }
        }
        Ok(())
    }
}
