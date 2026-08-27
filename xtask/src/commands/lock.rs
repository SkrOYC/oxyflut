//! Qualification-lock readiness status reporting.

use std::path::{Path, PathBuf};

use super::super::{CommandError, CommandOutcome};
use crate::contracts as validators;
use validators::readiness::GateStatus;

/// Validates one requested readiness gate without modifying the qualification lock.
///
/// The command runs the schema, instance, exact-set, registry, and digest families before the
/// readiness gate. A failed family returns exit code 1 as `lock status: invalid (FAMILY; PATH)`.
pub(crate) fn run(arguments: &[String]) -> CommandOutcome {
    let gate = match arguments {
        [flag, gate] if flag == "--gate" => gate.as_str(),
        _ => {
            return CommandOutcome::failed(CommandError::InvalidInput {
                code: "lock-status-arguments",
            });
        }
    };
    let root = match workspace_root() {
        Ok(root) => root,
        Err(()) => {
            return CommandOutcome::failed(CommandError::Execution {
                code: "workspace-root",
                hint: "rerun: lock status --gate GATE",
            });
        }
    };
    run_at_root(&root, gate)
}

fn run_at_root(root: &Path, gate: &str) -> CommandOutcome {
    if let Some(failure) = super::contracts::first_pre_implementation_input_failure(root) {
        println!("{}", invalid_status_line(&failure));
        return CommandOutcome::failed(CommandError::ValidationFailed {
            code: "lock-invalid",
            hint: "rerun: lock status --gate GATE",
        });
    }

    let report = match validators::readiness::validate_workspace(root) {
        Ok(report) => report,
        Err(_) => {
            return CommandOutcome::failed(CommandError::ValidationFailed {
                code: "lock-invalid",
                hint: "rerun: lock status --gate GATE",
            });
        }
    };

    match gate {
        "candidate-implementation" => report_gate(gate, &report.candidate_implementation),
        "measurement" => report_gate(gate, &report.measurement),
        _ => CommandOutcome::failed(CommandError::InvalidInput {
            code: "lock-status-gate",
        }),
    }
}

fn invalid_status_line(failure: &super::contracts::ValidationFamilyFailure) -> String {
    format!(
        "lock status: invalid ({}; {})",
        failure.family, failure.contract_path
    )
}

fn report_gate(gate: &str, status: &GateStatus) -> CommandOutcome {
    match status {
        GateStatus::Ready => {
            println!("lock status: ready ({gate})");
            CommandOutcome::Success
        }
        GateStatus::Open(_) => {
            println!("lock status: open ({gate})");
            for known_unknown in status.remaining_known_unknowns() {
                println!("remaining-ku: {known_unknown}");
            }
            CommandOutcome::ValidButOpen
        }
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
    use std::path::{Path, PathBuf};
    use std::process::ExitCode;

    use super::{invalid_status_line, run, run_at_root};
    use crate::CommandOutcome;

    #[test]
    fn committed_candidate_gate_is_valid_but_open_without_mutating_the_lock() {
        assert_eq!(
            run(&["--gate".to_owned(), "candidate-implementation".to_owned()]),
            CommandOutcome::ValidButOpen
        );
    }

    #[test]
    fn corrupt_platform_baseline_fails_before_reporting_an_open_gate() -> Result<(), Box<dyn Error>>
    {
        let source = workspace_root()?;
        let root = temporary_directory("corrupt-platform-baseline");
        copy_directory(&source.join(".constitution"), &root.join(".constitution"))?;
        copy_directory(&source.join("qualification"), &root.join("qualification"))?;

        let platform_path = root.join(".constitution/tech-spec/contracts/platform-contracts.json");
        let original = fs::read_to_string(&platform_path)?;
        let corrupted = original.replacen(
            "\"specificationVersion\": \"0.15.0\"",
            "\"specificationVersion\": \"0.15.1\"",
            1,
        );
        assert_ne!(corrupted, original);
        fs::write(platform_path, corrupted)?;

        let outcome = run_at_root(&root, "candidate-implementation");
        assert!(matches!(&outcome, CommandOutcome::Failed(_)));
        assert_eq!(outcome.exit_code(), ExitCode::FAILURE);
        let failure = super::super::contracts::first_pre_implementation_input_failure(&root)
            .ok_or("the corrupt platform baseline must fail a pre-implementation family")?;
        assert_eq!(
            invalid_status_line(&failure),
            "lock status: invalid (exact-set; .constitution/tech-spec/contracts/capability-traceability.json)"
        );
        fs::remove_dir_all(root)?;
        Ok(())
    }

    #[test]
    fn reports_measurement_gate_and_rejects_invalid_arguments() {
        assert_eq!(
            run(&["--gate".to_owned(), "measurement".to_owned()]),
            CommandOutcome::ValidButOpen
        );
        assert!(matches!(run(&[]), CommandOutcome::Failed(_)));
        assert!(matches!(
            run(&["--gate".to_owned(), "production".to_owned()]),
            CommandOutcome::Failed(_)
        ));
    }

    fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| "xtask must remain directly below the workspace root".into())
    }

    fn temporary_directory(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("oxyflut-lock-{name}-{}", std::process::id()))
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
