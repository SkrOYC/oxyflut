#![allow(
    dead_code,
    reason = "OXY-A005 consumes these library functions without adding an uncontracted dispatcher command."
)]
//! Resolution and verification of the staged native-contract toolchain.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use oxyflut_qualification::hash::{Sha256Digest, hash_file, hash_reader};
use serde_json::Value;

#[path = "toolchain/error.rs"]
mod error;
#[path = "toolchain/lock.rs"]
pub(crate) mod lock;
#[path = "toolchain/specs.rs"]
mod specs;
#[cfg(test)]
#[path = "toolchain_tests.rs"]
mod tests;
pub(crate) use error::ToolchainError;
use specs::{LINUX_LIBC_HEADERS_VERSION, TOOL_SPECS, ToolLocator, ToolSpec};
pub(crate) const STAGED_HOST: &str = "x86_64-unknown-linux-gnu";
const RUST_TOOLCHAIN_COMMIT: &str = "88d9e12ae178fab0fb5cc050a94da85685d449ea";
const RUST_TOOLCHAIN_NAME: &str = "1.98.0-x86_64-unknown-linux-gnu";
const RUSTC_RELATIVE_PATH: &str = "toolchains/1.98.0-x86_64-unknown-linux-gnu/bin/rustc";
const RUSTFMT_RELATIVE_PATH: &str = "toolchains/1.98.0-x86_64-unknown-linux-gnu/bin/rustfmt";
const STAGED_AUTHORITY: &str = "staged-proposal";
const STAGED_NOTE: &str = "Stage 3 reconciliation owns active qualification-lock pins; this host-local proposal cannot set readiness.";

/// Resolves and verifies the tools available to native-contract validation.
///
/// # Errors
///
/// Returns an error when the locked host cannot provide one immutable, pinned tool or SDK utility.
pub(crate) fn resolve() -> Result<ToolchainManifest, ToolchainError> {
    let host_triple = host_triple()?;
    if host_triple != STAGED_HOST {
        return Err(ToolchainError::UnsupportedHost {
            supported_host: STAGED_HOST,
            detected_host: host_triple,
        });
    }

    let mut resolved_tools = Vec::with_capacity(TOOL_SPECS.len());
    for specification in TOOL_SPECS {
        resolved_tools.push(resolve_tool(specification, &host_triple)?);
    }

    let header_checker = resolved_tools
        .iter()
        .find(|tool| tool.name == "c-header-checker")
        .ok_or_else(|| ToolchainError::MissingTool {
            name: "c-header-checker".to_owned(),
        })?;
    let sdk_utilities = vec![resolve_linux_libc_headers(header_checker, &host_triple)?];

    let manifest = ToolchainManifest {
        authority: STAGED_AUTHORITY.to_owned(),
        note: STAGED_NOTE.to_owned(),
        resolved_tools,
        sdk_utilities,
    };
    verify(&manifest)?;
    Ok(manifest)
}

/// Verifies that a staged manifest exactly matches the currently resolved native toolchain.
///
/// # Errors
///
/// Returns an error when the manifest is authoritative, incomplete, malformed, substituted, or
/// mismatched with the local immutable toolchain.
pub(crate) fn verify(manifest: &ToolchainManifest) -> Result<(), ToolchainError> {
    if manifest.authority != STAGED_AUTHORITY {
        return Err(ToolchainError::InvalidAuthority);
    }
    if manifest.note != STAGED_NOTE {
        return Err(ToolchainError::InvalidNote);
    }

    let host_triple = host_triple()?;
    if host_triple != STAGED_HOST {
        return Err(ToolchainError::UnsupportedHost {
            supported_host: STAGED_HOST,
            detected_host: host_triple,
        });
    }
    let mut names = BTreeSet::new();
    for tool in &manifest.resolved_tools {
        if !names.insert(tool.name.as_str()) {
            return Err(ToolchainError::DuplicateTool {
                name: tool.name.clone(),
            });
        }
        let specification = tool_specification(&tool.name)?;
        verify_tool(tool, specification, &host_triple)?;
    }

    for specification in TOOL_SPECS {
        if !names.contains(specification.name) {
            return Err(ToolchainError::MissingTool {
                name: specification.name.to_owned(),
            });
        }
    }

    verify_same_clang(manifest)?;
    verify_sdk_utilities(manifest, &host_triple)
}

