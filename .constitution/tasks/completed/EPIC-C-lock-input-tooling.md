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
- **Format assertions:** Format assertions (`uri`, `date-time`) are enforced for all registry schemas, including `qualification-lock`, `external-contract-lock`, and `ci-invocation`.
- **Touched Files:** `xtask/src/commands/external_contracts_tests.rs`
- **Justification:** The focused external-contract test module keeps the command implementation below the repository file-size limit while exercising the required external fixtures.
- **Touched Files:** `xtask/src/commands/external_contracts/dsse.rs`.
- **Justification:** DSSE pre-authentication encoding and strict standard padded Base64 verification are isolated from snapshot validation to keep both Rust modules below the repository file-size limit.
- **Touched Files:** `.gitattributes`
- **Justification:** The authoritative SPDX schema preserves upstream CRLF bytes, so its path disables Git trailing-whitespace diagnostics without rewriting the snapshot.
- **Touched Files:** `.constitution/tasks/active/EPIC-C-lock-input-tooling.md`
- **Justification:** The execution rules require this scope-deviation record.
- **Touched Files:** `qualification/schemas/README.md`.
- **Justification:** The fixture-only DSSE verifier identity requires an explicit OXY-D001 replacement with a production signature scheme before adoption.

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
- **Justification:** The OXY-A006 evidence writer provides the repository-confined `write_canonical_json_to_path` API that `baseline validate --output` uses to publish canonical content-addressed derived evidence without duplicating atomic publication.
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
- **Lead ruling for OXY-D001 Stage 3 revision:** the PRD launch and per-launch observation budgets belong to the measurement-harness contract. The staged sample-validity record deliberately doesn't bind them.
- **Lead ruling for OXY-D001 Stage 3 revision:** `monotonicNs` is non-decreasing per `(constraintId, launch)` because each launch uses its own monotonic clock. The raw-measurement schema must state this scope.
- **Lead ruling for OXY-D001 Stage 3 revision:** Template generation (`generate_templates`, `compute_comparison_bounds`) is intentionally library-only until the measurement-harness contract exists.
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
- **OXY-D001 input:** macOS `compositor`, `protocolVersion`, and `driverVersion`, plus Windows `compositor`, `session`, and `protocolVersion`, require a bounded manual capture because no authoritative content-free CLI provides them. Linux `protocolVersion` requires the same capture only when `wayland-info` or `xdpyinfo` is unavailable or unparseable. Output that exceeds the collector bound emits `missing { reason: inventory-exceeds-bound }` without parsing a partial response; otherwise the collectors emit `missing { reason: manual-capture }` rather than inferring a value.
- **Lead ruling:** The lead host cannot measure `wayland-info` or `xdpyinfo` output. Only those protocol sources use a provisional 256 KiB capture bound; every other command keeps `COMMAND_OUTPUT_LIMIT`. Truncated output remains fail-closed. OXY-D001 must confirm the bound against real Ubuntu 26.04 output sizes.
- **Exact-version rule:** The macOS operating-system pin matches `sw_vers -productVersion` exactly. A `26.5.1` host fails closed rather than matching the `macos-26.5` pin by prefix.
- **Session fail-closed rule:** Wayland and X11 collection rejects every missing session value before publishing either artifact.
- **Investigate ruling:** Linux `driverVersion` emits a Mesa package pairing only for allowlisted Mesa-backed kernel drivers: `amdgpu`, `i915`, `xe`, `nouveau`, and `radeon`. Other drivers emit `missing { reason: unsupported-by-source }`.
- **OXY-D001 input:** A partial Wayland interface observation remains in the companion inventory because lock v5 has no `protocolVersion` field. Stage 3 must define the required interface-set completeness rule before the lock can adopt this evidence.

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

