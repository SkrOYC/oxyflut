# Spike report: OXY-B004 X11 qualification baseline

## Time box

- Budget: 1 focused day.
- Clock start / stop: 2026-08-28T16:47:53Z / 2026-08-28T16:58:24Z.

## Question

- Decision this spike produces: Freeze the cited Ubuntu package inputs and documented protocol mechanics, use `GtkIMMulticontext` with the IBus GTK module and Orca as the reference Linux assistive technology, and retain the unproven allocation, native-Xorg, timing, routing, and recovery claims as gating KUs.

Table 1 answers each part of the decision.

| Question row | Answer | Status | Citations or preserved evidence | Next bounded probe for a gating KU |
| :-- | :-- | :-- | :-- | :-- |
| Reference distribution and package floor | Freeze Ubuntu 26.04 LTS package inputs: `xserver-xorg-core` `2:21.1.22-1ubuntu1`, `libgtk-4-1` `4.22.2+ds-1ubuntu1`, `at-spi2-core` `2.60.0-1`, `ibus-gtk4` `1.5.34~rc2-1`, and Orca `50.1.2-1ubuntu1`. This is an exact qualification input, not a claim about a live session. | KK | [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/); [xserver-xorg-core package](https://packages.ubuntu.com/resolute/xserver-xorg-core); [GTK package](https://packages.ubuntu.com/resolute/libgtk-4-1); [AT-SPI package](https://packages.ubuntu.com/resolute/at-spi2-core); [IBus GTK package](https://packages.ubuntu.com/resolute/ibus-gtk4); [Orca package](https://packages.ubuntu.com/resolute/orca). | Not applicable. |
| Native X server and extension floor | The host probe reports Xwayland, not an Ubuntu native Xorg session. It reports X.Org 24.1.13, `Present` 1.4, `SYNC` 3.1, XInput 2.4, and RandR 1.6. Those facts demonstrate only nonreference extension-query mechanics. The Resolute package page identifies a package version, not the native server release or extension floor, so neither is frozen. | KU (gating) | Preserved X11 probe output; [Ubuntu xserver-xorg-core package](https://packages.ubuntu.com/resolute/xserver-xorg-core); [Present protocol specification](https://cgit.freedesktop.org/xorg/proto/presentproto/plain/presentproto.txt). | On an Ubuntu 26.04 native Xorg session with the frozen `xserver-xorg-core` package, run `xdpyinfo -display "$DISPLAY"`, `xrandr --version`, and a 2-window XCB query for Present, RandR, Sync, and XInput versions. Preserve the server vendor and release, package version, negotiated versions, each window's RandR output and CRTC, and a Present submit/completion transcript. Pass only if the records bind to that native server and package list; then propose the observed server and extension floor. |
| GTK input-context mechanism and index units | GTK documents preedit start, change, end, commit, surrounding-text retrieval, deletion, focus, reset, cursor rectangle, and preedit attributes. `set_surrounding` receives UTF-8 and a byte cursor index, while `delete_surrounding` uses character offsets and counts. `GtkInputPurpose` supplies declarative password and PIN purposes; this report makes no numeric-handoff claim. | KK | [GtkIMContext](https://docs.gtk.org/gtk4/class.IMContext.html); [set_surrounding](https://docs.gtk.org/gtk4/method.IMContext.set_surrounding.html); [delete_surrounding](https://docs.gtk.org/gtk4/method.IMContext.delete_surrounding.html); [GtkInputPurpose](https://docs.gtk.org/gtk4/enum.InputPurpose.html). | Not applicable. |
| Complete IBus GTK input-method transcript for each allocation | Use `GtkIMMulticontext` and the frozen `ibus-gtk4` module as the reference IME path. No native-Xorg transcript demonstrates the required preedit, commit, surrounding, replacement, candidate-geometry, focus-transfer, reset, secure-field, or CJK behavior for either candidate. | KU (gating) | [GtkIMMulticontext](https://docs.gtk.org/gtk4/class.IMMulticontext.html); [IBus GTK package](https://packages.ubuntu.com/resolute/ibus-gtk4). | On the frozen Ubuntu native Xorg session, run a noncandidate GTK 4.22 harness with `GTK_IM_MODULE=ibus` and a selected IBus composition engine. Capture ordered signals and callbacks for every vector listed in `environments.x11.ime.testVectors`, including UTF-8 byte cursor values, character deletion values, rectangle coordinates, and redacted secure-field records. Repeat the same transcript through each candidate's adapter after its source identity exists. |
| Linux assistive technology and base AT-SPI surface | Select Orca `50.1.2-1ubuntu1` as the reference assistive technology and `at-spi2-core` `2.60.0-1` as the service package. Orca documents that it works through AT-SPI. The `Atspi.Text` and D-Bus references define text, attributes, extents, caret, selection, and offset operations. | KK | [Orca documentation](https://help.gnome.org/users/orca/stable/introduction.html.en); [Orca package](https://packages.ubuntu.com/resolute/orca); [AT-SPI Text interface](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.Text.html); [Ubuntu AT-SPI Text D-Bus reference](https://documentation.ubuntu.com/desktop/en/latest/reference/accessibility/dbus/org.a11y.atspi.Text/). | Not applicable. |
| Focused allocation forward and reverse AT-SPI map | No focused candidate exists to map every semantics role, property, relation, state, value, geometry, text range, event, and `Action` invocation to the selected AT-SPI interfaces. | KU (gating) | The AT-SPI references establish the target interface family but not an Oxyflut implementation: [AT-SPI Text interface](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.Text.html); [Ubuntu AT-SPI Text D-Bus reference](https://documentation.ubuntu.com/desktop/en/latest/reference/accessibility/dbus/org.a11y.atspi.Text/). | Build the focused candidate's two-window semantics fixture on the frozen native Xorg session with Orca and an AT-SPI D-Bus recorder. Produce a versioned map artifact and hash that enumerates forward events and reverse actions, routes each action by XID and view generation, and reports a stale-target error after teardown. |
| Integrated allocation forward and reverse AT-SPI map | The integrated fork commit is not pinned, and its inherited GTK and AT-SPI path has not been enumerated. A complete map cannot be inferred from Flutter or GTK documentation. | KU (gating) | [AT-SPI Text interface](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.Text.html); the qualification lock records no integrated fork commit. | After Stage 3 records the integrated fork commit, enumerate its Linux embedder sources and run the same two-window Orca and AT-SPI recorder fixture as the focused allocation. Preserve the inherited interface inventory, map artifact, hash, and reverse-action routing result. |
| Unicode-scalar AT-SPI offset fixtures | For GTK 4.22.2, the selected AT-SPI provider's Text offsets are Unicode-scalar indices: its `GetCharacterAtOffset` path bounds `offset` with `g_utf8_strlen`, converts it with `g_utf8_offset_to_pointer`, and returns `g_utf8_get_char` as `gunichar`. GLib defines that conversion as a UTF-8 byte sequence to a Unicode character. The AT-SPI XML separately states that `CharacterCount` can differ from the returned UTF-8 byte count. The preserved probe uses hand-listed logical, grapheme, UTF-8, and UTF-16 boundaries, tests both conversion directions, and rejects every interior byte and UTF-16-unit offset. It proves a scalar boundary can be inside the combining-sequence grapheme. | KK | [AT-SPI Text D-Bus XML](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/main/xml/Text.xml); [GTK 4.22.2 AT-SPI text bridge source](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/a11y/gtkatspitext.c); [GTK 4.22.2 accessible text source](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.c); [GLib UTF-8 character conversion](https://docs.gtk.org/glib/func.utf8_get_char.html); preserved Unicode probe output. | Not applicable to the GTK text-unit KK. The host has no AT-SPI service, so a real D-Bus offset observation remains in each allocation's native-Xorg forward and reverse map probe. |
| X Present feedback role | Require Present protocol 1.0 or later for presentation feedback. The specification says `PresentCompleteNotify` reports completion of a pending `PresentPixmap` request. It is acknowledgement feedback only. It is not an independent presentation-opportunity source, and no schedule may derive opportunities from it. | KK | [Present protocol specification](https://cgit.freedesktop.org/xorg/proto/presentproto/plain/presentproto.txt). | Not applicable. |
| Independent timing meter and per-output association | `GdkFrameClock` can use a timer and is the host scheduler, so it cannot measure itself. The DRM API can request vblank events and can report monotonic timestamps and CRTC IDs when its capabilities are present, but this spike did not establish permission, output-to-CRTC mapping, calibration, or independence on the Ubuntu native Xorg session. | KU (gating) | [GdkFrameClock](https://docs.gtk.org/gdk4/class.FrameClock.html); [Linux DRM user-space API](https://docs.kernel.org/gpu/drm-uapi.html); preserved host DRM-node probe. | On the frozen Ubuntu native Xorg session, open a harness-owned DRM card FD distinct from both candidates. Require `DRM_CAP_TIMESTAMP_MONOTONIC=1` and `DRM_CAP_CRTC_IN_VBLANK_EVENT=1`, correlate each RandR output to its CRTC, and record 10 seconds of `DRM_EVENT_VBLANK`, candidate schedule callbacks, and Present completions for two windows. The expected result is one calibrated, per-CRTC opportunity stream whose collection consumes neither candidate callback stream. Retain this KU if the session denies the FD or cannot establish the mapping. |
| Focused allocation interface inventory and service routing | The documented GTK, X11, AT-SPI, Present, and Vulkan error surfaces identify possible inputs, not a focused adapter. No multi-view trace proves that every request and callback carries the owning XID and view generation through the reentrancy barrier. | KU (gating) | [GtkIMContext](https://docs.gtk.org/gtk4/class.IMContext.html); [Present protocol specification](https://cgit.freedesktop.org/xorg/proto/presentproto/plain/presentproto.txt); [Vulkan registry](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/main/xml/vk.xml). | Run a focused two-window fixture with distinct XIDs and generations. Issue IME, clipboard, AT-SPI action, resize, Present, and teardown events while one window is destroyed and recreated. Preserve a trace that shows the correct owner for every accepted event and a structured stale-generation rejection for every late event. |
| Integrated allocation interface inventory and service routing | The integrated fork and its exact inherited Linux embedder interfaces are absent. Therefore no callback ownership, C ABI normalization, or multi-view service-routing claim is established. | KU (gating) | The qualification lock has no integrated fork commit; [Present protocol specification](https://cgit.freedesktop.org/xorg/proto/presentproto/plain/presentproto.txt) only describes one protocol input. | After the fork is pinned, enumerate every inherited X11, GTK, IME, AT-SPI, clipboard, scheduling, lifecycle, and graphics callback. Run the same distinct-XID and generation trace required for the focused allocation, then preserve its inventory and trace hash. |
| Focused allocation recovery injection | Vulkan defines `VK_ERROR_DEVICE_LOST` and permits `VK_ERROR_SURFACE_LOST_KHR` from surface and presentation operations, but neither source nor a probe establishes a focused adapter fault-injection seam or recovery result. | KU (gating) | [Vulkan registry](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/main/xml/vk.xml). | Add a qualification-only focused-adapter fault script after its source identity is pinned. Force surface-loss and device-lost results at acquire and present boundaries, then force resize and RandR topology change. Preserve injection point, monotonic fault time, recreation attempts, acknowledged frame, resource release, and terminal-error records against CON-REC-001 through CON-REC-007. |
| Integrated allocation recovery injection | No integrated fork source, lifecycle inventory, or graphics recovery seam is pinned. Vulkan error definitions do not establish that the inherited embedder can receive deterministic faults. | KU (gating) | [Vulkan registry](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/main/xml/vk.xml). | After the integrated fork is pinned, identify its Vulkan dispatch and lifecycle boundaries. Run the same scripted surface-loss, device-lost, resize, and RandR topology tests as the focused allocation, and preserve the C ABI ingress, recovery trace, attempt count, and terminal-error result. |

## Context and objective

- Triggering upstream file or section: `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.x11`.
- Target: Replace only the package, documented interface, AT-SPI selection, Unicode-fixture, and Present-feedback claims with KK evidence. Retain every candidate-dependent or native-session-dependent claim as a smaller, executable blocker.
- Archetype / surface: Library and SDK with system X11 desktop integration.
- Evidence terms: KK is a cited official source or preserved host probe. KU (gating) names a bounded unanswered qualification question. No claim in this report depends on architectural plausibility or on extension presence alone.

## Codebase baseline

- Stage 3 pins Ubuntu 26.04 LTS and GTK 4.20 API bindings but leaves the X11 package lock and X server behavior open.
- The official Ubuntu archive supplies the exact package versions in table 1. It does not substitute for a package-snapshot digest, a native Xorg session capture, or a server behavior probe.
- The Ubuntu GTK package is 4.22.2, which satisfies the existing GTK 4.20 API-binding ceiling. The report does not change the product surface or architecture boundary.
- `GtkIMContext` uses two different index conventions: UTF-8 byte offsets for surrounding-text cursor information and character offsets for deletion. The adapter must convert both through checked strong index types before state mutation.
- For the selected GTK 4.22.2 AT-SPI provider, Text offsets are Unicode-scalar indices: its bridge bounds an offset with `g_utf8_strlen`, converts it through `g_utf8_offset_to_pointer`, and returns `g_utf8_get_char` as `gunichar`. AT-SPI returns UTF-8 text and its XML warns that returned byte length can exceed its character offsets.

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
| AT-SPI maps and scalar conversion | A plus C | Freeze the GTK 4.22.2 Unicode-scalar conversion contract, audited fixtures, and Orca selection. Retain each allocation's complete forward and reverse map and real native-Xorg AT-SPI observation as separate gates because no candidate implementation exists. |
| Independent timing | C | Do not use `GdkFrameClock`, nominal timers, or Present completion as an independent source. Consider B only after the exact DRM sidecar probe passes. |
| Service routing and recovery | C | Require per-XID and generation traces plus allocation-specific scripted recovery faults. Do not infer either property from GTK, Vulkan result names, or the architecture documents. |

- Why it fits: The mix freezes reproducible upstream inputs without converting availability into behavior. It preserves candidate symmetry and keeps opportunity measurement independent from schedule and completion callbacks.
- Rejected options: Do not use extension availability as proof of server semantics. Do not use `PresentCompleteNotify` as an opportunity source. Do not select an unspecified assistive technology or IME module. Do not infer a default window, display, or view for a service request.

## Probe outputs

The following probes ran on this host. The host uses Wayland with Xwayland at `DISPLAY=:0`; these results are nonreference and establish extension-query mechanics only.

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

The upstream AT-SPI Text XML says, "CharacterCount: The total number of characters in a text object. This may differ from the number of bytes that would be returned if the text is fetched in cases where characters are expressed using multiple bytes." It also says, "Returns: a text string containing characters from @startOffset to @endOffset-1, inclusive, encoded as UTF-8." [AT-SPI Text D-Bus XML](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/main/xml/Text.xml) defines the protocol's non-byte character offsets.

The selected GTK 4.22.2 provider fixes the character unit. Its AT-SPI `GetCharacterAtOffset` implementation reads the D-Bus integer and executes `if (0 <= offset && offset < g_utf8_strlen (text, -1)) ch = g_utf8_get_char (g_utf8_offset_to_pointer (text, offset));`. Its `CharacterCount` property is `g_utf8_strlen (text, -1)`. The [GTK 4.22.2 AT-SPI text bridge source](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/a11y/gtkatspitext.c) therefore indexes by a UTF-8 character count, converts the character offset to a pointer, and returns `gunichar`. The [GLib UTF-8 character conversion documentation](https://docs.gtk.org/glib/func.utf8_get_char.html) says, "Converts a sequence of bytes encoded as UTF-8 to a Unicode character." The [GTK 4.22.2 accessible text source](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.c) independently documents ranges "in characters" and calculates the count with `g_utf8_strlen`. For this frozen GTK provider, an AT-SPI character offset is therefore a Unicode-scalar index, not a UTF-8 byte, UTF-16 unit, or grapheme index.

The host session has no advertised AT-SPI or Orca service, so this spike did not make a real AT-SPI call. This preserves a bounded runtime observation in the focused and integrated native-Xorg map probes, rather than treating the absent service as a negative compatibility result.

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
- Tickets unblocked in `tasks/active/`: OXY-D001 can consume the package baseline, index fixtures, and explicit retained gates. Comparable candidate qualification remains blocked by the nine KUs in table 1.
- Tickets to add or split: Add one native Ubuntu Xorg environment-capture probe for server behavior and DRM observer feasibility. Add one recovery-injection probe per allocation only after each source identity is pinned.
- Spec edits required:
  1. In `.constitution/tech-spec/stack.md`, update the X11 row in `Platform qualification pins` to this exact value: `x86-64 Ubuntu 26.04 LTS native Xorg session with xserver-xorg-core 2:21.1.22-1ubuntu1, libgtk-4-1 4.22.2+ds-1ubuntu1, at-spi2-core 2.60.0-1, ibus-gtk4 1.5.34~rc2-1, and orca 50.1.2-1ubuntu1; record the signed package-snapshot digest before measurement.`
  2. In `.constitution/tech-spec/contracts/platform-contracts.json`, set `environments.x11.reference` to `Ubuntu 26.04 LTS x86-64 native Xorg session with xserver-xorg-core 2:21.1.22-1ubuntu1, libgtk-4-1 4.22.2+ds-1ubuntu1, at-spi2-core 2.60.0-1, ibus-gtk4 1.5.34~rc2-1, and orca 50.1.2-1ubuntu1; package-snapshot digest required before measurement.`
  3. In `.constitution/tech-spec/contracts/platform-contracts.json`, retain `environments.x11.minimumVersion` as `{"status":"ku-gating","value":null,"evidence":[]}` and keep `environments.x11.status` as `ku-gating`. Do not infer a native Xorg server or extension floor from the Resolute package page; replace this value only after the native Ubuntu Xorg probe preserves the server vendor, release, package version, and negotiated extensions.
  4. In `.constitution/tech-spec/contracts/platform-contracts.json`, set the X11 `GTK` protocol row to `{"name":"GTK","version":"4.22.2+ds-1ubuntu1","status":"kk","evidence":["https://packages.ubuntu.com/resolute/libgtk-4-1","https://docs.gtk.org/gtk4/class.IMContext.html"]}`.
  5. In `.constitution/tech-spec/contracts/platform-contracts.json`, set the X11 `X Present` protocol row to `{"name":"X Present","version":"1.0","status":"kk","evidence":["https://cgit.freedesktop.org/xorg/proto/presentproto/plain/presentproto.txt"]}` and set `environments.x11.timing.presentationFeedback` to `X Present 1.0 PresentCompleteNotify events acknowledge a pending PresentPixmap request; they are feedback only and never an independent presentation-opportunity source.`
  6. In `.constitution/tech-spec/contracts/platform-contracts.json`, set the X11 `AT-SPI` protocol row to `{"name":"AT-SPI","version":"2.60.0-1","status":"kk","evidence":["https://packages.ubuntu.com/resolute/at-spi2-core","https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.Text.html"]}`.
  7. In `.constitution/tech-spec/contracts/platform-contracts.json`, set `environments.x11.ime.evidence` to `["https://docs.gtk.org/gtk4/class.IMMulticontext.html","https://packages.ubuntu.com/resolute/ibus-gtk4"]`, keep `environments.x11.ime.status` as `ku-gating`, and set `environments.x11.ime.numericNegotiation` to `GtkInputPurpose values are declarative input metadata; retain this item as KU until a complete native-Xorg IBus transcript establishes any required numeric exchange.`
  8. In `.constitution/tech-spec/contracts/platform-contracts.json`, set `environments.x11.timing.independentMeterSource` to `KU: a harness-owned DRM card FD must prove DRM_CAP_TIMESTAMP_MONOTONIC=1, DRM_CAP_CRTC_IN_VBLANK_EVENT=1, per-CRTC RandR association, calibrated timestamps, and independence from both candidate callback streams on the frozen native Xorg session.` Keep `environments.x11.timing.status` as `ku-gating`.
  9. In `.constitution/tech-spec/contracts/qualification-lock.json`, set `referenceEnvironments.x11-linux-x86_64.operatingSystem` to `Ubuntu 26.04 LTS native Xorg session; xserver-xorg-core 2:21.1.22-1ubuntu1; libgtk-4-1 4.22.2+ds-1ubuntu1; at-spi2-core 2.60.0-1; ibus-gtk4 1.5.34~rc2-1; orca 50.1.2-1ubuntu1`. Retain `referenceEnvironments.x11-linux-x86_64.minimumVersion` as `null` (gating) and `systemPackageLockDigest` as `null` until the native-Xorg probe establishes the server floor and Stage 3 records the real signed snapshot digest.

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
- [Orca documentation](https://help.gnome.org/users/orca/stable/introduction.html.en)
- [AT-SPI Text interface](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.Text.html)
- [AT-SPI Text D-Bus XML](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/main/xml/Text.xml)
- [GTK 4.22.2 AT-SPI text bridge source](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/a11y/gtkatspitext.c)
- [GTK 4.22.2 accessible text source](https://gitlab.gnome.org/GNOME/gtk/-/raw/4.22.2/gtk/gtkaccessibletext.c)
- [GLib UTF-8 character conversion](https://docs.gtk.org/glib/func.utf8_get_char.html)
- [Ubuntu AT-SPI Text D-Bus reference](https://documentation.ubuntu.com/desktop/en/latest/reference/accessibility/dbus/org.a11y.atspi.Text/)
- [Present protocol specification](https://cgit.freedesktop.org/xorg/proto/presentproto/plain/presentproto.txt)
- [Linux DRM user-space API](https://docs.kernel.org/gpu/drm-uapi.html)
- [Unicode text segmentation](https://www.unicode.org/reports/tr29/)
- [Vulkan registry](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/main/xml/vk.xml)
