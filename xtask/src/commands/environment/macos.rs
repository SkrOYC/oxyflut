//! macOS reference-environment collection from bounded local command responses.

#![cfg_attr(not(any(target_os = "macos", test)), allow(dead_code))]

#[cfg(target_os = "macos")]
use std::io::Read;
#[cfg(target_os = "macos")]
use std::process::{Command, Stdio};

use oxyflut_qualification::environment::{
    EnvironmentFields, EnvironmentInventory, InventoryValue, MissingReason, SystemPackage,
    SystemPackageLock,
};
use oxyflut_qualification::identifiers::EnvironmentId;
use serde::Deserialize;
use serde_json::Value;

use super::{EnvironmentCommandError, PlatformSource};

#[cfg(target_os = "macos")]
const OUTPUT_LIMIT: usize = 4096;

/// Collects the macOS Tier 1 environment only when executing on macOS.
pub(crate) struct MacosSource;

impl PlatformSource for MacosSource {
    fn environment(&self) -> EnvironmentId {
        EnvironmentId::Macos
    }

    fn collect(&self) -> Result<EnvironmentInventory, EnvironmentCommandError> {
        #[cfg(target_os = "macos")]
        {
            collect_macos_responses(&live_responses())
        }
        #[cfg(not(target_os = "macos"))]
        {
            Err(EnvironmentCommandError::UnsupportedHost)
        }
    }
}

