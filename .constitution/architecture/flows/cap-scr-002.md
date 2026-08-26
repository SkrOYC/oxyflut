# Platform-appropriate scrolling flow

## Mapping

`CAP-SCR-002`: The system must provide platform-appropriate wheel, precision-pointer, touch, momentum, and boundary scrolling behavior.

## Behavior

```mermaid
flowchart LR
    Input[Wheel, precision-pointer, or touch input] -->|normalized event| Interaction[Interaction and focus]
    Interaction -->|scroll intent| Scroll[Scrolling policy]
    Scroll -->|momentum and boundary update| Offset[Viewport offset]
    Offset -->|one invalidation event| View[View coordinator]
    Environment[Environment behavior baseline] -->|policy input| Scroll
```

## Failure path

If an event is stale, targets another view, or yields an invalid offset, the scrolling policy preserves the last valid position and cancels its momentum sequence.
