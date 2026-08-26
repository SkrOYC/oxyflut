# Actors

## Application developer

The application developer is the primary actor. They integrate Oxyflut into a desktop application and work through its safe public surface.

- **Goals:** Build responsive interfaces, create custom controls and layouts, manage application state, test behavior deterministically, and ship signed desktop artifacts.
- **Context:** The developer can target several desktop environments and expects one coherent application model across them.
- **Frictions:** Platform behavior differs, unsafe rendering boundaries are difficult to audit, and incomplete text or accessibility behavior appears late in delivery.

## Framework extension author

The framework extension author builds reusable components, layout policies, input recognizers, and drawing abstractions for application developers.

- **Goals:** Express advanced behavior without raw substrate handles, retain deterministic ownership, and preserve compatibility across substrate upgrades.
- **Context:** Extensions can participate in performance-sensitive layout, painting, semantics, and interaction paths.
- **Frictions:** Hidden allocations, ambiguous lifecycle ownership, and platform-specific behavior can make an extension unsafe or unpredictable.

## Application user

The application user interacts with an Oxyflut application through pointer, keyboard, touch, text input, and display output.

- **Goals:** Complete tasks with responsive input, correct text, predictable window behavior, and preserved state during display changes.
- **Context:** Users can move views between displays, change refresh rates, use several input devices, and work in several languages.
- **Frictions:** Missed frames, broken composition, lost selection, focus errors, and failed recovery interrupt their work.

## Assistive-technology user

The assistive-technology user operates the application through a screen reader, keyboard traversal, or another operating-system accessibility service.

- **Goals:** Discover content, understand names and states, navigate in a meaningful order, edit text, and invoke actions independently.
- **Context:** Accessibility focus and input focus can differ. Text indices, relationships, and actions cross system boundaries.
- **Frictions:** Missing properties, stale nodes, incorrect action routing, and inaccessible custom controls block task completion.

## Release maintainer

The release maintainer builds, verifies, signs, packages, and updates Oxyflut artifacts and their rendering substrate.

- **Goals:** Reproduce unsigned payloads, fulfill license obligations, apply security fixes promptly, and keep upgrades within the maintenance budget.
- **Context:** Every Tier 1 environment has different packaging and signing requirements.
- **Frictions:** Large dependency graphs, private boundaries, generated artifacts, and platform-specific failures increase upgrade risk.

## Operating environment

The operating environment is a non-human actor that supplies windows, displays, input, accessibility services, memory pressure, and lifecycle events.

- **Goals:** Enforce platform contracts and resource ownership while accepting valid application requests.
- **Context:** Events can be asynchronous, reordered across subsystems, unavailable on some environments, or accompanied by surface and device invalidation.
- **Frictions:** Candidate implementations can assume capabilities that the environment doesn't expose or can lose state during asynchronous transitions.

## Test and verification harness

The test and verification harness is a non-human actor that provides deterministic input, independent timing observations, fault injection, artifact verification, and preserved evidence.

- **Goals:** Distinguish product behavior from candidate callbacks, reproduce measurements, and reject incomplete evidence.
- **Context:** The harness runs interactive and surfaceless scenarios across every Tier 1 environment.
- **Frictions:** Unfrozen tools, candidate-dependent clocks, missing raw data, and incomparable platform metrics invalidate conclusions.
