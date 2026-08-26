# Qualification technical stack

- **Version:** v0.8.0
- **Status:** Phase 3A qualification specification
- **Production ready:** no
- **Required successor:** Phase 3B production specification

## Scope guard

This specification defines concrete builds for comparing both substrate candidates. It does not select a production substrate and cannot authorize a production implementation plan.

Stage 4 can use v0.8.0 only for qualification infrastructure, candidate probes, common contract tests, measurements, and evidence collection. Stage 4 must not plan the production framework, release delivery, or removal of either candidate until Phase 3B reaches v1.0.0.

The current qualification lock has `candidateImplementationReady: false` and `measurementReady: false`. Until `candidateImplementationReady` becomes true, Stage 4 is limited to qualification scaffolding, validators, environment discovery, baseline authoring, external-contract snapshotting, and pre-implementation lock finalization. Candidate implementation can then begin against the frozen suite. Comparable or scored evidence collection cannot begin until the completed candidate source identities are pinned and `measurementReady` becomes true.

The machine-readable `contracts/specification-phase.json` file enforces the same state. ADR-0001 defines the promotion gate, and ADR-0010 keeps substrate selection proposed.

## Shared bill of materials

The following entries are mandatory for both qualification candidates:

| Concern | Pinned choice | Posture | Reason |
| :-- | :-- | :-- | :-- |
| Primary language | Rust 1.98.0, commit `88d9e12ae178fab0fb5cc050a94da85685d449ea`, edition 2024 | Adopt for Phase 3A | Provides the safe application surface and matches the August 20, 2026, stable release. |
| Package resolver | Cargo 1.98.0 with resolver version 3 and one committed `Cargo.lock` file | Adopt for Phase 3A | Gives one reproducible dependency graph for the workspace. |
| Error types | `thiserror` 2.0.20 | Adopt for Phase 3A | Keeps library errors typed without a general error container in public contracts. |
| Generational identities | `slotmap` 1.1.1 | Trial | Supports runtime, view, component, semantics-node, and resource generation checks. Qualification must measure allocation behavior. |
| Bounded inline collections | `smallvec` 1.15.1 with default features disabled | Trial | Supports preallocated hot-path collections without adopting an alpha release. |
| Worker handoff | `crossbeam-channel` 0.5.16 | Trial | Provides bounded asynchronous asset and decoding queues without introducing an application-wide async runtime. |
| Flags | `bitflags` 2.13.1 | Adopt for Phase 3A | Models closed capability and state masks. |
| Text boundaries | `unicode-segmentation` 1.13.3 | Trial | Supplies grapheme and word boundaries for the shared editing model. Rendering geometry remains substrate-qualified. |
| Image decoding | `image` 0.25.10 with default features disabled and `gif`, `jpeg`, `png`, and `webp` enabled | Trial | Gives the focused candidate one bounded Rust decoder surface. The integrated candidate must expose equivalent behavior through its adapter. |
| Evidence serialization | `serde` 1.0.229 and `serde_json` 1.0.151 | Adopt for Phase 3A | Implements the owned JSON evidence formats in `data-models/`. |
| JSON Schema validation | `jsonschema` 0.51.0 with default features disabled | Adopt for Phase 3A | Validates local schemas without network resolution. |
| Property testing | `proptest` 1.11.0 | Adopt for Phase 3A | Exercises ownership, indices, reconciliation, and state-machine invariants. |
| Benchmarks | `criterion` 0.8.2 with default features disabled | Trial | Supports local microbenchmarks. PRD qualification meters remain authoritative. |
| Binding generation | `bindgen` 0.72.1 and `cbindgen` 0.29.4 | Trial | Generates and checks the C boundary layouts. Generated output is locked and diffed. |
| Fuzzing | `cargo-fuzz` 0.13.2 | Adopt for Phase 3A | Runs the required parser and callback campaigns. |
| Dependency policy | `cargo-deny` 0.20.2 and `cargo-audit` 0.22.2 | Adopt for Phase 3A | Enforces license, source, duplicate, and advisory policies. |
| Coverage | `cargo-llvm-cov` 0.9.0 | Trial | Produces qualification coverage evidence without defining a product acceptance percentage. |
| Documentation formatting | Prettier 3.9.6 executed as `bunx prettier@3.9.6` | Adopt for Phase 3A | Makes constitution formatting repeatable without adding a package manifest before repository scaffolding. |

