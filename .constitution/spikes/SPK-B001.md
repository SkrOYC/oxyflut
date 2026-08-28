# Spike report: OXY-B001 macOS qualification baseline

## Time box

- **Budget:** 1 focused day
- **Clock start / stop:** 2026-08-28T16:22:21Z / 2026-08-28T16:31:38Z
- **Scope result:** This report changes no product capability, architecture boundary, source tree, or specification. The only repository file changed is this report.

## Question

- **Decision this spike must produce:** Which exact supported macOS versions and interfaces provide the candidate-neutral input method editor, accessibility, per-view timing, independent timing observation, service-routing, and recovery baseline for both allocations?

### Decision register

| ID | Baseline question | Allocation | Status | Answer and cited evidence | Next bounded probe for a KU |
| :-- | :-- | :-- | :-- | :-- | :-- |
| B001-01 | Which SDK supplies the baseline? | Both | KK | Xcode 26.6 includes the macOS 26.5 SDK, and Apple identifies the Xcode 26.6 build as 17F113. AppKit documentation identifies the required input and accessibility interface families. See S1, S2, S4, S7, and S8. | Not applicable. |
| B001-02 | What is the minimum deployment target? | Both | KK | Set the deployment target to macOS 14.0. Apple marks `NSView.displayLink(target:selector:)` as macOS 14.0+, and this is the newest required candidate-neutral platform API. See S3. | Not applicable. |
| B001-03 | Does AppKit provide the native IME transport and UTF-16 index unit? | Both | KK | `NSTextInputClient` requires marked and selected ranges, marked-text replacement, unmarking, attributed substrings, insertion, candidate geometry, point-to-index conversion, and commands. `NSTextInputContext` owns a client, activates and deactivates, handles events, discards a conversion session, and invalidates character coordinates. These APIs use `NSRange`; Apple documents `NSString` indexes and ranges as UTF-16 code units. See S4, S5, and S6. | Not applicable for interface availability. |
| B001-04 | Does the proposed IME map preserve every required action and vector, including composition, replacement, cancellation, deletion, focus transfer, and candidate geometry? | Focused and integrated | KU (gating) | The documented interfaces establish available operations, not either allocation's complete callback transcript, conversion checks, or behavior with the required multilingual and secure-field vectors. The host preflight below could not run AppKit. STOP applies to this row. See S4 and S5. | P1: run the two-view, noncandidate AppKit IME transcript probe on a pinned arm64 macOS 14.0+ host in `/tmp/wf-epic-b/OXY-B001/mac-ime-probe/`; record every `NSTextInputClient` callback, its UTF-16 ranges, client identity, and view generation while exercising ASCII, emoji, combining, bidirectional, CJK, replacement, candidate-geometry, and secure-field cases. Expected output: one redacted JSONL transcript per vector with every callback attributable to one view, plus conversion pass or a named failed vector. |
| B001-05 | Can a numeric-input and sensitive-field policy be frozen? | Both | KU (gating) | Apple documents published text-input traits and input-context properties, but the fetched material does not establish a numeric negotiation contract or prove that a client returns only required redacted context in a secure field. Absence from documentation is not proof of unsupported behavior. STOP applies to this row. See S5 and S19. | P1: in the same probe, enumerate the input-context and text-trait values selected for numeric and secure fixtures, log only field classifications and byte counts, and verify that returned surrounding text is redacted. Expected output: a frozen supported setting and redaction transcript, or an explicit `unsupported` result with the cited API surface. |
| B001-06 | Which accessibility interface exposes semantics and reverse actions to assistive software? | Both | KK | `NSAccessibilityProtocol` supplies informational properties, action methods, and notifications. Apple states that custom views need role-specific protocol implementations and that custom non-view elements use `NSAccessibilityElement`. The protocol includes protected-content accessors. This establishes the AppKit accessibility interface, not either candidate map or VoiceOver behavior. See S7 and S8. | Not applicable for interface availability. |
| B001-07 | Is there a complete focused allocation forward and reverse VoiceOver map? | Focused | KU (gating) | No preserved map binds framework roles, states, values, text ranges, geometry, traversal, view identity, and reverse actions to AppKit elements. Apple documents the interface but cannot establish this allocation's mapping. STOP applies to this row. See S7 and S8. | P2: build a noncandidate custom `NSView` and `NSAccessibilityElement` fixture in `/tmp/wf-epic-b/OXY-B001/mac-accessibility-probe/`, export its role, children, text range, frame, focus, protected-content state, and actions, then use VoiceOver to invoke each action in two windows. Expected output: an immutable JSON forward map and action-result log keyed by view and semantics generation, or a missing AppKit mapping entry. |
| B001-08 | Is there a complete integrated allocation forward and reverse VoiceOver map? | Integrated | KU (gating) | The pinned integrated fork and its inherited macOS accessibility inventory are not frozen by this report, and no preserved map exists. Apple documents the destination interface only. STOP applies to this row. See S7 and S8. | P6, then P2: first freeze the integrated fork commit and enumerate its macOS accessibility source paths; run the same two-window probe through its C ABI. Expected output: immutable source inventory, a forward map, and reverse-action results keyed by view and semantics generation. |
| B001-09 | What supplies view-associated opportunities and presentation feedback? | Both | KK | `NSView.displayLink(target:selector:)` creates a new callback synchronized with the display containing that view and does not invoke it when the view is hidden or off-display. `MTLDrawable.addPresentedHandler(_:)` runs after presentation, and `presentedTime` reports the on-screen host time or `0.0` for an unpresented or dropped frame. See S3, S9, and S10. | Not applicable for interface availability. |
| B001-10 | Is the external timing observer independent of both candidate callback streams? | Both | KU (gating) | Apple documents a view-associated link, but documentation does not prove that a separate observer has an independent callback stream from either candidate, nor does it prove the causal matching required by CON-FRM-001. STOP applies to this row. See S3. | P3: run two visible target-display windows and a harness-owned third `NSView` in a separate process under `/tmp/wf-epic-b/OXY-B001/mac-timing-probe/`. Log observer PID, candidate PID, `NSScreen` identity, display-link timestamp, and drawable presentation time for 10 seconds; deliberately block each candidate scheduling callback in turn. Expected output: the harness link continues with the same display association while the blocked candidate stream stops, and trace IDs show no candidate callback is the observer source. |
| B001-11 | Can each view and observer migrate to its current display? | Both | KU (gating) | AppKit posts `NSWindow.didChangeScreenNotification` with the changing window as its object, and the display-link API ties a callback to the display a view is on. The fetched sources do not prove link rebinding timing or a correct epoch change after a cross-display move. STOP applies to this row. See S3 and S11. | P3: move each of two windows between differently timed screens, capture the screen notification object, before-and-after screen identities, display-link periods, and epoch IDs. Expected output: both candidate and harness observer rebind to the moved view's display, start one new epoch, and leave the idle peer unscheduled. |
| B001-12 | May deprecated `CVDisplayLink` serve as the baseline observer? | Both | not applicable-with-citation | No. Apple marks the Core Video display-link management functions as deprecated, and the ticket excludes deprecated timing APIs from independent evidence. See S17. | Not applicable. |
| B001-13 | Does focused service routing reject an implicit default window? | Focused | KU (gating) | `NSTextInputContext` has an explicit client, and AppKit screen-change notifications identify their window. Neither source proves the focused allocation attaches a view generation to every input, accessibility, clipboard, timing, and recovery request. STOP applies to this row. See S5 and S11. | P4: use two focused-host windows in `/tmp/wf-epic-b/OXY-B001/mac-routing-probe/`, issue interleaved IME, accessibility-action, pasteboard, display, resize, and teardown events, then compare every log record's view generation with the owning native object. Expected output: no default-window lookup, no cross-window delivery, and a typed stale-generation result after teardown. |
| B001-14 | Does integrated service routing have an exact inherited interface inventory and reject an implicit view? | Integrated | KU (gating) | No frozen fork commit or source inventory is available in this spike, so an inherited callback path cannot be classified from an official platform API. AppKit client ownership alone does not establish the engine-to-C-ABI route. STOP applies to this row. See S5. | P6, then P4: freeze the fork revision, emit a path-and-symbol inventory for its macOS embedder callbacks, and run the two-window routing trace through the C ABI. Expected output: inventory digest plus the same no-default, no-cross-window, stale-generation results. |
| B001-15 | Which observable native signals form the recovery baseline? | Both | KK | AppKit documents window resize, screen, and activation notifications. Metal documents command-buffer completion, terminal `error` status, and an error property when the GPU cannot run a command buffer. Apple also states that the legacy device-removal notification is deprecated and not applicable on Apple Silicon, so it is excluded from the arm64 baseline. See S11, S12, S13, S14, S15, S16, and S18. | Not applicable for observation-interface availability. |
| B001-16 | Is focused recovery injectable for resize, surface loss, resume/topology, and graphics failure? | Focused | KU (gating) | The sources establish observations, not a controllable Apple Silicon fault injection route or the focused allocation's recovery behavior, deadlines, retry bound, and release evidence. STOP applies to this row. See S13, S14, S15, and S18. | P5: in `/tmp/wf-epic-b/OXY-B001/mac-recovery-probe/`, first enumerate documented noncandidate actions that produce a completed command buffer with `status == error`; separately inject resize, activation, and screen-change events into two AppKit windows. Expected output: one reproducible action and recorded status/error for graphics failure, plus timestamped recovery traces for each event; otherwise emit `graphics-error-injection-unavailable` and retain this KU. |
| B001-17 | Is integrated recovery injectable through the inherited embedder and C ABI? | Integrated | KU (gating) | No frozen integrated source inventory or fault trace establishes where inherited lifecycle and Metal failures enter the C ABI or whether each can be injected. STOP applies to this row. See S13, S14, and S15. | P6, then P5: freeze the inherited recovery-path inventory and run the same event and error probe through the C ABI. Expected output: a source-inventory digest and one normalized trace per injected event, or a named unavailable injection point. |
| B001-18 | Is immutable evidence available for every remaining status-bearing candidate claim? | Both | KU (gating) | This report preserves the cited official sources and host preflight, but no candidate transcript, map, independent-timing trace, routing trace, recovery trace, or integrated source digest exists. STOP applies to all unresolved candidate claims. See S1-S19. | P7: write fetched S1-S19 response bodies and each successful P1-P6 result to a content-addressed artifact set with source revision, command, host hardware and OS identifiers, input fixture digest, raw output digest, and validator result. Expected output: one manifest whose SHA-256 entries verify every referenced artifact. |

