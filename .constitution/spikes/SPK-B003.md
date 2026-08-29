# Spike report: OXY-B003 Wayland qualification baseline

## Time box

- **Budget:** 1 focused day.
- **Initial report clock start / stop:** 2026-08-28T17:34:55Z / 2026-08-28T17:45:57Z.
- **Round-4 correction clock start / stop:** 2026-08-28T17:54:56Z / 2026-08-28T18:06:35Z.
- **Round-5 correction clock start / stop:** 2026-08-28T18:14:44Z / 2026-08-28T18:23:58Z.
- **Round-6 correction clock start / stop:** 2026-08-28T18:31:37Z / 2026-08-28T18:42:14Z.
- **Round-7 correction clock start / stop:** 2026-08-28T19:08:02Z / 2026-08-28T19:17:48Z.
- **Round-8 correction clock start / stop:** 2026-08-28T19:41:57Z / 2026-08-28T19:49:17Z.
- **Round-9 correction clock start / stop:** 2026-08-28T20:22:41Z / 2026-08-28T20:32:29Z.
- **Round-10 correction clock start / stop:** 2026-08-28T20:55:09Z / 2026-08-28T20:57:09Z.
- **Round-11 correction clock start / stop:** 2026-08-28T21:19:47Z / 2026-08-28T21:24:47Z.
- **Round-12 correction clock start / stop:** 2026-08-28T23:55:08Z / 2026-08-28T23:56:23Z.
- **Round-13 correction clock start / stop:** 2026-08-29T00:29:18Z / 2026-08-29T00:36:58Z.

## Question

- **Decision this spike produces:** Freeze source-level Wayland core, shell, scale, text-input, clipboard, and presentation protocol floors from the pinned XML. Keep Ubuntu reference-session advertisement and behavior as a gating KU until P1 records the selected session's package lock, registry, and complete P0-operation transcript. Treat the `gtk4` crate `v4_20` feature in `stack.md` as the separate GTK API-binding ceiling, and use the audited Ubuntu `libgtk-4-1` `4.22.2+ds-1ubuntu1` package identity for both the Wayland and X11 reference inputs. Retain the Wayland session package-manifest and candidate-behavior gates. Freeze AT-SPI 2.60.6 source interface definitions while using the audited Ubuntu `at-spi2-core` `2.60.0-1` package identity for both Linux reference environments. Use writable `GtkIMContext` input-purpose and input-hints properties, and convert documented UTF-8 byte cursor positions explicitly. Use Orca with AT-SPI 2 as the Linux assistive-technology test client. Freeze documented AT-SPI character offsets as Unicode scalar boundaries, but retain scalar-to-`TextIndex::Logical` conversion, text, caret, selection, and editable-operation behavior as gating KUs. Select the Linux DRM `drm:drm_vblank_event` tracepoint as P4's prospective trace candidate, while retaining Ubuntu kernel identity, live schema and call-site semantics, independence, source access, output attribution, and clock-calibration and causal-matching tolerance as gating KUs. Retain the complete-map, routing, and recovery gates until their bounded reference probes pass.

Table 1 answers each Wayland baseline question. KK is a verified fact. KU (gating) is a named unresolved gate. No row is not applicable.

Table 1. Wayland baseline decisions

