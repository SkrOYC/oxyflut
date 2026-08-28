//! Bounded Linux inventory helpers shared by Wayland and X11 sources.

use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};

use oxyflut_qualification::environment::{
    EnvironmentFields, EnvironmentInventory, InventoryValue, MissingReason, SystemPackage,
    SystemPackageLock,
};
use oxyflut_qualification::identifiers::EnvironmentId;

use super::EnvironmentCommandError;

const COMMAND_OUTPUT_LIMIT: usize = 4096;
const SOURCE_FILE_LIMIT: usize = 4096;
const PACKAGE_OUTPUT_LIMIT: usize = 512;
const LINUX_PACKAGE_NAMES: &[&str] = &[
    "binutils",
    "clang",
    "glib2",
    "gtk4",
    "lld",
    "rustc",
    "wayland",
    "xorg-x11-server-Xorg",
];

/// Collects a bounded Linux inventory for one Linux display environment.
pub(crate) fn collect_linux(
    environment: EnvironmentId,
) -> Result<EnvironmentInventory, EnvironmentCommandError> {
    #[cfg(target_os = "linux")]
    {
        let session = session_value(environment);
        let fields = EnvironmentFields {
            operating_system: operating_system(),
            minimum_version: InventoryValue::missing(MissingReason::NotDeclaredByLock),
            architecture: command_value("uname", &["-m"], "architecture"),
            hardware_id: hardware_model(),
            gpu_id: gpu_model(),
            driver_version: driver_version(),
            compiler_identity: compiler_identity(),
            sdk_identity: compiler_sdk_identity(),
            compositor: compositor_value(&session),
            session,
            protocol_version: InventoryValue::missing(MissingReason::SourceUnavailable),
            system_package_lock: system_package_lock(),
        };
        EnvironmentInventory::new(environment, fields).map_err(EnvironmentCommandError::Inventory)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = environment;
        Err(EnvironmentCommandError::UnsupportedHost)
    }
}

#[cfg(target_os = "linux")]
fn operating_system() -> InventoryValue {
    let Some(bytes) = read_file_bounded(Path::new("/etc/os-release"), SOURCE_FILE_LIMIT) else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Ok(contents) = String::from_utf8(bytes) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let id = os_release_value(&contents, "ID");
    let version = os_release_value(&contents, "VERSION_ID");
    match (id, version) {
        (Some(id), Some(version)) => observed_or_missing(format!("{id}-{version}")),
        _ => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

#[cfg(target_os = "linux")]
fn os_release_value<'contents>(contents: &'contents str, key: &str) -> Option<&'contents str> {
    contents.lines().find_map(|line| {
        let (name, value) = line.split_once('=')?;
        (name == key).then_some(value.trim_matches('"'))
    })
}

#[cfg(target_os = "linux")]
fn hardware_model() -> InventoryValue {
    for path in [
        "/sys/class/dmi/id/product_name",
        "/sys/devices/virtual/dmi/id/product_name",
    ] {
        if let Some(value) = read_single_line(Path::new(path)) {
            return observed_or_missing(value);
        }
    }
    InventoryValue::missing(MissingReason::SourceUnavailable)
}

#[cfg(target_os = "linux")]
fn gpu_model() -> InventoryValue {
    let directory = match fs::read_dir("/sys/class/drm") {
        Ok(directory) => directory,
        Err(_) => return gpu_from_lspci(),
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
    for path in paths {
        let vendor = read_single_line(&path.join("device/vendor"));
        let device = read_single_line(&path.join("device/device"));
        if let (Some(vendor), Some(device)) = (vendor, device) {
            return observed_or_missing(format!("pci:{vendor}:{device}"));
        }
    }
    gpu_from_lspci()
}

#[cfg(target_os = "linux")]
fn gpu_from_lspci() -> InventoryValue {
    let Some(output) = command_stdout("lspci", &["-Dn", "-d", "::0300"], COMMAND_OUTPUT_LIMIT)
    else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Ok(output) = String::from_utf8(output) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let Some(identifier) = output
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(2))
    else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    observed_or_missing(format!("pci:{identifier}"))
}

