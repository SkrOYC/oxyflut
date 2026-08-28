# Qualification assessors

- Ticket: OXY-B008
- Status: Completed - candidate implementation is blocked
- Clock start: 2026-08-28T16:22:14Z
- Clock stop: 2026-08-28T16:28:14Z
- Scope: assessor coordination only; this report assigns no candidate score and makes no candidate selection.

## Question

| ID | Question | Status | Answer and evidence | Next bounded probe |
| :-- | :-- | :-- | :-- | :-- |
| Q1 | Are the six frozen weighted criteria enumerated? | KK | Yes. The six exact criterion names and weights are quoted in [Frozen scoring criteria](#frozen-scoring-criteria). Source: `.constitution/prd/constraints.md`, `Substrate selection policy`; the preserved extraction probe confirms the text. | Not applicable. |
| Q2 | Are the scoring anchors frozen before candidate implementation? | KU (gating) | No. `.constitution/tech-spec/contracts/qualification-lock.json` sets `measurementPolicy.scoringAnchors` to `null`, and `.constitution/prd/constraints.md` states, "The scoring anchors must be frozen before either candidate implementation begins." | Before candidate implementation, run an HITL anchor workshop with both accepted assessors. Expected output: one immutable artifact that defines cited-evidence anchors for each integer in the 3-5 scale for each of the six criteria, with both assessors' written acceptance and a SHA-256 digest. |
| Q3 | Has Assessor 1 accepted the role, independence rule, scale, criteria, and consensus procedure? | KK | Yes. Oscar Y. <oscar@ocmasesorias.com> made the required direct session confirmation on 2026-08-28. The preserved `git var GIT_AUTHOR_IDENT` probe confirms the configured repository author identity. The confirmation covers participation, independence from candidate score conclusions, the integer 3-5 scale, all six criteria, and written consensus. | Not applicable. |
| Q4 | Is a distinct independent Assessor 2 named, available, and confirmed? | KU (gating) | No. No second distinct independent human has been named or has supplied a confirmation, availability declaration, or conflict disclosure. | Obtain the exact second-assessor confirmation in [Second-assessor confirmation procedure](#second-assessor-confirmation-procedure) before candidate implementation. Expected output: a named human's written confirmation, completed conflict disclosure, availability declaration, and affirmation that no candidate score conclusion was seen before independent scoring. |
| Q5 | Is the no-prior-score-conclusion independence rule established? | KK | Yes. Assessor 1 confirmed that no candidate score conclusions exist or were seen in this session. The procedure requires Assessor 2 to make the same declaration before receiving evidence or scoring. Source: OXY-B008 session confirmation and `.constitution/reports/2026-08-09-open-decisions.md`, `OD-01: Rendering substrate`. | Not applicable. |
| Q6 | Is the evidence-access and written-consensus procedure frozen? | KK | Yes. This report freezes the access order and consensus record in [Evidence access procedure](#evidence-access-procedure) and [Written consensus procedure](#written-consensus-procedure). The policy requires independent integer scores from cited evidence and one written consensus score for every disagreement. Sources: `.constitution/prd/constraints.md`, `Substrate selection policy`; `.constitution/reports/2026-08-09-open-decisions.md`, `OD-01: Rendering substrate`; and `.constitution/tech-spec/data-models/qualification-evidence.schema.json`, `$defs.score`. | Not applicable. |
| Q7 | Can the qualification lock's assessor field be set and candidate implementation begin? | KU (gating) | No. The current lock has `candidateImplementationReady: false`, `measurementReady: false`, and `measurementPolicy.assessors: null`. The lock also lists `scoring-anchors-and-two-assessors` among pre-implementation and gating known unknowns. A resolved measurement policy requires a SHA-256 for `assessors`. Sources: `.constitution/tech-spec/contracts/qualification-lock.json`, root and `measurementPolicy`; `.constitution/tech-spec/data-models/qualification-lock.schema.json`, `$defs.measurementPolicy` and `$defs.resolvedMeasurementPolicy`. | Complete Q2 and Q4, then have Stage 3 bind the completed immutable artifacts by SHA-256 in the qualification lock and validate the lock. Expected output: no assessor or scoring-anchor KU remains before candidate implementation. |

## Evidence

### Frozen scoring criteria

The following table quotes the six criteria exactly from `.constitution/prd/constraints.md`, `Substrate selection policy`.

| Criterion                                       | Weight |
| :---------------------------------------------- | -----: |
| Demonstrated P0 platform coverage               |     30 |
| Two-transition upgrade-maintenance cost         |     20 |
| Performance, startup, memory, and artifact size |     15 |
| Boundary safety, security, and privacy          |     15 |
| Distribution, licensing, and provenance         |     10 |
| Testing, diagnostics, and operational clarity   |     10 |

The same source states: "After hard-gate eligibility, two assessors independently assign an integer score from 3 through 5 to each criterion from cited evidence. They must record one consensus score for every disagreement." It also states: "The scoring anchors must be frozen before either candidate implementation begins."

`CAP-SUB-001` requires the same complete P0 and constraint suite for every candidate, and `CAP-SUB-002` rejects a candidate with a failed or unresolved gating P0 item. Source: `.constitution/prd/capabilities.md`, `Substrate qualification`.

### Contract fields

| Source path and field | Exact contract value or rule | Status |
| :-- | :-- | :-- |
| `.constitution/tech-spec/contracts/qualification-lock.json`, `candidateImplementationReady` | `false` | KK |
| `.constitution/tech-spec/contracts/qualification-lock.json`, `measurementReady` | `false` | KK |
| `.constitution/tech-spec/contracts/qualification-lock.json`, `measurementPolicy.scoringAnchors` | `null` | KK |
| `.constitution/tech-spec/contracts/qualification-lock.json`, `measurementPolicy.assessors` | `null` | KK |
| `.constitution/tech-spec/contracts/qualification-lock.json`, `preImplementationKnownUnknowns` and `gatingKnownUnknowns` | `scoring-anchors-and-two-assessors` is present in both arrays. | KK |
| `.constitution/tech-spec/data-models/qualification-lock.schema.json`, `$defs.measurementPolicy.properties.scoringAnchors` | `{ "$ref": "#/$defs/digestOrNull" }` | KK |
| `.constitution/tech-spec/data-models/qualification-lock.schema.json`, `$defs.measurementPolicy.properties.assessors` | `{ "$ref": "#/$defs/digestOrNull" }` | KK |
| `.constitution/tech-spec/data-models/qualification-lock.schema.json`, `$defs.resolvedMeasurementPolicy.properties.scoringAnchors` and `assessors` | Each field is `{ "$ref": "#/$defs/sha256" }`. | KK |
| `.constitution/tech-spec/data-models/qualification-evidence.schema.json`, `$defs.assessor` | Required fields are `id` and `frozenBeforeImplementation`, whose value is `true`. | KK |
| `.constitution/tech-spec/data-models/qualification-evidence.schema.json`, `$defs.score` | It requires two assessor scores, one integer consensus score, a nonempty written consensus rationale, and cited evidence; every score is an integer from 3 through 5. | KK |
| `.constitution/tech-spec/data-models/selection-decision.schema.json`, root `required` | `qualificationLockDigest`, `candidateEvidence`, `eligibility`, `decisionBasis`, `outcome`, `selectedCandidate`, `calculation`, and `rationale` are required. The schema is a decision-output contract, not an assessor registry. | KK |

The assessment record must satisfy the qualification-evidence contract only after hard-gate eligibility. This report doesn't create candidate evidence, an eligibility outcome, a calculation, a score, or a selected candidate.

### Assessor register

| Assessor | Role | Availability | Conflict disclosure | Confirmation and independence |
| :-- | :-- | :-- | :-- | :-- |
| Oscar Y. <oscar@ocmasesorias.com> | Assessor 1; independent evidence reviewer and written-consensus participant. | Participation was confirmed in this 2026-08-28 session. No future scoring schedule is claimed. | Repository owner. Oscar confirmed independence from candidate score conclusions; no candidate score conclusions exist or were seen before independent scoring. No other conflict disclosure was supplied in this session. | Confirmed participation, the integer 3-5 scale, all six criteria, and written consensus for disagreements. Oscar must record an independent score before seeing any other assessor's score conclusion. |
| BLOCKED | No role can be assigned until a distinct human is named and accepts it. | No availability declaration exists. | No identity or conflict disclosure exists. | No confirmation exists. This is the stop condition for candidate implementation. |

### Independence rules

1. Each assessor must record their own integer score for every criterion from cited evidence before viewing the other assessor's score conclusion.
2. An assessor who has seen a candidate score conclusion before recording their own independent score cannot serve for that candidate's independent-scoring pass.
3. Candidate implementation, candidate evidence, and candidate score conclusions must remain separate from this coordination record.
4. Each assessor must disclose repository ownership, employment or organizational role, financial or other interest in either candidate or its upstreams, candidate implementation or evidence-authoring involvement, and any prior exposure to candidate score conclusions.
5. A disclosed relationship doesn't silently waive independence. Both assessors must record whether it prevents independent scoring before evidence access begins.
6. A replacement assessor must complete the full confirmation procedure before seeing any candidate score conclusion.

### Evidence access procedure

1. The coordinator gives both accepted assessors the same immutable qualification lock and the same cited candidate evidence set at the same time.
2. Assessors can read the repository sources at `.constitution/prd/`, `.constitution/reports/`, `.constitution/spikes/`, `.constitution/tech-spec/contracts/`, and `.constitution/tech-spec/data-models/` and the qualification evidence at `qualification/evidence/`, `qualification/golden/`, `qualification/fixtures/`, and `qualification/schemas/`.
3. Before independent records are complete, the coordinator withholds the other assessor's score conclusion and any discussion that reveals it.
4. Each independent record identifies the evidence path and SHA-256, criterion, integer score, and rationale without assigning a score in this coordination report.
5. The coordinator releases both independent records together after both records are complete, then begins written consensus only for disagreements.

### Written consensus procedure

1. Confirm hard-gate eligibility before any scoring discussion. A candidate with a failed or unresolved gating P0 item is ineligible for scoring.
2. Each assessor records an independent integer score from 3 through 5 for each of the six criteria from cited evidence.
3. For each disagreement, both assessors create one written consensus record that identifies the criterion, both independent records, cited evidence, the consensus score, and the rationale.
4. The consensus record doesn't replace either independent record. It preserves the disagreement and explains its resolution.
5. The completed record must conform to `.constitution/tech-spec/data-models/qualification-evidence.schema.json`, including `assessors`, `scores`, and `weightedTotal`, before any selection-decision record is created.

### Second-assessor confirmation procedure

Before candidate implementation, the second distinct human must provide a written confirmation containing all of the following statements and disclosures.

1. State their name and a contact identity that is distinct from Oscar Y. <oscar@ocmasesorias.com>.
2. Accept the role of Assessor 2, including independent evidence review and written consensus for disagreements.
3. Declare an availability window that covers independent scoring and the later written-consensus step.
4. Disclose repository ownership or maintainer status, employer or organizational role, financial or other interest in either candidate or upstream, candidate implementation or evidence-authoring involvement, and any prior exposure to candidate score conclusions.
5. Confirm participation, the integer 3-5 scale, the six exact criteria in [Frozen scoring criteria](#frozen-scoring-criteria), and the written-consensus procedure in this report.
6. Confirm that they haven't seen any candidate score conclusion before recording independent scores and that they won't see another assessor's conclusion until their own record is complete.
7. State whether any disclosure prevents independent scoring. If it does, the coordinator must name a replacement assessor and repeat this procedure before evidence access.

### Preserved probe outputs

The following identity probe was run on this host. It confirms the configured Git author identity but doesn't replace the recorded HITL confirmation.

```text
command: git var GIT_AUTHOR_IDENT
Oscar Y. <oscar@ocmasesorias.com> 1787934187 -0400
command: date -u +%Y-%m-%dT%H:%M:%SZ
2026-08-28T16:23:07Z
```

The following source-inspection probe was run on this host.

```text
command: grep -A 8 "| Criterion" .constitution/prd/constraints.md
| Criterion                                       | Weight |
| :---------------------------------------------- | -----: |
| Demonstrated P0 platform coverage               |     30 |
| Two-transition upgrade-maintenance cost         |     20 |
| Performance, startup, memory, and artifact size |     15 |
| Boundary safety, security, and privacy          |     15 |
| Distribution, licensing, and provenance         |     10 |
| Testing, diagnostics, and operational clarity   |     10 |

command: grep qualification-lock status fields
  "candidateImplementationReady": false,
  "measurementReady": false,
    "scoringAnchors": null,
    "assessors": null,
    "scoring-anchors-and-two-assessors",
    "scoring-anchors-and-two-assessors",
command: grep -A 5 schema assessor and scoring-anchor fields
        "scoringAnchors",
        "assessors",
        "fuzzCorpora",
        "securityPatchRehearsal",
        "externalContractLock",
        "layoutVisitCap"
      ],
--
        "scoringAnchors": { "$ref": "#/$defs/digestOrNull" },
        "assessors": { "$ref": "#/$defs/digestOrNull" },
        "fuzzCorpora": { "$ref": "#/$defs/digestOrNull" },
        "securityPatchRehearsal": { "$ref": "#/$defs/digestOrNull" },
        "externalContractLock": { "$ref": "#/$defs/digestOrNull" },
        "layoutVisitCap": { "type": ["integer", "null"], "minimum": 1 }
      }
--
        "scoringAnchors": { "$ref": "#/$defs/sha256" },
        "assessors": { "$ref": "#/$defs/sha256" },
        "fuzzCorpora": { "$ref": "#/$defs/sha256" },
        "securityPatchRehearsal": { "$ref": "#/$defs/sha256" },
        "externalContractLock": { "$ref": "#/$defs/sha256" },
        "layoutVisitCap": { "type": "integer", "minimum": 1 }
      }
```

Two noncandidate source-inspection attempts couldn't run because this host has neither `python3` nor `node`. They created no files and supplied no qualification evidence.

```text
/nix/store/90nk33c4fkyg4x4dfk5cykqiryf2nlqq-bash-interactive-5.3p15/bin/bash: línea 1: python3: orden no encontrada
exit 127
/nix/store/90nk33c4fkyg4x4dfk5cykqiryf2nlqq-bash-interactive-5.3p15/bin/bash: línea 1: node: orden no encontrada
exit 127
```

## Recommendation

Choose Option B: stop candidate implementation until a second distinct independent human completes the confirmation procedure and the scoring anchors are frozen.

| Option | Disposition | Justification |
| :-- | :-- | :-- |
| A - treat Assessor 1 as sufficient | Rejected | The frozen policy requires two assessors, and Assessor 2 is unnamed and unavailable. |
| B - retain the assessor and scoring-anchor gates | Selected | It preserves the existing policy, prevents score-conclusion exposure, and satisfies both stop conditions. |
| C - begin candidate implementation before the second confirmation | Rejected | The ticket's first stop condition prohibits it. |

Candidate-neutral readiness work can continue only when it doesn't begin candidate implementation or create candidate score conclusions. No candidate score, candidate ranking, or candidate selection is authorized by this report.

## Spec edits required

No Stage 3 specification edit is authorized by this incomplete assessor record.

- `.constitution/tech-spec/contracts/qualification-lock.json`, `measurementPolicy.assessors`: retain the exact value `null` until both assessors accept through the procedure in this report.
- `.constitution/tech-spec/contracts/qualification-lock.json`, `measurementPolicy.scoringAnchors`: retain the exact value `null` until the accepted assessors freeze the anchors in a separate immutable artifact.
- `.constitution/tech-spec/contracts/qualification-lock.json`, `candidateImplementationReady`: retain the exact value `false` while either gate remains unresolved.
