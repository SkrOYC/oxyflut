# Technical specification changelog

All notable changes to the technical specification appear in this file.

## [v0.16.0] - 2026-08-29

### Added

- Added ADR-0011 for environment-sequenced readiness and provisional selection.
- Added the `shared-runtime` Phase 3A planning scope.

### Changed

- Applied sequential qualification: the integrated candidate enters first, and the focused candidate is built only on the first-environment hard-gate failure trigger.
- Applied per-environment readiness boundaries for candidate adapters, the engine bridge, and measurement; shared substrate-neutral crates and the candidate-neutral `oxyflut-substrate` contract crate are plannable with a null or test substrate.
- Re-pinned the Linux reference configuration and reference-host validation to `thinkpadp14s` on NixOS 26.05 with a Hyprland Wayland session, interactive Xwayland, and headless Xvfb; the Ubuntu 26.04 LTS Linux references are superseded.
- Rebound 14 raw-measurement and sample-validity fixture `lockDigest` values after the Linux lock change and updated the Tier 1 environment-pin assertion.
- Updated the scope guard, candidate rules, and verification-command contract, including the per-crate shared-runtime test command.

### Fixed

- Defined Linux `hardwareId` as the DMI product-name value, retained both Linux lock values as `null` because the preserved probe has no DMI output, and required separate hostname, AMD Renoir GPU-family, and Hyprland compositor validation.
- Accepted an X11 inspection in the declared Wayland session only with a matching Xwayland process, `DISPLAY`, and responding X server, while retaining the separate Xvfb path.
- Restored blocked `cargo +1.98.0 deny check advisories` wording until a pinned offline RustSec advisory database and refresh policy are bound.
- Kept the active machine-readable `specificationVersion` at `0.15.0`: a trial replaced all 62 `0.15.0` hits in `xtask`, `qualification`, and `.constitution/tech-spec`, updated the corrupt-platform-baseline mutation literal, and left `contracts validate` green, but `cargo test --workspace --all-features` failed 10 digest-bound fixture and readiness tests after changed hashed bytes invalidated parent digests. The full version migration and its digest cascade are routed through T6.1-T6.3 and T8.x.

### Scheduled landings routed to the next Stage 4 epic