## Context and objective

- **Triggering upstream file or section:** `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.macos`
- **Target:** Close the source-availability rows with cited KK evidence and retain only the bounded behavior, mapping, independence, routing, recovery-injection, and evidence-artifact KUs in the decision register.
- **Archetype / surface:** Library/SDK with System/Native macOS integration.
- **Interpretation:** A source-documented API is KK for availability only. It is not evidence that either allocation implements or qualifies against that API.

## Codebase baseline

- **State today:** Stage 3 pins Xcode 26.6, build 17F113, and the macOS 26.5 SDK. Apple confirms that this Xcode release includes that SDK. See S1 and S2.
- **Minimum baseline:** macOS 14.0 is the exact minimum because the required `NSView` display-link API is available from macOS 14.0. See S3.
- **Candidate-neutral interfaces:** The baseline uses `NSTextInputClient`, `NSTextInputContext`, `NSAccessibilityProtocol` and role-specific accessibility protocols or `NSAccessibilityElement`, `NSView.displayLink(target:selector:)`, `MTLDrawable.addPresentedHandler(_:)`, `MTLDrawable.presentedTime`, AppKit lifecycle notifications, and Metal command-buffer completion, status, and error observation. See S3-S16.
- **Excluded interface:** Do not use deprecated `CVDisplayLink` as an independent timing observer. See S17.
- **Apple Silicon recovery constraint:** Do not use `MTLDeviceNotificationName.wasRemoved`; Apple marks it deprecated and not applicable on Apple Silicon. See S18.

