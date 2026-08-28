//! macOS reference-environment collection from bounded local command responses.

#![cfg_attr(not(any(target_os = "macos", test)), allow(dead_code))]

#[cfg(any(target_os = "macos", test))]
use std::io::Read;
#[cfg(target_os = "macos")]
use std::process::{Command, Stdio};

use std::collections::BTreeMap;

use oxyflut_qualification::environment::{
    EnvironmentFields, EnvironmentInventory, InventoryValue, MAXIMUM_OBSERVED_VALUE_BYTES,
    MissingReason, SystemPackage, SystemPackageLock,
};
use oxyflut_qualification::identifiers::EnvironmentId;
use serde::Deserialize;
use serde_json::Value;

use super::{EnvironmentCommandError, PlatformSource};

#[cfg(any(target_os = "macos", test))]
const OUTPUT_LIMIT: usize = 16 * 1024;

// These receipts directly bind the Xcode 26.6 build, Command Line Tools executables, and macOS
// SDK 26.5 pinned by stack.md. Each receipt is queried individually, avoiding an unbounded
// `pkgutil --pkgs` inventory before content limits apply.
const MACOS_PACKAGE_REQUIREMENTS: &[&str] = &[
    "com.apple.pkg.CLTools_Executables",
    "com.apple.pkg.CLTools_SDK_macOS",
    "com.apple.pkg.Xcode",
];

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
    pkgutil_pkg_info: Option<BTreeMap<String, Option<String>>>,
    #[serde(default)]
    package_failures: BTreeMap<String, MissingReason>,
    #[serde(skip)]
    source_failures: BTreeMap<&'static str, MissingReason>,
}

impl MacosResponses {
    fn source_missing_reason(&self, source: &str) -> MissingReason {
        self.source_failures
            .get(source)
            .copied()
            .unwrap_or(MissingReason::SourceUnavailable)
    }
}

fn collect_macos_responses(
    responses: &MacosResponses,
) -> Result<EnvironmentInventory, EnvironmentCommandError> {
    let fields = EnvironmentFields {
        operating_system: command_identity(
            responses.sw_vers.as_deref(),
            "macos",
            responses.source_missing_reason("sw_vers"),
        ),
        minimum_version: InventoryValue::missing(MissingReason::NotDeclaredByLock),
        architecture: architecture(
            responses.uname.as_deref(),
            responses.source_missing_reason("uname"),
        ),
        hardware_id: command_identity(
            responses.sysctl_model.as_deref(),
            "hardware",
            responses.source_missing_reason("sysctl_model"),
        ),
        gpu_id: gpu_identity(
            responses.system_profiler.as_deref(),
            responses.source_missing_reason("system_profiler"),
        ),
        driver_version: InventoryValue::missing(MissingReason::ManualCapture),
        compiler_identity: compiler_identity(
            responses.compiler.as_deref(),
            responses.source_missing_reason("compiler"),
        ),
        sdk_identity: command_identity(
            responses.sdk.as_deref(),
            "macos-sdk",
            responses.source_missing_reason("sdk"),
        ),
        rust_toolchain: rust_toolchain_identity(
            responses.rust_toolchain.as_deref(),
            responses.source_missing_reason("rust_toolchain"),
        ),
        compositor: InventoryValue::missing(MissingReason::ManualCapture),
        session: raw_identity(
            responses.session.as_deref(),
            responses.source_missing_reason("session"),
        ),
        protocol_version: InventoryValue::missing(MissingReason::ManualCapture),
        system_package_lock: package_lock(
            responses.pkgutil_pkg_info.as_ref(),
            &responses.package_failures,
            responses.source_missing_reason("pkgutil_pkg_info"),
        ),
    };
    EnvironmentInventory::new(EnvironmentId::Macos, fields)
        .map_err(EnvironmentCommandError::Inventory)
}

fn architecture(raw: Option<&str>, missing_reason: MissingReason) -> InventoryValue {
    let Some(value) = raw.and_then(first_line) else {
        return InventoryValue::missing(missing_reason);
    };
    let normalized = match value {
        "aarch64" | "arm64" => "aarch64",
        "x86_64" | "amd64" => "x86_64",
        other => other,
    };
    observed_or_missing(normalized.to_owned())
}

