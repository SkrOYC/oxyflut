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
const OUTPUT_LIMIT: usize = 16 * 1024;

// These exact product identities implement the Visual Studio Build Tools 2022 17.14.39 and
// Windows SDK 10.0.26100.8876 pins in stack.md. The package query is filtered to this set before
// its output is bounded.
const WINDOWS_PACKAGE_REQUIREMENTS: &[&str] =
    &["Microsoft.VisualStudio.BuildTools", "Microsoft.WindowsSDK"];

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
            Err(EnvironmentCommandError::UnsupportedHost {
                reason: MissingReason::UnavailableOnHost,
            })
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
    pnp_signed_driver: Option<String>,
    compiler: Option<CompilerResponse>,
    compiler_env: Option<String>,
    compiler_vswhere: Option<String>,
    sdk: Option<String>,
    rust_toolchain: Option<String>,
    package_catalog: Option<String>,
}

/// A captured compiler banner and its process exit status.
///
/// `cl /Bv` writes a usable banner while returning a nonzero status when no source file is given.
/// The collector deliberately parses that banner regardless of its status and reserves process
/// success checks for commands whose output requires one.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CompilerResponse {
    /// Compatibility with manually captured fixtures that record only output.
    Output(String),
    /// A raw command result that retains the exit status alongside the banner.
    Command {
        stdout: String,
        #[serde(rename = "exitCode")]
        _exit_code: Option<i32>,
    },
}

