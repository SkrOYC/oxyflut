//! Offline Draft 2020-12 JSON Schema compilation and instance validation.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use jsonschema::{Draft, Registry, Retrieve, Uri, Validator};
use serde_json::Value;
use thiserror::Error;

const SCHEMA_FILE_SUFFIX: &str = ".schema.json";
const LOCAL_SCHEME_PREFIX: &str = "oxyflut://registry/";

/// A stable, machine-readable result for one failed schema assertion.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ValidationIssue {
    /// RFC 6901 JSON Pointer for the failing instance location. The root is the empty string.
    pub instance_path: String,
    /// RFC 6901 JSON Pointer for the failing schema keyword.
    pub schema_path: String,
    /// The final JSON Pointer segment naming the failed schema keyword.
    pub keyword: String,
}

/// Errors from loading, compiling, or applying a local schema registry.
#[derive(Debug, Error)]
pub enum SchemaError {
    /// A local directory or schema file could not be read.
    #[error("could not read a local schema input")]
    Io {
        /// The local path that could not be read.
        path: PathBuf,
        /// The underlying local I/O failure.
        #[source]
        source: io::Error,
    },
    /// A local schema or instance could not be parsed as JSON.
    #[error("could not parse local JSON")]
    Json {
        /// The local path that could not be parsed.
        path: PathBuf,
        /// The underlying JSON parsing failure.
        #[source]
        source: serde_json::Error,
    },
    /// A schema does not name its `$id` as a string.
    #[error("schema identity is invalid")]
    InvalidIdentity {
        /// The local schema that declares the invalid identity.
        path: PathBuf,
    },
    /// A schema tries to resolve a nonlocal identity or reference.
    #[error("remote schema resolution is forbidden")]
    RemoteReference {
        /// The local schema that contains the reference.
        path: PathBuf,
        /// The forbidden URI.
        uri: String,
    },
    /// A reference was not declared by the local registry.
    #[error("schema reference is not declared by the local registry")]
    UndeclaredReference {
        /// The schema identity being resolved.
        identity: String,
    },
    /// The validator rejected the schema document or a local reference graph.
    #[error("could not compile local schema registry")]
    Compilation,
    /// A caller selected a schema that is not in the local registry.
    #[error("schema identity is not declared by the local registry")]
    UnknownSchema {
        /// The requested schema identity.
        identity: String,
    },
    /// A caller supplied an identity that was intentionally superseded before durable evidence.
    #[error("schema identity is superseded")]
    SupersededIdentity {
        /// The rejected former identity.
        identity: String,
        /// The current local schema identity that supersedes it.
        superseded_by: String,
    },
    /// An instance failed local schema validation.
    #[error("instance failed schema validation")]
    Validation {
        /// The schema identity selected for validation.
        identity: String,
        /// Sorted, stable assertion failures.
        issues: Vec<ValidationIssue>,
    },
}

/// A registry of locally compiled Draft 2020-12 schema validators.
pub struct SchemaRegistry {
    validators: BTreeMap<String, Validator>,
    identities_by_path: BTreeMap<PathBuf, String>,
}

impl SchemaRegistry {
    /// Compiles every `.schema.json` file below each local directory.
    ///
    /// The registry has no file or HTTP resolver. A custom in-memory retriever can return only
    /// schemas collected from these directories, and all other references fail closed.
    ///
    /// # Errors
    ///
    /// Returns an error if a directory cannot be read, a schema is invalid, or a reference is
    /// remote or absent from the local registry.
    pub fn from_directories(directories: &[PathBuf]) -> Result<Self, SchemaError> {
        let sources = collect_sources(directories)?;
        let retriever = LocalRetriever::from_sources(&sources);
        let mut builder = Registry::new()
            .draft(Draft::Draft202012)
            .retriever(retriever.clone());

        for source in &sources {
            builder = builder
                .add(
                    &source.registry_uri,
                    Draft::Draft202012.create_resource(source.value.clone()),
                )
                .map_err(|_| SchemaError::Compilation)?;
        }

        let registry = builder.prepare().map_err(|error| {
            let identity = error.to_string();
            if let Some(identity) = local_identity_from_registry_error(&identity) {
                SchemaError::UndeclaredReference { identity }
            } else {
                SchemaError::Compilation
            }
        })?;

        let mut validators = BTreeMap::new();
        let mut identities_by_path = BTreeMap::new();
        for source in &sources {
            let validator = jsonschema::draft202012::options()
                .with_registry(&registry)
                .with_retriever(retriever.clone())
                .build(&source.value)
                .map_err(|_| SchemaError::Compilation)?;
            validators.insert(source.identity.clone(), validator);
            identities_by_path.insert(source.path.clone(), source.identity.clone());
        }

        Ok(Self {
            validators,
            identities_by_path,
        })
    }