- **Touched Files:** `xtask/src/contracts/{readiness,readiness_tests}.rs`, `xtask/src/toolchain/{lock,error}.rs`, `xtask/src/toolchain.rs`, `xtask/src/commands/{contracts,lock_tests}.rs`, `qualification/fixtures/{readiness,contracts/readiness}/`, and `.constitution/tasks/{active,completed}/EPIC-C-lock-input-tooling.md`.
- **Justification:** The command reuses the Foundation readiness validator and staged-toolchain verifier. Rustup-rooted fixture tools retain the manifest-relative path, while fixture loaders resolve only that path through the manifest `pathRoot` on the current host. Nix store paths remain byte-for-byte absolute. `qualification/fixtures/readiness/README.md` documents this choice, and affected immutable fixture references were refreshed. This ticket records the required scope deviation and OXY-D001 input.
- **Touched Files:** `qualification/fixtures/contracts/readiness/{ready,production-3b}/`.
- **Justification:** Each fixture lock with nonempty `resolvedTools` now stages the exact manifest and absolute entries. Dependent immutable fixture digests were refreshed after this input changed.
- **Touched Files:** `.constitution/tech-spec/guidelines.md`, `.constitution/tasks/completed/EPIC-C-lock-input-tooling.md`.
- **Justification:** PR review required factual command descriptions and a durable record of the manual-capture lock input after Epic C moved to completed tasks.
- **OXY-D001 input:** `qualification-lock.schema.json` binds `measurementPolicy.{scoringAnchors,assessors,fuzzCorpora,securityPatchRehearsal}` as path-less digests; the repository convention `qualification/staged/<field>.json` is proposed as their referent and should be typed by Stage 3.
- **External-lock decision:** The external lock has per-contract `epistemicStatus` values rather than one root status. When every active contract is `ku-gating`, readiness verifies the staged proposal. Otherwise, readiness verifies the active lock.
- **Touched Files:** `xtask/src/main.rs`, `.constitution/tech-spec/changelog.md`, `.constitution/tasks/active/EPIC-D-readiness-reconciliation.md`, `.constitution/tasks/completed/EPIC-C-lock-input-tooling.md`, and `.constitution/tasks/changelog.md`.
- **Justification:** PR review required removal of obsolete command-outcome allowances, correction of the staged SLSA license record, explicit OXY-D001 ownership gaps, and complete Epic C review records.
- **Touched Files:** `.constitution/tasks/{critical-path,changelog}.md`.
- **Justification:** PR review round 3 aligned the active-plan version with its changelog and recorded the review corrections without changing qualification readiness.
- **Touched Files:** `xtask/src/commands/contracts.rs`, `xtask/src/contracts/{readiness,readiness_tests}.rs`.
- **Justification:** PR review round 10 restores fail-closed contract validation for an unverifiable tool host, while preserving the lock command's typed valid-but-open report and validating promotion before the final host check.
- CI run for `63eafb2` was red (developer-host rustup paths in readiness fixtures); corrected in `ac14a9e`; gates re-verified on a clean checkout via CI before archive claims are relied upon.
- A staged external-contract proposal may satisfy the typed `externalContractLock` reference only while the `external-distribution-schema-snapshots-and-verifiers` KU independently gates; OXY-D001 must adopt or replace the proposal before clearing that KU.

## Completion

Epic C completed its 21 story points without claiming that any staged input is complete or changing either readiness flag.

### Ticket completion

