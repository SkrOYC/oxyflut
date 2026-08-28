use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicU64, Ordering};

use oxyflut_qualification::hash::hash_file;
use oxyflut_qualification::readiness::StagedInputRegistry;
use serde_json::Value;

use super::{
    candidate_report_at, candidate_report_error_lines, candidate_report_lines, invalid_status_line,
    run, run_at_root, workspace_root,
};
use crate::{CommandOutcome, toolchain};

const COMPLETE_SYNTHETIC: &str = "qualification/fixtures/readiness/complete.synthetic.json";
const INVALID: &str = "qualification/fixtures/readiness/invalid.json";
const CLEARED_WITHOUT_EVIDENCE: &str =
    "qualification/fixtures/readiness/cleared-without-evidence.json";
const COMPLETE_STAGED_INPUTS: &str = "qualification/fixtures/readiness/staged";
const READY_FIXTURE: &str = "qualification/fixtures/contracts/readiness/ready";
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn committed_candidate_gate_is_valid_but_open_with_the_exact_ku_set() -> Result<(), Box<dyn Error>>
{
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let root = source_root()?;
    let report = candidate_report_at(&root).map_err(|_| "committed report must parse")?;
    let known_unknowns = report
        .blocking
        .iter()
        .filter(|blocking| blocking.kind == oxyflut_qualification::readiness::BlockingKind::Ku)
        .map(|blocking| {
            blocking
                .field_path
                .strip_prefix("preImplementationKnownUnknowns.")
                .ok_or("KU paths must name the lock array member")
        })
        .collect::<Result<Vec<_>, _>>()?;

    assert_eq!(
        run(&["--gate".to_owned(), "candidate-implementation".to_owned()]),
        CommandOutcome::ValidButOpen
    );
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
            "resolved-tool-digests",
            "scoring-anchors-and-two-assessors",
            "security-patch-rehearsal",
        ]
    );
    Ok(())
}

#[test]
fn complete_synthetic_workspace_returns_exit_zero_through_the_command_path()
-> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let root = complete_fixture_root()?;
    let outcome = run_at_root(&root, "candidate-implementation");
    fs::remove_dir_all(&root)?;

    assert_eq!(outcome, CommandOutcome::Success);
    assert_eq!(outcome.exit_code(), ExitCode::SUCCESS);
    Ok(())
}

#[test]
fn mismatched_staged_input_digest_returns_exit_one() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let root = complete_fixture_root()?;
    let mut lock = read_json(&root.join(super::LOCK_PATH))?;
    *lock
        .pointer_mut("/measurementPolicy/sampleValidityRules")
        .ok_or("complete lock must bind sample-validity rules")? = Value::String("0".repeat(64));
    write_json(&root.join(super::LOCK_PATH), &lock)?;

    let error = candidate_report_at(&root)
        .err()
        .ok_or("digest mismatch must fail")?;
    assert_eq!(
        candidate_report_error_lines(&error),
        vec![
            "lock status: invalid (staged-input-digest-mismatch)",
            "blocking: field-path=measurementPolicy.sampleValidityRules kind=digest-mismatch evidence-path=qualification/schemas/sample-validity.schema.json upstream-owner=OXY-C003",
        ]
    );
    let outcome = run_at_root(&root, "candidate-implementation");
    fs::remove_dir_all(&root)?;

    assert!(matches!(outcome, CommandOutcome::Failed(_)));
    assert_eq!(outcome.exit_code(), ExitCode::FAILURE);
    Ok(())
}

#[test]
fn missing_conventional_staged_input_returns_exit_one() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let root = complete_fixture_root()?;
    fs::remove_file(root.join("qualification/staged/fuzz-corpora.json"))?;

    let error = candidate_report_at(&root)
        .err()
        .ok_or("missing staged input must fail")?;
    assert_eq!(
        candidate_report_error_lines(&error),
        vec!["lock status: invalid (staged-input-missing)"]
    );
    let outcome = run_at_root(&root, "candidate-implementation");
    fs::remove_dir_all(&root)?;

    assert!(matches!(outcome, CommandOutcome::Failed(_)));
    assert_eq!(outcome.exit_code(), ExitCode::FAILURE);
    Ok(())
}