    /// Returns all declared current schema identities in deterministic order.
    #[must_use]
    pub fn identities(&self) -> Vec<&str> {
        self.validators.keys().map(String::as_str).collect()
    }

    /// Returns the declared identity for a local schema path.
    ///
    /// # Errors
    ///
    /// Returns an error if the path is not one of the compiled local schemas.
    pub fn identity_for_path(&self, path: &Path) -> Result<&str, SchemaError> {
        let normalized = normalized_path(path)?;
        self.identities_by_path
            .get(&normalized)
            .map(String::as_str)
            .ok_or_else(|| SchemaError::UnknownSchema {
                identity: normalized.display().to_string(),
            })
    }

    /// Rejects a superseded identity or returns the same identity when it remains current.
    ///
    /// # Errors
    ///
    /// Returns an explicit supersession error for pre-evidence schema identities.
    pub fn require_current_identity<'identity>(
        &self,
        identity: &'identity str,
    ) -> Result<&'identity str, SchemaError> {
        if let Some(superseded_by) = superseding_identity(identity) {
            return Err(SchemaError::SupersededIdentity {
                identity: identity.to_owned(),
                superseded_by: superseded_by.to_owned(),
            });
        }
        Ok(identity)
    }

    /// Validates an instance against one declared current schema identity.
    ///
    /// # Errors
    ///
    /// Returns sorted validation issues or an error when the identity is unknown or superseded.
    pub fn validate(&self, identity: &str, instance: &Value) -> Result<(), SchemaError> {
        self.require_current_identity(identity)?;
        let validator =
            self.validators
                .get(identity)
                .ok_or_else(|| SchemaError::UnknownSchema {
                    identity: identity.to_owned(),
                })?;

        let mut issues = validator
            .iter_errors(instance)
            .map(|error| {
                let schema_path = error.schema_path().to_string();
                ValidationIssue {
                    instance_path: error.instance_path().to_string(),
                    keyword: last_json_pointer_segment(&schema_path),
                    schema_path,
                }
            })
            .collect::<Vec<_>>();
        issues.sort_unstable();

        if issues.is_empty() {
            Ok(())
        } else {
            Err(SchemaError::Validation {
                identity: identity.to_owned(),
                issues,
            })
        }
    }
}

#[derive(Clone)]
struct LocalRetriever {
    schemas: Arc<BTreeMap<String, Value>>,
}

impl LocalRetriever {
    fn from_sources(sources: &[SchemaSource]) -> Self {
        let mut schemas = BTreeMap::new();
        for source in sources {
            schemas.insert(source.registry_uri.clone(), source.value.clone());
            schemas.insert(source.identity.clone(), source.value.clone());
        }
        Self {
            schemas: Arc::new(schemas),
        }
    }
}

impl Retrieve for LocalRetriever {
    fn retrieve(&self, uri: &Uri<String>) -> Result<Value, Box<dyn Error + Send + Sync>> {
        self.schemas.get(uri.as_str()).cloned().ok_or_else(|| {
            Box::new(UndeclaredReference(uri.to_string())) as Box<dyn Error + Send + Sync>
        })
    }
}

#[derive(Debug)]
struct UndeclaredReference(String);

