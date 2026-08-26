# Safe texture drawing flow

## Mapping

`CAP-REN-002`: The system must let extension authors draw realized textures through the safe public surface.

## Behavior

```mermaid
flowchart LR
    Draw[Safe texture draw] -->|owned handle and geometry| Validate[Texture generation and owner validation]
    Validate -->|record call| Scene[Scene composition]
    Scene -->|immutable texture reference| Boundary[Rendering-substrate boundary]
    Boundary -->|use completion| Lifetime[Asset and resource manager]
```

## Failure path

If the texture is stale, released, cross-runtime, or invalid for the scene, Scene composition rejects the draw before submission.
