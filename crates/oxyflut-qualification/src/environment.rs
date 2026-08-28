//! Candidate-neutral reference-environment inventory primitives.
//!
//! The qualification lock permits only operating system, minimum version, hardware, GPU, driver,
//! and system-package-lock values. An [`EnvironmentInventory`] retains the additional observed
//! compiler, SDK, and session facts needed to derive that projection without treating them as
//! candidate data. [`EnvironmentInventory::lock_environment_value`] deliberately projects only
//! the fields admitted by the binding lock schema.

use serde::Deserialize;
use serde_json::{Value, json};
use thiserror::Error;

use crate::evidence::canonical_json_bytes;
use crate::hash::hash_reader;
use crate::identifiers::EnvironmentId;

/// Maximum number of system packages retained in one inventory.
pub const MAXIMUM_SYSTEM_PACKAGES: usize = 64;
/// Maximum length of one collected non-private observation.
pub const MAXIMUM_OBSERVED_VALUE_BYTES: usize = 256;

/// Explains why a collector did not emit an observed value.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum MissingReason {
    /// The active lock does not declare the required minimum value.
    NotDeclaredByLock,
    /// The current host does not provide this source.
    UnavailableOnHost,
    /// The authoritative source is unavailable on the host.
    SourceUnavailable,
    /// The authoritative source does not expose this value.
    UnsupportedBySource,
    /// The host is not running the requested display-session type.
    NotActiveSession,
    /// A required locally installed package was absent.
    NotInstalled,
    /// The source exceeded the collector's fixed capture bound.
    InventoryExceedsBound,
}

/// One observed non-private value or an explicit typed absence.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(tag = "status", rename_all = "kebab-case", deny_unknown_fields)]
pub enum InventoryValue {
    /// A value observed from an authoritative source.
    Observed {
        /// The bounded non-private observation.
        value: String,
    },
    /// An unavailable value that was not inferred or defaulted.
    Missing {
        /// The specific reason that the collector could not observe a value.
        reason: MissingReason,
    },
}

impl InventoryValue {
    /// Creates and validates an observed non-private value.
    ///
    /// # Errors
    ///
    /// Returns [`EnvironmentError::ObservedValue`] when `value` could contain unrestricted or
    /// identity-bearing content.
    pub fn observed(value: String) -> Result<Self, EnvironmentError> {
        validate_observed_value(&value)?;
        Ok(Self::Observed { value })
    }

    /// Creates one explicit missing value.
    #[must_use]
    pub const fn missing(reason: MissingReason) -> Self {
        Self::Missing { reason }
    }

    /// Returns the observed text, if one was collected.
    #[must_use]
    pub fn observed_value(&self) -> Option<&str> {
        match self {
            Self::Observed { value } => Some(value),
            Self::Missing { .. } => None,
        }
    }

    /// Returns true when the collector made an explicit missing observation.
    #[must_use]
    pub const fn is_missing(&self) -> bool {
        matches!(self, Self::Missing { .. })
    }

    fn lock_value(&self) -> Value {
        match self {
            Self::Observed { value } => Value::String(value.clone()),
            Self::Missing { .. } => Value::Null,
        }
    }
}

/// One bounded package name and version from an authoritative system package database.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SystemPackage {
    name: String,
    version: String,
}

impl SystemPackage {
    /// Creates one bounded package record.
    ///
    /// # Errors
    ///
    /// Returns [`EnvironmentError::ObservedValue`] when the package name or version is not a
    /// bounded machine-readable package-database value.
    pub fn new(name: String, version: String) -> Result<Self, EnvironmentError> {
        validate_observed_value(&name)?;
        validate_observed_value(&version)?;
        Ok(Self { name, version })
    }

    /// Returns the package name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the package version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    fn canonical_value(&self) -> Value {
        json!({"name": self.name, "version": self.version})
    }
}

