# Platform hosts

- **Status:** accepted for Phase 3A qualification
- **Date:** 2026-08-26

## Context

The focused candidate must supply complete operating-environment integration. The integrated candidate starts from pinned desktop embedders but can require changes. Both need concrete Tier 1 probes. The Linux reference configuration is `thinkpadp14s`: x86_64 NixOS 26.05 with an AMD Renoir integrated GPU and a Hyprland Wayland session; X11 uses interactive Xwayland and headless Xvfb.

## Decision

Use direct AppKit integration through `objc2` on macOS and direct Win32 integration through the `windows` crate on Windows. Use GTK 4 and GLib on `thinkpadp14s` for Linux, with direct Wayland and X11 crates for independent timing and window-system evidence. The Wayland and X11 paths are one NixOS/Hyprland reference-session family: Wayland is the Hyprland session, interactive X11 is Xwayland, and headless X11 is Xvfb.

The integrated candidate uses the pinned Flutter desktop embedders. All inherited callbacks pass through Oxyflut Platform integration before product state changes.

## Consequences

- Every substrate candidate that enters qualification can use different mechanisms but must satisfy the identical capability baseline and frozen suite.
- The Linux reference-session family is recorded as one configuration; its Hyprland/Xwayland composition is a qualification risk rather than a claim of a mainstream distribution session.
- Direct platform supplements are permitted only when the shared probe identifies an inherited gap.
- Phase 3B removes unused host dependencies and requalifies package size, memory, licenses, and security.
