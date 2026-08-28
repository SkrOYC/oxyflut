use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use oxyflut_qualification::hash::hash_file;
use serde_json::{Value, json};

use super::super::{digests::DigestError, schema, traceability};
use super::{
    GateStatus, PromotionStatus, ReadinessError, read_json, validate_documents,
    validate_platform_value,
};

#[test]
fn committed_phase_three_a_lock_is_valid_but_both_gates_remain_open() -> Result<(), Box<dyn Error>>
{
    let root = workspace_root()?;
    let registry = schema::compile_workspace(&root)?;
    let report = validate_documents(
        &root,
        &read_json(&root.join(super::LOCK_PATH))?,
        &read_json(&root.join(super::PHASE_PATH))?,
        &registry,
    )?;
    assert!(matches!(
        report.candidate_implementation,
        GateStatus::Open(_)
    ));
    assert!(matches!(report.measurement, GateStatus::Open(_)));
    assert_eq!(report.promotion, PromotionStatus::NotClaimed);
    Ok(())
}

#[test]
fn readiness_fixtures_fail_closed_for_unresolved_inputs_and_baseline_bindings()
-> Result<(), Box<dyn Error>> {
    let root = ready_fixture_root()?;
    let registry = schema::compile_workspace(&workspace_root()?)?;
    let phase = read_json(&root.join(super::PHASE_PATH))?;
    let unresolved = validate_documents(
        &root,
        &read_json(&root.join("negative/unresolved-readiness-lock.json"))?,
        &phase,
        &registry,
    );
    assert!(matches!(
        unresolved,
        Err(ReadinessError::InvalidClaim {
            gate: "candidate-implementation"
        })
    ));
    let synthetic = validate_documents(
        &root,
        &read_json(&root.join("negative/synthetic-baseline-lock.json"))?,
        &phase,
        &registry,
    );
    assert!(matches!(
        synthetic,
        Err(ReadinessError::Invariant {
            code: "capability-baseline-provenance"
        })
    ));
    let mismatched = validate_documents(
        &root,
        &read_json(&root.join("negative/mismatched-typed-reference-lock.json"))?,
        &phase,
        &registry,
    );
    assert!(matches!(
        mismatched,
        Err(ReadinessError::Invariant {
            code: "capability-baseline-approval-reference"
        })
    ));
    Ok(())
}

#[test]
fn nested_kk_claim_without_digest_bound_evidence_fails_closed() -> Result<(), Box<dyn Error>> {
    let root = ready_fixture_root()?;
    let mut issues = Vec::new();
    let result = validate_platform_value(
        &root,
        &read_json(&root.join("negative/missing-nested-kk-platform.json"))?,
        &mut issues,
    );
    assert!(matches!(
        result,
        Err(ReadinessError::Invariant {
            code: "platform-claim-evidence"
        })
    ));
    Ok(())
}

#[test]
fn schema_valid_platform_containers_validate_their_nested_kk_claims() -> Result<(), Box<dyn Error>>
{
    let root = ready_fixture_root()?;
    let registry = schema::compile_workspace(&workspace_root()?)?;
    let platform = read_json(&root.join(super::PLATFORM_CONTRACTS_PATH))?;
    registry.validate(super::PLATFORM_SCHEMA, &platform)?;

    let mut issues = Vec::new();
    validate_platform_value(&root, &platform, &mut issues)?;
    assert!(issues.is_empty());
    Ok(())
}

#[test]
fn ready_fixture_exercises_candidate_and_measurement_true_paths() -> Result<(), Box<dyn Error>> {
    let root = ready_fixture_root()?;
    let registry = schema::compile_workspace(&workspace_root()?)?;
    let report = validate_documents(
        &root,
        &read_json(&root.join(super::LOCK_PATH))?,
        &read_json(&root.join(super::PHASE_PATH))?,
        &registry,
    )?;
    assert_eq!(report.candidate_implementation, GateStatus::Ready);
    assert_eq!(report.measurement, GateStatus::Ready);
    Ok(())
}

