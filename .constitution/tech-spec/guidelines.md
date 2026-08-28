# Qualification implementation guidelines

## Status and reader

These guidelines govern Phase 3A qualification code. They do not define the production implementation. Phase 3B must replace qualification-only choices and explicitly authorize production planning.

## Target repository structure

The Stage 4 qualification plan must create the following target structure through the relevant package and build CLIs:

```text
.
├── .envrc
├── Cargo.toml
├── Cargo.lock
├── devenv.lock
├── devenv.nix
├── devenv.yaml
├── rust-toolchain.toml
├── crates
│   ├── oxyflut
│   ├── oxyflut-runtime
│   ├── oxyflut-layout
│   ├── oxyflut-scene
│   ├── oxyflut-assets
│   ├── oxyflut-view
│   ├── oxyflut-input
│   ├── oxyflut-text
│   ├── oxyflut-semantics
│   ├── oxyflut-platform
│   ├── oxyflut-diagnostics
│   ├── oxyflut-qualification
│   ├── oxyflut-substrate
│   ├── oxyflut-substrate-impeller
│   └── oxyflut-substrate-engine
├── native
│   └── engine-bridge
├── platform
│   ├── macos
│   ├── windows
│   └── linux
├── qualification
│   ├── fixtures
│   ├── golden
│   ├── probes
│   └── schemas
├── fuzz
├── xtask
└── .constitution
```

Every verification command must run inside `devenv shell`. You can use `direnv` to enter the same environment.

The `oxyflut` crate reexports only the reviewed public surface. Internal crates follow the logical boundaries in `architecture/containers.md`. The candidate adapter crates implement one internal substrate contract and cannot expose raw native handles to application crates.

## Rust standards

- Use Rust 1.98.0 and edition 2024 across the workspace.
- Deny warnings in continuous integration.
- Keep public items documented and follow the Rust API Guidelines.
- Keep helpers private by default. Use `pub(crate)` for intentional internal sharing.
- Model identifiers, lengths, indices, durations, and ownership generations with strong types and checked conversions.
- Use `Result` for expected failure and `Option` for absence. Don't use sentinel values or panic for boundary input.
- Don't use `unwrap`, `expect`, unchecked numeric casts, or wildcard enum matches in production library paths.
- Put every `unsafe` operation in the smallest possible function with a `SAFETY` comment that names its lifetime, aliasing, thread, nullability, and layout invariants.
- Keep host callback intake and mutable application or view state in separate logical executors, even when they share one operating-system thread. Drain callback intake only at nonreentrant application checkpoints. Use bounded message passing for worker and graphics-affine crossings.
- Don't hold a lock while invoking application code, candidate callbacks, platform callbacks, or diagnostic sinks.
- Keep the measured steady-state paint traversal free from global heap allocation.

## Native-boundary standards

- Treat `contracts/oxyflut-substrate.rs` as the common candidate-neutral Rust contract.
- Generate focused-candidate bindings from the pinned `impeller.h` file with `bindgen` 0.72.1, commit the generated hash, and run layout tests against the C compiler.
- Treat `contracts/oxyflut-substrate.h` as the authoritative integrated-candidate C ABI. Generate the Rust `sys` projection from that header with `bindgen` 0.72.1, lock its hash, and fail on layout, symbol, calling-convention, or nullability drift. `cbindgen` can generate test fixtures from Rust layout mirrors, but it isn't an independent ABI authority.
- Put `struct_size` and `abi_version` first in every extensible structure.
- Use fixed-width integers across the ABI. Don't expose C or C++ `bool`, `size_t`, exceptions, templates, standard-library types, or Rust layout.
- Treat every pointer as borrowed unless a named retain or release function transfers ownership.
- Copy callback-scoped data before callback return when the receiver needs to retain it.
- Catch Rust panics before returning to C and catch C++ exceptions before returning through the C ABI.
- Reject callbacks after the owner generation starts teardown.
- Acquire the integrated API only through `OxySubstrateGetApi`; verify the negotiated ABI prefix before calling a table entry.
- Treat mutable application owner objects as `!Send + !Sync`; copyable generation identifiers are `Send + Sync` but grant no access by themselves. Treat immutable scene or decoded-data values as `Send + Sync`, and native or graphics handles as owner-executor-affine unless the selected candidate proves a narrower transfer rule.

## Candidate rules

The focused candidate consumes only the pinned standalone SDK and implements platform integration above the substrate boundary. The integrated candidate can transport callbacks through the pinned engine fork but must pass them through the same normalization and ownership checks.

Both candidates must implement the same Rust `SubstrateAdapter` contract, use the same public Rust contract, emit the same evidence schemas, and run the same capability corpus. The focused adapter calls generated standalone SDK bindings. The integrated adapter calls the `OxySubstrateApi` table. Candidate-specific test exceptions are forbidden.

The integrated candidate must build a feasibility configuration that can link unused Dart components for diagnosis. Every scored, measured, packaged, or distributed configuration must set `flutter_enable_dart=false` and must pass binary inspection for forbidden runtime imports and strings.

## Contract and schema evolution

- Additive optional fields can advance a schema minor version after old readers pass fixtures for the new document.
- Removing a field, changing its meaning, narrowing a valid range, or changing an identifier requires a new major schema version and an explicit migration.
- Preserve raw evidence in its original version. Migrations create a new derived artifact and retain the source digest.
- Advance the C ABI version for any layout, calling-convention, ownership, lifetime, or semantic change.
- Advance the public crate major version for a breaking application-facing change after v1.0.0.

## Diagnostics and privacy

- Construct diagnostic records from `data-models/diagnostic-event.schema.json`.
- Keep record emission one-way, nonblocking, bounded, and content-free for private fields.
- Never record clipboard content, entered text, input method editor text, accessibility strings, file paths selected by users, or platform-message bodies.
- Compile no exporter into Phase 3A production-shaped measurement variants.

