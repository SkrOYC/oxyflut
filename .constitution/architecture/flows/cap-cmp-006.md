# Component teardown flow

## Mapping

`CAP-CMP-006`: When a component leaves the tree, the system must release its subscriptions and owned lifecycle resources.

## Behavior

```mermaid
flowchart LR
    Unmount[Component leaves tree] -->|lifecycle event| Closing[Mark owner closing]
    Closing -->|cancellation| Effects[Cancel effects and worker jobs]
    Closing -->|unsubscribe| Dependencies[Remove subscriptions]
    Closing -->|release request| Resources[Release owned resources]
    Resources -->|acknowledgement| Removed[Remove component identity]
```

## Failure path

If a completion arrives after closing begins, its generation check fails and teardown discards it. Teardown remains idempotent.
