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
- **Verification Command:** `cargo +1.98.0 run -p xtask -- baseline validate --input PATH`
- **Expected Success Output:** `exit 0` for the complete positive fixture and exit 1 for missing, duplicate, mismatched-flow, or empty-evidence fixtures
- **STOP Conditions:**
  - STOP if authoring requires a capability exception or an additional test-vector field not defined by Stage 3.
  - STOP if a baseline could pass without all 52 exact capability keys.
- **Description:** Replace the OXY-A001 baseline command placeholder. Implement parsing, exact-set validation, deterministic ordering, architecture-flow binding, evidence expectation validation, and content-addressed output for candidate-neutral capability baseline drafts. Include a synthetic complete fixture without pretending it is the approved product baseline.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Assertions:
- Exactly the authoritative 52 capability keys are required.
- Every entry resolves its architecture flow and has nonempty test vectors and expected evidence.
- Canonical output is deterministic and content-addressed.
- Synthetic fixtures are visibly marked and cannot be referenced by the qualification lock.
Command: cargo +1.98.0 run -p xtask -- baseline validate --input PATH
```

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
- **Verification Command:** `cargo +1.98.0 run -p xtask -- measurement validate --input PATH`
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
Command: cargo +1.98.0 run -p xtask -- measurement validate --input PATH
```

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
