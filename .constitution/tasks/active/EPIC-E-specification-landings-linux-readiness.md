# Epic E: Specification landings and Linux readiness

- **Status:** Active.
- **Total effort:** 49 points.
- **Plane:** Qualification.
- **Disjointness:** This epic owns only qualification-plane specifications, contracts, fixtures, lock inputs, `xtask`, and `oxyflut-qualification`; Epic F owns the integrated substrate candidate files, and Epic G owns the shared application-runtime crates.
- **Boundary:** This epic implements the Stage 3 reconciliation checklist rows stated by each ticket. It does not collect scored measurements, select a rendering substrate, or promote Phase 3A.

## Tickets

#### OXY-E001 Land the readiness schemas and contract types

- **Type:** Feature
- **Effort:** 8
- **Dependencies:** None
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/data-models/{capability-traceability,specification-phase,raw-measurement,accessibility-map,qualification-lock}.schema.json`
  - `.constitution/tech-spec/data-models/{semantic-role-registry,semantic-role-registry-snapshot}.schema.json` (proposed in SPK-B001, not committed)
  - `.constitution/tech-spec/data-models/{layout-qualification-record,layout-prequalification-run,layout-prequalification-suite}.schema.json` (proposed in SPK-B005, not committed)
  - `.constitution/tech-spec/contracts/{capability-traceability.json,qualification-lock.json,oxyflut-public.rs,oxyflut-substrate.rs,oxyflut-substrate.h}`
  - `.constitution/tech-spec/contracts/semantic-role-registry.json` (proposed in SPK-B001, not committed)
  - `xtask/src/contracts/{schema.rs,readiness.rs,readiness_promotion.rs,traceability/**}`
  - `xtask/src/commands/{environment/**,lock_tests.rs}`
  - `crates/oxyflut-qualification/src/readiness.rs`
- **Scope (Out-of-Scope Files):**
  - Candidate adapter crates, the native bridge, shared application-runtime crates, and every file owned by Epic F or Epic G (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a required schema field, role, or public symbol lacks an approved Stage 3 definition; route the gap to Stage 3."
  - "STOP if an upstream prerequisite in T0 changes the approved terminology or constraint; route the gap to its owning stage."
- **Description:** Apply T0a and T1.1-T1.6 in one compatibility-aware landing. Type traceability locations, specification-phase evidence, raw-measurement self-declaration and monotonic ordering, the semantic-role registry and snapshot schemas, accessibility-map v6, the per-environment qualification-lock v6 readiness fields from ADR-0011, and its named measurement-policy fields. Create the three layout schemas from the SPK-B005 canonical blocks with SHA-256 values `09d96af49384e47ee6154f386af2ef771985516a61c843d561835654283bd7b1`, `76dfee7dfcdfdd49e2d67afdf83ab43c29dbb6513652a8023b0869a7d59293e2`, and `27e3a876f3b8d5e88ad43089a9eff0c7ce225a6d9cece5fcd789f7759c05c924`.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
T0a preserves POLICY_FIELDS, KNOWN_UNKNOWN_BINDINGS, candidate_report_lines_are_stable_and_content_free, and staged_input_registry_binds_every_pathless_measurement_policy_digest while assigning readiness owners.
T1.1 uses discover_contract_instances and traceability contract-test resolution. T1.2 uses run_fixture_corpus. T1.3 uses ACCESSIBILITY_MAP_SCHEMA and accessibility-map validation.
T1.4 uses LOCK_SCHEMA and claimed-ready policy validation. T1.5 uses schema_compiles_committed_contract_instances_and_fixture_corpus. T1.6 uses POLICY_FIELDS and LOCK_SCHEMA.
The T1 landing changes LOCK_SCHEMA, candidate_input_issues, measurement_input_issues, validate_documents_with_attribution, readiness-policy reporting, and exact readiness assertions in the same change.
The lock schema requires qualificationSequence.candidateOrder, qualificationSequence.environmentOrder, referenceEnvironments.<environment>.candidateImplementationReady, referenceEnvironments.<environment>.measurementReady, sampleValidityRules, externalContractLock, layoutVisitCorpus, layoutQualificationRecordSchema, layoutPrequalificationRunSchema, layoutPrequalificationSuiteSchema, layoutVisitCountingRules, and layoutPrequalificationIdentities.
```

#### OXY-E002 Land contract instances and fixture corpora

- **Type:** Feature
- **Effort:** 8
- **Dependencies:** OXY-E001
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `qualification/fixtures/contracts/**`
  - `qualification/fixtures/readiness/**`
  - `.constitution/tech-spec/contracts/{capability-traceability.json,qualification-lock.json,platform-contracts.json,semantic-role-registry.json}`
  - `xtask/src/contracts/schema.rs`
  - `xtask/src/commands/{lock.rs,lock_tests.rs}`
  - `crates/oxyflut-qualification/src/readiness.rs`
- **Scope (Out-of-Scope Files):**
  - Candidate adapter crates, the native bridge, shared application-runtime crates, external-fixture preservation, and every file owned by Epic F or Epic G (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a fixture requires a schema rule or contract field absent from OXY-E001; route the gap to Stage 3."
  - "STOP if a fixture requires captured macOS or Windows evidence; retain the blocked input rather than inventing bytes."
- **Description:** Apply T2.1-T2.5 and T3.1-T3.3e after the schema landing. Add the semantic-role and layout fixture corpora and registry instance proposed in SPK-B001 and SPK-B005, not committed, with schema-valid and schema-invalid cases, v6 lock migrations, and typed policy references. Update the schema and instance counters from `18/6` to `23/7`, and make `lock status --gate candidate-implementation --environment ENVIRONMENT` distinguish `exit 0`, valid-but-open `exit 2`, and invalid `exit 1` without changing a readiness flag.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
T2.1-T2.3 use run_fixture_corpus. T2.2 and T2.4 also use $schema discovery, validate_migration_fixture, and LOCK_SCHEMA. T2.5 uses schema_compiles_committed_contract_instances_and_fixture_corpus.
T3.1 and T3.1a use discover_contract_instances, validate_required_symbol_edges, and the generated-role contract test. T3.2 uses validate_platform_baseline.
T3.3-T3.3e use candidate_implementation_report, LOCK_SCHEMA, StagedInputRegistry::candidate_status_input_bindings, collect_measurement_policy, collect_known_unknowns, validate_workspace, validate_lock_environment_projection, lock-status assertions, verify_lock_resolved_tools_classified, and POLICY_FIELDS.
schema_compiles_committed_contract_instances_and_fixture_corpus pins schema_count=23 and instance_count=7. lock status reports the per-environment candidate-implementation gate code without mutating the lock.
```

#### OXY-E003 Reconcile known-unknown sets and exact assertions

- **Type:** Chore
- **Effort:** 5
- **Dependencies:** OXY-E002
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/contracts/qualification-lock.json`
  - `qualification/fixtures/readiness/{invalid.json,cleared-without-evidence.json}`
  - `xtask/src/{commands/lock_tests.rs,contracts/{schema.rs,native_tests.rs,traceability/**}}`
  - `crates/oxyflut-qualification/src/readiness.rs`
- **Scope (Out-of-Scope Files):**
  - Candidate adapter crates, the native bridge, shared application-runtime crates, frozen binding goldens, and every file owned by Epic F or Epic G (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a changed KU has no named binding field or owner; route the gap to Stage 3."
  - "STOP if an assertion exposes an extra counter, schema, instance, or ABI transition not approved by the reconciliation checklist; route the gap to Stage 3."
- **Description:** Apply T4.1-T4.2 and T5.1-T5.6 as one lexicographic transaction. Move the active lock arrays from 13 to 29 pre-implementation known unknown (KU) values and from 15 to 31 gating KU values, preserve the stated fixture-specific counts, bind the B005 and B006 changes, and add exact-set, migration, accessibility-edge, counter, and application binary interface (ABI)-rejection assertions.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
T4.1 uses committed_candidate_gate_is_valid_but_open_with_the_exact_ku_set to accept exactly 29 sorted pre-implementation KUs and 31 sorted gating KUs.
T4.2 uses cleared_ku_without_evidence_remains_open_with_the_exact_remaining_ku_set, clearing_a_ku_string_without_its_evidence_keeps_the_gate_open, collect_known_unknowns, and invalid_referenced_input_fixture_returns_exit_one.
T5 uses candidate_report_lines_are_stable_and_content_free, schema_compiles_committed_contract_instances_and_fixture_corpus, abi_seven_through_ten_fail_before_callbacks_install, ACCESSIBILITY_MAP_SCHEMA, REQUIRED_ACCESSIBILITY_CATEGORIES, validate_required_symbol_edges, and validate_migration_fixture.
validate_migration_fixture checks source bytes, expected bytes, and v6 rejection for each named migration pair.
```

#### OXY-E004 Complete the deferred version and frozen-artifact migration

- **Type:** Chore
- **Effort:** 8
- **Dependencies:** OXY-E003, OXY-E007
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/{changelog.md,guidelines.md,stack.md,contracts/**,data-models/**}`
  - `qualification/{fixtures/generated-bindings/**,fixtures/native/**}`
  - `qualification/staged/**` (the B005 and B006 staged artifacts are proposed in those spikes, not committed)
  - `xtask/**`
  - `crates/oxyflut-qualification/**`
- **Scope (Out-of-Scope Files):**
  - Candidate adapter crates, the native bridge, shared application-runtime crates, external-fixture preservation, and every file owned by Epic F or Epic G (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a digest-bound input changes after its parent digest is frozen; return to the input ticket and regenerate the complete digest chain."
  - "STOP if an ABI, binding, layout, or specification-version change needs a contract not approved in the checklist; route the gap to Stage 3."
- **Description:** The technical-specification changelog defers T6, so apply T6.1-T6.3 and freeze T8 artifacts last. Migrate active specification-version literals and dependent digests, advance the integrated C ABI from `10u` to `11u`, regenerate the native layout probe and committed bindings golden, retain the layout probe contract, and freeze every dependent digest after schema, instance, corpus, and header edits settle.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
T6.1-T6.3 use validate_workspace, active-specification equality, and digests::validate_workspace after the version and digest migration.
T8.1 uses validate_interface, layout validation, and the generated-role contract test to accept ABI 11 and reject ABI 10 before callbacks install.
T8.2 uses validate_bindings and the generated-role contract test to accept the regenerated committed bindings golden and its SHA-256 sidecar.
The active specification version has one cross-document value after the migration, and E007 supplies the T8.3-T8.4 inputs before this ticket freezes dependent digests.
```

#### OXY-E005 Preserve Linux external fixtures with sidecars

- **Type:** Chore
- **Effort:** 5
- **Dependencies:** OXY-E001
- **Category:** Dependency-Upgrade
- **Scope (In-Scope Files):**
  - `qualification/fixtures/external-contracts/{wayland,x11}/` (proposed in SPK-B003 and SPK-B004, not committed)
  - `xtask/src/commands/external_contracts.rs`
- **Scope (Out-of-Scope Files):**
  - `qualification/fixtures/external-contracts/{macos,windows}/` (proposed in SPK-B001 and SPK-B002, not committed; don't touch), candidate adapter crates, shared application-runtime crates, and every file owned by Epic F or Epic G (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- external-contracts verify`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a canonical upstream byte source is unavailable or its license data is incomplete; preserve no proxy body and record the blocked source."
  - "STOP if a sidecar or its sibling digest differs from canonical source bytes; reject the fixture."
- **Description:** Apply the Linux portion of T2.6.1-T2.6.2. Use the explicitly permitted network fetch only to preserve canonical upstream bytes for the 11 Wayland fixtures and 15 X11 fixtures proposed in SPK-B003 and SPK-B004, not committed. Add `<FIXTURE>.source.json` sidecars and the validator that requires the regular sibling, equal SHA-256, canonical retrieval URL, upstream-relative source and license paths, required license fields, and exact Linux fixture sets. macOS fixtures and Windows captures remain blocked.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
external-contracts verify accepts 11 Wayland fixtures and 15 X11 fixtures only when every regular fixture has a valid sibling sidecar.
The validator rejects a missing sidecar, a mismatched sibling digest, an extra fixture, a missing fixture, a noncanonical retrieval URL, or incomplete license data.
The validator exempts the recorded Windows excerpt set without treating it as canonical capture.
```

#### OXY-E006 Capture Linux reference-environment inputs

- **Type:** Chore
- **Effort:** 5
- **Dependencies:** OXY-E001
- **Category:** DX
- **Scope (In-Scope Files):**
  - `qualification/{fixtures/environments/**,tools/native-contract-toolchain.json}`
  - `.constitution/tech-spec/contracts/qualification-lock.json`
  - `xtask/src/{commands/environment/**,commands/lock.rs,commands/lock_tests.rs,toolchain/**}`
  - `crates/oxyflut-qualification/src/readiness.rs`
- **Scope (Out-of-Scope Files):**
  - macOS and Windows captures, candidate adapter crates, the native bridge, shared application-runtime crates, and every file owned by Epic F or Epic G (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- environment inspect --environment ENVIRONMENT --output PATH`
- **Expected Success Output:** `exit 0` for each captured Linux environment.
- **STOP Conditions:**
  - "STOP if the capture is not from `thinkpadp14s`, or if it lacks a Nix system-closure digest; do not substitute an Ubuntu package snapshot."
  - "STOP if a resolved tool lacks an exact matching record in `qualification/tools/native-contract-toolchain.json`; retain the gate as open."
- **Description:** Capture the Wayland and X11 reference-environment projections on `thinkpadp14s`. Record the `systemPackageLockDigest` from the `narHash` for `/run/current-system` and bind only resolved tools classified by `qualification/tools/native-contract-toolchain.json`. Preserve the NixOS 26.05, AMD Renoir, Hyprland, Xwayland, and Xvfb reference details stated in the technical stack, including the recorded session-family risk.
- **Acceptance:**
  - **Mode:** hitl_sil
  - **Evidence:**

```text
Run environment inspect with environment=wayland and environment=x11 and a locked repository-relative output PATH.
Each accepted projection names thinkpadp14s, the required reference-environment identities, and systemPackageLockDigest from /run/current-system narHash.
lock status reports every unclassified or missing resolved tool as open rather than accepting a substitution.
```

#### OXY-E007 Define Linux workload and policy inputs

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-E001
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `qualification/staged/{fuzz-corpora.json,security-patch-rehearsal.json}` (proposed in SPK-B006, not committed)
  - `qualification/staged/{layout-visit-corpus.json,layout-visit-counting-rules.json}` (proposed in SPK-B005, not committed)
  - `qualification/{fixtures/**,probes/**}`
  - `qualification/schemas/sample-validity.schema.json`
  - `.constitution/tech-spec/contracts/qualification-lock.json`
  - `xtask/{src/commands/**,src/contracts/**}`
  - `crates/oxyflut-qualification/**`
- **Scope (Out-of-Scope Files):**
  - Candidate adapter crates, the native bridge, shared application-runtime crates, macOS and Windows captures, and every file owned by Epic F or Epic G (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if an input requires an unapproved schema or an external reference-host capture; leave the lock field open and route the schema gap to Stage 3."
  - "STOP if a staged byte stream differs from the canonical SHA-256 stated by its spike; reject it before lock binding."
- **Description:** Define the reference application, scenes, interaction scripts, fonts, assets, window matrix, cache states, and release flags as `workload.*` lock inputs. Stage `fuzzCorpora`, `securityPatchRehearsal`, `sampleValidityRules`, and `layoutVisitCap` inputs at the boundaries approved by SPK-B005 and SPK-B006. Preserve the proposed SPK-B005 corpus and counting-rule SHA-256 values `4972e43333984047b5a1d84200d5b89a29c5b59e47c5aca8773379320f2c6c84` and `6cd0d7c7b06587525d9127f15cceecdd6f9c21b8a62be93c70c9b3756ca459c2`, plus the SPK-B006 proposed, not committed, `fuzz-corpora.json` and `security-patch-rehearsal.json` values `59f239e1e9dffbca7eb9d15be6cb69139435a74d4d86b0c6d8e0ddcc1b93b80d` and `82037d5fd08495aee0ff2a2e7e8a4b9ade4c2f76b65f966586a5872667d9bd`.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
T8.3 uses POLICY_FIELDS and digests::validate_workspace for fuzzCorpora and securityPatchRehearsal.
T8.4 uses the layout-prequalification validator, POLICY_FIELDS, and digests::validate_workspace for the canonical layout artifacts.
The lock preserves layoutVisitCap as a known unknown (KU) until approved timing evidence exists.
The staged layout corpus and counting rules match their canonical SHA-256 values before lock binding, and the workload fields identify reference inputs without creating scored measurement evidence.
```

#### OXY-E008 Set Linux candidate-adapter readiness

- **Type:** Chore
- **Effort:** 5
- **Dependencies:** OXY-E004, OXY-E005, OXY-E006, OXY-E007
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/contracts/qualification-lock.json`
  - `qualification/{fixtures/readiness/**,fixtures/contracts/**,staged/**}`
  - `xtask/src/{commands/lock.rs,commands/lock_tests.rs,contracts/readiness.rs}`
  - `crates/oxyflut-qualification/src/readiness.rs`
- **Scope (Out-of-Scope Files):**
  - Candidate adapter crates, the native bridge, shared application-runtime crates, macOS and Windows capture files, and every file owned by Epic F or Epic G (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- lock status --gate candidate-implementation`
- **Expected Success Output:** `cargo +1.98.0 run -p xtask -- lock status --gate candidate-implementation --environment wayland` exits `0`, and the matching X11 command exits `0`; macOS and Windows exit `2`.
- **STOP Conditions:**
  - "STOP if any Linux pre-implementation input remains a KU or has a digest mismatch; do not set candidateImplementationReady."
  - "STOP if the change would require non-null scoringAnchors or assessors for one-candidate provisional selection; retain those fields as null and record the blocked assessor input."
- **Description:** Set `candidateImplementationReady` only for `wayland-linux-x86_64` and `x11-linux-x86_64` after their complete per-environment evidence passes the lock gate. Retain macOS and Windows as valid-but-open. Keep `scoringAnchors` and `assessors` null, because the blocked assessor input is non-gating for a one-candidate provisional selection under the substrate-selection policy.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
lock status reports candidate-implementation ready for Wayland and X11 without changing measurementReady.
lock status returns exit 2 for macOS and Windows without claiming their readiness.
The lock records null scoringAnchors and assessors with the blocked assessor input, and it makes no final selection claim.
```
