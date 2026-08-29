//! X11 reference-environment source backed by the bounded Linux collector.

use oxyflut_qualification::environment::EnvironmentInventory;
use oxyflut_qualification::identifiers::EnvironmentId;

use super::{EnvironmentCommandError, PlatformSource, linux};

/// Collects the X11 Tier 1 environment only on a Linux X11 host.
pub(crate) struct X11Source;

impl PlatformSource for X11Source {
    fn environment(&self) -> EnvironmentId {
        EnvironmentId::X11
    }

    fn collect(&self) -> Result<EnvironmentInventory, EnvironmentCommandError> {
        linux::collect_linux(EnvironmentId::X11)
    }

    fn reference_host_identity(&self) -> Result<Option<String>, EnvironmentCommandError> {
        Ok(linux::reference_host_identity())
    }
}
