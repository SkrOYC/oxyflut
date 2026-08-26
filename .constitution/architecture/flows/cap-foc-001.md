# Focus and keyboard flow

## Mapping

`CAP-FOC-001`: The system must provide focus scopes, keyboard routing and traversal, directional navigation, and visible focus indicators.

## Behavior

```mermaid
flowchart LR
    FocusRequest[Pointer, programmatic, or accessibility focus request] -->|scoped event| Focus[Focus hierarchy]
    Key[Keyboard event] -->|normalized event| Focus
    Focus -->|current target| Route[Component runtime]
    Traverse[Tab, reverse, or directional traversal] -->|graph operation| Focus
    Focus -->|focus-change event| Indicator[Visible focus indicator]
```

## Failure path

If a target is disabled, hidden, stale, or outside the active scope, Focus hierarchy rejects it or applies the frozen traversal fallback without routing the key elsewhere.
