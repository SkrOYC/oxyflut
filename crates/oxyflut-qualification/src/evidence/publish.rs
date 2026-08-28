//! Content-addressed derived-evidence publication.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Cursor, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hash::hash_reader;
use crate::identifiers::RepositoryPath;

use super::canonical_json_bytes;
use super::verify::verify_existing_destination;
use super::{
    DERIVED_EVIDENCE_DIRECTORY, EvidenceError, EvidencePublication, EvidenceRef, MediaType,
};

const TEMPORARY_ATTEMPTS: u8 = 16;
const LOCK_RECOVERY_ATTEMPTS: u8 = 2;
const STALE_LOCK_AGE: Duration = Duration::from_secs(5 * 60);
static TEMPORARY_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Writes canonical JSON below one repository-relative directory at a content-addressed path.
///
/// # Errors
///
/// Returns an error when the output directory is outside the evidence root, canonical encoding fails, or publication fails.
pub fn write_canonical_json_to_directory(
    root: &Path,
    directory: &RepositoryPath,
    record: &Value,
) -> Result<EvidencePublication, EvidenceError> {
    super::ensure_evidence_path(directory)?;
    let bytes = canonical_json_bytes(record)?;
    let digest = hash_reader(Cursor::new(&bytes)).map_err(|source| EvidenceError::Io {
        path: root.join(directory.as_str()),
        source,
    })?;
    let path = RepositoryPath::parse(&format!("{}/{digest}.json", directory.as_str())).map_err(
        |source| EvidenceError::InvalidPath {
            path: directory.as_str().to_owned(),
            source,
        },
    )?;
    write_canonical_json_bytes_to_path(root, &path, &bytes, |writer, bytes| writer.write_all(bytes))
}

/// Writes canonical JSON at one repository-relative immutable evidence path.
///
/// The returned reference contains the canonical bytes' SHA-256 even when the caller-selected file name binds another immutable record, such as a provenance sidecar.
///
/// # Errors
///
/// Returns an error when the output path is outside the evidence root, canonical encoding fails, or publication fails.
pub fn write_canonical_json_to_path(
    root: &Path,
    path: &RepositoryPath,
    record: &Value,
) -> Result<EvidencePublication, EvidenceError> {
    let bytes = canonical_json_bytes(record)?;
    write_canonical_json_bytes_to_path(root, path, &bytes, |writer, bytes| writer.write_all(bytes))
}

/// Writes a content-addressed derived JSON record below the default evidence directory with verified `sourcePath` and `sourceSha256` provenance without opening its source for writing.
///
/// # Errors
///
/// Returns an error when provenance is invalid, the record isn't an object, or publication fails.
pub fn write_derived_json(
    root: &Path,
    record: &Value,
    source: &EvidenceRef,
) -> Result<EvidencePublication, EvidenceError> {
    let directory = RepositoryPath::parse(DERIVED_EVIDENCE_DIRECTORY).map_err(|source| {
        EvidenceError::InvalidPath {
            path: DERIVED_EVIDENCE_DIRECTORY.to_owned(),
            source,
        }
    })?;
    write_derived_json_to_directory(root, &directory, record, source)
}

/// Writes a content-addressed derived JSON record below one repository-relative evidence directory.
///
/// The directory must itself be below `qualification/`. The returned reference is named by the canonical record digest and retains the verified source path and digest as derived provenance.
///
/// # Errors
///
/// Returns an error when the output directory is outside the evidence root, provenance is invalid, the record isn't an object, or publication fails.
pub fn write_derived_json_to_directory(
    root: &Path,
    directory: &RepositoryPath,
    record: &Value,
    source: &EvidenceRef,
) -> Result<EvidencePublication, EvidenceError> {
    write_derived_json_to_directory_with(root, directory, record, source, |writer, bytes| {
        writer.write_all(bytes)
    })
}

#[cfg(test)]
pub(super) fn write_derived_json_with<F>(
    root: &Path,
    record: &Value,
    source: &EvidenceRef,
    write: F,
) -> Result<EvidencePublication, EvidenceError>
where
    F: FnOnce(&mut dyn Write, &[u8]) -> io::Result<()>,
{
    let directory = RepositoryPath::parse(DERIVED_EVIDENCE_DIRECTORY).map_err(|source| {
        EvidenceError::InvalidPath {
            path: DERIVED_EVIDENCE_DIRECTORY.to_owned(),
            source,
        }
    })?;
    write_derived_json_to_directory_with(root, &directory, record, source, write)
}

