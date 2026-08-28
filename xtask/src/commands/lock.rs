//! Qualification-lock readiness status reporting.

use std::fs;
use std::path::{Path, PathBuf};

use oxyflut_qualification::readiness::{
    EXTERNAL_CONTRACT_LOCK_PATH, ReadinessBlocking, ReadinessError, ReadinessReport,
    ReadinessStatus, StagedInputRegistry, candidate_implementation_report,
};
use serde_json::{Map, Value};

use super::super::{CommandError, CommandOutcome};
use crate::contracts as validators;
use crate::toolchain::{self, ToolchainError, ToolchainManifest};
use validators::readiness::GateStatus;

const LOCK_PATH: &str = ".constitution/tech-spec/contracts/qualification-lock.json";
const TOOLCHAIN_MANIFEST_PATH: &str = "qualification/tools/native-contract-toolchain.json";

/// Validates one requested readiness gate without modifying the qualification lock.
///
/// The command reads the lock and external contract lock, verifies staged inputs, and derives the
/// candidate gate report before it runs the schema, instance, exact-set, registry, digest, and
/// readiness validators. A failed validator returns exit code 1 as `lock status: invalid (FAMILY; PATH)`.
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
    match gate {
        "candidate-implementation" => report_candidate_implementation_gate(root),
        "measurement" => report_measurement_gate(root, gate),
        _ => CommandOutcome::failed(CommandError::InvalidInput {
            code: "lock-status-gate",
        }),
    }
}

fn report_candidate_implementation_gate(root: &Path) -> CommandOutcome {
    let report = match candidate_report_at(root) {
        Ok(report) => report,
        Err(error) => return emit_invalid_candidate_report(error),
    };
    if let Some(failure) = super::contracts::first_pre_implementation_input_failure(root) {
        println!("{}", invalid_status_line(&failure));
        return invalid_outcome();
    }
    if validators::readiness::validate_workspace(root).is_err() {
        println!("lock status: invalid (readiness)");
        return invalid_outcome();
    }
    emit_candidate_report(&report)
}

fn report_measurement_gate(root: &Path, gate: &str) -> CommandOutcome {
    if let Some(failure) = super::contracts::first_pre_implementation_input_failure(root) {
        println!("{}", invalid_status_line(&failure));
        return invalid_outcome();
    }
    let validated = match validators::readiness::validate_workspace(root) {
        Ok(report) => report,
        Err(_) => {
            println!("lock status: invalid (readiness)");
            return invalid_outcome();
        }
    };
    report_gate(gate, &validated.measurement)
}

fn invalid_outcome() -> CommandOutcome {
    CommandOutcome::failed(CommandError::ValidationFailed {
        code: "lock-invalid",
        hint: "rerun: lock status --gate GATE",
    })
}

