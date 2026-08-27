use std::fs;
use std::path::Path;

use oxyflut_qualification::schema::SchemaRegistry;
use serde_json::Value;

use crate::contracts::{
    digests::{self, VerifiedReference},
    traceability,
};

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

struct PromotionBinding<'a> {
    root: &'a Path,
    registry: &'a SchemaRegistry,
    lock: &'a Value,
    lock_digest: &'a str,
    candidate: &'a str,
    phase: &'a Value,
}

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
    let binding = PromotionBinding {
        root,
        registry,
        lock,
        lock_digest,
        candidate,
        phase,
    };

    let mut references = Vec::with_capacity(PROMOTION_KEYS.len());
    for key in PROMOTION_KEYS {
        let reference = object_value(promotion, key)?;
        let verified = digests::verify_value_reference(root, reference)?;
        references.push((key, reference, verified));
    }

    let selection = promotion_reference(&references, "selectionDecision")?;
    let selection_value = read_json(&selection.2.resolved_path)?;
    digests::verify_references_for_schema(root, SELECTION_SCHEMA, &selection_value)?;
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
    validate_selection_evidence(&binding, &selection_value, selected.1)?;
    validate_selection_consistency(root, &selection_value)?;
    validate_qualification_evidence(&binding, &selected.2, "selected-candidate-qualification")?;
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
    validate_qualification_evidence(&binding, &all_tier_one.2, "all-tier-one-results")?;

    let release = promotion_reference(&references, "releaseQualification")?;
    let release_value = read_json(&release.2.resolved_path)?;
    digests::verify_references_for_schema(root, RELEASE_SCHEMA, &release_value)?;
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
    let adr_text =
        std::str::from_utf8(&adr_bytes).map_err(|_| invariant("accepted-adr-content"))?;
    if !adr_has_accepted_status(adr_text)
        || !adr_selects_candidate(adr_text, candidate)
        || !adr_cites_verified_evidence(root, adr_text)
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

fn adr_has_accepted_status(adr: &str) -> bool {
    adr.lines()
        .any(|line| line.trim() == "**Status:** accepted")
}

fn adr_selects_candidate(adr: &str, candidate: &str) -> bool {
    let expected = format!("Selected candidate: {candidate}.");
    let mut in_decision = false;
    let mut selections = 0_u8;
    for line in adr.lines() {
        let line = line.trim();
        if line == "## Decision" {
            in_decision = true;
            continue;
        }
        if in_decision && line.starts_with("## ") {
            break;
        }
        if in_decision && line.starts_with("Selected candidate:") {
            if line != expected {
                return false;
            }
            selections = selections.saturating_add(1);
        }
    }
    selections == 1
}