/// A deterministic, nonauthoritative native-toolchain proposal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ToolchainManifest {
    authority: String,
    note: String,
    resolved_tools: Vec<ResolvedTool>,
    sdk_utilities: Vec<SdkUtility>,
}

impl ToolchainManifest {
    /// Parses a staged manifest and rejects readiness declarations and unknown fields.
    ///
    /// # Errors
    ///
    /// Returns an error when the JSON is malformed or doesn't have the staged-manifest shape.
    pub(crate) fn from_json(bytes: &[u8]) -> Result<Self, ToolchainError> {
        let value: Value = serde_json::from_slice(bytes)?;
        let object = value.as_object().ok_or(ToolchainError::InvalidManifest {
            reason: "the root must be an object".to_owned(),
        })?;

        for field in ["candidateImplementationReady", "measurementReady"] {
            if object.contains_key(field) {
                return Err(ToolchainError::ReadinessField {
                    field: field.to_owned(),
                });
            }
        }
        reject_unknown_fields(
            object,
            &["authority", "note", "resolvedTools", "sdkUtilities"],
        )?;

        let authority = required_string(object, "authority")?;
        let note = required_string(object, "note")?;
        let resolved_tools = required_array(object, "resolvedTools")?
            .iter()
            .map(ResolvedTool::from_value)
            .collect::<Result<Vec<_>, _>>()?;
        let sdk_utilities = required_array(object, "sdkUtilities")?
            .iter()
            .map(SdkUtility::from_value)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            authority,
            note,
            resolved_tools,
            sdk_utilities,
        })
    }

    /// Returns canonical JSON bytes with recursively sorted object keys.
    ///
    /// # Errors
    ///
    /// Returns an error only when JSON serialization cannot encode the manifest.
    pub(crate) fn canonical_bytes(&self) -> Result<Vec<u8>, ToolchainError> {
        Ok(serde_json::to_vec(&self.to_value())?)
    }

    /// Returns one strict parsed manifest tool's resolved executable path.
    ///
    /// # Errors
    ///
    /// Returns an error when the named tool is absent or its parsed path root is invalid.
    pub(crate) fn executable_path(&self, name: &str) -> Result<PathBuf, ToolchainError> {
        resolve_manifest_executable_path(self.tool(name)?)
    }

    /// Returns one strict parsed manifest tool's declared host triple.
    ///
    /// # Errors
    ///
    /// Returns an error when the named tool is absent.
    pub(crate) fn host_triple(&self, name: &str) -> Result<&str, ToolchainError> {
        Ok(&self.tool(name)?.host_triple)
    }

    /// Returns a manifest tool's verified path without requiring native host support.
    pub(crate) fn verified_executable_path(&self, name: &str) -> Result<PathBuf, ToolchainError> {
        let tool = self.tool(name)?;
        let specification = tool_specification(name)?;
        let host_triple = host_triple()?;
        verify_tool(tool, specification, &host_triple)?;
        resolve_manifest_executable_path(tool)
    }

    fn tool(&self, name: &str) -> Result<&ResolvedTool, ToolchainError> {
        self.resolved_tools
            .iter()
            .find(|tool| tool.name == name)
            .ok_or_else(|| ToolchainError::MissingTool {
                name: name.to_owned(),
            })
    }

    fn to_value(&self) -> Value {
        sorted_object([
            ("authority", Value::String(self.authority.clone())),
            ("note", Value::String(self.note.clone())),
            (
                "resolvedTools",
                Value::Array(
                    self.resolved_tools
                        .iter()
                        .map(ResolvedTool::to_value)
                        .collect(),
                ),
            ),
            (
                "sdkUtilities",
                Value::Array(
                    self.sdk_utilities
                        .iter()
                        .map(SdkUtility::to_value)
                        .collect(),
                ),
            ),
        ])
    }
}

