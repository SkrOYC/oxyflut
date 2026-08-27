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
use thiserror::Error;

const STAGED_HOST: &str = "x86_64-unknown-linux-gnu";
const RUST_TOOLCHAIN_COMMIT: &str = "88d9e12ae178fab0fb5cc050a94da85685d449ea";
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
        return Err(ToolchainError::UnsupportedHost { host_triple });
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

/// One schema-compatible `resolvedTools` entry proposed for reconciliation.
#[derive(Clone, Debug, Eq, PartialEq)]
struct ResolvedTool {
    name: String,
    version: String,
    source_identity: String,
    host_triple: String,
    license_id: String,
    executable_path: String,
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

#[derive(Clone, Copy)]
struct ToolSpec {
    name: &'static str,
    locator: ToolLocator,
    version_arguments: &'static [&'static str],
    required_version_fragment: &'static str,
    source_version: &'static str,
    source_derivation_fragment: &'static str,
    license_id: &'static str,
}

#[derive(Clone, Copy)]
enum ToolLocator {
    Path(&'static str),
    Rustfmt,
}

const TOOL_SPECS: &[ToolSpec] = &[
    ToolSpec {
        name: "c-compiler",
        locator: ToolLocator::Path("cc"),
        version_arguments: &["--version"],
        required_version_fragment: "clang version 21.1.8",
        source_version: "21.1.8",
        source_derivation_fragment: "oxyflut-clang-with-cc",
        license_id: "Apache-2.0 WITH LLVM-exception",
    },
    ToolSpec {
        name: "cxx-compiler",
        locator: ToolLocator::Path("c++"),
        version_arguments: &["--version"],
        required_version_fragment: "clang version 21.1.8",
        source_version: "21.1.8",
        source_derivation_fragment: "oxyflut-clang-with-cc",
        license_id: "Apache-2.0 WITH LLVM-exception",
    },
    ToolSpec {
        name: "c-header-checker",
        locator: ToolLocator::Path("clang"),
        version_arguments: &["--version"],
        required_version_fragment: "clang version 21.1.8",
        source_version: "21.1.8",
        source_derivation_fragment: "oxyflut-clang-with-cc",
        license_id: "Apache-2.0 WITH LLVM-exception",
    },
    ToolSpec {
        name: "linker",
        locator: ToolLocator::Path("ld.lld"),
        version_arguments: &["--version"],
        required_version_fragment: "LLD 21.1.8",
        source_version: "21.1.8",
        source_derivation_fragment: "lld-21.1.8",
        license_id: "Apache-2.0 WITH LLVM-exception",
    },
    ToolSpec {
        name: "archiver",
        locator: ToolLocator::Path("ar"),
        version_arguments: &["--version"],
        required_version_fragment: "GNU ar (GNU Binutils) 2.46",
        source_version: "2.46",
        source_derivation_fragment: "binutils-wrapper-2.46",
        license_id: "GPL-3.0-or-later",
    },
    ToolSpec {
        name: "symbol-inspector",
        locator: ToolLocator::Path("nm"),
        version_arguments: &["--version"],
        required_version_fragment: "GNU nm (GNU Binutils) 2.46",
        source_version: "2.46",
        source_derivation_fragment: "binutils-wrapper-2.46",
        license_id: "GPL-3.0-or-later",
    },
    ToolSpec {
        name: "bindgen",
        locator: ToolLocator::Path("bindgen"),
        version_arguments: &["--version"],
        required_version_fragment: "bindgen 0.72.1",
        source_version: "0.72.1",
        source_derivation_fragment: "rust-bindgen-0.72.1",
        license_id: "BSD-3-Clause",
    },
    ToolSpec {
        name: "cbindgen",
        locator: ToolLocator::Path("cbindgen"),
        version_arguments: &["--version"],
        required_version_fragment: "cbindgen 0.29.4",
        source_version: "0.29.4",
        source_derivation_fragment: "rust-cbindgen-0.29.4",
        license_id: "MPL-2.0",
    },
    ToolSpec {
        name: "prettier",
        locator: ToolLocator::Path("bunx"),
        version_arguments: &["--no-install", "prettier@3.9.6", "--version"],
        required_version_fragment: "3.9.6",
        source_version: "1.3.13",
        source_derivation_fragment: "bun-1.3.13",
        license_id: "MIT",
    },
    ToolSpec {
        name: "rustfmt",
        locator: ToolLocator::Rustfmt,
        version_arguments: &["--version"],
        required_version_fragment: "88d9e12ae",
        source_version: "1.98.0",
        source_derivation_fragment: "",
        license_id: "MIT OR Apache-2.0",
    },
];

/// Reports a staged-toolchain resolution or verification failure.
#[derive(Debug, Error)]
pub(crate) enum ToolchainError {
    /// The host doesn't match the Linux staged manifest target.
    #[error("the staged native toolchain supports only {host_triple}")]
    UnsupportedHost {
        /// The detected Rust host triple.
        host_triple: String,
    },
    /// A required tool wasn't found in the locked developer environment.
    #[error("required native tool is missing: {name}")]
    MissingTool {
        /// The tool's stable manifest name.
        name: String,
    },
    /// A manifest tried to add a readiness state reserved for the active lock.
    #[error("staged manifest must not declare readiness: {field}")]
    ReadinessField {
        /// The prohibited readiness field.
        field: String,
    },
    /// A manifest authority didn't identify it as a proposal.
    #[error("staged manifest authority is not staged-proposal")]
    InvalidAuthority,
    /// A manifest note didn't explain its nonauthoritative status.
    #[error("staged manifest note doesn't delegate active pins to Stage 3")]
    InvalidNote,
    /// A manifest couldn't be decoded into the staged proposal shape.
    #[error("invalid staged manifest: {reason}")]
    InvalidManifest {
        /// The content-free reason the manifest doesn't have the required shape.
        reason: String,
    },
    /// A manifest repeated a tool entry.
    #[error("staged manifest contains a duplicate tool: {name}")]
    DuplicateTool {
        /// The repeated stable tool name.
        name: String,
    },
    /// A tool not declared by the staged native-contract specification appeared in the manifest.
    #[error("staged manifest contains an unrecognized tool: {name}")]
    UnknownTool {
        /// The unrecognized tool name.
        name: String,
    },
    /// A command was found at a path other than the staged one.
    #[error("native tool executable is substituted: {name}")]
    ExecutableSubstitution {
        /// The substituted stable tool name.
        name: String,
    },
    /// A required tool wasn't supplied by the required immutable source.
    #[error("native tool source identity is invalid: {name}")]
    SourceIdentityMismatch {
        /// The stable tool name.
        name: String,
    },
    /// A required tool didn't report the pinned version.
    #[error("native tool version is invalid: {name}")]
    VersionMismatch {
        /// The stable tool name.
        name: String,
    },
    /// A required tool had a different host triple or license.
    #[error("native tool metadata is invalid: {name}")]
    MetadataMismatch {
        /// The stable tool name.
        name: String,
    },
    /// A required tool's bytes didn't match the staged digest.
    #[error("native tool digest is invalid: {name}")]
    DigestMismatch {
        /// The stable tool name.
        name: String,
    },
    /// The C compiler and header checker don't resolve to the same Clang executable bytes.
    #[error("the C header checker isn't the staged C compiler")]
    HeaderCheckerMismatch,
    /// The Linux libc header directory couldn't be identified from the active Clang search path.
    #[error("the staged Clang libc header directory is missing")]
    MissingLibcHeaders,
    /// The Linux libc header directory didn't match the staged manifest.
    #[error("the staged Linux libc header directory is invalid")]
    LibcHeadersMismatch,
    /// An immutable tool or SDK utility couldn't be executed.
    #[error("native tool execution failed: {name}")]
    ToolExecution {
        /// The stable tool name.
        name: String,
        /// The underlying local execution error.
        #[source]
        source: io::Error,
    },
    /// An immutable tool exited unsuccessfully while being resolved.
    #[error("native tool returned a failure status: {name}")]
    ToolExecutionFailed {
        /// The stable tool name.
        name: String,
    },
    /// A local immutable path couldn't be read.
    #[error("native toolchain I/O failed")]
    Io(#[from] io::Error),
    /// A JSON manifest couldn't be parsed or encoded.
    #[error("native toolchain JSON failed")]
    Json(#[from] serde_json::Error),
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
        executable_path: path_string(&executable_path)?,
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

    let executable_path = executable_path(specification)?;
    if path_string(&executable_path)? != tool.executable_path {
        return Err(ToolchainError::ExecutableSubstitution {
            name: tool.name.clone(),
        });
    }

    let source_identity = source_identity(specification, &executable_path)?;
    if tool.source_identity != source_identity {
        return Err(ToolchainError::SourceIdentityMismatch {
            name: tool.name.clone(),
        });
    }

    let version = command_version(specification, &executable_path)?;
    if !version.contains(specification.required_version_fragment) || tool.version != version {
        return Err(ToolchainError::VersionMismatch {
            name: tool.name.clone(),
        });
    }

    let digest = hash_file(&executable_path)?.to_string();
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
        || utility.version != "glibc-2.42-67"
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
        ToolLocator::Rustfmt => rustfmt_path(),
    }
}

fn rustfmt_path() -> Result<PathBuf, ToolchainError> {
    let rustup = find_in_path("rustup").ok_or_else(|| ToolchainError::MissingTool {
        name: "rustfmt".to_owned(),
    })?;
    let output = Command::new(rustup)
        .args(["which", "--toolchain", "1.98.0", "rustfmt"])
        .output()
        .map_err(|source| ToolchainError::ToolExecution {
            name: "rustfmt".to_owned(),
            source,
        })?;
    if !output.status.success() {
        return Err(ToolchainError::ToolExecutionFailed {
            name: "rustfmt".to_owned(),
        });
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    let executable_path = PathBuf::from(path);
    if !executable_path.is_file() {
        return Err(ToolchainError::MissingTool {
            name: "rustfmt".to_owned(),
        });
    }
    Ok(executable_path)
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
        ToolLocator::Rustfmt => Ok(format!(
            "rustup-toolchain: 1.98.0; commit: {RUST_TOOLCHAIN_COMMIT}"
        )),
    }
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

fn host_triple() -> Result<String, ToolchainError> {
    let rustc = find_in_path("rustc").ok_or_else(|| ToolchainError::MissingTool {
        name: "rustc".to_owned(),
    })?;
    let output = Command::new(rustc)
        .args(["+1.98.0", "-vV"])
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

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::path::{Path, PathBuf};

    use super::{STAGED_HOST, ToolchainError, ToolchainManifest, resolve, verify};

    #[test]
    fn every_required_tool_has_complete_staged_metadata() -> Result<(), Box<dyn Error>> {
        let manifest = resolve()?;
        assert_eq!(manifest.authority, "staged-proposal");
        assert!(manifest.note.contains("Stage 3 reconciliation"));
        assert_eq!(manifest.resolved_tools.len(), super::TOOL_SPECS.len());
        assert_eq!(manifest.sdk_utilities.len(), 1);

        for tool in &manifest.resolved_tools {
            assert!(!tool.name.is_empty());
            assert!(!tool.version.is_empty());
            assert!(!tool.source_identity.is_empty());
            assert_eq!(tool.host_triple, STAGED_HOST);
            assert!(!tool.license_id.is_empty());
            assert!(Path::new(&tool.executable_path).is_file());
            assert!(tool.sha256.parse::<super::Sha256Digest>().is_ok());
        }

        let sdk = &manifest.sdk_utilities[0];
        assert_eq!(sdk.name, "linux-libc-headers");
        assert!(!sdk.version.is_empty());
        assert!(!sdk.source_identity.is_empty());
        assert_eq!(sdk.host_triple, STAGED_HOST);
        assert!(!sdk.license_id.is_empty());
        assert!(Path::new(&sdk.path).is_dir());
        assert!(sdk.sha256.parse::<super::Sha256Digest>().is_ok());
        assert!(sdk.purpose.contains("syntax-only"));
        Ok(())
    }

    #[test]
    fn re_resolution_is_byte_identical_on_the_locked_host() -> Result<(), Box<dyn Error>> {
        let first = resolve()?.canonical_bytes()?;
        let second = resolve()?.canonical_bytes()?;
        assert_eq!(first, second);
        Ok(())
    }

    #[test]
    fn committed_manifest_matches_this_locked_host_with_a_clear_cross_host_failure()
    -> Result<(), Box<dyn Error>> {
        assert_eq!(
            super::host_triple()?,
            STAGED_HOST,
            "native-contract-toolchain.json is staged only for x86_64-unknown-linux-gnu; re-resolve and reconcile a proposal for this host"
        );
        let generated = resolve()?.canonical_bytes()?;
        let committed =
            fs::read(workspace_root()?.join("qualification/tools/native-contract-toolchain.json"))?;
        assert_eq!(
            generated, committed,
            "the committed staged proposal differs from this locked host; regenerate it only after reviewing the immutable tool identities"
        );
        Ok(())
    }

    #[test]
    fn malformed_toolchain_fixtures_fail_before_native_validation() -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?.join("qualification/fixtures/toolchain");
        let cases = [
            ("missing-tool.json", "missing"),
            ("substituted-executable.json", "substituted"),
            ("digest-mismatch.json", "digest"),
            ("wrong-version.json", "version"),
        ];

        for (fixture, reason) in cases {
            let manifest = ToolchainManifest::from_json(&fs::read(root.join(fixture))?)?;
            let error = verify(&manifest)
                .err()
                .ok_or("fixture unexpectedly verified")?;
            match (reason, error) {
                ("missing", ToolchainError::MissingTool { .. })
                | ("substituted", ToolchainError::ExecutableSubstitution { .. })
                | ("digest", ToolchainError::DigestMismatch { .. })
                | ("version", ToolchainError::VersionMismatch { .. }) => {}
                (_, error) => return Err(format!("fixture {fixture} failed for {error}").into()),
            }
        }
        Ok(())
    }

    #[test]
    fn staged_manifest_rejects_readiness_declarations() -> Result<(), Box<dyn Error>> {
        let path =
            workspace_root()?.join("qualification/fixtures/toolchain/readiness-declaration.json");
        let result = ToolchainManifest::from_json(&fs::read(path)?);
        assert!(matches!(result, Err(ToolchainError::ReadinessField { .. })));
        Ok(())
    }

    fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| "xtask must remain directly below the workspace root".into())
    }
}
