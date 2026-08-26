# Qualification planning critical path

- **Version:** v0.1.0
- **Active story points:** 74
- **Active phase:** Pre-implementation qualification foundation and decision research

## Critical path

The primary critical path is 23 story points:

1. `OXY-A001`
2. `OXY-A002`
3. `OXY-A003`
4. `OXY-A004`
5. `OXY-A007`
6. `OXY-D001`

The co-critical baseline branch is `OXY-A001` → `OXY-A002` → `OXY-A003` → `OXY-C002` → `OXY-C005` → `OXY-D001`, also 23 story points. Parallel platform, layout, security, hardware-access, and assessor work must finish before `OXY-D001` even when it isn't on either longest dependency chain.

## Build order

An arrow points from a prerequisite to a ticket that depends on it.

```mermaid
flowchart LR
    A001[OXY-A001] --> A002[OXY-A002]
    A001 --> A005[OXY-A005]
    A002 --> A003[OXY-A003]
    A002 --> A006[OXY-A006]
    A002 --> C001[OXY-C001]
    A003 --> A004[OXY-A004]
    A003 --> A007[OXY-A007]
    A003 --> C002[OXY-C002]
    A004 --> A007
    A004 --> C005[OXY-C005]
    A005 --> A007
    A006 --> A007
    A006 --> C002
    A002 --> C003[OXY-C003]
    A006 --> C003
    A001 --> C004[OXY-C004]
    A006 --> C004
    C001 --> C005
    C002 --> C005
    C003 --> C005
    C004 --> C005
    A006 --> C005
    A007 --> D001[OXY-D001]
    C005 --> D001
    B001[OXY-B001] --> D001
    B002[OXY-B002] --> D001
    B003[OXY-B003] --> D001
    B004[OXY-B004] --> D001
    B005[OXY-B005] --> D001
    B006[OXY-B006] --> D001
    B007[OXY-B007] --> D001
    B008[OXY-B008] --> D001
```

## Phasing strategy

This active plan implements only work permitted while `contracts/qualification-lock.json` has `candidateImplementationReady: false`: repository scaffolding, offline contract validators, evidence writers, external-contract snapshots, environment-inventory tooling, baseline-authoring tooling, and time-boxed research into the remaining technical KUs. `OXY-D001` consolidates the evidence and identifies the exact Stage 3 revisions still required.

Candidate adapters, the integrated engine fork, shared product capabilities, scored probes, and measurements are deferred. After the spike recommendations receive their required Stage 3 revision and the pre-implementation lock validates with `candidateImplementationReady: true`, Stage 4 must release a minor version that plans both candidates symmetrically. Measurement and scoring remain deferred until the later `measurementReady: true` gate. Production planning remains prohibited until mandatory Phase 3B.