| Row | Answer and evidence | Status | Next bounded probe |
| :-- | :-- | :-- | :-- |
| Reference compositor, session, and package lock | [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/) establish Ubuntu 26.04 LTS, but the fetched release-note content names no compositor, session, package version, or package-lock digest. The non-reference host is NixOS 26.05 with Hyprland 0.55.4, so its registry cannot establish Ubuntu compositor behavior. | KU (gating) | P1: On the selected Ubuntu 26.04 x86-64 Wayland session, record `gnome-shell --version` or the selected compositor's version command, `dpkg-query -W` for the compositor, `gtk4`, `wayland-protocols`, and `at-spi2-core`, the package-manifest SHA-256, a filtered `wayland-info` registry, and the mechanically derived 97-member deterministic P1 checklist below, plus the four retained event gates. Run a 120-frame visible-surface probe with `WAYLAND_DEBUG=client` that binds every required global, creates every required non-global object, and emits every checklist member. The script parses the preserved floor derivation, so P1 must regenerate the checklist rather than maintain a manual operation list. The fixture uses `wl_pointer.set_cursor`, not `cursor-shape-v1`. Expected output: one named compositor version, one package-lock digest, negotiated versions for every required interface, the generated checklist, and a session-specific transcript covering every checklist member. |
| Wayland core object protocol floors | The pinned [Wayland core XML](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml) establishes these operation-derived floors: `wl_compositor` 1, `wl_surface` 1, `wl_callback` 1, `wl_seat` 5, `wl_pointer` 5, `wl_keyboard` 4, `wl_touch` 3, `wl_output` 3, `wl_data_device_manager` 2, `wl_data_device` 2, `wl_data_offer` 1, and `wl_data_source` 1. The preserved XML parser output names every required request and event. The P0 completeness derivation includes per-view and protocol-object teardown: `wl_surface.destroy`; `wl_seat.release`; `wl_pointer.release`; `wl_keyboard.release`; `wl_touch.release`; `wl_output.release`; and `wl_data_device.release`, as well as cursor, keyboard keymap and repeat, touch, output geometry and scale, clipboard selection and offers, and text-input candidate geometry. `wl_seat.release` raises its floor to 5; `wl_touch.release` and `wl_output.release` raise their floors to 3; and `wl_data_device.release` raises its object floor to 2. `wl_data_device_manager.get_data_device` creates `wl_data_device` through an interface-typed `new_id`, so the child inherits the manager's bound version and the factory-propagation pass raises the manager floor from its local 1 to 2. `wl_pointer` 5 still supplies `axis_source`, `axis_stop`, and `frame`; `wl_keyboard` 4 still supplies `repeat_info`. | KK | Not required for the source-level floors. P1 must bind each required global and create every listed non-global object at the listed floor. |
| Wayland shell, scale, IME, and presentation protocol floors | The pinned [xdg-shell](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/xdg-shell/xdg-shell.xml), [viewporter](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/viewporter/viewporter.xml), [fractional-scale](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/staging/fractional-scale/fractional-scale-v1.xml), [text-input-v3](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/unstable/text-input/text-input-unstable-v3.xml), and version-1 [presentation-time](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml) XML establish floor 1 for `xdg_wm_base`, `xdg_surface`, `xdg_toplevel`, `wp_viewporter`, `wp_viewport`, `wp_fractional_scale_manager_v1`, `wp_fractional_scale_v1`, `zwp_text_input_manager_v3`, `zwp_text_input_v3`, `wp_presentation`, and `wp_presentation_feedback`. The required operations cover toplevel configure acknowledgement, fractional-scale destination sizing, IME surrounding text, candidate geometry through `zwp_text_input_v3.set_cursor_rectangle`, commits, and per-commit `feedback`, `sync_output`, `presented`, or `discarded`; they also cover `xdg_wm_base.destroy`, `xdg_surface.destroy`, `xdg_toplevel.destroy`, `wp_viewporter.destroy`, `wp_viewport.destroy`, `wp_fractional_scale_manager_v1.destroy`, `wp_fractional_scale_v1.destroy`, `zwp_text_input_manager_v3.destroy`, `zwp_text_input_v3.disable`, `zwp_text_input_v3.destroy`, and `wp_presentation.destroy`. Version 2 changes only the variable-refresh `refresh` contract, which the harness does not consume. | KK | Not required for the source-level floors. P1 must bind each required manager global, create its listed non-global objects, and verify the `wp_presentation` transcript. |
| GTK 4.20.4 source API-binding ceiling | The official [GTK 4.20 source index](https://download.gnome.org/sources/gtk/4.20/) publishes GTK 4.20.4. The immutable [GTK 4.20.4 `gtkenums.h`](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.20.4/gtk/gtkenums.h) source defines `GTK_INPUT_HINT_PRIVATE` and describes it as a request not to update personalized data. The preserved source SHA-256 is `c2ef75dc175e7d8b6a28c1ace0e45898a0f2f4b14454b980fd310e545eb485c9`. This source fact supports the `gtk4` crate `v4_20` binding ceiling in `stack.md`; it is not the Ubuntu reference-package identity. | KK | Not required for the documented API-binding ceiling. P1 must lock the Ubuntu package that supplies the Wayland session. |
| GTK reference package and Wayland session lock | [SPK-B004's Ubuntu source-package audit](SPK-B004.md#ubuntu-source-package-audit) establishes the shared Ubuntu reference package identity as `libgtk-4-1` `4.22.2+ds-1ubuntu1`, with source-descriptor checksums and a patch audit. The official [Ubuntu package page](https://packages.ubuntu.com/resolute/libgtk-4-1) identifies the same binary package and version. That identity applies to both Linux sessions; the Wayland session still has no installed-package manifest digest or backend capture. The `v4_20` crate feature is a separate API-binding ceiling and does not require the reference package to be 4.20.4. | KU (gating) | P1: On the selected Ubuntu Wayland session, record `dpkg-query -W libgtk-4-1`, the package origin, and the immutable package-manifest digest. Accept this gate only when the installed package is `libgtk-4-1` `4.22.2+ds-1ubuntu1` and the manifest binds that package to the selected session. Expected output: package version, origin, manifest digest, and session identifier. |
| `GtkIMContext` surrounding text and input-purpose mechanism | [`set_surrounding`](https://docs.gtk.org/gtk4/method.IMContext.set_surrounding.html) takes UTF-8 text and a byte index for the cursor. [`input-purpose`](https://docs.gtk.org/gtk4/property.IMContext.input-purpose.html) and [`input-hints`](https://docs.gtk.org/gtk4/property.IMContext.input-hints.html) are writable properties. [`GtkInputPurpose`](https://docs.gtk.org/gtk4/enum.InputPurpose.html) supplies typed purpose values, including `PASSWORD` and `PIN`; [`GtkInputHints.PRIVATE`](https://docs.gtk.org/gtk4/flags.InputHints.html) requests that an input method not update personalized data. These are properties, not a compositor numeric negotiation. | KK | Not required. P2 verifies the selected input method's behavior rather than the documented interface shape. |
| Complete IME transcript and non-cursor operation units | GTK documents [`delete-surrounding`](https://docs.gtk.org/gtk4/signal.IMContext.delete-surrounding.html) arguments as character offsets and counts, but it does not state the scalar, grapheme, or another unit in the fetched API page. No selected Ubuntu IM module or candidate transcript exists. The report therefore does not infer a unit for deletion, preedit cursor position, or replacement behavior. | KU (gating) | P2: On the P1 session, use an instrumented noncandidate GTK 4.22.2 text widget from the locked `libgtk-4-1` `4.22.2+ds-1ubuntu1` package and the ASCII, multibyte, combining, bidirectional, CJK-composition, replacement, candidate-geometry, and secure-field corpus. Log every `preedit-*`, `commit`, `retrieve-surrounding`, `delete-surrounding`, `focus-*`, and `reset` callback with typed indices. Expected output: a transcript that identifies every operation's unit and round trips each valid boundary. |
| Linux assistive-technology selection | Select [Orca](https://help.gnome.org/users/orca/stable/) as the required screen-reader test client and AT-SPI 2 as its inspection and action transport. [GNOME's AT-SPI development documentation](https://gnome.pages.gitlab.gnome.org/at-spi2-core/devel-docs/atspi-python-stack.html) states that Orca builds a view of an application's accessible-object tree through `libatspi` and `pyatspi2`. | KK | Not required. P3 establishes the Ubuntu package lock and candidate behavior. |
| AT-SPI API floor | The official [at-spi2-core 2.60.6 release notes](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/NEWS) identify release 2.60.6. The immutable [AT-SPI 2.60.6 `Text.xml`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/Text.xml) defines `CharacterCount`, `GetText`, `SetCaretOffset`, and selections. It does not define editable text. The immutable [AT-SPI 2.60.6 `EditableText.xml`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/EditableText.xml) defines `SetTextContents`, `InsertText`, `CopyText`, `CutText`, `DeleteText`, and `PasteText`. Freeze 2.60.6 as the AT-SPI source API floor. That source floor is not the Ubuntu package identity: [SPK-B004's package audit](SPK-B004.md#ubuntu-source-package-audit) establishes `at-spi2-core` `2.60.0-1` as the shared package identity for both Linux reference environments. The preserved `Text.xml` SHA-256 is `5c2d5049d2e427d630ca1ae288d0abe321f39c683336cb8a1373f41c4414d614`; the preserved `EditableText.xml` SHA-256 is `2ea1b94822f19b0b00c80b918b89833cfb67d1eeef99d69b8421d0e6f40920ff`. | KK | Not required for the source API floor. P3 must lock the Ubuntu package and run the behavior transcript. |
| AT-SPI documented text-offset unit | The normative [AT-SPI 2.60.6 `Text.xml`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/Text.xml) defines `CharacterCount` as a number of characters that can differ from fetched UTF-8 byte count. It defines `GetText` end offsets as the first character past the range, while the UTF-8 result bytes can exceed those offsets. It also states that `GetCharacterAtOffset` returns "the UCS-4 unicode code point of the given character." The [AT-SPI 2.60.6 `EditableText.xml`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/EditableText.xml) defines edit positions as character offsets that can differ from UTF-8 byte offsets. Therefore the documented AT-SPI text, caret, selection, and editable-position unit is a Unicode scalar boundary, not a UTF-8 byte, UTF-16 unit, or grapheme boundary. The independent conversion fixture verifies scalar, UTF-8, UTF-16, and grapheme-boundary mechanics, not AT-SPI behavior or `TextIndex::Logical` conversion. | KK | Not required for the documented unit or scalar conversion mechanics. The next rows retain the representation and behavior gates. |
| AT-SPI scalar-to-`TextIndex::Logical` conversion | [`ADR-0007`](../tech-spec/adrs/ADR-0007-text-indexing.md) and the preserved contract probe establish that `TextIndex::Logical(u32)` is a logical text position within an immutable layout generation. Neither source defines its representation or a scalar-to-logical mapping. The scalar fixture therefore makes no scalar-to-logical claim. | KU (gating) | P3B: Before candidate geometry qualification, freeze the `TextIndex::Logical` representation in the public contract and add four hand-listed scalar-to-logical and logical-to-scalar pair tables for ASCII, multibyte, combining, and bidirectional layouts. Bind each pair to one `TextLayoutId` and assert rejection after its generation changes. Expected output: the adopted representation, four bidirectional pair tables, and stale-generation failures. |
| AT-SPI text, caret, selection, and editable behavior | The host has no `org.a11y.Bus`, and the fixture makes no AT-SPI calls. The AT-SPI source establishes the unit, not that a selected GTK exporter or either candidate applies it consistently to `GetText`, `CaretOffset`, selections, `SetCaretOffset`, and editable operations on the combining fixture. | KU (gating) | P3: On the P1 Ubuntu session, start a headless accessibility bus with `dbus-run-session` and `at-spi-bus-launcher`, then use a noncandidate GTK text widget and `pyatspi2` to record `CharacterCount`, `GetText`, `CaretOffset`, selection bounds, `SetCaretOffset`, and editable-operation results for every fixture. Expected output: for `e` plus combining acute plus `x`, `CharacterCount=3`; `GetCharacterAtOffset(1)` and `GetCharacterAtOffset(2)` return the distinct combining-mark and `x` code points; `GetText(0,1)` and `GetText(1,2)` distinguish the first two scalar ranges; and caret, selection, and editable operations round trip offsets 1 and 2. After P3B freezes the logical representation, the typed conversion fixture must assert the approved scalar-to-logical pairs and reject UTF-8, UTF-16, grapheme-interior, and stale-generation positions. |
| Focused allocation accessibility map | [GTK defines its own accessible object hierarchy](https://docs.gtk.org/gtk4/iface.Accessible.html) with role, state, property, and relation attributes and a platform accessibility context. No focused candidate source identity, exported tree, forward map, reverse action map, artifact path, or digest exists. | KU (gating) | P3F: After the focused source identity is locked, launch its two-view AT-SPI fixture under Orca and `pyatspi2`. Enumerate every required `accessibility-map.schema.json` forward key and reverse action, including Unicode-scalar text payloads, view generation, acknowledgement, stale target, and secure-field redaction. Expected output: one complete map JSON file and SHA-256. |
| Integrated allocation accessibility map | The [GTK accessibility interfaces](https://docs.gtk.org/gtk4/iface.Accessible.html) document a possible host mechanism, but they do not establish the pinned Flutter fork's inherited interfaces or its Oxyflut map. No fork commit, source tree, exported tree, forward map, reverse action map, artifact path, or digest exists. | KU (gating) | P3I: After the integrated fork and adapter commits are locked, run the same two-view Orca and `pyatspi2` fixture. First inventory inherited GTK and AT-SPI interfaces, then enumerate every forward key and reverse action. Expected output: the inventory, one complete map JSON file, and SHA-256. |
| Host scheduling and presentation feedback roles | [`GdkFrameClock`](https://docs.gtk.org/gdk4/class.FrameClock.html) tells an application when to update and repaint, but GTK states that it can use a simple timer instead of hardware vertical sync. The [version-1 presentation-time XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml) creates feedback for a submitted `wl_surface.commit` and emits one terminal presented or discarded result for that content update. Therefore `GdkFrameClock` is only a host wakeup mechanism until P4 qualifies it, and `wp_presentation` feedback is acknowledgement only, never an independent opportunity meter. | KK | Not required for the interface-role decision. P4 qualifies the meter and scheduling behavior. |
| Independent presentation-opportunity meter | No compositor evidence or host probe proves a qualified meter or the trace's independence from either candidate. The prospective trace is Linux DRM `drm:drm_vblank_event`, captured with `trace-cmd record -e drm:drm_vblank_event`. The fetched [upstream Linux stable v6.18.44 tracepoint definition](https://raw.githubusercontent.com/gregkh/linux/v6.18.44/drivers/gpu/drm/drm_trace.h) gives `TP_PROTO(int crtc, unsigned int seq, ktime_t time, bool high_prec)`. The fetched [upstream Linux stable v6.18.44 vblank call site](https://raw.githubusercontent.com/gregkh/linux/v6.18.44/drivers/gpu/drm/drm_vblank.c) calls `trace_drm_vblank_event(pipe, seq, now, high_prec)` after `drm_crtc_from_index(dev, pipe)`; its adjacent kernel comment defines `pipe` as the index of the CRTC where the event occurred. The [DRM UAPI CRTC-index documentation](https://docs.kernel.org/gpu/drm-uapi.html#crtc-index) states that an index and object ID differ and that `DRM_IOCTL_MODE_GETRESOURCES` returns CRTC IDs in index order. [ftrace documentation](https://docs.kernel.org/trace/ftrace.html) defines `mono` as `CLOCK_MONOTONIC` and documents `trace_marker`. `uname -r` establishes only this non-reference host's release string, not its source or patch identity, and establishes nothing about P4's Ubuntu kernel. The upstream sources therefore establish neither the live schema nor the call-site semantics of P4's kernel. Pipe-to-CRTC mapping remains KU (gating) until P4 preserves the Ubuntu package, source or patch identity, live format, and matching source evidence. The local source-selection probe stopped because the tracepoint and `trace-cmd` are absent, so it establishes no usable trace, output attribution, calibrated clock relation, or independence. | KU (gating) | P4: On the selected P1 Ubuntu 26.04 x86-64 session, first preserve `uname -r` and `dpkg -s linux-image-$(uname -r)`. Then preserve the package's source and patch identity by either running `apt-get source` for the source package selected by that installed image and recording its source version plus the `debian/patches` inventory, recording the installed `linux-source` package and its `debian/patches` inventory, or recording the Ubuntu kernel Git tag and commit resolved by the installed package. Capture `cat /sys/kernel/tracing/events/drm/drm_vblank_event/format` verbatim and its SHA-256. Compare its field schema with the identified kernel source, and preserve source excerpts showing both the tracepoint definition and the `trace_drm_vblank_event` call-site argument semantics. If the live format, source identity, or call-site semantics do not establish that the trace `crtc` field is the call site's pipe index, STOP P4 and retain this KU. Next verify `drm:drm_vblank_event` in `available_events`, permission to record it and write `trace_marker`, and the ability to set `trace_clock` to `mono`. If any check fails, STOP P4 and retain this KU. Before capture, use `drmModeGetResources`; for every established trace pipe `i`, record `resources->crtcs[i]` as the UAPI CRTC object ID, then record each active connector's CRTC object ID, connector identity, mode, and refresh interval. Pair that DRM inventory with contemporaneous `wl_surface.enter` or `leave` and `wl_output` logs; if a pairing is not unambiguous, STOP P4 and retain this KU. Capture a settled 10-second epoch with `trace-cmd record -e drm:drm_vblank_event` and record observer and candidate records on `CLOCK_MONOTONIC`. At epoch start and end, take `t_before = clock_gettime(CLOCK_MONOTONIC)`, write a uniquely identified `P4_CAL` `trace_marker`, then take `t_after = clock_gettime(CLOCK_MONOTONIC)`. Preserve the marker intervals and offset calculations, but do not apply a calibration pass/fail tolerance: it is KU (gating). Run P4C, a characterization-only fixed 10,000-marker calibration probe. For marker pair `i`, record ftrace marker timestamp `m_i`, `t_before_i`, and `t_after_i`; calculate `d_i = m_i - (t_before_i + t_after_i) / 2`, the offset estimate `d_bar = mean(d_i)`, and `SE(d_bar) = sample_sd(d_i) / sqrt(10,000)`. Record trace timestamp resolution `r_trace` as the smallest positive difference among monotonically increasing ftrace `mono` timestamps, and record clock timestamp resolution `r_clock` as the larger of `clock_getres(CLOCK_MONOTONIC)` and the smallest positive difference among monotonically increasing `clock_gettime` results. Let `w_max = max(t_after_i - t_before_i)`. Predeclare the characterization bound `U_95 = 1.96 * SE(d_bar) + r_trace / 2 + r_clock / 2 + w_max / 2`. Record the frozen causal-matching algorithm and its matching-window width, but do not choose either a maximum acceptable `U_95` or a matching tolerance from characterization data. P4C cannot close P4. Before candidate measurements, a reviewed Stage 3 decision must freeze the numeric maximum acceptable `U_95`, the causal-matching algorithm version, and the matching-window width; P4 qualification compares the predeclared calculation with that decision. Reject an epoch on output-association change. Prove no candidate callback or IPC path feeds the trace by preserving the observer process graph and callback or IPC edge inventory; any such edge fails P4. Apply `CON-FRM-001`'s 10% rule only to the measured 95th-percentile interval-error result after the causal matcher and independent meter are qualified, never to the clock-calibration offset. Expected output: Ubuntu image package record, source or patch identity, live format and SHA-256, source schema and call-site excerpts, trace command, selected trace clock, pipe-to-CRTC-ID-to-connector inventory, surface-output pairing log, four monotonic samples and two `P4_CAL` markers, the 10,000-marker uncertainty-budget record, observer process graph, callback or IPC edge inventory, per-output epoch log, and the separately calculated `CON-FRM-001` result. |
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

- **State today:** Stage 3 pins Ubuntu 26.04 LTS, the `gtk4` crate `v4_20` API-binding ceiling, `GtkIMContext`, AT-SPI families, `GdkFrameClock`, and `wp_presentation` feedback. SPK-B004 audits `libgtk-4-1` `4.22.2+ds-1ubuntu1` and `at-spi2-core` `2.60.0-1` as the shared Ubuntu package identities for X11 and Wayland. Stage 3 still leaves the selected Wayland session's package-manifest lock, compositor, complete candidate maps, independent meter, routing traces, and injectable recovery gates unresolved.
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

The following round-5 parser output is historical evidence only. Round 6 supersedes its operation set and checklist below because the earlier parser omitted foundational registry operations, included drag-and-drop-only events, and made nondeterministic events ordinary pass requirements. Round 9 supersedes every historical local-only floor where an interface-typed `new_id` factory child requires a higher bound version; in particular, the displayed `wl_data_device_manager` local floor 1 is not a usable protocol floor. XML members with no `since` attribute have version 1. Neither derivation establishes an Ubuntu compositor's advertisement or behavior.

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

### Round-5 historical P1 transcript checklist (superseded)

This historical checklist was generated from the round-5 derivation. It is not a valid P1 pass rule because it lacks foundational members, includes drag-and-drop-only events, and requires nondeterministic events. The round-6 parser below is the sole checklist generator for P1.

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

The historical parser ran against the report before the round-6 correction. Its output is retained for audit only and is not a P1 acceptance checklist:

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

### Round-6 correction: deterministic P1 operation derivation

The round-5 99-member checklist is retained above as historical evidence only. It is superseded: it omitted registry discovery and synchronization, included four operating-system drag-and-drop events, and treated events without a deterministic stimulus as ordinary pass members. No subsequent P1, Stage 3 edit, or recommendation may describe that historical 99-member list as complete.

[`CAP-CLP-001`](../prd/capabilities.md#interaction-and-text) makes copy, cut, and paste P0. [`CAP-INP-001`](../prd/capabilities.md#interaction-and-text) and [`CAP-INP-002`](../prd/capabilities.md#interaction-and-text) make pointer, touch, and deterministic gesture disambiguation P0. The PRD contains no P0 operating-system drag-and-drop capability. The corrected selection retains the two-client clipboard flow, but excludes `wl_data_device.enter`, `wl_data_device.leave`, `wl_data_device.motion`, `wl_data_device.drop`, and `wl_data_device.start_drag`. The absence of `start_drag` is intentional: the report does not claim a partial drag-and-drop source or target flow is P0.

The [Wayland protocol specification](https://wayland.freedesktop.org/docs/html/apa.html) states that `wl_display.sync` asks the server to emit `wl_callback.done`, and that clients use `wl_display.get_registry` followed by `sync` to finish the initial global burst before `wl_registry.bind`. The corrected source selection therefore adds `wl_display.get_registry`, `wl_display.sync`, `wl_display.delete_id`, `wl_registry.global`, `wl_registry.bind`, and the separately gated `wl_registry.global_remove`. The pinned [Wayland core XML](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml) is the source of record for each selected operation and its `since` value.

The derivation has one explicit rule for every listed operation: include a request or event only when it is a foundational discovery or synchronization operation, a P0 surface, input, clipboard, shell, scale, IME, presentation-acknowledgement, or teardown operation; attach that operation's rule, controlled stimulus, and expected result in the derivation output. Four selected events have no protocol-defined deterministic stimulus on an unselected compositor. They remain separate KU gates and are excluded from the deterministic P1 pass rule. The resulting source set has 101 selected operations: 97 deterministic P1 pass operations and four retained KU events. This is not a 101-member acceptance checklist.

Table 2. Corrected operation-set and event-gate answers

| Row | Answer and evidence | Status | Next bounded probe |
| :-- | :-- | :-- | :-- |
| Corrected operation selection and deterministic pass subset | The preserved derivation script verifies the SHA-256 of each pinned XML before parsing it, rejects the five drag-and-drop members, derives 101 selected operations, propagates interface-typed `new_id` factory child floors to a fixpoint, and mechanically marks 97 operations as deterministic pass members. The round-9 run records the six immutable input URL-to-SHA-256 pairs, `selected_operations=101 deterministic_pass_operations=97 ku_events_excluded_from_pass=4`, and one factory-floor change: `wl_data_device_manager` 1 to 2 through `get_data_device` to `wl_data_device`. The PRD citations above establish why clipboard is retained and operating-system drag-and-drop is not P0. | KK | Not required for the source-level selection. P1 must regenerate the parser output from the preserved derivation output rather than copy a list. |
| Nondeterministic foundational and compositor-selected events | `wl_registry.global_remove`, `wl_touch.cancel`, `xdg_wm_base.ping`, and `wp_presentation_feedback.discarded` are selected source operations, but neither their pinned XML nor the fetched protocol specification defines a deterministic stimulus available to this report's unselected reference compositor. They are excluded from the 97-member pass rule. In particular, rendering frames cannot deterministically produce `wl_touch.cancel`. | KU (gating) | P1E: On the selected Ubuntu compositor, record a documented compositor test-control or controlled physical procedure for each event. Invoke the procedure once per event and preserve the `WAYLAND_DEBUG=client` line and its stimulus log. Expected output: `global_remove` with the removed global name; `cancel` after an active touch sequence; `ping` followed by the fixture's `pong`; and `discarded` for the named presentation-feedback object. If the locked compositor exposes no deterministic procedure for an event, STOP that event subprobe and retain this KU. |

Table 3 maps every event that remains in the deterministic subset to a controlled P1 stimulus and expected transcript result. The script emits the matching `stimulus` and `expected` tags for every request and event, so the table is also the controlled-input procedure for the generated checklist.

| Stimulus tag | Controlled P1 procedure | Expected event result |
| :-- | :-- | :-- |
| `registry-sync` | Connect the noncandidate fixture, call `wl_display.get_registry`, record every `global`, bind each required advertised global at the derived floor, then call `wl_display.sync`. | `wl_registry.global` names every required global; `wl_callback.done` terminates the sync barrier; `wl_display.delete_id` releases the callback ID. |
| `map-surface` | Create an xdg toplevel, attach and damage a buffer, request a frame callback, commit, map it first on output A, then use the locked compositor's recorded window-move procedure to move it fully to connected output B. | `wl_surface.enter` identifies A; the move yields `wl_surface.leave` for A and `wl_surface.enter` for B; the compositor emits `xdg_surface.configure`. |
| `bind-seat` | Bind the selected seat at its derived floor before creating pointer, keyboard, and touch objects. | `wl_seat.capabilities` includes the required pointer, keyboard, and touch capabilities. |
| `hid-pointer` | Use a lab-controlled USB HID pointer to enter and leave the fixture, move it, click, send a wheel scroll sequence that ends, and set a cursor. | `wl_pointer.enter`, `leave`, `motion`, `button`, `axis`, `axis_source`, `axis_stop`, and `frame` appear in the recorded sequence. |
| `hid-keyboard` | Focus the fixture and a second controlled client in turn with the USB HID pointer, press and release a modifier and a key, then hold a repeatable key. | `wl_keyboard.keymap`, `enter`, `leave`, `key`, `modifiers`, and `repeat_info` appear; the key record shows the controlled press and release. |
| `hid-touch` | Use a lab-controlled USB HID touchscreen to send one contact down, move it, and lift it over the fixture. | `wl_touch.down`, `motion`, `up`, and `frame` occur in one completed touch sequence. |
| `bind-output` | Bind every output at the derived floor before mapping the fixture. | `wl_output.geometry`, `mode`, `done`, and `scale` describe each bound output. |
| `clipboard-two-client` | Client A creates a data source, offers a fixed UTF-8 MIME type, and sets the selection. Client B receives the selection and MIME offer, calls `receive`, and verifies the fixed payload. Client C then takes the selection. | Client B records `wl_data_device.data_offer`, `selection`, and `wl_data_offer.offer`; A records `wl_data_source.send` for B's receive and `wl_data_source.cancelled` when C takes selection. |
| `map-toplevel` | Map the titled, app-ID-bearing xdg toplevel and acknowledge its first configure. | `xdg_surface.configure` and `xdg_toplevel.configure` arrive; the fixture responds with `xdg_surface.ack_configure`. |
| `map-close-toplevel` | Use the locked compositor's recorded close-window procedure, such as the session's documented close shortcut, while the fixture is mapped. | `xdg_toplevel.close` arrives for the fixture. |
| `fractional-output` | Map the fixture onto a selected output whose locked session configuration has a fractional scale. | `wp_fractional_scale_v1.preferred_scale` reports the preferred scale. |
| `ime-composition` | Use the locked IME package and its predeclared input sequence to start composition, update preedit text, commit text, and request a surrounding-text deletion. | `zwp_text_input_v3.preedit_string`, `commit_string`, `delete_surrounding_text`, and `done` complete the corresponding committed transaction. |
| `present-frame` | Create presentation feedback immediately before a buffer commit, then wait for displayed output while the surface remains mapped. | `wp_presentation_feedback.sync_output` identifies the main output and `presented` completes that feedback object. |

The four event gates have deliberately different handling. `compositor-global-removal` means invoking the selected compositor's documented test control to remove a previously advertised global and expecting `wl_registry.global_remove`. `compositor-touch-cancel` means invoking its documented active-touch cancellation control and expecting `wl_touch.cancel`; no frame-rendering procedure substitutes for it. `compositor-ping` means invoking its documented client-hang detection control and expecting `xdg_wm_base.ping`, followed by the fixture's `pong`. `compositor-discard` means invoking its documented feedback-discard control and expecting `wp_presentation_feedback.discarded`. Each remains P1E KU (gating) until the stated procedure is actually preserved on the selected session.

The complete primary derivation script is preserved here. It uses only immutable pinned XML inputs, verifies each input digest before parsing, and asserts that every KU item is an event and every drag-and-drop operation is absent.

```python
#!/usr/bin/env python3
"""Derive selected Wayland P0 floors from pinned protocol XML."""
from __future__ import annotations

from hashlib import sha256
from pathlib import Path
import sys
import xml.etree.ElementTree as ET

SOURCES = (
    ("wayland.xml", "https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml", "7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610"),
    ("xdg-shell.xml", "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/xdg-shell/xdg-shell.xml", "f241ab95c262eb45af6507bf0178b8dcd88d5e1cc8fc6ec4a9944d9e5b15ac98"),
    ("viewporter.xml", "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/viewporter/viewporter.xml", "dcb12279a03746301fe490aaed4b38a403485a925abfce2ccfceb644e104fe71"),
    ("fractional-scale-v1.xml", "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/staging/fractional-scale/fractional-scale-v1.xml", "5941de5d28f427ecdadddc8623a6f6af0a30b0ab4726847236ba7a7652b81316"),
    ("text-input-unstable-v3.xml", "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/unstable/text-input/text-input-unstable-v3.xml", "49048087a67011a8840bca889cd2b0ba374382be1ed54ec98adf7837fdca1982"),
    ("presentation-time-v1.xml", "https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml", "91e5e14481a13717fef8403203a2eaa052c85fd853c1c440ba081effa7178d12"),
)
# interface|rule|stimulus|expected|members; every listed member inherits its row rule.
SPECS = """
wl_display|foundation-registry|registry-sync|registry-object-or-sync-barrier|get_registry sync
wl_display|foundation-id-reuse|registry-sync|delete-id-after-callback|delete_id
wl_registry|foundation-registry|registry-sync|advertised-globals-or-bound-object|global bind
wl_registry|foundation-global-removal|compositor-global-removal|global-remove|global_remove
wl_compositor|surface-factory|map-surface|surface-object|create_surface
wl_surface|surface-lifecycle|map-surface|mapped-commit-or-output-membership|attach damage frame commit enter leave destroy
wl_callback|synchronization|registry-sync|done|done
wl_seat|input-discovery|bind-seat|required-capabilities-and-devices|capabilities get_pointer get_keyboard get_touch release
wl_pointer|pointer-input|hid-pointer|pointer-event|enter leave motion button axis axis_source axis_stop frame set_cursor release
wl_keyboard|keyboard-input|hid-keyboard|keyboard-event|keymap enter leave key modifiers repeat_info release
wl_touch|touch-input|hid-touch|touch-event|down up motion frame release
wl_touch|touch-cancellation|compositor-touch-cancel|cancel|cancel
wl_output|output-description|bind-output|output-description|geometry mode done scale release
wl_data_device_manager|clipboard-factory|clipboard-two-client|clipboard-objects|create_data_source get_data_device
wl_data_device|clipboard-selection|clipboard-two-client|selection-offer|data_offer selection set_selection release
wl_data_offer|clipboard-receive|clipboard-two-client|mime-offer-and-payload|offer receive destroy
wl_data_source|clipboard-source|clipboard-two-client|mime-send-and-cancel|offer send cancelled destroy
xdg_wm_base|xdg-shell|map-toplevel|xdg-object-or-pong|get_xdg_surface pong destroy
xdg_wm_base|xdg-ping|compositor-ping|ping|ping
xdg_surface|xdg-shell|map-toplevel|configure-and-ack|get_toplevel ack_configure configure destroy
xdg_toplevel|toplevel-lifecycle|map-close-toplevel|configure-or-close|set_title set_app_id configure close destroy
wp_viewporter|scale-factory|map-surface|viewport-object|get_viewport destroy
wp_viewport|destination-sizing|map-surface|destination-size|set_destination destroy
wp_fractional_scale_manager_v1|fractional-scale-factory|fractional-output|scale-object|get_fractional_scale destroy
wp_fractional_scale_v1|fractional-scale|fractional-output|preferred-scale|preferred_scale destroy
zwp_text_input_manager_v3|ime-factory|ime-composition|text-input-object|get_text_input destroy
zwp_text_input_v3|ime-round-trip|ime-composition|ime-transaction|enable disable set_surrounding_text set_text_change_cause set_content_type set_cursor_rectangle commit preedit_string commit_string delete_surrounding_text done destroy
wp_presentation|presentation-ack-factory|present-frame|feedback-object|feedback destroy
wp_presentation_feedback|presentation-ack|present-frame|output-label-or-presented|sync_output presented
wp_presentation_feedback|presentation-discard|compositor-discard|discarded|discarded
""".strip()
KU_EVENTS = frozenset(("wl_registry.global_remove", "wl_touch.cancel", "xdg_wm_base.ping", "wp_presentation_feedback.discarded"))
FORBIDDEN_DND = frozenset(("wl_data_device.enter", "wl_data_device.leave", "wl_data_device.motion", "wl_data_device.drop", "wl_data_device.start_drag"))
CHAIN_CHECKS = (
    ("wl_seat",),
    ("xdg_wm_base", "xdg_surface", "xdg_toplevel"),
    ("wp_viewporter",),
    ("wp_fractional_scale_manager_v1",),
    ("zwp_text_input_manager_v3",),
    ("wp_presentation",),
)


def bare_factory_children(node: ET.Element) -> tuple[str, ...]:
    """Returns static interface-typed new_id children only when no `version` argument exists and `"interface" in argument.attrib`."""
    arguments = tuple(node.findall("arg"))
    if any(argument.attrib.get("name") == "version" for argument in arguments):
        return ()
    return tuple(
        argument.attrib["interface"]
        for argument in arguments
        if argument.attrib.get("type") == "new_id" and "interface" in argument.attrib
    )


def propagate_factory_floors(local_floors: dict[str, int], factory_edges: list[tuple[str, str, str]]) -> tuple[dict[str, int], list[list[tuple[str, int, int, str, str]]]]:
    """Propagates interface-typed new_id child floors to their factory interfaces to a fixpoint."""
    floors = dict(local_floors)
    passes: list[list[tuple[str, int, int, str, str]]] = []
    while True:
        changes: list[tuple[str, int, int, str, str]] = []
        for parent, child, member in factory_edges:
            if child not in floors:
                continue
            if floors[child] > floors[parent]:
                before = floors[parent]
                floors[parent] = floors[child]
                changes.append((parent, before, floors[parent], child, member))
        passes.append(changes)
        if not changes:
            return floors, passes


def main() -> None:
    root = Path(sys.argv[1]) if len(sys.argv) == 2 else Path(__file__).with_name("sources")
    interfaces = {}
    print("input-url-sha256-map")
    for filename, url, expected_hash in SOURCES:
        content = (root / filename).read_bytes()
        actual_hash = sha256(content).hexdigest()
        if actual_hash != expected_hash:
            raise SystemExit(f"digest mismatch: {filename} expected={expected_hash} actual={actual_hash}")
        print(f"input file={filename} url={url} sha256={actual_hash}")
        interfaces.update({node.attrib["name"]: node for node in ET.fromstring(content).findall("interface")})
    seen: set[str] = set()
    local_members: dict[str, list[tuple[str, int]]] = {}
    factory_edges: list[tuple[str, str, str]] = []
    operations: list[tuple[str, str]] = []
    print("derived-operations")
    for spec in SPECS.splitlines():
        interface, rule, stimulus, expected, names = spec.split("|")
        members = {node.attrib["name"]: node for node in (*interfaces[interface].findall("request"), *interfaces[interface].findall("event"))}
        for name in names.split():
            qualified = f"{interface}.{name}"
            if qualified in seen or qualified in FORBIDDEN_DND:
                raise SystemExit(f"invalid selected operation: {qualified}")
            seen.add(qualified)
            node = members[name]
            kind, since = node.tag, int(node.attrib.get("since", "1"))
            qualification = "ku" if qualified in KU_EVENTS else "pass"
            if qualification == "ku" and kind != "event":
                raise SystemExit(f"KU operation is not an event: {qualified}")
            operations.append((qualified, qualification))
            local_members.setdefault(interface, []).append((name, since))
            for child in bare_factory_children(node):
                factory_edges.append((interface, child, qualified))
            print(f"operation={qualified} kind={kind} since={since} qualification={qualification} rule={rule} stimulus={stimulus} expected={expected}")
    local_floors = {interface: max(since for _, since in members) for interface, members in local_members.items()}
    floors, propagation_passes = propagate_factory_floors(local_floors, factory_edges)
    print("factory-propagation")
    for pass_number, changes in enumerate(propagation_passes, start=1):
        if changes:
            for parent, before, after, child, member in changes:
                print(f"factory-pass={pass_number} factory={parent} child={child} member={member} local_or_prior_floor={before} propagated_floor={after} changed=yes")
        else:
            print(f"factory-pass={pass_number} changes=none fixpoint=yes")
    for chain in CHAIN_CHECKS:
        chain_members = ",".join(chain)
        result = ";".join(f"{interface}:local={local_floors[interface]},floor={floors[interface]}" for interface in chain)
        changed = "yes" if any(local_floors[interface] != floors[interface] for interface in chain) else "no"
        print(f"factory-chain={chain_members} result={result} changed={changed}")
    print("derived-floors")
    for interface, members in local_members.items():
        required = ",".join(f"{name}@{since}" for name, since in members)
        print(f"floor interface={interface} declared={interfaces[interface].attrib['version']} required={required} local_floor={local_floors[interface]} factory_floor={floors[interface]} floor={floors[interface]}")
    passed = sum(qualification == "pass" for _, qualification in operations)
    print(f"summary selected_operations={len(operations)} deterministic_pass_operations={passed} ku_events_excluded_from_pass={len(operations) - passed} factory_changed_interfaces={sum(local_floors[interface] != floors[interface] for interface in floors)}")


if __name__ == "__main__":
    main()
```

The complete parser is also preserved. It accepts only primary-derivation operation lines and emits the pass subset plus the excluded KU events; it does not parse prose or a manually maintained list.

```python
#!/usr/bin/env python3
"""Emit the deterministic P1 checklist from derive-wayland-floors.py output."""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROW = re.compile(r"^operation=(?P<member>[a-z0-9_]+\.[a-z0-9_]+) kind=(?P<kind>request|event) since=\d+ qualification=(?P<qualification>pass|ku) ")


def main() -> None:
    rows = Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()
    checklist: list[str] = []
    ku_events: list[str] = []
    for row in rows:
        match = ROW.match(row)
        if match is None:
            continue
        if match.group("qualification") == "pass":
            checklist.append(match.group("member"))
        else:
            ku_events.append(match.group("member"))
    if not checklist or not ku_events:
        raise SystemExit("missing pass operation or KU event")
    print(f"derived_deterministic_pass_operations={len(checklist)}")
    print("P1 deterministic transcript checklist:")
    print(*(f"- {member}" for member in checklist), sep="\n")
    print(f"retained_ku_events_excluded_from_pass={len(ku_events)}")
    print(*(f"- {member}" for member in ku_events), sep="\n")


if __name__ == "__main__":
    main()
```

The following round-6 command and trimmed output are retained as the historical local-member derivation. It omits factory propagation, so its `wl_data_device_manager` floor is not a usable protocol floor. Round 9 supersedes that floor below. The omitted `operation=` lines are reproducible only from the complete script and the digests shown here; the parser output after this block preserves the entire deterministic checklist.

```text
$ nix shell nixpkgs#python3 -c python3 /tmp/wf-epic-b/OXY-B003/round-6/derive-wayland-floors.py /tmp/wf-epic-b/OXY-B003/round-6/sources > /tmp/wf-epic-b/OXY-B003/round-6/derive-wayland-floors.out
$ grep -E '^(input file=|floor interface=|summary)' /tmp/wf-epic-b/OXY-B003/round-6/derive-wayland-floors.out
input file=wayland.xml url=https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml sha256=7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610
input file=xdg-shell.xml url=https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/xdg-shell/xdg-shell.xml sha256=f241ab95c262eb45af6507bf0178b8dcd88d5e1cc8fc6ec4a9944d9e5b15ac98
input file=viewporter.xml url=https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/viewporter/viewporter.xml sha256=dcb12279a03746301fe490aaed4b38a403485a925abfce2ccfceb644e104fe71
input file=fractional-scale-v1.xml url=https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/staging/fractional-scale/fractional-scale-v1.xml sha256=5941de5d28f427ecdadddc8623a6f6af0a30b0ab4726847236ba7a7652b81316
input file=text-input-unstable-v3.xml url=https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/unstable/text-input/text-input-unstable-v3.xml sha256=49048087a67011a8840bca889cd2b0ba374382be1ed54ec98adf7837fdca1982
input file=presentation-time-v1.xml url=https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml sha256=91e5e14481a13717fef8403203a2eaa052c85fd853c1c440ba081effa7178d12
floor interface=wl_display declared=1 required=get_registry@1,sync@1,delete_id@1 floor=1
floor interface=wl_registry declared=1 required=global@1,bind@1,global_remove@1 floor=1
floor interface=wl_compositor declared=6 required=create_surface@1 floor=1
floor interface=wl_surface declared=6 required=attach@1,damage@1,frame@1,commit@1,enter@1,leave@1,destroy@1 floor=1
floor interface=wl_callback declared=1 required=done@1 floor=1
floor interface=wl_seat declared=10 required=capabilities@1,get_pointer@1,get_keyboard@1,get_touch@1,release@5 floor=5
floor interface=wl_pointer declared=10 required=enter@1,leave@1,motion@1,button@1,axis@1,axis_source@5,axis_stop@5,frame@5,set_cursor@1,release@3 floor=5
floor interface=wl_keyboard declared=10 required=keymap@1,enter@1,leave@1,key@1,modifiers@1,repeat_info@4,release@3 floor=4
floor interface=wl_touch declared=10 required=down@1,up@1,motion@1,frame@1,release@3,cancel@1 floor=3
floor interface=wl_output declared=4 required=geometry@1,mode@1,done@2,scale@2,release@3 floor=3
floor interface=wl_data_device_manager declared=3 required=create_data_source@1,get_data_device@1 floor=1
floor interface=wl_data_device declared=3 required=data_offer@1,selection@1,set_selection@1,release@2 floor=2
floor interface=wl_data_offer declared=3 required=offer@1,receive@1,destroy@1 floor=1
floor interface=wl_data_source declared=3 required=offer@1,send@1,cancelled@1,destroy@1 floor=1
floor interface=xdg_wm_base declared=6 required=get_xdg_surface@1,pong@1,destroy@1,ping@1 floor=1
floor interface=xdg_surface declared=6 required=get_toplevel@1,ack_configure@1,configure@1,destroy@1 floor=1
floor interface=xdg_toplevel declared=6 required=set_title@1,set_app_id@1,configure@1,close@1,destroy@1 floor=1
floor interface=wp_viewporter declared=1 required=get_viewport@1,destroy@1 floor=1
floor interface=wp_viewport declared=1 required=set_destination@1,destroy@1 floor=1
floor interface=wp_fractional_scale_manager_v1 declared=1 required=get_fractional_scale@1,destroy@1 floor=1
floor interface=wp_fractional_scale_v1 declared=1 required=preferred_scale@1,destroy@1 floor=1
floor interface=zwp_text_input_manager_v3 declared=1 required=get_text_input@1,destroy@1 floor=1
floor interface=zwp_text_input_v3 declared=1 required=enable@1,disable@1,set_surrounding_text@1,set_text_change_cause@1,set_content_type@1,set_cursor_rectangle@1,commit@1,preedit_string@1,commit_string@1,delete_surrounding_text@1,done@1,destroy@1 floor=1
floor interface=wp_presentation declared=1 required=feedback@1,destroy@1 floor=1
floor interface=wp_presentation_feedback declared=1 required=sync_output@1,presented@1,discarded@1 floor=1
summary selected_operations=101 deterministic_pass_operations=97 ku_events_excluded_from_pass=4
exit=0
```

The parser output is retained because factory propagation changes only the `wl_data_device_manager` bind floor, not selected operations or their pass classification. The round-9 rerun below confirms the unchanged 97-member deterministic subset and four excluded KU events.

```text
$ nix shell nixpkgs#python3 -c python3 /tmp/wf-epic-b/OXY-B003/round-6/derive-p1-transcript-checklist.py /tmp/wf-epic-b/OXY-B003/round-6/derive-wayland-floors.out
derived_deterministic_pass_operations=97
P1 deterministic transcript checklist:
- wl_display.get_registry
- wl_display.sync
- wl_display.delete_id
- wl_registry.global
- wl_registry.bind
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
- wl_touch.release
- wl_output.geometry
- wl_output.mode
- wl_output.done
- wl_output.scale
- wl_output.release
- wl_data_device_manager.create_data_source
- wl_data_device_manager.get_data_device
- wl_data_device.data_offer
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
retained_ku_events_excluded_from_pass=4
- wl_registry.global_remove
- wl_touch.cancel
- xdg_wm_base.ping
- wp_presentation_feedback.discarded
exit=0
```

### Round-9 correction: factory-version propagation

The earlier derivation used only each interface's selected-member `since` maximum. That rule is insufficient for an object created through an interface-typed `new_id`: when the `new_id` has an `interface` attribute and the request or event has no `version` argument, the child inherits the factory object's bound version. A bare or untyped `new_id`, such as `wl_registry.bind` in the pinned [Wayland core XML](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml), has no `interface` attribute and does not inherit the version. The revised primary derivation script above detects only the interface-typed criterion, constructs the factory-to-child graph, and repeatedly raises each factory floor to the child floor until a pass makes no changes. It retains `local_floor` separately from the propagated `factory_floor` to make the correction auditable.

The revised derivation finds one change: `wl_data_device_manager.get_data_device` creates `wl_data_device` with an interface-typed `new_id`; `wl_data_device.release` has `since="2"`; and the manager must therefore bind at version 2. The pass reaches its fixpoint on the second iteration. It checks `wl_seat`, `xdg_wm_base` through `xdg_surface` through `xdg_toplevel`, `wp_viewporter`, `wp_fractional_scale_manager_v1`, `zwp_text_input_manager_v3`, and `wp_presentation`; none changes. The P1 operation count and membership remain 97 deterministic pass operations and four separate KU events.

The round-9 probe fetched the pinned core XML again from its canonical URL. The Ubuntu package-page response uses Jina only as transport. Its body is not a fixture and has no digest.

```text
$ probe_dir=/tmp/wf-epic-b/OXY-B003/round-9; wayland_url=https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml; curl -sS -fL --max-time 60 --output "$probe_dir/sources/wayland.xml" "$wayland_url"; printf 'url=%s bytes=%s sha256=%s\n' "$wayland_url" "$(wc -c < "$probe_dir/sources/wayland.xml")" "$(sha256sum "$probe_dir/sources/wayland.xml" | awk '{print $1}')"
url=https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml bytes=151742 sha256=7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610
$ ubuntu_url=https://packages.ubuntu.com/resolute/at-spi2-core; curl -sS -fL --max-time 60 --output "$probe_dir/ubuntu-at-spi2-core.txt" "https://r.jina.ai/$ubuntu_url"; printf 'url=%s transport=jina bytes=%s\n' "$ubuntu_url" "$(wc -c < "$probe_dir/ubuntu-at-spi2-core.txt")"; grep -F -m 1 '2.60.0-1' "$probe_dir/ubuntu-at-spi2-core.txt"
url=https://packages.ubuntu.com/resolute/at-spi2-core transport=jina bytes=2868
## Package: at-spi2-core (2.60.0-1)
exit=0
```

The following trimmed rerun preserves the revised output. It includes every derived floor and the required factory-chain checks.

```text
$ nix shell nixpkgs#python3 -c python3 /tmp/wf-epic-b/OXY-B003/round-9/derive-wayland-floors.py /tmp/wf-epic-b/OXY-B003/round-9/sources > /tmp/wf-epic-b/OXY-B003/round-9/derive-wayland-floors.out
$ grep -E '^(input file=wayland.xml|factory-pass=|factory-chain=|floor interface=|summary)' /tmp/wf-epic-b/OXY-B003/round-9/derive-wayland-floors.out
input file=wayland.xml url=https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml sha256=7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610
factory-pass=1 factory=wl_data_device_manager child=wl_data_device member=wl_data_device_manager.get_data_device local_or_prior_floor=1 propagated_floor=2 changed=yes
factory-pass=2 changes=none fixpoint=yes
factory-chain=wl_seat result=wl_seat:local=5,floor=5 changed=no
factory-chain=xdg_wm_base,xdg_surface,xdg_toplevel result=xdg_wm_base:local=1,floor=1;xdg_surface:local=1,floor=1;xdg_toplevel:local=1,floor=1 changed=no
factory-chain=wp_viewporter result=wp_viewporter:local=1,floor=1 changed=no
factory-chain=wp_fractional_scale_manager_v1 result=wp_fractional_scale_manager_v1:local=1,floor=1 changed=no
factory-chain=zwp_text_input_manager_v3 result=zwp_text_input_manager_v3:local=1,floor=1 changed=no
factory-chain=wp_presentation result=wp_presentation:local=1,floor=1 changed=no
floor interface=wl_display declared=1 required=get_registry@1,sync@1,delete_id@1 local_floor=1 factory_floor=1 floor=1
floor interface=wl_registry declared=1 required=global@1,bind@1,global_remove@1 local_floor=1 factory_floor=1 floor=1
floor interface=wl_compositor declared=6 required=create_surface@1 local_floor=1 factory_floor=1 floor=1
floor interface=wl_surface declared=6 required=attach@1,damage@1,frame@1,commit@1,enter@1,leave@1,destroy@1 local_floor=1 factory_floor=1 floor=1
floor interface=wl_callback declared=1 required=done@1 local_floor=1 factory_floor=1 floor=1
floor interface=wl_seat declared=10 required=capabilities@1,get_pointer@1,get_keyboard@1,get_touch@1,release@5 local_floor=5 factory_floor=5 floor=5
floor interface=wl_pointer declared=10 required=enter@1,leave@1,motion@1,button@1,axis@1,axis_source@5,axis_stop@5,frame@5,set_cursor@1,release@3 local_floor=5 factory_floor=5 floor=5
floor interface=wl_keyboard declared=10 required=keymap@1,enter@1,leave@1,key@1,modifiers@1,repeat_info@4,release@3 local_floor=4 factory_floor=4 floor=4
floor interface=wl_touch declared=10 required=down@1,up@1,motion@1,frame@1,release@3,cancel@1 local_floor=3 factory_floor=3 floor=3
floor interface=wl_output declared=4 required=geometry@1,mode@1,done@2,scale@2,release@3 local_floor=3 factory_floor=3 floor=3
floor interface=wl_data_device_manager declared=3 required=create_data_source@1,get_data_device@1 local_floor=1 factory_floor=2 floor=2
floor interface=wl_data_device declared=3 required=data_offer@1,selection@1,set_selection@1,release@2 local_floor=2 factory_floor=2 floor=2
floor interface=wl_data_offer declared=3 required=offer@1,receive@1,destroy@1 local_floor=1 factory_floor=1 floor=1
floor interface=wl_data_source declared=3 required=offer@1,send@1,cancelled@1,destroy@1 local_floor=1 factory_floor=1 floor=1
floor interface=xdg_wm_base declared=6 required=get_xdg_surface@1,pong@1,destroy@1,ping@1 local_floor=1 factory_floor=1 floor=1
floor interface=xdg_surface declared=6 required=get_toplevel@1,ack_configure@1,configure@1,destroy@1 local_floor=1 factory_floor=1 floor=1
floor interface=xdg_toplevel declared=6 required=set_title@1,set_app_id@1,configure@1,close@1,destroy@1 local_floor=1 factory_floor=1 floor=1
floor interface=wp_viewporter declared=1 required=get_viewport@1,destroy@1 local_floor=1 factory_floor=1 floor=1
floor interface=wp_viewport declared=1 required=set_destination@1,destroy@1 local_floor=1 factory_floor=1 floor=1
floor interface=wp_fractional_scale_manager_v1 declared=1 required=get_fractional_scale@1,destroy@1 local_floor=1 factory_floor=1 floor=1
floor interface=wp_fractional_scale_v1 declared=1 required=preferred_scale@1,destroy@1 local_floor=1 factory_floor=1 floor=1
floor interface=zwp_text_input_manager_v3 declared=1 required=get_text_input@1,destroy@1 local_floor=1 factory_floor=1 floor=1
floor interface=zwp_text_input_v3 declared=1 required=enable@1,disable@1,set_surrounding_text@1,set_text_change_cause@1,set_content_type@1,set_cursor_rectangle@1,commit@1,preedit_string@1,commit_string@1,delete_surrounding_text@1,done@1,destroy@1 local_floor=1 factory_floor=1 floor=1
floor interface=wp_presentation declared=1 required=feedback@1,destroy@1 local_floor=1 factory_floor=1 floor=1
floor interface=wp_presentation_feedback declared=1 required=sync_output@1,presented@1,discarded@1 local_floor=1 factory_floor=1 floor=1
summary selected_operations=101 deterministic_pass_operations=97 ku_events_excluded_from_pass=4 factory_changed_interfaces=1
exit=0
```

The regenerated checklist parser found the same pass and KU counts:

```text
$ nix shell nixpkgs#python3 -c python3 - /tmp/wf-epic-b/OXY-B003/round-9/derive-wayland-floors.out <<'PY'
import re
import sys
rows = open(sys.argv[1], encoding="utf-8").read().splitlines()
pattern = re.compile(r"^operation=(?P<member>[a-z0-9_]+\.[a-z0-9_]+) kind=(?:request|event) since=\d+ qualification=(?P<qualification>pass|ku) ")
items = [match.groupdict() for row in rows if (match := pattern.match(row))]
passed = [item["member"] for item in items if item["qualification"] == "pass"]
ku = [item["member"] for item in items if item["qualification"] == "ku"]
assert len(passed) == 97
assert ku == ["wl_registry.global_remove", "wl_touch.cancel", "xdg_wm_base.ping", "wp_presentation_feedback.discarded"]
print(f"derived_deterministic_pass_operations={len(passed)}")
print(f"retained_ku_events_excluded_from_pass={len(ku)}")
print("exit=0")
PY
derived_deterministic_pass_operations=97
retained_ku_events_excluded_from_pass=4
exit=0
```

### Round-6 correction: P4 calibration characterization and qualification boundary

P4C is a characterization probe, not qualification. Before any candidate measurement, P4C records exactly 10,000 trace-marker pairs. For pair `i`, it records the ftrace `mono` timestamp `m_i` for the unique marker and the surrounding userspace readings `t_before_i` and `t_after_i`. It calculates `d_i = m_i - (t_before_i + t_after_i) / 2`, `d_bar = mean(d_i)`, and `SE(d_bar) = sample_sd(d_i) / sqrt(10,000)`. It records `r_trace` as the smallest positive difference between monotonically increasing ftrace `mono` timestamps, `r_clock` as the larger of `clock_getres(CLOCK_MONOTONIC)` and the smallest positive difference between monotonically increasing `clock_gettime(CLOCK_MONOTONIC)` results, and `w_max = max(t_after_i - t_before_i)`.

P4C predeclares the recorded 95% characterization bound as `U_95 = 1.96 * SE(d_bar) + r_trace / 2 + r_clock / 2 + w_max / 2`. The first term is the 95% confidence half-width for the estimated offset; the remaining terms conservatively account for trace timestamp quantization, userspace timestamp resolution, and an unknown marker time within the widest bracket. P4C also records, but does not tune, the causal-matching algorithm version and matching-window width.

P4C cannot close P4 or any P0 timing gate by itself. A reviewed Stage 3 decision must freeze the numeric maximum acceptable `U_95`, the causal-matching algorithm version, and its matching-window width before candidate measurements begin. Qualification can only compare candidate data against those already frozen values. This report does not choose an acceptable tolerance, infer one from P4C data, or apply `CON-FRM-001` to offset uncertainty. `CON-FRM-001` remains a separate measured interval-error rule after the independent meter and causal matcher qualify.

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
- [Ubuntu `at-spi2-core` package](https://packages.ubuntu.com/resolute/at-spi2-core).
- [AT-SPI 2.60.0 `Text.xml`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.0/xml/Text.xml).
- [AT-SPI 2.60.0 `EditableText.xml`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.0/xml/EditableText.xml).
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
- [Wayland protocol specification](https://wayland.freedesktop.org/docs/html/apa.html).
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
atspi-text-2.60.0 sha256=602cdb27666912ac0cdf9ac53e5d718e002cd4fe1a37e9a9dc67c71f2acc4249
atspi-editable-text-2.60.0 sha256=2ea1b94822f19b0b00c80b918b89833cfb67d1eeef99d69b8421d0e6f40920ff
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

### Round-8 AT-SPI source and package-identity correction

SPK-B004's [Ubuntu source-package audit](SPK-B004.md#ubuntu-source-package-audit) establishes `at-spi2-core` `2.60.0-1` as the Ubuntu package identity for the X11 reference environment. The same package identity is the Wayland reference input. It is distinct from the 2.60.6 upstream XML source floor. The direct comparison below shows that `EditableText.xml` is byte-identical between the 2.60.0 and 2.60.6 tags. `Text.xml` differs only by a one-character documentation correction in an XML comment. Parsing both files without comments produces identical interface definitions. Therefore the 2.60.6 XML remains the cited source for the interface definitions, while the proposed Wayland protocol version uses the Ubuntu package identity `2.60.0-1`.

The direct-fetch probe stored all four source files under `/tmp/wf-epic-b/OXY-B003/round-8/`. The output is trimmed to the fetched URLs, sizes, digests, and only diff:

```text
$ probe_dir=/tmp/wf-epic-b/OXY-B003/round-8
$ for tag in 2.60.0 2.60.6; do for file in Text.xml EditableText.xml; do curl -sS -fL --max-time 60 -o "$probe_dir/${file%.xml}-$tag.xml" "https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/$tag/xml/$file"; printf 'url=%s bytes=%s sha256=%s\n' "https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/$tag/xml/$file" "$(wc -c < "$probe_dir/${file%.xml}-$tag.xml")" "$(sha256sum "$probe_dir/${file%.xml}-$tag.xml" | awk '{print $1}')"; done; done
url=https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.0/xml/Text.xml bytes=25182 sha256=602cdb27666912ac0cdf9ac53e5d718e002cd4fe1a37e9a9dc67c71f2acc4249
url=https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.0/xml/EditableText.xml bytes=4303 sha256=2ea1b94822f19b0b00c80b918b89833cfb67d1eeef99d69b8421d0e6f40920ff
url=https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/Text.xml bytes=25183 sha256=5c2d5049d2e427d630ca1ae288d0abe321f39c683336cb8a1373f41c4414d614
url=https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/EditableText.xml bytes=4303 sha256=2ea1b94822f19b0b00c80b918b89833cfb67d1eeef99d69b8421d0e6f40920ff
Text.xml identical=0
--- Text-2.60.0.xml
+++ Text-2.60.6.xml
@@ -85,7 +85,7 @@
-        If @granularity is ATSPI_TEXT_GRANULARITY_PARAGRAPH the returned strin
+        If @granularity is ATSPI_TEXT_GRANULARITY_PARAGRAPH the returned string
EditableText.xml identical=1
exit=0
```

The Jina transport fetched the official Ubuntu package page. Its response is not a fixture and has no digest:

```text
$ url='https://packages.ubuntu.com/resolute/at-spi2-core'; transport="https://r.jina.ai/$url"; http_code=$(curl -sS -fL --max-time 60 -w '%{http_code}' -o /tmp/wf-epic-b/OXY-B003/round-8/ubuntu-at-spi2-core.txt "$transport"); printf 'url=%s transport=jina http=%s bytes=%s\n' "$url" "$http_code" "$(wc -c < /tmp/wf-epic-b/OXY-B003/round-8/ubuntu-at-spi2-core.txt)"
url=https://packages.ubuntu.com/resolute/at-spi2-core transport=jina http=200 bytes=2868
$ grep -F -m 1 '2.60.0-1' /tmp/wf-epic-b/OXY-B003/round-8/ubuntu-at-spi2-core.txt
## Package: at-spi2-core (2.60.0-1)
```

The first parser command was unavailable because this host's base shell has no `python3`. The succeeding command uses the pinned Nix Python package and excludes XML comments before comparison:

```text
$ python3 - /tmp/wf-epic-b/OXY-B003/round-8
/nix/store/90nk33c4fkyg4x4dfk5cykqiryf2nlqq-bash-interactive-5.3p15/bin/bash: línea 3: python3: orden no encontrada
exit=127
$ nix shell nixpkgs#python3 -c python3 - /tmp/wf-epic-b/OXY-B003/round-8
Text.xml parsed_interface_definitions_identical=1
EditableText.xml parsed_interface_definitions_identical=1
exit=0
```

Round 8 reran the strict source-record SHA-256 validator after adding the 2.60.0 digests. It accepts every `sha256=` token only when it has exactly 64 lowercase hexadecimal characters:

````text
$ report=.constitution/spikes/SPK-B003.md
$ digest_block=$(mktemp /tmp/wf-epic-b/OXY-B003/round-8/source-record.XXXXXX)
$ tokens=$(mktemp /tmp/wf-epic-b/OXY-B003/round-8/source-record-tokens.XXXXXX)
$ sed -n '/^presentation-time-v1 sha256=/,/^```/p' "$report" > "$digest_block"
$ occurrence_count=$(grep -oF 'sha256=' "$digest_block" | wc -l | tr -d ' ')
$ grep -oE 'sha256=[^[:space:]|]+' "$digest_block" > "$tokens" || true
$ token_count=$(wc -l < "$tokens" | tr -d ' ')
$ invalid_count=$(grep -cvE '^sha256=[0-9a-f]{64}$' "$tokens" || true)
$ printf 'all_sha256_occurrences=%s complete_tokens=%s invalid_tokens=%s\n' "$occurrence_count" "$token_count" "$invalid_count"
all_sha256_occurrences=18 complete_tokens=18 invalid_tokens=0
$ test "$occurrence_count" -eq "$token_count" && test "$invalid_count" -eq 0
$ printf 'exit=%s\n' "$?"
exit=0
````

## Options and trade-offs

- **Option A:** Freeze the selected Ubuntu compositor session, package manifest, and protocol registry only after P1 records compositor/version evidence and the visible-surface transcript. This is required for a reference baseline, but it is not complete in this spike.
- **Option B:** Use a prospective Linux DRM `drm:drm_vblank_event` trace as the opportunity-meter design. P4 must establish Ubuntu kernel package and source or patch identity, live tracepoint schema and call-site semantics, trace access, pipe-index-to-CRTC-ID-to-output association, a `mono` trace clock, P4C's predeclared `U_95` characterization, the reviewed Stage 3 maximum-uncertainty decision, and no candidate callback or IPC path before it becomes a meter.
- **Option C:** Keep candidate behavior and environment-dependent rows as gating KUs. This prevents the reference distribution label, protocol advertisement, `GdkFrameClock`, or per-commit feedback from becoming unearned qualification evidence.

## Recommendation

- **Chosen option:** Use a mix of A, B, and C. Freeze the source-level core, shell, scale, text-input, clipboard, presentation, and AT-SPI interface definitions from cited upstream sources. Apply the shared Ubuntu `libgtk-4-1` `4.22.2+ds-1ubuntu1` and `at-spi2-core` `2.60.0-1` package identities to both Linux sessions while retaining the `gtk4` crate `v4_20` API-binding ceiling as a separate constraint. Use Orca and the AT-SPI 2.60.6 source definitions with documented Unicode-scalar offsets for the common accessibility baseline. Require the Option B DRM trace design for P4 only after its Ubuntu kernel identity, live format, and call-site semantics are evidenced, and retain Option C for every unproven reference-session and candidate-specific row, including the calibration acceptance bound until reviewed Stage 3 freezes it before candidate measurements.
- **Why it fits:** The corrected source selection contains 101 operations, including foundational registry and synchronization, creation, cursor, keyboard, touch, output, candidate geometry, clipboard source, offer, selection, and client-issued release and destroy operations needed for per-view lifecycle and teardown. It excludes operating-system drag-and-drop because the PRD does not make it P0. The mechanical parser makes 97 operations the deterministic P1 transcript and retains four compositor-specific events as separate KUs. The factory-propagation pass raises `wl_data_device_manager` from local floor 1 to binding floor 2 because its interface-typed `new_id` `get_data_device` child requires version 2. Presentation version 1 has acknowledgement and output-association operations. Version 2 only changes the variable-refresh `refresh` obligation, which the harness does not consume. Retaining KUs for server advertisement, behavior, and logical-index representation prevents source facts from becoming compositor or candidate claims. The DRM trace's independence from candidate callback streams remains unresolved until P4 proves it, along with Ubuntu kernel identity, live schema and call-site semantics, trace access, pipe-to-output attribution, P4C's predeclared clock-characterization calculation and the reviewed Stage 3 maximum-uncertainty decision made before candidate measurements. `CON-FRM-001` remains the separately applied measured interval-error gate.
- **Rejected options:** Reject a nominal refresh-rate timer, a harness-owned `wl_surface.frame` callback as an independent meter, `wp_presentation` feedback as an opportunity source, a protocol-global list as compositor behavior, an unspecified assistive technology, a scalar-to-logical equivalence assumption, a global IME index unit for every operation, and a candidate map inferred from GTK documentation.
- **Sensitive-field rule:** Set `GtkInputPurpose` to `PASSWORD` or `PIN` as applicable and set `GtkInputHints.PRIVATE`. Continue to provide only protocol-required redacted surrounding context and never emit raw text to diagnostics. GTK describes the hint as a request, not a privacy guarantee; P2 and P3 must verify the redaction path.

### Spec edits required

Stage 3 can make the following exact edits without changing product capabilities or architecture boundaries:

- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.protocols`: OXY-D001 must apply this replacement only after the preservation step commits all 10 regular fixture files, their 10 adjacent source-record sidecars, and replaces every exact `<to-be-computed-by-preservation-step>` token with the streamed SHA-256 of the associated fixture. The template is syntactically valid JSON, but it is deliberately not schema-valid or digest-validator-valid until that step completes. Each eventual `kk` entry has a non-null version and local evidence; P1 retains server advertisement and behavior as a separate gate.
- `qualification/fixtures/external-contracts/wayland/` -> preservation inputs: OXY-D001 must commit a `FIXTURE.source.json` sidecar for every capture-map fixture, using the exact Table 4 fields, including SPDX-only `license` values for source files and `LicenseRef-page-copyright-notice`, `licenseNote`, and `licenseUrl` for the two Ubuntu pages; each sidecar's `sha256` must equal its sibling regular fixture's streamed SHA-256. This spike is the sole authority for these 10 Wayland sidecars; SPK-B004 is the sole authority for the 12 X11 sidecars.

The Wayland GTK row uses the same Ubuntu `libgtk-4-1` `4.22.2+ds-1ubuntu1` package identity as SPK-B004's X11 row because both reference sessions use that package. This is not a change to the `gtk4` crate `v4_20` API-binding ceiling in `stack.md`. OXY-D001 must reconcile both platform rows to that package identity while preserving their separately captured fixture paths.

The Wayland AT-SPI row uses the same Ubuntu `at-spi2-core` `2.60.0-1` package identity as SPK-B004's X11 row. The Wayland fixtures retain the directly fetched 2.60.6 XML as the cited source for interface definitions. OXY-D001 must reconcile the AT-SPI protocol row and reference-configuration package identity across both environments while preserving their separately captured fixture paths.

The capture map covers all 29 evidence objects in the template, including the `wl_display` and `wl_registry` floors emitted by the source-of-record derivation. The URLs are canonical upstream sources and were fetched successfully for this correction. No Jina-proxied body digest is used: the preservation step must write the direct canonical-source response to the named repository fixture and hash that regular file. The Ubuntu AT-SPI package capture uses the same `<to-be-computed-by-preservation-step>` SHA-256 placeholder as every other evidence object.

| Evidence objects | Fixture path | Canonical source URL |
| :-- | :-- | :-- |
| `GTK` (1) | `qualification/fixtures/external-contracts/wayland/s01-ubuntu-libgtk-4-1.html` | https://packages.ubuntu.com/resolute/libgtk-4-1 |
| `wl_display`, `wl_registry`, `wl_compositor`, `wl_surface`, `wl_callback`, `wl_seat`, `wl_pointer`, `wl_keyboard`, `wl_touch`, `wl_output`, `wl_data_device_manager`, `wl_data_device`, `wl_data_offer`, `wl_data_source` (14) | `qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml` | https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml |
| `xdg_wm_base`, `xdg_surface`, `xdg_toplevel` (3) | `qualification/fixtures/external-contracts/wayland/s03-xdg-shell.xml` | https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/xdg-shell/xdg-shell.xml |
| `wp_viewporter`, `wp_viewport` (2) | `qualification/fixtures/external-contracts/wayland/s04-viewporter.xml` | https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/viewporter/viewporter.xml |
| `wp_fractional_scale_manager_v1`, `wp_fractional_scale_v1` (2) | `qualification/fixtures/external-contracts/wayland/s05-fractional-scale-v1.xml` | https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/staging/fractional-scale/fractional-scale-v1.xml |
| `zwp_text_input_manager_v3`, `zwp_text_input_v3` (2) | `qualification/fixtures/external-contracts/wayland/s06-text-input-unstable-v3.xml` | https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/unstable/text-input/text-input-unstable-v3.xml |
| `wp_presentation`, `wp_presentation_feedback` (2) | `qualification/fixtures/external-contracts/wayland/s07-presentation-time-v1.xml` | https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml |
| `AT-SPI` `Text.xml` (1) | `qualification/fixtures/external-contracts/wayland/s08-atspi-2.60.6-text.xml` | https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/fff349553d16f99de3258bdeed7b8a663469b84b/xml/Text.xml |
| `AT-SPI` `EditableText.xml` (1) | `qualification/fixtures/external-contracts/wayland/s09-atspi-2.60.6-editable-text.xml` | https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/fff349553d16f99de3258bdeed7b8a663469b84b/xml/EditableText.xml |
| Ubuntu `at-spi2-core` package identity (1) | `qualification/fixtures/external-contracts/wayland/s10-ubuntu-at-spi2-core.html`; `sha256`: `<to-be-computed-by-preservation-step>` | https://packages.ubuntu.com/resolute/at-spi2-core |

### Round-11 correction: source-record sidecars and license attribution

The external-contract convention records the upstream identity and terms in a sibling source record, not only in a contract evidence object. For every row in the existing capture map, OXY-D001 must write `FIXTURE.source.json` beside `FIXTURE`. This follows the existing `protocol.source.json` convention in `qualification/schemas/external/`; the source record is metadata, and the fixture remains the regular file that `platform-contracts.json` hashes. The sidecar is not an evidence object, so the capture map remains 29 objects and no existing row changes its number.

Every sidecar must use this exact authoritative-record shape. Its `sha256` value must be the same 64-character streamed SHA-256 written for its sibling fixture in the contract template; it is not the hash of the sidecar.

```json
{
  "kind": "authoritative",
  "repository": "https://github.com/wayland-mirror/wayland",
  "commit": "1ab6b693b16e1d9734496fe60c8a6ed277e4dec3",
  "path": "protocol/wayland.xml",
  "retrievalUrl": "https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml",
  "license": "MIT",
  "licenseSource": {
    "path": "COPYING",
    "commit": "1ab6b693b16e1d9734496fe60c8a6ed277e4dec3"
  },
  "version": null,
  "sha256": "7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610"
}
```

Table 4 supplies the exact non-digest values for every sidecar. `null` is intentional where the authoritative package-page publisher provides no source commit or where a commit-pinned source has no separately fetched release version. The two AT-SPI retrieval URLs use the immutable commit resolved from the fetched `2.60.6` tag; the preserved probe confirms that the tag and commit bytes are identical. This spike is the sole authority for these 10 Wayland sidecars. [SPK-B004](SPK-B004.md) is the sole authority for the 12 X11 sidecars; it must not be modified through this Wayland table.

| Fixture and required sidecar | `repository`; `commit`; `path`; `retrievalUrl`; `version` | `license`; `licenseSource.path`; `licenseSource.commit`; `licenseNote`; `licenseUrl` |
| :-- | :-- | :-- |
| `s01-ubuntu-libgtk-4-1.html`; `s01-ubuntu-libgtk-4-1.html.source.json` | `https://packages.ubuntu.com`; `null`; `resolute/libgtk-4-1`; `https://packages.ubuntu.com/resolute/libgtk-4-1`; `4.22.2+ds-1ubuntu1` | `LicenseRef-page-copyright-notice`; `resolute/libgtk-4-1`; `null`; `Canonical's page copyright notice applies to this captured HTML page; it is not an SPDX license grant.`; `https://ubuntu.com/legal/terms-and-policies/intellectual-property-policy` |
| `s02-wayland-core.xml`; `s02-wayland-core.xml.source.json` | `https://github.com/wayland-mirror/wayland`; `1ab6b693b16e1d9734496fe60c8a6ed277e4dec3`; `protocol/wayland.xml`; `https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml`; `null` | `MIT`; `COPYING`; `1ab6b693b16e1d9734496fe60c8a6ed277e4dec3`; `null`; `null` |
| `s03-xdg-shell.xml`; `s03-xdg-shell.xml.source.json` | `https://github.com/wayland-mirror/wayland-protocols`; `d5aed4e4903a77aefaef03359d1ffdc0d5093456`; `stable/xdg-shell/xdg-shell.xml`; `https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/xdg-shell/xdg-shell.xml`; `null` | `MIT`; `COPYING`; `d5aed4e4903a77aefaef03359d1ffdc0d5093456`; `null`; `null` |
| `s04-viewporter.xml`; `s04-viewporter.xml.source.json` | `https://github.com/wayland-mirror/wayland-protocols`; `d5aed4e4903a77aefaef03359d1ffdc0d5093456`; `stable/viewporter/viewporter.xml`; `https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/viewporter/viewporter.xml`; `null` | `MIT`; `COPYING`; `d5aed4e4903a77aefaef03359d1ffdc0d5093456`; `null`; `null` |
| `s05-fractional-scale-v1.xml`; `s05-fractional-scale-v1.xml.source.json` | `https://github.com/wayland-mirror/wayland-protocols`; `d5aed4e4903a77aefaef03359d1ffdc0d5093456`; `staging/fractional-scale/fractional-scale-v1.xml`; `https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/staging/fractional-scale/fractional-scale-v1.xml`; `null` | `MIT`; `COPYING`; `d5aed4e4903a77aefaef03359d1ffdc0d5093456`; `null`; `null` |
| `s06-text-input-unstable-v3.xml`; `s06-text-input-unstable-v3.xml.source.json` | `https://github.com/wayland-mirror/wayland-protocols`; `d5aed4e4903a77aefaef03359d1ffdc0d5093456`; `unstable/text-input/text-input-unstable-v3.xml`; `https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/unstable/text-input/text-input-unstable-v3.xml`; `null` | `MIT`; `COPYING`; `d5aed4e4903a77aefaef03359d1ffdc0d5093456`; `null`; `null` |
| `s07-presentation-time-v1.xml`; `s07-presentation-time-v1.xml.source.json` | `https://github.com/wayland-mirror/wayland-protocols`; `37a1560cf6981a11d44dd200d9409d09b4f0074e`; `stable/presentation-time/presentation-time.xml`; `https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml`; `null` | `MIT`; `COPYING`; `37a1560cf6981a11d44dd200d9409d09b4f0074e`; `null`; `null` |
| `s08-atspi-2.60.6-text.xml`; `s08-atspi-2.60.6-text.xml.source.json` | `https://gitlab.gnome.org/GNOME/at-spi2-core`; `fff349553d16f99de3258bdeed7b8a663469b84b`; `xml/Text.xml`; `https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/fff349553d16f99de3258bdeed7b8a663469b84b/xml/Text.xml`; `2.60.6` | `LGPL-2.1-or-later`; `COPYING`; `fff349553d16f99de3258bdeed7b8a663469b84b`; `null`; `null` |
| `s09-atspi-2.60.6-editable-text.xml`; `s09-atspi-2.60.6-editable-text.xml.source.json` | `https://gitlab.gnome.org/GNOME/at-spi2-core`; `fff349553d16f99de3258bdeed7b8a663469b84b`; `xml/EditableText.xml`; `https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/fff349553d16f99de3258bdeed7b8a663469b84b/xml/EditableText.xml`; `2.60.6` | `LGPL-2.1-or-later`; `COPYING`; `fff349553d16f99de3258bdeed7b8a663469b84b`; `null`; `null` |
| `s10-ubuntu-at-spi2-core.html`; `s10-ubuntu-at-spi2-core.html.source.json` | `https://packages.ubuntu.com`; `null`; `resolute/at-spi2-core`; `https://packages.ubuntu.com/resolute/at-spi2-core`; `2.60.0-1` | `LicenseRef-page-copyright-notice`; `resolute/at-spi2-core`; `null`; `Canonical's page copyright notice applies to this captured HTML page; it is not an SPDX license grant.`; `https://ubuntu.com/legal/terms-and-policies/intellectual-property-policy` |

Every Table 4 source-file sidecar has an SPDX-only `license`: Wayland and wayland-protocols use `MIT` from their cited `COPYING` files, and AT-SPI uses `LGPL-2.1-or-later` from `COPYING` at the cited commit. The two Ubuntu package pages are not SPDX-licensed source files, so they use `LicenseRef-page-copyright-notice`, their own repository-relative page path as `licenseSource.path` because the notice is inline, the required `licenseNote`, and the required `licenseUrl`. The capture map has no GTK source fixture: `s01` captures an Ubuntu page, not GTK source. The cited GTK header's inline notice is `LGPL-2.0-or-later`, which controls a future capture of that file despite the project-level Meson metadata in the preserved round-11 probe. A future GTK source sidecar must use the captured file itself as `licenseSource.path` when its notice is inline, or `COPYING` when the captured file relies on that repository-level notice. The capture map has no X.Org fixture. A future X.Org capture must determine `MIT` or `X11` from the captured file's notice and use that single SPDX expression, never a combined expression. The relevant fetched license sources are the [GTK `gtkenums.h` header](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.20.4/gtk/gtkenums.h), [GTK 4.20.4 COPYING](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.20.4/COPYING), [Wayland core COPYING](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/COPYING), [Wayland protocols COPYING at d5aed4e](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/COPYING), [Wayland protocols COPYING at 37a1560](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/COPYING), [AT-SPI 2.60.6 COPYING](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/COPYING), the [Ubuntu libgtk-4-1 package page](https://packages.ubuntu.com/resolute/libgtk-4-1), the [Ubuntu at-spi2-core package page](https://packages.ubuntu.com/resolute/at-spi2-core), and the [Ubuntu intellectual property policy](https://ubuntu.com/legal/terms-and-policies/intellectual-property-policy).

The direct-source license probe used only files under `/tmp/wf-epic-b/OXY-B003/round-11/`. The body digests below are direct-fetch probe results; no Jina-proxied body is hashed.

```text
$ license-source-probe
GTK Meson metadata url=https://gitlab.gnome.org/GNOME/gtk/-/raw/4.20.4/meson.build bytes=36681 sha256=846f70529404fb90429e60d930795a644a38c824f1ec36c943957e5a35e026bf excerpt=license: 'LGPL-2.1-or-later'
Wayland core COPYING url=https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/COPYING bytes=1340 sha256=6eefcb023622a463168a5c20add95fd24a38c7482622a9254a23b99b7c153061 excerpt=Permission is hereby granted, free of charge, to any person obtaining a copy of this software
Wayland protocols COPYING d5aed4e url=https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/COPYING bytes=1502 sha256=f1a2b233e8a9a71c40f4aa885be08a0842ac85bb8588703c1dd7e6e6502e3124 excerpt=Permission is hereby granted, free of charge, to any person obtaining a copy of this software
Wayland protocols COPYING 37a1560 url=https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/COPYING bytes=1502 sha256=f1a2b233e8a9a71c40f4aa885be08a0842ac85bb8588703c1dd7e6e6502e3124 excerpt=Permission is hereby granted, free of charge, to any person obtaining a copy of this software
AT-SPI COPYING url=https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/COPYING bytes=26530 sha256=dc626520dcd53a22f727af3ee42c770e56c97a64fe3adb063799d8ab032fe551 excerpt=GNU LESSER GENERAL PUBLIC LICENSE
X.Org license url=https://www.x.org/releases/X11R7.7/doc/xorg-docs/License.html bytes=35725 sha256=0a12f5da2ff694fb69b10024c7dc07ad969a41ea5a695ea24767836fb27bd948 excerpt=Permission is hereby granted, free of charge, to any person obtaining a copy of this software
Ubuntu package footer url=https://packages.ubuntu.com/resolute/libgtk-4-1 bytes=23257 sha256=ff9a602bc4daff9f97460bd7032359579f4f7a990f6cc532792d38f445fa8d42 excerpt=Content Copyright 2026 Canonical Ltd.; see Ubuntu legal terms
Ubuntu legal terms url=https://www.ubuntu.com/legal bytes=203395 sha256=ae5e61dff1cc32d8f76f2d460ad4e16a07fe3fa4737947edfc9cd2fdcfc3c7cd excerpt=Ubuntu legal terms and policies
AT-SPI tag 2.60.6 resolved commit=fff349553d16f99de3258bdeed7b8a663469b84b
AT-SPI tag and resolved-commit content identical=1
exit=0
```

### Round-13 correction: sidecar license fields and `new_id` terminology

Table 4 is the sole source of sidecar metadata for the 10 Wayland fixtures. It uses SPDX expressions only for source files and uses `LicenseRef-page-copyright-notice`, `licenseNote`, and `licenseUrl` only for the two Ubuntu package-page HTML captures. For an inline page notice, `licenseSource.path` is the page's repository-relative path. This avoids treating the Ubuntu HTML as GTK or AT-SPI source code. SPK-B004 is the sole source of sidecar metadata for the 12 X11 fixtures.

The pinned Wayland core XML distinguishes `wl_data_device_manager.get_data_device`, whose `new_id` argument has an `interface="wl_data_device"` attribute, from `wl_registry.bind`, whose `new_id` argument has no `interface` attribute. The derivation considers only the former, interface-typed form when there is no separate `version` argument. It does not apply factory version inheritance to bare or untyped `new_id` arguments. The updated docstrings name that criterion without changing the parser logic.

The updated parser ran against the preserved six pinned XML files. Its complete output is byte-identical to the preceding round-9 output because only docstrings changed.

```text
$ nix shell nixpkgs#python3 -c python3 /tmp/wf-epic-b/OXY-B003/round-13/derive-wayland-floors.py /tmp/wf-epic-b/OXY-B003/round-13/sources > /tmp/wf-epic-b/OXY-B003/round-13/derive-wayland-floors.out
$ cmp -s /tmp/wf-epic-b/OXY-B003/round-13/derive-wayland-floors.out /tmp/wf-epic-b/OXY-B003/round-9/derive-wayland-floors.out; printf 'derive_output_byte_identical_to_round_9=%s\n' "$?"
derive_output_byte_identical_to_round_9=0
$ sha256sum /tmp/wf-epic-b/OXY-B003/round-13/derive-wayland-floors.out
05b99ca632bb390d46625b47d78bc44d0954297b6a7710620fdb5cde52d3f4f2  /tmp/wf-epic-b/OXY-B003/round-13/derive-wayland-floors.out
exit=0
```

The round-13 license fetches succeeded through the Jina transport. The canonical [GTK `COPYING`](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.20.4/COPYING) starts with GNU Library General Public License version 2; the cited `gtkenums.h` header supplies the file-level LGPL v2-or-later notice. The canonical [AT-SPI `COPYING`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/COPYING) starts with GNU Lesser General Public License version 2.1. The canonical Ubuntu package pages identify `libgtk-4-1` `4.22.2+ds-1ubuntu1` and `at-spi2-core` `2.60.0-1`, and the canonical [Ubuntu intellectual property policy](https://ubuntu.com/legal/terms-and-policies/intellectual-property-policy) is the required page-notice URL. Jina-proxied pages are not hashed.

```text
canonical=https://gitlab.gnome.org/GNOME/gtk/-/raw/4.20.4/COPYING response=200 bytes=25479 excerpt="GNU LIBRARY GENERAL PUBLIC LICENSE Version 2"
canonical=https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/COPYING response=200 bytes=26635 excerpt="GNU LESSER GENERAL PUBLIC LICENSE Version 2.1"
canonical=https://packages.ubuntu.com/resolute/libgtk-4-1 response=200 bytes=7822 excerpt="Package: libgtk-4-1 (4.22.2+ds-1ubuntu1)"
canonical=https://packages.ubuntu.com/resolute/at-spi2-core response=200 bytes=2868 excerpt="Package: at-spi2-core (2.60.0-1)"
canonical=https://ubuntu.com/legal/terms-and-policies/intellectual-property-policy response=200 bytes=52654 excerpt="Intellectual property rights policy"
exit=0
```

To preserve the fixtures, OXY-D001 must create `qualification/fixtures/external-contracts/wayland/`, fetch each canonical URL with `curl --fail --location --max-time 60 --output FIXTURE URL`, require `test -f FIXTURE` and `test ! -L FIXTURE`, calculate `sha256sum FIXTURE`, write the corresponding `FIXTURE.source.json` with the Table 4 values and that same digest, replace every matching token in the template with that output, commit the regular files and sidecars, and run the contract digest validator. The validator then requires repository-relative paths, regular files, and streamed SHA-256 matches; OXY-D001 must additionally reject a missing, non-regular, or digest-mismatched source-record sidecar before accepting the preservation step.

The following trimmed Round-7 transport output records successful source fetches only. The Jina responses are not fixture bytes and their digests are intentionally absent. The source-package descriptor rechecks SPK-B004's package-audit input; it is not an additional object in this 29-object template. The round-9 Jina fetch of the new Ubuntu AT-SPI capture is preserved in the factory-propagation correction.

```text
$ fetch_jina ubuntu-libgtk-4-1.html https://packages.ubuntu.com/resolute/libgtk-4-1; fetch_jina gtk4-source.dsc https://launchpad.net/ubuntu/+archive/primary/+sourcefiles/gtk4/4.22.2+ds-1ubuntu1/gtk4_4.22.2+ds-1ubuntu1.dsc; fetch_jina gtk-gtkenums.h https://gitlab.gnome.org/GNOME/gtk/-/raw/4.20.4/gtk/gtkenums.h
ubuntu-libgtk-4-1.html http=200 bytes=7822 canonical=https://packages.ubuntu.com/resolute/libgtk-4-1
gtk4-source.dsc http=200 bytes=4501 canonical=https://launchpad.net/ubuntu/+archive/primary/+sourcefiles/gtk4/4.22.2+ds-1ubuntu1/gtk4_4.22.2+ds-1ubuntu1.dsc
gtk-gtkenums.h http=200 bytes=71988 canonical=https://gitlab.gnome.org/GNOME/gtk/-/raw/4.20.4/gtk/gtkenums.h
$ fetch_raw_github wayland.xml https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml; fetch_raw_github xdg-shell.xml https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/xdg-shell/xdg-shell.xml; fetch_raw_github viewporter.xml https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/viewporter/viewporter.xml
wayland.xml http=200 bytes=151742 canonical=https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml
xdg-shell.xml http=200 bytes=61089 canonical=https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/xdg-shell/xdg-shell.xml
viewporter.xml http=200 bytes=8133 canonical=https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/viewporter/viewporter.xml
$ fetch_raw_github fractional-scale-v1.xml https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/staging/fractional-scale/fractional-scale-v1.xml; fetch_raw_github text-input-unstable-v3.xml https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/unstable/text-input/text-input-unstable-v3.xml; fetch_raw_github presentation-time-v1.xml https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml
fractional-scale-v1.xml http=200 bytes=4636 canonical=https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/staging/fractional-scale/fractional-scale-v1.xml
text-input-unstable-v3.xml http=200 bytes=21491 canonical=https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/unstable/text-input/text-input-unstable-v3.xml
presentation-time-v1.xml http=200 bytes=12642 canonical=https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml
$ fetch_jina Text.xml https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/Text.xml; fetch_jina EditableText.xml https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/EditableText.xml
Text.xml http=200 bytes=25293 canonical=https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/Text.xml
EditableText.xml http=200 bytes=4421 canonical=https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/EditableText.xml
```

```json
[
  {
    "name": "GTK",
    "version": "4.22.2+ds-1ubuntu1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s01-ubuntu-libgtk-4-1.html",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wl_display",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wl_registry",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wl_compositor",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wl_surface",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wl_callback",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wl_seat",
    "version": "5",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wl_pointer",
    "version": "5",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wl_keyboard",
    "version": "4",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wl_touch",
    "version": "3",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wl_output",
    "version": "3",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wl_data_device_manager",
    "version": "2",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wl_data_device",
    "version": "2",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wl_data_offer",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wl_data_source",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s02-wayland-core.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "xdg_wm_base",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s03-xdg-shell.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "xdg_surface",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s03-xdg-shell.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "xdg_toplevel",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s03-xdg-shell.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wp_viewporter",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s04-viewporter.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wp_viewport",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s04-viewporter.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wp_fractional_scale_manager_v1",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s05-fractional-scale-v1.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wp_fractional_scale_v1",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s05-fractional-scale-v1.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "zwp_text_input_manager_v3",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s06-text-input-unstable-v3.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "zwp_text_input_v3",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s06-text-input-unstable-v3.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wp_presentation",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s07-presentation-time-v1.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "wp_presentation_feedback",
    "version": "1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s07-presentation-time-v1.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  },
  {
    "name": "AT-SPI",
    "version": "2.60.0-1",
    "status": "kk",
    "evidence": [
      {
        "path": "qualification/fixtures/external-contracts/wayland/s08-atspi-2.60.6-text.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      },
      {
        "path": "qualification/fixtures/external-contracts/wayland/s09-atspi-2.60.6-editable-text.xml",
        "sha256": "<to-be-computed-by-preservation-step>"
      },
      {
        "path": "qualification/fixtures/external-contracts/wayland/s10-ubuntu-at-spi2-core.html",
        "sha256": "<to-be-computed-by-preservation-step>"
      }
    ]
  }
]
```

The round-10 inline template validator verifies that the revised template contains 29 evidence objects, each with a local path and the exact preservation placeholder. OXY-D001 must rerun the repository fixture and digest validator after preserving all 10 regular files.

```text
$ nix shell nixpkgs#python3 -c python3 /tmp/wf-epic-b/OXY-B003/round-10/validate-wayland-protocol-template.py
template_evidence_objects=29
template_remote_or_absolute_paths=0
quoted_sha256_values=29 allowed_placeholders=29 malformed_quoted_values=0
exit=0
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
  "P1 must prove the selected Ubuntu session advertises every required global, creates every required non-global interface at its frozen floor, and emits all 97 members of the mechanically regenerated deterministic P1 transcript checklist. P1E separately retains wl_registry.global_remove, wl_touch.cancel, xdg_wm_base.ping, and wp_presentation_feedback.discarded until a documented compositor-specific controlled procedure produces each event.",
  "P3B must freeze the TextIndex::Logical representation and establish scalar-to-logical pairs bound to an immutable TextLayoutId.",
  "P3 must lock the Ubuntu reference package at-spi2-core 2.60.0-1 or a separately reviewed replacement and establish scalar text, caret, selection, and EditableText-operation behavior.",
  "P4C must characterize exactly 10,000 trace-marker pairs using U_95 = 1.96 * SE(d_bar) + r_trace / 2 + r_clock / 2 + w_max / 2; it cannot close P4. Before candidate measurements, reviewed Stage 3 must freeze the numeric maximum acceptable U_95, causal-matching algorithm version, and matching-window width. P4 must then record the Ubuntu kernel image package identity, source or patch identity, live drm_vblank_event format and SHA-256, and source evidence for matching tracepoint schema and pipe call-site semantics; then establish trace access, pipe-index-to-CRTC-object-ID-to-connector attribution, unambiguous surface-output pairing, a mono trace clock, the predeclared calibration calculation, and trace independence from each candidate. CON-FRM-001's 10% rule applies only to the measured interval-error result."
]
```

- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.ime.numericNegotiation`: replace the value with `"Use the writable Gtk.InputPurpose and Gtk.InputHints properties for each focus generation; no project-defined numeric handshake exists. Surrounding cursor and anchor positions use UTF-8 bytes. P2 must establish every other GtkIMContext operation unit."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.interactiveOpportunitySource`: replace the value with `"GdkFrameClock is a host wakeup only; each allocation must prove output-associated display-synchronized scheduling in P4."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.independentMeterSource`: replace the value with `"Prospective only: Linux DRM drm_vblank_event captured by trace-cmd with the mono trace clock. P4 must record the Ubuntu kernel image package identity, source or patch identity, live tracepoint format and SHA-256, and source evidence that its schema and call site establish pipe semantics. It must map each established pipe index through drmModeGetResources to a UAPI CRTC object ID and active connector, prove an unambiguous surface-output pairing and callback or IPC independence, and preserve a clock-calibration and causal-matching uncertainty budget. Before any candidate measurement, reviewed Stage 3 must record the numeric maximum acceptable U_95, causal-matching algorithm version, matching-window width, and review reference. P4C records U_95 = 1.96 * SE(d_bar) + r_trace / 2 + r_clock / 2 + w_max / 2 but cannot close this gate. Apply CON-FRM-001's 10% limit only to the measured interval-error result after meter and matcher qualification."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.presentationFeedback`: replace the value with `"wp_presentation v1 feedback for per-commit acknowledgement and main-output association only; never an independent presentation-opportunity meter."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.perDisplayAssociation`: replace the value with `"Track each wl_surface enter/leave output set and begin a display epoch on every set change. Use wp_presentation_feedback.sync_output only to label a submitted frame's main output."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.accessibilityMaps`, `recoveryBaseline`, `allocations.focused`, and `allocations.integrated`: retain every `"ku-gating"` status and `null` path/digest until P3F, P3I, P5F, P5I, P6F, and P6I produce the named immutable artifacts.
- `.constitution/tech-spec/stack.md` -> `Platform qualification pins` -> `Wayland` row -> `Reference configuration` column: replace the cell with `"x86-64 Ubuntu 26.04 LTS Wayland session with libgtk-4-1 4.22.2+ds-1ubuntu1 and at-spi2-core 2.60.0-1; the gtk4 crate v4_20 API-binding ceiling is separate; the Ubuntu compositor/session package manifest and selected-session advertisement of the frozen Wayland floors remain gating KUs; P1 must record package versions, manifest digest, registry, and a visible-surface transcript covering all 97 members of the mechanically regenerated deterministic P1 checklist, retaining the four named event gates separately, and including client-issued release and destroy operations."`.
- `.constitution/tech-spec/contracts/qualification-lock.json` -> `preImplementationKnownUnknowns` and `gatingKnownUnknowns`: add these eleven new strings as plain, unique array members: `"wayland-ubuntu-compositor-session-package-lock"`, `"wayland-frozen-protocol-reference-session-transcript"`, `"wayland-ime-operation-unit-transcript"`, `"wayland-atspi-scalar-logical-representation"`, `"wayland-atspi-text-caret-selection-editable-transcript"`, `"wayland-orca-atspi-maps-for-both-allocations"`, `"wayland-drm-vblank-kernel-identity-live-schema-callsite"`, `"wayland-drm-vblank-calibration-uncertainty-budget"`, `"wayland-service-routing-for-both-allocations"`, `"wayland-recovery-injection-for-both-allocations"`, and `"wayland-drm-vblank-calibration-acceptance-bound"`.
- `.constitution/tech-spec/contracts/qualification-lock.json` -> `preImplementationKnownUnknowns` and `gatingKnownUnknowns`: `"wayland-drm-vblank-calibration-acceptance-bound"` is one of the eleven new strings above and has its binding row below. Do not create a keyed entry or replace a value in either array; the acceptance text belongs in the schema-defined string field `environments.wayland.timing.independentMeterSource` in the replacement above.
- `crates/oxyflut-qualification/src/readiness.rs` -> `KNOWN_UNKNOWN_BINDINGS`: add the following exact `KnownUnknownBinding` rows so every added `preImplementationKnownUnknowns` value maps to a required lock field, evidence path, and upstream owner:

| `known_unknown` | `required_field` | `evidence_path` | `upstream_owner` |
| :-- | :-- | :-- | :-- |
| `wayland-ubuntu-compositor-session-package-lock` | `referenceEnvironments.wayland-linux-x86_64.systemPackageLockDigest` | `None` | `OXY-C004` |
| `wayland-frozen-protocol-reference-session-transcript` | `measurementPolicy.platformContracts` | `Some(".constitution/tech-spec/contracts/platform-contracts.json")` | `OXY-C004` |
| `wayland-ime-operation-unit-transcript` | `measurementPolicy.platformContracts` | `Some(".constitution/tech-spec/contracts/platform-contracts.json")` | `OXY-C004` |
| `wayland-atspi-scalar-logical-representation` | `measurementPolicy.platformContracts` | `Some(".constitution/tech-spec/contracts/platform-contracts.json")` | `OXY-C004` |
| `wayland-atspi-text-caret-selection-editable-transcript` | `measurementPolicy.platformContracts` | `Some(".constitution/tech-spec/contracts/platform-contracts.json")` | `OXY-C004` |
| `wayland-orca-atspi-maps-for-both-allocations` | `measurementPolicy.platformContracts` | `Some(".constitution/tech-spec/contracts/platform-contracts.json")` | `OXY-C004` |
| `wayland-drm-vblank-kernel-identity-live-schema-callsite` | `measurementPolicy.platformContracts` | `Some(".constitution/tech-spec/contracts/platform-contracts.json")` | `OXY-C004` |
| `wayland-drm-vblank-calibration-uncertainty-budget` | `measurementPolicy.platformContracts` | `Some(".constitution/tech-spec/contracts/platform-contracts.json")` | `OXY-C004` |
| `wayland-service-routing-for-both-allocations` | `measurementPolicy.platformContracts` | `Some(".constitution/tech-spec/contracts/platform-contracts.json")` | `OXY-C004` |
| `wayland-recovery-injection-for-both-allocations` | `measurementPolicy.platformContracts` | `Some(".constitution/tech-spec/contracts/platform-contracts.json")` | `OXY-C004` |
| `wayland-drm-vblank-calibration-acceptance-bound` | `measurementPolicy.platformContracts` | `Some(".constitution/tech-spec/contracts/platform-contracts.json")` | `OXY-C004` |

- `xtask/src/commands/lock_tests.rs` -> `committed_candidate_gate_is_valid_but_open_with_the_exact_ku_set`: update the expected `known_unknowns` vector to its exact 24-element set by retaining the existing 13 values, adding the eleven new strings listed in the preceding binding table, and removing no value. Keep the vector in the report's existing lexicographic order: append `wayland-atspi-scalar-logical-representation`, `wayland-atspi-text-caret-selection-editable-transcript`, `wayland-drm-vblank-calibration-acceptance-bound`, `wayland-drm-vblank-calibration-uncertainty-budget`, `wayland-drm-vblank-kernel-identity-live-schema-callsite`, `wayland-frozen-protocol-reference-session-transcript`, `wayland-ime-operation-unit-transcript`, `wayland-orca-atspi-maps-for-both-allocations`, `wayland-recovery-injection-for-both-allocations`, `wayland-service-routing-for-both-allocations`, and `wayland-ubuntu-compositor-session-package-lock` after `security-patch-rehearsal`.
- `.constitution/tech-spec/adrs/ADR-0005-platform-hosts.md` -> `Consequences`: add `"Wayland qualification freezes source-level core, shell, scale, text-input, clipboard, and wp_presentation floors, including all client-issued P0 teardown operations. P1 must prove the selected session advertises them and records the mechanically derived 97-member deterministic P1 transcript and retains the four named nondeterministic event gates separately. wp_presentation v1 supplies per-commit acknowledgement and output association only, not the independent presentation-opportunity meter. P4 evaluates a Linux DRM drm_vblank_event trace only after Ubuntu kernel package and source or patch identity, live-format and call-site-semantic evidence, access, pipe-index-to-CRTC-object-ID-to-output attribution, the trace_marker-bracketed U_95 characterization, the reviewed Stage 3 maximum-uncertainty decision made before candidate measurements, and callback or IPC independence pass. CON-FRM-001's 10% interval-error limit is applied only to qualified measured matching results."`.

## Downstream impact

- **ADRs to write or update:** Stage 3 updates `ADR-0005-platform-hosts.md` with the `wp_presentation` boundary. `ADR-0006-execution-domains.md` requires no change because the report does not alter its queue or ownership boundary.
- **Tickets unblocked in `tasks/active/`:** `OXY-D001` can consume the documented protocol and conversion mechanics, but it remains blocked from qualification measurements by P1 through P6.
- **Tickets to add or split:** Add P1 through P6 as bounded Wayland evidence tasks if the Stage 4 plan does not already schedule equivalent probes.
- **Remaining gates:** The 12 KU (gating) rows in Table 1 retain the Wayland environment as `ku-gating`. Neither allocation is eligible for scoring until they close.