#[test]
fn committed_complete_synthetic_resolved_tools_match_the_staged_manifest()
-> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let source = source_root()?;
    let lock = read_json(&source.join(COMPLETE_SYNTHETIC))?;
    let manifest = toolchain::ToolchainManifest::from_json(&fs::read(
        source.join("qualification/tools/native-contract-toolchain.json"),
    )?)?;
    let fixture_tools = lock
        .get("resolvedTools")
        .and_then(Value::as_array)
        .ok_or("complete fixture must contain resolved tools")?;
    let expected = toolchain::lock::lock_resolved_tools(&manifest)?;

    // This checks name, version, sha256, sourceIdentity, licenseId, hostTriple, and every Nix
    // path byte-for-byte. Only the Rustup prefix is host-resolved through manifest `pathRoot`.
    assert_eq!(
        toolchain::lock::normalize_rustup_paths(&manifest, fixture_tools)?,
        toolchain::lock::normalize_rustup_paths(&manifest, &expected)?,
    );
    Ok(())
}

#[test]
fn resolved_tools_require_the_exact_staged_manifest_set() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    for mutation in ["missing", "extra", "substituted"] {
        let root = complete_fixture_root()?;
        let mut lock = read_json(&root.join(super::LOCK_PATH))?;
        let tools = lock
            .pointer_mut("/resolvedTools")
            .and_then(Value::as_array_mut)
            .ok_or("complete lock must contain resolved tools")?;
        match mutation {
            "missing" => {
                let _ = tools.pop().ok_or("complete lock must contain a tool")?;
            }
            "extra" => {
                let mut extra = tools
                    .first()
                    .cloned()
                    .ok_or("complete lock must contain a tool")?;
                let extra_name = extra
                    .get_mut("name")
                    .and_then(|name| name.as_str())
                    .map(|name| format!("extra-{name}"))
                    .ok_or("complete tool must have a name")?;
                *extra
                    .get_mut("name")
                    .ok_or("complete tool must have a name field")? = Value::String(extra_name);
                tools.push(extra);
            }
            "substituted" => {
                let tool = tools
                    .first_mut()
                    .and_then(Value::as_object_mut)
                    .ok_or("complete lock must contain a tool object")?;
                *tool
                    .get_mut("sourceIdentity")
                    .ok_or("complete tool must have a source identity")? =
                    Value::String("substituted-source".to_owned());
            }
            _ => return Err("tool mutation must be declared".into()),
        }
        write_json(&root.join(super::LOCK_PATH), &lock)?;

        if mutation == "substituted" {
            let error = candidate_report_at(&root)
                .err()
                .ok_or("substituted resolved tool must fail")?;
            assert_eq!(
                candidate_report_error_lines(&error),
                vec!["lock status: invalid (resolved-tool-mismatch)"]
            );
        }
        let outcome = run_at_root(&root, "candidate-implementation");
        fs::remove_dir_all(&root)?;
        assert!(matches!(outcome, CommandOutcome::Failed(_)), "{mutation}");
        assert_eq!(outcome.exit_code(), ExitCode::FAILURE, "{mutation}");
    }
    Ok(())
}

#[test]
fn invalid_referenced_input_fixture_returns_exit_one() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let root = open_fixture_root("invalid")?;
    fs::copy(source_root()?.join(INVALID), root.join(super::LOCK_PATH))?;
    assert!(super::super::contracts::first_pre_implementation_input_failure(&root).is_none());
    let outcome = run_at_root(&root, "candidate-implementation");
    fs::remove_dir_all(&root)?;

    assert!(matches!(outcome, CommandOutcome::Failed(_)));
    assert_eq!(outcome.exit_code(), ExitCode::FAILURE);
    Ok(())
}

#[test]
fn cleared_ku_without_evidence_remains_open_with_the_exact_remaining_ku_set()
-> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let root = open_fixture_root("cleared")?;
    let report = candidate_report_at(&root).map_err(|_| "candidate readiness report must parse")?;
    let known_unknowns = report
        .blocking
        .iter()
        .filter(|blocking| blocking.kind == oxyflut_qualification::readiness::BlockingKind::Ku)
        .map(|blocking| {
            blocking
                .field_path
                .strip_prefix("preImplementationKnownUnknowns.")
                .ok_or("KU paths must name the lock array member")
        })
        .collect::<Result<Vec<_>, _>>()?;
    let outcome = run_at_root(&root, "candidate-implementation");
    fs::remove_dir_all(&root)?;

    assert_eq!(outcome, CommandOutcome::ValidButOpen);
    assert_eq!(outcome.exit_code(), ExitCode::from(2));
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
    Ok(())
}