/// One staged `resolvedTools` entry proposed for reconciliation.
#[derive(Clone, Debug, Eq, PartialEq)]
struct ResolvedTool {
    name: String,
    version: String,
    source_identity: String,
    host_triple: String,
    license_id: String,
    executable_path: String,
    path_root: Option<String>,
    sha256: String,
}

impl ResolvedTool {
    fn from_value(value: &Value) -> Result<Self, ToolchainError> {
        let object = value.as_object().ok_or(ToolchainError::InvalidManifest {
            reason: "a resolvedTools entry must be an object".to_owned(),
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
                "pathRoot",
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
            path_root: optional_string(object, "pathRoot")?,
            sha256: required_string(object, "sha256")?,
        })
    }

    fn to_value(&self) -> Value {
        let mut entries = BTreeMap::from([
            (
                "executablePath".to_owned(),
                Value::String(self.executable_path.clone()),
            ),
            (
                "hostTriple".to_owned(),
                Value::String(self.host_triple.clone()),
            ),
            (
                "licenseId".to_owned(),
                Value::String(self.license_id.clone()),
            ),
            ("name".to_owned(), Value::String(self.name.clone())),
            ("sha256".to_owned(), Value::String(self.sha256.clone())),
            (
                "sourceIdentity".to_owned(),
                Value::String(self.source_identity.clone()),
            ),
            ("version".to_owned(), Value::String(self.version.clone())),
        ]);
        if let Some(path_root) = &self.path_root {
            entries.insert("pathRoot".to_owned(), Value::String(path_root.clone()));
        }
        Value::Object(entries.into_iter().collect())
    }
}

/// One host SDK directory used by the staged C and C++ header checks.
#[derive(Clone, Debug, Eq, PartialEq)]
struct SdkUtility {
    name: String,
    version: String,
    source_identity: String,
    host_triple: String,
    license_id: String,
    path: String,
    sha256: String,
    purpose: String,
}

impl SdkUtility {
    fn from_value(value: &Value) -> Result<Self, ToolchainError> {
        let object = value.as_object().ok_or(ToolchainError::InvalidManifest {
            reason: "an sdkUtilities entry must be an object".to_owned(),
        })?;
        reject_unknown_fields(
            object,
            &[
                "name",
                "version",
                "sourceIdentity",
                "hostTriple",
                "licenseId",
                "path",
                "sha256",
                "purpose",
            ],
        )?;

        Ok(Self {
            name: required_string(object, "name")?,
            version: required_string(object, "version")?,
            source_identity: required_string(object, "sourceIdentity")?,
            host_triple: required_string(object, "hostTriple")?,
            license_id: required_string(object, "licenseId")?,
            path: required_string(object, "path")?,
            sha256: required_string(object, "sha256")?,
            purpose: required_string(object, "purpose")?,
        })
    }

    fn to_value(&self) -> Value {
        sorted_object([
            ("hostTriple", Value::String(self.host_triple.clone())),
            ("licenseId", Value::String(self.license_id.clone())),
            ("name", Value::String(self.name.clone())),
            ("path", Value::String(self.path.clone())),
            ("purpose", Value::String(self.purpose.clone())),
            ("sha256", Value::String(self.sha256.clone())),
            (
                "sourceIdentity",
                Value::String(self.source_identity.clone()),
            ),
            ("version", Value::String(self.version.clone())),
        ])
    }
}

