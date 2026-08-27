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
use oxyflut_qualification::schema::{SchemaError, SchemaRegistry};
use serde_json::Value;
use thiserror::Error;

use super::digests::{self, DigestError};
use super::schema::ContractSchemaError;

use super::registries::{self, RegistryError};

mod edges;
#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod tests;
mod tokens;
mod validation;

use edges::{EXPECTED_CONSTRAINTS, edge_matrix, validate_required_symbol_edges};
use tokens::symbol_resolves;
pub(crate) use validation::*;
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
const ACCESSIBILITY_MAP_SCHEMA: &str = "urn:oxyflut:schema:accessibility-map:5";
const CAPABILITY_BASELINE_SCHEMA: &str = "urn:oxyflut:schema:capability-baseline:4";

/// The status of physical common candidate contract-test resolution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ContractTestResolution {
    /// No Stage 3 document defines a file location for the identifiers.
    DeferredUntilCandidateImplementation,
    /// Every common candidate contract test resolves to a physical test location.
    #[allow(
        dead_code,
        reason = "Candidate implementation owns the future physical-test resolution report."
    )]
    Resolved,
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
    /// A repository-confined immutable evidence reference failed verification.
    #[error("traceability evidence reference failed")]
    Digest(#[from] DigestError),
    /// The diagnostic registry violated its own semantic contract.
    #[error("diagnostic registry validation failed")]
    Registry(#[source] RegistryError),
    /// The local schema registry required by a dereferenced document could not be compiled.
    #[error("local schema registry failed")]
    SchemaRegistry(#[source] ContractSchemaError),
    /// A digest-bound document failed its required local schema before semantic validation.
    #[error("traceability schema validation failed: {code}")]
    Schema {
        /// Stable failure code for the referenced document family.
        code: &'static str,
        /// The dereferenced document path.
        path: PathBuf,
        /// The local schema validation failure.
        #[source]
        source: SchemaError,
    },
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
            Self::Invariant { code } | Self::Schema { code, .. } => Some(code),
            Self::Digest(_) => Some("evidence-digest"),
            Self::Io { .. }
            | Self::Json { .. }
            | Self::Hash { .. }
            | Self::Registry(_)
            | Self::SchemaRegistry(_) => None,
        }
    }
}

/// Validates the committed upstream sets, active specification, physical contracts, and platform baseline.
///
/// # Errors
///
/// Returns an error for a local input failure or any exact-set, identifier, symbol, version, or immutable-reference mismatch.
pub(crate) fn validate_workspace(root: &Path) -> Result<TraceabilityRunReport, TraceabilityError> {
    let (active, capabilities) = capability_baseline_authority(root)?;
    let constraints = prd_constraints(root)?;
    let flows = architecture_flows(root)?;
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

/// Returns the exact authoritative product constraint set.
///
/// # Errors
///
/// Returns an error when the PRD constraint table cannot be read or its identifiers are invalid.
pub(crate) fn constraint_authority(
    root: &Path,
) -> Result<BTreeSet<ConstraintId>, TraceabilityError> {
    prd_constraints(root)
}

/// Resolves the exact active specification and 52-capability authority for baseline validation.
///
/// # Errors
///
/// Returns an error if the PRD or architecture flow sets are missing, malformed, duplicated, or not the same exact 52-capability set.
pub(crate) fn capability_baseline_authority(
    root: &Path,
) -> Result<(SpecificationVersion, BTreeSet<CapabilityId>), TraceabilityError> {
    let active = active_specification(root)?;
    let capabilities = prd_capabilities(root)?;
    let flows = architecture_flows(root)?;
    validate_capability_sets(&capabilities, &flows, None)?;
    Ok((active, capabilities))
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
    let schema_registry = schema_registry(root)?;
    validate_referenced_schema(
        &schema_registry,
        CAPABILITY_BASELINE_SCHEMA,
        &baseline_path,
        &baseline,
        "capability-baseline-schema",
    )?;
    if string_field(&baseline, "schemaVersion")? != string_from(reference, "schemaVersion")? {
        return fail("lock-baseline-schema-version");
    }
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
    let _ = digests::verify_reference(
        root,
        string_field(value, "path")?,
        string_field(value, "sha256")?,
    )?;
    Ok(())
}

fn resolve_evidence_object(
    root: &Path,
    value: &serde_json::Map<String, Value>,
) -> Result<(), TraceabilityError> {
    let _ = digests::verify_object_reference(root, value)?;
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

fn schema_registry(root: &Path) -> Result<SchemaRegistry, TraceabilityError> {
    super::schema::compile_workspace(root).map_err(TraceabilityError::SchemaRegistry)
}

fn validate_referenced_schema(
    registry: &SchemaRegistry,
    identity: &str,
    path: &Path,
    value: &Value,
    code: &'static str,
) -> Result<(), TraceabilityError> {
    registry
        .validate(identity, value)
        .map_err(|source| TraceabilityError::Schema {
            code,
            path: path.to_path_buf(),
            source,
        })
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
