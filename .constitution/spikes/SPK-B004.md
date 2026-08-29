# Spike report: OXY-B004 X11 qualification baseline

## Time box

- Budget: 1 focused day.
- Clock start / stop: 2026-08-28T17:06:40Z / 2026-08-28T17:26:23Z.
- Round-5 correction clock start / stop: 2026-08-28T21:17:23Z / 2026-08-28T21:26:36Z.
- Round-8 correction clock start / stop: 2026-08-29T00:30:52Z / 2026-08-29T00:37:32Z.
- Round-9 correction clock start / stop: 2026-08-29T01:03:00Z / 2026-08-29T01:11:13Z.

## Question

- Decision this spike produces: Freeze the cited Ubuntu package inputs and documented protocol mechanics, use `GtkIMMulticontext` with the IBus GTK module and Orca as the reference Linux assistive technology, and retain the unproven allocation, native-Xorg, timing, routing, and recovery claims as gating KUs.

Table 1 answers each part of the decision.

| Question row | Answer | Status | Citations or preserved evidence | Next bounded probe for a gating KU |
| :-- | :-- | :-- | :-- | :-- |
| Reference distribution and package floor | Freeze Ubuntu 26.04 LTS package inputs: `xserver-xorg-core` `2:21.1.22-1ubuntu1`, `libgtk-4-1` `4.22.2+ds-1ubuntu1`, `at-spi2-core` `2.60.0-1`, `ibus-gtk4` `1.5.34~rc2-1`, and Orca `50.1.2-1ubuntu1`. This is an exact qualification input, not a claim about a live session. | KK | [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/); [xserver-xorg-core package](https://packages.ubuntu.com/resolute/xserver-xorg-core); [GTK package](https://packages.ubuntu.com/resolute/libgtk-4-1); [AT-SPI package](https://packages.ubuntu.com/resolute/at-spi2-core); [IBus GTK package](https://packages.ubuntu.com/resolute/ibus-gtk4); [Orca package](https://packages.ubuntu.com/resolute/orca). | Not applicable. |
| Native X server and extension floor | The host probe reports Xwayland, not an Ubuntu native Xorg session. It reports X.Org 24.1.13, `Present` 1.4, `SYNC` 3.1, XInput 2.4, and RandR 1.6. Those facts demonstrate only nonreference extension-query mechanics. The Resolute package page identifies a package version, not the native server release or extension floor, so neither is frozen. | KU (gating) | Preserved X11 probe output; [Ubuntu xserver-xorg-core package](https://packages.ubuntu.com/resolute/xserver-xorg-core); [Present protocol specification](https://cgit.freedesktop.org/xorg/proto/presentproto/plain/presentproto.txt). | On an Ubuntu 26.04 native Xorg session with the frozen `xserver-xorg-core` package, run `xdpyinfo -display "$DISPLAY"`, `xrandr --version`, and a 2-window XCB query for Present, RandR, Sync, and XInput versions. Preserve the server vendor and release, package version, negotiated versions, each window's RandR output and CRTC, and a Present submit/completion transcript. Pass only if the records bind to that native server and package list; then propose the observed server and extension floor. |
| GTK input-context mechanism and index units | GTK documents preedit start, change, end, commit, surrounding-text retrieval, deletion, focus, reset, cursor rectangle, and preedit attributes. `set_surrounding` receives UTF-8 and a byte cursor index, while `delete_surrounding` uses character offsets and counts. `GtkInputPurpose` supplies declarative password and PIN purposes; this report makes no numeric-handoff claim. | KK | [GtkIMContext](https://docs.gtk.org/gtk4/class.IMContext.html); [set_surrounding](https://docs.gtk.org/gtk4/method.IMContext.set_surrounding.html); [delete_surrounding](https://docs.gtk.org/gtk4/method.IMContext.delete_surrounding.html); [GtkInputPurpose](https://docs.gtk.org/gtk4/enum.InputPurpose.html). | Not applicable. |
| Complete IBus GTK input-method transcript for each allocation | Use `GtkIMMulticontext` and the frozen `ibus-gtk4` module as the reference IME path. No native-Xorg transcript demonstrates the required preedit, commit, surrounding, replacement, candidate-geometry, focus-transfer, reset, secure-field, or CJK behavior for either candidate. | KU (gating) | [GtkIMMulticontext](https://docs.gtk.org/gtk4/class.IMMulticontext.html); [IBus GTK package](https://packages.ubuntu.com/resolute/ibus-gtk4). | On the frozen Ubuntu native Xorg session, run a noncandidate GTK 4.22 harness with `GTK_IM_MODULE=ibus` and a selected IBus composition engine. Capture ordered signals and callbacks for every vector listed in `environments.x11.ime.testVectors`, including UTF-8 byte cursor values, character deletion values, rectangle coordinates, and redacted secure-field records. Repeat the same transcript through each candidate's adapter after its source identity exists. |
| Linux assistive technology and base AT-SPI surface | Freeze the Orca `50.1.2-1ubuntu1` package identity and `at-spi2-core` `2.60.0-1` service package. The checked GNOME Orca repository has no `ORCA_50_1_2` tag, and the corresponding tagged documentation URL returns HTTP 404. The fetched Orca stable page says that Orca works with AT-SPI, but that page is mutable. The pinned AT-SPI sources establish the base D-Bus text surface, but no version-pinned Orca documentation establishes the selected Orca-to-AT-SPI statement. | KU (gating) | [Ubuntu Orca package](https://packages.ubuntu.com/resolute/orca); [Orca documentation (mutable; fetched 2026-08-29T01:05:29Z)](https://help.gnome.org/users/orca/stable/introduction.html.en); [GNOME Orca Git repository](https://gitlab.gnome.org/GNOME/orca.git); [AT-SPI Text interface](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.Text.html); [Ubuntu desktop documentation source at commit `786057d7a1ba1212d06c16880820681b40bb24d3`](https://raw.githubusercontent.com/canonical/ubuntu-desktop-documentation/786057d7a1ba1212d06c16880820681b40bb24d3/docs/reference/accessibility/dbus/org.a11y.atspi.Text.md). | Query `git ls-remote --tags https://gitlab.gnome.org/GNOME/orca.git 'refs/tags/ORCA_50*'`. If an Orca 50.1.2 tag appears, fetch `help/C/introduction.page` at that exact tag and preserve the resolved commit, HTTP 200 result, and AT-SPI excerpt. If no tag appears, fetch the Ubuntu `orca` source package for `50.1.2-1ubuntu1`, identify its signed source input and the matching documentation file, and record whether it supplies an immutable equivalent. |
| Focused allocation forward and reverse AT-SPI map | No focused candidate exists to map every semantics role, property, relation, state, value, geometry, text range, event, and `Action` invocation to the selected AT-SPI interfaces. | KU (gating) | The AT-SPI references establish the target interface family but not an Oxyflut implementation: [AT-SPI Text interface](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.Text.html); [Ubuntu desktop documentation source at commit `786057d7a1ba1212d06c16880820681b40bb24d3`](https://raw.githubusercontent.com/canonical/ubuntu-desktop-documentation/786057d7a1ba1212d06c16880820681b40bb24d3/docs/reference/accessibility/dbus/org.a11y.atspi.Text.md). | Build the focused candidate's two-window semantics fixture on the frozen native Xorg session with Orca and an AT-SPI D-Bus recorder. Produce a versioned map artifact and hash that enumerates forward events and reverse actions, routes each action by XID and view generation, and reports a stale-target error after teardown. |
| Integrated allocation forward and reverse AT-SPI map | The integrated fork commit is not pinned, and its inherited GTK and AT-SPI path has not been enumerated. A complete map cannot be inferred from Flutter or GTK documentation. | KU (gating) | [AT-SPI Text interface](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.Text.html); the qualification lock records no integrated fork commit. | After Stage 3 records the integrated fork commit, enumerate its Linux embedder sources and run the same two-window Orca and AT-SPI recorder fixture as the focused allocation. Preserve the inherited interface inventory, map artifact, hash, and reverse-action routing result. |
| Unicode-scalar AT-SPI offset fixtures | The audited Ubuntu `gtk4` source package used to build `libgtk-4-1` dispatches `GetCharacterAtOffset` first through `GtkAccessibleText` when `GTK_IS_ACCESSIBLE_TEXT` and only otherwise through the `GtkEditable` fallback. The generic path passes `offset, offset + 1` to `gtk_accessible_text_get_contents`; its GTK 4.22.2 contract names both bounds as character ranges and its `CharacterCount` helper uses `g_utf8_strlen`. The editable fallback separately bounds with `g_utf8_strlen` and explicitly converts through `g_utf8_offset_to_pointer` before `g_utf8_get_char`. GLib documents both functions in character units. The source-package audit confirms that `gtk/a11y/gtkatspitext.c`, `gtk/gtkaccessibletext.c`, and `gtk/gtkaccessibletext.h` byte-match the Ubuntu original tarball and that no header in the 25 applied Debian patches targets any of those paths. The AT-SPI XML separately states that `CharacterCount` can differ from returned UTF-8 byte count. The preserved fixture probe tests logical, grapheme, UTF-8, and UTF-16 boundaries in ASCII, multibyte, combining, and bidirectional text, and proves a scalar boundary can occur inside the combining-sequence grapheme. | KK | [AT-SPI Text D-Bus XML 2.60.0](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.0/xml/Text.xml); [GTK 4.22.2 AT-SPI text bridge source](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/a11y/gtkatspitext.c); [GTK 4.22.2 accessible text implementation](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.c); [GTK 4.22.2 GtkAccessibleText API documentation](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.h); [GLib UTF-8 string length](https://docs.gtk.org/glib/func.utf8_strlen.html); [GLib UTF-8 offset conversion](https://docs.gtk.org/glib/func.utf8_offset_to_pointer.html); [Ubuntu GTK source package](https://launchpad.net/ubuntu/+source/gtk4/4.22.2+ds-1ubuntu1); preserved Ubuntu source-package audit and Unicode probe output. | Not applicable to the audited provider-unit KK. Candidate compliance and a live D-Bus return remain gating KUs in each allocation's native-Xorg forward and reverse map probe. |
| X Present feedback role | Require Present protocol 1.0 or later for presentation feedback. The specification says `PresentCompleteNotify` reports completion of a pending `PresentPixmap` request. It is acknowledgement feedback only. It is not an independent presentation-opportunity source, and no schedule may derive opportunities from it. | KK | [Present protocol specification](https://cgit.freedesktop.org/xorg/proto/presentproto/plain/presentproto.txt). | Not applicable. |
| Independent timing meter and per-output association | `GdkFrameClock` can use a timer and is the host scheduler, so it cannot measure itself. The DRM API can request vblank events and can report monotonic timestamps and CRTC IDs when its capabilities are present, but this spike did not establish permission, output-to-CRTC mapping, calibration, or independence on the Ubuntu native Xorg session. | KU (gating) | [GdkFrameClock](https://docs.gtk.org/gdk4/class.FrameClock.html); [Linux DRM user-space API](https://docs.kernel.org/gpu/drm-uapi.html); preserved host DRM-node probe. | On the frozen Ubuntu native Xorg session, open a harness-owned DRM card FD distinct from both candidates. Require `DRM_CAP_TIMESTAMP_MONOTONIC=1` and `DRM_CAP_CRTC_IN_VBLANK_EVENT=1`, correlate each RandR output to its CRTC, and record 10 seconds of `DRM_EVENT_VBLANK`, candidate schedule callbacks, and Present completions for two windows. The expected result is one calibrated, per-CRTC opportunity stream whose collection consumes neither candidate callback stream. Retain this KU if the session denies the FD or cannot establish the mapping. |
| Focused allocation interface inventory and service routing | The documented GTK, X11, AT-SPI, Present, and Vulkan error surfaces identify possible inputs, not a focused adapter. No multi-view trace proves that every request and callback carries the owning XID and view generation through the reentrancy barrier. | KU (gating) | [GtkIMContext](https://docs.gtk.org/gtk4/class.IMContext.html); [Present protocol specification](https://cgit.freedesktop.org/xorg/proto/presentproto/plain/presentproto.txt); [Vulkan registry](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/20a9e5892e2aab7b9776b16a238b10fc8133090a/xml/vk.xml). | Run a focused two-window fixture with distinct XIDs and generations. Issue IME, clipboard, AT-SPI action, resize, Present, and teardown events while one window is destroyed and recreated. Preserve a trace that shows the correct owner for every accepted event and a structured stale-generation rejection for every late event. |
| Integrated allocation interface inventory and service routing | The integrated fork and its exact inherited Linux embedder interfaces are absent. Therefore no callback ownership, C ABI normalization, or multi-view service-routing claim is established. | KU (gating) | The qualification lock has no integrated fork commit; [Present protocol specification](https://cgit.freedesktop.org/xorg/proto/presentproto/plain/presentproto.txt) only describes one protocol input. | After the fork is pinned, enumerate every inherited X11, GTK, IME, AT-SPI, clipboard, scheduling, lifecycle, and graphics callback. Run the same distinct-XID and generation trace required for the focused allocation, then preserve its inventory and trace hash. |
| Focused allocation recovery injection | Vulkan defines `VK_ERROR_DEVICE_LOST` and permits `VK_ERROR_SURFACE_LOST_KHR` from surface and presentation operations, but neither source nor a probe establishes a focused adapter fault-injection seam or recovery result. | KU (gating) | [Vulkan registry](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/20a9e5892e2aab7b9776b16a238b10fc8133090a/xml/vk.xml). | Add a qualification-only focused-adapter fault script after its source identity is pinned. Force surface-loss and device-lost results at acquire and present boundaries, then force resize and RandR topology change. Preserve injection point, monotonic fault time, recreation attempts, acknowledged frame, resource release, and terminal-error records against CON-REC-001 through CON-REC-007. |
| Integrated allocation recovery injection | No integrated fork source, lifecycle inventory, or graphics recovery seam is pinned. Vulkan error definitions do not establish that the inherited embedder can receive deterministic faults. | KU (gating) | [Vulkan registry](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/20a9e5892e2aab7b9776b16a238b10fc8133090a/xml/vk.xml). | After the integrated fork is pinned, identify its Vulkan dispatch and lifecycle boundaries. Run the same scripted surface-loss, device-lost, resize, and RandR topology tests as the focused allocation, and preserve the C ABI ingress, recovery trace, attempt count, and terminal-error result. |

## Context and objective

- Triggering upstream file or section: `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.x11`.
- Target: Replace package, documented-interface, Unicode-fixture, and Present-feedback claims with KK evidence where sources permit. Retain candidate-dependent and native-session-dependent claims, plus the unavailable version-pinned Orca documentation claim, as smaller executable blockers.
- Archetype / surface: Library and SDK with system X11 desktop integration.
- Evidence terms: KK is a cited official source or preserved host probe. KU (gating) names a bounded unanswered qualification question. No claim in this report depends on architectural plausibility or on extension presence alone.

## Codebase baseline

- Stage 3 pins Ubuntu 26.04 LTS and GTK 4.20 API bindings but leaves the X11 package lock and X server behavior open.
- The official Ubuntu archive supplies the exact package versions in table 1. It does not substitute for a package-snapshot digest, a native Xorg session capture, or a server behavior probe.
- The Ubuntu GTK package is 4.22.2, which satisfies the existing GTK 4.20 API-binding ceiling. The report does not change the product surface or architecture boundary.
- `GtkIMContext` uses two different index conventions: UTF-8 byte offsets for surrounding-text cursor information and character offsets for deletion. The adapter must convert both through checked strong index types before state mutation.
- For the audited Ubuntu GTK 4.22.2 source, the AT-SPI bridge selects the `GtkAccessibleText` path before the `GtkEditable` fallback. The generic path requests a one-character range and counts full text with `g_utf8_strlen`; only the fallback explicitly converts with `g_utf8_offset_to_pointer`. AT-SPI returns UTF-8 text and its XML warns that returned byte length can exceed its character offsets.
- The Ubuntu `orca` package page establishes the frozen package identity, but the checked upstream repository has no Orca 50.1.2 source tag. The only preserved statement that Orca works with AT-SPI is a fetch-timestamped mutable documentation excerpt. That relationship stays a gating KU until a pinned upstream or Ubuntu-source equivalent is captured.

## Options and trade-offs

- Option A: Freeze the Ubuntu package inputs and documented interface contracts, then require a native-Xorg capture for server behavior.
- Option B: Use a harness-owned DRM vblank stream only after a native-Xorg probe proves separate FD access, monotonic timestamps, CRTC attribution, calibration, and independence from both candidate callbacks.
- Option C: Retain an affected item as a gating KU when a candidate, the native session, or a reproducible injection seam is absent.

## Recommendation

Table 2 selects an option for every decision area.

| Decision area | Chosen option | Recommendation and justification |
| :-- | :-- | :-- |
| Reference packages and protocol contracts | A | Freeze the exact Ubuntu package inputs in table 1. Use GTK 4.22.2, `GtkIMMulticontext` with `ibus-gtk4`, `at-spi2-core`, and Orca. Use Present 1.0 or later only for completion feedback. These are cited package and protocol facts. |
| Native X server and extension behavior | C | Keep the environment gate open until the frozen Ubuntu native Xorg session records its server identity, negotiated extension versions, two-window output association, and Present behavior. The Xwayland probe is deliberately nonreference. |
| AT-SPI maps and scalar conversion | A plus C | Freeze the audited Ubuntu GTK 4.22.2 dispatch and Unicode-scalar unit contract, the fixtures, and the Orca selection. Retain each allocation's complete forward and reverse map and live native-Xorg AT-SPI observation as separate gates because no candidate implementation exists. |
| Independent timing | C | Do not use `GdkFrameClock`, nominal timers, or Present completion as an independent source. Consider B only after the exact DRM sidecar probe passes. |
| Service routing and recovery | C | Require per-XID and generation traces plus allocation-specific scripted recovery faults. Do not infer either property from GTK, Vulkan result names, or the architecture documents. |

- Why it fits: The mix freezes reproducible upstream inputs without converting availability into behavior. It preserves candidate symmetry and keeps opportunity measurement independent from schedule and completion callbacks.
- Rejected options: Do not use extension availability as proof of server semantics. Do not use `PresentCompleteNotify` as an opportunity source. Do not select an unspecified assistive technology or IME module. Do not infer a default window, display, or view for a service request.

## Probe outputs

### Round-9 frozen Ubuntu capture and documentation pinning

The following reader-proxy captures fetched the three new canonical publisher pages. The excerpts are normalized reader text, not fixtures, so this report records no full-body digest for them. The future preservation step fetches each canonical URL into the named fixture and computes the `<to-be-computed-by-preservation-step>` digest.

```text
$ curl -sS -fL --max-time 60 https://r.jina.ai/https://packages.ubuntu.com/resolute/xserver-xorg-core -o /tmp/wf-epic-b/OXY-B004/round-9/xserver-xorg-core.txt
exit=0 bytes=5289
Title: Ubuntu - Details of package xserver-xorg-core in resolute
URL Source: https://packages.ubuntu.com/resolute/xserver-xorg-core
## Package: xserver-xorg-core (2:21.1.22-1ubuntu1)

$ curl -sS -fL --max-time 60 https://r.jina.ai/https://packages.ubuntu.com/resolute/orca -o /tmp/wf-epic-b/OXY-B004/round-9/orca-package.txt
exit=0 bytes=3893
Title: Ubuntu - Details of package orca in resolute
URL Source: https://packages.ubuntu.com/resolute/orca
## Package: orca (50.1.2-1ubuntu1)

$ curl -sS -fL --max-time 60 https://r.jina.ai/https://documentation.ubuntu.com/release-notes/26.04/ -o /tmp/wf-epic-b/OXY-B004/round-9/ubuntu-26.04-release-notes.txt
exit=0 bytes=3980
Title: Ubuntu 26.04 LTS release notes
URL Source: https://documentation.ubuntu.com/release-notes/26.04/
These release notes cover new features and changes in Ubuntu 26.04 LTS (Resolute Raccoon).

$ status=$(curl -sS -L --max-time 60 -o /tmp/wf-epic-b/OXY-B004/round-9/ubuntu-26.04-release-notes-direct.html -w '%{http_code}' https://documentation.ubuntu.com/release-notes/26.04/); printf 'http=%s bytes=%s\n' "$status" "$(wc -c < /tmp/wf-epic-b/OXY-B004/round-9/ubuntu-26.04-release-notes-direct.html)"
http=200 bytes=31057
$ rg -n -i -C 1 'copyright' /tmp/wf-epic-b/OXY-B004/round-9/ubuntu-26.04-release-notes-direct.html | tail -3
447:    <div class="copyright">
448-        &copy; 2026
```

The xserver and Orca captures state their exact package versions. The release-notes capture establishes the Ubuntu 26.04 LTS (Resolute Raccoon) distribution identity and its direct page exposes a copyright notice.

The following tag check establishes that the requested Orca 50.1.2 source-documentation tag and file are unavailable from the checked official Git remote. The mutable fallback excerpt has an explicit fetch timestamp and cannot close the Orca-to-AT-SPI gate.

```text
$ git ls-remote --tags https://gitlab.gnome.org/GNOME/orca.git "refs/tags/ORCA_50_1_2" "refs/tags/ORCA_50_1_2^{}"
exit=0
(no matching ref)
$ curl -sS -L --max-time 60 -o /tmp/wf-epic-b/OXY-B004/round-9/orca-ORCA_50_1_2-introduction.page -w '%{http_code}' https://gitlab.gnome.org/GNOME/orca/-/raw/ORCA_50_1_2/help/C/introduction.page
404
$ wc -c < /tmp/wf-epic-b/OXY-B004/round-9/orca-ORCA_50_1_2-introduction.page
2613
$ fetched=$(date -u '+%Y-%m-%dT%H:%M:%SZ'); curl -sS -fL --max-time 60 https://r.jina.ai/https://help.gnome.org/users/orca/stable/introduction.html.en -o /tmp/wf-epic-b/OXY-B004/round-9/orca-stable-introduction-mutable.txt; printf 'fetched=%s bytes=%s\n' "$fetched" "$(wc -c < /tmp/wf-epic-b/OXY-B004/round-9/orca-stable-introduction-mutable.txt)"
fetched=2026-08-29T01:05:29Z bytes=1942
Orca is a free, open source, flexible, and extensible screen reader that provides access to the graphical desktop via speech and refreshable braille.
Orca works with applications and toolkits that support the Assistive Technology Service Provider Interface (AT-SPI), which is the primary assistive technology infrastructure for Linux and Solaris.
```

The previously cited mutable Ubuntu documentation URL is replaced by its fetched source file at an immutable Git commit. The direct raw GitHub response is byte-stable, so the reported SHA-256 is a full-body digest.

```text
$ git ls-remote https://github.com/canonical/ubuntu-desktop-documentation.git HEAD
786057d7a1ba1212d06c16880820681b40bb24d3	HEAD
$ git -C /tmp/wf-epic-b/OXY-B004/round-9/ubuntu-desktop-documentation rev-parse HEAD
786057d7a1ba1212d06c16880820681b40bb24d3
$ curl -sS -fL --max-time 60 https://raw.githubusercontent.com/canonical/ubuntu-desktop-documentation/786057d7a1ba1212d06c16880820681b40bb24d3/docs/reference/accessibility/dbus/org.a11y.atspi.Text.md -o /tmp/wf-epic-b/OXY-B004/round-9/ubuntu-atspi-text-786057d.md
raw-http=200 bytes=5818
$ sha256sum /tmp/wf-epic-b/OXY-B004/round-9/ubuntu-atspi-text-786057d.md
5e338dcb4e788351346fcbe99a7050df1ed5d32165f6cf313c40c65ef23139d5  /tmp/wf-epic-b/OXY-B004/round-9/ubuntu-atspi-text-786057d.md
$ rg -n -C 1 'org.a11y.atspi.Text|GetText' /tmp/wf-epic-b/OXY-B004/round-9/ubuntu-atspi-text-786057d.md | head -12
1:# org.a11y.atspi.Text
37:### org.a11y.atspi.Text.GetText
39:    GetText (
40:      IN startOffset i,
41:      IN endOffset i,
42:      OUT unnamed_arg2 s
```

The following historical probes ran on this host. The host uses Wayland with Xwayland at `DISPLAY=:0`; these results are nonreference and establish extension-query mechanics only. Their unpinned `nixpkgs` invocations are retained verbatim as historical transcripts, not prescribed commands.

```text
$ nix shell nixpkgs#xorg.xdpyinfo -c xdpyinfo -display :0
exit=0
name of display:    :0
vendor string:    The X.Org Foundation
vendor release number:    12401013
X.Org version: 24.1.13
    Present
    RANDR
    SYNC
    XInputExtension
    XWAYLAND

$ nix shell nixpkgs#xorg.xrandr -c xrandr --version
exit=0
WARNING: running xrandr against an Xwayland server. See the xrandr man page for details.
xrandr program version       1.5.4
Server reports RandR version 1.6

$ nix shell nixpkgs#xdpyinfo -c sh -c 'xdpyinfo -display :0 -ext Present -ext RANDR -ext SYNC -ext XInputExtension'
SYNC version 3.1 opcode: 134, base event: 83, base error: 134
XInputExtension version 2.4 opcode: 131, base event: 66, base error: 129
Present version 1.4 opcode: 146
exit=0
```

The required re-run commands use the repository's CI and `devenv.lock` nixpkgs revision `56c02bc00adcf003215cc4bd996d6efaf4cff188` and the replacement root attributes. The compatibility `xorg` package set is deprecated; its `xorg.xdpyinfo` alias resolves only with a warning. Do not use the alias for new probes.

```text
nix shell github:NixOS/nixpkgs/56c02bc00adcf003215cc4bd996d6efaf4cff188#xdpyinfo --command xdpyinfo -display "$DISPLAY"
nix shell github:NixOS/nixpkgs/56c02bc00adcf003215cc4bd996d6efaf4cff188#xrandr --command xrandr --version
nix shell github:NixOS/nixpkgs/56c02bc00adcf003215cc4bd996d6efaf4cff188#xdpyinfo --command sh -c 'xdpyinfo -display "$DISPLAY" -ext Present -ext RANDR -ext SYNC -ext XInputExtension'
```

The following Round-8 package-resolution check ran on this host:

```text
$ nix eval --raw github:NixOS/nixpkgs/56c02bc00adcf003215cc4bd996d6efaf4cff188#xdpyinfo.name
xdpyinfo-1.4.0
$ nix shell github:NixOS/nixpkgs/56c02bc00adcf003215cc4bd996d6efaf4cff188#xdpyinfo --command true
exit=0
$ nix eval --raw github:NixOS/nixpkgs/56c02bc00adcf003215cc4bd996d6efaf4cff188#xrandr.name
xrandr-1.5.4
$ nix shell github:NixOS/nixpkgs/56c02bc00adcf003215cc4bd996d6efaf4cff188#xrandr --command true
exit=0
$ nix shell github:NixOS/nixpkgs/56c02bc00adcf003215cc4bd996d6efaf4cff188#xorg.xdpyinfo --command true
evaluation warning: The xorg package set has been deprecated, 'xorg.xdpyinfo' has been renamed to 'xdpyinfo'
exit=0
```

The Round-8 Vulkan registry fetch resolved at the required commit. Its full-body SHA-256 is stable because this is a direct raw GitHub response, not a Jina-proxied page.

```text
$ curl -sS -fL --max-time 60 -o /tmp/wf-epic-b/OXY-B004/round-8/vk.xml https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/20a9e5892e2aab7b9776b16a238b10fc8133090a/xml/vk.xml
exit=0 bytes=3305568
$ sha256sum /tmp/wf-epic-b/OXY-B004/round-8/vk.xml
3ff4984b841932e04eebeb4ce2a6613ebd37c00ffb2e96549785b2c5d7da9e1d  /tmp/wf-epic-b/OXY-B004/round-8/vk.xml
```

The host exposes DRM nodes, but the probe did not open or measure one because this Wayland and Xwayland session cannot qualify a native Ubuntu Xorg observer.

```text
$ ls -l /dev/dri
total 0
drwxr-xr-x  2 root root         80 ago 26 20:54 by-path
crw-rw----+ 1 root video  226,   1 ago 28 10:05 card1
crw-rw-rw-  1 root render 226, 128 ago 26 20:54 renderD128
$ printf 'DISPLAY=%s XDG_SESSION_TYPE=%s\n' "$DISPLAY" "$XDG_SESSION_TYPE"
DISPLAY=:0 XDG_SESSION_TYPE=wayland
```

### AT-SPI text-offset unit evidence

The upstream AT-SPI Text XML says, "CharacterCount: The total number of characters in a text object. This may differ from the number of bytes that would be returned if the text is fetched in cases where characters are expressed using multiple bytes." It also says, "Returns: a text string containing characters from @startOffset to @endOffset-1, inclusive, encoded as UTF-8." [AT-SPI Text D-Bus XML 2.60.0](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.0/xml/Text.xml) defines non-byte protocol character offsets.

The following host probe fetched the immutable GNOME `2.60.0` XML and recorded its SHA-256:

```text
$ curl -sS -fL --max-time 60 -o Text-2.60.0.xml https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.0/xml/Text.xml
exit=0 bytes=25182
$ sha256sum Text-2.60.0.xml
602cdb27666912ac0cdf9ac53e5d718e002cd4fe1a37e9a9dc67c71f2acc4249  Text-2.60.0.xml
```

#### GTK dispatch and unit contract

The [GTK 4.22.2 text bridge source](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/a11y/gtkatspitext.c) selects the generic `GtkAccessibleText` vtable before the editable vtable. It contains the following selector:

```c
if (GTK_IS_ACCESSIBLE_TEXT (accessible))
  return &accessible_text_vtable;
else if (GTK_IS_EDITABLE (accessible))
  return &editable_vtable;
```

For the primary `GtkAccessibleText` path, the [GTK 4.22.2 text bridge source](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/a11y/gtkatspitext.c) requests the one-character range and decodes the returned UTF-8 sequence:

```c
GBytes *text = gtk_accessible_text_get_contents (accessible_text, offset, offset + 1);

if (text != NULL)
  {
    const char *str = g_bytes_get_data (text, NULL);
    if (g_utf8_strlen (str, -1) > 0)
      ch = g_utf8_get_char (str);
  }
```

The GTK 4.22.2 `GtkAccessibleText` API documentation defines `get_contents` with "@start: the beginning of the range, in characters" and "@end: the end of the range, in characters," and returns the requested slice as UTF-8. Its implementation calculates the character count from the complete `get_contents (self, 0, G_MAXUINT)` result with `len = g_utf8_strlen (str, -1);`. The bridge exports that helper as `CharacterCount`. [GTK 4.22.2 GtkAccessibleText API documentation](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.h) and the [GTK 4.22.2 accessible text implementation](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.c) establish the primary path's character-range and character-count units.

The `GtkEditable` branch is a fallback, not the universal provider implementation. The [GTK 4.22.2 text bridge source](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/a11y/gtkatspitext.c) gives its `GetCharacterAtOffset` branch the separate explicit conversion:

```c
text = gtk_editable_get_text (GTK_EDITABLE (widget));
if (0 <= offset && offset < g_utf8_strlen (text, -1))
  ch = g_utf8_get_char (g_utf8_offset_to_pointer (text, offset));
```

[GLib UTF-8 string length](https://docs.gtk.org/glib/func.utf8_strlen.html) says that `g_utf8_strlen` computes a string length in characters. [GLib UTF-8 offset conversion](https://docs.gtk.org/glib/func.utf8_offset_to_pointer.html) says that `g_utf8_offset_to_pointer` converts an integer character offset to a pointer in the string. The primary path's character-range contract and `g_utf8_strlen` count, plus the fallback's explicit conversion, establish a Unicode-scalar index rather than a UTF-8 byte, UTF-16-unit, or grapheme index for this GTK provider.

#### Ubuntu source-package audit

The [Ubuntu GTK source package](https://launchpad.net/ubuntu/+source/gtk4/4.22.2+ds-1ubuntu1) links the fetched [source descriptor](https://launchpad.net/ubuntu/+archive/primary/+sourcefiles/gtk4/4.22.2+ds-1ubuntu1/gtk4_4.22.2+ds-1ubuntu1.dsc). Its `Checksums-Sha256` stanza names `gtk4_4.22.2+ds.orig.tar.xz` with SHA-256 `b06b9a4a82ed0b8a9260cc739296af1f96b7348d5e6b8d09435ab563910cd33d` and `gtk4_4.22.2+ds-1ubuntu1.debian.tar.xz` with SHA-256 `79d4685fc02fd7bcdb851f4ed7d0afc77becfa1a15b3489dd198df7ad3806e04`. The following preserved audit downloads both source inputs, verifies the checksums, compares all three upstream-tag files against the original tarball, lists the applied patch series, and searches every applied patch header for all three target paths.

```text
$ curl -sS -L --max-time 60 -o gtk4_4.22.2+ds-1ubuntu1.dsc https://launchpad.net/ubuntu/+archive/primary/+sourcefiles/gtk4/4.22.2+ds-1ubuntu1/gtk4_4.22.2+ds-1ubuntu1.dsc
http=200 bytes=4303
$ grep -A 3 "^Checksums-Sha256:" gtk4_4.22.2+ds-1ubuntu1.dsc
Checksums-Sha256:
 b06b9a4a82ed0b8a9260cc739296af1f96b7348d5e6b8d09435ab563910cd33d 17128400 gtk4_4.22.2+ds.orig.tar.xz
 79d4685fc02fd7bcdb851f4ed7d0afc77becfa1a15b3489dd198df7ad3806e04 4011848 gtk4_4.22.2+ds-1ubuntu1.debian.tar.xz
Files:
$ curl -sS -L --max-time 180 -o gtk4_4.22.2+ds.orig.tar.xz http://archive.ubuntu.com/ubuntu/pool/main/g/gtk4/gtk4_4.22.2+ds.orig.tar.xz
http=200 bytes=17128400
$ curl -sS -L --max-time 120 -o gtk4_4.22.2+ds-1ubuntu1.debian.tar.xz http://archive.ubuntu.com/ubuntu/pool/main/g/gtk4/gtk4_4.22.2+ds-1ubuntu1.debian.tar.xz
http=200 bytes=4011848
$ sha256sum gtk4_4.22.2+ds.orig.tar.xz gtk4_4.22.2+ds-1ubuntu1.debian.tar.xz
b06b9a4a82ed0b8a9260cc739296af1f96b7348d5e6b8d09435ab563910cd33d  gtk4_4.22.2+ds.orig.tar.xz
79d4685fc02fd7bcdb851f4ed7d0afc77becfa1a15b3489dd198df7ad3806e04  gtk4_4.22.2+ds-1ubuntu1.debian.tar.xz
$ curl -sS -fL --max-time 60 -o gtkatspitext.c https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/a11y/gtkatspitext.c
exit=0 bytes=37061
$ curl -sS -fL --max-time 60 -o gtkaccessibletext.c https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.c
exit=0 bytes=22443
$ curl -sS -fL --max-time 60 -o gtkaccessibletext.h https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.h
exit=0 bytes=20380
$ tar -tJf gtk4_4.22.2+ds.orig.tar.xz | grep -E "(^|/)gtk/(a11y/gtkatspitext\\.c|gtkaccessibletext\\.(c|h))$"
gtk-4.22.2/gtk/a11y/gtkatspitext.c
gtk-4.22.2/gtk/gtkaccessibletext.c
gtk-4.22.2/gtk/gtkaccessibletext.h
$ tar -xJOf gtk4_4.22.2+ds.orig.tar.xz gtk-4.22.2/gtk/a11y/gtkatspitext.c > orig-gtkatspitext.c; tar -xJOf gtk4_4.22.2+ds.orig.tar.xz gtk-4.22.2/gtk/gtkaccessibletext.c > orig-gtkaccessibletext.c; tar -xJOf gtk4_4.22.2+ds.orig.tar.xz gtk-4.22.2/gtk/gtkaccessibletext.h > orig-gtkaccessibletext.h
$ sha256sum gtkatspitext.c orig-gtkatspitext.c gtkaccessibletext.c orig-gtkaccessibletext.c gtkaccessibletext.h orig-gtkaccessibletext.h
65178d6d816cd7b1d91cdea177949aca249469d1b7b0072221e0fa1bb65b9e66  gtkatspitext.c
65178d6d816cd7b1d91cdea177949aca249469d1b7b0072221e0fa1bb65b9e66  orig-gtkatspitext.c
4efeb6bf88a05d47eb98fe69bc5905c5208343890a980b1fe89d7aaf99ff6e6e  gtkaccessibletext.c
4efeb6bf88a05d47eb98fe69bc5905c5208343890a980b1fe89d7aaf99ff6e6e  orig-gtkaccessibletext.c
a9dbead627c1b5abffed3aaaa2a6b2e49c7350c100e8349dffddc46ece79faab  gtkaccessibletext.h
a9dbead627c1b5abffed3aaaa2a6b2e49c7350c100e8349dffddc46ece79faab  orig-gtkaccessibletext.h
$ cmp -s gtkatspitext.c orig-gtkatspitext.c && cmp -s gtkaccessibletext.c orig-gtkaccessibletext.c && cmp -s gtkaccessibletext.h orig-gtkaccessibletext.h && printf "all three upstream-tag files match the source orig tarball\n"
all three upstream-tag files match the source orig tarball
```

```text
$ tar -xJf gtk4_4.22.2+ds-1ubuntu1.debian.tar.xz -C .
$ grep -vE "^[[:space:]]*(#|$)" debian/patches/series
debian/reftest_compare_surfaces-Report-how-much-the-images-diffe.patch
insttests/Revert-build-Drop-the-install-tests-option.patch
insttests/Revert-testsuite-Remove-leftover-test.in-files.patch
workarounds/reftests-Allow-minor-differences-to-be-tolerated.patch
workarounds/Disable-inscription-markup.ui-reftest.patch
workarounds/tests-Mark-gltexture-as-expected-to-fail-on-big-endian-ma.patch
workarounds/tests-Allow-longer-for-a-dialog-to-open.patch
workarounds/nodeparser-Adjust-test-for-pango-1.52.0.patch
workarounds/testsuite-skip-color-mix.patch
workarounds/scaling-test-Skip-floating-point-pixel-formats-with-Cairo.patch
workarounds/nodeparser-Mark-failing-tests-on-s390x.patch
x11-scale/gdk-x11-check-surface-scale-on-input-region-opaque-region.patch
x11-scale/gdk-x11-update-correct-shadow-size-according-to-window-si.patch
x11-scale/gdk-x11-update-cursor-size-on-the-extent-after-scale-chan.patch
x11-touch/xi2-Start-drag-grab-with-pointer-only-event-mask.patch
x11-touch/xi2-Do-not-discard-emulated-pointer-events-during-drag-an.patch
x11-touch/xi2-Expose-a-logical-touch-device.patch
printdialog-Keep-GTask-alive-for-portal-repsonse.patch
testsuite-Don-t-build-waylandsocket-test-if-Wayland-is-di.patch
print-Fix-listing-printers-with-synchronous-backends.patch
accessibility-Fix-regression.patch
gtkapplication-wayland-Add-a-missing-NULL-check-when-forg.patch
gskvulkanimage-fix-building-on-32-bit.patch
Revert-testutils-Warn-if-setting-up-language-didn-t-work.patch
application-wayland-Add-NULL-check-on-gtk_accessible_get_.patch
$ matches=0; while IFS= read -r patch; do if grep -HnE "^(---|\\+\\+\\+|diff --git ).*(gtk/a11y/gtkatspitext\\.c|gtk/gtkaccessibletext\\.(c|h))" "debian/patches/$patch"; then matches=$((matches + 1)); fi; done < <(grep -vE "^[[:space:]]*(#|$)" debian/patches/series); printf "applied-patches=%s target-path-header-match-files=%s\n" "$(grep -cvE "^[[:space:]]*(#|$)" debian/patches/series)" "$matches"
applied-patches=25 target-path-header-match-files=0
```

The original tarball contains byte-identical copies of the three audited upstream-tag files: `gtk/a11y/gtkatspitext.c`, `gtk/gtkaccessibletext.c`, and `gtk/gtkaccessibletext.h`. No applied Debian patch header targets any of those paths. This audit makes the cited dispatch and unit evidence applicable to the audited Ubuntu `gtk4` source package used to build `libgtk-4-1` `4.22.2+ds-1ubuntu1`.

The host session has no advertised AT-SPI or Orca service, so this spike did not make a real AT-SPI call. Candidate compliance and runtime D-Bus behavior remain bounded native-Xorg map probes for both allocations.

```text
$ printf 'DBUS_SESSION_BUS_ADDRESS=%s\n' "${DBUS_SESSION_BUS_ADDRESS:-<unset>}"
DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus
$ busctl --user list | rg -i 'atspi|a11y|orca'
atspi-service-query-exit=1
$ printf 'XDG_SESSION_TYPE=%s DISPLAY=%s\n' "${XDG_SESSION_TYPE:-<unset>}" "${DISPLAY:-<unset>}"
XDG_SESSION_TYPE=wayland DISPLAY=:0
```

The following complete probe source is preserved in this report. It uses hand-listed expected logical, grapheme, UTF-8, and UTF-16 boundary tables. It calculates encoding forward conversions by slicing and encoding, calculates inverse conversions by strict prefix decoding, and checks every non-boundary byte and UTF-16 unit for rejection. In the CJK-plus-emoji fixture, rejected UTF-8 offsets 4 through 6 are inside the emoji's four-byte sequence, and rejected UTF-16 unit 2 is inside its surrogate pair. The explicit combining expectation applies to this fixed fixture only; it does not claim complete Unicode text-segmentation conformance.

```python
from __future__ import annotations

from dataclasses import dataclass
from unicodedata import combining


@dataclass(frozen=True)
class Fixture:
    name: str
    text: str
    expected_logical: tuple[int, ...]
    expected_graphemes: tuple[int, ...]
    expected_utf8_bytes: tuple[int, ...]
    expected_utf16_units: tuple[int, ...]


FIXTURES = (
    Fixture("ASCII", "ab", (0, 1, 2), (0, 1, 2), (0, 1, 2), (0, 1, 2)),
    Fixture("multibyte CJK + emoji", "\u6f22\U0001f600", (0, 1, 2), (0, 1, 2), (0, 3, 7), (0, 1, 3)),
    Fixture("combining sequence", "e\u0301", (0, 1, 2), (0, 2), (0, 1, 3), (0, 1, 2)),
    Fixture("bidirectional", "A\u05d0B", (0, 1, 2, 3), (0, 1, 2, 3), (0, 1, 3, 4), (0, 1, 2, 3)),
)


def require_logical_boundary(text: str, logical: int) -> None:
    if not 0 <= logical <= len(text):
        raise ValueError("logical offset is outside the Unicode-scalar range")


def scalar_to_utf8_byte(text: str, logical: int) -> int:
    require_logical_boundary(text, logical)
    return len(text[:logical].encode("utf-8"))


def utf8_byte_to_scalar(text: str, byte_offset: int) -> int:
    encoded = text.encode("utf-8")
    if not 0 <= byte_offset <= len(encoded):
        raise ValueError("UTF-8 byte offset is outside the text")
    try:
        prefix = encoded[:byte_offset].decode("utf-8", "strict")
    except UnicodeDecodeError as error:
        raise ValueError("UTF-8 byte offset is inside a Unicode scalar") from error
    return len(prefix)


def scalar_to_utf16_unit(text: str, logical: int) -> int:
    require_logical_boundary(text, logical)
    return len(text[:logical].encode("utf-16-le")) // 2


def utf16_unit_to_scalar(text: str, unit_offset: int) -> int:
    encoded = text.encode("utf-16-le")
    if not 0 <= unit_offset <= len(encoded) // 2:
        raise ValueError("UTF-16 unit offset is outside the text")
    try:
        prefix = encoded[: unit_offset * 2].decode("utf-16-le", "strict")
    except UnicodeDecodeError as error:
        raise ValueError("UTF-16 unit offset is inside a surrogate pair") from error
    return len(prefix)


def expected_grapheme_boundaries(text: str) -> tuple[int, ...]:
    return tuple(
        offset
        for offset in range(len(text) + 1)
        if offset == 0 or offset == len(text) or not combining(text[offset])
    )


def rejected_offsets(converter, limit: int, boundaries: tuple[int, ...]) -> tuple[int, ...]:
    rejected = []
    for offset in range(limit + 1):
        if offset in boundaries:
            converter(offset)
            continue
        try:
            converter(offset)
        except ValueError:
            rejected.append(offset)
        else:
            raise AssertionError(f"accepted interior offset {offset}")
    return tuple(rejected)


for fixture in FIXTURES:
    text = fixture.text
    logical = tuple(range(len(text) + 1))
    assert logical == fixture.expected_logical
    assert expected_grapheme_boundaries(text) == fixture.expected_graphemes

    utf8_bytes = tuple(scalar_to_utf8_byte(text, offset) for offset in fixture.expected_logical)
    utf16_units = tuple(scalar_to_utf16_unit(text, offset) for offset in fixture.expected_logical)
    assert utf8_bytes == fixture.expected_utf8_bytes
    assert utf16_units == fixture.expected_utf16_units

    assert tuple(utf8_byte_to_scalar(text, offset) for offset in fixture.expected_utf8_bytes) == fixture.expected_logical
    assert tuple(utf16_unit_to_scalar(text, offset) for offset in fixture.expected_utf16_units) == fixture.expected_logical

    rejected_utf8 = rejected_offsets(lambda offset: utf8_byte_to_scalar(text, offset), len(text.encode("utf-8")), fixture.expected_utf8_bytes)
    rejected_utf16 = rejected_offsets(lambda offset: utf16_unit_to_scalar(text, offset), len(text.encode("utf-16-le")) // 2, fixture.expected_utf16_units)

    print(f"{fixture.name}: {ascii(text)}")
    print(f"expected logical boundaries: {fixture.expected_logical}")
    print(f"expected grapheme boundaries: {fixture.expected_graphemes}")
    print("logical | UTF-8 byte | UTF-16 unit | grapheme")
    for logical_offset, byte_offset, unit_offset in zip(fixture.expected_logical, utf8_bytes, utf16_units):
        grapheme = "boundary" if logical_offset in fixture.expected_graphemes else "inside"
        print(f"{logical_offset:7} | {byte_offset:10} | {unit_offset:11} | {grapheme}")
    print(f"rejected UTF-8 interior byte offsets: {rejected_utf8}")
    print(f"rejected UTF-16 interior unit offsets: {rejected_utf16}")

print("all forward and inverse conversions passed")
```

```text
$ nix shell nixpkgs#python3 -c python3 /tmp/wf-epic-b/OXY-B004/unicode_offsets.py
ASCII: 'ab'
expected logical boundaries: (0, 1, 2)
expected grapheme boundaries: (0, 1, 2)
logical | UTF-8 byte | UTF-16 unit | grapheme
      0 |          0 |           0 | boundary
      1 |          1 |           1 | boundary
      2 |          2 |           2 | boundary
rejected UTF-8 interior byte offsets: ()
rejected UTF-16 interior unit offsets: ()
multibyte CJK + emoji: '\u6f22\U0001f600'
expected logical boundaries: (0, 1, 2)
expected grapheme boundaries: (0, 1, 2)
logical | UTF-8 byte | UTF-16 unit | grapheme
      0 |          0 |           0 | boundary
      1 |          3 |           1 | boundary
      2 |          7 |           3 | boundary
rejected UTF-8 interior byte offsets: (1, 2, 4, 5, 6)
rejected UTF-16 interior unit offsets: (2,)
combining sequence: 'e\u0301'
expected logical boundaries: (0, 1, 2)
expected grapheme boundaries: (0, 2)
logical | UTF-8 byte | UTF-16 unit | grapheme
      0 |          0 |           0 | boundary
      1 |          1 |           1 | inside
      2 |          3 |           2 | boundary
rejected UTF-8 interior byte offsets: (2,)
rejected UTF-16 interior unit offsets: ()
bidirectional: 'A\u05d0B'
expected logical boundaries: (0, 1, 2, 3)
expected grapheme boundaries: (0, 1, 2, 3)
logical | UTF-8 byte | UTF-16 unit | grapheme
      0 |          0 |           0 | boundary
      1 |          1 |           1 | boundary
      2 |          3 |           2 | boundary
      3 |          4 |           3 | boundary
rejected UTF-8 interior byte offsets: (2,)
rejected UTF-16 interior unit offsets: ()
all forward and inverse conversions passed
exit=0
```

## Downstream impact

- ADRs to write or update: None. This report freezes technical qualification inputs and retains gates; it does not change the accepted host or execution-domain boundaries in ADR-0005 or ADR-0006.
- Tickets unblocked in `tasks/active/`: OXY-D001 can consume the package baseline, index fixtures, and explicit retained gates. Comparable candidate qualification remains blocked by the 10 KUs in table 1.
- Tickets to add or split: Add one native Ubuntu Xorg environment-capture probe for server behavior and DRM observer feasibility. Add one recovery-injection probe per allocation only after each source identity is pinned.

### Spec edits required

- The nine evidence-bearing edits below apply only after the preservation step commits every listed fixture. For each source, fetch the canonical URL, store the fetched bytes at the associated repository-relative `path`, compute its SHA-256, replace `<to-be-computed-by-preservation-step>`, and then apply the edit. Each evidence object contains only `path` and `sha256`; URLs do not belong in the JSON value.
- OXY-D001 must confirm that the Wayland and X11 environments stay aligned on the shared `libgtk-4-1` `4.22.2+ds-1ubuntu1` package identity; both now propose that identity, while `4.20.4` is only the `gtk4` crate `v4_20` API-binding ceiling.
- OXY-D001 must reconcile the `AT-SPI` rows in `environments.wayland.protocols` and `environments.x11.protocols` in `.constitution/tech-spec/contracts/platform-contracts.json`: set both to the shared `at-spi2-core` `2.60.0-1` package identity and retain each cited 2.60.x upstream XML as source-floor evidence, distinct from that package identity, as for GTK's package identity and API-binding ceiling.

1. In `.constitution/tech-spec/stack.md`, update only the _Reference configuration_ cell for the X11 row in `Platform qualification pins` to this exact value: `x86-64 Ubuntu 26.04 LTS native Xorg session with xserver-xorg-core 2:21.1.22-1ubuntu1, libgtk-4-1 4.22.2+ds-1ubuntu1, at-spi2-core 2.60.0-1, ibus-gtk4 1.5.34~rc2-1, and orca 50.1.2-1ubuntu1; record the signed package-snapshot digest before measurement.`
2. In `.constitution/tech-spec/contracts/platform-contracts.json`, set `environments.x11.reference` to `Ubuntu 26.04 LTS x86-64 native Xorg session with xserver-xorg-core 2:21.1.22-1ubuntu1, libgtk-4-1 4.22.2+ds-1ubuntu1, at-spi2-core 2.60.0-1, ibus-gtk4 1.5.34~rc2-1, and orca 50.1.2-1ubuntu1; package-snapshot digest required before measurement.`
3. In `.constitution/tech-spec/contracts/platform-contracts.json`, retain `environments.x11.minimumVersion` as `{"status":"ku-gating","value":null,"evidence":[{"path":"qualification/fixtures/external-contracts/x11/s13-ubuntu-xserver-xorg-core.html","sha256":"<to-be-computed-by-preservation-step>"},{"path":"qualification/fixtures/external-contracts/x11/s15-ubuntu-26.04-release-notes.html","sha256":"<to-be-computed-by-preservation-step>"}]}` and keep `environments.x11.status` as `ku-gating`. These captures establish only the Resolute distribution and frozen package input; do not infer a native Xorg server or extension floor from them. Replace this value only after the native Ubuntu Xorg probe preserves the server vendor, release, package version, and negotiated extensions.
4. In `.constitution/tech-spec/contracts/platform-contracts.json`, set the X11 `GTK` protocol row to `{"name":"GTK","version":"4.22.2+ds-1ubuntu1","status":"kk","evidence":[{"path":"qualification/fixtures/external-contracts/x11/s01-ubuntu-libgtk-4-1.html","sha256":"<to-be-computed-by-preservation-step>"},{"path":"qualification/fixtures/external-contracts/x11/s02-ubuntu-gtk4-source-package.html","sha256":"<to-be-computed-by-preservation-step>"},{"path":"qualification/fixtures/external-contracts/x11/s03-gtk-4.22.2-gtkatspitext.c","sha256":"<to-be-computed-by-preservation-step>"},{"path":"qualification/fixtures/external-contracts/x11/s04-gtk-4.22.2-gtkaccessibletext.c","sha256":"<to-be-computed-by-preservation-step>"},{"path":"qualification/fixtures/external-contracts/x11/s05-gtk-4.22.2-gtkaccessibletext.h","sha256":"<to-be-computed-by-preservation-step>"},{"path":"qualification/fixtures/external-contracts/x11/s06-gtk4-imcontext.html","sha256":"<to-be-computed-by-preservation-step>"}]}`. Preserve these canonical sources in the matching evidence-array order: [Ubuntu GTK package](https://packages.ubuntu.com/resolute/libgtk-4-1), [Ubuntu GTK source package](https://launchpad.net/ubuntu/+source/gtk4/4.22.2+ds-1ubuntu1), [GTK 4.22.2 text bridge source](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/a11y/gtkatspitext.c), [GTK 4.22.2 accessible text implementation](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.c), [GTK 4.22.2 GtkAccessibleText API documentation](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.h), and [GtkIMContext](https://docs.gtk.org/gtk4/class.IMContext.html).
5. In `.constitution/tech-spec/contracts/platform-contracts.json`, set the X11 `X Present` protocol row to `{"name":"X Present","version":"1.0","status":"kk","evidence":[{"path":"qualification/fixtures/external-contracts/x11/s07-presentproto.txt","sha256":"<to-be-computed-by-preservation-step>"}]}` and set `environments.x11.timing.presentationFeedback` to `X Present 1.0 PresentCompleteNotify events acknowledge a pending PresentPixmap request; they are feedback only and never an independent presentation-opportunity source.` Preserve the canonical [Present protocol specification](https://cgit.freedesktop.org/xorg/proto/presentproto/plain/presentproto.txt) at the listed fixture path.
6. In `.constitution/tech-spec/contracts/platform-contracts.json`, set the X11 `AT-SPI` protocol row to `{"name":"AT-SPI","version":"2.60.0-1","status":"kk","evidence":[{"path":"qualification/fixtures/external-contracts/x11/s08-ubuntu-at-spi2-core.html","sha256":"<to-be-computed-by-preservation-step>"},{"path":"qualification/fixtures/external-contracts/x11/s09-atspi-text-interface.html","sha256":"<to-be-computed-by-preservation-step>"},{"path":"qualification/fixtures/external-contracts/x11/s10-atspi-2.60.0-text.xml","sha256":"<to-be-computed-by-preservation-step>"},{"path":"qualification/fixtures/external-contracts/x11/s03-gtk-4.22.2-gtkatspitext.c","sha256":"<to-be-computed-by-preservation-step>"},{"path":"qualification/fixtures/external-contracts/x11/s04-gtk-4.22.2-gtkaccessibletext.c","sha256":"<to-be-computed-by-preservation-step>"},{"path":"qualification/fixtures/external-contracts/x11/s05-gtk-4.22.2-gtkaccessibletext.h","sha256":"<to-be-computed-by-preservation-step>"},{"path":"qualification/fixtures/external-contracts/x11/s02-ubuntu-gtk4-source-package.html","sha256":"<to-be-computed-by-preservation-step>"}]}`. Immediately after it, insert `{"name":"Orca reference assistive technology package","version":"50.1.2-1ubuntu1","status":"kk","evidence":[{"path":"qualification/fixtures/external-contracts/x11/s14-ubuntu-orca.html","sha256":"<to-be-computed-by-preservation-step>"}]}`. The Orca package identity is KK, but its version-pinned documentation and live mapping remain gating KUs. Preserve the AT-SPI canonical sources in the matching evidence-array order: [Ubuntu AT-SPI package](https://packages.ubuntu.com/resolute/at-spi2-core), [AT-SPI Text interface](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.Text.html), [AT-SPI Text D-Bus XML 2.60.0](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.0/xml/Text.xml), [GTK 4.22.2 text bridge source](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/a11y/gtkatspitext.c), [GTK 4.22.2 accessible text implementation](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.c), [GTK 4.22.2 GtkAccessibleText API documentation](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.h), and [Ubuntu GTK source package](https://launchpad.net/ubuntu/+source/gtk4/4.22.2+ds-1ubuntu1). Preserve the [Ubuntu Orca package](https://packages.ubuntu.com/resolute/orca) at `s14-ubuntu-orca.html`.
7. In `.constitution/tech-spec/contracts/platform-contracts.json`, set `environments.x11.ime.evidence` to `[{"path":"qualification/fixtures/external-contracts/x11/s11-gtk4-immulticontext.html","sha256":"<to-be-computed-by-preservation-step>"},{"path":"qualification/fixtures/external-contracts/x11/s12-ubuntu-ibus-gtk4.html","sha256":"<to-be-computed-by-preservation-step>"}]`, keep `environments.x11.ime.status` as `ku-gating`, and set `environments.x11.ime.numericNegotiation` to `GtkInputPurpose values are declarative input metadata; retain this item as KU until a complete native-Xorg IBus transcript establishes any required numeric exchange.` Preserve these canonical sources in the matching evidence-array order: [GtkIMMulticontext](https://docs.gtk.org/gtk4/class.IMMulticontext.html) and [Ubuntu IBus GTK package](https://packages.ubuntu.com/resolute/ibus-gtk4).
8. In `.constitution/tech-spec/contracts/platform-contracts.json`, set `environments.x11.timing.independentMeterSource` to `KU: a harness-owned DRM card FD must prove DRM_CAP_TIMESTAMP_MONOTONIC=1, DRM_CAP_CRTC_IN_VBLANK_EVENT=1, per-CRTC RandR association, calibrated timestamps, and independence from both candidate callback streams on the frozen native Xorg session.` Keep `environments.x11.timing.status` as `ku-gating`.
9. In `.constitution/tech-spec/contracts/qualification-lock.json`, set `referenceEnvironments.x11-linux-x86_64.operatingSystem` to `Ubuntu 26.04 LTS native Xorg session; xserver-xorg-core 2:21.1.22-1ubuntu1; libgtk-4-1 4.22.2+ds-1ubuntu1; at-spi2-core 2.60.0-1; ibus-gtk4 1.5.34~rc2-1; orca 50.1.2-1ubuntu1`. Retain `referenceEnvironments.x11-linux-x86_64.minimumVersion` as `null` (gating) and `systemPackageLockDigest` as `null` until the native-Xorg probe establishes the server floor and Stage 3 records the real signed snapshot digest.

- Stage 3 must commit a regular, non-symlinked same-stem `.source.json` sidecar for each of the 15 X11 fixtures listed in table 3. Contract evidence continues to reference the captured fixture, not its sidecar. The 10 Wayland sidecars are specified solely by [SPK-B003's Spec edits required](SPK-B003.md#spec-edits-required); this X11 report neither restates nor governs them.

#### Preservation provenance and license records

The sidecar convention follows `qualification/schemas/external/README.md` and the existing `qualification/schemas/external/dsse-envelope-v1/spec/protocol.source.json` file. Every sidecar sets `kind` to `authoritative`, records the exact retrieved identity, and sets `sha256` to the streamed SHA-256 of its adjacent fixture, which must equal every evidence-object digest for that fixture. For the eight SPDX-licensed fixtures, write fields in this order: `kind`, `repository`, `commit`, `path`, `retrievalUrl`, `license`, `licenseSource`, `version`, and `sha256`. `licenseSource` contains exactly `path` and `commit`; its `path` is repository-relative, never a URL. For the six Ubuntu package or source-package page fixtures, insert `licenseNote` and `licenseUrl` immediately after `license`. The Ubuntu release-notes page uses the same field placement with a page-content license note. Use JSON `null` for an unavailable publisher source revision; don't invent a commit.

The fetched [GTK COPYING file](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/COPYING) and [AT-SPI COPYING file](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.0/COPYING) contain the GNU Lesser General Public License version 2.1 terms with the later-version option, so their fixtures use SPDX `LGPL-2.1-or-later`. The fetched [X.Org COPYING file](https://cgit.freedesktop.org/xorg/proto/presentproto/plain/COPYING?id=bfdc7e052302c79c5803ad95a73c9b63b350c40c) contains the X.Org permission notice, so its fixture uses SPDX `X11`, never the non-SPDX hybrid `MIT/X11`. The fetched [Ubuntu intellectual-property policy](https://ubuntu.com/legal/terms-and-policies/intellectual-property-policy) distinguishes Ubuntu package and binary-file copyright from the licenses of individual components. Therefore each package-page capture uses `LicenseRef-page-copyright-notice`, records its inline page notice as the license source, and does not claim the package payload's license.

Table 3 defines the complete sidecar values for the 15 X11 fixtures only.

| Fixture and required sidecar | `repository`; `commit`; `path`; `retrievalUrl`; `version` | License fields |
| :-- | :-- | :-- |
| `s01-ubuntu-libgtk-4-1.html`; `s01-ubuntu-libgtk-4-1.html.source.json` | `https://packages.ubuntu.com`; `null`; `resolute/libgtk-4-1`; `https://packages.ubuntu.com/resolute/libgtk-4-1`; `4.22.2+ds-1ubuntu1` | `license`: `LicenseRef-page-copyright-notice`; `licenseNote`: `The captured publisher page contains a copyright notice and does not state the package payload's license.`; `licenseUrl`: `https://ubuntu.com/legal/terms-and-policies/intellectual-property-policy`; `licenseSource.path`: `s01-ubuntu-libgtk-4-1.html`; `licenseSource.commit`: `null`. |
| `s02-ubuntu-gtk4-source-package.html`; `s02-ubuntu-gtk4-source-package.html.source.json` | `https://launchpad.net`; `null`; `ubuntu/+source/gtk4/4.22.2+ds-1ubuntu1`; `https://launchpad.net/ubuntu/+source/gtk4/4.22.2+ds-1ubuntu1`; `4.22.2+ds-1ubuntu1` | `license`: `LicenseRef-page-copyright-notice`; `licenseNote`: `The captured publisher page contains a copyright notice and does not state the package payload's license.`; `licenseUrl`: `https://ubuntu.com/legal/terms-and-policies/intellectual-property-policy`; `licenseSource.path`: `s02-ubuntu-gtk4-source-package.html`; `licenseSource.commit`: `null`. |
| `s03-gtk-4.22.2-gtkatspitext.c`; `s03-gtk-4.22.2-gtkatspitext.c.source.json` | `https://gitlab.gnome.org/GNOME/gtk`; `5957885ec3c8c089965fc73ed2b882e605a1817e`; `gtk/a11y/gtkatspitext.c`; `https://gitlab.gnome.org/GNOME/gtk/-/raw/5957885ec3c8c089965fc73ed2b882e605a1817e/gtk/a11y/gtkatspitext.c`; `4.22.2` | `license`: `LGPL-2.1-or-later`; `licenseSource.path`: `COPYING`; `licenseSource.commit`: `5957885ec3c8c089965fc73ed2b882e605a1817e`. |
| `s04-gtk-4.22.2-gtkaccessibletext.c`; `s04-gtk-4.22.2-gtkaccessibletext.c.source.json` | `https://gitlab.gnome.org/GNOME/gtk`; `5957885ec3c8c089965fc73ed2b882e605a1817e`; `gtk/gtkaccessibletext.c`; `https://gitlab.gnome.org/GNOME/gtk/-/raw/5957885ec3c8c089965fc73ed2b882e605a1817e/gtk/gtkaccessibletext.c`; `4.22.2` | `license`: `LGPL-2.1-or-later`; `licenseSource.path`: `COPYING`; `licenseSource.commit`: `5957885ec3c8c089965fc73ed2b882e605a1817e`. |
| `s05-gtk-4.22.2-gtkaccessibletext.h`; `s05-gtk-4.22.2-gtkaccessibletext.h.source.json` | `https://gitlab.gnome.org/GNOME/gtk`; `5957885ec3c8c089965fc73ed2b882e605a1817e`; `gtk/gtkaccessibletext.h`; `https://gitlab.gnome.org/GNOME/gtk/-/raw/5957885ec3c8c089965fc73ed2b882e605a1817e/gtk/gtkaccessibletext.h`; `4.22.2` | `license`: `LGPL-2.1-or-later`; `licenseSource.path`: `COPYING`; `licenseSource.commit`: `5957885ec3c8c089965fc73ed2b882e605a1817e`. |
| `s06-gtk4-imcontext.html`; `s06-gtk4-imcontext.html.source.json` | `https://docs.gtk.org`; `null`; `gtk4/class.IMContext.html`; `https://docs.gtk.org/gtk4/class.IMContext.html`; `null` | `license`: `LGPL-2.1-or-later`; `licenseSource.path`: `COPYING`; `licenseSource.commit`: `5957885ec3c8c089965fc73ed2b882e605a1817e`. |
| `s07-presentproto.txt`; `s07-presentproto.txt.source.json` | `https://gitlab.freedesktop.org/xorg/proto/presentproto`; `bfdc7e052302c79c5803ad95a73c9b63b350c40c`; `presentproto.txt`; `https://cgit.freedesktop.org/xorg/proto/presentproto/plain/presentproto.txt?id=bfdc7e052302c79c5803ad95a73c9b63b350c40c`; `1.1` | `license`: `X11`; `licenseSource.path`: `COPYING`; `licenseSource.commit`: `bfdc7e052302c79c5803ad95a73c9b63b350c40c`. |
| `s08-ubuntu-at-spi2-core.html`; `s08-ubuntu-at-spi2-core.html.source.json` | `https://packages.ubuntu.com`; `null`; `resolute/at-spi2-core`; `https://packages.ubuntu.com/resolute/at-spi2-core`; `2.60.0-1` | `license`: `LicenseRef-page-copyright-notice`; `licenseNote`: `The captured publisher page contains a copyright notice and does not state the package payload's license.`; `licenseUrl`: `https://ubuntu.com/legal/terms-and-policies/intellectual-property-policy`; `licenseSource.path`: `s08-ubuntu-at-spi2-core.html`; `licenseSource.commit`: `null`. |
| `s09-atspi-text-interface.html`; `s09-atspi-text-interface.html.source.json` | `https://gnome.pages.gitlab.gnome.org/at-spi2-core`; `null`; `libatspi/iface.Text.html`; `https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.Text.html`; `null` | `license`: `LGPL-2.1-or-later`; `licenseSource.path`: `COPYING`; `licenseSource.commit`: `d8ab833f0230fccc009c271d9f23f53df2a32c88`. |
| `s10-atspi-2.60.0-text.xml`; `s10-atspi-2.60.0-text.xml.source.json` | `https://gitlab.gnome.org/GNOME/at-spi2-core`; `d8ab833f0230fccc009c271d9f23f53df2a32c88`; `xml/Text.xml`; `https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/d8ab833f0230fccc009c271d9f23f53df2a32c88/xml/Text.xml`; `2.60.0` | `license`: `LGPL-2.1-or-later`; `licenseSource.path`: `COPYING`; `licenseSource.commit`: `d8ab833f0230fccc009c271d9f23f53df2a32c88`. |
| `s11-gtk4-immulticontext.html`; `s11-gtk4-immulticontext.html.source.json` | `https://docs.gtk.org`; `null`; `gtk4/class.IMMulticontext.html`; `https://docs.gtk.org/gtk4/class.IMMulticontext.html`; `null` | `license`: `LGPL-2.1-or-later`; `licenseSource.path`: `COPYING`; `licenseSource.commit`: `5957885ec3c8c089965fc73ed2b882e605a1817e`. |
| `s12-ubuntu-ibus-gtk4.html`; `s12-ubuntu-ibus-gtk4.html.source.json` | `https://packages.ubuntu.com`; `null`; `resolute/ibus-gtk4`; `https://packages.ubuntu.com/resolute/ibus-gtk4`; `1.5.34~rc2-1` | `license`: `LicenseRef-page-copyright-notice`; `licenseNote`: `The captured publisher page contains a copyright notice and does not state the package payload's license.`; `licenseUrl`: `https://ubuntu.com/legal/terms-and-policies/intellectual-property-policy`; `licenseSource.path`: `s12-ubuntu-ibus-gtk4.html`; `licenseSource.commit`: `null`. |
| `s13-ubuntu-xserver-xorg-core.html`; `s13-ubuntu-xserver-xorg-core.html.source.json` | `https://packages.ubuntu.com`; `null`; `resolute/xserver-xorg-core`; `https://packages.ubuntu.com/resolute/xserver-xorg-core`; `2:21.1.22-1ubuntu1` | `license`: `LicenseRef-page-copyright-notice`; `licenseNote`: `The captured publisher page contains a copyright notice and does not state the package payload's license.`; `licenseUrl`: `https://ubuntu.com/legal/terms-and-policies/intellectual-property-policy`; `licenseSource.path`: `s13-ubuntu-xserver-xorg-core.html`; `licenseSource.commit`: `null`. |
| `s14-ubuntu-orca.html`; `s14-ubuntu-orca.html.source.json` | `https://packages.ubuntu.com`; `null`; `resolute/orca`; `https://packages.ubuntu.com/resolute/orca`; `50.1.2-1ubuntu1` | `license`: `LicenseRef-page-copyright-notice`; `licenseNote`: `The captured publisher page contains a copyright notice and does not state the package payload's license.`; `licenseUrl`: `https://ubuntu.com/legal/terms-and-policies/intellectual-property-policy`; `licenseSource.path`: `s14-ubuntu-orca.html`; `licenseSource.commit`: `null`. |
| `s15-ubuntu-26.04-release-notes.html`; `s15-ubuntu-26.04-release-notes.html.source.json` | `https://documentation.ubuntu.com`; `null`; `release-notes/26.04/`; `https://documentation.ubuntu.com/release-notes/26.04/`; `26.04 LTS (Resolute Raccoon)` | `license`: `LicenseRef-page-copyright-notice`; `licenseNote`: `The captured publisher page contains a copyright notice and does not state a page-content license.`; `licenseUrl`: `https://ubuntu.com/legal/terms-and-policies/intellectual-property-policy`; `licenseSource.path`: `s15-ubuntu-26.04-release-notes.html`; `licenseSource.commit`: `null`. |

The GTK, AT-SPI, and X.Org release commits in table 3 come from the preserved discovery probe. Table 3 records the `COPYING` path for every SPDX fixture and the adjacent captured fixture for every page-copyright notice. It does not govern the Wayland map.

```text
$ git ls-remote https://gitlab.gnome.org/GNOME/gtk.git "refs/tags/4.22.2^{}"
5957885ec3c8c089965fc73ed2b882e605a1817e	refs/tags/4.22.2^{}
$ git ls-remote https://gitlab.gnome.org/GNOME/at-spi2-core.git "refs/tags/2.60.0^{}"
d8ab833f0230fccc009c271d9f23f53df2a32c88	refs/tags/2.60.0^{}
$ git ls-remote https://gitlab.freedesktop.org/xorg/proto/presentproto.git "refs/tags/presentproto-1.1^{}"
bfdc7e052302c79c5803ad95a73c9b63b350c40c	refs/tags/presentproto-1.1^{}
exit=0
```

## Sources

- [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/)
- [Ubuntu xserver-xorg-core package](https://packages.ubuntu.com/resolute/xserver-xorg-core)
- [Ubuntu GTK package](https://packages.ubuntu.com/resolute/libgtk-4-1)
- [Ubuntu AT-SPI package](https://packages.ubuntu.com/resolute/at-spi2-core)
- [Ubuntu IBus GTK package](https://packages.ubuntu.com/resolute/ibus-gtk4)
- [Ubuntu Orca package](https://packages.ubuntu.com/resolute/orca)
- [GtkIMContext](https://docs.gtk.org/gtk4/class.IMContext.html)
- [GtkIMMulticontext](https://docs.gtk.org/gtk4/class.IMMulticontext.html)
- [GtkIMContext set_surrounding](https://docs.gtk.org/gtk4/method.IMContext.set_surrounding.html)
- [GtkIMContext delete_surrounding](https://docs.gtk.org/gtk4/method.IMContext.delete_surrounding.html)
- [GtkInputPurpose](https://docs.gtk.org/gtk4/enum.InputPurpose.html)
- [GdkFrameClock](https://docs.gtk.org/gdk4/class.FrameClock.html)
- [Orca documentation (mutable; fetched 2026-08-29T01:05:29Z)](https://help.gnome.org/users/orca/stable/introduction.html.en)
- [GNOME Orca Git repository](https://gitlab.gnome.org/GNOME/orca.git)
- [Ubuntu desktop documentation source at commit `786057d7a1ba1212d06c16880820681b40bb24d3`](https://raw.githubusercontent.com/canonical/ubuntu-desktop-documentation/786057d7a1ba1212d06c16880820681b40bb24d3/docs/reference/accessibility/dbus/org.a11y.atspi.Text.md)
- [AT-SPI Text interface](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.Text.html)
- [AT-SPI Text D-Bus XML 2.60.0](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.0/xml/Text.xml)
- [GTK 4.22.2 AT-SPI text bridge source](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/a11y/gtkatspitext.c)
- [GTK 4.22.2 accessible text implementation](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.c)
- [GTK 4.22.2 GtkAccessibleText API documentation](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.h)
- [GLib UTF-8 character conversion](https://docs.gtk.org/glib/func.utf8_get_char.html)
- [GLib UTF-8 string length](https://docs.gtk.org/glib/func.utf8_strlen.html)
- [GLib UTF-8 offset conversion](https://docs.gtk.org/glib/func.utf8_offset_to_pointer.html)
- [Ubuntu GTK source package](https://launchpad.net/ubuntu/+source/gtk4/4.22.2+ds-1ubuntu1)
- [Ubuntu GTK source descriptor](https://launchpad.net/ubuntu/+archive/primary/+sourcefiles/gtk4/4.22.2+ds-1ubuntu1/gtk4_4.22.2+ds-1ubuntu1.dsc)
- [Ubuntu GTK original source tarball](http://archive.ubuntu.com/ubuntu/pool/main/g/gtk4/gtk4_4.22.2+ds.orig.tar.xz)
- [Ubuntu GTK Debian patch tarball](http://archive.ubuntu.com/ubuntu/pool/main/g/gtk4/gtk4_4.22.2+ds-1ubuntu1.debian.tar.xz)
- [Ubuntu desktop documentation source at commit `786057d7a1ba1212d06c16880820681b40bb24d3` (AT-SPI Text)](https://raw.githubusercontent.com/canonical/ubuntu-desktop-documentation/786057d7a1ba1212d06c16880820681b40bb24d3/docs/reference/accessibility/dbus/org.a11y.atspi.Text.md)
- [Present protocol specification](https://cgit.freedesktop.org/xorg/proto/presentproto/plain/presentproto.txt)
- [Linux DRM user-space API](https://docs.kernel.org/gpu/drm-uapi.html)
- [Unicode text segmentation](https://www.unicode.org/reports/tr29/)
- [Vulkan registry](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/20a9e5892e2aab7b9776b16a238b10fc8133090a/xml/vk.xml)
- [GTK COPYING file](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/COPYING)
- [X.Org COPYING file](https://cgit.freedesktop.org/xorg/proto/presentproto/plain/COPYING?id=bfdc7e052302c79c5803ad95a73c9b63b350c40c)
- [AT-SPI COPYING file](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.0/COPYING)
- [Ubuntu intellectual-property policy](https://ubuntu.com/legal/terms-and-policies/intellectual-property-policy)
