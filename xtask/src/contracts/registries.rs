//! Diagnostic registry and machine-local sink validation.
//!
//! The registry is the sole authority for event and field privacy metadata. Event files carry only values and can never widen, narrow, or restate those classifications.

use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use oxyflut_qualification::identifiers::{CandidateId, EnvironmentId, EventName, RegistryVersion};
use serde_json::Value;
use thiserror::Error;

const REGISTRY_PATH: &str = ".constitution/tech-spec/contracts/diagnostic-event-registry.json";
const REGISTRY_VERSION: &str = "2.0.0";
const MAX_QUEUE_RECORDS: u32 = 65_536;
const DIAGNOSTIC_EVENTS: [&str; 13] = [
    "runtime.lifecycle",
    "view.lifecycle",
    "frame.scheduled",
    "frame.presented",
    "frame.missed",
    "input.rejected",
    "text.transaction",
    "semantics.update",
    "semantics.action",
    "asset.lifecycle",
    "recovery.transition",
    "boundary.failure",
    "diagnostics.dropped",
];

/// A validated, bounded machine-local diagnostic sink admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LocalSinkAdmission {
    destination: LocalSinkDestination,
    maximum_queued_records: u32,
}

/// A closed machine-local diagnostic destination.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LocalSinkDestination {
    /// A machine-local file selected by the user.
    SelectedMachineLocalFile,
    /// An application-local file explicitly enabled by the user.
    ApplicationLocalFile,
    /// An in-process buffer explicitly enabled by the user.
    MemoryBuffer,
}

impl LocalSinkDestination {
    fn parse(value: &str) -> Result<Self, RegistryError> {
        match value {
            "user-selected-machine-local-file" => Ok(Self::SelectedMachineLocalFile),
            "user-enabled-application-local-file" => Ok(Self::ApplicationLocalFile),
            "user-enabled-memory-buffer" => Ok(Self::MemoryBuffer),
            _ => Err(RegistryError::Invariant {
                code: "diagnostic-sink-destination",
            }),
        }
    }
}

/// Errors from diagnostic registry validation.
#[derive(Debug, Error)]
pub(crate) enum RegistryError {
    /// A required local input could not be read.
    #[error("could not read local diagnostic input")]
    Io {
        /// The input path.
        path: PathBuf,
        /// The underlying local I/O failure.
        #[source]
        source: io::Error,
    },
    /// A required local input was not valid JSON.
    #[error("could not parse local diagnostic input")]
    Json {
        /// The input path.
        path: PathBuf,
        /// The JSON parsing failure.
        #[source]
        source: serde_json::Error,
    },
    /// A semantic registry invariant failed.
    #[error("diagnostic registry invariant failed: {code}")]
    Invariant {
        /// Stable, content-free invariant code.
        code: &'static str,
    },
}

impl RegistryError {
    #[cfg(test)]
    fn code(&self) -> Option<&'static str> {
        match self {
            Self::Invariant { code } => Some(code),
            Self::Io { .. } | Self::Json { .. } => None,
        }
    }
}

/// Validates the committed diagnostic-event registry.
///
/// # Errors
///
/// Returns an error when the local registry cannot be read or violates its closed semantic contract.
pub(crate) fn validate_workspace(root: &Path) -> Result<(), RegistryError> {
    // Event records and sink admissions are validated as their durable inputs arrive in later families.
    let _ = (
        validate_event,
        admit_local_sink,
        validate_candidate_environment,
    );
    let registry = read_json(&root.join(REGISTRY_PATH))?;
    validate_registry(&registry)
}

/// Validates the closed diagnostic event registry.
///
/// # Errors
///
/// Returns an error when a registry version, event, field, privacy class, kind, range, or closed value declaration is invalid.
pub(crate) fn validate_registry(registry: &Value) -> Result<(), RegistryError> {
    let version = string_field(registry, "schemaVersion")?;
    RegistryVersion::parse(version).map_err(|_| RegistryError::Invariant {
        code: "diagnostic-registry-version",
    })?;
    if version != REGISTRY_VERSION {
        return Err(RegistryError::Invariant {
            code: "diagnostic-registry-version",
        });
    }

    let events = object_field(registry, "events")?;
    let actual_events = events.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected_events = DIAGNOSTIC_EVENTS.iter().copied().collect::<BTreeSet<_>>();
    if actual_events != expected_events {
        return Err(RegistryError::Invariant {
            code: "diagnostic-event-set",
        });
    }
    for (name, event) in events {
        EventName::parse(name).map_err(|_| RegistryError::Invariant {
            code: "diagnostic-event-name",
        })?;
        let privacy = string_field(event, "privacyClass")?;
        validate_privacy_class(privacy)?;
        let fields = object_field(event, "fields")?;
        for (field_name, field) in fields {
            if !is_field_name(field_name) {
                return Err(RegistryError::Invariant {
                    code: "diagnostic-field-name",
                });
            }
            validate_field_definition(field)?;
        }
    }
    Ok(())
}

