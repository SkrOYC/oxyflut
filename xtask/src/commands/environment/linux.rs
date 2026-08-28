//! Bounded Linux inventory collection shared by the Wayland and X11 sources.

use std::collections::BTreeMap;

#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::io::{self, Read};
#[cfg(target_os = "linux")]
use std::path::Path;
#[cfg(target_os = "linux")]
use std::process::{Command, Stdio};

use oxyflut_qualification::environment::{
    EnvironmentFields, EnvironmentInventory, InventoryValue, MAXIMUM_OBSERVED_VALUE_BYTES,
    MissingReason, SystemPackage, SystemPackageLock,
};
use oxyflut_qualification::identifiers::EnvironmentId;
use serde::Deserialize;

use super::EnvironmentCommandError;

const COMMAND_OUTPUT_LIMIT: usize = 4096;
const SOURCE_FILE_LIMIT: usize = 4096;
const PACKAGE_OUTPUT_LIMIT: usize = 512;
const MESA_DRIVER_PACKAGES: &[&str] = &["libgl1-mesa-dri", "mesa-vulkan-drivers"];
const WAYLAND_PROTOCOL_INTERFACES: &[&str] = &[
    "wl_compositor",
    "wl_shm",
    "wl_seat",
    "wl_output",
    "xdg_wm_base",
    "zwp_linux_dmabuf_v1",
    "wp_viewporter",
    "wp_fractional_scale_manager_v1",
];

// Verification source (OXY-C004): packages.ubuntu.com name searches over all suites returned
// HTTP 200 for each exact binary package name: libglib2.0-0t64, libglib2.0-0, libgtk-4-1,
// libwayland-client0, libwayland-server0, xserver-xorg-core, libx11-6, libxcb1, clang, lld,
// libc6, libc6-dev, binutils, and rustc. The GLib names are alternatives because
// Ubuntu's t64 transition replaces the non-t64 binary package on supported release lines.
const LINUX_PACKAGE_REQUIREMENTS: &[&[&str]] = &[
    &["binutils"],
    &["clang"],
    &["libc6"],
    &["libc6-dev"],
    &["libglib2.0-0t64", "libglib2.0-0"],
    &["libgtk-4-1"],
    &["libwayland-client0"],
    &["libwayland-server0"],
    &["libx11-6"],
    &["libxcb1"],
    &["lld"],
    &["rustc"],
    &["xserver-xorg-core"],
];

/// Collects a bounded Linux inventory for one active Linux display environment.
pub(crate) fn collect_linux(
    environment: EnvironmentId,
) -> Result<EnvironmentInventory, EnvironmentCommandError> {
    #[cfg(target_os = "linux")]
    {
        collect_linux_responses(environment, &live_responses(environment))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = environment;
        Err(EnvironmentCommandError::UnsupportedHost {
            reason: MissingReason::UnavailableOnHost,
        })
    }
}

/// Collects a Linux inventory from one fixture's raw platform responses.
#[cfg(test)]
pub(crate) fn collect_fixture_linux(
    environment: EnvironmentId,
    bytes: &[u8],
) -> Result<EnvironmentInventory, EnvironmentCommandError> {
    let responses = serde_json::from_slice(bytes).map_err(EnvironmentCommandError::FixtureJson)?;
    collect_linux_responses(environment, &responses)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct LinuxResponses {
    os_release: Option<String>,
    uname: Option<String>,
    sysfs_hardware: Vec<String>,
    gpu_cards: Vec<LinuxGpuCard>,
    rust_toolchain: Option<String>,
    compiler: Option<String>,
    session_type: Option<String>,
    current_desktop: Option<String>,
    wayland_info: Option<String>,
    xdpyinfo: Option<String>,
    dpkg_query: Option<BTreeMap<String, Option<String>>>,
    #[serde(skip)]
    wayland_info_truncated: bool,
    #[serde(skip)]
    xdpyinfo_truncated: bool,
    #[serde(skip)]
    source_failures: BTreeMap<&'static str, MissingReason>,
    #[serde(skip)]
    package_failures: BTreeMap<String, MissingReason>,
}

/// One graphics card observed from its matching DRM card directory.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct LinuxGpuCard {
    card: String,
    vendor: String,
    device: String,
    driver: Option<String>,
}

