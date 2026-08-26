# Invalidation coalescing flow

## Mapping

`CAP-VIEW-003`: When a view receives several invalidations before its next eligible frame, the system must coalesce them into one scheduled update.

## Behavior

```mermaid
flowchart LR
    A[First invalidation] -->|event| Pending[Per-view pending frame]
    B[Second invalidation] -->|merge event| Pending
    C[Third invalidation] -->|merge event| Pending
    Opportunity[Next eligible opportunity] -->|timing event| Pending
    Pending -->|one frame request| Runtime[Component runtime]
    Runtime -->|one scene| Submit[Submission]
    Pending -->|clear after acknowledgement| Idle[No pending frame]
```

## Failure path

If invalidations cross view identities or arrive during teardown, View coordinator rejects them. A failed submission retains one bounded retry state rather than duplicating requests.
