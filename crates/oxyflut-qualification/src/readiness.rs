//! Typed, content-free pre-implementation readiness reporting.
//!
//! This module classifies the candidate-implementation inputs recorded in a schema-validated
//! qualification lock. [`StagedInputRegistry`] reads the single measurement-policy field-to-path
//! table below. The table records fixed schema and contract inputs, conventional staged inputs,
//! and path-less policy fields. `xtask` confines and hashes the applicable files before presenting
//! this report. The report itself is read-only and never infers readiness from a missing error.

use serde_json::{Map, Value};
use thiserror::Error;

const ENVIRONMENTS: &[&str] = &[
    "macos-arm64",
    "windows-x86_64",
    "wayland-linux-x86_64",
    "x11-linux-x86_64",
];
const ENVIRONMENT_FIELDS: &[&str] = &[
    "minimumVersion",
    "hardwareId",
    "gpuId",
    "driverVersion",
    "systemPackageLockDigest",
];
const WORKLOAD_FIELDS: &[&str] = &[
    "referenceApplication",
    "scenes",
    "interactionScripts",
    "fonts",
    "assets",
    "windowMatrix",
    "cacheStates",
    "releaseFlags",
];
const POLICY_FIELDS: &[PolicyField] = &[
    PolicyField {
        name: "rawMeasurementSchema",
        evidence_path: Some(".constitution/tech-spec/data-models/raw-measurement.schema.json"),
        upstream_owner: "OXY-C003",
    },
    PolicyField {
        name: "sampleValidityRules",
        evidence_path: Some("qualification/schemas/sample-validity.schema.json"),
        upstream_owner: "OXY-C003",
    },
    PolicyField {
        name: "capabilityBaseline",
        evidence_path: None,
        upstream_owner: "OXY-C002",
    },
    PolicyField {
        name: "platformContracts",
        evidence_path: Some(".constitution/tech-spec/contracts/platform-contracts.json"),
        upstream_owner: "OXY-C004",
    },
    PolicyField {
        name: "scoringAnchors",
        evidence_path: Some("qualification/staged/scoring-anchors.json"),
        upstream_owner: "OXY-D001",
    },
    PolicyField {
        name: "assessors",
        evidence_path: Some("qualification/staged/assessors.json"),
        upstream_owner: "OXY-D001",
    },
    PolicyField {
        name: "fuzzCorpora",
        evidence_path: Some("qualification/staged/fuzz-corpora.json"),
        upstream_owner: "OXY-D001",
    },
    PolicyField {
        name: "securityPatchRehearsal",
        evidence_path: Some("qualification/staged/security-patch-rehearsal.json"),
        upstream_owner: "OXY-D001",
    },
    PolicyField {
        name: "externalContractLock",
        evidence_path: Some(".constitution/tech-spec/contracts/external-contract-lock.json"),
        upstream_owner: "OXY-C001",
    },
    PolicyField {
        name: "layoutVisitCap",
        evidence_path: None,
        upstream_owner: "OXY-D001",
    },
];
/// Looks up conventional repository paths for measurement-policy fields.
///
/// The qualification lock explicitly types `capabilityBaseline` but doesn't declare its path in
/// this table. `layoutVisitCap` is a scalar and has no file. The remaining path-less digest fields
/// use the conventional staged paths recorded in [`POLICY_FIELDS`] until Stage 3 types them.
pub struct StagedInputRegistry;

impl StagedInputRegistry {
    /// Returns the repository-relative input path when one field has a conventional referent.
    #[must_use]
    pub fn measurement_policy_path(field: &str) -> Option<&'static str> {
        POLICY_FIELDS
            .iter()
            .find(|policy_field| policy_field.name == field)
            .and_then(|policy_field| policy_field.evidence_path)
    }
}

