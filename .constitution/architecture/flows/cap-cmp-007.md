# Keyed component movement flow

## Mapping

`CAP-CMP-007`: When keyed components move within a dynamic collection, the system must preserve their state, focus, scroll position, and reusable render state.

## Behavior

```mermaid
flowchart LR
    Next[Next keyed collection] -->|reconcile call| Match[Match keys within owner]
    Match -->|existing key| Preserve[Move component with state]
    Match -->|new key| Create[Create component]
    Match -->|missing key| Remove[Run component teardown]
    Duplicate[Duplicate key] -->|validation failure| Reject[Reject reconciliation]
```

## Failure path

If keys are duplicate or unstable within one owner, Component runtime rejects the reconciliation before moving state, focus, scroll position, or render state.
