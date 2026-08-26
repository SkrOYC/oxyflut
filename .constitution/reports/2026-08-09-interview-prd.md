# Pre-stage interview report: PRD

- **Original date:** 2026-08-09
- **Amended:** 2026-08-25
- **Target:** Product requirements document (PRD)
- **Mode:** Greenfield
- **Domain:** Rust UI framework (`oxyflut`) over Flutter rendering technology
- **Status:** Product scope resolved; rendering substrate decision reopened

## Executive summary

`oxyflut` is a Rust UI framework for applications that need predictable native execution without a Dart virtual machine (VM) or tracing garbage collector. It provides reactive state, constraint-based layout, rendering, input, text editing, accessibility, and deterministic resource ownership.

The original interview assumed that stock Flutter Engine binaries exposed the internal rendering and framework-facing interfaces required by `oxyflut`. That assumption is incorrect. The stock Flutter Embedder API hosts a Flutter runtime; it doesn't let another language replace `dart:ui` or submit an application-owned Flutter layer tree.

Research completed on 2026-08-25 identified two candidate substrate paths:

- Use the standalone Impeller C SDK as a rendering and text library. `oxyflut` owns the frame pipeline and every platform service above the GPU API.
- Maintain a pinned Flutter Engine fork. The fork replaces Dart's runtime controller with a Rust runtime controller and retains the Flutter shell, rasterizer, compositor, and platform embedders. Starling is an example of this, with Swift as the framework language.

Neither candidate has demonstrated the complete P0 scope. The choice changes implementation cost, inherited platform behavior, distribution size, and long-term maintenance. The project can select a path only after it passes the validation gates in this report. If neither path passes, substrate research reopens without reducing P0.

## Product rulings

### R-01: Primary actor and encapsulation boundary

- **Ruling:** The primary actor is an application developer who builds a UI in Rust. Custom widget authors use safe Rust layout and rendering interfaces. They don't interact with C or C++ handles.
- **Rationale:** The public API must provide memory safety and must prevent raw engine pointers from escaping into application code.

### R-02: Target platforms and priorities

- **Ruling:** The project uses the following platform priorities:
  - **Tier 1 (P0):** macOS, Linux through Wayland and X11, and Windows through Win32.
  - **Tier 2 (P1):** iOS and Android.
  - **Tier 3 (P2):** WebAssembly and embedded Linux through Direct Rendering Manager and Kernel Mode Setting (DRM/KMS) or a framebuffer.
- **Rationale:** Desktop is the first release target. The architecture must not block later mobile-device and web work.

### R-03: Windowing and application lifecycle

- **Ruling:** P0 includes multi-window applications and surfaceless headless rendering. Headless mode renders and reads pixels without a native top-level or hidden window, display-server or compositor connection, window-system drawable, swapchain, or presentation call. It can use a GPU or software renderer. Each window has independent metrics, focus, semantics, damage, and presentation state. The implementation can share a GPU context where the selected substrate permits it.
- **Rationale:** Desktop applications require multiple windows. Automated tests require rendering without an interactive desktop session.

### R-04: Rendering substrate

- **Status:** Reopened on 2026-08-25.
- **Ruling:** `oxyflut` must evaluate the following candidate paths:
  - **Path A: standalone Impeller.** Consume the version-pinned standalone Impeller C SDK. Don't depend on the Flutter shell or Flutter platform embedders.
  - **Path B: full Flutter Engine.** Maintain a version-pinned engine fork that supplies a language-neutral runtime controller and C ABI. Don't expose Rust-specific code in the C++ engine layer when a language-neutral interface is practical. Starling is an example of this: it demonstrates runtime substitution but isn't the name or design basis of `oxyflut`.
- **Constraint:** The project must not describe either path as using an unmodified Flutter Engine. Path A uses a published engine subsystem. Path B modifies the engine.
- **Rationale:** These paths correspond to interfaces that exist and can be tested. The stock Flutter Embedder API alone can't implement the required framework replacement.

### R-05: Reactive state and lifecycle

- **Ruling:** P0 includes fine-grained signals with `signal`, `memo`, `effect`, and `batch` operations. Rust ownership and `Drop` must remove subscriptions when a component unmounts.
- **Rationale:** The framework must prevent dangling listeners and manual subscription cleanup in application code.

### R-06: Layout and custom layout protocol

- **Ruling:** P0 includes constraint-based box layout and a safe Rust custom layout protocol. The common case must visit each participating node a bounded number of times. Intrinsic measurement and text can require additional work that the performance specification measures separately.
- **Rationale:** Developers need custom virtualized lists, masonry grids, node editors, and data tables without C++ access.

### R-07: Text and editing

- **Ruling:** P0 includes styled text spans, runtime TrueType and OpenType font registration, caret movement, selection, clipboard integration, and operating system input method editor (IME) composition.
- **Path effect:**
  - Path A uses the Impeller typography API for shaping, layout, painting, line metrics, word boundaries, and glyph information. The public API doesn't provide the complete caret-affinity and range-box behavior required for editing. `oxyflut` must implement that behavior or maintain a pinned SDK extension.
  - Path B uses the Flutter Engine text stack and the platform embedder's text input transport. `oxyflut` still implements the editable-text model and the framework side of each platform-message contract.
- **Rationale:** Production desktop applications require complete editing and international text behavior.

### R-08: Canvas and vector drawing

- **Ruling:** P0 includes a safe Rust `Canvas` API with paths, rectangles, circles, gradients, transforms, clips, filters, textures, and reusable recorded pictures.
- **Path effect:** Path A wraps Impeller DisplayLists. Path B wraps the engine's `dart:ui`-equivalent canvas, picture, and scene interfaces through a C ABI.
- **Rationale:** Applications need custom vector rendering without access to substrate handles.

### R-09: Input routing and gesture disambiguation

- **Ruling:** P0 includes a pruned hit-test tree and a central gesture arena. Hit testing must avoid visiting unrelated subtrees when bounds make them ineligible.
- **Rationale:** The framework must resolve multi-touch pan and zoom, tap and drag, and nested scrolling conflicts.

### R-10: Focus hierarchy and keyboard traversal

- **Ruling:** P0 includes focus nodes, focus scopes, Tab and Shift+Tab traversal, directional navigation, and focus indicators.
- **Rationale:** Keyboard use and accessibility require a framework-owned focus model on every desktop platform.

### R-11: Semantics and accessibility

- **Ruling:** P0 includes a framework-owned semantics tree with incremental updates and platform accessibility integration.
- **Path effect:**
  - Path A implements platform bridges for macOS accessibility, Microsoft UI Automation, and Linux accessibility services.
  - Path B builds the engine's semantics update objects and lets each Flutter platform embedder translate them to operating system APIs. `oxyflut` must also handle semantics actions returned by the embedder.
