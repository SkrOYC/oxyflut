# Text indexing and geometry

- **Status:** accepted for Phase 3A
- **Date:** 2026-08-26

## Context

Editing, input method editor transactions, platform services, rendering, and accessibility use different text index units. Silent conversion errors corrupt selections and candidate geometry.

## Decision

The Rust editing model stores Unicode scalar text and exposes strong index types for UTF-8 bytes, UTF-16 code units, grapheme boundaries, and logical text positions. Conversion functions validate boundaries and return structured errors. Rendering geometry and semantics ranges reference one immutable text-layout generation.

## Consequences

- Bare integer indices cannot cross public, platform, or substrate boundaries.
- Candidate adapters must prove round-trip conversions with the shared multilingual corpus.
- Phase 3B freezes the production index contract after both candidates complete the geometry probe.
