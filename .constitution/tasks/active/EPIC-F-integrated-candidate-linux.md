# Epic F: Integrated candidate on Linux

- **Status:** Active.
- **Total effort:** 19 points.
- **Plane:** Qualification.
- **Disjointness:** This epic owns only `oxyflut-substrate`, `oxyflut-substrate-engine`, `native/engine-bridge`, the candidate command, and qualification probes; Epic E owns specifications and readiness inputs, and Epic G owns shared application-runtime crates.
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
  - `crates/oxyflut-substrate/**`, `crates/oxyflut-substrate-engine/**`, `native/engine-bridge/**`, `xtask/src/commands/candidate.rs`, `qualification/probes/**`, and every file owned by Epic E or Epic G (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- candidate build --candidate integrated --locked --dart-disabled`
- **Expected Success Output:** Preserve the time-box report with a decision, the build result, subsystem inventory, binary size, build time, GN arguments, and runtime-controller replacement inventory.
- **STOP Conditions:**
  - "STOP after 2 focused days; record an unresolved decision instead of extending the spike."
  - "STOP if the pinned source revision needs an unapproved patch or a contract change; route the decision to Stage 3."
- **Description:** Execute the 2-day SPK-F001 feasibility spike on `thinkpadp14s`. Test Flutter framework commit `4cf24164269a5ebf0c16a028a00727d0e77bbb05` for `linux-x64` with `flutter_enable_dart=false`, inventory linked subsystems, record binary size, build time, and GN arguments, identify application-runtime responsibilities the runtime controller must replace, and decide which subsystems the bridge retains and whether Dart-free compilation needs patches.
- **Acceptance:**
  - **Mode:** runbook_probe
  - **Evidence:**

```text
SPK-F001 records the question, options, time box, decision criteria, recommendation, and unblocked tickets.
The execution findings section records the pinned revision, linux-x64 result, Dart-disabled result, subsystem inventory, binary size, build duration, GN arguments, and runtime-controller replacement inventory.
The report ends at the time-box boundary and names OXY-F003 and OXY-F004 as dependent work.
```

#### OXY-F002 Implement the candidate-neutral substrate contract crate

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-E004
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/oxyflut-substrate/**`
- **Scope (Out-of-Scope Files):**
  - `crates/oxyflut-substrate-engine/**`, `native/engine-bridge/**`, `xtask/src/commands/candidate.rs`, `qualification/probes/**`, every other crate, and every file owned by Epic E (don't touch).
- **Verification Command:** `cargo +1.98.0 test -p CRATE --all-features`
- **Expected Success Output:** `exit 0` after replacing `CRATE` with `oxyflut-substrate`.
- **STOP Conditions:**
  - "STOP if a public type or function would be needed that `contracts/oxyflut-public.rs` does not define; route the gap to Stage 3."
  - "STOP if the Rust projection cannot represent the authoritative C header without a layout, ownership, lifetime, or calling-convention change; route the gap to Stage 3."
- **Description:** Implement the safe Rust surface for `contracts/oxyflut-substrate.rs` and generate its `sys` projection from `contracts/oxyflut-substrate.h`. Check the generated projection against the committed bindings golden and provide a null or test substrate. This candidate-neutral crate is not gated by candidateImplementationReady.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Contract tests compile the safe surface against contracts/oxyflut-substrate.rs.
The generated sys projection matches the committed bindings golden and rejects ABI-layout drift.
A null or test substrate exercises ownership, callback intake, headless output, and teardown without a rendering substrate candidate.
```

#### OXY-F003 Build the integrated native bridge skeleton

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-E008, OXY-F001, OXY-F002
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `native/engine-bridge/**`
  - `xtask/src/commands/candidate.rs`
- **Scope (Out-of-Scope Files):**
  - `crates/oxyflut-substrate/**`, `crates/oxyflut-substrate-engine/**`, `qualification/probes/**`, every other crate, and every file owned by Epic E or Epic G (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- candidate build --candidate integrated --locked --dart-disabled`
- **Expected Success Output:** `exit 0` and a Dart-disabled integrated-candidate artifact.
- **STOP Conditions:**
  - "STOP if Wayland candidateImplementationReady is not true; do not build candidate-adapter work."
  - "STOP if OxySubstrateGetApi negotiation requires an ABI change or an API-table entry absent from `contracts/oxyflut-substrate.h`; route the gap to Stage 3."
- **Description:** Export `OxySubstrateGetApi` from a runtime-controller skeleton that validates the negotiated ABI prefix and returns a stub API table. Implement the locked, Dart-disabled integrated candidate-build subcommand that produces the bridge input identified by SPK-F001.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
A native contract test accepts the supported ABI prefix and rejects incompatible or undersized prefixes before state exists.
The bridge exports OxySubstrateGetApi and returns a complete stub table with declared status values.
candidate build creates the Dart-disabled integrated artifact only after the Wayland candidate-implementation gate passes.
```

#### OXY-F004 Present one Wayland frame through the integrated adapter

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-F003
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/oxyflut-substrate-engine/**`
  - `qualification/probes/**`
- **Scope (Out-of-Scope Files):**
  - `crates/oxyflut-substrate/**`, `native/engine-bridge/**`, `xtask/src/commands/candidate.rs`, every other crate, and every file owned by Epic E or Epic G (don't touch).
- **Verification Command:** `cargo +1.98.0 run -p xtask -- probe --candidate CANDIDATE --environment ENVIRONMENT`
- **Expected Success Output:** `exit 0` after replacing `CANDIDATE` with `integrated` and `ENVIRONMENT` with `wayland`.
- **STOP Conditions:**
  - "STOP if a public type or function would be needed that `contracts/oxyflut-public.rs` does not define; route the gap to Stage 3."
  - "STOP if Platform integration normalization needs an undeclared callback, timestamp, or ownership rule; route the gap to Stage 3."
- **Description:** Use the bridge to create a runtime, view, and presentation mechanism, submit one scene, and receive presentation feedback through Platform integration normalization. Record cold launch to first complete acknowledged frame as the CON-PERF-003 meter value only; this ticket does not use it as a pass criterion.
- **Acceptance:**
  - **Mode:** gherkin
  - **Evidence:**

```text
Given a ready Wayland integrated artifact and a normalized callback receiver
When the probe creates a runtime and view and submits one scene
Then the adapter receives one normalized presentation acknowledgement for that frame
And the probe records cold launch to first complete acknowledged frame for CON-PERF-003 without evaluating the 50 ms goal
```

- **Supplemental Mode:** benchmark
- **Supplemental Evidence:** Record the CON-PERF-003 cold-launch meter and the exact probe command without assigning eligibility or a pass threshold.

#### OXY-F005 Render surfaceless pixels through the integrated adapter

- **Type:** Feature
- **Effort:** 3
- **Dependencies:** OXY-F004
- **Category:** Feature-Evolution
- **Scope (In-Scope Files):**
  - `crates/oxyflut-substrate-engine/**`
  - `qualification/probes/**`
- **Scope (Out-of-Scope Files):**
  - `crates/oxyflut-substrate/**`, `native/engine-bridge/**`, `xtask/src/commands/candidate.rs`, every other crate, and every file owned by Epic E or Epic G (don't touch).
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