fn resolve_tool(
    specification: &ToolSpec,
    host_triple: &str,
) -> Result<ResolvedTool, ToolchainError> {
    let executable_path = executable_path(specification)?;
    let version = command_version(specification, &executable_path)?;
    if !version.contains(specification.required_version_fragment) {
        return Err(ToolchainError::VersionMismatch {
            name: specification.name.to_owned(),
        });
    }

    let source_identity = source_identity(specification, &executable_path)?;
    let sha256 = hash_file(&executable_path)?.to_string();

    Ok(ResolvedTool {
        name: specification.name.to_owned(),
        version,
        source_identity,
        host_triple: host_triple.to_owned(),
        license_id: specification.license_id.to_owned(),
        executable_path: manifest_executable_path(specification, &executable_path)?,
        path_root: manifest_path_root(specification).map(str::to_owned),
        sha256,
    })
}

fn verify_tool(
    tool: &ResolvedTool,
    specification: &ToolSpec,
    host_triple: &str,
) -> Result<(), ToolchainError> {
    if tool.host_triple != host_triple || tool.license_id != specification.license_id {
        return Err(ToolchainError::MetadataMismatch {
            name: tool.name.clone(),
        });
    }

    if tool.path_root.as_deref() != manifest_path_root(specification) {
        return Err(ToolchainError::ExecutableSubstitution {
            name: tool.name.clone(),
        });
    }

    let executable_path = executable_path(specification)?;
    let resolved_manifest_path = resolve_manifest_executable_path(tool)?;
    if manifest_executable_path(specification, &executable_path)? != tool.executable_path
        || resolved_manifest_path != executable_path
    {
        return Err(ToolchainError::ExecutableSubstitution {
            name: tool.name.clone(),
        });
    }

    let source_identity = source_identity(specification, &resolved_manifest_path)?;
    if tool.source_identity != source_identity {
        return Err(ToolchainError::SourceIdentityMismatch {
            name: tool.name.clone(),
        });
    }

    let version = command_version(specification, &resolved_manifest_path)?;
    if !version.contains(specification.required_version_fragment) || tool.version != version {
        return Err(ToolchainError::VersionMismatch {
            name: tool.name.clone(),
        });
    }

    let digest = hash_file(&resolved_manifest_path)?.to_string();
    if tool.sha256.parse::<Sha256Digest>().is_err() || tool.sha256 != digest {
        return Err(ToolchainError::DigestMismatch {
            name: tool.name.clone(),
        });
    }

    Ok(())
}

fn verify_same_clang(manifest: &ToolchainManifest) -> Result<(), ToolchainError> {
    let c_compiler = manifest
        .resolved_tools
        .iter()
        .find(|tool| tool.name == "c-compiler")
        .ok_or_else(|| ToolchainError::MissingTool {
            name: "c-compiler".to_owned(),
        })?;
    let header_checker = manifest
        .resolved_tools
        .iter()
        .find(|tool| tool.name == "c-header-checker")
        .ok_or_else(|| ToolchainError::MissingTool {
            name: "c-header-checker".to_owned(),
        })?;

    if c_compiler.sha256 != header_checker.sha256
        || c_compiler.source_identity != header_checker.source_identity
    {
        return Err(ToolchainError::HeaderCheckerMismatch);
    }
    Ok(())
}

fn resolve_linux_libc_headers(
    header_checker: &ResolvedTool,
    host_triple: &str,
) -> Result<SdkUtility, ToolchainError> {
    let include_path = libc_header_path(Path::new(&header_checker.executable_path))?;
    let source_identity = sdk_source_identity(&include_path)?;
    let version = source_identity
        .split("derivation: ")
        .nth(1)
        .and_then(|value| value.split(';').next())
        .and_then(|value| value.strip_prefix("glibc-"))
        .and_then(|value| value.strip_suffix("-dev"))
        .ok_or(ToolchainError::MissingLibcHeaders)?;

    Ok(SdkUtility {
        name: "linux-libc-headers".to_owned(),
        version: format!("glibc-{version}"),
        source_identity,
        host_triple: host_triple.to_owned(),
        license_id: "LGPL-2.1-or-later".to_owned(),
        path: path_string(&include_path)?,
        sha256: hash_directory(&include_path)?,
        purpose: "Clang's active libc header directory for C11 and C++17 syntax-only checks; recorded because native validation must not use host headers outside the staged manifest.".to_owned(),
    })
}

