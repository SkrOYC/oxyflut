# Environment-sequenced qualification readiness

- **Status:** accepted for Phase 3A
- **Date:** 2026-08-29

## Context

D2 requires Tier 1 qualification in a declared environment sequence and makes a selection from complete first-environment evidence provisional. D3 makes `thinkpadp14s` the Linux reference configuration for Wayland and X11, while macOS and Windows remain blocked on accountable hardware. The existing qualification lock has global readiness flags, which cannot represent readiness independently for the four environments.

## Decision

Qualify environments in this order: Wayland, X11, macOS, then Windows. For each environment, `candidateImplementationReady` permits candidate-adapter and engine-bridge work only for that environment, and `measurementReady` permits comparable or scored evidence only for that environment after completed candidate source identities are pinned. The integrated candidate enters the frozen suite first. Build the focused candidate only if the integrated candidate fails hard-gate eligibility in Wayland, the first qualification environment.

A selection supported by complete Wayland evidence is provisional. It permits adapter work for the provisional selection in X11 and preserves the focused candidate build recipe when its trigger has not occurred. It does not permit Phase 3B, final candidate removal, or a production implementation plan. A selection is final only after all four Tier 1 environments pass under the frozen suite.

## Provisional selection artifact

The next Stage 4 epic must migrate `qualification-evidence.schema.json` so its `environments` object records per-environment eligibility. The object requires results and eligibility only for Tier 1 environments that have entered the declared sequence. It records every later environment with the explicit `not-entered` state and no result record. An entered environment carries the identical frozen suite required by CAP-SUB-001.

The next Stage 4 epic must migrate `selection-decision.schema.json` with a candidate-state enum of `entered`, `untriggered`, and `ineligible`. A record with one `entered` candidate and one `untriggered` candidate is valid. An `untriggered` candidate has no candidate evidence or score. A candidate that entered and failed a hard gate is `ineligible`.

The migrated selection-decision record has `selectionState` of `provisional` or `final`. It is `provisional` after complete evidence for the first entered Tier 1 environment. It changes to `final` only after every Tier 1 environment passes under the same qualification-lock digest. Assessor scores are required only when two candidates are `entered` and eligible.

The current schemas remain unchanged in this milestone because their bytes are digest-bound. The next Stage 4 epic must change `LOCK_SCHEMA`, the `readiness_promotion.rs` final-selection checks, the qualification-evidence and selection-decision schema fixture corpora, and every affected digest in one transaction. The changelog routes the schema, fixture, assertion, and digest landings.

## Consequences

The next Stage 4 epic must land `qualification-lock.schema.json` v6 with `qualificationSequence.candidateOrder`, `qualificationSequence.environmentOrder`, `referenceEnvironments.<environment>.candidateImplementationReady`, and `referenceEnvironments.<environment>.measurementReady`. The v6 schema also retains the checklist's named `measurementPolicy` fields: `sampleValidityRules`, `externalContractLock`, `layoutVisitCorpus`, `layoutQualificationRecordSchema`, `layoutPrequalificationRunSchema`, `layoutPrequalificationSuiteSchema`, `layoutVisitCountingRules`, and `layoutPrequalificationIdentities`.

That epic must change the enforcing `LOCK_SCHEMA`, `candidate_input_issues`, `measurement_input_issues`, and `validate_documents_with_attribution` checks in `xtask/src/contracts/readiness.rs`; the final-selection checks in `xtask/src/contracts/readiness_promotion.rs`; the readiness-policy reporting checks in `crates/oxyflut-qualification/src/readiness.rs`; and the exact readiness assertions in `xtask/src/commands/lock_tests.rs`. It must also migrate the lock instance and its digest-bound fixture corpus in one change.

Shared substrate-neutral crates and the candidate-neutral `oxyflut-substrate` contract crate remain plannable with a null or test substrate and do not depend on an environment readiness flag. Measurement and production promotion remain gated as stated above.
