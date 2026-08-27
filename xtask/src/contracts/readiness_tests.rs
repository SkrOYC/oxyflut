use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use oxyflut_qualification::hash::hash_file;
use serde_json::Value;

use super::super::{digests::DigestError, schema};
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
fn fabricated_promotion_fixtures_reject_lock_candidate_version_and_missing_artifacts()
-> Result<(), Box<dyn Error>> {
    let root = production_fixture_root()?;
    let registry = schema::compile_workspace(&workspace_root()?)?;
    for fixture in [
        "negative/promotion-wrong-lock-phase.json",
        "negative/promotion-wrong-candidate-phase.json",
        "negative/promotion-wrong-version-phase.json",
        "negative/promotion-untyped-wrong-lock-phase.json",
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
    let root = temporary_fixture_root();
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

fn temporary_fixture_root() -> PathBuf {
    std::env::temp_dir().join(format!("oxyflut-tampered-adr-{}", std::process::id()))
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
