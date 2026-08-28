//! Deterministic fixture-backed reference-environment sources.

use std::fs;
use std::path::{Path, PathBuf};

use oxyflut_qualification::environment::EnvironmentInventory;
use oxyflut_qualification::identifiers::EnvironmentId;

use super::{EnvironmentCommandError, PlatformSource};

const FIXTURES_DIRECTORY: &str = "qualification/fixtures/environments";
const INVENTORY_FILE: &str = "inventory.json";

/// Reads one checked environment inventory fixture for deterministic collector tests.
pub(crate) struct FixturePlatformSource {
    root: PathBuf,
    environment: EnvironmentId,
}

impl FixturePlatformSource {
    /// Creates a fixture source rooted at one repository directory.
    #[must_use]
    pub(crate) fn new(root: &Path, environment: EnvironmentId) -> Self {
        Self {
            root: root.to_path_buf(),
            environment,
        }
    }

    fn fixture_path(&self) -> PathBuf {
        self.root
            .join(FIXTURES_DIRECTORY)
            .join(self.environment.as_str())
            .join(INVENTORY_FILE)
    }
}

impl PlatformSource for FixturePlatformSource {
    fn environment(&self) -> EnvironmentId {
        self.environment
    }

    fn collect(&self) -> Result<EnvironmentInventory, EnvironmentCommandError> {
        let path = self.fixture_path();
        let bytes = fs::read(&path)
            .map_err(|source| EnvironmentCommandError::FixtureIo { path, source })?;
        let inventory = EnvironmentInventory::parse_fixture_json(&bytes)
            .map_err(EnvironmentCommandError::Inventory)?;
        if inventory.environment() == self.environment {
            Ok(inventory)
        } else {
            Err(EnvironmentCommandError::SourceEnvironment)
        }
    }
}
