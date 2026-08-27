//! Evidence, baseline, and platform traceability validation.

use super::edges::REQUIRED_ACCESSIBILITY_CATEGORIES;
use super::tokens::symbol_resolves;
use super::*;

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

#[allow(
    dead_code,
    reason = "Later contract-validation families validate artifact manifests."
)]
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

#[allow(
    dead_code,
    reason = "Later contract-validation families validate raw measurement records."
)]
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

/// Validates a promotion qualification-evidence record against the active fixture or workspace authority.
///
/// # Errors
///
/// Returns an error when the active specification, exact PRD gate sets, diagnostic registry, platform baseline, or typed absence proof is invalid.
pub(crate) fn validate_promotion_qualification_evidence(
    root: &Path,
    record: &Value,
    phase: &Value,
) -> Result<(), TraceabilityError> {
    let active = specification_field(phase)?;
    let capabilities = prd_capabilities(root)?;
    let constraints = prd_constraints(root)?;
    let registry = read_json(&root.join(REGISTRY_PATH))?;
    let active_platform_path =
        RepositoryPath::parse(PLATFORM_PATH).map_err(|_| error("not-applicable-baseline-path"))?;
    validate_qualification_evidence(
        root,
        record,
        &active,
        &capabilities,
        &constraints,
        &registry,
        &active_platform_path,
    )
}

#[allow(
    dead_code,
    reason = "Later contract-validation families validate qualification evidence records."
)]
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
pub(crate) fn validate_gate(
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
pub(crate) fn validate_absence_binding(
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

pub(crate) fn validate_absent_event(
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

pub(crate) fn validate_platform_baseline(
    root: &Path,
    baseline: &Value,
    active: &SpecificationVersion,
    registry: &Value,
) -> Result<(), TraceabilityError> {
    let schema_registry = schema_registry(root)?;
    validate_platform_baseline_with_schema(root, baseline, active, registry, &schema_registry)
}

fn validate_platform_baseline_with_schema(
    root: &Path,
    baseline: &Value,
    active: &SpecificationVersion,
    registry: &Value,
    schema_registry: &SchemaRegistry,
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
        validate_platform_environment(root, environment, environment_id, schema_registry)?;
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
    schema_registry: &SchemaRegistry,
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
            schema_registry,
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
    schema_registry: &SchemaRegistry,
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
    validate_referenced_schema(
        schema_registry,
        ACCESSIBILITY_MAP_SCHEMA,
        &map_path,
        &map,
        "accessibility-schema",
    )?;
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
    let substrate_path = root
        .join(SPEC_DIRECTORY)
        .join("contracts/oxyflut-substrate.rs");
    let public_path = root
        .join(SPEC_DIRECTORY)
        .join("contracts/oxyflut-public.rs");
    let substrate = read_text(&substrate_path)?;
    let public = read_text(&public_path)?;
    Ok(
        symbol_resolves(&substrate_path, &substrate, "TextLayoutGeneration")?
            && symbol_resolves(&public_path, &public, "TextLayoutId")?,
    )
}
