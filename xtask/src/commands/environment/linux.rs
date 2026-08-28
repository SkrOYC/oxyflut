//! Bounded Linux inventory collection shared by the Wayland and X11 sources.

use std::collections::BTreeMap;

#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::io::Read;
#[cfg(target_os = "linux")]
use std::path::Path;
#[cfg(target_os = "linux")]
use std::process::{Command, Stdio};

use oxyflut_qualification::environment::{
    EnvironmentFields, EnvironmentInventory, InventoryValue, MissingReason, SystemPackage,
    SystemPackageLock,
};
use oxyflut_qualification::identifiers::EnvironmentId;
use serde::Deserialize;

use super::EnvironmentCommandError;

const COMMAND_OUTPUT_LIMIT: usize = 4096;
const SOURCE_FILE_LIMIT: usize = 4096;
const PACKAGE_OUTPUT_LIMIT: usize = 512;
const MESA_DRIVER_PACKAGES: &[&str] = &["libgl1-mesa-dri", "mesa-vulkan-drivers"];

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
        collect_linux_responses(environment, &live_responses())
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
    sysfs_gpu: Vec<String>,
    lspci: Option<String>,
    drm_drivers: Vec<String>,
    rust_toolchain: Option<String>,
    compiler: Option<String>,
    session_type: Option<String>,
    current_desktop: Option<String>,
    dpkg_query: Option<BTreeMap<String, Option<String>>>,
}

fn collect_linux_responses(
    environment: EnvironmentId,
    responses: &LinuxResponses,
) -> Result<EnvironmentInventory, EnvironmentCommandError> {
    let session = session_value(environment, responses.session_type.as_deref());
    if matches!(
        session,
        InventoryValue::Missing {
            reason: MissingReason::NotActiveSession
        }
    ) {
        return Err(EnvironmentCommandError::EnvironmentMismatch);
    }
    let fields = EnvironmentFields {
        operating_system: operating_system(responses.os_release.as_deref()),
        minimum_version: InventoryValue::missing(MissingReason::NotDeclaredByLock),
        architecture: architecture(responses.uname.as_deref()),
        hardware_id: first_line_value(responses.sysfs_hardware.iter().map(String::as_str)),
        gpu_id: gpu_model(
            responses.sysfs_gpu.iter().map(String::as_str),
            responses.lspci.as_deref(),
        ),
        driver_version: driver_version(
            responses.drm_drivers.iter().map(String::as_str),
            responses.dpkg_query.as_ref(),
        ),
        compiler_identity: native_compiler_identity(responses.compiler.as_deref()),
        sdk_identity: linux_sdk_identity(responses.dpkg_query.as_ref()),
        rust_toolchain: compiler_identity(responses.rust_toolchain.as_deref()),
        compositor: compositor_value(responses.current_desktop.as_deref()),
        session,
        protocol_version: InventoryValue::missing(MissingReason::SourceUnavailable),
        system_package_lock: system_package_lock(responses.dpkg_query.as_ref()),
    };
    EnvironmentInventory::new(environment, fields).map_err(EnvironmentCommandError::Inventory)
}

fn operating_system(raw: Option<&str>) -> InventoryValue {
    let Some(contents) = raw else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
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

fn gpu_model<'responses>(
    mut sysfs_gpu: impl Iterator<Item = &'responses str>,
    lspci: Option<&str>,
) -> InventoryValue {
    if let Some(line) = sysfs_gpu.find_map(parse_sysfs_gpu) {
        return observed_or_missing(line);
    }
    gpu_from_lspci(lspci)
}

fn parse_sysfs_gpu(line: &str) -> Option<String> {
    let (vendor, device) = line.split_once('\t')?;
    pci_gpu_id(vendor, device)
}

