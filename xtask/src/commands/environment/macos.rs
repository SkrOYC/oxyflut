//! macOS reference-environment source using only local Apple command-line interfaces.

#[cfg(target_os = "macos")]
use std::io::Read;
#[cfg(target_os = "macos")]
use std::process::{Command, Stdio};

use oxyflut_qualification::environment::EnvironmentInventory;
#[cfg(target_os = "macos")]
use oxyflut_qualification::environment::{
    EnvironmentFields, InventoryValue, MissingReason, SystemPackageLock,
};
use oxyflut_qualification::identifiers::EnvironmentId;

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
            let fields = EnvironmentFields {
                operating_system: command_identity("sw_vers", &["-productVersion"], "macos"),
                minimum_version: InventoryValue::missing(MissingReason::NotDeclaredByLock),
                architecture: command_identity("uname", &["-m"], "architecture"),
                hardware_id: command_identity("sysctl", &["-n", "hw.model"], "hardware"),
                gpu_id: gpu_identity(),
                driver_version: InventoryValue::missing(MissingReason::UnsupportedBySource),
                compiler_identity: compiler_identity(),
                sdk_identity: command_identity(
                    "xcrun",
                    &["--sdk", "macosx", "--show-sdk-version"],
                    "macos-sdk",
                ),
                compositor: InventoryValue::missing(MissingReason::SourceUnavailable),
                session: InventoryValue::missing(MissingReason::SourceUnavailable),
                protocol_version: InventoryValue::missing(MissingReason::SourceUnavailable),
                system_package_lock: SystemPackageLock::missing(MissingReason::SourceUnavailable),
            };
            EnvironmentInventory::new(EnvironmentId::Macos, fields)
                .map_err(EnvironmentCommandError::Inventory)
        }
        #[cfg(not(target_os = "macos"))]
        {
            Err(EnvironmentCommandError::UnsupportedHost)
        }
    }
}

#[cfg(target_os = "macos")]
fn gpu_identity() -> InventoryValue {
    let Some(output) = command_stdout("system_profiler", &["SPDisplaysDataType"], OUTPUT_LIMIT)
    else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Ok(output) = String::from_utf8(output) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let Some(value) = output
        .lines()
        .find_map(|line| line.trim().strip_prefix("Chipset Model:").map(str::trim))
    else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    observed_or_missing(format!("gpu-{}", atomize(value)))
}

#[cfg(target_os = "macos")]
fn compiler_identity() -> InventoryValue {
    let Some(output) = command_stdout(
        "xcrun",
        &["--sdk", "macosx", "clang", "--version"],
        OUTPUT_LIMIT,
    ) else {
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
    observed_or_missing(format!("apple-clang-{}", atomize(version)))
}

#[cfg(target_os = "macos")]
fn command_identity(program: &str, arguments: &[&str], prefix: &str) -> InventoryValue {
    let Some(output) = command_stdout(program, arguments, OUTPUT_LIMIT) else {
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

#[cfg(target_os = "macos")]
fn observed_or_missing(value: String) -> InventoryValue {
    match InventoryValue::observed(value) {
        Ok(value) => value,
        Err(_) => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

#[cfg(target_os = "macos")]
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

#[cfg(target_os = "macos")]
fn command_stdout(program: &str, arguments: &[&str], limit: usize) -> Option<Vec<u8>> {
    let mut child = Command::new(program)
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let stdout = child.stdout.take()?;
    let capture_limit = limit.checked_add(1)?;
    let mut output = Vec::with_capacity(capture_limit);
    let mut stdout = stdout.take(u64::try_from(capture_limit).ok()?);
    stdout.read_to_end(&mut output).ok()?;
    if output.len() > limit {
        let _ = child.kill();
        let _ = child.wait();
        return None;
    }
    child.wait().ok()?.success().then_some(output)
}