#[cfg(target_os = "linux")]
fn driver_version() -> InventoryValue {
    for path in [
        "/sys/module/nvidia/version",
        "/sys/module/amdgpu/version",
        "/sys/module/i915/version",
    ] {
        if let Some(value) = read_single_line(Path::new(path)) {
            return observed_or_missing(value);
        }
    }
    InventoryValue::missing(MissingReason::UnsupportedBySource)
}

#[cfg(target_os = "linux")]
fn compiler_identity() -> InventoryValue {
    let Some(output) = command_stdout("rustc", &["+1.98.0", "--version"], COMMAND_OUTPUT_LIMIT)
    else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Ok(output) = String::from_utf8(output) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let mut words = output.split_whitespace();
    match (words.next(), words.next()) {
        (Some(name), Some(version)) => observed_or_missing(format!("{name}-{version}")),
        _ => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

#[cfg(target_os = "linux")]
fn compiler_sdk_identity() -> InventoryValue {
    let Some(output) = command_stdout("cc", &["--version"], COMMAND_OUTPUT_LIMIT) else {
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
    observed_or_missing(format!("cc-{version}"))
}

#[cfg(target_os = "linux")]
fn session_value(environment: EnvironmentId) -> InventoryValue {
    let session_value = std::env::var_os("XDG_SESSION_TYPE");
    let Some(session) = session_value.as_deref().and_then(|value| value.to_str()) else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let expected = environment.as_str();
    if session.eq_ignore_ascii_case(expected) {
        observed_or_missing(session.to_ascii_lowercase())
    } else {
        InventoryValue::missing(MissingReason::NotActiveSession)
    }
}

#[cfg(target_os = "linux")]
fn compositor_value(session: &InventoryValue) -> InventoryValue {
    if session.is_missing() {
        return InventoryValue::missing(MissingReason::NotActiveSession);
    }
    let compositor_value = std::env::var_os("XDG_CURRENT_DESKTOP");
    let Some(compositor) = compositor_value.as_deref().and_then(|value| value.to_str()) else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    observed_or_missing(compositor.to_ascii_lowercase())
}

#[cfg(target_os = "linux")]
fn system_package_lock() -> SystemPackageLock {
    if Path::new("/usr/bin/dpkg-query").is_file() {
        return dpkg_package_lock();
    }
    if Path::new("/usr/bin/rpm").is_file() {
        return rpm_package_lock();
    }
    if Path::new("/usr/bin/nix-store").is_file() {
        return nix_store_package_lock();
    }
    SystemPackageLock::missing(MissingReason::SourceUnavailable)
}

#[cfg(target_os = "linux")]
fn dpkg_package_lock() -> SystemPackageLock {
    let mut packages = Vec::new();
    for name in LINUX_PACKAGE_NAMES {
        let Some(output) = command_stdout(
            "dpkg-query",
            &[
                "--show",
                "--showformat=${binary:Package}\\t${Version}\\n",
                name,
            ],
            PACKAGE_OUTPUT_LIMIT,
        ) else {
            continue;
        };
        let Some((name, version)) = package_fields(&output) else {
            continue;
        };
        let Ok(package) = SystemPackage::new(name, version) else {
            return SystemPackageLock::missing(MissingReason::UnsupportedBySource);
        };
        packages.push(package);
    }
    package_lock_or_missing(packages)
}

#[cfg(target_os = "linux")]
fn rpm_package_lock() -> SystemPackageLock {
    let mut packages = Vec::new();
    for name in LINUX_PACKAGE_NAMES {
        let Some(output) = command_stdout(
            "rpm",
            &[
                "-q",
                "--qf",
                "%{NAME}\\t%{VERSION}-%{RELEASE}.%{ARCH}\\n",
                name,
            ],
            PACKAGE_OUTPUT_LIMIT,
        ) else {
            continue;
        };
        let Some((name, version)) = package_fields(&output) else {
            continue;
        };
        let Ok(package) = SystemPackage::new(name, version) else {
            return SystemPackageLock::missing(MissingReason::UnsupportedBySource);
        };
        packages.push(package);
    }
    package_lock_or_missing(packages)
}

#[cfg(target_os = "linux")]
fn nix_store_package_lock() -> SystemPackageLock {
    let Some(output) = command_stdout(
        "nix-store",
        &["--query", "--hash", "/run/current-system"],
        PACKAGE_OUTPUT_LIMIT,
    ) else {
        return SystemPackageLock::missing(MissingReason::SourceUnavailable);
    };
    let Ok(value) = String::from_utf8(output) else {
        return SystemPackageLock::missing(MissingReason::UnsupportedBySource);
    };
    let Some(value) = value.lines().next() else {
        return SystemPackageLock::missing(MissingReason::UnsupportedBySource);
    };
    let Ok(package) = SystemPackage::new("nix-system-profile".to_owned(), value.to_owned()) else {
        return SystemPackageLock::missing(MissingReason::UnsupportedBySource);
    };
    package_lock_or_missing(vec![package])
}

#[cfg(target_os = "linux")]
fn package_fields(bytes: &[u8]) -> Option<(String, String)> {
    let output = std::str::from_utf8(bytes).ok()?;
    let (name, version) = output.lines().next()?.split_once('\t')?;
    Some((name.to_owned(), version.to_owned()))
}

#[cfg(target_os = "linux")]
fn package_lock_or_missing(packages: Vec<SystemPackage>) -> SystemPackageLock {
    if packages.is_empty() {
        return SystemPackageLock::missing(MissingReason::NotInstalled);
    }
    match SystemPackageLock::from_packages(packages) {
        Ok(lock) => lock,
        Err(_) => SystemPackageLock::missing(MissingReason::UnsupportedBySource),
    }
}

#[cfg(target_os = "linux")]
fn command_value(program: &str, arguments: &[&str], label: &str) -> InventoryValue {
    let Some(output) = command_stdout(program, arguments, COMMAND_OUTPUT_LIMIT) else {
        return InventoryValue::missing(MissingReason::SourceUnavailable);
    };
    let Ok(output) = String::from_utf8(output) else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    let Some(value) = output.lines().next() else {
        return InventoryValue::missing(MissingReason::UnsupportedBySource);
    };
    observed_or_missing(format!("{label}-{value}"))
}

#[cfg(target_os = "linux")]
fn observed_or_missing(value: String) -> InventoryValue {
    match InventoryValue::observed(value) {
        Ok(value) => value,
        Err(_) => InventoryValue::missing(MissingReason::UnsupportedBySource),
    }
}

#[cfg(target_os = "linux")]
fn read_single_line(path: &Path) -> Option<String> {
    let bytes = read_file_bounded(path, SOURCE_FILE_LIMIT)?;
    let value = std::str::from_utf8(&bytes).ok()?.lines().next()?.trim();
    (!value.is_empty()).then_some(value.to_owned())
}

#[cfg(target_os = "linux")]
fn read_file_bounded(path: &Path, limit: usize) -> Option<Vec<u8>> {
    let file = fs::File::open(path).ok()?;
    read_bounded(file, limit)
}

#[cfg(target_os = "linux")]
fn command_stdout(program: &str, arguments: &[&str], limit: usize) -> Option<Vec<u8>> {
    let mut child = Command::new(program)
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let stdout = child.stdout.take()?;
    let output = read_bounded(stdout, limit);
    if output.is_none() {
        let _ = child.kill();
        let _ = child.wait();
        return None;
    }
    let status = child.wait().ok()?;
    if status.success() { output } else { None }
}

#[cfg(target_os = "linux")]
fn read_bounded(mut reader: impl Read, limit: usize) -> Option<Vec<u8>> {
    let capture_limit = limit.checked_add(1)?;
    let mut output = Vec::with_capacity(capture_limit);
    reader
        .by_ref()
        .take(u64::try_from(capture_limit).ok()?)
        .read_to_end(&mut output)
        .ok()?;
    (output.len() <= limit).then_some(output)
}