fn gpu_from_lspci(raw: Option<&str>) -> InventoryValue {
    let Some(identifier) = raw
        .and_then(|output| output.lines().next())
        .and_then(|line| line.split_whitespace().nth(2))
        .and_then(|identifier| identifier.split_once(':'))
        .and_then(|(vendor, device)| pci_gpu_id(vendor, device))
    else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    observed_or_missing(identifier)
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

fn compiler_identity(raw: Option<&str>) -> InventoryValue {
    let Some(output) = raw else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let mut words = output.split_whitespace();
    match (words.next(), words.next()) {
        (Some(name), Some(version)) => observed_or_missing(format!("{name}-{version}")),
        _ => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

fn native_compiler_identity(raw: Option<&str>) -> InventoryValue {
    let Some(output) = raw else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Some(version) = output
        .split_whitespace()
        .find(|word| word.bytes().any(|byte| byte.is_ascii_digit()))
    else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    observed_or_missing(format!("cc-{}", atomize(version)))
}

fn linux_sdk_identity(dpkg_query: Option<&BTreeMap<String, Option<String>>>) -> InventoryValue {
    let Some(raw) = dpkg_query
        .and_then(|packages| packages.get("libc6-dev"))
        .and_then(Option::as_deref)
    else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
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

fn session_value(environment: EnvironmentId, raw: Option<&str>) -> InventoryValue {
    let Some(session) = raw.and_then(first_line) else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    if !session.eq_ignore_ascii_case(environment.as_str()) {
        return InventoryValue::missing(MissingReason::NotActiveSession);
    }
    observed_or_missing(session.to_ascii_lowercase())
}

fn compositor_value(raw: Option<&str>) -> InventoryValue {
    let Some(compositor) = raw.and_then(first_line) else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    observed_or_missing(atomize(compositor))
}

fn driver_version<'driver>(
    mut drivers: impl Iterator<Item = &'driver str>,
    dpkg_query: Option<&BTreeMap<String, Option<String>>>,
) -> InventoryValue {
    let Some(driver) = drivers.next().and_then(first_line) else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Some(packages) = dpkg_query else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let package = if driver.eq_ignore_ascii_case("nvidia") {
        packages.iter().find_map(|(name, raw)| {
            name.starts_with("nvidia-driver-")
                .then_some(raw.as_deref())
                .flatten()
                .and_then(package_fields)
        })
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

fn system_package_lock(dpkg_query: Option<&BTreeMap<String, Option<String>>>) -> SystemPackageLock {
    let Some(dpkg_query) = dpkg_query else {
        return missing_package_records(MissingReason::SourceUnavailable);
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
                    MissingReason::NotInstalled
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

fn first_line_value<'responses>(mut raw: impl Iterator<Item = &'responses str>) -> InventoryValue {
    raw.find_map(first_line).map_or_else(
        || InventoryValue::missing(MissingReason::SourceUnavailable),
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
    match InventoryValue::observed(value) {
        Ok(value) => value,
        Err(_) => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

#[cfg(target_os = "linux")]
fn live_responses() -> LinuxResponses {
    LinuxResponses {
        os_release: read_file_bounded(Path::new("/etc/os-release"), SOURCE_FILE_LIMIT),
        uname: command_stdout("uname", &["-m"], COMMAND_OUTPUT_LIMIT),
        sysfs_hardware: [
            "/sys/class/dmi/id/product_name",
            "/sys/devices/virtual/dmi/id/product_name",
        ]
        .iter()
        .filter_map(|path| read_file_bounded(Path::new(path), SOURCE_FILE_LIMIT))
        .collect(),
        sysfs_gpu: live_sysfs_gpu(),
        lspci: command_stdout("lspci", &["-Dn", "-d", "::0300"], COMMAND_OUTPUT_LIMIT),
        drm_drivers: live_drm_drivers(),
        rust_toolchain: command_stdout("rustc", &["+1.98.0", "--version"], COMMAND_OUTPUT_LIMIT),
        compiler: command_stdout("cc", &["--version"], COMMAND_OUTPUT_LIMIT),
        session_type: std::env::var("XDG_SESSION_TYPE").ok(),
        current_desktop: std::env::var("XDG_CURRENT_DESKTOP").ok(),
        dpkg_query: Path::new("/usr/bin/dpkg-query")
            .is_file()
            .then(live_dpkg_query),
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
fn live_sysfs_gpu() -> Vec<String> {
    live_drm_card_paths()
        .iter()
        .filter_map(|path| {
            let vendor = read_file_bounded(&path.join("device/vendor"), SOURCE_FILE_LIMIT)?;
            let device = read_file_bounded(&path.join("device/device"), SOURCE_FILE_LIMIT)?;
            Some(format!(
                "{}\t{}",
                first_line(&vendor)?,
                first_line(&device)?
            ))
        })
        .collect()
}

#[cfg(target_os = "linux")]
fn live_drm_drivers() -> Vec<String> {
    live_drm_card_paths()
        .iter()
        .filter_map(|path| fs::read_link(path.join("device/driver")).ok())
        .filter_map(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(str::to_owned)
        })
        .collect()
}

#[cfg(target_os = "linux")]
fn live_dpkg_query() -> BTreeMap<String, Option<String>> {
    let mut output = BTreeMap::new();
    for alternatives in LINUX_PACKAGE_REQUIREMENTS {
        for name in *alternatives {
            output.insert(
                (*name).to_owned(),
                command_stdout(
                    "dpkg-query",
                    &[
                        "--show",
                        "--showformat=${binary:Package}\\t${Version}\\n",
                        name,
                    ],
                    PACKAGE_OUTPUT_LIMIT,
                ),
            );
        }
    }
    for name in MESA_DRIVER_PACKAGES {
        output.insert(
            (*name).to_owned(),
            command_stdout(
                "dpkg-query",
                &[
                    "--show",
                    "--showformat=${binary:Package}\\t${Version}\\n",
                    name,
                ],
                PACKAGE_OUTPUT_LIMIT,
            ),
        );
    }
    for record in command_stdout(
        "dpkg-query",
        &[
            "--show",
            "--showformat=${binary:Package}\\t${Version}\\n",
            "nvidia-driver-*",
        ],
        COMMAND_OUTPUT_LIMIT,
    )
    .into_iter()
    .flat_map(|records| records.lines().map(str::to_owned).collect::<Vec<_>>())
    {
        if let Some((name, _)) = package_fields(&record) {
            output.insert(name, Some(format!("{record}\n")));
        }
    }
    output
}

#[cfg(target_os = "linux")]
fn read_file_bounded(path: &Path, limit: usize) -> Option<String> {
    let file = fs::File::open(path).ok()?;
    read_bounded(file, limit).ok()
}

#[cfg(target_os = "linux")]
fn command_stdout(program: &str, arguments: &[&str], limit: usize) -> Option<String> {
    let mut child = Command::new(program)
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let stdout = child.stdout.take()?;
    let Ok(output) = read_bounded(stdout, limit) else {
        let _ = child.kill();
        let _ = child.wait();
        return None;
    };
    let status = child.wait().ok()?;
    status.success().then_some(output)
}

#[cfg(target_os = "linux")]
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