/// Validates one durable diagnostic event against the authoritative registry.
///
/// # Errors
///
/// Returns an error when the event uses an unknown registry version, event, field, value kind, value range, closed value, or privacy override.
pub(crate) fn validate_event(registry: &Value, event: &Value) -> Result<(), RegistryError> {
    validate_registry(registry)?;
    let version = string_field(event, "registryVersion")?;
    RegistryVersion::parse(version).map_err(|_| RegistryError::Invariant {
        code: "diagnostic-event-registry-version",
    })?;
    if version != string_field(registry, "schemaVersion")? {
        return Err(RegistryError::Invariant {
            code: "diagnostic-event-registry-version",
        });
    }

    let name = string_field(event, "name")?;
    let event_name = EventName::parse(name).map_err(|_| RegistryError::Invariant {
        code: "diagnostic-event-name",
    })?;
    let registry_event = object_field(registry, "events")?
        .get(event_name.as_str())
        .ok_or(RegistryError::Invariant {
            code: "diagnostic-event-unregistered",
        })?;
    let registry_fields = object_field(registry_event, "fields")?;
    let fields = object_field(event, "fields")?;

    if event.get("privacyClass").is_some() {
        return Err(RegistryError::Invariant {
            code: "diagnostic-event-privacy-override",
        });
    }
    for (field_name, field_value) in fields {
        let definition = registry_fields
            .get(field_name)
            .ok_or(RegistryError::Invariant {
                code: "diagnostic-field-unregistered",
            })?;
        validate_event_field_value(definition, field_value)?;
    }
    Ok(())
}

/// Admits one closed machine-local sink before any record delivery.
///
/// # Errors
///
/// Returns an error for remote or undeclared destinations and for zero or unbounded queue acknowledgements.
pub(crate) fn admit_local_sink(
    destination: &str,
    maximum_queued_records: Option<u32>,
) -> Result<LocalSinkAdmission, RegistryError> {
    let destination = LocalSinkDestination::parse(destination)?;
    let maximum_queued_records = maximum_queued_records.ok_or(RegistryError::Invariant {
        code: "diagnostic-sink-acknowledgement",
    })?;
    if maximum_queued_records == 0 || maximum_queued_records > MAX_QUEUE_RECORDS {
        return Err(RegistryError::Invariant {
            code: "diagnostic-sink-acknowledgement",
        });
    }
    Ok(LocalSinkAdmission {
        destination,
        maximum_queued_records,
    })
}

/// Confirms the cross-family closed candidate and Tier 1 environment sets.
///
/// This function keeps registry fixtures from accepting arbitrary candidate or environment strings.
///
/// # Errors
///
/// Returns an error when a candidate or environment is not an admitted identifier.
pub(crate) fn validate_candidate_environment(
    candidate: &str,
    environment: &str,
) -> Result<(), RegistryError> {
    candidate
        .parse::<CandidateId>()
        .map_err(|_| RegistryError::Invariant {
            code: "candidate-identifier",
        })?;
    environment
        .parse::<EnvironmentId>()
        .map_err(|_| RegistryError::Invariant {
            code: "environment-identifier",
        })?;
    Ok(())
}

fn validate_field_definition(field: &Value) -> Result<(), RegistryError> {
    validate_privacy_class(string_field(field, "privacyClass")?)?;
    let kind = string_field(field, "kind")?;
    if !matches!(kind, "boolean" | "integer" | "number") {
        return Err(RegistryError::Invariant {
            code: "diagnostic-field-kind",
        });
    }
    if field.get("enumValues").is_some() {
        if kind != "integer" {
            return Err(RegistryError::Invariant {
                code: "diagnostic-field-closed-values",
            });
        }
        let values = array_field(field, "enumValues")?;
        if values.is_empty() || values.iter().any(|value| value.as_i64().is_none()) {
            return Err(RegistryError::Invariant {
                code: "diagnostic-field-closed-values",
            });
        }
    }
    let minimum = number_field_optional(field, "minimum")?;
    let maximum = number_field_optional(field, "maximum")?;
    if minimum
        .zip(maximum)
        .is_some_and(|(minimum, maximum)| minimum > maximum)
    {
        return Err(RegistryError::Invariant {
            code: "diagnostic-field-range",
        });
    }
    Ok(())
}

