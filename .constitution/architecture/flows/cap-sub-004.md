# Maintenance tie-break flow

## Mapping

`CAP-SUB-004`: If two eligible candidates differ by fewer than 5 weighted points, the project must select the candidate with lower measured upgrade-maintenance cost; equal or inconclusive evidence keeps the decision open.

## Behavior

```mermaid
flowchart LR
    Scores[Consensus weighted scores] -->|difference calculation| Gap{Difference below five points}
    Gap -->|no| Higher[Select higher score]
    Gap -->|yes| Cost{Lower measured upgrade cost}
    Cost -->|one candidate| Lower[Select lower-cost candidate]
    Cost -->|equal or inconclusive| Open[Keep selection open]
    Higher -->|decision record| Register[Substrate decision record]
    Lower -->|decision record| Register
    Open -->|decision record| Register
```

## Failure path

Missing, equal, or inconclusive maintenance evidence cannot be guessed or resolved by another criterion; the substrate selection stays open.
