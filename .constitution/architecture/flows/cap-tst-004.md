# Rendering comparison flow

## Mapping

`CAP-TST-004`: The test harness must compare rendered output under pinned environments and declared cross-environment metrics.

## Behavior

```mermaid
flowchart LR
    Scene[Frozen scene] -->|twenty controlled renders| Pinned[Pinned reference environment]
    Pinned -->|encoded raster outputs| Exact{Byte exact or approved invariant}
    Cross[Other platform or rendering family] -->|platform baseline output| Metric[Frozen threshold or perceptual metric]
    Exact -->|comparison result| Evidence[Rendering evidence]
    Metric -->|comparison result| Evidence
```

## Failure path

Any unapproved byte difference, invariant failure, threshold excess, unfrozen environment, or missing raw output fails the comparison.
