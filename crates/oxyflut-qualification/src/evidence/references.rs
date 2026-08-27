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

/// Classifies how one schema family declares immutable references.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceDeclaration {
    /// The schema family contains immutable repository references.
    References,
    /// The schema family has `path` or `sha256` fields with another meaning.
    ReferenceFree,
    /// The schema family isn't part of this reference registry.
    Unknown,
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
    if reference_declaration(schema_identity) != ReferenceDeclaration::References {
        return Ok(Vec::new());
    }

    let mut references = Vec::new();
    collect_references(value, &mut references)?;
    Ok(references)
}

/// Returns the immutable-reference policy for a durable schema family.
#[must_use]
pub fn reference_declaration(identity: &str) -> ReferenceDeclaration {
    match schema_family(identity) {
        Some(
            "accessibility-map"
            | "capability-baseline"
            | "ci-invocation"
            | "external-contract-lock"
            | "ingress-inventory"
            | "platform-contracts"
            | "qualification-evidence"
            | "qualification-lock"
            | "raw-measurement"
            | "release-evidence-bundle"
            | "selection-decision"
            | "specification-phase",
        ) => ReferenceDeclaration::References,
        Some(
            "artifact-manifest"
            | "capability-traceability"
            | "diagnostic-event"
            | "diagnostic-event-registry",
        ) => ReferenceDeclaration::ReferenceFree,
        Some(_) | None => ReferenceDeclaration::Unknown,
    }
}

fn schema_family(identity: &str) -> Option<&str> {
    if let Some(identity) = identity.strip_prefix("urn:oxyflut:schema:") {
        return identity.rsplit_once(':').map(|(family, _version)| family);
    }

    identity
        .strip_suffix(".schema.json")
        .and_then(|path| path.rsplit('/').next())
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

#[cfg(test)]
mod tests {
    use super::{ReferenceDeclaration, reference_declaration};

    #[test]
    fn reference_policy_uses_schema_families_not_exact_versions() {
        assert_eq!(
            reference_declaration("urn:oxyflut:schema:external-contract-lock:2"),
            ReferenceDeclaration::References
        );
        assert_eq!(
            reference_declaration("../data-models/artifact-manifest.schema.json"),
            ReferenceDeclaration::ReferenceFree
        );
        assert_eq!(
            reference_declaration("urn:oxyflut:schema:unknown:1"),
            ReferenceDeclaration::Unknown
        );
    }
}
