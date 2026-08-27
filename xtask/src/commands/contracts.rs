//! The fail-closed contract-validation command.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use super::super::{CommandError, CommandOutcome};
use crate::contracts as validators;
use crate::toolchain;

const DATA_MODELS_PATH: &str = ".constitution/tech-spec/data-models";
const CONTRACTS_PATH: &str = ".constitution/tech-spec/contracts";
const TRACEABILITY_PATH: &str = ".constitution/tech-spec/contracts/capability-traceability.json";
const ACCESSIBILITY_GENERATION_PATH: &str =
    ".constitution/tech-spec/data-models/accessibility-map.schema.json";
const REGISTRY_PATH: &str = ".constitution/tech-spec/contracts/diagnostic-event-registry.json";
const LOCK_PATH: &str = ".constitution/tech-spec/contracts/qualification-lock.json";
const PHASE_PATH: &str = ".constitution/tech-spec/contracts/specification-phase.json";
const RUST_CONTRACTS_PATH: &str = ".constitution/tech-spec/contracts";
const HEADER_PATH: &str = ".constitution/tech-spec/contracts/oxyflut-substrate.h";
const BINDINGS_PATH: &str = "qualification/fixtures/generated-bindings/oxyflut-substrate.rs";
const TOOLCHAIN_MANIFEST_PATH: &str = "qualification/tools/native-contract-toolchain.json";
const UNSUPPORTED_HOST: &str = "unsupported host";
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Runs every contract-validation family in deterministic order.
pub(crate) fn run(arguments: &[String]) -> CommandOutcome {
    if !arguments.is_empty() {
        return CommandOutcome::failed(CommandError::InvalidInput {
            code: "contracts-validate-arguments",
        });
    }

    let root = match workspace_root() {
        Ok(root) => root,
        Err(()) => {
            return CommandOutcome::failed(CommandError::Execution {
                code: "workspace-root",
                hint: "rerun: contracts validate",
            });
        }
    };
    let report = validate_workspace(&root);
    report.emit();
    report.outcome()
}

fn validate_workspace(root: &Path) -> ContractValidationReport {
    let mut summaries = Vec::with_capacity(FAMILY_COUNT);
    summaries.extend(pre_implementation_summaries(root));
    summaries.extend(readiness_summaries(
        validators::readiness::validate_workspace(root),
    ));
    summaries.push(rust_contract_summary(validate_rust_contracts(root)));

    match validators::native::NativeTools::load(root) {
        Ok(tools) => summaries.extend(native_summaries(root, &tools)),
        Err(error) => summaries.extend(native_load_failure(native_failure_path(&error))),
    }

    ContractValidationReport { summaries }
}

/// Returns the first failed pre-implementation validation family in command order.
pub(crate) fn first_pre_implementation_input_failure(
    root: &Path,
) -> Option<ValidationFamilyFailure> {
    pre_implementation_summaries(root)
        .into_iter()
        .find(FamilySummary::is_failed)
        .map(|summary| ValidationFamilyFailure {
            family: summary.family,
            contract_path: summary.contract_path,
        })
}

/// Identifies one failed pre-implementation validation family.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ValidationFamilyFailure {
    /// The stable validation family name.
    pub(crate) family: &'static str,
    /// The contract path reported by the validation family.
    pub(crate) contract_path: String,
}

fn pre_implementation_summaries(root: &Path) -> Vec<FamilySummary> {
    let mut summaries = Vec::with_capacity(7);
    let registry = validators::schema::compile_workspace(root);
    summaries.push(match &registry {
        Ok(_) => FamilySummary::ok("schema", DATA_MODELS_PATH),
        Err(_) => FamilySummary::failed("schema", DATA_MODELS_PATH),
    });
    summaries.push(match registry {
        Ok(registry) => match validators::schema::validate_compiled_workspace(root, &registry) {
            Ok(_) => FamilySummary::ok("instance", CONTRACTS_PATH),
            Err(error) => FamilySummary::failed_path(
                "instance",
                schema_failure_path(root, &error, CONTRACTS_PATH),
            ),
        },
        Err(_) => FamilySummary::failed("instance", DATA_MODELS_PATH),
    });
    summaries.extend(traceability_summaries(
        validators::traceability::validate_workspace(root),
    ));
    summaries.push(summary_from_result(
        "registry",
        REGISTRY_PATH,
        validators::registries::validate_workspace(root),
    ));
    summaries.push(summary_from_result(
        "digest",
        CONTRACTS_PATH,
        validators::digests::validate_workspace(root),
    ));
    summaries
}

