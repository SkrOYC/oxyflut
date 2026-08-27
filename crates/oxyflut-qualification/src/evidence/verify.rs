//! Immutable evidence verification from one file snapshot.

use std::fs::{self, File, Metadata};
use std::io::{self, Cursor, Read};
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::hash::{Sha256Digest, hash_reader};
use crate::identifiers::RepositoryPath;

use super::canonical_json_bytes;
use super::{
    EvidenceError, EvidenceRef, MediaType, REPOSITORY_EVIDENCE_ROOT, VerifiedEvidence,
    ensure_evidence_path,
};

/// Records existing source bytes without modifying them.
///
/// # Errors
///
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

/// Verifies every field of one immutable evidence reference.
///
/// # Errors
///
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
///
/// Returns an error when the path is invalid, escapes the root, isn't a regular file, or has a different streamed digest.
pub fn verify_path_digest(
    root: &Path,
    path: &RepositoryPath,
    expected_digest: &Sha256Digest,
) -> Result<VerifiedEvidence, EvidenceError> {
    ensure_evidence_path(path)?;
    let resolved_path = resolve_regular_file(root, path)?;
    let snapshot = read_digest_snapshot(path, &resolved_path)?;
    if snapshot.sha256 != *expected_digest {
        return Err(EvidenceError::DigestMismatch { path: path.clone() });
    }
    Ok(VerifiedEvidence {
        path: path.clone(),
        resolved_path,
        sha256: snapshot.sha256,
        size_bytes: snapshot.size_bytes,
        json: None,
    })
}

/// Verifies one evidence file and checks canonical JSON only for records with `sourcePath` and `sourceSha256`; preserved source JSON is parsed but never rewritten.
///
/// # Errors
///
/// Returns an error if path, bytes, JSON, or derived provenance validation fails.
pub fn verify_file(
    root: &Path,
    path: &RepositoryPath,
    media_type: &MediaType,
) -> Result<VerifiedEvidence, EvidenceError> {
    ensure_evidence_path(path)?;
    let resolved_path = resolve_regular_file(root, path)?;
    let snapshot = read_buffered_snapshot(path, &resolved_path)?;
    let json = if media_type.is_json() {
        let value =
            serde_json::from_slice(&snapshot.bytes).map_err(|source| EvidenceError::Json {
                path: resolved_path.clone(),
                source,
            })?;
        verify_derived_json(root, &resolved_path, &snapshot.bytes, &value)?;
        Some(value)
    } else {
        None
    };

    Ok(VerifiedEvidence {
        path: path.clone(),
        resolved_path,
        sha256: snapshot.sha256,
        size_bytes: snapshot.size_bytes,
        json,
    })
}

pub(super) fn verify_existing_destination(
    root: &Path,
    reference: &EvidenceRef,
    destination: &Path,
) -> Result<(), EvidenceError> {
    match verify_path_digest(root, &reference.path, &reference.sha256) {
        Ok(verified) if verified.size_bytes() == reference.size_bytes => Ok(()),
        Ok(_) | Err(EvidenceError::DigestMismatch { .. }) => {
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

fn resolve_regular_file(root: &Path, path: &RepositoryPath) -> Result<PathBuf, EvidenceError> {
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
    let canonical_evidence_root = canonical_root.join(REPOSITORY_EVIDENCE_ROOT);
    if !resolved_path.starts_with(&canonical_evidence_root) {
        return Err(EvidenceError::SymlinkEscape { path: path.clone() });
    }
    Ok(resolved_path)
}

struct DigestSnapshot {
    sha256: Sha256Digest,
    size_bytes: u64,
}

struct BufferedFileSnapshot {
    bytes: Vec<u8>,
    sha256: Sha256Digest,
    size_bytes: u64,
}

fn read_digest_snapshot(
    path: &RepositoryPath,
    resolved_path: &Path,
) -> Result<DigestSnapshot, EvidenceError> {
    let mut file = open_snapshot(resolved_path)?;
    let before = metadata(path, resolved_path, &file)?;
    let sha256 = hash_reader(&mut file).map_err(|source| EvidenceError::Io {
        path: resolved_path.to_path_buf(),
        source,
    })?;
    let after = metadata(path, resolved_path, &file)?;
    if !metadata_matches(&before, &after, before.len(), resolved_path)? {
        return Err(EvidenceError::ChangedDuringRead { path: path.clone() });
    }
    Ok(DigestSnapshot {
        sha256,
        size_bytes: before.len(),
    })
}

fn read_buffered_snapshot(
    path: &RepositoryPath,
    resolved_path: &Path,
) -> Result<BufferedFileSnapshot, EvidenceError> {
    let mut file = open_snapshot(resolved_path)?;
    let before = metadata(path, resolved_path, &file)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|source| EvidenceError::Io {
            path: resolved_path.to_path_buf(),
            source,
        })?;
    let after = metadata(path, resolved_path, &file)?;
    let size_bytes = u64::try_from(bytes.len()).map_err(|_| EvidenceError::Io {
        path: resolved_path.to_path_buf(),
        source: io::Error::other("evidence exceeds supported file size"),
    })?;
    if !metadata_matches(&before, &after, size_bytes, resolved_path)? {
        return Err(EvidenceError::ChangedDuringRead { path: path.clone() });
    }
    let sha256 = hash_reader(Cursor::new(&bytes)).map_err(|source| EvidenceError::Io {
        path: resolved_path.to_path_buf(),
        source,
    })?;
    Ok(BufferedFileSnapshot {
        bytes,
        sha256,
        size_bytes,
    })
}

fn open_snapshot(resolved_path: &Path) -> Result<File, EvidenceError> {
    File::open(resolved_path).map_err(|source| EvidenceError::Io {
        path: resolved_path.to_path_buf(),
        source,
    })
}

fn metadata(
    path: &RepositoryPath,
    resolved_path: &Path,
    file: &File,
) -> Result<Metadata, EvidenceError> {
    let metadata = file.metadata().map_err(|source| EvidenceError::Io {
        path: resolved_path.to_path_buf(),
        source,
    })?;
    if metadata.is_file() {
        Ok(metadata)
    } else {
        Err(EvidenceError::NotRegularFile { path: path.clone() })
    }
}

fn metadata_matches(
    before: &Metadata,
    after: &Metadata,
    bytes_len: u64,
    path: &Path,
) -> Result<bool, EvidenceError> {
    let before_modified = before.modified().map_err(|source| EvidenceError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let after_modified = after.modified().map_err(|source| EvidenceError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(before.is_file()
        && after.is_file()
        && before.len() == bytes_len
        && after.len() == bytes_len
        && before_modified == after_modified)
}
