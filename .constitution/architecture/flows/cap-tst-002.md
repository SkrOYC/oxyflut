# Structural assertion flow

## Mapping

`CAP-TST-002`: The test harness must assert layout and semantics deterministically.

## Behavior

```mermaid
flowchart LR
    Scenario[Frozen component scenario] -->|controlled call| Runtime[Component runtime]
    Runtime -->|layout request| Layout[Layout and viewport]
    Runtime -->|semantic diffs| Semantics[Semantics]
    Layout -->|deterministic snapshot| Assert[Harness assertions]
    Semantics -->|deterministic snapshot| Assert
    Expected[Expected structure] -->|file handoff| Assert
```

## Failure path

If either snapshot is missing, nondeterministic, or differs from its expected structure, the harness records the exact structural difference and fails the case.
