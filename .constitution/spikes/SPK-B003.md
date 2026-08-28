# Spike report: OXY-B003 Wayland qualification baseline

## Time box

- **Budget:** 1 focused day.
- **Clock start / stop:** 2026-08-28T16:22:52Z / 2026-08-28T16:34:05Z.

## Question

- **Decision this spike produces:** Use `wp_presentation` version 2 only as per-commit acknowledgement. Use `GtkIMContext` writable input-purpose and input-hints properties, and convert its documented UTF-8 byte cursor positions explicitly. Use Orca with AT-SPI 2 as the Linux assistive-technology test client. Retain the reference-compositor, candidate-transcript, complete-map, independent-meter, routing, and recovery gates until their bounded reference probes pass.

Table 1 answers each Wayland baseline question. KK is a verified fact. KU (gating) is a named unresolved gate. No row is not applicable.

Table 1. Wayland baseline decisions

| Row | Answer and evidence | Status | Next bounded probe |
| :-- | :-- | :-- | :-- |
| Reference compositor, session, and package lock | [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/) establish Ubuntu 26.04 LTS, but the fetched release-note content names no compositor, session, package version, or package-lock digest. The non-reference host is NixOS 26.05 with Hyprland 0.55.4, so its registry cannot establish Ubuntu compositor behavior. | KU (gating) | P1: On the selected Ubuntu 26.04 x86-64 Wayland session, record `gnome-shell --version` or the selected compositor's version command, `dpkg-query -W` for the compositor, `gtk4`, `wayland-protocols`, and `at-spi2-core`, the package-manifest SHA-256, and a filtered `wayland-info` registry. Run a 120-frame visible-surface probe that records `wl_surface.frame`, `wp_presentation_feedback.presented` or `discarded`, and `sync_output` events. Expected output: one named compositor version, one package-lock digest, and a session-specific event transcript. |
| Wayland core and `wp_presentation` protocol floor | The [upstream core protocol](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml) defines `wl_surface` version 6 and its frame, enter, and leave events. The [pinned presentation-time XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/presentation-time/presentation-time.xml) defines `wp_presentation` version 2. A non-reference `wayland-info` probe advertises `wp_presentation` version 2, which verifies registry mechanics only. | KK | Not required. P1 remains required to establish availability and behavior on the Ubuntu reference session. |
| GTK release floor on the reference | [GNOME publishes GTK 4.20.4](https://download.gnome.org/sources/gtk/4.20/), and the documented APIs needed for selection-aware IME, [`GtkAccessible`](https://docs.gtk.org/gtk4/iface.Accessible.html), and [`GtkAccessibleText`](https://docs.gtk.org/gtk4/iface.AccessibleText.html) were introduced no later than GTK 4.14. This does not identify the Ubuntu package revision, package digest, or session backend. | KU (gating) | P1: Record the installed `gtk4` package version and immutable package-manifest digest on the Ubuntu reference. Accept it only when it is GTK 4.20.4 or a separately reviewed replacement that exposes the cited API set. Expected output: package version, package origin, and digest. |
| `GtkIMContext` surrounding text and input-purpose mechanism | [`set_surrounding`](https://docs.gtk.org/gtk4/method.IMContext.set_surrounding.html) takes UTF-8 text and a byte index for the cursor. [`input-purpose`](https://docs.gtk.org/gtk4/property.IMContext.input-purpose.html) and [`input-hints`](https://docs.gtk.org/gtk4/property.IMContext.input-hints.html) are writable properties. [`GtkInputPurpose`](https://docs.gtk.org/gtk4/enum.InputPurpose.html) supplies typed purpose values, including `PASSWORD` and `PIN`; [`GtkInputHints.PRIVATE`](https://docs.gtk.org/gtk4/flags.InputHints.html) requests that an input method not update personalized data. These are properties, not a compositor numeric negotiation. | KK | Not required. P2 verifies the selected input method's behavior rather than the documented interface shape. |
| Complete IME transcript and non-cursor operation units | GTK documents [`delete-surrounding`](https://docs.gtk.org/gtk4/signal.IMContext.delete-surrounding.html) arguments as character offsets and counts, but it does not state the scalar, grapheme, or another unit in the fetched API page. No selected Ubuntu IM module or candidate transcript exists. The report therefore does not infer a unit for deletion, preedit cursor position, or replacement behavior. | KU (gating) | P2: On the P1 session, use an instrumented noncandidate GTK 4.20.4 text widget and the ASCII, multibyte, combining, bidirectional, CJK-composition, replacement, candidate-geometry, and secure-field corpus. Log every `preedit-*`, `commit`, `retrieve-surrounding`, `delete-surrounding`, `focus-*`, and `reset` callback with typed indices. Expected output: a transcript that identifies every operation's unit and round trips each valid boundary. |
| Linux assistive-technology selection | Select [Orca](https://help.gnome.org/users/orca/stable/) as the required screen-reader test client and AT-SPI 2 as its inspection and action transport. [GNOME's AT-SPI development documentation](https://gnome.pages.gitlab.gnome.org/at-spi2-core/devel-docs/atspi-python-stack.html) states that Orca builds a view of an application's accessible-object tree through `libatspi` and `pyatspi2`. | KK | Not required. P3 establishes the Ubuntu package lock and candidate behavior. |
| AT-SPI text offsets and Unicode conversion | AT-SPI [`Text.get_text`](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/method.Text.get_text.html) states that UTF-8 result bytes can exceed text offsets. [`Text.get_character_at_offset`](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/method.Text.get_character_at_offset.html) returns the UCS-4 Unicode code point at an offset, and [`EditableText`](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.EditableText.html) says its character positions can differ from byte offsets. Freeze AT-SPI text, caret, selection, and editable-text positions as Unicode-scalar boundaries. The preserved fixture table proves the required scalar-to-UTF-8, scalar-to-UTF-16, scalar-to-grapheme, and scalar-to-logical conversions for the required corpus. | KK | Not required for conversion mechanics. P3 must prove the same calls against each candidate's exported AT-SPI tree. |
| Focused allocation accessibility map | [GTK defines an accessibility tree](https://docs.gtk.org/gtk4/iface.Accessible.html) with role, state, property, and relation attributes and a platform accessibility context. No focused candidate source identity, exported tree, forward map, reverse action map, artifact path, or digest exists. | KU (gating) | P3F: After the focused source identity is locked, launch its two-view AT-SPI fixture under Orca and `pyatspi2`. Enumerate every required `accessibility-map.schema.json` forward key and reverse action, including Unicode-scalar text payloads, view generation, acknowledgement, stale target, and secure-field redaction. Expected output: one complete map JSON file and SHA-256. |
| Integrated allocation accessibility map | The [GTK accessibility interfaces](https://docs.gtk.org/gtk4/iface.Accessible.html) document a possible host mechanism, but they do not establish the pinned Flutter fork's inherited interfaces or its Oxyflut map. No fork commit, source tree, exported tree, forward map, reverse action map, artifact path, or digest exists. | KU (gating) | P3I: After the integrated fork and adapter commits are locked, run the same two-view Orca and `pyatspi2` fixture. First inventory inherited GTK and AT-SPI interfaces, then enumerate every forward key and reverse action. Expected output: the inventory, one complete map JSON file, and SHA-256. |
| Host scheduling and presentation feedback roles | [`GdkFrameClock`](https://docs.gtk.org/gdk4/class.FrameClock.html) tells an application when to update and repaint, but GTK states that it can use a simple timer instead of hardware vertical sync. The [presentation-time protocol](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/presentation-time/presentation-time.xml) creates feedback for a submitted `wl_surface.commit` and emits one terminal presented or discarded result for that content update. Therefore `GdkFrameClock` is only a host wakeup mechanism until P4 qualifies it, and `wp_presentation` feedback is acknowledgement only, never an independent opportunity meter. | KK | Not required for the interface-role decision. P4 qualifies the meter and scheduling behavior. |
| Independent presentation-opportunity meter | No fetched compositor evidence or host probe proves an output-associated timing source that is independent of both candidate callback streams; the [presentation-time protocol](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/presentation-time/presentation-time.xml) only describes per-submission feedback. The host exposes a DRM card node and a `wp_drm_lease_device_v1` global, but that does not prove KMS authority, active-output attribution, calibration, or reference-session behavior. | KU (gating) | P4: On the P1 session, run a separately launched, harness-owned visible Wayland client with its own `wl_surface.frame` callbacks and monotonic log beside each candidate. Bind the observer and candidate surfaces to each entered output set, prove no shared callback or IPC path, compare 10-second epochs against an independently captured display trace, and record calibration error. Expected output: observer source digest, process graph, per-output epoch log, and calibration result that meets `CON-FRM-001`. |
| Output association mechanism | The [core protocol](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml) states that a surface can be displayed on zero or more outputs. It emits `wl_surface.enter` and `wl_surface.leave` when surface creation, movement, or resizing changes output membership. | KK | Not required for protocol mechanics. P4 and P5 apply the mechanism to each allocation. |
| Focused allocation service routing | No focused candidate exists to prove that every GTK, Wayland, IME, accessibility, clipboard, timing, and recovery request carries its owning `GdkSurface` and view generation across the reentrancy barrier; the [core protocol](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml) only establishes surface identity mechanics. | KU (gating) | P5F: Use an instrumented two-window focused fixture. Interleave focus, IME, AT-SPI reverse action, clipboard, output move, close, and late-callback events. Expected output: normalized event log in which every request has the expected surface identity and live view generation, and stale events return the defined error. |
| Integrated allocation service routing | No pinned Flutter fork or adapter exists to prove that every inherited callback carries its owning `GdkSurface` and view generation before the C ABI and reentrancy barrier; the [core protocol](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml) does not identify inherited Flutter callbacks. | KU (gating) | P5I: Run the P5F scenario through the locked integrated fork. Expected output: inherited-interface inventory and normalized C-ABI event log with the same ownership, generation, and stale-event results. |
| Focused allocation recovery injection | The [Vulkan registry](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/20a9e5892e2aab7b9776b16a238b10fc8133090a/xml/vk.xml) defines `VK_ERROR_DEVICE_LOST`, but it does not provide an injectable focused-candidate recovery baseline. The focused allocation has no source identity, fault seam, surface-loss control, retry trace, or recovery evidence. | KU (gating) | P6F: After the focused source identity is locked, expose test-only commands that inject resize completion, surface loss, resume or topology change, and `VK_ERROR_DEVICE_LOST` at the adapter boundary. Run each fault during a two-view fixture. Expected output: fault timestamp, three-or-fewer recreation attempts, recovery acknowledgement, superseded-resource release time, and structured terminal error when recovery fails. |
| Integrated allocation recovery injection | The [Vulkan device-loss result](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/20a9e5892e2aab7b9776b16a238b10fc8133090a/xml/vk.xml) does not establish the Flutter fork's lifecycle or graphics recovery path. The integrated allocation has no fork commit, test-only fault seam, retry trace, or recovery evidence. | KU (gating) | P6I: After the fork and adapter commits are locked, expose the same test-only fault commands at the normalized C ABI and run the P6F fixture. Expected output: the same recovery record fields and equivalent pass or terminal-error behavior. |

## Context and objective

- **Triggering upstream file or section:** `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland`.
- **Target:** Resolve documented protocol mechanics and reduce every remaining Wayland item to a bounded gate without claiming reference-session or candidate behavior.
- **Archetype / surface:** Library/SDK with system Wayland desktop integration.
- **STOP-condition result:** Neither STOP condition triggered. This report does not infer compositor behavior from advertised globals, and it does not treat `wp_presentation` feedback as an independent presentation-opportunity source.

## Codebase baseline

- **State today:** Stage 3 pins Ubuntu 26.04 LTS, GTK 4.20, `GtkIMContext`, AT-SPI families, `GdkFrameClock`, and `wp_presentation` feedback. It leaves the reference compositor, package lock, complete candidate maps, independent meter, routing traces, and injectable recovery gates unresolved.
- **Discovered constraints:** A Wayland registry reports interfaces, not reference-compositor timing behavior. `wp_presentation` feedback follows a submitted content update. `GdkFrameClock` can use a timer. Each allocation needs separate evidence even when both use the same host protocol.

### Preserved non-reference probes

The following probes ran on the live NixOS 26.05 Hyprland session. They establish local mechanics only. They do not establish Ubuntu 26.04 compositor behavior, package versions, or qualification results.

The host-identification output is trimmed to the fields used by this report:

```text
$ date -u; printf '%s %s %s\n' "$XDG_SESSION_TYPE" "$XDG_CURRENT_DESKTOP" "$WAYLAND_DISPLAY"; hyprctl version
2026-08-28T16:22:52Z
wayland Hyprland wayland-1
Hyprland 0.55.4 built from branch unknown at commit a0136d8c04687bb36eb8a28eb9d1ff92aea99704 dirty (unknown).
```

The registry probe used `nix shell nixpkgs#wayland-utils -c wayland-info`. The output is trimmed to relevant globals:

```text
interface: 'zwp_text_input_manager_v3',                  version:  1, name: 21
interface: 'wp_presentation',                            version:  2, name: 36
interface: 'wp_drm_lease_device_v1',                     version:  1, name: 64
interface: 'wl_output',                                  version:  4, name: 68
```

The GTK package probe used `nix build --no-link --print-out-paths nixpkgs#gtk4.dev` and `pkg-config` with that package's `lib/pkgconfig` directory:

```text
gtk4_dev_path=/nix/store/dczin2m9wvwsfjyfbnksm983ljgmibfk-gtk4-4.22.4-dev
gtk4_version=4.22.4
imcontext_header_symbols:
void gtk_im_context_focus_in (GtkIMContext *context);
void gtk_im_context_focus_out (GtkIMContext *context);
void gtk_im_context_reset (GtkIMContext *context);
void gtk_im_context_set_surrounding (GtkIMContext *context,
void gtk_im_context_set_surrounding_with_selection
gboolean gtk_im_context_delete_surrounding (GtkIMContext *context,
```

The AT-SPI package and session-bus probes used `nix build --no-link --print-out-paths nixpkgs#at-spi2-core.dev`, `pkg-config`, and `busctl --user`:

```text
atspi_dev_path=/nix/store/p0ijpx5x5kppx8jrpqzk5zk6ksfw55vl-at-spi2-core-2.60.6-dev
atspi2_version=2.60.6
org_a11y_bus=absent
```

The host's DRM access probe found a video-group card node and a world-readable render node. This result does not establish KMS-master authority or per-output timing attribution:

```text
crw-rw----+ 1 root video  226,   1 card1
crw-rw-rw-  1 root render 226, 128 renderD128
uid=1000(oscar) gid=100(users) groups=100(users),26(video),...
```

### Unicode-scalar offset fixture

The fixture script at `/tmp/wf-epic-b/OXY-B003/offset-fixtures.py` ran with `nix shell nixpkgs#python3 -c python3 /tmp/wf-epic-b/OXY-B003/offset-fixtures.py`. `scalar` is the AT-SPI boundary. `logical` preserves storage order, including the bidirectional fixture. `not-boundary` rejects a scalar position that is inside a grapheme cluster.

```text
fixture=ASCII repr='abZ'
scalar|utf8_bytes|utf16_units|grapheme|logical
0|0|0|0|0
1|1|1|1|1
2|2|2|2|2
3|3|3|3|3

fixture=multibyte repr='A界😀'
scalar|utf8_bytes|utf16_units|grapheme|logical
0|0|0|0|0
1|1|1|1|1
2|4|2|2|2
3|8|4|3|3

fixture=combining repr='éx'
scalar|utf8_bytes|utf16_units|grapheme|logical
0|0|0|0|0
1|1|1|not-boundary|1
2|3|2|1|2
3|4|3|2|3

fixture=bidirectional repr='AאB'
scalar|utf8_bytes|utf16_units|grapheme|logical
0|0|0|0|0
1|1|1|1|1
2|3|2|2|2
3|4|3|3|3

result=all Unicode-scalar boundaries round-trip through UTF-8 and UTF-16; only declared fixture grapheme boundaries convert to grapheme indices
```

The fixture covers scalar-boundary conversion only. It does not substitute for P2's actual IME transcript or P3's candidate AT-SPI calls.

### Source record

The report relies on the following fetched authoritative sources:

- [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/).
- [GNOME GTK 4.20 source index](https://download.gnome.org/sources/gtk/4.20/).
- [GTK 4.20.4 SHA-256 file](https://download.gnome.org/sources/gtk/4.20/gtk-4.20.4.sha256sum).
- [GTK `GtkIMContext` API](https://docs.gtk.org/gtk4/class.IMContext.html).
- [GTK `GtkIMContext.set_surrounding` API](https://docs.gtk.org/gtk4/method.IMContext.set_surrounding.html).
- [GTK `GtkIMContext::delete-surrounding` signal](https://docs.gtk.org/gtk4/signal.IMContext.delete-surrounding.html).
- [GTK `GtkIMContext:input-purpose` property](https://docs.gtk.org/gtk4/property.IMContext.input-purpose.html).
- [GTK `GtkInputPurpose` enumeration](https://docs.gtk.org/gtk4/enum.InputPurpose.html).
- [GTK `GtkIMContext:input-hints` property](https://docs.gtk.org/gtk4/property.IMContext.input-hints.html).
- [GTK `GtkInputHints` flags](https://docs.gtk.org/gtk4/flags.InputHints.html).
- [GTK `GtkAccessible` API](https://docs.gtk.org/gtk4/iface.Accessible.html).
- [GTK `GtkAccessibleText` API](https://docs.gtk.org/gtk4/iface.AccessibleText.html).
- [GTK `GdkFrameClock` API](https://docs.gtk.org/gdk4/class.FrameClock.html).
- [Orca screen-reader documentation](https://help.gnome.org/users/orca/stable/).
- [GNOME AT-SPI documentation for Orca and `libatspi`](https://gnome.pages.gitlab.gnome.org/at-spi2-core/devel-docs/atspi-python-stack.html).
- [AT-SPI `Text.get_text` API](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/method.Text.get_text.html).
- [AT-SPI `Text.get_character_at_offset` API](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/method.Text.get_character_at_offset.html).
- [AT-SPI `EditableText` API](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.EditableText.html).
- [Pinned Wayland presentation-time protocol XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/presentation-time/presentation-time.xml).
- [Pinned Wayland core protocol XML](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml).
- [Pinned Vulkan registry XML](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/20a9e5892e2aab7b9776b16a238b10fc8133090a/xml/vk.xml).

The pinned source probe produced these immutable content digests:

```text
presentation-time-pinned sha256=dffac93bcb2bb1d8c385e72b8a8c2c0d4d79a336866322f3ba886dce2b27b1e2
core-wayland-pinned sha256=7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610
vulkan-registry-pinned sha256=3ff4984b841932e04eebeb4ce2a6613ebd37c00ffb2e96549785b2c5d7da9e1d
gtk-4.20.4-official-sum=a21f825bd44afc4dd99ba4eea8ff57c8f2e51085cb402a68ed4cbb35299826a4
```

## Options and trade-offs

- **Option A:** Freeze the selected Ubuntu compositor session, package manifest, and protocol registry only after P1 records compositor/version evidence and the visible-surface transcript. This is required for a reference baseline, but it is not complete in this spike.
- **Option B:** Use a separately launched, harness-owned Wayland client with its own visible `wl_surface.frame` callback stream as the prospective opportunity observer. It has a separate process and callback path, but P4 must establish output attribution and timestamp calibration before it becomes a meter.
- **Option C:** Keep candidate behavior and environment-dependent rows as gating KUs. This prevents the reference distribution label, protocol advertisement, `GdkFrameClock`, or per-commit feedback from becoming unearned qualification evidence.

## Recommendation

- **Chosen option:** Use a mix of A, B, and C. Freeze protocol mechanics from cited upstream sources, use Orca and AT-SPI 2 with Unicode-scalar offsets for the common accessibility baseline, require the Option B observer design for P4, and retain Option C for every unproven reference-session and candidate-specific row.
- **Why it fits:** The recommendation gives both allocations the same documented IME and AT-SPI conversions without converting an interface description into evidence of compositor behavior. The observer design is structurally separate from either candidate, while P4 retains the required proof of independence and calibration.
- **Rejected options:** Reject a nominal refresh-rate timer, `wp_presentation` feedback as an opportunity source, a protocol-global list as compositor behavior, an unspecified assistive technology, a global IME index unit for every operation, and a candidate map inferred from GTK documentation.
- **Sensitive-field rule:** Set `GtkInputPurpose` to `PASSWORD` or `PIN` as applicable and set `GtkInputHints.PRIVATE`. Continue to provide only protocol-required redacted surrounding context and never emit raw text to diagnostics. GTK describes the hint as a request, not a privacy guarantee; P2 and P3 must verify the redaction path.

### Spec edits required

Stage 3 can make the following exact edits without changing product capabilities or architecture boundaries:

- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.protocols` -> `wp_presentation`: set `version` to `"2"`, set `status` to `"kk"`, and set `evidence` to `[{"path":"https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/presentation-time/presentation-time.xml","sha256":"dffac93bcb2bb1d8c385e72b8a8c2c0d4d79a336866322f3ba886dce2b27b1e2"}]`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.minimumVersion`: retain `status` as `"ku-gating"`, retain `value` as `null`, and add `"Ubuntu 26.04 compositor/session/package-manifest evidence from P1"` to `openQuestions`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.ime.numericNegotiation`: replace the value with `"Use the writable Gtk.InputPurpose and Gtk.InputHints properties for each focus generation; no project-defined numeric handshake exists. Surrounding cursor and anchor positions use UTF-8 bytes. P2 must establish every other GtkIMContext operation unit."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.interactiveOpportunitySource`: replace the value with `"GdkFrameClock is a host wakeup only; each allocation must prove output-associated display-synchronized scheduling in P4."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.independentMeterSource`: replace the value with `"A separately launched harness-owned visible Wayland client with its own wl_surface.frame callback and monotonic log; it is a meter only after P4 proves output association, timestamp calibration, and no shared candidate callback or IPC path."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.presentationFeedback`: replace the value with `"wp_presentation v2 feedback for per-commit acknowledgement and main-output association only; never an independent presentation-opportunity meter."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.perDisplayAssociation`: replace the value with `"Track each wl_surface enter/leave output set and begin a display epoch on every set change. Use wp_presentation_feedback.sync_output only to label a submitted frame's main output."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.accessibilityMaps`, `recoveryBaseline`, `allocations.focused`, and `allocations.integrated`: retain every `"ku-gating"` status and `null` path/digest until P3F, P3I, P5F, P5I, P6F, and P6I produce the named immutable artifacts.
- `.constitution/tech-spec/stack.md` -> `Platform qualification pins` -> `Wayland` row: replace `"minimum compositor and protocol versions are gating KUs"` with `"the Ubuntu compositor/session package manifest remains a gating KU; protocol mechanics require wp_presentation v2, and P1 must record the selected session's package versions, manifest digest, registry, and visible-surface transcript"`.
- `.constitution/tech-spec/contracts/qualification-lock.json` -> `preImplementationKnownUnknowns` and `gatingKnownUnknowns`: add `"wayland-ubuntu-compositor-session-package-lock"`, `"wayland-ime-operation-unit-transcript"`, `"wayland-orca-atspi-maps-for-both-allocations"`, `"wayland-independent-observer-calibration"`, `"wayland-service-routing-for-both-allocations"`, and `"wayland-recovery-injection-for-both-allocations"`.
- `.constitution/tech-spec/adrs/ADR-0005-platform-hosts.md` -> `Consequences`: add `"Wayland qualification uses wp_presentation v2 for per-commit acknowledgement and output labeling, not as the independent presentation-opportunity meter."`

## Downstream impact

- **ADRs to write or update:** Stage 3 updates `ADR-0005-platform-hosts.md` with the `wp_presentation` boundary. `ADR-0006-execution-domains.md` requires no change because the report does not alter its queue or ownership boundary.
- **Tickets unblocked in `tasks/active/`:** `OXY-D001` can consume the documented protocol and conversion mechanics, but it remains blocked from qualification measurements by P1 through P6.
- **Tickets to add or split:** Add P1 through P6 as bounded Wayland evidence tasks if the Stage 4 plan does not already schedule equivalent probes.
- **Remaining gates:** The 10 KU rows retain the Wayland environment as `ku-gating`. Neither allocation is eligible for scoring until they close.
