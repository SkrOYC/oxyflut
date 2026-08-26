# Lifecycle-bound effect flow

## Mapping

`CAP-CMP-003`: The system must run lifecycle-bound side effects when their reactive dependencies change.

## Behavior

```mermaid
flowchart LR
    Register[Register side effect] -->|owned in-process call| Runtime[Component runtime]
    Runtime -->|dependency capture| Effect[Lifecycle-bound effect]
    Change[Committed dependency change] -->|event| Effect
    Effect -->|post-commit call| External[Declared external operation]
    Unmount[Owner teardown] -->|cancellation event| Effect
```

## Failure path

If an effect fails, reenters mutation, or runs after owner teardown, Component runtime contains the failure, rejects stale work, and preserves the committed state.
