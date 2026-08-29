# Pre-implementation readiness reconciliation

- Original date: 2026-08-29
- Ticket: OXY-D001
- Status: Stage 4 report complete; Stage 3 reconciliation pending; readiness not set

This Stage 4 report applies no specification edit and cannot set `candidateImplementationReady` or `measurementReady`.

## Purpose and scope

The only in-scope file is `.constitution/reports/pre-implementation-readiness.md`.

The following ticket out-of-scope files and areas remain excluded:

- `.constitution/tech-spec/` (name the required Stage 3 pass; don't perform it in this ticket)
- `.constitution/tasks/` (the next Stage 4 version follows upstream reconciliation)
- Candidate source trees
- Qualification measurements

Any preservation step must re-fetch every source that a spike fetched through the Jina reader proxy from its canonical URL before preservation. Proxied bodies are never fixture bytes.

## Method and verification runs

Every command in this section ran inside `devenv shell` from the repository root and did not modify repository files. Exit code 2 for a lock-status command means valid but open.

`cargo run -q -p xtask -- lock status --gate candidate-implementation` exited 2. Trimmed verbatim excerpt:

```text
lock status: open (candidate-implementation)
blocking: field-path=candidateImplementationReady kind=unresolved upstream-owner=Stage-3-reconciliation
blocking: field-path=measurementPolicy.assessors kind=null evidence-path=qualification/staged/assessors.json upstream-owner=OXY-D001
blocking: field-path=measurementPolicy.capabilityBaseline kind=null upstream-owner=OXY-C002
blocking: field-path=measurementPolicy.externalContractLock kind=null evidence-path=qualification/schemas/external/proposed-external-contract-lock.json referent=proposal upstream-owner=OXY-C001
blocking: field-path=measurementPolicy.fuzzCorpora kind=null evidence-path=qualification/staged/fuzz-corpora.json upstream-owner=OXY-D001
blocking: field-path=measurementPolicy.layoutVisitCap kind=null upstream-owner=OXY-D001
blocking: field-path=measurementPolicy.platformContracts kind=null evidence-path=.constitution/tech-spec/contracts/platform-contracts.json upstream-owner=OXY-C004
blocking: field-path=measurementPolicy.rawMeasurementSchema kind=null evidence-path=.constitution/tech-spec/data-models/raw-measurement.schema.json upstream-owner=OXY-C003
blocking: field-path=measurementPolicy.sampleValidityRules kind=null evidence-path=qualification/schemas/sample-validity.schema.json upstream-owner=OXY-C003
blocking: field-path=measurementPolicy.scoringAnchors kind=null evidence-path=qualification/staged/scoring-anchors.json upstream-owner=OXY-D001
blocking: field-path=measurementPolicy.securityPatchRehearsal kind=null evidence-path=qualification/staged/security-patch-rehearsal.json upstream-owner=OXY-D001
```

The staged paths in that validator excerpt are conventional referents, not committed files: `qualification/staged/{assessors,scoring-anchors}.json` are proposed workload- and assessor-corpus referents for the next Stage 4 epic, and `qualification/staged/{fuzz-corpora,security-patch-rehearsal}.json` are proposed in SPK-B006, not committed.

`cargo run -q -p xtask -- lock status --gate measurement` exited 2. Trimmed verbatim excerpt:

```text
lock status: open (measurement)
remaining-ku: candidate-implementation-ready-not-claimed
remaining-ku: capability-and-platform-baselines
remaining-ku: complete-ime-editing-geometry-and-accessibility-maps
remaining-ku: external-distribution-schema-snapshots-and-verifiers
remaining-ku: final-candidate-source-identity
remaining-ku: fuzz-corpora
remaining-ku: hardware-gpu-driver-and-system-package-locks
remaining-ku: independent-presentation-opportunity-sources
remaining-ku: integrated-fork-commit
remaining-ku: layout-visit-cap
remaining-ku: measurement-ready-not-claimed
remaining-ku: minimum-platform-and-protocol-versions
remaining-ku: oxyflut-adapter-commit
remaining-ku: raw-measurement-and-sample-validity-contracts
remaining-ku: reference-application-scenes-scripts-fonts-assets-windows-cache-and-flags
remaining-ku: resolved-tool-digests
remaining-ku: scoring-anchors-and-two-assessors
remaining-ku: security-patch-rehearsal
```

`cargo run -q -p xtask -- contracts validate` exited 0. Trimmed verbatim excerpt:

```text
schema: ok (.constitution/tech-spec/data-models)
instance: ok (.constitution/tech-spec/contracts)
exact-set: ok (.constitution/tech-spec/contracts/capability-traceability.json)
contract-tests: deferred (52 pending candidate implementation; .constitution/tech-spec/contracts/capability-traceability.json)
accessibility-generation: deferred (schema lacks generation field; .constitution/tech-spec/data-models/accessibility-map.schema.json)
registry: ok (.constitution/tech-spec/contracts/diagnostic-event-registry.json)
digest: ok (.constitution/tech-spec/contracts)
readiness: ok (.constitution/tech-spec/contracts/qualification-lock.json)
promotion: ok (.constitution/tech-spec/contracts/specification-phase.json)
rust-contract: ok (.constitution/tech-spec/contracts)
c-cpp-header: ok (.constitution/tech-spec/contracts/oxyflut-substrate.h)
binding: ok (qualification/fixtures/generated-bindings/oxyflut-substrate.rs)
symbol: ok (.constitution/tech-spec/contracts/oxyflut-substrate.h)
layout: ok (.constitution/tech-spec/contracts/oxyflut-substrate.h)
```

`cargo run -q -p xtask -- external-contracts verify` exited 0. Trimmed verbatim excerpt:

```text
external-contracts: ok (spdx-3.0.1)
external-contracts: ok (in-toto-statement-v1)
external-contracts: ok (slsa-provenance-v1)
external-contracts: ok (dsse-envelope-v1)
```

`cargo run -q -p xtask -- evidence verify qualification/fixtures/evidence/positive-derived.json` exited 0. The command emitted no standard output.

```text

```

`cargo run -q -p xtask -- baseline validate --input qualification/fixtures/baselines/complete.synthetic.json` exited 0. Trimmed verbatim excerpt:

```text
baseline validate: ok
```

`cargo run -q -p xtask -- measurement validate --input qualification/fixtures/measurements/complete.synthetic.json` exited 0. Trimmed verbatim excerpt:

```text
measurement validate: ok
```

`cargo run -q -p xtask -- measurement validate --input qualification/fixtures/sample-validity/complete.synthetic.json` exited 0. Trimmed verbatim excerpt:

```text
measurement validate: ok
```

`environment inspect` was not run: `xtask/src/commands/environment/mod.rs:74-82` requires both `--environment` and `--output`, and `RepositoryPath::parse` makes `--output` a repository-relative path, so no read-only invocation exists. It writes a `PATH.inventory.json` companion artifact into the repository. The recorded Epic C facts and `.constitution/tech-spec/changelog.md:10-14` supply the environment-tooling state instead.

## Consolidated inputs

### Contract-validator results

`contracts validate` passed its schema, instance, traceability, digest, readiness, native-header, binding, symbol, and layout families. Its two deferred notices retain the physical contract-test location and accessibility text-layout generation gaps for Stage 3; `.constitution/tech-spec/changelog.md:24-28` names both revisions.

### Six spike recommendations

- SPK-B001 uses the A/C mix: Table 1 has KK=6 and KU=12, with documented interfaces retained and behavior gates open. [Spec edits required](../spikes/SPK-B001.md#spec-edits-required) requires the semantic-role and ABI reconciliation.
- SPK-B002 assigns options by row: Table 1 has KK=0 and KU=20, retaining Windows source fixtures, host capture, timing, routing, and recovery gates. [Spec edits required](../spikes/SPK-B002.md#spec-edits-required) supplies the five-KU delta.
- SPK-B003 uses the A/B/C mix: Table 1 has KK=8 and KU=13, retaining reference-session and behavior gates. [Spec edits required](../spikes/SPK-B003.md#spec-edits-required) supplies the Wayland additions.
- SPK-B004 uses A plus B plus C by decision area: Table 1 has KK=4 and KU=10, retaining the native-Xorg, Orca-documentation, map, meter, routing, and recovery gates. [Spec edits required](../spikes/SPK-B004.md#spec-edits-required) adds no KU string.
- SPK-B005 uses A for rows 1 through 5 and C for row 6: Table 1 has KK=5 and KU=1, retaining `layout-visit-cap`. [Spec edits required](../spikes/SPK-B005.md#spec-edits-required) defines the v5-to-v6 revision.
- SPK-B006 chooses B: Table 1 has KK=5 and KU=1, retaining the shared-patch applicability gate. [Spec edits required](../spikes/SPK-B006.md#spec-edits-required) replaces two staged-input KUs with one campaign-host KU after admission.

### Hardware-access register

`.constitution/reports/reference-hardware-access.md#answers` confirms the Wayland and X11 rows on `thinkpadp14s`, a non-reference NixOS host. The same register marks macOS arm64 and Windows x86-64 blocked because no owner or access procedure exists, and it records no second-configuration evidence for the Linux rows. `.constitution/reports/reference-hardware-access.md#spec-edits-required` forbids putting the non-reference host's hardware, GPU, driver, or access data in the lock; a `reference: false` campaign-host tool record is exempt because it identifies a toolchain rather than a qualification environment.

### Assessor confirmations

`.constitution/reports/qualification-assessors.md#preserved-confirmations` records Assessor 1's availability and disclosures, including candidate-code or qualification-evidence authorship. `.constitution/reports/qualification-assessors.md#question` leaves Assessor 2 blocked with no named, confirmed distinct person. `.constitution/reports/qualification-assessors.md#spec-edits-required` routes authorship independence to Stage 1 before Stage 3 contract conformance.

### Staged tool manifest

`qualification/tools/native-contract-toolchain.json` records the staged compiler, linker, archiver, symbol inspector, binding generators, formatter, Rust tools, and libc-header utility records. Every record has host triple `x86_64-unknown-linux-gnu`; `.constitution/tech-spec/changelog.md:10-11` retains other Tier 1 hosts as a lock input.

### External-contract manifest and snapshot convention

`qualification/schemas/external/proposed-external-contract-lock.json` contains the nonauthoritative proposed external-lock values, while the active external lock remains unresolved. `qualification/schemas/external/README.md` defines directory-level `source.json` records for upstream identity, license information, and neighbor digest; this convention differs from fixture `<FIXTURE>.source.json` sidecars. It does not adopt the proposal into the lock.

### Baseline tooling

`xtask/src/commands/baseline.rs:22-60` validates one candidate-neutral baseline from `--input`; its `--output` path publishes artifacts and was not used. The synthetic baseline fixture passed the read-only run, but `.constitution/tech-spec/contracts/qualification-lock.json:104-114` retains `measurementPolicy.capabilityBaseline` as `null`.

### Measurement templates

`crates/oxyflut-qualification/src/measurement.rs:535-640` provides library-only `generate_templates` and `compute_comparison_bounds`; no template-generation command surface exists. `.constitution/tech-spec/data-models/raw-measurement.schema.json:1-58` has a top-level `$schema` meta-schema keyword, but its `properties` object declares no `$schema` instance property, and `additionalProperties: false` bars instances from carrying that property. Separately, `discover_contract_instances` walks only `.constitution/tech-spec/contracts/` (`xtask/src/contracts/schema.rs:156-179`). The schema also lacks a per-`(constraintId, launch)` non-decreasing `monotonicNs` rule. The successful synthetic-fixture validation verifies parsing and bindings only; it reports no observation.

### Environment tooling

`.constitution/tech-spec/changelog.md:10-14` records the reproducible shell, the `x86_64-unknown-linux-gnu` staged-tool limit, and the `PATH.inventory.json` companion behavior. The environment command was not run because its output publication is outside this ticket.

### Read-only lock report

`.constitution/tech-spec/contracts/qualification-lock.json:1-149` has `candidateImplementationReady: false`, `measurementReady: false`, all ten `measurementPolicy` keys set to `null`, an empty `resolvedTools` array, and `null` values for each lock-input field family under every `referenceEnvironments` entry. The two preserved lock-status runs report both gates as open and enumerate the corresponding blocks.

### Count reconciliation

The live lock has 13 `preImplementationKnownUnknowns` and 15 `gatingKnownUnknowns` (`.constitution/tech-spec/contracts/qualification-lock.json:117-148`). Applying the [SPK-B002](../spikes/SPK-B002.md#spec-edits-required), [SPK-B003](../spikes/SPK-B003.md#spec-edits-required), [SPK-B004](../spikes/SPK-B004.md#spec-edits-required), [SPK-B005](../spikes/SPK-B005.md#spec-edits-required), and [SPK-B006](../spikes/SPK-B006.md#spec-edits-required) deltas of +5, +11, +0, +1, and -1 yields 29 and 31, respectively. The live `qualification/fixtures/readiness/invalid.json` has 13 pre-implementation and 15 gating entries; `qualification/fixtures/readiness/cleared-without-evidence.json` has 12 and 15. Applying only B005 +1 and B006 -1 leaves those final per-file counts unchanged. The two cleared-fixture exact-set assertions are `xtask/src/commands/lock_tests.rs` `cleared_ku_without_evidence_remains_open_with_the_exact_remaining_ku_set` and `crates/oxyflut-qualification/src/readiness.rs` `clearing_a_ku_string_without_its_evidence_keeps_the_gate_open`; `invalid.json` instead feeds `invalid_referenced_input_fixture_returns_exit_one` and its arrays have no exact-set assertion. The live `xtask/src/contracts/schema.rs:606-614` assertions are `schema_count`/`instance_count` 18/6; SPK-B001 contributes +2/+1 and SPK-B005 contributes +3/+0, giving 23/7. The live `.constitution/tech-spec/contracts/oxyflut-substrate.h:22` header has ABI `10u`; SPK-B001 requires `11u`. The required live command `grep -rl '0\.15\.0' xtask qualification .constitution/tech-spec | wc -l` returns 62, while SPK-B005's 56 count covers only `xtask` and `qualification`.

### Routed Stage 3 and upstream text replacements

- Stage 3 must replace identifier-only `.constitution/tech-spec/data-models/capability-traceability.schema.json` `mappings[].contractTests[]` entries with entries that include the physical contract-test file location.
- Stage 3 must replace `.constitution/tech-spec/data-models/accessibility-map.schema.json` `reverseActions[].textLayoutBinding` with a binding that includes the text-layout generation value.
- Stage 3 must replace generic `.constitution/tech-spec/data-models/specification-phase.schema.json` evidence references for `layoutQualification`, `finalContractSet`, `targetMatrix`, `losingCandidateRemoval`, and `billOfMaterials` with typed-schema references.
- Stage 3 must add the `$schema` property and the per-`(constraintId, launch)` non-decreasing `samples[].monotonicNs` rule to `.constitution/tech-spec/data-models/raw-measurement.schema.json`.
- Stage 3 must type `measurementPolicy.sampleValidityRules`, the external-contract proposal, `PATH.inventory.json`, Wayland interface completeness, and the conventional digest referents for scoring anchors, assessors, fuzz corpora, and security-patch rehearsal.
- Stage 1 must replace the `.constitution/prd/constraints.md` paragraph that begins "The numeric common-case node-visit limit" with: "The numeric common-case node-visit limit for CAP-LAY-001 remains a gating known unknown until the prequalification lock binds candidate and environment identities and the 48-tuple timing probe supplies schema-valid evidence under CON-PERF-001 on unblocked reference hardware."
- Stage 2 must replace `.constitution/architecture/risks.md` `ARC-R02` `Mitigation or follow-up` with: "Record the corpus and Table 4 finite per-policy visit-cap freeze as partially discharged; retain the numeric global layout-visit cap as the remaining gating condition until the prequalification lock binds candidate and environment identities and the 48-tuple timing probe supplies schema-valid evidence under CON-PERF-001 on unblocked reference hardware."
- Stage 1 must add `ordinary visit`, `attempted ordinary visits`, `layout prequalification suite`, `second-configuration score-4 evidence`, `semantic-role registry`, `authorship independence`, `display-epoch equality tuple` (including `targetModeSignature`), and `campaign host` to `.constitution/prd/glossary.md` before Stage 3 adopts them.
- Stage 1 must insert this exact sentence after the independent-scoring sentence in `.constitution/prd/constraints.md`: "A person who authors candidate implementation or qualification evidence for a candidate must not serve as an independent scorer for that candidate."
- `.constitution/tech-spec/changelog.md:223` is a v0.1.0 historical entry. Its statement that the implementation workspace and qualification commands don't exist is not rewritten; historical changelog entries are not rewritten.

The proposed `qualification/staged/` records, the proposed `qualification/fixtures/external-contracts/{macos,wayland,x11,windows,accessibility}/` directories, the proposed `.constitution/tech-spec/contracts/semantic-role-registry.json`, and proposed `OXY_SEMANTICS_ROLE_*` symbols are proposed in SPK-B001, SPK-B002, SPK-B003, SPK-B004, SPK-B005, or SPK-B006, not committed.

## Known-unknown classification

Table 1 classifies every committed pre-implementation KU and both gating-only source-pin KUs. Each cited result is one committed path and each owner is singular. The code's `upstream_owner` values still name archived Epic A, C, and D tickets, while this report routes its actions to live owners. The superseded 22-of-54 comparison counted 22 `OXY-D001` occurrences in `crates/oxyflut-qualification/src/readiness.rs` and 54 total lock-status output lines; the preserved candidate-gate run instead has 18 of 53 `blocking:` lines ending `upstream-owner=OXY-D001`. T5.2a routes artifact creation and binding to Stage 3, workload and scoring-anchor/assessor corpus definition to the next Stage 4 epic, and blocked inputs to their named external reference hosts or Assessor 2.

| Known unknown | Binding field | Cited result | Status | Owner | Next action |
| :-- | :-- | :-- | :-- | :-- | :-- |
| `minimum-platform-and-protocol-versions` | `referenceEnvironments` | `.constitution/tech-spec/contracts/qualification-lock.json:60-91` retains every `minimumVersion` as `null`. | blocked external input | macOS arm64 and Windows x86-64 reference hosts | Obtain the missing macOS, Windows, and Ubuntu reference-host captures, then apply the Stage 3 platform revisions. |
| `hardware-gpu-driver-and-system-package-locks` | `referenceEnvironments` | `.constitution/reports/reference-hardware-access.md#reference-conformance-and-feasibility` records non-reference Linux access and blocked macOS and Windows rows. | blocked external input | macOS arm64 and Windows x86-64 reference hosts | Obtain accountable reference-host access and capture hardware, GPU, driver, and package-lock identities. |
| `reference-application-scenes-scripts-fonts-assets-windows-cache-and-flags` | `workload` | `.constitution/tech-spec/contracts/qualification-lock.json:94-102` sets every workload field to `null`. | retained KU | next Stage 4 epic | Define the approved workload corpus; Stage 3 types its lock binding. |
| `raw-measurement-and-sample-validity-contracts` | `measurementPolicy.sampleValidityRules` | `.constitution/tech-spec/data-models/raw-measurement.schema.json:1-58` has a top-level `$schema` meta-schema keyword, but no `$schema` instance property under `properties`; `additionalProperties: false` bars self-declaration, and the monotonic-time schema rule is absent. | retained KU | Stage 3 | Add the raw-measurement revisions and type the staged sample-validity referent. |
| `capability-and-platform-baselines` | `measurementPolicy.capabilityBaseline` | `.constitution/tech-spec/contracts/qualification-lock.json:104-114` keeps `capabilityBaseline` and `platformContracts` `null`. | retained KU | Stage 3 | Bind approved capability and platform baseline artifacts with typed references. |
| `independent-presentation-opportunity-sources` | `measurementPolicy.platformContracts` | `.constitution/spikes/SPK-B003.md#spec-edits-required` retains the Wayland independent-meter capture requirements. | blocked external input | Wayland x86-64 reference host | Capture reference-session meter evidence and apply the Stage 3 platform-contract revision. |
| `complete-ime-editing-geometry-and-accessibility-maps` | `measurementPolicy.platformContracts` | `.constitution/reports/reference-hardware-access.md#answers` records blocked macOS and Windows host access. | blocked external input | macOS arm64 and Windows x86-64 reference hosts | Obtain the blocked host access and capture the required IME and accessibility artifacts. |
| `scoring-anchors-and-two-assessors` | `measurementPolicy.scoringAnchors` | `.constitution/reports/qualification-assessors.md#question` leaves Q2 through Q4 gating and Assessor 2 unnamed. | blocked external input | Assessor 2 | Obtain the second assessor confirmation and Stage 1 authorship-independence approval. |
| `fuzz-corpora` | `measurementPolicy.fuzzCorpora` | `.constitution/tech-spec/contracts/qualification-lock.json:104-114` sets `fuzzCorpora` to `null`. | retained KU | Stage 3 | Create and bind the `fuzz-corpora` record proposed in SPK-B006, not committed. |
| `security-patch-rehearsal` | `measurementPolicy.securityPatchRehearsal` | `.constitution/tech-spec/contracts/qualification-lock.json:104-114` sets `securityPatchRehearsal` to `null`. | retained KU | Stage 3 | Create and bind the security-patch record proposed in SPK-B006, not committed. |
| `layout-visit-cap` | `measurementPolicy.layoutVisitCap` | `.constitution/spikes/SPK-B005.md#question` retains row 6 as a gating KU. | blocked external input | macOS arm64 and Windows x86-64 reference hosts | Obtain the blocked reference hosts before the Stage 3 layout-lock revision can receive capture input. |
| `external-distribution-schema-snapshots-and-verifiers` | `measurementPolicy.externalContractLock` | `.constitution/tech-spec/contracts/qualification-lock.json:104-114` sets `externalContractLock` to `null`. | retained KU | Stage 3 | Adopt or replace the proposed external-contract lock through a typed Stage 3 binding. |
| `resolved-tool-digests` | `resolvedTools` | `.constitution/tech-spec/contracts/qualification-lock.json:116` keeps `resolvedTools` empty. | retained KU | Stage 3 | Bind an authoritative resolved-tool lock that conforms to the staged manifest. |
| `integrated-fork-commit` (gating-only) | `sourcePins.integratedFork.commit` | `.constitution/tech-spec/contracts/qualification-lock.json:20` sets the commit to `null`. | retained KU | Stage 3 | Record a verified immutable integrated-fork commit in the Stage 3 lock revision. |
| `oxyflut-adapter-commit` (gating-only) | `sourcePins.oxyflutAdapter.commit` | `.constitution/tech-spec/contracts/qualification-lock.json:21` sets the commit to `null`. | retained KU | Stage 3 | Record a verified immutable Oxyflut-adapter commit in the Stage 3 lock revision. |

## OXY-D001 decisions

Table 2 records repository-supported decisions and bounded deferrals. The report routes every implementation task to Stage 3 or an external input.

| Decision | Basis | Outcome | Stage 3 instruction | Owner |
| :-- | :-- | :-- | :-- | :-- |
| External-fixture `<FIXTURE>.source.json` sidecar validator ownership | No pass in `xtask/src/commands/external_contracts.rs` `verify_at` walks the proposed platform fixture subtrees; `verify_fixtures` covers only the fixed `positive/` set. | Adopt a new `external-contracts verify` sidecar-validation pass. | Add a sidecar-validation pass that requires and validates a `<FIXTURE>.source.json` sidecar, such as `s03-nsview-display-link.html.source.json`, for every regular fixture in the proposed platform subtrees; this fixture-sidecar convention differs from directory-level `source.json` records under `qualification/schemas/external/`. Require a regular sibling, equal sibling SHA-256, canonical retrieval URL, upstream-relative `path` and `licenseSource.path`, and required license fields. | Stage 3 |
| GTK and AT-SPI row alignment across Wayland and X11 | `SPK-B003.md#spec-edits-required` and `SPK-B004.md#spec-edits-required` propose the shared Ubuntu package identities. | Align both rows on `libgtk-4-1` `4.22.2+ds-1ubuntu1` and `at-spi2-core` `2.60.0-1`; keep the GTK API ceiling and AT-SPI XML source floors distinct. | Apply identical package identities to both platform rows and retain each environment's distinct source-floor evidence. | Stage 3 |
| `issuingFamily` corpus field and cap-1 rejection fixture | `SPK-B005.md#counting-rules-interpretation-and-stage-3-validator-requirements` requires `issuingFamily` for nonordinary families. | Add both before the v6 lock binds the SPK-B005 digests; re-freeze corpus, counting rules, model source, and changelog blocks. | Require `issuingFamily` for nonordinary fixtures, reject absent or unknown values, and add a cap-1 rejection fixture. | Stage 3 |
| Baseline-validation owner assignment | `xtask/src/commands/baseline.rs:22-60` validates one baseline artifact and its authority. | Assign schema and typing to Stage 3; assign workload and scoring-anchor corpus ownership to the next Stage 4 epic. | Type baseline references and validation inputs; the next Stage 4 epic defines the workload and scoring-anchor corpus. | next Stage 4 epic |
| Windows 77 excerpt fixtures | `SPK-B002.md#spec-edits-required` records excerpt fixtures because no Windows host supplies canonical bytes. | Retain excerpts while no Windows host exists; convert to canonical bytes plus sidecars after canonical host access exists. | Stage 3 defines the conversion trigger and sidecar validation rule after canonical-host access is supplied. | Windows x86-64 reference host |
| Migration-fixture mechanism | `xtask/src/contracts/schema.rs:505-540` validates one fixed `migration/{source.json,source.sha256,derived.json}` triple. | Generalize the helper to named per-migration input and derived-output pairs. | Add a per-migration registry that validates each named pair from the proposed SPK-B001 and SPK-B005 migrations, not committed. | Stage 3 |
| 256 KiB `wayland-info` and `xdpyinfo` capture bound | `.constitution/reports/reference-hardware-access.md#reference-conformance-and-feasibility` confirms only a non-reference NixOS host. | Defer and retain fail-closed truncation handling. | Add the bound only after real Ubuntu 26.04 capture sizes are available. | Ubuntu 26.04 reference host |
| Other Tier 1 hosts as a lock input | `.constitution/tech-spec/changelog.md:10-11` limits the staged toolchain to `x86_64-unknown-linux-gnu`. | Defer pending blocked hardware inputs. | Add host records only after accountable Tier 1 host capture exists. | next Stage 4 epic |
| Wayland interface-set completeness rule | `.constitution/tech-spec/changelog.md:32` identifies the absent completeness rule and partial `protocolVersion` representation. | Defer completeness determination pending a reference-session capture. | Add an `interfaceSetCompleteness` schema rule that requires the captured interface set and rejects absent or partial observed `protocolVersion`. | Stage 3 |
| Offline advisory database and refresh policy | `.constitution/tasks/completed/EPIC-D-readiness-reconciliation.md:42` records no pinned vendored advisory database. | Defer advisory validation. | Bind a pinned offline RustSec advisory database, its digest, refresh authority, cadence, and CI location. | Stage 3 |

The proposed SPK-B001 and SPK-B005 migration filenames use `<name>-v5-to-v6.input.json` and `<name>-v5-to-v6.expected.json`; they are proposed in SPK-B001 and SPK-B005, not committed.

## Missing approved or captured lock inputs

Table 3 identifies every null policy value, empty resolved-tool array, null source-pin commit, null reference-environment field family, and false readiness flag. The entries name inputs only and do not change a flag.

| Lock field | Current value | What fills it (exact approval or capture) | Owner |
| :-- | :-- | :-- | :-- |
| `measurementPolicy.rawMeasurementSchema` | `null` | Stage 3 approval of the revised raw-measurement schema with `$schema` and the non-decreasing per-`(constraintId, launch)` rule. | Stage 3 |
| `measurementPolicy.sampleValidityRules` | `null` | Stage 3 adoption of the committed staged sample-validity proposal with a typed lock referent and digest. | Stage 3 |
| `measurementPolicy.capabilityBaseline` | `null` | An approved 52-capability baseline with its approval-evidence digest and typed lock reference. | Stage 3 |
| `measurementPolicy.platformContracts` | `null` | A typed digest binding for the revised platform-contract baseline and its captured evidence. | Stage 3 |
| `measurementPolicy.scoringAnchors` | `null` | One immutable scoring-anchor corpus accepted by two confirmed assessors. | next Stage 4 epic |
| `measurementPolicy.assessors` | `null` | One immutable two-assessor corpus declaration that satisfies the Stage 1-approved authorship rule. | next Stage 4 epic |
| `measurementPolicy.fuzzCorpora` | `null` | Stage 3 creation and binding of the `fuzz-corpora` record proposed in SPK-B006, not committed. | Stage 3 |
| `measurementPolicy.securityPatchRehearsal` | `null` | Stage 3 creation and binding of the security-patch record proposed in SPK-B006, not committed. | Stage 3 |
| `measurementPolicy.externalContractLock` | `null` | Stage 3 adoption or replacement of `qualification/schemas/external/proposed-external-contract-lock.json`. | Stage 3 |
| `measurementPolicy.layoutVisitCap` | `null` | Schema-valid reference-host capture from the bounded layout probe and the approved cap decision. | macOS arm64 and Windows x86-64 reference hosts |
| `sourcePins.integratedFork.commit` | `null` | A verified immutable integrated-fork commit captured for the lock. | Stage 3 |
| `sourcePins.oxyflutAdapter.commit` | `null` | A verified immutable Oxyflut-adapter commit captured for the lock. | Stage 3 |
| `workload.referenceApplication` | `null` | The approved reference-application identity in the typed workload corpus. | next Stage 4 epic |
| `workload.scenes` | `null` | The approved reference scene set in the typed workload corpus. | next Stage 4 epic |
| `workload.interactionScripts` | `null` | The approved interaction-script set in the typed workload corpus. | next Stage 4 epic |
| `workload.fonts` | `null` | The captured font identities and immutable bytes for the approved workload. | next Stage 4 epic |
| `workload.assets` | `null` | The captured asset identities and immutable bytes for the approved workload. | next Stage 4 epic |
| `workload.windowMatrix` | `null` | The approved window matrix for the workload. | next Stage 4 epic |
| `workload.cacheStates` | `null` | The approved cache-state matrix for the workload. | next Stage 4 epic |
| `workload.releaseFlags` | `null` | The approved release-flag set for the workload. | next Stage 4 epic |
| `resolvedTools` | `[]` | An authoritative resolved-tool lock that matches `qualification/tools/native-contract-toolchain.json`. | Stage 3 |
| `referenceEnvironments.*.minimumVersion` | `null` | Captures that establish every reference OS and protocol minimum. | macOS arm64 and Windows x86-64 reference hosts |
| `referenceEnvironments.*.hardwareId` | `null` | Accountable reference-host hardware identity captures for every Tier 1 environment. | macOS arm64 and Windows x86-64 reference hosts |
| `referenceEnvironments.*.gpuId` | `null` | Accountable reference-host GPU identity captures for every Tier 1 environment. | macOS arm64 and Windows x86-64 reference hosts |
| `referenceEnvironments.*.driverVersion` | `null` | Accountable reference-host driver-version captures for every Tier 1 environment. | macOS arm64 and Windows x86-64 reference hosts |
| `referenceEnvironments.*.systemPackageLockDigest` | `null` | Signed or immutable package-lock captures for every reference environment. | macOS arm64 and Windows x86-64 reference hosts |
| `candidateImplementationReady` | `false` | Complete approved and captured inputs for every pre-implementation lock field and no pre-implementation KU. | Stage 3 |
| `measurementReady` | `false` | Complete approved and captured inputs for every measurement lock field and no gating KU. | Stage 3 |

## Stage 3 reconciliation checklist

This checklist orders prerequisites before dependents. Each row routes an existing source instruction; its anchor holds the normative text.

### T0 upstream prerequisites

Order rule: Stage 1 and Stage 2 approvals precede every Stage 3 change that uses their terms or replacement text.

| # | File | Field, section, or symbol | Enforcing check | Source anchor | Owner |
| :-- | :-- | :-- | :-- | :-- | :-- |
| T0.1 | `.constitution/prd/glossary.md` | Add eight terms: `ordinary visit`, `attempted ordinary visits`, `layout prequalification suite`, `second-configuration score-4 evidence`, `semantic-role registry`, `authorship independence`, `display-epoch equality tuple` including `targetModeSignature`, and `campaign host`. | `xtask/src/contracts/traceability/mod.rs` exact PRD sets. | [Epic D glossary inputs](../tasks/completed/EPIC-D-readiness-reconciliation.md#oxy-d001-inputs-from-epic-b) | Stage 1 |
| T0.2 | `.constitution/prd/constraints.md` | Replace the paragraph beginning `The numeric common-case node-visit limit` with the exact section 3 replacement. | `prd_constraints` and `EXPECTED_CONSTRAINTS`. | [SPK-B005, Spec edits required](../spikes/SPK-B005.md#spec-edits-required) | Stage 1 |
| T0.3 | `.constitution/prd/constraints.md` | Insert the exact authorship-independence sentence in section 3. | `$defs.score.properties.assessorScores` and `LOCK_SCHEMA`. | [OXY-B008, Spec edits required](qualification-assessors.md#spec-edits-required) | Stage 1 |
| T0.4 | `.constitution/architecture/risks.md` | Replace `ARC-R02` `Mitigation or follow-up` with the exact section 3 replacement. | `xtask/src/contracts/traceability/mod.rs` architecture authority. | [SPK-B005, Spec edits required](../spikes/SPK-B005.md#spec-edits-required) | Stage 2 |

### T0a independently actionable readiness-owner correction

Order rule: T5.2a depends on no T1-T4 work and can proceed independently.

| # | File | Field, section, or symbol | Enforcing check | Source anchor | Owner |
| :-- | :-- | :-- | :-- | :-- | :-- |
| T5.2a | `crates/oxyflut-qualification/src/readiness.rs` and `xtask/src/commands/lock_tests.rs` | Reassign `POLICY_FIELDS` and `KNOWN_UNKNOWN_BINDINGS` `upstream_owner` values: Stage 3 for artifact creation and binding, the next Stage 4 epic for workload and scoring-anchor/assessor corpus definition, and named external reference hosts or Assessor 2 for blocked inputs. Update all six reassigned hard-coded owner literals while keeping the enforcing-check names unchanged: `xtask/src/commands/lock_tests.rs:309` `upstream-owner=OXY-C001`, `:312` `upstream-owner=OXY-C002,OXY-C004`, and `:313-315` three `upstream-owner=OXY-D001` lines in `candidate_report_lines_are_stable_and_content_free`; and `crates/oxyflut-qualification/src/readiness.rs:889` `OXY-C003` in `staged_input_registry_binds_every_pathless_measurement_policy_digest`. Leave the `OXY-A008` literals at `xtask/src/commands/lock_tests.rs:306` and `:339` untouched because those assertions preserve the current emitted report. | `POLICY_FIELDS`, `KNOWN_UNKNOWN_BINDINGS`, `candidate_report_lines_are_stable_and_content_free`, and `staged_input_registry_binds_every_pathless_measurement_policy_digest`. | `crates/oxyflut-qualification/src/readiness.rs:45-215` and [SPK-B006, Spec edits required](../spikes/SPK-B006.md#spec-edits-required) | Stage 3 |

### T1 schema creation and migration

Order rule: Create or migrate schemas before their fixture corpora, instances, and assertions.

| # | File | Field, section, or symbol | Enforcing check | Source anchor | Owner |
| :-- | :-- | :-- | :-- | :-- | :-- |
| T1.1 | `.constitution/tech-spec/data-models/{capability-traceability,specification-phase,raw-measurement}.schema.json` | Add physical contract-test locations, five typed promotion-evidence references, raw `$schema`, and non-decreasing per-`(constraintId, launch)` `monotonicNs`. | `discover_contract_instances` `$schema` and traceability contract-test resolution. | `.constitution/tech-spec/changelog.md:24-28` | Stage 3 |
| T1.2 | `.constitution/tech-spec/data-models/{semantic-role-registry,semantic-role-registry-snapshot}.schema.json` (proposed in SPK-B001, not committed) | Create the D0 registry and snapshot schemas. | `run_fixture_corpus` directory-set equality. | [SPK-B001, Spec edits required](../spikes/SPK-B001.md#spec-edits-required) | Stage 3 |
| T1.3 | `.constitution/tech-spec/data-models/accessibility-map.schema.json` | Migrate v5 to v6 keyed `forward.roles`, registry provenance, and text-layout generation. | `ACCESSIBILITY_MAP_SCHEMA` and accessibility-map validation. | [SPK-B001, Accessibility-map version-6 landing inventory](../spikes/SPK-B001.md#accessibility-map-version-6-landing-inventory) | Stage 3 |
| T1.4 | `.constitution/tech-spec/data-models/qualification-lock.schema.json` | Migrate v5 to v6; type sample-validity, external-lock, inventory, conventional staged inputs, `layoutVisitCorpus`, `layoutQualificationRecordSchema`, `layoutPrequalificationRunSchema`, `layoutPrequalificationSuiteSchema`, `layoutVisitCountingRules`, and `layoutPrequalificationIdentities`. | `LOCK_SCHEMA` and claimed-ready policy validation. | [SPK-B005, Counting-rules interpretation](../spikes/SPK-B005.md#counting-rules-interpretation-and-stage-3-validator-requirements) | Stage 3 |
| T1.5 | `.constitution/tech-spec/data-models/{layout-qualification-record,layout-prequalification-run,layout-prequalification-suite}.schema.json` (proposed in SPK-B005, not committed) | Create the three layout schemas. | `schema_compiles_committed_contract_instances_and_fixture_corpus`. | [SPK-B005, Layout prequalification additions inventory](../spikes/SPK-B005.md#layout-prequalification-additions-inventory) | Stage 3 |
| T1.6 | `PATH.inventory.json` (proposed conventional referent from Epic C, not committed) | Type the environment inventory, Wayland interface-set completeness, and partial observed `protocolVersion`. | `POLICY_FIELDS` and `LOCK_SCHEMA`. | `.constitution/tech-spec/changelog.md:29-33` | Stage 3 |

### T2 schema-fixture corpora

Order rule: Land a schema and its corpus in one change because `run_fixture_corpus` requires equal schema and fixture-directory sets.

| # | File | Field, section, or symbol | Enforcing check | Source anchor | Owner |
| :-- | :-- | :-- | :-- | :-- | :-- |
| T2.1 | `qualification/fixtures/contracts/{semantic-role-registry,semantic-role-registry-snapshot}/` (proposed in SPK-B001, not committed) | Add valid and five invalid inputs with expected sidecars for each new schema. | `run_fixture_corpus`. | [SPK-B001, Spec edits required](../spikes/SPK-B001.md#spec-edits-required) | Stage 3 |
| T2.2 | `qualification/fixtures/contracts/accessibility-map/` and `migration/accessibility-map-v5-to-v6.{input,expected}.json` (migration pair proposed in SPK-B001, not committed) | Migrate the keyed-role, supersession, and traceability fixture corpus. | `run_fixture_corpus`, `discover_contract_instances` `$schema`, and `validate_migration_fixture`. | [SPK-B001, Accessibility-map version-6 landing inventory](../spikes/SPK-B001.md#accessibility-map-version-6-landing-inventory) | Stage 3 |
| T2.3 | `qualification/fixtures/contracts/{layout-qualification-record,layout-prequalification-run,layout-prequalification-suite}/` (proposed in SPK-B005, not committed) | Add all schema-valid and schema-invalid corpora with expected sidecars. | `run_fixture_corpus`. | [SPK-B005, Layout prequalification additions inventory](../spikes/SPK-B005.md#layout-prequalification-additions-inventory) | Stage 3 |
| T2.4 | `qualification/fixtures/contracts/qualification-lock/`, `qualification/fixtures/contracts/supersession.json`, and `qualification/fixtures/contracts/migration/qualification-lock-v5-to-v6.{input,expected}.json` (pair proposed in SPK-B005, not committed), plus the 13 lock-bearing readiness fixtures | Migrate fixtures to v6, add the six required layout fields, preserve the v5 migration input bytes, and retain false readiness flags and the two layout KUs where SPK-B005 specifies them. | `LOCK_SCHEMA`, `run_fixture_corpus`, and `validate_migration_fixture`. | [SPK-B005, Spec edits required](../spikes/SPK-B005.md#spec-edits-required) | Stage 3 |
| T2.5 | `xtask/src/contracts/schema.rs` | `is_non_schema_fixture_directory`: add `layout-prequalification` before its proposed non-schema corpus lands, so directory-set equality excludes only that custom-validator corpus. | `schema_compiles_committed_contract_instances_and_fixture_corpus`. | [SPK-B005, Spec edits required](../spikes/SPK-B005.md#spec-edits-required) | Stage 3 |

### T2.6 external-fixture preservation and sidecar validation

Order rule: Add the sidecar-validation pass and preserve canonical bytes before T3.2 records platform evidence. `SNAPSHOTS` and `require_license_fields` remain external-schema-snapshot checks and don't enforce external-fixture sidecars.

| # | File | Field, section, or symbol | Enforcing check | Source anchor | Owner |
| :-- | :-- | :-- | :-- | :-- | :-- |
| T2.6.1 | `xtask/src/commands/external_contracts.rs` | `run` and a new external-fixture-sidecar validation pass: enumerate every regular fixture in the proposed T2.6.2 `macos/`, `wayland/`, and `x11/` subtrees; require a `<FIXTURE>.source.json` sidecar, distinct from the directory-level `source.json` convention; then validate regular sibling bytes, equal SHA-256, canonical retrieval URL, upstream-relative `path` and `licenseSource.path`, and required license fields. Exempt `positive/`, `negative/`, `test-key.json`, and the recorded T2.6.3 `windows/` 77-excerpt exemption. In the same pass, compare each committed subtree's regular-fixture set with the expected set: SPK-B001's cited canonical URL list for macOS (`.constitution/spikes/SPK-B001.md:492-499`), SPK-B003 Table 4 for the 11 Wayland fixtures (`.constitution/spikes/SPK-B003.md:1382-1396`), and SPK-B004 Table 3 for the 15 X11 fixtures (`.constitution/spikes/SPK-B004.md:640-658`); fail on a missing or extra fixture. The pass checks sidecar coverage and these exact fixture sets without inferring a macOS cardinality. | `cargo run -q -p xtask -- external-contracts verify` with the new sidecar-validation pass. | [Epic D external-fixture input](../tasks/completed/EPIC-D-readiness-reconciliation.md#oxy-d001-inputs-from-epic-b); `.constitution/spikes/SPK-B001.md:492-499`; `.constitution/spikes/SPK-B003.md:1382-1396`; `.constitution/spikes/SPK-B004.md:640-658` | Stage 3 |
| T2.6.2 | `qualification/fixtures/external-contracts/{macos,wayland,x11}/` (proposed in SPK-B001, SPK-B003, and SPK-B004, not committed) | Re-fetch the macOS canonical-URL-list fixtures, the 11 Wayland fixtures, and the 15 X11 fixtures as regular files with `<FIXTURE>.source.json` sidecars, distinct from directory-level `source.json` records. | `cargo run -q -p xtask -- external-contracts verify` with the new sidecar-validation pass. | `.constitution/spikes/SPK-B001.md:492-499`; `.constitution/spikes/SPK-B003.md:1382-1396`; `.constitution/spikes/SPK-B004.md:640-658` | Stage 3 |
| T2.6.3 | `qualification/fixtures/external-contracts/windows/` (proposed in SPK-B002, not committed) | Retain the 77-excerpt exemption and record the Windows-host canonical-capture decision. | SPK-B002 `source-fixture capture procedure`. | [SPK-B002, Spec edits required](../spikes/SPK-B002.md#spec-edits-required) | Windows x86-64 reference host |

### T3 contract instances

Order rule: Populate instances after T1 schemas and T2 fixture shapes are present.

| # | File | Field, section, or symbol | Enforcing check | Source anchor | Owner |
| :-- | :-- | :-- | :-- | :-- | :-- |
| T3.1 | `.constitution/tech-spec/contracts/semantic-role-registry.json` (proposed in SPK-B001, not committed) and `.constitution/tech-spec/contracts/capability-traceability.json` | Create the self-declared registry; add CAP-SEM physical, generated-symbol, and registry-pointer bindings, plus contract-test locations. | `discover_contract_instances` `$schema` and `validate_required_symbol_edges`. | [SPK-B001, Spec edits required](../spikes/SPK-B001.md#spec-edits-required) | Stage 3 |
| T3.1a | `.constitution/tech-spec/contracts/{oxyflut-public.rs,oxyflut-substrate.rs,oxyflut-substrate.h}` | Generate `SemanticRole` and `OXY_SEMANTICS_ROLE_*` definitions from `semantic-role-registry.json`; the definitions are proposed in SPK-B001, not committed; add the generated-role contract test that compares every registry `name` and `code` with all three artifacts. | The generated-role contract test and `validate_required_symbol_edges`. | [SPK-B001, Spec edits required](../spikes/SPK-B001.md#spec-edits-required) | Stage 3 |
| T3.2 | `.constitution/tech-spec/contracts/platform-contracts.json` and `.constitution/tech-spec/stack.md` | After T2.6.1-T2.6.2, apply macOS retentions, Windows ten edits, Wayland replacement, X11 nine edits, aligned GTK and AT-SPI rows, and the retained Orca gate. | `validate_platform_baseline`. | [SPK-B002](../spikes/SPK-B002.md#spec-edits-required), [SPK-B003](../spikes/SPK-B003.md#spec-edits-required), and [SPK-B004](../spikes/SPK-B004.md#spec-edits-required) | Stage 3 |
| T3.3 | `.constitution/tech-spec/contracts/qualification-lock.json` | Apply typed raw, sample-validity, external-contract, environment-inventory, platform-baseline, and layout references; retain every null field required by its source. | `candidate_implementation_report` and `LOCK_SCHEMA`. | [Epic D inputs from Epics A and C](../tasks/completed/EPIC-D-readiness-reconciliation.md#oxy-d001-inputs-from-epics-a-and-c) | Stage 3 |
| T3.3a | `crates/oxyflut-qualification/src/readiness.rs` | `POLICY_FIELDS`, `StagedInputRegistry`, and `KNOWN_UNKNOWN_BINDINGS`: add five path-bound layout fields and the path-less `layoutPrequalificationIdentities` field and KU binding. | `StagedInputRegistry::candidate_status_input_bindings`, `collect_measurement_policy`, and `collect_known_unknowns`. | [SPK-B005, Spec edits required](../spikes/SPK-B005.md#spec-edits-required) | Stage 3 |
| T3.3b | `xtask/src/contracts/readiness.rs` | `LOCK_SCHEMA`, `candidate_input_issues`, and the claimed-ready policy checks: validate the v6 lock and its layout fields and identity matrix. | `validate_workspace` readiness family. | [SPK-B005, Spec edits required](../spikes/SPK-B005.md#spec-edits-required) | Stage 3 |
| T3.3c | `xtask/src/commands/environment/mod.rs` | `LOCK_SCHEMA` and `validate_lock_environment_projection`: validate projections against the v6 lock identity without running `environment inspect` in this ticket. | `validate_lock_environment_projection` unit coverage. | [SPK-B005, Spec edits required](../spikes/SPK-B005.md#spec-edits-required) | Stage 3 |
| T3.3d | `xtask/src/commands/lock.rs` | `validate_staged_candidate_inputs` and `StagedInputRegistry::candidate_status_input_bindings`: verify the five path-bound digests and emit the identity-field block. | `lock status --gate candidate-implementation` open-report and missing-input assertions. | [SPK-B005, Spec edits required](../spikes/SPK-B005.md#spec-edits-required) | Stage 3 |
| T3.3e | `.constitution/tech-spec/data-models/qualification-lock.schema.json`, `.constitution/tech-spec/contracts/qualification-lock.json`, and `xtask/src/toolchain/lock.rs` | `$defs.tool.properties` and `resolvedTools`: retain no `pathRoot` field and keep campaign-host records out of `resolvedTools`; bind them only through `measurementPolicy.fuzzCorpora`. | `verify_lock_resolved_tools_classified` and `POLICY_FIELDS`. | [SPK-B006, Spec edits required](../spikes/SPK-B006.md#spec-edits-required) | Stage 3 |
| T3.4 | `.constitution/tech-spec/contracts/{oxyflut-public.rs,oxyflut-qualification.rs}` and `.constitution/tech-spec/adrs/ADR-0005-platform-hosts.md` | Add `LayoutResult.attempted_ordinary_visits`, `LayoutTransactionCounters`, and `CandidateProbe::run_layout_fixture`; append the Windows `Decision` and Wayland `Consequences` text. | Rust-contract compilation assertion and `validate_workspace`. | [SPK-B005](../spikes/SPK-B005.md#spec-edits-required), [SPK-B002](../spikes/SPK-B002.md#spec-edits-required), and [SPK-B003](../spikes/SPK-B003.md#spec-edits-required) | Stage 3 |
| T3.4a | `xtask/src/commands/contracts.rs` | `validate_rust_contracts`: compile an external-client assertion that constructs `LayoutResult` with `attempted_ordinary_visits` and type-checks `CandidateProbe::run_layout_fixture`. | `contracts validate` `rust-contract` family. | [SPK-B005, Spec edits required](../spikes/SPK-B005.md#spec-edits-required) | Stage 3 |
| T3.4b | `xtask/src/commands/layout_prequalification.rs` and `qualification/fixtures/layout-prequalification/` (proposed in SPK-B005, not committed) | Create the command and its non-schema valid and invalid corpus; apply raw-byte digest binding, corpus-derived counters and outcomes, contiguous transaction validation, recomputed frame and percentile values, and complete 48-tuple validation. | `layout-prequalification validate` custom-validator fixture corpus. | [SPK-B005, Spec edits required](../spikes/SPK-B005.md#spec-edits-required) | Stage 3 |
| T3.4c | `.constitution/tech-spec/stack.md` | `Scope guard` paragraph beginning `The current qualification lock`: append exactly `Before candidateImplementationReady becomes true, Stage 4 may run unscored nonproduction candidate probes only to resolve a pre-implementation gating KU; each probe must use the frozen evidence contract and can't produce comparative scores or select a candidate.` | `.constitution/tech-spec/stack.md` Scope guard sentence review and `prettier --prose-wrap never --check '.constitution/**/*.md'`. | [SPK-B005, Spec edits required](../spikes/SPK-B005.md#spec-edits-required) | Stage 3; Applied in Stage 3 v0.16.0 |
| T3.4d | `.constitution/tech-spec/data-models/README.md` | `Durable qualification data` table: add the three layout-schema rows and the two semantic-role-registry rows with their exact compatibility rules; advance the accessibility-map supersession summary to v6 and state the keyed-role provenance migration. | Schema inventory review and `contracts validate` schema family. | [SPK-B001, Accessibility-map version-6 landing inventory](../spikes/SPK-B001.md#accessibility-map-version-6-landing-inventory) and [SPK-B005, Spec edits required](../spikes/SPK-B005.md#spec-edits-required) | Stage 3 |
| T3.4e | `.constitution/tech-spec/data-models/README.md` | Replace `No durable qualification instance was produced under the superseded identities because the implementation workspace and qualification commands don't exist. Git history preserves the earlier schema bytes. OXY-A002 must include old-reader rejection and explicit supersession fixtures; no evidence migration is required until an instance exists.` with exactly `No durable qualification instance was produced under the superseded identities. Git history preserves the earlier schema bytes. Stage 3 must include old-reader rejection and explicit supersession fixtures; no evidence migration is required until an instance exists.` | `.constitution/tech-spec/data-models/README.md` sentence review and `prettier --prose-wrap never --check '.constitution/**/*.md'`. | `.constitution/tech-spec/data-models/README.md:34` | Stage 3; Applied in Stage 3 v0.16.0 |
| T3.5 | `.constitution/tech-spec/changelog.md` and CI advisory configuration | Add the accessibility-map and qualification-lock migration notes; bind a pinned offline advisory database and refresh policy; assign remaining baseline owners; retain library-only template APIs pending their contract. | `baseline validate`, `measurement validate`, `generate_templates`, and `digests::validate_workspace`. | [Epic D inputs from Epics A and C](../tasks/completed/EPIC-D-readiness-reconciliation.md#oxy-d001-inputs-from-epics-a-and-c) and [SPK-B001, Spec edits required](../spikes/SPK-B001.md#spec-edits-required) | Stage 3 |
| T3.5a | `.constitution/tech-spec/guidelines.md` | Replace the sentence "The staged native toolchain supports only `x86_64-unknown-linux-gnu`; other Tier 1 hosts are an OXY-D001 lock input." with exactly "The staged native toolchain supports only `x86_64-unknown-linux-gnu`; other Tier 1 hosts are a decided-deferred lock input owned by the next Stage 4 epic, superseding the archived hardware-ticket recommendation." | `.constitution/tech-spec/guidelines.md` sentence review and `prettier --prose-wrap never --check '.constitution/**/*.md'`. | `.constitution/tech-spec/guidelines.md:115`; [OXY-B007 hardware register](reference-hardware-access.md#reference-conformance-and-feasibility) | next Stage 4 epic; Applied in Stage 3 v0.16.0 |
| T3.5b | `.constitution/tech-spec/guidelines.md` | Replace `cargo +1.98.0 deny check licenses bans sources` advisories wording `advisories deferred to OXY-D001.` with exactly `advisory validation is blocked until Stage 3 binds a pinned offline RustSec advisory database and refresh policy.` | `.constitution/tech-spec/guidelines.md` sentence review and `prettier --prose-wrap never --check '.constitution/**/*.md'`. | `.constitution/tech-spec/guidelines.md:137` | Stage 3 |
| T3.5c | `.constitution/tech-spec/changelog.md` | Replace the sentence "The staged native toolchain supports only `x86_64-unknown-linux-gnu`; other Tier 1 hosts remain an OXY-D001 lock input." with exactly "The staged native toolchain supports only `x86_64-unknown-linux-gnu`; other Tier 1 hosts are a decided-deferred lock input owned by the next Stage 4 epic, superseding the archived hardware-ticket recommendation." | `.constitution/tech-spec/changelog.md` sentence review and `prettier --prose-wrap never --check '.constitution/**/*.md'`. | `.constitution/tech-spec/changelog.md:11`; [OXY-B007 hardware register](reference-hardware-access.md#reference-conformance-and-feasibility) | next Stage 4 epic |
| T3.5d | `.constitution/tech-spec/changelog.md` | Replace `Epic D owns the remaining baseline-validation ownership gap.` with exactly `Baseline-validation ownership is assigned: Stage 3 owns schema and typing; Stage 4 owns the workload and scoring-anchor corpus.` | `.constitution/tech-spec/changelog.md` sentence review and `prettier --prose-wrap never --check '.constitution/**/*.md'`. | `.constitution/tech-spec/changelog.md:15` | Stage 3 |
| T3.5e | `.constitution/tech-spec/changelog.md` | Replace `OXY-D001 owns advisory checks.` with exactly `Stage 3 owns binding a pinned offline RustSec advisory database and refresh policy.` | `.constitution/tech-spec/changelog.md` sentence review and `prettier --prose-wrap never --check '.constitution/**/*.md'`. | `.constitution/tech-spec/changelog.md:16` | Stage 3 |

### T4 lock known-unknown arrays as one lexicographic transaction

Order rule: Change all affected KU arrays and bindings together, sorted lexicographically, before exact-set tests.

| # | File | Field, section, or symbol | Enforcing check | Source anchor | Owner |
| :-- | :-- | :-- | :-- | :-- | :-- |
| T4.1 | `.constitution/tech-spec/contracts/qualification-lock.json` | Apply B002 +5, B003 +11, B004 +0, B005 +1, and B006 -1: `preImplementationKnownUnknowns` 13 to 29 and `gatingKnownUnknowns` 15 to 31. | `committed_candidate_gate_is_valid_but_open_with_the_exact_ku_set`. | [Epic D KU composition](../tasks/completed/EPIC-D-readiness-reconciliation.md#oxy-d001-inputs-from-epic-b) | Stage 3 |
| T4.2 | `qualification/fixtures/readiness/{invalid,cleared-without-evidence}.json` and `crates/oxyflut-qualification/src/readiness.rs` | Replace the two B006 policy KUs with `campaign-host-tool-records`; B005 +1 and B006 -1 leave `invalid.json` at pre-implementation/gating 13/15 and `cleared-without-evidence.json` at 12/15; update `KNOWN_UNKNOWN_BINDINGS`. | `cleared_ku_without_evidence_remains_open_with_the_exact_remaining_ku_set`, `clearing_a_ku_string_without_its_evidence_keeps_the_gate_open`, and `collect_known_unknowns`; `invalid_referenced_input_fixture_returns_exit_one` covers `invalid.json` without an exact-set assertion. | [SPK-B006, Spec edits required](../spikes/SPK-B006.md#spec-edits-required) | Stage 3 |

### T5 exact-set and counter assertions

Order rule: Update assertions after T1-T4 source shapes, instances, and arrays are final.

| # | File | Field, section, or symbol | Enforcing check | Source anchor | Owner |
| :-- | :-- | :-- | :-- | :-- | :-- |
| T5.1 | `xtask/src/commands/lock_tests.rs` | Update both exact KU tests, the three hard-coded lines ending `upstream-owner=OXY-D001` in `candidate_report_lines_are_stable_and_content_free`, and the `0.15.0` to `0.15.1` mutation literal. | `committed_candidate_gate_is_valid_but_open_with_the_exact_ku_set`, `cleared_ku_without_evidence_remains_open_with_the_exact_remaining_ku_set`, `candidate_report_lines_are_stable_and_content_free`, and the corrupt-platform-baseline assertion. | [Epic D KU composition](../tasks/completed/EPIC-D-readiness-reconciliation.md#oxy-d001-inputs-from-epic-b) and [SPK-B006, Spec edits required](../spikes/SPK-B006.md#spec-edits-required) | Stage 3 |
| T5.2 | `crates/oxyflut-qualification/src/readiness.rs` | Update the cleared-fixture exact KU assertion and changed bindings. | `clearing_a_ku_string_without_its_evidence_keeps_the_gate_open` and `KNOWN_UNKNOWN_BINDINGS`. | [SPK-B006, Spec edits required](../spikes/SPK-B006.md#spec-edits-required) | Stage 3 |
| T5.3 | `xtask/src/contracts/schema.rs` | Change `schema_count`/`instance_count` from 18/6 to 23/7. | `schema_compiles_committed_contract_instances_and_fixture_corpus`. | [Epic D KU composition](../tasks/completed/EPIC-D-readiness-reconciliation.md#oxy-d001-inputs-from-epic-b) | Stage 3 |
| T5.4 | `xtask/src/contracts/native_tests.rs` | Rename `abi_seven_through_nine_fail_before_callbacks_install`; change `7..=9` to `7..=10`. | `abi_seven_through_ten_fail_before_callbacks_install`. | [SPK-B001, D0 C ABI compatibility landing inventory](../spikes/SPK-B001.md#d0-c-abi-compatibility-landing-inventory) | Stage 3 |
| T5.5 | `xtask/src/contracts/traceability/{mod.rs,edges.rs,validation.rs,fixtures.rs,tests.rs}` | Advance `ACCESSIBILITY_MAP_SCHEMA`; remove `roles` from `REQUIRED_ACCESSIBILITY_CATEGORIES`; add keyed-role and registry edges. | `ACCESSIBILITY_MAP_SCHEMA`, `REQUIRED_ACCESSIBILITY_CATEGORIES`, and `validate_required_symbol_edges`. | [SPK-B001, Accessibility-map version-6 landing inventory](../spikes/SPK-B001.md#accessibility-map-version-6-landing-inventory) | Stage 3 |
| T5.6 | `xtask/src/contracts/schema.rs` and `qualification/fixtures/contracts/migration/` | Generalize `validate_migration_fixture` to named input/expected pairs for accessibility-map and qualification-lock migrations. | Source-byte assertion, expected-byte comparison, and v6 rejection in `validate_migration_fixture`. | [OXY-D001 decisions, Migration-fixture mechanism](#oxy-d001-decisions) | Stage 3 |

### T6 version migration

Order rule: Apply active-version edits after T1-T5 content is stable and before the v0.16.0 entry freezes the release record.

| # | File | Field, section, or symbol | Enforcing check | Source anchor | Owner |
| :-- | :-- | :-- | :-- | :-- | :-- |
| T6.1 | `xtask/`, `qualification/`, and `.constitution/tech-spec/` | Replace `0.15.0` with `0.16.0` in 62 files as of this report (commit `HEAD` of this PR); Stage 3 recounts with the same grep after T1-T5 land and migrates every hit, then regenerates affected valid parents and sidecars. | `grep -rl '0\\.15\\.0' xtask qualification .constitution/tech-spec` followed by `wc -l`; `validate_workspace`. | [SPK-B005, Version migration inventory](../spikes/SPK-B005.md#version-migration-inventory) | Stage 3 |
| T6.2 | `.constitution/tech-spec/guidelines.md` and `.constitution/tech-spec/stack.md` | Update the command table, `Version`, and the sole Scope-guard version reference. | Active-specification equality in traceability validation. | [SPK-B005, Version migration inventory](../spikes/SPK-B005.md#version-migration-inventory) | Stage 3 |
| T6.3 | `.constitution/tech-spec/changelog.md` | Prepend v0.16.0 and supersede `Known gaps routed to OXY-D001`. | `digests::validate_workspace`. | [SPK-B005, Counting-rules interpretation](../spikes/SPK-B005.md#counting-rules-interpretation-and-stage-3-validator-requirements) | Stage 3 |

### T7 remaining capture and registry artifacts

Order rule: T2.6 preserves sidecar-backed canonical bytes before T3.2 references them. Generate the accessibility registry after T3.1, and retain the capture bound as blocked until an Ubuntu 26.04 reference host supplies real output.

| # | File | Field, section, or symbol | Enforcing check | Source anchor | Owner |
| :-- | :-- | :-- | :-- | :-- | :-- |
| T7.1 | `qualification/fixtures/external-contracts/accessibility/candidate-neutral-role-registry.json` and `.sha256` (proposed in SPK-B001, not committed) | Generate the candidate-neutral registry outside the upstream-fixture convention. | `discover_contract_instances` `$schema` and registry-pointer edges. | [SPK-B001, Stage 3 semantic-role decision and P2R registry freeze](../spikes/SPK-B001.md#stage-3-semantic-role-decision-and-p2r-registry-freeze) | Stage 3 |
| T7.2 | `PATH.inventory.json` (proposed conventional referent from Epic C, not committed) | Confirm the temporary 256 KiB `wayland-info` and `xdpyinfo` capture bound; retain fail-closed truncation. | Stage 3 fail-closed capture-bound assertion. | [Epic D inputs from Epics A and C](../tasks/completed/EPIC-D-readiness-reconciliation.md#oxy-d001-inputs-from-epics-a-and-c) | Ubuntu 26.04 reference host |

### T8 digest-bound artifacts, frozen last

Order rule: Freeze bytes and regenerate every dependent digest only after each upstream header, schema, corpus, and instance edit is final.

| # | File | Field, section, or symbol | Enforcing check | Source anchor | Owner |
| :-- | :-- | :-- | :-- | :-- | :-- |
| T8.1 | `.constitution/tech-spec/contracts/oxyflut-substrate.h`, `qualification/fixtures/native/{interface.json,layout-probe.c.in,layout.x86_64-unknown-linux-gnu.json}` | Advance ABI `10u` to `11u`, add generated `OXY_SEMANTICS_ROLE_*` constants (proposed in SPK-B001, not committed), and regenerate the macro-based layout fixture after all header edits. | `validate_interface`, layout validation, and the generated-role contract test. | [SPK-B001, D0 C ABI compatibility landing inventory](../spikes/SPK-B001.md#d0-c-abi-compatibility-landing-inventory) | Stage 3 |
| T8.2 | `qualification/fixtures/generated-bindings/oxyflut-substrate.rs` and `.sha256` | Regenerate the bindgen golden and sidecar once after all header edits; review ABI 11 and every generated `OXY_SEMANTICS_ROLE_*` constant (proposed in SPK-B001, not committed); no xtask regeneration subcommand exists. | `validate_bindings` and the generated-role contract test. | [SPK-B001, D0 C ABI compatibility landing inventory](../spikes/SPK-B001.md#d0-c-abi-compatibility-landing-inventory) | Stage 3 |
| T8.3 | `qualification/staged/{fuzz-corpora,security-patch-rehearsal}.json` (proposed in SPK-B006, not committed) | Create staged records; bind `measurementPolicy.fuzzCorpora` and `measurementPolicy.securityPatchRehearsal`. | `POLICY_FIELDS` and `digests::validate_workspace`. | [SPK-B006, Spec edits required](../spikes/SPK-B006.md#spec-edits-required) | Stage 3 |
| T8.4 | `qualification/staged/{layout-visit-corpus,layout-visit-counting-rules}.json` and three layout schemas (proposed in SPK-B005, not committed) | Freeze artifacts and bind layout `measurementPolicy` fields; re-freeze four canonical blocks after an `issuingFamily` or cap-1 fixture change. | `xtask/src/commands/layout_prequalification.rs` (proposed in SPK-B005, not committed), `POLICY_FIELDS`, and `digests::validate_workspace`. | [SPK-B005, Counting-rules interpretation](../spikes/SPK-B005.md#counting-rules-interpretation-and-stage-3-validator-requirements) | Stage 3 |
| T8.5 | `.constitution/tech-spec/adrs/ADR-0010-production-substrate.md` and `qualification/fixtures/contracts/readiness/production-3b/` | If Stage 1 approves the evidence-schema migration, apply the ADR-0010 citation and production-3b cascade after transformed evidence bytes settle. | `adr_cites_verified_evidence` and `digests::validate_workspace`. | [OXY-B008, Spec edits required](qualification-assessors.md#spec-edits-required) | Stage 3 |

### Lock inputs the checklist must name separately

| Lock input | Lock field | Current state |
| :-- | :-- | :-- |
| Approved 52-capability baseline | `measurementPolicy.capabilityBaseline` | `null` at `.constitution/tech-spec/contracts/qualification-lock.json:107` |
| Reference application | `workload.referenceApplication` | `null` at `.constitution/tech-spec/contracts/qualification-lock.json:95` |
| Scenes | `workload.scenes` | `null` at `.constitution/tech-spec/contracts/qualification-lock.json:96` |
| Interaction scripts | `workload.interactionScripts` | `null` at `.constitution/tech-spec/contracts/qualification-lock.json:97` |
| Fonts | `workload.fonts` | `null` at `.constitution/tech-spec/contracts/qualification-lock.json:98` |
| Assets | `workload.assets` | `null` at `.constitution/tech-spec/contracts/qualification-lock.json:99` |
| Window matrix | `workload.windowMatrix` | `null` at `.constitution/tech-spec/contracts/qualification-lock.json:100` |
| Cache states | `workload.cacheStates` | `null` at `.constitution/tech-spec/contracts/qualification-lock.json:101` |
| Release flags | `workload.releaseFlags` | `null` at `.constitution/tech-spec/contracts/qualification-lock.json:102` |
| Scoring anchors | `measurementPolicy.scoringAnchors` | `null` at `.constitution/tech-spec/contracts/qualification-lock.json:109` |
| Assessor assignments | `measurementPolicy.assessors` | `null` at `.constitution/tech-spec/contracts/qualification-lock.json:110` |
| Reference-environment captures | `referenceEnvironments.*.{minimumVersion,hardwareId,gpuId,driverVersion,systemPackageLockDigest}` | `null` at `.constitution/tech-spec/contracts/qualification-lock.json:60-91` |
| Authoritative resolved-tool lock | `resolvedTools` | `[]` at `.constitution/tech-spec/contracts/qualification-lock.json:116` |

## Blocked external inputs

| Input | Blocking condition | Evidence | Owner | Unblock procedure |
| :-- | :-- | :-- | :-- | :-- |
| macOS arm64 reference hardware (B007-Q01) | No accountable owner or usable access procedure. | `.constitution/reports/reference-hardware-access.md#answers` | macOS arm64 reference host | Obtain a named owner and complete the macOS owner-confirmation procedure. |
| Windows x86-64 reference hardware (B007-Q02) | No accountable owner or usable access procedure. | `.constitution/reports/reference-hardware-access.md#answers` | Windows x86-64 reference host | Obtain a named owner and complete the Windows owner-confirmation procedure. |
| Second-configuration score-4 evidence for Linux rows | Wayland and X11 share one physical machine. | `.constitution/reports/reference-hardware-access.md#reference-conformance-and-feasibility` | second Linux reference configuration | Supply a physically distinct configuration. |
| Assessor 2 (B008) | No distinct human is named, available, or confirmed. | `.constitution/reports/qualification-assessors.md#question` | Assessor 2 | Preserve the second-assessor confirmation. |
| Stage 1 authorship-independence policy sentence | The exact sentence is not applied in the PRD. | `.constitution/reports/qualification-assessors.md#spec-edits-required` | Stage 1 | Approve and apply the exact section 3 sentence. |
| Stage 1 glossary terms | Eight routed terms lack Stage 1 adoption: ordinary visit; attempted ordinary visits; layout prequalification suite; second-configuration score-4 evidence; semantic-role registry; authorship independence; display-epoch equality tuple including targetModeSignature; campaign host. | `.constitution/tasks/completed/EPIC-D-readiness-reconciliation.md:55` | Stage 1 | Add the eight terms to the glossary. |
| Stage 2 ARC-R02 update | The exact replacement remains outside the architecture record. | [SPK-B005, Spec edits required](../spikes/SPK-B005.md#spec-edits-required) | Stage 2 | Apply the exact section 3 replacement. |
| Windows source-fixture capture procedure | Canonical bytes require a Windows host. | [SPK-B002, Spec edits required](../spikes/SPK-B002.md#spec-edits-required) | Windows x86-64 reference host | Run the specified capture procedure on a Windows host. |

## Conditions for the next Stage 4 minor release

1. Stage 3 applies the [Stage 3 reconciliation checklist](#stage-3-reconciliation-checklist) tiers that need no external input and releases the technical specification.
2. `/planning-engineering-execution` produces the next Stage 4 epic from that release to define the workload and scoring-anchor/assessor corpus.
3. `candidateImplementationReady: true` remains the gate for candidate implementation and measurement, not for planning workload-definition work.

Stage 4 report complete; Stage 3 reconciliation pending; readiness not set. This iteration cannot set either readiness flag.

## Next action

Stage 3 applies the [Stage 3 reconciliation checklist](#stage-3-reconciliation-checklist) tiers that need no external input and releases the technical specification. `/planning-engineering-execution` then produces the next Stage 4 epic for workload and scoring-anchor/assessor corpus definition. `candidateImplementationReady: true` remains the gate for candidate implementation and measurement, not for planning workload-definition work. Independently actionable now, subject to its stated order, are T5.2a, T1.1, T1.6, T2.5, T2.6.1, T2.6.2, and T3.5. Approval-dependent rows are T0.1-T0.4; the semantic-role branch T1.2-T1.3, T2.1-T2.2, T3.1-T3.1a, T5.4-T5.6, T7.1, and T8.1-T8.2; the layout branch T1.4-T1.5, T2.3-T2.4, T3.3-T3.4d, T4.1-T4.2, T5.1-T5.3, T6.1-T6.3, and T8.4; and T8.5. T2.6.3 and T7.2 remain blocked external inputs for the Windows x86-64 and Ubuntu 26.04 reference hosts; lock bindings in T3.3 remain open until the named hardware and assessor inputs arrive.

## Sources

- `.constitution/tasks/{completed/EPIC-D-readiness-reconciliation.md,critical-path.md}`, `.constitution/tech-spec/{changelog.md,stack.md,guidelines.md,data-models/README.md}`, `.constitution/prd/{constraints.md,glossary.md}`, and `.constitution/architecture/risks.md`.
- `.constitution/tech-spec/data-models/{capability-traceability.schema.json,accessibility-map.schema.json,specification-phase.schema.json,raw-measurement.schema.json,qualification-lock.schema.json}` and `.constitution/tech-spec/contracts/{qualification-lock.json,capability-traceability.json,platform-contracts.json,oxyflut-public.rs,oxyflut-qualification.rs,oxyflut-substrate.rs,oxyflut-substrate.h,specification-phase.json}`.
- `.constitution/tech-spec/adrs/{ADR-0005-platform-hosts.md,ADR-0010-production-substrate.md}`, `.constitution/reports/{reference-hardware-access.md,qualification-assessors.md}`, `crates/oxyflut-qualification/src/{readiness.rs,measurement.rs}`, and `xtask/src/commands/{baseline.rs,lock_tests.rs,external_contracts.rs,contracts.rs}`.
- `xtask/src/contracts/{schema.rs,readiness.rs,native.rs,native_tests.rs,digests.rs,readiness_promotion.rs}` and `xtask/src/contracts/traceability/{mod.rs,edges.rs,validation.rs,fixtures.rs,tests.rs}`.
- `qualification/tools/native-contract-toolchain.json`, `qualification/schemas/{sample-validity.schema.json,external/README.md,external/proposed-external-contract-lock.json}`, and `qualification/fixtures/{readiness/cleared-without-evidence.json,readiness/invalid.json,readiness/complete.synthetic.json,evidence/positive-derived.json,baselines/complete.synthetic.json,measurements/complete.synthetic.json,sample-validity/complete.synthetic.json,native/interface.json,native/layout-probe.c.in,native/layout.x86_64-unknown-linux-gnu.json,generated-bindings/oxyflut-substrate.rs,generated-bindings/oxyflut-substrate.rs.sha256,contracts/migration/source.json,contracts/migration/source.sha256,contracts/migration/derived.json}`.
- [SPK-B001, Spec edits required](../spikes/SPK-B001.md#spec-edits-required), [D0 C ABI compatibility landing inventory](../spikes/SPK-B001.md#d0-c-abi-compatibility-landing-inventory), [Accessibility-map version-6 landing inventory](../spikes/SPK-B001.md#accessibility-map-version-6-landing-inventory), and [Stage 3 semantic-role decision and P2R registry freeze](../spikes/SPK-B001.md#stage-3-semantic-role-decision-and-p2r-registry-freeze).
- [SPK-B002](../spikes/SPK-B002.md#spec-edits-required), [SPK-B003](../spikes/SPK-B003.md#spec-edits-required), [SPK-B004](../spikes/SPK-B004.md#spec-edits-required), [SPK-B005](../spikes/SPK-B005.md#spec-edits-required), [Counting-rules interpretation](../spikes/SPK-B005.md#counting-rules-interpretation-and-stage-3-validator-requirements), [Version migration inventory](../spikes/SPK-B005.md#version-migration-inventory), [Layout prequalification additions inventory](../spikes/SPK-B005.md#layout-prequalification-additions-inventory), and [SPK-B006](../spikes/SPK-B006.md#spec-edits-required).
