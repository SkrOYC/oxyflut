# Diagnostic correlation flow

## Mapping

`CAP-DIA-003`: The system must correlate diagnostic records on a monotonic clock with bounded-lifetime identifiers for each runtime, view, and frame.

## Behavior

```mermaid
flowchart LR
    Context[Application-runtime, view, and frame context] -->|scoped propagation| Correlate[Correlation builder]
    Clock[Monotonic clock] -->|timestamp input| Correlate
    Event[Privacy-classified event] -->|one-way emission| Correlate
    Correlate -->|bounded-lifetime identifiers| Record[Diagnostic record]
    Teardown[Owner teardown] -->|generation expiry| Context
```

## Failure path

If correlation context is missing, stale, or cross-runtime, Local diagnostics drops the record and increments a correlation-error counter.
