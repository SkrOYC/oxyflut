use std::fs;
use std::path::Path;

use oxyflut_qualification::schema::SchemaRegistry;
use serde_json::Value;

use crate::contracts::digests::{self, VerifiedReference};

use super::{
    ReadinessError, fail, invariant, object_field, object_value, read_json, require_equal,
    same_reference, string_field, string_from, validate_schema,
};

const EXTERNAL_CONTRACT_LOCK_PATH: &str =
    ".constitution/tech-spec/contracts/external-contract-lock.json";
const EXTERNAL_LOCK_SCHEMA: &str = "urn:oxyflut:schema:external-contract-lock:1";
const EVIDENCE_SCHEMA: &str = "urn:oxyflut:schema:qualification-evidence:5";
const SELECTION_SCHEMA: &str = "urn:oxyflut:schema:selection-decision:1";
const RELEASE_SCHEMA: &str = "urn:oxyflut:schema:release-evidence-bundle:1";
const ACCEPTED_ADR_PATH: &str = ".constitution/tech-spec/adrs/ADR-0010-production-substrate.md";
const PROMOTION_KEYS: [&str; 10] = [
    "selectionDecision",
    "selectedCandidateQualification",
    "layoutQualification",
    "acceptedAdr0010",
    "finalContractSet",
    "targetMatrix",
    "allTier1Results",
    "losingCandidateRemoval",
    "billOfMaterials",
    "releaseQualification",
];
const UNTYPED_PROMOTION_KEYS: [&str; 5] = [
    "layoutQualification",
    "finalContractSet",
    "targetMatrix",
    "losingCandidateRemoval",
    "billOfMaterials",
];

pub(super) fn resolve(
    root: &Path,
    lock: &Value,
    phase: &Value,
    registry: &SchemaRegistry,
) -> Result<(), ReadinessError> {
    let promotion = object_field(phase, "promotionEvidence")?;
    let lock_digest = string_from(promotion, "qualificationLockDigest")?;
    let _ = digests::verify_reference(root, super::LOCK_PATH, lock_digest)?;
    let candidate = string_from(promotion, "selectedCandidate")?;
    let version = string_field(phase, "specificationVersion")?;
    validate_candidate_state(phase, candidate)?;

    let mut references = Vec::with_capacity(PROMOTION_KEYS.len());
    for key in PROMOTION_KEYS {
        let reference = object_value(promotion, key)?;
        let verified = digests::verify_value_reference(root, reference)?;
        references.push((key, reference, verified));
    }

    let selection = promotion_reference(&references, "selectionDecision")?;
    let selection_value = read_json(&selection.2.resolved_path)?;
    digests::verify_references_in_value(root, &selection_value)?;
    validate_schema(
        registry,
        SELECTION_SCHEMA,
        &selection_value,
        "selection-decision",
    )?;
    require_equal(
        string_field(&selection_value, "qualificationLockDigest")?,
        lock_digest,
        "selection-lock-digest",
    )?;
    require_equal(
        string_field(&selection_value, "selectedCandidate")?,
        candidate,
        "selection-candidate",
    )?;
    require_equal(
        string_field(&selection_value, "specificationVersion")?,
        version,
        "selection-specification-version",
    )?;
    let selected = promotion_reference(&references, "selectedCandidateQualification")?;
    validate_selection_evidence(
        root,
        registry,
        &selection_value,
        lock,
        lock_digest,
        candidate,
        selected.1,
    )?;
    validate_qualification_evidence(
        root,
        registry,
        &selected.2,
        lock,
        lock_digest,
        candidate,
        "selected-candidate-qualification",
    )?;
    if !same_reference(
        object_value(
            object_field(&selection_value, "candidateEvidence")?,
            candidate,
        )?,
        selected.1,
    )? {
        return fail("selection-selected-evidence");
    }

    let all_tier_one = promotion_reference(&references, "allTier1Results")?;
    validate_qualification_evidence(
        root,
        registry,
        &all_tier_one.2,
        lock,
        lock_digest,
        candidate,
        "all-tier-one-results",
    )?;

    let release = promotion_reference(&references, "releaseQualification")?;
    let release_value = read_json(&release.2.resolved_path)?;
    digests::verify_references_in_value(root, &release_value)?;
    validate_schema(
        registry,
        RELEASE_SCHEMA,
        &release_value,
        "release-qualification",
    )?;
    require_equal(
        string_field(&release_value, "candidate")?,
        candidate,
        "release-candidate",
    )?;
    let external_digest = string_field(&release_value, "externalContractLockDigest")?;
    let external = digests::verify_reference(root, EXTERNAL_CONTRACT_LOCK_PATH, external_digest)?;
    validate_schema(
        registry,
        EXTERNAL_LOCK_SCHEMA,
        &read_json(&external.resolved_path)?,
        "external-contract-lock",
    )?;

    let adr = promotion_reference(&references, "acceptedAdr0010")?;
    if adr.2.path.as_str() != ACCEPTED_ADR_PATH {
        return fail("accepted-adr-path");
    }
    let adr_bytes = fs::read(&adr.2.resolved_path).map_err(|source| ReadinessError::Io {
        path: adr.2.resolved_path.clone(),
        source,
    })?;
    if !adr_bytes
        .windows(b"**Status:** accepted".len())
        .any(|value| value == b"**Status:** accepted")
        || !adr_bytes
            .windows(candidate.len())
            .any(|value| value == candidate.as_bytes())
    {
        return fail("accepted-adr-content");
    }

    let Some(first_untyped) = UNTYPED_PROMOTION_KEYS.first() else {
        return Ok(());
    };
    for key in UNTYPED_PROMOTION_KEYS {
        let (_, _, verified) = promotion_reference(&references, key)?;
        validate_exposed_untyped_identity(
            root,
            &verified.resolved_path,
            lock_digest,
            candidate,
            version,
        )?;
    }
    Err(ReadinessError::ArtifactCannotProveBinding { key: first_untyped })
}

