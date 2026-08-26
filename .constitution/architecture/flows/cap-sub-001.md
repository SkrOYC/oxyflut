# Symmetric candidate qualification flow

## Mapping

`CAP-SUB-001`: Before selection, every substrate candidate must pass the same complete P0 capability set and every applicable safety, security, privacy, performance, recovery, diagnostics, distribution, licensing, provenance, and upgrade constraint under one frozen evidence suite.

## Behavior

```mermaid
flowchart LR
    Suite[Frozen evidence suite] -->|same controlled requests| A[First substrate candidate]
    Suite -->|same controlled requests| B[Second substrate candidate]
    A -->|complete evidence file handoff| Gates[Common P0 and constraint gates]
    B -->|complete evidence file handoff| Gates
    Gates -->|separate eligibility records| Qualification[Release qualification]
```

## Failure path

Missing, candidate-specific, incomparable, failed, or unresolved gating evidence makes that candidate ineligible and cannot weaken the common suite.
