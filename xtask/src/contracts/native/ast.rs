//! Clang AST inspection and header-record metadata helpers.

use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use serde_json::Value;

use super::{NativeContractError, NativeTools, run_tool_output};

/// Derives fixture nullability from the authoritative header and its Clang AST records.
pub(super) fn derive_header_nullability(
    header: &Path,
    tools: &NativeTools,
    actual: &mut Value,
) -> Result<(), NativeContractError> {
    let source = fs::read_to_string(header)?;
    let records = parse_header_records(header, tools)?;
    if !source.contains("Unless a field comment states otherwise, every pointer passed to or returned from this ABI is nonnull. An array pointer is null if and only if its count is zero.") {
        return Err(NativeContractError::MissingNullabilityAnnotation {
            annotation: "default pointer and array rule",
        });
    }

    let actual_structs = actual
        .get_mut("structs")
        .and_then(Value::as_array_mut)
        .ok_or(NativeContractError::InvalidFixture {
            fixture: "native layout probe",
        })?;
    let mut probe_fields = std::collections::BTreeMap::new();
    for layout in actual_structs {
        let layout_object = layout
            .as_object_mut()
            .ok_or(NativeContractError::InvalidFixture {
                fixture: "native layout probe",
            })?;
        let name = layout_object
            .get("name")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or(NativeContractError::InvalidFixture {
                fixture: "native layout probe",
            })?;
        let fields = layout_object
            .get_mut("fields")
            .and_then(Value::as_array_mut)
            .ok_or(NativeContractError::InvalidFixture {
                fixture: "native layout probe",
            })?;
        let record = records
            .get(&name)
            .ok_or_else(|| NativeContractError::HeaderAstCoverageMismatch { item: name.clone() })?;
        let mut names = std::collections::BTreeSet::new();
        for layout_field in fields {
            let field_object =
                layout_field
                    .as_object_mut()
                    .ok_or(NativeContractError::InvalidFixture {
                        fixture: "native layout probe",
                    })?;
            let field_name = field_object
                .get("name")
                .and_then(Value::as_str)
                .map(str::to_owned)
                .ok_or(NativeContractError::InvalidFixture {
                    fixture: "native layout probe",
                })?;
            let field = record
                .iter()
                .find(|field| field.name == field_name)
                .ok_or_else(|| NativeContractError::HeaderAstCoverageMismatch {
                    item: format!("{name}.{field_name}"),
                })?;
            names.insert(field_name);
            field_object.insert(
                "nullability".to_owned(),
                Value::String(header_field_nullability(&source, &name, field, record)?.to_owned()),
            );
        }
        probe_fields.insert(name, names);
    }

    let header_fields = records
        .iter()
        .filter(|(_, fields)| !fields.is_empty())
        .map(|(name, fields)| {
            (
                name.clone(),
                fields.iter().map(|field| field.name.clone()).collect(),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    if probe_fields != header_fields {
        return Err(NativeContractError::HeaderAstCoverageMismatch {
            item: "layout probe fields".to_owned(),
        });
    }
    Ok(())
}

fn header_field_nullability(
    source: &str,
    record_name: &str,
    field: &HeaderField,
    record: &[HeaderField],
) -> Result<&'static str, NativeContractError> {
    if !field.is_pointer {
        return Ok("not-pointer");
    }
    if field.comment.contains("Nullable;") {
        return Ok("nullable");
    }
    if field.comment.contains("Nonnull;") {
        return Ok("nonnull");
    }
    if record_name == "OxyBorrowedBytes" && field.name == "data" {
        if source.contains("data is null if and only if length is zero.") {
            return Ok("null-if-length-zero");
        }
        return Err(NativeContractError::MissingNullabilityAnnotation {
            annotation: "OxyBorrowedBytes.data length rule",
        });
    }
    if record_name == "OxyOwnedBytes" && matches!(field.name.as_str(), "data" | "release") {
        if source.contains("Empty values have null data, zero length, and null release.") {
            return Ok("null-if-length-zero");
        }
        return Err(NativeContractError::MissingNullabilityAnnotation {
            annotation: "OxyOwnedBytes empty-value rule",
        });
    }
    if record.iter().any(|candidate| {
        candidate
            .name
            .strip_suffix("_count")
            .is_some_and(|prefix| field.name.starts_with(prefix))
    }) {
        return Ok("null-if-count-zero");
    }
    Ok("nonnull")
}

fn parse_header_records(
    header: &Path,
    tools: &NativeTools,
) -> Result<std::collections::BTreeMap<String, Vec<HeaderField>>, NativeContractError> {
    let ast = parse_header_ast(header, tools)?;
    let mut records = std::collections::BTreeMap::new();
    collect_header_records(&ast, &mut records);
    Ok(records)
}

/// Parses the authoritative C header into the Clang JSON AST.
pub(super) fn parse_header_ast(
    header: &Path,
    tools: &NativeTools,
) -> Result<Value, NativeContractError> {
    let output = run_tool_output(
        &tools.c_header_checker,
        "c-header-checker",
        "header AST inspection",
        [
            OsStr::new("-Qunused-arguments"),
            OsStr::new("-Xclang"),
            OsStr::new("-ast-dump=json"),
            OsStr::new("-fparse-all-comments"),
            OsStr::new("-fsyntax-only"),
            OsStr::new("-x"),
            OsStr::new("c"),
            OsStr::new("-std=c11"),
            header.as_os_str(),
        ],
    )?;
    serde_json::from_slice(&output.stdout).map_err(NativeContractError::Json)
}

/// Collects matching integer enum constants from one Clang JSON AST.
pub(super) fn collect_enum_constants(
    value: &Value,
    prefix: &str,
    constants: &mut std::collections::BTreeMap<String, u64>,
) {
    if value.get("kind").and_then(Value::as_str) == Some("EnumConstantDecl")
        && let Some(name) = value.get("name").and_then(Value::as_str)
        && name.starts_with(prefix)
        && let Some(raw_value) = ast_constant_value(value)
        && let Ok(parsed_value) = raw_value.parse()
    {
        constants.insert(name.to_owned(), parsed_value);
    }
    if let Some(children) = value.get("inner").and_then(Value::as_array) {
        for child in children {
            collect_enum_constants(child, prefix, constants);
        }
    }
}

fn ast_constant_value(value: &Value) -> Option<&str> {
    value.get("value").and_then(Value::as_str).or_else(|| {
        value
            .get("inner")
            .and_then(Value::as_array)
            .and_then(|children| children.iter().find_map(ast_constant_value))
    })
}

fn collect_header_records(
    value: &Value,
    records: &mut std::collections::BTreeMap<String, Vec<HeaderField>>,
) {
    if value.get("kind").and_then(Value::as_str) == Some("RecordDecl")
        && value.get("completeDefinition").and_then(Value::as_bool) == Some(true)
        && let Some(name) = value.get("name").and_then(Value::as_str)
        && name.starts_with("Oxy")
    {
        let fields = value
            .get("inner")
            .and_then(Value::as_array)
            .map(|children| {
                children
                    .iter()
                    .filter(|child| child.get("kind").and_then(Value::as_str) == Some("FieldDecl"))
                    .filter_map(header_field)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        records.insert(name.to_owned(), fields);
    }
    if let Some(children) = value.get("inner").and_then(Value::as_array) {
        for child in children {
            collect_header_records(child, records);
        }
    }
}

fn header_field(value: &Value) -> Option<HeaderField> {
    let name = value.get("name")?.as_str()?.to_owned();
    let type_name = value
        .get("type")
        .and_then(Value::as_object)
        .and_then(|value| value.get("qualType"))
        .and_then(Value::as_str)?;
    Some(HeaderField {
        name,
        is_pointer: type_name.contains('*'),
        comment: comment_text(value),
    })
}

fn comment_text(value: &Value) -> String {
    let mut text = String::new();
    collect_comment_text(value, &mut text);
    text
}

fn collect_comment_text(value: &Value, text: &mut String) {
    if let Some(comment) = value.get("text").and_then(Value::as_str) {
        text.push_str(comment);
    }
    if let Some(children) = value.get("inner").and_then(Value::as_array) {
        for child in children {
            collect_comment_text(child, text);
        }
    }
}

struct HeaderField {
    name: String,
    is_pointer: bool,
    comment: String,
}