### Controlled probe record

The host did not provide macOS, AppKit, Xcode, or `xcrun`. The following noncandidate preflight ran from `/tmp/wf-epic-b/OXY-B001/` and is preserved here verbatim. It establishes why the functional AppKit probes stopped rather than producing fabricated results.

```text
$ /tmp/wf-epic-b/OXY-B001/probe-host.sh
timestamp=2026-08-28T16:26:34Z
kernel=Linux 6.18.44 x86_64
appkit_framework=absent
xcrun=absent
xcodebuild=absent
sw_vers=absent
sdk_path=not-run: xcrun absent
```

### Bounded follow-up probes

| Probe | Scope and command | Procedure and expected output |
| :-- | :-- | :-- |
| P1 | Run on a pinned arm64 macOS 14.0+ host: `cd /tmp/wf-epic-b/OXY-B001/mac-ime-probe && xcrun --sdk macosx clang -fobjc-arc -framework AppKit ime_probe.m -o ime_probe && ./ime_probe --two-views --jsonl transcript.jsonl`. | The noncandidate AppKit app creates one `NSTextInputContext(client:)` per view. Run every required text vector and an installed CJK input source. Expected output is redacted JSONL of callbacks, UTF-16 ranges, client pointer-to-view mapping, conversion results, and no raw secure-field text. |
| P2 | Run on a pinned arm64 macOS 14.0+ host: `cd /tmp/wf-epic-b/OXY-B001/mac-accessibility-probe && xcrun --sdk macosx clang -fobjc-arc -framework AppKit accessibility_probe.m -o accessibility_probe && ./accessibility_probe --two-windows --voiceover-log accessibility.jsonl`. | The noncandidate custom view and element expose one fixture tree. Use VoiceOver to traverse it and invoke every supported action. Expected output is a forward role/property/range/frame map and a reverse action acknowledgement keyed by view and semantics generation. |
| P3 | Run on a pinned arm64 macOS 14.0+ host with two target displays: `cd /tmp/wf-epic-b/OXY-B001/mac-timing-probe && xcrun --sdk macosx clang -fobjc-arc -framework AppKit -framework Metal timing_probe.m -o timing_probe && ./timing_probe --two-views --observer-process --move-displays --seconds 10 --jsonl timing.jsonl`. | The separate harness process owns a third visible view and display link. Block each candidate scheduling stream in turn and move each target window. Expected output records separate PIDs, screen identities, link timestamps, presentation times, and display epochs proving observer-stream independence and migration. |
| P4 | Run after each allocation exists: `cd /tmp/wf-epic-b/OXY-B001/mac-routing-probe && ./routing_probe --allocation focused --two-windows --interleave --teardown && ./routing_probe --allocation integrated --two-windows --interleave --teardown`. | Send interleaved native-service events and destroy one view. Expected output gives each record the owning view generation, rejects stale work, and contains no default-window routing. |
| P5 | Run on a pinned arm64 macOS 14.0+ host: `cd /tmp/wf-epic-b/OXY-B001/mac-recovery-probe && xcrun --sdk macosx clang -fobjc-arc -framework AppKit -framework Metal recovery_probe.m -o recovery_probe && ./recovery_probe --enumerate-command-buffer-errors --resize --activate --change-screen --jsonl recovery.jsonl`. | First determine whether a reproducible noncandidate action yields a completed command buffer with terminal error status. Then trace AppKit recovery events. Expected output names the graphics injection action and produces one recovery trace per event, or explicitly reports `graphics-error-injection-unavailable`. |
| P6 | Run after the integrated fork revision is frozen: `cd /tmp/wf-epic-b/OXY-B001/integrated-inventory && ./inventory.sh "$INTEGRATED_FORK_COMMIT" > inventory.json`, where `INTEGRATED_FORK_COMMIT` is the frozen integrated-fork commit. | Enumerate macOS embedder source paths and symbols for window/input, IME, accessibility, pasteboard, timing, lifecycle, Metal error, and C-ABI crossings. Expected output is a commit-bound JSON inventory with a SHA-256 digest. |
| P7 | Run after collecting S1-S19 response bodies and each successful P1-P6 result: `cd /tmp/wf-epic-b/OXY-B001/evidence-lock && ./lock.sh ../sources ../mac-* > manifest.json`. | Hash official-source bodies, source revisions, commands, host identifiers, fixture inputs, raw outputs, and validators. Expected output is a SHA-256 manifest whose entries verify each result referenced by a status-bearing claim. |

