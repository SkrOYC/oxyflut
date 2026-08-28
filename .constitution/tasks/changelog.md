# Engineering execution plan changelog

All notable changes to the Stage 4 execution plan appear in this file.

## [v0.2.15] - 2026-08-28

### Fixed

- Completed Epic C PR review rounds 1 and 2: `25dc154`, `ca0a25f`, `b983d0d`, `923ce71`, `f0d9b6b`, `91e7b60`, `2422a57`, `30ca7cf`, and `ff9d6a8`.
- Applied PR review round 3 corrections for SLSA schema derivation, reference-environment validation, immutable artifact pairs, readiness reporting, and meter parsing.

### Noted

- OXY-D001 must assign owners for workload, scoring-anchor, corpus, and sample-validity baseline validation. `contracts validate` covers platform and accessibility baselines, and `measurement validate` covers sample-validity records.

## [v0.2.14] - 2026-08-28

### Completed

- Completed and archived Epic C tickets OXY-C001–OXY-C005: `a731182`, `316f932`, `2fb134e`, `81ede6f`, `383021a`, `3831555`, `fd32fb9`, `3b0fc3a`, `05ee6f1`, and `c0015f5`.

### Noted

- OXY-D001 must route the raw-measurement `$schema` declaration, `measurementPolicy.sampleValidityRules` schema, proposed external-contract lock, `PATH.inventory.json` companion inventory schema, path-less staged measurement-policy input references, and `resolvedTools.pathRoot` Stage 3 gaps.

## [v0.2.13] - 2026-08-27

### Completed

- Completed and archived Epic A tickets OXY-A001–OXY-A008: `94966fc`, `9d9cece`, `000448d`, `ce3019b`, `971f12e`, `ddbc70c`, `f2c9087`, `7ceea79`, `6a8447e`, `a245e4b`, `538fde1`, `908c265`, `1a0545f`, `93999fe`, `8b23150`, `5e38738`, `8162b47`, and `39dd00c`.
- Reconciled and archived Epic A with `ced1cd7`.
- Completed PR review rounds 1–4: `e2d362e`, `923ad31`, `0f9aa53`, `f80e27a`, `13536a9`, `62019ad`, `09041ab`, `03ab783`, `fe3e533`, `4339a01`, `73b58e6`, and `chore: final review polish for the qualification foundation (PR review round 4)`.

### Changed

- Updated the Prettier checker command lines in the Epic B and Epic D plans to use the Nix-declared `prettier` executable.

### Noted

- OXY-D001 must record the three routed Stage 3 schema gaps and the required devenv toolchain.

## [v0.2.12] - 2026-08-26

### Fixed

- Required unique absent-event IDs and exact gate, event, candidate, environment, platform-baseline digest, schema, and active specification joins for `not-applicable-kk`.
- Required aggregate not-applicable constraints to prove absence across all four Tier 1 environments.

## [v0.2.11] - 2026-08-26

### Fixed

- Added eligible `not-applicable-kk` schema fixtures and matching frozen-platform-baseline evidence checks.

## [v0.2.10] - 2026-08-26

### Fixed

- Added ABI-9 rejection and closed presence and reserved-field fixtures for the ABI-10 semantics-selection contract.
- Required approved baseline provenance, digest-bound approval evidence, and matching typed lock references before readiness.

## [v0.2.9] - 2026-08-26

### Fixed

- Added ABI-7 and ABI-8 rejection, semantics-selection presence, and texture-realization parity fixtures for ABI 9.
- Replaced abstract baseline and measurement `PATH` placeholders with concrete positive fixtures and direct negative-fixture test commands.

## [v0.2.8] - 2026-08-26

### Fixed

- Required ABI-7 rejection and omitted texture-drawing edge fixtures for the ABI-8 qualification contracts.
- Kept valid-but-open exit code 2 exclusive to `lock status`; external-contract fixture verification now returns 0 only for complete staged fixtures.

## [v0.2.7] - 2026-08-26

### Fixed

- Moved the shared streaming SHA-256 primitive into schema foundation so dependency-ready validators can hash evidence without duplicating ownership.
- Added reusable-scene, presentation-status, and machine-local diagnostic-admission parity fixtures.

## [v0.2.6] - 2026-08-26

### Fixed

- Registered root evidence and toolchain modules, one canonical environment module, and an evidence-command placeholder during workspace scaffolding.
- Required reverse semantics-action ingress and active specification-version mismatch fixtures in capability traceability validation.

## [v0.2.5] - 2026-08-26

### Fixed

- Registered compile-safe command, crate, and validator placeholders before later tickets replace them.
- Gave pre-aggregation validator tickets direct module-test commands and made native validation depend on schema scaffolding.

## [v0.2.4] - 2026-08-26

### Fixed

- Moved stable command dispatch and placeholder ownership into workspace scaffolding so dependency-ready validator tickets can run their declared commands.
- Required the complete capability-to-contract edge matrix and Linux Unicode-scalar accessibility fixtures.

## [v0.2.3] - 2026-08-26

### Fixed

- Required file-qualified traceability resolution and immutable evidence for every nested KK platform claim.
- Gave raw-measurement validation an independent command and matching `xtask` scope.

## [v0.2.2] - 2026-08-26

### Fixed

- Required accessibility-map dereferencing, identity and digest checks, aggregate KK enforcement, and nested KU rejection.
- Added canonical path-alias and control-character negative fixtures.

## [v0.2.1] - 2026-08-26

### Fixed

- Required superseded-schema fixtures, canonical link-target validation, and artifact-root confinement in the contract validators.
- Aligned native-tool resolution acceptance with the expanded qualification-lock tool entries.

## [v0.2.0] - 2026-08-26

### Added

- Added a prerequisite ticket that resolves and stages exact native-contract tool identities before header validation.

### Changed

- Required registry-level diagnostic validation, unique canonical artifact paths, unique raw-sample keys, and exclusion-free valid samples.
- Clarified that this iteration produces tooling and reports but cannot close the candidate-implementation readiness gate.
- Pinned documentation checks to Prettier 3.9.6.

## [v0.1.0] - 2026-08-26

### Added

- Added the pre-implementation qualification foundation, research, lock-input tooling, and reconciliation epics.
- Added atomic tickets with explicit dependencies, Fibonacci estimates, bounded scopes, stop conditions, and mode-tagged acceptance evidence.
- Added six time-boxed spike reports for the four Tier 1 platform baselines, the common-case layout visit cap, and the shared security-patch and fuzz-corpus decision.
- Added an explicit handoff checkpoint that requires Stage 3 reconciliation before candidate implementation planning.

### Security

- Kept candidate code, measurements, and production planning outside active scope while their machine-readable readiness gates remain false.
