# Public Rust compatibility

- **Status:** accepted for Phase 3A
- **Date:** 2026-08-26

## Context

Application developers need a safe Library/SDK surface that hides candidate handles and logical-boundary implementation details.

## Decision

Use the contract in `contracts/oxyflut-public.rs` as the qualification public-surface baseline. Public types use strong identifiers, structured errors, explicit ownership, and no raw pointers. Before v1.0.0, breaking changes require a changelog entry and fixture migration. After v1.0.0, public compatibility follows semantic versioning.

## Consequences

- Candidate adapters cannot add candidate-specific public types.
- Qualification can refine the contract before Phase 3B, but every change must update traceability and contract tests.
- Phase 3B freezes the first production compatibility baseline.
