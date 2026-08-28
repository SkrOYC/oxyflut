//! Windows reference-environment collection from bounded PowerShell CIM responses.

#![cfg_attr(not(any(target_os = "windows", test)), allow(dead_code))]

use std::collections::BTreeMap;

#[cfg(any(target_os = "windows", test))]
use std::io::Read;
#[cfg(target_os = "windows")]
use std::process::{Command, Stdio};

use oxyflut_qualification::environment::{
    EnvironmentFields, EnvironmentInventory, InventoryValue, MAXIMUM_OBSERVED_VALUE_BYTES,
    MissingReason, SystemPackage, SystemPackageLock,
};
use oxyflut_qualification::identifiers::EnvironmentId;
use serde::Deserialize;
use serde_json::Value;

use super::{EnvironmentCommandError, PlatformSource};

#[cfg(any(target_os = "windows", test))]
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
    #[serde(skip)]
    source_failures: BTreeMap<&'static str, MissingReason>,
}

impl WindowsResponses {
    fn source_missing_reason(&self, source: &str) -> MissingReason {
        self.source_failures
            .get(source)
            .copied()
            .unwrap_or(MissingReason::SourceUnavailable)
    }
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
        operating_system: operating_system_identity(
            responses.operating_system.as_deref(),
            responses.source_missing_reason("operating_system"),
        ),
        minimum_version: InventoryValue::missing(MissingReason::NotDeclaredByLock),
        architecture: architecture(
            responses.processor.as_deref(),
            responses.source_missing_reason("processor"),
        ),
        hardware_id: json_identity(
            responses.computer_system.as_deref(),
            "Model",
            "hardware",
            responses.source_missing_reason("computer_system"),
        ),
        gpu_id: gpu_identity(
            responses.video_controller.as_deref(),
            responses.source_missing_reason("video_controller"),
        ),
        driver_version: driver_identity(
            responses.video_controller.as_deref(),
            responses.pnp_signed_driver.as_deref(),
            responses.source_missing_reason("video_controller"),
            responses.source_missing_reason("pnp_signed_driver"),
        ),
        compiler_identity: compiler_identity(
            responses.compiler.as_ref(),
            responses.compiler_env.as_deref(),
            responses.compiler_vswhere.as_deref(),
            responses.source_missing_reason("compiler"),
        ),
        sdk_identity: json_identity(
            responses.sdk.as_deref(),
            "Version",
            "windows-sdk",
            responses.source_missing_reason("sdk"),
        ),
        rust_toolchain: rust_toolchain_identity(
            responses.rust_toolchain.as_deref(),
            responses.source_missing_reason("rust_toolchain"),
        ),
        compositor: InventoryValue::missing(MissingReason::ManualCapture),
        session: InventoryValue::missing(MissingReason::ManualCapture),
        protocol_version: InventoryValue::missing(MissingReason::ManualCapture),
        system_package_lock: package_lock(
            responses.package_catalog.as_deref(),
            responses.source_missing_reason("package_catalog"),
        ),
    };
    EnvironmentInventory::new(EnvironmentId::Windows, fields)
        .map_err(EnvironmentCommandError::Inventory)
}

