//! Qualification-lock projection and comparison for the staged native toolchain.

use std::collections::BTreeSet;
use std::path::PathBuf;

use oxyflut_qualification::hash::Sha256Digest;
use serde_json::Value;

use super::{
    ResolvedTool, TOOL_SPECS, ToolchainError, ToolchainManifest, path_string,
    reject_unknown_fields, required_string, resolve_manifest_executable_path, sorted_object,
    verify,
};

/// Classifies a resolved-tool validation failure for stable readiness diagnostics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ResolvedToolValidationFailure {
    /// A required staged tool is absent from the lock.
    Missing,
    /// A declared tool differs from the staged manifest.
    Mismatch,
    /// A manifest or lock entry is malformed or cannot be validated.
    Invalid,
}

impl ResolvedToolValidationFailure {
    /// Returns the stable content-free readiness failure code.
    #[must_use]
    pub(crate) const fn code(self) -> &'static str {
        match self {
            Self::Missing => "resolved-tool-missing",
            Self::Mismatch => "resolved-tool-mismatch",
            Self::Invalid => "resolved-tool-invalid",
        }
    }
}

/// Verifies that qualification-lock tool entries exactly reproduce the staged manifest.
///
/// Lock entries carry resolved absolute executable paths because the durable lock schema has no
/// `pathRoot` field. The staged manifest's relative Rustup paths are resolved before comparison.
///
/// # Errors
///
/// Returns an error when either the staged manifest or a lock entry is malformed, substituted,
/// incomplete, duplicated, or differs from the resolved staged tool metadata.
pub(crate) fn verify_lock_resolved_tools(
    manifest: &ToolchainManifest,
    lock_tools: &[Value],
) -> Result<(), ToolchainError> {
    verify(manifest)?;

    let mut names = BTreeSet::new();
    for value in lock_tools {
        let lock_tool = LockResolvedTool::from_value(value)?;
        if !names.insert(lock_tool.name.clone()) {
            return Err(ToolchainError::DuplicateTool {
                name: lock_tool.name,
            });
        }
        let staged_tool = manifest.tool(&lock_tool.name)?;
        verify_lock_tool(&lock_tool, staged_tool)?;
    }

    for specification in TOOL_SPECS {
        if !names.contains(specification.name) {
            return Err(ToolchainError::MissingTool {
                name: specification.name.to_owned(),
            });
        }
    }

    Ok(())
}

/// Verifies lock tools and classifies any failure for readiness reporting.
pub(crate) fn verify_lock_resolved_tools_classified(
    manifest: &ToolchainManifest,
    lock_tools: &[Value],
) -> Result<(), ResolvedToolValidationFailure> {
    match verify_lock_resolved_tools(manifest, lock_tools) {
        Ok(()) => Ok(()),
        Err(
            ToolchainError::ExecutableSubstitution { .. }
            | ToolchainError::LockEntryMismatch { .. }
            | ToolchainError::SourceIdentityMismatch { .. }
            | ToolchainError::VersionMismatch { .. }
            | ToolchainError::MetadataMismatch { .. }
            | ToolchainError::DigestMismatch { .. }
            | ToolchainError::HeaderCheckerMismatch,
        ) => Err(ResolvedToolValidationFailure::Mismatch),
        Err(ToolchainError::MissingTool { .. }) => Err(ResolvedToolValidationFailure::Missing),
        Err(
            ToolchainError::UnsupportedHost { .. }
            | ToolchainError::ReadinessField { .. }
            | ToolchainError::InvalidAuthority
            | ToolchainError::InvalidNote
            | ToolchainError::InvalidManifest { .. }
            | ToolchainError::DuplicateTool { .. }
            | ToolchainError::UnknownTool { .. }
            | ToolchainError::MissingLibcHeaders
            | ToolchainError::LibcHeadersMismatch
            | ToolchainError::ToolExecution { .. }
            | ToolchainError::ToolExecutionFailed { .. }
            | ToolchainError::Io(_)
            | ToolchainError::Json(_),
        ) => Err(ResolvedToolValidationFailure::Invalid),
    }
}

/// Returns the resolved absolute lock entries for a verified staged manifest.
///
/// # Errors
///
/// Returns an error when the staged manifest isn't valid for the current locked host or one
/// resolved executable path isn't valid UTF-8.
pub(crate) fn lock_resolved_tools(
    manifest: &ToolchainManifest,
) -> Result<Vec<Value>, ToolchainError> {
    verify(manifest)?;
    manifest
        .resolved_tools
        .iter()
        .map(LockResolvedTool::from_staged)
        .map(|tool| tool.map(|tool| tool.to_value()))
        .collect()
}

/// Resolves manifest-relative Rustup fixture paths against the current host.
///
/// Fixtures retain the staged manifest's relative Rustup path. This helper replaces only that
/// path with the absolute path resolved through `pathRoot`, leaving every other fixture field for
/// the caller to verify.
///
/// # Errors
///
/// Returns an error when the manifest is invalid, the fixture isn't a lock tool, or a Rustup tool
/// doesn't retain its manifest-relative path.
#[cfg(test)]
pub(crate) fn resolve_fixture_rustup_paths(
    manifest: &ToolchainManifest,
    fixture_tools: &[Value],
) -> Result<Vec<Value>, ToolchainError> {
    let resolved_tools = lock_resolved_tools(manifest)?;
    fixture_tools
        .iter()
        .map(|value| {
            let mut fixture_tool = LockResolvedTool::from_value(value)?;
            let staged_tool = manifest.tool(&fixture_tool.name)?;
            if staged_tool.path_root.as_deref() == Some("rustup-home") {
                if fixture_tool.executable_path != staged_tool.executable_path {
                    return Err(ToolchainError::ExecutableSubstitution {
                        name: fixture_tool.name,
                    });
                }
                let resolved_tool = resolved_tools
                    .iter()
                    .find(|tool| {
                        tool.get("name") == Some(&Value::String(fixture_tool.name.clone()))
                    })
                    .ok_or_else(|| ToolchainError::MissingTool {
                        name: fixture_tool.name.clone(),
                    })?;
                fixture_tool.executable_path = resolved_tool
                    .get("executablePath")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
                    .ok_or_else(|| ToolchainError::MissingTool {
                        name: fixture_tool.name.clone(),
                    })?;
            }
            Ok(fixture_tool.to_value())
        })
        .collect()
}

