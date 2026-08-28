//! Offline verification for snapshotted external distribution contracts.
//!
//! The SPDX JSON Schema snapshot has no immutable upstream commit pin because SPDX publishes it
//! from `spdx.org`, not from a commit-addressed repository path. Its metadata binds the published
//! URL and digest to the pinned SPDX serialization document that publishes that URL.

use std::fs;
use std::io;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use oxyflut_qualification::hash::{Sha256Digest, hash_file};
use oxyflut_qualification::schema::{SchemaError, SchemaRegistry};
use serde_json::Value;
use thiserror::Error;

use super::super::{CommandError, CommandOutcome};

const FIXTURES_DIRECTORY: &str = "qualification/fixtures/external-contracts";
const DATA_MODELS_DIRECTORY: &str = ".constitution/tech-spec/data-models";
const PROPOSAL_PATH: &str = "qualification/schemas/external/proposed-external-contract-lock.json";
const STATEMENT_SCHEMA: &str = "urn:oxyflut:schema:external-in-toto-statement-v1:1";
const PROVENANCE_SCHEMA: &str = "urn:oxyflut:schema:external-slsa-provenance-v1:1";
const DSSE_SCHEMA: &str = "urn:oxyflut:schema:external-dsse-envelope-v1:1";
const EXTERNAL_LOCK_SCHEMA: &str = "urn:oxyflut:schema:external-contract-lock:1";
#[path = "external_contracts/dsse.rs"]
mod dsse;

use dsse::{
    PAYLOAD_TYPE as DSSE_PAYLOAD_TYPE, TEST_ALGORITHM as DSSE_TEST_ALGORITHM,
    TEST_KEY_ID as DSSE_TEST_KEY_ID, TestKey, decode_base64, pae as dsse_pae,
    verify_fixture_signature,
};
const SPDX_CONTEXT_PATH: &str =
    "qualification/schemas/external/spdx-3.0.1/jsonld-context/spdx-context.jsonld";
const SPDX_CONTEXT_URI: &str = "https://spdx.org/rdf/3.0.1/spdx-context.jsonld";
const SPDX_DOCUMENT_TERM: &str = "https://spdx.org/rdf/3.0.1/terms/Core/SpdxDocument";
const SPDX_SCHEMA_PATH: &str =
    "qualification/schemas/external/spdx-3.0.1/schema/spdx-json-schema.json";
const SPDX_SCHEMA_URL: &str = "https://spdx.org/schema/3.0.1/spdx-json-schema.json";
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Verifies every immutable external-contract snapshot and local fixture without network access.
pub(crate) fn run(arguments: &[String]) -> CommandOutcome {
    if !arguments.is_empty() {
        return CommandOutcome::failed(CommandError::InvalidInput {
            code: "external-contracts-verify-arguments",
        });
    }

    let root = match workspace_root() {
        Ok(root) => root,
        Err(()) => {
            return CommandOutcome::failed(CommandError::Execution {
                code: "workspace-root",
                hint: "rerun: external-contracts verify",
            });
        }
    };

    outcome_at(&root)
}

fn outcome_at(root: &Path) -> CommandOutcome {
    match verify_at(root) {
        Ok(()) => {
            for family in [
                "spdx-3.0.1",
                "in-toto-statement-v1",
                "slsa-provenance-v1",
                "dsse-envelope-v1",
            ] {
                println!("external-contracts: ok ({family})");
            }
            CommandOutcome::Success
        }
        Err(_) => CommandOutcome::failed(CommandError::ValidationFailed {
            code: "external-contracts-invalid",
            hint: "rerun: external-contracts verify",
        }),
    }
}

/// Runs the complete offline verifier at a supplied repository root.
fn verify_at(root: &Path) -> Result<(), ExternalContractsError> {
    verify_snapshots(root)?;
    verify_proposal(root)?;
    let registry = DerivedSchemaRegistry::load(root)?;
    verify_fixtures(root, registry.registry(), registry.spdx_schema_identity())
}

