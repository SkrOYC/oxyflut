# Asynchronous asset loading flow

## Mapping

`CAP-AST-001`: When an application requests an asset, the system must load it asynchronously without blocking interactive processing.

## Behavior

```mermaid
flowchart LR
    Request[Owned asset request] -->|asynchronous call| Queue[Bounded worker queue]
    Queue -->|worker handoff| Source[Asset source]
    Source -->|byte result| Owner{Owner still live}
    Owner -->|yes| Complete[Publish owned asset]
    Owner -->|no| Discard[Release result]
```

## Failure path

If the queue is full, the source fails, a cap is exceeded, or the owner closes, Asset and resource manager returns a structured error or cancellation without blocking frame processing.
