# Reference hardware access register

- Ticket: OXY-B007
- Status: completed access register
- Clock start: 2026-08-28T17:54:20Z
- Clock stop: 2026-08-28T17:55:46Z

## Purpose and scope

This register records owner-confirmed access information for Tier 1 environments. It is not qualification evidence, does not set readiness, and does not establish hardware, driver, package-lock, capability, performance, or score results.

A `CONFIRMED` row confirms an accountable owner, usable access procedure, and repeatable access window. It does not mean that the row conforms to the Stage 3 reference environment. `BLOCKED` means that no accountable owner and no usable access procedure were recorded, so the row stops at the stated unblock probe.

`Second-configuration score-4 evidence` means evidence from a physically distinct hardware configuration. It is not a candidate score and is outside this register's scope.

## Owner attestation

On 2026-08-28, Oscar Y. <oscar@ocmasesorias.com> confirmed during this session that they are the accountable owner of `thinkpadp14s`, have local interactive access and administrator rights, and consent to its use for the Wayland x86-64 and X11 x86-64 rows. The declared X11 access paths are Xwayland and Xvfb.

On 2026-08-28, Oscar Y. <oscar@ocmasesorias.com> confirmed: "The owner operates the machine and can run qualification sessions at any time on request; there is no notice requirement; sessions are owner-operated and are not left unattended."

