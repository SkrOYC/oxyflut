# Platform hosts

- **Status:** accepted for Phase 3A qualification
- **Date:** 2026-08-26

## Context

The focused candidate must supply complete operating-environment integration. The integrated candidate starts from pinned desktop embedders but can require changes. Both need concrete Tier 1 probes.

## Decision

Use direct AppKit integration through `objc2` on macOS and direct Win32 integration through the `windows` crate on Windows. Use GTK 4 and GLib for the Linux host, with direct Wayland and X11 crates for independent timing and window-system evidence.

The integrated candidate uses the pinned Flutter desktop embedders. All inherited callbacks pass through Oxyflut Platform integration before product state changes.

## Consequences

- The two candidates can use different mechanisms but must satisfy identical capability baselines.
- Direct platform supplements are permitted only when the shared probe identifies an inherited gap.
- Phase 3B removes unused host dependencies and requalifies package size, memory, licenses, and security.
