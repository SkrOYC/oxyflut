# Staged qualification schemas

This directory contains local schemas that support qualification tooling but aren't authoritative Stage 3 contracts.

The following gaps are routed to OXY-D001:

- `raw-measurement.schema.json` lacks the `$schema` property that sibling schemas declare, so raw-measurement instances cannot self-declare their schema.
- Stage 3 doesn't define `qualification-lock.json#measurementPolicy.sampleValidityRules`. `sample-validity.schema.json` proposes the digest-bound bytes and declares `authority: staged-proposal`.
- The staged sample-validity record intentionally excludes the PRD launch and per-launch observation budgets. OXY-D001 must type those budgets in the measurement-harness contract.

Stage 3 must resolve these gaps before the lock can claim readiness.
