# Spike report: OXY-B003 Wayland qualification baseline

## Time box

- **Budget:** 1 focused day
- **Clock start / stop:** Fill during execution.

## Question

- **Decision this spike must produce:** Which minimum compositor, protocol, GTK, input method editor, assistive-technology, independent timing, service-routing, and recovery mechanisms define the complete Wayland baseline for both allocations?

## Context and objective

- **Triggering upstream file or section:** `.constitution/tech-spec/contracts/platform-contracts.json` → `environments.wayland`
- **Target:** Replace every Wayland `ku-gating` item with cited KK evidence or a smaller named blocker.
- **Archetype / surface:** Library/SDK with System/Native Wayland desktop integration.

## Codebase baseline

- **State today:** Stage 3 pins Ubuntu 26.04 LTS, GTK 4.20, GtkIMContext, AT-SPI families, GdkFrameClock, and `wp_presentation` feedback, while the exact compositor/protocol floor, Linux assistive technology, external opportunity observer, maps, and recovery injection remain open.
- **Discovered constraints:** Protocol availability doesn't prove compositor behavior; feedback isn't an opportunity source; both candidates need the same externally observed baseline.

## Options and trade-offs

- Option A: Freeze one Ubuntu reference compositor/session and exact protocol versions, then use a separate compositor or DRM observation path when permissions and attribution are reproducible.
- Option B: Use a harness-level timing source with demonstrated timestamp calibration and independence from both candidate callbacks.
- Option C: Retain the affected row as a gating KU when environment isolation or permissions prevent equivalent external evidence.

## Recommendation

- **Chosen option:** Pending execution; choose the narrowest reproducible A/B combination supported by official protocol and controlled probe evidence, otherwise retain C.
- **Why it fits:** It avoids turning a distribution label into an assumed platform contract and keeps candidate evidence symmetric.
- **Rejected options:** Reject nominal refresh-rate timers, `wp_presentation` feedback as an opportunity source, unspecified “selected assistive technology,” and assumed GtkIMContext index behavior.

## Downstream impact

- **ADRs to write or update:** `ADR-0005-platform-hosts.md` and possibly `ADR-0006-execution-domains.md` through a Stage 3 pass.
- **Tickets unblocked in `tasks/active/`:** `OXY-D001`; later Wayland baseline and environment-capture tickets require the Stage 3 reconciliation.
- **Tickets to add or split:** Add compositor-specific follow-up only for a bounded retained KU.
- **Spec edits required:** Stage 3 must update `stack.md`, `contracts/platform-contracts.json`, and the qualification-lock inputs after accepting the report.
