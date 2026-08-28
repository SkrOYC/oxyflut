//! Deterministic fixture-backed raw platform-response sources.

use std::fs;
use std::path::{Path, PathBuf};

use oxyflut_qualification::environment::EnvironmentInventory;
use oxyflut_qualification::identifiers::EnvironmentId;

use super::{EnvironmentCommandError, PlatformSource, linux, macos, windows};

const FIXTURES_DIRECTORY: &str = "qualification/fixtures/environments";
const RESPONSES_FILE: &str = "responses.json";

/// Feeds checked raw platform responses through the same platform collectors used by live sources.
pub(crate) struct FixturePlatformSource {
    root: PathBuf,
    environment: EnvironmentId,
    fixture: String,
}

impl FixturePlatformSource {
    /// Creates a fixture source using the standard raw response fixture for one environment.
    #[must_use]
    pub(crate) fn new(root: &Path, environment: EnvironmentId) -> Self {
        Self::with_fixture(root, environment, environment.as_str())
    }

    /// Creates a fixture source using one named raw response fixture.
    #[must_use]
    pub(crate) fn with_fixture(root: &Path, environment: EnvironmentId, fixture: &str) -> Self {
        Self {
            root: root.to_path_buf(),
            environment,
            fixture: fixture.to_owned(),
        }
    }

    fn fixture_path(&self) -> PathBuf {
        self.root
            .join(FIXTURES_DIRECTORY)
            .join(&self.fixture)
            .join(RESPONSES_FILE)
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
        match self.environment {
            EnvironmentId::Macos => macos::collect_fixture_macos(&bytes),
            EnvironmentId::Windows => windows::collect_fixture_windows(&bytes),
            EnvironmentId::Wayland | EnvironmentId::X11 => {
                linux::collect_fixture_linux(self.environment, &bytes)
            }
        }
    }
}
