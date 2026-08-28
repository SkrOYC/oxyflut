# Qualification planning critical path

- **Version:** v0.3.0
- **Active story points:** 2
- **Active phase:** Readiness reconciliation

## Critical path

One path remains, and it is 2 story points:

1. `OXY-D001`

Every Epic B dependency of `OXY-D001` is satisfied for dependency sequencing. `OXY-B001` through `OXY-B006` delivered their spike reports, `OXY-B007` delivered the reference-hardware access register, and `OXY-B008` is satisfied as a blocked external input whose acceptance pass log remains unmet. Its assessor coordination record does not complete the two-assessor gate; the second-assessor confirmation remains a named external input for `OXY-D001`, and Stage 1 owns approval and application of the authorship-independence policy. Epics A and C were already archived, so `OXY-D001` has no unsatisfied prerequisite.

The critical path now runs through `OXY-D001` alone. The next critical epic is Epic D, Readiness reconciliation.

## Build order

An arrow points from a prerequisite to a ticket that depends on it. Dependencies on completed tickets are satisfied and don't appear in the active graph.

```mermaid
flowchart LR
    D001[OXY-D001]
```

## Phasing strategy

The pre-implementation research and coordination inputs now exist: Tier 1 platform-baseline recommendations for macOS, Windows, Wayland, and X11; the common-case layout visit-cap corpus, counting model, and derived probe threshold; the shared synthetic security-patch and frozen fuzz-corpus policy; the reference-hardware access register; and the assessor coordination record. Two inputs remain blocked on people outside this plan: macOS arm64 and Windows x86-64 reference-hardware access, which has no named owner or access procedure; and the second assessor plus the Stage 1 authorship-independence decision, without which the two-assessor and scoring-anchor gates cannot close.

`OXY-D001` consolidates those inputs with the completed Epic C tooling, names every missing approved or captured lock input, and routes required Stage 3 revisions without claiming readiness. The work runs inside the reproducible devenv shell.

Candidate adapters, the integrated engine fork, shared product capabilities, scored probes, and measurements remain deferred. After Stage 3 reconciles the research results and approved pre-implementation inputs into a lock with `candidateImplementationReady: true`, Stage 4 can release another minor version to plan both candidates symmetrically. Measurement and scoring remain deferred until the later `measurementReady: true` gate. Production planning remains prohibited until mandatory Phase 3B.
