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
