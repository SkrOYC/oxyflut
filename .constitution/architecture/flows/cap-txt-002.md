# Text geometry flow

## Mapping

`CAP-TXT-002`: The system must expose complete caret, boundary, range, affinity, and selection geometry for styled bidirectional text.

## Behavior

```mermaid
flowchart LR
    Styled[Styled bidirectional text and line layout] -->|geometry query| Text[Text and editing]
    Query[Caret, boundary, affinity, or range request] -->|index-aware call| Text
    Text -->|shared shaping result| Geometry[Carets and selection rectangles]
    Geometry -->|in-process response| Editor[Editing and semantics consumers]
```

## Failure path

If an index splits an invalid unit, affinity is ambiguous, or layout generation is stale, Text and editing rejects the query instead of approximating geometry.