#[test]
fn candidate_report_lines_are_stable_and_content_free() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let root = open_fixture_root("report-lines")?;
    let report = candidate_report_at(&root).map_err(|_| "candidate readiness report must parse")?;
    let lines = candidate_report_lines(&report);
    fs::remove_dir_all(&root)?;

    assert_eq!(
        lines.first().map(String::as_str),
        Some("lock status: open (candidate-implementation)")
    );
    assert!(lines.iter().all(|line| {
        !line.contains("macOS with Xcode") && !line.contains("https://") && !line.contains("Apple")
    }));
    assert!(lines.iter().any(|line| {
        line == "blocking: field-path=resolvedTools kind=missing evidence-path=qualification/tools/native-contract-toolchain.json upstream-owner=OXY-A008"
    }));
    assert!(lines.iter().any(|line| {
        line == "blocking: field-path=measurementPolicy.externalContractLock kind=null evidence-path=qualification/schemas/external/proposed-external-contract-lock.json referent=proposal upstream-owner=OXY-C001"
    }));
    for line in [
        "blocking: field-path=preImplementationKnownUnknowns.capability-and-platform-baselines kind=ku evidence-path=.constitution/tech-spec/contracts/platform-contracts.json upstream-owner=OXY-C002,OXY-C004",
        "blocking: field-path=preImplementationKnownUnknowns.scoring-anchors-and-two-assessors kind=ku evidence-path=qualification/staged/scoring-anchors.json upstream-owner=OXY-D001",
        "blocking: field-path=preImplementationKnownUnknowns.fuzz-corpora kind=ku evidence-path=qualification/staged/fuzz-corpora.json upstream-owner=OXY-D001",
        "blocking: field-path=preImplementationKnownUnknowns.security-patch-rehearsal kind=ku evidence-path=qualification/staged/security-patch-rehearsal.json upstream-owner=OXY-D001",
    ] {
        assert!(lines.iter().any(|actual| actual == line), "{line}");
    }
    Ok(())
}

#[test]
fn candidate_status_verifies_every_nonvalidator_policy_evidence_path() -> Result<(), Box<dyn Error>>
{
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let expected = StagedInputRegistry::measurement_policy_evidence_bindings()
        .filter(|(field, _, _)| !matches!(*field, "rawMeasurementSchema" | "platformContracts"))
        .collect::<Vec<_>>();
    assert_eq!(
        StagedInputRegistry::candidate_status_input_bindings().collect::<Vec<_>>(),
        expected
    );

    for (field, path, _) in expected {
        let root = complete_fixture_root()?;
        fs::remove_file(root.join(path))?;
        let error = candidate_report_at(&root)
            .err()
            .ok_or("each staged policy path must be verified")?;
        assert_eq!(
            candidate_report_error_lines(&error),
            vec!["lock status: invalid (staged-input-missing)"],
            "{field}"
        );
        fs::remove_dir_all(&root)?;
    }
    Ok(())
}