fn verify_sdk_utilities(
    manifest: &ToolchainManifest,
    host_triple: &str,
) -> Result<(), ToolchainError> {
    if manifest.sdk_utilities.len() != 1 {
        return Err(ToolchainError::LibcHeadersMismatch);
    }
    let utility = manifest
        .sdk_utilities
        .first()
        .ok_or(ToolchainError::LibcHeadersMismatch)?;
    if utility.name != "linux-libc-headers"
        || utility.host_triple != host_triple
        || utility.license_id != "LGPL-2.1-or-later"
        || utility.purpose
            != "Clang's active libc header directory for C11 and C++17 syntax-only checks; recorded because native validation must not use host headers outside the staged manifest."
    {
        return Err(ToolchainError::LibcHeadersMismatch);
    }

    let header_checker = manifest
        .resolved_tools
        .iter()
        .find(|tool| tool.name == "c-header-checker")
        .ok_or_else(|| ToolchainError::MissingTool {
            name: "c-header-checker".to_owned(),
        })?;
    let include_path = libc_header_path(Path::new(&header_checker.executable_path))?;
    if utility.path != path_string(&include_path)?
        || utility.source_identity != sdk_source_identity(&include_path)?
        || utility.version != LINUX_LIBC_HEADERS_VERSION
        || utility.sha256.parse::<Sha256Digest>().is_err()
        || utility.sha256 != hash_directory(&include_path)?
    {
        return Err(ToolchainError::LibcHeadersMismatch);
    }
    Ok(())
}

fn executable_path(specification: &ToolSpec) -> Result<PathBuf, ToolchainError> {
    match specification.locator {
        ToolLocator::Path(command) => {
            find_in_path(command).ok_or_else(|| ToolchainError::MissingTool {
                name: specification.name.to_owned(),
            })
        }
        ToolLocator::Rustfmt => rustup_component_path("rustfmt", RUSTFMT_RELATIVE_PATH),
        ToolLocator::Rustc => rustup_component_path("rustc", RUSTC_RELATIVE_PATH),
    }
}

fn rustup_component_path(
    component: &'static str,
    expected_relative_path: &str,
) -> Result<PathBuf, ToolchainError> {
    let rustup = find_in_path("rustup").ok_or_else(|| ToolchainError::MissingTool {
        name: component.to_owned(),
    })?;
    let output = Command::new(rustup)
        .args(["which", "--toolchain", "1.98.0", component])
        .output()
        .map_err(|source| ToolchainError::ToolExecution {
            name: component.to_owned(),
            source,
        })?;
    if !output.status.success() {
        return Err(ToolchainError::ToolExecutionFailed {
            name: component.to_owned(),
        });
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    let executable_path = PathBuf::from(path);
    if !executable_path.is_file()
        || rustup_relative_path(&executable_path, component)? != expected_relative_path
    {
        return Err(ToolchainError::MissingTool {
            name: component.to_owned(),
        });
    }
    Ok(executable_path)
}

fn manifest_path_root(specification: &ToolSpec) -> Option<&'static str> {
    match specification.locator {
        ToolLocator::Path(_) => None,
        ToolLocator::Rustfmt | ToolLocator::Rustc => Some("rustup-home"),
    }
}

fn manifest_executable_path(
    specification: &ToolSpec,
    executable_path: &Path,
) -> Result<String, ToolchainError> {
    match manifest_path_root(specification) {
        None => path_string(executable_path),
        Some("rustup-home") => rustup_relative_path(executable_path, specification.name),
        Some(_) => Err(ToolchainError::InvalidManifest {
            reason: "a staged executable path root is invalid".to_owned(),
        }),
    }
}