- **Correction:** The Embedder API's `update_semantics_callback2` invokes the embedder callback with a borrowed `const FlutterSemanticsUpdate2*`. Treat the pointee and every nested buffer as callback-scoped; an embedder that retains the update must deep-copy every required node, action, string, array, and attribute before the callback returns. Semantics actions return through `FlutterEngineSendSemanticsAction` or the deprecated `FlutterEngineDispatchSemanticsAction`. Path B requires an internal framework-to-engine semantics bridge because the stock Embedder API doesn't provide one.
- **Rationale:** Screen-reader and automation integration is a release requirement, not an optional enhancement.

### R-12: Assets and textures

- **Ruling:** P0 includes asynchronous asset loading, image decoding, memory caching, and GPU texture realization.
- **Path effect:** Path A decodes compressed image formats outside Impeller and uploads decompressed pixels. Path B can use the Flutter Engine image decoder through the runtime bridge.
- **Rationale:** Image decoding must not block frame processing. The framework must manage CPU and GPU resource lifetimes explicitly.

### R-13: Operating system capabilities

- **Ruling:** Rust crates can provide file dialogs, clipboard access, and other operating system services when they meet platform and maintenance requirements. A path can instead use an inherited embedder service when that service offers the required behavior without Dart.
- **Rationale:** The framework must avoid serialization layers that don't add a useful process or security boundary.

### R-14: Surface and device recovery

- **Ruling:** P0 includes recovery from resize, surface loss, display sleep-and-resume, and recoverable graphics-device loss.
- **Path effect:** Path A owns context, swapchain, drawable, and surface recreation. Path B must validate and, where necessary, extend the engine rasterizer and platform embedder recovery paths. Rust lifecycle callbacks preserve framework state across recovery.
- **Rationale:** Desktop applications must recover from normal display and graphics lifecycle events without process termination when recovery is possible.

### R-15: Out-of-scope boundaries

- **Ruling:** Phase 1 excludes:
  - `OUT-01`: Dart application execution.
  - `OUT-02`: Dart package execution.
  - `OUT-03`: WebAssembly delivery.
  - `OUT-04`: An HTML or Document Object Model renderer.
- **Clarification:** Path B can retain Dart source and build definitions in a pinned engine checkout. Production `oxyflut` binaries must not start a Dart VM or execute Dart code.
- **Rationale:** The first release focuses on the Rust runtime and desktop targets.

### R-16: Component identity

- **Ruling:** P0 includes stable component identity, including value and object keys, for dynamic-list reconciliation.
- **Rationale:** Reordering must preserve state, focus, scroll position, and reusable render objects.

### R-17: Scrolling and viewports

- **Ruling:** P0 includes platform-appropriate scrolling, momentum, overscroll, and virtualized viewports. Behavior must account for mouse wheels, precision trackpads, and touch input.
- **Rationale:** High-refresh-rate displays require stable frame pacing and input integration, not only named deceleration curves.

### R-18: Compositing and post-processing

- **Ruling:** P0 includes retained compositing layers for clips, opacity, transforms, backdrop filters, and reusable subtrees.
- **Path effect:** Path A records nested or reusable Impeller DisplayLists and owns damage tracking. Path B builds Flutter layer trees and uses the engine's compositor, raster thread, and retained-layer behavior.
- **Rationale:** The public API must express post-processing without forcing CPU rasterization.

### R-19: Frame scheduling

- **Ruling:** P0 includes display-synchronized frame scheduling, coalesced invalidation, frame timestamps, and independent pacing for multiple displays when the operating system exposes it.
- **Path effect:** Path A integrates each operating system's display timing and presentation APIs. Path B implements the runtime controller's `BeginFrame` and frame-submission contract and uses the Flutter shell animator. Flutter doesn't establish independent per-display pacing as a satisfied requirement for `oxyflut`; Path B can require shell, animator, runner, and embedder changes.
- **Correction:** `FlutterEngineOnVsync` completes a request made by a running Flutter engine. It isn't a standalone ticker for an application-owned renderer.
- **Rationale:** Animation correctness depends on the scheduling contract and presentation feedback.

### R-20: Test harness and golden images

- **Ruling:** P0 includes frame pumping, gesture simulation, semantics assertions, deterministic layout assertions, and PNG golden comparison.
- **Golden policy:**
  - Byte-exact comparison is required in a pinned reference environment.
  - Each backend or platform can maintain a separate canonical baseline.
  - Cross-backend checks must use an explicit channel threshold or perceptual metric.
  - Targeted pixel invariants can replace stored whole-window baselines when they detect the same regression with less incidental churn.
- **Path effect:** Each path must provide a deterministic surfaceless surface or snapshot harness that doesn't depend on an interactive or virtual compositor or window server.
- **Rationale:** Exact rendering regressions must be detectable without claiming byte identity across GPU vendors, drivers, and graphics APIs.

### R-21: Internationalization and bidirectional layout

- **Ruling:** P0 includes locale propagation, bidirectional text, and automatic left-to-right or right-to-left layout mirroring.
- **Rationale:** The framework must support international applications from its first production release.

## Substrate paths

The following comparison describes the implementation consequences of R-04.

| Area | Path A: standalone Impeller | Path B: full Flutter Engine |
| :-- | :-- | :-- |
| Consumed Flutter code | Renderer, DisplayList, text, and related dependencies | Shell, animator, rasterizer, compositor, text, renderer, and platform embedders |
| Public boundary | Published `impeller.h` C API | Project-owned runtime C ABI plus engine bridge |
| Framework ownership | All framework and platform behavior | Framework behavior; engine retains frame and platform transport |
| Windowing and input | Implemented by `oxyflut` and a selected platform host | Candidate embedder transport plus Rust framework behavior; requires validation |
| IME and accessibility | New platform adapters | Candidate embedder transport plus a Rust protocol implementation; requires validation |
| Rendering backends | Impeller only | Skia or Impeller according to engine platform support and policy |
| Multi-window model | One or more shared contexts with per-window surfaces | Requires engine, view, focus, semantics, service-routing, and pacing validation |
| Surface recovery | Owned by `oxyflut` | Must validate and, where necessary, extend shell and embedder recovery |
| API stability | No stability guarantee; pin the header and binary | Private fork ABI; pin the engine source and bridge |
| Upstream maintenance | SDK revision upgrades | Release-tag rebases and conflict resolution across the fork |
| Distribution | Smaller renderer-focused dependency | Larger engine artifacts and more complex packaging |
| P0 platform effort | Unknown until the platform probes | Unknown until inherited contracts work without Dart on every Tier 1 platform |

### Path A: standalone Impeller

