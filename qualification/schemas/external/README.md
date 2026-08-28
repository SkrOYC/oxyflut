# External distribution contract snapshots

These files are nonauthoritative proposals for Stage 3 reconciliation. They preserve bytes from immutable upstream commits and bind local offline verifier adapters, but they do not change `.constitution/tech-spec/contracts/external-contract-lock.json` or either readiness flag.

Each `source.json` records the upstream identity, license expression, license source, and digest for its neighboring source file. Preserved Markdown source bytes use the `.source` suffix so repository prose formatting cannot change them. Each derived schema has a separate `source.json` that records its source binding and digest. The DSSE fixture key is a public test-only keyed-SHA-256 value and must never be used to sign a release artifact.

The license-compatibility STOP condition is resolved: the snapshots preserve specification bytes for local verification. They aren't redistributed products or release payloads. Their source records retain the attribution and license information needed to review the fixtures without treating them as distributed artifacts.

The derived SLSA `DigestSet` accepts hexadecimal values only; this intentional local narrowing is routed to OXY-D001.

A staged external-contract proposal may satisfy the typed `externalContractLock` reference only while the `external-distribution-schema-snapshots-and-verifiers` KU independently gates; OXY-D001 must adopt or replace the proposal before clearing that KU.