fn validate_event_field_value(
    definition: &Value,
    event_value: &Value,
) -> Result<(), RegistryError> {
    let object = event_value.as_object().ok_or(RegistryError::Invariant {
        code: "diagnostic-field-value",
    })?;
    if object.len() != 1 || !object.contains_key("value") {
        return Err(RegistryError::Invariant {
            code: "diagnostic-field-privacy-override",
        });
    }
    let value = object.get("value").ok_or(RegistryError::Invariant {
        code: "diagnostic-field-value",
    })?;
    match string_field(definition, "kind")? {
        "boolean" if value.is_boolean() => {}
        "integer" if value.as_i64().is_some() || value.as_u64().is_some() => {}
        "number" if value.is_number() => {}
        "boolean" | "integer" | "number" => {
            return Err(RegistryError::Invariant {
                code: "diagnostic-field-kind",
            });
        }
        _ => {
            return Err(RegistryError::Invariant {
                code: "diagnostic-field-kind",
            });
        }
    }

    if let Some(allowed) = definition.get("enumValues") {
        let Some(integer) = value.as_i64() else {
            return Err(RegistryError::Invariant {
                code: "diagnostic-field-closed-values",
            });
        };
        let values = allowed.as_array().ok_or(RegistryError::Invariant {
            code: "diagnostic-field-closed-values",
        })?;
        if !values
            .iter()
            .any(|allowed| allowed.as_i64() == Some(integer))
        {
            return Err(RegistryError::Invariant {
                code: "diagnostic-field-closed-values",
            });
        }
    }

    let number = value.as_f64().ok_or(RegistryError::Invariant {
        code: "diagnostic-field-value",
    })?;
    if number_field_optional(definition, "minimum")?.is_some_and(|minimum| number < minimum)
        || number_field_optional(definition, "maximum")?.is_some_and(|maximum| number > maximum)
    {
        return Err(RegistryError::Invariant {
            code: "diagnostic-field-range",
        });
    }
    Ok(())
}

fn validate_privacy_class(value: &str) -> Result<(), RegistryError> {
    if matches!(value, "public" | "operational" | "private-redacted") {
        Ok(())
    } else {
        Err(RegistryError::Invariant {
            code: "diagnostic-privacy-class",
        })
    }
}

fn is_field_name(value: &str) -> bool {
    let mut bytes = value.bytes();
    matches!(bytes.next(), Some(first) if first.is_ascii_lowercase())
        && bytes.all(|byte| byte.is_ascii_alphanumeric())
}

fn read_json(path: &Path) -> Result<Value, RegistryError> {
    let bytes = fs::read(path).map_err(|source| RegistryError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    serde_json::from_slice(&bytes).map_err(|source| RegistryError::Json {
        path: path.to_path_buf(),
        source,
    })
}

fn object_field<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a serde_json::Map<String, Value>, RegistryError> {
    value
        .get(field)
        .and_then(Value::as_object)
        .ok_or(RegistryError::Invariant {
            code: "diagnostic-required-field",
        })
}

fn array_field<'a>(value: &'a Value, field: &str) -> Result<&'a Vec<Value>, RegistryError> {
    value
        .get(field)
        .and_then(Value::as_array)
        .ok_or(RegistryError::Invariant {
            code: "diagnostic-required-field",
        })
}

fn string_field<'a>(value: &'a Value, field: &str) -> Result<&'a str, RegistryError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or(RegistryError::Invariant {
            code: "diagnostic-required-field",
        })
}

