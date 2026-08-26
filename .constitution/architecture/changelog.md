# Architecture changelog

All notable changes to the logical architecture appear in this file.

## [v1.0.0] - 2026-08-26

### Added

- Defined the layered in-process software development kit and separate qualification plane.
- Defined application-runtime, view, platform, rendering-substrate, diagnostics, test, and release boundaries.
- Defined candidate-invariant ownership, failure handling, privacy, observability, and compatibility rules.
- Added one critical flow for every P0 capability in product requirements v1.0.0.
- Recorded architecture risks, sensitivity points, threat notes, and temporary dual-candidate debt.

### Fixed

- Replaced generic flow templates with capability-specific behavior and failure views.
- Separated canonical policy ownership from candidate-delegated mechanisms and added one allocation view per substrate candidate.
- Added execution domains, a complete ingress register, distinct recovery contracts, and one-way diagnostics.
