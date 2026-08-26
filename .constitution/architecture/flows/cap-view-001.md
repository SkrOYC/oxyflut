# Independent view lifecycle flow

## Mapping

`CAP-VIEW-001`: The system must operate multiple views with independent metrics, focus, input, semantics, invalidation, lifecycle, and teardown.

## Behavior

```mermaid
flowchart LR
    Create[Window or headless-view creation] -->|normalized event| Views[View coordinator]
    Views -->|new scoped identity| A[View A state]
    Views -->|new scoped identity| B[View B state]
    EventA[Input, focus, semantics, or lifecycle for A] -->|identity route| A
    EventB[Input, focus, semantics, or lifecycle for B] -->|identity route| B
    CloseA[Close A] -->|teardown event| A
    A -.->|no mutation| B
```

## Failure path

If an event or completion has a missing, stale, or cross-runtime view identity, View coordinator rejects it and doesn't infer a default view.
