# Parallel candidate qualification

- **Context:** Both substrate candidates could enter the frozen suite at the same time.
- **Decision:** deferred.
- **Reason:** The declared qualification sequence evaluates the integrated candidate first; its hard-gate eligibility failure in the first Tier 1 environment triggers sequential focused-candidate qualification under that sequence, not parallel qualification.
- **Consequences:** Downstream stages must not require parallel candidate qualification. Reintroducing it requires a Stage 1 Evolution pass.
