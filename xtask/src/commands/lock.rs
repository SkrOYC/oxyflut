//! Qualification-lock readiness status reporting.

use std::path::{Path, PathBuf};

use super::super::{CommandError, CommandOutcome};
use crate::contracts as validators;
use validators::readiness::GateStatus;

/// Validates one requested readiness gate without modifying the qualification lock.
pub(crate) fn run(arguments: &[String]) -> CommandOutcome {
    let gate = match arguments {
        [flag, gate] if flag == "--gate" => gate.as_str(),
        _ => {
            return CommandOutcome::failed(CommandError::InvalidInput(
                "lock status requires --gate candidate-implementation|measurement".to_owned(),
            ));
        }
    };
    let root = match workspace_root() {
        Ok(root) => root,
        Err(()) => return CommandOutcome::failed(CommandError::Execution("root".to_owned())),
    };
    let report = match validators::readiness::validate_workspace(&root) {
        Ok(report) => report,
        Err(_) => return CommandOutcome::failed(CommandError::ValidationFailed("lock".to_owned())),
    };

    match gate {
        "candidate-implementation" => report_gate(gate, &report.candidate_implementation),
        "measurement" => report_gate(gate, &report.measurement),
        _ => CommandOutcome::failed(CommandError::InvalidInput(
            "lock status requires --gate candidate-implementation|measurement".to_owned(),
        )),
    }
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
    use super::run;
    use crate::CommandOutcome;

    #[test]
    fn committed_candidate_gate_is_valid_but_open_without_mutating_the_lock() {
        assert_eq!(
            run(&["--gate".to_owned(), "candidate-implementation".to_owned()]),
            CommandOutcome::ValidButOpen
        );
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
}
