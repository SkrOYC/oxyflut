# Spike report: OXY-B001 macOS qualification baseline

## Time box

- **Budget:** 1 focused day
- **Clock start / stop:** Fill during execution.

## Question

- **Decision this spike must produce:** Which exact supported macOS versions and interfaces provide the candidate-neutral input method editor, accessibility, per-view timing, independent timing observation, service-routing, and recovery baseline for both allocations?

## Context and objective

- **Triggering upstream file or section:** `.constitution/tech-spec/contracts/platform-contracts.json` → `environments.macos`
- **Target:** Replace every macOS `ku-gating` item with cited KK evidence or a smaller named blocker.
- **Archetype / surface:** Library/SDK with System/Native macOS integration.

## Codebase baseline

- **State today:** Stage 3 pins Xcode 26.6, the macOS 26.5 SDK, AppKit text and accessibility families, view-associated display links, and Metal feedback, but leaves the deployment target, full mappings, observer independence, and recovery injection open.
- **Discovered constraints:** Both allocations must satisfy the same external baseline; deprecated CVDisplayLink APIs and candidate-internal clocks aren't admissible independent evidence.

## Options and trade-offs

- Option A: Freeze the pinned AppKit, VoiceOver, display-link, and Metal mechanisms at the earliest version where controlled probes pass; favors compatibility but increases fallback work.
- Option B: Freeze a higher minimum deployment version with fewer fallbacks; reduces implementation breadth but narrows product reach and requires explicit Stage 3 justification.
- Option C: Retain a gating KU for any behavior that official documentation and noncandidate probes cannot establish; delays candidate implementation but preserves evidence integrity.

## Recommendation

- **Chosen option:** Pending execution; choose A or B only from cited API availability and controlled probe evidence, otherwise choose C for the unresolved row.
- **Why it fits:** The PRD permits a not-applicable result only from cited KK evidence and forbids architectural plausibility from closing a gate.
- **Rejected options:** Reject deprecated timing APIs, candidate callbacks as independent meters, implicit default-window routing, and incomplete accessibility or text-input maps.

## Downstream impact

- **ADRs to write or update:** `ADR-0005-platform-hosts.md` and possibly `ADR-0006-execution-domains.md` through a Stage 3 pass.
- **Tickets unblocked in `tasks/active/`:** `OXY-D001`; later macOS baseline and environment-capture tickets require the Stage 3 reconciliation.
- **Tickets to add or split:** Add per-mechanism follow-up spikes only for rows retained as bounded KUs.
- **Spec edits required:** Stage 3 must update `stack.md`, `contracts/platform-contracts.json`, and the qualification-lock inputs after accepting the report.