#[test]
fn candidate_report_uses_distinct_codes_for_lock_and_ku_failures() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let unreadable = complete_fixture_root()?;
    fs::remove_file(unreadable.join(super::LOCK_PATH))?;
    let unreadable_error = candidate_report_at(&unreadable)
        .err()
        .ok_or("unreadable lock must fail")?;
    assert_eq!(
        candidate_report_error_lines(&unreadable_error),
        vec!["lock status: invalid (lock-read)"]
    );
    fs::remove_dir_all(&unreadable)?;

    let malformed = complete_fixture_root()?;
    fs::write(malformed.join(super::LOCK_PATH), b"{")?;
    let malformed_error = candidate_report_at(&malformed)
        .err()
        .ok_or("malformed lock must fail")?;
    assert_eq!(
        candidate_report_error_lines(&malformed_error),
        vec!["lock status: invalid (lock-json)"]
    );
    fs::remove_dir_all(&malformed)?;

    let unmapped = complete_fixture_root()?;
    let mut lock = read_json(&unmapped.join(super::LOCK_PATH))?;
    lock.get_mut("preImplementationKnownUnknowns")
        .and_then(Value::as_array_mut)
        .ok_or("complete lock must contain known unknowns")?
        .push(Value::String("unmapped-known-unknown".to_owned()));
    write_json(&unmapped.join(super::LOCK_PATH), &lock)?;
    let unmapped_error = candidate_report_at(&unmapped)
        .err()
        .ok_or("unmapped known unknown must fail")?;
    assert_eq!(
        candidate_report_error_lines(&unmapped_error),
        vec!["lock status: invalid (unmapped-known-unknown)"]
    );
    let outcome = run_at_root(&unmapped, "candidate-implementation");
    fs::remove_dir_all(&unmapped)?;
    assert!(matches!(outcome, CommandOutcome::Failed(_)));
    Ok(())
}

#[test]
fn staged_external_proposal_without_its_ku_returns_exit_one() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let root = open_fixture_root("external-proposal-without-ku")?;
    let mut lock = read_json(&root.join(super::LOCK_PATH))?;
    lock.get_mut("preImplementationKnownUnknowns")
        .and_then(Value::as_array_mut)
        .ok_or("open lock must contain pre-implementation known unknowns")?
        .retain(|known_unknown| {
            known_unknown.as_str()
                != Some(oxyflut_qualification::readiness::EXTERNAL_CONTRACT_LOCK_KNOWN_UNKNOWN)
        });
    write_json(&root.join(super::LOCK_PATH), &lock)?;

    let error = candidate_report_at(&root)
        .err()
        .ok_or("staged external proposal without its KU must fail")?;
    assert_eq!(
        candidate_report_error_lines(&error),
        vec!["lock status: invalid (external-lock-proposal-without-ku)"]
    );
    let outcome = run_at_root(&root, "candidate-implementation");
    fs::remove_dir_all(&root)?;

    assert!(matches!(outcome, CommandOutcome::Failed(_)));
    assert_eq!(outcome.exit_code(), ExitCode::FAILURE);
    Ok(())
}

#[test]
fn lock_status_never_mutates_constitution() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let root = source_root()?;
    let before = constitution_digest(&root)?;
    let outcome = run_at_root(&root, "candidate-implementation");
    let after = constitution_digest(&root)?;

    assert_eq!(outcome, CommandOutcome::ValidButOpen);
    assert_eq!(before, after);
    Ok(())
}

#[test]
fn corrupt_platform_baseline_fails_before_reporting_an_open_gate() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let source = workspace_root().map_err(|_| "xtask must remain below the workspace root")?;
    let root = temporary_directory("corrupt-platform-baseline");
    copy_directory(&source.join(".constitution"), &root.join(".constitution"))?;
    copy_directory(&source.join("qualification"), &root.join("qualification"))?;

    let platform_path = root.join(".constitution/tech-spec/contracts/platform-contracts.json");
    let original = fs::read_to_string(&platform_path)?;
    let corrupted = original.replacen(
        "\"specificationVersion\": \"0.15.0\"",
        "\"specificationVersion\": \"0.15.1\"",
        1,
    );
    assert_ne!(corrupted, original);
    fs::write(platform_path, corrupted)?;

    let outcome = run_at_root(&root, "candidate-implementation");
    assert!(matches!(&outcome, CommandOutcome::Failed(_)));
    assert_eq!(outcome.exit_code(), ExitCode::FAILURE);
    let failure = super::super::contracts::first_pre_implementation_input_failure(&root)
        .ok_or("the corrupt platform baseline must fail a pre-implementation family")?;
    assert_eq!(
        invalid_status_line(&failure),
        "lock status: invalid (exact-set; .constitution/tech-spec/contracts/capability-traceability.json)"
    );
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn reports_measurement_gate_and_rejects_invalid_arguments() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    assert_eq!(
        run(&["--gate".to_owned(), "measurement".to_owned()]),
        CommandOutcome::ValidButOpen
    );
    assert!(matches!(run(&[]), CommandOutcome::Failed(_)));
    assert!(matches!(
        run(&["--gate".to_owned(), "production".to_owned()]),
        CommandOutcome::Failed(_)
    ));
    Ok(())
}

