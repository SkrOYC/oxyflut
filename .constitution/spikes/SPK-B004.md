# Spike report: OXY-B004 X11 qualification baseline

## Time box

- **Budget:** 1 focused day
- **Clock start / stop:** Fill during execution.

## Question

- **Decision this spike must produce:** Which minimum X server, extension, GTK, input method editor, assistive-technology, independent timing, service-routing, and recovery mechanisms define the complete X11 baseline for both allocations?

## Context and objective

- **Triggering upstream file or section:** `.constitution/tech-spec/contracts/platform-contracts.json` → `environments.x11`
- **Target:** Replace every X11 `ku-gating` item with cited KK evidence or a smaller named blocker.
- **Archetype / surface:** Library/SDK with System/Native X11 desktop integration.

## Codebase baseline

- **State today:** Stage 3 pins Ubuntu 26.04 LTS, GTK 4.20, GtkIMContext, AT-SPI families, x11rb, and X Present completion, while the exact server/extension floor, Linux assistive technology, external opportunity observer, maps, and recovery injection remain open.
- **Discovered constraints:** Extension presence doesn't establish server semantics; completion isn't an opportunity source; output association and X11 window ownership must remain per view.

## Options and trade-offs

- Option A: Freeze the Ubuntu reference X server and exact extensions, with a separate X Sync or DRM observation path proven independent from both candidates.
- Option B: Use another externally attributable observer with calibrated monotonic timestamps and explicit output association.
- Option C: Retain the affected row as a gating KU when no observer or injection mechanism is reproducible.

## Recommendation

- **Chosen option:** Pending execution; choose A or B only from official extension evidence and controlled probes, otherwise retain C.
- **Why it fits:** It keeps timing, input, accessibility, and recovery claims tied to observed X11 behavior rather than shared GTK assumptions.
- **Rejected options:** Reject nominal timers, X Present completion as an opportunity source, implicit display routing, and unversioned AT-SPI assumptions.

## Downstream impact

- **ADRs to write or update:** `ADR-0005-platform-hosts.md` and possibly `ADR-0006-execution-domains.md` through a Stage 3 pass.
- **Tickets unblocked in `tasks/active/`:** `OXY-D001`; later X11 baseline and environment-capture tickets require the Stage 3 reconciliation.
- **Tickets to add or split:** Add X server or observer follow-up only for a bounded retained KU.
- **Spec edits required:** Stage 3 must update `stack.md`, `contracts/platform-contracts.json`, and the qualification-lock inputs after accepting the report.
