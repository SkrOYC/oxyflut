# Epic E: Specification landings and Linux readiness

- **Status:** Active.
- **Total effort:** 79 points.
- **Plane:** Qualification.
- **Disjointness:** This epic owns the listed qualification specifications, fixtures, evidence, lock inputs, and `xtask` files. It excludes `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates.
- **Boundary:** This epic lands the nonblocked reconciliation checklist rows and prepares per-environment candidate-adapter readiness. It does not collect comparative measurements, select a rendering substrate, or promote Phase 3B.

## Tickets

#### OXY-E001 Reassign readiness-input owners

- **Type:** Chore
- **Effort:** 2
- **Dependencies:** None
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `crates/oxyflut-qualification/src/readiness.rs`
  - `xtask/src/commands/lock_tests.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if an owner or binding lacks an approved Stage 3 definition; route the gap to Stage 3."
- **Description:** Apply T5.2a. Reassign the named `POLICY_FIELDS` and `KNOWN_UNKNOWN_BINDINGS` owners without changing their enforcing-check names or the historical report assertions.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
POLICY_FIELDS, KNOWN_UNKNOWN_BINDINGS, candidate_report_lines_are_stable_and_content_free, and staged_input_registry_binds_every_pathless_measurement_policy_digest preserve their stated checks while assigning the checklist owners.
```

#### OXY-E002 Land foundational schema and inventory types

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** None
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/data-models/capability-traceability.schema.json`
  - `.constitution/tech-spec/data-models/specification-phase.schema.json`
  - `.constitution/tech-spec/data-models/raw-measurement.schema.json`
  - `.constitution/tech-spec/data-models/ingress-inventory.schema.json`
  - `xtask/src/contracts/schema.rs`
  - `xtask/src/contracts/traceability/mod.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a physical contract-test location or typed evidence reference lacks an approved Stage 3 definition; route the gap to Stage 3."
- **Description:** Apply T1.1 and T1.6. Add the approved traceability, promotion-evidence, raw-measurement, and environment-inventory shapes before their instances and assertions land.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
discover_contract_instances validates self-declared raw measurements and traceability contract-test locations. POLICY_FIELDS and LOCK_SCHEMA validate the typed environment inventory.
```

#### OXY-E003 Land semantic-role schemas and fixture corpora

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-E002
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/data-models/semantic-role-registry.schema.json` (proposed in SPK-B001, not committed)
  - `.constitution/tech-spec/data-models/semantic-role-registry-snapshot.schema.json` (proposed in SPK-B001, not committed)
  - `qualification/fixtures/contracts/semantic-role-registry/` (proposed in SPK-B001, not committed)
  - `qualification/fixtures/contracts/semantic-role-registry-snapshot/` (proposed in SPK-B001, not committed)
  - `xtask/src/contracts/schema.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if the schema and fixture-directory sets cannot land together; route the gap to Stage 3."
- **Description:** Apply T1.2 and T2.1 as one coupled schema and corpus migration. Add schema-valid and schema-invalid registry and snapshot fixtures with expected sidecars.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
run_fixture_corpus accepts the semantic-role registry and snapshot directory sets only when each schema has the required valid and invalid corpus.
```

#### OXY-E004 Freeze the deferred version and digest migration

- **Type:** Chore
- **Effort:** 3
- **Dependencies:** OXY-E001, OXY-E002, OXY-E003, OXY-E005, OXY-E006, OXY-E007, OXY-E009, OXY-E010, OXY-E011, OXY-E012, OXY-E013, OXY-E014, OXY-E015, OXY-E016, OXY-E017, OXY-E018, OXY-E019, OXY-E020, OXY-E021, OXY-E022, OXY-E023, OXY-E024, OXY-E025, OXY-G008
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/changelog.md`
  - `.constitution/tech-spec/contracts/qualification-lock.json`
  - `.constitution/tech-spec/contracts/oxyflut-substrate.h`
  - `qualification/fixtures/generated-bindings/oxyflut-substrate.rs`
  - `qualification/fixtures/generated-bindings/oxyflut-substrate.rs.sha256`
  - `qualification/fixtures/native/interface.json`
  - `qualification/fixtures/native/layout-probe.c.in`
  - `qualification/fixtures/native/layout.x86_64-unknown-linux-gnu.json`
  - `xtask/src/contracts/digests.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if any digest-bound input changes after its dependent artifact is frozen; return to the input ticket and regenerate the complete digest chain."
- **Description:** Apply T6.1-T6.3 and freeze T8 artifacts after every digest-affecting landing, including external fixtures, Linux captures, the approved baseline, layout-cap evidence, and every split schema, corpus, instance, and assertion migration. Advance the integrated C ABI and regenerate the committed layout and bindings artifacts only after their source contracts settle.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
validate_workspace and digests::validate_workspace accept one active specification version and every regenerated dependent digest. validate_interface, layout validation, validate_bindings, and the generated-role contract test accept ABI 11 and reject ABI 10 before callbacks install.
```

