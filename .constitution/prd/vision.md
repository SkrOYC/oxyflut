# Product vision

- **Version:** v1.0.0

## Executive summary

Oxyflut is a user-interface software development kit for developers who need responsive desktop applications with predictable resource ownership and no secondary application runtime. It combines reactive composition, rich text, input, accessibility, window management, rendering, and deterministic testing behind a safe application-facing surface. The first release targets production-quality desktop behavior rather than a narrow rendering demo.

## Archetype

- **Primary:** Library/SDK.
- **Secondary:** System/Native.
- **Confidence:** high.
- **Rationale:** Application developers integrate Oxyflut as a software development kit. The resulting applications participate directly in operating-system window, input, accessibility, display, and graphics lifecycles.

## Jobs to be done

- Build desktop user interfaces with one coherent application model instead of assembling unrelated rendering and operating-system integrations.
- Create custom layouts, controls, drawings, and virtualized views without crossing an unsafe implementation boundary.
- Deliver international text editing, keyboard access, and assistive-technology behavior as release requirements.
- Run multiple independent windows across displays without coupling an idle view to an animated view.
- Diagnose rendering and lifecycle failures without collecting private user content or requiring remote telemetry.
- Test layout, interaction, semantics, and rendering without an interactive desktop session.
- Upgrade the rendering foundation without accepting unknown platform regressions or unbounded maintenance work.

## Positioning

For application developers who need responsive cross-platform desktop interfaces with predictable resource behavior, Oxyflut is a native UI software development kit that unifies composition, rendering, platform interaction, and testing. Unlike a hosted application framework, it doesn't require a secondary application-language runtime in production.

## Product principles

- P0 means release-blocking. A documented implementation plan doesn't satisfy a P0 capability.
- Desktop completeness takes priority over adding more platforms.
- Accessibility, international text, recovery, and multi-window behavior are core product behavior.
- Measurements and working probes decide substrate eligibility. Architectural plausibility doesn't.
- A platform-specific absence can be not applicable only when cited evidence establishes that absence.

## Appendix: Operator preferences

The operator prefers a safe Rust public API and production binaries that don't start a Dart virtual machine or execute Dart code. The rendering foundation must be selected between two version-pinned Flutter-derived families: the standalone Impeller C SDK or a full Flutter Engine with a language-neutral runtime boundary. The latter replaces the application runtime while retaining selected engine subsystems. Starling is an example of this. It demonstrates runtime substitution with Swift. Tier 1 targets are macOS, Windows through Win32, and Linux through both Wayland and X11. These preferences guide later technical stages but don't replace the product requirements.