/// Replaces a current-host Rustup prefix with the manifest-relative path for comparison.
///
/// The result lets tests compare committed fixtures and host-resolved entries without accepting a
/// substituted Rustup path.
///
/// # Errors
///
/// Returns an error when the manifest is invalid, the tool set is malformed, or a Rustup path is
/// neither the manifest-relative path nor the absolute path that `pathRoot` resolves.
#[cfg(test)]
pub(crate) fn normalize_rustup_paths(
    manifest: &ToolchainManifest,
    lock_tools: &[Value],
) -> Result<Vec<Value>, ToolchainError> {
    let resolved_tools = lock_resolved_tools(manifest)?;
    lock_tools
        .iter()
        .map(|value| {
            let mut lock_tool = LockResolvedTool::from_value(value)?;
            let staged_tool = manifest.tool(&lock_tool.name)?;
            if staged_tool.path_root.as_deref() == Some("rustup-home") {
                let resolved_tool = resolved_tools
                    .iter()
                    .find(|tool| tool.get("name") == Some(&Value::String(lock_tool.name.clone())))
                    .ok_or_else(|| ToolchainError::MissingTool {
                        name: lock_tool.name.clone(),
                    })?;
                let resolved_path = resolved_tool
                    .get("executablePath")
                    .and_then(Value::as_str)
                    .ok_or_else(|| ToolchainError::MissingTool {
                        name: lock_tool.name.clone(),
                    })?;
                if lock_tool.executable_path != staged_tool.executable_path
                    && lock_tool.executable_path != resolved_path
                {
                    return Err(ToolchainError::ExecutableSubstitution {
                        name: lock_tool.name,
                    });
                }
                lock_tool.executable_path = staged_tool.executable_path.clone();
            }
            Ok(lock_tool.to_value())
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LockResolvedTool {
    name: String,
    version: String,
    source_identity: String,
    host_triple: String,
    license_id: String,
    executable_path: String,
    sha256: String,
}

impl LockResolvedTool {
    fn from_value(value: &Value) -> Result<Self, ToolchainError> {
        let object = value.as_object().ok_or(ToolchainError::InvalidManifest {
            reason: "a qualification-lock resolvedTools entry must be an object".to_owned(),
        })?;
        reject_unknown_fields(
            object,
            &[
                "name",
                "version",
                "sourceIdentity",
                "hostTriple",
                "licenseId",
                "executablePath",
                "sha256",
            ],
        )?;
        Ok(Self {
            name: required_string(object, "name")?,
            version: required_string(object, "version")?,
            source_identity: required_string(object, "sourceIdentity")?,
            host_triple: required_string(object, "hostTriple")?,
            license_id: required_string(object, "licenseId")?,
            executable_path: required_string(object, "executablePath")?,
            sha256: required_string(object, "sha256")?,
        })
    }

    fn from_staged(tool: &ResolvedTool) -> Result<Self, ToolchainError> {
        Ok(Self {
            name: tool.name.clone(),
            version: tool.version.clone(),
            source_identity: tool.source_identity.clone(),
            host_triple: tool.host_triple.clone(),
            license_id: tool.license_id.clone(),
            executable_path: path_string(&resolve_manifest_executable_path(tool)?)?,
            sha256: tool.sha256.clone(),
        })
    }

    fn to_value(&self) -> Value {
        sorted_object([
            (
                "executablePath",
                Value::String(self.executable_path.clone()),
            ),
            ("hostTriple", Value::String(self.host_triple.clone())),
            ("licenseId", Value::String(self.license_id.clone())),
            ("name", Value::String(self.name.clone())),
            ("sha256", Value::String(self.sha256.clone())),
            (
                "sourceIdentity",
                Value::String(self.source_identity.clone()),
            ),
            ("version", Value::String(self.version.clone())),
        ])
    }
}

fn verify_lock_tool(
    lock_tool: &LockResolvedTool,
    staged_tool: &ResolvedTool,
) -> Result<(), ToolchainError> {
    if lock_tool.version != staged_tool.version
        || lock_tool.source_identity != staged_tool.source_identity
        || lock_tool.host_triple != staged_tool.host_triple
        || lock_tool.license_id != staged_tool.license_id
    {
        return Err(ToolchainError::LockEntryMismatch {
            name: lock_tool.name.clone(),
        });
    }
    if lock_tool.sha256.parse::<Sha256Digest>().is_err() || lock_tool.sha256 != staged_tool.sha256 {
        return Err(ToolchainError::DigestMismatch {
            name: lock_tool.name.clone(),
        });
    }

    let expected_path = resolve_manifest_executable_path(staged_tool)?;
    let actual_path = PathBuf::from(&lock_tool.executable_path);
    if !actual_path.is_absolute() || actual_path != expected_path {
        return Err(ToolchainError::ExecutableSubstitution {
            name: lock_tool.name.clone(),
        });
    }
    Ok(())
}