fn write_derived_json_to_directory_with<F>(
    root: &Path,
    directory: &RepositoryPath,
    record: &Value,
    source: &EvidenceRef,
    write: F,
) -> Result<EvidencePublication, EvidenceError>
where
    F: FnOnce(&mut dyn Write, &[u8]) -> io::Result<()>,
{
    super::ensure_evidence_path(directory)?;
    let _ = super::verify_reference(root, source)?;
    let mut record = record
        .as_object()
        .cloned()
        .ok_or(EvidenceError::DerivedRecordNotObject)?;
    if record.contains_key("sourcePath") || record.contains_key("sourceSha256") {
        return Err(EvidenceError::ReservedProvenanceField);
    }
    record.insert(
        "sourcePath".to_owned(),
        Value::String(source.path.as_str().to_owned()),
    );
    record.insert(
        "sourceSha256".to_owned(),
        Value::String(source.sha256.to_string()),
    );

    let bytes = canonical_json_bytes(&Value::Object(record))?;
    let digest = hash_reader(Cursor::new(&bytes)).map_err(|source| EvidenceError::Io {
        path: root.join(directory.as_str()),
        source,
    })?;
    let path = RepositoryPath::parse(&format!("{}/{digest}.json", directory.as_str())).map_err(
        |source| EvidenceError::InvalidPath {
            path: directory.as_str().to_owned(),
            source,
        },
    )?;
    write_canonical_json_bytes_to_path(root, &path, &bytes, write)
}

fn write_canonical_json_bytes_to_path<F>(
    root: &Path,
    path: &RepositoryPath,
    bytes: &[u8],
    write: F,
) -> Result<EvidencePublication, EvidenceError>
where
    F: FnOnce(&mut dyn Write, &[u8]) -> io::Result<()>,
{
    super::ensure_evidence_path(path)?;
    let digest = hash_reader(Cursor::new(bytes)).map_err(|source| EvidenceError::Io {
        path: root.join(path.as_str()),
        source,
    })?;
    let size_bytes = u64::try_from(bytes.len()).map_err(|_| EvidenceError::Io {
        path: root.join(path.as_str()),
        source: io::Error::other("canonical evidence exceeds supported file size"),
    })?;
    let reference = EvidenceRef {
        path: path.clone(),
        sha256: digest,
        media_type: MediaType::application_json(),
        size_bytes,
    };
    let created = publish_immutable(root, &reference, bytes, write)?;
    Ok(EvidencePublication { reference, created })
}

fn publish_immutable<F>(
    root: &Path,
    reference: &EvidenceRef,
    bytes: &[u8],
    write: F,
) -> Result<bool, EvidenceError>
where
    F: FnOnce(&mut dyn Write, &[u8]) -> io::Result<()>,
{
    let destination = root.join(reference.path.as_str());
    let _ = prepare_destination(root, &destination)?;
    let lock_path = lock_path(&destination)?;
    acquire_lock(&lock_path)?;

    let result = publish_without_replacing(root, reference, &destination, bytes, write);
    let lock_result = release_lock(&lock_path);
    match (result, lock_result) {
        (Ok(created), Ok(())) => Ok(created),
        (Ok(_), Err(error)) | (Err(error), Ok(_)) | (Err(error), Err(_)) => Err(error),
    }
}

fn publish_without_replacing<F>(
    root: &Path,
    reference: &EvidenceRef,
    destination: &Path,
    bytes: &[u8],
    write: F,
) -> Result<bool, EvidenceError>
where
    F: FnOnce(&mut dyn Write, &[u8]) -> io::Result<()>,
{
    let directory = prepare_destination(root, destination)?;
    reject_symlink_destination(destination)?;
    let (temporary_path, mut temporary_file) = open_temporary_file(destination)?;
    let write_result = write(&mut temporary_file, bytes).and_then(|()| temporary_file.sync_all());
    drop(temporary_file);
    if let Err(source) = write_result {
        return remove_temporary_after_failure(&temporary_path, source);
    }

    match fs::hard_link(&temporary_path, destination) {
        Ok(()) => {
            remove_temporary(&temporary_path)?;
            sync_directory(&directory)?;
            Ok(true)
        }
        Err(source) if source.kind() == io::ErrorKind::AlreadyExists => {
            remove_temporary(&temporary_path)?;
            reject_symlink_destination(destination)?;
            verify_existing_destination(root, reference, destination)?;
            Ok(false)
        }
        Err(source) => {
            let cleanup = remove_temporary(&temporary_path);
            match cleanup {
                Ok(()) => Err(EvidenceError::Publish {
                    path: destination.to_path_buf(),
                    source,
                }),
                Err(error) => Err(error),
            }
        }
    }
}