const KNOWN_UNKNOWN_BINDINGS: &[KnownUnknownBinding] = &[
    KnownUnknownBinding {
        known_unknown: "minimum-platform-and-protocol-versions",
        required_field: "referenceEnvironments",
        evidence_path: None,
        upstream_owner: "OXY-C004",
    },
    KnownUnknownBinding {
        known_unknown: "hardware-gpu-driver-and-system-package-locks",
        required_field: "referenceEnvironments",
        evidence_path: None,
        upstream_owner: "OXY-C004",
    },
    KnownUnknownBinding {
        known_unknown: "reference-application-scenes-scripts-fonts-assets-windows-cache-and-flags",
        required_field: "workload",
        evidence_path: None,
        upstream_owner: "OXY-D001",
    },
    KnownUnknownBinding {
        known_unknown: "raw-measurement-and-sample-validity-contracts",
        required_field: "measurementPolicy.sampleValidityRules",
        evidence_path: Some("qualification/schemas/sample-validity.schema.json"),
        upstream_owner: "OXY-C003",
    },
    KnownUnknownBinding {
        known_unknown: "capability-and-platform-baselines",
        required_field: "measurementPolicy.capabilityBaseline",
        evidence_path: None,
        upstream_owner: "OXY-C002,OXY-C004",
    },
    KnownUnknownBinding {
        known_unknown: "independent-presentation-opportunity-sources",
        required_field: "measurementPolicy.platformContracts",
        evidence_path: Some(".constitution/tech-spec/contracts/platform-contracts.json"),
        upstream_owner: "OXY-C004",
    },
    KnownUnknownBinding {
        known_unknown: "complete-ime-editing-geometry-and-accessibility-maps",
        required_field: "measurementPolicy.platformContracts",
        evidence_path: Some(".constitution/tech-spec/contracts/platform-contracts.json"),
        upstream_owner: "OXY-C004",
    },
    KnownUnknownBinding {
        known_unknown: "scoring-anchors-and-two-assessors",
        required_field: "measurementPolicy.scoringAnchors",
        evidence_path: None,
        upstream_owner: "OXY-D001",
    },
    KnownUnknownBinding {
        known_unknown: "fuzz-corpora",
        required_field: "measurementPolicy.fuzzCorpora",
        evidence_path: None,
        upstream_owner: "OXY-D001",
    },
    KnownUnknownBinding {
        known_unknown: "security-patch-rehearsal",
        required_field: "measurementPolicy.securityPatchRehearsal",
        evidence_path: None,
        upstream_owner: "OXY-D001",
    },
    KnownUnknownBinding {
        known_unknown: "layout-visit-cap",
        required_field: "measurementPolicy.layoutVisitCap",
        evidence_path: None,
        upstream_owner: "OXY-D001",
    },
    KnownUnknownBinding {
        known_unknown: "external-distribution-schema-snapshots-and-verifiers",
        required_field: "measurementPolicy.externalContractLock",
        evidence_path: Some(".constitution/tech-spec/contracts/external-contract-lock.json"),
        upstream_owner: "OXY-C001",
    },
    KnownUnknownBinding {
        known_unknown: "resolved-tool-digests",
        required_field: "resolvedTools",
        evidence_path: Some("qualification/tools/native-contract-toolchain.json"),
        upstream_owner: "OXY-A008",
    },
];

/// The readiness gate represented by a [`ReadinessReport`].
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReadinessGate {
    /// The gate that permits candidate implementation against frozen inputs.
    CandidateImplementation,
}

impl ReadinessGate {
    /// Returns the stable command-line identifier for this gate.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CandidateImplementation => "candidate-implementation",
        }
    }
}

/// The state of one readiness gate after all blocking inputs are classified.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReadinessStatus {
    /// Every required input is resolved and the lock already claims the gate.
    Ready,
    /// The lock is valid but one or more required inputs remain unresolved.
    Open,
}

impl ReadinessStatus {
    /// Returns the stable content-free status identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Open => "open",
        }
    }
}

/// The reason that a required readiness field blocks its gate.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BlockingKind {
    /// A required field has no value.
    Missing,
    /// A required field explicitly contains JSON `null`.
    Null,
    /// A named known unknown remains in the lock.
    Ku,
    /// A referenced immutable input has a different digest.
    DigestMismatch,
    /// A field has a syntactically present but unresolved value.
    Unresolved,
}

impl BlockingKind {
    /// Returns the stable content-free blocking-kind identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Null => "null",
            Self::Ku => "ku",
            Self::DigestMismatch => "digest-mismatch",
            Self::Unresolved => "unresolved",
        }
    }
}

/// One content-free input that blocks a readiness gate.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ReadinessBlocking {
    /// The exact dot-separated qualification-lock field path.
    pub field_path: String,
    /// The classified reason that this field blocks the gate.
    pub kind: BlockingKind,
    /// The immutable evidence path when the lock schema declares one.
    pub evidence_path: Option<String>,
    /// The ticket or upstream decision that owns resolution of this field.
    pub upstream_owner: Option<String>,
}