#[test]
fn contracts_validate_readiness_family_rejects_all_relative_resolved_tools()
-> Result<(), Box<dyn Error>> {
    let workspace = workspace_root()?;
    let root = temporary_fixture_root("all-relative-resolved-tools");
    copy_directory(
        &workspace.join(".constitution"),
        &root.join(".constitution"),
    )?;
    copy_directory(
        &workspace.join("qualification"),
        &root.join("qualification"),
    )?;

    let relative_path = "qualification/fixtures/readiness/relative-tool";
    let tool_path = root.join(relative_path);
    let parent = tool_path
        .parent()
        .ok_or("relative tool fixture must have a parent")?;
    fs::create_dir_all(parent)?;
    fs::write(&tool_path, b"digest-valid relative tool")?;
    let mut lock = read_json(&root.join(super::LOCK_PATH))?;
    *lock
        .get_mut("resolvedTools")
        .ok_or("lock must contain resolved tools")? = json!([{
        "name": "c-compiler",
        "version": "fixture-version",
        "sourceIdentity": "fixture-source",
        "hostTriple": "x86_64-unknown-linux-gnu",
        "licenseId": "MIT",
        "executablePath": relative_path,
        "sha256": hash_file(&tool_path)?.to_string(),
    }]);
    let registry = schema::compile_workspace(&workspace)?;
    let result = validate_documents(
        &root,
        &lock,
        &read_json(&root.join(super::PHASE_PATH))?,
        &registry,
    );
    fs::remove_dir_all(&root)?;

    assert!(matches!(
        result,
        Err(ReadinessError::Invariant {
            code: "resolved-tool-manifest"
        })
    ));
    Ok(())
}

#[test]
fn candidate_artifact_source_revisions_and_resolved_tools_fail_closed() -> Result<(), Box<dyn Error>>
{
    let root = ready_fixture_root()?;
    let registry = schema::compile_workspace(&workspace_root()?)?;
    let phase = read_json(&root.join(super::PHASE_PATH))?;

    let unresolved_tool_fields = validate_documents(
        &root,
        &read_json(&root.join("negative/unresolved-tool-fields-lock.json"))?,
        &phase,
        &registry,
    )?;
    let GateStatus::Open(issues) = unresolved_tool_fields.candidate_implementation else {
        return Err("unresolved tool fields must keep the candidate gate open".into());
    };
    assert!(issues.contains(&"resolved-tool".to_owned()));
    assert!(issues.contains(&"resolved-tool-digests".to_owned()));

    let wrong_engine = validate_documents(
        &root,
        &read_json(&root.join("negative/mismatched-engine-revision-lock.json"))?,
        &phase,
        &registry,
    );
    assert!(matches!(
        wrong_engine,
        Err(ReadinessError::Invariant {
            code: "candidate-artifact-source-revision"
        })
    ));

    let missing_tool = validate_documents(
        &root,
        &read_json(&root.join("negative/missing-tool-lock.json"))?,
        &phase,
        &registry,
    );
    assert!(matches!(
        missing_tool,
        Err(ReadinessError::Digest(DigestError::MissingFile { .. }))
    ));

    let wrong_tool = validate_documents(
        &root,
        &read_json(&root.join("negative/mismatched-tool-lock.json"))?,
        &phase,
        &registry,
    );
    assert!(matches!(
        wrong_tool,
        Err(ReadinessError::Digest(DigestError::DigestMismatch { .. }))
    ));
    Ok(())
}

#[test]
fn nonempty_resolved_tools_must_match_the_staged_manifest() -> Result<(), Box<dyn Error>> {
    if !crate::toolchain::is_staged_host()? {
        return Ok(());
    }
    let root = workspace_root()?;
    let manifest = crate::toolchain::ToolchainManifest::from_json(&fs::read(
        root.join("qualification/tools/native-contract-toolchain.json"),
    )?)?;
    let tools = crate::toolchain::lock::lock_resolved_tools(&manifest)?;
    super::verify_resolved_tools(&root, &tools)?;

    let mut altered = tools;
    let tool = altered
        .first_mut()
        .and_then(Value::as_object_mut)
        .ok_or("complete fixture must contain a tool object")?;
    *tool
        .get_mut("sha256")
        .ok_or("complete fixture must bind a tool digest")? = Value::String("0".repeat(64));
    assert!(matches!(
        super::verify_resolved_tools(&root, &altered),
        Err(ReadinessError::Invariant {
            code: "resolved-tool-manifest"
        })
    ));
    Ok(())
}

