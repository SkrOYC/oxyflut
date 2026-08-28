# Spike report: OXY-B001 macOS qualification baseline

## Time box

- **Budget:** 1 focused day.
- **Clock start / stop:** 2026-08-28T16:52:30Z / 2026-08-28T17:47:05Z.
- **Round-4 correction clock start / stop:** 2026-08-28T18:00:03Z / 2026-08-28T18:10:12Z.
- **Round-5 correction clock start / stop:** 2026-08-28T18:21:52Z / 2026-08-28T18:28:54Z.
- **Round-6 correction clock start / stop:** 2026-08-28T18:36:43Z / 2026-08-28T18:42:09Z.
- **Round-7 correction clock start / stop:** 2026-08-28T19:42:23Z / 2026-08-28T19:44:57Z.
- **Round-8 correction clock start / stop:** 2026-08-28T20:22:55Z / 2026-08-28T20:25:45Z.
- **Scope result:** This report changes no product capability, architecture boundary, source tree, or specification. The only repository file changed is this report.

## Question

- **Decision this spike must produce:** Which exact supported macOS versions and interfaces provide the platform-independent input method editor, accessibility, per-view timing, independent timing observation, service-routing, and recovery baseline for both allocations?

### Decision register

| ID | Baseline question | Allocation | Status | Answer and cited evidence | Next bounded probe for a KU |
| :-- | :-- | :-- | :-- | :-- | :-- |
| B001-01 | Which SDK supplies the baseline? | Both | KK | Xcode 26.6 includes the macOS 26.5 SDK, and Apple's release page identifies the build as `17F113`. See S1 and S2, whose preserved excerpts and digests appear in [Authoritative source records](#authoritative-source-records). | Not applicable. |
| B001-02 | What is the minimum deployment target? | Both | KU (gating) | `NSView.displayLink(target:selector:)` is macOS 14.0+, but the fetched pages for `NSTextInputClient`, `NSAccessibilityProtocol`, `NSWindow.didChangeScreenNotification`, `NSApplication.didBecomeActiveNotification`, and `NSWorkspace.didWakeNotification` identify only `macOS`, not a minimum version. The availability matrix therefore cannot derive a verified maximum. STOP: an official availability value was not fetched for every baseline interface. See S3-S7 and S11-S12, S20. | P8: on a macOS host with Xcode 26.6, fetch and preserve Apple's DocC availability metadata or the corresponding Apple SDK declaration for each `unavailable` matrix cell, then have a second command verify every declared minimum is at most the proposed target. Expected output: a source manifest with each API, Apple URL, stated minimum, source digest, and `maximumMinimum`; otherwise retain this KU. |
| B001-03 | Does AppKit provide the native input method editor transport and UTF-16 index unit? | Both | KK | `NSTextInputClient` lists marked and selected ranges, marked-text replacement, unmarking, insertion, character-index lookup, and first-rectangle operations. `NSTextInputContext` owns a client, activates and deactivates, discards a conversion session, and invalidates character coordinates. Apple states that an `NSString` presents itself as UTF-16 code units. These are interface-availability facts, not evidence that either allocation implements the contract. See S4-S6. | Not applicable. |
| B001-04 | Does the proposed input method editor map preserve composition, replacement, cancellation, deletion, focus transfer, candidate geometry, and checked UTF-16 conversion? | Focused and integrated | KU (gating) | Apple's interface documentation establishes operations but not either allocation's callback transcript, conversion behavior, secure-field handling, two-view routing, or an exact CJK input-source identity. S23 gives `com.apple.inputmethod.Kotoeri.Japanese` only as an example of an identifier, not as a host pin. The host preflight could not run AppKit. STOP: no controlled AppKit probe or input-source enumeration ran. See S4-S6, S23, and the [Controlled probe record](#controlled-probe-record). | P1a must inventory, select, and confirm the active host CJK source. P1 must then run the action-by-vector matrix separately through the focused standalone host and the P6-gated integrated fork on a pinned arm64 macOS host. Expected output: a digested inventory and selection record, then distinct redacted `focused/` and `integrated/` JSONL transcripts and validation records with client identity, view generation, UTF-16 ranges, conversion result, pass or fail, and the stated exit code. |
| B001-05 | Can a numeric-input and sensitive-field policy be frozen? | Both | KU (gating) | `NSTextInputTraits` exposes text-input traits, but the fetched page does not establish a numeric negotiation contract or prove that a secure field returns only redacted surrounding context. STOP: documentation does not establish either behavior. See S19. | P1: log only trait names, classification, range lengths, and redaction checks for numeric and secure fixtures. Expected output: a supported setting and no raw secure text, or a cited unsupported result. |
| B001-06 | Which accessibility interface exposes semantics and reverse actions to assistive software? | Both | KK | `NSAccessibilityProtocol` defines the informational properties and action methods for accessible elements; it doesn't define notification names. `NSAccessibility.Notification` is the notification-name type. Apple requires role-specific protocols for custom `NSView` subclasses and `NSAccessibilityElement` for custom non-view elements. See S7, S8, and S27. This proves the destination interface only. | Not applicable. |
| B001-07 | Is there a complete focused allocation forward and reverse VoiceOver map? | Focused | KU (gating) | No preserved map binds roles, states, values, relations, traversal, text range, selection, geometry, view identity, stale generation, reverse payloads, and acknowledgements to the focused allocation. The required candidate-neutral role registry does not exist because the inspected inputs enumerate no role identifiers. STOP: the map artifact and its Stage 3 role-registry prerequisite do not exist. B001-07 remains KU until D0, P2R, and P2 succeed. | D0, then P2R, then P2: Stage 3 must first create the candidate-neutral semantic-role registry specified in [Spec edits required](#spec-edits-required). P2R must validate and freeze its nonempty digest before P2 runs the corpus through the focused allocation and validates its output against `accessibility-map.schema.json`. Expected output: one valid nonempty registry snapshot, one complete map, and one reverse-action log with immutable evidence references, or a named decision, registry, schema, or behavioral failure. |
| B001-08 | Is there a complete integrated allocation forward and reverse VoiceOver map? | Integrated | KU (gating) | No pinned integrated fork inventory, candidate-neutral role registry, or preserved map establishes its macOS accessibility path. STOP: the integrated input and Stage 3 role-registry prerequisite are not frozen. B001-08 remains KU until D0, P2R, and P2 succeed. | P6, then D0, then P2R, then P2: freeze the integrated fork revision and inventory its macOS accessibility crossings. Stage 3 must create the candidate-neutral role registry, P2R must validate and freeze its digest, and P2 must run the same corpus through the C ABI. Expected output: a commit-bound inventory, nonempty registry snapshot, complete map, and reverse-action log, or a named missing crossing, decision, registry, or schema failure. |
| B001-09 | What supplies view-associated opportunities and presentation feedback? | Both | KK | `NSView.displayLink(target:selector:)` invokes its callback in sync with the display the view is on and omits callbacks when the view is hidden or off-display. `MTLDrawable.addPresentedHandler(_:)` runs after presentation, and `presentedTime` reports its onscreen host time or `0.0` for an unpresented or dropped frame. See S3, S9, and S10. | Not applicable. |
| B001-10 | Is the external timing observer independent of both candidate callback streams? | Both | KU (gating) | The documented display link is view-associated, but no result proves a harness-owned observer has a callback stream independent of either candidate or meets the causal matching rule in CON-FRM-001. STOP: no timing probe ran. See S3. | P3: run two target windows and a harness-owned third visible view in a separate process on two displays. Block each candidate scheduling callback in turn. Expected output: observer and candidate PIDs, display identities, timestamps, presentation acknowledgements, and trace IDs proving no candidate callback is the observer source. |
| B001-11 | Can each view and observer migrate to its current display? | Both | KU (gating) | `NSWindow.didChangeScreenNotification` identifies the changed window, and the display-link API associates a callback with a view's display. Neither source establishes rebind timing or a correct epoch after a cross-display move. STOP: no migration trace exists. See S3 and S11. | P3: move each target window between two displays, capture screen-notification object, screen identities, link periods, and epoch IDs. Expected output: each moved view and observer creates one new display epoch, and the idle peer remains unscheduled. |
| B001-12 | May deprecated `CVDisplayLink` serve as the baseline observer? | Both | KK (not applicable) | No. Apple labels the Core Video display-link management functions deprecated. The ticket excludes deprecated timing APIs from independent evidence. See S17. | Not applicable. |
| B001-13 | Does focused service routing reject an implicit default window? | Focused | KU (gating) | Explicit `NSTextInputContext` client ownership and window screen-change notifications do not prove that every focused input, accessibility, clipboard, timing, and recovery request carries its view generation. STOP: no two-window routing trace exists. See S5 and S11. | P4: interleave input method editor, accessibility action, pasteboard, display, resize, and teardown events in two focused-host windows. Expected output: each record has an owning native object and view generation; no default-window lookup or cross-window delivery occurs; stale work returns a typed error. |
| B001-14 | Does integrated service routing have an exact inherited interface inventory and reject an implicit view? | Integrated | KU (gating) | No frozen fork commit or source inventory exists, so Apple platform documentation cannot establish an engine-to-C-ABI route. STOP: the inherited route is unclassified. See S5. | P6, then P4: freeze the fork revision, inventory macOS embedder paths and symbols, and run the two-window routing trace through the C ABI. Expected output: an inventory digest and the same no-default-window, no-cross-window, stale-generation results. |
| B001-15 | Which observable built-in signals form the recovery baseline? | Both | KK | AppKit provides window resize and screen-change notifications, and `NSWorkspace.didWakeNotification` reports device wake from sleep. `CAMetalLayer.nextDrawable()` can return `nil` after drawable exhaustion or invalid layer properties. Metal exposes completion, terminal command-buffer status, and an error description. Apple excludes `MTLDeviceNotificationName.wasRemoved` on Apple Silicon. See S11, S13-S16, S18, S20, S21, and S26. | Not applicable. |
| B001-16 | Is focused recovery injectable for resize, drawable unavailability, OS resume, topology change, and graphics failure? | Focused | KU (gating) | The source evidence documents observable stimuli, not the focused allocation's recovery behavior, preservation of state, deadlines, retry bound, allocation bound, release deadline, or a reproducible graphics-device-loss injection. STOP: no macOS host is available for P5. See S14-S16 and S20-S21. | P5: use the documented drawable-exhaustion stimulus and a real sleep and wake, then run the normalized cases against the focused allocation. Expected output: timestamped trace records that satisfy every assertion in [Recovery qualification corpus](#recovery-qualification-corpus), or a named unavailable injection point. |
| B001-17 | Is integrated recovery injectable through the inherited embedder and C ABI? | Integrated | KU (gating) | No pinned integrated source inventory or fault trace establishes the inherited lifecycle and Metal paths through the C ABI. STOP: the integrated input and trace are absent. | P6, then P5: inventory inherited recovery paths and run each normalized P5 case through the integrated C ABI. Expected output: a source inventory digest and one trace per case, or a named unavailable injection point. |
| B001-18 | Is immutable evidence available for every remaining status-bearing candidate claim? | Both | KU (gating) | This report preserves authoritative excerpts and digests for source-availability claims and a host preflight output. It has no candidate input method editor transcript, accessibility map, independent-timing trace, routing trace, recovery trace, or integrated source inventory. STOP: those artifacts do not exist. See [Evidence preservation convention](#evidence-preservation-convention). | P7: publish fetched source bodies and every successful P1-P6 result under the stated repository fixture root, then write a manifest that names source URL, command, host identifier, input digest, output digest, and validator result. Expected output: SHA-256 verification for every referenced evidence object. |

## Context and objective

- **Triggering upstream file or section:** `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.macos`.
- **Target:** Close only source-documented interface questions. Retain behavior, mapping, timing independence, routing, recovery, source-availability, and evidence-publication questions as gating KUs.
- **Archetype / surface:** Library and SDK with system-native macOS integration.
- **Interpretation:** A source-documented API is KK for interface availability only. It is not evidence that either allocation implements or qualifies against that API.

## Codebase baseline

- **Reference SDK:** The source record establishes Xcode 26.6 build `17F113` and macOS 26.5 SDK. See S1 and S2.
- **Minimum baseline:** macOS 14.0 is a plausible lower bound because `NSView.displayLink(target:selector:)` requires macOS 14.0, but it is not a KK deployment target. The availability matrix has unverified historical introductions, so B001-02 remains a gating KU.
- **Candidate-neutral interfaces:** The interface baseline uses `NSTextInputClient`, `NSTextInputContext`, `NSAccessibilityProtocol`, role-specific accessibility protocols or `NSAccessibilityElement`, `NSView.displayLink(target:selector:)`, `MTLDrawable.addPresentedHandler(_:)`, `MTLDrawable.presentedTime`, AppKit lifecycle notifications, `NSWorkspace.didWakeNotification`, `CAMetalLayer.nextDrawable()`, and Metal command-buffer completion, status, and error observation. See S3-S16 and S18-S21.
- **Excluded interfaces:** Do not use deprecated `CVDisplayLink` as an independent timing observer or `MTLDeviceNotificationName.wasRemoved` on Apple Silicon. See S17 and S18.

### Availability matrix

Table 1 records every baseline interface cited by B001-03, B001-06, B001-09, and B001-15. A value of `not stated` means the fetched Apple page gives no numerical macOS introduction, not that the interface is unavailable.

| Baseline area | Interface | Documented minimum macOS | Apple source | Result |
| :-- | :-- | :-- | :-- | :-- |
| Input method editor | `NSTextInputClient` | Not stated; the page says only `macOS`. | S4 | Blocks B001-02. |
| Input method editor | `NSTextInputContext` | 10.6. | S5 | At most 14.0. |
| Input method editor | `NSTextInputContext.keyboardInputSources` | 10.6. | S23 | At most 14.0. |
| Text index unit | `NSString` UTF-16 code-unit indexing | Not stated; the page describes UTF-16 units but gives no numerical minimum. | S6 | Blocks B001-02. |
| Accessibility | `NSAccessibilityProtocol` and its members | Not stated; the page says only `macOS`. | S7 | Blocks B001-02. |
| Accessibility | Role-specific protocols and `NSAccessibilityElement` | Not stated in the fetched API collection. | S8 | Blocks B001-02. |
| View timing | `NSView.displayLink(target:selector:)` | 14.0. | S3 | At most 14.0. |
| Presentation feedback | `MTLDrawable.addPresentedHandler(_:)` | 10.15.4. | S9 | At most 14.0. |
| Presentation feedback | `MTLDrawable.presentedTime` | 10.15.4. | S10 | At most 14.0. |
| Window lifecycle | `NSWindow.willStartLiveResizeNotification` | 10.6. | S13 | At most 14.0. |
| Window lifecycle | `NSWindow.didEndLiveResizeNotification` | 10.6. | S26 | At most 14.0. |
| Window lifecycle | `NSWindow.didChangeScreenNotification` | Not stated; the page says only `macOS`. | S11 | Blocks B001-02. |
| Application lifecycle | `NSApplication.didBecomeActiveNotification` | Not stated; the page says only `macOS`. | S12 | Blocks B001-02. |
| OS resume | `NSWorkspace.didWakeNotification` | Not stated; the page says only `macOS`. | S20 | Blocks B001-02. |
| Drawable availability | `CAMetalLayer.nextDrawable()` | 10.11. | S21 | At most 14.0. |
| Metal recovery observation | `MTLCommandBuffer.addCompletedHandler(_:)` | 10.11. | S16 | At most 14.0. |
| Metal recovery observation | `MTLCommandBuffer.status` | 10.11. | S15 | At most 14.0. |
| Metal recovery observation | `MTLCommandBuffer.error` | 10.11. | S14 | At most 14.0. |

The maximum of the fetched numerical minima is macOS 14.0. This cannot become the deployment target until P8 obtains numerical availability for each `not stated` entry. No specification pin advances ahead of that gate.

### Controlled probe record

The host did not provide macOS, AppKit, Xcode, or `xcrun`. This noncandidate preflight ran from `/tmp/wf-epic-b/OXY-B001/`; its SHA-256 is `e00023d6b62c6a9138a4bbc99ff099a84816be8e960fb318f53b89ceb5c8781d`.

```text
$ /tmp/wf-epic-b/OXY-B001/probe-host.sh
timestamp=2026-08-28T16:58:02Z
kernel=Linux 6.18.44 x86_64
appkit_framework=absent
xcrun=absent
xcodebuild=absent
sw_vers=absent
sdk_path=not-run: xcrun absent
```

The absent AppKit and Xcode toolchain trigger the ticket STOP condition for every functional macOS probe. No emulator, GUI application, browser, or candidate code ran.

### Input method editor qualification corpus

The exact CJK input-source identity is KU (gating). S23 says that Text Input Source Services identifies sources by `kTISPropertyInputSourceID` and uses `com.apple.inputmethod.Kotoeri.Japanese` only as an example. It doesn't establish an identifier present on the qualification host. Before V5 runs, P1a must enumerate `TISCreateInputSourceList` on the pinned arm64 macOS host, recording every enabled source ID, localized name, `kTISPropertyInputSourceType`, and `kTISPropertyInputSourceLanguages` list, plus `sw_vers`, `xcodebuild -version`, and `uname -m`, in `output/input-sources.json`. P1a must write the SHA-256 of that exact JSON in its raw-output sidecar beside the output. The selection matcher must select only an enabled source whose `kTISPropertyInputSourceType` equals `kTISTypeKeyboardInputMode` or `kTISTypeKeyboardInputMethodModeEnabled`; it must reject `kTISTypeKeyboardLayout`. The matcher must parse every `kTISPropertyInputSourceLanguages` entry as a BCP-47 language tag and compare its case-insensitive primary language subtag to `zh`, `ja`, or `ko`, rather than comparing a display name or a locale string. P1a, not P1, must call `TISSelectInputSource` for the selected source, then call `TISCopyCurrentKeyboardInputSource` and require its `kTISPropertyInputSourceID` to equal the selected ID. P1a must write `output/input-source-selection.json` with the selected and active IDs, type, matching language tag, inventory digest, and result. P1a exits `0` only after all checks pass; it exits `20` with `input-source-missing`, `21` with `input-source-activation-mismatch`, or `22` with `input-source-inventory-invalid`, writes its selection result, and prevents either P1 allocation run. P1 must use the P1a selection record without reselecting the source, then independently reconfirm the current source as the continuity check defined below. P1 must preserve a preliminary `setMarkedText` callback for that already active source in each allocation's validation record before running any V5 matrix cell. If that callback is absent, P1 writes `cjk-composition-not-observed`, exits with the allocation-specific code, doesn't run V5 for that allocation, and retains B001-04 as KU. P1 must not substitute an unenumerated ID. The P1 implementation must preserve the Apple Text Input Source Services declaration excerpts for every TIS property, type constant, and function named in this procedure beside its transcript; S23 is the fetched Apple source for source IDs. See S23.

P1a uses the following command. It keeps selection ownership in P1a and hashes both produced raw JSON files even when the inventory process exits `20`, `21`, or `22`.

```sh
workdir=/tmp/wf-epic-b/OXY-B001/mac-ime-probe
cd "$workdir" || exit 24
mkdir -p output || exit 24
xcrun --sdk macosx clang -fobjc-arc -framework AppKit -framework Carbon ime_input_source_inventory.m -o ime_input_source_inventory
compile_status=$?
if [ "$compile_status" -ne 0 ]; then
  printf '%s\n' '{"result":"p1a-compile-failed"}' > output/p1a-compile-failure.json || exit 24
  shasum -a 256 output/p1a-compile-failure.json > output/p1a-raw-output.sha256 || exit 24
  exit 23
fi
set +e
./ime_input_source_inventory --enabled --selected-cjk --require-input-source-types kTISTypeKeyboardInputMode,kTISTypeKeyboardInputMethodModeEnabled --bcp47-primary-language zh,ja,ko --select --confirm-current-keyboard-input-source --output output/input-sources.json --selection-output output/input-source-selection.json
status=$?
shasum -a 256 output/input-sources.json output/input-source-selection.json > output/p1a-raw-output.sha256
write_status=$?
if [ "$write_status" -ne 0 ]; then
  exit 24
fi
exit "$status"
```

P1a writes `input-sources.json` and `input-source-selection.json` on every input-source result and uses exit `0`, `20`, `21`, or `22` as defined above. Exit `23` is `p1a-compile-failed`. Exit `24` is `p1a-evidence-write-failed`, including a missing raw result or sidecar write. The status capture and sidecar run after every inventory result, so a selection failure can't skip evidence preservation.

P1 has separate allocation invocations. The focused invocation is a standalone Objective-C AppKit probe with two `NSTextInputContext(client:)` instances, one per view, and contains no integrated candidate code. The integrated invocation is a P6-supplied host built from the frozen integrated fork and adapter inputs.

P6 has separate prebuild and post-build records. Before compilation, it must write `integrated-input-lock.json` with only the input keys `sourceDigestAlgorithm`, `qualificationLockSha256`, `forkCommit`, `forkSourceSha256`, `adapterCommit`, `adapterSourceSha256`, and `toolchainIdentities`. `toolchainIdentities` is a nonempty array sorted by `name`; every entry contains `name`, `version`, and `executableSha256`, and the array includes `git` plus every compiler, linker, SDK driver, build tool, and archive extractor that extracts a source archive or creates the executable. The prebuild lock contains no executable, artifact-manifest, attestation, or other post-build path or digest.

Neither the existing `artifact-manifest.schema.json` nor the qualification lock defines a source-digest algorithm: the artifact manifest records source commits, a qualification-lock digest, and file SHA-256 values, while the qualification lock records only the two candidate commits. P6 therefore defines `git-archive-tar-sha256-v2`; `git-archive-tar-sha256-v1` is not an acceptable input-lock value because it did not define one byte stream.

For each source root, P6 must require a clean ordinary nonbare Git worktree, reject every gitlink, and require `git -C "$ROOT" status --porcelain=v1 --untracked-files=all` to produce no bytes. In the following normative commands, `ROOT` is that absolute worktree, `COMMIT` is its frozen lower-case commit from `sourcePins.integratedFork.commit` or `sourcePins.oxyflutAdapter.commit`, `WORKDIR` is `/tmp/wf-epic-b/OXY-B001/integrated-inventory`, and `ARCHIVE` is either `$WORKDIR/fork-source.tar` or `$WORKDIR/adapter-source.tar`. The commands use the sole tree operand `"$COMMIT"`; no path operands, `--worktree-attributes`, compression format, or external tar implementation participate in the source digest.

```sh
set -o pipefail
mkdir -p "$WORKDIR" || exit 84
test "$(git -C "$ROOT" rev-parse --is-inside-work-tree)" = true || exit 82
test "$(git -C "$ROOT" rev-parse --is-bare-repository)" = false || exit 82
test -z "$(git -C "$ROOT" status --porcelain=v1 --untracked-files=all)" || exit 82
test "$(git -C "$ROOT" rev-parse HEAD)" = "$COMMIT" || exit 82
git -C "$ROOT" ls-tree -r "$COMMIT" > "$WORKDIR/tree.txt" || exit 82
if LC_ALL=C grep -q '^160000 ' "$WORKDIR/tree.txt"; then exit 82; fi
test ! -s "$(git -C "$ROOT" rev-parse --git-path info/attributes)" || exit 82
test -z "$(git -C "$ROOT" config --get-all core.attributesFile)" || exit 82
LC_ALL=C TZ=UTC git -C "$ROOT" -c tar.umask=0002 archive --format=tar --prefix=src/ "$COMMIT" > "$ARCHIVE" || exit 82
sha256sum "$ARCHIVE"
```

The first whitespace-delimited field from `sha256sum "$ARCHIVE"` is the only canonical-source digest and becomes `forkSourceSha256` or `adapterSourceSha256`. The saved `ARCHIVE` bytes, rather than either live worktree, are the only source material from which P6 may extract and compile the integrated host. Extraction is a later build operation and cannot redefine the source digest; its executable identity belongs in `toolchainIdentities`.

The normative command fixes the Git format, prefix, commit operand, tar file-mode policy, and commit-derived archive time. Git documents that `git archive` writes the named tree to standard output and, for a commit rather than a tree, uses the recorded commit time. The archive is deterministic for that recorded Git executable, commit, and effective committed attributes. The caveat is material: committed `.gitattributes` can apply `export-ignore`, which removes paths, or `export-subst`, which expands placeholders. P6 deliberately hashes that exported archive, not an assumed raw checkout; it rejects local `info/attributes` and `core.attributesFile` overrides and doesn't pass `--worktree-attributes`. See S32.

After both source archives and all toolchain identities are known but before compilation, P6 writes the input lock atomically as UTF-8 with LF line endings, then writes the SHA-256 of those exact bytes to `integrated-input-lock.json.sha256`. P6 embeds an `integrated-build-provenance.json` blob in the integrated probe executable at build time. That blob contains the source-digest algorithm, both input commit and source-digest pairs, the qualification-lock SHA-256, and the input-lock SHA-256. It contains no executable, artifact-manifest, or attestation digest.

After compilation, P6 hashes the executable, then writes a schema-valid artifact manifest for that executable. Its `source.candidateCommit`, `source.adapterCommit`, and `qualificationLockDigest` must equal the prebuild lock's fork commit, adapter commit, and qualification-lock SHA-256, and its `files` entry for the executable must contain the executable path and SHA-256. P6 then hashes the completed artifact manifest and writes `integrated-build-attestation.json` atomically as UTF-8 with LF line endings, followed by `integrated-build-attestation.json.sha256`. The attestation contains exactly `inputLockPath`, `inputLockSha256`, `executablePath`, `executableSha256`, `artifactManifestPath`, and `artifactManifestSha256`. The executable digest, artifact-manifest digest, and every other post-build digest appear only in this post-build attestation or the artifact manifest, never in the hashed input lock. This one-way sequence prevents a cryptographic fixed point.

Before it initializes either view, the integrated runner receives the prebuild input lock and sidecar, the qualification lock, and the post-build attestation and sidecar. It recomputes the exact-byte attestation SHA-256 and requires equality with `integrated-build-attestation.json.sha256` before parsing its fields. It then performs two independent input comparisons: it recomputes the exact-byte `integrated-input-lock.json` SHA-256 and requires equality with `integrated-input-lock.json.sha256` and with `inputLockSha256` returned by the executable's `--build-provenance-json`; separately, it recomputes the exact-byte `qualification-lock.json` SHA-256 and requires equality with the input lock's `qualificationLockSha256` field. It must never compare either file's digest with the other's identity field.

The runner then compares the input lock's fork and adapter commits with `sourcePins.integratedFork.commit` and `sourcePins.oxyflutAdapter.commit`, rejecting an unresolved status or nonmatching commit. It requires the embedded provenance's source-digest algorithm, two input commit-and-digest pairs, qualification-lock SHA-256, and input-lock SHA-256 to agree exactly with the input lock. It recomputes the executable SHA-256 and requires it to equal `integrated-build-attestation.json`'s `executableSha256`; it recomputes the artifact-manifest SHA-256 and requires it to equal the attestation's `artifactManifestSha256`; and it requires the manifest's source, qualification-lock, executable path, and executable SHA-256 fields to agree with the input lock and attestation. It also requires the attestation's `inputLockSha256` to equal the recomputed input-lock digest. On a preflight failure, it must write a transcript preflight record and validation record before returning. A mismatch fails before view initialization with exit `40` and `integrated-input-lock-invalid`.

P1 doesn't select an input source. Before it initializes a view, observes a preliminary callback, or runs a matrix action, each focused and integrated allocation invocation must call `TISCopyCurrentKeyboardInputSource`, read `kTISPropertyInputSourceID`, and compare that observed ID with `selectedInputSourceId` from the P1a selection record. Each invocation must write its first transcript record with `event: "input-source-continuity-check"`, the P1a selected ID, the observed ID, and the comparison result. If the call returns no source or the IDs differ, the focused run exits `34` with `input-source-changed-since-p1a` and the integrated run exits `44` with the same result. Each must write its validation record and raw-output sidecar, invoke no preliminary callback, and run no matrix action. This check detects a source change between P1a and either allocation without moving selection ownership out of P1a.

In the integrated invocation, the host must translate each `NSTextInputClient` callback into `OxyPlatformEvent` with its `OxyImeEvent` payload and deliver it to `OxySubstrateCallbacks.on_platform_event` from `oxyflut-substrate.h` before the capture hook records it. The integrated transcript header and validation record must contain `integratedInputLockSha256` from the runner's recomputation. Each allocation transcript record contains monotonic timestamp, allocation, vector, action, native client identity, view ID, view generation, input-source ID, input-source inventory digest, UTF-16 input and output ranges, callback or command name, geometry, redaction flag, expected assertion ID, result, and no raw secure-field text.

The action stimuli and assertions are as follows:

| Code | Stimulus | Required assertion |
| :-- | :-- | :-- |
| `CSU` | Make the view first responder and send composition start and update events. | The client records a marked range and selected range in UTF-16 units; the model has the same marked text and selection. |
| `RPL` | Send `insertText(_:replacementRange:)` with the fixture replacement range. | Only the requested UTF-16 range changes; the resulting text, selection, and view generation match the model. |
| `SMT` | Send `setMarkedText(_:selectedRange:replacementRange:)`. | The model contains marked text with the exact selected and replacement ranges. |
| `UM` | Call `unmarkText()` after `SMT`. | Marking ends without inserting an extra committed character. |
| `CMT` | Commit the active composition with `insertText(_:replacementRange:)`. | The marked range clears and the committed model text equals the expected text. |
| `CAN` | Call `discardMarkedText()` after a composition update. | The marked range clears, the precomposition text and selection return, and no commit callback changes text. |
| `DEL` | Send the deletion command through `doCommand(by:)` while the fixture exposes surrounding text. | The exact expected grapheme or selected UTF-16 range is removed; no adjacent surrogate half or combining mark is split. |
| `XFER` | Move first responder from view A to view B, deactivate A's context, and activate B's context. | Subsequent callbacks name B and B's generation; A receives no later edit, candidate rectangle, or commit. |
| `GEO` | Request `firstRect(forCharacterRange:actualRange:)`, then invalidate coordinates after changing geometry. | The candidate rectangle is for the requested UTF-16 range, has the expected screen-space transform, and refreshes after invalidation. |
| `U16` | Round-trip each native range through checked logical positions. | Valid boundaries round-trip exactly; a range inside a surrogate pair, combining sequence, or invalid span returns a structured conversion error. |

Table 2 supplies a pass or fail rule for every vector and action. `PASS=<code>(Vn)` means the action's required assertion in the preceding table passes for the named vector; `FAIL=not <code>(Vn)` means any missing callback, wrong text or range, wrong view generation, unredacted secure text, wrong geometry, or missing structured conversion error fails that cell.

| Vector | Composition start/update | Replace | Set marked text | Unmark | Commit | Cancel | Delete surrounding | Two-view focus transfer | Candidate rectangle geometry | Checked UTF-16 conversion |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |
| ASCII `V1` | `PASS=CSU(V1); FAIL=not CSU(V1)` | `PASS=RPL(V1); FAIL=not RPL(V1)` | `PASS=SMT(V1); FAIL=not SMT(V1)` | `PASS=UM(V1); FAIL=not UM(V1)` | `PASS=CMT(V1); FAIL=not CMT(V1)` | `PASS=CAN(V1); FAIL=not CAN(V1)` | `PASS=DEL(V1); FAIL=not DEL(V1)` | `PASS=XFER(V1); FAIL=not XFER(V1)` | `PASS=GEO(V1); FAIL=not GEO(V1)` | `PASS=U16(V1); FAIL=not U16(V1)` |
| Emoji grapheme `V2` | `PASS=CSU(V2); FAIL=not CSU(V2)` | `PASS=RPL(V2); FAIL=not RPL(V2)` | `PASS=SMT(V2); FAIL=not SMT(V2)` | `PASS=UM(V2); FAIL=not UM(V2)` | `PASS=CMT(V2); FAIL=not CMT(V2)` | `PASS=CAN(V2); FAIL=not CAN(V2)` | `PASS=DEL(V2); FAIL=not DEL(V2)` | `PASS=XFER(V2); FAIL=not XFER(V2)` | `PASS=GEO(V2); FAIL=not GEO(V2)` | `PASS=U16(V2); FAIL=not U16(V2)` |
| Combining sequence `V3` | `PASS=CSU(V3); FAIL=not CSU(V3)` | `PASS=RPL(V3); FAIL=not RPL(V3)` | `PASS=SMT(V3); FAIL=not SMT(V3)` | `PASS=UM(V3); FAIL=not UM(V3)` | `PASS=CMT(V3); FAIL=not CMT(V3)` | `PASS=CAN(V3); FAIL=not CAN(V3)` | `PASS=DEL(V3); FAIL=not DEL(V3)` | `PASS=XFER(V3); FAIL=not XFER(V3)` | `PASS=GEO(V3); FAIL=not GEO(V3)` | `PASS=U16(V3); FAIL=not U16(V3)` |
| Bidirectional selection `V4` | `PASS=CSU(V4); FAIL=not CSU(V4)` | `PASS=RPL(V4); FAIL=not RPL(V4)` | `PASS=SMT(V4); FAIL=not SMT(V4)` | `PASS=UM(V4); FAIL=not UM(V4)` | `PASS=CMT(V4); FAIL=not CMT(V4)` | `PASS=CAN(V4); FAIL=not CAN(V4)` | `PASS=DEL(V4); FAIL=not DEL(V4)` | `PASS=XFER(V4); FAIL=not XFER(V4)` | `PASS=GEO(V4); FAIL=not GEO(V4)` | `PASS=U16(V4); FAIL=not U16(V4)` |
| CJK composition `V5` | `PASS=CSU(V5); FAIL=not CSU(V5)` | `PASS=RPL(V5); FAIL=not RPL(V5)` | `PASS=SMT(V5); FAIL=not SMT(V5)` | `PASS=UM(V5); FAIL=not UM(V5)` | `PASS=CMT(V5); FAIL=not CMT(V5)` | `PASS=CAN(V5); FAIL=not CAN(V5)` | `PASS=DEL(V5); FAIL=not DEL(V5)` | `PASS=XFER(V5); FAIL=not XFER(V5)` | `PASS=GEO(V5); FAIL=not GEO(V5)` | `PASS=U16(V5); FAIL=not U16(V5)` |
| Replacement range `V6` | `PASS=CSU(V6); FAIL=not CSU(V6)` | `PASS=RPL(V6); FAIL=not RPL(V6)` | `PASS=SMT(V6); FAIL=not SMT(V6)` | `PASS=UM(V6); FAIL=not UM(V6)` | `PASS=CMT(V6); FAIL=not CMT(V6)` | `PASS=CAN(V6); FAIL=not CAN(V6)` | `PASS=DEL(V6); FAIL=not DEL(V6)` | `PASS=XFER(V6); FAIL=not XFER(V6)` | `PASS=GEO(V6); FAIL=not GEO(V6)` | `PASS=U16(V6); FAIL=not U16(V6)` |
| Candidate geometry `V7` | `PASS=CSU(V7); FAIL=not CSU(V7)` | `PASS=RPL(V7); FAIL=not RPL(V7)` | `PASS=SMT(V7); FAIL=not SMT(V7)` | `PASS=UM(V7); FAIL=not UM(V7)` | `PASS=CMT(V7); FAIL=not CMT(V7)` | `PASS=CAN(V7); FAIL=not CAN(V7)` | `PASS=DEL(V7); FAIL=not DEL(V7)` | `PASS=XFER(V7); FAIL=not XFER(V7)` | `PASS=GEO(V7); FAIL=not GEO(V7)` | `PASS=U16(V7); FAIL=not U16(V7)` |
| Secure field `V8` | `PASS=CSU(V8); FAIL=not CSU(V8)` | `PASS=RPL(V8); FAIL=not RPL(V8)` | `PASS=SMT(V8); FAIL=not SMT(V8)` | `PASS=UM(V8); FAIL=not UM(V8)` | `PASS=CMT(V8); FAIL=not CMT(V8)` | `PASS=CAN(V8); FAIL=not CAN(V8)` | `PASS=DEL(V8); FAIL=not DEL(V8)` | `PASS=XFER(V8); FAIL=not XFER(V8)` | `PASS=GEO(V8); FAIL=not GEO(V8)` | `PASS=U16(V8); FAIL=not U16(V8)` |

The focused P1 command follows. It captures the probe status before it hashes the P1a inventory and selection records plus every focused raw output.

```sh
workdir=/tmp/wf-epic-b/OXY-B001/mac-ime-probe
cd "$workdir" || exit 33
mkdir -p output/focused || exit 33
xcrun --sdk macosx clang -fobjc-arc -framework AppKit -framework Carbon ime_focused_probe.m -o ime_focused_probe
compile_status=$?
if [ "$compile_status" -ne 0 ]; then
  printf '%s\n' '{"result":"focused-ime-compile-failed"}' > output/focused/compile-failure.json || exit 33
  shasum -a 256 output/input-sources.json output/input-source-selection.json output/focused/compile-failure.json > output/focused/raw-output.sha256 || exit 33
  exit 32
fi
set +e
./ime_focused_probe --allocation focused --two-views --input-source-selection output/input-source-selection.json --input-source-inventory output/input-sources.json --require-current-keyboard-input-source-before-preliminary --require-preliminary-set-marked-text --matrix ime-matrix.json --jsonl output/focused/transcript.jsonl --validation output/focused/validation.json
status=$?
shasum -a 256 output/input-sources.json output/input-source-selection.json output/focused/transcript.jsonl output/focused/validation.json > output/focused/raw-output.sha256
write_status=$?
if [ "$write_status" -ne 0 ]; then
  exit 33
fi
exit "$status"
```

The focused run exits `0` only when its continuity check, preliminary callback, and every matrix validation pass. It exits `30` with `cjk-composition-not-observed` before V5, `31` with `focused-ime-validation-failed` after a matrix failure, `32` with `focused-ime-compile-failed`, `33` with `focused-ime-evidence-write-failed`, or `34` with `input-source-changed-since-p1a`. Every noncompile result writes `output/focused/validation.json`; the sidecar failure code takes precedence if an expected raw file or its sidecar can't be written.

The integrated P1 command runs only after P6. It captures the runner status before it hashes the P1a records, the P6 prebuild input lock and sidecar, the P6 post-build attestation and sidecar, and every integrated raw output.

```sh
repo_root=/home/oscar/GitHub/oxyflut
workdir=/tmp/wf-epic-b/OXY-B001/mac-ime-probe
lock=/tmp/wf-epic-b/OXY-B001/integrated-inventory/integrated-input-lock.json
attestation=/tmp/wf-epic-b/OXY-B001/integrated-inventory/integrated-build-attestation.json
cd "$workdir" || exit 45
mkdir -p output/integrated || exit 45
set +e
./run-integrated-ime-probe --input-lock "$lock" --build-attestation "$attestation" --qualification-lock "$repo_root/.constitution/tech-spec/contracts/qualification-lock.json" --allocation integrated --two-views --input-source-selection output/input-source-selection.json --input-source-inventory output/input-sources.json --c-abi-contract "$repo_root/.constitution/tech-spec/contracts/oxyflut-substrate.h" --require-build-provenance-json --require-current-keyboard-input-source-before-preliminary --require-nstextinputclient-through-oxy-platform-event --require-preliminary-set-marked-text --matrix ime-matrix.json --jsonl output/integrated/transcript.jsonl --validation output/integrated/validation.json
status=$?
shasum -a 256 output/input-sources.json output/input-source-selection.json "$lock" "$lock.sha256" "$attestation" "$attestation.sha256" output/integrated/transcript.jsonl output/integrated/validation.json > output/integrated/raw-output.sha256
write_status=$?
if [ "$write_status" -ne 0 ]; then
  exit 45
fi
exit "$status"
```

The runner must validate the P6 prebuild input lock, its sidecar, the qualification lock, the post-build attestation, the artifact manifest, the executable digest, and the executable's embedded provenance before it initializes either view. It must apply the two distinct input-lock and qualification-lock comparisons defined above; artifact and executable output checks come from the post-build attestation, not from the input lock. It exits `0` only when those checks, the input-source continuity check, C-ABI route, preliminary callback, and every matrix validation pass. It exits `40` with `integrated-input-lock-invalid`, `41` with `nstextinputclient-c-abi-route-missing`, `42` with `cjk-composition-not-observed` before V5, `43` with `integrated-ime-validation-failed`, `44` with `input-source-changed-since-p1a`, or `45` with `integrated-ime-evidence-write-failed`. Every nonzero result retains B001-04 as KU. The sidecar failure code takes precedence if an expected raw file, input-lock digest, attestation, or sidecar can't be written.

### Accessibility qualification corpus

CAP-SEM-001 requires every applicable role-specific property, relation, state, value, geometry, text range, traversal rule, and view identity. CAP-SEM-002 requires reverse routing to a live view and node with an acknowledgement or stale-target error. The two corresponding flows also require incremental insert, update, and delete behavior and prohibit retargeting stale input.

#### Stage 3 semantic-role decision and P2R registry freeze

The inspected candidate-neutral inputs don't enumerate a role identifier. `.constitution/prd/capabilities.md` gives CAP-SEM-001's role-specific coverage and CAP-SEM-002's reverse-action requirement, while the two semantics flows define mapping, update, action, and stale-target behavior. The inspected `accessibility-map.schema.json` defines `forward.roles` as one `mapping` object with `oxyflut`, `native`, and `status`; it can't represent every role in a registry. Therefore an `explicit-role-values-v1` freeze over those inputs necessarily yields an empty set. It must emit `role-registry-empty`; it must not treat that empty result as complete coverage.

D0 is an upstream Stage 3 decision, not a P2R probe. Before P2R, Stage 3 must add the candidate-neutral `contracts/semantic-role-registry.json` contract and its schemas specified in [Spec edits required](#spec-edits-required). Stage 3 must choose the closed Oxyflut role set from a recorded crosswalk of Apple `NSAccessibility.Role`, Microsoft UI Automation Control Types, and AT-SPI `AtspiRole`; native vocabularies inform the crosswalk but don't let either candidate define or expand the set. Every `roles` record must contain a lower-kebab-case `name`, a stable nonnegative `u32` `code`, an `ax` object with `role` and nullable `subrole` strings naming the `NSAccessibility.Role` and optional `NSAccessibility.Subrole`, a `uia` UI Automation Control Type string, and an `atspi` `AtspiRole` string. The registry's `codeStabilityRule` is `u32-append-only-never-reuse-v1`; code changes, deletions, and reuse require a compatibility migration that preserves the prior code assignment. The codes bind the existing `role: u32` fields in `contracts/oxyflut-public.rs` `SemanticsNode`, `contracts/oxyflut-substrate.rs` `SemanticsNode`, and `contracts/oxyflut-substrate.h` `OxySemanticsNode`. One generator must read that registry and regenerate the Rust `#[repr(u32)] SemanticRole` enums in the two Rust contracts and the C `OXY_SEMANTICS_ROLE_*` `uint32_t` constants in the header; a generated-contract test must reject a hand-edited or unequal code. Apple documents named `NSAccessibility.Role` symbols, Microsoft describes UI Automation control types as well-known control identifiers, and AT-SPI identifies `AtspiRole` as its role-value enumeration. See S28, S29, and S30.

Stage 3 must migrate `accessibility-map.schema.json`; it must not retain the one-object `forward.roles` shape. The registry remains the candidate-neutral catalog with vocabulary provenance, while the migrated map schema validates each candidate's per-role mappings. The atomic compatibility migration changes the map `$id` from `urn:oxyflut:schema:accessibility-map:5` to `urn:oxyflut:schema:accessibility-map:6`, changes its root `schemaVersion` constant from `5.0.0` to `6.0.0`, and makes `forward.roles` a nonempty keyed object. Each key is a lower-kebab-case registry role name and each value is exactly the existing `mapping` shape: nonempty `oxyflut`, nonempty `native`, and `status` of `kk` or `ku-gating`. The `forward` pattern that applies the single `mapping` shape must exclude `roles`; the new `roles` property alone applies `mapping` to each keyed member. The migration requires a migration note and contract tests under the compatibility policy. P2 must validate each candidate map against that complete version-6 schema and separately validate exact coverage of every frozen registry role.

After D0 succeeds, P2R must validate the source contract against `semantic-role-registry.schema.json`, recompute the SHA-256 of the exact source-registry bytes, and write the immutable snapshot at `qualification/fixtures/external-contracts/accessibility/candidate-neutral-role-registry.json` and its sidecar digest at `qualification/fixtures/external-contracts/accessibility/candidate-neutral-role-registry.json.sha256`. It must validate the snapshot against `semantic-role-registry-snapshot.schema.json`. The snapshot must contain `schemaVersion`, `sourceRegistryPath`, `sourceRegistrySha256`, `roleEnumerationRule`, `codeStabilityRule`, `roleVocabularySources`, and the sorted nonempty `roles` array of complete role records. The rule is `stage3-semantic-role-registry-v1`: copy each declared role record byte-for-byte in canonical JSON, sort by `name` bytewise, reject duplicate `name` or `code` values, and don't infer a role from a field name, a native platform role, or either candidate. P2R command: `cd /tmp/wf-epic-b/OXY-B001/mac-accessibility-registry && ./validate-and-freeze-role-registry.sh --registry /home/oscar/GitHub/oxyflut/.constitution/tech-spec/contracts/semantic-role-registry.json --registry-schema /home/oscar/GitHub/oxyflut/.constitution/tech-spec/data-models/semantic-role-registry.schema.json --snapshot-schema /home/oscar/GitHub/oxyflut/.constitution/tech-spec/data-models/semantic-role-registry-snapshot.schema.json --output /home/oscar/GitHub/oxyflut/qualification/fixtures/external-contracts/accessibility/candidate-neutral-role-registry.json --digest-output /home/oscar/GitHub/oxyflut/qualification/fixtures/external-contracts/accessibility/candidate-neutral-role-registry.json.sha256 --validation-output /tmp/wf-epic-b/OXY-B001/mac-accessibility-registry/role-registry-freeze-validation.json`. It exits `0` only after the source and snapshot schemas, source digest, copied role records, vocabulary records, code uniqueness, and code-stability rule pass. It exits `50` with `role-registry-source-invalid`, `51` with `role-registry-code-invalid`, `52` with `role-registry-snapshot-invalid`, or `53` with `role-registry-freeze-write-failed`, writing the validation output for every result. P2 must not start before this validation succeeds.

P2 can't claim a complete supported-role corpus before D0 creates the closed candidate-neutral registry and P2R freezes its digest. This is a gating KU, not an assumed empty role set. P2 must reject an output that omits any role from that frozen registry. B001-07 and B001-08 remain KU until D0, P2R, and P2 all succeed.

P2 can run only after D0 and P2R produce a nonempty, digested registry. The following corpus is frozen for P2's structure and coverage. It applies unchanged to focused and integrated output.

| Corpus part | Required fixture and assertion |
| :-- | :-- |
| Role coverage | Generate one live node for every role in the frozen candidate-neutral role registry, plus one root and one container. Reject an output unless the key set of `forward.roles` exactly equals the registry `roles[*].name` set and each `forward.roles[name]` has `oxyflut` exactly equal to `name`, a nonempty `native` identifier, and `status: "kk"`. |
| Forward schema coverage | Populate and validate every required `forward` key: `roles`, `states`, `actions`, `values`, `labels`, `accessibleNames`, `descriptions`, `hints`, `helpOrFullDescriptions`, `tooltips`, `attributedText`, `identifiers`, `bounds`, `transforms`, `traversal`, `labelledByRelations`, `describedByRelations`, `roleApplicableRelations`, `accessibilityFocus`, `inputFocus`, `hitTesting`, `textRanges`, `selection`, `scrollExtents`, `language`, `direction`, `headingLevels`, `liveRegions`, `hidden`, `disabled`, `secureFieldRedaction`, and `multiViewIsolation`. Reject any missing key, empty native mapping, or non-`kk` field in a claimed complete map. |
| Values and applicable states | Give each applicable role a false and true state, a changed value, disabled and hidden cases, and protected-content state. Reject a missing role-applicable value or state, an inapplicable state reported as applicable, or raw secure content. |
| Relations and traversal | Use labelled-by, described-by, parent-child, visible-child, and role-applicable relations. Give children a non-document order and a declared navigation order. Reject missing relation targets, wrong target view, duplicate identity, or navigation order that differs from the declared traversal. |
| Selection, caret, and text | Include text range, selected range, multiple selection if the native mapping declares it, caret position, visible range, and range geometry on ASCII, emoji, combining, and bidirectional text. Reject an index without its text-layout generation, a non-UTF-16 native range, or a range whose geometry belongs to another generation. |
| Geometry and transforms | Place equivalent nodes at nonzero transformed coordinates in two windows. Reject a frame that omits the transform, is not in screen coordinates when the mapping promises screen coordinates, or names the other view. |
| View identity and stale generation | Export two live views with distinct IDs and semantics generations, then delete one node, replace its generation, and invoke an action at the former target. Reject cross-view delivery; require the defined stale-target error. |
| Reverse actions | For every action reported by `forward.actions`, send accepted, rejected, invalid-payload, and stale-target invocations. Each record must include native identifier, complete payload encoding, text index unit, text-layout generation binding, view and node routing, acknowledgement, error result, and stale-target result as required by the schema. Reject an action lacking any field or acknowledgement. |

P2 writes one map per allocation with `schemaVersion: "6.0.0"`, `environment: "macos"`, its candidate name, `epistemicStatus: "kk-complete"`, the entire forward map, every reverse action, and an `evidence` array. Because the map root has `additionalProperties: false`, P2 must not add source-registry or P2R snapshot provenance as root properties. The `evidence` array must contain exactly one existing evidence-entry shape for the source registry: `path` equals `.constitution/tech-spec/contracts/semantic-role-registry.json` and `sha256` matches `^[0-9a-f]{64}$` over that file's exact bytes. It must also contain exactly one existing evidence-entry shape for the P2R snapshot: `path` equals `qualification/fixtures/external-contracts/accessibility/candidate-neutral-role-registry.json` and `sha256` matches `^[0-9a-f]{64}$` over that snapshot's exact bytes. Each entry has only `path` and `sha256`, as required by the existing `$defs/evidence`; P2 can add additional entries of that same shape for its traversal and action logs. Both output files must validate against the same complete version-6 `accessibility-map.schema.json`; P2 must not use a reduced focused schema or a reduced integrated schema. A VoiceOver traversal and action log is an additional behavioral check, not a substitute for schema validation.

The P2 command is `repo_root=/home/oscar/GitHub/oxyflut && source_registry="$repo_root/.constitution/tech-spec/contracts/semantic-role-registry.json" && source_schema="$repo_root/.constitution/tech-spec/data-models/semantic-role-registry.schema.json" && snapshot="$repo_root/qualification/fixtures/external-contracts/accessibility/candidate-neutral-role-registry.json" && snapshot_schema="$repo_root/.constitution/tech-spec/data-models/semantic-role-registry-snapshot.schema.json" && cd /tmp/wf-epic-b/OXY-B001/mac-accessibility-probe && mkdir -p output && ./validate-role-registry-inputs --registry "$source_registry" --registry-schema "$source_schema" --snapshot "$snapshot" --snapshot-schema "$snapshot_schema" --snapshot-digest "$snapshot.sha256" --validation-output output/registry-validation.json && xcrun --sdk macosx clang -fobjc-arc -framework AppKit accessibility_probe.m -o accessibility_probe && ./accessibility_probe --two-windows --schema "$repo_root/.constitution/tech-spec/data-models/accessibility-map.schema.json" --role-registry "$snapshot" --candidate focused --output output/focused-map.json --validation-output output/focused-validation.json && ./accessibility_probe --two-windows --schema "$repo_root/.constitution/tech-spec/data-models/accessibility-map.schema.json" --role-registry "$snapshot" --candidate integrated --through-c-abi --output output/integrated-map.json --validation-output output/integrated-validation.json`. Before compiling or starting either candidate, `validate-role-registry-inputs` validates the Stage 3 registry and the P2R snapshot against their respective schemas, recomputes the Stage 3 registry SHA-256 from its exact bytes and compares it to `snapshot.sourceRegistrySha256`, verifies the snapshot sidecar digest, and compares complete role records and `roleVocabularySources` for exact equality. It exits `0` only when every preflight check passes; it exits `60` with `role-registry-source-invalid`, `61` with `role-registry-snapshot-invalid`, `62` with `role-registry-source-digest-mismatch`, `63` with `role-registry-roles-mismatch`, or `64` with `role-registry-vocabulary-mismatch`, always writing `output/registry-validation.json`. The focused and integrated runs use `70` and `71`, respectively, for output-schema or corpus-validation failure. P2 must exit nonzero before either candidate run if any preflight check fails.

### Recovery qualification corpus

P5 uses actual operating-system and drawable conditions, not app activation, as recovery stimuli.

| Normalized case | Documented stimulus | Timestamp origin and pass criteria |
| :-- | :-- | :-- |
| Resize | Resize a visible window through a sequence ending in `NSWindow.didEndLiveResizeNotification`; record destination display refresh interval and resource-available timestamp. | From the later of final resize event and resource availability, acknowledge correctly sized output within two destination-display refresh intervals, as required by CON-REC-001. |
| Drawable unavailability | Retain all drawables in a `CAMetalLayer` pool, enable `allowsNextDrawableTimeout`, and request another drawable. Apple documents a one-second wait followed by `nil` when all drawables are in use; invalid layer properties also return `nil`. | Record the `nil` observation as the external surface-loss event. Acknowledge valid output within 250 ms, as required by CON-REC-002. |
| OS resume | Register on `NSWorkspace.shared.notificationCenter`, begin tracing, then have the host operator run `sudo pmset sleepnow` and wake the machine. The probe accepts only `NSWorkspace.didWakeNotification`, not an application-activation notification, as the resume event. | From the wake notification, acknowledge valid output within 500 ms, as required by CON-REC-003. |
| Display topology | Disconnect and reconnect a second display while two windows are visible, then record screen association and display epoch for each view. | From the operating-system topology event, acknowledge valid output within 500 ms, as required by CON-REC-003. |
| Recoverable graphics-device loss | Use only a documented, reproducible Metal action that completes a command buffer with terminal `MTLCommandBufferStatus.error` and a non-`nil` error. Do not substitute `MTLDeviceNotificationName.wasRemoved` on Apple Silicon. | From the external device-loss event, acknowledge valid output within 2 seconds, as required by CON-REC-004. If no such injection is available, emit `graphics-error-injection-unavailable` and retain the KU. |

For every normalized case and each allocation, P5 must assert the following: preserve an application-runtime state digest across recovery; record output-restoration timestamp and the applicable CON-REC-001 through CON-REC-004 deadline; measure transient graphics allocation at no more than 2x steady state under CON-REC-005; perform no more than three recreation attempts; emit a structured terminal error after the third failed attempt under CON-REC-006; and release every superseded resource within 500 ms after recovery success or terminal failure under CON-REC-007. The integrated run uses the identical normalized case IDs and assertions through the C ABI.

The P5 command is `cd /tmp/wf-epic-b/OXY-B001/mac-recovery-probe && xcrun --sdk macosx clang -fobjc-arc -framework AppKit -framework QuartzCore -framework Metal recovery_probe.m -o recovery_probe && ./recovery_probe --drawable-exhaustion --second-display --await-wake --jsonl recovery.jsonl && ./routing_probe --allocation focused --recovery-input recovery.jsonl && ./routing_probe --allocation integrated --through-c-abi --recovery-input recovery.jsonl`.

### Bounded follow-up probes

| Probe | Scope and command | Procedure and expected output |
| :-- | :-- | :-- |
| P1 | P1a and separate focused and integrated commands in [Input method editor qualification corpus](#input-method-editor-qualification-corpus). | P1a inventories, selects, confirms, and hashes the active source record with exit `0`, `20`, `21`, `22`, `23`, or `24`. Each allocation invocation rereads the current keyboard input source, records its observed ID before the preliminary callback, and fails `34` or `44` if it differs from P1a's selected ID. P1 hashes the selection record and every produced raw output after it captures each allocation status. The focused run is standalone. The integrated run follows P6, independently validates the prebuild input lock and qualification lock, then validates the post-build attestation, artifact manifest, executable digest, and embedded provenance, and records only after `OxySubstrateCallbacks.on_platform_event` receives the translated input method editor event. |
| D0 | Stage 3 semantic-role decision specified in [Spec edits required](#spec-edits-required). | Adds and validates the candidate-neutral closed role registry with complete `name`, stable `code`, `ax`, `uia`, and `atspi` records from the recorded AX, UIA, and AT-SPI vocabulary crosswalk. It also performs the version-6 per-role-map migration. This decision must complete before P2R; it is not a candidate probe. |
| P2R | `validate-and-freeze-role-registry.sh` command in [Stage 3 semantic-role decision and P2R registry freeze](#stage-3-semantic-role-decision-and-p2r-registry-freeze). | Validates the Stage 3 registry and the frozen snapshot against their separate schemas, records the source digest, and writes the snapshot and SHA-256 only when complete role records, vocabulary records, and stable unique codes pass. |
| P2 | `mac-accessibility-probe` command in [Accessibility qualification corpus](#accessibility-qualification-corpus), after D0 and P2R. | Before either candidate starts, validates the Stage 3 registry and snapshot against their schemas, recomputes and compares the source digest, and verifies exact equality of complete role and vocabulary-source records. It then validates focused and integrated version-6 maps against the same complete schema and records VoiceOver traversal and every declared reverse action. |
| P3 | `cd /tmp/wf-epic-b/OXY-B001/mac-timing-probe && xcrun --sdk macosx clang -fobjc-arc -framework AppKit -framework Metal timing_probe.m -o timing_probe && ./timing_probe --two-views --observer-process --move-displays --seconds 10 --jsonl timing.jsonl`. | Records observer and candidate process IDs, display identities, link timestamps, presentation times, and display epochs while each candidate stream is blocked. |
| P4 | `cd /tmp/wf-epic-b/OXY-B001/mac-routing-probe && ./routing_probe --allocation focused --two-windows --interleave --teardown && ./routing_probe --allocation integrated --through-c-abi --two-windows --interleave --teardown`. | Proves each request carries an owning view generation and stale target behavior. |
| P5 | `mac-recovery-probe` command in [Recovery qualification corpus](#recovery-qualification-corpus). | Runs drawable loss, real sleep and wake, display topology, resize, and available graphics-error cases through both allocations. |
| P6 | `repo_root=/home/oscar/GitHub/oxyflut && cd /tmp/wf-epic-b/OXY-B001/integrated-inventory && ./inventory.sh --qualification-lock "$repo_root/.constitution/tech-spec/contracts/qualification-lock.json" --fork-root "$FORK_ROOT" --adapter-root "$ADAPTER_ROOT" --inventory-output inventory.json --input-lock-output integrated-input-lock.json --artifact-manifest-output integrated-probe-artifact-manifest.json --build-attestation-output integrated-build-attestation.json`. `FORK_ROOT` and `ADAPTER_ROOT` name the two absolute local Git worktrees; the script rejects an unset, nonabsolute, dirty, non-Git, or submodule-containing root. | Reads the resolved integrated-fork and Oxyflut-adapter commits from the qualification lock, constructs the `git-archive-tar-sha256-v2` canonical source archives, writes and sidecars the input-only lock before compilation, compiles only from those saved archives, embeds the provenance blob, hashes the executable, writes the schema-valid executable artifact manifest, and finally writes and sidecars the post-build attestation. It emits the commit-bound macOS path and symbol inventory. It exits `0` only with both frozen identities, canonical-source digests, input-lock sidecar, provenance blob, executable manifest, post-build attestation, and complete inventory. It exits `80` with `integrated-input-pin-missing`, `81` with `integrated-inventory-incomplete`, `82` with `integrated-source-verification-failed`, `83` with `integrated-probe-compile-failed`, or `84` with `integrated-inventory-write-failed`. |
| P7 | `cd /tmp/wf-epic-b/OXY-B001/evidence-lock && ./lock.sh ../sources ../mac-* > manifest.json`. | Fetches each cited source again, preserves the exact fetched bytes under the evidence root, then computes fixture SHA-256 values and writes a manifest that verifies every path and digest. |
| P8 | `cd /tmp/wf-epic-b/OXY-B001/macos-availability && ./collect-apple-availability.sh > availability.json && ./verify-maximum-minimum.sh availability.json`. | Preserves authoritative availability for every `not stated` interface and either derives one verified maximum or retains B001-02 as a KU. |

## Options and trade-offs

- **Option A:** Freeze only the documented interface availability: Xcode 26.6 build `17F113`, macOS 26.5 SDK, AppKit text and accessibility interfaces, view-linked display timing, Metal presentation feedback, and recovery observations.
- **Option B:** Set macOS 14.0 as the deployment target before P8 completes.
- **Option C:** Retain each behavior, source-availability, mapping, independence, routing, recovery, and evidence-publication question as a gating KU with one bounded probe.

## Recommendation

- **Chosen option:** A/C mix. Choose A for B001-01, B001-03, B001-06, B001-09, B001-12, and B001-15. Choose C for B001-02, B001-04, B001-05, B001-07, B001-08, B001-10, B001-11, B001-13, B001-14, and B001-16 through B001-18.
- **Why it fits:** The recommendation treats only preserved, authoritative interface evidence as KK. It does not turn a platform API, a source listing, or a plausible deployment target into candidate behavior evidence. P1-P8 each specify a host, input, command, and expected output. P6 uses a one-way provenance chain: an input-only lock is embedded at build time, while the executable and artifact-manifest digests are recorded only after the build in a separate attestation.
- **Rejected option:** Reject B because the documented 14.0 display-link introduction does not prove the historical availability of every other required interface. Reject deprecated `CVDisplayLink`, candidate-internal clocks as independent meters, `MTLDeviceNotificationName.wasRemoved` on Apple Silicon, default-window routing, and a map or recovery claim without preserved traces.
- **Capability and architecture guard:** This recommendation preserves the P0 capabilities and the accepted Platform integration and reentrancy boundaries. It chooses no substrate and introduces no product capability or architecture boundary.
- **Stage 3 edits:** Apply only the exact deployment-target retentions and the D0 role-registry, per-role accessibility-map migration, generated-role-constant, traceability, and migration-note instructions in [Downstream impact](#downstream-impact). The P6 provenance correction is a report-runbook change and requires no current Stage 3 specification edit.

## Downstream impact

- **ADRs to write or update:** No architecture decision record change is required. Do not alter the accepted direct-AppKit or normalized-callback boundary in `ADR-0005-platform-hosts.md` or the execution boundary in `ADR-0006-execution-domains.md`.
- **Tickets unblocked in `tasks/active/`:** The documented source-interface portion of the macOS investigation is complete. Candidate implementation and measurement remain blocked by P1-P8.
- **Tickets to add or split:** Add D0 for the Stage 3 semantic-role decision before P2R, then add bounded work only for P1 input method editor behavior, P2 accessibility maps, P3 timing and migration, P4 service routing, P5 recovery injection, P6 integrated inventory, P7 evidence locking, and P8 historical availability.

### Spec edits required

B001-02 remains a gating KU, so Stage 3 must retain the deployment-target state until P8 produces one verified maximum-minimum record. The four atomic deployment-target edits are listed first. The accessibility-KU guard and D0 semantic-role prerequisite are separate requirements.

**Four atomic deployment-target edits**

| File and field or section | Exact instruction |
| :-- | :-- |
| `.constitution/tech-spec/stack.md` -> `Platform qualification pins` -> macOS row | Retain the exact phrase `minimum deployment target is a gating KU`. Do not replace it before P8 produces the verified maximum-minimum record. |
| `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.macos.minimumVersion` | Retain the exact current object `{"status":"ku-gating","value":null,"evidence":[]}`. Do not add an evidence URL string. |
| `.constitution/tech-spec/contracts/qualification-lock.json` -> `referenceEnvironments.macos-arm64.minimumVersion` | Retain `null` and retain `minimum-platform-and-protocol-versions` in both known-unknown arrays. |
| `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.macos.openQuestions` | Retain the exact entry `minimum deployment target`. Do not remove it before the first three changes are made in the same transaction after P8. |

**Accessibility-KU guard**

`.constitution/tech-spec/contracts/qualification-lock.json` -> `preImplementationKnownUnknowns` and `gatingKnownUnknowns`: Retain the exact entry `complete-ime-editing-geometry-and-accessibility-maps`. Do not remove it before P1, D0, P2R, and P2 preserve their required artifacts and validators pass.

**D0 semantic-role contract prerequisite**

| File and field or section | Exact instruction |
| :-- | :-- |
| `.constitution/tech-spec/data-models/semantic-role-registry.schema.json` -> new file | Create a Draft 2020-12 schema with `$id` `urn:oxyflut:schema:semantic-role-registry:1`, `additionalProperties: false`, and required `schemaVersion`, `codeStabilityRule`, `roleVocabularySources`, and `roles`. Set `schemaVersion` to `1.0.0` and `codeStabilityRule` to `u32-append-only-never-reuse-v1`. Require exactly three `roleVocabularySources` records with `environment`, `vocabulary`, and `url` strings. Require `roles` to be a nonempty array sorted by `name`; require validator-enforced unique `name` and `code`. Each role record has `additionalProperties: false` and required `name`, `code`, `ax`, `uia`, and `atspi`: `name` matches `^[a-z][a-z0-9-]*$`; `code` is an integer from `0` through `4294967295`; `ax` is an object with only required nonempty `role` string and nullable `subrole` string; `uia` and `atspi` are nonempty strings. |
| `.constitution/tech-spec/data-models/semantic-role-registry-snapshot.schema.json` -> new file | Create a Draft 2020-12 snapshot schema with `$id` `urn:oxyflut:schema:semantic-role-registry-snapshot:1`, `additionalProperties: false`, and required `schemaVersion`, `sourceRegistryPath`, `sourceRegistrySha256`, `roleEnumerationRule`, `codeStabilityRule`, `roleVocabularySources`, and `roles`. Set constants `schemaVersion: "1.0.0"`, `sourceRegistryPath: ".constitution/tech-spec/contracts/semantic-role-registry.json"`, `roleEnumerationRule: "stage3-semantic-role-registry-v1"`, and `codeStabilityRule: "u32-append-only-never-reuse-v1"`; require `sourceRegistrySha256` to match `^[0-9a-f]{64}$`; use the same exact vocabulary-source and role-record shapes as the source-registry schema. |
| `.constitution/tech-spec/contracts/semantic-role-registry.json` -> new file | Create a registry with `schemaVersion` equal to `1.0.0` and `codeStabilityRule` equal to `u32-append-only-never-reuse-v1`. Set exactly three `roleVocabularySources`: macOS `NSAccessibility.Role` at https://developer.apple.com/documentation/appkit/nsaccessibility-swift.struct/role, Windows `UI Automation Control Types` at https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-controltypesoverview, and Wayland and X11 `AtspiRole` at https://gnome.pages.gitlab.gnome.org/at-spi2-core/devel-docs/doc-org.a11y.atspi.Accessible.html. Set `roles` to a nonempty array sorted by `name`. Each exact record must contain the Stage 3 crosswalk's lower-kebab-case `name`, its stable unique `u32` `code`, `ax` with the selected `NSAccessibility.Role` and nullable selected `NSAccessibility.Subrole`, `uia` with the selected UI Automation Control Type, and `atspi` with the selected `AtspiRole`. Don't derive, add, remove, renumber, or reuse a role or code from either candidate implementation or a platform map. |
| `.constitution/tech-spec/data-models/accessibility-map.schema.json` -> schema identity, root `schemaVersion`, `forward.roles`, and `evidence` provenance | Perform one breaking compatibility migration: change `$id` from `urn:oxyflut:schema:accessibility-map:5` to `urn:oxyflut:schema:accessibility-map:6` and the root `schemaVersion` constant from `5.0.0` to `6.0.0`. Add `forward.properties.roles` as an object with `minProperties: 1`, `propertyNames.pattern` equal to `^[a-z][a-z0-9-]*$`, and `additionalProperties` referencing `#/$defs/mapping`. Replace the broad `forward.patternProperties` with a pattern that matches every required single-mapping field except `roles`, so `roles` isn't also validated as one `mapping`. Keep `mapping` unchanged. Retain root `additionalProperties: false` and the existing `$defs/evidence` entry shape. Add two root `allOf` constraints on `evidence`, each with `minContains: 1`, `maxContains: 1`, and a `contains` subschema for the existing evidence-entry shape: one requires `path` exactly `.constitution/tech-spec/contracts/semantic-role-registry.json`; the other requires `path` exactly `qualification/fixtures/external-contracts/accessibility/candidate-neutral-role-registry.json`. The existing `evidence.items` reference enforces that both entries have only that path and a 64-hex `sha256`. Do not add named root provenance properties. Add version-5-to-version-6 migration fixtures and contract tests for both required entries. |
| `.constitution/tech-spec/changelog.md` -> migration note | Add a migration note for the breaking `accessibility-map` 5.0.0 to 6.0.0 keyed-role change. State that each former one-object `forward.roles` value becomes one `forward.roles[role-name]` mapping, and that version-5 maps are rejected by version-6 validation. |
| `.constitution/tech-spec/contracts/oxyflut-public.rs`, `.constitution/tech-spec/contracts/oxyflut-substrate.rs`, and `.constitution/tech-spec/contracts/oxyflut-substrate.h` -> semantic role constants | Add a generated `#[repr(u32)] SemanticRole` enum to each Rust contract and generated `OXY_SEMANTICS_ROLE_*` `uint32_t` constants to the C header from `semantic-role-registry.json`; generate every numeric value from `roles[*].code` and forbid hand-authored numeric values. Retain the existing `SemanticsNode.role: u32` and `OxySemanticsNode.role: uint32_t` fields, and require each value to equal a generated role code. Add a generated-contract test that compares every registry record's `name` and `code` with all three generated artifacts. |
| `.constitution/tech-spec/contracts/capability-traceability.json` -> `mappings[capabilityId="CAP-SEM-001"].bindings` and `mappings[capabilityId="CAP-SEM-002"].bindings` | Add a binding to `contracts/semantic-role-registry.json` with symbols `#/codeStabilityRule`, `#/roleVocabularySources`, `#/roles/*/name`, `#/roles/*/code`, `#/roles/*/ax`, `#/roles/*/uia`, and `#/roles/*/atspi` to both mappings. In both mappings, add `SemanticRole` and `SemanticsNode.role` to the existing `oxyflut-public.rs` and `oxyflut-substrate.rs` bindings, and add `OXY_SEMANTICS_ROLE_*` and `OxySemanticsNode.role` to the existing `oxyflut-substrate.h` binding. |

### Accessibility-map version-6 landing inventory

The following inventory makes the D0 breaking migration land atomically. It doesn't change this report's recommendation, decision-register statuses, or KK and KU counts. A map with `epistemicStatus: "kk-complete"` must validate every keyed role mapping, not treat the `roles` object itself as one mapping.

The following source-tree search ran at 2026-08-28T20:22:55Z. The preserved output is trimmed to migration-relevant matches; the subsequent file reads verified the named constants, fixture contents, and shape consumers.

```text
$ grep -rnE 'accessibility-map:5|accessibility-map' xtask crates qualification .constitution/tech-spec
xtask/src/contracts/traceability/edges.rs:301:            "data-models/accessibility-map.schema.json",
xtask/src/contracts/traceability/edges.rs:311:            "data-models/accessibility-map.schema.json",
xtask/src/contracts/traceability/mod.rs:48:const ACCESSIBILITY_MAP_SCHEMA: &str = "urn:oxyflut:schema:accessibility-map:5";
xtask/src/commands/lock_tests.rs:604:                    .join("qualification/fixtures/contracts/accessibility-map/valid/minimal.json"),
xtask/src/commands/contracts.rs:16:    ".constitution/tech-spec/data-models/accessibility-map.schema.json";
crates/oxyflut-qualification/src/evidence/references.rs:65:            "accessibility-map"
qualification/fixtures/contracts/accessibility-map/invalid/superseded-identity.json:2:  "$schema": "urn:oxyflut:schema:accessibility-map:4",
qualification/fixtures/contracts/accessibility-map/invalid/superseded-identity.expected.json:5:  "supersededBy": "urn:oxyflut:schema:accessibility-map:5"
qualification/fixtures/contracts/supersession.json:4:      "name": "accessibility-map",
qualification/fixtures/contracts/supersession.json:5:      "superseded": "urn:oxyflut:schema:accessibility-map:4",
qualification/fixtures/contracts/supersession.json:6:      "current": "urn:oxyflut:schema:accessibility-map:5"
.constitution/tech-spec/contracts/capability-traceability.json:730:          "contract": "data-models/accessibility-map.schema.json",
.constitution/tech-spec/contracts/capability-traceability.json:806:          "contract": "data-models/accessibility-map.schema.json",
.constitution/tech-spec/data-models/README.md:7:| `accessibility-map.schema.json` | Records one candidate's complete forward property map and reverse action map for one environment. |
.constitution/tech-spec/data-models/README.md:34:The accessibility-map v5, artifact-manifest v4, capability-baseline v4, capability-traceability v3, qualification-evidence v5, raw-measurement v2, platform-contracts v5, and qualification-lock v5 identities supersede their earlier pre-evidence contracts.
.constitution/tech-spec/data-models/accessibility-map.schema.json:3:  "$id": "urn:oxyflut:schema:accessibility-map:5",
```

| File | Required Stage 3 change |
| :-- | :-- |
| `.constitution/tech-spec/data-models/accessibility-map.schema.json` | Apply the preceding `$id` and root `schemaVersion` migration to `urn:oxyflut:schema:accessibility-map:6` and `6.0.0`. Replace the single `forward.roles` mapping with a nonempty lower-kebab-case keyed object whose values reference `#/$defs/mapping`. Exclude `roles` from the single-mapping pattern, and change the `kk-complete` conditional so it requires `status: "kk"` for every keyed role mapping as well as every remaining required single-mapping field. Retain the two required registry-provenance evidence entries described above. |
| `.constitution/tech-spec/contracts/capability-traceability.json` | For both `CAP-SEM-001` and `CAP-SEM-002`, apply the preceding semantic-role-registry and generated-role-symbol binding additions. Expand each existing `data-models/accessibility-map.schema.json` binding to include `#/properties/forward/properties/roles` and both new root `allOf` provenance constraints. |
| `xtask/src/contracts/traceability/mod.rs` | Replace `ACCESSIBILITY_MAP_SCHEMA` exactly from `urn:oxyflut:schema:accessibility-map:5` to `urn:oxyflut:schema:accessibility-map:6`. |
| `xtask/src/contracts/traceability/edges.rs` | Remove `"roles"` from `REQUIRED_ACCESSIBILITY_CATEGORIES`, which represents only scalar forward mappings after v6. Add the required `contracts/semantic-role-registry.json` physical contract edge for both `CAP-SEM-001` and `CAP-SEM-002`, plus the generated `SemanticRole`, `OXY_SEMANTICS_ROLE_*`, and registry-pointer symbol edges that enforce the preceding traceability additions. |
| `xtask/src/contracts/traceability/validation.rs` | Keep the scalar-category loop for `REQUIRED_ACCESSIBILITY_CATEGORIES`. Add a separate `forward.roles` object check that rejects an empty object and requires every keyed role mapping to have `status: "kk"` when the referenced map is `kk-complete`; it must not request `forward.roles.status`. |
| `xtask/src/contracts/traceability/fixtures.rs` | In `nested-accessibility-ku`, change the expected schema pointer from `/forward/roles/status` to `/forward/roles/fixture-role/status`. Regenerate the two synthetic-map SHA-256 constants after their v6, keyed-role, and mandatory-evidence updates. |
| `xtask/src/contracts/traceability/tests.rs` | Update the matching nested-KU expected pointer to `/forward/roles/fixture-role/status` and regenerate the two hard-coded synthetic-map SHA-256 values after the source fixtures change. Retain the stale text-layout-generation assertion. |
| `qualification/fixtures/contracts/traceability/synthetic-accessibility-stale.json` | Change `schemaVersion` to `6.0.0`; replace the one `forward.roles` mapping with `forward.roles.fixture-role` containing that mapping; and add one evidence entry for `.constitution/tech-spec/contracts/semantic-role-registry.json` and one for `qualification/fixtures/external-contracts/accessibility/candidate-neutral-role-registry.json`, each with the actual Stage 3 file digest. Retain the stale `textLayoutBinding` value so the fixture still reaches that validator failure. |
| `qualification/fixtures/contracts/traceability/synthetic-accessibility-ku.json` | Change `schemaVersion` to `6.0.0`; replace the one `forward.roles` mapping with `forward.roles.fixture-role`; keep its inner `status: "ku-gating"`; and add the same two digest-correct mandatory registry evidence entries. This preserves its v6 conditional-schema failure at `/forward/roles/fixture-role/status`. |
| `xtask/src/commands/lock_tests.rs` | In `complete_accessibility_maps`, set every scalar forward mapping to `kk` but iterate the values of `forward.roles` separately. Replace the one proof-only evidence array with exactly one source-registry entry and one frozen-snapshot entry, computed from the copied temporary-root files, plus the existing proof entry; then hash each generated map after those entries are present. |
| `xtask/src/commands/contracts.rs` | No v5-to-v6 literal change is required: this file holds only the schema path. Keep its `accessibility-generation` summary deferred until the separate text-layout-generation decision is resolved; don't conflate that gate with the keyed-role migration. |
| `xtask/src/contracts/schema.rs` | Add a contract test that loads `qualification/fixtures/contracts/migration/accessibility-map-v5-to-v6.input.json`, derives the declared keyed role map, and byte-compares it with `accessibility-map-v5-to-v6.expected.json`. The test must then validate the expected v6 document and reject the v5 input through the v6 identity. Existing generic supersession validation must continue to validate the updated supersession fixture. |
| `qualification/fixtures/contracts/migration/accessibility-map-v5-to-v6.input.json` and `qualification/fixtures/contracts/migration/accessibility-map-v5-to-v6.expected.json` | Add the v5 source and v6 derived fixture pair used by the preceding test. The expected fixture sets `schemaVersion` to `6.0.0`, moves the former single `forward.roles` mapping to `forward.roles.fixture-role`, preserves the mapping fields byte-for-byte, and contains the two required registry evidence entries with test fixture digests. The input fixture retains the v5 identity and one-object `forward.roles` shape. |
| `qualification/fixtures/contracts/supersession.json` | Replace the accessibility-map entry's `superseded` value with `urn:oxyflut:schema:accessibility-map:5` and its `current` value with `urn:oxyflut:schema:accessibility-map:6`. |
| `qualification/fixtures/contracts/accessibility-map/invalid/superseded-identity.json` | Retain `$schema: "urn:oxyflut:schema:accessibility-map:4"` and `schemaVersion: "superseded"`; it remains the old-reader rejection input. |
| `qualification/fixtures/contracts/accessibility-map/invalid/superseded-identity.expected.json` | Replace `supersededBy` exactly from `urn:oxyflut:schema:accessibility-map:5` to `urn:oxyflut:schema:accessibility-map:6`. |
| `qualification/fixtures/contracts/accessibility-map/valid/minimal.json` | Change `schemaVersion` to `6.0.0`; replace the one `forward.roles` mapping with `forward.roles.fixture-role` containing that mapping; and replace the one generic evidence entry with the two required source-registry and frozen-snapshot entries. Use 64-hex fixture digests because schema fixtures validate shape only; `lock_tests.rs` replaces them with real digests before referenced-map validation. |
| `qualification/fixtures/contracts/accessibility-map/invalid/additional-properties.json` | Change `schemaVersion` to `6.0.0` and nest the existing role mapping at `forward.roles.fixture-role`; retain `unexpected: true` so the fixture continues to fail only for the extra root property. Its `.expected.json` sidecar remains unchanged. |
| `qualification/fixtures/contracts/accessibility-map/invalid/conditional.json` | Change `schemaVersion` to `6.0.0` and nest the existing KU role mapping at `forward.roles.fixture-role`. In `conditional.expected.json`, replace only the roles error path with `/forward/roles/fixture-role/status`; retain every other expected status path and the reverse-action path. |
| `qualification/fixtures/contracts/accessibility-map/invalid/enum.json` | Change `schemaVersion` to `6.0.0` and nest the role mapping at `forward.roles.fixture-role`; retain `environment: "unsupported"` so it continues to fail the environment enum. Its `.expected.json` sidecar remains unchanged. |
| `qualification/fixtures/contracts/accessibility-map/invalid/required.json` | Keep `schemaVersion` absent, and nest the role mapping at `forward.roles.fixture-role`; this preserves the sole required-property failure. Its `.expected.json` sidecar remains unchanged. |
| `qualification/fixtures/contracts/accessibility-map/invalid/type.json` | Retain numeric `schemaVersion: 1`, and nest the role mapping at `forward.roles.fixture-role`; this preserves the schema-version constant failure. Its `.expected.json` sidecar remains unchanged. |
| `.constitution/tech-spec/data-models/README.md` | Replace `accessibility-map v5` in the supersession summary with `accessibility-map v6` and state that v6 supersedes v5 because `forward.roles` is now a keyed role map with registry provenance. Retain the pre-evidence preservation statement. |
| `.constitution/tech-spec/changelog.md` | Add the preceding breaking 5.0.0-to-6.0.0 accessibility-map migration note, including old-reader rejection, the one-object-to-keyed-role transform, and the two required registry-provenance evidence entries. |
| `crates/oxyflut-qualification/src/evidence/references.rs` | No source change is required. `schema_family` strips the terminal version and therefore continues to classify both v5 and v6 as the `accessibility-map` reference-bearing family. Add a regression test only if the migration changes that version-independent behavior. |
| `crates/oxyflut-qualification/src/readiness.rs`, `xtask/src/commands/lock_tests.rs` known-unknown assertions, and `qualification/fixtures/readiness/**` and `qualification/fixtures/contracts/qualification-lock/**` | No accessibility-map schema migration is required. Retain the exact `complete-ime-editing-geometry-and-accessibility-maps` known-unknown string until P1, D0, P2R, and P2 meet the existing clearance rule. |

## Evidence preservation convention

The current `platform-contracts` schema accepts evidence objects with exactly `path` and a 64-hex `sha256`. Existing qualification fixtures use repository-relative paths with the same pair. The report preserves normalized source excerpts, not source bodies. Jina-proxied bodies are not byte-stable, so this report does not claim any source-body digest or predeclare a fixture digest. P7 must fetch each cited source, preserve the exact fetched bytes under `qualification/fixtures/external-contracts/macos/official-sources/`, record the origin and fetch URLs plus UTC timestamp in a manifest, and compute each fixture digest only after the fixture file exists. This report proposes the convention but creates no fixture files.

```json
{
  "path": "qualification/fixtures/external-contracts/macos/official-sources/s3-nsview-display-link.reader.md",
  "sha256": "<to-be-computed-by-P7>"
}
```

```json
{
  "path": "qualification/fixtures/external-contracts/macos/official-sources/s20-nsworkspace-did-wake.reader.md",
  "sha256": "<to-be-computed-by-P7>"
}
```

The proposed objects are intentionally not schema-valid until P7 replaces `<to-be-computed-by-P7>` with the digest of the preserved fixture. Stage 3 must not add either object to an evidence array before that replacement. A URL alone is not a schema-valid evidence item. P8 must produce its own exact source-body and manifest digest before any deployment-target object can change from `ku-gating` to `kk`.

## Sources

### Authoritative source records

Table 3 records every authoritative source used by a claim in this report. Each source was fetched through the Jina reader proxy (`https://r.jina.ai/<canonical URL>`) at the recorded UTC time; the table lists canonical source URLs, not proxy URLs. Each `Excerpt SHA-256` hashes only the corresponding fenced excerpt preserved below, not a fetched source body.

Source IDs S22, S24, and S25 are intentionally absent because their sources were withdrawn during review; the remaining IDs retain their original numbering.

| ID | Official source URL | Canonical source URL | UTC fetch | Excerpt SHA-256 |
| :-- | :-- | :-- | :-- | :-- |
| S1 | https://developer.apple.com/documentation/xcode-release-notes/xcode-26_6-release-notes | https://developer.apple.com/documentation/xcode-release-notes/xcode-26_6-release-notes | 2026-08-28T16:53:48Z | `eb2aa023070dbc6a88c1a796de5bad1788f628759542ef775a1551e9f2718a6e` |
| S2 | https://developer.apple.com/news/releases/?id=06252026a | https://developer.apple.com/news/releases/?id=06252026a | 2026-08-28T16:53:48Z | `b9f8ab4cc2e388065638b1fa1626b5cda18880dc976dac8969f2f186d66d52f0` |
| S3 | https://developer.apple.com/documentation/appkit/nsview/displaylink(target:selector:) | https://developer.apple.com/documentation/appkit/nsview/displaylink(target:selector:) | 2026-08-28T16:53:49Z | `b15a8f07d3136ffeb4b1320882bfaf5af02679b02c34936ec0c30c7bd3c06dcb` |
| S4 | https://developer.apple.com/documentation/appkit/nstextinputclient | https://developer.apple.com/documentation/appkit/nstextinputclient | 2026-08-28T16:53:50Z | `2de7feb44005e461202f9513177d8b107aeca811cb1779661a312f89bdcdea95` |
| S5 | https://developer.apple.com/documentation/appkit/nstextinputcontext | https://developer.apple.com/documentation/appkit/nstextinputcontext | 2026-08-28T16:53:50Z | `5bddd714383912afd6304b02b35d9dcb9a1aa8697d02b6282ba0045ecb7e50fe` |
| S6 | https://developer.apple.com/documentation/foundation/nsstring | https://developer.apple.com/documentation/foundation/nsstring | 2026-08-28T16:53:51Z | `63672a0b41b9624793644599fa49295bca91e787f317be56e258df5dae2a31bf` |
| S7 | https://developer.apple.com/documentation/appkit/nsaccessibilityprotocol | https://developer.apple.com/documentation/appkit/nsaccessibilityprotocol | 2026-08-28T16:53:52Z | `a679020d542e4317acf48f2d123030e9b416153085a88caaf9025c9d1a299ae8` |
| S8 | https://developer.apple.com/documentation/appkit/accessibility-for-appkit | https://developer.apple.com/documentation/appkit/accessibility-for-appkit | 2026-08-28T16:53:53Z | `b3ef2c0a7af6174961ef249063c5c54280bfa95e9eb35cb77a13b0e8918bcfe6` |
| S9 | https://developer.apple.com/documentation/metal/mtldrawable/addpresentedhandler(_:) | https://developer.apple.com/documentation/metal/mtldrawable/addpresentedhandler(_:) | 2026-08-28T16:53:54Z | `2b378706dbb4d4f4a7dd10058b38b9643c258c475f53b764d893e9a685e1d6b2` |
| S10 | https://developer.apple.com/documentation/metal/mtldrawable/presentedtime | https://developer.apple.com/documentation/metal/mtldrawable/presentedtime | 2026-08-28T16:53:54Z | `c00c93e2f2cdcc307d3bae3bbedd4fcc5ca6e65ad61b3b0d0be03fa463645a15` |
| S11 | https://developer.apple.com/documentation/appkit/nswindow/didchangescreennotification | https://developer.apple.com/documentation/appkit/nswindow/didchangescreennotification | 2026-08-28T16:53:55Z | `273db18cc424b8e8bb8e4148b4c5881a42ba38d9bed05c6b675819fc9fc1b32b` |
| S12 | https://developer.apple.com/documentation/appkit/nsapplication/didbecomeactivenotification | https://developer.apple.com/documentation/appkit/nsapplication/didbecomeactivenotification | 2026-08-28T16:53:56Z | `3e5e987460dedce7f9f7d7d8caeb3ba1b69b9e50aaad31626d9a40bc339eed79` |
| S13 | https://developer.apple.com/documentation/appkit/nswindow/willstartliveresizenotification | https://developer.apple.com/documentation/appkit/nswindow/willstartliveresizenotification | 2026-08-28T16:53:56Z | `1183e427c94e217a5d126677835687cd1a7ec7d8b8285f9cfc05ca0bd36f530c` |
| S14 | https://developer.apple.com/documentation/metal/mtlcommandbuffer/error | https://developer.apple.com/documentation/metal/mtlcommandbuffer/error | 2026-08-28T16:53:57Z | `bfb9065f88792bbdf37be1a81bebe2f7161e3aef938ab9bc300aa3fd30a52dd1` |
| S15 | https://developer.apple.com/documentation/metal/mtlcommandbuffer/status | https://developer.apple.com/documentation/metal/mtlcommandbuffer/status | 2026-08-28T16:53:58Z | `22f2a1fba9ac9c4ddd2ffeb683685b2e371801a208624d6ba59392ef1b479d8a` |
| S16 | https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:) | https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:) | 2026-08-28T16:53:58Z | `329501029ae1be4316d9068045342bc7a0666ccc812f730ab321a2a7cdd13241` |
| S17 | https://developer.apple.com/documentation/corevideo/cvdisplaylink | https://developer.apple.com/documentation/corevideo/cvdisplaylink | 2026-08-28T16:53:59Z | `0667fda62ceccd40b5beb28db58ce1e82315263b72fce0527e3277ba52142847` |
| S18 | https://developer.apple.com/documentation/metal/mtldevicenotificationname/wasremoved | https://developer.apple.com/documentation/metal/mtldevicenotificationname/wasremoved | 2026-08-28T16:54:00Z | `a42f1c7accf2a09bd232bfa80df64f90b38386b01662216d8e517c2a353c6628` |
| S19 | https://developer.apple.com/documentation/appkit/nstextinputtraits | https://developer.apple.com/documentation/appkit/nstextinputtraits | 2026-08-28T16:54:01Z | `1570c62967cbd535f42e0394f3db2ce6b9f1b985c86c1bfae0d3e27034721047` |
| S20 | https://developer.apple.com/documentation/appkit/nsworkspace/didwakenotification | https://developer.apple.com/documentation/appkit/nsworkspace/didwakenotification | 2026-08-28T16:54:02Z | `c7c8eddebe3b321971dad57174670ec8f2f625c798348a16cca3c26089eb274f` |
| S21 | https://developer.apple.com/documentation/quartzcore/cametallayer/nextdrawable() | https://developer.apple.com/documentation/quartzcore/cametallayer/nextdrawable() | 2026-08-28T16:54:04Z | `d116ba9effebca3d8cf64af5ea318f824c0c68230fb570040eddee77b3441483` |
| S23 | https://developer.apple.com/documentation/appkit/nstextinputcontext/keyboardinputsources | https://developer.apple.com/documentation/appkit/nstextinputcontext/keyboardinputsources | 2026-08-28T16:56:09Z | `e930d3f13ca1ce33e969a83a0906b7ef460ef66ed55c8e7217d7b66be6a704ba` |
| S26 | https://developer.apple.com/documentation/appkit/nswindow/didendliveresizenotification | https://developer.apple.com/documentation/appkit/nswindow/didendliveresizenotification | 2026-08-28T16:55:36Z | `cfb2de38a36ab5d2e5104a92ff7e4654fa30fd422ab62ad50cf08c2ab79718eb` |
| S27 | https://developer.apple.com/documentation/appkit/nsaccessibility-swift.struct/notification | https://developer.apple.com/documentation/appkit/nsaccessibility-swift.struct/notification | 2026-08-28T17:42:43Z | `ef763874545361c221e62b536d6c36c6f67b0c721fbb678144f5f6698222f611` |
| S28 | https://developer.apple.com/documentation/appkit/nsaccessibility-swift.struct/role | https://developer.apple.com/documentation/appkit/nsaccessibility-swift.struct/role | 2026-08-28T17:42:44Z | `c81ed7421727c218938cf5f53cd3a49bd16bcd75d402115c2892ff7d0f6fb9b5` |
| S29 | https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-controltypesoverview | https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-controltypesoverview | 2026-08-28T17:42:45Z | `acc76e7490add3c968e212242e95d54a51d7498eef0ce6e884ace5ee3b684aec` |
| S30 | https://gnome.pages.gitlab.gnome.org/at-spi2-core/devel-docs/doc-org.a11y.atspi.Accessible.html | https://gnome.pages.gitlab.gnome.org/at-spi2-core/devel-docs/doc-org.a11y.atspi.Accessible.html | 2026-08-28T17:42:46Z | `235e0fab627d0238c68b2987b283e0aab5b5cc7f1cf023b3c85f892a4d33a2d4` |
| S31 | https://git-scm.com/docs/git-archive | https://git-scm.com/docs/git-archive | 2026-08-28T18:20:02Z | `21b89779c56ae813b5a7383ab1dd942a3773d01e1dc732aac7e7f1bc828ba023` |
| S32 | https://git-scm.com/docs/git-archive | https://git-scm.com/docs/git-archive | 2026-08-28T18:36:43Z | `eeb017eadfe17d6e3a7587fbf07e966ae276674cc6f519b335318b77343dce4c` |

#### Preserved verbatim excerpts

Each block retains only the source text used by this report. The excerpt hash normalizes that block as UTF-8 with LF line endings and one trailing LF.

`S1`

```text
Xcode 26.6 includes Swift 6.3 and SDKs for iOS 26.5, iPadOS 26.5, tvOS 26.5, watchOS 26.5, macOS 26.5, and visionOS 26.5.
```

`S2`

```text
Xcode 26.6 (17F113)
```

`S3`

```text
Returns a new display link whose callback will be invoked in-sync with the display the view is on.
macOS 14.0+
```

`S4`

```text
A set of methods that text views need to implement to interact properly with the text input management system.
macOS
```

`S5`

```text
The text input system communicates primarily with the client of the activated input context via the `NSTextInputClient` protocol.
macOS 10.6+
```

`S6`

```text
A string object presents itself as a sequence of UTF-16 code units.
```

`S7`

```text
The complete list of properties and methods for accessible elements.
macOS
```

`S8`

```text
If your app contains custom user interface elements that subclass `NSView`, enhance the accessibility of those elements using the role-based protocols.
```

`S9`

```text
Registers a block of code to be called immediately after the drawable is presented.
macOS 10.15.4+
```

`S10`

```text
The host time, in seconds, when the drawable was displayed onscreen.
The property value is `0.0` if the drawable hasn't been presented or if its associated frame was dropped.
```

`S11`

```text
A notification that a portion of the window object's frame moved onto or off of a screen.
macOS
```

`S12`

```text
Posted immediately after the app becomes active.
macOS
```

`S13`

```text
A notification that the user is about to resize the window.
macOS 10.6+
```

`S14`

```text
A description of an error when the GPU encounters an issue as it runs the command buffer.
macOS 10.11+
```

`S15`

```text
A command buffer's unsuccessful, final state, which indicates the GPU stopped running the buffer's commands because of a runtime issue.
macOS 10.11+
```

`S16`

```text
Registers a completion handler the GPU device calls immediately after the GPU finishes running the commands in the command buffer.
macOS 10.11+
```

`S17`

```text
func CVDisplayLinkStart(CVDisplayLink) -> CVReturn Deprecated
```

`S18`

```text
Deprecated
Device notifications are not applicable on Apple Silicon
```

`S19`

```text
protocol NSTextInputTraits
var smartInsertDeleteType: NSTextInputTraitType
```

`S20`

```text
A notification that the workspace posts when the device wakes from sleep.
macOS
```

`S21`

```text
If all drawables are in use, the layer waits up to one second for one to become available, after which it returns `nil`.
This method returns `nil` if the layer's `pixelFormat` or other properties are invalid.
```

`S23`

```text
The Text Input Source Services API identifies text input sources with text input source identifier strings (for example, `com.apple.inputmethod.Kotoeri.Japanese`) supplied by the underlying text input sources framework. The ID corresponds to the `kTISPropertyInputSourceID` attribute.
macOS 10.6+
```

`S26`

```text
A notification that the user resized the window object.
macOS 10.6+
```

`S27`

```text
# NSAccessibility.Notification
The name of the notification.
```

`S28`

```text
static let popUpButton: NSAccessibility.Role
```

`S29`

```text
Microsoft UI Automation control types are properties that serve as well-known identifiers that indicate the kind of control that a particular UI element represents, such as a combo box or a button.
```

`S30`

```text
Role values - these are the enum values from AtspiRole in atspi-constants.h:
```

`S31`

```text
Creates an archive of the specified format containing the tree structure for the named tree, and writes it out to the standard output. If <prefix> is specified it is prepended to the filenames in the archive.
Set modification time of archive entries. Without this option the committer time is used if <tree-ish> is a commit or tag, and the current time if it is a tree.
```

`S32`

```text
Creates an archive of the specified format containing the tree structure for the named tree, and writes it out to the standard output. If <prefix> is specified it is prepended to the filenames in the archive.
On the other hand, when a commit ID or tag ID is provided, the commit time as recorded in the referenced commit object is used instead.
Files and directories with the attribute export-ignore won’t be added to archive files.
If the attribute export-subst is set for a file then Git will expand several placeholders when adding this file to an archive.
Note that attributes are by default taken from the `.gitattributes` files in the tree that is being archived.
```

The following command produced the table digests from these normalized excerpts.

```text
$ cd /tmp/wf-epic-b/OXY-B001/round2-excerpts && perl hash-excerpts.pl
eb2aa023070dbc6a88c1a796de5bad1788f628759542ef775a1551e9f2718a6e  S1.txt
b9f8ab4cc2e388065638b1fa1626b5cda18880dc976dac8969f2f186d66d52f0  S2.txt
b15a8f07d3136ffeb4b1320882bfaf5af02679b02c34936ec0c30c7bd3c06dcb  S3.txt
2de7feb44005e461202f9513177d8b107aeca811cb1779661a312f89bdcdea95  S4.txt
5bddd714383912afd6304b02b35d9dcb9a1aa8697d02b6282ba0045ecb7e50fe  S5.txt
63672a0b41b9624793644599fa49295bca91e787f317be56e258df5dae2a31bf  S6.txt
a679020d542e4317acf48f2d123030e9b416153085a88caaf9025c9d1a299ae8  S7.txt
b3ef2c0a7af6174961ef249063c5c54280bfa95e9eb35cb77a13b0e8918bcfe6  S8.txt
2b378706dbb4d4f4a7dd10058b38b9643c258c475f53b764d893e9a685e1d6b2  S9.txt
c00c93e2f2cdcc307d3bae3bbedd4fcc5ca6e65ad61b3b0d0be03fa463645a15  S10.txt
273db18cc424b8e8bb8e4148b4c5881a42ba38d9bed05c6b675819fc9fc1b32b  S11.txt
3e5e987460dedce7f9f7d7d8caeb3ba1b69b9e50aaad31626d9a40bc339eed79  S12.txt
1183e427c94e217a5d126677835687cd1a7ec7d8b8285f9cfc05ca0bd36f530c  S13.txt
bfb9065f88792bbdf37be1a81bebe2f7161e3aef938ab9bc300aa3fd30a52dd1  S14.txt
22f2a1fba9ac9c4ddd2ffeb683685b2e371801a208624d6ba59392ef1b479d8a  S15.txt
329501029ae1be4316d9068045342bc7a0666ccc812f730ab321a2a7cdd13241  S16.txt
0667fda62ceccd40b5beb28db58ce1e82315263b72fce0527e3277ba52142847  S17.txt
a42f1c7accf2a09bd232bfa80df64f90b38386b01662216d8e517c2a353c6628  S18.txt
1570c62967cbd535f42e0394f3db2ce6b9f1b985c86c1bfae0d3e27034721047  S19.txt
c7c8eddebe3b321971dad57174670ec8f2f625c798348a16cca3c26089eb274f  S20.txt
d116ba9effebca3d8cf64af5ea318f824c0c68230fb570040eddee77b3441483  S21.txt
e930d3f13ca1ce33e969a83a0906b7ef460ef66ed55c8e7217d7b66be6a704ba  S23.txt
cfb2de38a36ab5d2e5104a92ff7e4654fa30fd422ab62ad50cf08c2ab79718eb  S26.txt
ef763874545361c221e62b536d6c36c6f67b0c721fbb678144f5f6698222f611  S27.txt
c81ed7421727c218938cf5f53cd3a49bd16bcd75d402115c2892ff7d0f6fb9b5  S28.txt
acc76e7490add3c968e212242e95d54a51d7498eef0ce6e884ace5ee3b684aec  S29.txt
235e0fab627d0238c68b2987b283e0aab5b5cc7f1cf023b3c85f892a4d33a2d4  S30.txt
```

The following Round-5 command produced the S31 digest from the normalized UTF-8 excerpt with LF line endings and one trailing LF.

```text
$ sha256sum /tmp/wf-epic-b/OXY-B001/round5-excerpts/S31.txt
21b89779c56ae813b5a7383ab1dd942a3773d01e1dc732aac7e7f1bc828ba023  /tmp/wf-epic-b/OXY-B001/round5-excerpts/S31.txt
```

The following Round-6 command produced the S32 digest from the normalized UTF-8 excerpt with LF line endings and one trailing LF.

```text
$ sha256sum /tmp/wf-epic-b/OXY-B001/round6-git-archive/S32.txt
eeb017eadfe17d6e3a7587fbf07e966ae276674cc6f519b335318b77343dce4c  /tmp/wf-epic-b/OXY-B001/round6-git-archive/S32.txt
```

### Cited official source URLs

- S1: https://developer.apple.com/documentation/xcode-release-notes/xcode-26_6-release-notes
- S2: https://developer.apple.com/news/releases/?id=06252026a
- S3: https://developer.apple.com/documentation/appkit/nsview/displaylink(target:selector:)
- S4: https://developer.apple.com/documentation/appkit/nstextinputclient
- S5: https://developer.apple.com/documentation/appkit/nstextinputcontext
- S6: https://developer.apple.com/documentation/foundation/nsstring
- S7: https://developer.apple.com/documentation/appkit/nsaccessibilityprotocol
- S8: https://developer.apple.com/documentation/appkit/accessibility-for-appkit
- S9: https://developer.apple.com/documentation/metal/mtldrawable/addpresentedhandler(_:)
- S10: https://developer.apple.com/documentation/metal/mtldrawable/presentedtime
- S11: https://developer.apple.com/documentation/appkit/nswindow/didchangescreennotification
- S12: https://developer.apple.com/documentation/appkit/nsapplication/didbecomeactivenotification
- S13: https://developer.apple.com/documentation/appkit/nswindow/willstartliveresizenotification
- S14: https://developer.apple.com/documentation/metal/mtlcommandbuffer/error
- S15: https://developer.apple.com/documentation/metal/mtlcommandbuffer/status
- S16: https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:)
- S17: https://developer.apple.com/documentation/corevideo/cvdisplaylink
- S18: https://developer.apple.com/documentation/metal/mtldevicenotificationname/wasremoved
- S19: https://developer.apple.com/documentation/appkit/nstextinputtraits
- S20: https://developer.apple.com/documentation/appkit/nsworkspace/didwakenotification
- S21: https://developer.apple.com/documentation/quartzcore/cametallayer/nextdrawable()
- S23: https://developer.apple.com/documentation/appkit/nstextinputcontext/keyboardinputsources
- S26: https://developer.apple.com/documentation/appkit/nswindow/didendliveresizenotification
- S27: https://developer.apple.com/documentation/appkit/nsaccessibility-swift.struct/notification
- S28: https://developer.apple.com/documentation/appkit/nsaccessibility-swift.struct/role
- S29: https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-controltypesoverview
- S30: https://gnome.pages.gitlab.gnome.org/at-spi2-core/devel-docs/doc-org.a11y.atspi.Accessible.html
- S31: https://git-scm.com/docs/git-archive
- S32: https://git-scm.com/docs/git-archive
