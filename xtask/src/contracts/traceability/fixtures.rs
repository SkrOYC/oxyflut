//! Fixture-driven traceability validator test helpers.

use std::error::Error;
use std::path::{Path, PathBuf};

use oxyflut_qualification::identifiers::{CandidateId, EnvironmentId};
use serde_json::{Value, json};

pub(super) struct FixtureOutcome {
    pub(super) code: &'static str,
    pub(super) path: &'static str,
}

pub(super) fn run_negative_fixture(
    root: &Path,
    case: &str,
) -> Result<FixtureOutcome, Box<dyn Error>> {
    let active = super::active_specification(root)?;
    let capabilities = super::prd_capabilities(root)?;
    let flows = super::architecture_flows(root)?;
    let registry = super::read_json(&root.join(super::REGISTRY_PATH))?;
    let traceability = super::read_json(&root.join(super::TRACEABILITY_PATH))?;
    let platform = super::read_json(&root.join(super::PLATFORM_PATH))?;

    match case {
        "missing" => {
            let mut invalid = traceability;
            invalid
                .get_mut("mappings")
                .and_then(Value::as_array_mut)
                .ok_or("traceability mappings must be an array")?
                .pop()
                .ok_or("traceability must contain a mapping")?;
            traceability_fixture(
                super::validate_traceability(root, &invalid, &active, &capabilities, &flows),
                "/mappings",
            )
        }
        "duplicate" => {
            let mut invalid = traceability;
            let duplicate = invalid
                .pointer("/mappings/0/capabilityId")
                .and_then(Value::as_str)
                .ok_or("first capability ID must exist")?
                .to_owned();
            *invalid
                .pointer_mut("/mappings/1/capabilityId")
                .ok_or("second capability ID must exist")? = Value::String(duplicate);
            traceability_fixture(
                super::validate_traceability(root, &invalid, &active, &capabilities, &flows),
                "/mappings/1/capabilityId",
            )
        }
        "duplicated-contract-test-id" => {
            let mut invalid = traceability;
            let contract_test = invalid
                .pointer("/mappings/0/contractTests/0")
                .and_then(Value::as_str)
                .ok_or("contract test must exist")?
                .to_owned();
            *invalid
                .pointer_mut("/mappings/0/contractTests")
                .ok_or("contract tests must exist")? = json!([contract_test, contract_test]);
            traceability_fixture(
                super::validate_traceability(root, &invalid, &active, &capabilities, &flows),
                "/mappings/0/contractTests/1",
            )
        }
        "extra-contract-test-id" => {
            let mut invalid = traceability;
            *invalid
                .pointer_mut("/mappings/0/contractTests")
                .ok_or("contract tests must exist")? =
                json!(["contract::cap_cmp_001", "contract::cap_extra_001"]);
            traceability_fixture(
                super::validate_traceability(root, &invalid, &active, &capabilities, &flows),
                "/mappings/0/contractTests/1",
            )
        }
        "non-derivable-contract-test-id" => {
            let mut invalid = traceability;
            *invalid
                .pointer_mut("/mappings/0/contractTests/0")
                .ok_or("contract test must exist")? =
                Value::String("contract::cap_renamed_001".to_owned());
            traceability_fixture(
                super::validate_traceability(root, &invalid, &active, &capabilities, &flows),
                "/mappings/0/contractTests/0",
            )
        }
        "unknown" => {
            let mut invalid = traceability;
            *invalid
                .pointer_mut("/mappings/0/capabilityId")
                .ok_or("first capability ID must exist")? =
                Value::String("CAP-UNKNOWN-001".to_owned());
            traceability_fixture(
                super::validate_traceability(root, &invalid, &active, &capabilities, &flows),
                "/mappings/0/capabilityId",
            )
        }
        "stale-path" => {
            let mut invalid = traceability;
            *invalid
                .pointer_mut("/mappings/0/architectureFlow")
                .ok_or("architecture flow must exist")? =
                Value::String(".constitution/architecture/flows/stale.md".to_owned());
            traceability_fixture(
                super::validate_traceability(root, &invalid, &active, &capabilities, &flows),
                "/mappings/0/architectureFlow",
            )
        }
        "omitted-required-capability-to-contract-edge" => {
            let mut invalid = traceability;
            invalid
                .pointer_mut("/mappings/0/bindings")
                .and_then(Value::as_array_mut)
                .ok_or("bindings must be an array")?
                .clear();
            traceability_fixture(
                super::validate_traceability(root, &invalid, &active, &capabilities, &flows),
                "/mappings/0/bindings",
            )
        }
        "omitted-texture-drawing-edge" => {
            let mut invalid = traceability;
            remove_symbol(
                &mut invalid,
                "CAP-REN-002",
                "contracts/oxyflut-public.rs",
                "Canvas::draw_texture",
            )?;
            traceability_fixture(
                super::validate_traceability(root, &invalid, &active, &capabilities, &flows),
                "/mappings/CAP-REN-002/bindings/0/symbols",
            )
        }
        "omitted-reverse-action-ingress" => {
            let mut invalid = traceability;
            remove_symbol(
                &mut invalid,
                "CAP-SEM-002",
                "contracts/oxyflut-substrate.h",
                "OxySubstrateCallbacks.on_semantics_action",
            )?;
            traceability_fixture(
                super::validate_traceability(root, &invalid, &active, &capabilities, &flows),
                "/mappings/CAP-SEM-002/bindings/2/symbols",
            )
        }
        "unresolved-file-qualified-symbol" => {
            let mut invalid = traceability;
            *invalid
                .pointer_mut("/mappings/0/bindings/0/symbols/0")
                .ok_or("physical symbol must exist")? =
                Value::String("MissingPhysicalSymbol".to_owned());
            traceability_fixture(
                super::validate_traceability(root, &invalid, &active, &capabilities, &flows),
                "/mappings/0/bindings/0/symbols/0",
            )
        }
        "mismatched-active-specification-version" => {
            let mut invalid = traceability;
            *invalid
                .pointer_mut("/specificationVersion")
                .ok_or("traceability version must exist")? = Value::String("0.0.0".to_owned());
            traceability_fixture(
                super::validate_traceability(root, &invalid, &active, &capabilities, &flows),
                "/specificationVersion",
            )
        }
        "stale-platform-baseline-specification-version" => {
            let mut invalid = platform;
            *invalid
                .pointer_mut("/specificationVersion")
                .ok_or("platform version must exist")? = Value::String("0.0.0".to_owned());
            traceability_fixture(
                super::validate_platform_baseline(root, &invalid, &active, &registry),
                "/specificationVersion",
            )
        }
        "duplicate-absent-event-id" => {
            let mut invalid = platform;
            let absent_event = super::read_json(&root.join(
                "qualification/fixtures/contracts/traceability/synthetic-platform-baseline.json",
            ))?
            .pointer("/absentEvents/0")
            .ok_or("synthetic absent event must exist")?
            .clone();
            *invalid
                .pointer_mut("/absentEvents")
                .ok_or("absent events must exist")? = json!([absent_event.clone(), absent_event]);
            traceability_fixture(
                super::validate_platform_baseline(root, &invalid, &active, &registry),
                "/absentEvents/1/id",
            )
        }
        "synthetic-baseline-referenced-by-lock" => {
            let mut lock = lock_with_approved_baseline(root)?;
            super::validate_lock_baseline(root, &lock, &active, &capabilities)?;
            *lock
                    .pointer_mut("/measurementPolicy/capabilityBaseline/path")
                    .ok_or("lock baseline path must exist")? = Value::String(
                    "qualification/fixtures/contracts/traceability/synthetic-capability-baseline-synthetic.json"
                        .to_owned(),
                );
            *lock
                .pointer_mut("/measurementPolicy/capabilityBaseline/sha256")
                .ok_or("lock baseline digest must exist")? = Value::String(
                "47728d265ebbecffb8ba087f36410d8c1eb5f814f062971c846a121a181d337b".to_owned(),
            );
            traceability_fixture(
                super::validate_lock_baseline(root, &lock, &active, &capabilities),
                "/measurementPolicy/capabilityBaseline/path",
            )
        }
        "missing-baseline-approval-evidence" | "missing-lock-baseline-schema-version" => {
            let mut lock = lock_with_approved_baseline(root)?;
            let (fixture, digest, path) = if case == "missing-baseline-approval-evidence" {
                (
                    "synthetic-capability-baseline-missing-approval-evidence.json",
                    "58105df2aedfe4ff6193565f98ed457697cfc9d092dc92303279782e34c6531d",
                    "/provenance/approvalEvidence",
                )
            } else {
                (
                    "synthetic-capability-baseline-missing-schema-version.json",
                    "baa6ba1689308086e23e782251bda9edb03baac61c586f03419e483eeac23852",
                    "/schemaVersion",
                )
            };
            *lock
                .pointer_mut("/measurementPolicy/capabilityBaseline/path")
                .ok_or("lock baseline path must exist")? = Value::String(format!(
                "qualification/fixtures/contracts/traceability/{fixture}"
            ));
            *lock
                .pointer_mut("/measurementPolicy/capabilityBaseline/sha256")
                .ok_or("lock baseline digest must exist")? = Value::String(digest.to_owned());
            traceability_fixture(
                super::validate_lock_baseline(root, &lock, &active, &capabilities),
                path,
            )
        }
        "malformed-lock-baseline-entry" => {
            let mut lock = lock_with_approved_baseline(root)?;
            *lock
                    .pointer_mut("/measurementPolicy/capabilityBaseline/path")
                    .ok_or("lock baseline path must exist")? = Value::String(
                    "qualification/fixtures/contracts/traceability/synthetic-capability-baseline-malformed-entry.json"
                        .to_owned(),
                );
            *lock
                .pointer_mut("/measurementPolicy/capabilityBaseline/sha256")
                .ok_or("lock baseline digest must exist")? = Value::String(
                "95bd06f985755ddf42bbf97797a7944e8db4c3909e98b3f73c660b0e72d9fa61".to_owned(),
            );
            traceability_fixture(
                super::validate_lock_baseline(root, &lock, &active, &capabilities),
                "/capabilities/CAP-CMP-001",
            )
        }
        "mismatched-baseline-approval-evidence" => {
            let mut lock = lock_with_approved_baseline(root)?;
            *lock
                .pointer_mut("/measurementPolicy/capabilityBaseline/approvalEvidence/sha256")
                .ok_or("lock approval digest must exist")? = Value::String(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
            );
            traceability_fixture(
                super::validate_lock_baseline(root, &lock, &active, &capabilities),
                "/measurementPolicy/capabilityBaseline/approvalEvidence/sha256",
            )
        }
        "pass-with-nonnull-absence-binding" => traceability_fixture(
            super::validate_gate(
                root,
                &json!({"status":"pass","evidence":[],"notApplicable":{}}),
                &"CAP-CMP-001".parse()?,
                CandidateId::Focused,
                EnvironmentId::Macos,
                false,
                false,
                &active,
                &registry,
                &".constitution/tech-spec/contracts/platform-contracts.json".parse()?,
            ),
            "/environmentResults/macos/CAP-CMP-001/notApplicable",
        ),
        "not-applicable-missing-binding" => traceability_fixture(
            super::validate_gate(
                root,
                &json!({"status":"not-applicable-kk","evidence":[]}),
                &"CAP-CMP-001".parse()?,
                CandidateId::Focused,
                EnvironmentId::Macos,
                false,
                false,
                &active,
                &registry,
                &".constitution/tech-spec/contracts/platform-contracts.json".parse()?,
            ),
            "/environmentResults/macos/CAP-CMP-001/notApplicable",
        ),
        "mismatched-platform-baseline-path"
        | "mismatched-platform-baseline-digest"
        | "mismatched-platform-baseline-schema-version"
        | "mismatched-platform-baseline-specification-version"
        | "unknown-absent-event-id" => {
            let mut binding = absence_binding();
            let (pointer, value) = match case {
                "mismatched-platform-baseline-path" => (
                    "/platformBaseline/path",
                    "qualification/fixtures/contracts/traceability/missing.json",
                ),
                "mismatched-platform-baseline-digest" => (
                    "/platformBaseline/sha256",
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                ),
                "mismatched-platform-baseline-schema-version" => {
                    ("/platformBaseline/schemaVersion", "4.0.0")
                }
                "mismatched-platform-baseline-specification-version" => {
                    ("/platformBaseline/specificationVersion", "0.0.0")
                }
                "unknown-absent-event-id" => ("/absentEventId", "ABS-CMP-404"),
                _ => return Err("fixture case must be declared".into()),
            };
            *binding
                .pointer_mut(pointer)
                .ok_or("absence binding pointer must exist")? = Value::String(value.to_owned());
            traceability_fixture(
                    super::validate_absence_binding(
                        root,
                        binding.as_object().ok_or("absence binding must be an object")?,
                        &"CAP-CMP-001".parse()?,
                        CandidateId::Focused,
                        EnvironmentId::Macos,
                        false,
                        &active,
                        &registry,
                        &"qualification/fixtures/contracts/traceability/synthetic-platform-baseline.json"
                            .parse()?,
                    ),
                    pointer,
                )
        }
        "mismatched-absent-event-gate"
        | "mismatched-absent-event-event"
        | "unregistered-absent-event"
        | "mismatched-absent-event-candidate"
        | "mismatched-parent-environment"
        | "aggregate-constraint-without-four-environments" => {
            let baseline = super::read_json(&root.join(
                "qualification/fixtures/contracts/traceability/synthetic-platform-baseline.json",
            ))?;
            let mut entry = baseline
                .pointer("/absentEvents/0")
                .ok_or("synthetic absent event must exist")?
                .clone();
            let (gate, candidate, environment, aggregate, path) = match case {
                "mismatched-absent-event-gate" => {
                    *entry.pointer_mut("/gateId").ok_or("gate ID must exist")? =
                        Value::String("CAP-CMP-002".to_owned());
                    (
                        "CAP-CMP-001".parse()?,
                        CandidateId::Focused,
                        EnvironmentId::Macos,
                        false,
                        "/absentEvents/0/gateId",
                    )
                }
                "mismatched-absent-event-event" => {
                    *entry.pointer_mut("/eventId").ok_or("event ID must exist")? =
                        Value::String("unregistered.event".to_owned());
                    (
                        "CAP-CMP-001".parse()?,
                        CandidateId::Focused,
                        EnvironmentId::Macos,
                        false,
                        "/absentEvents/0/eventId",
                    )
                }
                "unregistered-absent-event" => {
                    *entry.pointer_mut("/eventId").ok_or("event ID must exist")? =
                        Value::String("unregistered.event".to_owned());
                    (
                        "CAP-CMP-001".parse()?,
                        CandidateId::Focused,
                        EnvironmentId::Macos,
                        false,
                        "/absentEvents/0/eventId",
                    )
                }
                "mismatched-absent-event-candidate" => (
                    "CAP-CMP-001".parse()?,
                    CandidateId::Integrated,
                    EnvironmentId::Macos,
                    false,
                    "/absentEvents/0/candidates",
                ),
                "mismatched-parent-environment" => (
                    "CAP-CMP-001".parse()?,
                    CandidateId::Focused,
                    EnvironmentId::Windows,
                    false,
                    "/absentEvents/0/environments",
                ),
                "aggregate-constraint-without-four-environments" => (
                    "CAP-CMP-001".parse()?,
                    CandidateId::Focused,
                    EnvironmentId::Macos,
                    true,
                    "/absentEvents/0/environments",
                ),
                _ => return Err("fixture case must be declared".into()),
            };
            traceability_fixture(
                super::validate_absent_event(
                    root,
                    &entry,
                    &gate,
                    candidate,
                    environment,
                    aggregate,
                    &registry,
                ),
                path,
            )
        }
        "remote-diagnostic-sink" => registry_fixture(
            super::registries::admit_local_sink("remote-exporter", Some(1)),
            "/destination",
        ),
        "undeclared-diagnostic-sink" => registry_fixture(
            super::registries::admit_local_sink("undeclared-sink", Some(1)),
            "/destination",
        ),
        "unbounded-sink-acknowledgement" => registry_fixture(
            super::registries::admit_local_sink("user-enabled-memory-buffer", None),
            "/maximumQueuedRecords",
        ),
        "missing-nested-kk-evidence" => {
            let mut invalid = platform;
            *invalid
                .pointer_mut("/environments/macos/ime/status")
                .ok_or("IME status must exist")? = Value::String("kk".to_owned());
            traceability_fixture(
                super::validate_platform_baseline(root, &invalid, &active, &registry),
                "/environments/macos/ime/evidence",
            )
        }
        "mismatched-nested-kk-evidence" => {
            let mut invalid = platform;
            *invalid
                .pointer_mut("/environments/macos/ime/status")
                .ok_or("IME status must exist")? = Value::String("kk".to_owned());
            *invalid
                .pointer_mut("/environments/macos/ime/evidence")
                .ok_or("IME evidence must exist")? = json!([{
                "path":"qualification/fixtures/contracts/traceability/evidence.txt",
                "sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            }]);
            traceability_fixture(
                super::validate_platform_baseline(root, &invalid, &active, &registry),
                "/environments/macos/ime/evidence/0/sha256",
            )
        }
        "mismatched-accessibility-identity"
        | "mismatched-accessibility-digest"
        | "nested-accessibility-ku"
        | "stale-accessibility-text-layout-generation" => {
            let mut invalid = platform;
            let (environment, reference, path) = match case {
                "mismatched-accessibility-identity" => (
                    "windows",
                    json!({
                        "status":"kk",
                        "path":"qualification/fixtures/contracts/traceability/synthetic-accessibility-stale.json",
                        "sha256":"408ddcaefead14480857e23f51f645a4b34591522f3e0612a6768ebc47289135"
                    }),
                    "/environments/windows/accessibilityMaps/focused",
                ),
                "mismatched-accessibility-digest" => (
                    "macos",
                    json!({
                        "status":"kk",
                        "path":"qualification/fixtures/contracts/traceability/synthetic-accessibility-stale.json",
                        "sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    }),
                    "/environments/macos/accessibilityMaps/focused/sha256",
                ),
                "nested-accessibility-ku" => (
                    "macos",
                    json!({
                        "status":"kk",
                        "path":"qualification/fixtures/contracts/traceability/synthetic-accessibility-ku.json",
                        "sha256":"632d705bee6fa895f4cc701e3e73ab373aa506ae4a97a58ebe450ce176970e9c"
                    }),
                    "/forward/roles/status",
                ),
                "stale-accessibility-text-layout-generation" => (
                    "macos",
                    json!({
                        "status":"kk",
                        "path":"qualification/fixtures/contracts/traceability/synthetic-accessibility-stale.json",
                        "sha256":"408ddcaefead14480857e23f51f645a4b34591522f3e0612a6768ebc47289135"
                    }),
                    "/reverseActions/0/textLayoutBinding",
                ),
                _ => return Err("fixture case must be declared".into()),
            };
            *invalid
                .pointer_mut(&format!(
                    "/environments/{environment}/accessibilityMaps/focused"
                ))
                .ok_or("accessibility reference must exist")? = reference;
            traceability_fixture(
                super::validate_platform_baseline(root, &invalid, &active, &registry),
                path,
            )
        }
        "empty-path-segment"
        | "trailing-path-segment"
        | "duplicate-path-separators"
        | "control-character-path" => {
            let path = match case {
                "empty-path-segment" => "",
                "trailing-path-segment" => "bin/",
                "duplicate-path-separators" => "bin//oxyflut",
                "control-character-path" => "bin/\u{0001}oxyflut",
                _ => return Err("fixture case must be declared".into()),
            };
            let invalid = artifact_manifest_with_path(path);
            traceability_fixture(super::validate_artifact_manifest(&invalid), "/files/0/path")
        }
        _ => Err(format!("fixture case is not implemented: {case}").into()),
    }
}

fn traceability_fixture<T>(
    result: Result<T, super::TraceabilityError>,
    path: &'static str,
) -> Result<FixtureOutcome, Box<dyn Error>> {
    let Err(error) = result else {
        return Err("fixture validator unexpectedly passed".into());
    };
    let Some(code) = error.code() else {
        return Err("fixture validator failed without a stable code".into());
    };
    Ok(FixtureOutcome { code, path })
}

fn registry_fixture<T>(
    result: Result<T, super::RegistryError>,
    path: &'static str,
) -> Result<FixtureOutcome, Box<dyn Error>> {
    let Err(error) = result else {
        return Err("fixture validator unexpectedly passed".into());
    };
    let super::RegistryError::Invariant { code } = error else {
        return Err("fixture validator failed without a stable code".into());
    };
    Ok(FixtureOutcome { code, path })
}

fn lock_with_approved_baseline(root: &Path) -> Result<Value, Box<dyn Error>> {
    let mut lock = super::read_json(&root.join(super::LOCK_PATH))?;
    *lock
        .pointer_mut("/measurementPolicy/capabilityBaseline")
        .ok_or("lock baseline must exist")? = baseline_reference(
        "qualification/fixtures/contracts/traceability/synthetic-capability-baseline-approved.json",
        "18b1568c013d20519f240991f28ba4b394c0d26ace136fe8d604b70937fc103e",
    );
    Ok(lock)
}

fn absence_binding() -> Value {
    json!({
        "platformBaseline": {
            "path": "qualification/fixtures/contracts/traceability/synthetic-platform-baseline.json",
            "sha256": "a68ecd2fac76ae263ced91fda4fb8c144857be46435d77eca33366ce8cd52a86",
            "schemaVersion": "5.0.0",
            "specificationVersion": "0.15.0"
        },
        "absentEventId": "ABS-CMP-001"
    })
}

fn artifact_manifest_with_path(path: &str) -> Value {
    json!({
        "files": [{
            "path": path,
            "kind": "file",
            "size": 3,
            "sha256": "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        }]
    })
}

pub(super) fn remove_symbol(
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

pub(super) fn baseline_reference(path: &str, sha256: &str) -> Value {
    json!({
        "path": path,
        "sha256": sha256,
        "schemaVersion": "4.0.0",
        "provenance": "approved",
        "approvalEvidence": {
            "path": "qualification/fixtures/contracts/traceability/evidence.txt",
            "sha256": "6ab11a71e6e2c7be933b6e2c3481a795e56ccd5bc7dfb51b72d6dcfd68458f4d"
        }
    })
}

pub(super) fn assert_code<T>(result: Result<T, super::TraceabilityError>, expected: &'static str) {
    let code = match result {
        Err(error) => error.code(),
        Ok(_) => None,
    };
    assert_eq!(code, Some(expected));
}

pub(super) fn assert_schema_failure<T>(
    result: Result<T, super::TraceabilityError>,
    expected_code: &'static str,
    expected_path: &Path,
    expected_issue_path: &str,
) -> Result<(), Box<dyn Error>> {
    let Err(super::TraceabilityError::Schema { code, path, source }) = result else {
        return Err("expected schema validation error".into());
    };
    if code != expected_code || path != expected_path {
        return Err("schema failure code or path did not match".into());
    }
    let oxyflut_qualification::schema::SchemaError::Validation { issues, .. } = source else {
        return Err("expected local schema validation error".into());
    };
    if issues
        .iter()
        .any(|issue| issue.instance_path == expected_issue_path)
    {
        Ok(())
    } else {
        Err("schema failure location did not match".into())
    }
}

pub(super) fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "xtask must remain directly below the workspace root".into())
}

pub(super) const NEGATIVE_FIXTURE_CASES: &[&str] = &[
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
    "missing-lock-baseline-schema-version",
    "malformed-lock-baseline-entry",
];
