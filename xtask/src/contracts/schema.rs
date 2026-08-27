//! Local schema compilation, committed-instance discovery, and fixture-corpus validation.

use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use oxyflut_qualification::hash::{Sha256Digest, hash_file};
use oxyflut_qualification::schema::{SchemaError, SchemaRegistry};
use serde_json::Value;
use thiserror::Error;

const DATA_MODELS_DIRECTORY: &str = ".constitution/tech-spec/data-models";
const CONTRACTS_DIRECTORY: &str = ".constitution/tech-spec/contracts";
const SCHEMA_SNAPSHOTS_DIRECTORY: &str = "qualification/schemas";
const FIXTURES_DIRECTORY: &str = "qualification/fixtures/contracts";
const SCHEMA_FILE_SUFFIX: &str = ".schema.json";
const INSTANCE_FILE_SUFFIX: &str = ".json";
const EXPECTED_FILE_SUFFIX: &str = ".expected.json";

/// Counts the offline schema work completed by one validation run.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct SchemaRunReport {
    pub(crate) schema_count: usize,
    pub(crate) instance_count: usize,
    pub(crate) fixture_count: usize,
}

/// Errors from the schema validation family.
#[derive(Debug, Error)]
pub(crate) enum ContractSchemaError {
    #[error("local schema registry failed")]
    Registry(#[from] SchemaError),
    #[error("could not read local contract input")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("could not parse local contract input")]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("contract instance does not declare a local schema")]
    MissingSchema { path: PathBuf },
    #[error("contract instance declares a remote schema")]
    RemoteSchema { path: PathBuf },
    #[error("fixture corpus does not meet its declared result")]
    Fixture { path: PathBuf },
    #[error("migration fixture does not preserve its source binding")]
    MigrationFixture,
}

/// Compiles all local schemas, validates every committed instance, and runs the fixture corpus.
///
/// # Errors
///
/// Returns an error for invalid local inputs, schema violations, or fixture expectation drift.
pub(crate) fn validate_workspace(root: &Path) -> Result<SchemaRunReport, ContractSchemaError> {
    let registry = SchemaRegistry::from_directories(&[
        root.join(DATA_MODELS_DIRECTORY),
        root.join(SCHEMA_SNAPSHOTS_DIRECTORY),
    ])?;
    let instances = discover_contract_instances(root, &registry)?;
    for instance in &instances {
        registry.validate(&instance.schema_identity, &instance.value)?;
    }
    let fixture_count = run_fixture_corpus(root, &registry)?;
    validate_migration_fixture(root)?;

    Ok(SchemaRunReport {
        schema_count: registry.identities().len(),
        instance_count: instances.len(),
        fixture_count,
    })
}

fn discover_contract_instances(
    root: &Path,
    registry: &SchemaRegistry,
) -> Result<Vec<ContractInstance>, ContractSchemaError> {
    let directory = root.join(CONTRACTS_DIRECTORY);
    let entries = sorted_files(&directory, INSTANCE_FILE_SUFFIX)?;
    let mut instances = Vec::with_capacity(entries.len());

    for path in entries {
        let value = read_json(&path)?;
        let schema_reference = value
            .get("$schema")
            .and_then(Value::as_str)
            .ok_or_else(|| ContractSchemaError::MissingSchema { path: path.clone() })?;
        let schema_identity = resolve_schema_reference(root, &path, schema_reference, registry)?;
        instances.push(ContractInstance {
            path,
            schema_identity,
            value,
        });
    }

    Ok(instances)
}

fn resolve_schema_reference(
    root: &Path,
    instance_path: &Path,
    reference: &str,
    registry: &SchemaRegistry,
) -> Result<String, ContractSchemaError> {
    if is_remote_reference(reference) {
        return Err(ContractSchemaError::RemoteSchema {
            path: instance_path.to_path_buf(),
        });
    }
    if reference.starts_with("urn:") {
        return Ok(registry.require_current_identity(reference)?.to_owned());
    }

    let Some(parent) = instance_path.parent() else {
        return Err(ContractSchemaError::MissingSchema {
            path: instance_path.to_path_buf(),
        });
    };
    let declared_path = parent.join(reference);
    let schema_path =
        fs::canonicalize(&declared_path).map_err(|source| ContractSchemaError::Io {
            path: declared_path,
            source,
        })?;
    let root_schemas = [
        root.join(DATA_MODELS_DIRECTORY),
        root.join(SCHEMA_SNAPSHOTS_DIRECTORY),
    ]
    .into_iter()
    .map(fs::canonicalize)
    .collect::<Result<Vec<_>, _>>()
    .map_err(|source| ContractSchemaError::Io {
        path: root.to_path_buf(),
        source,
    })?;
    if !root_schemas
        .iter()
        .any(|directory| schema_path.starts_with(directory))
    {
        return Err(ContractSchemaError::RemoteSchema {
            path: instance_path.to_path_buf(),
        });
    }

    Ok(registry.identity_for_path(&schema_path)?.to_owned())
}

fn run_fixture_corpus(
    root: &Path,
    registry: &SchemaRegistry,
) -> Result<usize, ContractSchemaError> {
    let fixture_root = root.join(FIXTURES_DIRECTORY);
    let entries = fs::read_dir(&fixture_root).map_err(|source| ContractSchemaError::Io {
        path: fixture_root.clone(),
        source,
    })?;
    let mut directories = entries
        .collect::<Result<Vec<_>, io::Error>>()
        .map_err(|source| ContractSchemaError::Io {
            path: fixture_root.clone(),
            source,
        })?
        .into_iter()
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
        .collect::<Vec<_>>();
    directories.sort_by_key(|entry| entry.file_name());

    let mut fixture_count = 0;
    for directory in directories {
        let schema_name = directory.file_name().to_string_lossy().into_owned();
        if schema_name == "migration" {
            continue;
        }
        let schema_identity = fixture_schema_identity(root, registry, &schema_name)?;
        fixture_count += validate_fixture_directory(&directory.path(), &schema_identity, registry)?;
    }

    validate_supersession_fixture(&fixture_root, registry)?;
    Ok(fixture_count)
}

fn fixture_schema_identity(
    root: &Path,
    registry: &SchemaRegistry,
    schema_name: &str,
) -> Result<String, ContractSchemaError> {
    let schema_path = if schema_name == "validation-keywords" {
        root.join(SCHEMA_SNAPSHOTS_DIRECTORY)
            .join(format!("{schema_name}{SCHEMA_FILE_SUFFIX}"))
    } else {
        root.join(DATA_MODELS_DIRECTORY)
            .join(format!("{schema_name}{SCHEMA_FILE_SUFFIX}"))
    };
    Ok(registry.identity_for_path(&schema_path)?.to_owned())
}

fn validate_fixture_directory(
    directory: &Path,
    schema_identity: &str,
    registry: &SchemaRegistry,
) -> Result<usize, ContractSchemaError> {
    let mut fixture_count = 0;
    for kind in ["valid", "invalid"] {
        let kind_directory = directory.join(kind);
        let files = sorted_files(&kind_directory, INSTANCE_FILE_SUFFIX)?;
        for path in files {
            if path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(EXPECTED_FILE_SUFFIX))
            {
                continue;
            }
            let value = read_json(&path)?;
            if kind == "valid" {
                registry
                    .validate(schema_identity, &value)
                    .map_err(|_| ContractSchemaError::Fixture { path: path.clone() })?;
            } else {
                validate_invalid_fixture(&path, schema_identity, &value, registry)?;
            }
            fixture_count += 1;
        }
    }
    Ok(fixture_count)
}