fn traceability_summaries(
    result: Result<
        validators::traceability::TraceabilityRunReport,
        validators::traceability::TraceabilityError,
    >,
) -> [FamilySummary; 3] {
    match result {
        Ok(report) => [
            FamilySummary::ok("exact-set", TRACEABILITY_PATH),
            contract_test_summary(&report),
            accessibility_generation_summary(&report),
        ],
        Err(_) => [
            FamilySummary::failed("exact-set", TRACEABILITY_PATH),
            FamilySummary::failed("contract-tests", TRACEABILITY_PATH),
            FamilySummary::failed("accessibility-generation", ACCESSIBILITY_GENERATION_PATH),
        ],
    }
}

fn contract_test_summary(
    report: &validators::traceability::TraceabilityRunReport,
) -> FamilySummary {
    match report.contract_test_resolution {
        validators::traceability::ContractTestResolution::DeferredUntilCandidateImplementation => {
            FamilySummary::deferred(
                "contract-tests",
                format!(
                    "{} pending candidate implementation",
                    report.deferred_contract_tests
                ),
                TRACEABILITY_PATH,
            )
        }
        validators::traceability::ContractTestResolution::Resolved => {
            FamilySummary::ok("contract-tests", TRACEABILITY_PATH)
        }
    }
}

fn accessibility_generation_summary(
    report: &validators::traceability::TraceabilityRunReport,
) -> FamilySummary {
    if report.accessibility_generation_deferred {
        FamilySummary::deferred(
            "accessibility-generation",
            "schema lacks generation field",
            ACCESSIBILITY_GENERATION_PATH,
        )
    } else {
        FamilySummary::ok("accessibility-generation", ACCESSIBILITY_GENERATION_PATH)
    }
}

fn schema_failure_path(
    root: &Path,
    error: &validators::schema::ContractSchemaError,
    fallback: &str,
) -> String {
    match error.failure_family(root) {
        validators::schema::ContractSchemaFailure::Compilation => fallback.to_owned(),
        validators::schema::ContractSchemaFailure::Instances(path)
        | validators::schema::ContractSchemaFailure::Fixtures(path) => summary_path(root, &path),
    }
}

fn summary_from_result<T, Error>(
    family: &'static str,
    contract_path: &'static str,
    result: Result<T, Error>,
) -> FamilySummary {
    match result {
        Ok(_) => FamilySummary::ok(family, contract_path),
        Err(_) => FamilySummary::failed(family, contract_path),
    }
}

fn native_summaries(root: &Path, tools: &validators::native::NativeTools) -> [FamilySummary; 4] {
    [
        summary_from_native_result(
            "c-cpp-header",
            HEADER_PATH,
            validators::native::validate_header_syntax(root, tools),
        ),
        summary_from_native_result(
            "binding",
            BINDINGS_PATH,
            validators::native::validate_generated_bindings(root, tools),
        ),
        summary_from_native_result(
            "symbol",
            HEADER_PATH,
            validators::native::validate_header_symbols(root, tools),
        ),
        summary_from_native_result(
            "layout",
            HEADER_PATH,
            validators::native::validate_host_layout(root, tools),
        ),
    ]
}

fn native_failure_path(error: &validators::native::NativeContractError) -> &'static str {
    match error {
        validators::native::NativeContractError::Toolchain(
            toolchain::ToolchainError::UnsupportedHost { .. },
        ) => UNSUPPORTED_HOST,
        validators::native::NativeContractError::MissingToolchainManifest
        | validators::native::NativeContractError::Toolchain(_)
        | validators::native::NativeContractError::Io(_)
        | validators::native::NativeContractError::Json(_)
        | validators::native::NativeContractError::InvalidFixture { .. }
        | validators::native::NativeContractError::InvalidHeaderPath
        | validators::native::NativeContractError::ToolExecution { .. }
        | validators::native::NativeContractError::ToolFailed { .. }
        | validators::native::NativeContractError::ToolOutputEncoding { .. }
        | validators::native::NativeContractError::MacroFixtureTargetMismatch { .. }
        | validators::native::NativeContractError::MacroExpansionMismatch { .. }
        | validators::native::NativeContractError::MissingNullabilityAnnotation { .. }
        | validators::native::NativeContractError::HeaderAstCoverageMismatch { .. }
        | validators::native::NativeContractError::GoldenDigestMismatch
        | validators::native::NativeContractError::GeneratedBindingsMismatch
        | validators::native::NativeContractError::InterfaceMismatch { .. }
        | validators::native::NativeContractError::LayoutMismatch
        | validators::native::NativeContractError::TemporaryDirectory => TOOLCHAIN_MANIFEST_PATH,
    }
}

