# Surfaceless rendering flow

## Mapping

`CAP-VIEW-005`: The system must render and return pixels without creating a visible or hidden top-level window or connecting to an interactive display service.

## Behavior

```mermaid
sequenceDiagram
    actor Harness as Test and verification harness
    participant Views as View coordinator
    participant Scene as Scene composition
    participant Boundary as Rendering-substrate boundary
    Harness->>Views: headless view and frame instant (in-process call)
    Views->>Scene: deterministic frame request (in-process call)
    Scene->>Boundary: surfaceless scene submission (graphics command)
    Boundary-->>Views: owned pixel result (response)
    Views-->>Harness: pixels and evidence (response)
```

## Failure path

If the candidate creates or contacts a window, compositor, interactive display service, drawable, swapchain, or presentation call, the harness fails the capability and releases the headless view.
