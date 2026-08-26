# Virtualized viewport flow

## Mapping

`CAP-SCR-001`: The system must create virtualized viewports whose work depends on visible content rather than total collection size.

## Behavior

```mermaid
flowchart LR
    Viewport[Viewport geometry and offset] -->|range query| Visible[Visible range calculation]
    Visible -->|materialization request| Runtime[Component runtime]
    Runtime -->|visible components| Layout[Layout and viewport]
    Layout -->|recycle event| Pool[Bounded reusable component pool]
    Offscreen[Unrelated offscreen items] -.->|no materialization| Runtime
```

## Failure path

If visibility can't be bounded or the reusable pool reaches its cap, Layout and viewport returns a structured resource error instead of materializing the full collection.
