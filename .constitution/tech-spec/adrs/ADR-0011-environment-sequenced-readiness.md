# Environment-sequenced qualification readiness

- **Status:** accepted for Phase 3A
- **Date:** 2026-08-29

## Context

D2 requires Tier 1 qualification in a declared environment sequence and makes a selection from complete first-environment evidence provisional. D3 makes `thinkpadp14s` the Linux reference configuration for Wayland and X11, while macOS and Windows remain blocked on accountable hardware. The existing qualification lock has global readiness flags, which cannot represent readiness independently for the four environments.

## Decision

Qualify environments in this order: Wayland, X11, macOS, then Windows. For each environment, `candidateImplementationReady` permits candidate-adapter and engine-bridge work only for that environment, and `measurementReady` permits comparable or scored evidence only for that environment after completed candidate source identities are pinned. The integrated candidate enters the frozen suite first. Build the focused candidate only if the integrated candidate fails hard-gate eligibility in Wayland, the first qualification environment.

A selection supported by complete Wayland evidence is provisional. It permits adapter work for the provisional selection in X11 and preserves the focused candidate build recipe when its trigger has not occurred. It does not permit Phase 3B, final candidate removal, or a production implementation plan. A selection is final only after all four Tier 1 environments pass under the frozen suite.

## Pre-implementation inputs per environment

Lock v6 scopes `candidateImplementationReady` to one Tier 1 environment. That environment's candidate-implementation gate requires these `measurementPolicy` fields: `rawMeasurementSchema`, `sampleValidityRules`, `capabilityBaseline`, `platformContracts`, `fuzzCorpora`, `securityPatchRehearsal`, `externalContractLock`, `layoutVisitCap`, `layoutVisitCorpus`, `layoutQualificationRecordSchema`, `layoutPrequalificationRunSchema`, `layoutPrequalificationSuiteSchema`, `layoutVisitCountingRules`, and `layoutPrequalificationIdentities`.

That environment's candidate-implementation gate also requires `referenceEnvironments.<environment>.minimumVersion`, `hardwareId`, `gpuId`, `driverVersion`, and `systemPackageLockDigest` for that environment only.

## Reference-environment minimum versions

A reference environment's `minimumVersion` is the version set observed on its own reference session by `environment inspect`. Capture the version set once and freeze it in the lock as that environment's floor. The lock must not claim a floor that differs from the captured evidence.

For Wayland, the capture records the compositor version and the advertised protocol and interface versions. For X11, the capture records the Xwayland and Xvfb server versions and the X protocol version.

`measurementPolicy.scoringAnchors` and `measurementPolicy.assessors` move to that environment's `measurementReady`. They are required there only when two substrate candidates have entered qualification; a one-candidate provisional selection requires neither field. Completed candidate source identities remain required for every environment's `measurementReady`.

## Layout-cap prequalification

To resolve the `layoutVisitCap` KU, run the SPK-B005 layout prequalification suite against the shared `oxyflut-layout` crate with a `null-substrate` on the Linux reference host. Record ordinary visits, attempted ordinary visits, intrinsic queries, and application-owned layout time as substrate-independent; record paint-submission time as not applicable. This suite requires no candidate adapter.

The next Stage 4 epic must revise the SPK-B005 layout record, run, and suite schemas, their fixture corpora, and the custom validator so the `candidate` enum includes `null-substrate` and `paintSubmissionNs` is nullable for that candidate. Enforce the schema and fixture changes with `schema_compiles_committed_contract_instances_and_fixture_corpus`, `run_fixture_corpus`, and the `layout-prequalification validate` custom-validator fixture corpus.

## Per-environment lock status

The lock v6 command contract is `lock status --gate candidate-implementation --environment ENVIRONMENT` and `lock status --gate measurement --environment ENVIRONMENT`. Each form exits 0 when the requested environment is ready, 2 when the lock is valid but that environment remains open, and 1 when the lock is invalid. Without `--environment`, `lock status` reports every environment and exits 2 unless every environment is ready.