fn verify_snapshots(root: &Path) -> Result<(), ExternalContractsError> {
    for snapshot in SNAPSHOTS {
        let artifact = root.join(snapshot.artifact_path);
        let metadata = root.join(snapshot.metadata_path);
        let actual = hash_file(&artifact).map_err(|source| ExternalContractsError::Io {
            path: artifact.clone(),
            source,
        })?;
        let expected = snapshot.sha256.parse::<Sha256Digest>().map_err(|_| {
            ExternalContractsError::Snapshot {
                path: artifact.clone(),
            }
        })?;
        if actual != expected {
            return Err(ExternalContractsError::Snapshot { path: artifact });
        }

        let record = read_json(&metadata)?;
        let object = record
            .as_object()
            .ok_or(ExternalContractsError::SourceIdentity {
                path: metadata.clone(),
            })?;
        require_string(object, "sha256", &expected.to_string(), &metadata)?;
        match &snapshot.identity {
            SnapshotIdentity::Authoritative(identity) => {
                verify_retrieval_source_consistency(object, &metadata)?;
                require_string(object, "kind", "authoritative", &metadata)?;
                require_string(object, "repository", identity.repository, &metadata)?;
                require_string(object, "commit", identity.commit, &metadata)?;
                require_string(object, "path", identity.path, &metadata)?;
                require_string(object, "retrievalUrl", identity.retrieval_url, &metadata)?;
                require_string(object, "license", identity.license, &metadata)?;
                require_license_source(object, identity, &metadata)?;
                require_string(object, "version", identity.version, &metadata)?;
            }
            SnapshotIdentity::Published(identity) => {
                verify_retrieval_source_consistency(object, &metadata)?;
                require_string(object, "kind", "authoritative", &metadata)?;
                require_absent(object, "repository", &metadata)?;
                require_absent(object, "commit", &metadata)?;
                require_absent(object, "path", &metadata)?;
                require_string(object, "retrievalUrl", identity.retrieval_url, &metadata)?;
                require_publication_source(
                    root,
                    object,
                    identity.publication_source,
                    identity.retrieval_url,
                    &metadata,
                )?;
                require_string(object, "license", identity.license, &metadata)?;
                require_license_fields(
                    object,
                    identity.license_source_path,
                    identity.license_source_commit,
                    &metadata,
                )?;
                require_string(object, "version", identity.version, &metadata)?;
            }
            SnapshotIdentity::Derived(identity) => {
                require_string(object, "kind", "derived", &metadata)?;
                require_string(object, "verifier", identity.verifier, &metadata)?;
                let source = object.get("derivedFrom").and_then(Value::as_object).ok_or(
                    ExternalContractsError::SourceIdentity {
                        path: metadata.clone(),
                    },
                )?;
                require_string(source, "localPath", identity.local_path, &metadata)?;
                require_string(source, "sha256", identity.sha256, &metadata)?;
                if object
                    .get("derivation")
                    .and_then(Value::as_str)
                    .is_none_or(str::is_empty)
                {
                    return Err(ExternalContractsError::SourceIdentity { path: metadata });
                }
            }
        }
    }
    Ok(())
}

fn require_string(
    object: &serde_json::Map<String, Value>,
    key: &str,
    expected: &str,
    path: &Path,
) -> Result<(), ExternalContractsError> {
    if object.get(key).and_then(Value::as_str) == Some(expected) {
        Ok(())
    } else {
        Err(ExternalContractsError::SourceIdentity {
            path: path.to_path_buf(),
        })
    }
}

fn require_absent(
    object: &serde_json::Map<String, Value>,
    key: &str,
    path: &Path,
) -> Result<(), ExternalContractsError> {
    if object.contains_key(key) {
        Err(ExternalContractsError::SourceIdentity {
            path: path.to_path_buf(),
        })
    } else {
        Ok(())
    }
}

fn verify_retrieval_source_consistency(
    object: &serde_json::Map<String, Value>,
    metadata_path: &Path,
) -> Result<(), ExternalContractsError> {
    let repository = object.get("repository").and_then(Value::as_str);
    let commit = object.get("commit").and_then(Value::as_str);
    let source_path = object.get("path").and_then(Value::as_str);
    match (repository, commit, source_path) {
        (Some(repository), Some(commit), Some(source_path)) => {
            let repository = repository
                .strip_prefix("https://github.com/")
                .filter(|value| !value.is_empty())
                .ok_or(ExternalContractsError::SourceIdentity {
                    path: metadata_path.to_path_buf(),
                })?;
            let expected =
                format!("https://raw.githubusercontent.com/{repository}/{commit}/{source_path}");
            require_string(object, "retrievalUrl", &expected, metadata_path)
        }
        (None, None, None) => {
            let publication = object
                .get("publicationSource")
                .and_then(Value::as_object)
                .ok_or(ExternalContractsError::SourceIdentity {
                    path: metadata_path.to_path_buf(),
                })?;
            for field in ["localPath", "sha256", "path", "commit"] {
                if publication
                    .get(field)
                    .and_then(Value::as_str)
                    .is_none_or(str::is_empty)
                {
                    return Err(ExternalContractsError::SourceIdentity {
                        path: metadata_path.to_path_buf(),
                    });
                }
            }
            Ok(())
        }
        _ => Err(ExternalContractsError::SourceIdentity {
            path: metadata_path.to_path_buf(),
        }),
    }
}

fn require_license_source(
    object: &serde_json::Map<String, Value>,
    identity: &AuthoritativeIdentity,
    metadata_path: &Path,
) -> Result<(), ExternalContractsError> {
    let license_path = if identity.repository == "https://github.com/slsa-framework/slsa" {
        "LICENSE.md"
    } else {
        "LICENSE"
    };
    require_license_fields(object, license_path, identity.commit, metadata_path)
}

fn require_license_fields(
    object: &serde_json::Map<String, Value>,
    license_path: &str,
    license_commit: &str,
    metadata_path: &Path,
) -> Result<(), ExternalContractsError> {
    let license_source = object
        .get("licenseSource")
        .and_then(Value::as_object)
        .ok_or(ExternalContractsError::SourceIdentity {
            path: metadata_path.to_path_buf(),
        })?;
    require_string(license_source, "path", license_path, metadata_path)?;
    require_string(license_source, "commit", license_commit, metadata_path)
}

