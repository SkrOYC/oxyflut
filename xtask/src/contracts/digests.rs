//! Repository-confined immutable-file digest validation.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use oxyflut_qualification::evidence::{
    ReferenceDeclaration, declared_references, reference_declaration,
};
use oxyflut_qualification::hash::{DigestParseError, Sha256Digest};
use oxyflut_qualification::identifiers::{IdentifierError, RepositoryPath};
use serde_json::{Map, Value};
use thiserror::Error;

const CONTRACTS_DIRECTORY: &str = ".constitution/tech-spec/contracts";

/// A repository file that was confined and verified against its declared digest.
#[derive(Debug)]
pub(crate) struct VerifiedReference {
    /// The canonical repository-relative path declared by the reference.
    pub(crate) path: RepositoryPath,
    /// The canonical local path used for digest verification.
    pub(crate) resolved_path: PathBuf,
}

/// Reports why a repository-relative immutable-file binding is invalid.
#[derive(Debug, Error)]
pub(crate) enum DigestError {
    /// The declared path violates the canonical repository-relative path contract.
    #[error("digest reference path is not canonical")]
    InvalidPath {
        /// The rejected path text.
        path: String,
        /// The canonical-path rule that rejected the text.
        #[source]
        source: IdentifierError,
    },
    /// The reference uses an absolute filesystem path.
    #[error("digest reference path is absolute")]
    AbsolutePath {
        /// The rejected absolute path.
        path: String,
    },
    /// The repository root could not be canonicalized.
    #[error("could not resolve repository root")]
    Root {
        /// The root path that could not be resolved.
        path: PathBuf,
        /// The underlying filesystem error.
        #[source]
        source: io::Error,
    },
    /// The referenced file does not exist.
    #[error("digest reference file is missing")]
    MissingFile {
        /// The missing repository-relative path.
        path: RepositoryPath,
        /// The underlying filesystem error.
        #[source]
        source: io::Error,
    },
    /// A local filesystem operation failed for a nonmissing reference.
    #[error("could not resolve digest reference")]
    Io {
        /// The affected repository-relative path.
        path: RepositoryPath,
        /// The underlying filesystem error.
        #[source]
        source: io::Error,
    },
    /// Resolving a symlink or other filesystem indirection escaped the repository root.
    #[error("digest reference escapes the repository root")]
    SymlinkEscape {
        /// The canonical path supplied by the reference.
        path: RepositoryPath,
    },
    /// The resolved reference is not a regular file.
    #[error("digest reference is not a regular file")]
    NotRegularFile {
        /// The canonical path supplied by the reference.
        path: RepositoryPath,
    },
    /// The expected digest is not lowercase SHA-256 hexadecimal.
    #[error("digest reference has an invalid SHA-256 value")]
    InvalidDigest {
        /// The canonical path supplied by the reference.
        path: RepositoryPath,
        /// The parser failure.
        #[source]
        source: DigestParseError,
    },
    /// The file's streamed SHA-256 digest differs from the declared immutable digest.
    #[error("digest reference SHA-256 does not match")]
    DigestMismatch {
        /// The canonical path supplied by the reference.
        path: RepositoryPath,
    },
    /// A typed reference object omitted or mistyped its `path` or `sha256` fields.
    #[error("digest reference is incomplete")]
    IncompleteReference,
    /// A contract input could not be read while discovering direct references.
    #[error("could not read contract input for digest validation")]
    ContractIo {
        /// The contract path that could not be read.
        path: PathBuf,
        /// The underlying filesystem error.
        #[source]
        source: io::Error,
    },
    /// A contract input could not be parsed while discovering direct references.
    #[error("could not parse contract input for digest validation")]
    ContractJson {
        /// The contract path that could not be parsed.
        path: PathBuf,
        /// The parser failure.
        #[source]
        source: serde_json::Error,
    },
    /// A contract declares an immutable reference outside a registered schema family.
    #[error("contract declares an unclassified immutable reference")]
    UnclassifiedReference,
}

/// Verifies one repository-relative path and SHA-256 binding without regenerating either value.
///
/// # Errors
///
/// Returns a typed error for malformed paths or digests, missing files, path escapes, I/O failures, and digest mismatches.
pub(crate) fn verify_reference(
    root: &Path,
    path: &str,
    expected_digest: &str,
) -> Result<VerifiedReference, DigestError> {
    if Path::new(path).is_absolute() {
        return Err(DigestError::AbsolutePath {
            path: path.to_owned(),
        });
    }
    let path = RepositoryPath::parse(path).map_err(|source| DigestError::InvalidPath {
        path: path.to_owned(),
        source,
    })?;
    let expected =
        expected_digest
            .parse::<Sha256Digest>()
            .map_err(|source| DigestError::InvalidDigest {
                path: path.clone(),
                source,
            })?;
    let canonical_root = fs::canonicalize(root).map_err(|source| DigestError::Root {
        path: root.to_path_buf(),
        source,
    })?;
    let unresolved = root.join(path.as_str());
    let resolved_path = fs::canonicalize(&unresolved).map_err(|source| {
        if source.kind() == io::ErrorKind::NotFound {
            DigestError::MissingFile {
                path: path.clone(),
                source,
            }
        } else {
            DigestError::Io {
                path: path.clone(),
                source,
            }
        }
    })?;
    if !resolved_path.starts_with(&canonical_root) {
        return Err(DigestError::SymlinkEscape { path });
    }
    let metadata = fs::metadata(&resolved_path).map_err(|source| DigestError::Io {
        path: path.clone(),
        source,
    })?;
    if !metadata.is_file() {
        return Err(DigestError::NotRegularFile { path });
    }
    let actual = oxyflut_qualification::hash::hash_file(&resolved_path).map_err(|source| {
        DigestError::Io {
            path: path.clone(),
            source,
        }
    })?;
    if actual != expected {
        return Err(DigestError::DigestMismatch { path });
    }

    Ok(VerifiedReference {
        path,
        resolved_path,
    })
}

