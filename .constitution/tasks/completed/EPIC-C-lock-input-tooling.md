# Epic C: Qualification-lock input tooling

Build the reusable tools and candidate-neutral templates needed to turn research and hardware observations into immutable pre-implementation lock inputs. This epic doesn't claim that any input is complete.

#### OXY-C001 Snapshot and verify external distribution contracts

- **Type:** Security
- **Effort:** 5
- **Dependencies:** OXY-A002
- **Category:** Security
- **Scope (In-Scope Files):**
  - `qualification/schemas/external/`
  - `qualification/fixtures/external-contracts/`
  - `xtask/src/commands/external_contracts.rs`
- **Scope (Out-of-Scope Files):**
  - `.constitution/tech-spec/contracts/external-contract-lock.json` (Stage 3 reconciliation owns active pins)
  - Release artifact generation
  - Signing keys or remote verification services
- **Verification Command:** `cargo +1.98.0 test --workspace --all-features`
- **Expected Success Output:** `exit 0` with staged snapshot and verifier fixtures passing; invalid and incomplete fixture locks return exit 1 without changing the active lock
- **STOP Conditions:**
  - STOP if an upstream source isn't authoritative, immutable, license-compatible, or retrievable by a pinned reference.
  - STOP if semantic validation requires a mutable network service.
- **Description:** Replace the OXY-A001 external-contracts command placeholder. Preserve source bytes for SPDX 3.0.1, in-toto Statement v1, SLSA Provenance v1, and DSSE Envelope v1; record source identity and SHA-256; pin local verifier adapters and versions; and add positive and negative fixtures. Produce the exact proposed external-contract lock values for Stage 3 reconciliation without editing the active lock.
- **Acceptance:**
  - **Mode:** invariant
  - **Evidence:**

```text
Invariants:
- Every snapshot preserves authoritative source bytes, source identity, version, license, and SHA-256.
- Semantic verification runs entirely against local bytes with pinned verifier behavior.
- Mutated schemas, envelopes, predicates, and registry digests fail.
Checker: cargo +1.98.0 test -p xtask external_contracts
```

##### OXY-C001 Deviations & Justifications

- **Touched Files:** `crates/oxyflut-qualification/src/schema.rs`
- **Justification:** The shared offline validator must assert JSON Schema `format` values for the SLSA RFC 3339 timestamp contract.
- **Touched Files:** `xtask/src/commands/external_contracts_tests.rs`
- **Justification:** The focused external-contract test module keeps the command implementation below the repository file-size limit while exercising the required external fixtures.
- **Touched Files:** `xtask/src/commands/external_contracts/dsse.rs`.
- **Justification:** DSSE pre-authentication encoding and strict standard padded Base64 verification are isolated from snapshot validation to keep both Rust modules below the repository file-size limit.
- **Touched Files:** `.gitattributes`
- **Justification:** The authoritative SPDX schema preserves upstream CRLF bytes, so its path disables Git trailing-whitespace diagnostics without rewriting the snapshot.
- **Touched Files:** `.constitution/tasks/active/EPIC-C-lock-input-tooling.md`
- **Justification:** The execution rules require this scope-deviation record.

#### OXY-C002 Implement capability-baseline authoring and validation

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-A003, OXY-A006
- **Category:** DX
- **Scope (In-Scope Files):**
  - `xtask/src/commands/baseline.rs`
  - `crates/oxyflut-qualification/src/baseline.rs`
  - `qualification/fixtures/baselines/`
- **Scope (Out-of-Scope Files):**
  - A completed 52-capability baseline
  - Product or candidate implementation
  - `.constitution/tech-spec/data-models/capability-baseline.schema.json` (binding schema)
- **Verification Command:** `cargo +1.98.0 run -p xtask -- baseline validate --input qualification/fixtures/baselines/complete.synthetic.json && cargo +1.98.0 test -p xtask baseline`
- **Expected Success Output:** `exit 0` for the complete positive fixture and exit 1 for missing, duplicate, mismatched-flow, or empty-evidence fixtures
- **STOP Conditions:**
  - STOP if authoring requires a capability exception or an additional test-vector field not defined by Stage 3.
  - STOP if a baseline could pass without all 52 exact capability keys.
