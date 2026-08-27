//! Local schema compilation, committed-instance discovery, and fixture-corpus validation.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use oxyflut_qualification::hash::{Sha256Digest, hash_file};
use oxyflut_qualification::schema::{SchemaError, SchemaRegistry, ValidationIssue};
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
    Registry(#[source] SchemaError),
    #[error("committed contract instance failed schema processing")]
    Instance {
        path: PathBuf,
        #[source]
        source: SchemaError,
    },
    #[error("fixture schema failed schema processing")]
    FixtureSchema {
        path: PathBuf,
        #[source]
        source: SchemaError,
    },
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

/// The validation family and path that should appear in a content-free command summary.
pub(crate) enum ContractSchemaFailure {
    /// The local schema registry did not compile.
    Compilation,
    /// A committed contract instance did not validate.
    Instances(PathBuf),
    /// A fixture or migration fixture did not validate.
    Fixtures(PathBuf),
}

impl ContractSchemaError {
    /// Returns the command-summary family for this schema error.
    #[must_use]
    pub(crate) fn failure_family(&self, root: &Path) -> ContractSchemaFailure {
        let fixture_root = root.join(FIXTURES_DIRECTORY);
        match self {
            Self::Registry(_) => ContractSchemaFailure::Compilation,
            Self::Instance { path, .. }
            | Self::MissingSchema { path }
            | Self::RemoteSchema { path } => ContractSchemaFailure::Instances(path.clone()),
            Self::FixtureSchema { path, .. } | Self::Fixture { path } => {
                ContractSchemaFailure::Fixtures(path.clone())
            }
            Self::MigrationFixture => {
                ContractSchemaFailure::Fixtures(fixture_root.join("migration"))
            }
            Self::Io { path, .. } | Self::Json { path, .. } if path.starts_with(&fixture_root) => {
                ContractSchemaFailure::Fixtures(path.clone())
            }
            Self::Io { path, .. } | Self::Json { path, .. } => {
                ContractSchemaFailure::Instances(path.clone())
            }
        }
    }
}

/// Compiles all local schemas, validates every committed instance, and runs the fixture corpus.
///
/// # Errors
///
/// Returns an error for invalid local inputs, schema violations, or fixture expectation drift.
#[cfg(test)]
pub(crate) fn validate_workspace(root: &Path) -> Result<SchemaRunReport, ContractSchemaError> {
    let registry = compile_workspace(root)?;
    validate_compiled_workspace(root, &registry)
}

/// Compiles the local schema registry before validating any instance or fixture.
///
/// # Errors
///
/// Returns an error only when the local schema registry cannot be compiled.
pub(crate) fn compile_workspace(root: &Path) -> Result<SchemaRegistry, ContractSchemaError> {
    SchemaRegistry::from_directories(&[
        root.join(DATA_MODELS_DIRECTORY),
        root.join(SCHEMA_SNAPSHOTS_DIRECTORY),
    ])
    .map_err(ContractSchemaError::Registry)
}

/// Validates committed instances and fixtures against a compiled local registry.
///
/// # Errors
///
/// Returns an error when a committed instance, fixture, or migration fixture fails validation.
pub(crate) fn validate_compiled_workspace(
    root: &Path,
    registry: &SchemaRegistry,
) -> Result<SchemaRunReport, ContractSchemaError> {
    let instances = discover_contract_instances(root, registry)?;
    for instance in &instances {
        registry
            .validate(&instance.schema_identity, &instance.value)
            .map_err(|source| ContractSchemaError::Instance {
                path: instance.path.clone(),
                source,
            })?;
    }
    let fixture_count = run_fixture_corpus(root, registry)?;
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
        return registry
            .require_current_identity(reference)
            .map(str::to_owned)
            .map_err(|source| ContractSchemaError::Instance {
                path: instance_path.to_path_buf(),
                source,
            });
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

    registry
        .identity_for_path(&schema_path)
        .map(str::to_owned)
        .map_err(|source| ContractSchemaError::Instance {
            path: instance_path.to_path_buf(),
            source,
        })
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
    registry
        .identity_for_path(&schema_path)
        .map(str::to_owned)
        .map_err(|source| ContractSchemaError::FixtureSchema {
            path: schema_path,
            source,
        })
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

    let expected_errors = expected_errors(&expected, &expected_path)?;
    match registry.validate(schema_identity, value) {
        Err(SchemaError::Validation { issues, .. })
            if error_locations_match(&expected_errors, &issues) =>
        {
            Ok(())
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

fn expected_errors(
    expected: &Value,
    path: &Path,
) -> Result<Vec<FixtureErrorLocation>, ContractSchemaError> {
    let Some(errors) = expected.get("errorPaths").and_then(Value::as_array) else {
        return Err(ContractSchemaError::Fixture {
            path: path.to_path_buf(),
        });
    };

    let mut locations = errors
        .iter()
        .map(|error| {
            let instance_path = error
                .get("instancePath")
                .and_then(Value::as_str)
                .map(str::to_owned)
                .ok_or_else(|| ContractSchemaError::Fixture {
                    path: path.to_path_buf(),
                })?;
            let keyword = error
                .get("keyword")
                .and_then(Value::as_str)
                .map(str::to_owned)
                .ok_or_else(|| ContractSchemaError::Fixture {
                    path: path.to_path_buf(),
                })?;
            Ok(FixtureErrorLocation {
                instance_path,
                keyword,
            })
        })
        .collect::<Result<Vec<_>, ContractSchemaError>>()?;
    locations.sort_unstable();
    Ok(locations)
}

fn error_locations_match(expected: &[FixtureErrorLocation], actual: &[ValidationIssue]) -> bool {
    let mut actual_locations = actual
        .iter()
        .map(|issue| FixtureErrorLocation {
            instance_path: issue.instance_path.clone(),
            keyword: issue.keyword.clone(),
        })
        .collect::<Vec<_>>();
    actual_locations.sort_unstable();
    expected == actual_locations
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FixtureErrorLocation {
    instance_path: String,
    keyword: String,
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

    use oxyflut_qualification::schema::ValidationIssue;

    use super::{FixtureErrorLocation, error_locations_match, validate_workspace};

    #[test]
    fn schema_compiles_committed_contract_instances_and_fixture_corpus()
    -> Result<(), Box<dyn Error>> {
        let report = validate_workspace(&workspace_root()?)?;
        assert_eq!(report.schema_count, 17);
        assert_eq!(report.instance_count, 6);
        assert!(report.fixture_count >= 80);
        Ok(())
    }

    #[test]
    fn fixture_errors_require_an_exact_path_and_keyword_multiset() {
        let expected = vec![FixtureErrorLocation {
            instance_path: "/gate/status".to_owned(),
            keyword: "enum".to_owned(),
        }];
        let matching = vec![ValidationIssue {
            instance_path: "/gate/status".to_owned(),
            schema_path: "/$defs/gate/status/enum".to_owned(),
            keyword: "enum".to_owned(),
        }];
        assert!(error_locations_match(&expected, &matching));

        let extra = vec![
            matching[0].clone(),
            ValidationIssue {
                instance_path: "".to_owned(),
                schema_path: "/required".to_owned(),
                keyword: "required".to_owned(),
            },
        ];
        assert!(!error_locations_match(&expected, &extra));

        let different_keyword = vec![ValidationIssue {
            instance_path: "/gate/status".to_owned(),
            schema_path: "/$defs/gate/status/const".to_owned(),
            keyword: "const".to_owned(),
        }];
        assert!(!error_locations_match(&expected, &different_keyword));
    }

    fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| "xtask must remain directly below the workspace root".into())
    }
}
