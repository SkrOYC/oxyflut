//! Exact upstream sets, physical contract bindings, and immutable qualification evidence.
//!
//! The common candidate contract tests have no Stage 3 file location while candidate implementation is prohibited. Their exact, capability-derived identifiers are therefore validated as a closed deferred set. [`ContractTestResolution`] makes that unresolved physical location explicit for a later candidate implementation ticket.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use oxyflut_qualification::hash::{Sha256Digest, hash_file, hash_reader};
use oxyflut_qualification::identifiers::{
    AbsentEventId, CandidateId, CapabilityId, ConstraintId, ContractTestId, EnvironmentId,
    EventName, GateId, LinkTarget, RepositoryPath, SpecificationVersion,
};
use serde_json::Value;
use thiserror::Error;

use super::registries::{self, RegistryError};

const CAPABILITIES_PATH: &str = ".constitution/prd/capabilities.md";
const CONSTRAINTS_PATH: &str = ".constitution/prd/constraints.md";
const FLOWS_DIRECTORY: &str = ".constitution/architecture/flows";
const SPEC_DIRECTORY: &str = ".constitution/tech-spec";
const TRACEABILITY_PATH: &str = ".constitution/tech-spec/contracts/capability-traceability.json";
const PLATFORM_PATH: &str = ".constitution/tech-spec/contracts/platform-contracts.json";
const LOCK_PATH: &str = ".constitution/tech-spec/contracts/qualification-lock.json";
const PHASE_PATH: &str = ".constitution/tech-spec/contracts/specification-phase.json";
const REGISTRY_PATH: &str = ".constitution/tech-spec/contracts/diagnostic-event-registry.json";
const CAPABILITY_COUNT: usize = 52;
const CONSTRAINT_COUNT: usize = 27;
const PLATFORM_SCHEMA_VERSION: &str = "5.0.0";
const BASELINE_SCHEMA_VERSION: &str = "4.0.0";

/// The status of physical common candidate contract-test resolution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ContractTestResolution {
    /// No Stage 3 document defines a file location for the identifiers.
    DeferredUntilCandidateImplementation,
}

/// A content-free report for the exact-set validation family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TraceabilityRunReport {
    /// Number of P0 capabilities in all exact upstream sets.
    pub(crate) capability_count: usize,
    /// Number of exact PRD constraints.
    pub(crate) constraint_count: usize,
    /// Common contract-test physical-resolution state.
    pub(crate) contract_test_resolution: ContractTestResolution,
    /// Number of pending common contract-test items.
    pub(crate) deferred_contract_tests: usize,
    /// Whether numeric live-node generation comparison remains schema-deferred.
    pub(crate) accessibility_generation_deferred: bool,
}

/// Reports a local traceability invariant failure.
#[derive(Debug, Error)]
pub(crate) enum TraceabilityError {
    /// A local required input could not be read.
    #[error("could not read local traceability input")]
    Io {
        /// The failing local path.
        path: PathBuf,
        /// The underlying I/O failure.
        #[source]
        source: io::Error,
    },
    /// A local required input could not be parsed as JSON.
    #[error("could not parse local traceability input")]
    Json {
        /// The failing local path.
        path: PathBuf,
        /// The parser failure.
        #[source]
        source: serde_json::Error,
    },
    /// A local immutable input could not be hashed.
    #[error("could not hash local traceability input")]
    Hash {
        /// The failing local path.
        path: PathBuf,
        /// The underlying I/O failure.
        #[source]
        source: io::Error,
    },
    /// The diagnostic registry violated its own semantic contract.
    #[error("diagnostic registry validation failed")]
    Registry(#[source] RegistryError),
    /// A stable, content-free semantic invariant failed.
    #[error("traceability invariant failed: {code}")]
    Invariant {
        /// Stable failure code.
        code: &'static str,
    },
}

impl TraceabilityError {
    #[cfg(test)]
    fn code(&self) -> Option<&'static str> {
        match self {
            Self::Invariant { code } => Some(code),
            Self::Io { .. } | Self::Json { .. } | Self::Hash { .. } | Self::Registry(_) => None,
        }
    }
}

/// Validates the committed upstream sets, active specification, physical contracts, and platform baseline.
///
/// # Errors
///
/// Returns an error for a local input failure or any exact-set, identifier, symbol, version, or immutable-reference mismatch.
pub(crate) fn validate_workspace(root: &Path) -> Result<TraceabilityRunReport, TraceabilityError> {
    // These validators are ready for the evidence families that first produce their durable inputs in later tickets.
    let _ = (
        validate_artifact_manifest,
        validate_raw_measurement,
        validate_qualification_evidence,
    );
    let active = active_specification(root)?;
    let capabilities = prd_capabilities(root)?;
    let constraints = prd_constraints(root)?;
    let flows = architecture_flows(root)?;
    validate_capability_sets(&capabilities, &flows, None)?;
    validate_constraint_set(&constraints)?;

    let traceability = read_json(&root.join(TRACEABILITY_PATH))?;
    let traceability_ids =
        validate_traceability(root, &traceability, &active, &capabilities, &flows)?;
    validate_capability_sets(&capabilities, &flows, Some(&traceability_ids))?;

    let registry = read_json(&root.join(REGISTRY_PATH))?;
    registries::validate_registry(&registry).map_err(TraceabilityError::Registry)?;
    let platform = read_json(&root.join(PLATFORM_PATH))?;
    validate_platform_baseline(root, &platform, &active, &registry)?;
    let lock = read_json(&root.join(LOCK_PATH))?;
    validate_lock_baseline(root, &lock, &active, &capabilities)?;

    Ok(TraceabilityRunReport {
        capability_count: CAPABILITY_COUNT,
        constraint_count: CONSTRAINT_COUNT,
        contract_test_resolution: ContractTestResolution::DeferredUntilCandidateImplementation,
        deferred_contract_tests: CAPABILITY_COUNT,
        accessibility_generation_deferred: true,
    })
}

/// Validates one capability baseline, including exact P0 keys and approval provenance.
///
/// # Errors
///
/// Returns an error when the baseline has stale versioning, a changed exact set, synthetic approval, or unresolved approval evidence.
pub(crate) fn validate_capability_baseline(
    root: &Path,
    baseline: &Value,
    active: &SpecificationVersion,
    capabilities: &BTreeSet<CapabilityId>,
) -> Result<(), TraceabilityError> {
    if &specification_field(baseline)? != active {
        return fail("baseline-specification-version");
    }
    let baseline_ids = capability_keys(
        object_field(baseline, "capabilities")?,
        "baseline-capability-set",
    )?;
    if &baseline_ids != capabilities {
        return fail("baseline-capability-set");
    }
    let provenance = object_field(baseline, "provenance")?;
    match string_from(provenance, "kind")? {
        "synthetic"
            if provenance
                .get("approvalEvidence")
                .is_some_and(Value::is_null) =>
        {
            Ok(())
        }
        "approved" => resolve_evidence_object(
            root,
            provenance
                .get("approvalEvidence")
                .and_then(Value::as_object)
                .ok_or_else(|| error("baseline-approval-evidence"))?,
        ),
        "synthetic" => fail("baseline-approval-evidence"),
        _ => fail("baseline-provenance"),
    }
}

/// Validates artifact paths and the declared link-content semantics without dereferencing links.
///
/// # Errors
///
/// Returns an error for duplicate or escaping paths, a regular-file target, or an inconsistent hardlink or symlink digest.
pub(crate) fn validate_artifact_manifest(manifest: &Value) -> Result<(), TraceabilityError> {
    let mut entries = BTreeMap::<RepositoryPath, &Value>::new();
    for entry in array_field(manifest, "files")? {
        let path = RepositoryPath::parse(string_field(entry, "path")?)
            .map_err(|_| error("artifact-path"))?;
        if entries.insert(path, entry).is_some() {
            return fail("artifact-path-duplicate");
        }
    }
    let mut link_targets = BTreeSet::new();
    for entry in entries.values() {
        match string_field(entry, "kind")? {
            "file" if entry.get("linkTarget").is_none() => {}
            "file" => return fail("artifact-file-link-target"),
            "hardlink" => {
                let target = link_target(entry)?;
                if !link_targets.insert(target.clone()) {
                    return fail("artifact-link-target-duplicate");
                }
                let target_entry = entries
                    .get(target.as_path())
                    .ok_or_else(|| error("artifact-hardlink-target"))?;
                if string_field(target_entry, "kind")? != "file"
                    || integer_field(target_entry, "size")? != integer_field(entry, "size")?
                    || string_field(target_entry, "sha256")? != string_field(entry, "sha256")?
                {
                    return fail("artifact-hardlink-content");
                }
            }
            "symlink" => {
                let target = link_target(entry)?;
                if !link_targets.insert(target.clone()) {
                    return fail("artifact-link-target-duplicate");
                }
                let bytes = target.as_path().as_str().as_bytes();
                let digest = hash_reader(bytes).map_err(|source| TraceabilityError::Hash {
                    path: PathBuf::from("artifact-link-target"),
                    source,
                })?;
                if digest != digest_field(entry, "sha256", "artifact-symlink-digest")?
                    || integer_field(entry, "size")?
                        != u64::try_from(bytes.len()).map_err(|_| error("artifact-symlink-size"))?
                {
                    return fail("artifact-symlink-content");
                }
            }
            _ => return fail("artifact-kind"),
        }
    }
    Ok(())
}

