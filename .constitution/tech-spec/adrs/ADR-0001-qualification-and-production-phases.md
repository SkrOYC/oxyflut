# Qualification and production specification phases

- **Status:** accepted for Phase 3A
- **Date:** 2026-08-26

## Context

The PRD requires symmetric evidence before substrate selection. Neither candidate has passed the P0 gates, the common-case layout visit cap remains open, and qualification proceeds in the declared candidate and Tier 1 environment sequence. A single production-ready Stage 3 specification would imply decisions that the evidence does not support.

## Decision

Stage 3 has two mandatory phases:

- Phase 3A uses a pre-1.0 version and specifies the qualification sequence, common contracts, probes, evidence, and the `shared-runtime` scope.
- Phase 3B begins only after qualification makes a final selection. It removes the losing candidate from production, freezes final contracts, and releases Stage 3 v1.0.0 or later.

The `contracts/specification-phase.json` file is the machine-readable authority for the active phase. Phase 3A sets `productionReady` to `false` and permits qualification and `shared-runtime` planning. Shared substrate-neutral crates and the candidate-neutral `oxyflut-substrate` contract crate can use a null or test substrate without a readiness flag. For each Tier 1 environment, `candidateImplementationReady` gates only candidate-adapter and engine-bridge work, and `measurementReady` gates comparable or scored evidence after completed candidate source identities are pinned. ADR-0011 defines the environment sequence and provisional-selection boundary.

Phase 3B requires typed immutable references for the qualification lock, candidate evidence, deterministic selection decision, assessor consensus, layout cap and corpus, accepted ADR-0010, final contracts and targets, all-Tier-1 P0 and constraint success, losing-candidate removal, production bill of materials, and release qualification. The cross-file validator resolves every path and digest and verifies that they bind to the same lock, candidate, and Stage 3 version. Changing booleans or supplying unverified hashes cannot promote the specification.

## Consequences

- Stage 4 can plan shared-runtime work now, candidate-adapter and engine-bridge work only after the target environment's candidate-implementation gate, and comparable or scored qualification probes only after the target environment's measurement gate.
- The integrated candidate is qualified first; the focused candidate enters only after the integrated candidate fails hard-gate eligibility in the first qualification environment.
- A first-environment selection is provisional and permits only next-environment adapter work; it cannot authorize Phase 3B or removal of the untriggered candidate build recipe.
- Stage 4 cannot plan production framework delivery from Phase 3A.
- Candidate-specific qualification code is temporary and can be deleted only after final selection.
- Phase 3B requires an explicit Stage 3 revision; an informal decision or implementation preference cannot promote Phase 3A.
