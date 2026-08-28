# Qualification assessors

- Ticket: OXY-B008
- Status: BLOCKED / incomplete - candidate implementation is blocked. OXY-B008 stays open until two distinct confirmations are preserved. This record is the preserved first half plus the frozen procedure.
- Clock start: 2026-08-28T17:02:24Z
- Clock stop: 2026-08-28T17:08:01Z
- Scope: assessor coordination only; this report assigns no candidate score and makes no candidate selection.

## Question

| ID | Question | Status | Answer and evidence | Next bounded probe |
| :-- | :-- | :-- | :-- | :-- |
| Q1 | Are the six frozen weighted criteria enumerated? | KK | Yes. [Frozen scoring criteria](#frozen-scoring-criteria) quotes the six exact names and weights from the source-inspection probe. The fresh `$defs.scores` probe in [Preserved probe outputs](#preserved-probe-outputs) records all six required qualification-evidence properties. | Not applicable. |
| Q2 | Are the scoring anchors frozen before candidate implementation? | KU (gating) | No. `.constitution/tech-spec/contracts/qualification-lock.json` sets `measurementPolicy.scoringAnchors` to `null`, and `.constitution/prd/constraints.md` states, "The scoring anchors must be frozen before either candidate implementation begins." The host probe preserves the current `null` value. | Before candidate implementation, run an HITL anchor workshop with two accepted assessors. Expected output: one immutable artifact that defines cited-evidence anchors for each integer in the 3-5 scale for each of the six criteria, with both assessors' written acceptance and a SHA-256 digest. |
| Q3 | Has Assessor 1 accepted the role, independence rule, scale, criteria, and consensus procedure? | KU (gating) | Partly. The preserved HITL confirmation proves availability for independent scoring and written consensus and gives every requested disclosure. It does not expressly accept the integer 3-5 scale or the six criteria. Assessor 1 disclosed candidate-code or qualification-evidence authorship. Until Stage 1 approves and applies the authorship-independence policy and Stage 3 updates conforming qualification contracts, that disclosure is gating regardless of self-assessment and requires whole-candidate replacement. See [Assessor 1 independence determination](#assessor-1-independence-determination). | Obtain from Oscar Y. <oscar@ocmasesorias.com> one dated, attributed written confirmation that explicitly accepts the role, the integer 3-5 scale, all six criteria, and the consensus procedure. Then have Stage 1 approve and apply the PRD amendment in [Spec edits required](#spec-edits-required), followed by the listed Stage 3 conforming contract updates. Expected output: the quoted confirmation, a cited Stage 1 policy decision, and a completed replacement-assessor confirmation for the whole candidate. |
| Q4 | Is a distinct independent Assessor 2 named, available, and confirmed? | KU (gating) | No. No second distinct human has been named or has supplied a confirmation, availability declaration, or conflict disclosure. This is the OXY-B008 stop condition. | Obtain the exact second-assessor confirmation in [Second-assessor confirmation procedure](#second-assessor-confirmation-procedure) before candidate implementation. Expected output: a named human's written confirmation, completed conflict disclosure, availability declaration, affirmation that no candidate score conclusion was seen before independent scoring, and no unresolved candidate-code or qualification-evidence authorship disclosure. |
| Q5 | Is the no-prior-score-conclusion independence rule established? | KK | Yes, as a frozen procedure rule. [Independence rules](#independence-rules) prohibits an assessor who saw a candidate score conclusion from serving in that candidate's independent-scoring pass. Oscar's dated self-declaration records no prior exposure and that no candidate score conclusions exist. This doesn't resolve the separate authorship conflict in Q3. | Not applicable. |
| Q6 | Is the evidence-access and written-consensus procedure frozen? | KK | Yes. [Evidence access procedure](#evidence-access-procedure) freezes independent evidence access before score-conclusion disclosure. [Written consensus procedure](#written-consensus-procedure) records `consensusScore` and a nonempty `consensusRationale` for every criterion, including agreements. The re-run `$defs.score` source probe in [Preserved probe outputs](#preserved-probe-outputs) preserves those required fields. | Not applicable. |
| Q7 | Can the qualification lock's assessor field be set and candidate implementation begin? | KU (gating) | No. The current lock has `candidateImplementationReady: false`, `measurementReady: false`, and `measurementPolicy.assessors: null`. The lock also lists `scoring-anchors-and-two-assessors` among pre-implementation and gating known unknowns. A resolved measurement policy requires a SHA-256 for `assessors`. Sources: `.constitution/tech-spec/contracts/qualification-lock.json`, `.constitution/tech-spec/data-models/qualification-lock.schema.json`, `$defs.measurementPolicy` and `$defs.resolvedMeasurementPolicy`; the current lock values are preserved by the host probe. | Complete Q2, Q3, and Q4, then have Stage 3 bind completed immutable artifacts by SHA-256 in the qualification lock and validate the lock. Expected output: no assessor or scoring-anchor KU remains before candidate implementation. |

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
| `.constitution/tech-spec/data-models/qualification-evidence.schema.json`, root `assessors` | The candidate record permits exactly two assessor identities: two `prefixItems`, `items: false`, `minItems: 2`, and `maxItems: 2`. | KK |
| `.constitution/tech-spec/data-models/qualification-evidence.schema.json`, `$defs.score.properties.assessorScores` | Every criterion has exactly two un-attributed integer entries, each from 3 through 5; it has no assessor-identity field. `consensusScore`, a nonempty `consensusRationale`, and cited evidence are also required. | KK |
| `.constitution/tech-spec/data-models/qualification-evidence.schema.json`, `$defs.scores.required` | All six required criterion objects are `platformCoverage`, `upgradeMaintenance`, `performance`, `safetySecurityPrivacy`, `distribution`, and `operationalClarity`. The re-run probes preserve the complete output. The schema doesn't define assessor independence or authoring-conflict eligibility. | KK |
| `.constitution/tech-spec/data-models/selection-decision.schema.json`, root `required` | `qualificationLockDigest`, `candidateEvidence`, `eligibility`, `decisionBasis`, `outcome`, `selectedCandidate`, `calculation`, and `rationale` are required. The schema is a decision-output contract, not an assessor registry. | KK |

The assessment record must satisfy the qualification-evidence contract only after hard-gate eligibility. This report doesn't create candidate evidence, an eligibility outcome, a calculation, a score, or a selected candidate.

### Preserved confirmations

On 2026-08-28, Oscar Y. <oscar@ocmasesorias.com> supplied the following HITL confirmations in this session:

> Availability: "Available for independent scoring and written consensus sessions at any time on request; no notice requirement."

> Conflict disclosures (enumerated): organizational role — "Repository owner and project lead" (APPLIES); candidate implementation or evidence-authoring involvement — "I expect to author candidate code and/or qualification evidence myself" (APPLIES); financial or other interest in either candidate or its upstreams — none declared; prior exposure to candidate score conclusions — none (no candidate score conclusions exist).

These are attributable self-declarations. In particular, "Repository owner and project lead" is not presented as a host probe or independently verified repository-ownership fact. The local Git identity probe only establishes the configured author identity.

### Assessor register

| Assessor | Role | Availability | Conflict disclosure | Confirmation and independence |
| :-- | :-- | :-- | :-- | :-- |
| Oscar Y. <oscar@ocmasesorias.com> | Provisional Assessor 1; proposed independent evidence reviewer and written-consensus participant. | "Available for independent scoring and written consensus sessions at any time on request; no notice requirement." Source: dated self-declaration in [Preserved confirmations](#preserved-confirmations). | Self-declared organizational role: "Repository owner and project lead" (APPLIES). Self-declared candidate implementation or evidence-authoring involvement: "I expect to author candidate code and/or qualification evidence myself" (APPLIES). Financial or other interest in either candidate or its upstreams: none declared. Prior exposure: none (no candidate score conclusions exist). | The preserved text confirms availability and disclosures. It establishes neither an explicit acceptance of the scale and criteria nor that the authoring disclosure permits independent scoring. Both remain Q3 gates. |
| BLOCKED | No role can be assigned until a distinct human is named and accepts it. | No availability declaration exists. | No identity or conflict disclosure exists. | No confirmation exists. This is the stop condition for candidate implementation. |

### Independence rules

1. Each assessor must record their own integer score for every criterion from cited evidence before viewing the other assessor's score conclusion.
2. An assessor who has seen a candidate score conclusion before recording their own independent score can't serve for that candidate's independent-scoring pass.
3. Candidate implementation, candidate evidence, and candidate score conclusions must remain separate from this coordination record.
4. Each assessor must disclose repository ownership, employment or organizational role, financial or other interest in either candidate or its upstreams, candidate implementation or evidence-authoring involvement, and any prior exposure to candidate score conclusions.
5. Until Stage 1 approves and applies an authorship-independence policy and Stage 3 updates the qualification contracts to conform, any assessor's disclosed authorship of candidate code or qualification evidence for that candidate is a gating conflict. It remains gating regardless of either assessor's self-assessment, and the assessor can't enter that candidate's independent-scoring pass.
6. Under the current qualification-evidence schema, the only representable mitigation for an authorship conflict is whole-candidate assessor replacement: the replacement completes the full confirmation procedure and enters the candidate's two-assessor record before seeing any candidate score conclusion.
7. Per-criterion recusal with a third assessor is blocked until Stage 3 changes the schema as specified in [Spec edits required](#spec-edits-required).

### Assessor 1 independence determination

The governing PRD text says: "After hard-gate eligibility, two assessors independently assign an integer score from 3 through 5 to each criterion from cited evidence." The `OD-01: Rendering substrate` text says: "After hard-gate eligibility, two frozen assessors independently assign an integer score of 3, 4, or 5 to each criterion from cited KK evidence." Source: `.constitution/prd/constraints.md`, `Substrate selection policy`; `.constitution/reports/2026-08-09-open-decisions.md`, `OD-01: Rendering substrate`; and [Independence rules](#independence-rules).

The re-run schema probe in [Preserved probe outputs](#preserved-probe-outputs) shows that the candidate record can name exactly two assessors and that each criterion stores exactly two un-attributed integer assessor scores. The schema doesn't state whether authoring candidate code or qualification evidence prevents independent scoring.

Determination: KU (gating). Oscar disclosed that candidate code and/or qualification evidence may be authored personally. Until Stage 1 approves and applies an authorship-independence policy and Stage 3 updates the qualification contracts to conform, the same rule applies to both assessors: any candidate-code or qualification-evidence authorship disclosure remains gating regardless of self-assessment. The disclosing assessor can't independently score that candidate. No disclosure is waived or softened.

The current schema cannot encode a third identity for a criterion-specific replacement, or connect either criterion score to a named assessor. Therefore, whole-candidate replacement with a confirmed non-authoring assessor is the only currently representable mitigation. This report recommends that mitigation in [Recommendation](#recommendation).

### Schema representability and recusal

KK: The re-run schema probe in [Preserved probe outputs](#preserved-probe-outputs) proves that `assessors` is an exactly-two-item tuple and `assessorScores` is an exactly-two-item integer tuple with no identity field. A per-criterion recusal that substitutes a third person cannot be encoded in the current contract.

KU (gating): Per-criterion recusal is a possible future policy option only. It remains blocked until Stage 1 approves that policy and Stage 3 applies the named schema changes, including the exact identity and recusal fields, in [Spec edits required](#spec-edits-required). Until then, replace the conflicted assessor for the entire candidate.

### Evidence access procedure

1. The coordinator gives both accepted assessors the same immutable qualification lock and the same cited candidate evidence set at the same time.
2. Assessors can read the repository sources at `.constitution/prd/`, `.constitution/reports/`, `.constitution/spikes/`, `.constitution/tech-spec/contracts/`, and `.constitution/tech-spec/data-models/` and the qualification evidence at `qualification/evidence/`, `qualification/golden/`, `qualification/fixtures/`, and `qualification/schemas/`.
3. Before independent records are complete, the coordinator withholds the other assessor's score conclusion and any discussion that reveals it.
4. Each independent record identifies the evidence path and SHA-256, criterion, integer score, and rationale without assigning a score in this coordination report.
5. The coordinator releases both independent records together after both records are complete. The assessors then create the required consensus record for every criterion; disagreement resolution starts only where the independent scores differ.

### Written consensus procedure

1. Confirm hard-gate eligibility before any scoring discussion. A candidate with a failed or unresolved gating P0 item is ineligible for scoring.
2. Each assessor records an independent integer score from 3 through 5 for each of the six criteria from cited evidence.
3. For every criterion, both assessors create one written consensus record with the criterion, both independent records, cited evidence, an integer `consensusScore`, and a nonempty `consensusRationale`.
4. For an agreement, `consensusScore` equals the agreed integer. `consensusRationale` is a joint statement that cites the evidence both assessors relied on; no disagreement-resolution discussion is required.
5. For a disagreement, both assessors jointly resolve it in writing. The record preserves both independent scores, identifies the cited evidence, states the resolved `consensusScore`, and explains the resolution in `consensusRationale`.
6. The consensus record doesn't replace either independent record. It preserves the disagreement when one occurred and explains its resolution.
7. The completed record must conform to `.constitution/tech-spec/data-models/qualification-evidence.schema.json`, including `assessors`, `scores`, and `weightedTotal`, before any selection-decision record is created.

### Second-assessor confirmation procedure

Before candidate implementation, the second distinct human must provide a written confirmation containing all of the following statements and disclosures.

1. State their name and a contact identity that is distinct from Oscar Y. <oscar@ocmasesorias.com>.
2. Accept the role of Assessor 2, including independent evidence review and written consensus for every criterion.
3. Declare an availability window that covers independent scoring and the later written-consensus step.
4. Disclose repository ownership or maintainer status, employer or organizational role, financial or other interest in either candidate or upstream, candidate implementation or evidence-authoring involvement, and any prior exposure to candidate score conclusions.
5. Confirm participation, the integer 3-5 scale, the six exact criteria in [Frozen scoring criteria](#frozen-scoring-criteria), and the consensus procedure in this report, including the agreement and disagreement requirements.
6. Confirm that they haven't seen any candidate score conclusion before recording independent scores and that they won't see another assessor's conclusion until their own record is complete.
7. A candidate-code or qualification-evidence authorship disclosure remains a gating conflict regardless of the assessor's self-assessment. The assessor can't independently score that candidate; the coordinator must name a whole-candidate replacement and repeat this procedure before evidence access. Other disclosures don't silently waive independence and must be referred to the coordinator and the approved governance policy. Per-criterion recusal with a third assessor is blocked by the current schema.

### Preserved probe outputs

The following identity probe was run on this host. It establishes the configured local Git author identity only. It doesn't identify a person who accepted an assessor role, establish repository ownership, or establish independence.

```text
command: git var GIT_AUTHOR_IDENT
Oscar Y. <oscar@ocmasesorias.com> 1787935626 -0400
command: date -u +%Y-%m-%dT%H:%M:%SZ
2026-08-28T16:47:06Z
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

command: grep -E "\"(candidateImplementationReady|measurementReady|scoringAnchors|assessors)\"|scoring-anchors-and-two-assessors" .constitution/tech-spec/contracts/qualification-lock.json
  "candidateImplementationReady": false,
  "measurementReady": false,
    "scoringAnchors": null,
    "assessors": null,
    "scoring-anchors-and-two-assessors",
    "scoring-anchors-and-two-assessors",
```

The following schema inspection probes were re-run on this host during the round-2 repair. The first block is an excerpt selected by `grep -A 48`; it is complete output for that command but not evidence of every `$defs.scores` member. The following exact `jq` probe separately preserves all six required `$defs.scores` properties.

```text
command: grep -A 48 '"score":' .constitution/tech-spec/data-models/qualification-evidence.schema.json
    "score": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "weight",
        "assessorScores",
        "consensusScore",
        "consensusRationale",
        "evidence"
      ],
      "properties": {
        "weight": { "type": "integer" },
        "assessorScores": {
          "type": "array",
          "prefixItems": [
            { "type": "integer", "minimum": 3, "maximum": 5 },
            { "type": "integer", "minimum": 3, "maximum": 5 }
          ],
          "items": false,
          "minItems": 2,
          "maxItems": 2
        },
        "consensusScore": { "type": "integer", "minimum": 3, "maximum": 5 },
        "consensusRationale": { "type": "string", "minLength": 1 },
        "evidence": {
          "type": "array",
          "items": { "$ref": "#/$defs/evidence" },
          "minItems": 1,
          "uniqueItems": true
        }
      }
    },
    "scores": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "platformCoverage",
        "upgradeMaintenance",
        "performance",
        "safetySecurityPrivacy",
        "distribution",
        "operationalClarity"
      ],
      "properties": {
        "platformCoverage": {
          "allOf": [
            { "$ref": "#/$defs/score" },
            { "type": "object", "properties": { "weight": { "const": 30 } } }
          ]

command: jq -r '."$defs".scores.required[]' .constitution/tech-spec/data-models/qualification-evidence.schema.json
platformCoverage
upgradeMaintenance
performance
safetySecurityPrivacy
distribution
operationalClarity

command: jq '{assessors: .properties.assessors, assessorScores: ."$defs".score.properties.assessorScores, scoresRequired: ."$defs".scores.required}' .constitution/tech-spec/data-models/qualification-evidence.schema.json
{
  "assessors": {
    "type": "array",
    "prefixItems": [
      {
        "$ref": "#/$defs/assessor"
      },
      {
        "$ref": "#/$defs/assessor"
      }
    ],
    "items": false,
    "minItems": 2,
    "maxItems": 2,
    "uniqueItems": true
  },
  "assessorScores": {
    "type": "array",
    "prefixItems": [
      {
        "type": "integer",
        "minimum": 3,
        "maximum": 5
      },
      {
        "type": "integer",
        "minimum": 3,
        "maximum": 5
      }
    ],
    "items": false,
    "minItems": 2,
    "maxItems": 2
  },
  "scoresRequired": [
    "platformCoverage",
    "upgradeMaintenance",
    "performance",
    "safetySecurityPrivacy",
    "distribution",
    "operationalClarity"
  ]
}

command: grep -A 12 "After hard-gate eligibility" .constitution/prd/constraints.md
After hard-gate eligibility, two assessors independently assign an integer score from 3 through 5 to each criterion from cited evidence. They must record one consensus score for every disagreement. Multiply each consensus score by its weight and divide by 5 to produce a 100-point result.

| Criterion                                       | Weight |
| :---------------------------------------------- | -----: |
| Demonstrated P0 platform coverage               |     30 |
| Two-transition upgrade-maintenance cost         |     20 |
| Performance, startup, memory, and artifact size |     15 |
| Boundary safety, security, and privacy          |     15 |
| Distribution, licensing, and provenance         |     10 |
| Testing, diagnostics, and operational clarity   |     10 |

The scoring anchors must be frozen before either candidate implementation begins. CAP-SUB-002 through CAP-SUB-004 govern eligibility, zero-candidate and one-candidate outcomes, weighted selection, and the maintenance-first tie-break.

command: grep -A 8 "After hard-gate eligibility" .constitution/reports/2026-08-09-open-decisions.md
- After hard-gate eligibility, two frozen assessors independently assign an integer score of 3, 4, or 5 to each criterion from cited KK evidence. They then record one written consensus score for every disagreement.

Both candidates can fail. A candidate with a failed or unresolved gating P0 item is ineligible for scoring; an implementation plan doesn't close the gate. The selection rule defines outcomes for zero, one, and two eligible candidates. A cross-family per-platform hybrid requires a separate architecture decision.
```

## Recommendation

Choose Option B for the ticket state and Option C for the authorship-conflict policy.

| Option | Disposition | Justification |
| :-- | :-- | :-- |
| A - treat Assessor 1 as sufficient and authoring as compatible with independent scoring | Rejected | The frozen policy requires two assessors, Assessor 2 is unnamed and unavailable, and the governing texts don't resolve the authoring conflict. |
| B - retain the assessor and scoring-anchor gates | Selected | It preserves the existing policy and both stop conditions. OXY-B008 remains open until two distinct confirmations are preserved, and Q3 must be resolved before evidence access. |
| C - use two assessors who don't author candidate code or qualification evidence for the candidate | Recommended for Stage 1 approval and subsequent Stage 3 conformance | It removes the declared authoring conflict instead of assuming it is harmless. Until Stage 1 applies the policy and Stage 3 updates conforming contracts, authorship remains a gating disclosure for either assessor. |

Candidate-neutral readiness work can continue only when it doesn't begin candidate implementation or create candidate score conclusions. No candidate score, candidate ranking, or candidate selection is authorized by this report.

## Spec edits required

No active-specification edit is authorized while Q2, Q3, or Q4 remains gating. Stage 1 must first approve and apply the PRD amendment below. Only after that approval and application may Stage 3 update the qualification contracts to conform; Stage 3 must not apply the PRD amendment.

- Stage 1 - `.constitution/prd/constraints.md`, `Substrate selection policy`: insert this exact sentence after the independent-scoring sentence: "A person who authors candidate implementation or qualification evidence for a candidate must not serve as an independent scorer for that candidate."
- Stage 3 - `.constitution/tech-spec/contracts/qualification-lock.json`, `measurementPolicy.assessors`: retain the exact value `null` until two named, available, confirmed assessors satisfy the Stage 1-approved authorship-independence policy; then replace `null` with the SHA-256 digest of their immutable assessor-declaration artifact. No digest is proposed because no such complete artifact exists.
- Stage 3 - `.constitution/tech-spec/contracts/qualification-lock.json`, `measurementPolicy.scoringAnchors`: retain the exact value `null` until the accepted assessors freeze the anchors in a separate immutable artifact.
- Stage 3 - `.constitution/tech-spec/contracts/qualification-lock.json`, `candidateImplementationReady`: retain the exact value `false` while any assessor, authorship-independence, or scoring-anchor gate remains unresolved.
- Stage 3, blocked future option only - `.constitution/tech-spec/data-models/qualification-evidence.schema.json`, root `assessors`: replace the current exactly-two tuple with the exact value `{ "type": "array", "items": { "$ref": "#/$defs/assessor" }, "minItems": 2, "uniqueItems": true }`; remove `prefixItems`, `items: false`, and `maxItems: 2` so a third frozen replacement identity is encodable.
- Stage 3, blocked future option only - `.constitution/tech-spec/data-models/qualification-evidence.schema.json`, add `$defs.assessorScore` with the exact value `{ "type": "object", "additionalProperties": false, "required": ["assessorId", "score"], "properties": { "assessorId": { "type": "string", "minLength": 1 }, "score": { "type": "integer", "minimum": 3, "maximum": 5 } } }`; replace `$defs.score.properties.assessorScores` with the exact value `{ "type": "array", "items": { "$ref": "#/$defs/assessorScore" }, "minItems": 2, "maxItems": 2 }`; append the exact string `"recusedAssessorIds"` to `$defs.score.required`; and add `$defs.score.properties.recusedAssessorIds` with the exact value `{ "type": "array", "items": { "type": "string", "minLength": 1 }, "uniqueItems": true }`. Stage 3 must enforce that score and recusal IDs name entries in root `assessors`, the two score IDs differ, and no score ID is recused for that criterion.

The future per-criterion option remains blocked until Stage 1 approves it and Stage 3 applies and validates both listed schema changes. Until then, whole-candidate assessor replacement is mandatory.