fn emit_invalid_candidate_report(error: CandidateReportError) -> CommandOutcome {
    for line in candidate_report_error_lines(&error) {
        println!("{line}");
    }
    invalid_outcome()
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

fn candidate_report_at(root: &Path) -> Result<ReadinessReport, CandidateReportError> {
    let bytes = fs::read(root.join(LOCK_PATH)).map_err(|_| CandidateReportError::LockRead)?;
    let lock: Value = serde_json::from_slice(&bytes).map_err(|_| CandidateReportError::LockJson)?;
    let external_bytes = fs::read(root.join(EXTERNAL_CONTRACT_LOCK_PATH))
        .map_err(|_| CandidateReportError::ExternalLockRead)?;
    let active_external_lock: Value = serde_json::from_slice(&external_bytes)
        .map_err(|_| CandidateReportError::ExternalLockJson)?;
    validate_staged_candidate_inputs(root, &lock).map_err(CandidateReportError::StagedInput)?;
    candidate_implementation_report(&lock, &active_external_lock)
        .map_err(CandidateReportError::Readiness)
}

fn validate_staged_candidate_inputs(
    root: &Path,
    lock: &Value,
) -> Result<(), StagedCandidateInputError> {
    let policy = lock
        .get("measurementPolicy")
        .and_then(Value::as_object)
        .ok_or(StagedCandidateInputError::Policy)?;
    for (field, path, upstream_owner) in StagedInputRegistry::candidate_status_input_bindings() {
        verify_optional_staged_digest(root, policy, field, path, upstream_owner)?;
    }
    verify_resolved_tools(root, lock)
}

fn verify_optional_staged_digest(
    root: &Path,
    policy: &Map<String, Value>,
    field: &'static str,
    path: &'static str,
    upstream_owner: &'static str,
) -> Result<(), StagedCandidateInputError> {
    match policy.get(field) {
        Some(Value::Null) => Ok(()),
        Some(Value::String(digest)) => {
            match validators::digests::verify_reference(root, path, digest) {
                Ok(_) => Ok(()),
                Err(validators::digests::DigestError::DigestMismatch { .. }) => {
                    Err(StagedCandidateInputError::DigestMismatch {
                        field,
                        path,
                        upstream_owner,
                    })
                }
                Err(validators::digests::DigestError::MissingFile { .. }) => {
                    Err(StagedCandidateInputError::MissingFile)
                }
                Err(
                    validators::digests::DigestError::InvalidPath { .. }
                    | validators::digests::DigestError::AbsolutePath { .. }
                    | validators::digests::DigestError::Root { .. }
                    | validators::digests::DigestError::Io { .. }
                    | validators::digests::DigestError::SymlinkEscape { .. }
                    | validators::digests::DigestError::NotRegularFile { .. }
                    | validators::digests::DigestError::InvalidDigest { .. }
                    | validators::digests::DigestError::IncompleteReference
                    | validators::digests::DigestError::ContractIo { .. }
                    | validators::digests::DigestError::ContractJson { .. }
                    | validators::digests::DigestError::UnclassifiedReference,
                ) => Err(StagedCandidateInputError::Invalid),
            }
        }
        Some(_) | None => Err(StagedCandidateInputError::Invalid),
    }
}

fn verify_resolved_tools(root: &Path, lock: &Value) -> Result<(), StagedCandidateInputError> {
    let tools = lock
        .get("resolvedTools")
        .and_then(Value::as_array)
        .ok_or(StagedCandidateInputError::ResolvedToolsShape)?;
    if tools.is_empty() {
        return Ok(());
    }
    let manifest_bytes = fs::read(root.join(TOOLCHAIN_MANIFEST_PATH))
        .map_err(|_| StagedCandidateInputError::ToolManifestRead)?;
    let manifest = ToolchainManifest::from_json(&manifest_bytes)
        .map_err(|_| StagedCandidateInputError::ToolManifest)?;
    match toolchain::verify_lock_resolved_tools(&manifest, tools) {
        Ok(()) => Ok(()),
        Err(
            ToolchainError::ExecutableSubstitution { .. }
            | ToolchainError::LockEntryMismatch { .. }
            | ToolchainError::SourceIdentityMismatch { .. }
            | ToolchainError::VersionMismatch { .. }
            | ToolchainError::MetadataMismatch { .. }
            | ToolchainError::DigestMismatch { .. }
            | ToolchainError::HeaderCheckerMismatch,
        ) => Err(StagedCandidateInputError::ResolvedToolMismatch),
        Err(ToolchainError::MissingTool { .. }) => {
            Err(StagedCandidateInputError::ResolvedToolMissing)
        }
        Err(
            ToolchainError::UnsupportedHost { .. }
            | ToolchainError::ReadinessField { .. }
            | ToolchainError::InvalidAuthority
            | ToolchainError::InvalidNote
            | ToolchainError::InvalidManifest { .. }
            | ToolchainError::DuplicateTool { .. }
            | ToolchainError::UnknownTool { .. }
            | ToolchainError::MissingLibcHeaders
            | ToolchainError::LibcHeadersMismatch
            | ToolchainError::ToolExecution { .. }
            | ToolchainError::ToolExecutionFailed { .. }
            | ToolchainError::Io(_)
            | ToolchainError::Json(_),
        ) => Err(StagedCandidateInputError::ResolvedToolsInvalid),
    }
}

#[derive(Debug)]
enum CandidateReportError {
    LockRead,
    LockJson,
    ExternalLockRead,
    ExternalLockJson,
    StagedInput(StagedCandidateInputError),
    Readiness(ReadinessError),
}

impl CandidateReportError {
    const fn code(&self) -> &'static str {
        match self {
            Self::LockRead => "lock-read",
            Self::LockJson => "lock-json",
            Self::ExternalLockRead => "external-lock-read",
            Self::ExternalLockJson => "external-lock-json",
            Self::StagedInput(error) => error.code(),
            Self::Readiness(ReadinessError::InvalidLock { .. }) => "readiness-lock-shape",
            Self::Readiness(ReadinessError::UnmappedKnownUnknown) => "unmapped-known-unknown",
            Self::Readiness(ReadinessError::ExternalContractReferent(_)) => {
                "external-lock-referent"
            }
        }
    }

    fn blocking(&self) -> Option<ReadinessBlocking> {
        match self {
            Self::StagedInput(error) => error.blocking(),
            Self::LockRead
            | Self::LockJson
            | Self::ExternalLockRead
            | Self::ExternalLockJson
            | Self::Readiness(ReadinessError::InvalidLock { .. })
            | Self::Readiness(ReadinessError::UnmappedKnownUnknown)
            | Self::Readiness(ReadinessError::ExternalContractReferent(_)) => None,
        }
    }
}