Path A treats Impeller as a GPU-accelerated drawing and text library. A Rust wrapper owns each opaque handle and maps `New`, `Retain`, and `Release` operations to Rust ownership. That mapping isn't sufficient by itself to make the wrapper safe; the cross-language safety contract in this report also applies.

The application pipeline has the following stages:

1. A platform adapter creates a window and receives operating system events.
2. The Rust runtime updates state and reconciles components.
3. The layout system computes geometry.
4. The paint traversal records an Impeller DisplayList.
5. The platform adapter acquires or wraps a drawable surface.
6. Impeller renders the DisplayList and presents the surface.

This path gives `oxyflut` full control over scheduling and resource policy. It also makes `oxyflut` responsible for the platform work that Flutter embedders normally provide. The P0 estimate must include IME, accessibility, clipboard, window lifecycle, cursor behavior, display timing, and recovery on every Tier 1 platform.

The project must pin an Impeller SDK to a Flutter commit. The SDK rejects an API version mismatch and doesn't promise API or ABI compatibility. The iOS renderer is mature inside Flutter, but the standalone SDK doesn't publish an iOS artifact. Mobile-device work therefore needs additional build and packaging validation.

### Path B: full Flutter Engine

Path B introduces a runtime controller interface between the Flutter shell and the framework implementation. A C callback table delivers frame, view, input, semantics, locale, lifecycle, and platform-message events to Rust. Starling is an example of this: it demonstrates runtime substitution with Swift.

Rust submits pictures, layer trees, and semantics updates through a separate C ABI that wraps the engine interfaces normally exposed through `dart:ui`. The Flutter shell continues to own animation timing, raster-thread submission, compositing, and interaction with platform embedders.

This path must use per-engine and per-view context handles. Global device-pixel ratio, font, and callback state doesn't meet `oxyflut`'s multi-window requirements.

Implementation must proceed in two phases:

1. Run the Rust runtime controller without starting a Dart isolate, while the build can still link unused Dart components.
2. After behavioral validation, add a build option that removes the Dart VM and Dart-bound `lib/ui` sources from production artifacts.

The Dart-linked-but-unused build provides feasibility and migration evidence only. Every shared-suite result, eligibility gate, nonfunctional measurement, distribution artifact, and scored result for Path B must use the final Dart-VM-removed production configuration.

The fork must remain based on a named Flutter release tag. Each engine upgrade requires a dependency sync, full rebuild, ABI conformance tests, rendering tests, and Tier 1 platform qualification. The build and release plan must budget for the engine checkout, generated artifacts, and security fixes.

## Independent configuration axes

The substrate family doesn't decide every platform choice. Each candidate evaluation must record the following decisions for macOS, Windows, Wayland, and X11:

- The platform host or window toolkit.
- The event-loop owner and task-runner integration.
- The window-to-engine or window-to-context model.
- The graphics backend and minimum driver requirements.
- The software or CPU fallback policy.
- The headless and offscreen backend.
- The IME, clipboard, cursor, drag-and-drop, and accessibility adapters.
- The packaging, signing, and update mechanism.

A selected family can use different hosts or graphics backends on different platforms. A cross-family hybrid, such as Path A on one Tier 1 platform and Path B on another, requires a new architecture decision because it creates two rendering and platform-integration contracts.

## Cross-language safety contract

Both candidates cross an unsafe language boundary. Before OD-01 closes, each candidate must document and test the following contract:

- ABI version negotiation, calling convention, structure sizes, alignment, field offsets, nullability, and error reporting.
- Fixed-width scalar types and documented representation for enums, `bool`, `size_t`, and callback signatures.
- Exact source-header and binary hashes, exported-symbol manifests, runtime version checks before other calls, and generated Rust binding layout checks.
- Target compiler, C++ standard library, C runtime, linker, and exception-mode compatibility.
- Ownership transfer, retained and borrowed handles, buffer lifetimes, and teardown order.
- Thread affinity for every handle and an explicit Rust `Send` and `Sync` policy.
- Task-runner handoff, callback reentrancy, cancellation, and process-shutdown behavior.
- Panic containment around every Rust callback and C++ exception containment around every exported wrapper. Rust unwinding and C++ exceptions must not cross the ABI.
- Callback-driven release of font, image, and message buffers.
- Maximum buffer sizes, integer conversion rules, and allocation-failure behavior.
- AddressSanitizer, ThreadSanitizer where supported, UndefinedBehaviorSanitizer, C-side and Rust-side `sizeof`, `alignof`, and `offsetof` tests, and teardown and reentrancy stress tests.

## Security and privacy requirements

The substrate decision must include a threat model for every implemented external ingress, including fonts, images, platform messages, clipboard data, IME text, and semantics strings. Custom shaders, plugins, and exported telemetry aren't P0 capabilities; if a candidate exposes one during a spike, that ingress or trust boundary enters the same threat model and gates that candidate. Both candidates must provide the following evidence:

- Fuzzing for decoders, parsers, serialized messages, and callback lifecycle transitions.
- Payload and resource limits that prevent unbounded memory and GPU allocation.
- Dependency and common vulnerabilities and exposures monitoring with named patch ownership and response targets.
- Tests for callback use-after-free, double release, malformed UTF-8, and allocator failure.
- A default logging policy that excludes raw clipboard, IME, entered text, and accessibility content.
- A default-off production telemetry policy. Any enabled telemetry must define its destination, transport protection, retention, access control, consent basis, identifier lifetime, and treatment of channel names and window metadata.
- A documented process boundary and sandbox assumption for application code and for any additional executable extension boundary the candidate actually exposes.

Before implementing either spike, inventory every actual ingress and explicitly record a cited KK absence for categories the candidate doesn't expose. Freeze the fuzz harnesses, seed corpora, payload caps, sanitizer toolchains, and finding-severity rubric. Run each implemented font, image, serialized-message, and other parser ingress for at least 24 CPU-hours under AddressSanitizer and UndefinedBehaviorSanitizer. Run concurrent callback and teardown harnesses for at least 8 CPU-hours under ThreadSanitizer where the target supports it. A crash, sanitizer report, deadlock, operation exceeding the frozen 5-second timeout, or allocation beyond a payload cap fails the gate until fixed and replayed. Preserve every reproducer in the regression corpus.

Triage an applicable critical dependency vulnerability within 1 business day and remediate it within 7 calendar days. Triage a high-severity vulnerability within 3 business days and remediate it within 14 calendar days. Triage a medium-severity vulnerability within 30 calendar days and record remediation or a documented risk acceptance within 90 calendar days. An overdue critical or high finding makes the candidate ineligible.

## Distribution and provenance requirements

Each candidate must produce installable release artifacts for every Tier 1 platform. The release process must include the following controls:

- Pinned source revisions and verified artifact hashes.
- Reproducible build instructions and recorded compiler, linker, and SDK versions. Reproducibility applies to the canonical unsigned payload, software bill of materials, notices, and provenance subject because signatures and notarization envelopes can contain timestamps.
- A software bill of materials, artifact-to-source inventory, artifact-to-license inventory with SPDX identifiers, and generated third-party notices.
- A compatibility review and fulfillment plan for attribution, notice, source-offer, redistribution, and modification obligations that apply to code, fonts, data, shaders, generated tools, and packaged artifacts.
- Authenticated build provenance or attestations that bind source revisions, toolchains, dependencies, and output hashes.
- A vulnerability inventory and security-update procedure.
- Stripped release binaries with separate symbol files for crash analysis.
- Platform signing, notarization, installer, and store-policy validation where applicable.
- An inventory of engine data, fonts, shaders, dynamic libraries, and runtime resources included in the package.

For comparison and reproducibility, run a pinned build-container image with `LC_ALL=C`, `TZ=UTC`, `umask 022`, and `POSIXLY_CORRECT` unset. Archive the canonical unsigned payload with GNU tar 1.35 in POSIX format and `zstd` 1.5.7 at level 19 with one compression thread. Sort paths bytewise; set modification time to Unix epoch 0, numeric user and group IDs to 0, and user and group names to empty; set the PAX extended-header name to `%d/PaxHeaders/%f`; delete access-time and change-time PAX fields; preserve the frozen manifest's permission and executable bits; encode paths as UTF-8 NFC; preserve relative symbolic links without dereferencing them; and reject absolute or parent-traversing link targets. Preserve a hard link only when both names and their common target are inside the payload and recorded in the manifest; otherwise materialize a regular file. Measure compressed size from that `tar.zst` file. Measure installed size as the sum of regular-file payload bytes from the canonical manifest, excluding filesystem block rounding.

Serialize the software bill of materials as SPDX 3.0.1 JSON-LD with `@context` set to `https://spdx.org/rdf/3.0.1/spdx-context.jsonld`. The single `software_Sbom` `ElementCollection` object inside `@graph` must carry `profileConformance` containing `core`, `software`, and `simpleLicensing`; a pre-spike amendment must explicitly select `expandedLicensing` instead if the artifact inventory requires it. Validate the document structurally against `https://spdx.org/schema/3.0.1/spdx-json-schema.json` and semantically against the SPDX 3.0.1 OWL and SHACL model at `https://spdx.org/rdf/3.0.1/spdx-model.ttl`. This project profile requires every `@graph` member to be an identifier-bearing Element and requires non-Element complex values, including `CreationInfo`, to be embedded in their owning Element; reject an identifier-less graph member. Before the spikes, freeze a deterministic SPDX identifier-generation algorithm and reject duplicate identifiers. Before final serialization, normalize the top-level `@graph` by the identifier emitted through the SPDX context, every semantically unordered nested array lexically by each entry's recursively canonicalized JSON bytes, and every relationship `to` array by target identifier while rejecting duplicates; preserve source order only for properties that the SPDX model defines as ordered. These project rules form a total array order because recursively canonical bytes break any remaining field-level tie. After array normalization, emit the document once using the complete SPDX 3.0.1 canonical serialization rules for whitespace, literals, integers, strings, arrays, and ordered object members. Encode notices as UTF-8 with LF endings and order entries by SPDX identifier and source path.

Serialize provenance as an in-toto Statement v1 whose `_type` is `https://in-toto.io/Statement/v1`, whose `predicateType` is `https://slsa.dev/provenance/v1`, and whose `subject` contains one ResourceDescriptor with the canonical unsigned payload name and `digest: {"sha256": "<lowercase artifact digest>"}`. Its predicate must conform to SLSA Build Provenance v1. Restrict the Statement to the RFC 8785 input domain and serialize it with RFC 8785 JSON Canonicalization Scheme; the resulting UTF-8 bytes are the exact DSSE payload. Wrap those bytes in a DSSE v1 envelope with payload type `application/vnd.in-toto+json` and an Ed25519 signature. Before the spikes, record hashes for the Statement, SLSA predicate, and RFC 8785 specifications; pin the validator implementation and version; generate and pin the release public-key fingerprint and approved `predicate.runDetails.builder.id` values in the verifier policy; and keep the private key in the declared hardware-backed signing boundary.

The independently signed continuous-integration invocation record uses a frozen project schema with `invocationId`, `repository`, `revision`, `jobStartedOn`, and `jobFinishedOn`; RFC 8785 canonical bytes; DSSE payload type `application/vnd.oxyflut.ci-invocation.v1+json`; and an Ed25519 CI-coordinator key distinct from the release provenance key. Freeze the schema hash, validator version, trusted coordinator public-key fingerprint, canonical repository URI and normalization rule, and repository identity before the spikes. The SLSA predicate must contain exactly one entry in `predicate.buildDefinition.resolvedDependencies[]` whose normalized `uri` equals that canonical repository URI; reject a missing or multiple match. That entry's `digest.gitCommit` must equal the invocation record's `revision`, and the record's normalized `repository` must equal the same canonical URI. Require exact equality between `predicate.runDetails.metadata.invocationId` and the record's `invocationId`, thereby joining the records by repository, revision, and invocation. Require `predicate.runDetails.metadata.startedOn` and `predicate.runDetails.metadata.finishedOn`, with the former no later than the latter and each within 5 minutes of the corresponding record time.

Verification must validate both DSSE signatures and payload types, require the allowlisted public keys and builder identity without treating either unauthenticated DSSE `keyid` hint as authority, validate each payload against its pinned rules, bind the provenance subject digest to the artifact, join the records by invocation and revision, and enforce the timestamp order and skew bounds. Invocation timestamps and DSSE signatures aren't byte-reproducibility inputs. Require exact SHA-256 equality for the canonical payload, software bill of materials, notices, and provenance subject across independent builds. Verify signed installers semantically after signing by checking payload hashes, identity, entitlements, timestamp validity, and notarization or store records rather than requiring byte-identical signed envelopes.

## Diagnostics and telemetry requirements

Both candidates must implement the same runtime-neutral local-diagnostics contract. Local diagnostics use machine-local, user-controlled sinks inside the declared application trust boundary. They can remain in production for troubleshooting and include in-process buffers, user-selected local files, operating system logging, counters, traces, crash metadata handled on the same machine, and an in-process inspection API. The contract must provide a versioned schema, monotonic-clock correlation, bounded buffering, sampling rules, dropped-event counters, field-level privacy classifications, stable event names, and bounded-lifetime per-engine, per-view, and per-frame identifiers for the following data:

