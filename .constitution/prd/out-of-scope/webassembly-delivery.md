# WebAssembly delivery

- **Context:** The product can later target browser-hosted or other WebAssembly environments.
- **Decision:** deferred.
- **Reason:** WebAssembly delivery has different window, input, accessibility, rendering, packaging, and lifecycle constraints from Tier 1 desktop environments.
- **Consequences:** Stage 2 doesn't include WebAssembly boundaries or flows. Delivery requires a product-requirements Evolution pass for CAP-PLT-003.
