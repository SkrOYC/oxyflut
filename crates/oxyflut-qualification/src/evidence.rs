//! Canonical local evidence writing and verification.
//!
//! ADR-0008 requires immutable UTF-8 JSON evidence below the repository's `qualification/` evidence root. Content-addressed derived paths deduplicate equal bytes and reject collisions.
use crate::hash::{DigestParseError, Sha256Digest, hash_file, hash_reader};
use crate::identifiers::{IdentifierError, RepositoryPath};
use serde_json::Value;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Cursor, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use thiserror::Error;
/// The repository-relative root that contains all qualification evidence.
pub const REPOSITORY_EVIDENCE_ROOT: &str = "qualification";
const DERIVED_EVIDENCE_DIRECTORY: &str = "qualification/evidence/derived";
const JSON_MEDIA_TYPE: &str = "application/json";
const TEMPORARY_ATTEMPTS: u8 = 16;
static TEMPORARY_SEQUENCE: AtomicU64 = AtomicU64::new(0);
/// A recorded media type for immutable evidence bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaType(String);

impl MediaType {
    /// Parses a nonempty media type without whitespace or control characters.
    ///
    /// # Errors
    /// Returns [`EvidenceError::InvalidMediaType`] when the value cannot identify a media type.
    pub fn parse(value: &str) -> Result<Self, EvidenceError> {
        let slash = value.find('/');
        let is_valid = !value.is_empty()
            && slash.is_some_and(|index| index > 0 && index + 1 < value.len())
            && !value
                .chars()
                .any(|character| character.is_control() || character.is_whitespace());
        if is_valid {
            Ok(Self(value.to_owned()))
        } else {
            Err(EvidenceError::InvalidMediaType {
                media_type: value.to_owned(),
            })
        }
    }

    /// Returns the registered JSON media type.
    #[must_use]
    pub fn application_json() -> Self {
        Self(JSON_MEDIA_TYPE.to_owned())
    }

    /// Returns the recorded media type text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn is_json(&self) -> bool {
        self.0 == JSON_MEDIA_TYPE || self.0.ends_with("+json")
    }
}

/// A repository-confined immutable evidence reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceRef {
    /// Canonical repository-relative path below [`REPOSITORY_EVIDENCE_ROOT`].
    pub path: RepositoryPath,
    /// Streamed SHA-256 digest of the referenced bytes.
    pub sha256: Sha256Digest,
    /// Recorded media type of the referenced bytes.
    pub media_type: MediaType,
    /// Exact number of referenced bytes.
    pub size_bytes: u64,
}

/// A verified local evidence file.
#[derive(Clone, Debug)]
pub struct VerifiedEvidence {
    path: RepositoryPath,
    resolved_path: PathBuf,
    sha256: Sha256Digest,
    size_bytes: u64,
    json: Option<Value>,
}

impl VerifiedEvidence {
    /// Returns the canonical repository-relative path that was verified.
    #[must_use]
    pub fn path(&self) -> &RepositoryPath {
        &self.path
    }

    /// Returns the canonical local path used for verification.
    #[must_use]
    pub fn resolved_path(&self) -> &Path {
        &self.resolved_path
    }

    /// Returns the streamed SHA-256 digest of the verified bytes.
    #[must_use]
    pub const fn sha256(&self) -> Sha256Digest {
        self.sha256
    }

    /// Returns the exact byte count of the verified file.
    #[must_use]
    pub const fn size_bytes(&self) -> u64 {
        self.size_bytes
    }

    /// Returns decoded JSON when the caller supplied a JSON media type.
    #[must_use]
    pub fn json(&self) -> Option<&Value> {
        self.json.as_ref()
    }
}

