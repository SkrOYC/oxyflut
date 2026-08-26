# Eligibility outcome flow

## Mapping

`CAP-SUB-003`: If zero candidates are eligible, the project must reopen substrate research; if one is eligible, it must select that candidate; if two are eligible, it must apply the frozen weighted comparison.

## Behavior

```mermaid
flowchart LR
    Eligible[Eligibility records] -->|count result| Count{Eligible candidates}
    Count -->|zero| Reopen[Reopen substrate research]
    Count -->|one| Select[Select eligible candidate]
    Count -->|two| Score[Apply frozen weighted comparison]
    Reopen -->|decision record| Register[Substrate decision record]
    Select -->|decision record| Register
    Score -->|scored evidence| Register
```

## Failure path

If candidate count or eligibility evidence is inconsistent, the selection remains open until Release qualification resolves the records.