| Ticket | Commits | Verification result | Deviations and assumptions |
| :-- | :-- | :-- | :-- |
| OXY-C001 | `a731182446550355898c6824ae25ee923346fb6d`, `316f93254ee500a76e1794aafde57d888b5f67fc`, `25dc154`, `ff9d6a8` | `cargo +1.98.0 test -p xtask external_contracts` passed with exit 0. | Extended shared format validation, split focused command tests, preserved authoritative SPDX CRLF bytes, and bound the published SPDX schema to its digest-verified publication source; see [deviations](#oxy-c001-deviations--justifications). |
| OXY-C002 | `2fb134ed3be6e10a353ada4ab14557a281d01d87`, `81ede6ff048b1e9b3c699545704d45bf6c9c4333`, `923ce71` | `cargo +1.98.0 run -p xtask -- baseline validate --input qualification/fixtures/baselines/complete.synthetic.json && cargo +1.98.0 test -p xtask baseline` passed with exit 0. | Reused the exact traceability authority and evidence writer to publish baseline drafts; see [deviations](#oxy-c002-deviations--justifications). |
| OXY-C003 | `383021aae1fd1a9842a4fe7fcda3e642b4162455`, `3831555703ab6c09ef2f355dedd6ca7fdf20ae04`, `ca0a25f`, `30ca7cf` | `cargo +1.98.0 run -p xtask -- measurement validate --input qualification/fixtures/measurements/complete.synthetic.json && cargo +1.98.0 run -p xtask -- measurement validate --input qualification/fixtures/sample-validity/complete.synthetic.json && cargo +1.98.0 test -p xtask measurement` passed with exit 0. | Added a nonauthoritative staged schema and shared registry entry; the missing raw-measurement declaration and sample-validity contract are routed below; see [deviations](#oxy-c003-deviations--justifications). |
| OXY-C004 | `fd32fb9565438aae4eeb47254c7dcece296acd23`, `3b0fc3ad18eaff0117ce67878315964e657c359b`, `b983d0d`, `ff9d6a8` | `cargo +1.98.0 test --workspace --all-features` passed with exit 0. | Added fixture-backed collector deserialization and a bounded projection-plus-inventory artifact pair, then bound same-card Linux GPU and driver identities plus explicit macOS and Windows package sets; the companion artifact needs a Stage 3 schema; see [deviations](#oxy-c004-deviations--justifications). |
| OXY-C005 | `05ee6f110e83c008c2252929f562c1a33e6fbed6`, `c0015f5bcf0c5e645275a08534ff71315a5ba484`, `f0d9b6b`, `2422a57`, `ff9d6a8` | `cargo +1.98.0 run -p xtask -- lock status --gate candidate-implementation` returned the required exit 2 for the valid, open committed lock. | Reused readiness and staged-toolchain validation; path-less measurement-policy inputs and staged tool paths are routed below; see [deviations](#oxy-c005-deviations--justifications). |
| Epic C reconciliation | `cfb3fb7` | Archived Epic C without changing either readiness flag. | Preserved the active-plan handoff and routed Stage 3 gaps to OXY-D001. |
| PR review round 1 | `25dc154`, `ca0a25f`, `b983d0d`, `923ce71`, `f0d9b6b`, `91e7b60`, `2422a57` | Targeted validation and the final review quality gate passed. | Applied review corrections without changing qualification readiness. |
| PR review round 2 | `30ca7cf`, `ff9d6a8`, `380d8b0` | Targeted validation and the final review quality gate passed. | Hardened schema serialization, external snapshot provenance, and environment lock collection, then reconciled review follow-ups without changing qualification readiness. |
| PR review round 3 | `ec115a2`, `1595b03`, `e1fc92d` | Targeted validation and the final review quality gate passed. | Corrected SLSA derivation, reference-environment validation, immutable artifacts, readiness reporting, and meter parsing without changing qualification readiness. |
| PR review round 4 | `1a5fbb2`, `119eacd`, `5ee7b8c` | Targeted validation and the final review quality gate passed. | Corrected Windows release normalization, bounded Linux protocol collection, immutable artifacts, readiness diagnostics, and meter parsing without changing qualification readiness. |
| PR review round 5 | `8d163ae`, `1234f60`, `7e79086` | Targeted validation and the final review quality gate passed. | Conservatively rejected truncated protocol responses, preserved pre-existing immutable artifacts after companion publication failures, and clarified immutable-publication results without changing qualification readiness. |
| PR review round 6 | `e5036d8`, `d024f02`, `00f4908`, `63eafb2` | Targeted validation and the final review quality gate passed. | Rejected unobservable Linux sessions and relative resolved tools, verified committed complete and readiness fixtures, scoped raw clocks per launch, narrowed the evidence writer surface, and aligned manifest-bound readiness fixtures without changing qualification readiness. |
| PR review round 6 (CI fix) | `ac14a9e`, `57bdc88` | The full suite passed with the default and alternate Rustup roots. | Replaced developer-home Rustup paths with manifest-relative fixture paths, resolved them through `pathRoot` in test loaders, refreshed immutable fixture bindings, and recorded the host-independent correction. |
| PR review round 7 | `df7a288` | Full final quality gate passed. | Preserved external snapshot bytes, rejected whitespace-only baseline fields, and reported oversized macOS and Windows observations as bounded missing values without changing qualification readiness. |
| PR review round 8 | `e9adbbe` | Full final quality gate passed. | Accepted Debian package-version tildes, typed bounded capture failures, enforced staged-proposal KU and resolved-tool invariants, and retained the fail-closed protocol capture rule without changing qualification readiness. |
| PR review round 9 | `28d8c5e` | Full final quality gate passed. | Classified unverifiable staged hosts as valid-but-open, skipped unreachable readiness toolchains, retained per-receipt macOS failures, documented all-schema format assertions, and added Windows and X11 collector polish without changing qualification readiness. |
| PR review round 10 | `a6179d0` | Full final quality gate passed. | Restored fail-closed contract validation for unverifiable hosts, retained typed open-gate reporting, evaluated promotion before host verification, narrowed readiness skips, and allowlisted Mesa-backed Linux drivers. |

### Stage 3 revisions required — routed to OXY-D001

The [OXY-D001 inputs from Epic C](../active/EPIC-D-readiness-reconciliation.md#oxy-d001-inputs-from-epics-a-and-c) must name these unresolved Stage 3 revisions:

- `.constitution/tech-spec/data-models/raw-measurement.schema.json` omits the `$schema` property, so raw-measurement instances cannot self-declare their schema.
- `.constitution/tech-spec/data-models/raw-measurement.schema.json` doesn't state that `samples[].monotonicNs` is non-decreasing per `(constraintId, launch)`.
- No Stage 3 schema defines `qualification-lock.schema.json#measurementPolicy.sampleValidityRules`; `qualification/schemas/sample-validity.schema.json` is the proposed staged schema and its digest is the proposed binding value.
- The proposed external-contract lock values in `qualification/schemas/external/proposed-external-contract-lock.json` await Stage 3 adoption.
- `xtask environment inspect` writes the `PATH.inventory.json` companion artifact, but no Stage 3 schema defines it and `qualification-lock.schema.json#referenceEnvironments` has no typed reference to it.
- Wayland interface-set completeness has no Stage 3 rule. The companion inventory retains a partial observed `protocolVersion`, which lock v5 cannot represent.
- `qualification-lock.schema.json#measurementPolicy.{scoringAnchors,assessors,fuzzCorpora,securityPatchRehearsal}` binds path-less digests; the repository convention `qualification/staged/<field>.json` is the proposed referent and needs Stage 3 typing.
- `qualification-lock.schema.json#resolvedTools` lacks the `pathRoot` field used by `qualification/tools/native-contract-toolchain.json` for rustup-home-relative tools.