On 2026-08-28, Oscar Y. <oscar@ocmasesorias.com> confirmed: "I confirm that thinkpadp14s is an x86_64 machine with an AMD Renoir (Radeon Vega) integrated GPU, running NixOS 26.05 with a Hyprland Wayland session and Xwayland/Xvfb for X11."

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
| B007-Q01 | KU (gating) | No macOS machine, accountable owner, or access procedure is recorded. STOP: this row is blocked. | Owner attestation; [Apple Technical Note TN2339](https://developer.apple.com/library/archive/technotes/tn2339/_index.html) | Run the [macOS owner-confirmation probe](#macos-and-windows-owner-confirmation-probes) on an identified machine. Preserve output in `/tmp/wf-epic-b/B007-macos/owner-confirmation.txt`, then append the owner's dated and signed identity, access, consent, and distinctness confirmation. The expected fields are listed with the probe. |
| B007-Q02 | KU (gating) | No Windows machine, accountable owner, or access procedure is recorded. STOP: this row is blocked. | Owner attestation; [Microsoft vswhere README](https://github.com/microsoft/vswhere) | Run the [Windows owner-confirmation probe](#macos-and-windows-owner-confirmation-probes) on an identified machine. Preserve output in `C:\Temp\wf-epic-b\B007-windows\owner-confirmation.txt`, then append the owner's dated and signed identity, access, consent, and distinctness confirmation. The expected fields are listed with the probe. |
| B007-Q03 | KK | Oscar Y. owns `thinkpadp14s`. The owner confirms that it is an x86_64 NixOS 26.05 machine with an AMD Renoir (Radeon Vega) integrated GPU and a Hyprland Wayland session. The owner also attests to local interactive access, administrator rights, and consent for Wayland use. The owner confirms scheduling constraints of "on request, any time, no notice requirement" and a repeatable access window of "owner-operated sessions on request; not unattended." The host-discovery probe corroborates the owner-confirmed environment, architecture, GPU inventory, and Wayland session. | [Owner attestation](#owner-attestation) (HITL confirmation); [host discovery probe](#host-discovery-probe) (corroborating evidence) | - |
| B007-Q04 | KK | Oscar Y. owns the same `thinkpadp14s`. The owner confirms that it is an x86_64 NixOS 26.05 machine with an AMD Renoir (Radeon Vega) integrated GPU, a Hyprland Wayland session, and Xwayland/Xvfb for X11. The owner also attests to local interactive access, administrator rights, and consent for the X11 compatibility path. The owner confirms scheduling constraints of "on request, any time, no notice requirement" and a repeatable access window of "owner-operated sessions on request; not unattended." The host-discovery and X11-access probes corroborate the owner-confirmed environment, architecture, GPU inventory, and session paths. The X11-access probe identifies active `Xwayland :0`, queries its server and extensions, has `xwininfo` connect to its root window, and establishes then stops `Xvfb :99`. | [Owner attestation](#owner-attestation) (HITL confirmation); [host discovery probe](#host-discovery-probe) and [X11 access probe](#x11-access-probe) (corroborating evidence) | - |
| B007-Q05 | KU (gating) | No macOS configuration exists in this register to compare with the required arm64 macOS 26.5 SDK reference. | [Stage 3 platform pins](../tech-spec/stack.md#platform-qualification-pins) | Complete B007-Q01's owner confirmation. Compare `sw_vers`, `xcodebuild -version`, `xcrun --sdk macosx --show-sdk-version`, and `xcrun --sdk macosx --show-sdk-path` with the pinned macOS 26.5 SDK and Xcode 26.6 build `17F113`; preserve output that either matches both pins or records each exact gap. |
| B007-Q06 | KU (gating) | No Windows configuration exists in this register to compare with the required Windows 11 25H2 x86-64 reference. | [Stage 3 platform pins](../tech-spec/stack.md#platform-qualification-pins) | Complete B007-Q02's owner confirmation. Compare the recorded operating-system build, `vswhere` `installationVersion`, Windows SDK include-directory name, and GPU driver version with the Windows 11 25H2, Visual Studio Build Tools 2022 17.14.39, and Windows SDK 10.0.26100.8876 pins; preserve output that either matches every pin or records each exact gap. |
| B007-Q07 | KK | No. The owner confirms that `thinkpadp14s` runs NixOS 26.05 with a Hyprland Wayland session, not Ubuntu 26.04 LTS. The host-discovery probe corroborates this with `PRETTY_NAME="NixOS 26.05 (Yarara)"`. This host is not the Ubuntu 26.04 LTS Wayland reference environment. | [Owner attestation](#owner-attestation) (HITL confirmation); [host discovery probe](#host-discovery-probe) (corroborating evidence); [Stage 3 platform pins](../tech-spec/stack.md#platform-qualification-pins); [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/) | - |
| B007-Q08 | KK | No. The owner confirms that `thinkpadp14s` runs NixOS 26.05 and provides Xwayland/Xvfb for X11, not an Ubuntu 26.04 LTS X11 session. The host-discovery probe corroborates the NixOS environment with `PRETTY_NAME="NixOS 26.05 (Yarara)"`. The X11-access probe corroborates that the interactive path is Xwayland and the separate Xvfb path is headless; neither demonstrates a native X11 desktop session. This host is not the Ubuntu 26.04 LTS X11 reference environment. | [Owner attestation](#owner-attestation) (HITL confirmation); [host discovery probe](#host-discovery-probe) and [X11 access probe](#x11-access-probe) (corroborating evidence); [Stage 3 platform pins](../tech-spec/stack.md#platform-qualification-pins); [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/) | - |
| B007-Q09 | KU (gating) | No macOS configuration is available, so a distinct second configuration is not established. | Owner attestation | Complete B007-Q01, then have the owner state whether the proposed physical machine is distinct from every other configuration recorded for score-4 evidence. Expected output is a dated identity and distinctness declaration. |
| B007-Q10 | KU (gating) | No Windows configuration is available, so a distinct second configuration is not established. | Owner attestation | Complete B007-Q02, then have the owner state whether the proposed physical machine is distinct from every other configuration recorded for score-4 evidence. Expected output is a dated identity and distinctness declaration. |
| B007-Q11 | KK | No. The owner confirms the Wayland and X11 paths are on `thinkpadp14s`, one physical machine. It counts as one hardware configuration, so this register cannot provide second-configuration score-4 evidence. | [Owner attestation](#owner-attestation) (HITL confirmation); [host discovery probe](#host-discovery-probe) (corroborating evidence) | - |
| B007-Q12 | KK | No. The owner confirms that Xwayland/Xvfb are X11 paths on `thinkpadp14s`, the same physical machine as the Wayland row. It counts as one hardware configuration, so this register cannot provide second-configuration score-4 evidence. | [Owner attestation](#owner-attestation) (HITL confirmation); [host discovery probe](#host-discovery-probe) (corroborating evidence) | - |

The answer rows contain six KK findings and six gating KUs. The owner attestation is a human-in-the-loop access input. The command transcript below is preserved probe evidence for host properties. Neither source qualifies an environment.

### macOS and Windows owner-confirmation probes

Apple documents `xcrun` as an Xcode command-line shim. Microsoft documents `vswhere` as the Visual Studio installation-discovery executable at the path used below. These are next probes, not evidence for the blocked rows. Do not infer a result if a command fails. Preserve its error output at the stated path.

On an identified macOS machine, run this command block:

```bash
mkdir -p /tmp/wf-epic-b/B007-macos
{
  uname -m
  sw_vers
  system_profiler SPHardwareDataType SPDisplaysDataType
  xcodebuild -version
  xcrun --sdk macosx --show-sdk-version
  xcrun --sdk macosx --show-sdk-path
} > /tmp/wf-epic-b/B007-macos/owner-confirmation.txt 2>&1
```

The output in `/tmp/wf-epic-b/B007-macos/owner-confirmation.txt` must identify the architecture, macOS release and build, hardware model, CPU, RAM, display and GPU inventory, Xcode version and build, selected macOS SDK version, and selected SDK path. The owner must append a dated and signed confirmation of the physical machine identity, interactive-session access, administrator requirements, repeatable access window, consent, and physical distinctness.

On an identified Windows machine, run this PowerShell block:

```powershell
New-Item -ItemType Directory -Force -Path 'C:\Temp\wf-epic-b\B007-windows' | Out-Null
$evidencePath = 'C:\Temp\wf-epic-b\B007-windows\owner-confirmation.txt'
& {
  hostname
  Get-CimInstance Win32_OperatingSystem | Select-Object Caption, Version, BuildNumber, OSArchitecture
  Get-CimInstance Win32_ComputerSystem | Select-Object Name, Model, TotalPhysicalMemory
  Get-CimInstance Win32_Processor | Select-Object Name, Architecture, AddressWidth
  & "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -products * -requires Microsoft.VisualStudio.Workload.VCTools -property installationVersion
  Get-ChildItem "${env:ProgramFiles(x86)}\Windows Kits\10\Include" | Select-Object Name
  (Get-CimInstance Win32_OperatingSystem).BuildNumber
  Get-CimInstance Win32_VideoController | Select-Object Name, DriverVersion
} *>&1 | Tee-Object -FilePath $evidencePath
```

The output in `C:\Temp\wf-epic-b\B007-windows\owner-confirmation.txt` must identify the machine, x86-64 status, Windows caption, version and build, CPU, RAM, Build Tools `installationVersion`, Windows SDK include-directory names, GPU name, and GPU driver version. The owner must append a dated and signed confirmation of the physical machine identity, interactive desktop access, administrator requirements, repeatable access window, consent, and physical distinctness.

## Access register

### Identification and hardware

| Environment | Architecture | Machine identity | Accountable owner | OS/distro and version | GPU | CPU | RAM |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |
| macOS arm64 | Required: arm64; no machine recorded | None | None | None observed | None observed | None observed | None observed |
| Windows x86-64 | Required: x86-64; no machine recorded | None | None | None observed | None observed | None observed | None observed |
| Wayland x86-64 | x86_64 ([Owner attestation](#owner-attestation), HITL; [host discovery probe](#host-discovery-probe), corroborating) | `thinkpadp14s` ([Owner attestation](#owner-attestation), HITL) | Oscar Y. <oscar@ocmasesorias.com> ([Owner attestation](#owner-attestation), HITL) | NixOS 26.05 (Yarara), kernel 6.18.44 ([Owner attestation](#owner-attestation), HITL; [host discovery probe](#host-discovery-probe), corroborating) | AMD Renoir (Radeon Vega) integrated GPU ([Owner attestation](#owner-attestation), HITL); the probe reports AMD/ATI Renoir Radeon Vega Series / Radeon Vega Mobile Series at PCI `07:00.0` ([host discovery probe](#host-discovery-probe), corroborating) | AMD Ryzen 7 PRO 4750U with Radeon Graphics; 8 cores and 16 logical CPUs ([host discovery probe](#host-discovery-probe)) | 29 GiB total ([host discovery probe](#host-discovery-probe)) |
| X11 x86-64 | x86_64 ([Owner attestation](#owner-attestation), HITL; [host discovery probe](#host-discovery-probe), corroborating) | `thinkpadp14s`, the same physical machine as Wayland ([Owner attestation](#owner-attestation), HITL) | Oscar Y. <oscar@ocmasesorias.com> ([Owner attestation](#owner-attestation), HITL) | NixOS 26.05 (Yarara), kernel 6.18.44 ([Owner attestation](#owner-attestation), HITL; [host discovery probe](#host-discovery-probe), corroborating) | AMD Renoir (Radeon Vega) integrated GPU ([Owner attestation](#owner-attestation), HITL); the probe reports AMD/ATI Renoir Radeon Vega Series / Radeon Vega Mobile Series at PCI `07:00.0` ([host discovery probe](#host-discovery-probe), corroborating) | AMD Ryzen 7 PRO 4750U with Radeon Graphics; 8 cores and 16 logical CPUs ([host discovery probe](#host-discovery-probe)) | 29 GiB total ([host discovery probe](#host-discovery-probe)) |

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
| Session type, compositor, or X server | The owner confirms a Hyprland Wayland session ([Owner attestation](#owner-attestation), HITL); the probe corroborates active `wayland`, `WAYLAND_DISPLAY=wayland-1`, and `DISPLAY=:0` ([host discovery probe](#host-discovery-probe)). |
| Interactive-session availability | The owner confirms local interactive access ([Owner attestation](#owner-attestation), HITL); `loginctl` corroborates seat session 2 for user `oscar` ([host discovery probe](#host-discovery-probe)). |
| Administrator requirements | The owner attests to administrator rights ([Owner attestation](#owner-attestation), HITL); coordinate privileged changes with the owner. |
| Access procedure | Coordinate with Oscar Y. and use the local interactive Wayland session ([Owner attestation](#owner-attestation), HITL); re-run the host-discovery probe before a session. |
| Scheduling constraints | on request, any time, no notice requirement ([Owner attestation](#owner-attestation), HITL) |
| Repeatable access window | owner-operated sessions on request; not unattended ([Owner attestation](#owner-attestation), HITL) |

### X11 x86-64 access

| Field | Value |
| :-- | :-- |
| Session type, compositor, or X server | The owner confirms Xwayland/Xvfb for X11 on the Hyprland Wayland host ([Owner attestation](#owner-attestation), HITL). The X11-access probe corroborates active `Xwayland :0` as the interactive compatibility path and a separately launched `Xvfb :99` as headless; neither is a native X11 desktop session. |
| Interactive-session availability | The owner confirms local interactive access for the X11 compatibility path ([Owner attestation](#owner-attestation), HITL). The X11-access probe corroborates that `xwininfo -root -display :0` connected and returned the root-window geometry; Xvfb is headless and provides no interactive desktop. |
| Administrator requirements | The owner attests to administrator rights ([Owner attestation](#owner-attestation), HITL); coordinate privileged changes with the owner. |
| Access procedure | Coordinate with Oscar Y. before following the exact [X11 access procedure](#x11-access-procedure) ([Owner attestation](#owner-attestation), HITL). |
| Scheduling constraints | on request, any time, no notice requirement ([Owner attestation](#owner-attestation), HITL) |
| Repeatable access window | owner-operated sessions on request; not unattended ([Owner attestation](#owner-attestation), HITL) |

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

1. Run the exact script preserved in the [X11 access probe](#x11-access-probe). It first creates `/tmp/wf-epic-b/OXY-B007` and exits 1 if that fails, then creates a unique log directory below it with `mktemp -d` and exits 1 if that fails. It writes the server log to `$LOGDIR/xvfb99.log` and records the server process ID in `XVFB_PID`.
2. The script runs `nix shell nixpkgs#xorg.xdpyinfo -c xdpyinfo -display :99 | head -15`. Success includes `name of display: :99`, `vendor string: The X.Org Foundation`, and `X.Org version: 21.1.24`.
3. The script signals only its recorded process with `kill "$XVFB_PID"`, then waits for that exact process with `wait "$XVFB_PID"`. The recorded `wait exit: 0` establishes that the child process terminated successfully. The script then runs `pgrep -a Xvfb` and prints its status. [pgrep(1)](https://man7.org/linux/man-pages/man1/pgrep.1.html) defines exit 1 as no matching processes and exits 2 and 3 as command-line and fatal errors. The recorded `pgrep exit: 1` therefore establishes that no host process matched `Xvfb` when the probe ended.

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
```

The following Bash script was executed exactly. It creates `/tmp/wf-epic-b/OXY-B007` before calling `mktemp`, exits 1 if either operation fails, preserves the server log, records the started process ID, signals and waits for that exact process, and separately records the process-wide `pgrep` status:

```bash
#!/usr/bin/env bash
set -u

BASE_DIR=/tmp/wf-epic-b/OXY-B007
mkdir -p "$BASE_DIR" || exit 1
LOGDIR=$(mktemp -d "$BASE_DIR/xvfb.XXXXXX") || exit 1
LOGFILE="$LOGDIR/xvfb99.log"
echo "log directory: $LOGDIR"
Xvfb :99 -screen 0 1280x720x24 >"$LOGFILE" 2>&1 &
XVFB_PID=$!
echo "Xvfb PID: $XVFB_PID"
sleep 2
nix shell nixpkgs#xorg.xdpyinfo -c xdpyinfo -display :99 | head -15
kill "$XVFB_PID"
KILL_STATUS=$?
wait "$XVFB_PID"
WAIT_STATUS=$?
echo "kill exit: $KILL_STATUS"
echo "wait exit: $WAIT_STATUS"
pgrep -a Xvfb
PGREP_STATUS=$?
echo "pgrep exit: $PGREP_STATUS"
head -5 "$LOGFILE"
```

The script was invoked as `bash /tmp/wf-epic-b/OXY-B007/xvfb-rerun.sh > /tmp/wf-epic-b/OXY-B007/xvfb-rerun.raw 2>&1`, which preserves both standard output and standard error in the raw transcript below.

```text
log directory: /tmp/wf-epic-b/OXY-B007/xvfb.6Qiocv
Xvfb PID: 2695946
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
kill exit: 0
wait exit: 0
pgrep exit: 1
The XKEYBOARD keymap compiler (xkbcomp) reports:
> Warning:          Multiple symbols for level 1/group 1 on key <FK23>
>                   Using F23, ignoring XF86TouchpadOff
> Warning:          Symbol map for key <FK23> redefined
>                   Using last definition for conflicting fields
```

`wait "$XVFB_PID"` completed with status 0 after `kill "$XVFB_PID"` returned 0, so the probe terminated and reaped the exact server process that it started. [pgrep(1)](https://man7.org/linux/man-pages/man1/pgrep.1.html) defines status 1 as no matching processes, status 2 as a command-line error, and status 3 as a fatal error. The separate `pgrep exit: 1` result establishes process-wide absence of a process matching `Xvfb` at the end of the probe.

After the script exited 0, an independent `pgrep -a Xvfb` produced no process lines and exited 1, confirming that no Xvfb process remained on the host:

```text
script exit: 0
post-run pgrep exit: 1
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

- [Apple Technical Note TN2339: Building from the command line with Xcode](https://developer.apple.com/library/archive/technotes/tn2339/_index.html) - fetched successfully through the Jina reader proxy on 2026-08-28; identifies `xcrun` as an Xcode command-line shim.
- [Microsoft vswhere README](https://github.com/microsoft/vswhere) - fetched successfully through the Jina reader proxy on 2026-08-28; identifies the installer path for `vswhere.exe`.
- [Ubuntu 26.04 LTS release notes](https://documentation.ubuntu.com/release-notes/26.04/) - fetched successfully through the Jina reader proxy on 2026-08-28; identifies the official Ubuntu 26.04 LTS release named by the Stage 3 reference pins.
- [pgrep(1) Linux manual page](https://man7.org/linux/man-pages/man1/pgrep.1.html) - fetched successfully through the Jina reader proxy on 2026-08-28; defines pgrep exit statuses 1, 2, and 3.