#[test]
fn external_snapshots_and_platform_versions_fail_closed() -> Result<(), Box<dyn Error>> {
    let workspace = workspace_root()?;
    let registry = schema::compile_workspace(&workspace)?;
    let root = temporary_fixture_root("external-platform");
    copy_directory(&ready_fixture_root()?, &root)?;
    let phase = read_json(&root.join(super::PHASE_PATH))?;

    fs::copy(
        root.join("negative/mismatched-external-contract-lock.json"),
        root.join(super::EXTERNAL_CONTRACT_LOCK_PATH),
    )?;
    let mut mismatched_lock = read_json(&root.join(super::LOCK_PATH))?;
    let mismatched_digest = hash_file(&root.join(super::EXTERNAL_CONTRACT_LOCK_PATH))?;
    let mismatched_field = mismatched_lock
        .pointer_mut("/measurementPolicy/externalContractLock")
        .ok_or("lock must include the external contract lock digest")?;
    *mismatched_field = Value::String(mismatched_digest.to_string());
    assert!(matches!(
        super::candidate_input_issues(&root, &mismatched_lock, "0.15.0", &registry),
        Err(ReadinessError::Digest(DigestError::DigestMismatch { .. }))
    ));

    fs::copy(
        root.join("negative/unresolved-external-contract-lock.json"),
        root.join(super::EXTERNAL_CONTRACT_LOCK_PATH),
    )?;
    let mut lock = read_json(&root.join(super::LOCK_PATH))?;
    let external_digest = hash_file(&root.join(super::EXTERNAL_CONTRACT_LOCK_PATH))?;
    let external_field = lock
        .pointer_mut("/measurementPolicy/externalContractLock")
        .ok_or("lock must include the external contract lock digest")?;
    *external_field = Value::String(external_digest.to_string());
    let issues = super::candidate_input_issues(&root, &lock, "0.15.0", &registry)?;
    assert_eq!(issues, vec!["external-contract-known-unknown"]);
    assert!(matches!(
        validate_documents(&root, &lock, &phase, &registry),
        Err(ReadinessError::InvalidClaim {
            gate: "candidate-implementation"
        })
    ));

    copy_directory(&ready_fixture_root()?, &root)?;
    fs::copy(
        root.join("negative/stale-platform-contracts.json"),
        root.join(super::PLATFORM_CONTRACTS_PATH),
    )?;
    let mut lock = read_json(&root.join(super::LOCK_PATH))?;
    let platform_digest = hash_file(&root.join(super::PLATFORM_CONTRACTS_PATH))?;
    let platform_field = lock
        .pointer_mut("/measurementPolicy/platformContracts")
        .ok_or("lock must include the platform contract digest")?;
    *platform_field = Value::String(platform_digest.to_string());
    let result = validate_documents(&root, &lock, &phase, &registry);
    fs::remove_dir_all(&root)?;
    assert!(matches!(
        result,
        Err(ReadinessError::Invariant {
            code: "platform-contracts-specification-version"
        })
    ));
    Ok(())
}

#[test]
fn measurement_rejects_missing_final_candidate_source_identities() -> Result<(), Box<dyn Error>> {
    let root = ready_fixture_root()?;
    let registry = schema::compile_workspace(&workspace_root()?)?;
    let result = validate_documents(
        &root,
        &read_json(&root.join("negative/missing-candidate-identities-lock.json"))?,
        &read_json(&root.join(super::PHASE_PATH))?,
        &registry,
    );
    assert!(matches!(
        result,
        Err(ReadinessError::InvalidClaim {
            gate: "measurement"
        })
    ));
    Ok(())
}

#[test]
fn production_fixture_resolves_typed_artifacts_then_rejects_untyped_promotion_artifacts()
-> Result<(), Box<dyn Error>> {
    let root = production_fixture_root()?;
    let registry = schema::compile_workspace(&workspace_root()?)?;
    let result = validate_documents(
        &root,
        &read_json(&root.join(super::LOCK_PATH))?,
        &read_json(&root.join("production-3b-phase.json"))?,
        &registry,
    );
    assert!(
        matches!(
            result,
            Err(ReadinessError::ArtifactCannotProveBinding {
                key: "layoutQualification"
            })
        ),
        "{result:?}"
    );
    Ok(())
}

#[test]
fn promotion_qualification_evidence_reuses_the_exact_semantic_validator()
-> Result<(), Box<dyn Error>> {
    let root = production_fixture_root()?;
    let phase = read_json(&root.join("production-3b-phase.json"))?;
    let fabricated =
        read_json(&root.join("negative/fabricated-not-applicable-qualification.json"))?;
    let result =
        traceability::validate_promotion_qualification_evidence(&root, &fabricated, &phase);
    assert!(matches!(
        result,
        Err(traceability::TraceabilityError::Invariant {
            code: "absent-event-gate"
        })
    ));
    Ok(())
}