- UI, raster, GPU, and presentation timing.
- Missed-frame count, queue depth, and invalidation coalescing.
- Window, display, surface, swapchain, and graphics-device lifecycle events.
- Platform-message, IME, clipboard, and accessibility failures without recording private content.
- Resource-cache size, allocation failures, and recoveries.
- Crash symbols and substrate version information.

Exported telemetry means diagnostics that cross the machine boundary or another trust boundary declared in the threat model. Exported telemetry and an exporter implementation aren't P0. The runtime-neutral diagnostics contract must expose bounded, privacy-classified records to user-controlled local sinks without coupling the framework to a transport. Freeze two benchmark variants: an instrumentation-disabled measurement baseline and privacy-safe local diagnostics with exporters absent. The telemetry-free production build uses the second variant and must contain no exporter and no path that collects raw private user content. The build system provides the first variant only for overhead measurement. Measure the second variant against the first for local-diagnostics overhead. A later exporter must receive a separate product decision, privacy policy, security review, and overhead budget.

## Knowledge register

The report uses the following epistemic states:

- **KK, known known:** Verified by a primary source, repository inspection, or a reproducible measurement.
- **KU, known unknown:** An unanswered question is identified and has a closure test or named investigation.
- **UK, unknown known:** Relevant knowledge might exist in upstream code, maintainers, platform specialists, or prior experiments, but isn't captured in these artifacts.
- **UU, unknown unknown:** The specific issue isn't identifiable in advance. The project can only reduce exposure through diverse tests, prototypes, review, and staged deployment.

The register records the state of the substrate decision as follows:

| ID | State | Statement | Closure or mitigation |
| :-- | :-- | :-- | :-- |
| K-01 | KK | The standalone Impeller SDK exposes a versioned C API for rendering and text and doesn't promise API or ABI stability. | Pin the header and binary to one Flutter revision. |
| K-02 | KK | The stock Flutter Embedder API doesn't expose the framework-to-engine scene and semantics submission boundary required by Path B. | Path B adds and owns that boundary. |
| K-03 | KK | Runtime substitution can retain the Flutter shell; Starling is an example of this and uses a Swift-specific callback contract. | Use the example as evidence for feasibility, not as the `oxyflut` interface. |
| K-04 | KK | Impeller is used on iOS inside Flutter, but the standalone SDK doesn't publish an iOS artifact. | Treat standalone iOS packaging as P1 validation work. |
| K-05 | KK | Neither candidate has demonstrated all P0 behavior on macOS, Windows, Wayland, and X11 in these artifacts. | Don't treat either candidate as eligible from document evidence alone. |
| K-06 | KU | Can Path B add independent per-display pacing and complete desktop service routing while retaining the selected engine components? | Run the multi-display and per-view tests in the full-engine spike. |
| K-07 | KU | Path A might require text APIs beyond the published caret and glyph information. | Complete the editing-geometry corpus and record extensions. |
| K-08 | KK | Neither candidate has demonstrated the 50 ms startup or 25 MiB idle-memory target in these artifacts. | Don't claim either nonfunctional target as met. |
| K-09 | UK | Platform and Flutter specialists might know constraints that repository-level analysis misses. | Require targeted review from one specialist for each Tier 1 platform and record the findings. |
| K-10 | UK | Existing internal tests or downstream forks might contain reusable failure cases that aren't indexed in these reports. | Search upstream trackers and interview maintainers during each spike. |
| K-11 | UU | Driver, assistive-technology, locale, device, and lifecycle combinations can expose unanticipated failures. | Use hardware diversity, fuzzing, fault injection, prerelease channels, and crash telemetry. |
| K-12 | KU | Upstream changes can invalidate private interfaces or build assumptions, but the migration cost across the selected revisions isn't measured. | Pin revisions, rehearse upgrades, and retain a rollback path. |
| K-13 | KK | The exact Rust-compatible runtime controller and language-neutral C ABI for Path B don't exist in `oxyflut`. | Treat the proposed interface as unimplemented. |
| K-14 | KU | Can each candidate pass every P0 row on macOS, Windows, Wayland, and X11? | Complete the shared suite and Tier 1 feasibility matrix. |
| K-15 | KU | What startup and idle-memory values does each candidate produce under the frozen measurement procedure? | Run normalized measurements on every reference configuration. |
| K-16 | KU | Can `oxyflut` implement a Rust-compatible Path B runtime controller and language-neutral C ABI that meets the safety contract? | Implement and validate the Path B proof of concept. |
| K-17 | KK | Upstream Flutter currently schedules all views on each frame rather than supplying separate per-display vsync. | Treat inherited behavior as insufficient; Path B must change and validate the scheduling contract. |

UK and UU entries describe exposure, not discovered facts. A review can't prove that no UK or UU remains.

## Substrate decision gates

The project must complete the same-platform comparison, both path-specific spikes, the Tier 1 feasibility matrix, and the upgrade rehearsal before closing OD-01.

### Same-platform comparison

Run both candidates on the same machine, operating system, display, release compiler settings, reference application, fonts, assets, and test input. Record cold and warm results separately. The comparison must use stripped binaries and must identify the graphics API, GPU, driver, cache state, and window count.

### Shared capability suite

Both candidates must pass the same substrate-neutral suite before path-specific evidence is scored. The suite includes the following cases:

- Two windows with isolated metrics, focus, input, semantics, platform services, invalidation, and teardown.
- Place one view on each of two displays with different refresh rates. Test simultaneous animation, one animated view with an idle peer, view migration between displays, and a runtime refresh-rate change. Assert display association, frame timestamps, invalidation counts, render counts, and presentation feedback independently for each view.
- Runtime font registration, fallback across scripts, styled bidirectional text, and deterministic font-buffer lifetime.
- Text insertion, replacement, grapheme and word deletion, cut, undo and redo, keyboard and pointer selection, grapheme navigation, caret affinity at bidirectional boundaries, multiline and mixed-run selection rectangles, combining marks, emoji sequences, and IME composing ranges.
- Asynchronous image decoding, cancellation, malformed input, upload lifetime, cache reuse, and teardown during in-flight work.
- The same paths, clips, transforms, gradients, images, filters, reusable pictures or DisplayLists, retained layers, and backdrop-filter scenes.
- Surfaceless headless rendering and pixel readback with no native top-level or hidden window, display-server or compositor connection, window-system drawable, swapchain, or presentation call. A GPU or software renderer is permitted, but the report must identify it.
- The same normalized performance, allocation, recovery, golden-image, security, distribution, and observability tests.