/// Validates raw-measurement constraint identity and unique `(constraintId, launch, ordinal)` keys.
///
/// # Errors
///
/// Returns an error when a sample names an unknown constraint or duplicates its composite key.
pub(crate) fn validate_raw_measurement(
    measurement: &Value,
    constraints: &BTreeSet<ConstraintId>,
) -> Result<(), TraceabilityError> {
    let mut seen = BTreeSet::new();
    for sample in array_field(measurement, "samples")? {
        let constraint = ConstraintId::parse(string_field(sample, "constraintId")?)
            .map_err(|_| error("raw-sample-constraint"))?;
        if !constraints.contains(&constraint) {
            return fail("raw-sample-constraint");
        }
        if !seen.insert((
            constraint,
            integer_field(sample, "launch")?,
            integer_field(sample, "ordinal")?,
        )) {
            return fail("raw-sample-key-duplicate");
        }
    }
    Ok(())
}

/// Validates a qualification evidence record's exact gate sets and typed absence proof.
///
/// # Errors
///
/// Returns an error when an eligible gate, evidence path or digest, candidate, environment, or absent-event binding is invalid.
pub(crate) fn validate_qualification_evidence(
    root: &Path,
    record: &Value,
    active: &SpecificationVersion,
    capabilities: &BTreeSet<CapabilityId>,
    constraints: &BTreeSet<ConstraintId>,
    registry: &Value,
    active_platform_path: &RepositoryPath,
) -> Result<(), TraceabilityError> {
    let candidate = string_field(record, "candidate")?
        .parse::<CandidateId>()
        .map_err(|_| error("candidate-identifier"))?;
    let eligible = string_field(record, "eligibility")? == "eligible";
    let environments = object_field(record, "environmentResults")?;
    let environment_ids = environment_keys(environments)?;
    if environment_ids.len() != 4
        || EnvironmentId::tier_one()
            .iter()
            .any(|item| !environment_ids.contains(item))
    {
        return fail("evidence-environment-set");
    }
    for (name, results) in environments {
        let environment = name
            .parse::<EnvironmentId>()
            .map_err(|_| error("environment-identifier"))?;
        let results = results
            .as_object()
            .ok_or_else(|| error("evidence-capability-set"))?;
        if &capability_keys(results, "evidence-capability-set")? != capabilities {
            return fail("evidence-capability-set");
        }
        for (id, result) in results {
            validate_gate(
                root,
                result,
                &GateId::parse(id).map_err(|_| error("evidence-gate"))?,
                candidate,
                environment,
                false,
                eligible,
                active,
                registry,
                active_platform_path,
            )?;
        }
    }
    let aggregate = object_field(record, "constraintResults")?;
    if &constraint_keys(aggregate)? != constraints {
        return fail("evidence-constraint-set");
    }
    for (id, result) in aggregate {
        validate_gate(
            root,
            result,
            &GateId::parse(id).map_err(|_| error("evidence-gate"))?,
            candidate,
            EnvironmentId::Macos,
            true,
            eligible,
            active,
            registry,
            active_platform_path,
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn validate_gate(
    root: &Path,
    result: &Value,
    gate: &GateId,
    candidate: CandidateId,
    environment: EnvironmentId,
    aggregate_constraint: bool,
    eligible: bool,
    active: &SpecificationVersion,
    registry: &Value,
    active_platform_path: &RepositoryPath,
) -> Result<(), TraceabilityError> {
    let status = string_field(result, "status")?;
    for evidence in array_field(result, "evidence")? {
        resolve_evidence(root, evidence)?;
    }
    if eligible && !matches!(status, "pass" | "not-applicable-kk") {
        return fail("eligible-gate-status");
    }
    match status {
        "pass" if result.get("notApplicable").is_some_and(Value::is_null) => Ok(()),
        "pass" => fail("pass-absence-binding"),
        "not-applicable-kk" => validate_absence_binding(
            root,
            result
                .get("notApplicable")
                .and_then(Value::as_object)
                .ok_or_else(|| error("not-applicable-binding"))?,
            gate,
            candidate,
            environment,
            aggregate_constraint,
            active,
            registry,
            active_platform_path,
        ),
        "fail" | "gating-ku" => Ok(()),
        _ => fail("gate-status"),
    }
}

#[allow(clippy::too_many_arguments)]
fn validate_absence_binding(
    root: &Path,
    binding: &serde_json::Map<String, Value>,
    gate: &GateId,
    candidate: CandidateId,
    environment: EnvironmentId,
    aggregate_constraint: bool,
    active: &SpecificationVersion,
    registry: &Value,
    active_platform_path: &RepositoryPath,
) -> Result<(), TraceabilityError> {
    let reference = binding
        .get("platformBaseline")
        .and_then(Value::as_object)
        .ok_or_else(|| error("not-applicable-binding"))?;
    let path = RepositoryPath::parse(string_from(reference, "path")?)
        .map_err(|_| error("not-applicable-baseline-path"))?;
    if &path != active_platform_path {
        return fail("not-applicable-baseline-path");
    }
    let baseline_path = root.join(path.as_str());
    if hash_file(&baseline_path).map_err(|source| TraceabilityError::Hash {
        path: baseline_path.clone(),
        source,
    })? != digest_from(
        string_from(reference, "sha256")?,
        "not-applicable-baseline-digest",
    )? {
        return fail("not-applicable-baseline-digest");
    }
    if string_from(reference, "schemaVersion")? != PLATFORM_SCHEMA_VERSION {
        return fail("not-applicable-baseline-schema-version");
    }
    if &SpecificationVersion::parse(string_from(reference, "specificationVersion")?)
        .map_err(|_| error("not-applicable-baseline-specification-version"))?
        != active
    {
        return fail("not-applicable-baseline-specification-version");
    }
    let baseline = read_json(&baseline_path)?;
    if &specification_field(&baseline)? != active {
        return fail("not-applicable-baseline-specification-version");
    }
    let absent_id = AbsentEventId::parse(string_from(binding, "absentEventId")?)
        .map_err(|_| error("absent-event-id"))?;
    let entry = array_field(&baseline, "absentEvents")?
        .iter()
        .find(|entry| entry.get("id").and_then(Value::as_str) == Some(absent_id.as_str()))
        .ok_or_else(|| error("absent-event-unknown"))?;
    validate_absent_event(
        root,
        entry,
        gate,
        candidate,
        environment,
        aggregate_constraint,
        registry,
    )
}

fn validate_absent_event(
    root: &Path,
    entry: &Value,
    gate: &GateId,
    candidate: CandidateId,
    environment: EnvironmentId,
    aggregate_constraint: bool,
    registry: &Value,
) -> Result<(), TraceabilityError> {
    if GateId::parse(string_field(entry, "gateId")?).map_err(|_| error("absent-event-gate"))?
        != *gate
    {
        return fail("absent-event-gate");
    }
    let event = EventName::parse(string_field(entry, "eventId")?)
        .map_err(|_| error("absent-event-event"))?;
    if !object_field(registry, "events")?.contains_key(event.as_str()) {
        return fail("absent-event-event");
    }
    if string_field(entry, "status")? != "absent-kk" {
        return fail("absent-event-status");
    }
    if !string_array(entry, "candidates")?.contains(&candidate.as_str()) {
        return fail("absent-event-candidate");
    }
    let environments = string_array(entry, "environments")?;
    if aggregate_constraint {
        if EnvironmentId::tier_one()
            .iter()
            .any(|item| !environments.contains(&item.as_str()))
        {
            return fail("absent-event-aggregate-environments");
        }
    } else if !environments.contains(&environment.as_str()) {
        return fail("absent-event-environment");
    }
    let evidence = array_field(entry, "evidence")?;
    if evidence.is_empty() {
        return fail("absent-event-evidence");
    }
    for item in evidence {
        resolve_evidence(root, item)?;
    }
    Ok(())
}

fn validate_platform_baseline(
    root: &Path,
    baseline: &Value,
    active: &SpecificationVersion,
    registry: &Value,
) -> Result<(), TraceabilityError> {
    if &specification_field(baseline)? != active {
        return fail("platform-specification-version");
    }
    let environments = object_field(baseline, "environments")?;
    let keys = environment_keys(environments)?;
    if keys.len() != 4
        || EnvironmentId::tier_one()
            .iter()
            .any(|item| !keys.contains(item))
    {
        return fail("platform-environment-set");
    }
    for (name, environment) in environments {
        let environment_id = name
            .parse::<EnvironmentId>()
            .map_err(|_| error("environment-identifier"))?;
        validate_platform_environment(root, environment, environment_id)?;
    }
    let mut absent_ids = BTreeSet::new();
    for entry in array_field(baseline, "absentEvents")? {
        let id = AbsentEventId::parse(string_field(entry, "id")?)
            .map_err(|_| error("absent-event-id"))?;
        if !absent_ids.insert(id) {
            return fail("absent-event-duplicate");
        }
        GateId::parse(string_field(entry, "gateId")?).map_err(|_| error("absent-event-gate"))?;
        let event = EventName::parse(string_field(entry, "eventId")?)
            .map_err(|_| error("absent-event-event"))?;
        if !object_field(registry, "events")?.contains_key(event.as_str()) {
            return fail("absent-event-event");
        }
        if string_field(entry, "status")? != "absent-kk" {
            return fail("absent-event-status");
        }
        for candidate in string_array(entry, "candidates")? {
            let _ = candidate
                .parse::<CandidateId>()
                .map_err(|_| error("candidate-identifier"))?;
        }
        for environment in string_array(entry, "environments")? {
            let _ = environment
                .parse::<EnvironmentId>()
                .map_err(|_| error("environment-identifier"))?;
        }
        let evidence = array_field(entry, "evidence")?;
        if evidence.is_empty() {
            return fail("absent-event-evidence");
        }
        for item in evidence {
            resolve_evidence(root, item)?;
        }
    }
    Ok(())
}

fn validate_platform_environment(
    root: &Path,
    environment: &Value,
    environment_id: EnvironmentId,
) -> Result<(), TraceabilityError> {
    validate_kk_claim(root, object_value(environment, "minimumVersion")?)?;
    for protocol in array_field(environment, "protocols")? {
        validate_kk_claim(root, protocol)?;
    }
    validate_kk_claim(root, object_value(environment, "ime")?)?;
    validate_kk_claim(root, object_value(environment, "timing")?)?;
    for candidate in CandidateId::all() {
        validate_kk_claim(
            root,
            object_value(
                object_value(environment, "allocations")?,
                candidate.as_str(),
            )?,
        )?;
        validate_accessibility_reference(
            root,
            object_value(
                object_value(environment, "accessibilityMaps")?,
                candidate.as_str(),
            )?,
            environment_id,
            candidate,
        )?;
    }
    validate_contract_reference(root, object_value(environment, "recoveryBaseline")?)
}

fn validate_kk_claim(root: &Path, claim: &Value) -> Result<(), TraceabilityError> {
    match string_field(claim, "status")? {
        "ku-gating" => Ok(()),
        "kk" => {
            let evidence = array_field(claim, "evidence")?;
            if evidence.is_empty() {
                return fail("kk-evidence-missing");
            }
            for item in evidence {
                resolve_evidence(root, item)?;
            }
            Ok(())
        }
        _ => fail("platform-claim-status"),
    }
}

fn validate_contract_reference(root: &Path, reference: &Value) -> Result<(), TraceabilityError> {
    match string_field(reference, "status")? {
        "ku-gating" => Ok(()),
        "kk" => resolve_evidence_fields(root, reference),
        _ => fail("platform-claim-status"),
    }
}

fn validate_accessibility_reference(
    root: &Path,
    reference: &Value,
    environment: EnvironmentId,
    candidate: CandidateId,
) -> Result<(), TraceabilityError> {
    if string_field(reference, "status")? != "kk" {
        return Ok(());
    }
    let path = RepositoryPath::parse(string_field(reference, "path")?)
        .map_err(|_| error("accessibility-path"))?;
    let map_path = root.join(path.as_str());
    if hash_file(&map_path).map_err(|source| TraceabilityError::Hash {
        path: map_path.clone(),
        source,
    })? != digest_field(reference, "sha256", "accessibility-digest")?
    {
        return fail("accessibility-digest");
    }
    let map = read_json(&map_path)?;
    if string_field(&map, "environment")? != environment.as_str()
        || string_field(&map, "candidate")? != candidate.as_str()
    {
        return fail("accessibility-identity");
    }
    if string_field(&map, "epistemicStatus")? != "kk-complete" {
        return fail("accessibility-status");
    }
    let forward = object_field(&map, "forward")?;
    for category in REQUIRED_ACCESSIBILITY_CATEGORIES {
        if string_field(
            forward
                .get(*category)
                .ok_or_else(|| error("accessibility-category"))?,
            "status",
        )? != "kk"
        {
            return fail("accessibility-ku");
        }
    }
    for action in array_field(&map, "reverseActions")? {
        if string_field(action, "status")? != "kk" {
            return fail("accessibility-ku");
        }
        if string_field(action, "textIndexUnit")? != "none" {
            let binding = string_field(action, "textLayoutBinding")?;
            if binding.is_empty()
                || !binding.contains("TextLayoutGeneration")
                || !text_layout_symbols_resolve(root)?
            {
                return fail("accessibility-text-layout-generation");
            }
        }
    }
    for item in array_field(&map, "evidence")? {
        resolve_evidence(root, item)?;
    }
    Ok(())
}

fn text_layout_symbols_resolve(root: &Path) -> Result<bool, TraceabilityError> {
    let substrate = read_text(
        &root
            .join(SPEC_DIRECTORY)
            .join("contracts/oxyflut-substrate.rs"),
    )?;
    let public = read_text(
        &root
            .join(SPEC_DIRECTORY)
            .join("contracts/oxyflut-public.rs"),
    )?;
    Ok(word(&substrate, "TextLayoutGeneration") && word(&public, "TextLayoutId"))
}

fn validate_traceability(
    root: &Path,
    traceability: &Value,
    active: &SpecificationVersion,
    capabilities: &BTreeSet<CapabilityId>,
    flows: &BTreeSet<CapabilityId>,
) -> Result<BTreeSet<CapabilityId>, TraceabilityError> {
    if &specification_field(traceability)? != active {
        return fail("traceability-specification-version");
    }
    let mut ids = BTreeSet::new();
    let mut tests = BTreeSet::new();
    for mapping in array_field(traceability, "mappings")? {
        let capability = CapabilityId::parse(string_field(mapping, "capabilityId")?)
            .map_err(|_| error("traceability-capability-id"))?;
        if !ids.insert(capability.clone()) {
            return fail("traceability-capability-duplicate");
        }
        let expected_flow = format!(
            ".constitution/architecture/flows/{}.md",
            capability.as_str().to_ascii_lowercase()
        );
        let flow = RepositoryPath::parse(string_field(mapping, "architectureFlow")?)
            .map_err(|_| error("traceability-flow-path"))?;
        if flow.as_str() != expected_flow || !root.join(flow.as_str()).is_file() {
            return fail("traceability-flow-path");
        }
        validate_bindings(root, mapping, &capability)?;
        let contract_tests = array_field(mapping, "contractTests")?;
        if contract_tests.len() != 1 {
            return fail("contract-test-cardinality");
        }
        let test = ContractTestId::parse(
            contract_tests[0]
                .as_str()
                .ok_or_else(|| error("contract-test-identifier"))?,
        )
        .map_err(|_| error("contract-test-identifier"))?;
        if test.as_str()
            != format!(
                "contract::{}",
                capability.as_str().to_ascii_lowercase().replace('-', "_")
            )
        {
            return fail("contract-test-derivation");
        }
        if !tests.insert(test) {
            return fail("contract-test-duplicate");
        }
    }
    if ids.len() != CAPABILITY_COUNT
        || tests.len() != CAPABILITY_COUNT
        || &ids != capabilities
        || &ids != flows
    {
        return fail("capability-set");
    }
    validate_required_symbol_edges(traceability)?;
    Ok(ids)
}

fn validate_bindings(
    root: &Path,
    mapping: &Value,
    capability: &CapabilityId,
) -> Result<(), TraceabilityError> {
    let mut contracts = BTreeSet::new();
    for binding in array_field(mapping, "bindings")? {
        let contract = RepositoryPath::parse(string_field(binding, "contract")?)
            .map_err(|_| error("contract-path"))?;
        if !contracts.insert(contract.clone()) {
            return fail("contract-binding-duplicate");
        }
        let path = root.join(SPEC_DIRECTORY).join(contract.as_str());
        let source = read_text(&path)?;
        let symbols = array_field(binding, "symbols")?;
        if symbols.is_empty() {
            return fail("contract-symbol-cardinality");
        }
        for symbol in symbols {
            let symbol = symbol.as_str().ok_or_else(|| error("contract-symbol"))?;
            if !symbol_resolves(&path, &source, symbol)? {
                return fail(if symbol.starts_with("#/") {
                    "contract-symbol-json"
                } else {
                    "contract-symbol-text"
                });
            }
        }
    }
    let expected =
        edge_matrix(capability.as_str()).ok_or_else(|| error("edge-matrix-capability"))?;
    if contracts
        .iter()
        .map(RepositoryPath::as_str)
        .collect::<BTreeSet<_>>()
        != expected.iter().copied().collect()
    {
        return fail("capability-contract-edge");
    }
    Ok(())
}

fn symbol_resolves(path: &Path, source: &str, symbol: &str) -> Result<bool, TraceabilityError> {
    if let Some(pointer) = symbol.strip_prefix('#') {
        return serde_json::from_str::<Value>(source)
            .map_err(|source| TraceabilityError::Json {
                path: path.to_path_buf(),
                source,
            })
            .map(|value| value.pointer(pointer).is_some());
    }
    if let Some((owner, member)) = symbol.split_once("::").or_else(|| symbol.split_once('.')) {
        return Ok(word(source, owner) && word(source, member));
    }
    Ok(word(source, symbol))
}

fn validate_required_symbol_edges(traceability: &Value) -> Result<(), TraceabilityError> {
    for (capability, contract, symbol) in REQUIRED_SYMBOL_EDGES {
        let mapping = array_field(traceability, "mappings")?
            .iter()
            .find(|mapping| {
                mapping.get("capabilityId").and_then(Value::as_str) == Some(*capability)
            })
            .ok_or_else(|| error("edge-matrix-capability"))?;
        let has_symbol = array_field(mapping, "bindings")?.iter().any(|binding| {
            binding.get("contract").and_then(Value::as_str) == Some(*contract)
                && binding
                    .get("symbols")
                    .and_then(Value::as_array)
                    .is_some_and(|symbols| {
                        symbols.iter().any(|item| item.as_str() == Some(*symbol))
                    })
        });
        if !has_symbol {
            return fail("capability-symbol-edge");
        }
    }
    Ok(())
}

fn validate_lock_baseline(
    root: &Path,
    lock: &Value,
    active: &SpecificationVersion,
    capabilities: &BTreeSet<CapabilityId>,
) -> Result<(), TraceabilityError> {
    let reference = object_field(lock, "measurementPolicy")?.get("capabilityBaseline");
    let Some(reference) = reference else {
        return fail("lock-baseline-reference");
    };
    if reference.is_null() {
        return Ok(());
    }
    let reference = reference
        .as_object()
        .ok_or_else(|| error("lock-baseline-reference"))?;
    if string_from(reference, "schemaVersion")? != BASELINE_SCHEMA_VERSION
        || string_from(reference, "provenance")? != "approved"
    {
        return fail("lock-baseline-reference");
    }
    let path = RepositoryPath::parse(string_from(reference, "path")?)
        .map_err(|_| error("lock-baseline-path"))?;
    let baseline_path = root.join(path.as_str());
    if hash_file(&baseline_path).map_err(|source| TraceabilityError::Hash {
        path: baseline_path.clone(),
        source,
    })? != digest_from(string_from(reference, "sha256")?, "lock-baseline-digest")?
    {
        return fail("lock-baseline-digest");
    }
    let baseline = read_json(&baseline_path)?;
    validate_capability_baseline(root, &baseline, active, capabilities)?;
    if string_field(object_value(&baseline, "provenance")?, "kind")? != "approved" {
        return fail("lock-baseline-provenance");
    }
    let declared = reference
        .get("approvalEvidence")
        .and_then(Value::as_object)
        .ok_or_else(|| error("lock-baseline-approval-evidence"))?;
    let actual = object_value(object_value(&baseline, "provenance")?, "approvalEvidence")?;
    if string_from(declared, "path")? != string_field(actual, "path")?
        || string_from(declared, "sha256")? != string_field(actual, "sha256")?
    {
        return fail("lock-baseline-approval-evidence");
    }
    resolve_evidence_object(root, declared)
}

fn validate_capability_sets(
    capabilities: &BTreeSet<CapabilityId>,
    flows: &BTreeSet<CapabilityId>,
    traceability: Option<&BTreeSet<CapabilityId>>,
) -> Result<(), TraceabilityError> {
    if capabilities.len() != CAPABILITY_COUNT
        || flows.len() != CAPABILITY_COUNT
        || capabilities != flows
        || traceability
            .is_some_and(|items| items.len() != CAPABILITY_COUNT || items != capabilities)
    {
        return fail("capability-set");
    }
    Ok(())
}

fn validate_constraint_set(constraints: &BTreeSet<ConstraintId>) -> Result<(), TraceabilityError> {
    let expected = EXPECTED_CONSTRAINTS
        .iter()
        .map(|item| ConstraintId::parse(item).map_err(|_| error("constraint-set")))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if constraints.len() != CONSTRAINT_COUNT || constraints != &expected {
        return fail("constraint-set");
    }
    Ok(())
}

fn active_specification(root: &Path) -> Result<SpecificationVersion, TraceabilityError> {
    specification_field(&read_json(&root.join(PHASE_PATH))?)
}

fn prd_capabilities(root: &Path) -> Result<BTreeSet<CapabilityId>, TraceabilityError> {
    let mut result = BTreeSet::new();
    for line in read_text(&root.join(CAPABILITIES_PATH))?
        .lines()
        .filter(|line| line.starts_with("| P0 |"))
    {
        let id = line
            .split('|')
            .nth(2)
            .map(str::trim)
            .ok_or_else(|| error("prd-capability-table"))?;
        if !result.insert(CapabilityId::parse(id).map_err(|_| error("prd-capability-id"))?) {
            return fail("prd-capability-duplicate");
        }
    }
    Ok(result)
}

fn prd_constraints(root: &Path) -> Result<BTreeSet<ConstraintId>, TraceabilityError> {
    let mut result = BTreeSet::new();
    for line in read_text(&root.join(CONSTRAINTS_PATH))?
        .lines()
        .filter(|line| line.starts_with("| CON-"))
    {
        let id = line
            .split('|')
            .nth(1)
            .map(str::trim)
            .ok_or_else(|| error("prd-constraint-table"))?;
        if !result.insert(ConstraintId::parse(id).map_err(|_| error("prd-constraint-id"))?) {
            return fail("prd-constraint-duplicate");
        }
    }
    Ok(result)
}

fn architecture_flows(root: &Path) -> Result<BTreeSet<CapabilityId>, TraceabilityError> {
    let directory = root.join(FLOWS_DIRECTORY);
    let mut result = BTreeSet::new();
    for entry in fs::read_dir(&directory).map_err(|source| TraceabilityError::Io {
        path: directory.clone(),
        source,
    })? {
        let entry = entry.map_err(|source| TraceabilityError::Io {
            path: directory.clone(),
            source,
        })?;
        let path = entry.path();
        if path.extension().and_then(|item| item.to_str()) == Some("md") {
            let stem = path
                .file_stem()
                .and_then(|item| item.to_str())
                .ok_or_else(|| error("architecture-flow-name"))?;
            if !result.insert(
                CapabilityId::parse(&stem.to_ascii_uppercase())
                    .map_err(|_| error("architecture-flow-name"))?,
            ) {
                return fail("architecture-flow-duplicate");
            }
        }
    }
    Ok(result)
}

fn resolve_evidence(root: &Path, value: &Value) -> Result<(), TraceabilityError> {
    resolve_evidence_parts(
        root,
        string_field(value, "path")?,
        string_field(value, "sha256")?,
    )
}

fn resolve_evidence_fields(root: &Path, value: &Value) -> Result<(), TraceabilityError> {
    resolve_evidence_parts(
        root,
        string_field(value, "path")?,
        string_field(value, "sha256")?,
    )
}

fn resolve_evidence_object(
    root: &Path,
    value: &serde_json::Map<String, Value>,
) -> Result<(), TraceabilityError> {
    resolve_evidence_parts(
        root,
        string_from(value, "path")?,
        string_from(value, "sha256")?,
    )
}

fn resolve_evidence_parts(root: &Path, path: &str, digest: &str) -> Result<(), TraceabilityError> {
    let path = RepositoryPath::parse(path).map_err(|_| error("evidence-path"))?;
    let evidence_path = root.join(path.as_str());
    if hash_file(&evidence_path).map_err(|source| TraceabilityError::Hash {
        path: evidence_path,
        source,
    })? != digest_from(digest, "evidence-digest")?
    {
        return fail("evidence-digest");
    }
    Ok(())
}

fn capability_keys(
    values: &serde_json::Map<String, Value>,
    code: &'static str,
) -> Result<BTreeSet<CapabilityId>, TraceabilityError> {
    values
        .keys()
        .map(|item| CapabilityId::parse(item).map_err(|_| error(code)))
        .collect()
}

fn constraint_keys(
    values: &serde_json::Map<String, Value>,
) -> Result<BTreeSet<ConstraintId>, TraceabilityError> {
    values
        .keys()
        .map(|item| ConstraintId::parse(item).map_err(|_| error("evidence-constraint-set")))
        .collect()
}

fn environment_keys(
    values: &serde_json::Map<String, Value>,
) -> Result<BTreeSet<EnvironmentId>, TraceabilityError> {
    values
        .keys()
        .map(|item| item.parse().map_err(|_| error("environment-identifier")))
        .collect()
}

fn link_target(value: &Value) -> Result<LinkTarget, TraceabilityError> {
    LinkTarget::parse(string_field(value, "linkTarget")?).map_err(|_| error("artifact-link-target"))
}

fn specification_field(value: &Value) -> Result<SpecificationVersion, TraceabilityError> {
    SpecificationVersion::parse(string_field(value, "specificationVersion")?)
        .map_err(|_| error("specification-version"))
}

fn digest_field(
    value: &Value,
    field: &str,
    code: &'static str,
) -> Result<Sha256Digest, TraceabilityError> {
    digest_from(string_field(value, field)?, code)
}

fn digest_from(value: &str, code: &'static str) -> Result<Sha256Digest, TraceabilityError> {
    value.parse().map_err(|_| error(code))
}

fn object_field<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a serde_json::Map<String, Value>, TraceabilityError> {
    value
        .get(field)
        .and_then(Value::as_object)
        .ok_or_else(|| error("required-object"))
}

fn object_value<'a>(value: &'a Value, field: &str) -> Result<&'a Value, TraceabilityError> {
    value.get(field).ok_or_else(|| error("required-object"))
}

fn array_field<'a>(value: &'a Value, field: &str) -> Result<&'a Vec<Value>, TraceabilityError> {
    value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| error("required-array"))
}

