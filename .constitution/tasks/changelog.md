# Engineering execution plan changelog

All notable changes to the Stage 4 execution plan appear in this file.

## [v0.5.0] - 2026-08-29

### Added

- Added Epic E for specification landings and Linux readiness, Epic F for the integrated substrate candidate on Linux, and Epic G for the shared application runtime.
- Added the time-boxed `SPK-F001` Dart-free integrated-candidate build spike.

### Changed

- Recomputed the active backlog to 94 points and the critical path through `OXY-E001`, `OXY-E002`, `OXY-E003`, `OXY-E004`, `OXY-E008`, `OXY-F003`, `OXY-F004`, and `OXY-F005`.

### Noted

- macOS and Windows reference hardware remain blocked, the focused substrate candidate remains conditional on the first-environment hard-gate trigger, and comparable measurement and scoring remain deferred.

## [v0.4.0] - 2026-08-29

### Added

- Added the pre-implementation readiness reconciliation report: `.constitution/reports/pre-implementation-readiness.md`.

### Changed

- Recomputed the critical path to zero active story points.
- Archived Epic D at `.constitution/tasks/completed/EPIC-D-readiness-reconciliation.md`.

### Noted

- No readiness flag was set; every `measurementPolicy` field, `resolvedTools`, and `referenceEnvironments` capture remains missing.
- `Known gaps routed to OXY-D001` in `.constitution/tech-spec/changelog.md` is superseded only by Stage 3's v0.16.0 entry and is not edited here.
- Stage 3 applies the reconciliation-checklist tiers that need no external input and releases the technical specification; `/planning-engineering-execution` then produces the next Stage 4 workload-definition epic. `candidateImplementationReady: true` remains the gate for candidate implementation and measurement, not for planning workload-definition work.
- macOS arm64 and Windows x86-64 reference hardware and Assessor 2 remain blocked external inputs.

## [v0.3.0] - 2026-08-28

### Added

- Added six completed spike reports for the Tier 1 platform baselines, the common-case layout visit cap, and the shared security-patch and fuzz-corpus policy: `.constitution/spikes/SPK-B001.md` through `.constitution/spikes/SPK-B006.md`.
- Added the reference-hardware access register in `.constitution/reports/reference-hardware-access.md` with owner attestations, host-discovery and X11-access probes, and per-environment conformance and second-configuration findings.
- Added the assessor coordination record in `.constitution/reports/qualification-assessors.md` with the frozen scoring criteria, independence rules, evidence-access procedure, written-consensus procedure, and second-assessor confirmation procedure.

### Changed

- Recomputed the critical path: `OXY-D001` is the single remaining active ticket at 2 story points, and every Epic B dependency is available to it; OXY-B008 is CLOSED AS BLOCKED as a named external input whose acceptance pass log remains unmet.
- Archived Epic B with its completion record and deviations, including the `.constitution/tasks/active/EPIC-B-readiness-research.md` to `.constitution/tasks/completed/EPIC-B-readiness-research.md` rename in this release.
- Routed every Epic B Stage 3 revision, retained gating KU, and blocked external input into the `OXY-D001 Inputs from Epic B` section of `.constitution/tasks/active/EPIC-D-readiness-reconciliation.md`.

### Noted

- macOS arm64 and Windows x86-64 reference-hardware access is blocked. Neither environment has a named accountable owner or a usable access procedure, so neither can be compared with its Stage 3 reference.
- OXY-B008 is CLOSED AS BLOCKED. Only one confirmation is preserved, so its acceptance pass log remains unmet; the second-assessor confirmation is a named external input for OXY-D001, `measurementPolicy.assessors` stays `null`, and the assessor gate is not complete.
- The authorship-independence rule is unresolved. Stage 1 must approve and apply the PRD amendment before Stage 3 can update the conforming qualification contracts, and until then a disclosed candidate-code or qualification-evidence authorship remains a gating conflict.
- The numeric layout visit cap, the Tier 1 platform and protocol floors, and the independent presentation-opportunity meters remain gating known unknowns. This Stage 4 iteration cannot set `candidateImplementationReady`.

## [v0.2.22] - 2026-08-28

### Fixed

- Applied PR review round 10 corrections in `a6179d0`: restored fail-closed `contracts validate` host handling, preserved typed `lock status` blocking, checked promotion before final tool verification, propagated staged-host failures in readiness tests, and restricted Mesa pairings to allowlisted Linux drivers.

## [v0.2.21] - 2026-08-28

### Fixed

- Applied PR review round 9 corrections in `28d8c5e`: classified unverifiable staged hosts as valid-but-open, skipped unreachable readiness toolchains, retained per-receipt macOS failures, and added Windows and X11 collector coverage.

### Noted

- Template generation remains library-only until OXY-D001 defines the measurement-harness contract and assigns a command surface.

## [v0.2.20] - 2026-08-28

### Fixed

- Applied PR review round 8 corrections in `e9adbbe`: accepted Debian package-version tildes, typed macOS and Windows capture-bound failures, enforced staged external-proposal and resolved-tool invariants, and raised only Linux protocol-source captures to 256 KiB pending OXY-D001 confirmation.

### Noted

- OXY-D001 must state the raw-measurement per-`(constraintId, launch)` monotonic-clock scope, define Wayland interface-set completeness, and confirm the temporary Linux protocol-source capture bound against real Ubuntu 26.04 output sizes.

## [v0.2.19] - 2026-08-28

### Fixed

- Applied PR review round 7 corrections: `df7a288` preserves external snapshot bytes, rejects whitespace-only baseline fields, and bounds macOS and Windows observations; updated completion records reconcile review commits.

## [v0.2.18] - 2026-08-28

### Fixed

- Applied the PR review round 6 CI fix in `ac14a9e`. Readiness fixtures retain manifest-relative Rustup paths, and test loaders resolve the host-specific prefix through `pathRoot`.

## [v0.2.17] - 2026-08-28

### Fixed

- Applied PR review round 6 corrections in `e5036d8` and `00f4908` for Linux session collection, resolved-tool validation, complete-fixture verification, raw-clock ordering, evidence writer visibility, and manifest-bound readiness fixtures.

### Noted

- OXY-D001 must define the raw-measurement monotonic-clock scope and Wayland required interface-set completeness.

## [v0.2.16] - 2026-08-28

### Fixed

- Applied PR review round 4 corrections in `5ee7b8c` for Windows release normalization, bounded Linux protocol collection, reference-environment pins, artifact-pair cleanup, readiness diagnostics, staged input verification, KU evidence paths, and PRD meter parsing.
- Amended 2026-08-28: Recorded PR review round 5 commits `8d163ae`, `1234f60`, and `7e79086` for fail-closed protocol capture, immutable artifact-pair cleanup, and publication-result clarification.

### Noted

- OXY-D001 must type the PRD launch and per-launch observation budgets in the measurement-harness contract. The staged sample-validity record deliberately excludes them.

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
