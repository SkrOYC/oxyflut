# Rust toolchain

- **Status:** accepted for Phase 3A
- **Date:** 2026-08-26

## Context

The application surface requires memory safety, predictable ownership, zero-allocation hot paths, and explicit native boundaries. Qualification also needs one reproducible compiler and package graph.

## Decision

Use Rust 1.98.0, Cargo 1.98.0, edition 2024, resolver version 3, and one committed lockfile. Pin every Phase 3A dependency exactly.

## Consequences

- All crates share one minimum supported Rust version during qualification.
- Compiler upgrades require the full shared suite and native-layout checks.
- Phase 3B must revalidate the pin and can change it only with recorded compatibility and measurement evidence.
