//! Staged native toolchain error types.

use std::io;

use thiserror::Error;

/// Reports a staged-toolchain resolution or verification failure.
#[derive(Debug, Error)]
pub(crate) enum ToolchainError {
    /// The host doesn't match the Linux staged manifest target.
    #[error("the staged native toolchain supports only {supported_host}; detected {detected_host}")]
    UnsupportedHost {
        /// The staged native toolchain's supported Rust host triple.
        supported_host: &'static str,
        /// The detected Rust host triple.
        detected_host: String,
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
    /// A qualification-lock entry differs from the staged tool's immutable metadata.
    #[error("qualification-lock tool entry is invalid: {name}")]
    LockEntryMismatch {
        /// The mismatched stable tool name.
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