## Options and trade-offs

- **Option A:** Freeze the source-established macOS 14.0, Xcode 26.6/macOS 26.5 SDK, AppKit text and accessibility APIs, `NSView` display link, and Metal presentation and error-observation APIs. This selects compatibility only where Apple availability documentation closes the interface question.
- **Option B:** Raise the deployment target above macOS 14.0 to reduce fallback work. This has no cited requirement because the identified view-linked timing API is already available on macOS 14.0, so this spike does not select it.
- **Option C:** Retain a gating KU for behavior that API documentation and the unavailable host probe cannot establish. This keeps candidate implementation and qualification blocked only on bounded evidence gaps.

## Recommendation

- **Chosen option:** A/C mix. Choose A for B001-01, B001-02, B001-03, B001-06, B001-09, and B001-15. Choose C for B001-04, B001-05, B001-07, B001-08, B001-10, B001-11, B001-13, B001-14, and B001-16 through B001-18. B001-12 is not applicable with citation.
- **Why it fits:** This result freezes only documented interface availability and does not treat platform plausibility, a native callback, or a source listing as evidence of candidate behavior. Each retained gate has one bounded next probe with a location, command, and expected output. P7 remains required before Stage 3 promotes any contract `ku-gating` value to `kk`, because that schema requires a preserved path and SHA-256 rather than a URL string.
- **Rejected options:** Reject Option B because no fetched availability evidence requires a higher target. Reject deprecated `CVDisplayLink`, candidate-internal clocks as independent meters, `MTLDeviceNotificationName.wasRemoved` on the arm64 reference, default-window routing, and a map or recovery claim without preserved traces.
- **Capability and architecture guard:** The recommendation preserves the existing P0 capabilities and the accepted Platform integration and reentrancy boundaries. It chooses no substrate and introduces no new product capability or architecture boundary.