/// A deterministic content-free report for one readiness gate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadinessReport {
    /// The gate whose state is reported.
    pub gate: ReadinessGate,
    /// The resolved readiness state.
    pub status: ReadinessStatus,
    /// Every blocking field in deterministic order.
    pub blocking: Vec<ReadinessBlocking>,
}

/// Reports why a lock cannot be classified into a complete readiness report.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum ReadinessError {
    /// The caller supplied a lock that is not structurally safe to classify.
    #[error("qualification lock is invalid for readiness reporting: {code}")]
    InvalidLock {
        /// The stable content-free structural failure code.
        code: &'static str,
    },
    /// A named KU cannot be attributed to a required field or upstream owner.
    #[error("qualification lock contains an unowned pre-implementation KU")]
    UnmappedKnownUnknown,
}

/// Classifies the candidate-implementation inputs in one schema-validated qualification lock.
///
/// The caller must validate the durable schema and every referenced immutable input before using
/// the result to make an exit-code decision. This function deliberately only reads `lock`, never
/// changes the lock or either readiness flag.
///
/// # Errors
///
/// Returns an error if a field needed to classify the report has an unexpected JSON shape or a KU
/// lacks the required field and upstream-owner attribution.
pub fn candidate_implementation_report(lock: &Value) -> Result<ReadinessReport, ReadinessError> {
    let lock = lock.as_object().ok_or(ReadinessError::InvalidLock {
        code: "lock-object",
    })?;
    let mut blocking = Vec::new();

    match lock.get("candidateImplementationReady") {
        Some(Value::Bool(true)) => {}
        Some(Value::Bool(false)) => push_block(
            &mut blocking,
            "candidateImplementationReady",
            BlockingKind::Unresolved,
            None,
            Some("Stage-3-reconciliation"),
        ),
        Some(_) | None => {
            return Err(ReadinessError::InvalidLock {
                code: "candidate-implementation-ready",
            });
        }
    }

    collect_known_unknowns(lock, &mut blocking)?;
    collect_candidate_artifacts(lock, &mut blocking);
    collect_reference_environments(lock, &mut blocking);
    collect_workload(lock, &mut blocking);
    collect_measurement_policy(lock, &mut blocking);
    collect_resolved_tools(lock, &mut blocking);

    blocking.sort_unstable();
    blocking.dedup();
    let status = if blocking.is_empty() {
        ReadinessStatus::Ready
    } else {
        ReadinessStatus::Open
    };

    Ok(ReadinessReport {
        gate: ReadinessGate::CandidateImplementation,
        status,
        blocking,
    })
}

fn collect_known_unknowns(
    lock: &Map<String, Value>,
    blocking: &mut Vec<ReadinessBlocking>,
) -> Result<(), ReadinessError> {
    let known_unknowns = lock
        .get("preImplementationKnownUnknowns")
        .and_then(Value::as_array)
        .ok_or(ReadinessError::InvalidLock {
            code: "pre-implementation-known-unknowns",
        })?;
    for known_unknown in known_unknowns {
        let known_unknown = known_unknown.as_str().ok_or(ReadinessError::InvalidLock {
            code: "pre-implementation-known-unknown",
        })?;
        let binding = KNOWN_UNKNOWN_BINDINGS
            .iter()
            .find(|binding| binding.known_unknown == known_unknown)
            .ok_or(ReadinessError::UnmappedKnownUnknown)?;
        if !required_field_is_present(lock, binding.required_field) {
            return Err(ReadinessError::InvalidLock {
                code: "ku-required-field",
            });
        }
        let field_path = format!("preImplementationKnownUnknowns.{known_unknown}");
        push_block(
            blocking,
            &field_path,
            BlockingKind::Ku,
            binding.evidence_path,
            Some(binding.upstream_owner),
        );
    }
    Ok(())
}

fn collect_candidate_artifacts(lock: &Map<String, Value>, blocking: &mut Vec<ReadinessBlocking>) {
    let Some(artifacts) = object_member(lock, "candidateArtifacts") else {
        push_block(
            blocking,
            "candidateArtifacts",
            BlockingKind::Missing,
            None,
            Some("OXY-D001"),
        );
        return;
    };
    for artifact in ["darwin-arm64", "linux-x64", "windows-x64"] {
        let Some(artifact_value) = object_member(artifacts, artifact) else {
            push_block(
                blocking,
                &format!("candidateArtifacts.{artifact}"),
                BlockingKind::Missing,
                None,
                Some("OXY-D001"),
            );
            continue;
        };
        let prefix = format!("candidateArtifacts.{artifact}");
        collect_nullable_member(
            blocking,
            artifact_value,
            "sha256",
            &format!("{prefix}.sha256"),
            None,
            "OXY-D001",
        );
        collect_nullable_member(
            blocking,
            artifact_value,
            "sizeBytes",
            &format!("{prefix}.sizeBytes"),
            None,
            "OXY-D001",
        );
        if artifact_value.get("httpVerified") == Some(&Value::Bool(false)) {
            push_block(
                blocking,
                &format!("{prefix}.httpVerified"),
                BlockingKind::Unresolved,
                None,
                Some("OXY-D001"),
            );
        }
    }
}

