# Technical specification changelog

All notable changes to the technical specification appear in this file.

## [v0.2.1] - 2026-08-26

### Fixed

- Defined content-free diagnostics and exit codes for qualification commands, including the distinct valid-but-open result from `lock status`.

## [v0.2.0] - 2026-08-26

### Added

- Added concrete pre-implementation commands for evidence verification, external-contract verification, baseline validation, environment inspection, and qualification-lock readiness status.

### Changed

- Bound Stage 4 pre-lock tooling tickets to named commands rather than leaving command design to the execution plan.

## [v0.1.1] - 2026-08-26

### Fixed

- Split pre-implementation suite readiness from measurement readiness so the candidate source commits can be produced without creating a circular lock dependency.
- Required the suite, environments, baselines, assessors, tools, corpora, security-patch rehearsal, external-contract snapshots, and layout cap before candidate implementation begins.
- Kept comparable and scored evidence blocked until completed candidate and adapter source identities are pinned.

## [v0.1.0] - 2026-08-26

### Added

- Added the concrete shared qualification toolchain and both pinned substrate candidate profiles.
- Added the target workspace, coding rules, compatibility policy, verification-command contract, ADRs, raw Rust and C surfaces, and durable JSON Schemas.
- Added machine-readable capability traceability and specification-phase state.
- Added exact keyed qualification evidence, selection-decision and promotion-evidence gates, raw measurement records, capability and platform baselines, accessibility maps, diagnostic registry, ingress threat inventory, release evidence, CI invocation, and external-contract locks.
- Separated the Flutter framework, engine, header, candidate, adapter, and artifact identities in the qualification lock.

### Security

- Defined unsafe-boundary containment, dependency-policy tools, ingress evidence, private-content exclusions, and Dart-free binary inspection.

### Known limitations

- This release is Phase 3A and isn't production-ready.
- Substrate selection and the common-case layout visit cap remain gating known unknowns.
- Candidate implementations, complete platform and accessibility maps, independent timing sources, reference hardware and workloads, SDK artifact digests, measurement inputs, scoring assessors, and locally snapshotted distribution contracts remain explicit gating known unknowns in `contracts/qualification-lock.json`.
- The implementation workspace and qualification commands don't exist.
- Phase 3B is mandatory before production Stage 4 planning. Phase 3B must select one eligible candidate, remove the losing candidate from production, freeze the final contracts, and release Stage 3 v1.0.0 or later.
