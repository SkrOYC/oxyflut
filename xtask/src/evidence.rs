//! Repository-root resolution and JSON validation for immutable evidence files.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use oxyflut_qualification::evidence::{
    EvidenceError as CoreEvidenceError, EvidenceRef, MediaType, verify_file, verify_path_digest,
    verify_reference,
};
use oxyflut_qualification::hash::Sha256Digest;
use oxyflut_qualification::identifiers::RepositoryPath;
use oxyflut_qualification::schema::{SchemaError, SchemaRegistry};
use serde_json::{Map, Value};
use thiserror::Error;

const DATA_MODELS_DIRECTORY: &str = ".constitution/tech-spec/data-models";
const SCHEMA_SNAPSHOTS_DIRECTORY: &str = "qualification/schemas";

/// Reports an evidence-command adapter failure without exposing evidence content.
#[derive(Debug, Error)]
pub(crate) enum EvidenceAdapterError {
    #[error("evidence input path is invalid")]
    InputPath,
    #[error("evidence media type is unsupported")]
    UnsupportedMediaType,
    #[error("local evidence verification failed")]
    Evidence(#[source] CoreEvidenceError),
    #[error("local schema registry failed")]
    SchemaRegistry(#[source] SchemaError),
    #[error("evidence schema declaration is invalid")]
    SchemaDeclaration,
    #[error("evidence schema validation failed")]
    SchemaValidation(#[source] SchemaError),
    #[error("could not resolve a local evidence schema")]
    SchemaIo(#[source] io::Error),
}

/// Resolves the workspace root from the compiled xtask location.
///
/// # Errors
///
/// Returns an error only if xtask is no longer directly below the workspace root.
pub(crate) fn repository_root() -> Result<PathBuf, EvidenceAdapterError> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or(EvidenceAdapterError::InputPath)
}

/// Verifies one repository-relative evidence path, including schema and digest bindings.
///
/// # Errors
///
/// Returns an error when the input path, media type, canonical derived JSON, local schema, or a
/// declared evidence digest is invalid.
pub(crate) fn verify(root: &Path, input: &str) -> Result<(), EvidenceAdapterError> {
    let path = RepositoryPath::parse(input).map_err(|_| EvidenceAdapterError::InputPath)?;
    let media_type = media_type_for(&path)?;
    let verified = verify_file(root, &path, &media_type).map_err(EvidenceAdapterError::Evidence)?;
    let Some(value) = verified.json() else {
        return Ok(());
    };

    validate_declared_schema(root, &path, value)?;
    verify_declared_digests(root, value)?;
    Ok(())
}

fn media_type_for(path: &RepositoryPath) -> Result<MediaType, EvidenceAdapterError> {
    if path.as_str().ends_with(".json") {
        Ok(MediaType::application_json())
    } else {
        Err(EvidenceAdapterError::UnsupportedMediaType)
    }
}

fn validate_declared_schema(
    root: &Path,
    instance_path: &RepositoryPath,
    value: &Value,
) -> Result<(), EvidenceAdapterError> {
    let Some(reference) = value.get("$schema").and_then(Value::as_str) else {
        return Ok(());
    };
    let registry = SchemaRegistry::from_directories(&[
        root.join(DATA_MODELS_DIRECTORY),
        root.join(SCHEMA_SNAPSHOTS_DIRECTORY),
    ])
    .map_err(EvidenceAdapterError::SchemaRegistry)?;
    let identity = resolve_schema_reference(root, instance_path, reference, &registry)?;
    registry
        .validate(&identity, value)
        .map_err(EvidenceAdapterError::SchemaValidation)
}

fn resolve_schema_reference(
    root: &Path,
    instance_path: &RepositoryPath,
    reference: &str,
    registry: &SchemaRegistry,
) -> Result<String, EvidenceAdapterError> {
    if reference.starts_with("http:")
        || reference.starts_with("https:")
        || reference.starts_with("file:")
        || reference.contains('\\')
    {
        return Err(EvidenceAdapterError::SchemaDeclaration);
    }
    if reference.starts_with("urn:") {
        return registry
            .require_current_identity(reference)
            .map(str::to_owned)
            .map_err(EvidenceAdapterError::SchemaValidation);
    }

    let instance_parent = Path::new(instance_path.as_str())
        .parent()
        .ok_or(EvidenceAdapterError::SchemaDeclaration)?;
    let declared_path = root.join(instance_parent).join(reference);
    let schema_path = fs::canonicalize(&declared_path).map_err(EvidenceAdapterError::SchemaIo)?;
    let schema_roots = [
        root.join(DATA_MODELS_DIRECTORY),
        root.join(SCHEMA_SNAPSHOTS_DIRECTORY),
    ]
    .into_iter()
    .map(fs::canonicalize)
    .collect::<Result<Vec<_>, io::Error>>()
    .map_err(EvidenceAdapterError::SchemaIo)?;
    if !schema_roots
        .iter()
        .any(|schema_root| schema_path.starts_with(schema_root))
    {
        return Err(EvidenceAdapterError::SchemaDeclaration);
    }
    registry
        .identity_for_path(&schema_path)
        .map(str::to_owned)
        .map_err(EvidenceAdapterError::SchemaValidation)
}

fn verify_declared_digests(root: &Path, value: &Value) -> Result<(), EvidenceAdapterError> {
    match value {
        Value::Array(values) => {
            for item in values {
                verify_declared_digests(root, item)?;
            }
        }
        Value::Object(object) => {
            if is_evidence_reference_shape(object) {
                verify_evidence_reference(root, object)?;
            }
            for item in object.values() {
                verify_declared_digests(root, item)?;
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
    Ok(())
}

fn is_evidence_reference_shape(object: &Map<String, Value>) -> bool {
    object.contains_key("path")
        && object.contains_key("sha256")
        && object
            .keys()
            .all(|key| matches!(key.as_str(), "path" | "sha256" | "mediaType" | "sizeBytes"))
}

fn verify_evidence_reference(
    root: &Path,
    object: &Map<String, Value>,
) -> Result<(), EvidenceAdapterError> {
    let path = object
        .get("path")
        .ok_or(EvidenceAdapterError::SchemaDeclaration)?;
    let digest = object
        .get("sha256")
        .ok_or(EvidenceAdapterError::SchemaDeclaration)?;
    if path.is_null() && digest.is_null() {
        return Ok(());
    }
    let path = path
        .as_str()
        .ok_or(EvidenceAdapterError::SchemaDeclaration)
        .and_then(|path| {
            RepositoryPath::parse(path).map_err(|_| EvidenceAdapterError::InputPath)
        })?;
    let digest = digest
        .as_str()
        .ok_or(EvidenceAdapterError::SchemaDeclaration)
        .and_then(|digest| {
            digest
                .parse::<Sha256Digest>()
                .map_err(|_| EvidenceAdapterError::SchemaDeclaration)
        })?;

    match (object.get("mediaType"), object.get("sizeBytes")) {
        (None, None) => {
            let _ =
                verify_path_digest(root, &path, &digest).map_err(EvidenceAdapterError::Evidence)?;
        }
        (Some(Value::String(media_type)), Some(Value::Number(size_bytes))) => {
            let media_type =
                MediaType::parse(media_type).map_err(EvidenceAdapterError::Evidence)?;
            let size_bytes = size_bytes
                .as_u64()
                .ok_or(EvidenceAdapterError::SchemaDeclaration)?;
            let reference = EvidenceRef {
                path,
                sha256: digest,
                media_type,
                size_bytes,
            };
            let _ = verify_reference(root, &reference).map_err(EvidenceAdapterError::Evidence)?;
        }
        _ => return Err(EvidenceAdapterError::SchemaDeclaration),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::{EvidenceAdapterError, repository_root, verify};

    #[test]
    fn verifies_positive_derived_evidence_and_rejects_bad_references() -> Result<(), Box<dyn Error>>
    {
        let root = repository_root()?;
        verify(
            &root,
            "qualification/fixtures/evidence/positive-derived.json",
        )?;
        verify(&root, "qualification/fixtures/evidence/schema-valid.json")?;
        let bad = verify(&root, "qualification/fixtures/evidence/bad-digest.json");
        assert!(matches!(bad, Err(EvidenceAdapterError::Evidence(_))));
        let out_of_root = verify(&root, "qualification/fixtures/evidence/out-of-root.json");
        assert!(matches!(
            out_of_root,
            Err(EvidenceAdapterError::Evidence(_))
        ));
        Ok(())
    }

    #[test]
    fn rejects_absolute_and_unsupported_evidence_inputs() -> Result<(), Box<dyn Error>> {
        let root = repository_root()?;
        assert!(matches!(
            verify(&root, "/absolute.json"),
            Err(EvidenceAdapterError::InputPath)
        ));
        assert!(matches!(
            verify(
                &root,
                "qualification/fixtures/evidence/preserved-source.txt"
            ),
            Err(EvidenceAdapterError::UnsupportedMediaType)
        ));
        Ok(())
    }
}
