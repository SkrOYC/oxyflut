# Secondary application runtime

- **Context:** A rendering foundation can include a separate application runtime and package ecosystem.
- **Decision:** rejected.
- **Reason:** Production Oxyflut applications require one application model and must not start or execute application code through a secondary application runtime.
- **Consequences:** Downstream stages must not add secondary-runtime application or package execution without a product-requirements Evolution pass.
