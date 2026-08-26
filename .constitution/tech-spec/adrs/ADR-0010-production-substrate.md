# Production rendering substrate

- **Status:** proposed
- **Date:** 2026-08-26

## Context

The focused standalone SDK candidate and integrated engine candidate are concrete enough to build, but neither has passed the complete P0 and nonfunctional gates. Selecting either candidate without evidence would contradict CAP-SUB-001 through CAP-SUB-004.

## Proposed decision

Keep both candidates at Trial during Phase 3A. Apply the shared contracts, qualification lock, Tier 1 matrix, security gates, upgrade rehearsal, and weighted selection policy without candidate-specific exceptions.

## Acceptance conditions

Phase 3B can accept this ADR only when:

- At least one candidate is eligible under every hard gate.
- The zero-candidate, one-candidate, or two-candidate rule produces a selection.
- The selected candidate passes the frozen common-case layout visit cap.
- The selection-decision record cites both immutable candidate records, recomputes the two-assessor consensus totals, and applies the score margin or maintenance tie-break exactly.
- Every promotion reference validates against the same ready qualification-lock digest and Stage 3 version.
- Complete platform, input method editor, accessibility, timing, recovery, distribution, and security baselines contain no gating P0 KU.
- The losing candidate is removed from production dependencies, commands, artifacts, and contract variants.

## Consequences while proposed

- Stage 3A isn't production-ready.
- Stage 4 can plan only qualification work.
- Neither candidate is described as the production architecture.
- The integrated approach replaces the application runtime while retaining selected engine subsystems. Starling is an example of this. It provides feasibility evidence for runtime substitution.
