# Accessibility action flow

## Mapping

`CAP-SEM-002`: When an accessibility service invokes an action, the system must route its payload to the correct live view and semantics node and return a defined acknowledgement or stale-target error.

## Behavior

```mermaid
sequenceDiagram
    actor Service as Accessibility service
    participant Platform as Platform integration
    participant Semantics as Semantics
    participant Runtime as Component runtime
    Service->>Platform: native action and payload (event)
    Platform->>Semantics: normalized view, node, action, and indices (call)
    Semantics->>Runtime: live component action (in-process call)
    Runtime-->>Semantics: acknowledgement or error (response)
    Semantics-->>Platform: mapped result (response)
    Platform-->>Service: native acknowledgement (response)
```

## Failure path

If the runtime, view, or node is stale or the payload is invalid, Semantics returns the defined stale-target or validation error and never retargets another node.