## Capability baseline approval

The approved 52-capability baseline is an Epic E deliverable, not an external input. Produce it with `baseline validate --input PATH`, publish its canonical draft with that command's `--output` form, and record project-owner approval in the baseline provenance fields. Enforce publication and approval with `baseline validate` and `validate_capability_baseline`.

## Workload input contract

The next Stage 4 epic must add `data-models/workload-input.schema.json` and its fixture corpus. The schema types every `workload.*` lock entry as an object with `schemaVersion`, `field`, `id`, `description`, `path`, `sha256`, `byteCount`, `identities`, and `license`.

The `field` enum contains `referenceApplication`, `scenes`, `interactionScripts`, `fonts`, `assets`, `windowMatrix`, `cacheStates`, and `releaseFlags`. Each `path` is repository-relative under `qualification/staged/workload/`. The `identities` object contains `name` and `revision`. The `license` value is an SPDX expression or an identifier that begins with `LicenseRef-`.

Each of the lock's eight workload digests binds the SHA-256 of its workload-input document. The schema and its fixtures must change `schema_count` from 23 to 24 and pass `run_fixture_corpus`.

## Provisional selection artifact

The next Stage 4 epic must migrate `qualification-evidence.schema.json` so its `environments` object records per-environment eligibility. The object requires results and eligibility only for Tier 1 environments that have entered the declared sequence. It records every later environment with the explicit `not-entered` state and no result record. An entered environment carries the identical frozen suite required by CAP-SUB-001.

The next Stage 4 epic must migrate `selection-decision.schema.json` with a candidate-state enum of `entered`, `untriggered`, and `ineligible`. A record with one `entered` candidate and one `untriggered` candidate is valid. An `untriggered` candidate has no candidate evidence or score. A candidate that entered and failed a hard gate is `ineligible`.

The migrated selection-decision record has `selectionState` of `provisional` or `final`. It is `provisional` after complete evidence for the first entered Tier 1 environment. It changes to `final` only after every Tier 1 environment passes under the same qualification-lock digest. Assessor scores are required only when two candidates are `entered` and eligible.

The current schemas remain unchanged in this milestone because their bytes are digest-bound. The next Stage 4 epic must change `LOCK_SCHEMA`, the `readiness_promotion.rs` final-selection checks, the qualification-evidence and selection-decision schema fixture corpora, and every affected digest in one transaction. The changelog routes the schema, fixture, assertion, and digest landings.

## Consequences

The committed v5 lock remains global and requires its fully resolved `measurementPolicy` for `candidateImplementationReady`. The next Stage 4 epic must land `qualification-lock.schema.json` v6 with `qualificationSequence.candidateOrder`, `qualificationSequence.environmentOrder`, `referenceEnvironments.<environment>.candidateImplementationReady`, and `referenceEnvironments.<environment>.measurementReady`, applying the per-environment field allocation in this ADR.

That epic must change `LOCK_SCHEMA`, `candidate_input_issues`, `measurement_input_issues`, and `validate_documents_with_attribution` in `xtask/src/contracts/readiness.rs`; `POLICY_FIELDS` and `KNOWN_UNKNOWN_BINDINGS` in `crates/oxyflut-qualification/src/readiness.rs`; the final-selection checks in `xtask/src/contracts/readiness_promotion.rs`; per-environment parsing and reporting in `xtask/src/commands/lock.rs`; and the exact readiness assertions in `xtask/src/commands/lock_tests.rs`. The `candidate_input_issues`, `POLICY_FIELDS`, `KNOWN_UNKNOWN_BINDINGS`, and `lock_tests.rs` exact sets enforce the D6 readiness-rule change. The lock-schema, fixture, and assertion migration must land in one digest-reconciled change.

Shared substrate-neutral crates and the candidate-neutral `oxyflut-substrate` contract crate remain plannable with a null or test substrate and do not depend on an environment readiness flag. Measurement and production promotion remain gated as stated above.
