//! Placeholder for the raw-measurement validation command.

use super::super::CommandOutcome;

pub(crate) fn run(_arguments: &[String]) -> CommandOutcome {
    CommandOutcome::not_implemented("measurement validate")
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::CommandOutcome;

    #[test]
    fn reports_the_measurement_placeholder_status() {
        assert_eq!(
            run(&[]),
            CommandOutcome::not_implemented("measurement validate")
        );
    }
}
