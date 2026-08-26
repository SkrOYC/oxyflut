# Candidate rejection flow

## Mapping

`CAP-SUB-002`: The project must reject a substrate candidate that fails or retains an unresolved gating P0 item.

## Behavior

```mermaid
flowchart LR
    Evidence[Candidate evidence] -->|file handoff| Gates[Hard-gate evaluation]
    Gates -->|failed or unresolved P0| Reject[Reject candidate]
    Gates -->|every hard gate passes| Eligible[Mark candidate eligible]
    Reject -->|decision record| Register[Substrate decision record]
    Eligible -->|decision record| Register
```

## Failure path

Release qualification records each failed or unresolved gate. It doesn't convert a plan, score, or inferred capability into eligibility.