#[derive(Debug)]
enum StagedCandidateInputError {
    Policy,
    DigestMismatch {
        field: &'static str,
        path: &'static str,
        upstream_owner: &'static str,
    },
    MissingFile,
    Invalid,
    ResolvedToolsShape,
    ToolManifestRead,
    ToolManifest,
    ResolvedToolMissing,
    ResolvedToolMismatch,
    ResolvedToolsInvalid,
}

impl StagedCandidateInputError {
    const fn code(&self) -> &'static str {
        match self {
            Self::Policy => "staged-input-policy",
            Self::DigestMismatch { .. } => "staged-input-digest-mismatch",
            Self::MissingFile => "staged-input-missing",
            Self::Invalid => "staged-input-invalid",
            Self::ResolvedToolsShape => "resolved-tools-shape",
            Self::ToolManifestRead => "resolved-tool-manifest-read",
            Self::ToolManifest => "resolved-tool-manifest",
            Self::ResolvedToolMissing => "resolved-tool-missing",
            Self::ResolvedToolMismatch => "resolved-tool-mismatch",
            Self::ResolvedToolsInvalid => "resolved-tool-invalid",
        }
    }

    fn blocking(&self) -> Option<ReadinessBlocking> {
        match self {
            Self::DigestMismatch {
                field,
                path,
                upstream_owner,
            } => Some(ReadinessBlocking {
                field_path: format!("measurementPolicy.{field}"),
                kind: oxyflut_qualification::readiness::BlockingKind::DigestMismatch,
                evidence_path: Some((*path).to_owned()),
                referent: None,
                upstream_owner: Some((*upstream_owner).to_owned()),
            }),
            Self::Policy
            | Self::MissingFile
            | Self::Invalid
            | Self::ResolvedToolsShape
            | Self::ToolManifestRead
            | Self::ToolManifest
            | Self::ResolvedToolMissing
            | Self::ResolvedToolMismatch
            | Self::ResolvedToolsInvalid => None,
        }
    }
}

fn candidate_report_error_lines(error: &CandidateReportError) -> Vec<String> {
    let mut lines = vec![format!("lock status: invalid ({})", error.code())];
    if let Some(blocking) = error.blocking() {
        lines.push(blocking_line(&blocking));
    }
    lines
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
