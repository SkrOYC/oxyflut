# Spike report: OXY-B003 Wayland qualification baseline

## Time box

- **Budget:** 1 focused day.
- **Clock start / stop:** 2026-08-28T16:48:45Z / 2026-08-28T16:59:54Z.

## Question

- **Decision this spike produces:** Use `wp_presentation` version 1 as the protocol floor for per-commit acknowledgement and main-output association. Keep the Ubuntu reference-session protocol status as a gating KU until P1 records the selected session's package lock and advertised version. Use writable `GtkIMContext` input-purpose and input-hints properties, and convert its documented UTF-8 byte cursor positions explicitly. Use Orca with AT-SPI 2 as the Linux assistive-technology test client. Freeze the documented AT-SPI text-offset unit as Unicode code-point (scalar) boundaries, but retain behavior across text, caret, selection, and editable operations as a gating KU until it is observed. Retain the reference-compositor, candidate-transcript, complete-map, independent-meter, routing, and recovery gates until their bounded reference probes pass.

Table 1 answers each Wayland baseline question. KK is a verified fact. KU (gating) is a named unresolved gate. No row is not applicable.

Table 1. Wayland baseline decisions

| Row | Answer and evidence | Status | Next bounded probe |
| :-- | :-- | :-- | :-- |
| Reference compositor, session, and package lock | [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/) establish Ubuntu 26.04 LTS, but the fetched release-note content names no compositor, session, package version, or package-lock digest. The non-reference host is NixOS 26.05 with Hyprland 0.55.4, so its registry cannot establish Ubuntu compositor behavior. | KU (gating) | P1: On the selected Ubuntu 26.04 x86-64 Wayland session, record `gnome-shell --version` or the selected compositor's version command, `dpkg-query -W` for the compositor, `gtk4`, `wayland-protocols`, and `at-spi2-core`, the package-manifest SHA-256, and a filtered `wayland-info` registry. Run a 120-frame visible-surface probe that records `wl_surface.frame`, `wp_presentation_feedback.presented` or `discarded`, and `sync_output` events. Expected output: one named compositor version, one package-lock digest, the advertised `wp_presentation` version, and a session-specific event transcript. |
| Wayland core and `wp_presentation` protocol floor | The version-1 [presentation-time XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml) declares both `wp_presentation` interfaces at version 1 and contains `feedback`, `sync_output`, `presented`, and `discarded`. Those are the only operations the harness needs for per-commit acknowledgement and main-output association. The version-2 [presentation-time XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/8cdb39103247fdde5764fc35b1b5cf60698db3e5/stable/presentation-time/presentation-time.xml) records the semantic change: "For version 2 and later" a non-constant-refresh output must provide an appropriate refresh rate or zero only when no rate exists, whereas version 1 requires zero. The harness does not consume that variable-refresh prediction, so version 2 is not required. The cited protocol floor is KK, but reference-session advertisement and availability remain unresolved. | KU (gating) | P1: On the selected Ubuntu session, record the `wp_presentation` global and advertised version from `wayland-info`, then run the visible-surface transcript. Accept this gate only when the session advertises version 1 or later and the transcript contains one `feedback` request, `sync_output` before `presented` when an output is bound, and one terminal `presented` or `discarded` event per submitted feedback object. Expected output: package lock, registry line, and event transcript. |
| GTK release floor on the reference | [GNOME publishes GTK 4.20.4](https://download.gnome.org/sources/gtk/4.20/), and the documented APIs needed for selection-aware IME, [`GtkAccessible`](https://docs.gtk.org/gtk4/iface.Accessible.html), and [`GtkAccessibleText`](https://docs.gtk.org/gtk4/iface.AccessibleText.html) were introduced no later than GTK 4.14. This does not identify the Ubuntu package revision, package digest, or session backend. | KU (gating) | P1: Record the installed `gtk4` package version and immutable package-manifest digest on the Ubuntu reference. Accept it only when it is GTK 4.20.4 or a separately reviewed replacement that exposes the cited API set. Expected output: package version, package origin, and digest. |
| `GtkIMContext` surrounding text and input-purpose mechanism | [`set_surrounding`](https://docs.gtk.org/gtk4/method.IMContext.set_surrounding.html) takes UTF-8 text and a byte index for the cursor. [`input-purpose`](https://docs.gtk.org/gtk4/property.IMContext.input-purpose.html) and [`input-hints`](https://docs.gtk.org/gtk4/property.IMContext.input-hints.html) are writable properties. [`GtkInputPurpose`](https://docs.gtk.org/gtk4/enum.InputPurpose.html) supplies typed purpose values, including `PASSWORD` and `PIN`; [`GtkInputHints.PRIVATE`](https://docs.gtk.org/gtk4/flags/InputHints.html) requests that an input method not update personalized data. These are properties, not a compositor numeric negotiation. | KK | Not required. P2 verifies the selected input method's behavior rather than the documented interface shape. |
| Complete IME transcript and non-cursor operation units | GTK documents [`delete-surrounding`](https://docs.gtk.org/gtk4/signal.IMContext.delete-surrounding.html) arguments as character offsets and counts, but it does not state the scalar, grapheme, or another unit in the fetched API page. No selected Ubuntu IM module or candidate transcript exists. The report therefore does not infer a unit for deletion, preedit cursor position, or replacement behavior. | KU (gating) | P2: On the P1 session, use an instrumented noncandidate GTK 4.20.4 text widget and the ASCII, multibyte, combining, bidirectional, CJK-composition, replacement, candidate-geometry, and secure-field corpus. Log every `preedit-*`, `commit`, `retrieve-surrounding`, `delete-surrounding`, `focus-*`, and `reset` callback with typed indices. Expected output: a transcript that identifies every operation's unit and round trips each valid boundary. |
| Linux assistive-technology selection | Select [Orca](https://help.gnome.org/users/orca/stable/) as the required screen-reader test client and AT-SPI 2 as its inspection and action transport. [GNOME's AT-SPI development documentation](https://gnome.pages.gitlab.gnome.org/at-spi2-core/devel-docs/atspi-python-stack.html) states that Orca builds a view of an application's accessible-object tree through `libatspi` and `pyatspi2`. | KK | Not required. P3 establishes the Ubuntu package lock and candidate behavior. |
| AT-SPI documented text-offset unit | The normative [AT-SPI 2.60.6 `Text.xml`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/Text.xml) defines `CharacterCount` as a number of characters that can differ from fetched UTF-8 byte count. It defines `GetText` end offsets as the first character past the range, while the UTF-8 result bytes can exceed those offsets. It also states that `GetCharacterAtOffset` returns "the UCS-4 unicode code point of the given character." Therefore the documented AT-SPI text, caret, selection, and editable-position unit is a Unicode code-point (scalar) boundary, not a UTF-8 byte, UTF-16 unit, or grapheme boundary. The independent conversion fixture verifies only the conversion tables. | KK | Not required for the documented unit or conversion mechanics. The next row retains the behavior gate. |
| AT-SPI text, caret, selection, and editable behavior | The host has no `org.a11y.Bus`, and the fixture makes no AT-SPI calls. The AT-SPI source establishes the unit, not that a selected GTK exporter or either candidate applies it consistently to `GetText`, `CaretOffset`, selections, `SetCaretOffset`, and editable operations on the combining fixture. | KU (gating) | P3: On the P1 Ubuntu session, start a headless accessibility bus with `dbus-run-session` and `at-spi-bus-launcher`, then use a noncandidate GTK text widget and `pyatspi2` to record `CharacterCount`, `GetText`, `CaretOffset`, selection bounds, `SetCaretOffset`, and editable-operation results for every fixture. Expected output: an AT-SPI transcript whose combining fixture distinguishes scalar offsets 1 and 2, whose text range is UTF-8 only in the returned value, and whose results identify rejected interior boundaries. |
| Focused allocation accessibility map | [GTK defines an accessibility tree](https://docs.gtk.org/gtk4/iface.Accessible.html) with role, state, property, and relation attributes and a platform accessibility context. No focused candidate source identity, exported tree, forward map, reverse action map, artifact path, or digest exists. | KU (gating) | P3F: After the focused source identity is locked, launch its two-view AT-SPI fixture under Orca and `pyatspi2`. Enumerate every required `accessibility-map.schema.json` forward key and reverse action, including Unicode-scalar text payloads, view generation, acknowledgement, stale target, and secure-field redaction. Expected output: one complete map JSON file and SHA-256. |
| Integrated allocation accessibility map | The [GTK accessibility interfaces](https://docs.gtk.org/gtk4/iface.Accessible.html) document a possible host mechanism, but they do not establish the pinned Flutter fork's inherited interfaces or its Oxyflut map. No fork commit, source tree, exported tree, forward map, reverse action map, artifact path, or digest exists. | KU (gating) | P3I: After the integrated fork and adapter commits are locked, run the same two-view Orca and `pyatspi2` fixture. First inventory inherited GTK and AT-SPI interfaces, then enumerate every forward key and reverse action. Expected output: the inventory, one complete map JSON file, and SHA-256. |
| Host scheduling and presentation feedback roles | [`GdkFrameClock`](https://docs.gtk.org/gdk4/class.FrameClock.html) tells an application when to update and repaint, but GTK states that it can use a simple timer instead of hardware vertical sync. The [version-1 presentation-time XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml) creates feedback for a submitted `wl_surface.commit` and emits one terminal presented or discarded result for that content update. Therefore `GdkFrameClock` is only a host wakeup mechanism until P4 qualifies it, and `wp_presentation` feedback is acknowledgement only, never an independent opportunity meter. | KK | Not required for the interface-role decision. P4 qualifies the meter and scheduling behavior. |
| Independent presentation-opportunity meter | No fetched compositor evidence or host probe proves an output-associated timing source that is independent of both candidate callback streams; the [version-1 presentation-time XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml) only describes per-submission feedback. The host exposes a DRM card node and a `wp_drm_lease_device_v1` global, but that does not prove KMS authority, active-output attribution, calibration, or reference-session behavior. | KU (gating) | P4: On the P1 session, run a separately launched, harness-owned visible Wayland client with its own `wl_surface.frame` callbacks and monotonic log beside each candidate. Bind the observer and candidate surfaces to each entered output set, prove no shared callback or IPC path, compare 10-second epochs against an independently captured display trace, and record calibration error. Expected output: observer source digest, process graph, per-output epoch log, and calibration result that meets `CON-FRM-001`. |
| Output association mechanism | The [core protocol](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml) states that a surface can be displayed on zero or more outputs. It emits `wl_surface.enter` and `wl_surface.leave` when surface creation, movement, or resizing changes output membership. | KK | Not required for protocol mechanics. P4 and P5 apply the mechanism to each allocation. |
| Focused allocation service routing | No focused candidate exists to prove that every GTK, Wayland, IME, accessibility, clipboard, timing, and recovery request carries its owning `GdkSurface` and view generation across the reentrancy barrier; the [core protocol](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml) only establishes surface identity mechanics. | KU (gating) | P5F: Use an instrumented two-window focused fixture. Interleave focus, IME, AT-SPI reverse action, clipboard, output move, close, and late-callback events. Expected output: normalized event log in which every request has the expected surface identity and live view generation, and stale events return the defined error. |
| Integrated allocation service routing | No pinned Flutter fork or adapter exists to prove that every inherited callback carries its owning `GdkSurface` and view generation before the C ABI and reentrancy barrier; the [core protocol](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml) does not identify inherited Flutter callbacks. | KU (gating) | P5I: Run the P5F scenario through the locked integrated fork. Expected output: inherited-interface inventory and normalized C-ABI event log with the same ownership, generation, and stale-event results. |
| Focused allocation recovery injection | The [Vulkan registry](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/20a9e5892e2aab7b9776b16a238b10fc8133090a/xml/vk.xml) defines `VK_ERROR_DEVICE_LOST`, but it does not provide an injectable focused-candidate recovery baseline. The focused allocation has no source identity, fault seam, surface-loss control, retry trace, or recovery evidence. | KU (gating) | P6F: After the focused source identity is locked, expose test-only commands that inject resize completion, surface loss, resume or topology change, and `VK_ERROR_DEVICE_LOST` at the adapter boundary. Run each fault during a two-view fixture and apply the recovery pass rule in this report. Expected output: the fault timestamp, valid and correctly sized acknowledged output, preserved framework state, three-or-fewer recreation attempts, transient-allocation ratio, superseded-resource release time, and a structured terminal error when recovery fails. |
| Integrated allocation recovery injection | The [Vulkan device-loss result](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/20a9e5892e2aab7b9776b16a238b10fc8133090a/xml/vk.xml) does not establish the Flutter fork's lifecycle or graphics recovery path. The integrated allocation has no fork commit, test-only fault seam, retry trace, or recovery evidence. | KU (gating) | P6I: After the fork and adapter commits are locked, expose the same test-only fault commands at the normalized C ABI and run the P6F fixture. Apply the recovery pass rule in this report. Expected output: the same recovery record fields and equivalent pass or terminal-error behavior. |

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

### Unicode-scalar offset fixture

The pinned AT-SPI 2.60.6 `Text.xml` source is normative for the unit. It states that `CharacterCount` can differ from UTF-8 byte count, that `GetText` uses character range offsets while returning UTF-8, and that `GetCharacterAtOffset` returns "the UCS-4 unicode code point of the given character." The fixture therefore tests conversions from that documented code-point unit. It does not declare Python string indexes to be AT-SPI boundaries and it does not substitute for an AT-SPI call.

The full fixture script at `/tmp/wf-epic-b/OXY-B003/review-fix/offset-fixtures-independent.py` uses hand-listed code points and expected tables. Scalar-to-encoding conversion encodes a prefix. Encoding-to-scalar conversion independently decodes a raw prefix and counts UTF-32 code points. It rejects UTF-8 and UTF-16 interior offsets, and rejects the combining mark's scalar offset as a grapheme boundary.

```python
from dataclasses import dataclass


@dataclass(frozen=True)
class Fixture:
    name: str
    code_points: tuple[int, ...]
    scalar_boundaries: tuple[int, ...]
    utf8_bytes: tuple[int, ...]
    utf16_units: tuple[int, ...]
    grapheme_boundaries: tuple[int, ...]
    logical_positions: tuple[int, ...]
    rejected_utf8_bytes: tuple[int, ...]
    rejected_utf16_units: tuple[int, ...]


FIXTURES = (
    Fixture("ASCII", (0x0061, 0x0062, 0x005A), (0, 1, 2, 3), (0, 1, 2, 3), (0, 1, 2, 3), (0, 1, 2, 3), (0, 1, 2, 3), (), ()),
    Fixture("multibyte", (0x0041, 0x754C, 0x1F600), (0, 1, 2, 3), (0, 1, 4, 8), (0, 1, 2, 4), (0, 1, 2, 3), (0, 1, 2, 3), (2, 3, 5, 6, 7), (3,)),
    Fixture("combining", (0x0065, 0x0301, 0x0078), (0, 1, 2, 3), (0, 1, 3, 4), (0, 1, 2, 3), (0, 2, 3), (0, 1, 2, 3), (2,), ()),
    Fixture("bidirectional", (0x0041, 0x05D0, 0x0042), (0, 1, 2, 3), (0, 1, 3, 4), (0, 1, 2, 3), (0, 1, 2, 3), (0, 1, 2, 3), (2,), ()),
)


def text_from_code_points(code_points: tuple[int, ...]) -> str:
    return "".join(chr(code_point) for code_point in code_points)


def scalar_to_utf8_byte(text: str, scalar_offset: int) -> int:
    if scalar_offset < 0 or scalar_offset > len(text):
        raise ValueError("scalar offset outside text")
    return len(text[:scalar_offset].encode("utf-8"))


def utf8_byte_to_scalar(text: str, byte_offset: int) -> int:
    utf8 = text.encode("utf-8")
    if byte_offset < 0 or byte_offset > len(utf8):
        raise ValueError("UTF-8 byte offset outside text")
    try:
        prefix = utf8[:byte_offset].decode("utf-8")
    except UnicodeDecodeError as error:
        raise ValueError("UTF-8 byte offset is inside a scalar") from error
    return len(prefix.encode("utf-32-le")) // 4


def scalar_to_utf16_unit(text: str, scalar_offset: int) -> int:
    if scalar_offset < 0 or scalar_offset > len(text):
        raise ValueError("scalar offset outside text")
    return len(text[:scalar_offset].encode("utf-16-le")) // 2


def utf16_unit_to_scalar(text: str, unit_offset: int) -> int:
    utf16 = text.encode("utf-16-le")
    if unit_offset < 0 or unit_offset > len(utf16) // 2:
        raise ValueError("UTF-16 unit offset outside text")
    try:
        prefix = utf16[: unit_offset * 2].decode("utf-16-le")
    except UnicodeDecodeError as error:
        raise ValueError("UTF-16 unit offset is inside a scalar") from error
    return len(prefix.encode("utf-32-le")) // 4


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
    print(f"fixture={fixture.name} repr={text!r}")
    print("expected_scalar|expected_utf8_byte|expected_utf16_unit|expected_grapheme|expected_logical")
    for row in zip(
        fixture.scalar_boundaries,
        fixture.utf8_bytes,
        fixture.utf16_units,
        tuple(grapheme_index(fixture.grapheme_boundaries, scalar) if scalar in fixture.grapheme_boundaries else "not-boundary" for scalar in fixture.scalar_boundaries),
        fixture.logical_positions,
    ):
        print("|".join(str(value) for value in row))
    for scalar_offset, expected_utf8, expected_utf16 in zip(fixture.scalar_boundaries, fixture.utf8_bytes, fixture.utf16_units):
        assert scalar_to_utf8_byte(text, scalar_offset) == expected_utf8
        assert utf8_byte_to_scalar(text, expected_utf8) == scalar_offset
        assert scalar_to_utf16_unit(text, scalar_offset) == expected_utf16
        assert utf16_unit_to_scalar(text, expected_utf16) == scalar_offset
    assert rejected_utf8_offsets(text, fixture.utf8_bytes) == fixture.rejected_utf8_bytes
    assert rejected_utf16_offsets(text, fixture.utf16_units) == fixture.rejected_utf16_units
    rejected_graphemes = tuple(
        scalar_offset
        for scalar_offset in fixture.scalar_boundaries
        if scalar_offset not in fixture.grapheme_boundaries
    )
    print(f"utf8_interior_rejected={fixture.rejected_utf8_bytes}")
    print(f"utf16_interior_rejected={fixture.rejected_utf16_units}")
    print(f"grapheme_interior_rejected={rejected_graphemes}")
    print()

print("result=all hand-listed scalar boundaries round-trip through UTF-8 bytes and UTF-16 units; interior encoding offsets and non-grapheme scalar offsets are rejected")
```

The fixture ran with the following command and exact captured output:

```text
$ nix shell nixpkgs#python3 -c python3 /tmp/wf-epic-b/OXY-B003/review-fix/offset-fixtures-independent.py
fixture=ASCII repr='abZ'
expected_scalar|expected_utf8_byte|expected_utf16_unit|expected_grapheme|expected_logical
0|0|0|0|0
1|1|1|1|1
2|2|2|2|2
3|3|3|3|3
utf8_interior_rejected=()
utf16_interior_rejected=()
grapheme_interior_rejected=()

fixture=multibyte repr='A界😀'
expected_scalar|expected_utf8_byte|expected_utf16_unit|expected_grapheme|expected_logical
0|0|0|0|0
1|1|1|1|1
2|4|2|2|2
3|8|4|3|3
utf8_interior_rejected=(2, 3, 5, 6, 7)
utf16_interior_rejected=(3,)
grapheme_interior_rejected=()

fixture=combining repr='éx'
expected_scalar|expected_utf8_byte|expected_utf16_unit|expected_grapheme|expected_logical
0|0|0|0|0
1|1|1|not-boundary|1
2|3|2|1|2
3|4|3|2|3
utf8_interior_rejected=(2,)
utf16_interior_rejected=()
grapheme_interior_rejected=(1,)

fixture=bidirectional repr='AאB'
expected_scalar|expected_utf8_byte|expected_utf16_unit|expected_grapheme|expected_logical
0|0|0|0|0
1|1|1|1|1
2|3|2|2|2
3|4|3|3|3
utf8_interior_rejected=(2,)
utf16_interior_rejected=()
grapheme_interior_rejected=()

result=all hand-listed scalar boundaries round-trip through UTF-8 bytes and UTF-16 units; interior encoding offsets and non-grapheme scalar offsets are rejected
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
- [GTK `GtkIMContext` API](https://docs.gtk.org/gtk4/class.IMContext.html).
- [GTK `GtkIMContext.set_surrounding` API](https://docs.gtk.org/gtk4/method.IMContext.set_surrounding.html).
- [GTK `GtkIMContext::delete-surrounding` signal](https://docs.gtk.org/gtk4/signal.IMContext.delete-surrounding.html).
- [GTK `GtkIMContext:input-purpose` property](https://docs.gtk.org/gtk4/property.IMContext.input-purpose.html).
- [GTK `GtkInputPurpose` enumeration](https://docs.gtk.org/gtk4/enum.InputPurpose.html).
- [GTK `GtkIMContext:input-hints` property](https://docs.gtk.org/gtk4/property.IMContext.input-hints.html).
- [GTK `GtkInputHints` flags](https://docs.gtk.org/gtk4/flags/InputHints.html).
- [GTK `GtkAccessible` API](https://docs.gtk.org/gtk4/iface.Accessible.html).
- [GTK `GtkAccessibleText` API](https://docs.gtk.org/gtk4/iface.AccessibleText.html).
- [GTK `GdkFrameClock` API](https://docs.gtk.org/gdk4/class.FrameClock.html).
- [Orca screen-reader documentation](https://help.gnome.org/users/orca/stable/).
- [GNOME AT-SPI documentation for Orca and `libatspi`](https://gnome.pages.gitlab.gnome.org/at-spi2-core/devel-docs/atspi-python-stack.html).
- [AT-SPI 2.60.6 `Text.xml`](https://gitlab.gnome.org/GNOME/at-spi2-core/-/raw/2.60.6/xml/Text.xml).
- [AT-SPI `Text.get_text` API](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/method.Text.get_text.html).
- [AT-SPI `Text.get_character_at_offset` API](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/method.Text.get_character_at_offset.html).
- [AT-SPI `EditableText` API](https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/iface.EditableText.html).
- [Version-1 Wayland presentation-time protocol XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml).
- [Version-2 Wayland presentation-time protocol XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/8cdb39103247fdde5764fc35b1b5cf60698db3e5/stable/presentation-time/presentation-time.xml).
- [Pinned Wayland presentation-time protocol XML](https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/d5aed4e4903a77aefaef03359d1ffdc0d5093456/stable/presentation-time/presentation-time.xml).
- [Pinned Wayland core protocol XML](https://raw.githubusercontent.com/wayland-mirror/wayland/1ab6b693b16e1d9734496fe60c8a6ed277e4dec3/protocol/wayland.xml).
- [Pinned Vulkan registry XML](https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/20a9e5892e2aab7b9776b16a238b10fc8133090a/xml/vk.xml).

The pinned source probe produced these immutable content digests:

```text
presentation-time-v1 sha256=91e5e14481a13717fef8403203a2eaa052c85fd853c1c440ba081effa7178d12
presentation-time-v2 sha256=dffac93bcb2bb1d8c385e72b8a8c2c0d4d79a336866322f3ba886dce2b27b1e2
presentation-time-pinned sha256=dffac93bcb2bb1d8c385e72b8a8c2c0d4d79a336866322f3ba886dce2b27b1e2
core-wayland-pinned sha256=7eb8569529235c85e16d15612fc367da4538b7d515b13e32ec48ba0742c42610
vulkan-registry-pinned sha256=3ff4984b841932e04eeb4ce2a6613ebd37c00ffb2e96549785b2c5d7da9e1d
gtk-4.20.4-official-sum=a21f825bd44afc4dd99ba4eea8ff57c8f2e51085cb402a68ed4cbb35299826a4
```

## Options and trade-offs

- **Option A:** Freeze the selected Ubuntu compositor session, package manifest, and protocol registry only after P1 records compositor/version evidence and the visible-surface transcript. This is required for a reference baseline, but it is not complete in this spike.
- **Option B:** Use a separately launched, harness-owned Wayland client with its own visible `wl_surface.frame` callback stream as the prospective opportunity observer. It has a separate process and callback path, but P4 must establish output attribution and timestamp calibration before it becomes a meter.
- **Option C:** Keep candidate behavior and environment-dependent rows as gating KUs. This prevents the reference distribution label, protocol advertisement, `GdkFrameClock`, or per-commit feedback from becoming unearned qualification evidence.

## Recommendation

- **Chosen option:** Use a mix of A, B, and C. Freeze the version-1 protocol mechanics from cited upstream sources. Use Orca and AT-SPI 2 with documented Unicode-scalar offsets for the common accessibility baseline. Require the Option B observer design for P4, and retain Option C for every unproven reference-session and candidate-specific row.
- **Why it fits:** Version 1 has the required acknowledgement and output-association operations. Version 2 only changes the variable-refresh `refresh` obligation, which the harness does not need. Retaining a KU for session availability prevents a protocol-floor fact from becoming a compositor claim. The recommendation gives both allocations documented IME and AT-SPI conversions without converting an interface description into candidate behavior. The observer design is structurally separate from either candidate, while P4 retains the required proof of independence and calibration.
- **Rejected options:** Reject a nominal refresh-rate timer, `wp_presentation` feedback as an opportunity source, a protocol-global list as compositor behavior, an unspecified assistive technology, a global IME index unit for every operation, and a candidate map inferred from GTK documentation.
- **Sensitive-field rule:** Set `GtkInputPurpose` to `PASSWORD` or `PIN` as applicable and set `GtkInputHints.PRIVATE`. Continue to provide only protocol-required redacted surrounding context and never emit raw text to diagnostics. GTK describes the hint as a request, not a privacy guarantee; P2 and P3 must verify the redaction path.

### Spec edits required

Stage 3 can make the following exact edits without changing product capabilities or architecture boundaries:

- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.protocols` -> entry where `name` is `wp_presentation`: set `version` to `"1"`, retain `status` as `"ku-gating"`, and set `evidence` to `[{"path":"https://raw.githubusercontent.com/wayland-mirror/wayland-protocols/37a1560cf6981a11d44dd200d9409d09b4f0074e/stable/presentation-time/presentation-time.xml","sha256":"91e5e14481a13717fef8403203a2eaa052c85fd853c1c440ba081effa7178d12"}]`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.openQuestions`: add `"P1 must prove the selected Ubuntu session advertises wp_presentation version 1 or later and emits its required acknowledgement and output-association transcript."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.minimumVersion`: retain `status` as `"ku-gating"`, retain `value` as `null`, and add `"Ubuntu 26.04 compositor/session/package-manifest evidence from P1"` to `openQuestions`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.ime.numericNegotiation`: replace the value with `"Use the writable Gtk.InputPurpose and Gtk.InputHints properties for each focus generation; no project-defined numeric handshake exists. Surrounding cursor and anchor positions use UTF-8 bytes. P2 must establish every other GtkIMContext operation unit."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.interactiveOpportunitySource`: replace the value with `"GdkFrameClock is a host wakeup only; each allocation must prove output-associated display-synchronized scheduling in P4."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.independentMeterSource`: replace the value with `"A separately launched harness-owned visible Wayland client with its own wl_surface.frame callback and monotonic log; it is a meter only after P4 proves output association, timestamp calibration, and no shared candidate callback or IPC path."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.presentationFeedback`: replace the value with `"wp_presentation v1 feedback for per-commit acknowledgement and main-output association only; never an independent presentation-opportunity meter."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.timing.perDisplayAssociation`: replace the value with `"Track each wl_surface enter/leave output set and begin a display epoch on every set change. Use wp_presentation_feedback.sync_output only to label a submitted frame's main output."`.
- `.constitution/tech-spec/contracts/platform-contracts.json` -> `environments.wayland.accessibilityMaps`, `recoveryBaseline`, `allocations.focused`, and `allocations.integrated`: retain every `"ku-gating"` status and `null` path/digest until P3F, P3I, P5F, P5I, P6F, and P6I produce the named immutable artifacts.
- `.constitution/tech-spec/stack.md` -> `Platform qualification pins` -> `Wayland` row: replace `"minimum compositor and protocol versions are gating KUs"` with `"the Ubuntu compositor/session package manifest and the selected session's availability of wp_presentation v1 remain gating KUs; version 1 supplies per-commit acknowledgement and output association, and P1 must record the selected session's package versions, manifest digest, registry, and visible-surface transcript"`.
- `.constitution/tech-spec/contracts/qualification-lock.json` -> `preImplementationKnownUnknowns` and `gatingKnownUnknowns`: add `"wayland-ubuntu-compositor-session-package-lock"`, `"wayland-wp-presentation-v1-reference-session-transcript"`, `"wayland-ime-operation-unit-transcript"`, `"wayland-atspi-text-caret-selection-editable-transcript"`, `"wayland-orca-atspi-maps-for-both-allocations"`, `"wayland-independent-observer-calibration"`, `"wayland-service-routing-for-both-allocations"`, and `"wayland-recovery-injection-for-both-allocations"`.
- `.constitution/tech-spec/adrs/ADR-0005-platform-hosts.md` -> `Consequences`: add `"Wayland qualification uses wp_presentation v1 for per-commit acknowledgement and output association, not as the independent presentation-opportunity meter. P1 must prove session availability before qualification."`.

## Downstream impact

- **ADRs to write or update:** Stage 3 updates `ADR-0005-platform-hosts.md` with the `wp_presentation` boundary. `ADR-0006-execution-domains.md` requires no change because the report does not alter its queue or ownership boundary.
- **Tickets unblocked in `tasks/active/`:** `OXY-D001` can consume the documented protocol and conversion mechanics, but it remains blocked from qualification measurements by P1 through P6.
- **Tickets to add or split:** Add P1 through P6 as bounded Wayland evidence tasks if the Stage 4 plan does not already schedule equivalent probes.
- **Remaining gates:** The 12 KU rows retain the Wayland environment as `ku-gating`. Neither allocation is eligible for scoring until they close.