fn collect_reference_environments(
    lock: &Map<String, Value>,
    blocking: &mut Vec<ReadinessBlocking>,
) {
    let Some(environments) = object_member(lock, "referenceEnvironments") else {
        push_block(
            blocking,
            "referenceEnvironments",
            BlockingKind::Missing,
            None,
            Some("OXY-C004"),
        );
        return;
    };
    for environment in ENVIRONMENTS {
        let Some(environment_value) = object_member(environments, environment) else {
            push_block(
                blocking,
                &format!("referenceEnvironments.{environment}"),
                BlockingKind::Missing,
                None,
                Some("OXY-C004"),
            );
            continue;
        };
        for field in ENVIRONMENT_FIELDS {
            collect_nullable_member(
                blocking,
                environment_value,
                field,
                &format!("referenceEnvironments.{environment}.{field}"),
                None,
                "OXY-C004",
            );
        }
    }
}

fn collect_workload(lock: &Map<String, Value>, blocking: &mut Vec<ReadinessBlocking>) {
    let Some(workload) = object_member(lock, "workload") else {
        push_block(
            blocking,
            "workload",
            BlockingKind::Missing,
            None,
            Some("OXY-D001"),
        );
        return;
    };
    for field in WORKLOAD_FIELDS {
        collect_nullable_member(
            blocking,
            workload,
            field,
            &format!("workload.{field}"),
            None,
            "OXY-D001",
        );
    }
}

fn collect_measurement_policy(lock: &Map<String, Value>, blocking: &mut Vec<ReadinessBlocking>) {
    let Some(policy) = object_member(lock, "measurementPolicy") else {
        push_block(
            blocking,
            "measurementPolicy",
            BlockingKind::Missing,
            None,
            Some("OXY-D001"),
        );
        return;
    };
    for field in POLICY_FIELDS {
        collect_nullable_member(
            blocking,
            policy,
            field.name,
            &format!("measurementPolicy.{}", field.name),
            field.evidence_path,
            field.upstream_owner,
        );
    }
}

fn collect_resolved_tools(lock: &Map<String, Value>, blocking: &mut Vec<ReadinessBlocking>) {
    match lock.get("resolvedTools") {
        Some(Value::Array(tools)) if tools.is_empty() => push_block(
            blocking,
            "resolvedTools",
            BlockingKind::Missing,
            Some("qualification/tools/native-contract-toolchain.json"),
            Some("OXY-A008"),
        ),
        Some(Value::Array(_)) => {}
        Some(Value::Null) => push_block(
            blocking,
            "resolvedTools",
            BlockingKind::Null,
            Some("qualification/tools/native-contract-toolchain.json"),
            Some("OXY-A008"),
        ),
        Some(_) | None => push_block(
            blocking,
            "resolvedTools",
            BlockingKind::Missing,
            Some("qualification/tools/native-contract-toolchain.json"),
            Some("OXY-A008"),
        ),
    }
}

fn required_field_is_present(lock: &Map<String, Value>, field_path: &str) -> bool {
    let mut object = lock;
    let mut segments = field_path.split('.').peekable();
    while let Some(segment) = segments.next() {
        let Some(value) = object.get(segment) else {
            return false;
        };
        if segments.peek().is_none() {
            return true;
        }
        let Some(next_object) = value.as_object() else {
            return false;
        };
        object = next_object;
    }
    false
}

fn object_member<'value>(
    object: &'value Map<String, Value>,
    field: &str,
) -> Option<&'value Map<String, Value>> {
    object.get(field).and_then(Value::as_object)
}