fn native_load_failure(contract_path: &'static str) -> [FamilySummary; 4] {
    [
        FamilySummary::failed("c-cpp-header", contract_path),
        FamilySummary::failed("binding", contract_path),
        FamilySummary::failed("symbol", contract_path),
        FamilySummary::failed("layout", contract_path),
    ]
}

fn summary_from_native_result(
    family: &'static str,
    contract_path: &'static str,
    result: Result<(), validators::native::NativeContractError>,
) -> FamilySummary {
    match result {
        Ok(()) => FamilySummary::ok(family, contract_path),
        Err(validators::native::NativeContractError::Toolchain(
            toolchain::ToolchainError::UnsupportedHost { .. },
        )) => FamilySummary::failed(family, UNSUPPORTED_HOST),
        Err(_) => FamilySummary::failed(family, contract_path),
    }
}

fn readiness_summaries(
    result: Result<
        validators::readiness::ReadinessReport,
        validators::readiness::ReadinessValidationError,
    >,
) -> [FamilySummary; 2] {
    match result {
        Ok(_) => [
            FamilySummary::ok("readiness", LOCK_PATH),
            FamilySummary::ok("promotion", PHASE_PATH),
        ],
        Err(error) if error.is_promotion_only() => [
            FamilySummary::ok("readiness", LOCK_PATH),
            FamilySummary::failed("promotion", PHASE_PATH),
        ],
        Err(_) => [
            FamilySummary::failed("readiness", LOCK_PATH),
            FamilySummary::failed("promotion", LOCK_PATH),
        ],
    }
}

fn rust_contract_summary(result: Result<(), RustContractError>) -> FamilySummary {
    match result {
        Ok(()) => FamilySummary::ok("rust-contract", RUST_CONTRACTS_PATH),
        Err(RustContractError::UnsupportedHost) => {
            FamilySummary::failed("rust-contract", UNSUPPORTED_HOST)
        }
        Err(RustContractError::ValidationFailed(path)) => {
            FamilySummary::failed_path("rust-contract", path)
        }
    }
}

/// Reports why authoritative Rust-contract validation could not complete.
enum RustContractError {
    /// The staged native toolchain does not support the current host.
    UnsupportedHost,
    /// Rust contract parsing or its setup failed at the reported repository path.
    ValidationFailed(String),
}

/// Parses all authoritative Rust contracts with the manifest-verified pinned compiler.
///
/// This validation loads the staged manifest independently from native-tool validation. It uses
/// the manifest's verified `rustc` path for each metadata-only library and doesn't require a C
/// toolchain. Exact declared-symbol resolution remains the responsibility of the traceability
/// family.
fn validate_rust_contracts(root: &Path) -> Result<(), RustContractError> {
    let rustc = validators::native::load_rust_contract_compiler(root).map_err(|error| {
        if matches!(
            error,
            validators::native::NativeContractError::Toolchain(
                toolchain::ToolchainError::UnsupportedHost { .. }
            )
        ) {
            RustContractError::UnsupportedHost
        } else {
            RustContractError::ValidationFailed(RUST_CONTRACTS_PATH.to_owned())
        }
    })?;
    let temporary = ContractTemporaryDirectory::new()
        .map_err(|_| RustContractError::ValidationFailed(RUST_CONTRACTS_PATH.to_owned()))?;
    for (crate_name, contract_path) in [
        ("oxyflut_public_contract", "oxyflut-public.rs"),
        ("oxyflut_substrate_contract", "oxyflut-substrate.rs"),
        ("oxyflut_qualification_contract", "oxyflut-qualification.rs"),
    ] {
        let path = root.join(RUST_CONTRACTS_PATH).join(contract_path);
        let status = Command::new(&rustc)
            .args([
                "--crate-name",
                crate_name,
                "--crate-type",
                "lib",
                "--edition",
                "2024",
                "--emit",
                "metadata",
                "--out-dir",
            ])
            .arg(temporary.path())
            .arg(&path)
            .output();
        if !status.is_ok_and(|output| output.status.success()) {
            return Err(RustContractError::ValidationFailed(summary_path(
                root, &path,
            )));
        }
    }
    Ok(())
}

struct ContractTemporaryDirectory {
    path: PathBuf,
}

impl ContractTemporaryDirectory {
    fn new() -> Result<Self, std::io::Error> {
        for _ in 0..128 {
            let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "oxyflut-contract-validation-{}-{sequence}",
                std::process::id()
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self { path }),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error),
            }
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "could not create a contract-validation temporary directory",
        ))
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for ContractTemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

const FAMILY_COUNT: usize = 14;

#[derive(Debug, Eq, PartialEq)]
struct ContractValidationReport {
    summaries: Vec<FamilySummary>,
}