No database, network service, remote telemetry system, application plugin runtime, or custom-shader API is part of Phase 3A.

## Substrate candidate pins

Both candidates use Flutter framework 3.47.0 at commit `4cf24164269a5ebf0c16a028a00727d0e77bbb05`, whose `bin/internal/engine.version` pins engine commit `5f77625673248ee5846fbcaf5d3e1a3878386fd7`. Upgrade rehearsal uses the consecutive 3.41.0, 3.44.0, and 3.47.0 stable feature-release lines frozen by the PRD evidence contract.

| Candidate | Concrete input | Posture | Qualification boundary |
| :-- | :-- | :-- | :-- |
| Focused drawing-and-text candidate | Standalone Impeller SDK artifacts retrieved under engine commit `5f77625673248ee5846fbcaf5d3e1a3878386fd7`; `impeller.h` Git blob `440f83aac6580495e488ba350e6d5cbbb32e2f11` from the framework checkout; `darwin-arm64`, `linux-x64`, and `windows-x64` downloads with SHA-256 and sizes in the qualification lock | Trial | Rust owns the Platform integration, View coordinator, asset decoding, and recovery policy. The Rust adapter implements `contracts/oxyflut-substrate.rs` through generated Impeller bindings. |
| Integrated candidate | Oxyflut engine fork based on Flutter commit `4cf24164269a5ebf0c16a028a00727d0e77bbb05` with a language-neutral runtime controller and `flutter_enable_dart=false` production-configuration probe | Trial | The fork can transport platform and timing callbacks but must normalize through the same C contract. It cannot package, start, or execute the Dart runtime in scored artifacts. |

The integrated approach replaces the application runtime while retaining selected engine subsystems. Starling is an example of this: it demonstrates runtime substitution and a Dart-free engine build with Swift. It is evidence for feasibility, not a dependency, fork base, contract, or design name.

## Platform qualification pins

These pins define the first reference configurations. Hardware identifiers, driver versions, installed shared-library hashes, and SDK artifact hashes must be added to the qualification lock before a measurement is valid.

| Environment | Reference configuration | Focused candidate host | Integrated candidate host |
| :-- | :-- | :-- | :-- |
| macOS | arm64 macOS 26.5 SDK through Xcode 26.6 build `17F113`; minimum deployment target is a gating KU | `objc2` 0.6.4 and `objc2-app-kit` 0.3.2 with direct AppKit, Core Graphics, accessibility, text-input, clipboard, and view-associated display-link integration | Flutter macOS embedder from the pinned fork, with callbacks normalized through the Oxyflut adapter and measured against the same external display observer |
| Windows | x86-64 Windows 11 25H2; Visual Studio Build Tools 2022 17.14.39; Windows SDK 10.0.26100.8876; minimum supported build is a gating KU | `windows` 0.62.2 with explicit Win32, text, accessibility, clipboard, per-output DXGI observation, and graphics features | Flutter Windows embedder from the pinned fork, with callbacks normalized through the Oxyflut adapter and measured against the same external per-output observer |
| Wayland | x86-64 Ubuntu 26.04 LTS Wayland session; minimum compositor and protocol versions are gating KUs | `gtk4` 0.11.4 with `v4_20`, `glib` 0.22.8, `wayland-client` 0.31.15, and `wayland-protocols` 0.32.13 | Flutter Linux embedder from the pinned fork, supplemented only where a P0 probe proves the inherited mechanism insufficient; an independent opportunity source remains a gating KU for both candidates |
| X11 | x86-64 Ubuntu 26.04 LTS X11 session; minimum X server and protocol versions are gating KUs | The same GTK and GLib pins plus `x11rb` 0.14.0 for separately observed display and window-system evidence | Flutter Linux embedder from the pinned fork, supplemented only where a P0 probe proves the inherited mechanism insufficient; an independent opportunity source remains a gating KU for both candidates |

