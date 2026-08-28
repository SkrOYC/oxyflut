# Reference hardware access register

- Ticket: OXY-B007
- Status: completed access register
- Clock start: 2026-08-28T16:50:42Z
- Clock stop: 2026-08-28T16:52:50Z

## Purpose and scope

This register records owner-confirmed access information for Tier 1 environments. It is not qualification evidence, does not set readiness, and does not establish hardware, driver, package-lock, capability, performance, or score results.

A `CONFIRMED` row confirms an accountable owner, usable access procedure, and repeatable access window. It does not mean that the row conforms to the Stage 3 reference environment. `BLOCKED` means that no accountable owner and no usable access procedure were recorded, so the row stops at the stated unblock probe.

`Second-configuration score-4 evidence` means evidence from a physically distinct hardware configuration. It is not a candidate score and is outside this register's scope.

## Owner attestation

On 2026-08-28, Oscar Y. <oscar@ocmasesorias.com> confirmed during this session that they are the accountable owner of `thinkpadp14s`, have local interactive access and administrator rights, and consent to its use for the Wayland x86-64 and X11 x86-64 rows. The declared X11 access paths are Xwayland and Xvfb.

On 2026-08-28, Oscar Y. <oscar@ocmasesorias.com> confirmed: "The owner operates the machine and can run qualification sessions at any time on request; there is no notice requirement; sessions are owner-operated and are not left unattended."

No macOS arm64 or Windows x86-64 machine, accountable owner, or access procedure was supplied for this register. Those rows are blocked under the ticket STOP condition.

## Question

| ID | Question | Environment |
| :-- | :-- | :-- |
| B007-Q01 | Does a named accountable owner and usable procedure provide macOS arm64 access? | macOS arm64 |
| B007-Q02 | Does a named accountable owner and usable procedure provide Windows x86-64 access? | Windows x86-64 |
| B007-Q03 | Does a named accountable owner and usable procedure provide Wayland x86-64 access? | Wayland x86-64 |
| B007-Q04 | Does a named accountable owner and usable procedure provide X11 x86-64 access? | X11 x86-64 |
| B007-Q05 | Can the recorded macOS arm64 configuration be compared with its Stage 3 reference environment? | macOS arm64 |
| B007-Q06 | Can the recorded Windows x86-64 configuration be compared with its Stage 3 reference environment? | Windows x86-64 |
| B007-Q07 | Does the available Wayland x86-64 configuration conform to the Stage 3 Ubuntu 26.04 LTS reference? | Wayland x86-64 |
| B007-Q08 | Does the available X11 x86-64 configuration conform to the Stage 3 Ubuntu 26.04 LTS reference? | X11 x86-64 |
| B007-Q09 | Is a physically distinct second configuration available for macOS arm64 score-4 evidence? | macOS arm64 |
| B007-Q10 | Is a physically distinct second configuration available for Windows x86-64 score-4 evidence? | Windows x86-64 |
| B007-Q11 | Is a physically distinct second configuration available for Wayland x86-64 score-4 evidence? | Wayland x86-64 |
| B007-Q12 | Is a physically distinct second configuration available for X11 x86-64 score-4 evidence? | X11 x86-64 |

## Answers

