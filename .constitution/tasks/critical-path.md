# Qualification planning critical path

- **Version:** v0.5.0
- **Active story points:** 131
- **Active phase:** Linux candidate-adapter readiness in Wayland-then-X11 order, integrated-candidate qualification preparation, and shared application-runtime work.

## Active backlog summary

| Epic   | Points | Scope                                       |
| :----- | -----: | :------------------------------------------ |
| Epic E |     80 | Specification landings and Linux readiness. |
| Epic F |     19 | Integrated substrate candidate on Linux.    |
| Epic G |     32 | Shared application runtime.                 |
| Total  |    131 | Active work only.                           |

## Critical path

The 43-point critical path has four equal sequences:

1. `OXY-E002` -> `OXY-E010` -> `OXY-E011` -> `OXY-E006` -> `OXY-E015` -> `OXY-E017` -> `OXY-E020` -> `OXY-E008` -> `OXY-F003` -> `OXY-F004` -> `OXY-F005`
2. `OXY-E002` -> `OXY-E010` -> `OXY-E011` -> `OXY-E007` -> `OXY-E015` -> `OXY-E017` -> `OXY-E020` -> `OXY-E008` -> `OXY-F003` -> `OXY-F004` -> `OXY-F005`
3. `OXY-E002` -> `OXY-E010` -> `OXY-E011` -> `OXY-E006` -> `OXY-E015` -> `OXY-E023` -> `OXY-E020` -> `OXY-E008` -> `OXY-F003` -> `OXY-F004` -> `OXY-F005`
4. `OXY-E002` -> `OXY-E010` -> `OXY-E011` -> `OXY-E007` -> `OXY-E015` -> `OXY-E023` -> `OXY-E020` -> `OXY-E008` -> `OXY-F003` -> `OXY-F004` -> `OXY-F005`

`OXY-E015` waits for both equal-cost Linux input branches, and `OXY-E020` waits for both equal-cost reconciliation branches. `OXY-E025` also reaches 30 points; `OXY-G008` reaches 24 points. Neither extends `OXY-F005`.

## Build order

Each arrow means "depends on."

```mermaid
flowchart LR
    OXY-E001
    OXY-E002
    OXY-E003 -->|depends on| OXY-E002
    OXY-E005
    OXY-E006 -->|depends on| OXY-E011
    OXY-E007 -->|depends on| OXY-E002
    OXY-E007 -->|depends on| OXY-E011
    OXY-E008 -->|depends on| OXY-E005
    OXY-E008 -->|depends on| OXY-E006
    OXY-E008 -->|depends on| OXY-E007
    OXY-E008 -->|depends on| OXY-E015
    OXY-E008 -->|depends on| OXY-E020
    OXY-E008 -->|depends on| OXY-E021
    OXY-E008 -->|depends on| OXY-E024
    OXY-E008 -->|depends on| OXY-G008
    OXY-E009 -->|depends on| OXY-E003
    OXY-E010 -->|depends on| OXY-E002
    OXY-E011 -->|depends on| OXY-E010
    OXY-E012 -->|depends on| OXY-E003
    OXY-E012 -->|depends on| OXY-E009
    OXY-E013 -->|depends on| OXY-E005
    OXY-E014 -->|depends on| OXY-E010
    OXY-E015 -->|depends on| OXY-E002
    OXY-E015 -->|depends on| OXY-E006
    OXY-E015 -->|depends on| OXY-E007
    OXY-E015 -->|depends on| OXY-E011
    OXY-E015 -->|depends on| OXY-E013
    OXY-E015 -->|depends on| OXY-E021
    OXY-E015 -->|depends on| OXY-E022
    OXY-E016 -->|depends on| OXY-E001
    OXY-E017 -->|depends on| OXY-E015
    OXY-E018 -->|depends on| OXY-E009
    OXY-E018 -->|depends on| OXY-E011
    OXY-E018 -->|depends on| OXY-E012
    OXY-E019 -->|depends on| OXY-E012
    OXY-E020 -->|depends on| OXY-E001
    OXY-E020 -->|depends on| OXY-E002
    OXY-E020 -->|depends on| OXY-E003
    OXY-E020 -->|depends on| OXY-E005
    OXY-E020 -->|depends on| OXY-E006
    OXY-E020 -->|depends on| OXY-E007
    OXY-E020 -->|depends on| OXY-E009
    OXY-E020 -->|depends on| OXY-E010
    OXY-E020 -->|depends on| OXY-E011
    OXY-E020 -->|depends on| OXY-E012
    OXY-E020 -->|depends on| OXY-E013
    OXY-E020 -->|depends on| OXY-E014
    OXY-E020 -->|depends on| OXY-E015
    OXY-E020 -->|depends on| OXY-E016
    OXY-E020 -->|depends on| OXY-E017
    OXY-E020 -->|depends on| OXY-E018
    OXY-E020 -->|depends on| OXY-E019
    OXY-E020 -->|depends on| OXY-E023
    OXY-E020 -->|depends on| OXY-E026
    OXY-E021 -->|depends on| OXY-E011
    OXY-E022 -->|depends on| OXY-E011
    OXY-E023 -->|depends on| OXY-E015
    OXY-E024 -->|depends on| OXY-E011
    OXY-E024 -->|depends on| OXY-E026
    OXY-E025 -->|depends on| OXY-E020
    OXY-E025 -->|depends on| OXY-E023
    OXY-E026 -->|depends on| OXY-E003
    OXY-E026 -->|depends on| OXY-E010
    OXY-F001
    OXY-F002
    OXY-F003 -->|depends on| OXY-E008
    OXY-F003 -->|depends on| OXY-E019
    OXY-F003 -->|depends on| OXY-F001
    OXY-F003 -->|depends on| OXY-F002
    OXY-F004 -->|depends on| OXY-F003
    OXY-F004 -->|depends on| OXY-G007
    OXY-F005 -->|depends on| OXY-F004
    OXY-G001
    OXY-G002 -->|depends on| OXY-G001
    OXY-G003 -->|depends on| OXY-G001
    OXY-G004 -->|depends on| OXY-G003
    OXY-G005
    OXY-G006
    OXY-G007
    OXY-G008 -->|depends on| OXY-E010
    OXY-G008 -->|depends on| OXY-E015
    OXY-G008 -->|depends on| OXY-E022
    OXY-G008 -->|depends on| OXY-G003
```

## Phasing strategy

This phase lands the qualification contracts and lock inputs, enables candidate-adapter readiness for Wayland and then X11, and prepares the integrated candidate under the frozen suite. Shared crates start against a null or test substrate without a readiness flag.

macOS and Windows remain deferred because their reference hardware is blocked. The focused substrate candidate remains deferred unless the integrated candidate fails hard-gate eligibility in Wayland. Comparable measurement, provisional and final selection evidence, Phase 3B promotion, and production delivery remain deferred.