/// Reports why evidence bytes, references, provenance, or local publication are invalid.
#[derive(Debug, Error)]
pub enum EvidenceError {
    /// A media type was empty, incomplete, or contained whitespace or controls.
    #[error("evidence media type is invalid")]
    InvalidMediaType {
        /// The rejected media type text.
        media_type: String,
    },
    /// A repository-relative path violated the canonical path contract.
    #[error("evidence path is not canonical")]
    InvalidPath {
        /// The rejected path text.
        path: String,
        /// The canonical path rule that rejected the path.
        #[source]
        source: IdentifierError,
    },
    /// A canonical path was outside the repository evidence root.
    #[error("evidence path is outside the repository evidence root")]
    OutsideEvidenceRoot {
        /// The path that did not begin below the evidence root.
        path: RepositoryPath,
    },
    /// The repository root couldn't be resolved before local verification.
    #[error("could not resolve the repository root")]
    Root {
        /// The requested repository root.
        path: PathBuf,
        /// The local filesystem failure.
        #[source]
        source: io::Error,
    },
    /// A referenced local evidence file didn't exist.
    #[error("evidence file is missing")]
    MissingFile {
        /// The missing canonical evidence path.
        path: RepositoryPath,
        /// The local filesystem failure.
        #[source]
        source: io::Error,
    },
    /// A local filesystem operation couldn't complete.
    #[error("local evidence I/O failed")]
    Io {
        /// The local path involved in the failed operation.
        path: PathBuf,
        /// The local filesystem failure.
        #[source]
        source: io::Error,
    },
    /// A resolved symlink or filesystem indirection escaped the repository root.
    #[error("evidence path escapes the repository root")]
    SymlinkEscape {
        /// The canonical evidence path that escaped the root.
        path: RepositoryPath,
    },
    /// A referenced evidence path didn't resolve to a regular file.
    #[error("evidence path is not a regular file")]
    NotRegularFile {
        /// The canonical evidence path that wasn't a regular file.
        path: RepositoryPath,
    },
    /// A declared SHA-256 value wasn't lowercase hexadecimal.
    #[error("evidence SHA-256 is invalid")]
    InvalidDigest {
        /// The path associated with the invalid digest.
        path: RepositoryPath,
        /// The digest parser failure.
        #[source]
        source: DigestParseError,
    },
    /// A streamed digest differed from the declared immutable digest.
    #[error("evidence SHA-256 does not match")]
    DigestMismatch {
        /// The path whose digest didn't match.
        path: RepositoryPath,
    },
    /// The evidence size didn't match the declared immutable size.
    #[error("evidence size does not match")]
    SizeMismatch {
        /// The path whose size didn't match.
        path: RepositoryPath,
    },
    /// JSON evidence couldn't be decoded as UTF-8 JSON.
    #[error("evidence JSON is invalid")]
    Json {
        /// The local JSON path.
        path: PathBuf,
        /// The JSON decoding failure.
        #[source]
        source: serde_json::Error,
    },
    /// A canonical JSON encoder couldn't serialize one JSON string.
    #[error("canonical evidence JSON could not be encoded")]
    JsonEncoding {
        /// The JSON encoding failure.
        #[source]
        source: serde_json::Error,
    },
    /// A derived record wasn't a JSON object.
    #[error("derived evidence record must be a JSON object")]
    DerivedRecordNotObject,
    /// A derived record attempted to replace reserved provenance fields.
    #[error("derived evidence record contains reserved provenance fields")]
    ReservedProvenanceField,
    /// A derived file had only one of its required source provenance fields.
    #[error("derived evidence provenance is incomplete")]
    IncompleteProvenance,
    /// A derived file's source provenance field had an invalid type or value.
    #[error("derived evidence provenance is invalid")]
    InvalidProvenance,
    /// A derived JSON file didn't use the canonical UTF-8 representation.
    #[error("derived evidence JSON is not canonical")]
    NonCanonicalJson {
        /// The local derived JSON path.
        path: PathBuf,
    },
    /// A content-addressed destination existed with bytes other than its address.
    #[error("content-addressed evidence destination conflicts with existing bytes")]
    ContentAddressCollision {
        /// The conflicting local destination.
        path: PathBuf,
    },
    /// Another local writer held the content-addressed destination lock.
    #[error("content-addressed evidence write is already in progress")]
    WriteInProgress {
        /// The lock file owned by the competing writer.
        path: PathBuf,
    },
    /// A temporary output file couldn't be created after bounded attempts.
    #[error("could not create a temporary evidence file")]
    TemporaryFile {
        /// The destination whose temporary sibling couldn't be created.
        path: PathBuf,
    },
    /// A temporary output couldn't be removed after a failed write or publication.
    #[error("could not remove a temporary evidence file")]
    TemporaryCleanup {
        /// The temporary output path.
        path: PathBuf,
        /// The local filesystem failure.
        #[source]
        source: io::Error,
    },
    /// A completed temporary output couldn't be atomically renamed into place.
    #[error("could not publish local evidence atomically")]
    Rename {
        /// The final evidence destination.
        path: PathBuf,
        /// The local filesystem failure.
        #[source]
        source: io::Error,
    },
    /// A write lock couldn't be released after publication completed.
    #[error("could not release the local evidence write lock")]
    LockCleanup {
        /// The write lock that couldn't be removed.
        path: PathBuf,
        /// The local filesystem failure.
        #[source]
        source: io::Error,
    },
}

