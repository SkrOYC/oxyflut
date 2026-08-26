# Retained composition flow

## Mapping

`CAP-REN-003`: The system must retain compositing state for opacity, clipping, transforms, reusable subtrees, and effects that read existing scene content.

## Behavior

```mermaid
flowchart LR
    Changes[Component paint changes] -->|in-process event| Layers[Retained composition tree]
    Layers -->|identity comparison| Reuse{Reusable subtree}
    Reuse -->|yes| Cached[Reuse retained layer]
    Reuse -->|no| Rebuild[Record changed layer]
    Cached -->|damage union| Scene[Scene submission]
    Rebuild -->|damage union| Scene
```

## Failure path

If layer identity, bounds, or effect ordering is invalid, Scene composition rebuilds the affected subtree or fails the scene without reusing ambiguous content.
