//! Placeholder for the qualification-lock status command.

use super::super::CommandOutcome;

pub(crate) fn run(_arguments: &[String]) -> CommandOutcome {
    CommandOutcome::not_implemented("lock status")
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::CommandOutcome;

    #[test]
    fn reports_the_lock_placeholder_status() {
        assert_eq!(run(&[]), CommandOutcome::not_implemented("lock status"));
    }
}
