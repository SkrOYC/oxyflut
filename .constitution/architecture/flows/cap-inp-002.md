# Gesture disambiguation flow

## Mapping

`CAP-INP-002`: The system must resolve competing gestures through one deterministic disambiguation model.

## Behavior

```mermaid
flowchart LR
    Contacts[Normalized contact stream] -->|event| Recognizers[Eligible recognizers]
    Recognizers -->|claims and evidence| Arena[Gesture arbitration]
    Arena -->|one winner or compatible team| Winner[Owned gesture stream]
    Arena -->|cancellation events| Losers[Rejected recognizers]
    Winner -->|component event| Runtime[Component runtime]
```

## Failure path

If arbitration is ambiguous, an owner closes, or the stream violates ordering, the arena cancels all claims and emits no partial gesture.