#### OXY-E005 Preserve macOS, Wayland, and X11 external fixtures

- **Type:** Chore
- **Effort:** 5
- **Dependencies:** None
- **Category:** Dependency-Upgrade
- **Scope (In-Scope Files):**
  - `qualification/fixtures/external-contracts/macos/` (proposed in SPK-B001, not committed)
  - `qualification/fixtures/external-contracts/wayland/` (proposed in SPK-B003, not committed)
  - `qualification/fixtures/external-contracts/x11/` (proposed in SPK-B004, not committed)
  - `xtask/src/commands/external_contracts.rs`
  - `xtask/src/commands/external_contracts_tests.rs`
- **Scope (Out-of-Scope Files):**
  - `qualification/fixtures/external-contracts/windows/` (proposed in SPK-B002, not committed; don't touch), `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- external-contracts verify`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a canonical upstream byte source is unavailable or its license data is incomplete; preserve no proxy body and record the blocked source."
  - "STOP if a sidecar or its sibling digest differs from canonical source bytes; reject the fixture."
- **Description:** Apply T2.6.1-T2.6.2. Use the permitted network fetch to preserve the macOS canonical-URL-list fixtures, 11 Wayland fixtures, and 15 X11 fixtures as regular files with `<FIXTURE>.source.json` sidecars. Keep the Windows excerpt set outside this ticket.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
external-contracts verify accepts each expected macOS, Wayland, and X11 fixture set only when every regular fixture has a valid sidecar. The validator rejects a missing sidecar, a mismatched sibling digest, an extra fixture, a missing fixture, a noncanonical retrieval URL, or incomplete license data.
```

#### OXY-E006 Capture Linux reference-environment projections

- **Type:** Chore
- **Effort:** 3
- **Dependencies:** OXY-E011
- **Category:** DX
- **Scope (In-Scope Files):**
  - `qualification/fixtures/environments/wayland/`
  - `qualification/fixtures/environments/x11/`
  - `qualification/tools/native-contract-toolchain.json`
  - `.constitution/tech-spec/contracts/qualification-lock.json`
  - `xtask/src/commands/environment/mod.rs`
  - `xtask/src/commands/lock.rs`
  - `xtask/src/commands/lock_tests.rs`
- **Scope (Out-of-Scope Files):**
  - `qualification/fixtures/environments/macos/`, `qualification/fixtures/environments/windows/`, `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- environment inspect --environment ENVIRONMENT --output PATH`
- **Expected Success Output:** `exit 0` for the Wayland and X11 captures.
- **STOP Conditions:**
  - "STOP if a capture is not from `thinkpadp14s`, or if it lacks a Nix system-closure digest; do not substitute an Ubuntu package snapshot."
  - "STOP if a resolved tool lacks an exact matching record in `qualification/tools/native-contract-toolchain.json`; retain the gate as open."
- **Description:** Apply T3.3c. Capture the Wayland and X11 projections from `thinkpadp14s`, bind `systemPackageLockDigest` from the `/run/current-system` `narHash`, and preserve the NixOS, AMD Renoir, Hyprland, Xwayland, Xvfb, and session-family-risk fields required by the technical stack.
- **Acceptance:**
  - **Mode:** hitl_sil
  - **Evidence:**

```text
Run environment inspect with environment=wayland and environment=x11 and a locked repository-relative PATH. Each accepted projection records the required reference-environment identities and the Nix system-closure digest. validate_lock_environment_projection rejects missing identity fields and unclassified tools.
```

#### OXY-E007 Define and publish the approved capability baseline

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-E002
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `qualification/fixtures/baselines/approved-capability-baseline.json`
  - `qualification/evidence/baselines/`
  - `.constitution/tech-spec/contracts/qualification-lock.json`
  - `xtask/src/commands/baseline.rs`
  - `xtask/src/commands/lock_tests.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- baseline validate --input PATH --output PATH`
- **Expected Success Output:** `exit 0` and a canonical baseline artifact with its provenance sidecar.
- **STOP Conditions:**
  - "STOP if the project owner has not recorded approval in the baseline provenance fields; do not bind the baseline to the lock."
  - "STOP if the baseline does not contain exactly the approved 52-capability candidate-neutral set; reject publication."
- **Description:** Create the approved 52-capability candidate-neutral baseline. Validate its input with `cargo +1.98.0 run -p xtask -- baseline validate --input PATH`, publish the canonical artifact with the `--output` form, record project-owner approval in provenance, and bind the published digest as `measurementPolicy.capabilityBaseline`.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
baseline validate accepts the approved 52-capability input and the published canonical artifact. The provenance sidecar binds the source and published digests, and lock validation rejects synthetic, unapproved, partial, or mismatched baseline references.
```

#### OXY-E008 Set Linux candidate-adapter readiness in declared order

- **Type:** Chore
- **Effort:** 3
- **Dependencies:** OXY-E004, OXY-E005, OXY-E006, OXY-E007, OXY-E015, OXY-E024, OXY-G008
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/contracts/qualification-lock.json`
  - `qualification/fixtures/readiness/complete.synthetic.json`
  - `qualification/fixtures/readiness/invalid.json`
  - `qualification/fixtures/readiness/cleared-without-evidence.json`
  - `xtask/src/commands/lock.rs`
  - `xtask/src/commands/lock_tests.rs`
  - `xtask/src/contracts/readiness.rs`
  - `crates/oxyflut-qualification/src/readiness.rs`
- **Scope (Out-of-Scope Files):**
  - `qualification/fixtures/environments/macos/`, `qualification/fixtures/environments/windows/`, `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- lock status --gate candidate-implementation --environment ENVIRONMENT`
- **Expected Success Output:** The Wayland command exits `0`; after the Wayland gate is ready, the X11 command exits `0`; the macOS and Windows commands exit `2`.
- **STOP Conditions:**
  - "STOP if a required Wayland or X11 pre-implementation input remains a KU or has a digest mismatch; do not set that environment's candidateImplementationReady."
  - "STOP if the readiness change would claim measurementReady, a provisional selection, or a final selection."
- **Description:** Set `candidateImplementationReady` for Wayland after complete Wayland evidence passes its gate, then set it for X11 after complete X11 evidence passes its gate. The approved baseline, workload inputs, Linux projections, external fixtures, lock v6 enforcement, and null-substrate layout-cap evidence are planned prerequisites. `scoringAnchors` and `assessors` do not gate one-candidate provisional selection; they become measurement inputs only if two substrate candidates enter qualification.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
cargo +1.98.0 run -p xtask -- lock status --gate candidate-implementation --environment wayland exits 0 before the matching x11 command exits 0. cargo +1.98.0 run -p xtask -- lock status --gate candidate-implementation --environment macos and the matching windows command exit 2. No environment claims measurementReady or a selection.
```

#### OXY-E009 Land the accessibility-map schema and corpus migration

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-E003
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/data-models/accessibility-map.schema.json`
  - `qualification/fixtures/contracts/accessibility-map/`
  - `qualification/fixtures/contracts/migration/accessibility-map-v5-to-v6.input.json` (proposed in SPK-B001, not committed)
  - `qualification/fixtures/contracts/migration/accessibility-map-v5-to-v6.expected.json` (proposed in SPK-B001, not committed)
  - `xtask/src/contracts/schema.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if keyed-role provenance or text-layout generation needs a term or contract absent from Stage 3; route the gap to Stage 3."
- **Description:** Apply T1.3 and T2.2 as one schema and corpus migration. Add keyed forward roles, registry provenance, text-layout generation, supersession coverage, and the required migration pair.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
ACCESSIBILITY_MAP_SCHEMA, run_fixture_corpus, discover_contract_instances, and validate_migration_fixture accept the v6 corpus and reject invalid keyed-role, provenance, and migration inputs.
```

#### OXY-E010 Land the layout prequalification schemas, corpus, and validator

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-E002
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/data-models/layout-qualification-record.schema.json` (proposed in SPK-B005, not committed)
  - `.constitution/tech-spec/data-models/layout-prequalification-run.schema.json` (proposed in SPK-B005, not committed)
  - `.constitution/tech-spec/data-models/layout-prequalification-suite.schema.json` (proposed in SPK-B005, not committed)
  - `qualification/fixtures/contracts/layout-qualification-record/` (proposed in SPK-B005, not committed)
  - `qualification/fixtures/contracts/layout-prequalification-run/` (proposed in SPK-B005, not committed)
  - `qualification/fixtures/contracts/layout-prequalification-suite/` (proposed in SPK-B005, not committed)
  - `qualification/fixtures/layout-prequalification/` (proposed in SPK-B005, not committed)
  - `xtask/src/commands/layout_prequalification.rs`
  - `xtask/src/commands/mod.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- layout-prequalification validate --lock LOCK_PATH --corpus CORPUS_PATH --suite-schema SUITE_SCHEMA_PATH --suite SUITE_PATH --output RESULT_PATH`
- **Expected Success Output:** `exit 0` for the valid corpus and `exit 1` for every invalid fixture.
- **STOP Conditions:**
  - "STOP if the null-substrate candidate shape or nullable paint-submission field lacks an approved Stage 3 schema definition; route the gap to Stage 3."
- **Description:** Apply T1.5, T2.3, and T3.4b in one coupled landing. Create the three layout schemas, their schema corpus, and the custom validator. Include the D7 null-substrate candidate, separate ordinary and attempted ordinary visits, intrinsic queries, application-owned layout time, and paint-submission time that is not applicable for the null substrate.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
run_fixture_corpus accepts the three layout schema corpora. layout-prequalification validate checks raw-byte digests, corpus-derived counters and outcomes, contiguous transactions, aggregate arithmetic, and complete suite identities while rejecting every invalid custom fixture.
```

#### OXY-E011 Migrate the qualification lock and its fixture corpus

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-E010
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/data-models/qualification-lock.schema.json`
  - `qualification/fixtures/contracts/qualification-lock/`
  - `qualification/fixtures/contracts/migration/qualification-lock-v5-to-v6.input.json` (proposed in SPK-B005, not committed)
  - `qualification/fixtures/contracts/migration/qualification-lock-v5-to-v6.expected.json` (proposed in SPK-B005, not committed)
  - `qualification/fixtures/contracts/supersession.json`
  - `xtask/src/contracts/schema.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a lock field, migration pair, or readiness fixture lacks an approved v6 definition; route the gap to Stage 3."
- **Description:** Apply T1.4, T2.4, and T2.5. Migrate the lock schema and its fixture corpus together, type the layout fields and declared candidate and environment sequence, retain false readiness where evidence is absent, and exclude only the custom layout-prequalification corpus from schema-directory equality.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
LOCK_SCHEMA, run_fixture_corpus, validate_migration_fixture, and schema_compiles_committed_contract_instances_and_fixture_corpus accept the v6 lock corpus and reject v5 or incomplete layout-field inputs.
```

#### OXY-E012 Land semantic-role contracts and the registry artifact

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-E003, OXY-E009
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/contracts/semantic-role-registry.json` (proposed in SPK-B001, not committed)
  - `.constitution/tech-spec/contracts/capability-traceability.json`
  - `.constitution/tech-spec/contracts/oxyflut-public.rs`
  - `.constitution/tech-spec/contracts/oxyflut-substrate.rs`
  - `.constitution/tech-spec/contracts/oxyflut-substrate.h`
  - `qualification/fixtures/external-contracts/accessibility/candidate-neutral-role-registry.json` (proposed in SPK-B001, not committed)
  - `qualification/fixtures/external-contracts/accessibility/candidate-neutral-role-registry.json.sha256` (proposed in SPK-B001, not committed)
  - `xtask/src/contracts/traceability/edges.rs`
  - `xtask/src/contracts/traceability/validation.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a generated symbol, registry value, or physical test location differs across the approved contracts; route the gap to Stage 3."
- **Description:** Apply T3.1, T3.1a, and T7.1. Bind CAP-SEM to the registry, generate matching role definitions in every approved contract, and preserve the candidate-neutral registry artifact and digest outside the upstream-fixture convention.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
discover_contract_instances, validate_required_symbol_edges, and the generated-role contract test require every registry name and code in each approved contract and the candidate-neutral registry artifact.
```

#### OXY-E013 Land the platform-contract baseline

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-E005
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/contracts/platform-contracts.json`
  - `.constitution/tech-spec/stack.md`
  - `xtask/src/contracts/traceability/validation.rs`
  - `xtask/src/contracts/traceability/tests.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a platform row references a fixture that T2.6.1-T2.6.2 did not preserve; return to OXY-E005."
- **Description:** Apply T3.2 after external-fixture preservation. Land the macOS retentions, Windows edits, Wayland replacement, X11 edits, aligned GTK and AT-SPI rows, and retained Orca gate without claiming unavailable Windows canonical capture.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
validate_platform_baseline accepts only platform-contract references backed by the preserved macOS, Wayland, and X11 fixture sets and rejects a missing or mismatched reference.
```

#### OXY-E014 Land layout public-contract and inventory changes

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-E010
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/contracts/oxyflut-public.rs`
  - `.constitution/tech-spec/contracts/oxyflut-qualification.rs`
  - `.constitution/tech-spec/adrs/ADR-0005-platform-hosts.md`
  - `.constitution/tech-spec/data-models/README.md`
  - `xtask/src/commands/contracts.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a public field, harness operation, or inventory compatibility rule lacks an approved Stage 3 contract; route the gap to Stage 3."
- **Description:** Apply T3.4, T3.4a, and T3.4d. Add `attempted_ordinary_visits`, `LayoutTransactionCounters`, and `CandidateProbe::run_layout_fixture`; add the external-client Rust-contract assertion; update the platform-host ADR text; and update the durable-data inventory.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
The rust-contract family constructs LayoutResult with attempted_ordinary_visits and type-checks CandidateProbe::run_layout_fixture. The schema inventory lists the layout and semantic-role records with their approved compatibility rules.
```

#### OXY-E015 Land lock policy bindings and per-environment status

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-E006, OXY-E007, OXY-E011, OXY-E013, OXY-E022
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/contracts/qualification-lock.json`
  - `qualification/fixtures/readiness/complete.synthetic.json`
  - `qualification/fixtures/readiness/invalid.json`
  - `qualification/fixtures/readiness/cleared-without-evidence.json`
  - `crates/oxyflut-qualification/src/readiness.rs`
  - `xtask/src/contracts/readiness.rs`
  - `xtask/src/commands/lock.rs`
  - `xtask/src/commands/lock_tests.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a readiness rule needs an unapproved lock v6 field, fixture, or assertion; route the gap to Stage 3."
  - "STOP if a one-candidate provisional selection would require scoringAnchors or assessors at candidate-implementation readiness; apply the ADR-0011 rule instead."
- **Description:** Apply T3.3, T3.3a, T3.3b, T3.3d, and T3.3e with D6 and D8. Bind typed policy inputs, validate v6 lock identities, classify resolved tools, and add the Stage 3 CLI contract `lock status --gate candidate-implementation --environment ENVIRONMENT` and `lock status --gate measurement --environment ENVIRONMENT`. Invoke them through `cargo +1.98.0 run -p xtask`; they are missing until this ticket lands. Require `scoringAnchors` and `assessors` at measurement readiness only when two substrate candidates have entered qualification.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
candidate_input_issues, measurement_input_issues, POLICY_FIELDS, KNOWN_UNKNOWN_BINDINGS, validate_documents_with_attribution, validate_lock_environment_projection, and lock_tests.rs exact sets enforce the v6 policy. Each per-environment lock-status command returns 0 when ready, 2 when valid but open, and 1 when invalid; without --environment it reports every environment and returns 2 unless every environment is ready.
```

#### OXY-E016 Land migration notes, advisory policy, and baseline ownership

- **Type:** Chore
- **Effort:** 3
- **Dependencies:** OXY-E001
- **Category:** Dependency-Upgrade
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/changelog.md`
  - `.constitution/tech-spec/guidelines.md`
  - `.constitution/tech-spec/data-models/README.md`
  - `devenv.nix`
  - `devenv.lock`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if an advisory database source, digest, refresh authority, or cadence is absent; retain advisory validation as blocked."
- **Description:** Apply T3.5, including T3.5b, T3.5c, T3.5d, and T3.5e. Record the accessibility-map and qualification-lock migrations, bind the offline RustSec advisory database and refresh policy, and record the Stage 3 schema-and-typing and Stage 4 workload-and-scoring-anchor ownership split.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
baseline validate, measurement validate, generate_templates, and digests::validate_workspace preserve the migration and ownership records. The advisory command remains blocked until its pinned offline database and refresh policy validate.
```

#### OXY-E017 Reconcile known-unknown arrays and readiness assertions

- **Type:** Chore
- **Effort:** 3
- **Dependencies:** OXY-E011
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/contracts/qualification-lock.json`
  - `qualification/fixtures/readiness/invalid.json`
  - `qualification/fixtures/readiness/cleared-without-evidence.json`
  - `crates/oxyflut-qualification/src/readiness.rs`
  - `xtask/src/commands/lock_tests.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a changed KU lacks a named binding field and owner; route the gap to Stage 3."
- **Description:** Apply T4.1, T4.2, T5.1, and T5.2 as one lexicographic transaction. Reconcile active and fixture KU sets, changed bindings, owner literals, and exact assertions after their lock fields settle.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
committed_candidate_gate_is_valid_but_open_with_the_exact_ku_set, cleared_ku_without_evidence_remains_open_with_the_exact_remaining_ku_set, clearing_a_ku_string_without_its_evidence_keeps_the_gate_open, collect_known_unknowns, and candidate_report_lines_are_stable_and_content_free accept only the reconciled arrays and output.
```

#### OXY-E018 Update schema counts, traceability edges, and migration assertions

- **Type:** Chore
- **Effort:** 3
- **Dependencies:** OXY-E009, OXY-E010, OXY-E011, OXY-E012, OXY-E017
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `xtask/src/contracts/schema.rs`
  - `xtask/src/contracts/traceability/edges.rs`
  - `xtask/src/contracts/traceability/fixtures.rs`
  - `xtask/src/contracts/traceability/mod.rs`
  - `xtask/src/contracts/traceability/tests.rs`
  - `xtask/src/contracts/traceability/validation.rs`
  - `qualification/fixtures/contracts/migration/`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a count, exact set, or migration pair has no named source landing; return to that ticket."
- **Description:** Apply T5.3, T5.5, and T5.6 after the schema, corpus, and KU shapes settle. Update the 23-schema and seven-instance assertions, accessibility registry edges, and named migration-pair validation.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
schema_compiles_committed_contract_instances_and_fixture_corpus pins schema_count=23 and instance_count=7. ACCESSIBILITY_MAP_SCHEMA, REQUIRED_ACCESSIBILITY_CATEGORIES, validate_required_symbol_edges, and validate_migration_fixture reject incorrect edges, source bytes, expected bytes, and v6 migration output.
```

#### OXY-E019 Regenerate ABI and binding artifacts

- **Type:** Chore
- **Effort:** 3
- **Dependencies:** OXY-E012
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/contracts/oxyflut-substrate.h`
  - `qualification/fixtures/native/interface.json`
  - `qualification/fixtures/native/layout-probe.c.in`
  - `qualification/fixtures/native/layout.x86_64-unknown-linux-gnu.json`
  - `qualification/fixtures/generated-bindings/oxyflut-substrate.rs`
  - `qualification/fixtures/generated-bindings/oxyflut-substrate.rs.sha256`
  - `xtask/src/contracts/native_tests.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a header, layout fixture, or generated binding differs from the approved ABI contract; route the gap to Stage 3."
- **Description:** Apply T5.4, T8.1, and T8.2. Advance the C ABI to 11, regenerate the native layout fixture and generated-bindings golden after the semantic-role constants settle, and update the ABI rejection assertion.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
abi_seven_through_ten_fail_before_callbacks_install, validate_interface, layout validation, validate_bindings, and the generated-role contract test accept ABI 11 and reject every earlier ABI before callbacks install.
```

#### OXY-E020 Migrate active specification versions

- **Type:** Chore
- **Effort:** 3
- **Dependencies:** OXY-E018, OXY-E019
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/changelog.md`
  - `.constitution/tech-spec/guidelines.md`
  - `.constitution/tech-spec/stack.md`
  - `qualification/fixtures/contracts/`
  - `qualification/fixtures/readiness/`
  - `qualification/staged/`
  - `xtask/src/contracts/digests.rs`
  - `xtask/src/contracts/readiness.rs`
  - `xtask/src/contracts/schema.rs`
  - `xtask/src/commands/contracts.rs`
  - `xtask/src/commands/environment/mod.rs`
  - `xtask/src/commands/external_contracts.rs`
  - `xtask/src/commands/lock.rs`
  - `xtask/src/commands/lock_tests.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if the active-version inventory contains an unreviewed occurrence after the migration; reconcile it before regenerating parent digests."
- **Description:** Apply T6.1-T6.3 after T1-T5 shapes settle. Recount the active-version inventory, migrate every active occurrence, regenerate affected parents and sidecars, update the command and stack version references, and prepend the technical-spec release record.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
The active-version inventory has one reviewed value. Active-specification equality, validate_workspace, and digests::validate_workspace accept each regenerated parent and sidecar.
```

#### OXY-E021 Stage fuzz-corpus and security-patch records

- **Type:** Feature
- **Effort:** 2
- **Dependencies:** OXY-E011
- **Category:** Security
- **Scope (In-Scope Files):**
  - `qualification/staged/fuzz-corpora.json` (proposed in SPK-B006, not committed)
  - `qualification/staged/security-patch-rehearsal.json` (proposed in SPK-B006, not committed)
  - `.constitution/tech-spec/contracts/qualification-lock.json`
  - `crates/oxyflut-qualification/src/readiness.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a staged byte stream differs from its canonical SPK-B006 digest; reject it before lock binding."
- **Description:** Apply T8.3. Create the proposed fuzz-corpus and security-patch records, bind their digests, and preserve the campaign-host classification.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
POLICY_FIELDS and digests::validate_workspace accept only the canonical staged records and reject a changed byte stream or unclassified campaign host.
```

#### OXY-E022 Stage layout artifacts and lock bindings

- **Type:** Feature
- **Effort:** 2
- **Dependencies:** OXY-E010
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `qualification/staged/layout-visit-corpus.json` (proposed in SPK-B005, not committed)
  - `qualification/staged/layout-visit-counting-rules.json` (proposed in SPK-B005, not committed)
  - `.constitution/tech-spec/contracts/qualification-lock.json`
  - `crates/oxyflut-qualification/src/readiness.rs`
  - `xtask/src/contracts/readiness.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a corpus, counting-rule, or schema digest differs from the approved SPK-B005 bytes; re-freeze the complete dependent set."
- **Description:** Apply T8.4. Freeze the layout corpus and counting rules, bind the layout policy references, and retain `layoutVisitCap` as unresolved until OXY-G008 supplies the null-substrate suite evidence.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
POLICY_FIELDS and digests::validate_workspace accept the canonical layout artifacts and their bound schema digests. The lock rejects an altered artifact or a layoutVisitCap claim without OXY-G008 evidence.
```

#### OXY-E023 Land provisional-selection records and checks

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-E015
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/data-models/qualification-evidence.schema.json`
  - `.constitution/tech-spec/data-models/selection-decision.schema.json`
  - `qualification/fixtures/contracts/qualification-evidence/`
  - `qualification/fixtures/contracts/selection-decision/`
  - `xtask/src/contracts/readiness_promotion.rs`
  - `xtask/src/contracts/readiness_tests.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a provisional record can promote Phase 3B, remove an untriggered candidate recipe, or require scores for one entered candidate; reject the record."
- **Description:** Apply the ADR-0011 provisional-selection landing and D6. Migrate qualification evidence to record entered and not-entered environments, migrate selection decisions to record entered, untriggered, and ineligible candidates, and require assessor scores only when two candidates enter and are eligible.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Selection and qualification-evidence fixture corpora accept one entered integrated candidate and one untriggered focused candidate with a provisional state. readiness_promotion rejects premature final state, Phase 3B promotion, candidate-recipe removal, missing entered evidence, and inappropriate assessor-score requirements.
```

#### OXY-E024 Define candidate-readiness workload inputs

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-E002
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `qualification/staged/workload-reference-application.json`
  - `qualification/staged/workload-scenes.json`
  - `qualification/staged/workload-interaction-scripts.json`
  - `qualification/staged/workload-fonts.json`
  - `qualification/staged/workload-assets.json`
  - `qualification/staged/workload-window-matrix.json`
  - `qualification/staged/workload-cache-states.json`
  - `qualification/staged/workload-release-flags.json`
  - `.constitution/tech-spec/contracts/qualification-lock.json`
  - `xtask/src/commands/lock_tests.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if a workload input needs a schema or lock reference absent from Stage 3; route the gap to Stage 3."
- **Description:** Define and bind the reference application, scenes, interaction scripts, fonts, assets, window matrix, cache states, and release flags as candidate-readiness inputs. Do not record a CON-* meter value, benchmark row, comparative score, or selection result.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Lock validation accepts only digest-bound workload inputs with complete required identities. Candidate status reports an incomplete, missing, or mismatched workload input as open without producing measurement or selection evidence.
```

#### OXY-E025 Land the selection-citation migration

- **Type:** Chore
- **Effort:** 2
- **Dependencies:** OXY-E023
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `.constitution/tech-spec/adrs/ADR-0010-production-substrate.md`
  - `qualification/fixtures/contracts/readiness/production-3b/`
  - `xtask/src/contracts/readiness_promotion.rs`
  - `xtask/src/contracts/digests.rs`
- **Scope (Out-of-Scope Files):**
  - `xtask/src/commands/candidate.rs`, `xtask/src/commands/probe.rs`, `qualification/probes/`, candidate adapter crates, and shared application-runtime crates (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0`.
- **STOP Conditions:**
  - "STOP if transformed selection evidence, its schema, or its digest has not settled; retain the ADR citation and production-3b fixture cascade."
- **Description:** Apply T8.5 after the ADR-0011 evidence-schema migration. Reconcile the ADR-0010 citation and production-3b fixture cascade only with transformed evidence that passes the migrated schema. This ticket does not promote Phase 3B or claim a final selection.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
adr_cites_verified_evidence and digests::validate_workspace accept transformed evidence only when its schema, digest, and citation agree. The checks reject a provisional selection, incomplete Tier 1 evidence, or an unverified transformed artifact as a Phase 3B basis.
```