impl LinuxResponses {
    fn source_missing_reason(&self, source: &str) -> MissingReason {
        self.source_failures
            .get(source)
            .copied()
            .unwrap_or(MissingReason::SourceUnavailable)
    }

    fn protocol_missing_reason(&self, source: &str) -> MissingReason {
        match self.source_missing_reason(source) {
            MissingReason::InventoryExceedsBound => MissingReason::InventoryExceedsBound,
            MissingReason::NotDeclaredByLock
            | MissingReason::ManualCapture
            | MissingReason::UnavailableOnHost
            | MissingReason::SourceUnavailable
            | MissingReason::UnsupportedBySource
            | MissingReason::NotActiveSession
            | MissingReason::NotInstalled
            | MissingReason::AmbiguousSource => MissingReason::ManualCapture,
        }
    }
}

fn collect_linux_responses(
    environment: EnvironmentId,
    responses: &LinuxResponses,
) -> Result<EnvironmentInventory, EnvironmentCommandError> {
    let session = session_value(
        environment,
        responses.session_type.as_deref(),
        responses.source_missing_reason("session_type"),
    );
    if matches!(
        session,
        InventoryValue::Missing {
            reason: MissingReason::NotActiveSession
        }
    ) {
        return Err(EnvironmentCommandError::EnvironmentMismatch);
    }
    let (gpu_id, driver_version) = gpu_and_driver(
        &responses.gpu_cards,
        responses.dpkg_query.as_ref(),
        responses.source_missing_reason("gpu_cards"),
        &responses.package_failures,
        responses.source_missing_reason("dpkg_query"),
    );
    let protocol_version = match environment {
        EnvironmentId::Wayland => wayland_protocol_version(
            responses.wayland_info.as_deref(),
            responses.protocol_missing_reason("wayland_info"),
            responses.wayland_info_truncated,
        ),
        EnvironmentId::X11 => x11_protocol_version(
            responses.xdpyinfo.as_deref(),
            responses.protocol_missing_reason("xdpyinfo"),
            responses.xdpyinfo_truncated,
        ),
        EnvironmentId::Macos | EnvironmentId::Windows => {
            InventoryValue::missing(MissingReason::ManualCapture)
        }
    };
    let fields = EnvironmentFields {
        operating_system: operating_system(
            responses.os_release.as_deref(),
            responses.source_missing_reason("os_release"),
        ),
        minimum_version: InventoryValue::missing(MissingReason::NotDeclaredByLock),
        architecture: architecture(
            responses.uname.as_deref(),
            responses.source_missing_reason("uname"),
        ),
        hardware_id: first_line_value(
            responses.sysfs_hardware.iter().map(String::as_str),
            responses.source_missing_reason("sysfs_hardware"),
        ),
        gpu_id,
        driver_version,
        compiler_identity: native_compiler_identity(
            responses.compiler.as_deref(),
            responses.source_missing_reason("compiler"),
        ),
        sdk_identity: linux_sdk_identity(
            responses.dpkg_query.as_ref(),
            responses.source_missing_reason("dpkg_query"),
        ),
        rust_toolchain: compiler_identity(
            responses.rust_toolchain.as_deref(),
            responses.source_missing_reason("rust_toolchain"),
        ),
        compositor: compositor_value(
            responses.current_desktop.as_deref(),
            responses.source_missing_reason("current_desktop"),
        ),
        session,
        protocol_version,
        system_package_lock: system_package_lock(
            responses.dpkg_query.as_ref(),
            &responses.package_failures,
            responses.source_missing_reason("dpkg_query"),
        ),
    };
    EnvironmentInventory::new(environment, fields).map_err(EnvironmentCommandError::Inventory)
}

