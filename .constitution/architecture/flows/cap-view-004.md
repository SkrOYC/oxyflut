# Independent display pacing flow

## Mapping

`CAP-VIEW-004`: When views occupy displays with different timing, the system must pace each active view from its associated display without rendering an idle peer.

## Behavior

```mermaid
flowchart LR
    ClockA[Display A opportunities] -->|timing events| EpochA[View A display epoch]
    ClockB[Display B opportunities] -->|timing events| EpochB[View B display epoch]
    EpochA -->|active-view frame requests| SubmitA[View A submissions]
    EpochB -->|active-view frame requests| SubmitB[View B submissions]
    IdleB[Idle peer B] -.->|no invalidation, no frame| SubmitB
    Move[View migration or rate change] -->|new epoch event| EpochA
```

## Failure path

If a view uses another display's clock, misses the adoption bound, renders an idle peer, or lacks independent evidence, qualification fails the capability.
