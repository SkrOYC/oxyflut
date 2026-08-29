# Epic G: Shared application runtime

- **Status:** Active.
- **Total effort:** 32 points.
- **Plane:** Shared runtime.
- **Disjointness:** This epic owns only the listed shared application-runtime crates and `oxyflut` reexports; Epic E owns qualification specifications and readiness inputs, and Epic F owns the integrated substrate candidate files.
- **Boundary:** These substrate-neutral crates implement approved contracts against a null or test substrate. They do not depend on candidateImplementationReady.

## Tickets

#### OXY-G001 Implement reactive component state

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** None
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/oxyflut-runtime/**`
  - `crates/oxyflut/**` for reviewed public-surface reexports only
- **Scope (Out-of-Scope Files):**
  - Every other crate, native code, qualification files, and every file owned by Epic E or Epic F (don't touch).
- **Verification Command:** `cargo +1.98.0 test -p CRATE --all-features`; `cargo +1.98.0 clippy --workspace --all-targets --all-features -- -D warnings`
- **Expected Success Output:** `exit 0` after replacing `CRATE` with `oxyflut-runtime`.
- **STOP Conditions:**
  - "STOP if a public type or function would be needed that `contracts/oxyflut-public.rs` does not define; route the gap to Stage 3."
- **Description:** Implement CAP-CMP-001 through CAP-CMP-006: mutable reactive state, cached derived values, lifecycle-bound effects, nested batching, dependency-scoped updates, and idempotent teardown.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Contract tests exercise Signal, Memo, EffectHandle, ApplicationRuntime.batch, owner-scoped dependency updates, and teardown against contracts/oxyflut-public.rs.
Proptest invariants preserve prior committed state after a failed batch, publish no intermediate state, and reject stale owner work.
```

#### OXY-G002 Preserve keyed component state on reorder

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-G001
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/oxyflut-runtime/**`
  - `crates/oxyflut/**` for reviewed public-surface reexports only
- **Scope (Out-of-Scope Files):**
  - Every other crate, native code, qualification files, and every file owned by Epic E or Epic F (don't touch).
- **Verification Command:** `cargo +1.98.0 test -p CRATE --all-features`; `cargo +1.98.0 clippy --workspace --all-targets --all-features -- -D warnings`
- **Expected Success Output:** `exit 0` after replacing `CRATE` with `oxyflut-runtime`.
- **STOP Conditions:**
  - "STOP if keyed reconciliation requires a public type or function absent from `contracts/oxyflut-public.rs`; route the gap to Stage 3."
- **Description:** Implement CAP-CMP-007 keyed reconciliation. Preserve component state, focus, scroll position, and reusable render state during an owned reorder, and reject duplicate or unstable keys before state changes.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
A contract test reconciles a reordered key sequence and preserves state, focus, scroll position, and reusable render state for each retained key.
A duplicate or unstable key rejects reconciliation before any owner state changes.
A removed key follows the OXY-G001 teardown path exactly once.
```

#### OXY-G003 Implement bounded constraint layout

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-G001
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/oxyflut-layout/**`
  - `crates/oxyflut/**` for reviewed public-surface reexports only
- **Scope (Out-of-Scope Files):**
  - Every other crate, native code, qualification files, and every file owned by Epic E or Epic F (don't touch).
- **Verification Command:** `cargo +1.98.0 test -p CRATE --all-features`; `cargo +1.98.0 clippy --workspace --all-targets --all-features -- -D warnings`
- **Expected Success Output:** `exit 0` after replacing `CRATE` with `oxyflut-layout`.
- **STOP Conditions:**
  - "STOP if a layout counter, policy rule, public type, or function is absent from `contracts/oxyflut-public.rs`; route the gap to Stage 3."
- **Description:** Implement CAP-LAY-001 and CAP-LAY-002 with finite per-policy visit caps. Use the SPK-B005 counting rules as harness counters: record attempts before the cap check, record completed ordinary visits only after invocation, and keep intrinsic and text operations separate.
- **Acceptance:**
  - **Mode:** invariant
  - **Evidence:**

```text
Every ordinary request targets a realized direct child and increments attempted visits before its applicable cap check.
Every cap rejection preserves the attempted count, rejects before invocation, and leaves completed ordinary visits unchanged.
Intrinsic queries and text operations use separate counters, and custom policies cannot expose rendering-substrate state.
Property tests generate valid and invalid finite policy sequences and assert the stated counter invariants.
```

#### OXY-G004 Record retained scenes against a null substrate

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-G003
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/oxyflut-scene/**`
  - `crates/oxyflut/**` for reviewed public-surface reexports only
- **Scope (Out-of-Scope Files):**
  - Every other crate, native code, qualification files, and every file owned by Epic E or Epic F (don't touch).
- **Verification Command:** `cargo +1.98.0 test -p CRATE --all-features`; `cargo +1.98.0 clippy --workspace --all-targets --all-features -- -D warnings`
- **Expected Success Output:** `exit 0` after replacing `CRATE` with `oxyflut-scene`.
- **STOP Conditions:**
  - "STOP if scene recording needs a public type or function absent from `contracts/oxyflut-public.rs`; route the gap to Stage 3."
- **Description:** Implement CAP-REN-001 through CAP-REN-003 against a null substrate. Record paths, shapes, gradients, transforms, clips, filters, images, pictures, textures, and retained compositing state as immutable scene values without exposing substrate handles. Keep the measured steady-state paint traversal free from global heap allocation.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Canvas contract tests record each approved drawing operation and finish an immutable picture against a null substrate.
Invalid geometry, ownership, clip, filter, transform, or texture input returns an error without emitting a partial picture.
Retained-layer tests preserve opaque composition state and rebuild only an invalid or changed subtree.
A test allocator records zero global allocations during the measured steady-state paint traversal.
```

#### OXY-G005 Implement bounded local diagnostics

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** None
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/oxyflut-diagnostics/**`
  - `crates/oxyflut/**` for reviewed public-surface reexports only
- **Scope (Out-of-Scope Files):**
  - Every other crate, native code, qualification files, and every file owned by Epic E or Epic F (don't touch).
- **Verification Command:** `cargo +1.98.0 test -p CRATE --all-features`; `cargo +1.98.0 clippy --workspace --all-targets --all-features -- -D warnings`
- **Expected Success Output:** `exit 0` after replacing `CRATE` with `oxyflut-diagnostics`.
- **STOP Conditions:**
  - "STOP if a diagnostic record, privacy classification, or public function is absent from `contracts/oxyflut-public.rs`; route the gap to Stage 3."
- **Description:** Implement CAP-DIA-001 through CAP-DIA-004 with versioned local-diagnostic records, bounded buffers, sampling and drop counts, monotonic correlation, bounded-lifetime runtime, view, and frame identifiers, and machine-local user-controlled sinks.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
LocalDiagnosticSinkAdmission and LocalDiagnosticSink tests accept only declared machine-local destinations.
A full buffer or unavailable sink increments the relevant drop count and returns without blocking the producer.
Correlation tests reject stale, missing, or cross-runtime identifiers and never copy private text content into a record.
```

#### OXY-G006 Implement the rich-text editing model

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** None
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/oxyflut-text/**`
  - `crates/oxyflut/**` for reviewed public-surface reexports only
- **Scope (Out-of-Scope Files):**
  - Every other crate, native code, qualification files, and every file owned by Epic E or Epic F (don't touch).
- **Verification Command:** `cargo +1.98.0 test -p CRATE --all-features`; `cargo +1.98.0 clippy --workspace --all-targets --all-features -- -D warnings`
- **Expected Success Output:** `exit 0` after replacing `CRATE` with `oxyflut-text`.
- **STOP Conditions:**
  - "STOP if an editing operation needs a public type or function absent from `contracts/oxyflut-public.rs`; route the gap to Stage 3."
  - "STOP if geometry realization requires a rendering substrate candidate; retain it for substrate-qualified work."
- **Description:** Implement the CAP-TXT-002 and CAP-TXT-003 model layer: checked indices, grapheme and word boundaries, selection geometry contract ownership, insertion, deletion, undo, redo, and selection state. Keep geometry realization substrate-qualified.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
EditableText contract tests accept checked insertion, replacement, selection, grapheme deletion, word deletion, undo, and redo.
Boundary tests reject indices that split an invalid text unit and preserve the prior text and selection.
The model exposes the approved geometry contract without realizing geometry through a rendering substrate candidate.
```

#### OXY-G007 Implement Platform integration normalization and serialization skeleton

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** None
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/oxyflut-platform/`
  - `crates/oxyflut/` for reviewed public-surface reexports only
- **Scope (Out-of-Scope Files):**
  - Candidate adapter crates, `native/engine-bridge/`, qualification files, and every file owned by Epic E or Epic F (don't touch).
- **Verification Command:** `cargo +1.98.0 test -p CRATE --all-features`; `cargo +1.98.0 clippy --workspace --all-targets --all-features -- -D warnings`
- **Expected Success Output:** `exit 0` after replacing `CRATE` with `oxyflut-platform`.
- **STOP Conditions:**
  - "STOP if callback serialization, normalization, timestamp, or ownership behavior needs a public contract absent from `contracts/oxyflut-public.rs`; route the gap to Stage 3."
- **Description:** Implement the substrate-neutral Platform integration skeleton. Serialize callback intake and normalize window, display, input, lifecycle, and presentation feedback before application state changes. Expose a normalized test interface for candidate adapters without giving the adapter policy ownership.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Contract tests serialize reentrant callback intake, normalize each declared event to the approved interface, and reject callback data after its owner begins teardown. Candidate-facing tests consume the normalized interface without mutating application state through callback intake.
```

#### OXY-G008 Run the layout prequalification suite against the null substrate on the reference host

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-E010, OXY-E015, OXY-E022, OXY-G003
- **Category:** Perf
- **Scope (In-Scope Files):**
  - `crates/oxyflut-layout/`
  - `qualification/evidence/layout-prequalification/`
- **Scope (Out-of-Scope Files):**
  - Candidate adapter crates, `native/engine-bridge/`, `qualification/probes/`, and every file owned by Epic F (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- layout-prequalification validate --lock LOCK_PATH --corpus CORPUS_PATH --suite-schema SUITE_SCHEMA_PATH --suite SUITE_PATH --output RESULT_PATH`
- **Expected Success Output:** `exit 0`, one schema-valid null-substrate layout-cap result at `RESULT_PATH`, and no lock mutation.
- **STOP Conditions:**
  - "STOP if `thinkpadp14s` cannot supply the required reference-host identity or if the null-substrate evidence fails schema or lock validation; retain layoutVisitCap as a KU."
  - "STOP if the suite needs a candidate adapter or a paint-submission value for the null substrate; route the gap to Stage 3."
- **Description:** The Stage 3 command contract is `cargo +1.98.0 run -p xtask -- layout-prequalification validate --lock LOCK_PATH --corpus CORPUS_PATH --suite-schema SUITE_SCHEMA_PATH --suite SUITE_PATH --output RESULT_PATH`; it is missing until OXY-E010 lands it. Run the layout prequalification suite from `oxyflut-layout` with a null substrate on the Linux reference host. Store the result in `qualification/evidence/layout-prequalification/`, a subdirectory of the `qualification/evidence/` target structure in Stage 3 guidelines. Record ordinary visits, attempted ordinary visits, intrinsic queries, and application-owned layout time as substrate-independent evidence. Record paint-submission time as not applicable for the null substrate. Produce the layoutVisitCap binding input without recording a CON-* meter value, benchmark row, comparative score, or selection result.
- **Acceptance:**
  - **Mode:** runbook_probe
  - **Evidence:**

```text
The layout-prequalification command accepts the null-substrate suite with the declared Linux reference-host identity. The emitted evidence binds the corpus, counting rules, ordinary visits, attempted ordinary visits, intrinsic queries, application-owned layout time, and not-applicable paint-submission field. Lock validation accepts the layoutVisitCap binding input and rejects an absent, mismatched, or candidate-adapter-derived substitute.
```
