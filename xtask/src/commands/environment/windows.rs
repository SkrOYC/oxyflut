//! Windows reference-environment collection from bounded PowerShell CIM responses.

#![cfg_attr(not(any(target_os = "windows", test)), allow(dead_code))]

#[cfg(target_os = "windows")]
use std::io::Read;
#[cfg(target_os = "windows")]
use std::process::{Command, Stdio};

use oxyflut_qualification::environment::{
    EnvironmentFields, EnvironmentInventory, InventoryValue, MissingReason, SystemPackage,
    SystemPackageLock,
};
use oxyflut_qualification::identifiers::EnvironmentId;
use serde::Deserialize;
use serde_json::Value;

use super::{EnvironmentCommandError, PlatformSource};

#[cfg(target_os = "windows")]
const OUTPUT_LIMIT: usize = 4096;

/// Collects the Windows Tier 1 environment only when executing on Windows.
pub(crate) struct WindowsSource;

impl PlatformSource for WindowsSource {
    fn environment(&self) -> EnvironmentId {
        EnvironmentId::Windows
    }

    fn collect(&self) -> Result<EnvironmentInventory, EnvironmentCommandError> {
        #[cfg(target_os = "windows")]
        {
            collect_windows_responses(&live_responses())
        }
        #[cfg(not(target_os = "windows"))]
        {
            Err(EnvironmentCommandError::UnsupportedHost)
        }
    }
}

/// Collects a Windows inventory from one fixture's raw PowerShell responses.
#[cfg(test)]
pub(crate) fn collect_fixture_windows(
    bytes: &[u8],
) -> Result<EnvironmentInventory, EnvironmentCommandError> {
    let responses = serde_json::from_slice(bytes).map_err(EnvironmentCommandError::FixtureJson)?;
    collect_windows_responses(&responses)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct WindowsResponses {
    operating_system: Option<String>,
    processor: Option<String>,
    computer_system: Option<String>,
    video_controller: Option<String>,
    compiler: Option<String>,
    sdk: Option<String>,
    compositor: Option<String>,
    session: Option<String>,
    protocol_version: Option<String>,
    package_receipts: Vec<String>,
}

fn collect_windows_responses(
    responses: &WindowsResponses,
) -> Result<EnvironmentInventory, EnvironmentCommandError> {
    let fields = EnvironmentFields {
        operating_system: json_identity(
            responses.operating_system.as_deref(),
            "Version",
            "windows",
        ),
        minimum_version: InventoryValue::missing(MissingReason::NotDeclaredByLock),
        architecture: architecture(responses.processor.as_deref()),
        hardware_id: json_identity(responses.computer_system.as_deref(), "Model", "hardware"),
        gpu_id: gpu_identity(responses.video_controller.as_deref()),
        driver_version: json_identity(
            responses.video_controller.as_deref(),
            "DriverVersion",
            "driver",
        ),
        compiler_identity: command_identity(responses.compiler.as_deref(), "msvc"),
        sdk_identity: json_identity(responses.sdk.as_deref(), "Version", "windows-sdk"),
        compositor: raw_identity(responses.compositor.as_deref()),
        session: raw_identity(responses.session.as_deref()),
        protocol_version: raw_identity(responses.protocol_version.as_deref()),
        system_package_lock: package_lock(&responses.package_receipts),
    };
    EnvironmentInventory::new(EnvironmentId::Windows, fields)
        .map_err(EnvironmentCommandError::Inventory)
}

fn architecture(raw: Option<&str>) -> InventoryValue {
    let Some(raw) = raw else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Ok(value) = serde_json::from_str::<Value>(raw) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let value = first_json_object(&value)
        .and_then(|object| object.get("Architecture"))
        .and_then(Value::as_u64);
    match value {
        Some(9) => observed_or_missing("x86_64".to_owned()),
        Some(12) => observed_or_missing("aarch64".to_owned()),
        Some(_) => InventoryValue::missing(MissingReason::UnsupportedBySource),
        None => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

fn gpu_identity(raw: Option<&str>) -> InventoryValue {
    let Some(value) = json_string(raw, "PNPDeviceID") else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Some((bus, pnp_id)) = value.split_once('\\') else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    if !bus.eq_ignore_ascii_case("PCI") {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    }
    let vendor = pnp_id.split('&').find(|part| part.starts_with("VEN_"));
    let device = pnp_id.split('&').find(|part| part.starts_with("DEV_"));
    match (vendor, device) {
        (Some(vendor), Some(device)) => observed_or_missing(format!("pci-{vendor}-{device}")),
        _ => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

fn json_identity(raw: Option<&str>, field: &str, prefix: &str) -> InventoryValue {
    json_string(raw, field).map_or_else(
        || InventoryValue::missing(MissingReason::SourceUnavailable),
        |value| observed_or_missing(format!("{prefix}-{}", atomize(&value))),
    )
}

fn command_identity(raw: Option<&str>, prefix: &str) -> InventoryValue {
    let Some(raw) = raw else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Some(version) = raw
        .split_whitespace()
        .find(|word| word.bytes().any(|byte| byte.is_ascii_digit()))
    else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    observed_or_missing(format!("{prefix}-{}", atomize(version)))
}

fn raw_identity(raw: Option<&str>) -> InventoryValue {
    raw.and_then(first_line).map_or_else(
        || InventoryValue::missing(MissingReason::SourceUnavailable),
        |value| observed_or_missing(atomize(value)),
    )
}

fn json_string(raw: Option<&str>, field: &str) -> Option<String> {
    let raw = raw?;
    let value = serde_json::from_str::<Value>(raw).ok()?;
    first_json_object(&value)?
        .get(field)?
        .as_str()
        .map(str::to_owned)
}

fn first_json_object(value: &Value) -> Option<&serde_json::Map<String, Value>> {
    match value {
        Value::Object(object) => Some(object),
        Value::Array(values) => values.first().and_then(Value::as_object),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => None,
    }
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

#[cfg(target_os = "windows")]
fn live_responses() -> WindowsResponses {
    WindowsResponses {
        operating_system: powershell_json(
            "Get-CimInstance Win32_OperatingSystem | Select-Object Version | ConvertTo-Json -Compress",
        ),
        processor: powershell_json(
            "Get-CimInstance Win32_Processor | Select-Object -First 1 Architecture | ConvertTo-Json -Compress",
        ),
        computer_system: powershell_json(
            "Get-CimInstance Win32_ComputerSystem | Select-Object Model | ConvertTo-Json -Compress",
        ),
        video_controller: powershell_json(
            "Get-CimInstance Win32_VideoController | Select-Object -First 1 PNPDeviceID,DriverVersion | ConvertTo-Json -Compress",
        ),
        compiler: command_stdout("cl", &["/Bv"]),
        sdk: None,
        compositor: None,
        session: None,
        protocol_version: None,
        package_receipts: Vec::new(),
    }
}

#[cfg(target_os = "windows")]
fn powershell_json(script: &str) -> Option<String> {
    command_stdout(
        "powershell",
        &["-NoProfile", "-NonInteractive", "-Command", script],
    )
}

#[cfg(target_os = "windows")]
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