- **Description:** Replace the OXY-A001 baseline command placeholder. Implement parsing, exact-set validation, deterministic ordering, architecture-flow binding, evidence expectation validation, explicit synthetic or approved provenance, digest-bound approval evidence, and content-addressed output for candidate-neutral capability baseline drafts. Mark the complete fixture as synthetic with null approval evidence; only Stage 3 reconciliation can produce an approved baseline and matching typed lock reference.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Assertions:
- Exactly the authoritative 52 capability keys are required.
- Every entry resolves its architecture flow and has nonempty test vectors and expected evidence.
- Canonical output is deterministic and content-addressed.
- Synthetic fixtures carry `provenance.kind: synthetic` and null approval evidence. They cannot satisfy the qualification lock's typed approved-baseline reference.
Command: cargo +1.98.0 run -p xtask -- baseline validate --input qualification/fixtures/baselines/complete.synthetic.json && cargo +1.98.0 test -p xtask baseline
```

##### OXY-C002 Deviations & Justifications

- **Touched Files:** `xtask/src/contracts/traceability/mod.rs`.
- **Justification:** The baseline command uses the OXY-A003 exact PRD and architecture-flow validator through a focused authority helper instead of duplicating the 52-capability derivation.
- **Touched Files:** `crates/oxyflut-qualification/src/evidence/{mod.rs,publish.rs}`.
- **Justification:** The OXY-A006 evidence writer needs a repository-confined output-directory API so `baseline validate --output` can publish canonical content-addressed derived evidence without duplicating atomic publication.
- **Touched Files:** `.constitution/tasks/active/EPIC-C-lock-input-tooling.md`.
- **Justification:** The execution rules require this scope-deviation record.

#### OXY-C003 Implement raw-measurement and sample-validity templates

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-A002, OXY-A006
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `crates/oxyflut-qualification/src/measurement.rs`
  - `xtask/src/commands/measurement.rs`
  - `qualification/fixtures/measurements/`
  - `qualification/fixtures/sample-validity/`
- **Scope (Out-of-Scope Files):**
  - Candidate measurements
  - Statistical thresholds other than those already defined by the PRD
  - Outlier-removal logic
- **Verification Command:** `cargo +1.98.0 run -p xtask -- measurement validate --input qualification/fixtures/measurements/complete.synthetic.json && cargo +1.98.0 run -p xtask -- measurement validate --input qualification/fixtures/sample-validity/complete.synthetic.json && cargo +1.98.0 test -p xtask measurement`
- **Expected Success Output:** `exit 0` for complete templates and exit 1 for unapproved exclusions, missing raw samples, or altered meters
- **STOP Conditions:**
  - STOP if a template drops outliers or introduces an exclusion outside the three PRD categories.
  - STOP if a meter needs an unstated statistic or confidence rule; return to Stage 3.
- **Description:** Replace the OXY-A001 measurement command placeholder. Implement typed template generation and validation for raw samples, harness logs, launch/sample ordinals, monotonic times, units, admitted samples, the three permitted exclusion categories, comparison-bound calculation inputs, and source/lock attribution. Don't execute a measurement.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Assertions:
- Every raw sample and exclusion is preserved with its harness log.
- Every `(constraintId, launch, ordinal)` tuple is unique within one measurement record.
- A valid sample cannot carry an exclusion reason.
- Only measurement-tool failure, unrelated operating-system interruption, and physical disconnect are valid exclusions.
- Nearest-rank and maximum-bound inputs retain every valid observation without outlier deletion.
- Templates bind to one environment, candidate, meter version, and lock digest.
Command: cargo +1.98.0 run -p xtask -- measurement validate --input qualification/fixtures/measurements/complete.synthetic.json && cargo +1.98.0 run -p xtask -- measurement validate --input qualification/fixtures/sample-validity/complete.synthetic.json && cargo +1.98.0 test -p xtask measurement
```

##### OXY-C003 Deviations & Justifications

- **Touched Files:** `qualification/schemas/sample-validity.schema.json` and `qualification/schemas/README.md`.
- **Justification:** Stage 3 defines no sample-validity schema. The lead ruling requires a documented staged, nonauthoritative proposal under `qualification/schemas/` without editing the Stage 3 data-model directory.
- **Assumption for OXY-D001 Stage 3 revision:** `raw-measurement.schema.json` omits the `$schema` property that sibling schemas declare, so raw-measurement instances cannot self-declare their schema.
- **Assumption for OXY-D001 Stage 3 revision:** no Stage 3 schema exists for `measurementPolicy.sampleValidityRules`; the staged schema's digest is the proposed binding value.
- **Touched Files:** `crates/oxyflut-qualification/src/measurement_tests.rs`.
- **Justification:** The measurement unit tests are split from the implementation module to keep the Rust library path below the repository's hard file-size limit.
- **Touched Files:** `crates/oxyflut-qualification/src/schema.rs` and `xtask/src/contracts/schema.rs`.
- **Justification:** The local schema-registry count increases from 17 to 18 when it compiles the required staged sample-validity schema.
- **Touched Files:** `xtask/src/contracts/traceability/mod.rs`.
- **Justification:** The measurement command reuses the existing exact PRD constraint authority instead of duplicating the constraint parser.
- **Touched Files:** `.constitution/tasks/active/EPIC-C-lock-input-tooling.md`.
- **Justification:** The execution rules require this scope-deviation record and the lead-ruling assumptions.