For the multi-display cases, obtain presentation opportunities from an operating system display-link, vertical-blank, or presentation-timing source observed outside the candidate's callback stream. Under variable refresh, count actual opportunities reported by that source rather than deriving them from a nominal rate. If a platform provides no independent source, the platform row remains a gating KU until an external measurement supplies equivalent evidence. Before the spikes, calibrate and freeze the maximum cross-source timestamp error for each reference configuration. After a 1-second settling interval, record opportunity timestamps for a 10-second measurement window on one monotonic timebase, then drain presentation feedback for two maximum observed display intervals. Count opportunities and presentations by their event timestamps, not callback-receipt times, and exclude an opportunity whose full adjacent-interval matching window falls outside the measurement window. Within each display epoch, process presentations chronologically. Pair each presentation with the nearest unassigned opportunity that is later than the preceding matched opportunity and satisfies `opportunity_time <= presentation_time + clock_error`; break an equal-distance tie toward the earlier opportunity. Reject a pair whose distance exceeds half the opportunity's local interval plus the clock error, where the local interval is the smaller of the preceding and following opportunity intervals. A presentation or opportunity can appear in at most one pair. An unpaired presentation fails the case, and an unpaired opportunity counts as a missed presentation. The first valid pair establishes the interval boundary and isn't included in interval-error statistics. For each later pair, compare its presentation interval with the interval between its matched opportunity and the preceding presentation's matched opportunity; intervening unpaired opportunities remain counted as misses but aren't collapsed out of elapsed-time accuracy. Calculate `interval_error = abs(presentation_interval - opportunity_interval) / opportunity_interval`; the 95th percentile must not exceed 10%. Each continuously animated view must pair presentations with between 95% and 100% of the independently counted opportunities for its associated display. An idle peer must render zero frames after settling until it receives an explicit invalidation.

For migration, `t0` is the operating system event that associates the view with the destination display. For a refresh-rate change, `t0` is the operating system mode or rate-change event. Before the spikes, freeze for each platform an independent event-delivery-and-render lead-time bound measured by the harness without either candidate. Close the prior display epoch at `t0`, start a new opportunity and presentation sequence on the destination display, and apply the same one-to-one pairing algorithm. The first eligible opportunity is the first destination opportunity at or after `t0 + lead_time`. Success requires acknowledged presentations paired with that opportunity and the immediately following destination opportunity; the first pair establishes the new interval boundary. Both scenarios must therefore succeed within the first two eligible opportunities and must not trigger a render in an unrelated idle view.

### Path A spike

The standalone Impeller spike must demonstrate the following evidence:

- Record whether editing geometry uses the published API, framework algorithms, or a pinned SDK extension.
- Demonstrate platform-host event routing, frame pacing, damage, resize, surface recreation, and teardown.
- Identify every platform service and lifecycle behavior that `oxyflut` must implement outside Impeller.
- Produce the ABI safety, platform ownership, security, distribution, and observability evidence required by this report.
- Report normalized frame timing, allocations, startup, memory, and artifact size.

### Path B spike

The full-engine spike must demonstrate the following evidence:

- Start a Flutter shell with a Rust runtime controller and no Dart isolate.
- Receive view metrics, pointer and keyboard input, platform messages, lifecycle events, and frame callbacks.
- Submit pictures and retained layer trees through a C ABI.
- Submit incremental semantics updates and receive semantics actions.
- Demonstrate that the Rust C ABI exposes the editing geometry, runtime font, image-decoding, and scene behavior used by the shared suite.
- Identify required changes to the runtime, shell, animator, runner, and each platform embedder.
- Build once with Dart linked but unused and once with the Dart VM removed.
- Produce the ABI safety, security, distribution, and observability evidence required by this report.
- Report normalized frame timing, allocations, startup, memory, artifact size, and changed upstream files.

### Tier 1 feasibility matrix

Both candidates must run hard feasibility probes on macOS, Windows, Wayland, and X11. A responsibility map without working evidence doesn't close a platform row. Each row must demonstrate the following behavior:

- Two windows with independent size, device-pixel ratio, focus, keyboard routing, pointer routing, and teardown.
- Before choosing an IME adapter, freeze a candidate-neutral per-environment capability baseline that names required behaviors, metadata, actions, index units, and test vectors without mandating one native integration technology. Each candidate must declare and justify its current, nondeprecated native interface and minimum supported operating-system or compositor version. Native IME composition must preserve framework composition styling and cursor handling, commit, cancellation, replacement behavior, keyboard and framework action mapping, candidate positioning after scrolling, transforms, and device-pixel-ratio changes, focus transfer, surrounding-text synchronization, deletion around the cursor, stale-transaction rejection, protocol ordering, lifecycle composition reset, content type and redaction for sensitive fields, dead keys, reconversion where the platform baseline requires it, and correct UTF-8 and UTF-16 index conversion. Where the chosen interface exposes numeric protocol negotiation, record the advertised and negotiated versions and negotiate the highest mutually supported nondeprecated version. Where it doesn't, record the operating-system build, exact interface contract, capability probes, and `not applicable` for numeric negotiation. Preserve every metadata field required by the baseline and exposed by the negotiated or probed interface, including language, preedit hints or segment attributes, replacement ranges, and native action events. A capability genuinely absent from the candidate-neutral platform baseline is KK platform behavior. A documented interface incompatibility is also KK; any unresolved consequence is a separately named gating KU. A baseline capability that the chosen adapter loses fails the gate or remains a gating KU.
- Clipboard copy, cut, and paste; insertion, replacement, grapheme and word deletion; undo and redo; keyboard and pointer selection; and bidirectional rich-text selection.
- Before the spikes, freeze two role-specific accessibility contracts for AppKit accessibility, Microsoft UI Automation, and Linux AT-SPI: a forward framework-to-platform property and relation map, and a reverse native-action-to-framework map. Incremental insertion, update, and deletion must preserve every applicable distinct property and relation, including accessible name, label or title, description, hint, help or full description, tooltip, attributed text, identifier, heading level, roles, states, values, bounds, transforms, traversal order, accessibility focus distinct from input focus, hit-test routing, text ranges, selection, scroll extents, language, direction, live regions, hidden and disabled nodes, secure-field redaction, and multi-view isolation. The reverse map must define each standard, custom, and scroll action's native control pattern or action identifier, payload encoding, text-index unit, target engine, view, and node routing, acknowledgement, error result, and stale-target behavior. Distinct native properties aren't substitutes for one another; API inspection and end-to-end assistive-technology tests must confirm every mapped value and returned action independently.
- End-to-end tasks with expected events and results through VoiceOver on macOS, Narrator plus Microsoft UI Automation inspection on Windows, and the selected assistive technology plus accessibility API inspection on Wayland and X11.
- Resize, minimize or occlusion, sleep and resume where available, display detach or hotplug, surface loss, and recoverable device loss where the API permits fault injection.
- State preservation and the fixed recovery limits in this report for every recoverable event.
- Surfaceless headless rendering and pixel readback with no native top-level or hidden window, display-server or compositor connection, window-system drawable, swapchain, or presentation call.