fn resolve_manifest_executable_path(tool: &ResolvedTool) -> Result<PathBuf, ToolchainError> {
    match tool.path_root.as_deref() {
        None => {
            let executable_path = PathBuf::from(&tool.executable_path);
            if !executable_path.is_absolute() {
                return Err(ToolchainError::InvalidManifest {
                    reason: "an executable path without a root must be absolute".to_owned(),
                });
            }
            Ok(executable_path)
        }
        Some("rustup-home") => {
            let relative_path = Path::new(&tool.executable_path);
            if !is_safe_relative_path(relative_path) {
                return Err(ToolchainError::InvalidManifest {
                    reason: "a rustup-home executable path must be relative and confined"
                        .to_owned(),
                });
            }
            Ok(rustup_home()?.join(relative_path))
        }
        Some(_) => Err(ToolchainError::InvalidManifest {
            reason: "an executable path root is invalid".to_owned(),
        }),
    }
}

fn rustup_relative_path(executable_path: &Path, component: &str) -> Result<String, ToolchainError> {
    let relative_path = executable_path.strip_prefix(rustup_home()?).map_err(|_| {
        ToolchainError::SourceIdentityMismatch {
            name: component.to_owned(),
        }
    })?;
    if !is_safe_relative_path(relative_path) {
        return Err(ToolchainError::SourceIdentityMismatch {
            name: component.to_owned(),
        });
    }
    path_string(relative_path)
}

fn rustup_home() -> Result<PathBuf, ToolchainError> {
    let path = std::env::var_os("RUSTUP_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".rustup")))
        .ok_or_else(|| ToolchainError::MissingTool {
            name: "rustup home".to_owned(),
        })?;
    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn is_safe_relative_path(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && path
            .components()
            .all(|component| matches!(component, std::path::Component::Normal(_)))
}

fn find_in_path(command: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|directory| directory.join(command))
        .find(|candidate| candidate.is_file())
}

fn command_version(
    specification: &ToolSpec,
    executable_path: &Path,
) -> Result<String, ToolchainError> {
    let output = Command::new(executable_path)
        .args(specification.version_arguments)
        .output()
        .map_err(|source| ToolchainError::ToolExecution {
            name: specification.name.to_owned(),
            source,
        })?;
    if !output.status.success() {
        return Err(ToolchainError::ToolExecutionFailed {
            name: specification.name.to_owned(),
        });
    }

    let combined = if output.stdout.is_empty() {
        String::from_utf8_lossy(&output.stderr).into_owned()
    } else {
        String::from_utf8_lossy(&output.stdout).into_owned()
    };
    let version = combined
        .lines()
        .next()
        .unwrap_or_default()
        .trim()
        .to_owned();
    if version.is_empty() {
        return Err(ToolchainError::VersionMismatch {
            name: specification.name.to_owned(),
        });
    }
    Ok(version)
}

fn source_identity(
    specification: &ToolSpec,
    executable_path: &Path,
) -> Result<String, ToolchainError> {
    match specification.locator {
        ToolLocator::Path(_) => {
            let (store_path, derivation) = nix_store_identity(executable_path)?;
            if !derivation.contains(specification.source_derivation_fragment) {
                return Err(ToolchainError::SourceIdentityMismatch {
                    name: specification.name.to_owned(),
                });
            }
            Ok(format!(
                "nix-store: {}; derivation: {derivation}; version: {}",
                store_path.display(),
                specification.source_version
            ))
        }
        ToolLocator::Rustfmt | ToolLocator::Rustc => rustup_toolchain_identity(),
    }
}

fn rustup_toolchain_identity() -> Result<String, ToolchainError> {
    let output = Command::new(rustup_component_path("rustc", RUSTC_RELATIVE_PATH)?)
        .arg("-vV")
        .output()
        .map_err(|source| ToolchainError::ToolExecution {
            name: "rustc".to_owned(),
            source,
        })?;
    if !output.status.success() {
        return Err(ToolchainError::ToolExecutionFailed {
            name: "rustc".to_owned(),
        });
    }

    let version = String::from_utf8_lossy(&output.stdout);
    let release = rustc_verbose_field(&version, "release").ok_or_else(|| {
        ToolchainError::SourceIdentityMismatch {
            name: "rustc".to_owned(),
        }
    })?;
    let host = rustc_verbose_field(&version, "host").ok_or_else(|| {
        ToolchainError::SourceIdentityMismatch {
            name: "rustc".to_owned(),
        }
    })?;
    let commit = rustc_verbose_field(&version, "commit-hash").ok_or_else(|| {
        ToolchainError::SourceIdentityMismatch {
            name: "rustc".to_owned(),
        }
    })?;
    let toolchain = format!("{release}-{host}");

    if toolchain != RUST_TOOLCHAIN_NAME || commit != RUST_TOOLCHAIN_COMMIT {
        return Err(ToolchainError::SourceIdentityMismatch {
            name: "rustc".to_owned(),
        });
    }

    Ok(format!("rustup-toolchain: {toolchain}; commit: {commit}"))
}

fn rustc_verbose_field<'output>(output: &'output str, field: &str) -> Option<&'output str> {
    output
        .lines()
        .find_map(|line| line.strip_prefix(&format!("{field}: ")))
}

