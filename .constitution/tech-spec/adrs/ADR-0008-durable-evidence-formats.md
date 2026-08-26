# Durable evidence formats

- **Status:** accepted for Phase 3A
- **Date:** 2026-08-26

## Context

Qualification creates durable diagnostic records, artifact manifests, ingress inventories, environment locks, and candidate decisions. These shapes need validation and migration without adding a database.

## Decision

Use JSON encoded as UTF-8 and validated against the Draft 2020-12 schemas in `data-models/`. Canonical hashing and provenance follow the stricter PRD distribution contract. Preserve original evidence and create a new derived artifact during migration.

## Consequences

- Network schema resolution is disabled.
- Schema identifiers and versions are explicit.
- Breaking schema changes require a major schema version and a migration note.
- No database or remote evidence service is introduced.