/// Encodes one JSON value as deterministic UTF-8 JSON with sorted keys, fixed pinned number formatting, and a trailing line feed.
///
/// # Errors
/// Returns an error only if a JSON string cannot be encoded.
pub fn canonical_json_bytes(value: &Value) -> Result<Vec<u8>, EvidenceError> {
    let mut output = Vec::new();
    write_canonical_json_value(&mut output, value)?;
    output.push(b'\n');
    Ok(output)
}

/// Records existing source bytes without modifying them.
///
/// # Errors
/// Returns an error when the path is outside the evidence root or can't be verified locally.
pub fn preserve_source(
    root: &Path,
    path: RepositoryPath,
    media_type: MediaType,
) -> Result<EvidenceRef, EvidenceError> {
    ensure_evidence_path(&path)?;
    let verified = verify_file(root, &path, &media_type)?;
    Ok(EvidenceRef {
        path,
        sha256: verified.sha256(),
        media_type,
        size_bytes: verified.size_bytes(),
    })
}

/// Writes a content-addressed derived JSON record with verified `sourcePath` and `sourceSha256` provenance without opening its source for writing.
///
/// # Errors
/// Returns an error when provenance is invalid, the record isn't an object, or publication fails.
pub fn write_derived_json(
    root: &Path,
    record: &Value,
    source: &EvidenceRef,
) -> Result<EvidenceRef, EvidenceError> {
    write_derived_json_with(root, record, source, |writer, bytes| {
        writer.write_all(bytes)
    })
}

/// Verifies every field of one immutable evidence reference.
///
/// # Errors
/// Returns an error if the reference is outside the evidence root or its digest or size differs.
pub fn verify_reference(
    root: &Path,
    reference: &EvidenceRef,
) -> Result<VerifiedEvidence, EvidenceError> {
    ensure_evidence_path(&reference.path)?;
    let verified = verify_file(root, &reference.path, &reference.media_type)?;
    if verified.sha256() != reference.sha256 {
        return Err(EvidenceError::DigestMismatch {
            path: reference.path.clone(),
        });
    }
    if verified.size_bytes() != reference.size_bytes {
        return Err(EvidenceError::SizeMismatch {
            path: reference.path.clone(),
        });
    }
    Ok(verified)
}

/// Verifies one digest-bound path below the repository evidence root for schemas without the full [`EvidenceRef`] fields.
///
/// # Errors
/// Returns an error when the path is invalid, escapes the root, isn't a regular file, or has a different streamed digest.
pub fn verify_path_digest(
    root: &Path,
    path: &RepositoryPath,
    expected_digest: &Sha256Digest,
) -> Result<VerifiedEvidence, EvidenceError> {
    ensure_evidence_path(path)?;
    let (resolved_path, size_bytes) = resolve_regular_file(root, path)?;
    let actual = hash_file(&resolved_path).map_err(|source| EvidenceError::Io {
        path: resolved_path.clone(),
        source,
    })?;
    if actual != *expected_digest {
        return Err(EvidenceError::DigestMismatch { path: path.clone() });
    }
    Ok(VerifiedEvidence {
        path: path.clone(),
        resolved_path,
        sha256: actual,
        size_bytes,
        json: None,
    })
}

