//! Readiness gates and Phase 3B promotion binding validation.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use oxyflut_qualification::hash::Sha256Digest;
use oxyflut_qualification::schema::{SchemaError, SchemaRegistry};
use serde_json::{Map, Value};
use thiserror::Error;

use super::digests::{self, DigestError};
use super::schema::ContractSchemaError;

#[path = "readiness_promotion.rs"]
mod promotion;

const LOCK_PATH: &str = ".constitution/tech-spec/contracts/qualification-lock.json";
const PHASE_PATH: &str = ".constitution/tech-spec/contracts/specification-phase.json";
const PLATFORM_CONTRACTS_PATH: &str = ".constitution/tech-spec/contracts/platform-contracts.json";
const EXTERNAL_CONTRACT_LOCK_PATH: &str =
    ".constitution/tech-spec/contracts/external-contract-lock.json";
const RAW_MEASUREMENT_SCHEMA_PATH: &str =
    ".constitution/tech-spec/data-models/raw-measurement.schema.json";
const LOCK_SCHEMA: &str = "urn:oxyflut:schema:qualification-lock:5";
const PHASE_SCHEMA: &str = "urn:oxyflut:schema:specification-phase:1";
const BASELINE_SCHEMA: &str = "urn:oxyflut:schema:capability-baseline:4";
const PLATFORM_SCHEMA: &str = "urn:oxyflut:schema:platform-contracts:5";
const EXTERNAL_LOCK_SCHEMA: &str = "urn:oxyflut:schema:external-contract-lock:1";

/// The validation state of one readiness gate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum GateStatus {
    /// The committed lock claims the gate and every required input validates.
    Ready,
    /// The lock is valid but the gate remains deliberately open.
    Open(Vec<String>),
}

impl GateStatus {
    /// Returns the remaining content-free known-unknown identifiers for an open gate.
    #[must_use]
    pub(crate) fn remaining_known_unknowns(&self) -> &[String] {
        match self {
            Self::Ready => &[],
            Self::Open(known_unknowns) => known_unknowns,
        }
    }
}

/// The Phase 3B promotion state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PromotionStatus {
    /// The active Phase 3A specification does not claim production promotion.
    NotClaimed,
}

/// The content-free result of validating lock readiness and promotion inputs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ReadinessReport {
    /// Candidate implementation readiness state.
    pub(crate) candidate_implementation: GateStatus,
    /// Measurement readiness state.
    pub(crate) measurement: GateStatus,
    /// Production-promotion resolution state.
    pub(crate) promotion: PromotionStatus,
}

