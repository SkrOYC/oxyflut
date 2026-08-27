//! Placeholder for the external-contract validation command.

use super::super::CommandOutcome;

pub(crate) fn run(_arguments: &[String]) -> CommandOutcome {
    CommandOutcome::not_implemented("external-contracts verify")
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::CommandOutcome;

    #[test]
    fn reports_the_external_contracts_placeholder_status() {
        assert_eq!(
            run(&[]),
            CommandOutcome::not_implemented("external-contracts verify")
        );
    }
}
