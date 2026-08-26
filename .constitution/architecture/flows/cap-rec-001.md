# Surface recovery flow

## Mapping

`CAP-REC-001`: When a recoverable presentation or graphics fault occurs, the system must restore valid output within the applicable recovery deadline and preserve framework state.

## Behavior

```mermaid
stateDiagram-v2
    [*] --> Presenting
    Presenting --> Resize: final resize and resources available
    Presenting --> SurfaceLoss: surface-loss event
    Presenting --> ResumeTopology: resume or topology event
    Presenting --> DeviceLoss: recoverable device-loss event
    state "Resize: two refresh intervals" as Resize
    state "Surface loss: 250 ms" as SurfaceLoss
    state "Resume or topology: 500 ms" as ResumeTopology
    state "Device loss: 2 seconds" as DeviceLoss
    Resize --> Recreate: invalidate affected presentation resources
    SurfaceLoss --> Recreate: invalidate surface resources
    ResumeTopology --> Recreate: rebind display resources
    DeviceLoss --> Recreate: invalidate device resources
    Recreate --> Presenting: valid output and release acknowledgement
    Recreate --> Recreate: retry below three attempts and 2x memory cap
    Recreate --> Terminal: deadline, attempt, or memory cap exceeded
    Terminal --> [*]: structured error and release within 500 ms
    note right of Recreate: Application-runtime and component state remain live
```

## Failure path

If any deadline, three-attempt cap, 2x transient-memory cap, or 500 ms superseded-resource release bound fails, View coordinator returns a structured terminal error and preserves the evidence.
