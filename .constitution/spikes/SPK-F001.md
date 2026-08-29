# Spike report: OXY-F001 Dart-free integrated-candidate build

## Time box

- **Budget:** 2 focused days.
- **Clock start / stop:** Record during spike execution.

## Question

- **Decision this spike must produce:** Does Flutter framework commit `4cf24164269a5ebf0c16a028a00727d0e77bbb05`, with its pinned source revision `5f77625673248ee5846fbcaf5d3e1a3878386fd7`, build for `linux-x64` with `flutter_enable_dart=false` without patches, and which source subsystems must the bridge retain while the runtime controller replaces the application runtime?

## Context and objective

- **Triggering upstream file or section:** `.constitution/tech-spec/stack.md` defines the integrated candidate and the Dart-disabled production-configuration probe.
- **Target:** The Linux integrated-candidate build boundary, linked-subsystem inventory, GN arguments, binary size, build duration, and application-runtime replacement inventory.
- **Archetype / surface:** Library/SDK with a System/Native rendering-substrate boundary.

## Codebase baseline

- **State today:** Planning inspection found a 3-line `native/engine-bridge/README.md`, an 18-line `xtask/src/commands/candidate.rs`, and one-line `oxyflut-substrate` and `oxyflut-substrate-engine` library files.
- **Discovered constraints:** The technical stack requires `flutter_enable_dart=false` for every scored, measured, packaged, or distributed configuration. The time-box evidence determines the Linux build result, retained subsystems, binary size, build duration, and GN arguments.

## Options and trade-offs

- **Option A:** The pinned source builds Dart-free without patches. Retain only the bridge subsystems identified by the linked-subsystem inventory.
- **Option B:** The pinned source builds Dart-free only with a patch. Record the patch boundary, its compatibility effect, and the Stage 3 decision required before candidate-adapter work.
- **Option C:** The pinned source does not build Dart-free. Record the failing configuration and preserve the focused-candidate trigger without weakening the frozen suite.

## Recommendation

- **Chosen option:** Determined by the recorded 2-day execution result.
- **Why it fits:** The build result, binary inspection, GN arguments, subsystem inventory, and runtime-controller replacement inventory provide the evidence required to choose among the options.
- **Rejected options:** No option is rejected during planning.

## Downstream impact

- **ADRs to write or update:** None if the build uses the approved contract. Route an ABI, ownership, lifecycle, or callback-contract change to Stage 3.
- **Tickets unblocked in `tasks/active/`:** `OXY-F003`; `OXY-F004` follows `OXY-F003`.
- **Tickets to add or split:** Split only if the time-box result identifies an approved bridge boundary that exceeds the 8-point ticket limit.
- **Spec edits required:** Name Stage 3 if the pinned source requires a patch or a contract change. Do not edit active specifications during the spike.