fn nix_store_identity(path: &Path) -> Result<(PathBuf, String), ToolchainError> {
    let store = Path::new("/nix/store");
    let relative =
        path.strip_prefix(store)
            .map_err(|_| ToolchainError::SourceIdentityMismatch {
                name: path.display().to_string(),
            })?;
    let entry = relative
        .components()
        .next()
        .and_then(|component| component.as_os_str().to_str())
        .ok_or_else(|| ToolchainError::SourceIdentityMismatch {
            name: path.display().to_string(),
        })?;
    let (_, derivation) =
        entry
            .split_once('-')
            .ok_or_else(|| ToolchainError::SourceIdentityMismatch {
                name: path.display().to_string(),
            })?;
    Ok((store.join(entry), derivation.to_owned()))
}

fn libc_header_path(clang_path: &Path) -> Result<PathBuf, ToolchainError> {
    let output = Command::new(clang_path)
        .args(["-E", "-x", "c", "-v", "-"])
        .output()
        .map_err(|source| ToolchainError::ToolExecution {
            name: "c-header-checker".to_owned(),
            source,
        })?;
    if !output.status.success() {
        return Err(ToolchainError::ToolExecutionFailed {
            name: "c-header-checker".to_owned(),
        });
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    stderr
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("/nix/store/") && line.ends_with("/include"))
        .map(PathBuf::from)
        .find(|path| {
            nix_store_identity(path)
                .map(|(_, derivation)| {
                    derivation.starts_with("glibc-") && derivation.ends_with("-dev")
                })
                .unwrap_or(false)
        })
        .filter(|path| path.is_dir())
        .ok_or(ToolchainError::MissingLibcHeaders)
}

fn sdk_source_identity(path: &Path) -> Result<String, ToolchainError> {
    let (store_path, derivation) = nix_store_identity(path)?;
    if !derivation.starts_with("glibc-") || !derivation.ends_with("-dev") {
        return Err(ToolchainError::MissingLibcHeaders);
    }
    Ok(format!(
        "nix-store: {}; derivation: {derivation}; role: active-clang-libc-headers",
        store_path.display()
    ))
}

fn hash_directory(directory: &Path) -> Result<String, ToolchainError> {
    let mut records = Vec::new();
    append_directory_records(directory, directory, &mut records)?;
    Ok(hash_reader(std::io::Cursor::new(records))?.to_string())
}

