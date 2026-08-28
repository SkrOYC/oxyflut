//! Strict local DSSE pre-authentication encoding and fixture-signature verification.

use std::io::Cursor;

use oxyflut_qualification::hash::hash_reader;
use serde_json::Value;

use super::ExternalContractsError;

pub(super) const PAYLOAD_TYPE: &str = "application/vnd.in-toto+json";
pub(super) const TEST_KEY_ID: &str = "oxyflut-fixture-sha256-test-key-v1";
pub(super) const TEST_ALGORITHM: &str = "OXYFLUT-TEST-SHA256-KEYED-V1";
const PAE_PREFIX: &[u8] = b"DSSEv1";

pub(super) struct TestKey {
    pub(super) key_id: String,
    pub(super) algorithm: String,
    pub(super) key: String,
    pub(super) purpose: String,
}

pub(super) fn pae(payload_type: &[u8], payload: &[u8]) -> Result<Vec<u8>, ExternalContractsError> {
    let payload_type_length = payload_type.len().to_string();
    let payload_length = payload.len().to_string();
    let capacity = PAE_PREFIX
        .len()
        .checked_add(1)
        .and_then(|value| value.checked_add(payload_type_length.len()))
        .and_then(|value| value.checked_add(1))
        .and_then(|value| value.checked_add(payload_type.len()))
        .and_then(|value| value.checked_add(1))
        .and_then(|value| value.checked_add(payload_length.len()))
        .and_then(|value| value.checked_add(1))
        .and_then(|value| value.checked_add(payload.len()))
        .ok_or(ExternalContractsError::Pae)?;
    let mut pae = Vec::with_capacity(capacity);
    pae.extend_from_slice(PAE_PREFIX);
    pae.push(b' ');
    pae.extend_from_slice(payload_type_length.as_bytes());
    pae.push(b' ');
    pae.extend_from_slice(payload_type);
    pae.push(b' ');
    pae.extend_from_slice(payload_length.as_bytes());
    pae.push(b' ');
    pae.extend_from_slice(payload);
    Ok(pae)
}

pub(super) fn verify_fixture_signature(
    signature: &Value,
    pae: &[u8],
    key: &TestKey,
) -> Result<bool, ExternalContractsError> {
    if key.key_id != TEST_KEY_ID
        || key.algorithm != TEST_ALGORITHM
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

pub(super) fn decode_base64(input: &str) -> Option<Vec<u8>> {
    let bytes = input.as_bytes();
    if bytes.is_empty() || !bytes.len().is_multiple_of(4) {
        return None;
    }
    let padding = bytes.iter().rev().take_while(|byte| **byte == b'=').count();
    if padding > 2 || bytes[..bytes.len().checked_sub(padding)?].contains(&b'=') {
        return None;
    }
    let raw = &bytes[..bytes.len().checked_sub(padding)?];
    match (padding, raw.len() % 4) {
        (0, 0) | (1, 3) | (2, 2) => {}
        _ => return None,
    }

    let capacity = raw.len().checked_mul(3)?.checked_div(4)?.checked_add(2)?;
    let mut decoded = Vec::with_capacity(capacity);
    let (groups, remainder) = raw.as_chunks::<4>();
    for [first, second, third, fourth] in groups {
        let first = standard_base64_value(*first)?;
        let second = standard_base64_value(*second)?;
        let third = standard_base64_value(*third)?;
        let fourth = standard_base64_value(*fourth)?;
        decoded.push((first << 2) | (second >> 4));
        decoded.push((second << 4) | (third >> 2));
        decoded.push((third << 6) | fourth);
    }
    match remainder {
        [] => Some(decoded),
        [first, second] if padding == 2 => {
            let first = standard_base64_value(*first)?;
            let second = standard_base64_value(*second)?;
            if second & 0x0F != 0 {
                return None;
            }
            decoded.push((first << 2) | (second >> 4));
            Some(decoded)
        }
        [first, second, third] if padding == 1 => {
            let first = standard_base64_value(*first)?;
            let second = standard_base64_value(*second)?;
            let third = standard_base64_value(*third)?;
            if third & 0x03 != 0 {
                return None;
            }
            decoded.push((first << 2) | (second >> 4));
            decoded.push((second << 4) | (third >> 2));
            Some(decoded)
        }
        [_] | [_, _] | [_, _, _] | [_, _, _, _] | [_, _, _, _, ..] => None,
    }
}

const fn standard_base64_value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}
