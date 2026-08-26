# Bounds-pruned hit-test flow

## Mapping

`CAP-INP-001`: The system must route pointer and touch input through bounds-pruned hit testing.

## Behavior

```mermaid
flowchart LR
    Event[Pointer or touch event] -->|normalized coordinates| Index[Layout spatial index]
    Index -->|bounds-pruned candidates| Hit[Hit-test traversal]
    Hit -->|frontmost eligible path| Route[Interaction and focus]
    Unrelated[Out-of-bounds subtrees] -.->|not visited| Hit
```

## Failure path

If coordinates, transforms, view identity, or tree generation are stale, Interaction and focus rejects the event rather than searching unrelated subtrees.
