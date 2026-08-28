# Spike report: OXY-B003 Wayland qualification baseline

## Time box

- **Budget:** 1 focused day.
- **Initial report clock start / stop:** 2026-08-28T17:34:55Z / 2026-08-28T17:45:57Z.
- **Round-4 correction clock start / stop:** 2026-08-28T17:54:56Z / 2026-08-28T18:06:35Z.
- **Round-5 correction clock start / stop:** 2026-08-28T18:14:44Z / 2026-08-28T18:23:58Z.

## Question

- **Decision this spike produces:** Freeze source-level Wayland core, shell, scale, text-input, clipboard, and presentation protocol floors from the pinned XML. Keep Ubuntu reference-session advertisement and behavior as a gating KU until P1 records the selected session's package lock, registry, and complete P0-operation transcript. Freeze GTK 4.20.4 and AT-SPI 2.60.6 source API floors, but retain their reference-package and candidate-behavior gates. Use writable `GtkIMContext` input-purpose and input-hints properties, and convert documented UTF-8 byte cursor positions explicitly. Use Orca with AT-SPI 2 as the Linux assistive-technology test client. Freeze documented AT-SPI character offsets as Unicode scalar boundaries, but retain scalar-to-`TextIndex::Logical` conversion, text, caret, selection, and editable-operation behavior as gating KUs. Select the Linux DRM `drm:drm_vblank_event` tracepoint as P4's prospective trace candidate, while retaining Ubuntu kernel identity, live schema and call-site semantics, independence, source access, output attribution, and clock-calibration and causal-matching tolerance as gating KUs. Retain the complete-map, routing, and recovery gates until their bounded reference probes pass.

Table 1 answers each Wayland baseline question. KK is a verified fact. KU (gating) is a named unresolved gate. No row is not applicable.

Table 1. Wayland baseline decisions