- T1.1: `.constitution/tech-spec/data-models/{capability-traceability,specification-phase,raw-measurement}.schema.json`; land the routed schema revisions; enforce with `discover_contract_instances` and traceability contract-test resolution; see [T1](../reports/pre-implementation-readiness.md#t1-schema-creation-and-migration).
- T1.2: `.constitution/tech-spec/data-models/{semantic-role-registry,semantic-role-registry-snapshot}.schema.json`; land the proposed registry schemas; enforce with `run_fixture_corpus`; see [T1](../reports/pre-implementation-readiness.md#t1-schema-creation-and-migration).
- T1.3: `.constitution/tech-spec/data-models/accessibility-map.schema.json`; land the v6 accessibility-map migration; enforce with `ACCESSIBILITY_MAP_SCHEMA` and accessibility-map validation; see [T1](../reports/pre-implementation-readiness.md#t1-schema-creation-and-migration).
- T1.4: `.constitution/tech-spec/data-models/qualification-lock.schema.json`; land the v6 lock fields; enforce with `LOCK_SCHEMA` and claimed-ready policy validation; see [T1](../reports/pre-implementation-readiness.md#t1-schema-creation-and-migration).
- T1.5: `.constitution/tech-spec/data-models/{layout-qualification-record,layout-prequalification-run,layout-prequalification-suite}.schema.json`; land the proposed layout schemas; enforce with `schema_compiles_committed_contract_instances_and_fixture_corpus`; see [T1](../reports/pre-implementation-readiness.md#t1-schema-creation-and-migration).
- T1.6: `PATH.inventory.json`; type the environment inventory and Wayland interface completeness; enforce with `POLICY_FIELDS` and `LOCK_SCHEMA`; see [T1](../reports/pre-implementation-readiness.md#t1-schema-creation-and-migration).
- T1.7: `.constitution/tech-spec/data-models/{qualification-evidence,selection-decision}.schema.json`; migrate per-environment eligibility, `not-entered` records, candidate states, and `selectionState`; enforce with `LOCK_SCHEMA` and schema validation; see ADR-0011.
- T2.1: `qualification/fixtures/contracts/{semantic-role-registry,semantic-role-registry-snapshot}/`; land the proposed fixture corpora; enforce with `run_fixture_corpus`; see [T2](../reports/pre-implementation-readiness.md#t2-schema-fixture-corpora).
- T2.2: `qualification/fixtures/contracts/accessibility-map/` and `migration/accessibility-map-v5-to-v6.{input,expected}.json`; land the accessibility-map corpus and migration pair; enforce with `run_fixture_corpus`, `$schema` discovery, and `validate_migration_fixture`; see [T2](../reports/pre-implementation-readiness.md#t2-schema-fixture-corpora).
- T2.3: `qualification/fixtures/contracts/{layout-qualification-record,layout-prequalification-run,layout-prequalification-suite}/`; land layout fixture corpora; enforce with `run_fixture_corpus`; see [T2](../reports/pre-implementation-readiness.md#t2-schema-fixture-corpora).
- T2.4: qualification-lock and readiness fixture corpora plus the qualification-lock migration pair; land the v6 fixture migration; enforce with `LOCK_SCHEMA`, `run_fixture_corpus`, and `validate_migration_fixture`; see [T2](../reports/pre-implementation-readiness.md#t2-schema-fixture-corpora).
- T2.5: `xtask/src/contracts/schema.rs`; classify the layout-prequalification corpus directory; enforce with `schema_compiles_committed_contract_instances_and_fixture_corpus`; see [T2](../reports/pre-implementation-readiness.md#t2-schema-fixture-corpora).
- T2.6.1: `xtask/src/commands/external_contracts.rs`; land external-fixture sidecar validation; enforce with `cargo run -q -p xtask -- external-contracts verify`; see [T2.6](../reports/pre-implementation-readiness.md#t26-external-fixture-preservation-and-sidecar-validation).
- T2.6.2: `qualification/fixtures/external-contracts/{macos,wayland,x11}/`; preserve the proposed sidecar-backed fixture sets; enforce with `cargo run -q -p xtask -- external-contracts verify`; see [T2.6](../reports/pre-implementation-readiness.md#t26-external-fixture-preservation-and-sidecar-validation).
- T2.6.3: `qualification/fixtures/external-contracts/windows/`; retain the Windows excerpt exemption until reference-host capture; enforce with the SPK-B002 capture procedure; see [T2.6](../reports/pre-implementation-readiness.md#t26-external-fixture-preservation-and-sidecar-validation).
- T2.7: `qualification/fixtures/contracts/{qualification-evidence,selection-decision}/`; add provisional-selection valid and invalid fixture corpora; enforce with `run_fixture_corpus`; see ADR-0011.
- T3.1: `.constitution/tech-spec/contracts/semantic-role-registry.json` and capability traceability; land registry and physical contract-test bindings; enforce with `$schema` discovery and `validate_required_symbol_edges`; see [T3](../reports/pre-implementation-readiness.md#t3-contract-instances).
- T3.1a: `.constitution/tech-spec/contracts/{oxyflut-public.rs,oxyflut-substrate.rs,oxyflut-substrate.h}`; land generated semantic-role definitions; enforce with the generated-role contract test and `validate_required_symbol_edges`; see [T3](../reports/pre-implementation-readiness.md#t3-contract-instances).
- T3.2: `.constitution/tech-spec/contracts/platform-contracts.json` and `stack.md`; land the remaining platform retentions and replacements after external fixtures; enforce with `validate_platform_baseline`; see [T3](../reports/pre-implementation-readiness.md#t3-contract-instances).
- T3.3: `.constitution/tech-spec/contracts/qualification-lock.json`; land typed policy references while retaining source-required nulls; enforce with `candidate_implementation_report` and `LOCK_SCHEMA`; see [T3](../reports/pre-implementation-readiness.md#t3-contract-instances).
- T3.3a: `crates/oxyflut-qualification/src/readiness.rs`; land layout policy bindings and known-unknown collection; enforce with `StagedInputRegistry::candidate_status_input_bindings`, `collect_measurement_policy`, and `collect_known_unknowns`; see [T3](../reports/pre-implementation-readiness.md#t3-contract-instances).
- T3.3b: `xtask/src/contracts/readiness.rs`; land v6 readiness validation; enforce with the readiness family in `validate_workspace`; see [T3](../reports/pre-implementation-readiness.md#t3-contract-instances).
- T3.3c: `xtask/src/commands/environment/mod.rs`; validate v6 lock projections without environment inspection; enforce with `validate_lock_environment_projection`; see [T3](../reports/pre-implementation-readiness.md#t3-contract-instances).
- T3.3d: `xtask/src/commands/lock.rs`; land staged layout digest and identity reporting; enforce with `lock status --gate candidate-implementation` assertions; see [T3](../reports/pre-implementation-readiness.md#t3-contract-instances).
- T3.3e: qualification-lock schema and lock instance plus `xtask/src/toolchain/lock.rs`; preserve resolved-tool classification; enforce with `verify_lock_resolved_tools_classified` and `POLICY_FIELDS`; see [T3](../reports/pre-implementation-readiness.md#t3-contract-instances).
- T3.3f: `xtask/src/contracts/readiness_promotion.rs`; migrate the final-selection checks for provisional and final `selectionState`, entered and untriggered candidate states, and the conditional two-assessor calculation; enforce with the readiness-promotion family; see ADR-0011.
- T3.4: public and qualification Rust contracts plus ADR-0005; land layout counter and probe-contract changes; enforce with Rust-contract compilation and `validate_workspace`; see [T3](../reports/pre-implementation-readiness.md#t3-contract-instances).
- T3.4a: `xtask/src/commands/contracts.rs`; land the external-client layout-contract assertion; enforce with the `rust-contract` family; see [T3](../reports/pre-implementation-readiness.md#t3-contract-instances).
- T3.4b: `xtask/src/commands/layout_prequalification.rs` and its corpus; land the layout-prequalification command; enforce with the custom-validator fixture corpus; see [T3](../reports/pre-implementation-readiness.md#t3-contract-instances).
- T3.4d: `.constitution/tech-spec/data-models/README.md`; land the schema-inventory revisions accompanying the semantic-role and layout schemas; enforce with schema inventory review and `contracts validate`; see [T3](../reports/pre-implementation-readiness.md#t3-contract-instances).
- T3.5: changelog and CI advisory configuration; land remaining migration notes, advisory database policy, and baseline ownership; enforce with `baseline validate`, `measurement validate`, `generate_templates`, and `digests::validate_workspace`; see [T3](../reports/pre-implementation-readiness.md#t3-contract-instances).
- T4.1: `.constitution/tech-spec/contracts/qualification-lock.json`; land the one-transaction known-unknown-array update; enforce with `committed_candidate_gate_is_valid_but_open_with_the_exact_ku_set`; see [T4](../reports/pre-implementation-readiness.md#t4-lock-known-unknown-arrays-as-one-lexicographic-transaction).
- T4.2: readiness fixtures and `crates/oxyflut-qualification/src/readiness.rs`; land B005 and B006 binding changes; enforce with the named cleared-known-unknown and `collect_known_unknowns` checks; see [T4](../reports/pre-implementation-readiness.md#t4-lock-known-unknown-arrays-as-one-lexicographic-transaction).
- T5.1: `xtask/src/commands/lock_tests.rs`; land exact known-unknown and mutation assertions; enforce with the named lock-status tests; see [T5](../reports/pre-implementation-readiness.md#t5-exact-set-and-counter-assertions).
- T5.2: `crates/oxyflut-qualification/src/readiness.rs`; land cleared-fixture exact-set and binding assertions; enforce with `clearing_a_ku_string_without_its_evidence_keeps_the_gate_open` and `KNOWN_UNKNOWN_BINDINGS`; see [T5](../reports/pre-implementation-readiness.md#t5-exact-set-and-counter-assertions).
- T5.3: `xtask/src/contracts/schema.rs`; land schema and instance counter assertions; enforce with `schema_compiles_committed_contract_instances_and_fixture_corpus`; see [T5](../reports/pre-implementation-readiness.md#t5-exact-set-and-counter-assertions).
- T5.4: `xtask/src/contracts/native_tests.rs`; land ABI 11 rejection coverage; enforce with `abi_seven_through_ten_fail_before_callbacks_install`; see [T5](../reports/pre-implementation-readiness.md#t5-exact-set-and-counter-assertions).
- T5.5: `xtask/src/contracts/traceability/{mod.rs,edges.rs,validation.rs,fixtures.rs,tests.rs}`; land accessibility registry edges; enforce with `ACCESSIBILITY_MAP_SCHEMA`, `REQUIRED_ACCESSIBILITY_CATEGORIES`, and `validate_required_symbol_edges`; see [T5](../reports/pre-implementation-readiness.md#t5-exact-set-and-counter-assertions).
- T5.6: `xtask/src/contracts/schema.rs` and migration fixtures; generalize named migration-pair validation; enforce with `validate_migration_fixture`; see [T5](../reports/pre-implementation-readiness.md#t5-exact-set-and-counter-assertions).
- T6.1: `xtask/`, `qualification/`, and `.constitution/tech-spec/`; migrate active specification-version literals and regenerate digests; enforce with the prescribed grep recount and `validate_workspace`; see [T6](../reports/pre-implementation-readiness.md#t6-version-migration).
- T6.2: `guidelines.md` and `stack.md`; reconcile the active-version command and scope text with the completed T6.1 migration; enforce with active-specification equality; see [T6](../reports/pre-implementation-readiness.md#t6-version-migration).
- T6.3: `changelog.md`; close the version-migration release record after digest regeneration; enforce with `digests::validate_workspace`; see [T6](../reports/pre-implementation-readiness.md#t6-version-migration).
- T7.1: the candidate-neutral role-registry fixture and digest; land the generated registry artifact; enforce with `$schema` discovery and registry-pointer edges; see [T7](../reports/pre-implementation-readiness.md#t7-remaining-capture-and-registry-artifacts).
- T7.2: `PATH.inventory.json`; set the capture bound from Linux reference-host output; enforce with the fail-closed capture-bound assertion; see [T7](../reports/pre-implementation-readiness.md#t7-remaining-capture-and-registry-artifacts).
- T8.1: the integrated C header and native fixtures; land ABI 11 and regenerate layout bytes; enforce with interface and layout validation plus the generated-role contract test; see [T8](../reports/pre-implementation-readiness.md#t8-digest-bound-artifacts-frozen-last).
- T8.2: generated bindings and sidecar; regenerate the bindgen golden after header finalization; enforce with `validate_bindings` and the generated-role contract test; see [T8](../reports/pre-implementation-readiness.md#t8-digest-bound-artifacts-frozen-last).
- T8.3: staged fuzz-corpora and security-patch-rehearsal records; create and bind the records; enforce with `POLICY_FIELDS` and `digests::validate_workspace`; see [T8](../reports/pre-implementation-readiness.md#t8-digest-bound-artifacts-frozen-last).
- T8.4: staged layout artifacts and schemas; freeze and bind layout policy artifacts; enforce with the layout-prequalification validator, `POLICY_FIELDS`, and `digests::validate_workspace`; see [T8](../reports/pre-implementation-readiness.md#t8-digest-bound-artifacts-frozen-last).
- T8.5: ADR-0010 and production-3b fixtures; apply the acceptance cascade after the approved evidence migration; enforce with `adr_cites_verified_evidence` and `digests::validate_workspace`; see [T8](../reports/pre-implementation-readiness.md#t8-digest-bound-artifacts-frozen-last).
- T8.6: qualification-evidence and selection-decision parents, sidecars, and dependent lock references; re-freeze every affected digest after the provisional-selection migration; enforce with `digests::validate_workspace`; see ADR-0011.

## [v0.15.0] - 2026-08-26

### Changed

- Amended 2026-08-27: Replaced `bunx prettier@3.9.6` with the immutable Nix-declared `prettier` 3.9.6 executable from the hash-pinned npm tarball. This doesn't change evidence formats or their compatibility.
- Amended 2026-08-27: Added `devenv.nix`, `devenv.yaml`, `devenv.lock`, and `.envrc` as the required reproducible verification shell. Every verification command runs inside `devenv shell`.
- Amended 2026-08-27: The staged native toolchain supports only `x86_64-unknown-linux-gnu`; other Tier 1 hosts remain an OXY-D001 lock input.
- Amended 2026-08-28: Staged external-contract snapshots preserve SPDX 3.0.1 (`Community-Spec-1.0 AND CC-BY-3.0`), in-toto Statement v1 (`Apache-2.0`), SLSA Provenance v1 (`Community-Spec-1.0`), and DSSE Envelope v1 (`Apache-2.0`) source bytes with pinned local verifier adapters. They remain nonauthoritative pending Stage 3 adoption.
- Amended 2026-08-28: Added the nonauthoritative staged `qualification/schemas/sample-validity.schema.json` proposal for `qualification-lock.schema.json#measurementPolicy.sampleValidityRules`.
- Amended 2026-08-28: `environment inspect` writes a lock-compatible projection at `PATH` and a complete, digest-bound `PATH.inventory.json` companion inventory artifact. The companion artifact remains untyped by Stage 3.
- Amended 2026-08-28: Narrowed the `baseline validate` command contract to one candidate-neutral capability baseline. Epic D owns the remaining baseline-validation ownership gap.
- Amended 2026-08-28: Marked `cargo fmt`, `clippy`, `test`, and `contracts validate` as available; `cargo deny` runs `licenses bans sources`, while OXY-D001 owns advisory checks.
- Amended 2026-08-28: Format assertions (`uri`, `date-time`) are enforced for all registry schemas, including `qualification-lock`, `external-contract-lock`, and `ci-invocation`.
- The specification remains v0.15.0 because `contracts/specification-phase.json` and committed baselines require exact `specificationVersion` equality. Updating only the specification version would invalidate those bindings, so this amendment doesn't change the version.
- Advanced platform contracts to v5 with the active specification version and typed immutable absent-event entries keyed by gate, event, environment, and candidate.
- Advanced qualification evidence to v5 so every `not-applicable-kk` gate names an exact absent-event entry through a versioned, digest-bound platform-baseline reference.

### Known gaps (routed to Epic D, now superseded by v0.16.0 scheduled landings)

- `.constitution/tech-spec/data-models/capability-traceability.schema.json` `mappings[].contractTests[]` identifies a contract test but has no physical file location.
- `.constitution/tech-spec/data-models/accessibility-map.schema.json` `reverseActions[].textLayoutBinding` has no text-layout generation value.
- `.constitution/tech-spec/data-models/specification-phase.schema.json` `promotionEvidence.layoutQualification`, `finalContractSet`, `targetMatrix`, `losingCandidateRemoval`, and `billOfMaterials` use generic evidence references instead of typed schemas.
- `.constitution/tech-spec/data-models/raw-measurement.schema.json` omits the `$schema` property, so raw-measurement instances cannot self-declare their schema.
- `.constitution/tech-spec/data-models/raw-measurement.schema.json` doesn't state that `samples[].monotonicNs` is non-decreasing per `(constraintId, launch)`.
- No Stage 3 schema defines `qualification-lock.schema.json#measurementPolicy.sampleValidityRules`; `qualification/schemas/sample-validity.schema.json` is the proposed staged schema and its digest is the proposed binding value.
- The proposed external-contract lock values in `qualification/schemas/external/proposed-external-contract-lock.json` await Stage 3 adoption.
- `xtask environment inspect` writes the `PATH.inventory.json` companion artifact, but no Stage 3 schema defines it and `qualification-lock.schema.json#referenceEnvironments` has no typed reference to it.
- Wayland interface-set completeness has no Stage 3 rule. The companion inventory retains a partial observed `protocolVersion`, which lock v5 cannot represent.
- `qualification-lock.schema.json#measurementPolicy.{scoringAnchors,assessors,fuzzCorpora,securityPatchRehearsal}` binds path-less digests; the repository convention `qualification/staged/<field>.json` is the proposed referent and needs Stage 3 typing.
- `qualification-lock.schema.json#resolvedTools` lacks the `pathRoot` field used by `qualification/tools/native-contract-toolchain.json` for rustup-home-relative tools.

## [v0.14.0] - 2026-08-26

### Changed

- Advanced qualification evidence to v4 so eligible capability and constraint gates can record either `pass` or a cited `not-applicable-kk` result.
- Required cross-file proof that every eligible `not-applicable-kk` result matches a frozen platform-baseline row where the event cannot occur.

## [v0.13.0] - 2026-08-26

### Changed

- Advanced the integrated C ABI to version 10 and closed semantics-selection presence to zero or one while reserving only zero.
- Advanced capability baselines to v4 with synthetic or approved provenance and digest-bound approval evidence.
- Advanced qualification locks to v5 with a typed approved-baseline reference that cannot accept synthetic provenance.
- Normalized Starling feasibility references to the exact “Starling is an example of this” wording.

## [v0.12.0] - 2026-08-26

### Changed

- Advanced the integrated C ABI to version 9 and added explicit semantics-selection presence so absent selections remain distinct from valid zero-length selections.
- Aligned texture realization on nonzero physical dimensions, the closed pixel format, and exact checked packed-byte lengths across common Rust and C.
- Assigned the shared Rust image decoder above the substrate boundary for both qualification candidates.

## [v0.11.0] - 2026-08-26

### Changed

- Advanced the integrated C ABI to version 8 so ABI-7 implementations cannot negotiate successfully without the presentation-status and timestamp semantics.
- Corrected realized-texture traceability to bind texture drawing on the public Rust, common Rust, and integrated C surfaces.

## [v0.10.0] - 2026-08-26

### Added

- Added closed presentation outcomes with explicit success and failure timestamp semantics to the common substrate callback.
- Added typed admission for bounded, user-controlled machine-local diagnostic sinks.
- Pinned `sha2` 0.11.0 as the shared streaming SHA-256 implementation for qualification validators and evidence writers.

### Changed

- Unified reusable pictures with the immutable common scene type and bound scene recording, replay, submission, and presentation symbols into capability traceability.

## [v0.9.0] - 2026-08-26

### Added

- Bound reverse semantics actions to the public Rust, common Rust, and integrated C ingress contracts.

### Changed

- Advanced capability baselines and capability traceability to v3, and moved active specification-version equality to cross-file validation so future specification releases don't require new schema identities.

## [v0.8.0] - 2026-08-26

### Added

- Bound semantics selections, attributed-text ranges, and indexed reverse actions to one immutable text-layout generation.

### Changed

- Advanced the integrated C ABI to version 7 for semantics text-layout identity.
- Advanced accessibility maps to v5 before durable qualification evidence existed.

## [v0.7.0] - 2026-08-26

### Added

- Added a closed typed semantics relation set across the public Rust, common Rust, and C contracts.
- Added Unicode-scalar reverse-action indexing for Linux accessibility maps.
- Added decisive accessibility, selection, release-bundle, and CI contract edges to capability traceability.

### Changed

- Required all 27 aggregate constraint results to pass before a candidate can be eligible; platform-level unsupported events remain in the platform baseline.
- Advanced the integrated C ABI to version 6 for typed semantics relations.
- Advanced accessibility maps to v4, capability baselines to v2, and qualification evidence to v3 before durable qualification evidence existed.

## [v0.6.0] - 2026-08-26

### Added

- Added distinct accessible name, description, help, and attributed-text properties plus reverse-action errors to semantics contracts and accessibility maps.
- Added immutable evidence references to every status-bearing platform claim.
- Added file-qualified symbol bindings to capability traceability.
- Added a closed native input method editor index-unit enum shared by Rust, C, and platform contracts.

### Changed

- Advanced the integrated C ABI to version 5 for the expanded semantics-node layout and native index-unit constants.
- Advanced accessibility maps to v3, platform contracts to v4, and capability traceability to v2 before durable qualification evidence existed.
- Downgraded platform protocol claims without committed immutable evidence from KK to KU.

## [v0.5.0] - 2026-08-26

### Added

- Added strongly tagged text-index units, affinity-preserving paragraph hit testing, and index conversion to both substrate contracts.
- Added explicit failed and canceled input method editor query acknowledgements.
- Added complete headless logical, physical, and device-pixel-ratio metrics at the substrate boundary.
- Added aggregate KK constraints and the complete semantics property set to accessibility maps.

### Changed

- Advanced the integrated C ABI to version 4 for text geometry, input method editor acknowledgements, and headless metrics.
- Advanced accessibility maps to v2 and artifact manifests to v4 before durable qualification evidence existed.
- Rejected control characters, empty path segments, duplicate separators, and trailing separators from canonical artifact paths and link targets.

## [v0.4.0] - 2026-08-26

### Added

- Added shared paragraph drawing, complete semantics-node projections, and bidirectional attributed input method editor queries across the safe Rust, common Rust, and C contracts.
- Added explicit logical size, physical size, and device-pixel-ratio consistency rules for headless views.
- Added closed environment status and KK evidence conditions to the platform-contract schema.

### Changed

- Advanced the integrated C ABI to version 3 for the input method editor response operation and complete semantics layout.
- Expanded resolved-tool lock entries with source identity, host triple, license, executable path, version, and digest.
- Advanced artifact manifests to v3, raw measurements to v2, platform contracts to v3, and qualification locks to v4 before any durable evidence existed.
- Confined link targets, defined link hash semantics, and forbade link metadata on regular files.

## [v0.3.0] - 2026-08-26

### Added

- Added pointer, keyboard, and transaction-level input method editor fields to the common Rust substrate contract.
- Added path clipping, color-matrix filtering, and exact headless-raster descriptors across the Rust and C contracts.
- Added a public headless-view and raster-output surface.
- Added field kinds, bounds, closed values, and registry versions to the diagnostic contracts.

### Changed

- Advanced the integrated C ABI to version 2 after adding handles, table entries, paint state, and raster metadata.
- Advanced the qualification base to Flutter 3.47.0 and kept 3.41.0, 3.44.0, and 3.47.0 as the consecutive upgrade-rehearsal lines.
- Recorded verified standalone Impeller SDK hashes and sizes for the Flutter 3.47.0 engine revision.
- Pinned constitution formatting to Prettier 3.9.6.
- Tightened artifact-path and raw-measurement invariants.

### Security

- Removed arbitrary string values from durable diagnostics and required semantic registry validation of event and field privacy classes.

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