/// Verifies one evidence file and checks canonical JSON only for records with `sourcePath` and `sourceSha256`; preserved source JSON is parsed but never rewritten.
///
/// # Errors
/// Returns an error if path, bytes, JSON, or derived provenance validation fails.
pub fn verify_file(
    root: &Path,
    path: &RepositoryPath,
    media_type: &MediaType,
) -> Result<VerifiedEvidence, EvidenceError> {
    ensure_evidence_path(path)?;
    let (resolved_path, size_bytes) = resolve_regular_file(root, path)?;
    let sha256 = hash_file(&resolved_path).map_err(|source| EvidenceError::Io {
        path: resolved_path.clone(),
        source,
    })?;
    let json = if media_type.is_json() {
        let bytes = fs::read(&resolved_path).map_err(|source| EvidenceError::Io {
            path: resolved_path.clone(),
            source,
        })?;
        let value = serde_json::from_slice(&bytes).map_err(|source| EvidenceError::Json {
            path: resolved_path.clone(),
            source,
        })?;
        verify_derived_json(root, &resolved_path, &bytes, &value)?;
        Some(value)
    } else {
        None
    };

    Ok(VerifiedEvidence {
        path: path.clone(),
        resolved_path,
        sha256,
        size_bytes,
        json,
    })
}

fn write_derived_json_with<F>(
    root: &Path,
    record: &Value,
    source: &EvidenceRef,
    write: F,
) -> Result<EvidenceRef, EvidenceError>
where
    F: FnOnce(&mut dyn Write, &[u8]) -> io::Result<()>,
{
    let _ = verify_reference(root, source)?;
    let mut record = record
        .as_object()
        .cloned()
        .ok_or(EvidenceError::DerivedRecordNotObject)?;
    if record.contains_key("sourcePath") || record.contains_key("sourceSha256") {
        return Err(EvidenceError::ReservedProvenanceField);
    }
    record.insert(
        "sourcePath".to_owned(),
        Value::String(source.path.as_str().to_owned()),
    );
    record.insert(
        "sourceSha256".to_owned(),
        Value::String(source.sha256.to_string()),
    );

    let bytes = canonical_json_bytes(&Value::Object(record))?;
    let digest = hash_reader(Cursor::new(&bytes)).map_err(|source| EvidenceError::Io {
        path: root.join(DERIVED_EVIDENCE_DIRECTORY),
        source,
    })?;
    let path = RepositoryPath::parse(&format!("{DERIVED_EVIDENCE_DIRECTORY}/{digest}.json"))
        .map_err(|source| EvidenceError::InvalidPath {
            path: DERIVED_EVIDENCE_DIRECTORY.to_owned(),
            source,
        })?;
    let size_bytes = u64::try_from(bytes.len()).map_err(|_| EvidenceError::Io {
        path: root.join(DERIVED_EVIDENCE_DIRECTORY),
        source: io::Error::other("derived evidence exceeds supported file size"),
    })?;
    let reference = EvidenceRef {
        path,
        sha256: digest,
        media_type: MediaType::application_json(),
        size_bytes,
    };
    publish_content_addressed(root, &reference, &bytes, write)?;
    Ok(reference)
}

fn publish_content_addressed<F>(
    root: &Path,
    reference: &EvidenceRef,
    bytes: &[u8],
    write: F,
) -> Result<(), EvidenceError>
where
    F: FnOnce(&mut dyn Write, &[u8]) -> io::Result<()>,
{
    let destination = root.join(reference.path.as_str());
    let Some(directory) = destination.parent() else {
        return Err(EvidenceError::Io {
            path: destination,
            source: io::Error::other("derived evidence destination has no parent"),
        });
    };
    fs::create_dir_all(directory).map_err(|source| EvidenceError::Io {
        path: directory.to_path_buf(),
        source,
    })?;
    let lock_path = lock_path(&destination)?;
    acquire_lock(&lock_path)?;

    let result = if destination.exists() {
        verify_existing_destination(root, reference, &destination)
    } else {
        match atomic_write(&destination, |writer| write(writer, bytes)) {
            Ok(()) => verify_existing_destination(root, reference, &destination),
            Err(error) => Err(error),
        }
    };
    let lock_result = release_lock(&lock_path);
    match (result, lock_result) {
        (Ok(()), Ok(())) => Ok(()),
        (Ok(()), Err(error)) | (Err(error), Ok(())) | (Err(error), Err(_)) => Err(error),
    }
}

