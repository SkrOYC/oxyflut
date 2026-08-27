//! The `evidence verify PATH` command.

use crate::evidence;

use super::super::{CommandError, CommandOutcome};

/// Verifies one repository-relative evidence file without modifying it.
pub(crate) fn run(arguments: &[String]) -> CommandOutcome {
    let [path] = arguments else {
        return CommandOutcome::failed(CommandError::InvalidInput {
            code: "evidence-verify-arguments",
        });
    };
    let outcome = evidence::repository_root().and_then(|root| evidence::verify(&root, path));
    match outcome {
        Ok(()) => CommandOutcome::Success,
        Err(error) => CommandOutcome::failed(CommandError::ValidationFailed {
            code: error.code(),
            hint: "rerun: evidence verify PATH",
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{CommandError, CommandOutcome};

    #[test]
    fn verifies_positive_fixture_and_rejects_invalid_evidence() {
        assert_eq!(
            run(&["qualification/fixtures/evidence/positive-derived.json".to_owned()]),
            CommandOutcome::Success
        );
        assert_eq!(
            run(&["qualification/fixtures/evidence/bad-digest.json".to_owned()]),
            CommandOutcome::Failed(CommandError::ValidationFailed {
                code: "digest-mismatch",
                hint: "rerun: evidence verify PATH",
            })
        );
        assert!(matches!(run(&[]), CommandOutcome::Failed(_)));
    }
}
