# Register of open decisions

- **Original date:** 2026-08-09
- **Amended:** 2026-08-25
- **Target:** Product requirements document (PRD)
- **Status:** One architecture-enabling decision reopened

All product capability and priority decisions remain resolved. Research into the standalone Impeller SDK and a full Flutter Engine path invalidated the original assumption that stock Flutter Engine binaries expose the required framework-facing rendering interfaces. Starling is an example that demonstrates the full-engine path with Swift.

## Open decision

| ID | Topic | Status | Resolution condition |
| :-- | :-- | :-- | :-- |
| OD-01 | Rendering substrate | Open | Complete every hard gate, comparison, platform probe, and upgrade rehearsal in the interview report. Then apply its scored selection rule. |

### OD-01: Rendering substrate

Evaluate the following candidate paths:

- **Path A: standalone Impeller.** Use the published C SDK for rendering and text. Own windowing, frame scheduling, input, IME, clipboard, editable selection, accessibility, image decoding, and surface recovery in `oxyflut`.
- **Path B: full Flutter Engine.** Add a language-neutral runtime controller and C ABI to a pinned Flutter Engine fork. Retain the shell, rasterizer, compositor, frame pipeline, and platform embedders. Starling is an example that demonstrates this architecture but doesn't define the `oxyflut` design.

The decision must not change the P0 capability list. It selects where those capabilities are implemented and which maintenance costs the project accepts.

Record the final decision with the following evidence:

- A controlled same-platform comparison of both paths.
- Working feasibility probes for macOS, Windows, Wayland, and X11.
- Shared two-window and different-refresh-display results for both candidates, including per-view focus, device-pixel ratio, IME, semantics, service routing, invalidation, and teardown.
- The same runtime-font, text-editing geometry, asynchronous-image, canvas and layer, native IME, clipboard, bidirectional selection, accessibility, surfaceless headless, and recovery suite for both candidates on every Tier 1 platform.
- Normalized startup, one-window and two-window memory, frame-cost, allocation, privacy-safe local-diagnostics overhead with exporters absent, and artifact-size measurements.
- Completed ABI safety, security, privacy, distribution, licensing, provenance, and observability gates.
- A list of changed Flutter Engine files for Path B.
- A list of platform components that `oxyflut` must own for Path A.
- A three-consecutive-release rehearsal covering two upgrades for both paths. It must measure migration work, fork-rebase work, regression results, and urgent security-patch handling.
- After hard-gate eligibility, two frozen assessors independently assign an integer score of 3, 4, or 5 to each criterion from cited KK evidence. They then record one written consensus score for every disagreement.

Both candidates can fail. A candidate with a failed or unresolved gating P0 item is ineligible for scoring; an implementation plan doesn't close the gate. The selection rule defines outcomes for zero, one, and two eligible candidates. A cross-family per-platform hybrid requires a separate architecture decision.

## Epistemic status

The decision record uses KK for verified facts, KU for named unanswered questions, UK for knowledge that might exist outside these artifacts, and UU for risks that can't be enumerated in advance.

- **KK:** The standalone Impeller C API exists. The stock Embedder API alone doesn't provide the required framework-submission boundary. Starling is an example that demonstrates runtime substitution with a Swift-specific callback contract. The exact Rust-compatible Path B interface doesn't exist in `oxyflut`. Upstream Flutter currently schedules all views together rather than providing separate per-display vsync. Neither candidate has demonstrated the complete P0 or nonfunctional scope in these artifacts.
- **KU:** Can the Path B interface meet the safety contract? Can Path B replace upstream Flutter's currently shared all-view frame scheduling with compliant per-display pacing? Can each candidate pass complete Tier 1 behavior, editing geometry, recovery, nonfunctional targets, and the frozen upgrade rehearsal?
- **UK:** Upstream maintainers, platform specialists, downstream forks, or unindexed tests might hold relevant constraints and failure cases. Each platform probe requires specialist review and a recorded search.
- **UU:** Hardware, driver, locale, assistive-technology, and lifecycle combinations can expose failures that this register can't predict. Hardware diversity, fuzzing, fault injection, prerelease deployment, and telemetry reduce this exposure but don't eliminate it.

## Closed decisions

The following clarifications don't require separate product decisions:

- P0 continues to include full IME, clipboard, selection, accessibility, multi-window behavior, surface recovery, rich text, surfaceless headless rendering, and all Tier 1 desktop platforms.
- Exact PNG comparison applies to a pinned rendering environment. Different platforms or graphics backends can use separate baselines.
- Impeller is the production iOS renderer inside Flutter. The missing item is an officially published standalone iOS SDK artifact, not iOS renderer support.
- Production `oxyflut` binaries don't start a Dart VM or execute Dart code under either path.