fn require_publication_source(
    root: &Path,
    object: &serde_json::Map<String, Value>,
    expected: &PublicationSource,
    retrieval_url: &str,
    metadata_path: &Path,
) -> Result<(), ExternalContractsError> {
    let publication = object
        .get("publicationSource")
        .and_then(Value::as_object)
        .ok_or(ExternalContractsError::SourceIdentity {
            path: metadata_path.to_path_buf(),
        })?;
    require_string(publication, "localPath", expected.local_path, metadata_path)?;
    require_string(publication, "sha256", expected.sha256, metadata_path)?;
    require_string(publication, "path", expected.path, metadata_path)?;
    require_string(publication, "commit", expected.commit, metadata_path)?;
    verify_publication_source_bytes(
        &root.join(expected.local_path),
        expected.sha256,
        retrieval_url,
        metadata_path,
    )
}

fn verify_publication_source_bytes(
    source_path: &Path,
    expected_sha256: &str,
    retrieval_url: &str,
    metadata_path: &Path,
) -> Result<(), ExternalContractsError> {
    let expected = expected_sha256.parse::<Sha256Digest>().map_err(|_| {
        ExternalContractsError::SourceIdentity {
            path: metadata_path.to_path_buf(),
        }
    })?;
    let actual = hash_file(source_path).map_err(|source| ExternalContractsError::Io {
        path: source_path.to_path_buf(),
        source,
    })?;
    if actual != expected {
        return Err(ExternalContractsError::SourceIdentity {
            path: metadata_path.to_path_buf(),
        });
    }
    let bytes = fs::read(source_path).map_err(|source| ExternalContractsError::Io {
        path: source_path.to_path_buf(),
        source,
    })?;
    if bytes
        .windows(retrieval_url.len())
        .any(|window| window == retrieval_url.as_bytes())
    {
        Ok(())
    } else {
        Err(ExternalContractsError::SourceIdentity {
            path: metadata_path.to_path_buf(),
        })
    }
}

fn verify_proposal(root: &Path) -> Result<(), ExternalContractsError> {
    let proposal_path = root.join(PROPOSAL_PATH);
    let proposal = read_json(&proposal_path)?;
    let schema_registry = SchemaRegistry::from_directories(&[root.join(DATA_MODELS_DIRECTORY)])
        .map_err(ExternalContractsError::ProposalSchema)?;
    schema_registry
        .validate(EXTERNAL_LOCK_SCHEMA, &proposal)
        .map_err(|source| proposal_schema_error(&proposal, source, &proposal_path))?;

    let contracts = proposal.get("contracts").and_then(Value::as_object).ok_or(
        ExternalContractsError::Proposal {
            path: proposal_path.clone(),
            code: ProposalCode::ContractIdentityMismatch,
        },
    )?;
    for contract in PROPOSED_CONTRACTS {
        let Some(value) = contracts.get(contract.name).and_then(Value::as_object) else {
            return Err(ExternalContractsError::Proposal {
                path: proposal_path.clone(),
                code: ProposalCode::ContractIdentityMismatch,
            });
        };
        require_proposal_string(
            value,
            "version",
            contract.version,
            ProposalCode::ContractIdentityMismatch,
            &proposal_path,
        )?;
        require_proposal_string(
            value,
            "source",
            contract.source,
            ProposalCode::ContractIdentityMismatch,
            &proposal_path,
        )?;
        require_proposal_string(
            value,
            "epistemicStatus",
            "kk-locked",
            ProposalCode::ContractIdentityMismatch,
            &proposal_path,
        )?;
        require_proposal_string(
            value,
            "localPath",
            contract.local_path,
            ProposalCode::ContractIdentityMismatch,
            &proposal_path,
        )?;
        require_proposal_string(
            value,
            "sha256",
            contract.sha256,
            ProposalCode::RegistryDigestMismatch,
            &proposal_path,
        )?;
        require_proposal_string(
            value,
            "verifier",
            contract.verifier,
            ProposalCode::ContractIdentityMismatch,
            &proposal_path,
        )?;
    }
    if contracts.len() != PROPOSED_CONTRACTS.len() {
        return Err(ExternalContractsError::Proposal {
            path: proposal_path,
            code: ProposalCode::ContractIdentityMismatch,
        });
    }
    Ok(())
}

fn proposal_schema_error(
    proposal: &Value,
    source: SchemaError,
    proposal_path: &Path,
) -> ExternalContractsError {
    let missing_dsse_envelope = proposal
        .get("contracts")
        .and_then(Value::as_object)
        .is_some_and(|contracts| !contracts.contains_key("dsse-envelope-v1"));
    if missing_dsse_envelope {
        ExternalContractsError::Proposal {
            path: proposal_path.to_path_buf(),
            code: ProposalCode::MissingDsseEnvelopeEntry,
        }
    } else {
        ExternalContractsError::ProposalSchema(source)
    }
}

