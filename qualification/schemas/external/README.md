# External distribution contract snapshots

These files are nonauthoritative proposals for Stage 3 reconciliation. They preserve bytes from immutable upstream commits and bind local offline verifier adapters, but they do not change `.constitution/tech-spec/contracts/external-contract-lock.json` or either readiness flag.

Each `source.json` records the upstream identity and digest for its neighboring source file. Preserved Markdown source bytes use the `.source` suffix so repository prose formatting cannot change them. Each derived schema has a separate `source.json` that records its source binding and digest. The DSSE fixture key is a public test-only keyed-SHA-256 value and must never be used to sign a release artifact.
