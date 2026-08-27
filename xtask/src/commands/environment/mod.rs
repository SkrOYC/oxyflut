//! Placeholder for the reference-environment inspection command.

use super::super::CommandOutcome;

pub(crate) fn run(_arguments: &[String]) -> CommandOutcome {
    CommandOutcome::not_implemented("environment inspect")
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::CommandOutcome;

    #[test]
    fn reports_the_environment_placeholder_status() {
        assert_eq!(
            run(&[]),
            CommandOutcome::not_implemented("environment inspect")
        );
    }
}
