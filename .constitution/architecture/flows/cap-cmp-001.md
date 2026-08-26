# Mutable reactive state flow

## Mapping

`CAP-CMP-001`: The system must let application developers define mutable reactive state.

## Behavior

```mermaid
flowchart LR
    Declare[Developer declares state] -->|in-process call| Surface[Application surface]
    Surface -->|owned creation| Runtime[Component runtime]
    Runtime -->|owner-scoped record| State[Mutable reactive state]
    Mutate[Validated mutation] -->|in-process call| State
    State -->|change event| Dependencies[Dependency index]
```

## Failure path

If creation or mutation lacks a live owner, Component runtime rejects it and does not publish a change.
