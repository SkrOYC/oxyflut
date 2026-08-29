# Pre-implementation readiness reconciliation

- Original date: 2026-08-29
- Ticket: OXY-D001
- Status: reconciliation complete; readiness not set

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

The commands in this section ran from the repository root and did not modify repository files. Exit code 2 for a lock-status command means valid but open.

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

The staged paths in that validator excerpt are conventional referents, not committed files: `qualification/staged/{assessors,scoring-anchors}.json` are proposed by the OXY-D001 inputs, and `qualification/staged/{fuzz-corpora,security-patch-rehearsal}.json` are proposed in SPK-B006, not committed.

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

`environment inspect` was not run because it writes a `PATH.inventory.json` companion artifact into the repository. The recorded Epic C facts and `.constitution/tech-spec/changelog.md:7-19` supply the environment-tooling state instead.

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

`qualification/tools/native-contract-toolchain.json` records the staged compiler, linker, archiver, symbol inspector, binding generators, formatter, Rust tools, and libc-header utility records. Every record has host triple `x86_64-unknown-linux-gnu`; `.constitution/tech-spec/changelog.md:9-10` retains other Tier 1 hosts as a lock input.

### External-contract manifest and snapshot convention

`qualification/schemas/external/proposed-external-contract-lock.json` contains the nonauthoritative proposed external-lock values, while the active external lock remains unresolved. `qualification/schemas/external/README.md` defines same-directory `source.json` records for upstream identity, license information, and neighbor digest; it does not adopt the proposal into the lock.

### Baseline tooling

`xtask/src/commands/baseline.rs:22-60` validates one candidate-neutral baseline from `--input`; its `--output` path publishes artifacts and was not used. The synthetic baseline fixture passed the read-only run, but `.constitution/tech-spec/contracts/qualification-lock.json:94-105` retains `measurementPolicy.capabilityBaseline` as `null`.

### Measurement templates

`crates/oxyflut-qualification/src/measurement.rs:1-24` provides library-only `generate_templates` and `compute_comparison_bounds`; no template-generation command surface exists. `.constitution/tech-spec/data-models/raw-measurement.schema.json:1-58` is the committed raw schema, but it omits `$schema` and has no per-`(constraintId, launch)` non-decreasing `monotonicNs` schema rule. The successful synthetic-fixture validation verifies parsing and bindings only; it reports no observation.

### Environment tooling

`.constitution/tech-spec/changelog.md:9-18` records the reproducible shell, the `x86_64-unknown-linux-gnu` staged-tool limit, and the `PATH.inventory.json` companion behavior. The environment command was not run because its output publication is outside this ticket.

### Read-only lock report

`.constitution/tech-spec/contracts/qualification-lock.json:1-131` has `candidateImplementationReady: false`, `measurementReady: false`, all ten `measurementPolicy` keys set to `null`, an empty `resolvedTools` array, and `null` values for each lock-input field family under every `referenceEnvironments` entry. The two preserved lock-status runs report both gates as open and enumerate the corresponding blocks.

### Count reconciliation

