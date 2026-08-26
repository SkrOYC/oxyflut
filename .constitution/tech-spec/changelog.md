# Technical specification changelog

All notable changes to the technical specification appear in this file.

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
