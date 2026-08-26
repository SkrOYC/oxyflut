# Resilience and cross-cutting concerns

## Ownership and state integrity

Each application runtime owns its component graph. Each view owns mutable window, display, focus, semantics, invalidation, and presentation state. Shared caches contain immutable entries or entries with explicit owners and release acknowledgements. No boundary can infer a default view when more than one view exists.

Every asynchronous request carries an owner identity and cancellation state. Completion after owner teardown is discarded with a recorded reason. Resource realization uses staged ownership: source data, decoded data, substrate resource, submitted use, and release acknowledgement.

## Failure handling

Production boundaries return structured failures. They don't continue with partial state when ownership, identity, or index validation fails. Callback failures remain inside their boundary, and teardown is idempotent.

Surface recovery is a bounded state machine. It stops ordinary submission while application-runtime and component state remain live. Resize starts from the later of the final resize event and resource availability and has a two-refresh-interval deadline. Surface loss has a 250 ms deadline. Resume or display-topology change has a 500 ms deadline. Recoverable graphics-device loss has a 2-second deadline. Recovery recreates only invalidated resources, caps transient graphics memory at 2x steady state, and requires superseded-resource release acknowledgement within 500 ms after success or terminal failure. Three failed attempts produce a structured terminal error. A platform capability baseline marks genuinely unsupported events as not applicable and preserves the cited evidence.

Asset loading, decoding, and operating-environment requests use bounded queues and explicit cancellation. Queue saturation rejects or coalesces work according to the operation's semantics. It doesn't allocate an unbounded backlog.

## Frame-overrun policy

The View coordinator coalesces invalidations until the next eligible presentation opportunity. Work that misses an opportunity remains attributable to that view and doesn't trigger an idle peer. The system reports the miss and continues from the next eligible opportunity without fabricating timestamps.

## Trust boundaries and ingress

The application process trusts only validated, owned data. Platform integration, the Rendering-substrate boundary, Asset and resource manager inputs, and Local diagnostics sink acknowledgements cross its edge. The qualification plane is separate from candidate artifacts, artifact producers, and independent-verifier inputs.

The following logical register defines invariant ingress ownership. Candidate qualification expands each row into the final implemented-ingress inventory and records candidate-specific additions or cited absences.

| Source | Owning boundary | Validation and cap responsibility | Privacy class | Failure containment |
| :-- | :-- | :-- | :-- | :-- |
| Application assets, fonts, and images | Asset and resource manager | Format validation, byte and decoded-size caps, cancellation, and allocation limits. | Application content. | Reject the resource and release partial stages. |
| Pointer, touch, keyboard, window, display, and lifecycle events | Platform integration | Event shape, view identity, ordering, and rate bounds. | Interaction metadata. | Reject or coalesce the event for its target view. |
| Input method editor, clipboard, and platform-message content | Platform integration and Text and editing | Transaction, focus, index, size, sensitive-field, and ownership validation. | Raw private content. | Preserve the last valid editing state and emit content-free diagnostics. |
| Accessibility properties and actions | Platform integration and Semantics | View and node identity, role mapping, payload, index, and stale-target validation. | Potentially private semantic content. | Reject the update or action without retargeting. |
| Candidate callbacks, resources, and errors | Rendering-substrate boundary | Compatibility, handle ownership, lifetime, thread domain, size, and error validation. | Internal state and timing. | Contain the error and notify the canonical owner. |
| Local-sink acknowledgement or failure | Local diagnostics | Sink identity, machine-local trust status, and bounded response handling. | Content-free operational metadata. | Drop the record and increment the loss counter. |
| Candidate artifacts and evidence | Release qualification | Identity, integrity, provenance, completeness, environment, and reproducibility validation. | Release evidence. | Reject the artifact or candidate. |
| Independent verification result | Release qualification | Verifier identity, scope, artifact binding, and result completeness. | Release evidence. | Keep qualification open. |

Unsafe handles remain inside the Rendering-substrate boundary and never enter application or extension code.

Text, clipboard, input method editor, and semantics content is private. Local diagnostics record classifications, sizes, result categories, and bounded identifiers instead of raw content. Production has no exported-telemetry boundary.

## Diagnostics

Every production boundary can make a one-way, nonblocking emission to Local diagnostics. Admission applies privacy classification, sampling, and buffer capacity before enqueueing. Records use a monotonic timebase and bounded-lifetime application-runtime, view, and frame identifiers. Buffer saturation or sink failure increments a dropped-record counter. Diagnostic failure cannot return control flow to a producer, stop frame processing, or expand memory without a bound.

The Test and verification harness correlates independent operating-environment observations with candidate records. Candidate records can explain a failure but cannot replace an independent meter required by the PRD.

## Configuration

Reference environments, capability baselines, queue limits, recovery caps, diagnostic sampling, privacy classifications, and qualification meters are versioned inputs. Production defaults are immutable during one run. A configuration mismatch fails before the rendering substrate accepts other work.

## Compatibility and change

The Rendering-substrate boundary performs compatibility negotiation before creating a view or resource. Unsupported or ambiguous capability states fail qualification. An upgrade cannot silently weaken a capability baseline, change ownership, or broaden a trust boundary.
