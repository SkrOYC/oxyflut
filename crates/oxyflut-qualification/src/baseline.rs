//! Candidate-neutral capability-baseline parsing and structural validation.
//!
//! This module validates the durable baseline fields that are independent of the repository's active lock. The `xtask` command supplies the exact capability authority and reuses the traceability validator for approved-provenance evidence bindings.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::de::{Error as _, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::identifiers::{CapabilityId, SpecificationVersion};

const CAPABILITY_COUNT: usize = 52;
const SCHEMA_VERSION: &str = "4.0.0";
const FLOWS_DIRECTORY: &str = ".constitution/architecture/flows";

/// Supplies the exact active capability set used to validate one baseline.
#[derive(Clone, Debug)]
pub struct BaselineAuthority {
    specification_version: SpecificationVersion,
    capabilities: BTreeSet<CapabilityId>,
}

impl BaselineAuthority {
    /// Creates an authority from the active specification version and the exact P0 capability set.
    ///
    /// # Errors
    ///
    /// Returns [`BaselineError::AuthorityCapabilityCount`] unless the caller supplies all 52 capability identifiers.
    pub fn new(
        specification_version: SpecificationVersion,
        capabilities: BTreeSet<CapabilityId>,
    ) -> Result<Self, BaselineError> {
        if capabilities.len() != CAPABILITY_COUNT {
            return Err(BaselineError::AuthorityCapabilityCount);
        }
        Ok(Self {
            specification_version,
            capabilities,
        })
    }
}

/// A parsed capability baseline with duplicate capability keys rejected.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CapabilityBaseline {
    schema_version: String,
    specification_version: String,
    provenance: Provenance,
    capabilities: CapabilityEntries,
}

impl CapabilityBaseline {
    /// Parses a baseline JSON document while rejecting duplicate capability keys.
    ///
    /// # Errors
    ///
    /// Returns [`BaselineError::Json`] when the bytes do not encode the baseline structure or repeat a capability key.
    pub fn parse_json(bytes: &[u8]) -> Result<Self, BaselineError> {
        serde_json::from_slice(bytes).map_err(BaselineError::Json)
    }

    /// Validates the active specification, exact capabilities, architecture-flow bindings, vectors, evidence expectations, and provenance shape.
    ///
    /// Approved approval-evidence digest resolution is intentionally delegated to the shared traceability validator so that baseline and qualification-lock validation use the same immutable-reference rules.
    ///
    /// # Errors
    ///
    /// Returns a typed error when the baseline does not match the supplied authority or violates a candidate-neutral baseline invariant.
    pub fn validate_structure(
        &self,
        root: &Path,
        authority: &BaselineAuthority,
    ) -> Result<(), BaselineError> {
        if self.schema_version != SCHEMA_VERSION {
            return Err(BaselineError::SchemaVersion);
        }
        if self.specification_version != authority.specification_version.as_str() {
            return Err(BaselineError::SpecificationVersion);
        }
        validate_provenance(&self.provenance)?;

        let capabilities = self
            .capabilities
            .0
            .keys()
            .map(|identifier| {
                CapabilityId::parse(identifier).map_err(|_| BaselineError::CapabilityIdentifier)
            })
            .collect::<Result<BTreeSet<_>, _>>()?;
        if capabilities != authority.capabilities {
            return Err(BaselineError::CapabilitySet);
        }

        for (identifier, entry) in &self.capabilities.0 {
            let capability =
                CapabilityId::parse(identifier).map_err(|_| BaselineError::CapabilityIdentifier)?;
            let expected_flow = format!(
                "{FLOWS_DIRECTORY}/{}.md",
                capability.as_str().to_ascii_lowercase()
            );
            if entry.flow != expected_flow || !root.join(&entry.flow).is_file() {
                return Err(BaselineError::ArchitectureFlow);
            }
            validate_unique_nonempty(&entry.test_vectors, BaselineError::TestVectors)?;
            validate_unique_nonempty(&entry.expected_evidence, BaselineError::ExpectedEvidence)?;
        }

        Ok(())
    }

