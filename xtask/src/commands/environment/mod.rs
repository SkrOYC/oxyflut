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
use oxyflut_qualification::evidence::{EvidenceError, write_canonical_json_to_path};
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

/// Inspects one live reference environment and writes its lock-compatible evidence projection.
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
        Ok(reference) => {
            println!("environment inspect: ok ({})", reference.path.as_str());
            CommandOutcome::Success
        }
        Err(_) => CommandOutcome::failed(CommandError::ValidationFailed {
            code: "environment-inspect-invalid",
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

fn inspect_with_source(
    root: &Path,
    source: &dyn PlatformSource,
    output: &RepositoryPath,
) -> Result<oxyflut_qualification::evidence::EvidenceRef, EnvironmentCommandError> {
    let inventory = source.collect()?;
    if inventory.environment() != source.environment() {
        return Err(EnvironmentCommandError::SourceEnvironment);
    }
    let projection = inventory.lock_environment_value();
    validate_lock_environment_projection(root, inventory.environment(), &projection)?;
    write_canonical_json_to_path(root, output, &projection)
        .map_err(EnvironmentCommandError::Evidence)
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
    /// The host does not run the requested operating system.
    #[error("environment source is unavailable on this host")]
    UnsupportedHost,
    /// A fixture inventory could not be read.
    #[cfg(test)]
    #[error("environment fixture could not be read")]
    FixtureIo {
        /// The local fixture path.
        path: PathBuf,
        /// The local I/O cause.
        #[source]
        source: io::Error,
    },
    /// A collected inventory was invalid.
    #[error("environment inventory is invalid")]
    Inventory(#[source] oxyflut_qualification::environment::EnvironmentError),
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
    /// The immutable evidence writer could not publish the validated projection.
    #[error("environment evidence publication failed")]
    Evidence(#[source] EvidenceError),
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::error::Error;
    use std::fs;
    use std::path::PathBuf;

    use oxyflut_qualification::environment::{EnvironmentInventory, InventoryValue};
    use oxyflut_qualification::evidence::{MediaType, canonical_json_bytes, verify_file};
    use oxyflut_qualification::identifiers::{EnvironmentId, RepositoryPath};

    use super::{
        EnvironmentCommandError, PlatformSource, inspect_with_source, parse_arguments,
        workspace_root,
    };
    use crate::CommandOutcome;
    use crate::commands::environment::fixtures::FixturePlatformSource;

    #[test]
    fn fixture_collectors_emit_one_candidate_neutral_inventory_shape() -> Result<(), Box<dyn Error>>
    {
        let root = test_workspace_root()?;
        for environment in EnvironmentId::tier_one() {
            let source = FixturePlatformSource::new(&root, environment);
            let inventory = source.collect()?;
            assert_eq!(inventory.environment(), environment);
            assert_eq!(EnvironmentInventory::field_names().len(), 12);
            assert!(inventory.fields().architecture.observed_value().is_some());
            assert!(
                inventory
                    .fields()
                    .compiler_identity
                    .observed_value()
                    .is_some()
            );
            assert!(inventory.fields().sdk_identity.observed_value().is_some());
            assert!(inventory.fields().session.observed_value().is_some());
            assert!(
                inventory.system_package_lock().packages().len()
                    <= oxyflut_qualification::environment::MAXIMUM_SYSTEM_PACKAGES
            );
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
    fn fixture_collectors_keep_missing_values_explicit_and_never_default_them()
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
    fn fixture_adapters_are_deterministic_and_schema_valid_for_every_lock_fragment()
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
    fn inspect_writes_only_the_schema_permitted_projection_through_the_evidence_writer()
    -> Result<(), Box<dyn Error>> {
        let root = test_workspace_root()?;
        let output_text = format!(
            "qualification/fixtures/environments/output-test-{}.json",
            std::process::id()
        );
        let output = output_text.parse::<RepositoryPath>()?;
        let output_path = root.join(&output_text);
        if output_path.exists() {
            fs::remove_file(&output_path)?;
        }
        let source = FixturePlatformSource::new(&root, EnvironmentId::Wayland);
        let reference = inspect_with_source(&root, &source, &output)?;
        let verified = verify_file(&root, &output, &MediaType::application_json())?;
        let value: serde_json::Value = serde_json::from_slice(&fs::read(&output_path)?)?;
        let keys = value
            .as_object()
            .ok_or("inventory projection must be an object")?
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        assert_eq!(
            keys,
            BTreeSet::from([
                "driverVersion".to_owned(),
                "gpuId".to_owned(),
                "hardwareId".to_owned(),
                "minimumVersion".to_owned(),
                "operatingSystem".to_owned(),
                "systemPackageLockDigest".to_owned(),
            ])
        );
        assert_eq!(reference.sha256, verified.sha256());
        assert_eq!(fs::read(&output_path)?, canonical_json_bytes(&value)?);
        fs::remove_file(output_path)?;
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

    struct WrongEnvironmentSource;

    impl PlatformSource for WrongEnvironmentSource {
        fn environment(&self) -> EnvironmentId {
            EnvironmentId::Wayland
        }

        fn collect(&self) -> Result<EnvironmentInventory, EnvironmentCommandError> {
            let root = workspace_root().map_err(|_| EnvironmentCommandError::UnsupportedHost)?;
            FixturePlatformSource::new(&root, EnvironmentId::X11).collect()
        }
    }

    #[test]
    fn live_sources_are_host_gated() {
        #[cfg(not(target_os = "macos"))]
        assert!(matches!(
            super::macos::MacosSource.collect(),
            Err(EnvironmentCommandError::UnsupportedHost)
        ));
        #[cfg(not(target_os = "windows"))]
        assert!(matches!(
            super::windows::WindowsSource.collect(),
            Err(EnvironmentCommandError::UnsupportedHost)
        ));
    }

    #[test]
    fn environment_command_stays_registered_and_returns_an_outcome() {
        let outcome = super::run(&[]);
        assert!(matches!(outcome, CommandOutcome::Failed(_)));
    }
}