fn require_proposal_string(
    object: &serde_json::Map<String, Value>,
    key: &str,
    expected: &str,
    code: ProposalCode,
    proposal_path: &Path,
) -> Result<(), ExternalContractsError> {
    require_string(object, key, expected, proposal_path).map_err(|_| {
        ExternalContractsError::Proposal {
            path: proposal_path.to_path_buf(),
            code,
        }
    })
}

fn verify_fixtures(
    root: &Path,
    registry: &SchemaRegistry,
    spdx_schema_identity: &str,
) -> Result<(), ExternalContractsError> {
    let fixtures = root.join(FIXTURES_DIRECTORY).join("positive");
    let statement = read_json(&fixtures.join("statement.json"))?;
    validate_schema(registry, STATEMENT_SCHEMA, &statement, &fixtures)?;

    let provenance = read_json(&fixtures.join("provenance.json"))?;
    validate_schema(registry, PROVENANCE_SCHEMA, &provenance, &fixtures)?;

    let spdx = read_json(&fixtures.join("spdx-document.json"))?;
    verify_spdx_document(root, registry, spdx_schema_identity, &spdx, &fixtures)?;

    let envelope = read_json(&fixtures.join("envelope.json"))?;
    validate_schema(registry, DSSE_SCHEMA, &envelope, &fixtures)?;
    verify_dsse_envelope(root, registry, &envelope, &fixtures)
}

fn verify_spdx_document(
    root: &Path,
    registry: &SchemaRegistry,
    schema_identity: &str,
    document: &Value,
    fixture_root: &Path,
) -> Result<(), ExternalContractsError> {
    verify_spdx_context(root, document, fixture_root)?;
    registry
        .validate(schema_identity, document)
        .map_err(|_| ExternalContractsError::SpdxSchema {
            path: fixture_root.to_path_buf(),
        })
}

fn verify_spdx_context(
    root: &Path,
    document: &Value,
    fixture_root: &Path,
) -> Result<(), ExternalContractsError> {
    if document.get("@context").and_then(Value::as_str) != Some(SPDX_CONTEXT_URI) {
        return Err(ExternalContractsError::SpdxContext {
            path: fixture_root.to_path_buf(),
        });
    }

    let context_path = root.join(SPDX_CONTEXT_PATH);
    let context = read_json(&context_path)?;
    if context
        .pointer("/@context/SpdxDocument")
        .and_then(Value::as_str)
        == Some(SPDX_DOCUMENT_TERM)
    {
        Ok(())
    } else {
        Err(ExternalContractsError::SpdxContext { path: context_path })
    }
}

fn validate_schema(
    registry: &SchemaRegistry,
    identity: &str,
    value: &Value,
    fixture_root: &Path,
) -> Result<(), ExternalContractsError> {
    registry
        .validate(identity, value)
        .map_err(|_| ExternalContractsError::Fixture {
            path: fixture_root.to_path_buf(),
        })
}

fn verify_dsse_envelope(
    root: &Path,
    registry: &SchemaRegistry,
    envelope: &Value,
    fixture_root: &Path,
) -> Result<(), ExternalContractsError> {
    let object = envelope
        .as_object()
        .ok_or(ExternalContractsError::Fixture {
            path: fixture_root.to_path_buf(),
        })?;
    let payload_type = object.get("payloadType").and_then(Value::as_str).ok_or(
        ExternalContractsError::Fixture {
            path: fixture_root.to_path_buf(),
        },
    )?;
    if payload_type != DSSE_PAYLOAD_TYPE {
        return Err(ExternalContractsError::Fixture {
            path: fixture_root.to_path_buf(),
        });
    }
    let payload_text =
        object
            .get("payload")
            .and_then(Value::as_str)
            .ok_or(ExternalContractsError::Fixture {
                path: fixture_root.to_path_buf(),
            })?;
    let payload = decode_base64(payload_text).ok_or(ExternalContractsError::Fixture {
        path: fixture_root.to_path_buf(),
    })?;
    let payload_value =
        serde_json::from_slice(&payload).map_err(|_| ExternalContractsError::Fixture {
            path: fixture_root.to_path_buf(),
        })?;
    validate_schema(registry, STATEMENT_SCHEMA, &payload_value, fixture_root)?;

    let pae = dsse_pae(payload_type.as_bytes(), &payload)?;
    let key = read_test_key(root)?;
    let signatures = object.get("signatures").and_then(Value::as_array).ok_or(
        ExternalContractsError::Fixture {
            path: fixture_root.to_path_buf(),
        },
    )?;
    let verified = signatures.iter().any(|signature| {
        verify_fixture_signature(signature, &pae, &key).is_ok_and(|verified| verified)
    });
    if verified {
        Ok(())
    } else {
        Err(ExternalContractsError::Fixture {
            path: fixture_root.to_path_buf(),
        })
    }
}

