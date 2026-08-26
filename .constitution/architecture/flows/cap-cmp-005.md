# Dependency-targeted update flow

## Mapping

`CAP-CMP-005`: When application state changes, the system must update only dependent parts of the component tree.

## Behavior

```mermaid
flowchart LR
    Change[Committed state change] -->|dependency event| Index[Dependency index]
    Index -->|affected identities| Dirty[Dirty component set]
    Dirty -->|in-process work| Reconcile[Targeted reconciliation]
    Reconcile -->|bounded invalidation| Layout[Layout, paint, or semantics owners]
    Unrelated[Unrelated components] -.->|no event| Layout
```

## Failure path

If dependency ownership is ambiguous, Component runtime fails the update instead of scanning or invalidating unrelated component subtrees.
