# Local diagnostics

- **Status:** accepted for Phase 3A
- **Date:** 2026-08-26

## Context

P0 requires bounded, privacy-safe, machine-local diagnostics and explicitly excludes an exporter.

## Decision

Use `data-models/diagnostic-event.schema.json` for durable diagnostic files and the Rust contract for in-process records. Emission is one-way and nonblocking. Records contain monotonic timestamps, bounded-lifetime runtime, view, and frame identifiers, stable names, and dropped-record counters. The versioned diagnostic registry is the sole privacy-classification authority; event files cannot override its event or field classifications.

## Consequences

- Private clipboard, editing, input method editor, accessibility, file-path, and platform-message content cannot enter records.
- Sink failure increments loss counters and cannot fail frame processing.
- Adding an exporter requires a Stage 1 Evolution pass and a new security and privacy decision.
