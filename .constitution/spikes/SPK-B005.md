# Spike report: OXY-B005 common-case layout visit cap

## Time box

- **Budget:** 1 focused day
- **Clock start / stop:** Fill during execution.

## Question

- **Decision this spike must produce:** What candidate-neutral ordinary-layout corpus, visit-counting rule, and finite per-node cap can be frozen before candidate implementation without hiding intrinsic measurement or text work?

## Context and objective

- **Triggering upstream file or section:** `.constitution/prd/constraints.md` common-case node-visit KU and `.constitution/tech-spec/contracts/qualification-lock.json` → `measurementPolicy.layoutVisitCap`
- **Target:** Recommend the exact corpus and numeric cap or prove that a further bounded prototype is required before Stage 3 can freeze them.
- **Archetype / surface:** Library/SDK layout policy under System/Native frame constraints.

## Codebase baseline

- **State today:** The public contract reports `node_visits`, CAP-LAY-001 requires bounded propagation, and CON-PERF-001 caps aggregate application-owned layout and paint submission at 2.0 ms, but no corpus or numeric cap exists.
- **Discovered constraints:** Ordinary policies must declare finite caps; intrinsic measurement and text work are distinct; shallow scenes cannot be the sole evidence.

## Options and trade-offs

- Option A: Freeze per-policy algebraic caps derived from child-count and depth invariants, then validate them over a corpus containing deep, wide, nested, virtualized, reordered, and failure cases.
- Option B: Freeze one global cap for all ordinary policies; easier to meter but likely either too weak for bounded policies or too strict for legitimate composition.
- Option C: Require one additional nonproduction model prototype before selecting a number; delays the gate but avoids an intuition-based cap.

## Recommendation

- **Chosen option:** Pending execution; prefer A if the counting model produces finite testable bounds across the full corpus, otherwise choose C. Reject B unless evidence proves it tighter and clearer.
- **Why it fits:** Per-policy finite bounds align with the existing PRD wording and expose accidental repeated visits without conflating different layout mechanisms.
- **Rejected options:** Reject timing-only acceptance, average visit counts, unbounded intrinsic recursion, and a number selected only to fit shallow reference scenes.

## Downstream impact

- **ADRs to write or update:** `ADR-0003-public-rust-compatibility.md` only if the measurement surface changes; otherwise record the cap in Stage 3 stack/lock contracts.
- **Tickets unblocked in `tasks/active/`:** `OXY-D001`; complete capability-baseline authoring follows the Stage 3 reconciliation.
- **Tickets to add or split:** Add a separate bounded model-prototype spike if option C wins.
- **Spec edits required:** Stage 3 must freeze the corpus digest, counting rules, and cap before setting `candidateImplementationReady`.
