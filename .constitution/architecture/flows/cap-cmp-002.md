# Derived reactive value flow

## Mapping

`CAP-CMP-002`: The system must derive cached values from reactive dependencies.

## Behavior

```mermaid
flowchart LR
    Read[Derived-value read] -->|in-process call| Cache{Cached and valid}
    Cache -->|yes| Return[Return cached value]
    Cache -->|no| Compute[Compute with dependency capture]
    Compute -->|dependency registration| Index[Dependency index]
    Compute -->|owned result| Return
    Index -->|source changed event| Invalidate[Invalidate cached value]
```

## Failure path

If computation fails or forms a dependency cycle, Component runtime preserves the previous valid cache state and returns a structured derived-value error.
