# Input method composition flow

## Mapping

`CAP-IME-001`: When the operating environment starts text composition, the system must preserve composition, candidate geometry, surrounding text, replacement, commit, cancellation, actions, metadata, index conversion, focus transfer, and sensitive-field behavior.

## Behavior

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Composing: focused composition starts
    Composing --> Composing: preedit metadata and surrounding-text exchange
    Composing --> Composing: candidate geometry and index conversion update
    Composing --> Committed: commit or replacement
    Composing --> Cancelled: cancel or lifecycle reset
    Composing --> Cancelled: focus transfers
    Committed --> Idle: editing acknowledgement
    Cancelled --> Idle: reset acknowledgement
    state Sensitive {
        [*] --> Redacted
        Redacted --> Redacted: metadata without raw-content diagnostics
    }
```

## Failure path

If transaction order, focus, index conversion, replacement range, or candidate geometry is invalid, Text and editing rejects the transaction and preserves the last acknowledged composition state.