/// Verifies a JSON evidence-style object containing string `path` and `sha256` fields.
///
/// # Errors
///
/// Returns [`DigestError::IncompleteReference`] when the object does not carry both required strings, or a typed verification error otherwise.
pub(crate) fn verify_value_reference(
    root: &Path,
    reference: &Value,
) -> Result<VerifiedReference, DigestError> {
    let reference = reference
        .as_object()
        .ok_or(DigestError::IncompleteReference)?;
    verify_object_reference(root, reference)
}

/// Verifies every direct `{path, sha256}` or `{localPath, sha256}` binding in committed contract JSON files.
///
/// # Errors
///
/// Returns a typed error for a malformed contract input or any missing, escaping, or mismatched immutable reference.
pub(crate) fn validate_workspace(root: &Path) -> Result<(), DigestError> {
    let directory = root.join(CONTRACTS_DIRECTORY);
    let entries = fs::read_dir(&directory).map_err(|source| DigestError::ContractIo {
        path: directory.clone(),
        source,
    })?;
    let mut paths = entries
        .collect::<Result<Vec<_>, io::Error>>()
        .map_err(|source| DigestError::ContractIo {
            path: directory.clone(),
            source,
        })?
        .into_iter()
        .map(|entry| {
            let path = entry.path();
            let kind = entry
                .file_type()
                .map_err(|source| DigestError::ContractIo {
                    path: path.clone(),
                    source,
                })?;
            let is_json = path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(".json"));
            Ok((kind.is_file() && is_json).then_some(path))
        })
        .collect::<Result<Vec<_>, DigestError>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    paths.sort();

    for path in paths {
        let bytes = fs::read(&path).map_err(|source| DigestError::ContractIo {
            path: path.clone(),
            source,
        })?;
        let value = serde_json::from_slice(&bytes)
            .map_err(|source| DigestError::ContractJson { path, source })?;
        let _ = verify_references_in_value(root, &value)?;
    }
    Ok(())
}

/// Verifies a JSON object containing `path` or `localPath` plus `sha256` strings.
///
/// # Errors
///
/// Returns [`DigestError::IncompleteReference`] for missing fields or a typed confinement or digest error otherwise.
pub(crate) fn verify_object_reference(
    root: &Path,
    reference: &Map<String, Value>,
) -> Result<VerifiedReference, DigestError> {
    let path = reference
        .get("path")
        .or_else(|| reference.get("localPath"))
        .and_then(Value::as_str)
        .ok_or(DigestError::IncompleteReference)?;
    let digest = reference
        .get("sha256")
        .and_then(Value::as_str)
        .ok_or(DigestError::IncompleteReference)?;
    verify_reference(root, path, digest)
}

/// Verifies every immutable reference declared by the JSON value's `$schema` identity.
///
/// Returns the number of verified immutable references. Objects with a `path` or `localPath` but
/// no `sha256` key aren't references and are skipped. A reference-shaped object without a known
/// schema family fails closed instead of allowing the digest family to report success.
///
/// # Errors
///
/// Returns a typed error for an unclassified, incomplete, missing, escaping, or mismatched
/// schema-typed reference.
pub(crate) fn verify_references_in_value(root: &Path, value: &Value) -> Result<usize, DigestError> {
    let declaration = value
        .get("$schema")
        .and_then(Value::as_str)
        .map(reference_declaration)
        .unwrap_or(ReferenceDeclaration::Unknown);
    match declaration {
        ReferenceDeclaration::References => {
            let schema_identity = value
                .get("$schema")
                .and_then(Value::as_str)
                .ok_or(DigestError::UnclassifiedReference)?;
            verify_references_for_schema(root, schema_identity, value)
        }
        ReferenceDeclaration::ReferenceFree => Ok(0),
        ReferenceDeclaration::Unknown => {
            if reference_object_count(value) == 0 {
                Ok(0)
            } else {
                Err(DigestError::UnclassifiedReference)
            }
        }
    }
}

