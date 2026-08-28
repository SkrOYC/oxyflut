//! Wayland reference-environment source backed by the bounded Linux collector.

use oxyflut_qualification::environment::EnvironmentInventory;
use oxyflut_qualification::identifiers::EnvironmentId;

use super::{EnvironmentCommandError, PlatformSource, linux};

/// Collects the Wayland Tier 1 environment only on a Linux Wayland host.
pub(crate) struct WaylandSource;

impl PlatformSource for WaylandSource {
    fn environment(&self) -> EnvironmentId {
        EnvironmentId::Wayland
    }

    fn collect(&self) -> Result<EnvironmentInventory, EnvironmentCommandError> {
        linux::collect_linux(EnvironmentId::Wayland)
    }
}
