# Epic D: Readiness reconciliation

Close this Stage 4 iteration without crossing an unresolved upstream decision. This epic produces a reconciliation report and routes required changes back to Stage 3.

#### OXY-D001 Reconcile pre-implementation readiness evidence

- **Type:** Chore
- **Effort:** 2
- **Dependencies:** OXY-A007, OXY-B001, OXY-B002, OXY-B003, OXY-B004, OXY-B005, OXY-B006, OXY-B007, OXY-B008, OXY-C005
- **Category:** Docs
- **Scope (In-Scope Files):**
  - `.constitution/reports/pre-implementation-readiness.md`
- **Scope (Out-of-Scope Files):**
  - `.constitution/tech-spec/` (name the required Stage 3 pass; don't perform it in this ticket)
  - `.constitution/tasks/` (the next Stage 4 version follows upstream reconciliation)
  - Candidate source trees
  - Qualification measurements
- **Verification Command:** `prettier --prose-wrap never --check '.constitution/**/*.md'`
- **Expected Success Output:** `exit 0` and a report that classifies every pre-implementation KU as resolved KK, retained KU, or blocked external input
- **STOP Conditions:**
  - STOP if any result is inferred from a plan, implementation intention, inaccessible hardware, or candidate-internal counter.
  - STOP after naming the exact Stage 3 revisions and remaining user or external inputs; don't set `candidateImplementationReady` from Stage 4.
- **Description:** Consolidate the contract-validator results, six spike recommendations, hardware-access register, assessor confirmations, staged tool and external-contract manifests, baseline tooling, measurement templates, environment tooling, and read-only lock report. Produce an exact Stage 3 reconciliation checklist and the conditions for the next Stage 4 minor release. The checklist must separately name the approved 52-capability baseline, reference application, scenes, interaction scripts, fonts, assets, window matrix, cache states, release flags, scoring anchors, assessor assignments, reference-environment captures, and authoritative resolved-tool lock.
- **Acceptance:**
  - **Mode:** invariant
  - **Evidence:**

```text
Invariants:
- Every entry in preImplementationKnownUnknowns has one cited result and one owner.
- KK means verified evidence exists; KU means a named unanswered question remains; blocked external input is not relabeled as technical completion.
- The report contains no candidate implementation ticket, measurement result, score, selection, or production plan.
- The report states that this Stage 4 iteration cannot set candidateImplementationReady and identifies every missing approved or captured lock input.
- The next action is either an exact Stage 3 reconciliation pass or an explicit blocked state with required user/external input.
Checker: prettier --prose-wrap never --check '.constitution/**/*.md'
```
