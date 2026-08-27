//! Process-level output routing tests for qualification commands.

use std::error::Error;
use std::process::Command;

#[test]
fn contract_reports_use_stdout_and_keep_diagnostics_empty() -> Result<(), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(["contracts", "validate"])
        .output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("schema: ok"));
    assert!(output.stderr.is_empty());
    Ok(())
}

#[test]
fn open_lock_reports_use_stdout_and_keep_diagnostics_empty() -> Result<(), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(["lock", "status", "--gate", "candidate-implementation"])
        .output()?;
    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("lock status: open (candidate-implementation)"));
    assert!(output.stderr.is_empty());
    Ok(())
}
