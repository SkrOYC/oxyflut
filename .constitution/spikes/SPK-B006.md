# Spike report: OXY-B006 shared security patch and fuzz corpora

## Time box

- **Budget:** 1 focused day
- **Clock start / stop:** Fill during execution.

## Question

- **Decision this spike must produce:** Which shared upstream security patch and frozen, attributable corpus set can exercise both candidates symmetrically before implementation begins?

## Context and objective

- **Triggering upstream file or section:** `.constitution/prd/constraints.md` CON-SEC-001 through CON-SEC-003 and `.constitution/tech-spec/contracts/qualification-lock.json` measurement-policy KUs
- **Target:** Select one applicable real patch or a predeclared synthetic patch, expected tests, target ingresses, corpus sources, licenses, payload caps, and digest procedure.
- **Archetype / surface:** Library/SDK and System/Native unsafe, parser, artifact, and callback boundaries.

## Codebase baseline

- **State today:** Stage 3 pins cargo-fuzz and the ingress schema, while the implemented-ingress inventory, seed corpora, instrumentation details, and shared patch rehearsal input remain open.
- **Discovered constraints:** Each implemented untrusted parser needs 24 CPU-hours; the patch must affect code consumed by both candidates; unrelated upstream changes aren't admissible.

## Options and trade-offs

- Option A: Select a disclosed renderer, text, image, or shared dependency patch applicable to all three frozen Flutter lines and both candidate consumption paths.
- Option B: Predeclare a minimal synthetic validation patch and exact expected tests when no suitable disclosed patch exists.
- Option C: Delay selection until candidate ingress inventories exist; reduces premature corpus work but violates the pre-implementation freeze requirement.

## Recommendation

- **Chosen option:** Pending execution; choose A when applicability and redistribution evidence are complete, otherwise choose B. Reject C.
- **Why it fits:** It freezes comparable patchability and fuzz inputs without assuming either candidate's eventual implementation surface.
- **Rejected options:** Reject candidate-specific patches, unlicensed corpora, mutable rolling corpora, and patches whose expected tests are unknown.

## Downstream impact

- **ADRs to write or update:** None unless patch distribution or corpus provenance requires an additional durable contract family.
- **Tickets unblocked in `tasks/active/`:** `OXY-D001`; fuzz-target implementation follows candidate implementation readiness.
- **Tickets to add or split:** Add ingress-specific corpus work after actual candidate inventories reveal candidate-specific additions.
- **Spec edits required:** Stage 3 must freeze selected patch identity, expected tests, corpus registry digest, and instrumentation before setting `candidateImplementationReady`.
