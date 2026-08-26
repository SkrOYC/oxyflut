# Deterministic input harness flow

## Mapping

`CAP-TST-001`: The test harness must pump frames and simulate pointer, touch, keyboard, and gesture input deterministically.

## Behavior

```mermaid
sequenceDiagram
    actor Author as Test author
    participant Harness as Test and verification harness
    participant Surface as Application surface
    participant Platform as Platform integration
    Author->>Harness: frozen clock and input script (file handoff)
    Harness->>Surface: controlled frame instants (in-process call)
    Harness->>Platform: pointer, touch, keyboard, and gesture events (event injection)
    Surface-->>Harness: application observations (response)
    Platform-->>Harness: routed-event evidence (response)
```

## Failure path

If virtual time, input order, owner identity, or expected event count diverges, the harness preserves the trace and fails the deterministic scenario.