fn string_field<'a>(value: &'a Value, field: &str) -> Result<&'a str, TraceabilityError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| error("required-string"))
}

fn string_from<'a>(
    value: &'a serde_json::Map<String, Value>,
    field: &str,
) -> Result<&'a str, TraceabilityError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| error("required-string"))
}

fn integer_field(value: &Value, field: &str) -> Result<u64, TraceabilityError> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| error("required-integer"))
}

fn string_array<'a>(value: &'a Value, field: &str) -> Result<Vec<&'a str>, TraceabilityError> {
    array_field(value, field)?
        .iter()
        .map(|item| item.as_str().ok_or_else(|| error("required-string-array")))
        .collect()
}

fn word(source: &str, term: &str) -> bool {
    source.match_indices(term).any(|(index, _)| {
        let before = source[..index].chars().next_back();
        let after = source[index + term.len()..].chars().next();
        !before.is_some_and(identifier_char) && !after.is_some_and(identifier_char)
    })
}

const fn identifier_char(value: char) -> bool {
    value.is_ascii_alphanumeric() || value == '_'
}

fn read_json(path: &Path) -> Result<Value, TraceabilityError> {
    serde_json::from_slice(&fs::read(path).map_err(|source| TraceabilityError::Io {
        path: path.to_path_buf(),
        source,
    })?)
    .map_err(|source| TraceabilityError::Json {
        path: path.to_path_buf(),
        source,
    })
}