impl CompilerResponse {
    fn banner(&self) -> &str {
        match self {
            Self::Output(stdout) | Self::Command { stdout, .. } => stdout,
        }
    }
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
        driver_version: driver_identity(
            responses.video_controller.as_deref(),
            responses.pnp_signed_driver.as_deref(),
        ),
        compiler_identity: compiler_identity(
            responses.compiler.as_ref(),
            responses.compiler_env.as_deref(),
            responses.compiler_vswhere.as_deref(),
        ),
        sdk_identity: json_identity(responses.sdk.as_deref(), "Version", "windows-sdk"),
        rust_toolchain: rust_toolchain_identity(responses.rust_toolchain.as_deref()),
        compositor: InventoryValue::missing(MissingReason::ManualCapture),
        session: InventoryValue::missing(MissingReason::ManualCapture),
        protocol_version: InventoryValue::missing(MissingReason::ManualCapture),
        system_package_lock: package_lock(responses.package_catalog.as_deref()),
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
    let vendor = pnp_id.split('&').find_map(|part| part.strip_prefix("VEN_"));
    let device = pnp_id.split('&').find_map(|part| part.strip_prefix("DEV_"));
    match (vendor, device) {
        (Some(vendor), Some(device)) if is_hex4(vendor) && is_hex4(device) => {
            observed_or_missing(format!(
                "pci:{}:{}",
                vendor.to_ascii_lowercase(),
                device.to_ascii_lowercase()
            ))
        }
        _ => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

fn json_identity(raw: Option<&str>, field: &str, prefix: &str) -> InventoryValue {
    json_string(raw, field).map_or_else(
        || InventoryValue::missing(MissingReason::SourceUnavailable),
        |value| observed_or_missing(format!("{prefix}-{}", atomize(&value))),
    )
}

fn driver_identity(
    video_controller: Option<&str>,
    pnp_signed_driver: Option<&str>,
) -> InventoryValue {
    let Some(video_controller) = video_controller else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Some(pnp_signed_driver) = pnp_signed_driver else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Some(video_device_id) = json_string(Some(video_controller), "PNPDeviceID") else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let Ok(driver) = serde_json::from_str::<Value>(pnp_signed_driver) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let Some(driver) = first_json_object(&driver) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let device_id = driver.get("DeviceID").and_then(Value::as_str);
    let device_class = driver.get("DeviceClass").and_then(Value::as_str);
    let version = driver.get("DriverVersion").and_then(Value::as_str);
    match (device_id, device_class, version) {
        (Some(device_id), Some("DISPLAY"), Some(version))
            if device_id == video_device_id && !version.trim().is_empty() =>
        {
            observed_or_missing(format!("driver-{}", atomize(version)))
        }
        _ => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

fn compiler_identity(
    compiler: Option<&CompilerResponse>,
    vctools_version: Option<&str>,
    vswhere_version: Option<&str>,
) -> InventoryValue {
    let version = compiler
        .map(CompilerResponse::banner)
        .and_then(version_token)
        .or_else(|| vctools_version.and_then(version_token))
        .or_else(|| vswhere_version.and_then(version_token));
    version.map_or_else(
        || InventoryValue::missing(MissingReason::SourceUnavailable),
        |version| observed_or_missing(format!("msvc-{}", atomize(version))),
    )
}

fn version_token(raw: &str) -> Option<&str> {
    raw.split_whitespace()
        .find(|word| word.bytes().any(|byte| byte.is_ascii_digit()))
        .map(|word| {
            word.trim_matches(|character: char| !character.is_ascii_digit() && character != '.')
        })
        .filter(|version| {
            !version.is_empty()
                && version.bytes().any(|byte| byte == b'.')
                && version
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || byte == b'.')
        })
}

fn rust_toolchain_identity(raw: Option<&str>) -> InventoryValue {
    let Some(raw) = raw else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Some(version) = raw.split_whitespace().nth(1) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    observed_or_missing(format!("rustc-{}", atomize(version)))
}

fn is_hex4(value: &str) -> bool {
    value.len() == 4 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
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

fn package_lock(raw: Option<&str>) -> SystemPackageLock {
    let Some(raw) = raw else {
        return missing_package_records(MissingReason::SourceUnavailable);
    };
    let Ok(value) = serde_json::from_str::<Value>(raw) else {
        return missing_package_records(MissingReason::UnsupportedBySource);
    };
    let values = match value {
        Value::Array(values) => values,
        Value::Object(value) => vec![Value::Object(value)],
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            return missing_package_records(MissingReason::UnsupportedBySource);
        }
    };
    let mut observed = std::collections::BTreeMap::new();
    for value in &values {
        let Some(name) = value
            .as_object()
            .and_then(|object| object.get("Name"))
            .and_then(Value::as_str)
        else {
            return missing_package_records(MissingReason::UnsupportedBySource);
        };
        if !WINDOWS_PACKAGE_REQUIREMENTS.contains(&name) {
            continue;
        }
        let Some((name, version)) = package_fields(value) else {
            return missing_package_records(MissingReason::UnsupportedBySource);
        };
        if observed.insert(name, version).is_some() {
            return missing_package_records(MissingReason::UnsupportedBySource);
        }
    }
    let records = WINDOWS_PACKAGE_REQUIREMENTS
        .iter()
        .map(|name| match observed.remove(*name) {
            Some(version) => SystemPackage::new((*name).to_owned(), version)
                .map_err(|_| MissingReason::UnsupportedBySource),
            None => SystemPackage::missing((*name).to_owned(), MissingReason::NotInstalled)
                .map_err(|_| MissingReason::UnsupportedBySource),
        })
        .collect::<Result<Vec<_>, _>>();
    match records.and_then(|records| {
        SystemPackageLock::from_records(records).map_err(|_| MissingReason::UnsupportedBySource)
    }) {
        Ok(lock) => lock,
        Err(reason) => SystemPackageLock::missing(reason),
    }
}

fn missing_package_records(reason: MissingReason) -> SystemPackageLock {
    let records = WINDOWS_PACKAGE_REQUIREMENTS
        .iter()
        .map(|name| SystemPackage::missing((*name).to_owned(), reason))
        .collect::<Result<Vec<_>, _>>();
    match records.and_then(SystemPackageLock::from_records) {
        Ok(lock) => lock,
        Err(_) => SystemPackageLock::missing(MissingReason::UnsupportedBySource),
    }
}

fn package_fields(value: &Value) -> Option<(String, String)> {
    let object = value.as_object()?;
    let name = object.get("Name")?.as_str()?.to_owned();
    let version = object.get("Version")?.as_str()?.to_owned();
    Some((name, version))
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
            "Get-ComputerInfo | Select-Object @{Name='Version';Expression={$_.OsVersion}} | ConvertTo-Json -Compress",
        ),
        processor: powershell_json(
            "Get-CimInstance Win32_Processor | Select-Object -First 1 Architecture | ConvertTo-Json -Compress",
        ),
        computer_system: powershell_json(
            "Get-CimInstance Win32_ComputerSystem | Select-Object Model | ConvertTo-Json -Compress",
        ),
        video_controller: powershell_json(
            "Get-CimInstance Win32_VideoController | Select-Object -First 1 PNPDeviceID | ConvertTo-Json -Compress",
        ),
        pnp_signed_driver: powershell_json(
            "$video = Get-CimInstance Win32_VideoController | Select-Object -First 1 PNPDeviceID; if ($null -ne $video) { Get-CimInstance Win32_PnPSignedDriver | Where-Object {$_.DeviceClass -eq 'DISPLAY' -and $_.DeviceID -eq $video.PNPDeviceID} | Select-Object -First 1 DeviceID,DeviceClass,DriverVersion | ConvertTo-Json -Compress }",
        ),
        compiler: command_output_regardless_of_status("cmd", &["/C", "cl /Bv 2>&1"]),
        compiler_env: std::env::var("VCToolsVersion").ok(),
        compiler_vswhere: command_stdout(
            "vswhere",
            &["-latest", "-property", "installationVersion"],
        ),
        sdk: powershell_json(
            "Get-ItemProperty 'HKLM:\\SOFTWARE\\WOW6432Node\\Microsoft\\Microsoft SDKs\\Windows\\v10.0' | Select-Object @{Name='Version';Expression={$_.ProductVersion}} | ConvertTo-Json -Compress",
        ),
        rust_toolchain: command_stdout("rustc", &["+1.98.0", "--version"]),
        package_catalog: powershell_json(
            "Get-Package | Where-Object {$_.Name -in @('Microsoft.VisualStudio.BuildTools','Microsoft.WindowsSDK')} | Select-Object Name,@{Name='Version';Expression={$_.Version.ToString()}} | ConvertTo-Json -Compress",
        ),
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
    command_output(program, arguments).and_then(|output| output.success().then_some(output.stdout))
}

#[cfg(target_os = "windows")]
fn command_output_regardless_of_status(
    program: &str,
    arguments: &[&str],
) -> Option<CompilerResponse> {
    command_output(program, arguments).map(|output| CompilerResponse::Command {
        stdout: output.stdout,
        _exit_code: output.exit_code,
    })
}

#[cfg(target_os = "windows")]
struct CommandOutput {
    stdout: String,
    exit_code: Option<i32>,
    success: bool,
}

#[cfg(target_os = "windows")]
impl CommandOutput {
    const fn success(&self) -> bool {
        self.success
    }
}

#[cfg(target_os = "windows")]
fn command_output(program: &str, arguments: &[&str]) -> Option<CommandOutput> {
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
    let status = child.wait().ok()?;
    let stdout = String::from_utf8(output).ok()?;
    Some(CommandOutput {
        stdout,
        exit_code: status.code(),
        success: status.success(),
    })
}
