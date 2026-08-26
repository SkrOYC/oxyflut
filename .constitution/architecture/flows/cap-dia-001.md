# Diagnostic record contract flow

## Mapping

`CAP-DIA-001`: The system must emit versioned local-diagnostic records with stable event names and field-level privacy classifications.

## Behavior

```mermaid
flowchart LR
    Event[Production event] -->|one-way emission| Validate[Schema version and stable-name validation]
    Validate -->|field classification| Privacy[Privacy classifier]
    Privacy -->|accepted record| Buffer[Bounded diagnostic buffer]
    Registry[Versioned record registry] -->|contract lookup| Validate
```

## Failure path

An unknown version, unstable name, or unclassified field is dropped before buffering and increments the contract-error counter.