fn read_text(path: &Path) -> Result<String, TraceabilityError> {
    fs::read_to_string(path).map_err(|source| TraceabilityError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn fail<T>(code: &'static str) -> Result<T, TraceabilityError> {
    Err(error(code))
}

const fn error(code: &'static str) -> TraceabilityError {
    TraceabilityError::Invariant { code }
}

const EXPECTED_CONSTRAINTS: [&str; CONSTRAINT_COUNT] = [
    "CON-PERF-001",
    "CON-PERF-002",
    "CON-PERF-003",
    "CON-MEM-001",
    "CON-SIZE-001",
    "CON-SIZE-002",
    "CON-FRM-001",
    "CON-FRM-002",
    "CON-REC-001",
    "CON-REC-002",
    "CON-REC-003",
    "CON-REC-004",
    "CON-REC-005",
    "CON-REC-006",
    "CON-REC-007",
    "CON-DET-001",
    "CON-DET-002",
    "CON-UPG-001",
    "CON-COMP-001",
    "CON-SAFE-001",
    "CON-SEC-001",
    "CON-SEC-002",
    "CON-SEC-003",
    "CON-PRV-001",
    "CON-DIA-001",
    "CON-DST-001",
    "CON-LIC-001",
];

const REQUIRED_ACCESSIBILITY_CATEGORIES: &[&str] = &[
    "roles",
    "states",
    "actions",
    "values",
    "labels",
    "accessibleNames",
    "descriptions",
    "hints",
    "helpOrFullDescriptions",
    "tooltips",
    "attributedText",
    "identifiers",
    "bounds",
    "transforms",
    "traversal",
    "labelledByRelations",
    "describedByRelations",
    "roleApplicableRelations",
    "accessibilityFocus",
    "inputFocus",
    "hitTesting",
    "textRanges",
    "selection",
    "scrollExtents",
    "language",
    "direction",
    "headingLevels",
    "liveRegions",
    "hidden",
    "disabled",
    "secureFieldRedaction",
    "multiViewIsolation",
];

const REQUIRED_SYMBOL_EDGES: &[(&str, &str, &str)] = &[
    (
        "CAP-REN-002",
        "contracts/oxyflut-public.rs",
        "Canvas::draw_texture",
    ),
    (
        "CAP-REN-002",
        "contracts/oxyflut-substrate.rs",
        "SceneBuilder::draw_texture",
    ),
    (
        "CAP-REN-002",
        "contracts/oxyflut-substrate.h",
        "OxySubstrateApi.scene_builder_draw_texture",
    ),
    (
        "CAP-SEM-002",
        "contracts/oxyflut-public.rs",
        "SemanticsBridge::perform_action",
    ),
    (
        "CAP-SEM-002",
        "contracts/oxyflut-substrate.rs",
        "SubstrateEvents::semantics_action",
    ),
    (
        "CAP-SEM-002",
        "contracts/oxyflut-substrate.rs",
        "SubstrateAdapter::respond_semantics_action",
    ),
    (
        "CAP-SEM-002",
        "contracts/oxyflut-substrate.h",
        "OxySubstrateCallbacks.on_semantics_action",
    ),
    (
        "CAP-SEM-002",
        "contracts/oxyflut-substrate.h",
        "OxySubstrateApi.respond_semantics_action",
    ),
];

fn edge_matrix(capability: &str) -> Option<&'static [&'static str]> {
    CAPABILITY_CONTRACT_EDGES
        .iter()
        .find(|(id, _)| *id == capability)
        .map(|(_, contracts)| *contracts)
}

const CAPABILITY_CONTRACT_EDGES: &[(&str, &[&str])] = &[
    ("CAP-CMP-001", &["contracts/oxyflut-public.rs"]),
    ("CAP-CMP-002", &["contracts/oxyflut-public.rs"]),
    ("CAP-CMP-003", &["contracts/oxyflut-public.rs"]),
    ("CAP-CMP-004", &["contracts/oxyflut-public.rs"]),
    ("CAP-CMP-005", &["contracts/oxyflut-public.rs"]),
    ("CAP-CMP-006", &["contracts/oxyflut-public.rs"]),
    ("CAP-CMP-007", &["contracts/oxyflut-public.rs"]),
    ("CAP-LAY-001", &["contracts/oxyflut-public.rs"]),
    ("CAP-LAY-002", &["contracts/oxyflut-public.rs"]),
    ("CAP-SCR-001", &["contracts/oxyflut-public.rs"]),
    ("CAP-SCR-002", &["contracts/oxyflut-public.rs"]),
    (
        "CAP-REN-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-REN-002",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-REN-003",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    ("CAP-AST-001", &["contracts/oxyflut-public.rs"]),
    ("CAP-AST-002", &["contracts/oxyflut-public.rs"]),
    ("CAP-AST-003", &["contracts/oxyflut-public.rs"]),
    (
        "CAP-AST-004",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-VIEW-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-VIEW-002",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-VIEW-003",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-VIEW-004",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-VIEW-005",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-REC-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    ("CAP-INP-001", &["contracts/oxyflut-public.rs"]),
    ("CAP-INP-002", &["contracts/oxyflut-public.rs"]),
    ("CAP-FOC-001", &["contracts/oxyflut-public.rs"]),
    (
        "CAP-TXT-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-TXT-002",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-TXT-003",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-IME-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-CLP-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-I18N-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-SEM-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
            "contracts/platform-contracts.json",
            "data-models/accessibility-map.schema.json",
        ],
    ),
    (
        "CAP-SEM-002",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
            "contracts/platform-contracts.json",
            "data-models/accessibility-map.schema.json",
        ],
    ),
    (
        "CAP-PLT-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/platform-contracts.json",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-OS-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/platform-contracts.json",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-OS-002",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/platform-contracts.json",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-TST-001",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "data-models/qualification-evidence.schema.json",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-TST-002",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "data-models/qualification-evidence.schema.json",
        ],
    ),
    (
        "CAP-TST-003",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "data-models/qualification-evidence.schema.json",
        ],
    ),
    (
        "CAP-TST-004",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "data-models/qualification-evidence.schema.json",
        ],
    ),
    (
        "CAP-DST-001",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "data-models/artifact-manifest.schema.json",
            "data-models/qualification-evidence.schema.json",
            "data-models/release-evidence-bundle.schema.json",
            "data-models/ci-invocation.schema.json",
        ],
    ),
    (
        "CAP-SEC-001",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "data-models/ingress-inventory.schema.json",
            "data-models/qualification-evidence.schema.json",
        ],
    ),
    (
        "CAP-DIA-001",
        &[
            "contracts/diagnostic-event-registry.json",
            "data-models/diagnostic-event.schema.json",
        ],
    ),
    (
        "CAP-DIA-002",
        &[
            "contracts/diagnostic-event-registry.json",
            "data-models/diagnostic-event.schema.json",
        ],
    ),
    (
        "CAP-DIA-003",
        &[
            "contracts/diagnostic-event-registry.json",
            "data-models/diagnostic-event.schema.json",
        ],
    ),
    (
        "CAP-DIA-004",
        &[
            "contracts/diagnostic-event-registry.json",
            "data-models/diagnostic-event.schema.json",
            "contracts/oxyflut-public.rs",
        ],
    ),
    (
        "CAP-SUB-001",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "contracts/specification-phase.json",
            "data-models/qualification-evidence.schema.json",
        ],
    ),
    (
        "CAP-SUB-002",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "contracts/specification-phase.json",
            "data-models/qualification-evidence.schema.json",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-SUB-003",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "contracts/specification-phase.json",
            "data-models/qualification-evidence.schema.json",
            "data-models/selection-decision.schema.json",
        ],
    ),
    (
        "CAP-SUB-004",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "contracts/specification-phase.json",
            "data-models/qualification-evidence.schema.json",
            "data-models/selection-decision.schema.json",
        ],
    ),
];

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::path::{Path, PathBuf};

    use oxyflut_qualification::identifiers::{CandidateId, EnvironmentId, RepositoryPath};
    use serde_json::{Value, json};

    use super::{ContractTestResolution, validate_workspace};

    #[test]
    fn committed_constitution_has_exact_upstream_traceability_sets() -> Result<(), Box<dyn Error>> {
        let report = validate_workspace(&workspace_root()?)?;
        assert_eq!(report.capability_count, 52);
        assert_eq!(report.constraint_count, 27);
        assert_eq!(
            report.contract_test_resolution,
            ContractTestResolution::DeferredUntilCandidateImplementation
        );
        assert_eq!(report.deferred_contract_tests, 52);
        assert!(report.accessibility_generation_deferred);
        Ok(())
    }

    #[test]
    fn exact_sets_bind_missing_duplicate_unknown_stale_and_physical_edge_fixtures()
    -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let active = super::active_specification(&root)?;
        let capabilities = super::prd_capabilities(&root)?;
        let flows = super::architecture_flows(&root)?;
        let original = super::read_json(&root.join(super::TRACEABILITY_PATH))?;

        let mut missing = original.clone();
        missing
            .get_mut("mappings")
            .and_then(Value::as_array_mut)
            .ok_or("mappings must be an array")?
            .pop()
            .ok_or("positive traceability must contain a mapping")?;
        assert_code(
            super::validate_traceability(&root, &missing, &active, &capabilities, &flows),
            "capability-set",
        );

        let mut duplicate = original.clone();
        let duplicated_id = duplicate
            .pointer("/mappings/0/capabilityId")
            .and_then(Value::as_str)
            .ok_or("first capability ID must exist")?
            .to_owned();
        *duplicate
            .pointer_mut("/mappings/1/capabilityId")
            .ok_or("second capability ID must exist")? = Value::String(duplicated_id);
        assert_code(
            super::validate_traceability(&root, &duplicate, &active, &capabilities, &flows),
            "traceability-capability-duplicate",
        );

        let mut unknown = original.clone();
        *unknown
            .pointer_mut("/mappings/0/capabilityId")
            .ok_or("first capability ID must exist")? = Value::String("CAP-UNKNOWN-001".to_owned());
        assert_code(
            super::validate_traceability(&root, &unknown, &active, &capabilities, &flows),
            "traceability-flow-path",
        );

        let mut stale_path = original.clone();
        *stale_path
            .pointer_mut("/mappings/0/architectureFlow")
            .ok_or("first architecture flow must exist")? =
            Value::String(".constitution/architecture/flows/stale.md".to_owned());
        assert_code(
            super::validate_traceability(&root, &stale_path, &active, &capabilities, &flows),
            "traceability-flow-path",
        );

        let mut omitted_contract = original.clone();
        omitted_contract
            .pointer_mut("/mappings/0/bindings")
            .and_then(Value::as_array_mut)
            .ok_or("first binding set must exist")?
            .clear();
        assert_code(
            super::validate_traceability(&root, &omitted_contract, &active, &capabilities, &flows),
            "capability-contract-edge",
        );

        let mut unresolved_symbol = original.clone();
        *unresolved_symbol
            .pointer_mut("/mappings/0/bindings/0/symbols/0")
            .ok_or("first physical symbol must exist")? =
            Value::String("MissingPhysicalSymbol".to_owned());
        assert_code(
            super::validate_traceability(&root, &unresolved_symbol, &active, &capabilities, &flows),
            "contract-symbol-text",
        );

        let mut stale_version = original;
        *stale_version
            .pointer_mut("/specificationVersion")
            .ok_or("traceability specification version must exist")? =
            Value::String("0.0.0".to_owned());
        assert_code(
            super::validate_traceability(&root, &stale_version, &active, &capabilities, &flows),
            "traceability-specification-version",
        );
        Ok(())
    }

    #[test]
    fn texture_reverse_ingress_and_contract_test_bijection_are_closed() -> Result<(), Box<dyn Error>>
    {
        let root = workspace_root()?;
        let active = super::active_specification(&root)?;
        let capabilities = super::prd_capabilities(&root)?;
        let flows = super::architecture_flows(&root)?;
        let original = super::read_json(&root.join(super::TRACEABILITY_PATH))?;

        let mut texture = original.clone();
        remove_symbol(
            &mut texture,
            "CAP-REN-002",
            "contracts/oxyflut-public.rs",
            "Canvas::draw_texture",
        )?;
        assert_code(
            super::validate_traceability(&root, &texture, &active, &capabilities, &flows),
            "capability-symbol-edge",
        );

        let mut reverse = original.clone();
        remove_symbol(
            &mut reverse,
            "CAP-SEM-002",
            "contracts/oxyflut-substrate.h",
            "OxySubstrateCallbacks.on_semantics_action",
        )?;
        assert_code(
            super::validate_traceability(&root, &reverse, &active, &capabilities, &flows),
            "capability-symbol-edge",
        );

        let mut missing = original.clone();
        *missing
            .pointer_mut("/mappings/0/contractTests")
            .ok_or("contract tests must exist")? = json!([]);
        assert_code(
            super::validate_traceability(&root, &missing, &active, &capabilities, &flows),
            "contract-test-cardinality",
        );

        let mut duplicate = original.clone();
        let contract_test = duplicate
            .pointer("/mappings/0/contractTests/0")
            .and_then(Value::as_str)
            .ok_or("contract test must exist")?
            .to_owned();
        *duplicate
            .pointer_mut("/mappings/0/contractTests")
            .ok_or("contract tests must exist")? = json!([contract_test, contract_test]);
        assert_code(
            super::validate_traceability(&root, &duplicate, &active, &capabilities, &flows),
            "contract-test-cardinality",
        );

        let mut extra = original.clone();
        *extra
            .pointer_mut("/mappings/0/contractTests")
            .ok_or("contract tests must exist")? =
            json!(["contract::cap_cmp_001", "contract::cap_extra_001"]);
        assert_code(
            super::validate_traceability(&root, &extra, &active, &capabilities, &flows),
            "contract-test-cardinality",
        );

        let mut renamed = original.clone();
        *renamed
            .pointer_mut("/mappings/0/contractTests/0")
            .ok_or("contract test must exist")? =
            Value::String("contract::cap_renamed_001".to_owned());
        assert_code(
            super::validate_traceability(&root, &renamed, &active, &capabilities, &flows),
            "contract-test-derivation",
        );
        Ok(())
    }

    #[test]
    fn artifact_paths_links_and_raw_sample_keys_fail_closed() -> Result<(), Box<dyn Error>> {
        let manifest = json!({
            "files": [
                {"path":"bin/oxyflut","kind":"file","size":3,"sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"},
                {"path":"bin/oxyflut-hard","kind":"hardlink","linkTarget":"bin/oxyflut","size":3,"sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"},
                {"path":"bin/oxyflut-link","kind":"symlink","linkTarget":"bin/link-target","size":15,"sha256":"f8e5c629216c6cfe97bee39dde9d3d59ee01558e5e8fb38474cc2bba8e3528c6"}
            ]
        });
        assert!(super::validate_artifact_manifest(&manifest).is_ok());
        for (path, expected) in [
            ("", "artifact-path"),
            ("bin/", "artifact-path"),
            ("bin//oxyflut", "artifact-path"),
            ("bin/\u{0001}oxyflut", "artifact-path"),
        ] {
            let mut invalid = manifest.clone();
            *invalid
                .pointer_mut("/files/0/path")
                .ok_or("artifact path must exist")? = Value::String(path.to_owned());
            assert_code(super::validate_artifact_manifest(&invalid), expected);
        }

        let constraints = super::prd_constraints(&workspace_root()?)?;
        let raw = json!({"samples":[
            {"constraintId":"CON-PERF-001","launch":1,"ordinal":1},
            {"constraintId":"CON-PERF-001","launch":1,"ordinal":1}
        ]});
        assert_code(
            super::validate_raw_measurement(&raw, &constraints),
            "raw-sample-key-duplicate",
        );
        Ok(())
    }

    #[test]
    fn absence_proof_platform_claims_and_accessibility_references_fail_closed()
    -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let active = super::active_specification(&root)?;
        let registry = super::read_json(&root.join(super::REGISTRY_PATH))?;
        let platform_path =
            "qualification/fixtures/contracts/traceability/synthetic-platform-baseline.json";
        let expected_path = platform_path.parse()?;
        let binding = json!({
            "platformBaseline": {
                "path": platform_path,
                "sha256": "a68ecd2fac76ae263ced91fda4fb8c144857be46435d77eca33366ce8cd52a86",
                "schemaVersion": "5.0.0",
                "specificationVersion": "0.15.0"
            },
            "absentEventId": "ABS-CMP-001"
        });
        super::validate_absence_binding(
            &root,
            binding.as_object().ok_or("binding must be an object")?,
            &"CAP-CMP-001".parse()?,
            CandidateId::Focused,
            EnvironmentId::Macos,
            false,
            &active,
            &registry,
            &expected_path,
        )?;
        for (pointer, value, code) in [
            ("/absentEventId", "ABS-CMP-404", "absent-event-unknown"),
            (
                "/platformBaseline/path",
                "qualification/fixtures/contracts/traceability/missing.json",
                "not-applicable-baseline-path",
            ),
            (
                "/platformBaseline/sha256",
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "not-applicable-baseline-digest",
            ),
            (
                "/platformBaseline/schemaVersion",
                "4.0.0",
                "not-applicable-baseline-schema-version",
            ),
            (
                "/platformBaseline/specificationVersion",
                "0.0.0",
                "not-applicable-baseline-specification-version",
            ),
        ] {
            let mut invalid = binding.clone();
            *invalid
                .pointer_mut(pointer)
                .ok_or("binding pointer must exist")? = Value::String(value.to_owned());
            assert_code(
                super::validate_absence_binding(
                    &root,
                    invalid.as_object().ok_or("binding must be an object")?,
                    &"CAP-CMP-001".parse()?,
                    CandidateId::Focused,
                    EnvironmentId::Macos,
                    false,
                    &active,
                    &registry,
                    &expected_path,
                ),
                code,
            );
        }

        let baseline = super::read_json(&root.join(platform_path))?;
        let entry = baseline
            .pointer("/absentEvents/0")
            .ok_or("absent event must exist")?;
        let mut mismatched_gate = entry.clone();
        *mismatched_gate
            .pointer_mut("/gateId")
            .ok_or("gate ID must exist")? = Value::String("CAP-CMP-002".to_owned());
        assert_code(
            super::validate_absent_event(
                &root,
                &mismatched_gate,
                &"CAP-CMP-001".parse()?,
                CandidateId::Focused,
                EnvironmentId::Macos,
                false,
                &registry,
            ),
            "absent-event-gate",
        );
        let mut unregistered = entry.clone();
        *unregistered
            .pointer_mut("/eventId")
            .ok_or("event ID must exist")? = Value::String("unregistered.event".to_owned());
        assert_code(
            super::validate_absent_event(
                &root,
                &unregistered,
                &"CAP-CMP-001".parse()?,
                CandidateId::Focused,
                EnvironmentId::Macos,
                false,
                &registry,
            ),
            "absent-event-event",
        );
        assert_code(
            super::validate_absent_event(
                &root,
                entry,
                &"CAP-CMP-001".parse()?,
                CandidateId::Integrated,
                EnvironmentId::Macos,
                false,
                &registry,
            ),
            "absent-event-candidate",
        );
        assert_code(
            super::validate_absent_event(
                &root,
                entry,
                &"CAP-CMP-001".parse()?,
                CandidateId::Focused,
                EnvironmentId::Windows,
                false,
                &registry,
            ),
            "absent-event-environment",
        );
        assert_code(
            super::validate_absent_event(
                &root,
                entry,
                &"CAP-CMP-001".parse()?,
                CandidateId::Focused,
                EnvironmentId::Macos,
                true,
                &registry,
            ),
            "absent-event-aggregate-environments",
        );

        let committed = super::read_json(&root.join(super::PLATFORM_PATH))?;
        let mut stale_platform = committed.clone();
        *stale_platform
            .pointer_mut("/specificationVersion")
            .ok_or("platform version must exist")? = Value::String("0.0.0".to_owned());
        assert_code(
            super::validate_platform_baseline(&root, &stale_platform, &active, &registry),
            "platform-specification-version",
        );
        let mut missing_kk = committed.clone();
        *missing_kk
            .pointer_mut("/environments/macos/ime/status")
            .ok_or("IME status must exist")? = Value::String("kk".to_owned());
        assert_code(
            super::validate_platform_baseline(&root, &missing_kk, &active, &registry),
            "kk-evidence-missing",
        );

        let stale_reference = json!({
            "status": "kk",
            "path": "qualification/fixtures/contracts/traceability/synthetic-accessibility-stale.json",
            "sha256": "2b933b3b8093f35619e6dc703a910eef1049720bd2fda14925a506b5ce41e045"
        });
        let mut stale_accessibility = committed.clone();
        *stale_accessibility
            .pointer_mut("/environments/macos/accessibilityMaps/focused")
            .ok_or("accessibility reference must exist")? = stale_reference.clone();
        assert_code(
            super::validate_platform_baseline(&root, &stale_accessibility, &active, &registry),
            "accessibility-text-layout-generation",
        );
        let mut wrong_accessibility_digest = stale_accessibility.clone();
        *wrong_accessibility_digest
            .pointer_mut("/environments/macos/accessibilityMaps/focused/sha256")
            .ok_or("accessibility digest must exist")? = Value::String(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
        );
        assert_code(
            super::validate_platform_baseline(
                &root,
                &wrong_accessibility_digest,
                &active,
                &registry,
            ),
            "accessibility-digest",
        );
        let mut wrong_accessibility_identity = committed.clone();
        *wrong_accessibility_identity
            .pointer_mut("/environments/windows/accessibilityMaps/focused")
            .ok_or("accessibility reference must exist")? = stale_reference;
        assert_code(
            super::validate_platform_baseline(
                &root,
                &wrong_accessibility_identity,
                &active,
                &registry,
            ),
            "accessibility-identity",
        );
        let mut nested_accessibility_ku = committed;
        *nested_accessibility_ku
            .pointer_mut("/environments/macos/accessibilityMaps/focused")
            .ok_or("accessibility reference must exist")? = json!({
            "status": "kk",
            "path": "qualification/fixtures/contracts/traceability/synthetic-accessibility-ku.json",
            "sha256": "5ab80313abfd55a93eae5d8fc9c1ce14cf7234a4fe5ec9feff4e72380bec3ab2"
        });
        assert_code(
            super::validate_platform_baseline(&root, &nested_accessibility_ku, &active, &registry),
            "accessibility-ku",
        );
        Ok(())
    }

    #[test]
    fn eligible_evidence_and_baseline_provenance_are_exact_and_digest_bound()
    -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let active = super::active_specification(&root)?;
        let capabilities = super::prd_capabilities(&root)?;
        let constraints = super::prd_constraints(&root)?;
        let registry = super::read_json(&root.join(super::REGISTRY_PATH))?;
        let evidence_reference = json!({
            "path": "qualification/fixtures/contracts/traceability/evidence.txt",
            "sha256": "6ab11a71e6e2c7be933b6e2c3481a795e56ccd5bc7dfb51b72d6dcfd68458f4d"
        });
        let capability_results = capabilities
            .iter()
            .map(|id| (id.as_str().to_owned(), json!({"status":"pass","evidence":[evidence_reference.clone()],"notApplicable":null})))
            .collect::<serde_json::Map<_, _>>();
        let constraint_results = constraints
            .iter()
            .map(|id| (id.as_str().to_owned(), json!({"status":"pass","evidence":[evidence_reference.clone()],"notApplicable":null})))
            .collect::<serde_json::Map<_, _>>();
        let record = json!({
            "candidate":"focused",
            "eligibility":"eligible",
            "environmentResults": {
                "macos": capability_results.clone(), "windows": capability_results.clone(),
                "wayland": capability_results.clone(), "x11": capability_results
            },
            "constraintResults": constraint_results
        });
        let platform_path: RepositoryPath =
            ".constitution/tech-spec/contracts/platform-contracts.json".parse()?;
        super::validate_qualification_evidence(
            &root,
            &record,
            &active,
            &capabilities,
            &constraints,
            &registry,
            &platform_path,
        )?;
        let mut nonnull_pass = record.clone();
        *nonnull_pass
            .pointer_mut("/environmentResults/macos/CAP-CMP-001/notApplicable")
            .ok_or("capability gate must exist")? = json!({});
        assert_code(
            super::validate_qualification_evidence(
                &root,
                &nonnull_pass,
                &active,
                &capabilities,
                &constraints,
                &registry,
                &platform_path,
            ),
            "pass-absence-binding",
        );
        let mut open_gate = record;
        *open_gate
            .pointer_mut("/environmentResults/macos/CAP-CMP-001/status")
            .ok_or("capability gate must exist")? = Value::String("gating-ku".to_owned());
        assert_code(
            super::validate_qualification_evidence(
                &root,
                &open_gate,
                &active,
                &capabilities,
                &constraints,
                &registry,
                &platform_path,
            ),
            "eligible-gate-status",
        );

        let baseline_capabilities = capabilities
            .iter()
            .map(|id| (id.as_str().to_owned(), json!({})))
            .collect::<serde_json::Map<_, _>>();
        let synthetic = json!({
            "specificationVersion":"0.15.0",
            "provenance":{"kind":"synthetic","approvalEvidence":null},
            "capabilities": baseline_capabilities.clone()
        });
        super::validate_capability_baseline(&root, &synthetic, &active, &capabilities)?;
        let approved = json!({
            "specificationVersion":"0.15.0",
            "provenance":{"kind":"approved","approvalEvidence":evidence_reference},
            "capabilities": baseline_capabilities
        });
        super::validate_capability_baseline(&root, &approved, &active, &capabilities)?;
        let mut missing_approval = approved;
        *missing_approval
            .pointer_mut("/provenance/approvalEvidence")
            .ok_or("approval evidence must exist")? = Value::Null;
        assert_code(
            super::validate_capability_baseline(&root, &missing_approval, &active, &capabilities),
            "baseline-approval-evidence",
        );
        Ok(())
    }

    #[test]
    fn every_ticket_named_negative_fixture_has_a_checked_case_file() -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        for case in NEGATIVE_FIXTURE_CASES {
            let fixture = super::read_json(
                &root
                    .join("qualification/fixtures/contracts/traceability")
                    .join(format!("{case}.json")),
            )?;
            assert_eq!(fixture.get("case").and_then(Value::as_str), Some(*case));
        }
        Ok(())
    }

    fn remove_symbol(
        traceability: &mut Value,
        capability: &str,
        contract: &str,
        symbol: &str,
    ) -> Result<(), Box<dyn Error>> {
        let mappings = traceability
            .get_mut("mappings")
            .and_then(Value::as_array_mut)
            .ok_or("mappings must be an array")?;
        let mapping = mappings
            .iter_mut()
            .find(|item| item.get("capabilityId").and_then(Value::as_str) == Some(capability))
            .ok_or("mapping must exist")?;
        let bindings = mapping
            .get_mut("bindings")
            .and_then(Value::as_array_mut)
            .ok_or("bindings must be an array")?;
        let binding = bindings
            .iter_mut()
            .find(|item| item.get("contract").and_then(Value::as_str) == Some(contract))
            .ok_or("binding must exist")?;
        let symbols = binding
            .get_mut("symbols")
            .and_then(Value::as_array_mut)
            .ok_or("symbols must be an array")?;
        let position = symbols
            .iter()
            .position(|item| item.as_str() == Some(symbol))
            .ok_or("symbol must exist")?;
        let _removed = symbols.remove(position);
        Ok(())
    }

    fn assert_code<T>(result: Result<T, super::TraceabilityError>, expected: &'static str) {
        let code = match result {
            Err(error) => error.code(),
            Ok(_) => None,
        };
        assert_eq!(code, Some(expected));
    }

    fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| "xtask must remain directly below the workspace root".into())
    }

    const NEGATIVE_FIXTURE_CASES: &[&str] = &[
        "missing",
        "duplicate",
        "duplicated-contract-test-id",
        "extra-contract-test-id",
        "non-derivable-contract-test-id",
        "unknown",
        "stale-path",
        "omitted-required-capability-to-contract-edge",
        "omitted-texture-drawing-edge",
        "omitted-reverse-action-ingress",
        "unresolved-file-qualified-symbol",
        "mismatched-active-specification-version",
        "stale-platform-baseline-specification-version",
        "duplicate-absent-event-id",
        "synthetic-baseline-referenced-by-lock",
        "missing-baseline-approval-evidence",
        "mismatched-baseline-approval-evidence",
        "pass-with-nonnull-absence-binding",
        "not-applicable-missing-binding",
        "mismatched-platform-baseline-path",
        "mismatched-platform-baseline-digest",
        "mismatched-platform-baseline-schema-version",
        "mismatched-platform-baseline-specification-version",
        "unknown-absent-event-id",
        "mismatched-absent-event-gate",
        "mismatched-absent-event-event",
        "unregistered-absent-event",
        "mismatched-absent-event-candidate",
        "mismatched-parent-environment",
        "aggregate-constraint-without-four-environments",
        "remote-diagnostic-sink",
        "undeclared-diagnostic-sink",
        "unbounded-sink-acknowledgement",
        "missing-nested-kk-evidence",
        "mismatched-nested-kk-evidence",
        "mismatched-accessibility-identity",
        "mismatched-accessibility-digest",
        "nested-accessibility-ku",
        "stale-accessibility-text-layout-generation",
        "empty-path-segment",
        "trailing-path-segment",
        "duplicate-path-separators",
        "control-character-path",
    ];
}