impl ContractValidationReport {
    fn emit(&self) {
        for summary in &self.summaries {
            println!("{}", summary.line());
        }
    }

    fn outcome(&self) -> CommandOutcome {
        if self
            .summaries
            .iter()
            .all(|summary| !matches!(summary.status, FamilyStatus::Failed))
        {
            CommandOutcome::Success
        } else {
            CommandOutcome::failed(CommandError::ValidationFailed {
                code: "contracts-invalid",
                hint: "rerun: contracts validate",
            })
        }
    }

    #[cfg(test)]
    fn summary_lines(&self) -> Vec<String> {
        self.summaries.iter().map(FamilySummary::line).collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct FamilySummary {
    family: &'static str,
    status: FamilyStatus,
    contract_path: String,
}

impl FamilySummary {
    fn ok(family: &'static str, contract_path: &'static str) -> Self {
        Self {
            family,
            status: FamilyStatus::Ok,
            contract_path: contract_path.to_owned(),
        }
    }

    fn failed(family: &'static str, contract_path: &'static str) -> Self {
        Self {
            family,
            status: FamilyStatus::Failed,
            contract_path: contract_path.to_owned(),
        }
    }

    fn failed_path(family: &'static str, contract_path: String) -> Self {
        Self {
            family,
            status: FamilyStatus::Failed,
            contract_path,
        }
    }

    fn deferred(
        family: &'static str,
        reason: impl Into<String>,
        contract_path: &'static str,
    ) -> Self {
        Self {
            family,
            status: FamilyStatus::Deferred {
                reason: reason.into(),
            },
            contract_path: contract_path.to_owned(),
        }
    }

    fn is_failed(&self) -> bool {
        matches!(&self.status, FamilyStatus::Failed)
    }

    fn line(&self) -> String {
        match &self.status {
            FamilyStatus::Ok => format!("{}: ok ({})", self.family, self.contract_path),
            FamilyStatus::Failed => format!("{}: failed ({})", self.family, self.contract_path),
            FamilyStatus::Deferred { reason } => {
                format!(
                    "{}: deferred ({}; {})",
                    self.family, reason, self.contract_path
                )
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum FamilyStatus {
    Ok,
    Failed,
    Deferred { reason: String },
}

fn summary_path(root: &Path, path: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(relative) => relative.display().to_string(),
        Err(_) if path.is_relative() => path.display().to_string(),
        Err(_) => "unknown-local-path".to_owned(),
    }
}

fn workspace_root() -> Result<PathBuf, ()> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::path::Path;

    use super::{
        ContractTemporaryDirectory, FAMILY_COUNT, RustContractError,
        accessibility_generation_summary, contract_test_summary, readiness_summaries,
        rust_contract_summary, validate_workspace, validators, workspace_root,
    };
    use crate::CommandOutcome;

    #[test]
    fn contracts_validation_runs_all_families_with_stable_content_free_summaries()
    -> Result<(), Box<dyn Error>> {
        if skip_on_unsupported_host()? {
            return Ok(());
        }
        let root = workspace_root()
            .map_err(|_| std::io::Error::other("xtask must remain below the workspace root"))?;
        let report = validate_workspace(&root);
        assert_eq!(report.summaries.len(), FAMILY_COUNT);
        assert_eq!(
            report.summary_lines(),
            vec![
                "schema: ok (.constitution/tech-spec/data-models)",
                "instance: ok (.constitution/tech-spec/contracts)",
                "exact-set: ok (.constitution/tech-spec/contracts/capability-traceability.json)",
                "contract-tests: deferred (52 pending candidate implementation; .constitution/tech-spec/contracts/capability-traceability.json)",
                "accessibility-generation: deferred (schema lacks generation field; .constitution/tech-spec/data-models/accessibility-map.schema.json)",
                "registry: ok (.constitution/tech-spec/contracts/diagnostic-event-registry.json)",
                "digest: ok (.constitution/tech-spec/contracts)",
                "readiness: ok (.constitution/tech-spec/contracts/qualification-lock.json)",
                "promotion: ok (.constitution/tech-spec/contracts/specification-phase.json)",
                "rust-contract: ok (.constitution/tech-spec/contracts)",
                "c-cpp-header: ok (.constitution/tech-spec/contracts/oxyflut-substrate.h)",
                "binding: ok (qualification/fixtures/generated-bindings/oxyflut-substrate.rs)",
                "symbol: ok (.constitution/tech-spec/contracts/oxyflut-substrate.h)",
                "layout: ok (.constitution/tech-spec/contracts/oxyflut-substrate.h)",
            ]
        );
        assert_eq!(report.outcome(), CommandOutcome::Success);
        Ok(())
    }

    #[test]
    fn rust_contract_unsupported_host_uses_native_failure_marker() {
        assert_eq!(
            rust_contract_summary(Err(RustContractError::UnsupportedHost)).line(),
            "rust-contract: failed (unsupported host)"
        );
    }

    #[test]
    fn traceability_deferred_summaries_retire_when_the_report_resolves_them() {
        let report = validators::traceability::TraceabilityRunReport {
            capability_count: 52,
            constraint_count: 27,
            contract_test_resolution: validators::traceability::ContractTestResolution::Resolved,
            deferred_contract_tests: 0,
            accessibility_generation_deferred: false,
        };
        assert_eq!(
            contract_test_summary(&report).line(),
            "contract-tests: ok (.constitution/tech-spec/contracts/capability-traceability.json)"
        );
        assert_eq!(
            accessibility_generation_summary(&report).line(),
            "accessibility-generation: ok (.constitution/tech-spec/data-models/accessibility-map.schema.json)"
        );
    }

    #[test]
    fn a_failed_family_returns_exit_one_and_identifies_its_contract_path()
    -> Result<(), Box<dyn Error>> {
        let source = workspace_root()
            .map_err(|_| std::io::Error::other("xtask must remain below the workspace root"))?;
        let temporary = ContractTemporaryDirectory::new()?;
        copy_directory(
            &source.join(".constitution"),
            &temporary.path().join(".constitution"),
        )?;
        copy_directory(
            &source.join("qualification"),
            &temporary.path().join("qualification"),
        )?;
        fs::write(
            temporary
                .path()
                .join(".constitution/tech-spec/contracts/diagnostic-event-registry.json"),
            "{}\n",
        )?;

        let report = validate_workspace(temporary.path());
        assert!(report.summary_lines().contains(&(
            "registry: failed (.constitution/tech-spec/contracts/diagnostic-event-registry.json)"
                .to_owned()
        )));
        assert!(matches!(report.outcome(), CommandOutcome::Failed(_)));
        Ok(())
    }

    #[test]
    fn production_promotion_failure_is_not_attributed_to_readiness() -> Result<(), Box<dyn Error>> {
        let source = workspace_root()
            .map_err(|_| std::io::Error::other("xtask must remain below the workspace root"))?;
        let temporary = ContractTemporaryDirectory::new()?;
        copy_directory(
            &source.join(".constitution"),
            &temporary.path().join(".constitution"),
        )?;
        copy_directory(
            &source.join("qualification"),
            &temporary.path().join("qualification"),
        )?;
        let fixture = source.join("qualification/fixtures/contracts/readiness/production-3b");
        copy_directory(&fixture, temporary.path())?;
        fs::copy(
            fixture.join("production-3b-phase.json"),
            temporary
                .path()
                .join(".constitution/tech-spec/contracts/specification-phase.json"),
        )?;

        let result = validators::readiness::validate_workspace(temporary.path());
        assert!(
            matches!(
                &result,
                Err(validators::readiness::ReadinessValidationError::Promotion(
                    validators::readiness::ReadinessError::ArtifactCannotProveBinding {
                        key: "layoutQualification"
                    }
                ))
            ),
            "{result:?}"
        );
        let summaries = readiness_summaries(result);
        assert_eq!(
            summaries.map(|summary| summary.line()),
            [
                "readiness: ok (.constitution/tech-spec/contracts/qualification-lock.json)"
                    .to_owned(),
                "promotion: failed (.constitution/tech-spec/contracts/specification-phase.json)"
                    .to_owned(),
            ]
        );
        Ok(())
    }

    #[test]
    fn contracts_validation_rejects_arguments() {
        assert!(matches!(
            super::run(&["unexpected".to_owned()]),
            CommandOutcome::Failed(_)
        ));
    }

    fn skip_on_unsupported_host() -> Result<bool, Box<dyn Error>> {
        if crate::toolchain::is_staged_host()? {
            Ok(false)
        } else {
            eprintln!("skipped: staged toolchain host is x86_64-unknown-linux-gnu");
            Ok(true)
        }
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

#[cfg(test)]
mod traceability {
    use std::error::Error;
    use std::path::{Path, PathBuf};

    use super::validators::{registries, traceability};

    #[test]
    fn committed_inputs_and_registry_validate_through_ticket_module_path()
    -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let report = traceability::validate_workspace(&root)?;
        assert_eq!(report.deferred_contract_tests, 52);
        registries::validate_workspace(&root)?;
        Ok(())
    }

    fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| "xtask must remain directly below the workspace root".into())
    }
}