fn validate_invalid_fixture(
    path: &Path,
    schema_identity: &str,
    value: &Value,
    registry: &SchemaRegistry,
) -> Result<(), ContractSchemaError> {
    let expected_path = expected_sidecar_path(path)?;
    let expected = read_json(&expected_path)?;
    let expected_kind = expected
        .get("kind")
        .and_then(Value::as_str)
        .ok_or_else(|| ContractSchemaError::Fixture {
            path: expected_path.clone(),
        })?;

    if expected_kind == "superseded-identity" {
        let identity = value
            .get("$schema")
            .and_then(Value::as_str)
            .ok_or_else(|| ContractSchemaError::Fixture {
                path: path.to_path_buf(),
            })?;
        let current = expected
            .get("supersededBy")
            .and_then(Value::as_str)
            .ok_or_else(|| ContractSchemaError::Fixture {
                path: expected_path.clone(),
            })?;
        match registry.require_current_identity(identity) {
            Err(SchemaError::SupersededIdentity { superseded_by, .. })
                if superseded_by == current =>
            {
                return Ok(());
            }
            _ => {
                return Err(ContractSchemaError::Fixture {
                    path: path.to_path_buf(),
                });
            }
        }
    }

    let expected_paths = expected_error_paths(&expected, &expected_path)?;
    match registry.validate(schema_identity, value) {
        Err(SchemaError::Validation { issues, .. }) => {
            let actual_paths = issues
                .iter()
                .map(|issue| issue.instance_path.clone())
                .collect::<BTreeSet<_>>();
            if expected_paths
                .iter()
                .all(|path| actual_paths.contains(path))
            {
                Ok(())
            } else {
                Err(ContractSchemaError::Fixture {
                    path: path.to_path_buf(),
                })
            }
        }
        _ => Err(ContractSchemaError::Fixture {
            path: path.to_path_buf(),
        }),
    }
}

fn expected_sidecar_path(path: &Path) -> Result<PathBuf, ContractSchemaError> {
    let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
        return Err(ContractSchemaError::Fixture {
            path: path.to_path_buf(),
        });
    };
    Ok(path.with_file_name(format!("{stem}{EXPECTED_FILE_SUFFIX}")))
}