fn complete_fixture_root() -> Result<PathBuf, Box<dyn Error>> {
    let source = source_root()?;
    let root = temporary_directory("complete");
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }
    copy_directory(&source.join(".constitution"), &root.join(".constitution"))?;
    copy_directory(&source.join("qualification"), &root.join("qualification"))?;

    let ready = source.join(READY_FIXTURE);
    copy_directory(&ready.join("baselines"), &root.join("baselines"))?;
    copy_directory(&ready.join("evidence"), &root.join("evidence"))?;
    copy_directory(
        &source.join(COMPLETE_STAGED_INPUTS),
        &root.join("qualification/staged"),
    )?;
    fs::copy(
        source.join("qualification/schemas/external/proposed-external-contract-lock.json"),
        root.join(".constitution/tech-spec/contracts/external-contract-lock.json"),
    )?;

    let platform_path = root.join(".constitution/tech-spec/contracts/platform-contracts.json");
    let mut platform =
        read_json(&ready.join(".constitution/tech-spec/contracts/platform-contracts.json"))?;
    platform
        .as_object_mut()
        .ok_or("complete platform contract must be an object")?
        .insert(
            "$schema".to_owned(),
            Value::String("../data-models/platform-contracts.schema.json".to_owned()),
        );
    complete_accessibility_maps(&root, &source, &mut platform)?;
    write_json(&platform_path, &platform)?;

    let mut lock = read_readiness_fixture(&source, &source.join(COMPLETE_SYNTHETIC))?;
    let policy = lock
        .get_mut("measurementPolicy")
        .and_then(Value::as_object_mut)
        .ok_or("complete lock must have a measurement policy")?;
    for (field, path) in [
        (
            "rawMeasurementSchema",
            ".constitution/tech-spec/data-models/raw-measurement.schema.json",
        ),
        (
            "sampleValidityRules",
            "qualification/schemas/sample-validity.schema.json",
        ),
        (
            "platformContracts",
            ".constitution/tech-spec/contracts/platform-contracts.json",
        ),
        (
            "externalContractLock",
            "qualification/schemas/external/proposed-external-contract-lock.json",
        ),
        (
            "scoringAnchors",
            "qualification/staged/scoring-anchors.json",
        ),
        ("assessors", "qualification/staged/assessors.json"),
        ("fuzzCorpora", "qualification/staged/fuzz-corpora.json"),
        (
            "securityPatchRehearsal",
            "qualification/staged/security-patch-rehearsal.json",
        ),
    ] {
        *policy
            .get_mut(field)
            .ok_or("complete policy must contain every staged input")? =
            Value::String(hash_file(&root.join(path))?.to_string());
    }
    write_json(&root.join(super::LOCK_PATH), &lock)?;
    Ok(root)
}

