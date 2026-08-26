# Cancelable image decoding flow

## Mapping

`CAP-AST-002`: When an application requests image decoding, the system must perform the work asynchronously and permit cancellation.

## Behavior

```mermaid
stateDiagram-v2
    [*] --> Queued: decode request
    Queued --> Decoding: worker accepts
    Queued --> Cancelled: cancellation wins
    Decoding --> Cancelled: cancellation observed
    Decoding --> Completed: validated decode
    Completed --> Published: owner generation valid
    Completed --> Discarded: owner closed
    Cancelled --> [*]
    Published --> [*]
    Discarded --> [*]
```

## Failure path

Malformed input, resource-cap failure, and cancellation are distinct outcomes. None publishes partial decoded data.
