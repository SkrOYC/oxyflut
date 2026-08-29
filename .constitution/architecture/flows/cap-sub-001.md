# Symmetric candidate qualification flow

## Mapping

`CAP-SUB-001`: Before selection, every substrate candidate must pass the same complete P0 capability set and every applicable safety, security, privacy, performance, recovery, diagnostics, distribution, licensing, provenance, and upgrade constraint under one frozen evidence suite.

## Behavior

```mermaid
flowchart TD
    Suite[Frozen evidence suite] -->|same controlled requests| Integrated[Integrated substrate candidate]
    Integrated -->|first-environment evidence handoff| IntegratedGates{Integrated candidate common P0 and constraint hard gates}
    IntegratedGates -->|pass| Provisional[Provisional selection per environment under CAP-SUB-003]
    Provisional -->|eligibility record| Records[Eligibility records]
    IntegratedGates -->|fail on first Tier 1 environment| Focused[Focused substrate candidate]
    Focused -->|same frozen-suite controlled requests| CommonGates[Common P0 and constraint gates]
    CommonGates -->|eligibility record| Records
```

## Sequencing

Substrate candidates enter the frozen suite in the declared order: the integrated candidate first, then the focused candidate only if the integrated candidate fails hard-gate eligibility on the first qualification environment.

## Failure path

Missing, candidate-specific, incomparable, failed, or unresolved gating evidence makes that candidate ineligible and cannot weaken the common suite. A hard-gate failure of the integrated candidate on the first Tier 1 environment starts focused-candidate qualification under the same frozen suite. Evidence from a later Tier 1 environment that makes a provisional selection ineligible reverses the provisional selection and prevents final selection.