Before the spikes, freeze a cited platform capability baseline for recovery events. A genuinely unsupported event is `not applicable` with KK evidence. If the platform supports an event but the harness can't yet observe or induce it, record a gating KU and use the closest deterministic fault injection; the platform row and OD-01 remain open until equivalent evidence exists.

Use a monotonic clock for the following P0 recovery limits. Before either spike, map each start condition to an externally observable operating system or graphics-API event on every Tier 1 platform. For resize, `t0` is the later of the final resize notification and the first external signal that presentation resources are available; the deadline includes surface or swapchain recreation, rendering, submission, and acknowledgement. Log resource-unavailable time separately and don't let implementation-selected internal events define `t0`. Success is the first correctly sized and scaled frame acknowledged by presentation feedback. Under variable refresh, one refresh interval means the nominal period reported for the destination display when the timer starts. After display reassignment, use the new display's nominal period.

- From the defined resize `t0`, resume ordinary presentation within two refresh intervals.
- After a surface-loss notification, recover within 250 ms.
- After a resume or display-topology notification, present the first valid frame within 500 ms.
- After a recoverable graphics-device-loss notification, recreate required resources and present within 2 seconds.

Recovery must not loop without a bound. Permit at most three consecutive recreation attempts per fault, cap transient recovery memory at twice the measured steady-state graphics allocation, release superseded resources within 500 ms after success or terminal failure, and return a structured terminal error when the bound is exhausted.

### Upgrade rehearsal

Run both candidates across the same three consecutive stable feature-release lines: Flutter `3.38.6` at `8b872868494e429d94fa06dca855c306438b22c0`, Flutter `3.41.0` at `44a626f4f0027bc38a46dc68aed5964b05a83c18`, and Flutter `3.44.0` at `559ffa3f75e7402d65a8def9c28389a9b2e6fe42`. This rehearsal covers two transitions. For each transition, record changed APIs, bridge changes, conflicts, changed upstream files, clean and incremental build time, artifact changes, regression results, and engineering time. One person-day equals 8 hours of attributable engineering work. A manually resolved file is any non-generated file changed by a person to restore the candidate after updating the pinned revision, whether or not Git reported a textual conflict.

Before implementing either spike, select one upstream renderer, text, or shared-dependency security patch that applies to code consumed by both candidates. If no disclosed patch is suitable, predeclare one synthetic validation patch and its expected tests. Demonstrate how each candidate applies that patch without accepting unrelated upstream changes. Record time and changed files separately from the two-transition maintenance score. Security patchability is a hard gate, not part of the weighted upgrade-cost anchor.

### Hard rejection gates

A candidate is ineligible if any of the following conditions applies:

- A Tier 1 platform has a known architectural blocker for a mandatory P0 capability.
- Any Tier 1 feasibility row, shared-suite requirement, or other mandatory P0 gate is failed or remains a gating KU. A bounded implementation plan can keep the candidate under investigation but can't make it eligible for scoring.
- The candidate requires a Dart VM or Dart application code in a production binary.
- The bridge can't meet the safety contract or contain panics and exceptions at the language boundary.
- The candidate fails multi-window isolation, per-view service routing, surfaceless headless rendering, or recoverable lifecycle behavior.
- The candidate can't produce signed and attributable Tier 1 release artifacts or fulfill every applicable license and redistribution obligation.
- A measured lower bound makes a nonfunctional target impossible, and the product owner hasn't revised that target through the requirements process.
- The upgrade rehearsal shows no sustainable security-patch or release-rebase procedure.

Only non-gating KUs outside the P0 substrate decision, such as later P1 packaging work, can remain after OD-01 closes. Record them, but don't include them in a P0 score.

### Scored selection

Use a strict two-stage process. First, compare each candidate against every Score 3 anchor and hard gate. Reject a candidate before scoring if it misses any Score 3 anchor. Second, score only eligible candidates with an integer score of 3, 4, or 5. Before implementing either spike, freeze the reference configurations, raw measurement templates, and two scoring assessors. The assessors score independently from cited KK evidence and must reach one written consensus score for every criterion on which they differ. Multiply each consensus score by the weight and divide by 5 to produce a 100-point result.

| Criterion | Weight | Score 3 | Score 4 | Score 5 |
| :-- | --: | :-- | :-- | :-- |
| Demonstrated P0 platform coverage | 30 | Every Tier 1 row and shared case passes on the frozen macOS, Windows, Wayland, and X11 reference configurations. | Every row also passes on a second GPU or hardware configuration for each environment. | Every row and fault case runs in continuous automated qualification on both configurations for each environment. |
| Two-transition upgrade-maintenance cost | 20 | The two transitions total at most 10 person-days and 40 manually resolved files. | The transitions total at most 5 person-days and 20 manually resolved files. | The transitions total at most 2 person-days and 10 manually resolved files. |
| Performance, startup, memory, and artifact size | 15 | Every frozen target passes under its defined statistic and confidence rule. | Every upper-bound target has at least 15% headroom. | Every upper-bound target has at least 30% headroom. |
| ABI safety, security, and privacy | 15 | The required contract, sanitizer runs, fuzzing, threat model, and privacy controls pass. | An independent review reports no unresolved high-priority finding after fixes. | An independent audit and repeated stress campaign report no unresolved high- or medium-priority finding. |
| Distribution, licensing, and provenance | 10 | Every Tier 1 artifact installs, verifies, carries complete obligations, and passes signing checks. | Two independent builders reproduce each canonical unsigned payload and metadata set; signed envelopes verify semantically. | An external rebuild reproduces each canonical unsigned payload and metadata set, and applicable store or notarization admission passes. |
| Testing, diagnostics, and operational clarity | 10 | The shared suite, fault injection, symbols, traces, and documented triage workflow pass. | All deterministic gates run automatically, and injected failures identify the responsible view and subsystem. | A 30-day prerelease exercise reports no unresolved high-priority diagnostic or recovery gap. |

Apply the selection outcome as follows:

- If no candidate is eligible, reopen substrate research without reducing P0.
- If one candidate is eligible, select it. Passing every Score 3 anchor gives it at least 60 points.
- If both candidates are eligible and differ by at least 5 points, select the higher score.
- If both candidates are eligible and differ by fewer than 5 points, select the candidate with the lower measured upgrade-maintenance cost. This maintenance-first tie-breaker is an explicit product policy in addition to the 20-point maintenance weight.
- If the tie-break evidence is equal or inconclusive, continue the investigation without selecting a candidate.

## Nonfunctional quality attributes

