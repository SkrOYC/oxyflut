# Cursor and lifecycle integration flow

## Mapping

`CAP-OS-001`: The system must integrate required operating-system cursors and application lifecycle behavior without exposing unsafe substrate handles.

## Behavior

```mermaid
flowchart LR
    Events[Cursor, window, display, and lifecycle events] -->|system callbacks| Platform[Platform integration]
    Platform -->|serialized canonical events| Views[View coordinator]
    Platform -->|cursor and focus events| Interaction[Interaction and focus]
    Views -->|lifecycle commands| Boundary[Rendering-substrate boundary]
    Boundary -->|completion callback| Platform
```

## Failure path

If callback order, identity, capability, or lifecycle state is invalid, Platform integration rejects the event and contains the failure to its view.