/// A content-bounded system-package lock with a digest derived from its retained package list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemPackageLock {
    digest: InventoryValue,
    packages: Vec<SystemPackage>,
}

impl SystemPackageLock {
    /// Builds a package lock from no more than [`MAXIMUM_SYSTEM_PACKAGES`] sorted package records.
    ///
    /// # Errors
    ///
    /// Returns an error when the list is empty, oversized, duplicated, or cannot be canonically
    /// encoded and hashed.
    pub fn from_packages(mut packages: Vec<SystemPackage>) -> Result<Self, EnvironmentError> {
        if packages.is_empty() || packages.len() > MAXIMUM_SYSTEM_PACKAGES {
            return Err(EnvironmentError::SystemPackages);
        }
        packages.sort_unstable();
        if packages.windows(2).any(|pair| pair[0].name == pair[1].name) {
            return Err(EnvironmentError::SystemPackages);
        }
        let values = packages
            .iter()
            .map(SystemPackage::canonical_value)
            .collect::<Vec<_>>();
        let bytes = canonical_json_bytes(&Value::Array(values))
            .map_err(EnvironmentError::CanonicalEncoding)?;
        let digest = hash_reader(std::io::Cursor::new(bytes)).map_err(EnvironmentError::Hash)?;
        Ok(Self {
            digest: InventoryValue::Observed {
                value: digest.to_string(),
            },
            packages,
        })
    }

    /// Creates an explicit missing package lock with no fabricated digest or packages.
    #[must_use]
    pub const fn missing(reason: MissingReason) -> Self {
        Self {
            digest: InventoryValue::Missing { reason },
            packages: Vec::new(),
        }
    }

    /// Returns the observed lock digest or its explicit absence.
    #[must_use]
    pub fn digest(&self) -> &InventoryValue {
        &self.digest
    }

    /// Returns the bounded package list used to derive the lock digest.
    #[must_use]
    pub fn packages(&self) -> &[SystemPackage] {
        &self.packages
    }
}

/// The collected fields used to construct an [`EnvironmentInventory`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnvironmentFields {
    /// Operating-system identity and observed version.
    pub operating_system: InventoryValue,
    /// The declared minimum supported version, when the lock provides one.
    pub minimum_version: InventoryValue,
    /// Processor architecture.
    pub architecture: InventoryValue,
    /// Non-serial hardware model identity.
    pub hardware_id: InventoryValue,
    /// Non-serial graphics model identity.
    pub gpu_id: InventoryValue,
    /// Graphics-driver version.
    pub driver_version: InventoryValue,
    /// Compiler identity.
    pub compiler_identity: InventoryValue,
    /// Platform SDK identity.
    pub sdk_identity: InventoryValue,
    /// Compositor identity.
    pub compositor: InventoryValue,
    /// Display-session identity.
    pub session: InventoryValue,
    /// Display-protocol version.
    pub protocol_version: InventoryValue,
    /// Bounded system package lock.
    pub system_package_lock: SystemPackageLock,
}

/// Candidate-neutral reference-environment observations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnvironmentInventory {
    environment: EnvironmentId,
    fields: EnvironmentFields,
}

impl EnvironmentInventory {
    /// Creates a validated candidate-neutral environment inventory.
    ///
    /// # Errors
    ///
    /// Returns an error when an observed value is not content bounded or the package lock is
    /// internally inconsistent.
    pub fn new(
        environment: EnvironmentId,
        fields: EnvironmentFields,
    ) -> Result<Self, EnvironmentError> {
        for value in [
            &fields.operating_system,
            &fields.minimum_version,
            &fields.architecture,
            &fields.hardware_id,
            &fields.gpu_id,
            &fields.driver_version,
            &fields.compiler_identity,
            &fields.sdk_identity,
            &fields.compositor,
            &fields.session,
            &fields.protocol_version,
            fields.system_package_lock.digest(),
        ] {
            if let Some(observed) = value.observed_value() {
                validate_observed_value(observed)?;
            }
        }
        validate_package_lock(&fields.system_package_lock)?;
        Ok(Self {
            environment,
            fields,
        })
    }