/// Reports why readiness or promotion validation failed.
#[derive(Debug, Error)]
pub(crate) enum ReadinessError {
    /// The local schema registry required for typed artifacts could not be compiled.
    #[error("readiness schema registry failed")]
    SchemaRegistry(#[source] ContractSchemaError),
    /// A typed document violated its declared local schema.
    #[error("readiness typed document failed schema validation")]
    Schema {
        /// The typed document family.
        family: &'static str,
        /// The schema validator failure.
        #[source]
        source: SchemaError,
    },
    /// A local readiness input could not be read.
    #[error("could not read readiness input")]
    Io {
        /// The affected local path.
        path: PathBuf,
        /// The underlying filesystem error.
        #[source]
        source: io::Error,
    },
    /// A local readiness input could not be parsed as JSON.
    #[error("could not parse readiness input")]
    Json {
        /// The affected local path.
        path: PathBuf,
        /// The parser failure.
        #[source]
        source: serde_json::Error,
    },
    /// A repository-relative immutable reference failed confinement or digest validation.
    #[error("immutable readiness reference failed")]
    Digest(#[from] DigestError),
    /// A required readiness relationship was absent, malformed, or inconsistent.
    #[error("readiness invariant failed: {code}")]
    Invariant {
        /// Stable content-free failure code.
        code: &'static str,
    },
    /// A lock claimed readiness while a required input remained unresolved.
    #[error("readiness gate was claimed before inputs resolved")]
    InvalidClaim {
        /// The incorrectly claimed gate.
        gate: &'static str,
    },
    /// A qualification evidence record failed the shared semantic validator.
    #[error("qualification evidence semantic validation failed")]
    Traceability(#[from] super::traceability::TraceabilityError),
    /// A production artifact cannot prove its required lock, candidate, and version binding.
    #[error("artifact cannot prove lock binding: {key}")]
    ArtifactCannotProveBinding {
        /// The Phase 3B promotion artifact key.
        key: &'static str,
    },
}

/// Distinguishes readiness-gate errors from promotion-only errors.
#[derive(Debug, Error)]
pub(crate) enum ReadinessValidationError {
    /// A lock input or readiness gate failed validation.
    #[error("readiness validation failed")]
    Readiness(#[from] ReadinessError),
    /// A Phase 3B promotion artifact failed validation after readiness succeeded.
    #[error("promotion validation failed")]
    Promotion(#[source] ReadinessError),
}

impl ReadinessValidationError {
    /// Returns whether the failure occurred only while resolving Phase 3B promotion artifacts.
    #[must_use]
    pub(crate) fn is_promotion_only(&self) -> bool {
        matches!(self, Self::Promotion(_))
    }

    #[cfg(test)]
    fn into_readiness_error(self) -> ReadinessError {
        match self {
            Self::Readiness(error) | Self::Promotion(error) => error,
        }
    }
}

/// Validates the committed lock, requested immutable bindings, and specification phase.
///
/// # Errors
///
/// Returns a typed readiness or promotion error for an invalid lock claim, a malformed local input, a missing or mismatched immutable reference, or unresolved production promotion.
pub(crate) fn validate_workspace(root: &Path) -> Result<ReadinessReport, ReadinessValidationError> {
    let registry =
        super::schema::compile_workspace(root).map_err(ReadinessError::SchemaRegistry)?;
    let lock_path = root.join(LOCK_PATH);
    let phase_path = root.join(PHASE_PATH);
    let lock = read_json(&lock_path)?;
    let phase = read_json(&phase_path)?;
    validate_schema(&registry, LOCK_SCHEMA, &lock, "qualification-lock")?;
    validate_schema(&registry, PHASE_SCHEMA, &phase, "specification-phase")?;
    validate_documents_with_attribution(root, &lock, &phase, &registry)
}

#[cfg(test)]
fn validate_documents(
    root: &Path,
    lock: &Value,
    phase: &Value,
    registry: &SchemaRegistry,
) -> Result<ReadinessReport, ReadinessError> {
    validate_documents_with_attribution(root, lock, phase, registry)
        .map_err(ReadinessValidationError::into_readiness_error)
}

fn validate_documents_with_attribution(
    root: &Path,
    lock: &Value,
    phase: &Value,
    registry: &SchemaRegistry,
) -> Result<ReadinessReport, ReadinessValidationError> {
    let active_version = string_field(phase, "specificationVersion")?;
    let candidate_issues = candidate_input_issues(root, lock, active_version, registry)?;
    let candidate_claimed = bool_field(lock, "candidateImplementationReady")?;
    if candidate_claimed && !candidate_issues.is_empty() {
        return Err(ReadinessError::InvalidClaim {
            gate: "candidate-implementation",
        }
        .into());
    }
    let candidate_implementation = gate_status(
        candidate_claimed,
        "candidate-implementation-ready-not-claimed",
        candidate_issues,
    );

    let measurement_issues = measurement_input_issues(lock, candidate_claimed)?;
    let measurement_claimed = bool_field(lock, "measurementReady")?;
    if measurement_claimed && !measurement_issues.is_empty() {
        return Err(ReadinessError::InvalidClaim {
            gate: "measurement",
        }
        .into());
    }
    let measurement = gate_status(
        measurement_claimed,
        "measurement-ready-not-claimed",
        measurement_issues,
    );

    let promotion = match string_field(phase, "phase")? {
        "qualification-3a" => PromotionStatus::NotClaimed,
        "production-3b" => {
            if !matches!(measurement, GateStatus::Ready) {
                return Err(invariant("promotion-measurement-readiness").into());
            }
            promotion::resolve(root, lock, phase, registry)
                .map_err(ReadinessValidationError::Promotion)?;
            return Err(ReadinessValidationError::Promotion(invariant(
                "promotion-untyped-artifact-set",
            )));
        }
        _ => return Err(invariant("specification-phase").into()),
    };

    Ok(ReadinessReport {
        candidate_implementation,
        measurement,
        promotion,
    })
}

fn candidate_input_issues(
    root: &Path,
    lock: &Value,
    active_version: &str,
    registry: &SchemaRegistry,
) -> Result<Vec<String>, ReadinessError> {
    let mut issues = string_array_field(lock, "preImplementationKnownUnknowns")?;
    let source_pins = object_field(lock, "sourcePins")?;
    let engine_commit = string_field(object_value(source_pins, "flutterEngine")?, "commit")?;
    let artifacts = object_field(lock, "candidateArtifacts")?;
    for artifact in ["darwin-arm64", "linux-x64", "windows-x64"] {
        let artifact = object_value(artifacts, artifact)?
            .as_object()
            .ok_or_else(|| invariant("candidate-artifact"))?;
        if !artifact
            .get("httpVerified")
            .and_then(Value::as_bool)
            .ok_or_else(|| invariant("candidate-artifact"))?
        {
            issues.push("candidate-artifact-http-verification".to_owned());
        }
        require_equal(
            string_from(artifact, "sourceRevision")?,
            engine_commit,
            "candidate-artifact-source-revision",
        )?;
        require_digest_or_issue(artifact, "sha256", "candidate-artifact-digest", &mut issues)?;
        require_positive_integer_or_issue(
            artifact,
            "sizeBytes",
            "candidate-artifact-size",
            &mut issues,
        )?;
    }

    let environments = object_field(lock, "referenceEnvironments")?;
    for environment in [
        "macos-arm64",
        "windows-x86_64",
        "wayland-linux-x86_64",
        "x11-linux-x86_64",
    ] {
        let environment = object_value(environments, environment)?
            .as_object()
            .ok_or_else(|| invariant("reference-environment"))?;
        for field in ["minimumVersion", "hardwareId", "gpuId", "driverVersion"] {
            require_nonempty_string_or_issue(
                environment,
                field,
                "reference-environment-identity",
                &mut issues,
            )?;
        }
        require_digest_or_issue(
            environment,
            "systemPackageLockDigest",
            "reference-environment-package-lock",
            &mut issues,
        )?;
    }

    let workload = object_field(lock, "workload")?;
    for field in [
        "referenceApplication",
        "scenes",
        "interactionScripts",
        "fonts",
        "assets",
        "windowMatrix",
        "cacheStates",
        "releaseFlags",
    ] {
        require_digest_or_issue(workload, field, "workload-input", &mut issues)?;
    }

    let policy = object_field(lock, "measurementPolicy")?;
    require_bound_digest_or_issue(
        root,
        policy,
        "rawMeasurementSchema",
        RAW_MEASUREMENT_SCHEMA_PATH,
        "raw-measurement-schema",
        &mut issues,
    )?;
    require_bound_digest_or_issue(
        root,
        policy,
        "platformContracts",
        PLATFORM_CONTRACTS_PATH,
        "platform-contracts",
        &mut issues,
    )?;
    validate_external_contract_lock(root, policy, registry, &mut issues)?;
    for field in [
        "sampleValidityRules",
        "scoringAnchors",
        "assessors",
        "fuzzCorpora",
        "securityPatchRehearsal",
    ] {
        require_digest_or_issue(policy, field, "measurement-policy-input", &mut issues)?;
    }
    require_positive_integer_or_issue(policy, "layoutVisitCap", "layout-visit-cap", &mut issues)?;
    validate_capability_baseline(root, policy, active_version, registry, &mut issues)?;
    validate_platform_contracts(root, policy, active_version, registry, &mut issues)?;

    let tools = array_field(lock, "resolvedTools")?;
    if tools.is_empty() {
        issues.push("resolved-tools".to_owned());
    }
    for tool in tools {
        let tool = tool.as_object().ok_or_else(|| invariant("resolved-tool"))?;
        for field in [
            "name",
            "version",
            "sourceIdentity",
            "hostTriple",
            "licenseId",
            "executablePath",
        ] {
            require_nonempty_string_or_issue(tool, field, "resolved-tool", &mut issues)?;
        }
        require_digest_or_issue(tool, "sha256", "resolved-tool-digests", &mut issues)?;
        if let (Some(executable_path), Some(digest)) = (
            nonempty_string(tool, "executablePath")?,
            nonempty_string(tool, "sha256")?,
        ) {
            let _ = digests::verify_reference(root, executable_path, digest)?;
        }
    }

    sort_and_deduplicate(&mut issues);
    Ok(issues)
}

fn measurement_input_issues(
    lock: &Value,
    candidate_claimed: bool,
) -> Result<Vec<String>, ReadinessError> {
    let mut issues = Vec::new();
    if !candidate_claimed {
        issues.push("candidate-implementation-ready-not-claimed".to_owned());
    }
    let source_pins = object_field(lock, "sourcePins")?;
    for field in ["integratedFork", "oxyflutAdapter"] {
        let pin = object_value(source_pins, field)?;
        if string_field(pin, "status")? != "kk" || !sha40_field(pin, "commit")? {
            issues.push("final-candidate-source-identity".to_owned());
        }
    }
    let gating = string_array_field(lock, "gatingKnownUnknowns")?;
    issues.extend(gating);
    sort_and_deduplicate(&mut issues);
    Ok(issues)
}

fn validate_external_contract_lock(
    root: &Path,
    policy: &Map<String, Value>,
    registry: &SchemaRegistry,
    issues: &mut Vec<String>,
) -> Result<(), ReadinessError> {
    let Some(value) = policy.get("externalContractLock") else {
        return fail("external-contract-lock");
    };
    if value.is_null() {
        issues.push("external-contract-lock".to_owned());
        return Ok(());
    }
    let digest = value
        .as_str()
        .ok_or_else(|| invariant("external-contract-lock"))?;
    let verified = digests::verify_reference(root, EXTERNAL_CONTRACT_LOCK_PATH, digest)?;
    let external_lock = read_json(&verified.resolved_path)?;
    validate_schema(
        registry,
        EXTERNAL_LOCK_SCHEMA,
        &external_lock,
        "external-contract-lock",
    )?;
    let contracts = object_field(&external_lock, "contracts")?;
    for contract in contracts.values() {
        let contract = contract
            .as_object()
            .ok_or_else(|| invariant("external-contract"))?;
        match string_from(contract, "epistemicStatus")? {
            "ku-gating" => issues.push("external-contract-known-unknown".to_owned()),
            "kk-locked" => {
                let path = contract
                    .get("localPath")
                    .and_then(Value::as_str)
                    .filter(|path| !path.is_empty())
                    .ok_or_else(|| invariant("external-contract-snapshot"))?;
                let digest = contract
                    .get("sha256")
                    .and_then(Value::as_str)
                    .ok_or_else(|| invariant("external-contract-snapshot"))?;
                if contract
                    .get("verifier")
                    .and_then(Value::as_str)
                    .is_none_or(str::is_empty)
                {
                    return fail("external-contract-verifier");
                }
                let _ = digests::verify_reference(root, path, digest)?;
            }
            _ => return fail("external-contract-status"),
        }
    }
    Ok(())
}

fn validate_capability_baseline(
    root: &Path,
    policy: &Map<String, Value>,
    active_version: &str,
    registry: &SchemaRegistry,
    issues: &mut Vec<String>,
) -> Result<(), ReadinessError> {
    let reference = policy
        .get("capabilityBaseline")
        .ok_or_else(|| invariant("capability-baseline-reference"))?;
    if reference.is_null() {
        issues.push("capability-baseline".to_owned());
        return Ok(());
    }
    let reference = reference
        .as_object()
        .ok_or_else(|| invariant("capability-baseline-reference"))?;
    if string_from(reference, "schemaVersion")? != "4.0.0"
        || string_from(reference, "provenance")? != "approved"
    {
        return fail("capability-baseline-reference");
    }
    let verified = digests::verify_object_reference(root, reference)?;
    let baseline = read_json(&verified.resolved_path)?;
    validate_schema(registry, BASELINE_SCHEMA, &baseline, "capability-baseline")?;
    if string_field(&baseline, "schemaVersion")? != string_from(reference, "schemaVersion")?
        || string_field(&baseline, "specificationVersion")? != active_version
    {
        return fail("capability-baseline-version");
    }
    let provenance = object_field(&baseline, "provenance")?;
    if string_from(provenance, "kind")? != "approved" {
        return fail("capability-baseline-provenance");
    }
    let declared_approval = object_value(reference, "approvalEvidence")?;
    let actual_approval = object_value(provenance, "approvalEvidence")?;
    if !same_reference(declared_approval, actual_approval)? {
        return fail("capability-baseline-approval-reference");
    }
    let _ = digests::verify_value_reference(root, declared_approval)?;
    Ok(())
}

fn validate_platform_contracts(
    root: &Path,
    policy: &Map<String, Value>,
    active_version: &str,
    registry: &SchemaRegistry,
    issues: &mut Vec<String>,
) -> Result<(), ReadinessError> {
    let Some(digest) = policy.get("platformContracts") else {
        return fail("platform-contracts");
    };
    if digest.is_null() {
        return Ok(());
    }
    let digest = digest
        .as_str()
        .ok_or_else(|| invariant("platform-contracts"))?;
    let verified = digests::verify_reference(root, PLATFORM_CONTRACTS_PATH, digest)?;
    let platform = read_json(&verified.resolved_path)?;
    validate_schema(registry, PLATFORM_SCHEMA, &platform, "platform-contracts")?;
    require_equal(
        string_field(&platform, "specificationVersion")?,
        active_version,
        "platform-contracts-specification-version",
    )?;
    validate_platform_value(root, &platform, issues)
}

fn validate_platform_value(
    root: &Path,
    value: &Value,
    issues: &mut Vec<String>,
) -> Result<(), ReadinessError> {
    match value {
        Value::Array(values) => {
            for value in values {
                validate_platform_value(root, value, issues)?;
            }
        }
        Value::Object(values) => {
            if let Some(status) = values.get("status") {
                match status.as_str() {
                    Some("ku-gating") => issues.push("platform-known-unknown".to_owned()),
                    Some("kk") => validate_kk_platform_claim(root, values)?,
                    Some("absent-kk") => {}
                    Some(_) | None => return fail("platform-claim-status"),
                }
            }
            if values
                .get("openQuestions")
                .and_then(Value::as_array)
                .is_some_and(|questions| !questions.is_empty())
            {
                issues.push("platform-open-questions".to_owned());
            }
            for value in values.values() {
                validate_platform_value(root, value, issues)?;
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
    Ok(())
}

fn validate_kk_platform_claim(
    root: &Path,
    claim: &Map<String, Value>,
) -> Result<(), ReadinessError> {
    if let Some(evidence) = claim.get("evidence") {
        let evidence = evidence
            .as_array()
            .ok_or_else(|| invariant("platform-claim-evidence"))?;
        if evidence.is_empty() {
            return fail("platform-claim-evidence");
        }
        for reference in evidence {
            let _ = digests::verify_value_reference(root, reference)?;
        }
        return Ok(());
    }
    if claim.contains_key("path") || claim.contains_key("sha256") {
        let _ = digests::verify_object_reference(root, claim)?;
        return Ok(());
    }
    if contains_nested_status(claim) {
        return Ok(());
    }
    fail("platform-claim-evidence")
}

fn contains_nested_status(value: &Map<String, Value>) -> bool {
    value.values().any(|value| match value {
        Value::Array(values) => values.iter().any(|value| {
            value
                .as_object()
                .is_some_and(|value| value.contains_key("status"))
        }),
        Value::Object(values) => values.contains_key("status"),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => false,
    })
}

fn validate_schema(
    registry: &SchemaRegistry,
    identity: &str,
    value: &Value,
    family: &'static str,
) -> Result<(), ReadinessError> {
    registry
        .validate(identity, value)
        .map_err(|source| ReadinessError::Schema { family, source })
}

fn require_bound_digest_or_issue(
    root: &Path,
    value: &Map<String, Value>,
    field: &str,
    path: &str,
    issue: &str,
    issues: &mut Vec<String>,
) -> Result<(), ReadinessError> {
    let Some(value) = value.get(field) else {
        return fail("required-digest");
    };
    if value.is_null() {
        issues.push(issue.to_owned());
        return Ok(());
    }
    let digest = value.as_str().ok_or_else(|| invariant("required-digest"))?;
    let _ = digests::verify_reference(root, path, digest)?;
    Ok(())
}

fn require_digest_or_issue(
    value: &Map<String, Value>,
    field: &str,
    issue: &str,
    issues: &mut Vec<String>,
) -> Result<(), ReadinessError> {
    let Some(value) = value.get(field) else {
        return fail("required-digest");
    };
    if value.is_null() {
        issues.push(issue.to_owned());
        return Ok(());
    }
    let digest = value.as_str().ok_or_else(|| invariant("required-digest"))?;
    let _: Sha256Digest = digest.parse().map_err(|_| invariant("required-digest"))?;
    Ok(())
}

fn require_nonempty_string_or_issue(
    value: &Map<String, Value>,
    field: &str,
    issue: &str,
    issues: &mut Vec<String>,
) -> Result<(), ReadinessError> {
    if nonempty_string(value, field)?.is_none() {
        issues.push(issue.to_owned());
    }
    Ok(())
}

fn nonempty_string<'value>(
    value: &'value Map<String, Value>,
    field: &str,
) -> Result<Option<&'value str>, ReadinessError> {
    let Some(value) = value.get(field) else {
        return fail("required-string");
    };
    if value.is_null() {
        return Ok(None);
    }
    value
        .as_str()
        .filter(|value| !value.is_empty())
        .map(Some)
        .ok_or_else(|| invariant("required-string"))
}

fn require_positive_integer_or_issue(
    value: &Map<String, Value>,
    field: &str,
    issue: &str,
    issues: &mut Vec<String>,
) -> Result<(), ReadinessError> {
    let Some(value) = value.get(field) else {
        return fail("required-integer");
    };
    if value.is_null() {
        issues.push(issue.to_owned());
        return Ok(());
    }
    if value.as_u64().is_none_or(|integer| integer == 0) {
        return fail("required-integer");
    }
    Ok(())
}

fn gate_status(claimed: bool, not_claimed: &str, mut issues: Vec<String>) -> GateStatus {
    if claimed {
        GateStatus::Ready
    } else {
        issues.push(not_claimed.to_owned());
        sort_and_deduplicate(&mut issues);
        GateStatus::Open(issues)
    }
}

fn sort_and_deduplicate(values: &mut Vec<String>) {
    values.sort_unstable();
    values.dedup();
}

fn same_reference(first: &Value, second: &Value) -> Result<bool, ReadinessError> {
    Ok(
        string_field(first, "path")? == string_field(second, "path")?
            && string_field(first, "sha256")? == string_field(second, "sha256")?,
    )
}

fn sha40_field(value: &Value, field: &str) -> Result<bool, ReadinessError> {
    let Some(value) = value.get(field) else {
        return fail("source-identity");
    };
    Ok(value.as_str().is_some_and(is_sha40))
}

fn is_sha40(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn read_json(path: &Path) -> Result<Value, ReadinessError> {
    let bytes = fs::read(path).map_err(|source| ReadinessError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    serde_json::from_slice(&bytes).map_err(|source| ReadinessError::Json {
        path: path.to_path_buf(),
        source,
    })
}

fn object_field<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a Map<String, Value>, ReadinessError> {
    value
        .get(field)
        .and_then(Value::as_object)
        .ok_or_else(|| invariant("required-object"))
}

fn object_value<'a>(
    value: &'a Map<String, Value>,
    field: &str,
) -> Result<&'a Value, ReadinessError> {
    value.get(field).ok_or_else(|| invariant("required-object"))
}

fn array_field<'a>(value: &'a Value, field: &str) -> Result<&'a Vec<Value>, ReadinessError> {
    value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| invariant("required-array"))
}

fn string_array_field(value: &Value, field: &str) -> Result<Vec<String>, ReadinessError> {
    array_field(value, field)?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| invariant("required-string-array"))
        })
        .collect()
}

fn string_field<'a>(value: &'a Value, field: &str) -> Result<&'a str, ReadinessError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| invariant("required-string"))
}

fn string_from<'a>(value: &'a Map<String, Value>, field: &str) -> Result<&'a str, ReadinessError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| invariant("required-string"))
}

fn bool_field(value: &Value, field: &str) -> Result<bool, ReadinessError> {
    value
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| invariant("required-boolean"))
}

fn require_equal(actual: &str, expected: &str, code: &'static str) -> Result<(), ReadinessError> {
    if actual == expected {
        Ok(())
    } else {
        fail(code)
    }
}

fn fail<T>(code: &'static str) -> Result<T, ReadinessError> {
    Err(invariant(code))
}

const fn invariant(code: &'static str) -> ReadinessError {
    ReadinessError::Invariant { code }
}

#[cfg(test)]
#[path = "readiness_tests.rs"]
mod tests;
