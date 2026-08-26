# Operating-system service flow

## Mapping

`CAP-OS-002`: The system must let applications invoke required operating-system services, including dialogs and platform messages, without exposing unsafe substrate handles.

## Behavior

```mermaid
sequenceDiagram
    actor App as Application developer
    participant Surface as Application surface
    participant Platform as Platform integration
    participant Environment as Operating environment
    App->>Surface: dialog or platform-message request (call)
    Surface->>Platform: owned canonical request (in-process call)
    Platform->>Environment: system service request (request)
    Environment-->>Platform: service result (response)
    Platform-->>Surface: scoped result or structured error (response)
    Surface-->>App: result (response)
```

## Failure path

If the owner closes, the response is oversized, or service routing is ambiguous, Platform integration rejects the result and doesn't deliver it to another view.