    /// Parses a fixture-backed inventory document.
    ///
    /// Fixture package records are canonicalized and hashed locally; fixtures do not supply a
    /// mutable or hand-written package-lock digest.
    ///
    /// # Errors
    ///
    /// Returns an error when the fixture is malformed, names values outside the bounded inventory
    /// grammar, or supplies packages beside an explicit missing lock.
    pub fn parse_fixture_json(bytes: &[u8]) -> Result<Self, EnvironmentError> {
        let document = serde_json::from_slice::<FixtureInventory>(bytes)
            .map_err(EnvironmentError::FixtureJson)?;
        let system_package_lock = match document.system_package_lock {
            FixtureSystemPackageLock::Observed { packages } => {
                let packages = packages
                    .into_iter()
                    .map(|package| SystemPackage::new(package.name, package.version))
                    .collect::<Result<Vec<_>, _>>()?;
                SystemPackageLock::from_packages(packages)?
            }
            FixtureSystemPackageLock::Missing { reason } => SystemPackageLock::missing(reason),
        };
        let environment = document
            .environment
            .parse::<EnvironmentId>()
            .map_err(|_| EnvironmentError::FixtureEnvironment)?;
        Self::new(
            environment,
            EnvironmentFields {
                operating_system: document.operating_system,
                minimum_version: document.minimum_version,
                architecture: document.architecture,
                hardware_id: document.hardware_id,
                gpu_id: document.gpu_id,
                driver_version: document.driver_version,
                compiler_identity: document.compiler_identity,
                sdk_identity: document.sdk_identity,
                compositor: document.compositor,
                session: document.session,
                protocol_version: document.protocol_version,
                system_package_lock,
            },
        )
    }

    /// Returns the requested Tier 1 environment.
    #[must_use]
    pub const fn environment(&self) -> EnvironmentId {
        self.environment
    }

    /// Returns all collected candidate-neutral fields, including values not serializable by lock v5.
    #[must_use]
    pub const fn fields(&self) -> &EnvironmentFields {
        &self.fields
    }

    /// Returns the operating-system observation.
    #[must_use]
    pub fn operating_system(&self) -> &InventoryValue {
        &self.fields.operating_system
    }

    /// Returns the minimum-version observation.
    #[must_use]
    pub fn minimum_version(&self) -> &InventoryValue {
        &self.fields.minimum_version
    }

    /// Returns the non-serial hardware-model observation.
    #[must_use]
    pub fn hardware_id(&self) -> &InventoryValue {
        &self.fields.hardware_id
    }

    /// Returns the non-serial graphics-model observation.
    #[must_use]
    pub fn gpu_id(&self) -> &InventoryValue {
        &self.fields.gpu_id
    }

    /// Returns the graphics-driver observation.
    #[must_use]
    pub fn driver_version(&self) -> &InventoryValue {
        &self.fields.driver_version
    }

    /// Returns the content-bounded system-package lock.
    #[must_use]
    pub fn system_package_lock(&self) -> &SystemPackageLock {
        &self.fields.system_package_lock
    }

    /// Returns the complete candidate-neutral inventory field names in stable order.
    #[must_use]
    pub const fn field_names() -> [&'static str; 12] {
        [
            "operatingSystem",
            "minimumVersion",
            "architecture",
            "hardwareId",
            "gpuId",
            "driverVersion",
            "compilerIdentity",
            "sdkIdentity",
            "compositor",
            "session",
            "protocolVersion",
            "systemPackageLock",
        ]
    }

