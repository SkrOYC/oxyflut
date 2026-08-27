//! Deterministic JSON encoding for derived evidence.

use serde_json::Value;

use super::EvidenceError;

/// Encodes one JSON value as deterministic UTF-8 JSON with sorted keys, fixed pinned number formatting, and a trailing line feed.
///
/// # Errors
///
/// Returns an error only if a JSON string cannot be encoded.
pub fn canonical_json_bytes(value: &Value) -> Result<Vec<u8>, EvidenceError> {
    let mut output = Vec::new();
    write_canonical_json_value(&mut output, value)?;
    output.push(b'\n');
    Ok(output)
}

pub(super) fn write_canonical_json_value(
    output: &mut Vec<u8>,
    value: &Value,
) -> Result<(), EvidenceError> {
    match value {
        Value::Null => output.extend_from_slice(b"null"),
        Value::Bool(true) => output.extend_from_slice(b"true"),
        Value::Bool(false) => output.extend_from_slice(b"false"),
        Value::Number(number) => write_canonical_number(output, number),
        Value::String(string) => serde_json::to_writer(&mut *output, string)
            .map_err(|source| EvidenceError::JsonEncoding { source })?,
        Value::Array(values) => {
            output.push(b'[');
            for (index, item) in values.iter().enumerate() {
                if index > 0 {
                    output.push(b',');
                }
                write_canonical_json_value(output, item)?;
            }
            output.push(b']');
        }
        Value::Object(values) => {
            let mut entries = values.iter().collect::<Vec<_>>();
            entries.sort_unstable_by(|left, right| left.0.cmp(right.0));
            output.push(b'{');
            for (index, (key, item)) in entries.into_iter().enumerate() {
                if index > 0 {
                    output.push(b',');
                }
                serde_json::to_writer(&mut *output, key)
                    .map_err(|source| EvidenceError::JsonEncoding { source })?;
                output.push(b':');
                write_canonical_json_value(output, item)?;
            }
            output.push(b'}');
        }
    }
    Ok(())
}

fn write_canonical_number(output: &mut Vec<u8>, number: &serde_json::Number) {
    if let Some(integer) = number.as_i64() {
        output.extend_from_slice(integer.to_string().as_bytes());
    } else if let Some(integer) = number.as_u64() {
        output.extend_from_slice(integer.to_string().as_bytes());
    } else if let Some(float) = number.as_f64() {
        if float == 0.0 {
            output.push(b'0');
        } else {
            output.extend_from_slice(float.to_string().as_bytes());
        }
    }
}
