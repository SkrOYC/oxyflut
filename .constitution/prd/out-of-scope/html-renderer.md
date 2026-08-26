# HTML renderer

- **Context:** Browser delivery can use an HTML or Document Object Model rendering surface.
- **Decision:** rejected.
- **Reason:** The product focuses on one retained graphics and system-integration model. An HTML renderer would introduce a second layout, input, accessibility, and rendering contract.
- **Consequences:** Downstream stages must not add an HTML or Document Object Model renderer without a product-requirements Evolution pass.