fn verify_existing_destination(
    root: &Path,
    reference: &EvidenceRef,
    destination: &Path,
) -> Result<(), EvidenceError> {
    match verify_reference(root, reference) {
        Ok(_) => Ok(()),
        Err(EvidenceError::DigestMismatch { .. }) | Err(EvidenceError::SizeMismatch { .. }) => {
            Err(EvidenceError::ContentAddressCollision {
                path: destination.to_path_buf(),
            })
        }
        Err(error) => Err(error),
    }
}

fn verify_derived_json(
    root: &Path,
    path: &Path,
    bytes: &[u8],
    value: &Value,
) -> Result<(), EvidenceError> {
    let Some(object) = value.as_object() else {
        return Ok(());
    };
    match (object.get("sourcePath"), object.get("sourceSha256")) {
        (None, None) => Ok(()),
        (Some(Value::String(source_path)), Some(Value::String(source_digest))) => {
            let canonical = canonical_json_bytes(value)?;
            if bytes != canonical {
                return Err(EvidenceError::NonCanonicalJson {
                    path: path.to_path_buf(),
                });
            }
            let source_path = RepositoryPath::parse(source_path).map_err(|source| {
                EvidenceError::InvalidPath {
                    path: source_path.to_owned(),
                    source,
                }
            })?;
            let source_digest =
                source_digest
                    .parse()
                    .map_err(|source| EvidenceError::InvalidDigest {
                        path: source_path.clone(),
                        source,
                    })?;
            let _ = verify_path_digest(root, &source_path, &source_digest)?;
            Ok(())
        }
        (Some(_), Some(_)) => Err(EvidenceError::InvalidProvenance),
        (None, Some(_)) | (Some(_), None) => Err(EvidenceError::IncompleteProvenance),
    }
}

fn ensure_evidence_path(path: &RepositoryPath) -> Result<(), EvidenceError> {
    if path
        .as_str()
        .strip_prefix(REPOSITORY_EVIDENCE_ROOT)
        .is_some_and(|suffix| suffix.starts_with('/'))
    {
        Ok(())
    } else {
        Err(EvidenceError::OutsideEvidenceRoot { path: path.clone() })
    }
}

fn resolve_regular_file(
    root: &Path,
    path: &RepositoryPath,
) -> Result<(PathBuf, u64), EvidenceError> {
    let canonical_root = fs::canonicalize(root).map_err(|source| EvidenceError::Root {
        path: root.to_path_buf(),
        source,
    })?;
    let unresolved = root.join(path.as_str());
    let resolved_path = fs::canonicalize(&unresolved).map_err(|source| {
        if source.kind() == io::ErrorKind::NotFound {
            EvidenceError::MissingFile {
                path: path.clone(),
                source,
            }
        } else {
            EvidenceError::Io {
                path: unresolved,
                source,
            }
        }
    })?;
    if !resolved_path.starts_with(&canonical_root) {
        return Err(EvidenceError::SymlinkEscape { path: path.clone() });
    }
    let metadata = fs::metadata(&resolved_path).map_err(|source| EvidenceError::Io {
        path: resolved_path.clone(),
        source,
    })?;
    if !metadata.is_file() {
        return Err(EvidenceError::NotRegularFile { path: path.clone() });
    }
    Ok((resolved_path, metadata.len()))
}

