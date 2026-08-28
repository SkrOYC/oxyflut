//! Candidate-neutral reference-environment inventory primitives.
//!
//! The qualification lock admits a six-field environment projection. An [`EnvironmentInventory`]
//! retains the bounded architecture, toolchain, session, protocol, and package facts that bind that
//! projection to a complete companion inventory artifact. On PCI platforms, `gpuId` uses
//! `pci:<vendor-hex4>:<device-hex4>` with lowercase hexadecimal values. On Apple silicon,
//! `gpuId` uses `apple:<model-slug>`.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use thiserror::Error;

use crate::evidence::canonical_json_bytes;
use crate::hash::{Sha256Digest, hash_reader};
use crate::identifiers::{EnvironmentId, RepositoryPath};

/// Maximum number of system-package records retained in one inventory.
pub const MAXIMUM_SYSTEM_PACKAGES: usize = 64;
/// Maximum length of one collected non-private observation.
pub const MAXIMUM_OBSERVED_VALUE_BYTES: usize = 256;

/// Explains why a collector did not emit an observed value.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MissingReason {
    /// The active lock does not declare the required minimum value.
    NotDeclaredByLock,
    /// The platform requires a bounded manual capture because no authoritative CLI exposes it.
    ManualCapture,
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
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

    /// Returns the typed missing reason, if the source could not supply a value.
    #[must_use]
    pub const fn missing_reason(&self) -> Option<MissingReason> {
        match self {
            Self::Observed { .. } => None,
            Self::Missing { reason } => Some(*reason),
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

/// One bounded package name and its observed or explicitly missing package-database version.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemPackage {
    name: String,
    version: InventoryValue,
}

impl SystemPackage {
    /// Creates one observed bounded package record.
    ///
    /// # Errors
    ///
    /// Returns [`EnvironmentError::ObservedValue`] when the package name or version isn't a
    /// bounded machine-readable package-database value.
    pub fn new(name: String, version: String) -> Result<Self, EnvironmentError> {
        validate_observed_value(&name)?;
        Ok(Self {
            name,
            version: InventoryValue::observed(version)?,
        })
    }

    /// Creates one typed missing package record.
    ///
    /// # Errors
    ///
    /// Returns [`EnvironmentError::ObservedValue`] when `name` isn't a bounded package-database
    /// identifier.
    pub fn missing(name: String, reason: MissingReason) -> Result<Self, EnvironmentError> {
        validate_observed_value(&name)?;
        Ok(Self {
            name,
            version: InventoryValue::missing(reason),
        })
    }

    /// Returns the package name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the observed version or typed missing value.
    #[must_use]
    pub fn version(&self) -> &InventoryValue {
        &self.version
    }

    fn canonical_value(&self) -> Option<Value> {
        self.version
            .observed_value()
            .map(|version| json!({"name": self.name, "version": version}))
    }

    fn inventory_value(&self) -> Value {
        json!({"name": self.name, "version": self.version})
    }
}

/// A content-bounded system-package lock with a digest derived from its complete package list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemPackageLock {
    digest: InventoryValue,
    packages: Vec<SystemPackage>,
}

impl SystemPackageLock {
    /// Builds a package lock from no more than [`MAXIMUM_SYSTEM_PACKAGES`] package records.
    ///
    /// The digest is present only when every retained required package has an observed version.
    /// A missing package is retained as a typed record and leaves the digest null in lock
    /// projections.
    ///
    /// # Errors
    ///
    /// Returns an error when the list is empty, oversized, duplicated, or cannot be canonically
    /// encoded and hashed.
    pub fn from_records(mut packages: Vec<SystemPackage>) -> Result<Self, EnvironmentError> {
        if packages.is_empty() || packages.len() > MAXIMUM_SYSTEM_PACKAGES {
            return Err(EnvironmentError::SystemPackages);
        }
        packages.sort_unstable_by(|left, right| left.name.cmp(&right.name));
        if packages
            .windows(2)
            .any(|pair| pair[0].name.as_str() == pair[1].name.as_str())
        {
            return Err(EnvironmentError::SystemPackages);
        }

        let missing_reason = packages
            .iter()
            .find_map(|package| package.version.missing_reason());
        let digest = match missing_reason {
            Some(reason) => InventoryValue::missing(reason),
            None => package_digest(&packages)?,
        };
        Ok(Self { digest, packages })
    }

    /// Builds a complete observed package lock.
    ///
    /// # Errors
    ///
    /// Returns an error when the package records cannot form one bounded unique lock.
    pub fn from_packages(packages: Vec<SystemPackage>) -> Result<Self, EnvironmentError> {
        Self::from_records(packages)
    }