## Downstream impact

- **ADRs to write or update:** No architecture decision record change is required for the KK availability facts. Stage 3 can update `ADR-0005-platform-hosts.md` only if it needs to record the macOS 14.0 host baseline; do not alter the accepted direct-AppKit or normalized-callback boundary. Do not change `ADR-0006-execution-domains.md`.
- **Tickets unblocked in `tasks/active/`:** Stage 3 can freeze the source-availability portions of the macOS environment. Candidate implementation and measurement remain blocked by the KU rows, particularly P1-P7.
- **Tickets to add or split:** Add bounded follow-up work only for P1 IME behavior, P2 accessibility maps, P3 independent timing and migration, P4 service routing, P5 recovery injection, P6 integrated inventory, and P7 evidence locking if Epic B cannot schedule them as part of the existing macOS qualification work.
- **Spec edits required:** Apply only the following Stage 3 edits after accepting this report; do not use these edits to close any KU row.

| File and field or section | Exact proposed value or instruction |
| :-- | :-- |
| `.constitution/tech-spec/stack.md` -> `Platform qualification pins` -> macOS reference configuration | Replace `minimum deployment target is a gating KU` with `minimum deployment target macOS 14.0; contract promotion awaits P7 immutable evidence`. |
| `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.macos.minimumVersion` | Retain the exact current object `{"status":"ku-gating","value":null,"evidence":[]}` until P7 emits the immutable evidence object. Then set `status` to `kk`, `value` to `14.0`, and `evidence` to the P7-produced `path` and command-produced SHA-256. Do not use a URL string as `evidence`. |
| `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.macos.protocols[0]` | Retain the exact current `status:"ku-gating"`, `version:null`, and `evidence:[]` until P7 emits immutable evidence. Then set `version` to `macOS 26.5 SDK via Xcode 26.6 (17F113)`, set `status` to `kk`, and use the P7-produced evidence object. |
| `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.macos.ime.evidence` | Retain the exact current `[]` and `status:"ku-gating"`; P1 and P7 must produce the evidence object before any `kk` promotion. The cited interface URLs are report citations, not schema-valid evidence entries. |
| `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.macos.timing.presentationFeedback` | Replace with `MTLDrawable.addPresentedHandler(_:) with MTLDrawable.presentedTime`; retain `status:"ku-gating"` and `evidence:[]` until P3 and P7 complete. |
| `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.macos.allocations.focused.recoveryInterfaces` | Replace the second value with `Metal command-buffer completion, status, and error signals; exclude MTLDeviceNotificationName.wasRemoved on Apple Silicon`. Retain `status:"ku-gating"` and `evidence:[]` until P1-P5 and P7 produce focused allocation evidence. |
| `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.macos.accessibilityMaps`, `recoveryBaseline`, and `allocations.integrated` | Retain every existing `ku-gating` status and every `null` path or SHA-256 value until P2, P5, P6, and P7 produce immutable artifacts. Do not add a map path, recovery path, or integrated interface claim before then. |
| `.constitution/tech-spec/contracts/qualification-lock.json` -> `referenceEnvironments.macos-arm64.minimumVersion` | Retain `null` until P7 supplies the immutable evidence required to synchronize this lock with the platform contract. Retain the shared `minimum-platform-and-protocol-versions` KU entries because other environments and macOS behavior gates remain unresolved. |

## Sources

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
