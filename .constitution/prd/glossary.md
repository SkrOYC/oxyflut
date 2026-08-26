# Glossary

Use the following terms throughout the constitution:

| Term | Definition | Do not use |
| :-- | :-- | :-- |
| Application developer | A developer who builds an application with Oxyflut's safe public surface. | App author, consumer |
| Application runtime | The Oxyflut-owned state, component, layout, interaction, and lifecycle behavior inside an application process. | Framework core, app engine |
| Capability baseline | A candidate-neutral statement of required platform behavior and evidence for one environment. | Adapter checklist, implementation baseline |
| Component | A stable unit of user-interface composition with identity, state, lifecycle, layout, and rendering behavior. | Widget, element |
| Display epoch | A measured interval during which a view has one display association and display timing source. | Vsync period, screen session |
| Input method editor | Operating-system text-entry behavior for composition, conversion, candidate selection, and commit. | IME bridge, text service |
| Local diagnostics | Bounded diagnostic records written only to machine-local, user-controlled sinks within the application trust boundary. | Telemetry, analytics |
| Exported telemetry | Diagnostic records that cross the machine boundary or another declared trust boundary. | Local diagnostics, local logging |
| Presentation opportunity | An independently observed display event at which a completed frame can be presented. | Tick, frame callback |
| Rendering substrate | The replaceable rendering and presentation foundation beneath the application runtime. | Engine, backend |
| Semantics tree | The framework-owned accessible representation of visible content, relationships, state, geometry, and actions. | Accessibility tree, a11y tree |
| Substrate candidate | One complete rendering-substrate family evaluated against the same release gates. | Path, option |
| Surface recovery | Restoration of valid presentation after resize, surface loss, display change, resume, or recoverable graphics-device loss. | Reset, restart |
| Tier 1 environment | A release-blocking desktop environment: macOS, Windows, Wayland, or X11. | Primary platform, supported desktop |
| View | An independently measured, focused, rendered, and accessible application surface associated with a window or headless target. | Viewport, surface |
| Window | An operating-environment top-level container that can be associated with one view. | View, viewport |
