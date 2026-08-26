# Oxyflut

Oxyflut is a native user interface (UI) software development kit for building responsive desktop applications in Rust. It combines reactive composition, rich text, input, accessibility, window management, rendering, and deterministic testing behind a safe application-facing API.

Oxyflut runs in process with the application. It integrates directly with operating-system services and doesn't require a secondary application-language runtime.

## Core capabilities

Oxyflut provides one application model for the complete desktop UI lifecycle:

- **Composition and layout:** Reactive state, derived values, lifecycle-bound effects, batched updates, keyed components, custom layouts, and virtualized views.
- **Rendering:** Vector graphics, gradients, transforms, clips, filters, images, textures, retained compositing state, and surfaceless output.
- **Input and focus:** Pointer and touch hit testing, gesture resolution, keyboard routing, focus scopes, traversal, and directional navigation.
- **Text and editing:** Styled bidirectional text, runtime fonts, rich-text editing, selection geometry, input method editor integration, clipboard operations, and locale-aware layout.
- **Accessibility:** Incremental semantics trees, stable node identity, platform properties and relations, and routed accessibility actions.
- **Windows and displays:** Independent metrics, focus, input, semantics, frame scheduling, lifecycle, and recovery for each view.
- **Testing and diagnostics:** Deterministic time and input, layout and semantics assertions, fault injection, pixel comparison, and bounded local diagnostics.

## Architecture

Oxyflut uses a layered, in-process architecture with explicit ownership boundaries. A small Rust API separates application code from component lifecycle, layout, text, platform integration, scene composition, and the rendering substrate.

Each view owns its mutable state and presentation lifecycle. Shared services can cache immutable resources, but one view's activity doesn't force work in another view. Platform events pass through a single normalization boundary before they reach product logic.

Unsafe graphics and operating-system interactions remain behind narrow interfaces. Owned commands and immutable data cross worker and graphics domains, which keeps teardown and resource lifetimes explicit.

## Platform scope

Oxyflut defines complete desktop behavior across these Tier 1 environments:

- macOS
- Windows through Win32
- Linux through Wayland
- Linux through X11

Accessibility, international text, multi-window behavior, recovery, and deterministic testing are release requirements on every Tier 1 environment.

## Project documentation

The project documentation traces product requirements into architecture, technical contracts, and implementation tasks:

- [Product vision](.constitution/prd/vision.md)
- [Product capabilities](.constitution/prd/capabilities.md)
- [Architecture strategy](.constitution/architecture/strategy.md)
- [Logical boundaries](.constitution/architecture/containers.md)
- [Technical stack](.constitution/tech-spec/stack.md)
- [Contracts and data models](.constitution/tech-spec/data-models/README.md)
- [Implementation plan](.constitution/tasks/critical-path.md)