| ID | Status | Answer | Citation | Next bounded probe for KU rows |
| :-- | :-- | :-- | :-- | :-- |
| B007-Q01 | KU (gating) | No macOS machine, accountable owner, or access procedure is recorded. STOP: this row is blocked. | Owner attestation | On an identified macOS machine, its owner must run `uname -m; sw_vers; system_profiler SPHardwareDataType SPDisplaysDataType; xcodebuild -version` and preserve the output in `/tmp/wf-epic-b/B007-macos/owner-confirmation.txt`. The owner must date and sign a confirmation of physical machine identity, GPU, CPU, RAM, interactive-session access, administrator requirements, repeatable access window, consent, and physical distinctness. Expected output identifies `arm64`, macOS release and build, hardware and display inventory, and Xcode and SDK versions. |
| B007-Q02 | KU (gating) | No Windows machine, accountable owner, or access procedure is recorded. STOP: this row is blocked. | Owner attestation | On an identified Windows machine, its owner must run `hostname; Get-CimInstance Win32_OperatingSystem; Get-CimInstance Win32_ComputerSystem; Get-CimInstance Win32_Processor; Get-CimInstance Win32_VideoController` in PowerShell and preserve the output in `C:\Temp\wf-epic-b\B007-windows\owner-confirmation.txt`. The owner must date and sign a confirmation of x86-64 architecture, interactive desktop access, administrator requirements, repeatable access window, consent, and physical distinctness. Expected output identifies the Windows release and build, machine, CPU, RAM, GPU, and driver. |
| B007-Q03 | KK | Oscar Y. owns `thinkpadp14s`; the owner attested to local interactive access, administrator rights, and consent for Wayland use. The owner confirmed scheduling constraints of "on request, any time, no notice requirement" and a repeatable access window of "owner-operated sessions on request; not unattended." The preserved probe identifies an x86-64 NixOS host in a Wayland Hyprland session. | Owner attestation; [host discovery probe](#host-discovery-probe) | - |
| B007-Q04 | KK | Oscar Y. owns the same `thinkpadp14s`; the owner attested to local interactive access, administrator rights, and consent for the X11 compatibility path. The owner confirmed scheduling constraints of "on request, any time, no notice requirement" and a repeatable access window of "owner-operated sessions on request; not unattended." The preserved probe identifies active `Xwayland :0`, queries its server and extensions, and has `xwininfo` connect to its root window. A separately launched `Xvfb :99` also accepts an `xdpyinfo` connection and is then stopped. | Owner attestation; [X11 access probe](#x11-access-probe) | - |
| B007-Q05 | KU (gating) | No macOS configuration exists in this register to compare with the required arm64 macOS 26.5 SDK reference. | [Stage 3 platform pins](../tech-spec/stack.md#platform-qualification-pins) | Complete B007-Q01's owner confirmation. Compare its `sw_vers` and `xcodebuild -version` output against the pinned macOS 26.5 SDK and Xcode 26.6 build `17F113`; expected output either matches both pins or records the exact gap. |
| B007-Q06 | KU (gating) | No Windows configuration exists in this register to compare with the required Windows 11 25H2 x86-64 reference. | [Stage 3 platform pins](../tech-spec/stack.md#platform-qualification-pins) | Complete B007-Q02's owner confirmation. Compare its `Win32_OperatingSystem` output against Windows 11 25H2 and record the Visual Studio Build Tools and Windows SDK versions; expected output either matches every pin or records each exact gap. |
| B007-Q07 | KK | No. The available host reports `PRETTY_NAME="NixOS 26.05 (Yarara)"`, not Ubuntu 26.04 LTS. This host is not the Ubuntu 26.04 LTS Wayland reference environment. | [Host discovery probe](#host-discovery-probe); [Stage 3 platform pins](../tech-spec/stack.md#platform-qualification-pins); [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/) | - |
| B007-Q08 | KK | No. The available host reports `PRETTY_NAME="NixOS 26.05 (Yarara)"`, not Ubuntu 26.04 LTS. The confirmed interactive X11 path is Xwayland, and the separate Xvfb path is headless; neither demonstrates a native X11 desktop session. This host is not the Ubuntu 26.04 LTS X11 reference environment. | [Host discovery probe](#host-discovery-probe); [X11 access probe](#x11-access-probe); [Stage 3 platform pins](../tech-spec/stack.md#platform-qualification-pins); [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/) | - |
| B007-Q09 | KU (gating) | No macOS configuration is available, so a distinct second configuration is not established. | Owner attestation | Complete B007-Q01, then have the owner state whether the proposed physical machine is distinct from every other configuration recorded for score-4 evidence. Expected output is a dated identity and distinctness declaration. |
| B007-Q10 | KU (gating) | No Windows configuration is available, so a distinct second configuration is not established. | Owner attestation | Complete B007-Q02, then have the owner state whether the proposed physical machine is distinct from every other configuration recorded for score-4 evidence. Expected output is a dated identity and distinctness declaration. |
| B007-Q11 | KK | No. The Wayland and X11 rows are the same physical `thinkpadp14s` machine. They count as one hardware configuration, so this register cannot provide second-configuration score-4 evidence. | Owner attestation; [host discovery probe](#host-discovery-probe) | - |
| B007-Q12 | KK | No. The X11 row is an Xwayland/Xvfb access path on the same physical `thinkpadp14s` machine as the Wayland row. It counts as one hardware configuration, so this register cannot provide second-configuration score-4 evidence. | Owner attestation; [host discovery probe](#host-discovery-probe) | - |

The answer rows contain six KK findings and six gating KUs. The owner attestation is a human-in-the-loop access input. The command transcript below is preserved probe evidence for host properties. Neither source qualifies an environment.

## Access register

### Identification and hardware

| Environment | Architecture | Machine identity | Accountable owner | OS/distro and version | GPU | CPU | RAM |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |
| macOS arm64 | Required: arm64; no machine recorded | None | None | None observed | None observed | None observed | None observed |
| Windows x86-64 | Required: x86-64; no machine recorded | None | None | None observed | None observed | None observed | None observed |
| Wayland x86-64 | x86_64 | `thinkpadp14s` | Oscar Y. <oscar@ocmasesorias.com> | NixOS 26.05 (Yarara), kernel 6.18.44 | AMD/ATI Renoir Radeon Vega Series / Radeon Vega Mobile Series, PCI `07:00.0` | AMD Ryzen 7 PRO 4750U with Radeon Graphics; 8 cores and 16 logical CPUs | 29 GiB total |
| X11 x86-64 | x86_64 | `thinkpadp14s`; same physical machine as Wayland | Oscar Y. <oscar@ocmasesorias.com> | NixOS 26.05 (Yarara), kernel 6.18.44 | AMD/ATI Renoir Radeon Vega Series / Radeon Vega Mobile Series, PCI `07:00.0` | AMD Ryzen 7 PRO 4750U with Radeon Graphics; 8 cores and 16 logical CPUs | 29 GiB total |

### Session and access procedure

### macOS arm64 access

| Field | Value |
| :-- | :-- |
| Session type, compositor, or X server | No session recorded |
| Interactive-session availability | No access procedure |
| Administrator requirements | Unknown; no owner |
| Access procedure | None |
| Scheduling constraints and repeatable access window | None; blocked pending B007-Q01 |

### Windows x86-64 access

| Field | Value |
| :-- | :-- |
| Session type, compositor, or X server | No session recorded |
| Interactive-session availability | No access procedure |
| Administrator requirements | Unknown; no owner |
| Access procedure | None |
| Scheduling constraints and repeatable access window | None; blocked pending B007-Q02 |

### Wayland x86-64 access

| Field | Value |
| :-- | :-- |
| Session type, compositor, or X server | Active `wayland` session with Hyprland, `WAYLAND_DISPLAY=wayland-1`, and `DISPLAY=:0` |
| Interactive-session availability | Local interactive access confirmed by owner; `loginctl` lists seat session 2 for user `oscar` |
| Administrator requirements | Owner attested to administrator rights; coordinate privileged changes with the owner |
| Access procedure | Coordinate with Oscar Y., use the local interactive Wayland session, and re-run the host discovery probe before a session |
| Scheduling constraints | on request, any time, no notice requirement |
| Repeatable access window | owner-operated sessions on request; not unattended |

### X11 x86-64 access

| Field | Value |
| :-- | :-- |
| Session type, compositor, or X server | Active `Xwayland :0` provides the interactive X11 compatibility path inside the Wayland Hyprland session. `Xvfb :99` was separately launched and queried as a headless path. Neither is a native X11 desktop session. |
| Interactive-session availability | Local interactive Xwayland access is confirmed: `xwininfo -root -display :0` connected and returned the root-window geometry. Xvfb is headless and provides no interactive desktop. |
| Administrator requirements | Owner attested to administrator rights; coordinate privileged changes with the owner |
| Access procedure | Follow the exact [X11 access procedure](#x11-access-procedure). |
| Scheduling constraints | on request, any time, no notice requirement |
| Repeatable access window | owner-operated sessions on request; not unattended |

### Reference conformance and feasibility

| Environment | Reference-environment conformance | Suitability notes | Second-configuration score-4 feasibility | Status |
| :-- | :-- | :-- | :-- | :-- |
| macOS arm64 | Not assessed: no machine is available to compare with the Stage 3 arm64 macOS 26.5 SDK reference. | Hardware and GPU suitability cannot be assessed. | Not established; no configuration is recorded. | BLOCKED |
| Windows x86-64 | Not assessed: no machine is available to compare with the Stage 3 Windows 11 25H2 x86-64 reference. | Hardware and GPU suitability cannot be assessed. | Not established; no configuration is recorded. | BLOCKED |
| Wayland x86-64 | No. NixOS 26.05 is not the Stage 3 Ubuntu 26.04 LTS Wayland reference. | Available for owner-coordinated, non-reference exploratory access only. This register has no GPU driver, package-lock, or measurement evidence. | Not feasible from this register. The Wayland and X11 paths share one physical machine and count as one hardware configuration. | CONFIRMED |
| X11 x86-64 | No. NixOS 26.05 is not the Stage 3 Ubuntu 26.04 LTS X11 reference. The confirmed interactive path is Xwayland, and Xvfb is headless; neither establishes a native X11 desktop session. | Available for owner-coordinated, non-reference X11-compatibility exploration only. The native-session and reference-OS gaps prevent reference use. | Not feasible from this register. The X11 and Wayland paths share one physical machine and count as one hardware configuration. | CONFIRMED |

## Host discovery probe

The following raw output was captured on `thinkpadp14s` during this ticket. The probe records host discovery only; it does not test graphics, X11 server behavior, a compositor protocol, drivers, or qualification capabilities.

```text
$ cat /etc/os-release
ANSI_COLOR="0;38;2;126;186;228"
BUG_REPORT_URL="https://github.com/NixOS/nixpkgs/issues"
BUILD_ID="26.05.20260827.d57af92"
CPE_NAME="cpe:/o:nixos:nixos:26.05"
DEFAULT_HOSTNAME=nixos
DOCUMENTATION_URL="https://nixos.org/learn.html"
HOME_URL="https://nixos.org/"
ID=nixos
ID_LIKE=""
IMAGE_ID=""
IMAGE_VERSION=""
LOGO="nix-snowflake"
NAME=NixOS
PRETTY_NAME="NixOS 26.05 (Yarara)"
SUPPORT_END="2026-12-31"
SUPPORT_URL="https://nixos.org/community.html"
VARIANT=""
VARIANT_ID=""
VENDOR_NAME=NixOS
VENDOR_URL="https://nixos.org/"
VERSION="26.05 (Yarara)"
VERSION_CODENAME=yarara
VERSION_ID="26.05"

$ uname -mr
6.18.44 x86_64

$ lscpu | head -20
Arquitectura:                             x86_64
modo(s) de operación de las CPUs:         32-bit, 64-bit
Tamaños de las direcciones:               48 bits physical, 48 bits virtual
Orden de los bytes:                       Little Endian
CPU(s):                                   16
Lista de la(s) CPU(s) en línea:           0-15
ID de fabricante:                         AuthenticAMD
Nombre del modelo:                        AMD Ryzen 7 PRO 4750U with Radeon Graphics
Familia de CPU:                           23
Modelo:                                   96
Hilo(s) de procesamiento por núcleo:      2
Núcleo(s) por «socket»:                   8
«Socket(s)»:                              1
Revisión:                                 1
Microcode version:                        0x860010d
Aumento de frecuencia:                    activada
CPU(s) factor de escala MHz:              109%
CPU MHz máx.:                             1700,0000
CPU MHz mín.:                             1400,0000
BogoMIPS:                                 3393,37

$ lspci | grep -iE 'vga|3d|display'
07:00.0 VGA compatible controller: Advanced Micro Devices, Inc. [AMD/ATI] Renoir [Radeon Vega Series / Radeon Vega Mobile Series] (rev d1)

$ free -h
               total       usado       libre  compartido   búf/caché  disponible
Mem:            29Gi        10Gi       330Mi       784Mi        19Gi        18Gi
Inter:          14Gi       5,0Gi       9,6Gi

$ echo $XDG_SESSION_TYPE $XDG_CURRENT_DESKTOP $WAYLAND_DISPLAY $DISPLAY
wayland Hyprland wayland-1 :0

$ loginctl list-sessions
SESSION  UID USER  SEAT  LEADER CLASS   TTY  IDLE SINCE
      2 1000 oscar seat0 2801   user    tty2 no   -
      3 1000 oscar -     2817   manager -    no   -

2 sessions listed.

$ which Xvfb Xwayland
/run/current-system/sw/bin/Xvfb
/run/current-system/sw/bin/Xwayland

$ nix --version
nix (Nix) 2.34.8

$ hostname
thinkpadp14s
```

### X11 access procedure

The following procedure establishes access only. It does not qualify graphics, drivers, X11 behavior, or product capabilities.

For interactive Xwayland access:

1. Coordinate with Oscar Y. and use the local Hyprland session on `thinkpadp14s`.
2. Run `pgrep -a Xwayland`. Success is a process line containing `Xwayland :0`, as recorded in the probe.
3. Connect clients to that server by setting `DISPLAY=:0`; do not select a display merely because an executable exists.
4. Run `nix shell nixpkgs#xorg.xdpyinfo -c xdpyinfo -display :0 | head -40`. Success includes `name of display: :0`, `vendor string: The X.Org Foundation`, and `X.Org version: 24.1.13`; its extension list includes `Present` and `RANDR`.
5. Run the Present and XInput extension queries recorded in the probe. Success includes `Present version 1.4` and `XInputExtension version 2.4`. `xdpyinfo -ext RANDR` reports `RANDR extension not supported by xdpyinfo`, so this procedure does not claim a RANDR version.
6. Run `nix shell nixpkgs#xorg.xwininfo -c xwininfo -root -display :0 | head -15`. Success includes `Window id: 0x350 (the root window)`, `Width: 1920`, and `Height: 1080`, demonstrating that the minimal X11 client connected to `:0`.

For headless Xvfb access:

1. Create `/tmp/wf-epic-b/OXY-B007/` and run `Xvfb :99 -screen 0 1280x720x24 >/tmp/wf-epic-b/OXY-B007/xvfb-99.log 2>&1 &`; immediately store its process ID as `XVFB_PID=$!`.
2. Run `nix shell nixpkgs#xorg.xdpyinfo -c xdpyinfo -display :99 | head -15`. Success includes `name of display: :99`, `vendor string: The X.Org Foundation`, and `X.Org version: 21.1.24`.
3. Stop only the server started in step 1 with `kill "$XVFB_PID"; wait "$XVFB_PID"`, then run `pgrep -a Xvfb`. During this probe, `pgrep` exited 1 with no matching process after the stop.

This evidence establishes that X11 clients can connect to active Xwayland on `:0` and to a temporary Xvfb server on `:99`. It does not establish native X11 desktop-session behavior, a native X server session, graphics or driver behavior, any P0 capability, or conformance to the Stage 3 Ubuntu 26.04 LTS X11 reference.

## X11 access probe

The following raw output was captured on `thinkpadp14s` during this ticket. Long `xdpyinfo -ext` reports are trimmed to the relevant raw lines; the retained lines below are the outputs used by this register.

```text
$ pgrep -a Xwayland
3128 Xwayland :0 -rootless -core -listenfd 45 -listenfd 46 -displayfd 94 -wm 91
[pgrep exit: 0]

$ nix shell nixpkgs#xorg.xdpyinfo -c xdpyinfo -display :0 | head -40
name of display:    :0
version number:    11.0
vendor string:    The X.Org Foundation
vendor release number:    12401013
X.Org version: 24.1.13
maximum request size:  16777212 bytes
motion buffer size:  256
bitmap unit, bit order, padding:    32, LSBFirst, 32
image byte order:    LSBFirst
number of supported pixmap formats:    7
supported pixmap formats:
    depth 1, bits_per_pixel 1, scanline_pad 32
    depth 4, bits_per_pixel 8, scanline_pad 32
    depth 8, bits_per_pixel 8, scanline_pad 32
    depth 15, bits_per_pixel 16, scanline_pad 32
    depth 16, bits_per_pixel 16, scanline_pad 32
    depth 24, bits_per_pixel 32, scanline_pad 32
    depth 32, bits_per_pixel 32, scanline_pad 32
keycode range:    minimum 8, maximum 255
focus:  None
number of extensions:    25
    BIG-REQUESTS
    Composite
    DAMAGE
    DOUBLE-BUFFER
    DRI3
    GLX
    Generic Event Extension
    MIT-SHM
    Present
    RANDR
    RECORD
    RENDER
    SECURITY
    SHAPE
    SYNC
    X-Resource
    XC-MISC
    XFIXES
    XFree86-VidModeExtension
[pipeline exit: 141 0]

$ nix shell nixpkgs#xorg.xdpyinfo -c xdpyinfo -display :0 -ext Present
Present version 1.4 opcode: 146
  screen #0 capabilities: 0x19 (PresentCapabilityAsync | PresentCapabilityAsyncMayTear | PresentCapabilitySyncobj)

$ nix shell nixpkgs#xorg.xdpyinfo -c xdpyinfo -display :0 -ext RANDR
RANDR extension not supported by xdpyinfo

$ nix shell nixpkgs#xorg.xdpyinfo -c xdpyinfo -display :0 -ext XInputExtension
XInputExtension version 2.4 opcode: 131, base event: 66, base error: 129
  Extended devices :
    "Virtual core pointer" [XPointer]
    "xwayland-pointer:1" [XExtensionPointer]
    "xwayland-relative-pointer:1" [XExtensionPointer]
    "xwayland-pointer-gestures:1" [XExtensionPointer]
    "xwayland-keyboard:1" [XExtensionKeyboard]

$ nix shell nixpkgs#xorg.xwininfo -c xwininfo -root -display :0 | head -15
evaluation warning: The xorg package set has been deprecated, 'xorg.xwininfo' has been renamed to 'xwininfo'
this path will be fetched (25.3 KiB download, 61.9 KiB unpacked):
  /nix/store/kprk0hfljdxg538ipvlnhc7ibz42xxi4-xwininfo-1.1.6
copying path '/nix/store/kprk0hfljdxg538ipvlnhc7ibz42xxi4-xwininfo-1.1.6' from 'https://cache.nixos.org'...

xwininfo: Window id: 0x350 (the root window) (has no name)

  Absolute upper-left X:  0
  Absolute upper-left Y:  0
  Relative upper-left X:  0
  Relative upper-left Y:  0
  Width: 1920
  Height: 1080
  Depth: 24
  Visual: 0x40
  Visual Class: TrueColor
  Border width: 0
  Class: InputOutput
  Colormap: 0x3f (installed)
[pipeline exit: 0 0]

$ Xvfb :99 -screen 0 1280x720x24 &
[started PID: 1998478]
[Xvfb startup check: running]

$ nix shell nixpkgs#xorg.xdpyinfo -c xdpyinfo -display :99 | head -15
name of display:    :99
version number:    11.0
vendor string:    The X.Org Foundation
vendor release number:    12101024
X.Org version: 21.1.24
maximum request size:  16777212 bytes
motion buffer size:  256
bitmap unit, bit order, padding:    32, LSBFirst, 32
image byte order:    LSBFirst
number of supported pixmap formats:    6
supported pixmap formats:
    depth 1, bits_per_pixel 1, scanline_pad 32
    depth 4, bits_per_pixel 8, scanline_pad 32
    depth 8, bits_per_pixel 8, scanline_pad 32
    depth 16, bits_per_pixel 16, scanline_pad 32
[pipeline exit: 141 0]

$ kill Xvfb_PID; wait Xvfb_PID
[kill and wait exit: 0]

$ pgrep -a Xvfb
[pgrep exit: 1; 1 means no Xvfb process matched]

$ cat /tmp/wf-epic-b/OXY-B007/xvfb-99.log
The XKEYBOARD keymap compiler (xkbcomp) reports:
> Warning:          Multiple symbols for level 1/group 1 on key <FK23>
>                   Using F23, ignoring XF86TouchpadOff
> Warning:          Symbol map for key <FK23> redefined
>                   Using last definition for conflicting fields
> Warning:          Symbol map for key <FK24> redefined
>                   Using last definition for conflicting fields
Errors from xkbcomp are not fatal to the X server
[Xvfb log exit: 0]
```

## Recommendation

Use a mix of option B and option C.

| Environment | Option | Decision and justification |
| :-- | :-- | :-- |
| macOS arm64 | C - remain blocked | Keep the row blocked until B007-Q01 produces a named owner and a usable procedure. No hardware, session, or reference conformance can be inferred. |
| Windows x86-64 | C - remain blocked | Keep the row blocked until B007-Q02 produces a named owner and a usable procedure. No hardware, session, or reference conformance can be inferred. |
| Wayland x86-64 | B - non-reference exploratory access | Use `thinkpadp14s` for owner-operated exploratory access on request, at any time, with no notice requirement. Its NixOS 26.05 environment differs from the Ubuntu 26.04 LTS reference, and the shared hardware cannot supply a second configuration. |
| X11 x86-64 | B - non-reference X11-compatibility access | Use the confirmed Xwayland `:0` path for owner-operated interactive exploration on request, at any time, with no notice requirement, and use the temporary Xvfb `:99` path for headless exploration. Do not treat either path as native X11 or as the Ubuntu 26.04 LTS reference. It is the same hardware configuration as the Wayland row. |

### Spec edits required

None. This access register does not justify a Stage 3 specification edit. In particular, do not add `thinkpadp14s` hardware, GPU, driver, or access information to the qualification lock because the host is not a Stage 3 reference environment and this register does not set readiness.

## Sources

- [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/) - fetched successfully through the Jina reader proxy on 2026-08-28; used only to identify the official Ubuntu 26.04 LTS release named by the Stage 3 reference pins.
