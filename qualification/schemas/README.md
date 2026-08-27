# Staged qualification schemas

This directory contains local schemas that support qualification tooling but aren't authoritative Stage 3 contracts.

`sample-validity.schema.json` is a staged proposal for the bytes that `qualification-lock.json#measurementPolicy.sampleValidityRules` will digest-bind. Its `authority` value is `staged-proposal`. Stage 3 must adopt, replace, or remove this schema before the lock can claim readiness.
