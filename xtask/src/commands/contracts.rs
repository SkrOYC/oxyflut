//! The fail-closed contract-validation command.

#[path = "../contracts/mod.rs"]
mod validators;

use std::path::{Path, PathBuf};

use super::super::{CommandError, CommandOutcome};

/// Runs the schema and instance validators, then reports deferred contract families.
pub(crate) fn run(arguments: &[String]) -> CommandOutcome {
    if !arguments.is_empty() {
        return CommandOutcome::failed(CommandError::InvalidInput(
            "contracts validate accepts no arguments".to_owned(),
        ));
    }

    let root = match workspace_root() {
        Ok(root) => root,
        Err(()) => return CommandOutcome::failed(CommandError::Execution("root".to_owned())),
    };
    let registry = match validators::schema::compile_workspace(&root) {
        Ok(registry) => registry,
        Err(_) => {
            eprintln!("schema: failed");
            eprintln!("instances: not-run");
            return CommandOutcome::failed(CommandError::ValidationFailed("schema".to_owned()));
        }
    };
    let report = match validators::schema::validate_compiled_workspace(&root, &registry) {
        Ok(report) => report,
        Err(error) => {
            report_schema_failure(&error, &root);
            return CommandOutcome::failed(CommandError::ValidationFailed("schema".to_owned()));
        }
    };

    let _ = report;
    let traceability = match validators::traceability::validate_workspace(&root) {
        Ok(report) => report,
        Err(_) => {
            eprintln!("schema: ok");
            eprintln!("instances: ok");
            eprintln!("fixtures: ok");
            eprintln!("traceability: failed");
            return CommandOutcome::failed(CommandError::ValidationFailed(
                "traceability".to_owned(),
            ));
        }
    };
    if validators::registries::validate_workspace(&root).is_err() {
        eprintln!("schema: ok");
        eprintln!("instances: ok");
        eprintln!("fixtures: ok");
        eprintln!("traceability: ok");
        eprintln!("registries: failed");
        return CommandOutcome::failed(CommandError::ValidationFailed("registries".to_owned()));
    }

    eprintln!("schema: ok");
    eprintln!("instances: ok");
    eprintln!("fixtures: ok");
    eprintln!("traceability: ok");
    eprintln!(
        "contract-tests: deferred ({} pending candidate implementation)",
        traceability.deferred_contract_tests
    );
    eprintln!("accessibility-generation: deferred (schema lacks generation field)");
    eprintln!("registries: ok");
    for family in validators::unimplemented_families() {
        eprintln!("{family}: not-implemented");
    }
    CommandOutcome::not_implemented("contracts validation families")
}

fn report_schema_failure(error: &validators::schema::ContractSchemaError, root: &Path) {
    for line in schema_failure_lines(error, root) {
        eprintln!("{line}");
    }
}

fn schema_failure_lines(
    error: &validators::schema::ContractSchemaError,
    root: &Path,
) -> Vec<String> {
    match error.failure_family(root) {
        validators::schema::ContractSchemaFailure::Compilation => {
            vec!["schema: failed".to_owned(), "instances: not-run".to_owned()]
        }
        validators::schema::ContractSchemaFailure::Instances(path) => vec![
            "schema: ok".to_owned(),
            format!("instances: failed ({})", summary_path(root, &path)),
        ],
        validators::schema::ContractSchemaFailure::Fixtures(path) => vec![
            "schema: ok".to_owned(),
            "instances: ok".to_owned(),
            format!("fixtures: failed ({})", summary_path(root, &path)),
        ],
    }
}

fn summary_path(root: &Path, path: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(relative) => relative.display().to_string(),
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
    use std::path::Path;

    use oxyflut_qualification::schema::SchemaError;

    use super::{run, schema_failure_lines, validators};
    use crate::CommandOutcome;

    #[test]
    fn contracts_schema_family_runs_before_deferred_families_fail_closed() {
        assert_eq!(
            run(&[]),
            CommandOutcome::not_implemented("contracts validation families")
        );
    }

    #[test]
    fn contracts_schema_failure_summary_identifies_the_failure_family() {
        let root = Path::new("/workspace");
        let compilation =
            validators::schema::ContractSchemaError::Registry(SchemaError::Compilation);
        assert_eq!(
            schema_failure_lines(&compilation, root),
            vec!["schema: failed", "instances: not-run"]
        );

        let instance = validators::schema::ContractSchemaError::MissingSchema {
            path: root.join(".constitution/tech-spec/contracts/invalid.json"),
        };
        assert_eq!(
            schema_failure_lines(&instance, root),
            vec![
                "schema: ok",
                "instances: failed (.constitution/tech-spec/contracts/invalid.json)",
            ]
        );

        let fixture = validators::schema::ContractSchemaError::Fixture {
            path: root.join("qualification/fixtures/contracts/example/invalid.json"),
        };
        assert_eq!(
            schema_failure_lines(&fixture, root),
            vec![
                "schema: ok",
                "instances: ok",
                "fixtures: failed (qualification/fixtures/contracts/example/invalid.json)",
            ]
        );
    }

    #[test]
    fn contracts_schema_family_rejects_arguments() {
        assert!(matches!(
            run(&["unexpected".to_owned()]),
            CommandOutcome::Failed(_)
        ));
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