fn write_canonical_json_value(output: &mut Vec<u8>, value: &Value) -> Result<(), EvidenceError> {
    match value {
        Value::Null => output.extend_from_slice(b"null"),
        Value::Bool(true) => output.extend_from_slice(b"true"),
        Value::Bool(false) => output.extend_from_slice(b"false"),
        Value::Number(number) => write_canonical_number(output, number),
        Value::String(string) => serde_json::to_writer(&mut *output, string)
            .map_err(|source| EvidenceError::JsonEncoding { source })?,
        Value::Array(values) => {
            output.push(b'[');
            for (index, item) in values.iter().enumerate() {
                if index > 0 {
                    output.push(b',');
                }
                write_canonical_json_value(output, item)?;
            }
            output.push(b']');
        }
        Value::Object(values) => {
            let mut entries = values.iter().collect::<Vec<_>>();
            entries.sort_unstable_by(|left, right| left.0.cmp(right.0));
            output.push(b'{');
            for (index, (key, item)) in entries.into_iter().enumerate() {
                if index > 0 {
                    output.push(b',');
                }
                serde_json::to_writer(&mut *output, key)
                    .map_err(|source| EvidenceError::JsonEncoding { source })?;
                output.push(b':');
                write_canonical_json_value(output, item)?;
            }
            output.push(b'}');
        }
    }
    Ok(())
}

fn write_canonical_number(output: &mut Vec<u8>, number: &serde_json::Number) {
    if let Some(integer) = number.as_i64() {
        output.extend_from_slice(integer.to_string().as_bytes());
    } else if let Some(integer) = number.as_u64() {
        output.extend_from_slice(integer.to_string().as_bytes());
    } else if let Some(float) = number.as_f64() {
        if float == 0.0 {
            output.push(b'0');
        } else {
            output.extend_from_slice(float.to_string().as_bytes());
        }
    }
}

fn lock_path(destination: &Path) -> Result<PathBuf, EvidenceError> {
    let file_name = destination.file_name().ok_or_else(|| EvidenceError::Io {
        path: destination.to_path_buf(),
        source: io::Error::other("derived evidence destination has no file name"),
    })?;
    let mut lock_name = file_name.to_os_string();
    lock_name.push(".lock");
    Ok(destination.with_file_name(lock_name))
}

fn acquire_lock(path: &Path) -> Result<(), EvidenceError> {
    match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(lock) => lock.sync_all().map_err(|source| EvidenceError::Io {
            path: path.to_path_buf(),
            source,
        }),
        Err(source) if source.kind() == io::ErrorKind::AlreadyExists => {
            Err(EvidenceError::WriteInProgress {
                path: path.to_path_buf(),
            })
        }
        Err(source) => Err(EvidenceError::Io {
            path: path.to_path_buf(),
            source,
        }),
    }
}

fn release_lock(path: &Path) -> Result<(), EvidenceError> {
    fs::remove_file(path).map_err(|source| EvidenceError::LockCleanup {
        path: path.to_path_buf(),
        source,
    })
}

fn atomic_write<F>(destination: &Path, write: F) -> Result<(), EvidenceError>
where
    F: FnOnce(&mut dyn Write) -> io::Result<()>,
{
    let (temporary_path, mut temporary_file) = open_temporary_file(destination)?;
    let write_result = write(&mut temporary_file).and_then(|()| temporary_file.sync_all());
    drop(temporary_file);
    if let Err(source) = write_result {
        return remove_temporary_after_failure(&temporary_path, source);
    }

    if let Err(rename_error) = fs::rename(&temporary_path, destination) {
        return match fs::remove_file(&temporary_path) {
            Ok(()) => Err(EvidenceError::Rename {
                path: destination.to_path_buf(),
                source: rename_error,
            }),
            Err(source) => Err(EvidenceError::TemporaryCleanup {
                path: temporary_path,
                source,
            }),
        };
    }
    let Some(directory) = destination.parent() else {
        return Err(EvidenceError::Io {
            path: destination.to_path_buf(),
            source: io::Error::other("derived evidence destination has no parent"),
        });
    };
    File::open(directory)
        .and_then(|directory| directory.sync_all())
        .map_err(|source| EvidenceError::Io {
            path: directory.to_path_buf(),
            source,
        })
}