pub(super) fn prepare_destination(
    root: &Path,
    destination: &Path,
) -> Result<PathBuf, EvidenceError> {
    let canonical_root = fs::canonicalize(root).map_err(|source| EvidenceError::Root {
        path: root.to_path_buf(),
        source,
    })?;
    let relative_parent = destination
        .strip_prefix(root)
        .map_err(|_| EvidenceError::UnsafeDestinationParent {
            path: destination.to_path_buf(),
        })?
        .parent()
        .ok_or_else(|| EvidenceError::UnsafeDestinationParent {
            path: destination.to_path_buf(),
        })?;

    let mut current = canonical_root;
    for component in relative_parent.components() {
        let component = component.as_os_str().to_str().ok_or_else(|| {
            EvidenceError::UnsafeDestinationParent {
                path: current.clone(),
            }
        })?;
        current.push(component);
        ensure_real_directory(&current)?;
    }
    Ok(current)
}

fn ensure_real_directory(path: &Path) -> Result<(), EvidenceError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            Err(EvidenceError::UnsafeDestinationParent {
                path: path.to_path_buf(),
            })
        }
        Ok(_) => Ok(()),
        Err(source) if source.kind() == io::ErrorKind::NotFound => {
            match fs::create_dir(path) {
                Ok(()) => {}
                Err(create_error) if create_error.kind() == io::ErrorKind::AlreadyExists => {}
                Err(create_error) => {
                    return Err(EvidenceError::Io {
                        path: path.to_path_buf(),
                        source: create_error,
                    });
                }
            }
            match fs::symlink_metadata(path) {
                Ok(metadata) if !metadata.file_type().is_symlink() && metadata.is_dir() => Ok(()),
                Ok(_) => Err(EvidenceError::UnsafeDestinationParent {
                    path: path.to_path_buf(),
                }),
                Err(metadata_error) => Err(EvidenceError::Io {
                    path: path.to_path_buf(),
                    source: metadata_error,
                }),
            }
        }
        Err(source) => Err(EvidenceError::Io {
            path: path.to_path_buf(),
            source,
        }),
    }
}

fn reject_symlink_destination(destination: &Path) -> Result<(), EvidenceError> {
    match fs::symlink_metadata(destination) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            Err(EvidenceError::UnsafeDestinationParent {
                path: destination.to_path_buf(),
            })
        }
        Ok(_) => Ok(()),
        Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(EvidenceError::Io {
            path: destination.to_path_buf(),
            source,
        }),
    }
}

pub(super) fn lock_path(destination: &Path) -> Result<PathBuf, EvidenceError> {
    let file_name = destination.file_name().ok_or_else(|| EvidenceError::Io {
        path: destination.to_path_buf(),
        source: io::Error::other("derived evidence destination has no file name"),
    })?;
    let mut lock_name = file_name.to_os_string();
    lock_name.push(".lock");
    Ok(destination.with_file_name(lock_name))
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct WriterLock {
    pub(super) pid: u32,
    pub(super) created_at_unix_seconds: u64,
    pub(super) process_start_ticks: Option<u64>,
}

impl WriterLock {
    fn current() -> Self {
        let created_at_unix_seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_secs());
        Self {
            pid: std::process::id(),
            created_at_unix_seconds,
            process_start_ticks: current_process_start_ticks(),
        }
    }
}

pub(super) fn acquire_lock(path: &Path) -> Result<(), EvidenceError> {
    for _ in 0..LOCK_RECOVERY_ATTEMPTS {
        let lock = WriterLock::current();
        let bytes = serde_json::to_vec(&lock).map_err(|source| EvidenceError::Io {
            path: path.to_path_buf(),
            source: io::Error::other(source),
        })?;
        let (temporary_path, mut temporary_file) = open_temporary_file(path)?;
        let write_result = temporary_file
            .write_all(&bytes)
            .and_then(|()| temporary_file.sync_all());
        drop(temporary_file);
        if let Err(source) = write_result {
            return remove_temporary_after_failure(&temporary_path, source);
        }

        match fs::hard_link(&temporary_path, path) {
            Ok(()) => {
                remove_temporary(&temporary_path)?;
                return Ok(());
            }
            Err(source) if source.kind() == io::ErrorKind::AlreadyExists => {
                remove_temporary(&temporary_path)?;
                if reclaim_abandoned_lock(path)? {
                    continue;
                }
                return Err(EvidenceError::WriteInProgress {
                    path: path.to_path_buf(),
                });
            }
            Err(source) => {
                let cleanup = remove_temporary(&temporary_path);
                return match cleanup {
                    Ok(()) => Err(EvidenceError::Publish {
                        path: path.to_path_buf(),
                        source,
                    }),
                    Err(error) => Err(error),
                };
            }
        }
    }
    Err(EvidenceError::WriteInProgress {
        path: path.to_path_buf(),
    })
}

