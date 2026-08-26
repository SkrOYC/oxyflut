# Ingress threat qualification flow

## Mapping

`CAP-SEC-001`: Before qualification, the project must inventory each implemented external ingress and trust boundary and must define threats, payload limits, and mitigations for each one.

## Behavior

```mermaid
flowchart LR
    Candidate[Substrate candidate] -->|declared surface handoff| Inventory[Implemented ingress and trust-boundary inventory]
    Baseline[Architecture ingress register] -->|required categories| Inventory
    Inventory -->|source, owner, cap, privacy, containment| Threats[Threat analysis]
    Threats -->|frozen test plan| Harness[Test and verification harness]
    Harness -->|results and cited absences| Gate[Release qualification]
```

## Failure path

Any undeclared ingress, unjustified absence, missing owner, missing payload cap, unclassified private content, or untested mitigation keeps the candidate ineligible.
