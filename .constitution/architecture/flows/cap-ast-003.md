# Decoded resource caching flow

## Mapping

`CAP-AST-003`: The system must cache reusable decoded resources within declared memory limits.

## Behavior

```mermaid
flowchart LR
    Request[Decoded resource request] -->|cache lookup| Hit{Matching live entry}
    Hit -->|yes| Reuse[Retain cached resource]
    Hit -->|no| Decode[Request asynchronous decode]
    Decode -->|owned result| Insert[Insert within memory cap]
    Pressure[Memory pressure] -->|eviction event| Evict[Evict unretained entries]
    Insert -->|cap enforcement| Evict
```

## Failure path

If no eligible entry can be evicted within the memory cap, the cache rejects insertion and returns the owned decoded result or a structured allocation error.