fn reclaim_abandoned_lock(path: &Path) -> Result<bool, EvidenceError> {
    let bytes = fs::read(path).map_err(|source| EvidenceError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let abandoned = if bytes.is_empty() {
        true
    } else {
        match serde_json::from_slice::<WriterLock>(&bytes) {
            Ok(lock) => !writer_is_alive(&lock) || writer_lock_is_expired(&lock),
            Err(_) => false,
        }
    };
    if !abandoned {
        return Ok(false);
    }
    match fs::remove_file(path) {
        Ok(()) => Ok(true),
        Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(true),
        Err(source) => Err(EvidenceError::Io {
            path: path.to_path_buf(),
            source,
        }),
    }
}

#[cfg(target_os = "linux")]
pub(super) fn current_process_start_ticks() -> Option<u64> {
    process_start_ticks(std::process::id()).ok().flatten()
}

#[cfg(not(target_os = "linux"))]
pub(super) const fn current_process_start_ticks() -> Option<u64> {
    None
}

#[cfg(target_os = "linux")]
fn writer_is_alive(lock: &WriterLock) -> bool {
    match lock.process_start_ticks {
        Some(expected_start_ticks) => match process_start_ticks(lock.pid) {
            Ok(Some(actual_start_ticks)) => actual_start_ticks == expected_start_ticks,
            Ok(None) => false,
            Err(_) => true,
        },
        None => true,
    }
}

#[cfg(not(target_os = "linux"))]
const fn writer_is_alive(_: &WriterLock) -> bool {
    true
}

fn writer_lock_is_expired(lock: &WriterLock) -> bool {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|now| now.checked_sub(Duration::from_secs(lock.created_at_unix_seconds)))
        .is_some_and(|age| age >= STALE_LOCK_AGE)
}

#[cfg(target_os = "linux")]
fn process_start_ticks(pid: u32) -> io::Result<Option<u64>> {
    let path = Path::new("/proc").join(pid.to_string()).join("stat");
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(source) if source.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(source) => return Err(source),
    };
    let Some(close_parenthesis) = contents.rfind(')') else {
        return Err(io::Error::other("invalid Linux process stat record"));
    };
    let Some(start_ticks) = contents
        .get(close_parenthesis + 1..)
        .and_then(|record| record.split_ascii_whitespace().nth(19))
    else {
        return Err(io::Error::other("incomplete Linux process stat record"));
    };
    start_ticks
        .parse()
        .map(Some)
        .map_err(|_| io::Error::other("invalid Linux process start time"))
}

pub(super) fn release_lock(path: &Path) -> Result<(), EvidenceError> {
    fs::remove_file(path).map_err(|source| EvidenceError::LockCleanup {
        path: path.to_path_buf(),
        source,
    })
}

fn open_temporary_file(destination: &Path) -> Result<(PathBuf, File), EvidenceError> {
    let directory = destination.parent().ok_or_else(|| EvidenceError::Io {
        path: destination.to_path_buf(),
        source: io::Error::other("derived evidence destination has no parent"),
    })?;
    let file_name = destination.file_name().ok_or_else(|| EvidenceError::Io {
        path: destination.to_path_buf(),
        source: io::Error::other("derived evidence destination has no file name"),
    })?;

    for _ in 0..TEMPORARY_ATTEMPTS {
        let sequence = next_temporary_sequence();
        let temporary_name = format!(
            ".{}.{}.{}.tmp",
            file_name.to_string_lossy(),
            std::process::id(),
            sequence
        );
        let temporary_path = directory.join(temporary_name);
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary_path)
        {
            Ok(file) => return Ok((temporary_path, file)),
            Err(source) if source.kind() == io::ErrorKind::AlreadyExists => {}
            Err(source) => {
                return Err(EvidenceError::Io {
                    path: temporary_path,
                    source,
                });
            }
        }
    }
    Err(EvidenceError::TemporaryFile {
        path: destination.to_path_buf(),
    })
}

pub(super) fn next_temporary_sequence() -> u64 {
    TEMPORARY_SEQUENCE.fetch_add(1, Ordering::Relaxed)
}

fn remove_temporary(temporary_path: &Path) -> Result<(), EvidenceError> {
    fs::remove_file(temporary_path).map_err(|source| EvidenceError::TemporaryCleanup {
        path: temporary_path.to_path_buf(),
        source,
    })
}

fn remove_temporary_after_failure<T>(
    temporary_path: &Path,
    write_error: io::Error,
) -> Result<T, EvidenceError> {
    remove_temporary(temporary_path)?;
    Err(EvidenceError::Io {
        path: temporary_path.to_path_buf(),
        source: write_error,
    })
}

#[cfg(unix)]
fn sync_directory(directory: &Path) -> Result<(), EvidenceError> {
    File::open(directory)
        .and_then(|directory| directory.sync_all())
        .map_err(|source| EvidenceError::Io {
            path: directory.to_path_buf(),
            source,
        })
}

#[cfg(not(unix))]
fn sync_directory(_directory: &Path) -> Result<(), EvidenceError> {
    Ok(())
}