fn read_test_key(root: &Path) -> Result<TestKey, ExternalContractsError> {
    let path = root.join(FIXTURES_DIRECTORY).join("test-key.json");
    let value = read_json(&path)?;
    let object = value
        .as_object()
        .ok_or(ExternalContractsError::Fixture { path: path.clone() })?;
    let key_id = object
        .get("keyid")
        .and_then(Value::as_str)
        .filter(|value| *value == DSSE_TEST_KEY_ID)
        .ok_or(ExternalContractsError::Fixture { path: path.clone() })?;
    let algorithm = object
        .get("algorithm")
        .and_then(Value::as_str)
        .filter(|value| *value == DSSE_TEST_ALGORITHM)
        .ok_or(ExternalContractsError::Fixture { path: path.clone() })?;
    let key = object
        .get("key")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or(ExternalContractsError::Fixture { path: path.clone() })?;
    let purpose = object
        .get("purpose")
        .and_then(Value::as_str)
        .filter(|value| *value == "non-production DSSE verifier fixture only")
        .ok_or(ExternalContractsError::Fixture { path })?;
    Ok(TestKey {
        key_id: key_id.to_owned(),
        algorithm: algorithm.to_owned(),
        key: key.to_owned(),
        purpose: purpose.to_owned(),
    })
}

fn read_json(path: &Path) -> Result<Value, ExternalContractsError> {
    let bytes = fs::read(path).map_err(|source| ExternalContractsError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    serde_json::from_slice(&bytes).map_err(|source| ExternalContractsError::Json {
        path: path.to_path_buf(),
        source,
    })
}

fn workspace_root() -> Result<PathBuf, ()> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or(())
}

struct DerivedSchemaRegistry {
    /// Keeps copied schema inputs alive for the registry's complete lifetime.
    _temporary_directory: TemporaryDirectory,
    registry: SchemaRegistry,
    spdx_schema_identity: String,
}

impl DerivedSchemaRegistry {
    fn load(root: &Path) -> Result<Self, ExternalContractsError> {
        let temporary_directory = TemporaryDirectory::new()?;
        for (source, destination) in DERIVED_SCHEMA_FILES {
            let source_path = root.join(source);
            fs::copy(&source_path, temporary_directory.path().join(destination)).map_err(
                |source| ExternalContractsError::Io {
                    path: source_path,
                    source,
                },
            )?;
        }
        let registry =
            SchemaRegistry::from_directories(&[temporary_directory.path().to_path_buf()])
                .map_err(ExternalContractsError::DerivedSchema)?;
        let spdx_schema_identity = registry
            .identity_for_path(&temporary_directory.path().join("spdx-document.schema.json"))
            .map_err(ExternalContractsError::DerivedSchema)?
            .to_owned();
        Ok(Self {
            _temporary_directory: temporary_directory,
            registry,
            spdx_schema_identity,
        })
    }

    fn registry(&self) -> &SchemaRegistry {
        &self.registry
    }

    fn spdx_schema_identity(&self) -> &str {
        &self.spdx_schema_identity
    }
}

struct TemporaryDirectory {
    path: PathBuf,
}

