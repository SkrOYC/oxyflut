# Substrate adapter and engine C ABI

- **Status:** accepted for Phase 3A
- **Date:** 2026-08-26

## Context

Both substrate candidates cross a Rust and C or C++ boundary, but forcing both through another shared C wrapper would distort the focused candidate. Candidate symmetry requires one Rust ownership, callback, error, view, frame, scene, paragraph, bidirectional input method editor, semantics, and recovery contract.

## Decision

Both adapters implement `contracts/oxyflut-substrate.rs`. The focused adapter calls generated bindings for the pinned standalone C SDK. The integrated adapter uses `contracts/oxyflut-substrate.h` as its engine bridge.

The C ABI uses fixed-width integers, opaque handles, explicit retain and release operations, versioned structures, callback user data, structured status values, one exported `OxySubstrateGetApi` negotiation symbol, `OXY_CALL` on every function pointer, and no language-runtime layout. The header is the authoritative raw ABI; the Rust `sys` layer is generated from it and layout-checked against the same C compiler used for the bridge.

Borrowed callback data remains valid only during the callback unless the contract explicitly transfers ownership. Scene submission takes an internal retained reference through presentation acknowledgement or terminal view teardown. Callback user data remains live until `begin_shutdown` disables production and `drain` completes. Semantics actions carry a request generation and receive exactly one typed acknowledgement. Panics and C++ exceptions cannot cross the ABI.

## Consequences

- The focused adapter translates the standalone SDK into the common Rust trait without an extra C hop.
- The integrated adapter implements the C ABI in the engine fork and wraps it with the common Rust trait.
- Any ABI change requires layout tests, generated-binding diffs, and an ABI version change.
- Phase 3B can remove the losing adapter but cannot weaken the accepted ownership contract without a superseding ADR.
