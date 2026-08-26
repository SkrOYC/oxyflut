# Plugin and custom-shader systems

- **Context:** A framework can expose executable plugins or application-supplied shader ingestion.
- **Decision:** deferred.
- **Reason:** Neither capability is required for the first production release, and each adds an executable or parser trust boundary.
- **Consequences:** Downstream stages must inventory only implemented ingresses. They must not add a plugin system or custom-shader API without a product-requirements Evolution pass.
