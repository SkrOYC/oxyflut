# Qualification and production specification phases

- **Status:** accepted for Phase 3A
- **Date:** 2026-08-26

## Context

The PRD requires symmetric evidence before substrate selection. Neither candidate has passed the P0 gates, and the common-case layout visit cap remains open. A single production-ready Stage 3 specification would imply decisions that the evidence does not support.

## Decision

Stage 3 has two mandatory phases:

- Phase 3A uses a pre-1.0 version and specifies both qualification candidates, common contracts, probes, and evidence.
- Phase 3B begins only after qualification selects one eligible candidate. It removes the losing candidate from production, freezes final contracts, and releases Stage 3 v1.0.0 or later.

The `contracts/specification-phase.json` file is the machine-readable authority for the active phase. Phase 3A sets `productionReady` to `false` and permits only qualification planning. The separate qualification lock initially has `candidateImplementationReady: false` and `measurementReady: false`. The first flag permits candidate implementation only after the suite and reference inputs are frozen. The second permits comparable evidence only after completed candidate source identities are also pinned.

Phase 3B requires typed immutable references for the qualification lock, candidate evidence, deterministic selection decision, assessor consensus, layout cap and corpus, accepted ADR-0010, final contracts and targets, all-Tier-1 P0 and constraint success, losing-candidate removal, production bill of materials, and release qualification. The cross-file validator resolves every path and digest and verifies that they bind to the same lock, candidate, and Stage 3 version. Changing booleans or supplying unverified hashes cannot promote the specification.

## Consequences

- Stage 4 can plan qualification scaffolding until the pre-implementation suite is ready, candidate implementation after that gate, and qualification probes only after the measurement gate.
- Stage 4 cannot plan production framework delivery from Phase 3A.
- Candidate-specific qualification code is temporary and can be deleted after selection.
- Phase 3B requires an explicit Stage 3 revision; an informal decision or implementation preference cannot promote Phase 3A.
