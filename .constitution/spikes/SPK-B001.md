# Spike report: OXY-B001 macOS qualification baseline

## Time box

- **Budget:** 1 focused day.
- **Clock start / stop:** 2026-08-28T16:52:30Z / 2026-08-28T17:06:38Z.
- **Scope result:** This report changes no product capability, architecture boundary, source tree, or specification. The only repository file changed is this report.

## Question

- **Decision this spike must produce:** Which exact supported macOS versions and interfaces provide the platform-independent input method editor, accessibility, per-view timing, independent timing observation, service-routing, and recovery baseline for both allocations?

### Decision register

| ID | Baseline question | Allocation | Status | Answer and cited evidence | Next bounded probe for a KU |
| :-- | :-- | :-- | :-- | :-- | :-- |
| B001-01 | Which SDK supplies the baseline? | Both | KK | Xcode 26.6 includes the macOS 26.5 SDK, and Apple's release page identifies the build as `17F113`. See S1 and S2, whose preserved excerpts and digests appear in [Authoritative source records](#authoritative-source-records). | Not applicable. |
| B001-02 | What is the minimum deployment target? | Both | KU (gating) | `NSView.displayLink(target:selector:)` is macOS 14.0+, but the fetched pages for `NSTextInputClient`, `NSAccessibilityProtocol`, `NSWindow.didChangeScreenNotification`, `NSApplication.didBecomeActiveNotification`, and `NSWorkspace.didWakeNotification` identify only `macOS`, not a minimum version. The availability matrix therefore cannot derive a verified maximum. STOP: an official availability value was not fetched for every baseline interface. See S3-S7 and S11-S12, S20. | P8: on a macOS host with Xcode 26.6, fetch and preserve Apple's DocC availability metadata or the corresponding Apple SDK declaration for each `unavailable` matrix cell, then have a second command verify every declared minimum is at most the proposed target. Expected output: a source manifest with each API, Apple URL, stated minimum, source digest, and `maximumMinimum`; otherwise retain this KU. |
| B001-03 | Does AppKit provide the native input method editor transport and UTF-16 index unit? | Both | KK | `NSTextInputClient` lists marked and selected ranges, marked-text replacement, unmarking, insertion, character-index lookup, and first-rectangle operations. `NSTextInputContext` owns a client, activates and deactivates, discards a conversion session, and invalidates character coordinates. Apple states that an `NSString` presents itself as UTF-16 code units. These are interface-availability facts, not evidence that either allocation implements the contract. See S4-S6. | Not applicable. |
| B001-04 | Does the proposed input method editor map preserve composition, replacement, cancellation, deletion, focus transfer, candidate geometry, and checked UTF-16 conversion? | Focused and integrated | KU (gating) | Apple's interface documentation establishes operations but not either allocation's callback transcript, conversion behavior, secure-field handling, or two-view routing. The host preflight could not run AppKit. STOP: no controlled AppKit probe ran. See S4-S6 and the [Controlled probe record](#controlled-probe-record). | P1: run the action-by-vector matrix in [Input method editor qualification corpus](#input-method-editor-qualification-corpus) on a pinned arm64 macOS host. Expected output: one redacted JSONL transcript per allocation, vector, and action with client identity, view generation, UTF-16 ranges, conversion result, and pass or fail. |
| B001-05 | Can a numeric-input and sensitive-field policy be frozen? | Both | KU (gating) | `NSTextInputTraits` exposes text-input traits, but the fetched page does not establish a numeric negotiation contract or prove that a secure field returns only redacted surrounding context. STOP: documentation does not establish either behavior. See S19. | P1: log only trait names, classification, range lengths, and redaction checks for numeric and secure fixtures. Expected output: a supported setting and no raw secure text, or a cited unsupported result. |
| B001-06 | Which accessibility interface exposes semantics and reverse actions to assistive software? | Both | KK | `NSAccessibilityProtocol` defines informational properties, action methods, and notifications. Apple requires role-specific protocols for custom `NSView` subclasses and `NSAccessibilityElement` for custom non-view elements. See S7 and S8. This proves the destination interface only. | Not applicable. |
| B001-07 | Is there a complete focused allocation forward and reverse VoiceOver map? | Focused | KU (gating) | No preserved map binds roles, states, values, relations, traversal, text range, selection, geometry, view identity, stale generation, reverse payloads, and acknowledgements to the focused allocation. STOP: the required map artifact does not exist. | P2: run the corpus in [Accessibility qualification corpus](#accessibility-qualification-corpus) through the focused allocation and validate its output against `accessibility-map.schema.json`. Expected output: one complete map and reverse-action log with immutable evidence references, or a schema or behavioral failure. |
| B001-08 | Is there a complete integrated allocation forward and reverse VoiceOver map? | Integrated | KU (gating) | No pinned integrated fork inventory or preserved map establishes its macOS accessibility path. STOP: the integrated input is not frozen. | P6, then P2: freeze the integrated fork revision and inventory its macOS accessibility crossings, then run the same P2 corpus through the C ABI. Expected output: a commit-bound inventory, complete map, and reverse-action log, or a named missing crossing. |
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

P1 has one pinned CJK input-source identity: `com.apple.inputmethod.SCIM.ITABC`. Before the CJK vector runs, the probe must enumerate `NSTextInputContext.keyboardInputSources` and require that exact ID. Apple describes those values as input-source identifier strings corresponding to `kTISPropertyInputSourceID`. If the ID is absent, P1 emits `input-source-missing` and fails the CJK vector without selecting a substitute. See S23.

P1 creates two `NSTextInputContext(client:)` instances, one per view. Each transcript record contains monotonic timestamp, allocation, vector, action, native client identity, view ID, view generation, input-source ID, UTF-16 input and output ranges, callback or command name, geometry, redaction flag, expected assertion ID, result, and no raw secure-field text.

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

The P1 command is `cd /tmp/wf-epic-b/OXY-B001/mac-ime-probe && xcrun --sdk macosx clang -fobjc-arc -framework AppKit ime_probe.m -o ime_probe && ./ime_probe --two-views --input-source com.apple.inputmethod.SCIM.ITABC --matrix ime-matrix.json --jsonl transcript.jsonl`.

### Accessibility qualification corpus

CAP-SEM-001 requires every applicable role-specific property, relation, state, value, geometry, text range, traversal rule, and view identity. CAP-SEM-002 requires reverse routing to a live view and node with an acknowledgement or stale-target error. The two corresponding flows also require incremental insert, update, and delete behavior and prohibit retargeting stale input.

The PRD and architecture flows do not enumerate role-value names. `accessibility-map.schema.json` requires the `forward.roles` mapping but also does not enumerate role values. Therefore P2 cannot claim a complete supported-role corpus before a candidate-neutral role registry is frozen. This is a gating KU, not an assumed empty role set. P2 must reject an output that omits any role from that frozen registry.

The following corpus is frozen for P2's structure and coverage. It applies unchanged to focused and integrated output.

| Corpus part | Required fixture and assertion |
| :-- | :-- |
| Role coverage | Generate one live node for every role in the frozen candidate-neutral role registry, plus one root and one container. Reject an output whose `forward.roles` lacks a registry role or maps a registry role to an empty native identifier. |
| Forward schema coverage | Populate and validate every required `forward` key: `roles`, `states`, `actions`, `values`, `labels`, `accessibleNames`, `descriptions`, `hints`, `helpOrFullDescriptions`, `tooltips`, `attributedText`, `identifiers`, `bounds`, `transforms`, `traversal`, `labelledByRelations`, `describedByRelations`, `roleApplicableRelations`, `accessibilityFocus`, `inputFocus`, `hitTesting`, `textRanges`, `selection`, `scrollExtents`, `language`, `direction`, `headingLevels`, `liveRegions`, `hidden`, `disabled`, `secureFieldRedaction`, and `multiViewIsolation`. Reject any missing key, empty native mapping, or non-`kk` field in a claimed complete map. |
| Values and applicable states | Give each applicable role a false and true state, a changed value, disabled and hidden cases, and protected-content state. Reject a missing role-applicable value or state, an inapplicable state reported as applicable, or raw secure content. |
| Relations and traversal | Use labelled-by, described-by, parent-child, visible-child, and role-applicable relations. Give children a non-document order and a declared navigation order. Reject missing relation targets, wrong target view, duplicate identity, or navigation order that differs from the declared traversal. |
| Selection, caret, and text | Include text range, selected range, multiple selection if the native mapping declares it, caret position, visible range, and range geometry on ASCII, emoji, combining, and bidirectional text. Reject an index without its text-layout generation, a non-UTF-16 native range, or a range whose geometry belongs to another generation. |
| Geometry and transforms | Place equivalent nodes at nonzero transformed coordinates in two windows. Reject a frame that omits the transform, is not in screen coordinates when the mapping promises screen coordinates, or names the other view. |
| View identity and stale generation | Export two live views with distinct IDs and semantics generations, then delete one node, replace its generation, and invoke an action at the former target. Reject cross-view delivery; require the defined stale-target error. |
| Reverse actions | For every action reported by `forward.actions`, send accepted, rejected, invalid-payload, and stale-target invocations. Each record must include native identifier, complete payload encoding, text index unit, text-layout generation binding, view and node routing, acknowledgement, error result, and stale-target result as required by the schema. Reject an action lacking any field or acknowledgement. |

P2 writes one map per allocation with `schemaVersion: "5.0.0"`, `environment: "macos"`, its candidate name, `epistemicStatus: "kk-complete"`, the entire forward map, every reverse action, and evidence objects. Both output files must validate against the same complete `accessibility-map.schema.json`; P2 must not use a reduced focused schema or a reduced integrated schema. A VoiceOver traversal and action log is an additional behavioral check, not a substitute for schema validation.

The P2 command is `cd /tmp/wf-epic-b/OXY-B001/mac-accessibility-probe && xcrun --sdk macosx clang -fobjc-arc -framework AppKit accessibility_probe.m -o accessibility_probe && ./accessibility_probe --two-windows --schema .constitution/tech-spec/data-models/accessibility-map.schema.json --candidate focused --output focused-map.json && ./accessibility_probe --two-windows --schema .constitution/tech-spec/data-models/accessibility-map.schema.json --candidate integrated --through-c-abi --output integrated-map.json`.

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
| P1 | `mac-ime-probe` command in [Input method editor qualification corpus](#input-method-editor-qualification-corpus). | Runs all 80 action-vector cells, requires `com.apple.inputmethod.SCIM.ITABC`, and writes redacted JSONL with conversion and routing results. |
| P2 | `mac-accessibility-probe` command in [Accessibility qualification corpus](#accessibility-qualification-corpus). | Validates focused and integrated outputs against the same complete schema, then records VoiceOver traversal and every declared reverse action. |
| P3 | `cd /tmp/wf-epic-b/OXY-B001/mac-timing-probe && xcrun --sdk macosx clang -fobjc-arc -framework AppKit -framework Metal timing_probe.m -o timing_probe && ./timing_probe --two-views --observer-process --move-displays --seconds 10 --jsonl timing.jsonl`. | Records observer and candidate process IDs, display identities, link timestamps, presentation times, and display epochs while each candidate stream is blocked. |
| P4 | `cd /tmp/wf-epic-b/OXY-B001/mac-routing-probe && ./routing_probe --allocation focused --two-windows --interleave --teardown && ./routing_probe --allocation integrated --through-c-abi --two-windows --interleave --teardown`. | Proves each request carries an owning view generation and stale target behavior. |
| P5 | `mac-recovery-probe` command in [Recovery qualification corpus](#recovery-qualification-corpus). | Runs drawable loss, real sleep and wake, display topology, resize, and available graphics-error cases through both allocations. |
| P6 | `cd /tmp/wf-epic-b/OXY-B001/integrated-inventory && ./inventory.sh "$INTEGRATED_FORK_COMMIT" > inventory.json`, where `INTEGRATED_FORK_COMMIT` is the frozen integrated-fork commit. | Emits a commit-bound macOS path and symbol inventory for input method editor, accessibility, clipboard, timing, lifecycle, Metal error, and C-ABI crossings. |
| P7 | `cd /tmp/wf-epic-b/OXY-B001/evidence-lock && ./lock.sh ../sources ../mac-* > manifest.json`. | Copies source and probe bytes to the evidence root and writes a manifest that verifies every path and SHA-256. |
| P8 | `cd /tmp/wf-epic-b/OXY-B001/macos-availability && ./collect-apple-availability.sh > availability.json && ./verify-maximum-minimum.sh availability.json`. | Preserves authoritative availability for every `not stated` interface and either derives one verified maximum or retains B001-02 as a KU. |

## Options and trade-offs

- **Option A:** Freeze only the documented interface availability: Xcode 26.6 build `17F113`, macOS 26.5 SDK, AppKit text and accessibility interfaces, view-linked display timing, Metal presentation feedback, and recovery observations.
- **Option B:** Set macOS 14.0 as the deployment target before P8 completes.
- **Option C:** Retain each behavior, source-availability, mapping, independence, routing, recovery, and evidence-publication question as a gating KU with one bounded probe.

## Recommendation

- **Chosen option:** A/C mix. Choose A for B001-01, B001-03, B001-06, B001-09, B001-12, and B001-15. Choose C for B001-02, B001-04, B001-05, B001-07, B001-08, B001-10, B001-11, B001-13, B001-14, and B001-16 through B001-18.
- **Why it fits:** The recommendation treats only preserved, authoritative interface evidence as KK. It does not turn a platform API, a source listing, or a plausible deployment target into candidate behavior evidence. P1-P8 each specify a host, input, command, and expected output.
- **Rejected option:** Reject B because the documented 14.0 display-link introduction does not prove the historical availability of every other required interface. Reject deprecated `CVDisplayLink`, candidate-internal clocks as independent meters, `MTLDeviceNotificationName.wasRemoved` on Apple Silicon, default-window routing, and a map or recovery claim without preserved traces.
- **Capability and architecture guard:** This recommendation preserves the P0 capabilities and the accepted Platform integration and reentrancy boundaries. It chooses no substrate and introduces no product capability or architecture boundary.

## Downstream impact

- **ADRs to write or update:** No architecture decision record change is required. Do not alter the accepted direct-AppKit or normalized-callback boundary in `ADR-0005-platform-hosts.md` or the execution boundary in `ADR-0006-execution-domains.md`.
- **Tickets unblocked in `tasks/active/`:** The documented source-interface portion of the macOS investigation is complete. Candidate implementation and measurement remain blocked by P1-P8.
- **Tickets to add or split:** Add bounded work only for P1 input method editor behavior, P2 accessibility maps and role registry, P3 timing and migration, P4 service routing, P5 recovery injection, P6 integrated inventory, P7 evidence locking, and P8 historical availability.
- **Spec edits required:** No current specification value changes. B001-02 is a gating KU, so Stage 3 must retain the current deployment-target state. After P8 verifies a single target, Stage 3 must apply the following four edits in one atomic specification transaction; Stage 3 must not apply the `stack.md` edit ahead of the other three edits.

| File and field or section | Exact instruction |
| :-- | :-- |
| `.constitution/tech-spec/stack.md` -> `Platform qualification pins` -> macOS row | Retain the exact phrase `minimum deployment target is a gating KU`. Do not replace it before P8 produces the verified maximum-minimum record. |
| `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.macos.minimumVersion` | Retain the exact current object `{"status":"ku-gating","value":null,"evidence":[]}`. Do not add an evidence URL string. |
| `.constitution/tech-spec/contracts/qualification-lock.json` -> `referenceEnvironments.macos-arm64.minimumVersion` | Retain `null` and retain `minimum-platform-and-protocol-versions` in both known-unknown arrays. |
| `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.macos.openQuestions` | Retain the exact entry `minimum deployment target`. Do not remove it before the first three changes are made in the same transaction after P8. |

## Evidence preservation convention

The current `platform-contracts` schema accepts evidence objects with exactly `path` and a 64-hex `sha256`. Existing qualification fixtures use repository-relative paths with the same pair. P7 must preserve fetched source bytes without rewriting them under `qualification/fixtures/external-contracts/macos/official-sources/`, write the origin URL and fetch timestamp in a manifest, and reference that manifest or a source body through these shapes. This report proposes the convention but creates no fixture files.

```json
{
  "path": "qualification/fixtures/external-contracts/macos/official-sources/s3-nsview-display-link.reader.md",
  "sha256": "fec80148e0975ee4f0231922381f56e10ef6b9713d8d819a2c42b438fec4dd50"
}
```

```json
{
  "path": "qualification/fixtures/external-contracts/macos/official-sources/s20-nsworkspace-did-wake.reader.md",
  "sha256": "9e8a391d7508241438cebbfb0d6e6d4df8c3e772256def39c28c362c7e6d1875"
}
```

After P7 creates those exact files from the fetched bytes, Stage 3 can use the objects in evidence arrays. A URL alone is not a schema-valid evidence item. P8 must produce its own exact source-body and manifest digest before any deployment-target object can change from `ku-gating` to `kk`.

## Sources

### Authoritative source records

Table 3 preserves every authoritative source body used by a claim in this report. `Bytes` and `SHA-256` apply to the body fetched through the Jina reader URL `https://r.jina.ai/` plus the listed Apple URL. The excerpt is verbatim from that fetched body and supports the stated interface claim.

| ID | Official Apple URL | UTC fetch | Bytes | SHA-256 | Verbatim authoritative excerpt |
| :-- | :-- | :-- | --: | :-- | :-- |
| S1 | https://developer.apple.com/documentation/xcode-release-notes/xcode-26_6-release-notes | 2026-08-28T16:53:48Z | 24843 | `3e7df02bd2134f9c6ab6b582feeac749580c390cb0395d4831093651f2ac8614` | "Xcode 26.6 includes Swift 6.3 and SDKs for iOS 26.5, iPadOS 26.5, tvOS 26.5, watchOS 26.5, macOS 26.5, and visionOS 26.5." |
| S2 | https://developer.apple.com/news/releases/?id=06252026a | 2026-08-28T16:53:48Z | 13729 | `2b0d1d9f9b0051dccfa561e87cf79934e33e67c6f31ade24a99d2266a480225e` | "Xcode 26.6 (17F113)" |
| S3 | https://developer.apple.com/documentation/appkit/nsview/displaylink(target:selector:) | 2026-08-28T16:53:49Z | 10592 | `fec80148e0975ee4f0231922381f56e10ef6b9713d8d819a2c42b438fec4dd50` | "Returns a new display link whose callback will be invoked in-sync with the display the view is on." "macOS 14.0+" |
| S4 | https://developer.apple.com/documentation/appkit/nstextinputclient | 2026-08-28T16:53:50Z | 32369 | `00f4d6c499eded2cbb021fdd9930f05ea4ac901394df07e53ce3124df8736a36` | "A set of methods that text views need to implement to interact properly with the text input management system." "macOS" |
| S5 | https://developer.apple.com/documentation/appkit/nstextinputcontext | 2026-08-28T16:53:50Z | 30565 | `440249e61571602dc293ccc0937c8211d7fc70fe84cbd9b0c29dde0d445667ee` | "The text input system communicates primarily with the client of the activated input context via the `NSTextInputClient` protocol." "macOS 10.6+" |
| S6 | https://developer.apple.com/documentation/foundation/nsstring | 2026-08-28T16:53:51Z | 87792 | `5598d22e7a51a2821703b6f1cbfbc54622bf7f3674b679847904217b472c045c` | "A string object presents itself as a sequence of UTF-16 code units." |
| S7 | https://developer.apple.com/documentation/appkit/nsaccessibilityprotocol | 2026-08-28T16:53:52Z | 110429 | `daf40da28078ff2da6a8d73bc0be09107a73059cdf36b20b0377116501a5c2c6` | "The complete list of properties and methods for accessible elements." "macOS" |
| S8 | https://developer.apple.com/documentation/appkit/accessibility-for-appkit | 2026-08-28T16:53:53Z | 29097 | `0fdec6204a888d1bc512b4d40bb2274e494d88ea95183b983dd26e2358ac96cb` | "If your app contains custom user interface elements that subclass `NSView`, enhance the accessibility of those elements using the role-based protocols." |
| S9 | https://developer.apple.com/documentation/metal/mtldrawable/addpresentedhandler(_:) | 2026-08-28T16:53:54Z | 33599 | `bff44b058a2c5289fd5e2cbb90b9ce464c79d2aede7d1eea7d5be32c1b661c50` | "Registers a block of code to be called immediately after the drawable is presented." "macOS 10.15.4+" |
| S10 | https://developer.apple.com/documentation/metal/mtldrawable/presentedtime | 2026-08-28T16:53:54Z | 32134 | `0f1a384f05937646a603ad9ab63352db87797ddce99f195eae2d52edbc721072` | "The host time, in seconds, when the drawable was displayed onscreen." "The property value is `0.0` if the drawable hasn't been presented or if its associated frame was dropped." |
| S11 | https://developer.apple.com/documentation/appkit/nswindow/didchangescreennotification | 2026-08-28T16:53:55Z | 36872 | `02b77526748837d3b515afc8eb8cbe6fcdb92e3bcbf43ea712c0ca2fca827234` | "A notification that a portion of the window object's frame moved onto or off of a screen." "macOS" |
| S12 | https://developer.apple.com/documentation/appkit/nsapplication/didbecomeactivenotification | 2026-08-28T16:53:56Z | 38052 | `de013ee86e96082f7ba9395599bc2125665a8c012d877b956c37e205269e3be2` | "Posted immediately after the app becomes active." "macOS" |
| S13 | https://developer.apple.com/documentation/appkit/nswindow/willstartliveresizenotification | 2026-08-28T16:53:56Z | 36967 | `27c604d28d642b8cfe683fdca24d3f36af46941221d11c32ea733108eba80922` | "A notification that the user is about to resize the window." "macOS 10.6+" |
| S14 | https://developer.apple.com/documentation/metal/mtlcommandbuffer/error | 2026-08-28T16:53:57Z | 25032 | `7d42cd77570104c1f8bce2ee8ab966abe213691a35fe78a22d8393729964bd05` | "A description of an error when the GPU encounters an issue as it runs the command buffer." "macOS 10.11+" |
| S15 | https://developer.apple.com/documentation/metal/mtlcommandbuffer/status | 2026-08-28T16:53:58Z | 35506 | `c6329bd97cf28267f135ce2186ff622d0d15ab2a47fb91de7bb943fc296c1bd1` | "A command buffer's unsuccessful, final state, which indicates the GPU stopped running the buffer's commands because of a runtime issue." "macOS 10.11+" |
| S16 | https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:) | 2026-08-28T16:53:58Z | 35587 | `524b3f06ca091f41188a61287646f3bf6e2446d2599b7a6959bec870a07a8563` | "Registers a completion handler the GPU device calls immediately after the GPU finishes running the commands in the command buffer." "macOS 10.11+" |
| S17 | https://developer.apple.com/documentation/corevideo/cvdisplaylink | 2026-08-28T16:53:59Z | 21768 | `dcc516eaff45a18ad3e88ce601742991c8814ce79fd659aef5f32c26f63eb86c` | "func CVDisplayLinkStart(CVDisplayLink) -> CVReturn Deprecated" |
| S18 | https://developer.apple.com/documentation/metal/mtldevicenotificationname/wasremoved | 2026-08-28T16:54:00Z | 23670 | `1cc3e87fc858eb89408ddc63b0081ee0724cfbb4d458a3542963bb63ed735a22` | "Deprecated" "Device notifications are not applicable on Apple Silicon" |
| S19 | https://developer.apple.com/documentation/appkit/nstextinputtraits | 2026-08-28T16:54:01Z | 32200 | `a114062d5ed23eda0fe307922f70e997464a38a12b56292a1e08e8e1e60abf74` | "protocol NSTextInputTraits" "var smartInsertDeleteType: NSTextInputTraitType" |
| S20 | https://developer.apple.com/documentation/appkit/nsworkspace/didwakenotification | 2026-08-28T16:54:02Z | 37921 | `9e8a391d7508241438cebbfb0d6e6d4df8c3e772256def39c28c362c7e6d1875` | "A notification that the workspace posts when the device wakes from sleep." "macOS" |
| S21 | https://developer.apple.com/documentation/quartzcore/cametallayer/nextdrawable() | 2026-08-28T16:54:04Z | 30819 | `c015842fcbec80b8bd49704ed5d1b45cb47a52dae9ddf64354cc5e222fd3f081` | "If all drawables are in use, the layer waits up to one second for one to become available, after which it returns `nil`." "This method returns `nil` if the layer's `pixelFormat` or other properties are invalid." |
| S23 | https://developer.apple.com/documentation/appkit/nstextinputcontext/keyboardinputsources | 2026-08-28T16:56:09Z | 32968 | `85844b79c1a26de17e381e55346f2b43fbdac93da1ef0a0240ded0b43c128f2c` | "The array of keyboard text input source identifier strings available to the receiver." "The ID corresponds to the `kTISPropertyInputSourceID` attribute." |
| S26 | https://developer.apple.com/documentation/appkit/nswindow/didendliveresizenotification | 2026-08-28T16:55:36Z | 36793 | `2b04e36dfc7f1e23305983d1243dd2146dd716bf8290626105bb7655ec73ede1` | "A notification that the user resized the window object." "macOS 10.6+" |

### Additional fetched-body record

Table 4 records all other Jina fetch attempts in this spike. These rows support no claim in this report. Their bytes and digests remain recorded to make the source-research boundary auditable. `404` and `page not found` rows are not citations.

| ID | Official URL | UTC fetch | Bytes | SHA-256 | Result |
| :-- | :-- | :-- | --: | :-- | :-- |
| S22 | https://developer.apple.com/documentation/coregraphics/1455336-cgdisplayregisterreconfiguration | 2026-08-28T16:54:08Z | 11587 | `7e542a5339580cfba35d8343e2dbbb05162de9e18484825391d24efb584b8291` | Fetched page did not expose an API excerpt; not relied on. |
| S23-404 | https://developer.apple.com/documentation/carbon/1455837-tiscopyinputsourceforlanguage | 2026-08-28T16:54:23Z | 11571 | `fb63cda4822fdf960f3aae152c946137758c9b0f5f91add0b10ee5528d510688` | `404 Not Found`; not cited. |
| S23A | https://developer.apple.com/library/archive/documentation/Carbon/Reference/TextInputSourceServicesReference/index.html | 2026-08-28T16:55:29Z | 14021 | `ffeb1e74058a3c11193de248b966315ccd8a1db6f85b6bf73e0ae04fc592a2d2` | Page Not Found; not cited. |
| S23B | https://developer.apple.com/library/archive/documentation/Carbon/Reference/TextInputSourceServicesReference/Reference/reference.html | 2026-08-28T16:55:31Z | 272 | `6140859b6d18d7b8ee9e459e5b9e357b92ea39184ed1388f45f08d68fe05ceb1` | Empty reader content; not cited. |
| S24 | https://developer.apple.com/documentation/carbon/1455997-tisgetinputsourceproperty | 2026-08-28T16:54:39Z | 11563 | `6d292b65bd5d15d710de3c7d1f67d6e53464f2c3a87d35bf0fcb62ac12303f12` | `404 Not Found`; not cited. |
| S25 | https://developer.apple.com/documentation/appkit/nsaccessibilityelement | 2026-08-28T16:55:35Z | 6629 | `1b49ad707413b78de8efe2b0c854446cc69f33509dac3ab85912e8e32f7f40bf` | Page not found; not cited. |
| S27 | https://developer.apple.com/documentation/appkit/nswindow/didresizenotification | 2026-08-28T16:55:41Z | 11577 | `7e380747f328f1908b0b38f691e544f1e3e5675252af778eabc913c25e12c2a3` | `404 Not Found`; not cited. |
| S28 | https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:) | 2026-08-28T16:55:43Z | 35587 | `524b3f06ca091f41188a61287646f3bf6e2446d2599b7a6959bec870a07a8563` | Duplicate of S16; not separately cited. |
| S29 | https://developer.apple.com/documentation/metal/mtlcommandbuffer/status | 2026-08-28T16:55:44Z | 35506 | `c6329bd97cf28267f135ce2186ff622d0d15ab2a47fb91de7bb943fc296c1bd1` | Duplicate of S15; not separately cited. |
| S30 | https://developer.apple.com/documentation/metal/mtldrawable/presentedtime | 2026-08-28T16:55:45Z | 32134 | `0f1a384f05937646a603ad9ab63352db87797ddce99f195eae2d52edbc721072` | Duplicate of S10; not separately cited. |
| S31 | https://developer.apple.com/documentation/appkit/nstextinputcontext | 2026-08-28T16:55:45Z | 30565 | `440249e61571602dc293ccc0937c8211d7fc70fe84cbd9b0c29dde0d445667ee` | Duplicate of S5; not separately cited. |
| J4 | https://developer.apple.com/tutorials/data/documentation/appkit/nstextinputclient.json | 2026-08-28T16:59:12Z | 53797 | `a5861a3966451f98e93314a9f5879dbadeec601a4cfcce7a36f86c3b7008e2c1` | Metadata did not provide a numerical minimum; B001-02 remains a KU. |
| J7 | https://developer.apple.com/tutorials/data/documentation/appkit/nsaccessibilityprotocol.json | 2026-08-28T16:59:13Z | 492656 | `7ecab0d6b395c0a2dafdba9cbfeb70a73c75c083910ad63fbf3d234b2a2007e8` | Metadata did not provide a numerical minimum; B001-02 remains a KU. |
| J11 | https://developer.apple.com/tutorials/data/documentation/appkit/nswindow/didchangescreennotification.json | 2026-08-28T16:59:15Z | 33201 | `6ffdaeca31c44e8fb31fa2daefdd29520984c5905f6f06af7b6d6c808cfd2ea2` | Metadata declares `macOS` without a numerical minimum; B001-02 remains a KU. |
| J12 | https://developer.apple.com/tutorials/data/documentation/appkit/nsapplication/didbecomeactivenotification.json | 2026-08-28T16:59:16Z | 41723 | `cfd0316957731b9237ecd44f9f6d188b421d71c86e68ddecd505f29c7cbab49b` | Metadata declares `macOS` without a numerical minimum; B001-02 remains a KU. |
| J20 | https://developer.apple.com/tutorials/data/documentation/appkit/nsworkspace/didwakenotification.json | 2026-08-28T16:59:16Z | 36480 | `ca2dcf346c38c644d01db73214fec1e3438cc24c70132906a1874a8eb19d91e8` | Metadata declares `macOS` without a numerical minimum; B001-02 remains a KU. |

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
