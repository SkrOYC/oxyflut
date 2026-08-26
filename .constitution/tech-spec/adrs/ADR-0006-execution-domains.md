# Execution domains

- **Status:** accepted for Phase 3A
- **Date:** 2026-08-26

## Context

The architecture assigns callback serialization, mutable product state, cancelable asset work, and graphics-affine operations to distinct logical domains.

## Decision

Keep four logical executors even when the host-callback and application executors share one operating-system thread. Platform integration owns a bounded multi-producer, single-consumer callback-intake queue. Native callbacks validate and copy bounded payloads into that queue, request one host wakeup, and return without running application code. At a nonreentrant event-loop checkpoint, the application executor drains normalized events and exclusively mutates component, layout, interaction, text, semantics, and view policy.

The asynchronous worker executor owns bounded loading and decoding queues. Every job and completion carries its runtime, view, request, and resource generations plus cancellation state. The graphics executor owns a bounded command queue and all graphics-affine resource creation, submission, presentation acknowledgement, recovery, and release. The adapter supplies the required runner when the candidate owns a graphics thread; otherwise the host installs and pumps it through the same ordered contract.

Only immutable owned values and generation identifiers cross queues. Mutable application owner objects are neither `Send` nor `Sync`, while their copyable generation identifiers are `Send + Sync` but grant no access by themselves. Worker inputs and results are `Send`; immutable scenes and decoded data can be `Send + Sync`; native views and graphics handles are `Send` only when the selected candidate documents transfer to their owner executor and are never `Sync`. Each queue has one declared producer policy, one owner, a fixed capacity, and a wakeup that coalesces while pending.

Every crossing carries an owner generation. Reentrant callbacks queue after the active mutation. Teardown marks the owner closing, rejects new commands, calls `begin_shutdown`, cancels workers, drains or rejects late callback and worker completions, drains graphics work through an absolute monotonic deadline, releases graphics resources, and then deletes canonical state and callback user data. Queue saturation returns a typed resource-limit result or performs an operation-specific coalescing rule; it never grows a queue or invokes work inline.

## Consequences

- No general async runtime is part of Phase 3A.
- Cross-domain operations require bounded queues and structured cancellation.
- `on_wakeup` and `pump_platform_tasks` form the integrated adapter's host wakeup protocol; neither is an application executor.
- A candidate that cannot obey the ordering and teardown contract is ineligible.
