//! Schema-typed discovery of immutable repository references.

use serde_json::Value;
use thiserror::Error;

/// A borrowed repository path and SHA-256 binding declared by a durable schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclaredEvidenceReference<'value> {
    /// The declared repository-relative path.
    pub path: &'value str,
    /// The declared lowercase SHA-256 digest.
    pub sha256: &'value str,
    /// The containing object, used to inspect schema-defined optional metadata.
    pub object: &'value serde_json::Map<String, Value>,
}

/// Reports an invalid schema-typed immutable reference shape.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum DeclaredReferenceError {
    /// A declared reference contains a `sha256` key but does not provide paired string path and digest values.
    #[error("declared immutable reference is incomplete")]
    IncompleteReference,
}

/// Collects immutable repository references for one declared durable schema identity.
///
/// Documents outside the known reference-bearing schema identities produce no references. A
/// `path` or `localPath` without a `sha256` key isn't a reference and is skipped. Once a
/// `sha256` key is present, both values must be strings, except that a paired null path and
/// digest represents an absent optional reference.
///
/// # Errors
///
/// Returns [`DeclaredReferenceError::IncompleteReference`] when a declared digest is null or
/// otherwise mismatched with a path value.
pub fn declared_references<'value>(
    schema_identity: &str,
    value: &'value Value,
) -> Result<Vec<DeclaredEvidenceReference<'value>>, DeclaredReferenceError> {
    if !schema_declares_references(schema_identity) {
        return Ok(Vec::new());
    }

    let mut references = Vec::new();
    collect_references(value, &mut references)?;
    Ok(references)
}

fn schema_declares_references(identity: &str) -> bool {
    matches!(
        identity,
        "urn:oxyflut:schema:accessibility-map:5"
            | "urn:oxyflut:schema:capability-baseline:4"
            | "urn:oxyflut:schema:ci-invocation:1"
            | "urn:oxyflut:schema:external-contract-lock:1"
            | "urn:oxyflut:schema:ingress-inventory:2"
            | "urn:oxyflut:schema:platform-contracts:5"
            | "urn:oxyflut:schema:qualification-evidence:5"
            | "urn:oxyflut:schema:qualification-lock:5"
            | "urn:oxyflut:schema:raw-measurement:2"
            | "urn:oxyflut:schema:release-evidence-bundle:1"
            | "urn:oxyflut:schema:selection-decision:1"
            | "urn:oxyflut:schema:specification-phase:1"
    ) || [
        "accessibility-map.schema.json",
        "capability-baseline.schema.json",
        "ci-invocation.schema.json",
        "external-contract-lock.schema.json",
        "ingress-inventory.schema.json",
        "platform-contracts.schema.json",
        "qualification-evidence.schema.json",
        "qualification-lock.schema.json",
        "raw-measurement.schema.json",
        "release-evidence-bundle.schema.json",
        "selection-decision.schema.json",
        "specification-phase.schema.json",
    ]
    .iter()
    .any(|name| identity.ends_with(name))
}

fn collect_references<'value>(
    value: &'value Value,
    references: &mut Vec<DeclaredEvidenceReference<'value>>,
) -> Result<(), DeclaredReferenceError> {
    match value {
        Value::Array(values) => {
            for value in values {
                collect_references(value, references)?;
            }
        }
        Value::Object(values) => {
            let path = values.get("path").or_else(|| values.get("localPath"));
            if let (Some(path), Some(digest)) = (path, values.get("sha256")) {
                match (path, digest) {
                    (Value::String(path), Value::String(sha256)) => {
                        references.push(DeclaredEvidenceReference {
                            path,
                            sha256,
                            object: values,
                        });
                    }
                    (Value::Null, Value::Null) => {}
                    (Value::Null, _)
                    | (_, Value::Null)
                    | (Value::Bool(_) | Value::Number(_) | Value::Array(_) | Value::Object(_), _)
                    | (_, Value::Bool(_) | Value::Number(_) | Value::Array(_) | Value::Object(_)) =>
                    {
                        return Err(DeclaredReferenceError::IncompleteReference);
                    }
                }
            }
            for value in values.values() {
                collect_references(value, references)?;
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
    Ok(())
}