    /// Creates an explicit missing package lock with no available package records.
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

    /// Returns the bounded package records used to derive the digest when it is present.
    #[must_use]
    pub fn packages(&self) -> &[SystemPackage] {
        &self.packages
    }

    fn inventory_value(&self) -> Value {
        let packages = self
            .packages
            .iter()
            .map(SystemPackage::inventory_value)
            .collect::<Vec<_>>();
        json!({"digest": self.digest, "packages": packages})
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
    /// Rust toolchain identity retained only in the complete inventory artifact.
    pub rust_toolchain: InventoryValue,
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
    /// Returns an error when an observed value isn't content bounded or the package lock is
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
            &fields.rust_toolchain,
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

    /// Returns the requested Tier 1 environment.
    #[must_use]
    pub const fn environment(&self) -> EnvironmentId {
        self.environment
    }

    /// Returns all collected candidate-neutral fields, including values excluded from lock v5.
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
    pub const fn field_names() -> [&'static str; 13] {
        [
            "operatingSystem",
            "minimumVersion",
            "architecture",
            "hardwareId",
            "gpuId",
            "driverVersion",
            "compilerIdentity",
            "sdkIdentity",
            "rustToolchain",
            "compositor",
            "session",
            "protocolVersion",
            "systemPackageLock",
        ]
    }

    /// Projects this inventory to exactly the environment fields permitted by qualification-lock v5.
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

    /// Returns the complete durable inventory bound to one immutable lock projection.
    ///
    /// This sidecar preserves all content-bounded observations that lock v5 cannot represent. Its
    /// `lockProjection` path and SHA-256 bind it to the exact six-field projection.
    #[must_use]
    pub fn inventory_value(
        &self,
        projection_path: &RepositoryPath,
        projection_sha256: &Sha256Digest,
    ) -> Value {
        json!({
            "environment": self.environment.as_str(),
            "lockProjection": {
                "path": projection_path.as_str(),
                "sha256": projection_sha256.to_string(),
            },
            "operatingSystem": self.fields.operating_system,
            "minimumVersion": self.fields.minimum_version,
            "architecture": self.fields.architecture,
            "hardwareId": self.fields.hardware_id,
            "gpuId": self.fields.gpu_id,
            "driverVersion": self.fields.driver_version,
            "compilerIdentity": self.fields.compiler_identity,
            "sdkIdentity": self.fields.sdk_identity,
            "rustToolchain": self.fields.rust_toolchain,
            "compositor": self.fields.compositor,
            "session": self.fields.session,
            "protocolVersion": self.fields.protocol_version,
            "systemPackageLock": self.fields.system_package_lock.inventory_value(),
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
}

fn package_digest(packages: &[SystemPackage]) -> Result<InventoryValue, EnvironmentError> {
    let values = packages
        .iter()
        .map(SystemPackage::canonical_value)
        .collect::<Option<Vec<_>>>()
        .ok_or(EnvironmentError::SystemPackages)?;
    let bytes =
        canonical_json_bytes(&Value::Array(values)).map_err(EnvironmentError::CanonicalEncoding)?;
    let digest = hash_reader(std::io::Cursor::new(bytes)).map_err(EnvironmentError::Hash)?;
    Ok(InventoryValue::Observed {
        value: digest.to_string(),
    })
}

fn validate_observed_value(value: &str) -> Result<(), EnvironmentError> {
    if value.is_empty()
        || value.len() > MAXIMUM_OBSERVED_VALUE_BYTES
        || !value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(byte, b'.' | b'-' | b'_' | b':' | b'+' | b'=' | b'/')
        })
    {
        return Err(EnvironmentError::ObservedValue);
    }
    Ok(())
}

fn validate_package_lock(lock: &SystemPackageLock) -> Result<(), EnvironmentError> {
    match (lock.digest(), lock.packages().is_empty()) {
        (InventoryValue::Observed { value }, false) => {
            if lock
                .packages()
                .iter()
                .any(|package| package.version().is_missing())
            {
                return Err(EnvironmentError::SystemPackages);
            }
            let calculated = package_digest(lock.packages())?;
            if calculated.observed_value() == Some(value) {
                Ok(())
            } else {
                Err(EnvironmentError::SystemPackages)
            }
        }
        (InventoryValue::Missing { .. }, true) => Ok(()),
        (InventoryValue::Missing { .. }, false)
            if lock
                .packages()
                .iter()
                .any(|package| package.version().is_missing()) =>
        {
            Ok(())
        }
        (InventoryValue::Observed { .. }, true) | (InventoryValue::Missing { .. }, false) => {
            Err(EnvironmentError::SystemPackages)
        }
    }
}
