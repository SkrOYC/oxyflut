# Parallel candidate qualification

- **Context:** Both substrate candidates could enter the frozen suite at the same time.
- **Decision:** deferred.
- **Reason:** The qualification sequence evaluates the integrated candidate first and creates focused-candidate work only after the integrated candidate fails hard-gate eligibility in the first Tier 1 environment.
- **Consequences:** Downstream stages must not require simultaneous candidate qualification. This deferral reopens automatically when the integrated candidate fails hard-gate eligibility in the first Tier 1 environment.
