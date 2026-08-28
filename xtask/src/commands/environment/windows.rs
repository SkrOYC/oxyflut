//! Windows reference-environment source using bounded PowerShell CIM queries.

#[cfg(target_os = "windows")]
use std::io::Read;
#[cfg(target_os = "windows")]
use std::process::{Command, Stdio};

use oxyflut_qualification::environment::EnvironmentInventory;
#[cfg(target_os = "windows")]
use oxyflut_qualification::environment::{
    EnvironmentFields, InventoryValue, MissingReason, SystemPackageLock,
};
use oxyflut_qualification::identifiers::EnvironmentId;

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
            let fields = EnvironmentFields {
                operating_system: powershell_identity(
                    "(Get-CimInstance Win32_OperatingSystem).Version",
                    "windows",
                ),
                minimum_version: InventoryValue::missing(MissingReason::NotDeclaredByLock),
                architecture: powershell_identity(
                    "(Get-CimInstance Win32_OperatingSystem).OSArchitecture",
                    "architecture",
                ),
                hardware_id: powershell_identity(
                    "(Get-CimInstance Win32_ComputerSystem).Model",
                    "hardware",
                ),
                gpu_id: gpu_identity(),
                driver_version: powershell_identity(
                    "(Get-CimInstance Win32_VideoController | Select-Object -First 1).DriverVersion",
                    "driver",
                ),
                compiler_identity: command_identity("cl", &["/Bv"], "msvc"),
                sdk_identity: InventoryValue::missing(MissingReason::SourceUnavailable),
                compositor: InventoryValue::missing(MissingReason::SourceUnavailable),
                session: InventoryValue::missing(MissingReason::SourceUnavailable),
                protocol_version: InventoryValue::missing(MissingReason::SourceUnavailable),
                system_package_lock: SystemPackageLock::missing(MissingReason::SourceUnavailable),
            };
            EnvironmentInventory::new(EnvironmentId::Windows, fields)
                .map_err(EnvironmentCommandError::Inventory)
        }
        #[cfg(not(target_os = "windows"))]
        {
            Err(EnvironmentCommandError::UnsupportedHost)
        }
    }
}

#[cfg(target_os = "windows")]
fn gpu_identity() -> InventoryValue {
    let Some(output) = powershell_stdout(
        "(Get-CimInstance Win32_VideoController | Select-Object -First 1).PNPDeviceID",
    ) else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Ok(output) = String::from_utf8(output) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let value = output.trim();
    let vendor = value.split('&').find(|part| part.starts_with("VEN_"));
    let device = value.split('&').find(|part| part.starts_with("DEV_"));
    match (vendor, device) {
        (Some(vendor), Some(device)) => observed_or_missing(format!("pci-{vendor}-{device}")),
        _ => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

#[cfg(target_os = "windows")]
fn powershell_identity(script: &str, prefix: &str) -> InventoryValue {
    let Some(output) = powershell_stdout(script) else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Ok(output) = String::from_utf8(output) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let Some(value) = output.lines().next() else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    observed_or_missing(format!("{prefix}-{}", atomize(value)))
}

#[cfg(target_os = "windows")]
fn command_identity(program: &str, arguments: &[&str], prefix: &str) -> InventoryValue {
    let Some(output) = command_stdout(program, arguments) else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Ok(output) = String::from_utf8(output) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let Some(version) = output
        .split_whitespace()
        .find(|word| word.bytes().any(|byte| byte.is_ascii_digit()))
    else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    observed_or_missing(format!("{prefix}-{}", atomize(version)))
}

#[cfg(target_os = "windows")]
fn observed_or_missing(value: String) -> InventoryValue {
    match InventoryValue::observed(value) {
        Ok(value) => value,
        Err(_) => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

#[cfg(target_os = "windows")]
fn atomize(value: &str) -> String {
    value
        .bytes()
        .filter_map(|byte| {
            (byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_' | b':' | b'+'))
                .then_some(byte)
                .or_else(|| byte.is_ascii_whitespace().then_some(b'-'))
        })
        .collect()
}

#[cfg(target_os = "windows")]
fn powershell_stdout(script: &str) -> Option<Vec<u8>> {
    command_stdout(
        "powershell",
        &["-NoProfile", "-NonInteractive", "-Command", script],
    )
}

#[cfg(target_os = "windows")]
fn command_stdout(program: &str, arguments: &[&str]) -> Option<Vec<u8>> {
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
    child.wait().ok()?.success().then_some(output)
}
