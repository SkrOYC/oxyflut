# Safe vector drawing flow

## Mapping

`CAP-REN-001`: The system must let extension authors record vector paths, shapes, gradients, transforms, clips, filters, images, and reusable pictures through the safe public surface.

## Behavior

```mermaid
flowchart LR
    Commands[Safe vector drawing commands] -->|in-process calls| Validate[Geometry and resource validation]
    Validate -->|record calls| Recorder[Scene composition]
    Recorder -->|immutable picture| Scene[Retained scene]
    Scene -->|submission handoff| Boundary[Rendering-substrate boundary]
```

## Failure path

If geometry, transforms, clips, filters, images, or ownership are invalid, Scene composition rejects the command and never emits a partial picture.