/// Collects a macOS inventory from one fixture's raw command responses.
#[cfg(test)]
pub(crate) fn collect_fixture_macos(
    bytes: &[u8],
) -> Result<EnvironmentInventory, EnvironmentCommandError> {
    let responses = serde_json::from_slice(bytes).map_err(EnvironmentCommandError::FixtureJson)?;
    collect_macos_responses(&responses)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct MacosResponses {
    sw_vers: Option<String>,
    uname: Option<String>,
    sysctl_model: Option<String>,
    system_profiler: Option<String>,
    compiler: Option<String>,
    sdk: Option<String>,
    compositor: Option<String>,
    session: Option<String>,
    protocol_version: Option<String>,
    package_receipts: Vec<String>,
}

fn collect_macos_responses(
    responses: &MacosResponses,
) -> Result<EnvironmentInventory, EnvironmentCommandError> {
    let fields = EnvironmentFields {
        operating_system: command_identity(responses.sw_vers.as_deref(), "macos"),
        minimum_version: InventoryValue::missing(MissingReason::NotDeclaredByLock),
        architecture: architecture(responses.uname.as_deref()),
        hardware_id: command_identity(responses.sysctl_model.as_deref(), "hardware"),
        gpu_id: gpu_identity(responses.system_profiler.as_deref()),
        driver_version: InventoryValue::missing(MissingReason::UnsupportedBySource),
        compiler_identity: compiler_identity(responses.compiler.as_deref()),
        sdk_identity: command_identity(responses.sdk.as_deref(), "macos-sdk"),
        compositor: raw_identity(responses.compositor.as_deref()),
        session: raw_identity(responses.session.as_deref()),
        protocol_version: raw_identity(responses.protocol_version.as_deref()),
        system_package_lock: package_lock(&responses.package_receipts),
    };
    EnvironmentInventory::new(EnvironmentId::Macos, fields)
        .map_err(EnvironmentCommandError::Inventory)
}

fn architecture(raw: Option<&str>) -> InventoryValue {
    let Some(value) = raw.and_then(first_line) else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let normalized = match value {
        "aarch64" | "arm64" => "aarch64",
        "x86_64" | "amd64" => "x86_64",
        other => other,
    };
    observed_or_missing(normalized.to_owned())
}

fn gpu_identity(raw: Option<&str>) -> InventoryValue {
    let Some(raw) = raw else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Ok(profile) = serde_json::from_str::<Value>(raw) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let Some(model) = profile
        .pointer("/SPDisplaysDataType/0/sppci_model")
        .or_else(|| profile.pointer("/SPDisplaysDataType/0/_name"))
        .and_then(Value::as_str)
    else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    observed_or_missing(format!("gpu-{}", atomize(model)))
}

fn compiler_identity(raw: Option<&str>) -> InventoryValue {
    let Some(raw) = raw else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Some(version) = raw
        .split_whitespace()
        .find(|word| word.bytes().any(|byte| byte.is_ascii_digit()))
    else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    observed_or_missing(format!("apple-clang-{}", atomize(version)))
}

fn command_identity(raw: Option<&str>, prefix: &str) -> InventoryValue {
    let Some(value) = raw.and_then(first_line) else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    observed_or_missing(format!("{prefix}-{}", atomize(value)))
}

fn raw_identity(raw: Option<&str>) -> InventoryValue {
    raw.and_then(first_line).map_or_else(
        || InventoryValue::missing(MissingReason::SourceUnavailable),
        |value| observed_or_missing(atomize(value)),
    )
}

fn package_lock(receipts: &[String]) -> SystemPackageLock {
    if receipts.is_empty() {
        return SystemPackageLock::missing(MissingReason::SourceUnavailable);
    }
    let records = receipts
        .iter()
        .map(|receipt| {
            package_fields(receipt)
                .ok_or(MissingReason::UnsupportedBySource)
                .and_then(|(name, version)| {
                    SystemPackage::new(name, version)
                        .map_err(|_| MissingReason::UnsupportedBySource)
                })
        })
        .collect::<Result<Vec<_>, _>>();
    match records.and_then(|records| {
        SystemPackageLock::from_records(records).map_err(|_| MissingReason::UnsupportedBySource)
    }) {
        Ok(lock) => lock,
        Err(reason) => SystemPackageLock::missing(reason),
    }
}

fn package_fields(raw: &str) -> Option<(String, String)> {
    let (name, version) = raw.lines().next()?.split_once('\t')?;
    Some((name.to_owned(), version.to_owned()))
}

fn first_line(value: &str) -> Option<&str> {
    value
        .lines()
        .next()
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn atomize(value: &str) -> String {
    value
        .bytes()
        .filter_map(|byte| {
            (byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_' | b':' | b'+'))
                .then_some(byte)
                .or_else(|| byte.is_ascii_whitespace().then_some(b'-'))
        })
        .map(char::from)
        .collect()
}

fn observed_or_missing(value: String) -> InventoryValue {
    match InventoryValue::observed(value) {
        Ok(value) => value,
        Err(_) => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

#[cfg(target_os = "macos")]
fn live_responses() -> MacosResponses {
    MacosResponses {
        sw_vers: command_stdout("sw_vers", &["-productVersion"]),
        uname: command_stdout("uname", &["-m"]),
        sysctl_model: command_stdout("sysctl", &["-n", "hw.model"]),
        system_profiler: command_stdout("system_profiler", &["SPDisplaysDataType", "-json"]),
        compiler: command_stdout("xcrun", &["--sdk", "macosx", "clang", "--version"]),
        sdk: command_stdout("xcrun", &["--sdk", "macosx", "--show-sdk-version"]),
        compositor: None,
        session: None,
        protocol_version: None,
        package_receipts: Vec::new(),
    }
}

#[cfg(target_os = "macos")]
fn command_stdout(program: &str, arguments: &[&str]) -> Option<String> {
    let mut child = Command::new(program)
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let stdout = child.stdout.take()?;
    let capture_limit = OUTPUT_LIMIT.checked_add(1)?;
    let mut output = Vec::with_capacity(capture_limit);
    let mut stdout = stdout.take(u64::try_from(capture_limit).ok()?);
    stdout.read_to_end(&mut output).ok()?;
    if output.len() > OUTPUT_LIMIT {
        let _ = child.kill();
        let _ = child.wait();
        return None;
    }
    if !child.wait().ok()?.success() {
        return None;
    }
    String::from_utf8(output).ok()
}
