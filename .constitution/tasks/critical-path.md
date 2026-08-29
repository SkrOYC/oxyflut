# Qualification planning critical path

- **Version:** v0.5.0
- **Active story points:** 130
- **Active phase:** Linux candidate-adapter readiness in Wayland-then-X11 order, integrated-candidate qualification preparation, and shared application-runtime work.

## Active backlog summary

| Epic   | Points | Scope                                       |
| :----- | -----: | :------------------------------------------ |
| Epic E |     79 | Specification landings and Linux readiness. |
| Epic F |     19 | Integrated substrate candidate on Linux.    |
| Epic G |     32 | Shared application runtime.                 |
| Total  |    130 | Active work only.                           |

## Critical path

The 45-point critical path is:

1. `OXY-E002`
2. `OXY-E010`
3. `OXY-E011`
4. `OXY-E006`
5. `OXY-E015`
6. `OXY-E023`
7. `OXY-E025`
8. `OXY-E004`
9. `OXY-E008`
10. `OXY-F003`
11. `OXY-F004`
12. `OXY-F005`

`OXY-G008` reaches 24 points before `OXY-E004`; `OXY-E023` and `OXY-E025` reach 26 points and determine the freeze date.

## Build order

Each arrow means "depends on."

```mermaid
flowchart LR
    OXY-E001
    OXY-E002
    OXY-E003 -->|depends on| OXY-E002
    OXY-E004 -->|depends on| OXY-E001
    OXY-E004 -->|depends on| OXY-E002
    OXY-E004 -->|depends on| OXY-E003
    OXY-E004 -->|depends on| OXY-E005
    OXY-E004 -->|depends on| OXY-E006
    OXY-E004 -->|depends on| OXY-E007
    OXY-E004 -->|depends on| OXY-E009
    OXY-E004 -->|depends on| OXY-E010
    OXY-E004 -->|depends on| OXY-E011
    OXY-E004 -->|depends on| OXY-E012
    OXY-E004 -->|depends on| OXY-E013
    OXY-E004 -->|depends on| OXY-E014
    OXY-E004 -->|depends on| OXY-E015
    OXY-E004 -->|depends on| OXY-E016
    OXY-E004 -->|depends on| OXY-E017
    OXY-E004 -->|depends on| OXY-E018
    OXY-E004 -->|depends on| OXY-E019
    OXY-E004 -->|depends on| OXY-E020
    OXY-E004 -->|depends on| OXY-E021
    OXY-E004 -->|depends on| OXY-E022
    OXY-E004 -->|depends on| OXY-E023
    OXY-E004 -->|depends on| OXY-E024
    OXY-E004 -->|depends on| OXY-E025
    OXY-E004 -->|depends on| OXY-G008
    OXY-E005
    OXY-E006 -->|depends on| OXY-E011
    OXY-E007 -->|depends on| OXY-E002
    OXY-E008 -->|depends on| OXY-E004
    OXY-E008 -->|depends on| OXY-E005
    OXY-E008 -->|depends on| OXY-E006
    OXY-E008 -->|depends on| OXY-E007
    OXY-E008 -->|depends on| OXY-E015
    OXY-E008 -->|depends on| OXY-E024
    OXY-E008 -->|depends on| OXY-G008
    OXY-E009 -->|depends on| OXY-E003
    OXY-E010 -->|depends on| OXY-E002
    OXY-E011 -->|depends on| OXY-E010
    OXY-E012 -->|depends on| OXY-E003
    OXY-E012 -->|depends on| OXY-E009
    OXY-E013 -->|depends on| OXY-E005
    OXY-E014 -->|depends on| OXY-E010
    OXY-E015 -->|depends on| OXY-E006
    OXY-E015 -->|depends on| OXY-E007
    OXY-E015 -->|depends on| OXY-E011
    OXY-E015 -->|depends on| OXY-E013
    OXY-E015 -->|depends on| OXY-E022
    OXY-E016 -->|depends on| OXY-E001
    OXY-E017 -->|depends on| OXY-E011
    OXY-E018 -->|depends on| OXY-E009
    OXY-E018 -->|depends on| OXY-E010
    OXY-E018 -->|depends on| OXY-E011
    OXY-E018 -->|depends on| OXY-E012
    OXY-E018 -->|depends on| OXY-E017
    OXY-E019 -->|depends on| OXY-E012
    OXY-E020 -->|depends on| OXY-E018
    OXY-E020 -->|depends on| OXY-E019
    OXY-E021 -->|depends on| OXY-E011
    OXY-E022 -->|depends on| OXY-E010
    OXY-E023 -->|depends on| OXY-E015
    OXY-E024 -->|depends on| OXY-E002
    OXY-E025 -->|depends on| OXY-E023
    OXY-F001
    OXY-F002
    OXY-F003 -->|depends on| OXY-E004
    OXY-F003 -->|depends on| OXY-E008
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