The live lock has 13 committed pre-implementation KUs (`.constitution/tech-spec/contracts/qualification-lock.json:114-144`). Applying the deltas from [SPK-B002](../spikes/SPK-B002.md#spec-edits-required), [SPK-B003](../spikes/SPK-B003.md#spec-edits-required), [SPK-B004](../spikes/SPK-B004.md#spec-edits-required), [SPK-B005](../spikes/SPK-B005.md#spec-edits-required), and [SPK-B006](../spikes/SPK-B006.md#spec-edits-required): +5, +11, +0, +1, and -1, gives 29. The live `qualification/fixtures/readiness/cleared-without-evidence.json` fixture has 12 KUs; its B005 +1 plus B006 -1 deltas retain 12. The live `xtask/src/contracts/schema.rs:606-614` assertions are `schema_count`/`instance_count` 18/6; SPK-B001 contributes +2/+1 and SPK-B005 contributes +3/+0, giving 23/7. The live `.constitution/tech-spec/contracts/oxyflut-substrate.h:22` header has ABI `10u`; SPK-B001 requires `11u`. The required live command `grep -rl '0\.15\.0' xtask qualification .constitution/tech-spec | wc -l` returns 62, while SPK-B005's 56 count covers only `xtask` and `qualification`.

### Routed Stage 3 and upstream text replacements

- Stage 3 must replace identifier-only `.constitution/tech-spec/data-models/capability-traceability.schema.json` `mappings[].contractTests[]` entries with entries that include the physical contract-test file location.
- Stage 3 must replace `.constitution/tech-spec/data-models/accessibility-map.schema.json` `reverseActions[].textLayoutBinding` with a binding that includes the text-layout generation value.
- Stage 3 must replace generic `.constitution/tech-spec/data-models/specification-phase.schema.json` evidence references for `layoutQualification`, `finalContractSet`, `targetMatrix`, `losingCandidateRemoval`, and `billOfMaterials` with typed-schema references.
- Stage 3 must add the `$schema` property and the per-`(constraintId, launch)` non-decreasing `samples[].monotonicNs` rule to `.constitution/tech-spec/data-models/raw-measurement.schema.json`.
- Stage 3 must type `measurementPolicy.sampleValidityRules`, the external-contract proposal, `PATH.inventory.json`, Wayland interface completeness, and the conventional digest referents for scoring anchors, assessors, fuzz corpora, and security-patch rehearsal.
- Stage 1 must replace the `.constitution/prd/constraints.md` paragraph that begins "The numeric common-case node-visit limit" with: "The numeric common-case node-visit limit for CAP-LAY-001 remains a gating known unknown until the prequalification lock binds candidate and environment identities and the 48-tuple timing probe supplies schema-valid evidence under CON-PERF-001 on unblocked reference hardware."
- Stage 2 must replace `.constitution/architecture/risks.md` `ARC-R02` `Mitigation or follow-up` with: "Record the corpus and Table 4 finite per-policy visit-cap freeze as partially discharged; retain the numeric global layout-visit cap as the remaining gating condition until the prequalification lock binds candidate and environment identities and the 48-tuple timing probe supplies schema-valid evidence under CON-PERF-001 on unblocked reference hardware."
- Stage 1 must add `ordinary visit`, `attempted ordinary visits`, `layout prequalification suite`, `second-configuration score-4 evidence`, `semantic-role registry`, `authorship independence`, `display-epoch equality tuple`, `targetModeSignature`, and `campaign host` to `.constitution/prd/glossary.md` before Stage 3 adopts them.
- Stage 1 must insert this exact sentence after the independent-scoring sentence in `.constitution/prd/constraints.md`: "A person who authors candidate implementation or qualification evidence for a candidate must not serve as an independent scorer for that candidate."

The proposed `qualification/staged/` records, the proposed `qualification/fixtures/external-contracts/{macos,wayland,x11,windows,accessibility}/` directories, the proposed `.constitution/tech-spec/contracts/semantic-role-registry.json`, and proposed `OXY_SEMANTICS_ROLE_*` symbols are proposed in SPK-B001, SPK-B002, SPK-B003, SPK-B004, SPK-B005, or SPK-B006, not committed.

## Known-unknown classification

Table 1 classifies every committed pre-implementation KU and both gating-only source-pin KUs. Each cited result is one committed path and each owner is singular.

| Known unknown | Binding field | Cited result | Status | Owner | Next action |
| :-- | :-- | :-- | :-- | :-- | :-- |
| `minimum-platform-and-protocol-versions` | `referenceEnvironments` | `.constitution/tech-spec/contracts/qualification-lock.json:53-88` retains every `minimumVersion` as `null`. | blocked external input | OXY-B007 | Obtain the missing macOS, Windows, and Ubuntu reference-host captures, then apply the Stage 3 platform revisions. |
| `hardware-gpu-driver-and-system-package-locks` | `referenceEnvironments` | `.constitution/reports/reference-hardware-access.md#reference-conformance-and-feasibility` records non-reference Linux access and blocked macOS and Windows rows. | blocked external input | OXY-B007 | Obtain accountable reference-host access and capture hardware, GPU, driver, and package-lock identities. |
| `reference-application-scenes-scripts-fonts-assets-windows-cache-and-flags` | `workload` | `.constitution/tech-spec/contracts/qualification-lock.json:89-98` sets every workload field to `null`. | retained KU | OXY-D001 | Define and type the approved workload corpus in the Stage 3 lock revision. |
| `raw-measurement-and-sample-validity-contracts` | `measurementPolicy.sampleValidityRules` | `.constitution/tech-spec/data-models/raw-measurement.schema.json:1-58` lacks `$schema` and the monotonic-time schema rule. | retained KU | Stage 3 | Add the raw-measurement revisions and type the staged sample-validity referent. |
| `capability-and-platform-baselines` | `measurementPolicy.capabilityBaseline` | `.constitution/tech-spec/contracts/qualification-lock.json:99-112` keeps `capabilityBaseline` and `platformContracts` `null`. | retained KU | Stage 3 | Bind approved capability and platform baseline artifacts with typed references. |
| `independent-presentation-opportunity-sources` | `measurementPolicy.platformContracts` | `.constitution/spikes/SPK-B003.md#spec-edits-required` retains the Wayland independent-meter capture requirements. | blocked external input | OXY-B007 | Capture reference-session meter evidence and apply the Stage 3 platform-contract revision. |
| `complete-ime-editing-geometry-and-accessibility-maps` | `measurementPolicy.platformContracts` | `.constitution/reports/reference-hardware-access.md#answers` records blocked macOS and Windows host access. | blocked external input | OXY-B007 | Obtain the blocked host access and capture the required IME and accessibility artifacts. |
| `scoring-anchors-and-two-assessors` | `measurementPolicy.scoringAnchors` | `.constitution/reports/qualification-assessors.md#question` leaves Q2 through Q4 gating and Assessor 2 unnamed. | blocked external input | OXY-B008 | Obtain the second assessor confirmation and Stage 1 authorship-independence approval. |
| `fuzz-corpora` | `measurementPolicy.fuzzCorpora` | `.constitution/tech-spec/contracts/qualification-lock.json:99-112` sets `fuzzCorpora` to `null`. | retained KU | OXY-D001 | Admit the `fuzz-corpora` record proposed in SPK-B006, not committed, through a typed Stage 3 binding. |
| `security-patch-rehearsal` | `measurementPolicy.securityPatchRehearsal` | `.constitution/tech-spec/contracts/qualification-lock.json:99-112` sets `securityPatchRehearsal` to `null`. | retained KU | OXY-D001 | Admit the security-patch record proposed in SPK-B006, not committed, through a typed Stage 3 binding. |
| `layout-visit-cap` | `measurementPolicy.layoutVisitCap` | `.constitution/spikes/SPK-B005.md#question` retains row 6 as a gating KU. | blocked external input | OXY-B007 | Obtain the blocked reference hosts before the Stage 3 layout-lock revision can receive capture input. |
| `external-distribution-schema-snapshots-and-verifiers` | `measurementPolicy.externalContractLock` | `.constitution/tech-spec/contracts/qualification-lock.json:99-112` sets `externalContractLock` to `null`. | retained KU | Stage 3 | Adopt or replace the proposed external-contract lock through a typed Stage 3 binding. |
| `resolved-tool-digests` | `resolvedTools` | `.constitution/tech-spec/contracts/qualification-lock.json:113-113` keeps `resolvedTools` empty. | retained KU | OXY-A008 | Bind an authoritative resolved-tool lock that conforms to the staged manifest. |
| `integrated-fork-commit` (gating-only) | `sourcePins.integratedFork.commit` | `.constitution/tech-spec/contracts/qualification-lock.json:17-18` sets the commit to `null`. | retained KU | Stage 3 | Record a verified immutable integrated-fork commit in the Stage 3 lock revision. |
| `oxyflut-adapter-commit` (gating-only) | `sourcePins.oxyflutAdapter.commit` | `.constitution/tech-spec/contracts/qualification-lock.json:18-18` sets the commit to `null`. | retained KU | Stage 3 | Record a verified immutable Oxyflut-adapter commit in the Stage 3 lock revision. |

## OXY-D001 decisions

Table 2 records repository-supported decisions and bounded deferrals. The report routes every implementation task to Stage 3 or an external input.

| Decision | Basis | Outcome | Stage 3 instruction | Owner |
| :-- | :-- | :-- | :-- | :-- |
| External-fixture `source.json` sidecar validator ownership | `xtask/src/commands/external_contracts.rs:797-920` hard-codes `SNAPSHOTS`, and `xtask/src/contracts/digests.rs:214-257` scans only contract JSON. | Adopt validator extension. | Extend `external-contracts verify` to validate every regular same-stem `qualification/fixtures/external-contracts/**/*.source.json` sidecar: require a regular sibling, equal sibling SHA-256, canonical retrieval URL, upstream-relative `path` and `licenseSource.path`, and required license fields. | Stage 3 |
| GTK and AT-SPI row alignment across Wayland and X11 | `SPK-B003.md#spec-edits-required` and `SPK-B004.md#spec-edits-required` propose the shared Ubuntu package identities. | Align both rows on `libgtk-4-1` `4.22.2+ds-1ubuntu1` and `at-spi2-core` `2.60.0-1`; keep the GTK API ceiling and AT-SPI XML source floors distinct. | Apply identical package identities to both platform rows and retain each environment's distinct source-floor evidence. | Stage 3 |
| `issuingFamily` corpus field and cap-1 rejection fixture | `SPK-B005.md#counting-rules-interpretation-and-stage-3-validator-requirements` requires `issuingFamily` for nonordinary families. | Add both before the v6 lock binds the SPK-B005 digests; re-freeze corpus, counting rules, model source, and changelog blocks. | Require `issuingFamily` for nonordinary fixtures, reject absent or unknown values, and add a cap-1 rejection fixture. | Stage 3 |
| Baseline-validation owner assignment | `xtask/src/commands/baseline.rs:22-60` validates one baseline artifact and its authority. | Assign schema and typing to Stage 3; assign workload and scoring-anchor corpus ownership to Stage 4. | Type baseline references and validation inputs, then define the workload and scoring-anchor corpus owners. | OXY-D001 |
| Windows 77 excerpt fixtures | `SPK-B002.md#spec-edits-required` records excerpt fixtures because no Windows host supplies canonical bytes. | Retain excerpts while no Windows host exists; convert to canonical bytes plus sidecars after canonical host access exists. | Define the conversion trigger and sidecar validation rule in the Stage 3 external-fixture revision. | OXY-D001 |
| Migration-fixture mechanism | `xtask/src/contracts/schema.rs:505-540` validates one fixed `migration/{source.json,source.sha256,derived.json}` triple. | Generalize the helper to named per-migration input and derived-output pairs. | Add a per-migration registry that validates each named pair from the proposed SPK-B001 and SPK-B005 migrations, not committed. | Stage 3 |
| 256 KiB `wayland-info` and `xdpyinfo` capture bound | `.constitution/reports/reference-hardware-access.md#reference-conformance-and-feasibility` confirms only a non-reference NixOS host. | Defer and retain fail-closed truncation handling. | Add the bound only after real Ubuntu 26.04 capture sizes are available. | OXY-B007 |
| Other Tier 1 hosts as a lock input | `.constitution/tech-spec/changelog.md:9-10` limits the staged toolchain to `x86_64-unknown-linux-gnu`. | Defer pending blocked hardware inputs. | Add host records only after accountable Tier 1 host capture exists. | OXY-B007 |
| Wayland interface-set completeness rule | `.constitution/tech-spec/changelog.md:30-31` identifies the absent completeness rule and partial `protocolVersion` representation. | Defer completeness determination pending a reference-session capture. | Add an `interfaceSetCompleteness` schema rule that requires the captured interface set and rejects absent or partial observed `protocolVersion`. | Stage 3 |
| Offline advisory database and refresh policy | `.constitution/tasks/active/EPIC-D-readiness-reconciliation.md:58-59` records no pinned vendored advisory database. | Defer advisory validation. | Bind a pinned offline RustSec advisory database, its digest, refresh authority, cadence, and CI location. | Stage 3 |

The proposed SPK-B001 and SPK-B005 migration filenames use `<name>-v5-to-v6.input.json` and `<name>-v5-to-v6.expected.json`; they are proposed in SPK-B001 and SPK-B005, not committed.

## Missing approved or captured lock inputs

Table 3 identifies every null policy value, empty resolved-tool array, null reference-environment field family, and false readiness flag. The entries name inputs only and do not change a flag.

| Lock field | Current value | What fills it (exact approval or capture) | Owner |
| :-- | :-- | :-- | :-- |
| `measurementPolicy.rawMeasurementSchema` | `null` | Stage 3 approval of the revised raw-measurement schema with `$schema` and the non-decreasing per-`(constraintId, launch)` rule. | Stage 3 |
| `measurementPolicy.sampleValidityRules` | `null` | Stage 3 adoption of the committed staged sample-validity proposal with a typed lock referent and digest. | Stage 3 |
| `measurementPolicy.capabilityBaseline` | `null` | An approved 52-capability baseline with its approval-evidence digest and typed lock reference. | Stage 3 |
| `measurementPolicy.platformContracts` | `null` | A typed digest binding for the revised platform-contract baseline and its captured evidence. | Stage 3 |
| `measurementPolicy.scoringAnchors` | `null` | One immutable scoring-anchor artifact accepted by two confirmed assessors. | OXY-B008 |
| `measurementPolicy.assessors` | `null` | One immutable two-assessor declaration that satisfies the Stage 1-approved authorship rule. | OXY-B008 |
| `measurementPolicy.fuzzCorpora` | `null` | Admission of the `fuzz-corpora` record proposed in SPK-B006, not committed, with its typed digest binding. | OXY-D001 |
| `measurementPolicy.securityPatchRehearsal` | `null` | Admission of the security-patch record proposed in SPK-B006, not committed, with its typed digest binding. | OXY-D001 |
| `measurementPolicy.externalContractLock` | `null` | Stage 3 adoption or replacement of `qualification/schemas/external/proposed-external-contract-lock.json`. | Stage 3 |
| `measurementPolicy.layoutVisitCap` | `null` | Schema-valid reference-host capture from the bounded layout probe and the approved cap decision. | OXY-B007 |
| `resolvedTools` | `[]` | An authoritative resolved-tool lock that matches `qualification/tools/native-contract-toolchain.json`. | OXY-A008 |
| `referenceEnvironments.*.minimumVersion` | `null` | Captures that establish every reference OS and protocol minimum. | OXY-B007 |
| `referenceEnvironments.*.hardwareId` | `null` | Accountable reference-host hardware identity captures for every Tier 1 environment. | OXY-B007 |
| `referenceEnvironments.*.gpuId` | `null` | Accountable reference-host GPU identity captures for every Tier 1 environment. | OXY-B007 |
| `referenceEnvironments.*.driverVersion` | `null` | Accountable reference-host driver-version captures for every Tier 1 environment. | OXY-B007 |
| `referenceEnvironments.*.systemPackageLockDigest` | `null` | Signed or immutable package-lock captures for every reference environment. | OXY-B007 |
| `candidateImplementationReady` | `false` | Complete approved and captured inputs for every pre-implementation lock field and no pre-implementation KU. | Stage 3 |
| `measurementReady` | `false` | Complete approved and captured inputs for every measurement lock field and no gating KU. | Stage 3 |

<!-- M2 sections follow -->
