# Display-synchronized frame flow

## Mapping

`CAP-VIEW-002`: The system must schedule frames from display-synchronized presentation opportunities and expose the corresponding frame timestamps.

## Behavior

```mermaid
sequenceDiagram
    participant Environment as Operating environment
    participant Platform as Platform integration
    participant Views as View coordinator
    participant Runtime as Component runtime
    participant Boundary as Rendering-substrate boundary
    Environment->>Platform: presentation opportunity (event)
    Platform->>Views: normalized opportunity and timestamp (event)
    Views->>Runtime: frame instant (in-process call)
    Runtime->>Views: immutable scene (response)
    Views->>Boundary: scene and target timing (graphics command)
    Boundary-->>Views: presentation feedback (callback)
```

## Failure path

If timing identity, monotonic order, scene ownership, or feedback validation fails, View coordinator records a missed or failed frame for the affected view.