The selected host dependencies are qualification choices. Phase 3B must remove any dependency that the winning candidate doesn't need and must repeat artifact, memory, license, and security validation.

## Engine and platform build tools

The pinned Flutter checkout and its dependency lock define Clang, GN, Ninja, sysroot, and engine dependency revisions. The qualification lock must record every resolved tool digest and generated configuration. It must reject an unrecorded tool substitution.

Apple builds use the Xcode pin in this file. Windows builds use the Visual Studio and Windows SDK pins. Linux builds use the signed Ubuntu 26.04 package repositories frozen by snapshot and package version in the qualification lock.

## Compatibility policy

- Commit `rust-toolchain.toml`, `Cargo.lock`, the engine dependency lock, generated binding hashes, and qualification environment locks.
- Pin every Cargo dependency with an exact `=` requirement during Phase 3A.
- Disable default features unless this file names them.
- Treat changes to `contracts/oxyflut-public.rs`, `contracts/oxyflut-substrate.rs`, `contracts/oxyflut-substrate.h`, or a durable JSON Schema as compatibility changes that require contract tests and a migration note.
- Reject a substrate library or header mismatch before creating a runtime, view, or resource.
- Run the complete shared suite after any engine revision, platform SDK, compiler, host dependency, or ABI change.
- Preserve the two-transition upgrade evidence outside active specifications under `.constitution/reports/` or `.constitution/spikes/`.

## Durable data posture

Oxyflut has no database and no durable application-state store in Phase 3A. The project owns durable qualification evidence, diagnostic record files, artifact manifests, ingress inventories, capability traceability, and the specification-phase state. Their JSON Schema files under `data-models/` are the contracts.

## Mandatory production promotion

Phase 3B is required. Stage 3 remains incomplete for production until all of the following conditions hold:

- The qualification lock reached `candidateImplementationReady: true` before candidate implementation, then reached `measurementReady: true` after completed candidate source identities were pinned and before evidence collection. Every cited result binds to that unchanged measurement-ready lock digest.
- CAP-SUB-001 through CAP-SUB-004 produce a selected eligible candidate from preserved evidence.
- The common-case layout visit cap is frozen and passes its qualification corpus.
- ADR-0010 changes from `proposed` to `accepted` with the selected candidate and cited evidence.
- The losing candidate leaves the production workspace, dependency graph, artifacts, and production verification commands.
- Every remaining bill-of-materials entry receives an Adopt, Trial, or Hold posture for production and a verified source hash.
- The public Rust surface, native ABI, platform contracts, evidence migrations, and supported target matrix are frozen for the selected candidate.
- The selected production configuration passes every P0 capability and PRD constraint on all Tier 1 environments.
- `contracts/specification-phase.json` changes to `production-3b`, sets `productionReady` to `true`, and permits production Stage 4 planning.
- The Stage 3 version advances to v1.0.0 or later and records the promotion in `changelog.md`.

Failure of any condition keeps Phase 3A active. It does not permit a partial production specification.

## Verified sources

- [Rust 1.98.0 release announcement](https://blog.rust-lang.org/releases/1.98.0/)
- [Rust 2024 Edition Guide](https://doc.rust-lang.org/stable/edition-guide/rust-2024/)
- [Flutter 3.47.0 release notes](https://docs.flutter.dev/release/release-notes/release-notes-3.47.0)
- [Impeller Standalone SDK](https://github.com/flutter/flutter/tree/4cf24164269a5ebf0c16a028a00727d0e77bbb05/engine/src/flutter/impeller/toolkit/interop)
- [Flutter 3.47.0 engine revision](https://github.com/flutter/flutter/blob/4cf24164269a5ebf0c16a028a00727d0e77bbb05/bin/internal/engine.version)
- [Xcode 26.6 release](https://developer.apple.com/news/releases/?id=06252026a)
- [NSView display-link API](<https://developer.apple.com/documentation/appkit/nsview/displaylink(target:selector:)>)
- [Windows SDK release notes](https://learn.microsoft.com/en-us/windows/apps/windows-sdk/release-notes)
- [IDXGIOutput::WaitForVBlank](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-waitforvblank)
- [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/)
