//! Offline verification for snapshotted external distribution contracts.

use std::fs;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use oxyflut_qualification::hash::{Sha256Digest, hash_file, hash_reader};
use oxyflut_qualification::schema::{SchemaError, SchemaRegistry};
use serde_json::Value;
use thiserror::Error;

use super::super::{CommandError, CommandOutcome};

const FIXTURES_DIRECTORY: &str = "qualification/fixtures/external-contracts";
const DATA_MODELS_DIRECTORY: &str = ".constitution/tech-spec/data-models";
const PROPOSAL_PATH: &str = "qualification/schemas/external/proposed-external-contract-lock.json";
const STATEMENT_SCHEMA: &str = "urn:oxyflut:schema:external-in-toto-statement-v1:1";
const PROVENANCE_SCHEMA: &str = "urn:oxyflut:schema:external-slsa-provenance-v1:1";
const SPDX_SCHEMA: &str = "urn:oxyflut:schema:external-spdx-3.0.1:1";
const DSSE_SCHEMA: &str = "urn:oxyflut:schema:external-dsse-envelope-v1:1";
const EXTERNAL_LOCK_SCHEMA: &str = "urn:oxyflut:schema:external-contract-lock:1";
const DSSE_PAYLOAD_TYPE: &str = "application/vnd.in-toto+json";
const DSSE_TEST_KEY_ID: &str = "oxyflut-fixture-sha256-test-key-v1";
const DSSE_TEST_ALGORITHM: &str = "OXYFLUT-TEST-SHA256-KEYED-V1";
const MAX_DSSE_LENGTH: u64 = 9_223_372_036_854_775_807;
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
    verify_fixtures(root, registry.registry())
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
                require_string(object, "kind", "authoritative", &metadata)?;
                require_string(object, "repository", identity.repository, &metadata)?;
                require_string(object, "commit", identity.commit, &metadata)?;
                require_string(object, "path", identity.path, &metadata)?;
                require_string(object, "retrievalUrl", identity.retrieval_url, &metadata)?;
                require_string(object, "license", identity.license, &metadata)?;
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

fn verify_proposal(root: &Path) -> Result<(), ExternalContractsError> {
    let proposal_path = root.join(PROPOSAL_PATH);
    let proposal = read_json(&proposal_path)?;
    let schema_registry = SchemaRegistry::from_directories(&[root.join(DATA_MODELS_DIRECTORY)])
        .map_err(ExternalContractsError::ProposalSchema)?;
    schema_registry
        .validate(EXTERNAL_LOCK_SCHEMA, &proposal)
        .map_err(ExternalContractsError::ProposalSchema)?;

    let contracts = proposal.get("contracts").and_then(Value::as_object).ok_or(
        ExternalContractsError::Proposal {
            path: proposal_path.clone(),
        },
    )?;
    for contract in PROPOSED_CONTRACTS {
        let Some(value) = contracts.get(contract.name).and_then(Value::as_object) else {
            return Err(ExternalContractsError::Proposal {
                path: proposal_path.clone(),
            });
        };
        require_proposal_string(value, "version", contract.version, &proposal_path)?;
        require_proposal_string(value, "source", contract.source, &proposal_path)?;
        require_proposal_string(value, "epistemicStatus", "kk-locked", &proposal_path)?;
        require_proposal_string(value, "localPath", contract.local_path, &proposal_path)?;
        require_proposal_string(value, "sha256", contract.sha256, &proposal_path)?;
        require_proposal_string(value, "verifier", contract.verifier, &proposal_path)?;
    }
    if contracts.len() != PROPOSED_CONTRACTS.len() {
        return Err(ExternalContractsError::Proposal {
            path: proposal_path,
        });
    }
    Ok(())
}

fn require_proposal_string(
    object: &serde_json::Map<String, Value>,
    key: &str,
    expected: &str,
    proposal_path: &Path,
) -> Result<(), ExternalContractsError> {
    require_string(object, key, expected, proposal_path).map_err(|_| {
        ExternalContractsError::Proposal {
            path: proposal_path.to_path_buf(),
        }
    })
}

