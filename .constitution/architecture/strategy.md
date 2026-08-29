# Architecture strategy

- **Version:** v1.1.0

## Architectural pattern

Oxyflut uses a layered in-process software development kit with explicit ownership boundaries around the application runtime, operating-environment integration, and rendering substrate. A separate qualification plane drives deterministic tests, measurements, and release evidence without participating in production frame processing.

The application process contains one application runtime and one or more views. Each view owns independent metrics, focus, semantics identity, invalidation, lifecycle, and presentation state. Shared services can cache immutable resources, but mutable view state cannot be global.

## Why this pattern fits

The Library/SDK archetype requires a small application-facing surface that hides lifecycle, rendering, text, and operating-environment complexity. The System/Native secondary archetype requires explicit event-loop, display, accessibility, and resource ownership. In-process calls protect the 2.0 ms application-owned frame budget, while bounded asynchronous work isolates asset decoding and operating-environment requests.

The rendering substrate remains replaceable while the declared qualification sequence runs. The integrated substrate candidate is qualified first and becomes the provisional selection if it clears hard-gate eligibility on the first Tier 1 environment. The focused substrate candidate remains specified but enters the same frozen suite only if the integrated candidate fails hard-gate eligibility on that environment. The provisional selection becomes final only after every Tier 1 environment passes. One substrate candidate exposes focused drawing and text services, which leaves view scheduling and operating-environment behavior above the substrate boundary. The other candidate retains more integrated scheduling, presentation, and operating-environment transport. In both cases, Oxyflut owns the application-facing semantics, canonical per-view policy and state, acceptance behavior, and evidence contract. Candidate-specific delegation can provide mechanisms but cannot bypass those logical responsibilities.

The qualification plane observes presentation opportunities independently from production callbacks. It also owns fault injection, deterministic input, artifact verification, and substrate scoring. This separation prevents a candidate from defining the evidence used to qualify itself.

## Architectural invariants

- Every mutable operation is scoped to an application runtime, view, component, or resource owner.
- Every operating-environment event enters through one normalization boundary before reaching product logic.
- Every interactively presented frame begins with one eligible presentation opportunity and ends with presentation feedback or a structured failure.
- Every surfaceless frame begins with a harness-controlled frame instant that isn't a presentation opportunity and cannot create an interactive-display connection.
- Every semantics action identifies a live application runtime, view, and semantics node.
- Every substrate candidate implements the same logical rendering, resource, timing, recovery, and evidence contracts.
- Shared boundaries above the rendering substrate boundary have no dependency on any substrate candidate and are implementable against a null substrate.
- Production diagnostics remain bounded, privacy-classified, and machine-local.
- Qualification evidence is immutable, attributable, and independent from candidate-internal counters where an external observation is possible.

## Trade-offs accepted

- Stable logical boundaries add translation work and can prevent direct use of candidate-specific shortcuts.
- Per-view isolation increases bookkeeping compared with global state.
- A separate qualification plane adds test infrastructure and artifact-management cost.
- Sequential qualification reduces duplicate candidate implementation, but it moves discovery and build cost later if the focused candidate is needed.
- Strict ownership and bounded queues can reject work instead of hiding overload through unbounded buffering.