fn validate_candidate_state(phase: &Value, candidate: &str) -> Result<(), ReadinessError> {
    let states = object_field(phase, "candidateStates")?;
    let other = match candidate {
        "focused" => "integrated",
        "integrated" => "focused",
        _ => return fail("promotion-candidate"),
    };
    if string_from(states, candidate)? != "selected" || string_from(states, other)? != "removed" {
        return fail("promotion-candidate-state");
    }
    Ok(())
}

fn validate_selection_evidence(
    root: &Path,
    registry: &SchemaRegistry,
    selection: &Value,
    lock: &Value,
    lock_digest: &str,
    selected_candidate: &str,
    selected_reference: &Value,
) -> Result<(), ReadinessError> {
    let evidence = object_field(selection, "candidateEvidence")?;
    let eligibility = object_field(selection, "eligibility")?;
    for candidate in ["focused", "integrated"] {
        let reference = object_value(evidence, candidate)?;
        let verified = digests::verify_value_reference(root, reference)?;
        let value = read_json(&verified.resolved_path)?;
        digests::verify_references_in_value(root, &value)?;
        validate_schema(
            registry,
            EVIDENCE_SCHEMA,
            &value,
            "selection-candidate-evidence",
        )?;
        validate_evidence_source_identity(&value, lock, candidate)?;
        require_equal(
            string_field(&value, "lockDigest")?,
            lock_digest,
            "selection-evidence-lock-digest",
        )?;
        require_equal(
            string_field(&value, "candidate")?,
            candidate,
            "selection-evidence-candidate",
        )?;
        require_equal(
            string_field(&value, "eligibility")?,
            string_from(eligibility, candidate)?,
            "selection-evidence-eligibility",
        )?;
        if candidate == selected_candidate && !same_reference(reference, selected_reference)? {
            return fail("selection-selected-evidence");
        }
    }
    Ok(())
}

fn validate_qualification_evidence(
    root: &Path,
    registry: &SchemaRegistry,
    reference: &VerifiedReference,
    lock: &Value,
    lock_digest: &str,
    candidate: &str,
    family: &'static str,
) -> Result<(), ReadinessError> {
    let evidence = read_json(&reference.resolved_path)?;
    digests::verify_references_in_value(root, &evidence)?;
    validate_schema(registry, EVIDENCE_SCHEMA, &evidence, family)?;
    validate_evidence_source_identity(&evidence, lock, candidate)?;
    require_equal(
        string_field(&evidence, "lockDigest")?,
        lock_digest,
        "qualification-evidence-lock-digest",
    )?;
    require_equal(
        string_field(&evidence, "candidate")?,
        candidate,
        "qualification-evidence-candidate",
    )?;
    if string_field(&evidence, "eligibility")? != "eligible" {
        return fail("qualification-evidence-eligibility");
    }
    Ok(())
}

fn validate_evidence_source_identity(
    evidence: &Value,
    lock: &Value,
    candidate: &str,
) -> Result<(), ReadinessError> {
    let source = object_field(evidence, "source")?;
    let pins = object_field(lock, "sourcePins")?;
    for (source_field, pin_field) in [
        ("flutterFrameworkCommit", "flutterFramework"),
        ("flutterEngineCommit", "flutterEngine"),
        ("adapterCommit", "oxyflutAdapter"),
    ] {
        require_equal(
            string_from(source, source_field)?,
            string_field(object_value(pins, pin_field)?, "commit")?,
            "qualification-evidence-source-identity",
        )?;
    }
    if candidate == "integrated" {
        require_equal(
            string_from(source, "candidateCommit")?,
            string_field(object_value(pins, "integratedFork")?, "commit")?,
            "qualification-evidence-source-identity",
        )?;
    }
    Ok(())
}

fn validate_exposed_untyped_identity(
    root: &Path,
    path: &Path,
    lock_digest: &str,
    candidate: &str,
    version: &str,
) -> Result<(), ReadinessError> {
    if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
        return Ok(());
    }
    let value = read_json(path)?;
    digests::verify_references_in_value(root, &value)?;
    let Some(object) = value.as_object() else {
        return Ok(());
    };
    for field in ["qualificationLockDigest", "lockDigest"] {
        if let Some(value) = object.get(field) {
            require_equal(
                value
                    .as_str()
                    .ok_or_else(|| invariant("promotion-artifact-lock-digest"))?,
                lock_digest,
                "promotion-artifact-lock-digest",
            )?;
        }
    }
    for field in ["candidate", "selectedCandidate"] {
        if let Some(value) = object.get(field) {
            require_equal(
                value
                    .as_str()
                    .ok_or_else(|| invariant("promotion-artifact-candidate"))?,
                candidate,
                "promotion-artifact-candidate",
            )?;
        }
    }
    if let Some(value) = object.get("specificationVersion") {
        require_equal(
            value
                .as_str()
                .ok_or_else(|| invariant("promotion-artifact-specification-version"))?,
            version,
            "promotion-artifact-specification-version",
        )?;
    }
    Ok(())
}

fn promotion_reference<'a>(
    references: &'a [(&'static str, &'a Value, VerifiedReference)],
    key: &'static str,
) -> Result<&'a (&'static str, &'a Value, VerifiedReference), ReadinessError> {
    references
        .iter()
        .find(|(current, _, _)| *current == key)
        .ok_or_else(|| invariant("promotion-reference"))
}