fn complete_accessibility_maps(
    root: &Path,
    source: &Path,
    platform: &mut Value,
) -> Result<(), Box<dyn Error>> {
    let proof_path = "evidence/platform.txt";
    let proof_digest = hash_file(&root.join(proof_path))?.to_string();
    for environment in ["macos", "windows", "wayland", "x11"] {
        for candidate in ["focused", "integrated"] {
            let mut map = read_json(
                &source
                    .join("qualification/fixtures/contracts/accessibility-map/valid/minimal.json"),
            )?;
            let map_object = map
                .as_object_mut()
                .ok_or("accessibility fixture must be an object")?;
            *map_object
                .get_mut("environment")
                .ok_or("accessibility fixture must contain environment")? =
                Value::String(environment.to_owned());
            *map_object
                .get_mut("candidate")
                .ok_or("accessibility fixture must contain candidate")? =
                Value::String(candidate.to_owned());
            *map_object
                .get_mut("epistemicStatus")
                .ok_or("accessibility fixture must contain status")? =
                Value::String("kk-complete".to_owned());
            let forward = map_object
                .get_mut("forward")
                .and_then(Value::as_object_mut)
                .ok_or("accessibility fixture must contain forward mappings")?;
            for mapping in forward.values_mut() {
                *mapping
                    .get_mut("status")
                    .ok_or("forward mapping must contain status")? = Value::String("kk".to_owned());
            }
            let actions = map_object
                .get_mut("reverseActions")
                .and_then(Value::as_array_mut)
                .ok_or("accessibility fixture must contain reverse actions")?;
            for action in actions {
                *action
                    .get_mut("status")
                    .ok_or("reverse action must contain status")? = Value::String("kk".to_owned());
            }
            *map_object
                .get_mut("evidence")
                .ok_or("accessibility fixture must contain evidence")? =
                Value::Array(vec![Value::Object(
                    [
                        ("path".to_owned(), Value::String(proof_path.to_owned())),
                        ("sha256".to_owned(), Value::String(proof_digest.clone())),
                    ]
                    .into_iter()
                    .collect(),
                )]);

            let relative_path = format!("evidence/accessibility-{environment}-{candidate}.json");
            let map_path = root.join(&relative_path);
            write_json(&map_path, &map)?;
            let reference = platform
                .pointer_mut(&format!(
                    "/environments/{environment}/accessibilityMaps/{candidate}"
                ))
                .and_then(Value::as_object_mut)
                .ok_or("complete platform contract must contain accessibility reference")?;
            *reference
                .get_mut("path")
                .ok_or("accessibility reference must contain path")? = Value::String(relative_path);
            *reference
                .get_mut("sha256")
                .ok_or("accessibility reference must contain digest")? =
                Value::String(hash_file(&map_path)?.to_string());
        }
    }
    Ok(())
}

fn open_fixture_root(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let source = source_root()?;
    let root = temporary_directory(name);
    copy_directory(&source.join(".constitution"), &root.join(".constitution"))?;
    copy_directory(&source.join("qualification"), &root.join("qualification"))?;
    fs::copy(
        source.join(CLEARED_WITHOUT_EVIDENCE),
        root.join(super::LOCK_PATH),
    )?;
    Ok(root)
}

fn source_root() -> Result<PathBuf, Box<dyn Error>> {
    workspace_root().map_err(|_| "xtask must remain directly below the workspace root".into())
}

fn skip_on_unsupported_host() -> Result<bool, Box<dyn Error>> {
    if crate::toolchain::is_staged_host()? {
        Ok(false)
    } else {
        eprintln!("skipped: staged toolchain host is x86_64-unknown-linux-gnu");
        Ok(true)
    }
}

fn read_json(path: &Path) -> Result<Value, Box<dyn Error>> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn read_readiness_fixture(root: &Path, path: &Path) -> Result<Value, Box<dyn Error>> {
    let mut lock = read_json(path)?;
    let manifest = toolchain::ToolchainManifest::from_json(&fs::read(
        root.join("qualification/tools/native-contract-toolchain.json"),
    )?)?;
    let tools = lock
        .get_mut("resolvedTools")
        .and_then(Value::as_array_mut)
        .ok_or("readiness fixture must contain resolved tools")?;
    *tools = toolchain::lock::resolve_fixture_rustup_paths(&manifest, tools)?;
    Ok(lock)
}

fn write_json(path: &Path, value: &Value) -> Result<(), Box<dyn Error>> {
    fs::write(path, format!("{}\n", serde_json::to_string_pretty(value)?))?;
    Ok(())
}

fn constitution_digest(root: &Path) -> Result<BTreeMap<PathBuf, String>, Box<dyn Error>> {
    let directory = root.join(".constitution");
    let mut digests = BTreeMap::new();
    collect_file_digests(&directory, &mut digests)?;
    Ok(digests)
}

fn collect_file_digests(
    directory: &Path,
    digests: &mut BTreeMap<PathBuf, String>,
) -> Result<(), Box<dyn Error>> {
    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, std::io::Error>>()?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            collect_file_digests(&path, digests)?;
        } else {
            let digest = hash_file(&path)?.to_string();
            digests.insert(path, digest);
        }
    }
    Ok(())
}

fn temporary_directory(name: &str) -> PathBuf {
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "oxyflut-c005-{name}-{}-{sequence}",
        std::process::id()
    ))
}

fn copy_directory(source: &Path, destination: &Path) -> Result<(), std::io::Error> {
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