#[test]
fn fabricated_promotion_fixtures_reject_lock_candidate_version_selection_and_missing_artifacts()
-> Result<(), Box<dyn Error>> {
    let root = production_fixture_root()?;
    let registry = schema::compile_workspace(&workspace_root()?)?;
    for fixture in [
        "negative/promotion-wrong-lock-phase.json",
        "negative/promotion-wrong-candidate-phase.json",
        "negative/promotion-wrong-version-phase.json",
        "negative/promotion-untyped-wrong-lock-phase.json",
        "negative/promotion-selection-lower-score-phase.json",
        "negative/promotion-missing-artifact-phase.json",
    ] {
        let result = validate_documents(
            &root,
            &read_json(&root.join(super::LOCK_PATH))?,
            &read_json(&root.join(fixture))?,
            &registry,
        );
        match fixture {
            "negative/promotion-wrong-lock-phase.json" => assert!(matches!(
                result,
                Err(ReadinessError::Digest(DigestError::DigestMismatch { .. }))
            )),
            "negative/promotion-wrong-candidate-phase.json" => assert!(matches!(
                result,
                Err(ReadinessError::Invariant {
                    code: "selection-candidate"
                })
            )),
            "negative/promotion-wrong-version-phase.json" => assert!(matches!(
                result,
                Err(ReadinessError::Invariant {
                    code: "capability-baseline-version"
                })
            )),
            "negative/promotion-untyped-wrong-lock-phase.json" => assert!(matches!(
                result,
                Err(ReadinessError::Invariant {
                    code: "promotion-artifact-lock-digest"
                })
            )),
            "negative/promotion-selection-lower-score-phase.json" => assert!(matches!(
                result,
                Err(ReadinessError::Invariant {
                    code: "selection-selected-candidate"
                })
            )),
            "negative/promotion-missing-artifact-phase.json" => assert!(matches!(
                result,
                Err(ReadinessError::Digest(DigestError::MissingFile { .. }))
            )),
            _ => return Err("fixture must have a declared expected failure".into()),
        };
    }
    Ok(())
}

#[test]
fn tampered_canonical_adr_bytes_fail_closed() -> Result<(), Box<dyn Error>> {
    let workspace = workspace_root()?;
    let root = temporary_fixture_root("tampered-adr");
    copy_directory(&production_fixture_root()?, &root)?;
    fs::copy(
        workspace.join("qualification/fixtures/contracts/readiness/tampered-adr.md"),
        root.join(".constitution/tech-spec/adrs/ADR-0010-production-substrate.md"),
    )?;
    let mut phase = read_json(&root.join("production-3b-phase.json"))?;
    let digest =
        hash_file(&root.join(".constitution/tech-spec/adrs/ADR-0010-production-substrate.md"))?;
    let declared = phase
        .pointer_mut("/promotionEvidence/acceptedAdr0010/sha256")
        .ok_or("fixture must include the accepted ADR digest")?;
    *declared = Value::String(digest.to_string());
    fs::write(
        root.join("production-3b-phase.json"),
        format!("{}\n", serde_json::to_string_pretty(&phase)?),
    )?;

    let registry = schema::compile_workspace(&workspace)?;
    let result = validate_documents(
        &root,
        &read_json(&root.join(super::LOCK_PATH))?,
        &phase,
        &registry,
    );
    fs::remove_dir_all(&root)?;
    assert!(matches!(
        result,
        Err(ReadinessError::Invariant {
            code: "accepted-adr-content"
        })
    ));
    Ok(())
}

#[test]
fn selection_recomputes_score_margin_and_maintenance_tie_breaks() -> Result<(), Box<dyn Error>> {
    let root = temporary_fixture_root("selection-consistency");
    fs::create_dir_all(root.join("evidence"))?;
    let proof_path = root.join("evidence/proof.txt");
    fs::write(&proof_path, b"immutable maintenance proof")?;
    let proof = evidence_reference("evidence/proof.txt", &hash_file(&proof_path)?.to_string());

    let focused = selection_evidence([5, 5, 5, 5, 5, 5], proof.clone());
    let integrated = selection_evidence([4, 4, 4, 4, 4, 4], proof.clone());
    let focused_reference = write_selection_evidence(&root, "focused.json", &focused)?;
    let integrated_reference = write_selection_evidence(&root, "integrated.json", &integrated)?;
    let margin = two_candidate_selection(
        focused_reference.clone(),
        integrated_reference.clone(),
        100.0,
        80.0,
        20.0,
        "score-margin",
        "select-focused",
        "focused",
        false,
        None,
    );
    super::promotion::validate_selection_consistency(&root, &margin)?;

    let mut lower_scoring_selected = margin;
    *lower_scoring_selected
        .pointer_mut("/outcome")
        .ok_or("selection outcome must exist")? = Value::String("select-integrated".to_owned());
    *lower_scoring_selected
        .pointer_mut("/selectedCandidate")
        .ok_or("selection candidate must exist")? = Value::String("integrated".to_owned());
    assert!(matches!(
        super::promotion::validate_selection_consistency(&root, &lower_scoring_selected),
        Err(ReadinessError::Invariant {
            code: "selection-selected-candidate"
        })
    ));

    let near_tie = selection_evidence([5, 4, 5, 5, 5, 5], proof.clone());
    let near_tie_reference = write_selection_evidence(&root, "near-tie.json", &near_tie)?;
    let maintenance_tie_break = two_candidate_selection(
        focused_reference.clone(),
        near_tie_reference,
        100.0,
        96.0,
        4.0,
        "maintenance-tie-break",
        "select-focused",
        "focused",
        true,
        Some(proof.clone()),
    );
    super::promotion::validate_selection_consistency(&root, &maintenance_tie_break)?;

    let equal_reference = write_selection_evidence(&root, "equal.json", &focused)?;
    let inconclusive = two_candidate_selection(
        focused_reference,
        equal_reference,
        100.0,
        100.0,
        0.0,
        "inconclusive-tie-break",
        "continue-investigation",
        "none",
        true,
        Some(proof),
    );
    let result = super::promotion::validate_selection_consistency(&root, &inconclusive);
    fs::remove_dir_all(&root)?;
    result?;
    Ok(())
}

