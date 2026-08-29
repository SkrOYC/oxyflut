# Product capabilities

All P0 capabilities are release-blocking on every Tier 1 environment unless a capability states otherwise.

## Composition and layout

| Priority | Capability ID | Capability | Rationale |
| :-- | :-- | :-- | :-- |
| P0 | CAP-CMP-001 | The system must let application developers define mutable reactive state. | State changes drive user-interface updates. |
| P0 | CAP-CMP-002 | The system must derive cached values from reactive dependencies. | Derived state must remain consistent without manual invalidation. |
| P0 | CAP-CMP-003 | The system must run lifecycle-bound side effects when their reactive dependencies change. | Applications need controlled integration with external effects. |
| P0 | CAP-CMP-004 | When an application batches state changes, the system must publish their effects atomically and coalesce dependent work. | Intermediate states must not trigger redundant or inconsistent updates. |
| P0 | CAP-CMP-005 | When application state changes, the system must update only dependent parts of the component tree. | Fine-grained updates keep interactive work bounded. |
| P0 | CAP-CMP-006 | When a component leaves the tree, the system must release its subscriptions and owned lifecycle resources. | Automatic cleanup prevents dangling work and manual teardown errors. |
| P0 | CAP-CMP-007 | When keyed components move within a dynamic collection, the system must preserve their state, focus, scroll position, and reusable render state. | Reordering must not appear as deletion and recreation. |
| P0 | CAP-LAY-001 | The system must lay out components through bounded constraint propagation. | Predictable layout work protects the frame budget. |
| P0 | CAP-LAY-002 | The system must let extension authors define safe custom layout policies. | Applications need advanced controls without unsafe substrate access. |
| P0 | CAP-SCR-001 | The system must create virtualized viewports whose work depends on visible content rather than total collection size. | Large data views require bounded work. |
| P0 | CAP-SCR-002 | The system must provide platform-appropriate wheel, precision-pointer, touch, momentum, and boundary scrolling behavior. | Scrolling must match user expectations on each environment. |

## Rendering, assets, and views

| Priority | Capability ID | Capability | Rationale |
| :-- | :-- | :-- | :-- |
| P0 | CAP-REN-001 | The system must let extension authors record vector paths, shapes, gradients, transforms, clips, filters, images, and reusable pictures through the safe public surface. | Applications need custom drawing without substrate access. |
| P0 | CAP-REN-002 | The system must let extension authors draw realized textures through the safe public surface. | Dynamic and decoded pixel content requires explicit texture drawing. |
| P0 | CAP-REN-003 | The system must retain compositing state for opacity, clipping, transforms, reusable subtrees, and effects that read existing scene content. | Complex scenes must not require full CPU rerasterization. |
| P0 | CAP-AST-001 | When an application requests an asset, the system must load it asynchronously without blocking interactive processing. | Asset input must not stall a frame. |
| P0 | CAP-AST-002 | When an application requests image decoding, the system must perform the work asynchronously and permit cancellation. | Decode work can be expensive or become obsolete. |
| P0 | CAP-AST-003 | The system must cache reusable decoded resources within declared memory limits. | Repeated use must not repeat avoidable work. |
| P0 | CAP-AST-004 | The system must realize decoded pixels as graphics resources and preserve ownership through upload, use, and teardown. | CPU and graphics lifetimes must remain deterministic. |
| P0 | CAP-VIEW-001 | The system must operate multiple views with independent metrics, focus, input, semantics, invalidation, lifecycle, and teardown. | Desktop applications require isolated windows. |
| P0 | CAP-VIEW-002 | The system must schedule frames from display-synchronized presentation opportunities and expose the corresponding frame timestamps. | Animation correctness depends on an explicit timing source. |
| P0 | CAP-VIEW-003 | When a view receives several invalidations before its next eligible frame, the system must coalesce them into one scheduled update. | Duplicate scheduling wastes work. |
| P0 | CAP-VIEW-004 | When views occupy displays with different timing, the system must pace each active view from its associated display without rendering an idle peer. | Shared pacing wastes work and produces incorrect animation. |
| P0 | CAP-VIEW-005 | The system must render and return pixels without creating a visible or hidden top-level window or connecting to an interactive display service. | Automated tests require truly surfaceless rendering. |
| P0 | CAP-REC-001 | When a recoverable presentation or graphics fault occurs, the system must restore valid output within the applicable recovery deadline and preserve framework state. | Normal desktop lifecycle events must not terminate or reset the application. |

## Interaction and text

| Priority | Capability ID | Capability | Rationale |
| :-- | :-- | :-- | :-- |
| P0 | CAP-INP-001 | The system must route pointer and touch input through bounds-pruned hit testing. | Input work must avoid unrelated subtrees. |
| P0 | CAP-INP-002 | The system must resolve competing gestures through one deterministic disambiguation model. | Nested pan, zoom, tap, and drag interactions need consistent ownership. |
| P0 | CAP-FOC-001 | The system must provide focus scopes, keyboard routing and traversal, directional navigation, and visible focus indicators. | Keyboard and accessibility operation require explicit focus behavior. |
| P0 | CAP-TXT-001 | The system must render styled bidirectional text from fonts registered at runtime. | Applications need international typography and runtime fonts. |
| P0 | CAP-TXT-002 | The system must expose complete caret, boundary, range, affinity, and selection geometry for styled bidirectional text. | Rich editing depends on consistent geometry. |
| P0 | CAP-TXT-003 | The system must provide insertion, replacement, grapheme and word deletion, undo, redo, and keyboard and pointer selection for rich text. | Desktop editing requires more than text display. |
| P0 | CAP-IME-001 | When the operating environment starts text composition, the system must preserve composition, candidate geometry, surrounding text, replacement, commit, cancellation, actions, metadata, index conversion, focus transfer, and sensitive-field behavior. | International input requires complete round-trip integration. |
| P0 | CAP-CLP-001 | The system must provide copy, cut, and paste while preserving rich-text selection behavior and private-content boundaries. | Clipboard operations are part of the editing contract. |
| P0 | CAP-I18N-001 | When locale or text direction changes, the system must propagate locale, render bidirectional text, and mirror directional layout where required. | International applications need coherent text and layout direction. |

