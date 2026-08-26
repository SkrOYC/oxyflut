# Custom layout policy flow

## Mapping

`CAP-LAY-002`: The system must let extension authors define safe custom layout policies.

## Behavior

```mermaid
flowchart LR
    Author[Extension author policy] -->|safe registration| Validate[Policy validation]
    Validate -->|accepted in-process call| Layout[Layout and viewport]
    Layout -->|constraints and safe child operations| Policy[Custom policy]
    Policy -->|placed geometry| Layout
    Policy -->|invalid operation| Error[Structured layout error]
```

## Failure path

If a custom policy violates constraints, ownership, reentrancy, or its visit cap, Layout and viewport stops that policy without exposing substrate state.