fn operating_system(raw: Option<&str>, missing_reason: MissingReason) -> InventoryValue {
    let Some(contents) = raw else {
        return InventoryValue::missing(missing_reason);
    };
    let id = os_release_value(contents, "ID");
    let version = os_release_value(contents, "VERSION_ID");
    match (id, version) {
        (Some(id), Some(version)) => observed_or_missing(format!("{id}-{version}")),
        _ => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

fn os_release_value<'contents>(contents: &'contents str, key: &str) -> Option<&'contents str> {
    contents.lines().find_map(|line| {
        let (name, value) = line.split_once('=')?;
        (name == key).then_some(value.trim_matches('"'))
    })
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

fn gpu_and_driver(
    cards: &[LinuxGpuCard],
    dpkg_query: Option<&BTreeMap<String, Option<String>>>,
    gpu_missing_reason: MissingReason,
    package_failures: &BTreeMap<String, MissingReason>,
    package_missing_reason: MissingReason,
) -> (InventoryValue, InventoryValue) {
    let qualifying = cards
        .iter()
        .filter_map(|card| {
            card.card
                .strip_prefix("card")
                .filter(|suffix| {
                    !suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit())
                })
                .and_then(|_| pci_gpu_id(&card.vendor, &card.device))
                .map(|gpu_id| (card, gpu_id))
        })
        .collect::<Vec<_>>();
    let [(card, gpu_id)] = qualifying.as_slice() else {
        let reason = if qualifying.is_empty() {
            gpu_missing_reason
        } else {
            MissingReason::UnsupportedBySource
        };
        return (
            InventoryValue::missing(reason),
            InventoryValue::missing(reason),
        );
    };
    let gpu_id = observed_or_missing(gpu_id.clone());
    let driver_version = driver_version(
        card.driver.as_deref(),
        dpkg_query,
        package_failures,
        package_missing_reason,
    );
    (gpu_id, driver_version)
}

fn pci_gpu_id(vendor: &str, device: &str) -> Option<String> {
    let vendor = vendor.trim().strip_prefix("0x").unwrap_or(vendor.trim());
    let device = device.trim().strip_prefix("0x").unwrap_or(device.trim());
    let is_hex4 =
        |value: &str| value.len() == 4 && value.bytes().all(|byte| byte.is_ascii_hexdigit());
    (is_hex4(vendor) && is_hex4(device)).then(|| {
        format!(
            "pci:{}:{}",
            vendor.to_ascii_lowercase(),
            device.to_ascii_lowercase()
        )
    })
}

