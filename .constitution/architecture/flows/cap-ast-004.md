# Graphics resource realization flow

## Mapping

`CAP-AST-004`: The system must realize decoded pixels as graphics resources and preserve ownership through upload, use, and teardown.

## Behavior

```mermaid
flowchart LR
    Pixels[Owned decoded pixels] -->|graphics-affine command| Boundary[Rendering-substrate boundary]
    Boundary -->|realization mechanism| Resource[Owned graphics resource]
    Resource -->|generation-scoped handle| Scene[Scene composition]
    Scene -->|use completion| Manager[Asset and resource manager]
    Manager -->|release command| Boundary
    Boundary -->|release acknowledgement| Manager
```

## Failure path

If realization, submission, or release fails, the manager preserves ownership, prevents stale publication, and reports the failed stage without leaking a graphics resource.