/// Verifies every immutable reference declared by one known durable schema identity.
///
/// Callers that obtain a document through a typed parent binding pass its schema identity directly,
/// because preserved child documents don't need to repeat `$schema`.
///
/// # Errors
///
/// Returns the number of verified immutable references or a typed error for an incomplete,
/// missing, escaping, or mismatched schema-typed reference.
pub(crate) fn verify_references_for_schema(
    root: &Path,
    schema_identity: &str,
    value: &Value,
) -> Result<usize, DigestError> {
    let references = declared_references(schema_identity, value)
        .map_err(|_| DigestError::IncompleteReference)?;
    for reference in &references {
        let _ = verify_reference(root, reference.path, reference.sha256)?;
    }
    Ok(references.len())
}

fn reference_object_count(value: &Value) -> usize {
    match value {
        Value::Array(values) => values.iter().map(reference_object_count).sum(),
        Value::Object(values) => {
            let is_absent_optional_reference = matches!(
                (
                    values.get("path").or_else(|| values.get("localPath")),
                    values.get("sha256")
                ),
                (Some(Value::Null), Some(Value::Null))
            );
            let is_reference_object = values.contains_key("sha256")
                && (values.contains_key("path") || values.contains_key("localPath"))
                && !is_absent_optional_reference;
            usize::from(is_reference_object)
                + values.values().map(reference_object_count).sum::<usize>()
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::path::PathBuf;

    use oxyflut_qualification::hash::hash_file;

    use serde_json::json;

    use super::{DigestError, verify_reference, verify_references_in_value};

    #[test]
    fn verifies_a_confined_streamed_digest() -> Result<(), Box<dyn Error>> {
        let root = temporary_directory("verified");
        fs::create_dir_all(root.join("evidence"))?;
        let file = root.join("evidence/proof.txt");
        fs::write(&file, b"immutable proof")?;
        let expected = hash_file(&file)?;
        let verified = verify_reference(&root, "evidence/proof.txt", &expected.to_string())?;
        assert_eq!(verified.path.as_str(), "evidence/proof.txt");
        assert_eq!(
            expected.to_string(),
            "fa9c0a5ee6a0e72bdb81dd8a330ed0c74c878f83cd5e03ffeb00a6011d2ff838"
        );
        fs::remove_dir_all(root)?;
        Ok(())
    }

    #[test]
    fn schema_typed_walker_skips_path_without_sha256_and_rejects_null_digests()
    -> Result<(), Box<dyn Error>> {
        let root = temporary_directory("reference-policy");
        fs::create_dir_all(&root)?;
        let skipped = json!({
            "$schema": "urn:oxyflut:schema:qualification-evidence:5",
            "path": "ordinary-data"
        });
        assert_eq!(verify_references_in_value(&root, &skipped)?, 0);
        let null_digest = json!({
            "$schema": "urn:oxyflut:schema:qualification-evidence:5",
            "path": "proof.txt",
            "sha256": null
        });
        assert!(matches!(
            verify_references_in_value(&root, &null_digest),
            Err(DigestError::IncompleteReference)
        ));
        let unclassified = json!({
            "path": "proof.txt",
            "sha256": "0".repeat(64)
        });
        assert!(matches!(
            verify_references_in_value(&root, &unclassified),
            Err(DigestError::UnclassifiedReference)
        ));
        let unknown_schema = json!({
            "$schema": "urn:oxyflut:schema:unknown:1",
            "path": "proof.txt",
            "sha256": "0".repeat(64)
        });
        assert!(matches!(
            verify_references_in_value(&root, &unknown_schema),
            Err(DigestError::UnclassifiedReference)
        ));
        fs::remove_dir_all(root)?;
        Ok(())
    }

    #[test]
    fn distinguishes_missing_and_mismatched_files() -> Result<(), Box<dyn Error>> {
        let root = temporary_directory("missing-mismatch");
        fs::create_dir_all(&root)?;
        let missing = verify_reference(&root, "missing.txt", &"0".repeat(64));
        assert!(matches!(missing, Err(DigestError::MissingFile { .. })));

        let file = root.join("proof.txt");
        fs::write(&file, b"different bytes")?;
        let mismatch = verify_reference(&root, "proof.txt", &"0".repeat(64));
        assert!(matches!(mismatch, Err(DigestError::DigestMismatch { .. })));
        fs::remove_dir_all(root)?;
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn rejects_absolute_and_symlink_escape_paths() -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::symlink;

        let root = temporary_directory("escape");
        let outside = temporary_directory("outside");
        fs::create_dir_all(&root)?;
        fs::create_dir_all(&outside)?;
        let outside_file = outside.join("proof.txt");
        fs::write(&outside_file, b"outside")?;
        symlink(&outside_file, root.join("escape.txt"))?;

        assert!(matches!(
            verify_reference(&root, "/absolute", &"0".repeat(64)),
            Err(DigestError::AbsolutePath { .. })
        ));
        assert!(matches!(
            verify_reference(&root, "escape.txt", &hash_file(&outside_file)?.to_string()),
            Err(DigestError::SymlinkEscape { .. })
        ));
        fs::remove_dir_all(root)?;
        fs::remove_dir_all(outside)?;
        Ok(())
    }

    fn temporary_directory(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("oxyflut-digests-{name}-{}", std::process::id()))
    }
}