fn collect_nullable_member(
    blocking: &mut Vec<ReadinessBlocking>,
    object: &Map<String, Value>,
    field: &str,
    field_path: &str,
    evidence_path: Option<&str>,
    upstream_owner: &str,
) {
    match object.get(field) {
        Some(Value::Null) => push_block(
            blocking,
            field_path,
            BlockingKind::Null,
            evidence_path,
            Some(upstream_owner),
        ),
        Some(_) => {}
        None => push_block(
            blocking,
            field_path,
            BlockingKind::Missing,
            evidence_path,
            Some(upstream_owner),
        ),
    }
}

fn push_block(
    blocking: &mut Vec<ReadinessBlocking>,
    field_path: &str,
    kind: BlockingKind,
    evidence_path: Option<&str>,
    upstream_owner: Option<&str>,
) {
    blocking.push(ReadinessBlocking {
        field_path: field_path.to_owned(),
        kind,
        evidence_path: evidence_path.map(str::to_owned),
        upstream_owner: upstream_owner.map(str::to_owned),
    });
}

struct PolicyField {
    name: &'static str,
    evidence_path: Option<&'static str>,
    upstream_owner: &'static str,
}

struct KnownUnknownBinding {
    known_unknown: &'static str,
    required_field: &'static str,
    evidence_path: Option<&'static str>,
    upstream_owner: &'static str,
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{
        BlockingKind, ReadinessGate, ReadinessStatus, StagedInputRegistry,
        candidate_implementation_report,
    };

    const COMPLETE: &[u8] =
        include_bytes!("../../../qualification/fixtures/readiness/complete.synthetic.json");
    const CLEARED_WITHOUT_EVIDENCE: &[u8] =
        include_bytes!("../../../qualification/fixtures/readiness/cleared-without-evidence.json");

    #[test]
    fn staged_input_registry_binds_every_pathless_measurement_policy_digest() {
        assert_eq!(
            StagedInputRegistry::measurement_policy_path("scoringAnchors"),
            Some("qualification/staged/scoring-anchors.json")
        );
        assert_eq!(
            StagedInputRegistry::measurement_policy_path("assessors"),
            Some("qualification/staged/assessors.json")
        );
        assert_eq!(
            StagedInputRegistry::measurement_policy_path("fuzzCorpora"),
            Some("qualification/staged/fuzz-corpora.json")
        );
        assert_eq!(
            StagedInputRegistry::measurement_policy_path("securityPatchRehearsal"),
            Some("qualification/staged/security-patch-rehearsal.json")
        );
        assert_eq!(
            StagedInputRegistry::measurement_policy_path("layoutVisitCap"),
            None
        );
    }

    #[test]
    fn complete_synthetic_lock_reports_a_ready_candidate_gate()
    -> Result<(), Box<dyn std::error::Error>> {
        let lock: Value = serde_json::from_slice(COMPLETE)?;
        let report = candidate_implementation_report(&lock)?;

        assert_eq!(report.gate, ReadinessGate::CandidateImplementation);
        assert_eq!(report.status, ReadinessStatus::Ready);
        assert!(report.blocking.is_empty());
        Ok(())
    }

    #[test]
    fn clearing_a_ku_string_without_its_evidence_keeps_the_gate_open()
    -> Result<(), Box<dyn std::error::Error>> {
        let lock: Value = serde_json::from_slice(CLEARED_WITHOUT_EVIDENCE)?;
        let report = candidate_implementation_report(&lock)?;
        let known_unknowns = report
            .blocking
            .iter()
            .filter(|blocking| blocking.kind == BlockingKind::Ku)
            .map(|blocking| {
                blocking
                    .field_path
                    .strip_prefix("preImplementationKnownUnknowns.")
                    .ok_or("KU field path must use the stable lock prefix")
            })
            .collect::<Result<Vec<_>, _>>()?;

        assert_eq!(report.status, ReadinessStatus::Open);
        assert_eq!(
            known_unknowns,
            vec![
                "capability-and-platform-baselines",
                "complete-ime-editing-geometry-and-accessibility-maps",
                "external-distribution-schema-snapshots-and-verifiers",
                "fuzz-corpora",
                "hardware-gpu-driver-and-system-package-locks",
                "independent-presentation-opportunity-sources",
                "layout-visit-cap",
                "minimum-platform-and-protocol-versions",
                "raw-measurement-and-sample-validity-contracts",
                "reference-application-scenes-scripts-fonts-assets-windows-cache-and-flags",
                "scoring-anchors-and-two-assessors",
                "security-patch-rehearsal",
            ]
        );
        assert!(report.blocking.iter().any(|blocking| {
            blocking.field_path == "resolvedTools" && blocking.kind == BlockingKind::Missing
        }));
        Ok(())
    }
}
