# Tier-one qualification flow

## Mapping

`CAP-PLT-001`: The system must satisfy every P0 capability on macOS, Windows, Wayland, and X11 before the first production release.

## Behavior

```mermaid
flowchart TD
    Suite[Frozen complete P0 suite] -->|controlled runs| Wayland[Wayland readiness]
    Wayland -->|evidence file handoff| WaylandPass{Wayland row passes?}
    WaylandPass -->|pass| X11[X11 readiness]
    WaylandPass -->|fail| Reject[Candidate ineligible]
    X11 -->|evidence file handoff| X11Pass{X11 row passes?}
    X11Pass -->|pass| Mac[macOS readiness]
    X11Pass -->|fail| Reject
    Mac -->|evidence file handoff| MacPass{macOS row passes?}
    MacPass -->|pass| Windows[Windows readiness]
    MacPass -->|fail| Reject
    Windows -->|evidence file handoff| WindowsPass{Windows row passes?}
    WindowsPass -->|pass| AllPass[All four Tier 1 environments pass]
    WindowsPass -->|fail| Reject
    AllPass -->|final transition| Final[Final selection and production release]
```

Tier 1 environments are qualified in the declared order: Wayland, X11, macOS, then Windows; the first production release waits for all four to pass.

## Failure path

A failed, missing, incomparable, or gating-known-unknown row makes the candidate ineligible. A cited unsupported event can be not applicable only under the frozen baseline.
