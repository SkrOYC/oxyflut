# Incremental semantics flow

## Mapping

`CAP-SEM-001`: The system must maintain an incremental semantics tree that preserves every applicable role-specific property, relation, state, value, geometry, text range, traversal rule, and view identity.

## Behavior

```mermaid
flowchart LR
    Changes[Component semantic changes] -->|in-process diffs| Tree[Per-view semantics tree]
    Geometry[Layout geometry and transforms] -->|scoped update| Tree
    Tree -->|insert, update, and delete set| Map[Role-specific platform mapping]
    Map -->|normalized property exchange| Platform[Platform integration]
    Platform -->|acknowledgement| Tree
```

## Failure path

If a diff has duplicate identities, invalid relationships, stale geometry, or an unmapped required property, Semantics rejects the affected update and preserves the last acknowledged tree.