impl std::fmt::Display for UndeclaredReference {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for UndeclaredReference {}

struct SchemaSource {
    path: PathBuf,
    registry_uri: String,
    identity: String,
    value: Value,
}

fn collect_sources(directories: &[PathBuf]) -> Result<Vec<SchemaSource>, SchemaError> {
    let mut schema_paths = BTreeSet::new();
    for directory in directories {
        collect_schema_paths(directory, &mut schema_paths)?;
    }

    let mut sources = Vec::with_capacity(schema_paths.len());
    let mut identities = BTreeSet::new();
    for (index, path) in schema_paths.into_iter().enumerate() {
        let bytes = fs::read(&path).map_err(|source| SchemaError::Io {
            path: path.clone(),
            source,
        })?;
        let value = serde_json::from_slice(&bytes).map_err(|source| SchemaError::Json {
            path: path.clone(),
            source,
        })?;
        forbid_remote_references(&path, &value)?;
        let identity = schema_identity(&path, &value, index)?;
        if !identities.insert(identity.clone()) {
            return Err(SchemaError::InvalidIdentity { path });
        }
        let registry_uri = format!("{LOCAL_SCHEME_PREFIX}{index}");
        sources.push(SchemaSource {
            path,
            registry_uri,
            identity,
            value,
        });
    }

    if sources.is_empty() {
        return Err(SchemaError::Compilation);
    }
    Ok(sources)
}

fn collect_schema_paths(
    directory: &Path,
    paths: &mut BTreeSet<PathBuf>,
) -> Result<(), SchemaError> {
    let entries = fs::read_dir(directory).map_err(|source| SchemaError::Io {
        path: directory.to_path_buf(),
        source,
    })?;
    let mut entries = entries
        .collect::<Result<Vec<_>, io::Error>>()
        .map_err(|source| SchemaError::Io {
            path: directory.to_path_buf(),
            source,
        })?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();
        let file_type = entry.file_type().map_err(|source| SchemaError::Io {
            path: path.clone(),
            source,
        })?;
        if file_type.is_dir() {
            collect_schema_paths(&path, paths)?;
        } else if file_type.is_file()
            && path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(SCHEMA_FILE_SUFFIX))
        {
            paths.insert(normalized_path(&path)?);
        }
    }
    Ok(())
}