fn selection_evidence(scores: [u32; 6], maintenance_evidence: Value) -> Value {
    let weighted_total = [30_u32, 20, 15, 15, 10, 10]
        .into_iter()
        .zip(scores)
        .map(|(weight, score)| weight * score)
        .sum::<u32>();
    let score = |weight: u32, consensus: u32| {
        json!({
            "weight": weight,
            "consensusScore": consensus,
            "evidence": [maintenance_evidence.clone()]
        })
    };
    json!({
        "scores": {
            "platformCoverage": score(30, scores[0]),
            "upgradeMaintenance": score(20, scores[1]),
            "performance": score(15, scores[2]),
            "safetySecurityPrivacy": score(15, scores[3]),
            "distribution": score(10, scores[4]),
            "operationalClarity": score(10, scores[5])
        },
        "weightedTotal": f64::from(weighted_total) / 5.0
    })
}

fn evidence_reference(path: &str, sha256: &str) -> Value {
    json!({"path": path, "sha256": sha256})
}

fn write_selection_evidence(
    root: &Path,
    name: &str,
    evidence: &Value,
) -> Result<Value, Box<dyn Error>> {
    let path = root.join("evidence").join(name);
    fs::write(&path, format!("{}\n", serde_json::to_string(evidence)?))?;
    Ok(evidence_reference(
        &format!("evidence/{name}"),
        &hash_file(&path)?.to_string(),
    ))
}

#[allow(clippy::too_many_arguments)]
fn two_candidate_selection(
    focused: Value,
    integrated: Value,
    focused_score: f64,
    integrated_score: f64,
    difference: f64,
    decision_basis: &str,
    outcome: &str,
    selected_candidate: &str,
    tie_break_applied: bool,
    maintenance_evidence: Option<Value>,
) -> Value {
    let mut calculation = json!({
        "focusedScore": focused_score,
        "integratedScore": integrated_score,
        "absoluteDifference": difference,
        "tieBreakApplied": tie_break_applied
    });
    if let Some(maintenance_evidence) = maintenance_evidence
        && let Some(object) = calculation.as_object_mut()
    {
        object.insert("maintenanceEvidence".to_owned(), maintenance_evidence);
    }
    json!({
        "candidateEvidence": {"focused": focused, "integrated": integrated},
        "eligibility": {"focused": "eligible", "integrated": "eligible"},
        "decisionBasis": decision_basis,
        "outcome": outcome,
        "selectedCandidate": selected_candidate,
        "calculation": calculation
    })
}

fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "xtask must remain directly below the workspace root".into())
}

fn ready_fixture_root() -> Result<PathBuf, Box<dyn Error>> {
    Ok(workspace_root()?.join("qualification/fixtures/contracts/readiness/ready"))
}

fn production_fixture_root() -> Result<PathBuf, Box<dyn Error>> {
    Ok(workspace_root()?.join("qualification/fixtures/contracts/readiness/production-3b"))
}

fn temporary_fixture_root(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("oxyflut-readiness-{name}-{}", std::process::id()))
}

fn copy_directory(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    if destination.exists() {
        fs::remove_dir_all(destination)?;
    }
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let destination_path = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_directory(&entry.path(), &destination_path)?;
        } else {
            fs::copy(entry.path(), destination_path)?;
        }
    }
    Ok(())
}