## Commits

Use Conventional Commits. A commit that changes a public contract, ABI, schema, qualification meter, or frozen dependency must explain the compatibility effect in its body.

## Verification commands

The following commands are the Stage 4 command contract. The documentation formatter and the qualification commands marked Available exist in the repository at v0.15.0; Stage 4 can schedule implementation of the remaining missing commands for qualification work only.

The staged native toolchain supports only `x86_64-unknown-linux-gnu`; other Tier 1 hosts are an OXY-D001 lock input.

Qualification planning has three states. While `contracts/qualification-lock.json` has `candidateImplementationReady: false`, Stage 4 can plan only repository scaffolding, contract validators, evidence writers, external-schema snapshotting, environment discovery, baseline authoring, and pre-implementation lock finalization. When `candidateImplementationReady` becomes true, Stage 4 can plan both candidate adapters, the integrated engine changes, and the shared capability implementation against that frozen suite, but it cannot collect comparable or scored evidence. Evidence collection begins only after completed candidate sources and adapters are pinned and the lock validates with `measurementReady: true`. Changing a pre-implementation input resets both readiness flags and invalidates affected work; changing a source pin after measurement begins creates a new lock and restarts affected evidence.

| Command | Purpose | Availability at v0.15.0 |
| :-- | :-- | :-- |
| `prettier --prose-wrap never --check '**/*.md' '!target/**' '!.devenv/**' '!qualification/fixtures/**'` | Check repository Markdown formatting without hard wrapping while preserving digest-pinned qualification fixtures. | Available. |
| `cargo +1.98.0 fmt --all --check` | Check Rust formatting. | Available. |
| `cargo +1.98.0 clippy --workspace --all-targets --all-features -- -D warnings` | Check Rust code and all feature combinations. | Available. |
| `cargo +1.98.0 test --workspace --all-features` | Run unit, integration, contract, and documentation tests. | Available. |
| `cargo +1.98.0 run -p xtask -- contracts validate` | Validate local schemas, instances, exact upstream sets, digests, Rust contracts, the authoritative C header and generated bindings, platform and accessibility baselines, score arithmetic, the selection decision, and Phase 3B promotion references. | Available. |
| `cargo +1.98.0 run -p xtask -- evidence verify PATH` | Verify one repository-relative evidence file, its schema or media type, canonical derived form when applicable, and every declared digest without rewriting preserved source bytes. Replace `PATH` with the evidence path. | Available. |
| `cargo +1.98.0 run -p xtask -- external-contracts verify` | Verify local SPDX, in-toto, SLSA, and DSSE snapshots and the staged external-contract-lock proposal without network resolution. | Available. |
| `cargo +1.98.0 run -p xtask -- baseline validate --input PATH` | Validate one candidate-neutral capability baseline and optionally publish its canonical draft. Replace `PATH` with the baseline path. | Available. |
| `cargo +1.98.0 run -p xtask -- measurement validate --input PATH` | Validate one raw-measurement or sample-validity record without executing a candidate measurement. Replace `PATH` with the record path. | Available. |
| `cargo +1.98.0 run -p xtask -- environment inspect --environment ENVIRONMENT --output PATH` | Capture one content-bounded reference-environment inventory for `macos`, `windows`, `wayland`, or `x11`. Replace `ENVIRONMENT` and `PATH` with locked values. | Available. |
| `cargo +1.98.0 run -p xtask -- lock status --gate candidate-implementation` | Validate all pre-implementation inputs and report remaining KUs without changing either readiness flag. | Available. |
| `cargo +1.98.0 run -p xtask -- candidate build --candidate focused --locked` | Build the focused candidate from the qualification lock. | Missing until the adapter exists. |
| `cargo +1.98.0 run -p xtask -- candidate build --candidate integrated --locked --dart-disabled` | Build the integrated candidate without the secondary runtime. | Missing until the fork and adapter exist. |
| `cargo +1.98.0 run -p xtask -- probe --candidate CANDIDATE --environment ENVIRONMENT` | Run one frozen Tier 1 capability matrix. Replace `CANDIDATE` and `ENVIRONMENT` with locked identifiers. | Missing until the harness exists. |
| `cargo +1.98.0 run -p xtask -- qualify --all-candidates --locked` | Run hard gates and produce schema-valid eligibility records without selecting from incomplete evidence. | Missing until both probes exist. |
| `cargo +1.98.0 fuzz run FUZZ_TARGET` | Run one frozen fuzz target. Replace `FUZZ_TARGET` with the ingress target identifier. | Missing until fuzz targets exist. |
| `cargo +1.98.0 deny check licenses bans sources` | Enforce dependency source, license, and duplicate policy. | Available for `licenses bans sources`; advisories deferred to OXY-D001. |
| `cargo +1.98.0 audit` | Check the lockfile against RustSec advisories. | Missing until the workspace exists. |

Qualification CLI commands write content-free diagnostics to standard error. Validation, evidence, external-contract, baseline, environment-inspection, build, probe, and qualification commands return exit code 0 on success and 1 on invalid input, failed validation, or execution failure. `lock status` returns 0 when the requested gate is ready, 2 when the lock is valid but the requested gate remains open, and 1 when the lock itself is invalid. No command converts an open gate into readiness.

## Production-planning prohibition

Stage 4 must read and validate `contracts/specification-phase.json` before planning work. If `productionReady` is `false`, Stage 4 can plan only the qualification commands in this file.

Phase 3B must replace this section with the selected production commands, remove losing-candidate commands, accept ADR-0010, and advance Stage 3 to v1.0.0 or later. No other document can waive this prohibition.