fn gpu_identity(raw: Option<&str>, missing_reason: MissingReason) -> InventoryValue {
    let Some(raw) = raw else {
        return InventoryValue::missing(missing_reason);
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

fn compiler_identity(raw: Option<&str>, missing_reason: MissingReason) -> InventoryValue {
    let Some(raw) = raw else {
        return InventoryValue::missing(missing_reason);
    };
    let Some(version) = raw
        .split_whitespace()
        .find(|word| word.bytes().any(|byte| byte.is_ascii_digit()))
    else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    observed_or_missing(format!("apple-clang-{}", atomize(version)))
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

fn command_identity(
    raw: Option<&str>,
    prefix: &str,
    missing_reason: MissingReason,
) -> InventoryValue {
    let Some(value) = raw.and_then(first_line) else {
        return InventoryValue::missing(missing_reason);
    };
    observed_or_missing(format!("{prefix}-{}", atomize(value)))
}

fn raw_identity(raw: Option<&str>, missing_reason: MissingReason) -> InventoryValue {
    raw.and_then(first_line).map_or_else(
        || InventoryValue::missing(missing_reason),
        |value| observed_or_missing(atomize(value)),
    )
}

fn package_lock(
    package_info: Option<&BTreeMap<String, Option<String>>>,
    package_failures: &BTreeMap<String, MissingReason>,
    missing_reason: MissingReason,
) -> SystemPackageLock {
    let Some(package_info) = package_info else {
        return missing_package_records(missing_reason);
    };
    if missing_reason == MissingReason::InventoryExceedsBound {
        return missing_package_records(missing_reason);
    }
    let records = MACOS_PACKAGE_REQUIREMENTS
        .iter()
        .map(
            |package| match package_info.get(*package).and_then(Option::as_deref) {
                Some(raw) => match package_version(raw) {
                    Some(version) => SystemPackage::new((*package).to_owned(), version)
                        .map_err(|_| MissingReason::UnsupportedBySource),
                    None => Err(MissingReason::UnsupportedBySource),
                },
                None => SystemPackage::missing(
                    (*package).to_owned(),
                    package_failures
                        .get(*package)
                        .copied()
                        .unwrap_or(MissingReason::NotInstalled),
                )
                .map_err(|_| MissingReason::UnsupportedBySource),
            },
        )
        .collect::<Result<Vec<_>, _>>();
    match records.and_then(|records| {
        SystemPackageLock::from_records(records).map_err(|_| MissingReason::UnsupportedBySource)
    }) {
        Ok(lock) => lock,
        Err(reason) => missing_package_records(reason),
    }
}

fn missing_package_records(reason: MissingReason) -> SystemPackageLock {
    let records = MACOS_PACKAGE_REQUIREMENTS
        .iter()
        .map(|package| SystemPackage::missing((*package).to_owned(), reason))
        .collect::<Result<Vec<_>, _>>();
    match records.and_then(SystemPackageLock::from_records) {
        Ok(lock) => lock,
        Err(_) => SystemPackageLock::missing(MissingReason::UnsupportedBySource),
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
    let exceeds_observation_bound = value.len() > MAXIMUM_OBSERVED_VALUE_BYTES;
    match InventoryValue::observed(value) {
        Ok(value) => value,
        Err(_) if exceeds_observation_bound => {
            InventoryValue::missing(MissingReason::InventoryExceedsBound)
        }
        Err(_) => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

#[cfg(target_os = "macos")]
fn live_responses() -> MacosResponses {
    let mut source_failures = BTreeMap::new();
    let (pkgutil_pkg_info, package_failures) = live_pkgutil_package_info(&mut source_failures);
    MacosResponses {
        sw_vers: capture_response(
            command_stdout("sw_vers", &["-productVersion"]),
            "sw_vers",
            &mut source_failures,
        ),
        uname: capture_response(
            command_stdout("uname", &["-m"]),
            "uname",
            &mut source_failures,
        ),
        sysctl_model: capture_response(
            command_stdout("sysctl", &["-n", "hw.model"]),
            "sysctl_model",
            &mut source_failures,
        ),
        system_profiler: capture_response(
            command_stdout("system_profiler", &["SPDisplaysDataType", "-json"]),
            "system_profiler",
            &mut source_failures,
        ),
        compiler: capture_response(
            command_stdout("xcrun", &["--sdk", "macosx", "clang", "--version"]),
            "compiler",
            &mut source_failures,
        ),
        sdk: capture_response(
            command_stdout("xcrun", &["--sdk", "macosx", "--show-sdk-version"]),
            "sdk",
            &mut source_failures,
        ),
        rust_toolchain: capture_response(
            command_stdout("rustc", &["+1.98.0", "--version"]),
            "rust_toolchain",
            &mut source_failures,
        ),
        session: capture_response(
            command_stdout("launchctl", &["managername"]),
            "session",
            &mut source_failures,
        ),
        pkgutil_pkg_info: Some(pkgutil_pkg_info),
        package_failures,
        source_failures,
    }
}

#[cfg(target_os = "macos")]
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

#[cfg(target_os = "macos")]
fn live_pkgutil_package_info(
    source_failures: &mut BTreeMap<&'static str, MissingReason>,
) -> (
    BTreeMap<String, Option<String>>,
    BTreeMap<String, MissingReason>,
) {
    let mut package_info = BTreeMap::new();
    let mut package_failures = BTreeMap::new();
    for package in MACOS_PACKAGE_REQUIREMENTS {
        let info = match command_stdout("pkgutil", &["--pkg-info", package]) {
            Ok(info) => info,
            Err(reason) => {
                source_failures.insert("pkgutil_pkg_info", reason);
                package_failures.insert((*package).to_owned(), reason);
                None
            }
        };
        package_info.insert((*package).to_owned(), info);
    }
    (package_info, package_failures)
}

#[cfg(target_os = "macos")]
fn command_stdout(program: &str, arguments: &[&str]) -> Result<Option<String>, MissingReason> {
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
    Ok(status.success().then_some(stdout))
}

#[cfg(any(target_os = "macos", test))]
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
    use std::io::Cursor;

    use oxyflut_qualification::environment::{
        InventoryValue, MAXIMUM_OBSERVED_VALUE_BYTES, MissingReason,
    };

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
    fn oversized_observation_reports_the_bound() {
        assert_eq!(
            super::observed_or_missing("a".repeat(MAXIMUM_OBSERVED_VALUE_BYTES + 1)),
            InventoryValue::missing(MissingReason::InventoryExceedsBound)
        );
    }
}
