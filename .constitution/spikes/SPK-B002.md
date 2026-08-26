# Spike report: OXY-B002 Windows qualification baseline

## Time box

- **Budget:** 1 focused day
- **Clock start / stop:** Fill during execution.

## Question

- **Decision this spike must produce:** Which exact Windows build and Win32, TSF, UI Automation, DXGI timing, service-routing, and recovery interfaces form the complete candidate-neutral baseline for both allocations?

## Context and objective

- **Triggering upstream file or section:** `.constitution/tech-spec/contracts/platform-contracts.json` → `environments.windows`
- **Target:** Replace every Windows `ku-gating` item with cited KK evidence or a smaller named blocker.
- **Archetype / surface:** Library/SDK with System/Native Windows integration.

## Codebase baseline

- **State today:** Stage 3 pins Windows 11 25H2, Build Tools 17.14.39, SDK 10.0.26100.8876, TSF/UI Automation families, per-output DXGI observation, and presentation acknowledgement, but leaves minimum support and complete mappings open.
- **Discovered constraints:** `DwmGetCompositionTimingInfo` isn't a per-view observer; UTF-16 conversion, stale-target acknowledgements, per-HWND routing, and external timing must remain explicit.

## Options and trade-offs

- Option A: Use per-output `IDXGIOutput::WaitForVBlank` plus a separate presentation-acknowledgement path when controlled tests prove independence and migration behavior.
- Option B: Select another official per-output timing mechanism with equal external observability and tighter variable-refresh support.
- Option C: Retain timing or recovery rows as gating KUs when neither mechanism supplies externally attributable evidence.

## Recommendation

- **Chosen option:** Pending execution; select the smallest exact interface set that passes official availability review and noncandidate probes, otherwise retain C for the failed row.
- **Why it fits:** It preserves the external-meter and multi-view invariants without granting the integrated allocation a weaker mapping.
- **Rejected options:** Reject global DWM timing, undocumented COM behavior, candidate-only counters, implicit HWND selection, and property substitutions in UI Automation.

## Downstream impact

- **ADRs to write or update:** `ADR-0005-platform-hosts.md` and possibly `ADR-0006-execution-domains.md` through a Stage 3 pass.
- **Tickets unblocked in `tasks/active/`:** `OXY-D001`; later Windows baseline and environment-capture tickets require the Stage 3 reconciliation.
- **Tickets to add or split:** Add a timing or fault-injection follow-up spike only when the report leaves a bounded KU.
- **Spec edits required:** Stage 3 must update `stack.md`, `contracts/platform-contracts.json`, and the qualification-lock inputs after accepting the report.
