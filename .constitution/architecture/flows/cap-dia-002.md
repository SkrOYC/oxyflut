# Bounded diagnostic collection flow

## Mapping

`CAP-DIA-002`: The system must bound diagnostic buffers and sampling and must report dropped records.

## Behavior

```mermaid
flowchart LR
    Event[Privacy-classified event] -->|one-way emission| Sample{Sampling admits}
    Sample -->|no| Sampled[Increment sampled-out counter]
    Sample -->|yes| Capacity{Buffer has capacity}
    Capacity -->|yes| Queue[Enqueue bounded record]
    Capacity -->|no| Dropped[Increment dropped-record counter]
    Queue -->|nonblocking return| Producer[Producer continues]
```

## Failure path

Sampling or capacity rejection is an expected bounded outcome. It cannot block the producer or allocate beyond the frozen buffer cap.