fn verify_fixtures(root: &Path, registry: &SchemaRegistry) -> Result<(), ExternalContractsError> {
    let fixtures = root.join(FIXTURES_DIRECTORY).join("positive");
    let statement = read_json(&fixtures.join("statement.json"))?;
    validate_schema(registry, STATEMENT_SCHEMA, &statement, &fixtures)?;

    let provenance = read_json(&fixtures.join("provenance.json"))?;
    validate_schema(registry, PROVENANCE_SCHEMA, &provenance, &fixtures)?;

    let spdx = read_json(&fixtures.join("spdx-document.json"))?;
    validate_schema(registry, SPDX_SCHEMA, &spdx, &fixtures)?;

    let envelope = read_json(&fixtures.join("envelope.json"))?;
    validate_schema(registry, DSSE_SCHEMA, &envelope, &fixtures)?;
    verify_dsse_envelope(root, registry, &envelope, &fixtures)
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

fn dsse_pae(payload_type: &[u8], payload: &[u8]) -> Result<Vec<u8>, ExternalContractsError> {
    let payload_type_length =
        u64::try_from(payload_type.len()).map_err(|_| ExternalContractsError::Pae)?;
    let payload_length = u64::try_from(payload.len()).map_err(|_| ExternalContractsError::Pae)?;
    if payload_type_length > MAX_DSSE_LENGTH || payload_length > MAX_DSSE_LENGTH {
        return Err(ExternalContractsError::Pae);
    }
    let capacity = 24_usize
        .checked_add(payload_type.len())
        .and_then(|value| value.checked_add(payload.len()))
        .ok_or(ExternalContractsError::Pae)?;
    let mut pae = Vec::with_capacity(capacity);
    pae.extend_from_slice(&2_u64.to_le_bytes());
    pae.extend_from_slice(&payload_type_length.to_le_bytes());
    pae.extend_from_slice(payload_type);
    pae.extend_from_slice(&payload_length.to_le_bytes());
    pae.extend_from_slice(payload);
    Ok(pae)
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

fn verify_fixture_signature(
    signature: &Value,
    pae: &[u8],
    key: &TestKey,
) -> Result<bool, ExternalContractsError> {
    if key.key_id != DSSE_TEST_KEY_ID
        || key.algorithm != DSSE_TEST_ALGORITHM
        || key.purpose != "non-production DSSE verifier fixture only"
    {
        return Ok(false);
    }
    let signature = signature.as_object().ok_or(ExternalContractsError::Pae)?;
    if signature.get("keyid").and_then(Value::as_str) != Some(&key.key_id) {
        return Ok(false);
    }
    let encoded = signature
        .get("sig")
        .and_then(Value::as_str)
        .ok_or(ExternalContractsError::Pae)?;
    let actual = decode_base64(encoded).ok_or(ExternalContractsError::Pae)?;
    let capacity = pae
        .len()
        .checked_add(key.key.len())
        .ok_or(ExternalContractsError::Pae)?;
    let mut verification_input = Vec::with_capacity(capacity);
    verification_input.extend_from_slice(pae);
    verification_input.extend_from_slice(key.key.as_bytes());
    let expected =
        hash_reader(Cursor::new(verification_input)).map_err(|_| ExternalContractsError::Pae)?;
    Ok(actual.as_slice() == expected.as_bytes())
}

fn decode_base64(input: &str) -> Option<Vec<u8>> {
    let bytes = input.as_bytes();
    let padding = bytes.iter().rev().take_while(|byte| **byte == b'=').count();
    if padding > 2 || bytes[..bytes.len().saturating_sub(padding)].contains(&b'=') {
        return None;
    }
    let raw = &bytes[..bytes.len().saturating_sub(padding)];
    if raw.len() % 4 == 1 {
        return None;
    }
    if padding > 0 && !bytes.len().is_multiple_of(4) {
        return None;
    }
    let capacity = raw.len().checked_mul(3)?.checked_div(4)?.checked_add(2)?;
    let mut decoded = Vec::with_capacity(capacity);
    let (groups, remainder) = raw.as_chunks::<4>();
    for [first, second, third, fourth] in groups {
        let first = base64_value(*first)?;
        let second = base64_value(*second)?;
        let third = base64_value(*third)?;
        let fourth = base64_value(*fourth)?;
        decoded.push((first << 2) | (second >> 4));
        decoded.push((second << 4) | (third >> 2));
        decoded.push((third << 6) | fourth);
    }
    match remainder {
        [] => Some(decoded),
        [first, second] => {
            let first = base64_value(*first)?;
            let second = base64_value(*second)?;
            if second & 0x0F != 0 {
                return None;
            }
            decoded.push((first << 2) | (second >> 4));
            Some(decoded)
        }
        [first, second, third] => {
            let first = base64_value(*first)?;
            let second = base64_value(*second)?;
            let third = base64_value(*third)?;
            if third & 0x03 != 0 {
                return None;
            }
            decoded.push((first << 2) | (second >> 4));
            decoded.push((second << 4) | (third >> 2));
            Some(decoded)
        }
        [_] | [_, _, _, _] | [_, _, _, _, ..] => None,
    }
}

const fn base64_value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' | b'-' => Some(62),
        b'/' | b'_' => Some(63),
        _ => None,
    }
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
    temporary_directory: TemporaryDirectory,
    registry: SchemaRegistry,
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
        Ok(Self {
            temporary_directory,
            registry,
        })
    }

    fn registry(&self) -> &SchemaRegistry {
        let _ = self.temporary_directory.path();
        &self.registry
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

struct TestKey {
    key_id: String,
    algorithm: String,
    key: String,
    purpose: String,
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
    #[error("external-contract proposal is invalid")]
    Proposal { path: PathBuf },
    #[error("external-contract proposal schema validation failed")]
    ProposalSchema(#[source] SchemaError),
    #[error("derived external-contract schema validation failed")]
    DerivedSchema(#[source] SchemaError),
    #[error("external-contract fixture is invalid")]
    Fixture { path: PathBuf },
    #[error("DSSE pre-authentication encoding is invalid")]
    Pae,
    #[error("could not create external-contract temporary directory")]
    TemporaryDirectory,
}

struct SnapshotSpec {
    artifact_path: &'static str,
    metadata_path: &'static str,
    sha256: &'static str,
    identity: SnapshotIdentity,
}

enum SnapshotIdentity {
    Authoritative(AuthoritativeIdentity),
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
            license: "CC-BY-4.0",
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
            license: "CC-BY-4.0",
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
            license: "CC-BY-4.0",
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
        artifact_path: "qualification/schemas/external/spdx-3.0.1/derived/spdx-document.derived.json",
        metadata_path: "qualification/schemas/external/spdx-3.0.1/derived/source.json",
        sha256: "2397a7e74ed57e3cf7495e01ebe87e463ae60b79cad7a9ff1e4eaf0512ff3cdd",
        identity: SnapshotIdentity::Derived(DerivedIdentity {
            local_path: "qualification/schemas/external/spdx-3.0.1/jsonld-context/spdx-context.jsonld",
            sha256: "c72b0928f094c83e5c127784edb1ebca2af74a104fcacc007c332b23cbc788bd",
            verifier: "jsonschema-0.51.0-draft-2020-12",
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
        sha256: "4969ef98828ddcca7932af375eced1bd7cb24556d9ab16dbbaa2fc588e0fc9f0",
        identity: SnapshotIdentity::Derived(DerivedIdentity {
            local_path: "qualification/schemas/external/slsa-provenance-v1/cue/provenance.cue",
            sha256: "0d68f4ce799a5152151e0efd0fc7ae3b3769b512810d8667eab2ce77e25de40f",
            verifier: "jsonschema-0.51.0-draft-2020-12",
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
    (
        "qualification/schemas/external/spdx-3.0.1/derived/spdx-document.derived.json",
        "spdx-document.schema.json",
    ),
    (
        "qualification/schemas/external/dsse-envelope-v1/derived/envelope.derived.json",
        "envelope.schema.json",
    ),
];

const PROPOSED_CONTRACTS: &[ProposedContract] = &[
    ProposedContract {
        name: "spdx-3.0.1",
        version: "3.0.1",
        source: "https://raw.githubusercontent.com/spdx/spdx-spec/61a649da8ca27924ac1ca8d2a061cb228839b24c/rdf/spdx-context.jsonld",
        local_path: "qualification/schemas/external/spdx-3.0.1/jsonld-context/spdx-context.jsonld",
        sha256: "c72b0928f094c83e5c127784edb1ebca2af74a104fcacc007c332b23cbc788bd",
        verifier: "jsonschema-0.51.0-draft-2020-12:derived-spdx-jsonld-structural-v1",
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
mod tests {
    use std::error::Error;
    use std::fs;
    use std::path::Path;

    use super::{
        ExternalContractsError, PROPOSAL_PATH, SNAPSHOTS, TemporaryDirectory, decode_base64,
        outcome_at, run, verify_at, workspace_root,
    };
    use crate::CommandOutcome;

    #[test]
    fn verifies_authoritative_snapshots_and_local_semantics_without_network()
    -> Result<(), Box<dyn Error>> {
        let root = workspace_root().map_err(|_| "xtask must remain below the workspace root")?;
        verify_at(&root)?;
        assert_eq!(run(&[]), CommandOutcome::Success);
        Ok(())
    }

    #[test]
    fn mutated_snapshot_bytes_fail_the_recorded_digest() -> Result<(), Box<dyn Error>> {
        let temporary = staged_root()?;
        let fixture = temporary
            .path()
            .join("qualification/fixtures/external-contracts/negative/mutated-schema.bytes");
        fs::copy(&fixture, temporary.path().join(SNAPSHOTS[7].artifact_path))?;

        let error = verify_at(temporary.path())
            .err()
            .ok_or("mutation must fail")?;
        assert!(matches!(error, ExternalContractsError::Snapshot { .. }));
        Ok(())
    }

    #[test]
    fn mutated_statement_predicate_and_envelope_fail_local_semantics() -> Result<(), Box<dyn Error>>
    {
        for (fixture_name, target_name) in [
            ("mutated-statement.json", "statement.json"),
            ("mutated-predicate.json", "provenance.json"),
            ("mutated-envelope.json", "envelope.json"),
        ] {
            let temporary = staged_root()?;
            let fixture = temporary
                .path()
                .join("qualification/fixtures/external-contracts/negative")
                .join(fixture_name);
            let target = temporary
                .path()
                .join("qualification/fixtures/external-contracts/positive")
                .join(target_name);
            fs::copy(fixture, target)?;

            let error = verify_at(temporary.path())
                .err()
                .ok_or("mutated semantic fixture must fail")?;
            assert!(matches!(error, ExternalContractsError::Fixture { .. }));
        }
        Ok(())
    }

    #[test]
    fn wrong_registry_digest_and_incomplete_proposal_fail_without_touching_active_lock()
    -> Result<(), Box<dyn Error>> {
        for fixture_name in [
            "wrong-registry-digest.json",
            "incomplete-proposed-lock.json",
        ] {
            let temporary = staged_root()?;
            let active_lock = temporary
                .path()
                .join(".constitution/tech-spec/contracts/external-contract-lock.json");
            let before = fs::read(&active_lock)?;
            let fixture = temporary
                .path()
                .join("qualification/fixtures/external-contracts/negative")
                .join(fixture_name);
            fs::copy(fixture, temporary.path().join(PROPOSAL_PATH))?;

            let error = verify_at(temporary.path())
                .err()
                .ok_or("invalid proposal fixture must fail")?;
            assert!(
                matches!(
                    error,
                    ExternalContractsError::Proposal { .. }
                        | ExternalContractsError::ProposalSchema(_)
                ),
                "unexpected proposal failure: {error:?}"
            );
            assert!(matches!(
                outcome_at(temporary.path()),
                CommandOutcome::Failed(_)
            ));
            assert_eq!(fs::read(active_lock)?, before);
        }
        Ok(())
    }

    #[test]
    fn base64_decoder_accepts_dsse_standard_and_url_safe_forms() {
        assert_eq!(decode_base64("AA=="), Some(vec![0]));
        assert_eq!(decode_base64("-_8="), Some(vec![251, 255]));
        assert_eq!(decode_base64("A"), None);
        assert_eq!(decode_base64("A=AA"), None);
    }

    fn staged_root() -> Result<TemporaryDirectory, Box<dyn Error>> {
        let source = workspace_root().map_err(|_| "xtask must remain below the workspace root")?;
        let temporary = TemporaryDirectory::new()?;
        copy_directory(
            &source.join("qualification"),
            &temporary.path().join("qualification"),
        )?;
        copy_directory(
            &source.join(".constitution/tech-spec/data-models"),
            &temporary.path().join(".constitution/tech-spec/data-models"),
        )?;
        copy_directory(
            &source.join(".constitution/tech-spec/contracts"),
            &temporary.path().join(".constitution/tech-spec/contracts"),
        )?;
        Ok(temporary)
    }

    fn copy_directory(source: &Path, destination: &Path) -> Result<(), std::io::Error> {
        fs::create_dir_all(destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let destination_path = destination.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                copy_directory(&entry.path(), &destination_path)?;
            } else {
                fs::copy(entry.path(), destination_path)?;
            }
        }
        Ok(())
    }
}
