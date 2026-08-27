//! Placeholder for the candidate-build command.

use super::super::CommandOutcome;

pub(crate) fn run(_arguments: &[String]) -> CommandOutcome {
    CommandOutcome::not_implemented("candidate build")
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::CommandOutcome;

    #[test]
    fn reports_the_candidate_placeholder_status() {
        assert_eq!(run(&[]), CommandOutcome::not_implemented("candidate build"));
    }
}
