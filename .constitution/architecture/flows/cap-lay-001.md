# Constraint propagation flow

## Mapping

`CAP-LAY-001`: The system must lay out components through bounded constraint propagation.

## Behavior

```mermaid
flowchart LR
    Constraints[Parent constraints] -->|in-process call| Policy[Ordinary layout policy]
    Policy -->|bounded child visits| Children[Participating components]
    Children -->|size response| Policy
    Policy -->|placed geometry| Result[Layout result and visit count]
    Result -->|measurement event| Harness[Verification harness]
```

## Failure path

If constraints are invalid or a policy exceeds its declared visit cap, Layout and viewport rejects the result and records the gating layout failure.