| Row | Answer and evidence | Status | Next bounded probe |
| :-- | :-- | :-- | :-- |
| Reference compositor, session, and package lock | [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/) establish Ubuntu 26.04 LTS, but the fetched release-note content names no compositor, session, package version, or package-lock digest. The non-reference host is NixOS 26.05 with Hyprland 0.55.4, so its registry cannot establish Ubuntu compositor behavior. | KU (gating) | P1: On the selected Ubuntu 26.04 x86-64 Wayland session, record `gnome-shell --version` or the selected compositor's version command, `dpkg-query -W` for the compositor, `gtk4`, `wayland-protocols`, and `at-spi2-core`, the package-manifest SHA-256, a filtered `wayland-info` registry, and the mechanically derived 99-member P1 checklist below. Run a 120-frame visible-surface probe with `WAYLAND_DEBUG=client` that binds every required global, creates every required non-global object, and emits every checklist member. The script parses the preserved floor derivation, so P1 must regenerate the checklist rather than maintain a manual operation list. The fixture uses `wl_pointer.set_cursor`, not `cursor-shape-v1`. Expected output: one named compositor version, one package-lock digest, negotiated versions for every required interface, the generated checklist, and a session-specific transcript covering every checklist member. |
| Wayland core object protocol floors | The pinned [Wayland core XML](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml) establishes these operation-derived floors: `wl_compositor` 1, `wl_surface` 1, `wl_callback` 1, `wl_seat` 5, `wl_pointer` 5, `wl_keyboard` 4, `wl_touch` 3, `wl_output` 3, `wl_data_device_manager` 1, `wl_data_device` 2, `wl_data_offer` 1, and `wl_data_source` 1. The preserved XML parser output names every required request and event. The P0 completeness derivation now includes per-view and protocol-object teardown: `wl_surface.destroy`; `wl_seat.release`; `wl_pointer.release`; `wl_keyboard.release`; `wl_touch.release`; `wl_output.release`; and `wl_data_device.release`, as well as cursor, keyboard keymap and repeat, touch, output geometry and scale, clipboard selection and offers, and text-input candidate geometry. `wl_seat.release` raises its floor to 5; `wl_touch.release` and `wl_output.release` raise their floors to 3; and `wl_data_device.release` raises its floor to 2. `wl_pointer` 5 still supplies `axis_source`, `axis_stop`, and `frame`; `wl_keyboard` 4 still supplies `repeat_info`. | KK | Not required for the source-level floors. P1 must bind each required global and create every listed non-global object at the listed floor. |
| Wayland shell, scale, IME, and presentation protocol floors | The pinned [xdg-shell](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/xdg-shell/xdg-shell.xml), [viewporter](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/viewporter/viewporter.xml), [fractional-scale](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/staging/fractional-scale/fractional-scale-v1.xml), [text-input-v3](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/unstable/text-input/text-input-unstable-v3.xml), and version-1 [presentation-time](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml) XML establish floor 1 for `xdg_wm_base`, `xdg_surface`, `xdg_toplevel`, `wp_viewporter`, `wp_viewport`, `wp_fractional_scale_manager_v1`, `wp_fractional_scale_v1`, `zwp_text_input_manager_v3`, `zwp_text_input_v3`, `wp_presentation`, and `wp_presentation_feedback`. The required operations cover toplevel configure acknowledgement, fractional-scale destination sizing, IME surrounding text, candidate geometry through `zwp_text_input_v3.set_cursor_rectangle`, commits, and per-commit `feedback`, `sync_output`, `presented`, or `discarded`; they also cover `xdg_wm_base.destroy`, `xdg_surface.destroy`, `xdg_toplevel.destroy`, `wp_viewporter.destroy`, `wp_viewport.destroy`, `wp_fractional_scale_manager_v1.destroy`, `wp_fractional_scale_v1.destroy`, `zwp_text_input_manager_v3.destroy`, `zwp_text_input_v3.disable`, `zwp_text_input_v3.destroy`, and `wp_presentation.destroy`. Version 2 changes only the variable-refresh `refresh` contract, which the harness does not consume. | KK | Not required for the source-level floors. P1 must bind each required manager global, create its listed non-global objects, and verify the `wp_presentation` transcript. |
| GTK 4.20.4 source API floor | The official [GTK 4.20 source index](https://download.gnome.org/sources/gtk/4.20/) publishes GTK 4.20.4. The immutable [GTK 4.20.4 `gtkenums.h`](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.20.4/gtk/gtkenums.h) source defines `GTK_INPUT_HINT_PRIVATE` and describes it as a request not to update personalized data. The preserved source SHA-256 is `c2ef75dc175e7d8b6a28c1ace0e45898a0f2f4b14454b980fd310e545eb485c9`. | KK | Not required for the source API floor. P1 must lock the Ubuntu package that supplies it. |
| GTK release floor on the reference | The GTK 4.20.4 source floor does not identify the Ubuntu package revision, package digest, or session backend. | KU (gating) | P1: Record the installed `gtk4` package version and immutable package-manifest digest on the Ubuntu reference. Accept this gate only when it is GTK 4.20.4 or a separately reviewed replacement that exposes the cited API set. Expected output: package version, package origin, and digest. |
| `GtkIMContext` surrounding text and input-purpose mechanism | [`set_surrounding`](https://docs.gtk.org/gtk4/method.IMContext.set_surrounding.html) takes UTF-8 text and a byte index for the cursor. [`input-purpose`](https://docs.gtk.org/gtk4/property.IMContext.input-purpose.html) and [`input-hints`](https://docs.gtk.org/gtk4/property.IMContext.input-hints.html) are writable properties. [`GtkInputPurpose`](https://docs.gtk.org/gtk4/enum.InputPurpose.html) supplies typed purpose values, including `PASSWORD` and `PIN`; [`GtkInputHints.PRIVATE`](https://docs.gtk.org/gtk4/flags.InputHints.html) requests that an input method not update personalized data. These are properties, not a compositor numeric negotiation. | KK | Not required. P2 verifies the selected input method's behavior rather than the documented interface shape. |
| Complete IME transcript and non-cursor operation units | GTK documents [`delete-surrounding`](https://docs.gtk.org/gtk4/signal.IMContext.delete-surrounding.html) arguments as character offsets and counts, but it does not state the scalar, grapheme, or another unit in the fetched API page. No selected Ubuntu IM module or candidate transcript exists. The report therefore does not infer a unit for deletion, preedit cursor position, or replacement behavior. | KU (gating) | P2: On the P1 session, use an instrumented noncandidate GTK 4.20.4 text widget and the ASCII, multibyte, combining, bidirectional, CJK-composition, replacement, candidate-geometry, and secure-field corpus. Log every `preedit-*`, `commit`, `retrieve-surrounding`, `delete-surrounding`, `focus-*`, and `reset` callback with typed indices. Expected output: a transcript that identifies every operation's unit and round trips each valid boundary. |
| Linux assistive-technology selection | Select [Orca](https://help.gnome.org/users/orca/stable/) as the required screen-reader test client and AT-SPI 2 as its inspection and action transport. [GNOME's AT-SPI development documentation](https://gnome.pages.gitlab.gnome.org/at-spi2-core/devel-docs/atspi-python-stack.html) states that Orca builds a view of an application's accessible-object tree through `libatspi` and `pyatspi2`. | KK | Not required. P3 establishes the Ubuntu package lock and candidate behavior. |
| AT-SPI API floor | The official [at-spi2-core 2.60.6 release notes](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/NEWS) identify release 2.60.6. The immutable [AT-SPI 2.60.6 `Text.xml`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/Text.xml) defines `CharacterCount`, `GetText`, `SetCaretOffset`, and selections. It does not define editable text. The immutable [AT-SPI 2.60.6 `EditableText.xml`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/EditableText.xml) defines `SetTextContents`, `InsertText`, `CopyText`, `CutText`, `DeleteText`, and `PasteText`. Freeze 2.60.6 as the AT-SPI source API floor. The preserved `Text.xml` SHA-256 is `5c2d5049d2e427d630ca1ae288d0abe321f39c683336cb8a1373f41c4414d614`; the preserved `EditableText.xml` SHA-256 is `2ea1b94822f19b0b00c80b918b89833cfb67d1eeef99d69b8421d0e6f40920ff`. | KK | Not required for the source API floor. P3 must lock the Ubuntu package and run the behavior transcript. |
| AT-SPI documented text-offset unit | The normative [AT-SPI 2.60.6 `Text.xml`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/Text.xml) defines `CharacterCount` as a number of characters that can differ from fetched UTF-8 byte count. It defines `GetText` end offsets as the first character past the range, while the UTF-8 result bytes can exceed those offsets. It also states that `GetCharacterAtOffset` returns "the UCS-4 unicode code point of the given character." The [AT-SPI 2.60.6 `EditableText.xml`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/EditableText.xml) defines edit positions as character offsets that can differ from UTF-8 byte offsets. Therefore the documented AT-SPI text, caret, selection, and editable-position unit is a Unicode scalar boundary, not a UTF-8 byte, UTF-16 unit, or grapheme boundary. The independent conversion fixture verifies scalar, UTF-8, UTF-16, and grapheme-boundary mechanics, not AT-SPI behavior or `TextIndex::Logical` conversion. | KK | Not required for the documented unit or scalar conversion mechanics. The next rows retain the representation and behavior gates. |
| AT-SPI scalar-to-`TextIndex::Logical` conversion | [`ADR-0007`](../tech-spec/adrs/ADR-0007-text-indexing.md) and the preserved contract probe establish that `TextIndex::Logical(u32)` is a logical text position within an immutable layout generation. Neither source defines its representation or a scalar-to-logical mapping. The scalar fixture therefore makes no scalar-to-logical claim. | KU (gating) | P3B: Before candidate geometry qualification, freeze the `TextIndex::Logical` representation in the public contract and add four hand-listed scalar-to-logical and logical-to-scalar pair tables for ASCII, multibyte, combining, and bidirectional layouts. Bind each pair to one `TextLayoutId` and assert rejection after its generation changes. Expected output: the adopted representation, four bidirectional pair tables, and stale-generation failures. |
| AT-SPI text, caret, selection, and editable behavior | The host has no `org.a11y.Bus`, and the fixture makes no AT-SPI calls. The AT-SPI source establishes the unit, not that a selected GTK exporter or either candidate applies it consistently to `GetText`, `CaretOffset`, selections, `SetCaretOffset`, and editable operations on the combining fixture. | KU (gating) | P3: On the P1 Ubuntu session, start a headless accessibility bus with `dbus-run-session` and `at-spi-bus-launcher`, then use a noncandidate GTK text widget and `pyatspi2` to record `CharacterCount`, `GetText`, `CaretOffset`, selection bounds, `SetCaretOffset`, and editable-operation results for every fixture. Expected output: for `e` plus combining acute plus `x`, `CharacterCount=3`; `GetCharacterAtOffset(1)` and `GetCharacterAtOffset(2)` return the distinct combining-mark and `x` code points; `GetText(0,1)` and `GetText(1,2)` distinguish the first two scalar ranges; and caret, selection, and editable operations round trip offsets 1 and 2. After P3B freezes the logical representation, the typed conversion fixture must assert the approved scalar-to-logical pairs and reject UTF-8, UTF-16, grapheme-interior, and stale-generation positions. |
| Focused allocation accessibility map | [GTK defines an accessibility tree](https://docs.gtk.org/gtk4/iface.Accessible.html) with role, state, property, and relation attributes and a platform accessibility context. No focused candidate source identity, exported tree, forward map, reverse action map, artifact path, or digest exists. | KU (gating) | P3F: After the focused source identity is locked, launch its two-view AT-SPI fixture under Orca and `pyatspi2`. Enumerate every required `accessibility-map.schema.json` forward key and reverse action, including Unicode-scalar text payloads, view generation, acknowledgement, stale target, and secure-field redaction. Expected output: one complete map JSON file and SHA-256. |
| Integrated allocation accessibility map | The [GTK accessibility interfaces](https://docs.gtk.org/gtk4/iface.Accessible.html) document a possible host mechanism, but they do not establish the pinned Flutter fork's inherited interfaces or its Oxyflut map. No fork commit, source tree, exported tree, forward map, reverse action map, artifact path, or digest exists. | KU (gating) | P3I: After the integrated fork and adapter commits are locked, run the same two-view Orca and `pyatspi2` fixture. First inventory inherited GTK and AT-SPI interfaces, then enumerate every forward key and reverse action. Expected output: the inventory, one complete map JSON file, and SHA-256. |
| Host scheduling and presentation feedback roles | [`GdkFrameClock`](https://docs.gtk.org/gdk4/class.FrameClock.html) tells an application when to update and repaint, but GTK states that it can use a simple timer instead of hardware vertical sync. The [version-1 presentation-time XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml) creates feedback for a submitted `wl_surface.commit` and emits one terminal presented or discarded result for that content update. Therefore `GdkFrameClock` is only a host wakeup mechanism until P4 qualifies it, and `wp_presentation` feedback is acknowledgement only, never an independent opportunity meter. | KK | Not required for the interface-role decision. P4 qualifies the meter and scheduling behavior. |
| Independent presentation-opportunity meter | No compositor evidence or host probe proves a qualified meter or the trace's independence from either candidate. The prospective trace is Linux DRM `drm:drm_vblank_event`, captured with `trace-cmd record -e drm:drm_vblank_event`. The fetched [upstream Linux stable v6.18.44 tracepoint definition](https://raw.githubusercontent.com/gregkh/linux/v6.18.44/drivers/gpu/drm/drm_trace.h) gives `TP_PROTO(int crtc, unsigned int seq, ktime_t time, bool high_prec)`. The fetched [upstream Linux stable v6.18.44 vblank call site](https://raw.githubusercontent.com/gregkh/linux/v6.18.44/drivers/gpu/drm/drm_vblank.c) calls `trace_drm_vblank_event(pipe, seq, now, high_prec)` after `drm_crtc_from_index(dev, pipe)`; its adjacent kernel comment defines `pipe` as the index of the CRTC where the event occurred. The [DRM UAPI CRTC-index documentation](https://docs.kernel.org/gpu/drm-uapi.html#crtc-index) states that an index and object ID differ and that `DRM_IOCTL_MODE_GETRESOURCES` returns CRTC IDs in index order. [ftrace documentation](https://docs.kernel.org/trace/ftrace.html) defines `mono` as `CLOCK_MONOTONIC` and documents `trace_marker`. `uname -r` establishes only this non-reference host's release string, not its source or patch identity, and establishes nothing about P4's Ubuntu kernel. The upstream sources therefore establish neither the live schema nor the call-site semantics of P4's kernel. Pipe-to-CRTC mapping remains KU (gating) until P4 preserves the Ubuntu package, source or patch identity, live format, and matching source evidence. The local source-selection probe stopped because the tracepoint and `trace-cmd` are absent, so it establishes no usable trace, output attribution, calibrated clock relation, or independence. | KU (gating) | P4: On the selected P1 Ubuntu 26.04 x86-64 session, first preserve `uname -r` and `dpkg -s linux-image-$(uname -r)`. Then preserve the package's source and patch identity by either running `apt-get source` for the source package selected by that installed image and recording its source version plus the `debian/patches` inventory, recording the installed `linux-source` package and its `debian/patches` inventory, or recording the Ubuntu kernel Git tag and commit resolved by the installed package. Capture `cat /sys/kernel/tracing/events/drm/drm_vblank_event/format` verbatim and its SHA-256. Compare its field schema with the identified kernel source, and preserve source excerpts showing both the tracepoint definition and the `trace_drm_vblank_event` call-site argument semantics. If the live format, source identity, or call-site semantics do not establish that the trace `crtc` field is the call site's pipe index, STOP P4 and retain this KU. Next verify `drm:drm_vblank_event` in `available_events`, permission to record it and write `trace_marker`, and the ability to set `trace_clock` to `mono`. If any check fails, STOP P4 and retain this KU. Before capture, use `drmModeGetResources`; for every established trace pipe `i`, record `resources->crtcs[i]` as the UAPI CRTC object ID, then record each active connector's CRTC object ID, connector identity, mode, and refresh interval. Pair that DRM inventory with contemporaneous `wl_surface.enter` or `leave` and `wl_output` logs; if a pairing is not unambiguous, STOP P4 and retain this KU. Capture a settled 10-second epoch with `trace-cmd record -e drm:drm_vblank_event` and record observer and candidate records on `CLOCK_MONOTONIC`. At epoch start and end, take `t_before = clock_gettime(CLOCK_MONOTONIC)`, write a uniquely identified `P4_CAL` `trace_marker`, then take `t_after = clock_gettime(CLOCK_MONOTONIC)`. Preserve the marker intervals and offset calculations, but do not apply a calibration pass/fail tolerance: it is KU (gating). To derive that tolerance, run a fixed 10,000-marker calibration probe and preserve (1) the selected ftrace clock and its observed timestamp resolution, (2) `clock_getres(CLOCK_MONOTONIC)` and the observed `clock_gettime` resolution, (3) the distribution of `t_after - t_before` as the trace-marker write-latency bound, and (4) the frozen causal-matching algorithm and its matching-window width. Use those four terms to derive and justify a clock-calibration and causal-matching uncertainty budget before setting any acceptance tolerance. Reject an epoch on output-association change. Prove no candidate callback or IPC path feeds the trace by preserving the observer process graph and callback or IPC edge inventory; any such edge fails P4. Apply `CON-FRM-001`'s 10% rule only to the measured 95th-percentile interval-error result after the causal matcher and independent meter are qualified, never to the clock-calibration offset. Expected output: Ubuntu image package record, source or patch identity, live format and SHA-256, source schema and call-site excerpts, trace command, selected trace clock, pipe-to-CRTC-ID-to-connector inventory, surface-output pairing log, four monotonic samples and two `P4_CAL` markers, the 10,000-marker uncertainty-budget record, observer process graph, callback or IPC edge inventory, per-output epoch log, and the separately calculated `CON-FRM-001` result. |
| Output association mechanism | The [core protocol](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml) states that a surface can be displayed on zero or more outputs. It emits `wl_surface.enter` and `wl_surface.leave` when surface creation, movement, or resizing changes output membership. | KK | Not required for protocol mechanics. P4 and P5 apply the mechanism to each allocation. |
| Focused allocation service routing | No focused candidate exists to prove that every GTK, Wayland, IME, accessibility, clipboard, timing, and recovery request carries its owning `GdkSurface` and view generation across the reentrancy barrier; the [core protocol](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml) only establishes surface identity mechanics. | KU (gating) | P5F: Use an instrumented two-window focused fixture. Interleave focus, IME, AT-SPI reverse action, clipboard, output move, close, and late-callback events. Expected output: normalized event log in which every request has the expected surface identity and live view generation, and stale events return the defined error. |
| Integrated allocation service routing | No pinned Flutter fork or adapter exists to prove that every inherited callback carries its owning `GdkSurface` and view generation before the C ABI and reentrancy barrier; the [core protocol](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml) does not identify inherited Flutter callbacks. | KU (gating) | P5I: Run the P5F scenario through the locked integrated fork. Expected output: inherited-interface inventory and normalized C-ABI event log with the same ownership, generation, and stale-event results. |
| Focused allocation recovery injection | The [Vulkan registry](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/20a9e5892e2aab7b9776b16a238b10fc8133090a/xml/vk.xml) defines `VK_ERROR_DEVICE_LOST`, but it does not provide an injectable focused-candidate recovery baseline. The focused allocation has no source identity, fault seam, surface-loss control, retry trace, or recovery evidence. | KU (gating) | P6F: After the focused source identity is locked, expose test-only commands that inject resize completion, surface loss, resume or topology change, and `VK_ERROR_DEVICE_LOST` at the adapter boundary. Run each fault during a two-view fixture and apply the recovery pass rule in this report. Expected output: the fault timestamp, valid and correctly sized acknowledged output, preserved framework state, three-or-fewer recreation attempts, transient-allocation ratio, superseded-resource release time, and a structured terminal error when recovery fails. |
| Integrated allocation recovery injection | The [Vulkan device-loss result](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/20a9e5892e2aab7b9776b16a238b10fc8133090a/xml/vk.xml) does not establish the Flutter fork's lifecycle or graphics recovery path. The integrated allocation has no fork commit, test-only fault seam, retry trace, or recovery evidence. | KU (gating) | P6I: After the fork and adapter commits are locked, expose the same test-only fault commands at the normalized C ABI and run the P6F fixture. Apply the recovery pass rule in this report. Expected output: the same recovery record fields and equivalent pass or terminal-error behavior. |

## Context and objective

- **Triggering upstream file or section:** `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland`.
- **Target:** Resolve documented protocol mechanics and reduce every remaining Wayland item to a bounded gate without claiming reference-session or candidate behavior.
- **Archetype / surface:** Library/SDK with system Wayland desktop integration.
- **STOP-condition result:** Neither ticket STOP condition triggered. This report does not infer compositor behavior from advertised globals, and it does not treat `wp_presentation` feedback as an independent presentation-opportunity source. The non-reference P4 source-selection subprobe stopped because `drm:drm_vblank_event`, `trace-cmd`, and `trace_clock` are unavailable; the independent-meter row retains its gating KU.

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

The valid [GTK `GtkInputHints` flags page](https://docs.gtk.org/gtk4/flags.InputHints.html) and immutable GTK 4.20.4 source both describe the `PRIVATE` hint. The following preserved fetch excerpt supplies the Stage 3 source URL and digest:

```text
url=https://docs.gtk.org/gtk4/flags.InputHints.html
`GTK_INPUT_HINT_PRIVATE`[](https://docs.gtk.org/gtk4/flags.InputHints.html#private)
Request that the input method should not update personalized data (like typing history).
url=https://gitlab.gnome.org/GNOME/gtk/-/raw/4.20.4/gtk/gtkenums.h
 * @GTK_INPUT_HINT_PRIVATE: Request that the input method should not
 *    update personalized data (like typing history)
 *
c2ef75dc175e7d8b6a28c1ace0e45898a0f2f4b14454b980fd310e545eb485c9  /tmp/wf-epic-b/OXY-B003/round-2/sources/gtk-gtkenums.h
exit=0
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

### `wp_presentation` version comparison

The version-1 source records the harness operations. Its `feedback` request associates with the `wl_surface.commit`, its feedback object has `sync_output`, and the object completes with `presented` or `discarded`. The version-2 source changes only the variable-refresh `refresh` contract required by the newer version. The probe fetched both immutable source revisions and compared the relevant declarations:

```text
v1 source=37a1560cf6981a11d44dd200d9409d09b4f0074e
28:  <interface name="wp_presentation" version="1">
73:    <request name="feedback">
126:  <interface name="wp_presentation_feedback" version="1">
141:    <event name="sync_output">
200:    <event name="presented" type="destructor">
258:    <event name="discarded" type="destructor">

v2 source=8cdb39103247fdde5764fc35b1b5cf60698db3e5
28:  <interface name="wp_presentation" version="2">
126:  <interface name="wp_presentation_feedback" version="2">
227:        For version 2 and later, if the output does not have a constant
229:        refresh argument must be either an appropriate rate picked by the
231:        For version 1, if the output does not have a constant refresh rate,
232:        the refresh argument must be zero.
```

The v1 source SHA-256 is `91e5e14481a13717fef8403203a2eaa052c85fd853c1c440ba081effa7178d12`. The v2 source SHA-256 is `dffac93bcb2bb1d8c385e72b8a8c2c0d4d79a336866322f3ba886dce2b27b1e2`. The protocol comparison establishes the version floor only. The non-reference registry's version 2 line does not establish reference-session availability or behavior.

### Wayland baseline source floors

The following XML parser reads the pinned source files and computes each floor as the highest `since` value among the required P0 operations, including every client-issued teardown or release operation for an object used by the P0 flow. XML members with no `since` attribute have version 1. This establishes source API floors only. It does not establish an Ubuntu compositor's advertisement or behavior.

```text
$ nix shell nixpkgs#python3 -c python3 /tmp/wf-epic-b/OXY-B003/round-5/derive-wayland-floors.py
wl_compositor declared=6 required=create_surface@1 floor=1
wl_surface declared=6 required=attach@1,damage@1,frame@1,commit@1,enter@1,leave@1,destroy@1 floor=1
wl_callback declared=1 required=done@1 floor=1
wl_seat declared=10 required=capabilities@1,get_pointer@1,get_keyboard@1,get_touch@1,release@5 floor=5
wl_pointer declared=10 required=enter@1,leave@1,motion@1,button@1,axis@1,axis_source@5,axis_stop@5,frame@5,set_cursor@1,release@3 floor=5
wl_keyboard declared=10 required=keymap@1,enter@1,leave@1,key@1,modifiers@1,repeat_info@4,release@3 floor=4
wl_touch declared=10 required=down@1,up@1,motion@1,frame@1,cancel@1,release@3 floor=3
wl_output declared=4 required=geometry@1,mode@1,done@2,scale@2,release@3 floor=3
wl_data_device_manager declared=3 required=create_data_source@1,get_data_device@1 floor=1
wl_data_device declared=3 required=data_offer@1,enter@1,leave@1,motion@1,drop@1,selection@1,set_selection@1,release@2 floor=2
wl_data_offer declared=3 required=offer@1,receive@1,destroy@1 floor=1
wl_data_source declared=3 required=offer@1,send@1,cancelled@1,destroy@1 floor=1
xdg_wm_base declared=6 required=get_xdg_surface@1,pong@1,ping@1,destroy@1 floor=1
xdg_surface declared=6 required=get_toplevel@1,ack_configure@1,configure@1,destroy@1 floor=1
xdg_toplevel declared=6 required=set_title@1,set_app_id@1,configure@1,close@1,destroy@1 floor=1
wp_viewporter declared=1 required=get_viewport@1,destroy@1 floor=1
wp_viewport declared=1 required=set_destination@1,destroy@1 floor=1
wp_fractional_scale_manager_v1 declared=1 required=get_fractional_scale@1,destroy@1 floor=1
wp_fractional_scale_v1 declared=1 required=preferred_scale@1,destroy@1 floor=1
zwp_text_input_manager_v3 declared=1 required=get_text_input@1,destroy@1 floor=1
zwp_text_input_v3 declared=1 required=enable@1,disable@1,set_surrounding_text@1,set_text_change_cause@1,set_content_type@1,set_cursor_rectangle@1,commit@1,preedit_string@1,commit_string@1,delete_surrounding_text@1,done@1,destroy@1 floor=1
wp_presentation declared=1 required=feedback@1,destroy@1 floor=1
wp_presentation_feedback declared=1 required=sync_output@1,presented@1,discarded@1 floor=1
exit=0
```

### P1 mechanically-derived transcript checklist

The P1 transcript checklist is generated only from the required member list in the preserved Wayland floor derivation. Do not maintain a manual P1 operation list: after any derivation change, run this script again and require the transcript to cover every emitted interface member.

````python
#!/usr/bin/env python3
"""Emit the P1 Wayland transcript checklist from the preserved floor derivation."""

from __future__ import annotations

import re
import sys
from pathlib import Path


REPORT_ANCHOR = "$ nix shell nixpkgs#python3 -c python3 /tmp/wf-epic-b/OXY-B003/round-5/derive-wayland-floors.py"
ROW = re.compile(r"^(?P<interface>[a-z0-9_]+) declared=\d+ required=(?P<operations>[^ ]+) floor=\d+$")


def main() -> None:
    report = Path(sys.argv[1]).read_text(encoding="utf-8")
    start = report.index(REPORT_ANCHOR)
    end = report.index("\n```", start)
    rows = report[start:end].splitlines()[1:]
    checklist: list[str] = []
    for row in rows:
        match = ROW.fullmatch(row)
        if match is None:
            continue
        interface = match.group("interface")
        for operation in match.group("operations").split(","):
            name, _since = operation.rsplit("@", 1)
            checklist.append(f"- {interface}.{name}")
    if not checklist:
        raise SystemExit("no derivation operations found")
    print(f"derived_operations={len(checklist)}")
    print("P1 transcript interface.request/event checklist:")
    print(*checklist, sep="\n")


if __name__ == "__main__":
    main()
````

The parser ran against this report after the corrected floor derivation. Its output is the complete P1 acceptance checklist:

```text
$ nix shell nixpkgs#python3 -c python3 /tmp/wf-epic-b/OXY-B003/round-5/derive-p1-transcript-checklist.py .constitution/spikes/SPK-B003.md
derived_operations=99
P1 transcript interface.request/event checklist:
- wl_compositor.create_surface
- wl_surface.attach
- wl_surface.damage
- wl_surface.frame
- wl_surface.commit
- wl_surface.enter
- wl_surface.leave
- wl_surface.destroy
- wl_callback.done
- wl_seat.capabilities
- wl_seat.get_pointer
- wl_seat.get_keyboard
- wl_seat.get_touch
- wl_seat.release
- wl_pointer.enter
- wl_pointer.leave
- wl_pointer.motion
- wl_pointer.button
- wl_pointer.axis
- wl_pointer.axis_source
- wl_pointer.axis_stop
- wl_pointer.frame
- wl_pointer.set_cursor
- wl_pointer.release
- wl_keyboard.keymap
- wl_keyboard.enter
- wl_keyboard.leave
- wl_keyboard.key
- wl_keyboard.modifiers
- wl_keyboard.repeat_info
- wl_keyboard.release
- wl_touch.down
- wl_touch.up
- wl_touch.motion
- wl_touch.frame
- wl_touch.cancel
- wl_touch.release
- wl_output.geometry
- wl_output.mode
- wl_output.done
- wl_output.scale
- wl_output.release
- wl_data_device_manager.create_data_source
- wl_data_device_manager.get_data_device
- wl_data_device.data_offer
- wl_data_device.enter
- wl_data_device.leave
- wl_data_device.motion
- wl_data_device.drop
- wl_data_device.selection
- wl_data_device.set_selection
- wl_data_device.release
- wl_data_offer.offer
- wl_data_offer.receive
- wl_data_offer.destroy
- wl_data_source.offer
- wl_data_source.send
- wl_data_source.cancelled
- wl_data_source.destroy
- xdg_wm_base.get_xdg_surface
- xdg_wm_base.pong
- xdg_wm_base.ping
- xdg_wm_base.destroy
- xdg_surface.get_toplevel
- xdg_surface.ack_configure
- xdg_surface.configure
- xdg_surface.destroy
- xdg_toplevel.set_title
- xdg_toplevel.set_app_id
- xdg_toplevel.configure
- xdg_toplevel.close
- xdg_toplevel.destroy
- wp_viewporter.get_viewport
- wp_viewporter.destroy
- wp_viewport.set_destination
- wp_viewport.destroy
- wp_fractional_scale_manager_v1.get_fractional_scale
- wp_fractional_scale_manager_v1.destroy
- wp_fractional_scale_v1.preferred_scale
- wp_fractional_scale_v1.destroy
- zwp_text_input_manager_v3.get_text_input
- zwp_text_input_manager_v3.destroy
- zwp_text_input_v3.enable
- zwp_text_input_v3.disable
- zwp_text_input_v3.set_surrounding_text
- zwp_text_input_v3.set_text_change_cause
- zwp_text_input_v3.set_content_type
- zwp_text_input_v3.set_cursor_rectangle
- zwp_text_input_v3.commit
- zwp_text_input_v3.preedit_string
- zwp_text_input_v3.commit_string
- zwp_text_input_v3.delete_surrounding_text
- zwp_text_input_v3.done
- zwp_text_input_v3.destroy
- wp_presentation.feedback
- wp_presentation.destroy
- wp_presentation_feedback.sync_output
- wp_presentation_feedback.presented
- wp_presentation_feedback.discarded
exit=0
```

### DRM trace source selection

The non-reference host's `uname -r` output is the release string `6.18.44`; it is not an exact kernel source or patch identity. The requested `include/trace/events/drm.h` location in the upstream Linux v6.18 source tree is absent, so the definition is in [Linux v6.18 `drivers/gpu/drm/drm_trace.h`](https://raw.githubusercontent.com/torvalds/linux/v6.18/drivers/gpu/drm/drm_trace.h). The fetched [upstream Linux stable v6.18.44 tracepoint definition](https://raw.githubusercontent.com/gregkh/linux/v6.18.44/drivers/gpu/drm/drm_trace.h), [upstream Linux stable v6.18.44 vblank call site](https://raw.githubusercontent.com/gregkh/linux/v6.18.44/drivers/gpu/drm/drm_vblank.c), and [upstream Linux stable v6.18.44 CRTC-index helper](https://raw.githubusercontent.com/gregkh/linux/v6.18.44/include/drm/drm_crtc.h) are source facts for that upstream tag only. The [DRM UAPI CRTC-index documentation](https://docs.kernel.org/gpu/drm-uapi.html#crtc-index) states that a CRTC object ID and index differ, and that `DRM_IOCTL_MODE_GETRESOURCES` supplies CRTC object IDs in index order. They do not establish this host's implementation or P4's Ubuntu call-site semantics. Consequently `pipe = drm_crtc_index(crtc) = crtc->index` and `crtc->base.id = drmModeGetResources(...)->crtcs[pipe]` remain a P4 KU (gating), not an exact-host claim; P4 must never treat `pipe` itself as a UAPI object ID.

The fetched [kernel event-tracing documentation](https://docs.kernel.org/trace/events.html) defines discovery of traceable events in `/sys/kernel/tracing/available_events`. The fetched [ftrace documentation](https://docs.kernel.org/trace/ftrace.html) defines `mono` as the fast `CLOCK_MONOTONIC` clock and states that writing a string to `trace_marker` writes it into the ftrace buffer. P4 would use the generic ftrace record timestamp under `mono`, not the tracepoint payload's `ktime_t time` field, for user-space causal matching. These sources do not establish that the P1 system exposes the tracepoint, grants recording or marker-write access, maps output identities, has an observer independent of either candidate, or supports a clock-calibration tolerance. The tolerance remains KU (gating) until P4 preserves and budgets ftrace timestamp resolution, `clock_gettime` resolution, trace-marker write latency, and the causal-matching window. `CON-FRM-001` applies its 10% limit to the qualified measured interval-error result, not to these clock-calibration terms.

The preserved non-reference source-selection probe is trimmed to the failed primary path, immutable upstream-source digests, the tracepoint schema, and the call site:

```text
$ uname -r
6.18.44
$ curl -fsSL --max-time 60 https://raw.githubusercontent.com/torvalds/linux/v6.18/include/trace/events/drm.h -o /tmp/wf-epic-b/OXY-B003/round-4/sources/drm-trace-events.h
curl: (22) The requested URL returned error: 404
$ sha256sum /tmp/wf-epic-b/OXY-B003/round-4/sources/drm_trace-v6.18.44.h /tmp/wf-epic-b/OXY-B003/round-4/sources/drm_vblank-v6.18.44.c /tmp/wf-epic-b/OXY-B003/round-4/sources/drm_crtc-v6.18.44.h
0b4779e5ccc62e11e2854a89797cb39f97ef21030c114d05e0a2782e670b54f6  /tmp/wf-epic-b/OXY-B003/round-4/sources/drm_trace-v6.18.44.h
c6edb115c1457be17d9a9aa44972694c67ffbb6b331cd858f21a51f39895868e  /tmp/wf-epic-b/OXY-B003/round-4/sources/drm_vblank-v6.18.44.c
5256a74b6b1d614bd8410c01c0c9c654d38355af660ba4b6928ea96ad183ac27  /tmp/wf-epic-b/OXY-B003/round-4/sources/drm_crtc-v6.18.44.h
$ grep -A10 -B1 -F 'TRACE_EVENT(drm_vblank_event,' /tmp/wf-epic-b/OXY-B003/round-4/sources/drm_trace-v6.18.44.h
TRACE_EVENT(drm_vblank_event,
            TP_PROTO(int crtc, unsigned int seq, ktime_t time, bool high_prec),
            TP_ARGS(crtc, seq, time, high_prec),
            TP_STRUCT__entry(
                    __field(int, crtc)
                    __field(unsigned int, seq)
                    __field(ktime_t, time)
                    __field(bool, high_prec)
$ grep -n -A8 -B8 -F 'trace_drm_vblank_event(pipe' /tmp/wf-epic-b/OXY-B003/round-4/sources/drm_vblank-v6.18.44.c
1909-        list_del(&e->base.link);
1910-        drm_vblank_put(dev, pipe);
1911-        send_vblank_event(dev, e, seq, now);
1912-    }
1913-
1914-    if (crtc && crtc->funcs->get_vblank_timestamp)
1915-        high_prec = true;
1916-
1917:    trace_drm_vblank_event(pipe, seq, now, high_prec);
1918-}
1919-
1920-/**
1921- * drm_handle_vblank - handle a vblank event
1922- * @dev: DRM device
1923- * @pipe: index of CRTC where this event occurred
```

The local source-selection probe stopped before capture because the required tracepoint, `trace-cmd`, and trace clock are absent. It makes no timing claim:

```text
$ test -r /sys/kernel/tracing/events/drm/drm_vblank_event/format && sed -n "1,24p" /sys/kernel/tracing/events/drm/drm_vblank_event/format || echo drm_vblank_event_format=unavailable
drm_vblank_event_format=unavailable
$ command -v trace-cmd || echo trace-cmd=absent
trace-cmd=absent
$ test -r /sys/kernel/tracing/trace_clock && cat /sys/kernel/tracing/trace_clock || echo trace_clock=unavailable
trace_clock=unavailable
$ id
uid=1000(oscar) gid=100(users) groups=100(users),1(wheel),20(lp),26(video),57(networkmanager),59(scanner),67(libvirtd),131(docker),174(input),302(kvm),984(davfs2)
exit=0
```

### Unicode-scalar offset fixture

The pinned AT-SPI 2.60.6 `Text.xml` source is normative for the unit. It states that `CharacterCount` can differ from UTF-8 byte count, that `GetText` uses character range offsets while returning UTF-8, and that `GetCharacterAtOffset` returns "the UCS-4 unicode code point of the given character." `EditableText.xml` applies the same character-offset distinction to editable positions. The fixture tests conversions from that documented scalar unit. It does not declare Python string indexes to be AT-SPI boundaries, does not substitute for an AT-SPI call, and does not convert scalar offsets to `TextIndex::Logical`.

The full fixture script at `/tmp/wf-epic-b/OXY-B003/round-3/atspi-scalar-fixtures-uba.py` has independently hand-listed scalar, UTF-8, UTF-16, grapheme, and bidirectional visual-order expectations for each fixture. It asserts scalar-to-UTF-8 and scalar-to-UTF-16 conversions in both directions, validates the hand-listed Unicode Bidirectional Algorithm (UBA) visual-to-scalar map in both directions, and rejects interior UTF-8, UTF-16, and grapheme positions. `TextIndex::Logical` has no specified representation, so the fixture deliberately performs no scalar-to-logical conversion.

The contract probe reports `Logical(u32)` and its immutable-generation scope, but no representation or scalar conversion rule:

```text
$ grep -n -E "Logical|immutable text-layout generation|convert_index" .constitution/tech-spec/contracts/oxyflut-public.rs
34:/// Identifies one immutable text-layout generation.
299:    Logical(u32),
311:    /// Logical positions within one immutable layout generation.
312:    Logical,
327:    /// Logical view size.
409:    /// Logical-key identity.
761:    /// Logical text direction.
1072:    fn convert_index(
exit=0
```

```python
from dataclasses import dataclass

from bidi.algorithm import get_display


@dataclass(frozen=True)
class Fixture:
    name: str
    code_points: tuple[int, ...]
    scalar_boundaries: tuple[int, ...]
    utf8_bytes: tuple[int, ...]
    utf16_units: tuple[int, ...]
    grapheme_boundaries: tuple[int, ...]
    expected_visual_to_scalar: tuple[int, ...]
    rejected_utf8_bytes: tuple[int, ...]
    rejected_utf16_units: tuple[int, ...]


FIXTURES = (
    Fixture("ASCII", (0x0061, 0x0062, 0x005A), (0, 1, 2, 3), (0, 1, 2, 3), (0, 1, 2, 3), (0, 1, 2, 3), (0, 1, 2), (), ()),
    Fixture("multibyte", (0x0041, 0x754C, 0x1F600), (0, 1, 2, 3), (0, 1, 4, 8), (0, 1, 2, 4), (0, 1, 2, 3), (0, 1, 2), (2, 3, 5, 6, 7), (3,)),
    Fixture("combining", (0x0065, 0x0301, 0x0078), (0, 1, 2, 3), (0, 1, 3, 4), (0, 1, 2, 3), (0, 2, 3), (0, 1, 2), (2,), ()),
    Fixture("bidirectional", (0x0041, 0x05D0, 0x05D1, 0x0042), (0, 1, 2, 3, 4), (0, 1, 3, 5, 6), (0, 1, 2, 3, 4), (0, 1, 2, 3, 4), (0, 2, 1, 3), (2, 4), ()),
)


def text_from_code_points(code_points: tuple[int, ...]) -> str:
    return "".join(chr(code_point) for code_point in code_points)


def require_boundary(limit: int, offset: int, unit: str) -> None:
    if offset < 0 or offset > limit:
        raise ValueError(f"{unit} offset outside paragraph")


def scalar_to_utf8_byte(text: str, scalar_offset: int) -> int:
    require_boundary(len(text), scalar_offset, "scalar")
    return len(text[:scalar_offset].encode("utf-8"))


def utf8_byte_to_scalar(text: str, byte_offset: int) -> int:
    utf8 = text.encode("utf-8")
    require_boundary(len(utf8), byte_offset, "UTF-8 byte")
    try:
        prefix = utf8[:byte_offset].decode("utf-8")
    except UnicodeDecodeError as error:
        raise ValueError("UTF-8 byte offset is inside a scalar") from error
    return len(prefix.encode("utf-32-le")) // 4


def scalar_to_utf16_unit(text: str, scalar_offset: int) -> int:
    require_boundary(len(text), scalar_offset, "scalar")
    return len(text[:scalar_offset].encode("utf-16-le")) // 2


def utf16_unit_to_scalar(text: str, unit_offset: int) -> int:
    utf16 = text.encode("utf-16-le")
    require_boundary(len(utf16) // 2, unit_offset, "UTF-16 unit")
    try:
        prefix = utf16[: unit_offset * 2].decode("utf-16-le")
    except UnicodeDecodeError as error:
        raise ValueError("UTF-16 unit offset is inside a scalar") from error
    return len(prefix.encode("utf-32-le")) // 4


def visual_to_scalar_map(paragraph: tuple[int, ...]) -> tuple[int, ...]:
    source_text = text_from_code_points(paragraph)
    visual_text = get_display(source_text, base_dir="L")
    visual_code_points = tuple(ord(character) for character in visual_text)
    if len(set(paragraph)) != len(paragraph):
        raise ValueError("fixture code points must be unique for UBA map derivation")
    return tuple(paragraph.index(code_point) for code_point in visual_code_points)


def inverse_map(visual_to_scalar: tuple[int, ...]) -> tuple[int, ...]:
    scalar_to_visual = [0] * len(visual_to_scalar)
    for visual_position, scalar_position in enumerate(visual_to_scalar):
        scalar_to_visual[scalar_position] = visual_position
    return tuple(scalar_to_visual)


def grapheme_index(boundaries: tuple[int, ...], scalar_offset: int) -> int:
    if scalar_offset not in boundaries:
        raise ValueError("scalar offset is inside a grapheme")
    return boundaries.index(scalar_offset)


def rejected_utf8_offsets(text: str, boundaries: tuple[int, ...]) -> tuple[int, ...]:
    rejected = []
    for byte_offset in range(len(text.encode("utf-8")) + 1):
        if byte_offset in boundaries:
            continue
        try:
            utf8_byte_to_scalar(text, byte_offset)
        except ValueError:
            rejected.append(byte_offset)
        else:
            raise AssertionError(f"accepted UTF-8 interior byte {byte_offset}")
    return tuple(rejected)


def rejected_utf16_offsets(text: str, boundaries: tuple[int, ...]) -> tuple[int, ...]:
    rejected = []
    for unit_offset in range(len(text.encode("utf-16-le")) // 2 + 1):
        if unit_offset in boundaries:
            continue
        try:
            utf16_unit_to_scalar(text, unit_offset)
        except ValueError:
            rejected.append(unit_offset)
        else:
            raise AssertionError(f"accepted UTF-16 interior unit {unit_offset}")
    return tuple(rejected)


for fixture in FIXTURES:
    text = text_from_code_points(fixture.code_points)
    visual_to_scalar = visual_to_scalar_map(fixture.code_points)
    scalar_to_visual = inverse_map(visual_to_scalar)
    assert visual_to_scalar == fixture.expected_visual_to_scalar
    assert inverse_map(scalar_to_visual) == visual_to_scalar
    print(f"fixture={fixture.name} repr={text!r}")
    print("scalar|utf8_byte|utf16_unit|code_point|visual_position")
    for scalar_offset, expected_utf8, expected_utf16 in zip(fixture.scalar_boundaries, fixture.utf8_bytes, fixture.utf16_units):
        assert scalar_to_utf8_byte(text, scalar_offset) == expected_utf8
        assert utf8_byte_to_scalar(text, expected_utf8) == scalar_offset
        assert scalar_to_utf16_unit(text, scalar_offset) == expected_utf16
        assert utf16_unit_to_scalar(text, expected_utf16) == scalar_offset
        visual_position = "boundary"
        code_point = "end"
        if scalar_offset < len(fixture.code_points):
            visual_position = scalar_to_visual[scalar_offset]
            assert visual_to_scalar[visual_position] == scalar_offset
            code_point = hex(fixture.code_points[scalar_offset])
        print(f"{scalar_offset}|{expected_utf8}|{expected_utf16}|{code_point}|{visual_position}")
    assert rejected_utf8_offsets(text, fixture.utf8_bytes) == fixture.rejected_utf8_bytes
    assert rejected_utf16_offsets(text, fixture.utf16_units) == fixture.rejected_utf16_units
    rejected_graphemes = tuple(
        scalar_offset
        for scalar_offset in fixture.scalar_boundaries
        if scalar_offset not in fixture.grapheme_boundaries
    )
    for scalar_offset in rejected_graphemes:
        try:
            grapheme_index(fixture.grapheme_boundaries, scalar_offset)
        except ValueError:
            pass
        else:
            raise AssertionError(f"accepted grapheme interior scalar {scalar_offset}")
    print(f"uba_visual_to_scalar={visual_to_scalar}")
    print(f"utf8_interior_rejected={fixture.rejected_utf8_bytes}")
    print(f"utf16_interior_rejected={fixture.rejected_utf16_units}")
    print(f"grapheme_interior_rejected={rejected_graphemes}")
    print()

print("result=hand-listed scalar-to-UTF-8 and scalar-to-UTF-16 pairs round-trip; UBA scalar maps round-trip; logical-index conversion is not tested because TextIndex::Logical has no specified representation")
```

A `nix-shell` Python package environment ran the corrected fixture with the following exact captured output:

```text
$ nix-shell -p python3Packages.python-bidi --run 'python3 /tmp/wf-epic-b/OXY-B003/round-3/atspi-scalar-fixtures-uba.py'
fixture=ASCII repr='abZ'
scalar|utf8_byte|utf16_unit|code_point|visual_position
0|0|0|0x61|0
1|1|1|0x62|1
2|2|2|0x5a|2
3|3|3|end|boundary
uba_visual_to_scalar=(0, 1, 2)
utf8_interior_rejected=()
utf16_interior_rejected=()
grapheme_interior_rejected=()

fixture=multibyte repr='A界😀'
scalar|utf8_byte|utf16_unit|code_point|visual_position
0|0|0|0x41|0
1|1|1|0x754c|1
2|4|2|0x1f600|2
3|8|4|end|boundary
uba_visual_to_scalar=(0, 1, 2)
utf8_interior_rejected=(2, 3, 5, 6, 7)
utf16_interior_rejected=(3,)
grapheme_interior_rejected=()

fixture=combining repr='éx'
scalar|utf8_byte|utf16_unit|code_point|visual_position
0|0|0|0x65|0
1|1|1|0x301|1
2|3|2|0x78|2
3|4|3|end|boundary
uba_visual_to_scalar=(0, 1, 2)
utf8_interior_rejected=(2,)
utf16_interior_rejected=()
grapheme_interior_rejected=(1,)

fixture=bidirectional repr='AאבB'
scalar|utf8_byte|utf16_unit|code_point|visual_position
0|0|0|0x41|0
1|1|1|0x5d0|2
2|3|2|0x5d1|1
3|5|3|0x42|3
4|6|4|end|boundary
uba_visual_to_scalar=(0, 2, 1, 3)
utf8_interior_rejected=(2, 4)
utf16_interior_rejected=()
grapheme_interior_rejected=()

result=hand-listed scalar-to-UTF-8 and scalar-to-UTF-16 pairs round-trip; UBA scalar maps round-trip; logical-index conversion is not tested because TextIndex::Logical has no specified representation
exit=0
```

### Minimal noncandidate AT-SPI probe

The supplied `nix shell` import command failed because that shell did not contain a `python3` executable. The failure is preserved rather than treated as an AT-SPI result:

```text
$ nix shell nixpkgs#at-spi2-core nixpkgs#python3Packages.pygobject3 -c python3 -c 'import gi; gi.require_version("Atspi", "2.0"); from gi.repository import Atspi; print(Atspi.get_version())'
error: unable to execute 'python3': No such file or directory
exit=1
```

A matching Python package environment imported AT-SPI and reported the host library version. This probe verifies import mechanics only; it does not create an accessibility bus or inspect an accessible object:

```text
$ nix-shell -p python314Packages.pygobject3 at-spi2-core --run 'python3 -c "import gi; gi.require_version(\"Atspi\", \"2.0\"); from gi.repository import Atspi; print(Atspi.get_version())"'
(major=2, minor=60, micro=6)
exit=0
```

The host can run `dbus-run-session`, but it has no `at-spi-bus-launcher` command and no session `org.a11y.Bus`. This stops the host probe before any AT-SPI text, caret, selection, or editable result is claimed:

```text
$ command -v dbus-run-session
/run/current-system/sw/bin/dbus-run-session
$ command -v at-spi-bus-launcher
at-spi-bus-launcher=absent
$ busctl --user list | grep -F org.a11y.Bus
org.a11y.Bus=absent
exit=0
```

### Recovery-probe pass rule

`CAP-REC-001` requires valid output within the applicable deadline and preserved framework state. The recovery flow says that application-runtime and component state remain live. For P6F and P6I, a successful recovery must therefore produce valid, correctly sized acknowledged output after a resize and preserve framework state for every fault.

The fixture passes only when all applicable PRD constraints below pass. A failed deadline, allocation cap, attempt cap, release bound, output check, or state-preservation check must return the structured terminal error specified by `CON-REC-006` and the recovery flow.

- `CON-REC-001`: From the later of the final resize event and resource availability to acknowledged correctly sized output, at most two destination-display refresh intervals.
- `CON-REC-002`: From an externally observed surface-loss event to acknowledged valid output, at most 250 ms.
- `CON-REC-003`: From the operating-system resume or display-topology event to acknowledged valid output, at most 500 ms.
- `CON-REC-004`: From the external recoverable device-loss event to acknowledged valid output, at most 2 seconds.
- `CON-REC-005`: Maximum recovery allocation relative to steady-state graphics allocation, at most 2x steady state.
- `CON-REC-006`: At most three recreation attempts from the first fault through success or terminal failure; if recovery fails, return a structured terminal error rather than make a fourth attempt.
- `CON-REC-007`: Release resources superseded during recovery within 500 ms after acknowledged recovery success or terminal failure.

### Source record

The report relies on the following fetched authoritative sources:

- [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/).
- [GNOME GTK 4.20 source index](https://download.gnome.org/sources/gtk/4.20/).
- [GTK 4.20.4 SHA-256 file](https://download.gnome.org/sources/gtk/4.20/gtk-4.20.4.sha256sum).
- [Immutable GTK 4.20.4 `gtkenums.h` source](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.20.4/gtk/gtkenums.h).
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
- [AT-SPI 2.60.6 release notes](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/NEWS).
- [AT-SPI 2.60.6 `Text.xml`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/Text.xml).
- [AT-SPI 2.60.6 `EditableText.xml`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/EditableText.xml).
- [AT-SPI `Text.get_text` API](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/method.Text.get_text.html).
- [AT-SPI `Text.get_character_at_offset` API](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/method.Text.get_character_at_offset.html).
- [AT-SPI `EditableText` API](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.EditableText.html).
- [Version-1 Wayland presentation-time protocol XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml).
- [Version-2 Wayland presentation-time protocol XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/8cdb39103247fdde5764fc35b1b5cf60698db3e5/stable/presentation-time/presentation-time.xml).
- [Pinned Wayland presentation-time protocol XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/presentation-time/presentation-time.xml).
- [Pinned xdg-shell protocol XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/xdg-shell/xdg-shell.xml).
- [Pinned viewporter protocol XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/viewporter/viewporter.xml).
- [Pinned fractional-scale-v1 protocol XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/staging/fractional-scale/fractional-scale-v1.xml).
- [Pinned text-input-v3 protocol XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/unstable/text-input/text-input-unstable-v3.xml).
- [Pinned Wayland core protocol XML](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml).
- [Pinned Vulkan registry XML](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/20a9e5892e2aab7b9776b16a238b10fc8133090a/xml/vk.xml).
- [Linux v6.18 DRM tracepoint fallback source](https://raw.githubusercontent.com/torvalds/linux/v6.18/drivers/gpu/drm/drm_trace.h).
- [Upstream Linux stable v6.18.44 DRM tracepoint definition](https://raw.githubusercontent.com/gregkh/linux/v6.18.44/drivers/gpu/drm/drm_trace.h).
- [Upstream Linux stable v6.18.44 DRM vblank call site](https://raw.githubusercontent.com/gregkh/linux/v6.18.44/drivers/gpu/drm/drm_vblank.c).
- [Upstream Linux stable v6.18.44 CRTC-index helper](https://raw.githubusercontent.com/gregkh/linux/v6.18.44/include/drm/drm_crtc.h).
- [Linux kernel event-tracing documentation](https://docs.kernel.org/trace/events.html).
- [Linux kernel ftrace documentation](https://docs.kernel.org/trace/ftrace.html).
- [Linux DRM UAPI documentation](https://docs.kernel.org/gpu/drm-uapi.html).

The pinned source probe produced these immutable content digests:

```text
presentation-time-v1 sha256=91e5e14481a13717fef8403203a2eaa052c85fd853c1c440ba081effa7178d12
presentation-time-v2 sha256=dffac93bcb2bb1d8c385e72b8a8c2c0d4d79a336866322f3ba886dce2b27b1e2
presentation-time-pinned sha256=dffac93bcb2bb1d8c385e72b8a8c2c0d4d79a336866322f3ba886dce2b27b1e2
core-wayland-pinned sha256=7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610
xdg-shell-pinned sha256=f241ab95c262eb45af6507bf0178b8dcd88d5e1cc8fc6ec4a9944d9e5b15ac98
viewporter-pinned sha256=dcb12279a03746301fe490aaed4b38a403485a925abfce2ccfceb644e104fe71
fractional-scale-v1-pinned sha256=5941de5d28f427ecdadddc8623a6f6af0a30b0ab4726847236ba7a7652b81316
text-input-unstable-v3-pinned sha256=49048087a67011a8840bca889cd2b0ba374382be1ed54ec98adf7837fdca1982
atspi-text-2.60.6 sha256=5c2d5049d2e427d630ca1ae288d0abe321f39c683336cb8a1373f41c4414d614
atspi-editable-text-2.60.6 sha256=2ea1b94822f19b0b00c80b918b89833cfb67d1eeef99d69b8421d0e6f40920ff
gtk-4.20.4-gtkenums-h sha256=c2ef75dc175e7d8b6a28c1ace0e45898a0f2f4b14454b980fd310e545eb485c9
vulkan-registry-pinned sha256=3ff4984b841932e04eebeb4ce2a6613ebd37c00ffb2e96549785b2c5d7da9e1d
linux-v6.18-drm-trace-h sha256=0b4779e5ccc62e11e2854a89797cb39f97ef21030c114d05e0a2782e670b54f6
linux-v6.18.44-drm-trace-h sha256=0b4779e5ccc62e11e2854a89797cb39f97ef21030c114d05e0a2782e670b54f6
linux-v6.18.44-drm-vblank-c sha256=c6edb115c1457be17d9a9aa44972694c67ffbb6b331cd858f21a51f39895868e
linux-v6.18.44-drm-crtc-h sha256=5256a74b6b1d614bd8410c01c0c9c654d38355af660ba4b6928ea96ad183ac27
gtk-4.20.4-official-sum=a21f825bd44afc4dd99ba4eea8ff57c8f2e51085cb402a68ed4cbb35299826a4
```

The round-3 source-record recheck fetched the cited immutable URLs, recomputed the Vulkan digest, and verified every displayed digest has 64 hexadecimal characters:

```text
$ sha256sum /tmp/wf-epic-b/OXY-B003/round-3/sources/{wayland.xml,EditableText.xml,Text.xml,vk.xml}
7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610  /tmp/wf-epic-b/OXY-B003/round-3/sources/wayland.xml
2ea1b94822f19b0b00c80b918b89833cfb67d1eeef99d69b8421d0e6f40920ff  /tmp/wf-epic-b/OXY-B003/round-3/sources/EditableText.xml
5c2d5049d2e427d630ca1ae288d0abe321f39c683336cb8a1373f41c4414d614  /tmp/wf-epic-b/OXY-B003/round-3/sources/Text.xml
3ff4984b841932e04eebeb4ce2a6613ebd37c00ffb2e96549785b2c5d7da9e1d  /tmp/wf-epic-b/OXY-B003/round-3/sources/vk.xml
SOURCE_RECORD_CHECK
ok /tmp/wf-epic-b/OXY-B003/round-3/sources/wayland.xml
ok /tmp/wf-epic-b/OXY-B003/round-3/sources/EditableText.xml
ok /tmp/wf-epic-b/OXY-B003/round-3/sources/Text.xml
ok /tmp/wf-epic-b/OXY-B003/round-3/sources/vk.xml
exit=0
```

Round 5 validated every `sha256=` occurrence in the source-record digest block. The validator counts all occurrences, extracts complete non-whitespace and non-pipe-delimited tokens, validates every token against exactly 64 lowercase hexadecimal characters, compares the occurrence and token counts, and exits nonzero on any mismatch:

````text
$ report=.constitution/spikes/SPK-B003.md
$ digest_block=$(mktemp /tmp/wf-epic-b/OXY-B003/round-5/source-record.XXXXXX)
$ tokens=$(mktemp /tmp/wf-epic-b/OXY-B003/round-5/source-record-tokens.XXXXXX)
$ sed -n '/^presentation-time-v1 sha256=/,/^```/p' "$report" > "$digest_block"
$ occurrence_count=$(grep -oF 'sha256=' "$digest_block" | wc -l | tr -d ' ')
$ grep -oE 'sha256=[^[:space:]|]+' "$digest_block" > "$tokens" || true
$ token_count=$(wc -l < "$tokens" | tr -d ' ')
$ invalid_count=$(grep -cvE '^sha256=[0-9a-f]{64}$' "$tokens" || true)
$ printf 'all_sha256_occurrences=%s complete_tokens=%s invalid_tokens=%s\n' "$occurrence_count" "$token_count" "$invalid_count"
all_sha256_occurrences=16 complete_tokens=16 invalid_tokens=0
$ if [ "$occurrence_count" -eq "$token_count" ] && [ "$invalid_count" -eq 0 ]; then status=0; else status=1; fi
$ rm -f "$digest_block" "$tokens"
$ printf 'exit=%s\n' "$status"
exit=0
$ exit "$status"
````

## Options and trade-offs

- **Option A:** Freeze the selected Ubuntu compositor session, package manifest, and protocol registry only after P1 records compositor/version evidence and the visible-surface transcript. This is required for a reference baseline, but it is not complete in this spike.
- **Option B:** Use a prospective Linux DRM `drm:drm_vblank_event` trace as the opportunity-meter design. P4 must establish Ubuntu kernel package and source or patch identity, live tracepoint schema and call-site semantics, trace access, pipe-index-to-CRTC-ID-to-output association, a `mono` trace clock, a justified calibration uncertainty budget, and no candidate callback or IPC path before it becomes a meter.
- **Option C:** Keep candidate behavior and environment-dependent rows as gating KUs. This prevents the reference distribution label, protocol advertisement, `GdkFrameClock`, or per-commit feedback from becoming unearned qualification evidence.

## Recommendation

- **Chosen option:** Use a mix of A, B, and C. Freeze the source-level core, shell, scale, text-input, clipboard, presentation, GTK, and AT-SPI floors from cited upstream sources. Use Orca and AT-SPI 2.60.6 with documented Unicode-scalar offsets for the common accessibility baseline. Require the Option B DRM trace design for P4 only after its Ubuntu kernel identity, live format, and call-site semantics are evidenced, and retain Option C for every unproven reference-session and candidate-specific row, including the calibration tolerance.
- **Why it fits:** The source floors contain every derived P0 operation, including creation, cursor, keyboard, touch, output, candidate geometry, clipboard source, offer, selection, and the client-issued release and destroy operations needed for per-view lifecycle and teardown. Presentation version 1 has acknowledgement and output-association operations. Version 2 only changes the variable-refresh `refresh` obligation, which the harness does not consume. Retaining KUs for server advertisement, behavior, and logical-index representation prevents source facts from becoming compositor or candidate claims. The DRM trace's independence from candidate callback streams remains unresolved until P4 proves it, along with Ubuntu kernel identity, live schema and call-site semantics, trace access, pipe-to-output attribution, and a justified clock-calibration and causal-matching uncertainty budget. `CON-FRM-001` remains the separately applied measured interval-error gate.
- **Rejected options:** Reject a nominal refresh-rate timer, a harness-owned `wl_surface.frame` callback as an independent meter, `wp_presentation` feedback as an opportunity source, a protocol-global list as compositor behavior, an unspecified assistive technology, a scalar-to-logical equivalence assumption, a global IME index unit for every operation, and a candidate map inferred from GTK documentation.
- **Sensitive-field rule:** Set `GtkInputPurpose` to `PASSWORD` or `PIN` as applicable and set `GtkInputHints.PRIVATE`. Continue to provide only protocol-required redacted surrounding context and never emit raw text to diagnostics. GTK describes the hint as a request, not a privacy guarantee; P2 and P3 must verify the redaction path.

### Spec edits required

Stage 3 can make the following exact edits without changing product capabilities or architecture boundaries:

- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.protocols`: replace the array with this schema-valid JSON value. Each `kk` entry has a non-null version and evidence; P1 retains server advertisement and behavior as a separate gate.

```json
[
  {
    "name": "GTK",
    "version": "4.20.4",
    "status": "kk",
    "evidence": [
      {
        "path": "https://gitlab.gnome.org/GNOME/gtk/-/raw/4.20.4/gtk/gtkenums.h",
        "sha256": "c2ef75dc175e7d8b6a28c1ace0e45898a0f2f4b14454b980fd310e545eb485c9"
      }
    ]
  },
  {
    "name": "wl_compositor",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml",
        "sha256": "7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610"
      }
    ]
  },
  {
    "name": "wl_surface",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml",
        "sha256": "7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610"
      }
    ]
  },
  {
    "name": "wl_callback",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml",
        "sha256": "7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610"
      }
    ]
  },
  {
    "name": "wl_seat",
    "version": "5",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml",
        "sha256": "7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610"
      }
    ]
  },
  {
    "name": "wl_pointer",
    "version": "5",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml",
        "sha256": "7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610"
      }
    ]
  },
  {
    "name": "wl_keyboard",
    "version": "4",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml",
        "sha256": "7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610"
      }
    ]
  },
  {
    "name": "wl_touch",
    "version": "3",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml",
        "sha256": "7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610"
      }
    ]
  },
  {
    "name": "wl_output",
    "version": "3",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml",
        "sha256": "7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610"
      }
    ]
  },
  {
    "name": "wl_data_device_manager",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml",
        "sha256": "7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610"
      }
    ]
  },
  {
    "name": "wl_data_device",
    "version": "2",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml",
        "sha256": "7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610"
      }
    ]
  },
  {
    "name": "wl_data_offer",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml",
        "sha256": "7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610"
      }
    ]
  },
  {
    "name": "wl_data_source",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml",
        "sha256": "7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610"
      }
    ]
  },
  {
    "name": "xdg_wm_base",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/xdg-shell/xdg-shell.xml",
        "sha256": "f241ab95c262eb45af6507bf0178b8dcd88d5e1cc8fc6ec4a9944d9e5b15ac98"
      }
    ]
  },
  {
    "name": "xdg_surface",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/xdg-shell/xdg-shell.xml",
        "sha256": "f241ab95c262eb45af6507bf0178b8dcd88d5e1cc8fc6ec4a9944d9e5b15ac98"
      }
    ]
  },
  {
    "name": "xdg_toplevel",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/xdg-shell/xdg-shell.xml",
        "sha256": "f241ab95c262eb45af6507bf0178b8dcd88d5e1cc8fc6ec4a9944d9e5b15ac98"
      }
    ]
  },
  {
    "name": "wp_viewporter",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/viewporter/viewporter.xml",
        "sha256": "dcb12279a03746301fe490aaed4b38a403485a925abfce2ccfceb644e104fe71"
      }
    ]
  },
  {
    "name": "wp_viewport",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/viewporter/viewporter.xml",
        "sha256": "dcb12279a03746301fe490aaed4b38a403485a925abfce2ccfceb644e104fe71"
      }
    ]
  },
  {
    "name": "wp_fractional_scale_manager_v1",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/staging/fractional-scale/fractional-scale-v1.xml",
        "sha256": "5941de5d28f427ecdadddc8623a6f6af0a30b0ab4726847236ba7a7652b81316"
      }
    ]
  },
  {
    "name": "wp_fractional_scale_v1",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/staging/fractional-scale/fractional-scale-v1.xml",
        "sha256": "5941de5d28f427ecdadddc8623a6f6af0a30b0ab4726847236ba7a7652b81316"
      }
    ]
  },
  {
    "name": "zwp_text_input_manager_v3",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/unstable/text-input/text-input-unstable-v3.xml",
        "sha256": "49048087a67011a8840bca889cd2b0ba374382be1ed54ec98adf7837fdca1982"
      }
    ]
  },
  {
    "name": "zwp_text_input_v3",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/unstable/text-input/text-input-unstable-v3.xml",
        "sha256": "49048087a67011a8840bca889cd2b0ba374382be1ed54ec98adf7837fdca1982"
      }
    ]
  },
  {
    "name": "wp_presentation",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml",
        "sha256": "91e5e14481a13717fef8403203a2eaa052c85fd853c1c440ba081effa7178d12"
      }
    ]
  },
  {
    "name": "wp_presentation_feedback",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml",
        "sha256": "91e5e14481a13717fef8403203a2eaa052c85fd853c1c440ba081effa7178d12"
      }
    ]
  },
  {
    "name": "AT-SPI",
    "version": "2.60.6",
    "status": "kk",
    "evidence": [
      {
        "path": "https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/Text.xml",
        "sha256": "5c2d5049d2e427d630ca1ae288d0abe321f39c683336cb8a1373f41c4414d614"
      },
      {
        "path": "https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/EditableText.xml",
        "sha256": "2ea1b94822f19b0b00c80b918b89833cfb67d1eeef99d69b8421d0e6f40920ff"
      }
    ]
  }
]
```

- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.minimumVersion`: replace the value with `{"status":"ku-gating","value":null,"evidence":[]}`. Do not add `openQuestions` to this object because its schema has `additionalProperties: false`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.openQuestions`: replace the array with this sibling value of `minimumVersion`:

```json
[
  "minimum compositor and protocol versions",
  "complete IME mapping",
  "complete accessibility maps for both candidates",
  "independent presentation-opportunity source",
  "injectable recovery mechanisms",
  "immutable evidence for every status-bearing platform claim",
  "P1 must prove the selected Ubuntu session advertises every required global, creates every required non-global interface at its frozen floor, and emits every member of the mechanically regenerated P1 transcript checklist derived from the preserved Wayland floor derivation.",
  "P3B must freeze the TextIndex::Logical representation and establish scalar-to-logical pairs bound to an immutable TextLayoutId.",
  "P3 must lock at-spi2-core 2.60.6 or a separately reviewed replacement and establish scalar text, caret, selection, and EditableText-operation behavior.",
  "P4 must record the Ubuntu kernel image package identity, source or patch identity, live drm_vblank_event format and SHA-256, and source evidence for matching tracepoint schema and pipe call-site semantics; then establish trace access, pipe-index-to-CRTC-object-ID-to-connector attribution, unambiguous surface-output pairing, a mono trace clock, a justified clock-calibration and causal-matching uncertainty budget, and trace independence from each candidate. CON-FRM-001's 10% rule applies only to the measured interval-error result."
]
```

- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.ime.numericNegotiation`: replace the value with `"Use the writable Gtk.InputPurpose and Gtk.InputHints properties for each focus generation; no project-defined numeric handshake exists. Surrounding cursor and anchor positions use UTF-8 bytes. P2 must establish every other GtkIMContext operation unit."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.interactiveOpportunitySource`: replace the value with `"GdkFrameClock is a host wakeup only; each allocation must prove output-associated display-synchronized scheduling in P4."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.independentMeterSource`: replace the value with `"Prospective only: Linux DRM drm_vblank_event captured by trace-cmd with the mono trace clock. P4 must record the Ubuntu kernel image package identity, source or patch identity, live tracepoint format and SHA-256, and source evidence that its schema and call site establish pipe semantics. It must map each established pipe index through drmModeGetResources to a UAPI CRTC object ID and active connector, prove an unambiguous surface-output pairing and callback or IPC independence, and preserve a clock-calibration and causal-matching uncertainty budget. No clock-calibration acceptance tolerance is frozen. Apply CON-FRM-001's 10% limit only to the measured interval-error result after meter and matcher qualification."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.presentationFeedback`: replace the value with `"wp_presentation v1 feedback for per-commit acknowledgement and main-output association only; never an independent presentation-opportunity meter."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.perDisplayAssociation`: replace the value with `"Track each wl_surface enter/leave output set and begin a display epoch on every set change. Use wp_presentation_feedback.sync_output only to label a submitted frame's main output."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.accessibilityMaps`, `recoveryBaseline`, `allocations.focused`, and `allocations.integrated`: retain every `"ku-gating"` status and `null` path/digest until P3F, P3I, P5F, P5I, P6F, and P6I produce the named immutable artifacts.
- `.constitution/tech-spec/stack.md` -> `Platform qualification pins` -> `Wayland` row: replace `"minimum compositor and protocol versions are gating KUs"` with `"the Ubuntu compositor/session package manifest and selected-session advertisement of the frozen Wayland floors remain gating KUs; P1 must record package versions, manifest digest, registry, and a visible-surface transcript covering all 99 members of the mechanically regenerated P0 floor-derivation checklist, including client-issued release and destroy operations"`.
- `.constitution/tech-spec/contracts/qualification-lock.json` -> `preImplementationKnownUnknowns` and `gatingKnownUnknowns`: add `"wayland-ubuntu-compositor-session-package-lock"`, `"wayland-frozen-protocol-reference-session-transcript"`, `"wayland-ime-operation-unit-transcript"`, `"wayland-atspi-scalar-logical-representation"`, `"wayland-atspi-text-caret-selection-editable-transcript"`, `"wayland-orca-atspi-maps-for-both-allocations"`, `"wayland-drm-vblank-kernel-identity-live-schema-callsite"`, `"wayland-drm-vblank-calibration-uncertainty-budget"`, `"wayland-service-routing-for-both-allocations"`, and `"wayland-recovery-injection-for-both-allocations"`.
- `.constitution/tech-spec/adrs/ADR-0005-platform-hosts.md` -> `Consequences`: add `"Wayland qualification freezes source-level core, shell, scale, text-input, clipboard, and wp_presentation floors, including all client-issued P0 teardown operations. P1 must prove the selected session advertises them and records the mechanically derived 99-member complete P0-operation transcript. wp_presentation v1 supplies per-commit acknowledgement and output association only, not the independent presentation-opportunity meter. P4 evaluates a Linux DRM drm_vblank_event trace only after Ubuntu kernel package and source or patch identity, live-format and call-site-semantic evidence, access, pipe-index-to-CRTC-object-ID-to-output attribution, a justified trace_marker-bracketed CLOCK_MONOTONIC uncertainty budget, and callback or IPC independence pass. CON-FRM-001's 10% interval-error limit is applied only to qualified measured matching results."`.

## Downstream impact

- **ADRs to write or update:** Stage 3 updates `ADR-0005-platform-hosts.md` with the `wp_presentation` boundary. `ADR-0006-execution-domains.md` requires no change because the report does not alter its queue or ownership boundary.
- **Tickets unblocked in `tasks/active/`:** `OXY-D001` can consume the documented protocol and conversion mechanics, but it remains blocked from qualification measurements by P1 through P6.
- **Tickets to add or split:** Add P1 through P6 as bounded Wayland evidence tasks if the Stage 4 plan does not already schedule equivalent probes.
- **Remaining gates:** The 12 KU (gating) rows retain the Wayland environment as `ku-gating`. Neither allocation is eligible for scoring until they close.