fn adr_cites_verified_evidence(root: &Path, adr: &str) -> bool {
    for line in adr.lines() {
        let digests = line
            .split(|character: char| !character.is_ascii_hexdigit())
            .filter(|value| {
                value.len() == 64
                    && value
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
            })
            .collect::<Vec<_>>();
        if digests.is_empty() {
            continue;
        }
        let mut fragments = line.split('`');
        let _ = fragments.next();
        while let Some(path) = fragments.next() {
            let _ = fragments.next();
            if !path.starts_with("evidence/") {
                continue;
            }
            if digests
                .iter()
                .any(|digest| digests::verify_reference(root, path, digest).is_ok())
            {
                return true;
            }
        }
    }
    false
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
    binding: &PromotionBinding<'_>,
    selection: &Value,
    selected_reference: &Value,
) -> Result<(), ReadinessError> {
    let evidence = object_field(selection, "candidateEvidence")?;
    let eligibility = object_field(selection, "eligibility")?;
    for candidate in ["focused", "integrated"] {
        let reference = object_value(evidence, candidate)?;
        let verified = digests::verify_value_reference(binding.root, reference)?;
        let value = read_json(&verified.resolved_path)?;
        digests::verify_references_for_schema(binding.root, EVIDENCE_SCHEMA, &value)?;
        validate_schema(
            binding.registry,
            EVIDENCE_SCHEMA,
            &value,
            "selection-candidate-evidence",
        )?;
        traceability::validate_promotion_qualification_evidence(
            binding.root,
            &value,
            binding.phase,
        )?;
        validate_evidence_source_identity(&value, binding.lock, candidate)?;
        require_equal(
            string_field(&value, "lockDigest")?,
            binding.lock_digest,
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
        if candidate == binding.candidate && !same_reference(reference, selected_reference)? {
            return fail("selection-selected-evidence");
        }
    }
    Ok(())
}

fn validate_qualification_evidence(
    binding: &PromotionBinding<'_>,
    reference: &VerifiedReference,
    family: &'static str,
) -> Result<(), ReadinessError> {
    let evidence = read_json(&reference.resolved_path)?;
    digests::verify_references_for_schema(binding.root, EVIDENCE_SCHEMA, &evidence)?;
    validate_schema(binding.registry, EVIDENCE_SCHEMA, &evidence, family)?;
    traceability::validate_promotion_qualification_evidence(
        binding.root,
        &evidence,
        binding.phase,
    )?;
    validate_evidence_source_identity(&evidence, binding.lock, binding.candidate)?;
    require_equal(
        string_field(&evidence, "lockDigest")?,
        binding.lock_digest,
        "qualification-evidence-lock-digest",
    )?;
    require_equal(
        string_field(&evidence, "candidate")?,
        binding.candidate,
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

/// Recomputes a schema-valid selection decision from its immutable candidate evidence.
///
/// # Errors
///
/// Returns an error when eligibility, weighted totals, the margin winner, or the maintenance tie-break differs from the cited evidence.
pub(super) fn validate_selection_consistency(
    root: &Path,
    selection: &Value,
) -> Result<(), ReadinessError> {
    let eligibility = object_field(selection, "eligibility")?;
    let focused_eligible = selection_eligibility(eligibility, "focused")?;
    let integrated_eligible = selection_eligibility(eligibility, "integrated")?;
    let calculation = object_field(selection, "calculation")?;

    match (focused_eligible, integrated_eligible) {
        (false, false) => {
            require_selection_decision(
                selection,
                calculation,
                "no-eligible-candidate",
                "reopen-research",
                "none",
                false,
            )?;
            require_null_calculation(calculation, "focusedScore")?;
            require_null_calculation(calculation, "integratedScore")?;
            require_null_calculation(calculation, "absoluteDifference")?;
            require_no_maintenance_evidence(calculation)
        }
        (true, false) => {
            require_selection_decision(
                selection,
                calculation,
                "sole-focused-eligible",
                "select-focused",
                "focused",
                false,
            )?;
            require_null_calculation(calculation, "focusedScore")?;
            require_null_calculation(calculation, "integratedScore")?;
            require_null_calculation(calculation, "absoluteDifference")?;
            require_no_maintenance_evidence(calculation)
        }
        (false, true) => {
            require_selection_decision(
                selection,
                calculation,
                "sole-integrated-eligible",
                "select-integrated",
                "integrated",
                false,
            )?;
            require_null_calculation(calculation, "focusedScore")?;
            require_null_calculation(calculation, "integratedScore")?;
            require_null_calculation(calculation, "absoluteDifference")?;
            require_no_maintenance_evidence(calculation)
        }
        (true, true) => validate_two_eligible_selection(root, selection, calculation),
    }
}

fn selection_eligibility(
    eligibility: &serde_json::Map<String, Value>,
    candidate: &str,
) -> Result<bool, ReadinessError> {
    match string_from(eligibility, candidate)? {
        "eligible" => Ok(true),
        "ineligible" => Ok(false),
        _ => fail("selection-eligibility"),
    }
}

fn validate_two_eligible_selection(
    root: &Path,
    selection: &Value,
    calculation: &serde_json::Map<String, Value>,
) -> Result<(), ReadinessError> {
    let evidence = object_field(selection, "candidateEvidence")?;
    let focused = read_selection_evidence(root, object_value(evidence, "focused")?)?;
    let integrated = read_selection_evidence(root, object_value(evidence, "integrated")?)?;
    let focused_total = weighted_total(&focused)?;
    let integrated_total = weighted_total(&integrated)?;
    let difference = focused_total.abs_diff(integrated_total);
    require_calculation_score(calculation, "focusedScore", focused_total)?;
    require_calculation_score(calculation, "integratedScore", integrated_total)?;
    require_calculation_score(calculation, "absoluteDifference", difference)?;

    if difference >= 25 {
        let (outcome, selected) = if focused_total > integrated_total {
            ("select-focused", "focused")
        } else {
            ("select-integrated", "integrated")
        };
        require_selection_decision(
            selection,
            calculation,
            "score-margin",
            outcome,
            selected,
            false,
        )?;
        return require_no_maintenance_evidence(calculation);
    }

    let maintenance = object_value(calculation, "maintenanceEvidence")?;
    let _ = digests::verify_value_reference(root, maintenance)?;
    if !maintenance_evidence_is_cited(&focused, maintenance)?
        && !maintenance_evidence_is_cited(&integrated, maintenance)?
    {
        return fail("selection-maintenance-evidence");
    }
    let focused_maintenance = maintenance_score(&focused)?;
    let integrated_maintenance = maintenance_score(&integrated)?;
    if focused_maintenance == integrated_maintenance {
        require_selection_decision(
            selection,
            calculation,
            "inconclusive-tie-break",
            "continue-investigation",
            "none",
            true,
        )
    } else {
        let (outcome, selected) = if focused_maintenance > integrated_maintenance {
            ("select-focused", "focused")
        } else {
            ("select-integrated", "integrated")
        };
        require_selection_decision(
            selection,
            calculation,
            "maintenance-tie-break",
            outcome,
            selected,
            true,
        )
    }
}

fn read_selection_evidence(root: &Path, reference: &Value) -> Result<Value, ReadinessError> {
    let verified = digests::verify_value_reference(root, reference)?;
    read_json(&verified.resolved_path)
}

fn weighted_total(evidence: &Value) -> Result<u32, ReadinessError> {
    let scores = object_field(evidence, "scores")?;
    let mut total = 0_u32;
    for (criterion, expected_weight) in [
        ("platformCoverage", 30_u32),
        ("upgradeMaintenance", 20),
        ("performance", 15),
        ("safetySecurityPrivacy", 15),
        ("distribution", 10),
        ("operationalClarity", 10),
    ] {
        let score = object_value(scores, criterion)?;
        let weight = u32::try_from(
            score
                .get("weight")
                .and_then(Value::as_u64)
                .ok_or_else(|| invariant("selection-score"))?,
        )
        .map_err(|_| invariant("selection-score"))?;
        let consensus = u32::try_from(
            score
                .get("consensusScore")
                .and_then(Value::as_u64)
                .ok_or_else(|| invariant("selection-score"))?,
        )
        .map_err(|_| invariant("selection-score"))?;
        if weight != expected_weight || !(3..=5).contains(&consensus) {
            return fail("selection-score");
        }
        let weighted = weight
            .checked_mul(consensus)
            .ok_or_else(|| invariant("selection-score"))?;
        total = total
            .checked_add(weighted)
            .ok_or_else(|| invariant("selection-score"))?;
    }
    let declared = evidence
        .get("weightedTotal")
        .and_then(Value::as_f64)
        .ok_or_else(|| invariant("selection-weighted-total"))?;
    let expected = f64::from(total) / 5.0;
    if (declared - expected).abs() > f64::EPSILON {
        return fail("selection-weighted-total");
    }
    Ok(total)
}

fn maintenance_score(evidence: &Value) -> Result<u32, ReadinessError> {
    let score = object_value(object_field(evidence, "scores")?, "upgradeMaintenance")?;
    u32::try_from(
        score
            .get("consensusScore")
            .and_then(Value::as_u64)
            .ok_or_else(|| invariant("selection-maintenance-score"))?,
    )
    .map_err(|_| invariant("selection-maintenance-score"))
}

fn maintenance_evidence_is_cited(
    candidate: &Value,
    maintenance: &Value,
) -> Result<bool, ReadinessError> {
    let score = object_value(object_field(candidate, "scores")?, "upgradeMaintenance")?;
    let evidence = score
        .get("evidence")
        .and_then(Value::as_array)
        .ok_or_else(|| invariant("selection-maintenance-evidence"))?;
    for reference in evidence {
        if same_reference(reference, maintenance)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn require_selection_decision(
    selection: &Value,
    calculation: &serde_json::Map<String, Value>,
    basis: &str,
    outcome: &str,
    candidate: &str,
    tie_break_applied: bool,
) -> Result<(), ReadinessError> {
    require_equal(
        string_field(selection, "decisionBasis")?,
        basis,
        "selection-decision-basis",
    )?;
    require_equal(
        string_field(selection, "selectedCandidate")?,
        candidate,
        "selection-selected-candidate",
    )?;
    require_equal(
        string_field(selection, "outcome")?,
        outcome,
        "selection-outcome",
    )?;
    if calculation.get("tieBreakApplied").and_then(Value::as_bool) != Some(tie_break_applied) {
        return fail("selection-tie-break");
    }
    Ok(())
}

fn require_null_calculation(
    calculation: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<(), ReadinessError> {
    if calculation.get(field).is_some_and(Value::is_null) {
        Ok(())
    } else {
        fail("selection-calculation")
    }
}

fn require_calculation_score(
    calculation: &serde_json::Map<String, Value>,
    field: &str,
    fifths: u32,
) -> Result<(), ReadinessError> {
    let actual = calculation
        .get(field)
        .and_then(Value::as_f64)
        .ok_or_else(|| invariant("selection-calculation"))?;
    let expected = f64::from(fifths) / 5.0;
    if (actual - expected).abs() > f64::EPSILON {
        return fail(match field {
            "focusedScore" => "selection-focused-score",
            "integratedScore" => "selection-integrated-score",
            "absoluteDifference" => "selection-absolute-difference",
            _ => "selection-calculation",
        });
    }
    Ok(())
}

fn require_no_maintenance_evidence(
    calculation: &serde_json::Map<String, Value>,
) -> Result<(), ReadinessError> {
    if calculation.contains_key("maintenanceEvidence") {
        fail("selection-maintenance-evidence")
    } else {
        Ok(())
    }
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
