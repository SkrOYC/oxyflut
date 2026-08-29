# Glossary

Use the following terms throughout the constitution:

| Term | Definition | Do not use |
| :-- | :-- | :-- |
| Application developer | A developer who builds an application with Oxyflut's safe public surface. | App author, consumer |
| Application runtime | The Oxyflut-owned state, component, layout, interaction, and lifecycle behavior inside an application process. | Framework core, app engine |
| Attempted ordinary visit | A requested ordinary visit counted before the applicable per-child cap check. It remains recorded when that check rejects the invocation. | Requested ordinary visit, attempted layout visit |
| Authorship independence | An eligibility condition that prevents a person who authors candidate implementation or qualification evidence from independently scoring that candidate. It preserves independent assessment. | Scorer independence |
| Capability baseline | The candidate-neutral complete set of required P0 capability behavior and evidence. | Adapter checklist, implementation baseline |
| Campaign host | A non-reference host that runs fuzzing and security campaigns. It is not a qualification environment and cannot supply reference-environment evidence. | Campaign machine |
| Component | A stable unit of user-interface composition with identity, state, lifecycle, layout, and rendering behavior. | Widget, element |
| Display epoch | A measured interval during which a view has one display association and display timing source. Evidence for the same interval uses one display-epoch equality tuple. | Vsync period, screen session |
| Display-epoch equality tuple | The identity tuple that establishes whether observations belong to the same display epoch. It includes the display association and `targetModeSignature`, the declared display-timing mode signature. | Display-epoch identity tuple |
| Input method editor | Operating-system text-entry behavior for composition, conversion, candidate selection, and commit. | IME bridge, text service |
| Known unknown (KU) | A named unanswered question that blocks the relevant gate until evidence resolves it. | Assumption, resolved item |
| Known known (KK) | A claim backed by cited, immutable, verified evidence. | Belief, implementation intention |
| Layout prequalification suite | A frozen corpus and procedure that assesses layout behavior under declared candidate and environment identities. It records required timing, counter, and validity evidence. | Layout prequalification test suite |
| Local diagnostics | Bounded diagnostic records written only to machine-local, user-controlled sinks within the application trust boundary. | Telemetry, analytics |
| Exported telemetry | Diagnostic records that cross the machine boundary or another declared trust boundary. | Local diagnostics, local logging |
| Ordinary visit | A regular layout request from a policy to a realized direct child in one root layout transaction. It completes only after it passes the applicable per-child cap. | Regular layout visit |
| Presentation opportunity | An independently observed display event at which a completed frame can be presented. | Tick, frame callback |
| Provisional selection | A substrate selection made from complete evidence for the first qualified Tier 1 environment. It becomes final only after every Tier 1 environment passes. | Temporary selection |
| Qualification sequence | The declared progression of substrate candidates and Tier 1 environments for readiness and selection. It evaluates the integrated candidate first and the focused candidate only after the integrated candidate fails hard-gate eligibility in the first Tier 1 environment. | Qualification order |
| Rendering substrate | The replaceable rendering and presentation foundation beneath the application runtime. | Engine, backend |
| Second-configuration score-4 evidence | Qualification evidence from a physically distinct hardware configuration that supports score 4 for a criterion. It is not a candidate score and does not substitute for a complete qualification result. | Score-4 second-configuration evidence |
| Semantic-role registry | A governed record of canonical semantic roles and their stable identities. It lets semantics tree evidence name roles consistently during qualification. | Semantic-role catalog |
| Semantics tree | The framework-owned accessible representation of visible content, relationships, state, geometry, and actions. | Accessibility tree, a11y tree |
| Substrate candidate | One complete rendering-substrate family evaluated against the same release gates. | Path, option |
| Surface recovery | Restoration of valid presentation after resize, surface loss, display change, resume, or recoverable graphics-device loss. | Reset, restart |
| Tier 1 environment | A release-blocking desktop environment: macOS, Windows, Wayland, or X11. | Primary platform, supported desktop |
| View | An independently measured, focused, rendered, and accessible application surface associated with a window or headless target. | Viewport, surface |
| Window | An operating-environment top-level container that can be associated with one view. | View, viewport |
