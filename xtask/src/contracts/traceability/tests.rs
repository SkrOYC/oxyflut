//! Traceability validator unit tests.

use oxyflut_qualification::identifiers::{CandidateId, EnvironmentId, RepositoryPath};
use serde_json::{Value, json};
use std::error::Error;

use super::fixtures::{
    NEGATIVE_FIXTURE_CASES, assert_code, assert_schema_failure, baseline_reference, remove_symbol,
    run_negative_fixture, workspace_root,
};
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
fn texture_reverse_ingress_and_contract_test_bijection_are_closed() -> Result<(), Box<dyn Error>> {
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
        .ok_or("contract test must exist")? = Value::String("contract::cap_renamed_001".to_owned());
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
        Some(EnvironmentId::Macos),
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
                Some(EnvironmentId::Macos),
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
            Some(EnvironmentId::Macos),
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
            Some(EnvironmentId::Macos),
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
            Some(EnvironmentId::Macos),
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
            Some(EnvironmentId::Windows),
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
            None,
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

    let mut unknown_accessibility_status = committed.clone();
    *unknown_accessibility_status
        .pointer_mut("/environments/macos/accessibilityMaps/focused/status")
        .ok_or("accessibility status must exist")? = Value::String("unknown".to_owned());
    assert_code(
        super::validate_platform_baseline(&root, &unknown_accessibility_status, &active, &registry),
        "accessibility-reference-status",
    );

    let stale_reference = json!({
        "status": "kk",
        "path": "qualification/fixtures/contracts/traceability/synthetic-accessibility-stale.json",
        "sha256": "408ddcaefead14480857e23f51f645a4b34591522f3e0612a6768ebc47289135"
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
        super::validate_platform_baseline(&root, &wrong_accessibility_digest, &active, &registry),
        "accessibility-digest",
    );
    let mut wrong_accessibility_identity = committed.clone();
    *wrong_accessibility_identity
        .pointer_mut("/environments/windows/accessibilityMaps/focused")
        .ok_or("accessibility reference must exist")? = stale_reference;
    assert_code(
        super::validate_platform_baseline(&root, &wrong_accessibility_identity, &active, &registry),
        "accessibility-identity",
    );
    let mut nested_accessibility_ku = committed;
    *nested_accessibility_ku
        .pointer_mut("/environments/macos/accessibilityMaps/focused")
        .ok_or("accessibility reference must exist")? = json!({
        "status": "kk",
        "path": "qualification/fixtures/contracts/traceability/synthetic-accessibility-ku.json",
        "sha256": "632d705bee6fa895f4cc701e3e73ab373aa506ae4a97a58ebe450ce176970e9c"
    });
    assert_schema_failure(
        super::validate_platform_baseline(&root, &nested_accessibility_ku, &active, &registry),
        "accessibility-schema",
        &root.join("qualification/fixtures/contracts/traceability/synthetic-accessibility-ku.json"),
        "/forward/roles/status",
    )?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn evidence_references_reject_symlinks_that_escape_the_repository() -> Result<(), Box<dyn Error>> {
    use std::fs;
    use std::os::unix::fs::symlink;

    let root = std::env::temp_dir().join(format!(
        "oxyflut-traceability-symlink-{}",
        std::process::id()
    ));
    let outside = std::env::temp_dir().join(format!(
        "oxyflut-traceability-outside-{}",
        std::process::id()
    ));
    fs::create_dir_all(root.join("qualification"))?;
    fs::create_dir_all(&outside)?;
    let outside_proof = outside.join("proof.txt");
    fs::write(&outside_proof, b"outside")?;
    symlink(&outside_proof, root.join("qualification/proof.txt"))?;
    let reference = json!({
        "path": "qualification/proof.txt",
        "sha256": oxyflut_qualification::hash::hash_file(&outside_proof)?.to_string()
    });

    let result = super::resolve_evidence(&root, &reference);
    fs::remove_dir_all(&root)?;
    fs::remove_dir_all(&outside)?;
    assert!(matches!(
        result,
        Err(super::TraceabilityError::Digest(
            super::DigestError::SymlinkEscape { .. }
        ))
    ));
    Ok(())
}

#[test]
fn file_qualified_symbols_require_a_declaration_scoped_owner_body() -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let fixture_root = root.join("qualification/fixtures/contracts/traceability");
    for fixture in [
        "symbol-member-under-different-owner.rs",
        "symbol-owner-without-member.rs",
        "symbol-comment-or-string.rs",
    ] {
        let path = fixture_root.join(fixture);
        assert!(!super::symbol_resolves(
            &path,
            &super::read_text(&path)?,
            "Owner::member"
        )?);
    }
    let comment_path = fixture_root.join("symbol-comment-or-string.rs");
    assert!(!super::symbol_resolves(
        &comment_path,
        &super::read_text(&comment_path)?,
        "Owner",
    )?);
    let header_path = fixture_root.join("symbol-c-header.h");
    let header = super::read_text(&header_path)?;
    assert!(super::symbol_resolves(
        &header_path,
        &header,
        "DifferentOwner::member"
    )?);
    assert!(!super::symbol_resolves(
        &header_path,
        &header,
        "Owner::member"
    )?);
    let declaration_forms = "struct Owner; impl Owner { fn member(&self) {} } trait Trait {} impl Trait for Owner { const ASSOCIATED: u8 = 1; } mod module_owner { type member = u8; }";
    let declaration_path = fixture_root.join("symbol-declaration-forms.rs");
    assert!(super::symbol_resolves(
        &declaration_path,
        declaration_forms,
        "Owner::member"
    )?);
    assert!(super::symbol_resolves(
        &declaration_path,
        declaration_forms,
        "module_owner::member"
    )?);
    assert!(super::symbol_resolves(
        &declaration_path,
        "pub const TopLevel: u8 = 1; fn outer() { let NotTopLevel = 1; }",
        "TopLevel",
    )?);
    assert!(!super::symbol_resolves(
        &declaration_path,
        "fn outer() { let NotTopLevel = 1; }",
        "NotTopLevel",
    )?);
    Ok(())
}

#[test]
fn character_literals_do_not_hide_following_declarations() -> Result<(), Box<dyn Error>> {
    let source = "pub const PROBE_SEPARATOR: char = 'a';\n/// don't hide this declaration\npub struct Probe;";
    let path = workspace_root()?
        .join("qualification/fixtures/contracts/traceability/character-literal.rs");
    assert!(super::symbol_resolves(&path, source, "Probe")?);
    Ok(())
}

#[test]
fn digest_bound_capability_baselines_schema_validate_before_semantics() -> Result<(), Box<dyn Error>>
{
    let root = workspace_root()?;
    let active = super::active_specification(&root)?;
    let capabilities = super::prd_capabilities(&root)?;
    let approved_path =
        "qualification/fixtures/contracts/traceability/synthetic-capability-baseline-approved.json";
    let mut lock = super::read_json(&root.join(super::LOCK_PATH))?;
    *lock
        .pointer_mut("/measurementPolicy/capabilityBaseline")
        .ok_or("lock baseline must exist")? = baseline_reference(
        approved_path,
        "18b1568c013d20519f240991f28ba4b394c0d26ace136fe8d604b70937fc103e",
    );
    super::validate_lock_baseline(&root, &lock, &active, &capabilities)?;

    let mut synthetic = lock.clone();
    *synthetic
            .pointer_mut("/measurementPolicy/capabilityBaseline/path")
            .ok_or("lock baseline path must exist")? = Value::String(
            "qualification/fixtures/contracts/traceability/synthetic-capability-baseline-synthetic.json"
                .to_owned(),
        );
    *synthetic
        .pointer_mut("/measurementPolicy/capabilityBaseline/sha256")
        .ok_or("lock baseline digest must exist")? = Value::String(
        "47728d265ebbecffb8ba087f36410d8c1eb5f814f062971c846a121a181d337b".to_owned(),
    );
    assert_code(
        super::validate_lock_baseline(&root, &synthetic, &active, &capabilities),
        "lock-baseline-provenance",
    );

    let mut mismatched_approval = lock.clone();
    *mismatched_approval
        .pointer_mut("/measurementPolicy/capabilityBaseline/approvalEvidence/sha256")
        .ok_or("lock approval digest must exist")? = Value::String(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
    );
    assert_code(
        super::validate_lock_baseline(&root, &mismatched_approval, &active, &capabilities),
        "lock-baseline-approval-evidence",
    );

    for (fixture, digest, expected_path) in [
        (
            "synthetic-capability-baseline-missing-schema-version.json",
            "baa6ba1689308086e23e782251bda9edb03baac61c586f03419e483eeac23852",
            "",
        ),
        (
            "synthetic-capability-baseline-malformed-entry.json",
            "95bd06f985755ddf42bbf97797a7944e8db4c3909e98b3f73c660b0e72d9fa61",
            "/capabilities/CAP-CMP-001",
        ),
    ] {
        let path = format!("qualification/fixtures/contracts/traceability/{fixture}");
        let mut invalid = lock.clone();
        *invalid
            .pointer_mut("/measurementPolicy/capabilityBaseline/path")
            .ok_or("lock baseline path must exist")? = Value::String(path.clone());
        *invalid
            .pointer_mut("/measurementPolicy/capabilityBaseline/sha256")
            .ok_or("lock baseline digest must exist")? = Value::String(digest.to_owned());
        assert_schema_failure(
            super::validate_lock_baseline(&root, &invalid, &active, &capabilities),
            "capability-baseline-schema",
            &root.join(path),
            expected_path,
        )?;
    }
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
fn every_ticket_named_negative_fixture_runs_the_expected_validator_case()
-> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    for case in NEGATIVE_FIXTURE_CASES {
        let fixture = super::read_json(
            &root
                .join("qualification/fixtures/contracts/traceability")
                .join(format!("{case}.json")),
        )?;
        let expected_code = fixture
            .get("expectedCode")
            .and_then(Value::as_str)
            .ok_or("fixture must name its expected failure code")?;
        let expected_path = fixture
            .get("expectedPath")
            .and_then(Value::as_str)
            .ok_or("fixture must name its expected failure path")?;
        assert_eq!(fixture.get("case").and_then(Value::as_str), Some(*case));

        let actual = run_negative_fixture(&root, case)?;
        assert_eq!(actual.code, expected_code, "fixture {case}");
        assert_eq!(actual.path, expected_path, "fixture {case}");
    }
    Ok(())
}