fn append_directory_records(
    root: &Path,
    directory: &Path,
    records: &mut Vec<u8>,
) -> Result<(), ToolchainError> {
    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let path = entry.path();
        let relative = path
            .strip_prefix(root)
            .map_err(|_| ToolchainError::LibcHeadersMismatch)?;
        let relative = relative
            .to_str()
            .ok_or(ToolchainError::LibcHeadersMismatch)?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            append_directory_records(root, &path, records)?;
        } else if file_type.is_file() {
            records.extend_from_slice(b"file\0");
            records.extend_from_slice(relative.as_bytes());
            records.push(0);
            records.extend_from_slice(hash_file(&path)?.to_string().as_bytes());
            records.push(b'\n');
        } else if file_type.is_symlink() {
            let target = fs::read_link(&path)?;
            let target = target.to_str().ok_or(ToolchainError::LibcHeadersMismatch)?;
            records.extend_from_slice(b"symlink\0");
            records.extend_from_slice(relative.as_bytes());
            records.push(0);
            records.extend_from_slice(target.as_bytes());
            records.push(b'\n');
        } else {
            return Err(ToolchainError::LibcHeadersMismatch);
        }
    }
    Ok(())
}

/// Returns whether the executing Rust host matches the staged native toolchain host.
///
/// # Errors
///
/// Returns an error when the pinned Rust compiler cannot report its host triple.
pub(crate) fn is_staged_host() -> Result<bool, ToolchainError> {
    Ok(host_triple()? == STAGED_HOST)
}

fn host_triple() -> Result<String, ToolchainError> {
    let output = Command::new(rustup_component_path("rustc", RUSTC_RELATIVE_PATH)?)
        .arg("-vV")
        .output()
        .map_err(|source| ToolchainError::ToolExecution {
            name: "rustc".to_owned(),
            source,
        })?;
    if !output.status.success() {
        return Err(ToolchainError::ToolExecutionFailed {
            name: "rustc".to_owned(),
        });
    }
    let output = String::from_utf8_lossy(&output.stdout);
    output
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .map(str::to_owned)
        .ok_or_else(|| ToolchainError::MissingTool {
            name: "rustc host triple".to_owned(),
        })
}

fn tool_specification(name: &str) -> Result<&'static ToolSpec, ToolchainError> {
    TOOL_SPECS
        .iter()
        .find(|specification| specification.name == name)
        .ok_or_else(|| ToolchainError::UnknownTool {
            name: name.to_owned(),
        })
}

fn path_string(path: &Path) -> Result<String, ToolchainError> {
    path.to_str()
        .map(str::to_owned)
        .ok_or_else(|| ToolchainError::InvalidManifest {
            reason: "a staged path must be UTF-8".to_owned(),
        })
}

fn sorted_object<const COUNT: usize>(entries: [(&str, Value); COUNT]) -> Value {
    let values = entries
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value))
        .collect::<BTreeMap<_, _>>();
    Value::Object(values.into_iter().collect())
}

fn required_string(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<String, ToolchainError> {
    object
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| ToolchainError::InvalidManifest {
            reason: format!("{field} must be a nonempty string"),
        })
}

fn optional_string(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<Option<String>, ToolchainError> {
    match object.get(field) {
        None => Ok(None),
        Some(Value::String(value)) if !value.is_empty() => Ok(Some(value.clone())),
        Some(_) => Err(ToolchainError::InvalidManifest {
            reason: format!("{field} must be a nonempty string when present"),
        }),
    }
}

fn required_array<'a>(
    object: &'a serde_json::Map<String, Value>,
    field: &str,
) -> Result<&'a [Value], ToolchainError> {
    object
        .get(field)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| ToolchainError::InvalidManifest {
            reason: format!("{field} must be an array"),
        })
}

fn reject_unknown_fields(
    object: &serde_json::Map<String, Value>,
    allowed: &[&str],
) -> Result<(), ToolchainError> {
    let unknown = object
        .keys()
        .find(|field| !allowed.iter().any(|allowed_field| field == allowed_field));
    if let Some(field) = unknown {
        return Err(ToolchainError::InvalidManifest {
            reason: format!("unknown field {field}"),
        });
    }
    Ok(())
}