fn architecture(raw: Option<&str>, missing_reason: MissingReason) -> InventoryValue {
    let Some(raw) = raw else {
        return InventoryValue::missing(missing_reason);
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

fn gpu_identity(raw: Option<&str>, missing_reason: MissingReason) -> InventoryValue {
    let Some(raw) = raw else {
        return InventoryValue::missing(missing_reason);
    };
    let Some(value) = json_string(Some(raw), "PNPDeviceID") else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
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

fn operating_system_identity(raw: Option<&str>, missing_reason: MissingReason) -> InventoryValue {
    let Some(raw) = raw else {
        return InventoryValue::missing(missing_reason);
    };
    let Ok(value) = serde_json::from_str::<Value>(raw) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let Some(object) = first_json_object(&value) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let Some(product_name) = object.get("ProductName").and_then(Value::as_str) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let Some(display_version) = object.get("DisplayVersion").and_then(Value::as_str) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    windows_operating_system_token(product_name, display_version).map_or_else(
        || InventoryValue::missing(MissingReason::UnsupportedBySource),
        observed_or_missing,
    )
}

fn windows_operating_system_token(product_name: &str, display_version: &str) -> Option<String> {
    let mut product_tokens = product_name.split_ascii_whitespace();
    if !product_tokens.next()?.eq_ignore_ascii_case("Windows") {
        return None;
    }
    let generation = match product_tokens.next()? {
        "10" => "10",
        "11" => "11",
        _ => return None,
    };
    let (release, half) = display_version.split_once('H')?;
    if release.is_empty()
        || half.is_empty()
        || !release.bytes().all(|byte| byte.is_ascii_digit())
        || !half.bytes().all(|byte| byte.is_ascii_digit())
    {
        return None;
    }
    Some(format!("windows-{generation}-{release}H{half}"))
}

fn json_identity(
    raw: Option<&str>,
    field: &str,
    prefix: &str,
    missing_reason: MissingReason,
) -> InventoryValue {
    let Some(raw) = raw else {
        return InventoryValue::missing(missing_reason);
    };
    json_string(Some(raw), field).map_or_else(
        || InventoryValue::missing(MissingReason::UnsupportedBySource),
        |value| observed_or_missing(format!("{prefix}-{}", atomize(&value))),
    )
}

fn driver_identity(
    video_controller: Option<&str>,
    pnp_signed_driver: Option<&str>,
    video_missing_reason: MissingReason,
    driver_missing_reason: MissingReason,
) -> InventoryValue {
    let Some(video_controller) = video_controller else {
        return InventoryValue::missing(video_missing_reason);
    };
    let Some(pnp_signed_driver) = pnp_signed_driver else {
        return InventoryValue::missing(driver_missing_reason);
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
    missing_reason: MissingReason,
) -> InventoryValue {
    let version = compiler
        .map(CompilerResponse::banner)
        .and_then(version_token)
        .or_else(|| vctools_version.and_then(version_token))
        .or_else(|| vswhere_version.and_then(version_token));
    version.map_or_else(
        || InventoryValue::missing(missing_reason),
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

fn rust_toolchain_identity(raw: Option<&str>, missing_reason: MissingReason) -> InventoryValue {
    let Some(raw) = raw else {
        return InventoryValue::missing(missing_reason);
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

fn package_lock(raw: Option<&str>, missing_reason: MissingReason) -> SystemPackageLock {
    let Some(raw) = raw else {
        return missing_package_records(missing_reason);
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
    let exceeds_observation_bound = value.len() > MAXIMUM_OBSERVED_VALUE_BYTES;
    match InventoryValue::observed(value) {
        Ok(value) => value,
        Err(_) if exceeds_observation_bound => {
            InventoryValue::missing(MissingReason::InventoryExceedsBound)
        }
        Err(_) => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

#[cfg(target_os = "windows")]
fn live_responses() -> WindowsResponses {
    let mut source_failures = BTreeMap::new();
    WindowsResponses {
        operating_system: capture_response(
            powershell_json(
                "Get-ComputerInfo | Select-Object @{Name='ProductName';Expression={$_.WindowsProductName}},@{Name='DisplayVersion';Expression={$_.OsDisplayVersion}} | ConvertTo-Json -Compress",
            ),
            "operating_system",
            &mut source_failures,
        ),
        processor: capture_response(
            powershell_json(
                "Get-CimInstance Win32_Processor | Select-Object -First 1 Architecture | ConvertTo-Json -Compress",
            ),
            "processor",
            &mut source_failures,
        ),
        computer_system: capture_response(
            powershell_json(
                "Get-CimInstance Win32_ComputerSystem | Select-Object Model | ConvertTo-Json -Compress",
            ),
            "computer_system",
            &mut source_failures,
        ),
        video_controller: capture_response(
            powershell_json(
                "Get-CimInstance Win32_VideoController | Select-Object -First 1 PNPDeviceID | ConvertTo-Json -Compress",
            ),
            "video_controller",
            &mut source_failures,
        ),
        pnp_signed_driver: capture_response(
            powershell_json(
                "$video = Get-CimInstance Win32_VideoController | Select-Object -First 1 PNPDeviceID; if ($null -ne $video) { Get-CimInstance Win32_PnPSignedDriver | Where-Object {$_.DeviceClass -eq 'DISPLAY' -and $_.DeviceID -eq $video.PNPDeviceID} | Select-Object -First 1 DeviceID,DeviceClass,DriverVersion | ConvertTo-Json -Compress }",
            ),
            "pnp_signed_driver",
            &mut source_failures,
        ),
        compiler: capture_response(
            command_output_regardless_of_status("cmd", &["/C", "cl /Bv 2>&1"]),
            "compiler",
            &mut source_failures,
        ),
        compiler_env: std::env::var("VCToolsVersion").ok(),
        compiler_vswhere: capture_response(
            command_stdout("vswhere", &["-latest", "-property", "installationVersion"]),
            "compiler",
            &mut source_failures,
        ),
        sdk: capture_response(
            powershell_json(
                "Get-ItemProperty 'HKLM:\\SOFTWARE\\WOW6432Node\\Microsoft\\Microsoft SDKs\\Windows\\v10.0' | Select-Object @{Name='Version';Expression={$_.ProductVersion}} | ConvertTo-Json -Compress",
            ),
            "sdk",
            &mut source_failures,
        ),
        rust_toolchain: capture_response(
            command_stdout("rustc", &["+1.98.0", "--version"]),
            "rust_toolchain",
            &mut source_failures,
        ),
        package_catalog: capture_response(
            powershell_json(
                "Get-Package | Where-Object {$_.Name -in @('Microsoft.VisualStudio.BuildTools','Microsoft.WindowsSDK')} | Select-Object Name,@{Name='Version';Expression={$_.Version.ToString()}} | ConvertTo-Json -Compress",
            ),
            "package_catalog",
            &mut source_failures,
        ),
        source_failures,
    }
}

#[cfg(target_os = "windows")]
fn capture_response<T>(
    response: Result<Option<T>, MissingReason>,
    source: &'static str,
    failures: &mut BTreeMap<&'static str, MissingReason>,
) -> Option<T> {
    match response {
        Ok(value) => value,
        Err(reason) => {
            failures.insert(source, reason);
            None
        }
    }
}

#[cfg(target_os = "windows")]
fn powershell_json(script: &str) -> Result<Option<String>, MissingReason> {
    command_stdout(
        "powershell",
        &["-NoProfile", "-NonInteractive", "-Command", script],
    )
}

#[cfg(target_os = "windows")]
fn command_stdout(program: &str, arguments: &[&str]) -> Result<Option<String>, MissingReason> {
    Ok(command_output(program, arguments)?
        .and_then(|output| output.success().then_some(output.stdout)))
}

#[cfg(target_os = "windows")]
fn command_output_regardless_of_status(
    program: &str,
    arguments: &[&str],
) -> Result<Option<CompilerResponse>, MissingReason> {
    Ok(
        command_output(program, arguments)?.map(|output| CompilerResponse::Command {
            stdout: output.stdout,
            _exit_code: output.exit_code,
        }),
    )
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
fn command_output(
    program: &str,
    arguments: &[&str],
) -> Result<Option<CommandOutput>, MissingReason> {
    let mut child = match Command::new(program)
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => return Ok(None),
    };
    let stdout = child
        .stdout
        .take()
        .ok_or(MissingReason::SourceUnavailable)?;
    let stdout = match read_bounded(stdout, OUTPUT_LIMIT) {
        Ok(stdout) => stdout,
        Err(reason) => {
            let _ = child.kill();
            let _ = child.wait();
            return Err(reason);
        }
    };
    let status = child.wait().map_err(|_| MissingReason::SourceUnavailable)?;
    Ok(Some(CommandOutput {
        stdout,
        exit_code: status.code(),
        success: status.success(),
    }))
}

#[cfg(any(target_os = "windows", test))]
fn read_bounded(mut reader: impl Read, limit: usize) -> Result<String, MissingReason> {
    let capture_limit = limit
        .checked_add(1)
        .ok_or(MissingReason::InventoryExceedsBound)?;
    let mut output = Vec::with_capacity(capture_limit);
    reader
        .by_ref()
        .take(u64::try_from(capture_limit).map_err(|_| MissingReason::InventoryExceedsBound)?)
        .read_to_end(&mut output)
        .map_err(|_| MissingReason::SourceUnavailable)?;
    if output.len() > limit {
        return Err(MissingReason::InventoryExceedsBound);
    }
    String::from_utf8(output).map_err(|_| MissingReason::UnsupportedBySource)
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::io::Cursor;

    use oxyflut_qualification::environment::{
        InventoryValue, MAXIMUM_OBSERVED_VALUE_BYTES, MissingReason,
    };
    use oxyflut_qualification::identifiers::EnvironmentId;

    use super::collect_fixture_windows;

    #[test]
    fn oversized_observation_reports_the_bound() {
        assert_eq!(
            super::observed_or_missing("a".repeat(MAXIMUM_OBSERVED_VALUE_BYTES + 1)),
            InventoryValue::missing(MissingReason::InventoryExceedsBound)
        );
    }

    #[test]
    fn oversized_capture_reports_the_bound() {
        assert!(matches!(
            super::read_bounded(
                Cursor::new(vec![b'x'; super::OUTPUT_LIMIT + 1]),
                super::OUTPUT_LIMIT
            ),
            Err(MissingReason::InventoryExceedsBound)
        ));
    }

    #[test]
    fn windows_operating_system_normalizer_accepts_windows_11_25h2_editions()
    -> Result<(), Box<dyn Error>> {
        for product_name in ["Windows 11 Pro", "Windows 11 Enterprise"] {
            let inventory = inventory_with_operating_system(product_name, "25H2")?;
            assert_eq!(
                inventory.operating_system().observed_value(),
                Some("windows-11-25H2")
            );
            super::super::validate_operating_system(EnvironmentId::Windows, &inventory)?;
        }
        Ok(())
    }

    #[test]
    fn windows_operating_system_normalizer_rejects_unpinned_releases() -> Result<(), Box<dyn Error>>
    {
        for (product_name, display_version) in [
            ("Windows 10 Pro", "22H2"),
            ("Windows 11 Enterprise", "24H2"),
        ] {
            let inventory = inventory_with_operating_system(product_name, display_version)?;
            assert!(matches!(
                super::super::validate_operating_system(EnvironmentId::Windows, &inventory),
                Err(super::super::EnvironmentCommandError::EnvironmentMismatch)
            ));
        }
        Ok(())
    }

    #[test]
    fn windows_operating_system_normalizer_keeps_unparseable_product_fields_missing()
    -> Result<(), Box<dyn Error>> {
        let inventory = inventory_with_operating_system("Windows Server 2025", "25H2")?;
        assert_eq!(
            inventory.operating_system().missing_reason(),
            Some(MissingReason::UnsupportedBySource)
        );
        Ok(())
    }

    fn inventory_with_operating_system(
        product_name: &str,
        display_version: &str,
    ) -> Result<oxyflut_qualification::environment::EnvironmentInventory, Box<dyn Error>> {
        let operating_system = serde_json::to_string(&serde_json::json!({
            "ProductName": product_name,
            "DisplayVersion": display_version,
        }))?;
        let responses = serde_json::to_vec(&serde_json::json!({
            "operatingSystem": operating_system,
        }))?;
        Ok(collect_fixture_windows(&responses)?)
    }
}