fn expected_error_paths(
    expected: &Value,
    path: &Path,
) -> Result<BTreeSet<String>, ContractSchemaError> {
    let Some(paths) = expected.get("errorPaths").and_then(Value::as_array) else {
        return Err(ContractSchemaError::Fixture {
            path: path.to_path_buf(),
        });
    };
    paths
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| ContractSchemaError::Fixture {
                    path: path.to_path_buf(),
                })
        })
        .collect()
}

fn validate_supersession_fixture(
    fixture_root: &Path,
    registry: &SchemaRegistry,
) -> Result<(), ContractSchemaError> {
    let path = fixture_root.join("supersession.json");
    let fixture = read_json(&path)?;
    let Some(schemas) = fixture.get("schemas").and_then(Value::as_array) else {
        return Err(ContractSchemaError::Fixture { path });
    };
    for schema in schemas {
        let Some(old_identity) = schema.get("superseded").and_then(Value::as_str) else {
            return Err(ContractSchemaError::Fixture { path: path.clone() });
        };
        let Some(current_identity) = schema.get("current").and_then(Value::as_str) else {
            return Err(ContractSchemaError::Fixture { path: path.clone() });
        };
        match registry.require_current_identity(old_identity) {
            Err(SchemaError::SupersededIdentity { superseded_by, .. })
                if superseded_by == current_identity => {}
            _ => return Err(ContractSchemaError::Fixture { path: path.clone() }),
        }
    }
    Ok(())
}

fn validate_migration_fixture(root: &Path) -> Result<(), ContractSchemaError> {
    let directory = root.join(FIXTURES_DIRECTORY).join("migration");
    let source_path = directory.join("source.json");
    let expected_digest =
        fs::read_to_string(directory.join("source.sha256")).map_err(|source| {
            ContractSchemaError::Io {
                path: directory.join("source.sha256"),
                source,
            }
        })?;
    let expected_digest: Sha256Digest = expected_digest
        .trim()
        .parse()
        .map_err(|_| ContractSchemaError::MigrationFixture)?;
    let before = hash_file(&source_path).map_err(|source| ContractSchemaError::Io {
        path: source_path.clone(),
        source,
    })?;
    let derived_path = directory.join("derived.json");
    let derived = read_json(&derived_path)?;
    let declared_digest: Sha256Digest = derived
        .pointer("/derivedFrom/sha256")
        .and_then(Value::as_str)
        .ok_or(ContractSchemaError::MigrationFixture)?
        .parse()
        .map_err(|_| ContractSchemaError::MigrationFixture)?;
    let after = hash_file(&source_path).map_err(|source| ContractSchemaError::Io {
        path: source_path,
        source,
    })?;
    if before == expected_digest && after == expected_digest && declared_digest == expected_digest {
        Ok(())
    } else {
        Err(ContractSchemaError::MigrationFixture)
    }
}

fn sorted_files(directory: &Path, suffix: &str) -> Result<Vec<PathBuf>, ContractSchemaError> {
    let entries = fs::read_dir(directory).map_err(|source| ContractSchemaError::Io {
        path: directory.to_path_buf(),
        source,
    })?;
    let entries = entries
        .collect::<Result<Vec<_>, io::Error>>()
        .map_err(|source| ContractSchemaError::Io {
            path: directory.to_path_buf(),
            source,
        })?;
    let mut paths = Vec::new();
    for entry in entries {
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|source| ContractSchemaError::Io {
                path: path.clone(),
                source,
            })?;
        let file_name = path.file_name().and_then(|name| name.to_str());
        if file_type.is_file() && file_name.is_some_and(|name| name.ends_with(suffix)) {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

fn read_json(path: &Path) -> Result<Value, ContractSchemaError> {
    let bytes = fs::read(path).map_err(|source| ContractSchemaError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    serde_json::from_slice(&bytes).map_err(|source| ContractSchemaError::Json {
        path: path.to_path_buf(),
        source,
    })
}

fn is_remote_reference(reference: &str) -> bool {
    reference.starts_with('/')
        || reference.starts_with("http:")
        || reference.starts_with("https:")
        || reference.starts_with("file:")
        || reference.contains('\\')
}

struct ContractInstance {
    #[allow(dead_code, reason = "Instance paths become diagnostics in OXY-A003.")]
    path: PathBuf,
    schema_identity: String,
    value: Value,
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::path::{Path, PathBuf};

    use super::validate_workspace;

    #[test]
    fn schema_compiles_committed_contract_instances_and_fixture_corpus()
    -> Result<(), Box<dyn Error>> {
        let report = validate_workspace(&workspace_root()?)?;
        assert_eq!(report.schema_count, 17);
        assert_eq!(report.instance_count, 6);
        assert!(report.fixture_count >= 80);
        Ok(())
    }

    fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| "xtask must remain directly below the workspace root".into())
    }
}
