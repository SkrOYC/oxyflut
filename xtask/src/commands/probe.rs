//! Placeholder for the candidate-probe command.

use super::super::CommandOutcome;

pub(crate) fn run(_arguments: &[String]) -> CommandOutcome {
    CommandOutcome::not_implemented("probe")
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::CommandOutcome;

    #[test]
    fn reports_the_probe_placeholder_status() {
        assert_eq!(run(&[]), CommandOutcome::not_implemented("probe"));
    }
}