#### OXY-C004 Implement reference-environment inspection

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-A001, OXY-A006
- **Category:** DX
- **Scope (In-Scope Files):**
  - `xtask/src/commands/environment/mod.rs`
  - `xtask/src/commands/environment/*.rs`
  - `crates/oxyflut-qualification/src/environment.rs`
  - `qualification/fixtures/environments/`
- **Scope (Out-of-Scope Files):**
  - Candidate build or probe logic
  - Hardware provisioning
  - `.constitution/tech-spec/contracts/qualification-lock.json` (don't mark readiness automatically)
- **Verification Command:** `cargo +1.98.0 test --workspace --all-features`
- **Expected Success Output:** `exit 0` with fixture-backed collectors for macOS, Windows, Wayland, and X11
- **STOP Conditions:**
  - STOP if collection requires private user content, stable user identity, or an unbounded system inventory.
  - STOP if a platform value can't be obtained from an authoritative API or package database; emit a typed missing value.
- **Description:** Replace the OXY-A001 environment command placeholder with testable platform collectors. Capture operating-system and minimum-version evidence, architecture, hardware and GPU identifiers, driver versions, compiler and SDK identities, compositor/session/protocol versions, and a content-bounded system-package lock. Output schema-valid evidence without changing readiness.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Assertions:
- Each Tier 1 collector emits the same candidate-neutral environment shape.
- Output contains only the fields permitted by the qualification lock and no user content.
- Missing or unsupported fields remain explicit and never receive guessed defaults.
- Fixture-backed platform adapters make collection behavior deterministic in CI.
Commands:
- cargo +1.98.0 test --workspace --all-features
- cargo +1.98.0 run -p xtask -- environment inspect --environment ENVIRONMENT --output PATH
```

##### OXY-C004 Deviations & Justifications

- **Touched Files:** `xtask/Cargo.toml`, `Cargo.lock`.
- **Justification:** Fixture-backed collectors deserialize bounded raw platform responses through the same collector parsers as live sources; the existing stack-pinned `serde` dependency supplies those derives and records xtask's direct use in the workspace lockfile.
- **Touched Files:** `.constitution/tasks/active/EPIC-C-lock-input-tooling.md`.
- **Justification:** The execution rules require this scope-deviation record.
- **OXY-D001 input:** macOS `compositor`, `protocolVersion`, and `driverVersion`, plus Windows `compositor`, `session`, and `protocolVersion`, require a bounded manual capture because no authoritative content-free CLI provides them. The collectors emit `missing { reason: manual-capture }` rather than inferring a value.

#### OXY-C005 Implement the pre-implementation readiness report

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-A004, OXY-A006, OXY-C001, OXY-C002, OXY-C003, OXY-C004
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `xtask/src/commands/lock.rs`
  - `crates/oxyflut-qualification/src/readiness.rs`
  - `qualification/fixtures/readiness/`
- **Scope (Out-of-Scope Files):**
  - Automatic edits to either readiness flag
  - Candidate implementation
  - Phase 3B promotion
- **Verification Command:** `cargo +1.98.0 run -p xtask -- lock status --gate candidate-implementation`
- **Expected Success Output:** `exit 2` with the exact pre-implementation KU set in the committed lock; complete synthetic fixtures return exit 0 and invalid fixtures return exit 1
- **STOP Conditions:**
  - STOP if the command would mutate the lock or infer readiness from the absence of an error.
  - STOP if a KU can't be tied to an exact required field, evidence path, or upstream decision.
- **Description:** Replace the OXY-A001 lock command placeholder with the read-only readiness command and stable content-free report. Distinguish an invalid lock from a valid open gate, list exact blocking fields and KUs, verify staged input digests, and prove that clearing a string without supplying its evidence doesn't change readiness.
- **Acceptance:**
  - **Mode:** invariant
  - **Evidence:**

```text
Invariants:
- Exit 0 means every pre-implementation field and digest validates and the flag is already true.
- Exit 2 means the lock is valid but one or more exact inputs remain open.
- Exit 1 means the lock or a referenced input is invalid.
- The command never changes the lock or either readiness flag.
Checker: cargo +1.98.0 run -p xtask -- lock status --gate candidate-implementation
```

##### OXY-C005 Deviations & Justifications

- **Touched Files:** `xtask/src/contracts/readiness.rs`, `xtask/src/toolchain/{lock,error}.rs`, `xtask/src/toolchain.rs`, `xtask/src/commands/lock_tests.rs`, and `.constitution/tasks/active/EPIC-C-lock-input-tooling.md`.
- **Justification:** The command reuses the Foundation readiness validator and staged-toolchain verifier. The focused test module keeps the command implementation within the repository file-size guidance. This ticket records the required scope deviation and OXY-D001 input.
- **Touched Files:** `.constitution/tech-spec/guidelines.md`, `.constitution/tasks/completed/EPIC-C-lock-input-tooling.md`.
- **Justification:** PR review required factual command descriptions and a durable record of the manual-capture lock input after Epic C moved to completed tasks.
- **OXY-D001 input:** `qualification-lock.schema.json` binds `measurementPolicy.{scoringAnchors,assessors,fuzzCorpora,securityPatchRehearsal}` as path-less digests; the repository convention `qualification/staged/<field>.json` is proposed as their referent and should be typed by Stage 3.
- **External-lock decision:** The external lock has per-contract `epistemicStatus` values rather than one root status. When every active contract is `ku-gating`, readiness verifies the staged proposal. Otherwise, readiness verifies the active lock.

## Completion

Epic C completed its 21 story points without claiming that any staged input is complete or changing either readiness flag.

### Ticket completion

| Ticket | Commits | Verification result | Deviations and assumptions |
| :-- | :-- | :-- | :-- |
| OXY-C001 | `a731182446550355898c6824ae25ee923346fb6d`, `316f93254ee500a76e1794aafde57d888b5f67fc` | `cargo +1.98.0 test -p xtask external_contracts` passed with exit 0. | Extended shared format validation, split focused command tests, and preserved authoritative SPDX CRLF bytes; see [deviations](#oxy-c001-deviations--justifications). |
| OXY-C002 | `2fb134ed3be6e10a353ada4ab14557a281d01d87`, `81ede6ff048b1e9b3c699545704d45bf6c9c4333` | `cargo +1.98.0 run -p xtask -- baseline validate --input qualification/fixtures/baselines/complete.synthetic.json && cargo +1.98.0 test -p xtask baseline` passed with exit 0. | Reused the exact traceability authority and evidence writer to publish baseline drafts; see [deviations](#oxy-c002-deviations--justifications). |
| OXY-C003 | `383021aae1fd1a9842a4fe7fcda3e642b4162455`, `3831555703ab6c09ef2f355dedd6ca7fdf20ae04` | `cargo +1.98.0 run -p xtask -- measurement validate --input qualification/fixtures/measurements/complete.synthetic.json && cargo +1.98.0 run -p xtask -- measurement validate --input qualification/fixtures/sample-validity/complete.synthetic.json && cargo +1.98.0 test -p xtask measurement` passed with exit 0. | Added a nonauthoritative staged schema and shared registry entry; the missing raw-measurement declaration and sample-validity contract are routed below; see [deviations](#oxy-c003-deviations--justifications). |
| OXY-C004 | `fd32fb9565438aae4eeb47254c7dcece296acd23`, `3b0fc3ad18eaff0117ce67878315964e657c359b` | `cargo +1.98.0 test --workspace --all-features` passed with exit 0. | Added fixture-backed collector deserialization and a bounded projection-plus-inventory artifact pair; the companion artifact needs a Stage 3 schema; see [deviations](#oxy-c004-deviations--justifications). |
| OXY-C005 | `05ee6f110e83c008c2252929f562c1a33e6fbed6`, `c0015f5bcf0c5e645275a08534ff71315a5ba484` | `cargo +1.98.0 run -p xtask -- lock status --gate candidate-implementation` returned the required exit 2 for the valid, open committed lock. | Reused readiness and staged-toolchain validation; path-less measurement-policy inputs and staged tool paths are routed below; see [deviations](#oxy-c005-deviations--justifications). |

### Stage 3 revisions required — routed to OXY-D001

The [OXY-D001 inputs from Epic C](../active/EPIC-D-readiness-reconciliation.md#oxy-d001-inputs-from-epics-a-and-c) must name these unresolved Stage 3 revisions:

- `.constitution/tech-spec/data-models/raw-measurement.schema.json` omits the `$schema` property, so raw-measurement instances cannot self-declare their schema.
- No Stage 3 schema defines `qualification-lock.schema.json#measurementPolicy.sampleValidityRules`; `qualification/schemas/sample-validity.schema.json` is the proposed staged schema and its digest is the proposed binding value.
- The proposed external-contract lock values in `qualification/schemas/external/proposed-external-contract-lock.json` await Stage 3 adoption.
- `xtask environment inspect` writes the `PATH.inventory.json` companion artifact, but no Stage 3 schema defines it and `qualification-lock.schema.json#referenceEnvironments` has no typed reference to it.
- `qualification-lock.schema.json#measurementPolicy.{scoringAnchors,assessors,fuzzCorpora,securityPatchRehearsal}` binds path-less digests; the repository convention `qualification/staged/<field>.json` is the proposed referent and needs Stage 3 typing.
- `qualification-lock.schema.json#resolvedTools` lacks the `pathRoot` field used by `qualification/tools/native-contract-toolchain.json` for rustup-home-relative tools.