fn normalized_path(path: &Path) -> Result<PathBuf, SchemaError> {
    fs::canonicalize(path).map_err(|source| SchemaError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn schema_identity(path: &Path, value: &Value, index: usize) -> Result<String, SchemaError> {
    match value.get("$id") {
        Some(Value::String(identity)) => Ok(identity.clone()),
        Some(_) => Err(SchemaError::InvalidIdentity {
            path: path.to_path_buf(),
        }),
        None => Ok(format!("{LOCAL_SCHEME_PREFIX}{index}")),
    }
}

fn forbid_remote_references(path: &Path, value: &Value) -> Result<(), SchemaError> {
    match value {
        Value::Array(values) => {
            for value in values {
                forbid_remote_references(path, value)?;
            }
        }
        Value::Object(values) => {
            for (key, value) in values {
                if matches!(key.as_str(), "$id" | "$ref" | "$dynamicRef") {
                    let Some(uri) = value.as_str() else {
                        return Err(SchemaError::InvalidIdentity {
                            path: path.to_path_buf(),
                        });
                    };
                    if reference_is_nonlocal(key, uri) {
                        return Err(SchemaError::RemoteReference {
                            path: path.to_path_buf(),
                            uri: uri.to_owned(),
                        });
                    }
                }
                forbid_remote_references(path, value)?;
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
    Ok(())
}

fn reference_is_nonlocal(key: &str, uri: &str) -> bool {
    if uri.starts_with("http:") || uri.starts_with("https:") || uri.starts_with("file:") {
        return true;
    }
    if key == "$id" {
        return !(uri.starts_with("urn:oxyflut:") || uri.starts_with(LOCAL_SCHEME_PREFIX));
    }
    uri.contains(':') && !(uri.starts_with("urn:oxyflut:") || uri.starts_with(LOCAL_SCHEME_PREFIX))
}

fn last_json_pointer_segment(path: &str) -> String {
    if let Some(segment) = path.rsplit('/').next() {
        segment.to_owned()
    } else {
        String::new()
    }
}

// `jsonschema` 0.51.0 doesn't expose the unresolved registry identity as structured data.
// This deliberately parses its current quoted `Display` message; update this coupling when the
// pinned validator version changes.
fn local_identity_from_registry_error(error: &str) -> Option<String> {
    error
        .split('`')
        .nth(1)
        .or_else(|| error.split('\'').nth(1))
        .map(str::to_owned)
}

fn superseding_identity(identity: &str) -> Option<&'static str> {
    match identity {
        "urn:oxyflut:schema:accessibility-map:4" => Some("urn:oxyflut:schema:accessibility-map:5"),
        "urn:oxyflut:schema:artifact-manifest:3" => Some("urn:oxyflut:schema:artifact-manifest:4"),
        "urn:oxyflut:schema:capability-baseline:3" => {
            Some("urn:oxyflut:schema:capability-baseline:4")
        }
        "urn:oxyflut:schema:capability-traceability:2" => {
            Some("urn:oxyflut:schema:capability-traceability:3")
        }
        "urn:oxyflut:schema:qualification-evidence:4" => {
            Some("urn:oxyflut:schema:qualification-evidence:5")
        }
        "urn:oxyflut:schema:raw-measurement:1" => Some("urn:oxyflut:schema:raw-measurement:2"),
        "urn:oxyflut:schema:platform-contracts:4" => {
            Some("urn:oxyflut:schema:platform-contracts:5")
        }
        "urn:oxyflut:schema:qualification-lock:4" => {
            Some("urn:oxyflut:schema:qualification-lock:5")
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::path::{Path, PathBuf};

    use serde_json::json;

    use super::{SchemaError, SchemaRegistry};

    #[test]
    fn schema_compiles_every_durable_schema_offline() -> Result<(), Box<dyn Error>> {
        let registry = workspace_registry()?;
        assert_eq!(registry.identities().len(), 17);
        Ok(())
    }

    #[test]
    fn schema_rejects_remote_and_undeclared_references() -> Result<(), Box<dyn Error>> {
        let directory = temporary_schema_directory("references");
        fs::create_dir_all(&directory)?;
        fs::write(
            directory.join("remote.schema.json"),
            r#"{"$id":"urn:oxyflut:test:remote","$ref":"https://example.invalid/schema.json"}"#,
        )?;
        let remote = SchemaRegistry::from_directories(std::slice::from_ref(&directory));
        assert!(matches!(remote, Err(SchemaError::RemoteReference { .. })));

        fs::remove_file(directory.join("remote.schema.json"))?;
        fs::write(
            directory.join("undeclared.schema.json"),
            r#"{"$id":"urn:oxyflut:test:undeclared","$ref":"urn:oxyflut:test:missing"}"#,
        )?;
        let undeclared = SchemaRegistry::from_directories(std::slice::from_ref(&directory));
        assert!(matches!(
            undeclared,
            Err(SchemaError::UndeclaredReference { .. })
        ));
        fs::remove_dir_all(directory)?;

        Ok(())
    }

    #[test]
    fn schema_sorts_instance_errors_by_instance_then_schema_path() -> Result<(), Box<dyn Error>> {
        let directory = temporary_schema_directory("ordering");
        fs::create_dir_all(&directory)?;
        fs::write(
            directory.join("ordering.schema.json"),
            r#"{
                "$id":"urn:oxyflut:test:ordering",
                "type":"object",
                "additionalProperties":false,
                "required":["a","z"],
                "properties":{"a":{"type":"string"},"z":{"type":"integer"}}
            }"#,
        )?;
        let registry = SchemaRegistry::from_directories(std::slice::from_ref(&directory))?;
        let result = registry.validate("urn:oxyflut:test:ordering", &json!({"a": 1, "z": "x"}));
        let Err(SchemaError::Validation { issues, .. }) = result else {
            return Err("expected sorted validation issues".into());
        };
        assert!(issues.windows(2).all(|pair| pair[0] <= pair[1]));
        assert!(
            issues
                .iter()
                .all(|issue| issue.instance_path.starts_with('/'))
        );
        fs::remove_dir_all(directory)?;

        Ok(())
    }

    #[test]
    fn schema_rejects_superseded_pre_evidence_identities() -> Result<(), Box<dyn Error>> {
        let registry = workspace_registry()?;
        let result =
            registry.require_current_identity("urn:oxyflut:schema:qualification-evidence:4");
        assert!(matches!(
            result,
            Err(SchemaError::SupersededIdentity { .. })
        ));
        Ok(())
    }

    fn workspace_registry() -> Result<SchemaRegistry, SchemaError> {
        let root = workspace_root()?;
        SchemaRegistry::from_directories(&[
            root.join(".constitution/tech-spec/data-models"),
            root.join("qualification/schemas"),
        ])
    }

    fn workspace_root() -> Result<PathBuf, SchemaError> {
        let crate_directory = Path::new(env!("CARGO_MANIFEST_DIR"));
        let Some(crates_directory) = crate_directory.parent() else {
            return Err(SchemaError::Compilation);
        };
        let Some(root) = crates_directory.parent() else {
            return Err(SchemaError::Compilation);
        };
        Ok(root.to_path_buf())
    }

    fn temporary_schema_directory(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("oxyflut-schema-{name}-{}", std::process::id()))
    }
}
