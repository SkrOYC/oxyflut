# Architecture risks

## Risk register

| Risk ID | Risk and sensitivity point | Consequence | Mitigation or follow-up |
| :-- | :-- | :-- | :-- |
| ARC-R01 | The substrate selection governed by CAP-SUB-001 through CAP-SUB-004 remains open. A small responsibility-allocation change can alter memory, startup, pacing, and maintenance cost. | Stage 3 cannot freeze one concrete substrate contract. | Keep canonical per-view policy above the boundary, implement both qualification probes, and use the allocation matrix. |
| ARC-R02 | The numeric common-case layout visit cap is a gating known unknown. | A layout design can meet shallow benchmarks but scale poorly. | Freeze the layout corpus and finite per-policy visit caps before substrate qualification. |
| ARC-R03 | Shared mutable state can couple views on different displays. | Idle views render, timing becomes incorrect, or focus and semantics leak between views. | Scope mutable state by application runtime and view; test two displays and teardown under independent observation. |
| ARC-R04 | Text geometry can diverge from rendered text. | Carets, selection, input method editor candidates, and accessibility ranges become incorrect. | Keep shaping, layout, selection geometry, and semantics ranges in one Text and editing boundary with shared index conversions. |
| ARC-R05 | Asynchronous resource completion can race teardown. | Memory corruption, leaks, or stale texture use. | Carry owner identity and cancellation through every stage and require release acknowledgement. |
| ARC-R06 | Integrated substrate behavior can bypass canonical platform or view state. | Candidate parity appears to pass while service routing or recovery differs. | Treat inherited behavior as an implementation behind the same logical contracts and verify outcomes outside candidate callbacks. |
| ARC-R07 | Strict release gates can leave both candidates ineligible. | The project has no selected rendering substrate. | Preserve the zero-candidate outcome and reopen substrate research without reducing P0. |
| ARC-R08 | Diagnostic instrumentation can perturb the frame and memory meters. | Qualification results describe the observer rather than production behavior. | Measure matched variants and enforce CON-DIA-001. |
| ARC-R09 | A single in-process fault can affect the whole application. | A substrate or parser failure can terminate all views. | Contain panics and exceptions at boundaries, validate inputs, fuzz ingresses, and preserve structured terminal failures. Process isolation remains a later architecture option if evidence requires it. |
| ARC-R10 | Host callbacks can reenter product state or arrive after teardown. | State corrupts, owners leak, or stale work targets a replacement view. | Serialize callbacks in Platform integration, use generation-scoped owners, disable callbacks before draining, and reject late completion. |
| ARC-R11 | Memory pressure can collide with zero-allocation frame work. | Cache eviction or allocation failure disrupts a frame or exceeds a cap. | Keep eviction outside measured paint traversal, reserve bounded frame resources, coalesce optional work, and fail allocation explicitly. |

## Threat notes

| STRIDE category | Boundary or asset | Threat | Architectural control |
| :-- | :-- | :-- | :-- |
| Spoofing | View, semantics node, and callback identity | A stale or forged identifier targets another live view. | Scoped identities, generation checks, and stale-target errors. |
| Tampering | Candidate artifact and qualification evidence | A changed binary or result is evaluated as the frozen candidate. | Immutable evidence, artifact digests, independent verification, and attributable records. |
| Repudiation | Release and substrate decision | An assessor or builder can deny the source of a score or artifact. | Signed provenance, preserved raw evidence, and written consensus records. |
| Information disclosure | Text, clipboard, input method editor, and semantics data | Diagnostics or failures expose raw private content. | Field-level privacy classification and no raw-content collection path. |
| Denial of service | Asset, message, diagnostics, and recovery queues | Untrusted or repeated work exhausts memory or frame time. | Payload caps, bounded queues, cancellation, coalescing, and attempt limits. |
| Elevation of privilege | Unsafe substrate and operating-environment handles | Application or extension code gains unrestricted low-level access. | Safe application surface, opaque ownership, and contained unsafe boundaries. |

## Accepted structural debt

Two substrate adapters and their qualification evidence coexist until CAP-SUB-001 through CAP-SUB-004 produce a selection. This duplication is intentional and temporary. The project must remove the losing candidate from active production architecture after selection, while preserving its evidence report for decision history.