## Accessibility

| Priority | Capability ID | Capability | Rationale |
| :-- | :-- | :-- | :-- |
| P0 | CAP-SEM-001 | The system must maintain an incremental semantics tree that preserves every applicable role-specific property, relation, state, value, geometry, text range, traversal rule, and view identity. | Assistive technologies require a complete and stable representation. |
| P0 | CAP-SEM-002 | When an accessibility service invokes an action, the system must route its payload to the correct live view and semantics node and return a defined acknowledgement or stale-target error. | Accessible content isn't operable without reliable reverse actions. |

## Platform delivery and verification

| Priority | Capability ID | Capability | Rationale |
| :-- | :-- | :-- | :-- |
| P0 | CAP-PLT-001 | The system must satisfy every P0 capability on macOS, Windows, Wayland, and X11 before the first production release. | Partial desktop coverage doesn't meet the product promise. |
| P0 | CAP-OS-001 | The system must integrate required operating-system cursors and application lifecycle behavior without exposing unsafe substrate handles. | Desktop applications depend on behavior beyond rendering. |
| P0 | CAP-OS-002 | The system must let applications invoke required operating-system services, including dialogs and platform messages, without exposing unsafe substrate handles. | Applications need system services through one safe surface. |
| P0 | CAP-TST-001 | The test harness must pump frames and simulate pointer, touch, keyboard, and gesture input deterministically. | Interaction tests require controlled time and input. |
| P0 | CAP-TST-002 | The test harness must assert layout and semantics deterministically. | Structural regressions need direct evidence. |
| P0 | CAP-TST-003 | The test harness must inject every supported recoverable lifecycle and graphics fault. | Recovery claims require reproducible failures. |
| P0 | CAP-TST-004 | The test harness must compare rendered output under pinned environments and declared cross-environment metrics. | Rendering regressions need stable evidence. |
| P0 | CAP-DST-001 | The project must produce installable, signed, attributable, license-complete, and independently verifiable artifacts for every Tier 1 environment. | Users and maintainers need trustworthy release artifacts. |
| P0 | CAP-SEC-001 | Before qualification, the project must inventory each implemented external ingress and trust boundary and must define threats, payload limits, and mitigations for each one. | Security evidence must match the candidate's actual attack surface. |

## Local diagnostics

| Priority | Capability ID | Capability | Rationale |
| :-- | :-- | :-- | :-- |
| P0 | CAP-DIA-001 | The system must emit versioned local-diagnostic records with stable event names and field-level privacy classifications. | Tools and maintainers need a durable, reviewable contract. |
| P0 | CAP-DIA-002 | The system must bound diagnostic buffers and sampling and must report dropped records. | Diagnostics must not create unbounded resource use or hide data loss. |
| P0 | CAP-DIA-003 | The system must correlate diagnostic records on a monotonic clock with bounded-lifetime identifiers for each runtime, view, and frame. | Failures must be attributable without stable user identifiers. |
| P0 | CAP-DIA-004 | The system must expose diagnostic records only to machine-local, user-controlled sinks inside the application trust boundary. | P0 troubleshooting must not require exported telemetry. |

## Substrate qualification

| Priority | Capability ID | Capability | Rationale |
| :-- | :-- | :-- | :-- |
| P0 | CAP-SUB-001 | Before selection, every substrate candidate must pass the same complete P0 capability set and every applicable safety, security, privacy, performance, recovery, diagnostics, distribution, licensing, provenance, and upgrade constraint under one frozen evidence suite. | Candidate symmetry prevents a preferred design from receiving weaker gates. |
| P0 | CAP-SUB-002 | The project must reject a substrate candidate that fails or retains an unresolved gating P0 item. | A plan or architectural argument isn't release evidence. |
| P0 | CAP-SUB-003 | If zero candidates are eligible, the project must reopen substrate research; if one is eligible, it must select that candidate; if two are eligible, it must apply the frozen weighted comparison. | Every eligibility outcome needs a deterministic decision. |
| P0 | CAP-SUB-004 | If two eligible candidates differ by fewer than 5 weighted points, the project must select the candidate with lower measured upgrade-maintenance cost; equal or inconclusive evidence keeps the decision open. | Maintenance cost is the explicit final tie-break. |

Substrate candidates and Tier 1 environments enter the frozen suite in the qualification sequence defined in `constraints.md`; a provisional selection cannot satisfy CAP-PLT-001.

## Later platform delivery

| Priority | Capability ID | Capability | Rationale |
| :-- | :-- | :-- | :-- |
| P1 | CAP-PLT-002 | The system can extend the complete application model to iOS and Android after desktop release. | Mobile-device delivery is valuable but doesn't reduce desktop P0. |
| P2 | CAP-PLT-003 | The system can extend the application model to WebAssembly delivery after a separate product and architecture revision. | Browser delivery has different rendering and platform contracts. |
| P2 | CAP-PLT-004 | The system can extend the application model to embedded Linux without a desktop compositor after a separate product and architecture revision. | Embedded delivery has different lifecycle and resource constraints. |
