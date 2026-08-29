# Tier-one qualification flow

## Mapping

`CAP-PLT-001`: The system must satisfy every P0 capability on macOS, Windows, Wayland, and X11 before the first production release.

## Behavior

```mermaid
flowchart LR
    Suite[Frozen complete P0 suite] -->|controlled runs| Mac[macOS row]
    Suite -->|controlled runs| Win[Windows row]
    Suite -->|controlled runs| Wayland[Wayland row]
    Suite -->|controlled runs| X11[X11 row]
    Mac -->|evidence file handoff| Gate{All rows pass}
    Win -->|evidence file handoff| Gate
    Wayland -->|evidence file handoff| Gate
    X11 -->|evidence file handoff| Gate
    Gate -->|yes| Eligible[Platform coverage eligible]
    Gate -->|no| Reject[Candidate ineligible]
```

Tier 1 environments are qualified in the declared order: Wayland, X11, macOS, then Windows; the first production release waits for all four to pass.

## Failure path

A failed, missing, incomparable, or gating-known-unknown row makes the candidate ineligible. A cited unsupported event can be not applicable only under the frozen baseline.
