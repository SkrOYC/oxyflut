//! Qualification-lock readiness status reporting.

use std::fs;
use std::path::{Path, PathBuf};

use oxyflut_qualification::readiness::{
    EXTERNAL_CONTRACT_LOCK_PATH, ReadinessBlocking, ReadinessReport, ReadinessStatus,
    StagedInputRegistry, candidate_implementation_report,
};
use serde_json::{Map, Value};

use super::super::{CommandError, CommandOutcome};
use crate::contracts as validators;
use crate::toolchain::{self, ToolchainManifest};
use validators::readiness::GateStatus;

const LOCK_PATH: &str = ".constitution/tech-spec/contracts/qualification-lock.json";
const TOOLCHAIN_MANIFEST_PATH: &str = "qualification/tools/native-contract-toolchain.json";

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

    let validated = match validators::readiness::validate_workspace(root) {
        Ok(report) => report,
        Err(_) => {
            println!("lock status: invalid (readiness)");
            return CommandOutcome::failed(CommandError::ValidationFailed {
                code: "lock-invalid",
                hint: "rerun: lock status --gate GATE",
            });
        }
    };

    match gate {
        "candidate-implementation" => report_candidate_implementation_gate(root),
        "measurement" => report_gate(gate, &validated.measurement),
        _ => CommandOutcome::failed(CommandError::InvalidInput {
            code: "lock-status-gate",
        }),
    }
}

fn report_candidate_implementation_gate(root: &Path) -> CommandOutcome {
    let report = match candidate_report_at(root) {
        Ok(report) => report,
        Err(()) => {
            println!("lock status: invalid (readiness-report)");
            return CommandOutcome::failed(CommandError::ValidationFailed {
                code: "lock-invalid",
                hint: "rerun: lock status --gate GATE",
            });
        }
    };
    emit_candidate_report(&report)
}

fn emit_candidate_report(report: &ReadinessReport) -> CommandOutcome {
    for line in candidate_report_lines(report) {
        println!("{line}");
    }
    match report.status {
        ReadinessStatus::Ready => CommandOutcome::Success,
        ReadinessStatus::Open => CommandOutcome::ValidButOpen,
    }
}

fn candidate_report_at(root: &Path) -> Result<ReadinessReport, ()> {
    let bytes = fs::read(root.join(LOCK_PATH)).map_err(|_| ())?;
    let lock: Value = serde_json::from_slice(&bytes).map_err(|_| ())?;
    let external_bytes = fs::read(root.join(EXTERNAL_CONTRACT_LOCK_PATH)).map_err(|_| ())?;
    let active_external_lock: Value = serde_json::from_slice(&external_bytes).map_err(|_| ())?;
    validate_staged_candidate_inputs(root, &lock)?;
    candidate_implementation_report(&lock, &active_external_lock).map_err(|_| ())
}

fn validate_staged_candidate_inputs(root: &Path, lock: &Value) -> Result<(), ()> {
    let policy = lock
        .get("measurementPolicy")
        .and_then(Value::as_object)
        .ok_or(())?;
    for field in [
        "sampleValidityRules",
        "scoringAnchors",
        "assessors",
        "fuzzCorpora",
        "securityPatchRehearsal",
    ] {
        let path = StagedInputRegistry::measurement_policy_path(field).ok_or(())?;
        verify_optional_staged_digest(root, policy, field, path)?;
    }
    verify_resolved_tools(root, lock)
}

fn verify_optional_staged_digest(
    root: &Path,
    policy: &Map<String, Value>,
    field: &str,
    path: &str,
) -> Result<(), ()> {
    match policy.get(field) {
        Some(Value::Null) => Ok(()),
        Some(Value::String(digest)) => validators::digests::verify_reference(root, path, digest)
            .map(|_| ())
            .map_err(|_| ()),
        Some(_) | None => Err(()),
    }
}

fn verify_resolved_tools(root: &Path, lock: &Value) -> Result<(), ()> {
    let tools = lock
        .get("resolvedTools")
        .and_then(Value::as_array)
        .ok_or(())?;
    if tools.is_empty() {
        return Ok(());
    }
    let manifest = ToolchainManifest::from_json(
        &fs::read(root.join(TOOLCHAIN_MANIFEST_PATH)).map_err(|_| ())?,
    )
    .map_err(|_| ())?;
    toolchain::verify_lock_resolved_tools(&manifest, tools).map_err(|_| ())
}

fn candidate_report_lines(report: &ReadinessReport) -> Vec<String> {
    let mut lines = Vec::with_capacity(report.blocking.len().saturating_add(1));
    lines.push(format!(
        "lock status: {} ({})",
        report.status.as_str(),
        report.gate.as_str()
    ));
    lines.extend(report.blocking.iter().map(blocking_line));
    lines
}

fn blocking_line(blocking: &ReadinessBlocking) -> String {
    let mut line = format!(
        "blocking: field-path={} kind={}",
        blocking.field_path,
        blocking.kind.as_str()
    );
    if let Some(evidence_path) = &blocking.evidence_path {
        line.push_str(" evidence-path=");
        line.push_str(evidence_path);
    }
    if let Some(referent) = blocking.referent {
        line.push_str(" referent=");
        line.push_str(referent.as_str());
    }
    if let Some(upstream_owner) = &blocking.upstream_owner {
        line.push_str(" upstream-owner=");
        line.push_str(upstream_owner);
    }
    line
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
#[path = "lock_tests.rs"]
mod tests;