impl TemporaryDirectory {
    fn new() -> Result<Self, ExternalContractsError> {
        for _ in 0..128 {
            let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "oxyflut-external-contracts-{}-{sequence}",
                std::process::id()
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self { path }),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(source) => return Err(ExternalContractsError::Io { path, source }),
            }
        }
        Err(ExternalContractsError::TemporaryDirectory)
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[derive(Debug, Error)]
enum ExternalContractsError {
    #[error("could not read local external-contract input")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("could not parse local external-contract JSON")]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("external-contract snapshot digest does not match")]
    Snapshot { path: PathBuf },
    #[error("external-contract snapshot identity does not match")]
    SourceIdentity { path: PathBuf },
    #[error("external-contract proposal is invalid ({code:?})")]
    Proposal { path: PathBuf, code: ProposalCode },
    #[error("external-contract proposal schema validation failed")]
    ProposalSchema(#[source] SchemaError),
    #[error("derived external-contract schema validation failed")]
    DerivedSchema(#[source] SchemaError),
    #[error("external-contract fixture is invalid")]
    Fixture { path: PathBuf },
    #[error("SPDX document does not bind the local JSON-LD context")]
    SpdxContext { path: PathBuf },
    #[error("SPDX document does not satisfy the authoritative local schema")]
    SpdxSchema { path: PathBuf },
    #[error("DSSE pre-authentication encoding is invalid")]
    Pae,
    #[error("could not create external-contract temporary directory")]
    TemporaryDirectory,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProposalCode {
    MissingDsseEnvelopeEntry,
    RegistryDigestMismatch,
    ContractIdentityMismatch,
}

struct SnapshotSpec {
    artifact_path: &'static str,
    metadata_path: &'static str,
    sha256: &'static str,
    identity: SnapshotIdentity,
}

enum SnapshotIdentity {
    Authoritative(AuthoritativeIdentity),
    Published(PublishedIdentity),
    Derived(DerivedIdentity),
}

struct AuthoritativeIdentity {
    repository: &'static str,
    commit: &'static str,
    path: &'static str,
    retrieval_url: &'static str,
    license: &'static str,
    version: &'static str,
}

struct PublishedIdentity {
    retrieval_url: &'static str,
    publication_source: &'static PublicationSource,
    license: &'static str,
    license_source_path: &'static str,
    license_source_commit: &'static str,
    version: &'static str,
}

struct PublicationSource {
    local_path: &'static str,
    sha256: &'static str,
    path: &'static str,
    commit: &'static str,
}

struct DerivedIdentity {
    local_path: &'static str,
    sha256: &'static str,
    verifier: &'static str,
}

struct ProposedContract {
    name: &'static str,
    version: &'static str,
    source: &'static str,
    local_path: &'static str,
    sha256: &'static str,
    verifier: &'static str,
}

const SNAPSHOTS: &[SnapshotSpec] = &[
    SnapshotSpec {
        artifact_path: "qualification/schemas/external/spdx-3.0.1/jsonld-context/spdx-context.jsonld",
        metadata_path: "qualification/schemas/external/spdx-3.0.1/jsonld-context/source.json",
        sha256: "c72b0928f094c83e5c127784edb1ebca2af74a104fcacc007c332b23cbc788bd",
        identity: SnapshotIdentity::Authoritative(AuthoritativeIdentity {
            repository: "https://github.com/spdx/spdx-spec",
            commit: "61a649da8ca27924ac1ca8d2a061cb228839b24c",
            path: "rdf/spdx-context.jsonld",
            retrieval_url: "https://raw.githubusercontent.com/spdx/spdx-spec/61a649da8ca27924ac1ca8d2a061cb228839b24c/rdf/spdx-context.jsonld",
            license: "Community-Spec-1.0 AND CC-BY-3.0",
            version: "3.0.1",
        }),
    },
    SnapshotSpec {
        artifact_path: "qualification/schemas/external/spdx-3.0.1/spec/serializations.source",
        metadata_path: "qualification/schemas/external/spdx-3.0.1/spec/source.json",
        sha256: "cd62cd4edc55a80a2ca8ca4bb431405f017312f640eccf8f1a7bebdbdf84031a",
        identity: SnapshotIdentity::Authoritative(AuthoritativeIdentity {
            repository: "https://github.com/spdx/spdx-spec",
            commit: "61a649da8ca27924ac1ca8d2a061cb228839b24c",
            path: "docs/serializations.md",
            retrieval_url: "https://raw.githubusercontent.com/spdx/spdx-spec/61a649da8ca27924ac1ca8d2a061cb228839b24c/docs/serializations.md",
            license: "Community-Spec-1.0 AND CC-BY-3.0",
            version: "3.0.1",
        }),
    },
    SnapshotSpec {
        artifact_path: SPDX_SCHEMA_PATH,
        metadata_path: "qualification/schemas/external/spdx-3.0.1/schema/source.json",
        sha256: "582c64e809d5b3ef9bd0c4de13a32391b47b0284a3e8d199569fb96f649234b1",
        identity: SnapshotIdentity::Published(PublishedIdentity {
            retrieval_url: SPDX_SCHEMA_URL,
            publication_source: &PublicationSource {
                local_path: "qualification/schemas/external/spdx-3.0.1/spec/serializations.source",
                sha256: "cd62cd4edc55a80a2ca8ca4bb431405f017312f640eccf8f1a7bebdbdf84031a",
                path: "docs/serializations.md",
                commit: "61a649da8ca27924ac1ca8d2a061cb228839b24c",
            },
            license: "Community-Spec-1.0 AND CC-BY-3.0",
            license_source_path: "LICENSE",
            license_source_commit: "61a649da8ca27924ac1ca8d2a061cb228839b24c",
            version: "3.0.1",
        }),
    },
    SnapshotSpec {
        artifact_path: "qualification/schemas/external/in-toto-statement-v1/spec/statement.source",
        metadata_path: "qualification/schemas/external/in-toto-statement-v1/spec/source.json",
        sha256: "e8fce554c62cd1ea6d7a18cf8bd75f4eb0cc85c761ccde596133968ded093d9e",
        identity: SnapshotIdentity::Authoritative(AuthoritativeIdentity {
            repository: "https://github.com/in-toto/attestation",
            commit: "ee16c68a11dfcfbdc891600cacd767896fe6e724",
            path: "spec/v1.0/statement.md",
            retrieval_url: "https://raw.githubusercontent.com/in-toto/attestation/ee16c68a11dfcfbdc891600cacd767896fe6e724/spec/v1.0/statement.md",
            license: "Apache-2.0",
            version: "1",
        }),
    },
    SnapshotSpec {
        artifact_path: "qualification/schemas/external/in-toto-statement-v1/proto/statement.proto",
        metadata_path: "qualification/schemas/external/in-toto-statement-v1/proto/source.json",
        sha256: "dc52793724dbb1806aa827f0fb54d0e0acfdfc734084dfa4ca75c32d13a38ee2",
        identity: SnapshotIdentity::Authoritative(AuthoritativeIdentity {
            repository: "https://github.com/in-toto/attestation",
            commit: "ee16c68a11dfcfbdc891600cacd767896fe6e724",
            path: "spec/v1.0/statement.proto",
            retrieval_url: "https://raw.githubusercontent.com/in-toto/attestation/ee16c68a11dfcfbdc891600cacd767896fe6e724/spec/v1.0/statement.proto",
            license: "Apache-2.0",
            version: "1",
        }),
    },
    SnapshotSpec {
        artifact_path: "qualification/schemas/external/slsa-provenance-v1/spec/provenance-v1.source",
        metadata_path: "qualification/schemas/external/slsa-provenance-v1/spec/source.json",
        sha256: "7639cd8644d5e3f6faec32cdb5891e9fe368c31976e1020fe639bd56639bd877",
        identity: SnapshotIdentity::Authoritative(AuthoritativeIdentity {
            repository: "https://github.com/slsa-framework/slsa",
            commit: "4d7f142300264276bd3d45ab91d2b0eeb4227932",
            path: "docs/provenance/v1.md",
            retrieval_url: "https://raw.githubusercontent.com/slsa-framework/slsa/4d7f142300264276bd3d45ab91d2b0eeb4227932/docs/provenance/v1.md",
            license: "Community-Spec-1.0",
            version: "1.0",
        }),
    },
    SnapshotSpec {
        artifact_path: "qualification/schemas/external/slsa-provenance-v1/cue/provenance.cue",
        metadata_path: "qualification/schemas/external/slsa-provenance-v1/cue/source.json",
        sha256: "0d68f4ce799a5152151e0efd0fc7ae3b3769b512810d8667eab2ce77e25de40f",
        identity: SnapshotIdentity::Authoritative(AuthoritativeIdentity {
            repository: "https://github.com/slsa-framework/slsa",
            commit: "4d7f142300264276bd3d45ab91d2b0eeb4227932",
            path: "docs/provenance/schema/v1/provenance.cue",
            retrieval_url: "https://raw.githubusercontent.com/slsa-framework/slsa/4d7f142300264276bd3d45ab91d2b0eeb4227932/docs/provenance/schema/v1/provenance.cue",
            license: "Community-Spec-1.0",
            version: "1.0",
        }),
    },
    SnapshotSpec {
        artifact_path: "qualification/schemas/external/dsse-envelope-v1/spec/envelope.source",
        metadata_path: "qualification/schemas/external/dsse-envelope-v1/spec/source.json",
        sha256: "3607b3e4702c4dfd2f3dc90a38546d89b95993227691fdfeb6a38c43759e6f2e",
        identity: SnapshotIdentity::Authoritative(AuthoritativeIdentity {
            repository: "https://github.com/secure-systems-lab/dsse",
            commit: "cacf247ea07024437c91e69ae65f3f4a2df3c657",
            path: "envelope.md",
            retrieval_url: "https://raw.githubusercontent.com/secure-systems-lab/dsse/cacf247ea07024437c91e69ae65f3f4a2df3c657/envelope.md",
            license: "Apache-2.0",
            version: "1",
        }),
    },
    SnapshotSpec {
        artifact_path: "qualification/schemas/external/dsse-envelope-v1/spec/protocol.source",
        metadata_path: "qualification/schemas/external/dsse-envelope-v1/spec/protocol.source.json",
        sha256: "d37115be77f71f793f411b02de02c58a040ad2c421782e5c87a06fb8264b1a48",
        identity: SnapshotIdentity::Authoritative(AuthoritativeIdentity {
            repository: "https://github.com/secure-systems-lab/dsse",
            commit: "cacf247ea07024437c91e69ae65f3f4a2df3c657",
            path: "protocol.md",
            retrieval_url: "https://raw.githubusercontent.com/secure-systems-lab/dsse/cacf247ea07024437c91e69ae65f3f4a2df3c657/protocol.md",
            license: "Apache-2.0",
            version: "1.0.0",
        }),
    },
    SnapshotSpec {
        artifact_path: "qualification/schemas/external/dsse-envelope-v1/proto/envelope.proto",
        metadata_path: "qualification/schemas/external/dsse-envelope-v1/proto/source.json",
        sha256: "cee196c0679fb38e164a8ed71f362cf2667fb697939a41e39d692c4092f0fc12",
        identity: SnapshotIdentity::Authoritative(AuthoritativeIdentity {
            repository: "https://github.com/secure-systems-lab/dsse",
            commit: "cacf247ea07024437c91e69ae65f3f4a2df3c657",
            path: "envelope.proto",
            retrieval_url: "https://raw.githubusercontent.com/secure-systems-lab/dsse/cacf247ea07024437c91e69ae65f3f4a2df3c657/envelope.proto",
            license: "Apache-2.0",
            version: "1",
        }),
    },
    SnapshotSpec {
        artifact_path: "qualification/schemas/external/in-toto-statement-v1/derived/statement.derived.json",
        metadata_path: "qualification/schemas/external/in-toto-statement-v1/derived/source.json",
        sha256: "be6857f91bacbe274ca003b2e4a27b934d8850abf84a8d9359b9cae28e667c17",
        identity: SnapshotIdentity::Derived(DerivedIdentity {
            local_path: "qualification/schemas/external/in-toto-statement-v1/spec/statement.source",
            sha256: "e8fce554c62cd1ea6d7a18cf8bd75f4eb0cc85c761ccde596133968ded093d9e",
            verifier: "jsonschema-0.51.0-draft-2020-12",
        }),
    },
    SnapshotSpec {
        artifact_path: "qualification/schemas/external/slsa-provenance-v1/derived/provenance.derived.json",
        metadata_path: "qualification/schemas/external/slsa-provenance-v1/derived/source.json",
        sha256: "4679b0c1ed9958f9bcbf267c5859a5b945603b9b6601a7b17a4813126a228fb5",
        identity: SnapshotIdentity::Derived(DerivedIdentity {
            local_path: "qualification/schemas/external/slsa-provenance-v1/cue/provenance.cue",
            sha256: "0d68f4ce799a5152151e0efd0fc7ae3b3769b512810d8667eab2ce77e25de40f",
            verifier: "jsonschema-0.51.0-draft-2020-12-format-assertions",
        }),
    },
    SnapshotSpec {
        artifact_path: "qualification/schemas/external/dsse-envelope-v1/derived/envelope.derived.json",
        metadata_path: "qualification/schemas/external/dsse-envelope-v1/derived/source.json",
        sha256: "03507cf10198bd79d9e64d20efcb69966966024603ea69c3fbedc45eb31ae924",
        identity: SnapshotIdentity::Derived(DerivedIdentity {
            local_path: "qualification/schemas/external/dsse-envelope-v1/proto/envelope.proto",
            sha256: "cee196c0679fb38e164a8ed71f362cf2667fb697939a41e39d692c4092f0fc12",
            verifier: "jsonschema-0.51.0-draft-2020-12 plus dsse-pae-test-sha256-keyed-v1",
        }),
    },
];

const DERIVED_SCHEMA_FILES: &[(&str, &str)] = &[
    (
        "qualification/schemas/external/in-toto-statement-v1/derived/statement.derived.json",
        "statement.schema.json",
    ),
    (
        "qualification/schemas/external/slsa-provenance-v1/derived/provenance.derived.json",
        "provenance.schema.json",
    ),
    (SPDX_SCHEMA_PATH, "spdx-document.schema.json"),
    (
        "qualification/schemas/external/dsse-envelope-v1/derived/envelope.derived.json",
        "envelope.schema.json",
    ),
];

const PROPOSED_CONTRACTS: &[ProposedContract] = &[
    ProposedContract {
        name: "spdx-3.0.1",
        version: "3.0.1",
        source: SPDX_SCHEMA_URL,
        local_path: SPDX_SCHEMA_PATH,
        sha256: "582c64e809d5b3ef9bd0c4de13a32391b47b0284a3e8d199569fb96f649234b1",
        verifier: "jsonschema-0.51.0-draft-2020-12:spdx-3.0.1-authoritative-json-schema-plus-jsonld-context-v1",
    },
    ProposedContract {
        name: "in-toto-statement-v1",
        version: "1",
        source: "https://raw.githubusercontent.com/in-toto/attestation/ee16c68a11dfcfbdc891600cacd767896fe6e724/spec/v1.0/statement.md",
        local_path: "qualification/schemas/external/in-toto-statement-v1/spec/statement.source",
        sha256: "e8fce554c62cd1ea6d7a18cf8bd75f4eb0cc85c761ccde596133968ded093d9e",
        verifier: "jsonschema-0.51.0-draft-2020-12:derived-in-toto-statement-v1",
    },
    ProposedContract {
        name: "slsa-provenance-v1",
        version: "1.0",
        source: "https://raw.githubusercontent.com/slsa-framework/slsa/4d7f142300264276bd3d45ab91d2b0eeb4227932/docs/provenance/v1.md",
        local_path: "qualification/schemas/external/slsa-provenance-v1/spec/provenance-v1.source",
        sha256: "7639cd8644d5e3f6faec32cdb5891e9fe368c31976e1020fe639bd56639bd877",
        verifier: "jsonschema-0.51.0-draft-2020-12:derived-slsa-provenance-v1",
    },
    ProposedContract {
        name: "dsse-envelope-v1",
        version: "1",
        source: "https://raw.githubusercontent.com/secure-systems-lab/dsse/cacf247ea07024437c91e69ae65f3f4a2df3c657/envelope.md",
        local_path: "qualification/schemas/external/dsse-envelope-v1/spec/envelope.source",
        sha256: "3607b3e4702c4dfd2f3dc90a38546d89b95993227691fdfeb6a38c43759e6f2e",
        verifier: "jsonschema-0.51.0-draft-2020-12:derived-dsse-envelope-v1-plus-pae-test-sha256-keyed-v1",
    },
];

#[cfg(test)]
#[path = "external_contracts_tests.rs"]
mod tests;