    /// Returns the canonicalizable JSON value reconstructed from the duplicate-checked baseline.
    ///
    /// # Errors
    ///
    /// Returns [`BaselineError::Serialization`] only if serialization of the parsed baseline fails.
    pub fn canonical_value(&self) -> Result<Value, BaselineError> {
        serde_json::to_value(self).map_err(BaselineError::Serialization)
    }
}

/// Reports why a candidate-neutral capability baseline is invalid.
#[derive(Debug, Error)]
pub enum BaselineError {
    /// The JSON bytes were malformed, had an invalid baseline shape, or repeated a capability key.
    #[error("capability baseline JSON is invalid")]
    Json(#[source] serde_json::Error),
    /// The parsed baseline could not be converted into a JSON value for canonical output.
    #[error("capability baseline could not be serialized")]
    Serialization(#[source] serde_json::Error),
    /// The supplied authoritative set did not contain the required 52 capabilities.
    #[error("capability baseline authority does not contain exactly 52 capabilities")]
    AuthorityCapabilityCount,
    /// The baseline did not declare schema version 4.0.0.
    #[error("capability baseline schema version is invalid")]
    SchemaVersion,
    /// The baseline specification version differed from the active specification version.
    #[error("capability baseline specification version does not match the active specification")]
    SpecificationVersion,
    /// A capability key did not use the closed capability identifier syntax.
    #[error("capability baseline contains an invalid capability identifier")]
    CapabilityIdentifier,
    /// The baseline capability keys differed from the exact authoritative 52-key set.
    #[error("capability baseline capability keys do not match the authoritative set")]
    CapabilitySet,
    /// A capability entry did not bind to its existing architecture flow.
    #[error("capability baseline architecture flow binding is invalid")]
    ArchitectureFlow,
    /// A capability entry did not contain a unique nonempty test-vector list.
    #[error("capability baseline test vectors are invalid")]
    TestVectors,
    /// A capability entry did not contain a unique nonempty expected-evidence list.
    #[error("capability baseline expected evidence is invalid")]
    ExpectedEvidence,
    /// Synthetic provenance carried approval evidence or approved provenance omitted it.
    #[error("capability baseline provenance is invalid")]
    Provenance,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Provenance {
    kind: ProvenanceKind,
    approval_evidence: Option<ApprovalEvidence>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum ProvenanceKind {
    Synthetic,
    Approved,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ApprovalEvidence {
    path: String,
    sha256: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct CapabilityEntry {
    flow: String,
    test_vectors: Vec<String>,
    expected_evidence: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
struct CapabilityEntries(BTreeMap<String, CapabilityEntry>);

impl<'de> Deserialize<'de> for CapabilityEntries {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(CapabilityEntriesVisitor)
    }
}

struct CapabilityEntriesVisitor;

impl<'de> Visitor<'de> for CapabilityEntriesVisitor {
    type Value = CapabilityEntries;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a map of unique capability baseline entries")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut entries = BTreeMap::new();
        while let Some((identifier, entry)) = map.next_entry::<String, CapabilityEntry>()? {
            if entries.insert(identifier.clone(), entry).is_some() {
                return Err(M::Error::custom("duplicate capability baseline key"));
            }
        }
        Ok(CapabilityEntries(entries))
    }
}

fn validate_provenance(provenance: &Provenance) -> Result<(), BaselineError> {
    match (provenance.kind, provenance.approval_evidence.as_ref()) {
        (ProvenanceKind::Synthetic, None) => Ok(()),
        (ProvenanceKind::Approved, Some(evidence))
            if !evidence.path.is_empty() && !evidence.sha256.is_empty() =>
        {
            Ok(())
        }
        (ProvenanceKind::Synthetic, Some(_)) | (ProvenanceKind::Approved, None | Some(_)) => {
            Err(BaselineError::Provenance)
        }
    }
}

fn validate_unique_nonempty(values: &[String], error: BaselineError) -> Result<(), BaselineError> {
    if values.is_empty() || values.iter().any(String::is_empty) {
        return Err(error);
    }
    let unique = values.iter().collect::<BTreeSet<_>>();
    if unique.len() != values.len() {
        return Err(error);
    }
    Ok(())
}
