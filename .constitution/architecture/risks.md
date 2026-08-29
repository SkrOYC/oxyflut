# Architecture risks

## Risk register

| Risk ID | Risk and sensitivity point | Consequence | Mitigation or follow-up |
| :-- | :-- | :-- | :-- |
| ARC-R01 | The integrated substrate candidate is qualified first; if it clears hard-gate eligibility on the first qualification environment, it resolves the open selection provisionally. A small responsibility-allocation change can alter memory, startup, pacing, and maintenance cost. | A provisional selection can't freeze one concrete rendering substrate contract before all Tier 1 environments pass. | Keep canonical per-view policy above the boundary, qualify the integrated candidate under the frozen suite first, and build the focused candidate only if the integrated candidate fails hard-gate eligibility on the first qualification environment. |
| ARC-R02 | The numeric common-case layout visit cap is a gating known unknown. | A layout design can meet shallow benchmarks but scale poorly. | Record the corpus and Table 4 finite per-policy visit-cap freeze as partially discharged; retain the numeric global layout-visit cap as the remaining gating condition until the prequalification lock binds candidate and environment identities and the 48-tuple timing probe supplies schema-valid evidence under CON-PERF-001 on unblocked reference hardware. |
| ARC-R03 | Shared mutable state can couple views on different displays. | Idle views render, timing becomes incorrect, or focus and semantics leak between views. | Scope mutable state by application runtime and view; test two displays and teardown under independent observation. |
| ARC-R04 | Text geometry can diverge from rendered text. | Carets, selection, input method editor candidates, and accessibility ranges become incorrect. | Keep shaping, layout, selection geometry, and semantics ranges in one Text and editing boundary with shared index conversions. |
| ARC-R05 | Asynchronous resource completion can race teardown. | Memory corruption, leaks, or stale texture use. | Carry owner identity and cancellation through every stage and require release acknowledgement. |
| ARC-R06 | Integrated substrate behavior can bypass canonical platform or view state. | Candidate parity appears to pass while service routing or recovery differs. | Treat inherited behavior as an implementation behind the same logical contracts and verify outcomes outside candidate callbacks. |
| ARC-R07 | Strict release gates can leave no substrate candidate eligible: the integrated candidate fails hard-gate eligibility on the first qualification environment, and the focused candidate is then also ineligible. | The project has no selected rendering substrate. | Preserve the zero-candidate outcome and reopen substrate research without reducing P0. |
| ARC-R08 | Diagnostic instrumentation can perturb the frame and memory meters. | Qualification results describe the observer rather than production behavior. | Measure matched variants and enforce CON-DIA-001. |
| ARC-R09 | A single in-process fault can affect the whole application. | A substrate or parser failure can terminate all views. | Contain panics and exceptions at boundaries, validate inputs, fuzz ingresses, and preserve structured terminal failures. Process isolation remains a later architecture option if evidence requires it. |
| ARC-R10 | Host callbacks can reenter product state or arrive after teardown. | State corrupts, owners leak, or stale work targets a replacement view. | Serialize callbacks in Platform integration, use generation-scoped owners, disable callbacks before draining, and reject late completion. |
| ARC-R11 | Memory pressure can collide with zero-allocation frame work. | Cache eviction or allocation failure disrupts a frame or exceeds a cap. | Keep eviction outside measured paint traversal, reserve bounded frame resources, coalesce optional work, and fail allocation explicitly. |
| ARC-R12 | Provisional selection from one Tier 1 environment can be reversed by later environment evidence. Platform-integration allocation is the sensitivity point. | An early allocation decision can be invalidated after additional candidate work is planned. | Keep canonical policy above the rendering substrate boundary and prohibit Phase 3B promotion until all Tier 1 environments pass. |
| ARC-R13 | Each Linux Tier 1 environment has one reference configuration in one compositor/session family and no second configuration. | No second-configuration evidence exists. | Record this as a blocked external input, not as a pass. |

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

The focused candidate adapter isn't built unless the integrated candidate fails hard-gate eligibility on the first qualification environment. The retained contract seam and the untriggered focused candidate build recipe are intentional structural debt; they preserve the later qualification path without maintaining two live adapters.
