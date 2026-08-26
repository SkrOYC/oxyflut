# Atomic state batch flow

## Mapping

`CAP-CMP-004`: When an application batches state changes, the system must publish their effects atomically and coalesce dependent work.

## Behavior

```mermaid
flowchart LR
    Changes[One or more state changes] -->|in-process calls| Batch[Owner-scoped batch buffer]
    Batch -->|nested batch merge| Batch
    Batch -->|outer commit| Publish[Atomic dependency publication]
    Publish -->|one event per owner| Reconcile[Reconciliation]
    Publish -->|one post-commit event| Effects[Side effects]
```

## Failure path

If a batched mutation fails, Component runtime cancels the uncommitted batch and publishes no intermediate state or dependent work.