fn compiler_identity(raw: Option<&str>, missing_reason: MissingReason) -> InventoryValue {
    let Some(output) = raw else {
        return InventoryValue::missing(missing_reason);
    };
    let mut words = output.split_whitespace();
    match (words.next(), words.next()) {
        (Some(name), Some(version)) => observed_or_missing(format!("{name}-{version}")),
        _ => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

fn native_compiler_identity(raw: Option<&str>, missing_reason: MissingReason) -> InventoryValue {
    let Some(output) = raw else {
        return InventoryValue::missing(missing_reason);
    };
    let Some(version) = output
        .split_whitespace()
        .find(|word| word.bytes().any(|byte| byte.is_ascii_digit()))
    else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    observed_or_missing(format!("cc-{}", atomize(version)))
}

fn linux_sdk_identity(
    dpkg_query: Option<&BTreeMap<String, Option<String>>>,
    missing_reason: MissingReason,
) -> InventoryValue {
    let Some(raw) = dpkg_query
        .and_then(|packages| packages.get("libc6-dev"))
        .and_then(Option::as_deref)
    else {
        return InventoryValue::missing(missing_reason);
    };
    let Some((name, version)) = package_fields(raw) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    observed_or_missing(format!(
        "linux-sdk-{}-{}",
        atomize(&name),
        atomize(&version)
    ))
}

fn session_value(
    environment: EnvironmentId,
    raw: Option<&str>,
    missing_reason: MissingReason,
) -> InventoryValue {
    let Some(session) = raw.and_then(first_line) else {
        return InventoryValue::missing(missing_reason);
    };
    if !session.eq_ignore_ascii_case(environment.as_str()) {
        return InventoryValue::missing(MissingReason::NotActiveSession);
    }
    observed_or_missing(session.to_ascii_lowercase())
}

fn compositor_value(raw: Option<&str>, missing_reason: MissingReason) -> InventoryValue {
    let Some(compositor) = raw.and_then(first_line) else {
        return InventoryValue::missing(missing_reason);
    };
    observed_or_missing(atomize(compositor))
}

fn wayland_protocol_version(
    raw: Option<&str>,
    missing_reason: MissingReason,
    truncated: bool,
) -> InventoryValue {
    if truncated {
        return InventoryValue::missing(MissingReason::InventoryExceedsBound);
    }
    let Some(raw) = raw else {
        return InventoryValue::missing(missing_reason);
    };
    let mut globals = BTreeMap::new();
    for line in raw.lines() {
        let Some(interface) = line
            .split_once("interface: '")
            .and_then(|(_, remainder)| remainder.split_once('\''))
            .map(|(interface, _)| interface)
        else {
            continue;
        };
        let Some(version) = line
            .split_once("version:")
            .and_then(|(_, remainder)| remainder.trim_start().split_once(','))
            .map(|(version, _)| version.trim())
            .filter(|version| {
                !version.is_empty() && version.bytes().all(|byte| byte.is_ascii_digit())
            })
        else {
            continue;
        };
        globals.insert(interface, version);
    }
    let version = WAYLAND_PROTOCOL_INTERFACES
        .iter()
        .filter_map(|interface| {
            globals
                .get(interface)
                .map(|version| format!("{interface}-{version}"))
        })
        .collect::<Vec<_>>();
    if version.is_empty() {
        return InventoryValue::missing(missing_reason);
    }
    observed_or_missing(format!("wayland-{}", version.join("-")))
}

fn x11_protocol_version(
    raw: Option<&str>,
    missing_reason: MissingReason,
    truncated: bool,
) -> InventoryValue {
    if truncated {
        return InventoryValue::missing(MissingReason::InventoryExceedsBound);
    }
    let Some(raw) = raw else {
        return InventoryValue::missing(missing_reason);
    };
    let Some(version) = raw
        .lines()
        .find_map(|line| line.trim_start().strip_prefix("version number:"))
        .map(str::trim)
        .filter(|version| {
            !version.is_empty()
                && version
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || byte == b'.')
        })
    else {
        return InventoryValue::missing(missing_reason);
    };
    observed_or_missing(format!("x11-{version}"))
}

fn driver_version(
    driver: Option<&str>,
    dpkg_query: Option<&BTreeMap<String, Option<String>>>,
    package_failures: &BTreeMap<String, MissingReason>,
    package_missing_reason: MissingReason,
) -> InventoryValue {
    let Some(driver) = driver.and_then(first_line) else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Some(packages) = dpkg_query else {
        return InventoryValue::missing(package_missing_reason);
    };
    let package = if driver.eq_ignore_ascii_case("nvidia") {
        let matches = packages
            .iter()
            .filter(|(name, _)| name.starts_with("nvidia-driver-"))
            .filter_map(|(_, raw)| raw.as_deref().and_then(package_fields))
            .collect::<Vec<_>>();
        match matches.as_slice() {
            [(name, version)] => Some((name.clone(), version.clone())),
            [] => {
                let failure = package_failures
                    .iter()
                    .find(|(name, _)| name.starts_with("nvidia-driver-"))
                    .map(|(_, reason)| *reason)
                    .unwrap_or(MissingReason::NotInstalled);
                return InventoryValue::missing(failure);
            }
            [_, _, ..] => return InventoryValue::missing(MissingReason::AmbiguousSource),
        }
    } else {
        MESA_DRIVER_PACKAGES.iter().find_map(|name| {
            packages
                .get(*name)
                .and_then(Option::as_deref)
                .and_then(package_fields)
        })
    };
    match package {
        Some((package, version)) => observed_or_missing(format!(
            "{}/{package}={version}",
            atomize(driver).to_ascii_lowercase()
        )),
        None => InventoryValue::missing(MissingReason::NotInstalled),
    }
}

fn system_package_lock(
    dpkg_query: Option<&BTreeMap<String, Option<String>>>,
    package_failures: &BTreeMap<String, MissingReason>,
    missing_reason: MissingReason,
) -> SystemPackageLock {
    let Some(dpkg_query) = dpkg_query else {
        return missing_package_records(missing_reason);
    };
    let mut records = Vec::with_capacity(LINUX_PACKAGE_REQUIREMENTS.len());
    for alternatives in LINUX_PACKAGE_REQUIREMENTS {
        let mut package = None;
        let mut malformed = false;
        for name in *alternatives {
            if let Some(raw) = dpkg_query.get(*name).and_then(Option::as_deref) {
                match package_fields(raw) {
                    Some((actual_name, version)) => {
                        match SystemPackage::new(actual_name, version) {
                            Ok(record) => {
                                package = Some(record);
                                break;
                            }
                            Err(_) => malformed = true,
                        }
                    }
                    None => malformed = true,
                }
            }
        }
        let Some(missing_name) = alternatives.first().copied() else {
            return SystemPackageLock::missing(MissingReason::UnsupportedBySource);
        };
        let record = match package {
            Some(record) => record,
            None => match SystemPackage::missing(
                missing_name.to_owned(),
                if malformed {
                    MissingReason::UnsupportedBySource
                } else {
                    missing_package_reason(alternatives, package_failures)
                },
            ) {
                Ok(record) => record,
                Err(_) => return SystemPackageLock::missing(MissingReason::UnsupportedBySource),
            },
        };
        records.push(record);
    }
    match SystemPackageLock::from_records(records) {
        Ok(lock) => lock,
        Err(_) => SystemPackageLock::missing(MissingReason::UnsupportedBySource),
    }
}

fn missing_package_reason(
    alternatives: &[&str],
    package_failures: &BTreeMap<String, MissingReason>,
) -> MissingReason {
    alternatives
        .iter()
        .find_map(|name| package_failures.get(*name).copied())
        .unwrap_or(MissingReason::NotInstalled)
}

fn missing_package_records(reason: MissingReason) -> SystemPackageLock {
    let records = LINUX_PACKAGE_REQUIREMENTS
        .iter()
        .filter_map(|alternatives| alternatives.first())
        .map(|name| SystemPackage::missing((*name).to_owned(), reason))
        .collect::<Result<Vec<_>, _>>();
    match records.and_then(SystemPackageLock::from_records) {
        Ok(lock) => lock,
        Err(_) => SystemPackageLock::missing(MissingReason::UnsupportedBySource),
    }
}

fn package_fields(raw: &str) -> Option<(String, String)> {
    let (name, version) = raw.lines().next()?.split_once('\t')?;
    Some((name.to_owned(), version.to_owned()))
}

fn first_line_value<'responses>(
    mut raw: impl Iterator<Item = &'responses str>,
    missing_reason: MissingReason,
) -> InventoryValue {
    raw.find_map(first_line).map_or_else(
        || InventoryValue::missing(missing_reason),
        |value| observed_or_missing(atomize(value)),
    )
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
    let exceeds_observation_bound = value.len() > MAXIMUM_OBSERVED_VALUE_BYTES;
    match InventoryValue::observed(value) {
        Ok(value) => value,
        Err(_) if exceeds_observation_bound => {
            InventoryValue::missing(MissingReason::InventoryExceedsBound)
        }
        Err(_) => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

#[cfg(target_os = "linux")]
fn live_responses(environment: EnvironmentId) -> LinuxResponses {
    let mut source_failures = BTreeMap::new();
    let mut package_failures = BTreeMap::new();
    let sysfs_hardware = [
        "/sys/class/dmi/id/product_name",
        "/sys/devices/virtual/dmi/id/product_name",
    ]
    .iter()
    .filter_map(|path| {
        capture_response(
            read_file_bounded(Path::new(path), SOURCE_FILE_LIMIT),
            "sysfs_hardware",
            &mut source_failures,
        )
    })
    .collect();
    let (wayland_info, wayland_info_truncated, xdpyinfo, xdpyinfo_truncated) = match environment {
        EnvironmentId::Wayland => {
            let (response, truncated) = capture_protocol_response(
                command_stdout_prefix("wayland-info", &[], COMMAND_OUTPUT_LIMIT),
                "wayland_info",
                &mut source_failures,
            );
            (response, truncated, None, false)
        }
        EnvironmentId::X11 => {
            let (response, truncated) = capture_protocol_response(
                command_stdout_prefix("xdpyinfo", &[], COMMAND_OUTPUT_LIMIT),
                "xdpyinfo",
                &mut source_failures,
            );
            (None, false, response, truncated)
        }
        EnvironmentId::Macos | EnvironmentId::Windows => (None, false, None, false),
    };
    let dpkg_query = Path::new("/usr/bin/dpkg-query")
        .is_file()
        .then(|| live_dpkg_query(&mut source_failures, &mut package_failures));

    LinuxResponses {
        os_release: capture_response(
            read_file_bounded(Path::new("/etc/os-release"), SOURCE_FILE_LIMIT),
            "os_release",
            &mut source_failures,
        ),
        uname: capture_response(
            command_stdout("uname", &["-m"], COMMAND_OUTPUT_LIMIT),
            "uname",
            &mut source_failures,
        ),
        sysfs_hardware,
        gpu_cards: live_gpu_cards(&mut source_failures),
        rust_toolchain: capture_response(
            command_stdout("rustc", &["+1.98.0", "--version"], COMMAND_OUTPUT_LIMIT),
            "rust_toolchain",
            &mut source_failures,
        ),
        compiler: capture_response(
            command_stdout("cc", &["--version"], COMMAND_OUTPUT_LIMIT),
            "compiler",
            &mut source_failures,
        ),
        session_type: std::env::var("XDG_SESSION_TYPE").ok(),
        current_desktop: std::env::var("XDG_CURRENT_DESKTOP").ok(),
        wayland_info,
        xdpyinfo,
        dpkg_query,
        wayland_info_truncated,
        xdpyinfo_truncated,
        source_failures,
        package_failures,
    }
}

#[cfg(target_os = "linux")]
fn capture_response(
    response: Result<Option<String>, MissingReason>,
    source: &'static str,
    failures: &mut BTreeMap<&'static str, MissingReason>,
) -> Option<String> {
    match response {
        Ok(value) => value,
        Err(reason) => {
            failures.insert(source, reason);
            None
        }
    }
}

#[cfg(target_os = "linux")]
fn capture_protocol_response(
    response: Result<Option<BoundedOutput>, MissingReason>,
    source: &'static str,
    failures: &mut BTreeMap<&'static str, MissingReason>,
) -> (Option<String>, bool) {
    match response {
        Ok(Some(output)) => (Some(output.contents), output.truncated),
        Ok(None) => (None, false),
        Err(reason) => {
            failures.insert(source, reason);
            (None, false)
        }
    }
}

#[cfg(target_os = "linux")]
fn live_drm_card_paths() -> Vec<std::path::PathBuf> {
    let Ok(directory) = fs::read_dir("/sys/class/drm") else {
        return Vec::new();
    };
    let mut paths = directory
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("card") && !name.contains('-'))
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

#[cfg(target_os = "linux")]
fn live_gpu_cards(
    source_failures: &mut BTreeMap<&'static str, MissingReason>,
) -> Vec<LinuxGpuCard> {
    live_drm_card_paths()
        .iter()
        .filter_map(|path| {
            let card = path.file_name()?.to_str()?.to_owned();
            let vendor = capture_response(
                read_file_bounded(&path.join("device/vendor"), SOURCE_FILE_LIMIT),
                "gpu_cards",
                source_failures,
            )?;
            let device = capture_response(
                read_file_bounded(&path.join("device/device"), SOURCE_FILE_LIMIT),
                "gpu_cards",
                source_failures,
            )?;
            let driver = fs::read_link(path.join("device/driver"))
                .ok()
                .and_then(|driver_path| {
                    driver_path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .map(str::to_owned)
                });
            Some(LinuxGpuCard {
                card,
                vendor: first_line(&vendor)?.to_owned(),
                device: first_line(&device)?.to_owned(),
                driver,
            })
        })
        .collect()
}

#[cfg(target_os = "linux")]
fn live_dpkg_query(
    source_failures: &mut BTreeMap<&'static str, MissingReason>,
    package_failures: &mut BTreeMap<String, MissingReason>,
) -> BTreeMap<String, Option<String>> {
    let mut output = BTreeMap::new();
    for alternatives in LINUX_PACKAGE_REQUIREMENTS {
        for name in *alternatives {
            output.insert(
                (*name).to_owned(),
                capture_package_response(
                    command_stdout(
                        "dpkg-query",
                        &[
                            "--show",
                            "--showformat=${binary:Package}\\t${Version}\\n",
                            name,
                        ],
                        PACKAGE_OUTPUT_LIMIT,
                    ),
                    name,
                    source_failures,
                    package_failures,
                ),
            );
        }
    }
    for name in MESA_DRIVER_PACKAGES {
        output.insert(
            (*name).to_owned(),
            capture_package_response(
                command_stdout(
                    "dpkg-query",
                    &[
                        "--show",
                        "--showformat=${binary:Package}\\t${Version}\\n",
                        name,
                    ],
                    PACKAGE_OUTPUT_LIMIT,
                ),
                name,
                source_failures,
                package_failures,
            ),
        );
    }
    let nvidia = command_stdout(
        "dpkg-query",
        &[
            "--show",
            "--showformat=${binary:Package}\\t${Version}\\n",
            "nvidia-driver-*",
        ],
        COMMAND_OUTPUT_LIMIT,
    );
    match nvidia {
        Ok(Some(records)) => {
            for record in records.lines() {
                if let Some((name, _)) = package_fields(record) {
                    output.insert(name, Some(format!("{record}\n")));
                }
            }
        }
        Ok(None) => {}
        Err(reason) => {
            source_failures.insert("dpkg_query", reason);
            package_failures.insert("nvidia-driver-*".to_owned(), reason);
        }
    }
    output
}

#[cfg(target_os = "linux")]
fn capture_package_response(
    response: Result<Option<String>, MissingReason>,
    package: &str,
    source_failures: &mut BTreeMap<&'static str, MissingReason>,
    package_failures: &mut BTreeMap<String, MissingReason>,
) -> Option<String> {
    match response {
        Ok(value) => value,
        Err(reason) => {
            source_failures.insert("dpkg_query", reason);
            package_failures.insert(package.to_owned(), reason);
            None
        }
    }
}

#[cfg(target_os = "linux")]
fn read_file_bounded(path: &Path, limit: usize) -> Result<Option<String>, MissingReason> {
    match fs::File::open(path) {
        Ok(file) => read_bounded(file, limit).map(Some),
        Err(_) => Ok(None),
    }
}

#[cfg(target_os = "linux")]
fn command_stdout(
    program: &str,
    arguments: &[&str],
    limit: usize,
) -> Result<Option<String>, MissingReason> {
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
    let output = match read_bounded(stdout, limit) {
        Ok(output) => output,
        Err(reason) => {
            let _ = child.kill();
            let _ = child.wait();
            return Err(reason);
        }
    };
    let status = child.wait().map_err(|_| MissingReason::SourceUnavailable)?;
    Ok(status.success().then_some(output))
}

#[cfg(target_os = "linux")]
fn command_stdout_prefix(
    program: &str,
    arguments: &[&str],
    limit: usize,
) -> Result<Option<BoundedOutput>, MissingReason> {
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
    let output = read_bounded_prefix(stdout, limit)?;
    if output.truncated {
        let terminated = match child.kill() {
            Ok(()) => true,
            Err(error) if error.kind() == io::ErrorKind::InvalidInput => false,
            Err(_) => return Err(MissingReason::SourceUnavailable),
        };
        let status = child.wait().map_err(|_| MissingReason::SourceUnavailable)?;
        if status.success() || (terminated && status.code().is_none()) {
            return Ok(Some(output));
        }
        return Ok(None);
    }
    let status = child.wait().map_err(|_| MissingReason::SourceUnavailable)?;
    Ok(status.success().then_some(output))
}

#[cfg(target_os = "linux")]
struct BoundedOutput {
    contents: String,
    truncated: bool,
}

#[cfg(target_os = "linux")]
fn read_bounded(mut reader: impl Read, limit: usize) -> Result<String, MissingReason> {
    let output = read_bounded_prefix(&mut reader, limit)?;
    if output.truncated {
        Err(MissingReason::InventoryExceedsBound)
    } else {
        Ok(output.contents)
    }
}

#[cfg(target_os = "linux")]
fn read_bounded_prefix(
    mut reader: impl Read,
    limit: usize,
) -> Result<BoundedOutput, MissingReason> {
    let capture_limit = limit
        .checked_add(1)
        .ok_or(MissingReason::InventoryExceedsBound)?;
    let mut output = Vec::with_capacity(capture_limit);
    reader
        .by_ref()
        .take(u64::try_from(capture_limit).map_err(|_| MissingReason::InventoryExceedsBound)?)
        .read_to_end(&mut output)
        .map_err(|_| MissingReason::SourceUnavailable)?;
    let truncated = output.len() > limit;
    if truncated {
        let _ = output.pop();
    }
    let contents = String::from_utf8(output).map_err(|_| MissingReason::UnsupportedBySource)?;
    Ok(BoundedOutput {
        contents,
        truncated,
    })
}

#[cfg(all(test, target_os = "linux"))]
#[path = "linux_tests.rs"]
mod tests;