    /// Projects this inventory to exactly the environment fields permitted by qualification-lock v5.
    ///
    /// Fields that v5 does not define, including architecture, compiler, SDK, compositor, session,
    /// protocol, and the bounded package list, remain in this typed inventory and are never
    /// serialized into the lock projection. This preserves the lock's closed schema while making
    /// unavailable values explicit to collectors and fixtures.
    #[must_use]
    pub fn lock_environment_value(&self) -> Value {
        json!({
            "operatingSystem": self.fields.operating_system.lock_value(),
            "minimumVersion": self.fields.minimum_version.lock_value(),
            "hardwareId": self.fields.hardware_id.lock_value(),
            "gpuId": self.fields.gpu_id.lock_value(),
            "driverVersion": self.fields.driver_version.lock_value(),
            "systemPackageLockDigest": self.fields.system_package_lock.digest().lock_value(),
        })
    }
}

/// Reports invalid bounded environment inventory data.
#[derive(Debug, Error)]
pub enum EnvironmentError {
    /// An observed value was blank, too large, or contained content outside the machine-readable grammar.
    #[error("environment observation is invalid")]
    ObservedValue,
    /// The retained package list was empty, oversized, duplicated, or mismatched to its digest.
    #[error("system package lock is invalid")]
    SystemPackages,
    /// Canonical package-lock bytes could not be encoded.
    #[error("system package lock could not be canonically encoded")]
    CanonicalEncoding(#[source] crate::evidence::EvidenceError),
    /// Canonical package-lock bytes could not be hashed.
    #[error("system package lock could not be hashed")]
    Hash(#[source] std::io::Error),
    /// A fixture was not a valid bounded inventory document.
    #[error("environment fixture JSON is invalid")]
    FixtureJson(#[source] serde_json::Error),
    /// A fixture named an environment outside the closed Tier 1 set.
    #[error("environment fixture environment is invalid")]
    FixtureEnvironment,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FixtureInventory {
    environment: String,
    operating_system: InventoryValue,
    minimum_version: InventoryValue,
    architecture: InventoryValue,
    hardware_id: InventoryValue,
    gpu_id: InventoryValue,
    driver_version: InventoryValue,
    compiler_identity: InventoryValue,
    sdk_identity: InventoryValue,
    compositor: InventoryValue,
    session: InventoryValue,
    protocol_version: InventoryValue,
    system_package_lock: FixtureSystemPackageLock,
}

#[derive(Deserialize)]
#[serde(tag = "status", rename_all = "kebab-case", deny_unknown_fields)]
enum FixtureSystemPackageLock {
    Observed { packages: Vec<FixtureSystemPackage> },
    Missing { reason: MissingReason },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FixtureSystemPackage {
    name: String,
    version: String,
}

fn validate_observed_value(value: &str) -> Result<(), EnvironmentError> {
    if value.is_empty()
        || value.len() > MAXIMUM_OBSERVED_VALUE_BYTES
        || !value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_' | b':' | b'+')
        })
    {
        return Err(EnvironmentError::ObservedValue);
    }
    Ok(())
}

fn validate_package_lock(lock: &SystemPackageLock) -> Result<(), EnvironmentError> {
    match (lock.digest(), lock.packages().is_empty()) {
        (InventoryValue::Missing { .. }, true) => Ok(()),
        (InventoryValue::Observed { value }, false) => {
            let packages = lock
                .packages()
                .iter()
                .map(SystemPackage::canonical_value)
                .collect::<Vec<_>>();
            let bytes = canonical_json_bytes(&Value::Array(packages))
                .map_err(EnvironmentError::CanonicalEncoding)?;
            let digest =
                hash_reader(std::io::Cursor::new(bytes)).map_err(EnvironmentError::Hash)?;
            if value == &digest.to_string() {
                Ok(())
            } else {
                Err(EnvironmentError::SystemPackages)
            }
        }
        (InventoryValue::Missing { .. }, false) | (InventoryValue::Observed { .. }, true) => {
            Err(EnvironmentError::SystemPackages)
        }
    }
}
