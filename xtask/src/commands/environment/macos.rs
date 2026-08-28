//! macOS reference-environment collection from bounded local command responses.

#![cfg_attr(not(any(target_os = "macos", test)), allow(dead_code))]

#[cfg(target_os = "macos")]
use std::io::Read;
#[cfg(target_os = "macos")]
use std::process::{Command, Stdio};

use std::collections::BTreeMap;

use oxyflut_qualification::environment::{
    EnvironmentFields, EnvironmentInventory, InventoryValue, MissingReason, SystemPackage,
    SystemPackageLock,
};
use oxyflut_qualification::identifiers::EnvironmentId;
use serde::Deserialize;
use serde_json::Value;

use super::{EnvironmentCommandError, PlatformSource};

#[cfg(target_os = "macos")]
const OUTPUT_LIMIT: usize = 16 * 1024;

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
            Err(EnvironmentCommandError::UnsupportedHost {
                reason: MissingReason::UnavailableOnHost,
            })
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
    rust_toolchain: Option<String>,
    session: Option<String>,
    pkgutil_pkgs: Option<String>,
    pkgutil_pkg_info: Option<BTreeMap<String, Option<String>>>,
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
        driver_version: InventoryValue::missing(MissingReason::ManualCapture),
        compiler_identity: compiler_identity(responses.compiler.as_deref()),
        sdk_identity: command_identity(responses.sdk.as_deref(), "macos-sdk"),
        rust_toolchain: rust_toolchain_identity(responses.rust_toolchain.as_deref()),
        compositor: InventoryValue::missing(MissingReason::ManualCapture),
        session: raw_identity(responses.session.as_deref()),
        protocol_version: InventoryValue::missing(MissingReason::ManualCapture),
        system_package_lock: package_lock(
            responses.pkgutil_pkgs.as_deref(),
            responses.pkgutil_pkg_info.as_ref(),
        ),
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
    observed_or_missing(format!("apple:{}", slug(model)))
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

fn rust_toolchain_identity(raw: Option<&str>) -> InventoryValue {
    let Some(raw) = raw else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Some(version) = raw.split_whitespace().nth(1) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    observed_or_missing(format!("rustc-{}", atomize(version)))
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

fn package_lock(
    package_ids: Option<&str>,
    package_info: Option<&BTreeMap<String, Option<String>>>,
) -> SystemPackageLock {
    let Some(package_ids) = package_ids else {
        return SystemPackageLock::missing(MissingReason::SourceUnavailable);
    };
    let Some(package_info) = package_info else {
        return SystemPackageLock::missing(MissingReason::SourceUnavailable);
    };
    let mut package_ids = package_ids
        .lines()
        .map(str::trim)
        .filter(|package| !package.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    package_ids.sort_unstable();
    package_ids.dedup();
    package_ids.truncate(oxyflut_qualification::environment::MAXIMUM_SYSTEM_PACKAGES);
    if package_ids.is_empty() {
        return SystemPackageLock::missing(MissingReason::SourceUnavailable);
    }
    let records = package_ids
        .iter()
        .map(|package| {
            package_info
                .get(package)
                .and_then(Option::as_deref)
                .and_then(package_version)
                .map(|version| (package.clone(), version))
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

fn package_version(raw: &str) -> Option<String> {
    raw.lines()
        .find_map(|line| line.strip_prefix("version:").map(str::trim))
        .filter(|version| !version.is_empty())
        .map(str::to_owned)
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

fn slug(value: &str) -> String {
    atomize(value).to_ascii_lowercase()
}

fn observed_or_missing(value: String) -> InventoryValue {
    match InventoryValue::observed(value) {
        Ok(value) => value,
        Err(_) => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

#[cfg(target_os = "macos")]
fn live_responses() -> MacosResponses {
    let pkgutil_pkgs = command_stdout("pkgutil", &["--pkgs"]);
    let pkgutil_pkg_info = pkgutil_pkgs.as_deref().map(live_pkgutil_package_info);
    MacosResponses {
        sw_vers: command_stdout("sw_vers", &["-productVersion"]),
        uname: command_stdout("uname", &["-m"]),
        sysctl_model: command_stdout("sysctl", &["-n", "hw.model"]),
        system_profiler: command_stdout(
            "system_profiler",
            &["SPDisplaysDataType", "SPSoftwareDataType", "-json"],
        ),
        compiler: command_stdout("xcrun", &["--sdk", "macosx", "clang", "--version"]),
        sdk: command_stdout("xcrun", &["--sdk", "macosx", "--show-sdk-version"]),
        rust_toolchain: command_stdout("rustc", &["+1.98.0", "--version"]),
        session: command_stdout("launchctl", &["managername"]),
        pkgutil_pkgs,
        pkgutil_pkg_info,
    }
}

#[cfg(target_os = "macos")]
fn live_pkgutil_package_info(package_ids: &str) -> BTreeMap<String, Option<String>> {
    let mut package_ids = package_ids
        .lines()
        .map(str::trim)
        .filter(|package| !package.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    package_ids.sort_unstable();
    package_ids.dedup();
    package_ids.truncate(oxyflut_qualification::environment::MAXIMUM_SYSTEM_PACKAGES);
    package_ids
        .into_iter()
        .map(|package| {
            let info = command_stdout("pkgutil", &["--pkg-info", &package]);
            (package, info)
        })
        .collect()
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
