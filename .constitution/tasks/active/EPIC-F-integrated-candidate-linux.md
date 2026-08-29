# Epic F: Integrated candidate on Linux

- **Status:** Active.
- **Total effort:** 19 points.
- **Plane:** Qualification.
- **Disjointness:** This epic owns `oxyflut-substrate`, `oxyflut-substrate-engine`, `native/engine-bridge`, `xtask/src/commands/candidate.rs`, and `qualification/probes/`. Epic E excludes those paths, and Epic G owns the shared application-runtime crates.
- **Boundary:** The integrated substrate candidate enters the frozen suite first. This epic does not produce a final selection, comparable measurement, or a focused-candidate adapter.

## Tickets

#### OXY-F001 Test the Dart-free integrated-candidate build

- **Type:** Spike
- **Effort:** 3
- **Dependencies:** None
- **Category:** Dependency-Upgrade
- **Scope (In-Scope Files):**
  - `.constitution/spikes/SPK-F001.md`
- **Scope (Out-of-Scope Files):**
  - `crates/oxyflut-substrate/`, `crates/oxyflut-substrate-engine/`, `native/engine-bridge/`, `xtask/src/commands/candidate.rs`, `qualification/probes/`, and every file owned by Epic E or Epic G (don't touch).
- **Verification Command:** `prettier --prose-wrap never --check '**/*.md' '!target/**' '!.devenv/**' '!qualification/fixtures/**'`
- **Expected Success Output:** `exit 0` and the report exists at `.constitution/spikes/SPK-F001.md`.
- **STOP Conditions:**
  - "STOP after 2 focused days; record an unresolved decision instead of extending the spike."
  - "STOP if the pinned source revision needs an unapproved patch or a contract change; route the decision to Stage 3."
- **Description:** Execute the 2-day SPK-F001 feasibility spike on `thinkpadp14s`. Follow the feasibility configuration build procedure documented by the spike and technical stack as research output. Record the Dart-disabled result, linked-subsystem inventory, binary size, build duration, GN arguments, runtime-controller replacement inventory, and the retained bridge boundary. This ticket does not implement `candidate build`.
- **Acceptance:**
  - **Mode:** runbook_probe
  - **Evidence:**

```text
.constitution/spikes/SPK-F001.md exists and passes the stated Prettier command. The report records the question, options, time box, decision criteria, recommendation, pinned revision, build result, subsystem inventory, binary size, build duration, GN arguments, runtime-controller replacement inventory, and unblocked tickets.
```

#### OXY-F002 Implement the candidate-neutral substrate contract crate

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** None
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/oxyflut-substrate/`
- **Scope (Out-of-Scope Files):**
  - `.constitution/tech-spec/contracts/oxyflut-substrate.h`, `qualification/fixtures/generated-bindings/oxyflut-substrate.rs`, `crates/oxyflut-substrate-engine/`, `native/engine-bridge/`, `xtask/src/commands/candidate.rs`, `qualification/probes/`, every other crate, and every file owned by Epic E (don't touch).
- **Verification Command:** `cargo +1.98.0 test -p CRATE --all-features`
- **Expected Success Output:** `exit 0` after replacing `CRATE` with `oxyflut-substrate`.
- **STOP Conditions:**
  - "STOP if a public type or function would be needed that `contracts/oxyflut-public.rs` or `contracts/oxyflut-substrate.rs` does not define; route the gap to Stage 3."
- **Description:** Implement the common Rust contract from `contracts/oxyflut-public.rs` and `contracts/oxyflut-substrate.rs` with a null or test implementation. This candidate-neutral crate has no rendering-substrate dependency and does not depend on candidateImplementationReady.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Contract tests compile the safe surface against contracts/oxyflut-substrate.rs. A null or test substrate exercises ownership, callback intake, headless output, and teardown without a substrate candidate.
```

#### OXY-F003 Build the integrated native bridge skeleton

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-E008, OXY-E019, OXY-F001, OXY-F002
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `native/engine-bridge/`
  - `xtask/src/commands/candidate.rs`
- **Scope (Out-of-Scope Files):**
  - `crates/oxyflut-substrate/`, `crates/oxyflut-substrate-engine/`, `qualification/probes/`, every other crate, and every file owned by Epic E or Epic G (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- candidate build --candidate integrated --locked --dart-disabled`
- **Expected Success Output:** `exit 0` and a Dart-disabled integrated-candidate artifact.
- **STOP Conditions:**
  - "STOP if Wayland candidateImplementationReady is not true; do not build candidate-adapter work."
  - "STOP if OxySubstrateGetApi negotiation requires an ABI change or an API-table entry absent from `contracts/oxyflut-substrate.h`; route the gap to Stage 3."
- **Description:** Export `OxySubstrateGetApi` from a runtime-controller skeleton that validates the negotiated ABI prefix and returns a stub API table. Generate the C-header Rust `sys` bindings in the bridge, validate them against the committed golden, and implement the locked Dart-disabled `candidate build` command using the boundary identified by SPK-F001.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Native contract tests accept the supported ABI prefix and reject incompatible or undersized prefixes before state exists. The generated bridge sys bindings match the committed golden. The bridge exports OxySubstrateGetApi and returns a complete stub table with declared status values. candidate build creates the Dart-disabled integrated artifact only after the Wayland candidate-implementation gate passes.
```

#### OXY-F004 Present one Wayland frame through the integrated adapter

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-F003, OXY-G007
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/oxyflut-substrate-engine/`
  - `qualification/probes/`
- **Scope (Out-of-Scope Files):**
  - `crates/oxyflut-substrate/`, `native/engine-bridge/`, `crates/oxyflut-platform/`, `xtask/src/commands/candidate.rs`, every other crate, and every file owned by Epic E or Epic G (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- probe --candidate CANDIDATE --environment ENVIRONMENT`
- **Expected Success Output:** `exit 0` after replacing `CANDIDATE` with `integrated` and `ENVIRONMENT` with `wayland`.
- **STOP Conditions:**
  - "STOP if a public type or function would be needed that `contracts/oxyflut-public.rs` does not define; route the gap to Stage 3."
  - "STOP if the normalized Platform integration interface needs an undeclared callback, timestamp, or ownership rule; route the gap to Stage 3."
- **Description:** Use the bridge to create a runtime, view, and presentation mechanism, submit one scene, and consume the normalized Platform integration interface from OXY-G007. Timing observations from this probe are diagnostics only. Do not record a CON-* meter value, benchmark row, eligibility result, score, or lock binding.
- **Acceptance:**
  - **Mode:** gherkin
  - **Evidence:**

```text
Given a ready Wayland integrated artifact and a normalized callback receiver
When the probe creates a runtime and view and submits one scene
Then the adapter receives one normalized presentation acknowledgement for that frame
And the probe retains any timing observation as diagnostics without a meter, benchmark, eligibility, score, or lock binding
```

#### OXY-F005 Render surfaceless pixels through the integrated adapter

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-F004
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/oxyflut-substrate-engine/`
  - `qualification/probes/`
- **Scope (Out-of-Scope Files):**
  - `crates/oxyflut-substrate/`, `native/engine-bridge/`, `xtask/src/commands/candidate.rs`, every other crate, and every file owned by Epic E or Epic G (don't touch).
- **Verification Command:** `cargo +1.98.0 test -p CRATE --all-features`
- **Expected Success Output:** `exit 0` after replacing `CRATE` with `oxyflut-substrate-engine`.
- **STOP Conditions:**
  - "STOP if surfaceless output creates a window, contacts an interactive display service, or needs a public type absent from `contracts/oxyflut-public.rs`; route the gap to Stage 3."
- **Description:** Implement CAP-VIEW-005 through the integrated adapter by recording and rendering one scene to owned pixels without a top-level window or interactive-display connection.
- **Acceptance:**
  - **Mode:** gherkin
  - **Evidence:**

```text
Given a surfaceless scene and valid headless metrics
When the adapter renders the scene
Then the adapter returns tightly packed RGBA8888 premultiplied sRGB pixels
And the probe detects no window, compositor, interactive display service, drawable, swapchain, or presentation call
```
