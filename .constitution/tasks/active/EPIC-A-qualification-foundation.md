# Epic A: Qualification foundation

Build the repository and validation foundation permitted before candidate implementation. This epic doesn't implement either rendering substrate or any P0 product capability.

#### OXY-A001 Scaffold the qualification workspace

- **Type:** Chore
- **Effort:** 3
- **Dependencies:** None
- **Category:** DX
- **Scope (In-Scope Files):**
  - `Cargo.toml`
  - `Cargo.lock`
  - `rust-toolchain.toml`
  - `crates/*/Cargo.toml`
  - `crates/*/src/lib.rs`
  - `crates/oxyflut-qualification/src/{schema,identifiers,readiness,evidence,hash,baseline,measurement,environment}.rs`
  - `xtask/Cargo.toml`
  - `xtask/src/main.rs`
  - `xtask/src/commands/*.rs`
  - `xtask/src/commands/environment/mod.rs`
  - `xtask/src/{evidence,toolchain}.rs`
  - `qualification/{fixtures,golden,probes,schemas}/`
  - `fuzz/`
- **Scope (Out-of-Scope Files):**
  - `crates/oxyflut-substrate-impeller/src/` beyond an empty package skeleton
  - `crates/oxyflut-substrate-engine/src/` beyond an empty package skeleton
  - `native/engine-bridge/` (don't implement the integrated candidate)
  - `platform/` (don't implement operating-environment integration)
- **Verification Command:** `cargo +1.98.0 fmt --all --check`
- **Expected Success Output:** `exit 0`
- **STOP Conditions:**
  - STOP if a package requires behavior or an API not already present in `.constitution/tech-spec/`; don't invent it.
  - STOP if scaffolding would compile or link candidate code while `candidateImplementationReady` is false.
- **Description:** Use Cargo CLI commands to create the edition-2024 workspace and package skeletons specified by the target repository structure. Pin Rust 1.98.0, resolver version 3, exact dependencies, warnings policy, and one workspace lockfile. Create the `xtask` command dispatcher, registered root `evidence` and `toolchain` modules, and compile-safe placeholder modules for every command in `guidelines.md`. Use only `commands/environment/mod.rs` for the environment command; don't create a competing `commands/environment.rs` file. Create and register compile-safe `oxyflut-qualification` placeholder modules for every later qualification ticket. Later tickets replace only their owned placeholder modules and don't edit `main.rs` or `lib.rs`. Other package bodies remain empty except for documentation and compile-safe placeholders.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Assertions:
- Cargo metadata reports every Stage 3 workspace member exactly once.
- Every package uses edition 2024 and inherits the workspace Rust version and lint policy.
- The workspace builds no candidate, platform, or product-capability behavior.
- The `xtask` dispatcher recognizes every qualification command name and routes each unimplemented command to its named placeholder without claiming success.
- Cargo.lock contains only dependencies allowed by stack.md.
Command: cargo +1.98.0 test --workspace --all-features
```

##### OXY-A001 Deviations & Justifications

- **Touched Files:** `devenv.*`, `.envrc`, `.gitignore`
- **Justification:** The host lacks a C toolchain; devenv provides the reproducible, hashable tool set that OXY-A008 will record.
- **Touched Files:** `native/engine-bridge/README.md`, `platform/{macos,windows,linux}/README.md`
- **Justification:** The required target structure specifies documented empty placeholder directories; they contain no candidate or operating-environment implementation.
- **Touched Files:** `.constitution/tasks/active/EPIC-A-qualification-foundation.md`
- **Justification:** The execution skill requires deviations to be recorded in the ticket block.

#### OXY-A002 Implement offline JSON Schema and instance validation

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-A001
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `xtask/src/contracts/schema.rs`
  - `xtask/src/contracts/{traceability,registries,readiness,digests,native}.rs`
  - `xtask/src/contracts/mod.rs`
  - `xtask/src/commands/contracts.rs`
  - `crates/oxyflut-qualification/src/schema.rs`
  - `crates/oxyflut-qualification/src/hash.rs`
  - `qualification/fixtures/contracts/`
- **Scope (Out-of-Scope Files):**
  - `.constitution/tech-spec/data-models/*.json` (binding inputs; don't redesign)
  - `.constitution/tech-spec/contracts/*.json` (binding instances; don't weaken)
  - Candidate and platform crates
- **Verification Command:** `cargo +1.98.0 test --workspace --all-features schema && cargo +1.98.0 test --workspace --all-features hash`
- **Expected Success Output:** `exit 0` with every local schema compiled and every committed instance validated without network access
- **STOP Conditions:**
  - STOP if a schema requires remote resolution; route the missing snapshot to OXY-C001 instead of enabling network resolution.
  - STOP if validation requires changing a schema's meaning; trigger a Stage 3 correction.
- **Description:** Replace the OXY-A001 contracts command and shared hash placeholders with the offline JSON Schema 2020-12 entry point and the pinned streaming SHA-256 primitive. Create and register compile-safe placeholder validator modules for OXY-A003, OXY-A004, and OXY-A005 so their code is compiled before OXY-A007 aggregates execution. Implement schema compilation, local reference resolution, instance discovery, deterministic error ordering, shared file and byte hashing, and positive and negative fixtures for every durable Stage 3 shape.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Assertions:
- Every .schema.json file compiles under the pinned validator.
- Every committed contract instance validates against its declared local schema.
- Eligible qualification-evidence fixtures accept `pass` with a null absence binding and `not-applicable-kk` with a typed platform-baseline and absent-event binding, while rejecting `fail`, `gating-ku`, and missing or contradictory bindings.
- Streaming file and byte hashing returns the published SHA-256 vectors without loading whole evidence files into memory.
- Network and undeclared schema resolution fail closed.
- Invalid type, required-field, enum, additional-property, and conditional fixtures fail with stable paths.
- Superseded pre-evidence schema identities have explicit rejection and supersession fixtures; migrations preserve source bytes after durable evidence exists.
Command: cargo +1.98.0 test --workspace --all-features schema && cargo +1.98.0 test --workspace --all-features hash
```

#### OXY-A003 Validate exact upstream sets and registries

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-A002
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `xtask/src/contracts/traceability.rs`
  - `xtask/src/contracts/registries.rs`
  - `crates/oxyflut-qualification/src/identifiers.rs`
  - `qualification/fixtures/contracts/traceability/`
- **Scope (Out-of-Scope Files):**
  - `.constitution/prd/` (binding input)
  - `.constitution/architecture/` (binding input)
  - `.constitution/tech-spec/contracts/capability-traceability.json` (don't add exceptions)
- **Verification Command:** `cargo +1.98.0 test -p xtask contracts::traceability`
- **Expected Success Output:** `exit 0` with exact PRD, architecture, traceability, diagnostic, and evidence ID sets
- **STOP Conditions:**
  - STOP if the three 52-capability sets differ; report the owning upstream stage rather than normalizing the mismatch.
  - STOP if a symbol, contract path, or contract-test identifier doesn't resolve.
- **Description:** Validate the exact 52 P0 IDs across PRD tables, architecture flow filenames, traceability, and the required capability-to-physical-contract edge matrix; the exact 27 constraint IDs; equality between the active specification version and every capability baseline, platform baseline, or traceability instance; approved-versus-synthetic baseline provenance; digest-bound approval evidence and typed approved-baseline lock references; file-qualified contract symbols and contract-test identifiers; diagnostic names and fields; the closed machine-local diagnostic destination set; candidate names; Tier 1 environment identifiers; unique canonical artifact paths, link targets, absent-event IDs, and raw-sample keys. Resolve and hash every evidence reference behind a KK minimum-version, protocol, input method editor, timing, allocation, recovery, accessibility, or absent-event claim with the OXY-A002 primitive. Admit an eligible `not-applicable-kk` capability result only when its typed binding resolves the exact platform-baseline path, digest, schema version, active specification version, absent-event ID, gate ID, event ID, candidate, and parent environment. Admit an aggregate constraint result only when the absent-event entry covers the evidence candidate and all four Tier 1 environments. The same result without matching immutable absence proof is invalid. Dereference each platform accessibility map, verify its path digest, environment and candidate identity, required semantics categories, aggregate status, and nested mapping and action statuses. Resolve every hardlink to a regular-file entry and every symlink within the artifact root without dereferencing it. For diagnostics, resolve each event's registry version and validate the registered event and field privacy classes, field kind, range, closed integer values, local sink admission, and bounded acknowledgement contract.
- **Acceptance:**
  - **Mode:** invariant
  - **Evidence:**

```text
Invariant: No identifier, file-qualified contract binding, canonical artifact path, canonical link target, absent-event ID, or raw-sample key can be missing, duplicated, renamed, or added in one downstream set without the authoritative upstream set changing first. Every baseline and traceability instance names the active specification version. A typed approved-baseline lock reference resolves the same baseline path, digest, schema version, approved provenance, and approval-evidence path and digest; synthetic provenance cannot satisfy readiness. An eligible gate is `pass` or `not-applicable-kk`. A pass has a null absence binding. A not-applicable result resolves its typed baseline reference and absent-event ID to the same active platform baseline, matching gate, candidate, event, and parent environment; an aggregate constraint requires all four environments. Each declared physical contract has exactly one binding with one or more symbols that resolve inside that file. Every nested KK platform claim resolves at least one immutable evidence path and digest. A KK platform accessibility reference resolves to the matching environment and candidate, has the declared digest, contains every required semantics category, has no nested KU, and binds each indexed reverse action to the live node's immutable text-layout generation. Hardlinks resolve to regular-file entries with equal size and digest; symlinks remain inside the artifact root; regular files carry no link target. Diagnostic values must resolve to the registry's event class and match the field class, kind, bounds, and closed integer values. Diagnostic sink admission accepts only the closed machine-local destinations with a nonzero queue bound; remote, undeclared, and unbounded destinations fail before record delivery. Event files cannot override registry privacy metadata.
Checker: cargo +1.98.0 test -p xtask contracts::traceability
Corpus: positive committed constitution plus fixtures for missing, duplicate, unknown, stale-path, omitted required capability-to-contract edge, omitted texture-drawing edge, omitted reverse-action ingress, unresolved file-qualified symbol, mismatched active specification version, stale platform-baseline specification version, duplicate absent-event ID, synthetic baseline referenced by the lock, missing or mismatched baseline approval evidence, pass with a nonnull absence binding, not-applicable with a missing binding, mismatched platform-baseline path, digest, schema or specification version, unknown absent-event ID, mismatched gate, event, candidate, or parent environment, aggregate constraint without all four environments, remote or undeclared diagnostic sink, unbounded sink acknowledgement, missing or mismatched nested KK evidence, mismatched accessibility identity or digest, nested accessibility KU, stale accessibility text-layout generation, empty or trailing path segments, duplicate separators, and control-character paths.
```

#### OXY-A004 Enforce readiness, promotion, and immutable evidence bindings

- **Type:** Security
- **Effort:** 5
- **Dependencies:** OXY-A003
- **Category:** Security
- **Scope (In-Scope Files):**
  - `xtask/src/contracts/readiness.rs`
  - `xtask/src/contracts/digests.rs`
  - `qualification/fixtures/contracts/readiness/`
- **Scope (Out-of-Scope Files):**
  - Candidate source trees
  - `.constitution/tech-spec/data-models/qualification-lock.schema.json` (don't weaken the gate)
  - `.constitution/tech-spec/data-models/specification-phase.schema.json` (don't weaken promotion)
- **Verification Command:** `cargo +1.98.0 test -p xtask contracts::readiness`
- **Expected Success Output:** `exit 0`, with negative fixtures proving that unresolved readiness and fabricated promotion fail
- **STOP Conditions:**
  - STOP if a referenced file is absent or its SHA-256 differs; don't regenerate or accept it implicitly.
  - STOP if cross-file validation cannot bind one Stage 3 version, lock digest, candidate identity, and evidence set.
- **Description:** Implement repository-relative path confinement, SHA-256 verification, the pre-implementation readiness gate, the measurement-readiness gate, Phase 3B promotion resolution, candidate-selection consistency, and fail-closed negative fixtures.
- **Acceptance:**
  - **Mode:** invariant
  - **Evidence:**

```text
Invariants:
- candidateImplementationReady cannot become true while any pre-implementation input is null, missing, mismatched, or listed as a KU.
- candidateImplementationReady cannot become true while any nested KK platform claim lacks resolvable digest-bound evidence.
- candidateImplementationReady cannot become true when the capability baseline is synthetic or its typed lock reference, schema version, provenance, approval evidence, path, or digest doesn't match the resolved baseline.
- measurementReady cannot become true without candidateImplementationReady and final candidate source identities.
- productionReady cannot become true without every typed Phase 3B promotion artifact resolving to the same lock, candidate, and Stage 3 version.
Checker: cargo +1.98.0 test -p xtask contracts::readiness
```

#### OXY-A008 Resolve the native contract toolchain

- **Type:** Chore
- **Effort:** 3
- **Dependencies:** OXY-A001
- **Category:** DX
- **Scope (In-Scope Files):**
  - `xtask/src/toolchain.rs`
  - `qualification/tools/native-contract-toolchain.json`
  - `qualification/fixtures/toolchain/`
- **Scope (Out-of-Scope Files):**
  - `.constitution/tech-spec/contracts/qualification-lock.json` (Stage 3 reconciliation owns active pins)
  - Candidate source trees
  - Native contract compilation
- **Verification Command:** `cargo +1.98.0 test --workspace --all-features`
- **Expected Success Output:** `exit 0` with a staged compiler, binding-generator, formatter, and SDK tool manifest whose files match their recorded SHA-256 values
- **STOP Conditions:**
  - STOP if a required tool has no immutable source identity, license, supported host build, or verified digest.
  - STOP if resolving a tool would silently use an executable outside the staged manifest.
- **Description:** Resolve or fetch the exact C and C++ compiler, linker-independent header checker, `bindgen`, `cbindgen`, Rust formatter, Prettier, and required SDK utilities. Record source identities, host triples, versions, executable paths, and SHA-256 values in a staged manifest. Produce the exact `resolvedTools` proposal for Stage 3 reconciliation without editing the active qualification lock.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Assertions:
- Every required tool has an immutable source identity, version, host triple, license, executable path, and SHA-256.
- Re-resolution produces the same manifest bytes on the same locked host.
- Missing, substituted, or digest-mismatched tools fail before native contract validation starts.
- The staged manifest is visibly nonauthoritative and cannot set either readiness flag.
Command: cargo +1.98.0 test --workspace --all-features
```

#### OXY-A005 Compile and layout-check native contract headers

- **Type:** Chore
- **Effort:** 5
- **Dependencies:** OXY-A002, OXY-A008
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `xtask/src/contracts/native.rs`
  - `qualification/fixtures/native/`
  - `qualification/fixtures/generated-bindings/`
- **Scope (Out-of-Scope Files):**
  - `native/engine-bridge/` (don't implement the bridge)
  - Candidate adapter crates
  - `.constitution/tech-spec/contracts/oxyflut-substrate.h` (authoritative input)
- **Verification Command:** `cargo +1.98.0 test -p xtask contracts::native`
- **Expected Success Output:** `exit 0` after C11, C++17, generated-binding, calling-convention, symbol, and layout checks
- **STOP Conditions:**
  - STOP if the staged tool manifest is absent, incomplete, or differs from any resolved executable; return to OXY-A008 instead of substituting a tool.
  - STOP if generated bindings require a semantic ABI decision; return to Stage 3.
- **Description:** Add deterministic native-contract validation that uses only the OXY-A008 staged tool manifest, syntax-checks the integrated header as C11 and C++17, generates Rust declarations with the pinned bindgen version, inspects the acquisition symbol and calling convention, and compares sizes, alignment, offsets, nullability metadata, and generated hashes without linking a candidate.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Assertions:
- The authoritative header passes C11 and C++17 with all configured warnings treated as errors.
- Generated bindings are byte-stable under the locked toolchain.
- ABI table prefix, struct_size, abi_version, OXY_CALL, OXY_EXPORT, opaque handles, and callback signatures match fixtures.
- Native IME index-unit constants match the Rust enum and platform-contract strings; unknown numeric values fail before range conversion.
- Every declared C presentation status maps to the matching common Rust presentation status, success and failure timestamp invariants are enforced, and unknown status values fail before callback delivery.
- Texture realization accepts the same nonzero physical dimensions, closed pixel format, checked packed-byte length, and rejection cases through the common Rust and C contracts.
- Semantics selections project `Some` and `None` losslessly through `has_text_selection`, and invalid presence, endpoint, or reserved-field combinations fail.
- ABI-7, ABI-8, and ABI-9 implementations fail compatibility negotiation against the ABI-10 contract before callbacks are installed.
- Deliberate layout and symbol mutations fail.
Command: cargo +1.98.0 test -p xtask contracts::native
```

#### OXY-A006 Implement canonical evidence writing

- **Type:** Feature
- **Effort:** 5
- **Dependencies:** OXY-A002
- **Category:** Correctness
- **Scope (In-Scope Files):**
  - `crates/oxyflut-qualification/src/evidence.rs`
  - `xtask/src/evidence.rs`
  - `xtask/src/commands/evidence.rs`
  - `qualification/fixtures/evidence/`
- **Scope (Out-of-Scope Files):**
  - Candidate probes and measurements
  - Release signing implementation
  - Remote evidence storage
- **Verification Command:** `cargo +1.98.0 test --workspace --all-features`
- **Expected Success Output:** `exit 0` with deterministic local bytes and verified SHA-256 references
- **STOP Conditions:**
  - STOP if canonicalization would rewrite preserved source evidence; derived records must retain the source bytes and digest.
  - STOP if a format needs an unpinned external schema; route it to OXY-C001.
- **Description:** Replace the OXY-A001 root evidence and evidence-command placeholders. Reuse the OXY-A002 streaming SHA-256 primitive and implement local atomic writes, deterministic JSON encoding, repository-relative evidence references, media-type recording, collision-safe paths, source/derived provenance, and verification APIs used by later lock-input tools.
- **Acceptance:**
  - **Mode:** invariant
  - **Evidence:**

```text
Invariants:
- Equal logical records produce byte-identical derived JSON.
- Preserved source bytes are never rewritten.
- Every evidence reference resolves within the repository evidence root and matches its SHA-256.
- Interrupted writes publish neither partial files nor valid references.
Checker: cargo +1.98.0 test --workspace --all-features
```

#### OXY-A007 Assemble the contract-validation command

- **Type:** Chore
- **Effort:** 3
- **Dependencies:** OXY-A003, OXY-A004, OXY-A005, OXY-A006
- **Category:** DX
- **Scope (In-Scope Files):**
  - `xtask/src/contracts/mod.rs`
  - `xtask/src/commands/contracts.rs`
  - `.github/workflows/contracts.yml`
  - `qualification/fixtures/contracts/`
- **Scope (Out-of-Scope Files):**
  - Candidate build commands
  - Candidate probe commands
  - Production release workflows
- **Verification Command:** `cargo +1.98.0 run -p xtask -- contracts validate`
- **Expected Success Output:** `exit 0` with a stable content-free summary of every validation family
- **STOP Conditions:**
  - STOP if CI needs a secret, remote schema, or mutable network resource.
  - STOP if a validation family is skipped on a host without an explicit fail-closed result.
- **Description:** Complete the contracts command module created by OXY-A002. Wire every contract validator into the dispatcher created by OXY-A001, add deterministic diagnostic output and exit behavior, and run the same command in continuous integration without enabling candidate builds or network schema resolution. Don't edit `xtask/src/main.rs`.
- **Acceptance:**
  - **Mode:** contract_test
  - **Evidence:**

```text
Assertions:
- One command runs schema, instance, exact-set, registry, digest, readiness, promotion, Rust contract, C/C++ header, binding, symbol, and layout validation.
- Any failed family produces exit nonzero and identifies its contract path without private content.
- The clean repository produces exit 0 locally and in CI.
Command: cargo +1.98.0 run -p xtask -- contracts validate
```
