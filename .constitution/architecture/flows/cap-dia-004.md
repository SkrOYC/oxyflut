# Machine-local diagnostic sink flow

## Mapping

`CAP-DIA-004`: The system must expose diagnostic records only to machine-local, user-controlled sinks inside the application trust boundary.

## Behavior

```mermaid
flowchart LR
    Buffer[Bounded local buffer] -->|record delivery| Trust{Machine-local user-controlled sink}
    Trust -->|yes| Sink[Local sink]
    Trust -->|no| Reject[Reject destination]
    Sink -->|bounded acknowledgement| Buffer
    Failure[Sink failure] -->|loss event| Dropped[Dropped-record counter]
```

## Failure path

A remote, undeclared, unavailable, or slow sink receives no records. Local diagnostics drops bounded work without returning failure to the production source.
