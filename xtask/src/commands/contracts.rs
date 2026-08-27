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
    let report = match validators::schema::validate_workspace(&root) {
        Ok(report) => report,
        Err(_) => {
            eprintln!("schema: failed");
            eprintln!("instances: not-run");
            return CommandOutcome::failed(CommandError::ValidationFailed("schema".to_owned()));
        }
    };

    let _ = report;
    eprintln!("schema: ok");
    eprintln!("instances: ok");
    eprintln!("fixtures: ok");
    for family in validators::unimplemented_families() {
        eprintln!("{family}: not-implemented");
    }
    CommandOutcome::not_implemented("contracts validation families")
}

fn workspace_root() -> Result<PathBuf, ()> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or(())
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::CommandOutcome;

    #[test]
    fn contracts_schema_family_runs_before_deferred_families_fail_closed() {
        assert_eq!(
            run(&[]),
            CommandOutcome::not_implemented("contracts validation families")
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
