# Exported telemetry

- **Context:** Diagnostic records can be sent beyond the machine or another declared trust boundary.
- **Decision:** deferred.
- **Reason:** P0 requires privacy-safe local diagnostics, not a remote exporter. Export creates separate consent, retention, identity, transport, and operational requirements.
- **Consequences:** Downstream stages can define exporter-independent local records but must not implement an exporter without a product-requirements Evolution pass and privacy review.