fn open_temporary_file(destination: &Path) -> Result<(PathBuf, File), EvidenceError> {
    let Some(directory) = destination.parent() else {
        return Err(EvidenceError::Io {
            path: destination.to_path_buf(),
            source: io::Error::other("derived evidence destination has no parent"),
        });
    };
    let file_name = destination.file_name().ok_or_else(|| EvidenceError::Io {
        path: destination.to_path_buf(),
        source: io::Error::other("derived evidence destination has no file name"),
    })?;

    for _ in 0..TEMPORARY_ATTEMPTS {
        let sequence = TEMPORARY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let temporary_name = format!(
            ".{}.{}.{}.tmp",
            file_name.to_string_lossy(),
            std::process::id(),
            sequence
        );
        let temporary_path = directory.join(temporary_name);
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary_path)
        {
            Ok(file) => return Ok((temporary_path, file)),
            Err(source) if source.kind() == io::ErrorKind::AlreadyExists => {}
            Err(source) => {
                return Err(EvidenceError::Io {
                    path: temporary_path,
                    source,
                });
            }
        }
    }
    Err(EvidenceError::TemporaryFile {
        path: destination.to_path_buf(),
    })
}

fn remove_temporary_after_failure(
    temporary_path: &Path,
    write_error: io::Error,
) -> Result<(), EvidenceError> {
    fs::remove_file(temporary_path).map_err(|source| EvidenceError::TemporaryCleanup {
        path: temporary_path.to_path_buf(),
        source,
    })?;
    Err(EvidenceError::Io {
        path: temporary_path.to_path_buf(),
        source: write_error,
    })
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::io::{self, Write};
    use std::path::{Path, PathBuf};

    use serde_json::Value;

    use super::{
        EvidenceError, MediaType, RepositoryPath, atomic_write, canonical_json_bytes,
        preserve_source, verify_file, verify_reference, write_derived_json,
        write_derived_json_with,
    };

    const FIXTURE_DIRECTORY: &str = "qualification/fixtures/evidence";

    #[test]
    fn equal_logical_records_produce_byte_identical_derived_json() -> Result<(), Box<dyn Error>> {
        let values = fixture_json("equal-logical-records.json")?;
        let first = values
            .get("first")
            .ok_or("fixture must contain first record")?;
        let second = values
            .get("second")
            .ok_or("fixture must contain second record")?;
        assert_eq!(canonical_json_bytes(first)?, canonical_json_bytes(second)?);

        let root = TestRepository::new("equal-records")?;
        copy_fixture(root.path(), "preserved-source.json")?;
        let source = preserve_source(
            root.path(),
            RepositoryPath::parse("qualification/fixtures/evidence/preserved-source.json")?,
            MediaType::application_json(),
        )?;
        let first_reference = write_derived_json(root.path(), first, &source)?;
        let second_reference = write_derived_json(root.path(), second, &source)?;
        assert_eq!(first_reference, second_reference);
        assert_eq!(
            fs::read(root.path().join(first_reference.path.as_str()))?,
            fs::read(root.path().join(second_reference.path.as_str()))?
        );
        Ok(())
    }

    #[test]
    fn source_bytes_remain_preserved_and_derived_provenance_verifies() -> Result<(), Box<dyn Error>>
    {
        let root = TestRepository::new("preserved-source")?;
        let source_path = copy_fixture(root.path(), "preserved-source.json")?;
        let before = fs::read(&source_path)?;
        let source = preserve_source(
            root.path(),
            RepositoryPath::parse("qualification/fixtures/evidence/preserved-source.json")?,
            MediaType::application_json(),
        )?;
        let record = fixture_json("partial-write.json")?;
        let derived = write_derived_json(root.path(), &record, &source)?;

        assert_eq!(fs::read(&source_path)?, before);
        let verified = verify_reference(root.path(), &derived)?;
        let provenance = verified
            .json()
            .and_then(|value| value.as_object())
            .ok_or("derived record must decode as an object")?;
        assert_eq!(
            provenance.get("sourcePath").and_then(Value::as_str),
            Some(source.path.as_str())
        );
        let source_digest = source.sha256.to_string();
        assert_eq!(
            provenance.get("sourceSha256").and_then(Value::as_str),
            Some(source_digest.as_str())
        );
        Ok(())
    }

    #[test]
    fn bad_digest_and_out_of_root_fixtures_fail_closed() -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let media_type = MediaType::application_json();
        let bad_digest = RepositoryPath::parse("qualification/fixtures/evidence/bad-digest.json")?;
        assert!(matches!(
            verify_file(&root, &bad_digest, &media_type),
            Err(EvidenceError::DigestMismatch { .. })
        ));
        let out_of_root =
            RepositoryPath::parse("qualification/fixtures/evidence/out-of-root.json")?;
        assert!(matches!(
            verify_file(&root, &out_of_root, &media_type),
            Err(EvidenceError::OutsideEvidenceRoot { .. })
        ));
        Ok(())
    }

    #[test]
    fn interrupted_write_publishes_no_partial_file_or_reference() -> Result<(), Box<dyn Error>> {
        let root = TestRepository::new("interrupted-write")?;
        let source_path = copy_fixture(root.path(), "preserved-source.json")?;
        let source = preserve_source(
            root.path(),
            RepositoryPath::parse("qualification/fixtures/evidence/preserved-source.json")?,
            MediaType::application_json(),
        )?;
        let record = fixture_json("partial-write.json")?;
        let result = write_derived_json_with(root.path(), &record, &source, |writer, bytes| {
            let mut failing_writer = FailingWriter::new(writer, bytes.len() / 2);
            failing_writer.write_all(bytes)
        });
        assert!(result.is_err());
        assert_eq!(
            fs::read(&source_path)?,
            fs::read(fixture_path("preserved-source.json"))?
        );

        let derived_directory = root.path().join("qualification/evidence/derived");
        let files = if derived_directory.exists() {
            fs::read_dir(derived_directory)?.collect::<Result<Vec<_>, io::Error>>()?
        } else {
            Vec::new()
        };
        assert!(
            files.is_empty(),
            "interrupted writes must leave no published files"
        );

        let target = root.path().join("qualification/evidence/partial.json");
        let failed_atomic_write = atomic_write(&target, |writer| {
            let mut failing_writer = FailingWriter::new(writer, 3);
            failing_writer.write_all(b"partial")
        });
        assert!(failed_atomic_write.is_err());
        assert!(!target.exists());
        Ok(())
    }

    fn fixture_json(name: &str) -> Result<Value, Box<dyn Error>> {
        Ok(serde_json::from_slice(&fs::read(fixture_path(name))?)?)
    }

    fn fixture_path(name: &str) -> PathBuf {
        workspace_root_path().join(FIXTURE_DIRECTORY).join(name)
    }

    fn copy_fixture(root: &Path, name: &str) -> Result<PathBuf, Box<dyn Error>> {
        let destination = root.join(FIXTURE_DIRECTORY).join(name);
        let parent = destination
            .parent()
            .ok_or("fixture destination must have a parent")?;
        fs::create_dir_all(parent)?;
        fs::copy(fixture_path(name), &destination)?;
        Ok(destination)
    }

    fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
        Ok(workspace_root_path())
    }

    fn workspace_root_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."))
    }

    struct TestRepository {
        path: PathBuf,
    }

    impl TestRepository {
        fn new(name: &str) -> Result<Self, Box<dyn Error>> {
            let sequence = super::TEMPORARY_SEQUENCE.fetch_add(1, super::Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "oxyflut-evidence-{name}-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir_all(&path)?;
            Ok(Self { path })
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestRepository {
        fn drop(&mut self) {
            if fs::remove_dir_all(&self.path).is_err() {}
        }
    }

    struct FailingWriter<'writer> {
        writer: &'writer mut dyn Write,
        remaining: usize,
    }

    impl<'writer> FailingWriter<'writer> {
        fn new(writer: &'writer mut dyn Write, remaining: usize) -> Self {
            Self { writer, remaining }
        }
    }

    impl Write for FailingWriter<'_> {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            if self.remaining == 0 {
                return Err(io::Error::other("simulated interrupted write"));
            }
            let count = buffer.len().min(self.remaining);
            let written = self.writer.write(&buffer[..count])?;
            self.remaining -= written;
            Ok(written)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.writer.flush()
        }
    }
}