fn number_field_optional(value: &Value, field: &str) -> Result<Option<f64>, RegistryError> {
    match value.get(field) {
        Some(value) => value.as_f64().map(Some).ok_or(RegistryError::Invariant {
            code: "diagnostic-field-range",
        }),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::path::{Path, PathBuf};

    use serde_json::{Value, json};

    use super::{
        LocalSinkDestination, RegistryError, admit_local_sink, validate_candidate_environment,
        validate_event, validate_workspace,
    };

    #[test]
    fn committed_registry_is_closed_and_valid() -> Result<(), Box<dyn Error>> {
        validate_workspace(&workspace_root()?)?;
        Ok(())
    }

    #[test]
    fn diagnostic_values_resolve_the_registered_kind_bounds_and_closed_values()
    -> Result<(), Box<dyn Error>> {
        let registry = registry_fixture();
        let valid = json!({
            "registryVersion": "2.0.0",
            "name": "runtime.lifecycle",
            "fields": { "state": { "value": 1 } }
        });
        validate_event(&registry, &valid)?;

        let invalid_closed = json!({
            "registryVersion": "2.0.0",
            "name": "runtime.lifecycle",
            "fields": { "state": { "value": 9 } }
        });
        assert_code(
            validate_event(&registry, &invalid_closed),
            "diagnostic-field-closed-values",
        );
        let invalid_kind = json!({
            "registryVersion": "2.0.0",
            "name": "runtime.lifecycle",
            "fields": { "state": { "value": true } }
        });
        assert_code(
            validate_event(&registry, &invalid_kind),
            "diagnostic-field-kind",
        );
        Ok(())
    }

    #[test]
    fn event_files_cannot_override_registry_privacy_metadata() -> Result<(), Box<dyn Error>> {
        let event = json!({
            "registryVersion": "2.0.0",
            "name": "runtime.lifecycle",
            "privacyClass": "public",
            "fields": { "state": { "value": 1, "privacyClass": "public" } }
        });
        assert_code(
            validate_event(&registry_fixture(), &event),
            "diagnostic-event-privacy-override",
        );
        Ok(())
    }

    #[test]
    fn only_closed_machine_local_sinks_have_a_nonzero_bounded_acknowledgement()
    -> Result<(), Box<dyn Error>> {
        let admission = admit_local_sink("user-enabled-memory-buffer", Some(1))?;
        assert_eq!(admission.destination, LocalSinkDestination::MemoryBuffer);
        assert_eq!(admission.maximum_queued_records, 1);
        assert_code(
            admit_local_sink("remote-exporter", Some(1)),
            "diagnostic-sink-destination",
        );
        assert_code(
            admit_local_sink("user-enabled-memory-buffer", None),
            "diagnostic-sink-acknowledgement",
        );
        assert_code(
            admit_local_sink("user-enabled-memory-buffer", Some(0)),
            "diagnostic-sink-acknowledgement",
        );
        assert_code(
            admit_local_sink("user-enabled-memory-buffer", Some(65_537)),
            "diagnostic-sink-acknowledgement",
        );
        Ok(())
    }

    #[test]
    fn candidate_and_environment_identifiers_are_closed() {
        assert!(validate_candidate_environment("focused", "macos").is_ok());
        assert!(validate_candidate_environment("synthetic", "macos").is_err());
        assert!(validate_candidate_environment("focused", "remote").is_err());
    }

    fn registry_fixture() -> Value {
        json!({
            "schemaVersion": "2.0.0",
            "events": {
                "runtime.lifecycle": {
                    "privacyClass": "operational",
                    "fields": {
                        "state": {
                            "privacyClass": "operational",
                            "kind": "integer",
                            "enumValues": [1, 2]
                        }
                    }
                },
                "view.lifecycle": { "privacyClass": "operational", "fields": {} },
                "frame.scheduled": { "privacyClass": "operational", "fields": {} },
                "frame.presented": { "privacyClass": "operational", "fields": {} },
                "frame.missed": { "privacyClass": "operational", "fields": {} },
                "input.rejected": { "privacyClass": "private-redacted", "fields": {} },
                "text.transaction": { "privacyClass": "private-redacted", "fields": {} },
                "semantics.update": { "privacyClass": "private-redacted", "fields": {} },
                "semantics.action": { "privacyClass": "private-redacted", "fields": {} },
                "asset.lifecycle": { "privacyClass": "private-redacted", "fields": {} },
                "recovery.transition": { "privacyClass": "operational", "fields": {} },
                "boundary.failure": { "privacyClass": "operational", "fields": {} },
                "diagnostics.dropped": { "privacyClass": "operational", "fields": {} }
            }
        })
    }

    fn assert_code<T>(result: Result<T, RegistryError>, expected: &'static str) {
        assert!(matches!(result, Err(error) if error.code() == Some(expected)));
    }

    fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| "xtask must remain directly below the workspace root".into())
    }
}