Before implementing either spike, define and version the reference application, scenes, interaction scripts, fonts, assets, window sizes, window counts, cache states, release flags, and one reference configuration for macOS, Windows, Wayland, and X11. Record the operating system, graphics backend, GPU, driver, compiler, substrate revision, and measurement tool with every result. Run 60 independent process launches for startup results. For frame results, run 20 independent process launches with 300 warmup frames and 500 measured frames each, for 10,000 measured frames in total.

Predeclare valid-sample rules. Exclude a sample only for a measurement-tool failure, an unrelated operating system interruption recorded by the harness, or a physical disconnect outside the fault being tested. Don't remove statistical outliers. Preserve and report every raw sample, exclusion, reason, and harness log.

Use conservative directly observed bounds instead of inferring an unseen population distribution. Use nearest-rank percentiles. The frame-time `comparison_bound` is the maximum of the 20 per-launch 99th percentiles. The startup `comparison_bound` is the maximum of 60 independent cold-launch values; under independent identically distributed launches, that maximum is a distribution-free one-sided tolerance bound with about 95.4% confidence of covering the population 95th percentile. The memory `comparison_bound` is the maximum of the 10 per-launch 95th percentiles. The artifact `comparison_bound` is the direct measured value. Require every `comparison_bound` to meet its limit.

For a nonzero upper-bound target, calculate `headroom = (limit - comparison_bound) / limit`. The performance score uses the smallest headroom across UI frame time, cold startup, one-window memory, two-window memory, compressed artifact size, and installed artifact size on all reference configurations. Exact-zero and exact-match targets must pass but don't contribute extra headroom. Recovery and diagnostics-overhead limits are hard gates under their separate criteria and don't contribute to the performance headroom score. Preserve the calculation implementation and version with the raw dataset.

1. **Frame pipeline budget:** The frame-time `comparison_bound` for layout and paint submission must not exceed 2.0 ms on the UI thread. Report median, 95th-percentile, and 99th-percentile UI, raster, GPU, and presentation time separately for each launch.
2. **Steady-state allocations:** Framework-owned paint traversal must perform zero global heap allocations in every one of the 10,000 measured frames after cache warmup. Report substrate allocations separately because neither candidate guarantees zero allocation internally.
3. **Cold startup:** The maximum of 60 independent cold launches must not exceed 50 ms on any reference configuration in a stripped release build. Startup begins when the operating system loader transfers control to the process entry point and ends at platform presentation feedback for the first complete application frame. Define cold filesystem, shader, font, and process cache state separately from warm startup.
4. **Idle memory:** Run 10 independent process launches for both one-window and two-window baselines. In each launch, reach the frozen idle state, wait 10 seconds without scheduled work, and sample once per second for 60 seconds. Calculate the per-launch 95th percentile; the maximum of those 10 values must not exceed 25 MiB. Use proportional set size on Linux and frozen equivalent tools on macOS and Windows. Separate private CPU memory, shared mappings, engine artifacts, and graphics-driver allocations. Don't compare unlike platform metrics as if they were identical.
5. **Golden determinism:** The pinned reference renderer must produce byte-identical PNG files across 20 repeated runs. Cross-platform and cross-backend comparisons follow the policy in R-20.
6. **Release artifact size:** Excluding application assets and separate debug symbols, the canonical unsigned runtime payload must not exceed 75 MiB as the frozen `tar.zst` archive and must not exceed 300 MiB installed on any Tier 1 platform. Both caps must pass. Count all dynamic libraries, fonts, engine data, shaders, and runtime resources. Report both values.
7. **Recovery:** Every recoverable fault in the Tier 1 matrix must meet the event-specific limits in the decision gates. Framework state, focus, selection, and semantics identity must survive unless the operating system invalidates them.
8. **Diagnostics overhead:** Run 20 matched pairs of the frozen variants, using a fresh process for every variant. After a 10-second warmup, the harness emits monotonic workload-start and workload-end markers around the same 60-second interaction workload and then terminates the process. CPU overhead is the difference in process CPU seconds sampled at those markers divided by 60 wall-clock seconds. Memory overhead is the difference in the operating system's process-lifetime peak resident high-water value, read after the workload and before exit; before the comparison, freeze one exact platform API, measurement definition, and tool version and use it for both variants on that platform. Because every variant is a fresh process with the same startup, warmup, and workload lifetime, the peak covers the same interval without depending on periodic scheduler wake-ups. Frame overhead is the difference between per-launch 99th-percentile frame times inside the workload markers. Alternate variant order with a frozen balanced schedule. A missed marker or incomplete 60-second trace invalidates and reruns the pair. For each metric, the `comparison_bound` is the maximum of the 20 paired differences. Privacy-safe local diagnostics must add less than 1 percentage point of CPU, 1 MiB resident memory, and 0.05 ms frame time relative to the instrumentation-disabled baseline. The telemetry-free production build must contain no exporters or raw-private-content collection paths.

## Evidence reviewed for the amendment

- [Impeller Standalone SDK](https://github.com/flutter/flutter/tree/master/engine/src/flutter/impeller/toolkit/interop)
- [Impeller C API header](https://github.com/flutter/flutter/blob/master/engine/src/flutter/impeller/toolkit/interop/impeller.h)
- [Starling engine repository](https://github.com/starling-build/starling-engine)
- [Starling runtime controller interface](https://github.com/starling-build/starling-engine/blob/starling/engine/src/flutter/runtime/runtime_controller_interface.h)
- [Starling runtime callback table](https://github.com/starling-build/starling-engine/blob/starling/engine/src/flutter/lib/ui/swift/include/swift_runtime_callbacks.h)
- [Starling Dart-free engine change](https://github.com/starling-build/starling-engine/commit/082739678cf39a7d73afb2fea23010f2f87f97e8)
- [SPDX 3.0.1 serialization](https://spdx.github.io/spdx-spec/v3.0.1/serializations/)
- [SLSA Build Provenance v1](https://slsa.dev/spec/v1.2/build-provenance)
- [in-toto Statement v1](https://github.com/in-toto/attestation/blob/main/spec/v1/statement.md)
- [Dead Simple Signing Envelope v1](https://github.com/secure-systems-lab/dsse/blob/master/protocol.md)
- [Flutter per-display vsync issue](https://github.com/flutter/flutter/issues/146249)
- [Flutter desktop multi-view tracker](https://github.com/flutter/flutter/issues/142845)
- [Flutter licensing FAQ](https://docs.flutter.dev/resources/faq#which-software-licenses-apply-to-flutter-and-its-dependencies)
- [Wayland text input version 3 protocol](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/text-input/text-input-unstable-v3.xml)
- [Flutter text input system channel](https://api.flutter.dev/flutter/services/SystemChannels/textInput-constant.html)
