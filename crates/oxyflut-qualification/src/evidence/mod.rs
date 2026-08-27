//! Canonical local evidence writing and verification.
//!
//! ADR-0008 requires immutable UTF-8 JSON evidence below the repository's `qualification/` evidence root. Content-addressed derived paths deduplicate equal bytes and reject collisions. Schema-typed reference discovery treats a `path` without `sha256` as ordinary data; a declared `sha256` requires paired string path and digest values, except for a paired null optional reference.

mod canonical;
mod publish;
mod references;
mod verify;

use crate::hash::{DigestParseError, Sha256Digest};
use crate::identifiers::{IdentifierError, RepositoryPath};
use serde_json::Value;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub use canonical::canonical_json_bytes;
pub use publish::{write_derived_json, write_derived_json_to_directory};
pub use references::{
    DeclaredEvidenceReference, DeclaredReferenceError, ReferenceDeclaration, declared_references,
    reference_declaration,
};
pub use verify::{preserve_source, verify_file, verify_path_digest, verify_reference};

/// The repository-relative root that contains all qualification evidence.
pub const REPOSITORY_EVIDENCE_ROOT: &str = "qualification";
pub(super) const DERIVED_EVIDENCE_DIRECTORY: &str = "qualification/evidence/derived";
pub(super) const JSON_MEDIA_TYPE: &str = "application/json";

/// A recorded media type for immutable evidence bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaType(String);

impl MediaType {
    /// Parses a nonempty media type without whitespace or control characters.
    ///
    /// # Errors
    ///
    /// Returns [`EvidenceError::InvalidMediaType`] when the value cannot identify a media type.
    pub fn parse(value: &str) -> Result<Self, EvidenceError> {
        let slash = value.find('/');
        let is_valid = !value.is_empty()
            && slash.is_some_and(|index| index > 0 && index + 1 < value.len())
            && !value
                .chars()
                .any(|character| character.is_control() || character.is_whitespace());
        if is_valid {
            Ok(Self(value.to_owned()))
        } else {
            Err(EvidenceError::InvalidMediaType {
                media_type: value.to_owned(),
            })
        }
    }

    /// Returns the registered JSON media type.
    #[must_use]
    pub fn application_json() -> Self {
        Self(JSON_MEDIA_TYPE.to_owned())
    }

    /// Returns the recorded media type text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(super) fn is_json(&self) -> bool {
        self.0 == JSON_MEDIA_TYPE || self.0.ends_with("+json")
    }
}

/// A repository-confined immutable evidence reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceRef {
    /// Canonical repository-relative path below [`REPOSITORY_EVIDENCE_ROOT`].
    pub path: RepositoryPath,
    /// Streamed SHA-256 digest of the referenced bytes.
    pub sha256: Sha256Digest,
    /// Recorded media type of the referenced bytes.
    pub media_type: MediaType,
    /// Exact number of referenced bytes.
    pub size_bytes: u64,
}

/// A verified local evidence file.
#[derive(Clone, Debug)]
pub struct VerifiedEvidence {
    pub(super) path: RepositoryPath,
    pub(super) resolved_path: PathBuf,
    pub(super) sha256: Sha256Digest,
    pub(super) size_bytes: u64,
    pub(super) json: Option<Value>,
}

impl VerifiedEvidence {
    /// Returns the canonical repository-relative path that was verified.
    #[must_use]
    pub fn path(&self) -> &RepositoryPath {
        &self.path
    }

    /// Returns the canonical local path used for verification.
    #[must_use]
    pub fn resolved_path(&self) -> &Path {
        &self.resolved_path
    }

    /// Returns the streamed SHA-256 digest of the verified bytes.
    #[must_use]
    pub const fn sha256(&self) -> Sha256Digest {
        self.sha256
    }

    /// Returns the exact byte count of the verified file.
    #[must_use]
    pub const fn size_bytes(&self) -> u64 {
        self.size_bytes
    }

    /// Returns decoded JSON when the caller supplied a JSON media type.
    #[must_use]
    pub fn json(&self) -> Option<&Value> {
        self.json.as_ref()
    }
}

/// Reports why evidence bytes, references, provenance, or local publication are invalid.
#[derive(Debug, Error)]
pub enum EvidenceError {
    /// A media type was empty, incomplete, or contained whitespace or controls.
    #[error("evidence media type is invalid")]
    InvalidMediaType {
        /// The rejected media type text.
        media_type: String,
    },
    /// A repository-relative path violated the canonical path contract.
    #[error("evidence path is not canonical")]
    InvalidPath {
        /// The rejected path text.
        path: String,
        /// The canonical path rule that rejected the path.
        #[source]
        source: IdentifierError,
    },
    /// A canonical path was outside the repository evidence root.
    #[error("evidence path is outside the repository evidence root")]
    OutsideEvidenceRoot {
        /// The path that did not begin below the evidence root.
        path: RepositoryPath,
    },
    /// The repository root couldn't be resolved before local verification.
    #[error("could not resolve the repository root")]
    Root {
        /// The requested repository root.
        path: PathBuf,
        /// The local filesystem failure.
        #[source]
        source: io::Error,
    },
    /// A referenced local evidence file didn't exist.
    #[error("evidence file is missing")]
    MissingFile {
        /// The missing canonical evidence path.
        path: RepositoryPath,
        /// The local filesystem failure.
        #[source]
        source: io::Error,
    },
    /// A local filesystem operation couldn't complete.
    #[error("local evidence I/O failed")]
    Io {
        /// The local path involved in the failed operation.
        path: PathBuf,
        /// The local filesystem failure.
        #[source]
        source: io::Error,
    },
    /// A resolved symlink or filesystem indirection escaped the repository root.
    #[error("evidence path escapes the repository evidence root")]
    SymlinkEscape {
        /// The canonical evidence path that escaped the root.
        path: RepositoryPath,
    },
    /// A referenced evidence path didn't resolve to a regular file.
    #[error("evidence path is not a regular file")]
    NotRegularFile {
        /// The canonical evidence path that wasn't a regular file.
        path: RepositoryPath,
    },
    /// A destination parent was not a real directory below the repository root.
    #[error("evidence destination parent is unsafe")]
    UnsafeDestinationParent {
        /// The parent that was a symlink or not a directory.
        path: PathBuf,
    },
    /// A declared SHA-256 value wasn't lowercase hexadecimal.
    #[error("evidence SHA-256 is invalid")]
    InvalidDigest {
        /// The path associated with the invalid digest.
        path: RepositoryPath,
        /// The digest parser failure.
        #[source]
        source: DigestParseError,
    },
    /// A streamed digest differed from the declared immutable digest.
    #[error("evidence SHA-256 does not match")]
    DigestMismatch {
        /// The path whose digest didn't match.
        path: RepositoryPath,
    },
    /// The evidence size didn't match the declared immutable size.
    #[error("evidence size does not match")]
    SizeMismatch {
        /// The path whose size didn't match.
        path: RepositoryPath,
    },
    /// A file changed while its bytes were read for evidence verification.
    #[error("evidence file changed while it was read")]
    ChangedDuringRead {
        /// The canonical evidence path whose metadata changed.
        path: RepositoryPath,
    },
    /// JSON evidence couldn't be decoded as UTF-8 JSON.
    #[error("evidence JSON is invalid")]
    Json {
        /// The local JSON path.
        path: PathBuf,
        /// The JSON decoding failure.
        #[source]
        source: serde_json::Error,
    },
    /// A canonical JSON encoder couldn't serialize one JSON string.
    #[error("canonical evidence JSON could not be encoded")]
    JsonEncoding {
        /// The JSON encoding failure.
        #[source]
        source: serde_json::Error,
    },
    /// A derived record wasn't a JSON object.
    #[error("derived evidence record must be a JSON object")]
    DerivedRecordNotObject,
    /// A derived record attempted to replace reserved provenance fields.
    #[error("derived evidence record contains reserved provenance fields")]
    ReservedProvenanceField,
    /// A derived file had only one of its required source provenance fields.
    #[error("derived evidence provenance is incomplete")]
    IncompleteProvenance,
    /// A derived file's source provenance field had an invalid type or value.
    #[error("derived evidence provenance is invalid")]
    InvalidProvenance,
    /// A derived JSON file didn't use the canonical UTF-8 representation.
    #[error("derived evidence JSON is not canonical")]
    NonCanonicalJson {
        /// The local derived JSON path.
        path: PathBuf,
    },
    /// A content-addressed destination existed with bytes other than its address.
    #[error("content-addressed evidence destination conflicts with existing bytes")]
    ContentAddressCollision {
        /// The conflicting local destination.
        path: PathBuf,
    },
    /// Another local writer held the content-addressed destination lock.
    #[error("content-addressed evidence write is already in progress")]
    WriteInProgress {
        /// The lock file owned by the competing writer.
        path: PathBuf,
    },
    /// A temporary output file couldn't be created after bounded attempts.
    #[error("could not create a temporary evidence file")]
    TemporaryFile {
        /// The destination whose temporary sibling couldn't be created.
        path: PathBuf,
    },
    /// A temporary output couldn't be removed after a failed write or publication.
    #[error("could not remove a temporary evidence file")]
    TemporaryCleanup {
        /// The temporary output path.
        path: PathBuf,
        /// The local filesystem failure.
        #[source]
        source: io::Error,
    },
    /// A completed temporary output couldn't be published without replacing an existing file.
    #[error("could not publish local evidence without replacing an existing file")]
    Publish {
        /// The final evidence destination.
        path: PathBuf,
        /// The local filesystem failure.
        #[source]
        source: io::Error,
    },
    /// A write lock couldn't be released after publication completed.
    #[error("could not release the local evidence write lock")]
    LockCleanup {
        /// The write lock that couldn't be removed.
        path: PathBuf,
        /// The local filesystem failure.
        #[source]
        source: io::Error,
    },
}

impl EvidenceError {
    /// Returns the stable content-free code for this evidence failure.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidMediaType { .. } => "invalid-media-type",
            Self::InvalidPath { .. } => "invalid-path",
            Self::OutsideEvidenceRoot { .. } => "outside-evidence-root",
            Self::Root { .. } => "repository-root",
            Self::MissingFile { .. } => "missing-file",
            Self::Io { .. } => "local-io",
            Self::SymlinkEscape { .. } => "symlink-escape",
            Self::NotRegularFile { .. } => "not-regular-file",
            Self::UnsafeDestinationParent { .. } => "unsafe-destination",
            Self::InvalidDigest { .. } => "invalid-digest",
            Self::DigestMismatch { .. } => "digest-mismatch",
            Self::SizeMismatch { .. } => "size-mismatch",
            Self::ChangedDuringRead { .. } => "changed-during-read",
            Self::Json { .. } => "invalid-json",
            Self::JsonEncoding { .. } => "json-encoding",
            Self::DerivedRecordNotObject => "derived-record-not-object",
            Self::ReservedProvenanceField => "reserved-provenance-field",
            Self::IncompleteProvenance => "incomplete-provenance",
            Self::InvalidProvenance => "invalid-provenance",
            Self::NonCanonicalJson { .. } => "noncanonical-json",
            Self::ContentAddressCollision { .. } => "content-address-collision",
            Self::WriteInProgress { .. } => "write-in-progress",
            Self::TemporaryFile { .. } => "temporary-file",
            Self::TemporaryCleanup { .. } => "temporary-cleanup",
            Self::Publish { .. } => "publish-failed",
            Self::LockCleanup { .. } => "lock-cleanup",
        }
    }
}

pub(super) fn ensure_evidence_path(path: &RepositoryPath) -> Result<(), EvidenceError> {
    if path
        .as_str()
        .strip_prefix(REPOSITORY_EVIDENCE_ROOT)
        .is_some_and(|suffix| suffix.starts_with('/'))
    {
        Ok(())
    } else {
        Err(EvidenceError::OutsideEvidenceRoot { path: path.clone() })
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::io::{self, Write};
    use std::path::{Path, PathBuf};

    use serde_json::{Value, json};

    use super::publish::{WriterLock, acquire_lock, lock_path, prepare_destination, release_lock};
    use super::{
        EvidenceError, MediaType, RepositoryPath, canonical_json_bytes, preserve_source,
        verify_file, verify_reference, write_derived_json,
    };

    const FIXTURE_DIRECTORY: &str = "qualification/fixtures/evidence";

    #[test]
    fn declared_reference_walker_skips_untyped_paths_and_rejects_mismatched_digests() {
        let skipped = json!({"path": "ordinary-data"});
        assert_eq!(
            super::declared_references("urn:oxyflut:schema:qualification-evidence:5", &skipped),
            Ok(Vec::new())
        );
        let null_digest = json!({"path": "qualification/proof.json", "sha256": null});
        assert!(matches!(
            super::declared_references("urn:oxyflut:schema:qualification-evidence:5", &null_digest),
            Err(super::DeclaredReferenceError::IncompleteReference)
        ));
        let artifact = json!({"files": [{"path": "bin/oxyflut", "sha256": "not-a-reference"}]});
        assert_eq!(
            super::declared_references("urn:oxyflut:schema:artifact-manifest:4", &artifact),
            Ok(Vec::new())
        );
    }

    #[test]
    fn equal_logical_records_produce_byte_identical_derived_json() -> Result<(), Box<dyn Error>> {
        let values = fixture_json("equal-logical-records.json")?;
        let first = values
            .get("first")
            .ok_or("fixture must contain first record")?;
        let second = values
            .get("second")
            .ok_or("fixture must contain second record")?;
        assert_eq!(canonical_json_bytes(first)?, canonical_json_bytes(second)?);

        let root = TestRepository::new("equal-records")?;
        copy_fixture(root.path(), "preserved-source.json")?;
        let source = preserved_source(root.path())?;
        let first_reference = write_derived_json(root.path(), first, &source)?;
        let second_reference = write_derived_json(root.path(), second, &source)?;
        assert_eq!(first_reference, second_reference);
        assert_eq!(
            fs::read(root.path().join(first_reference.path.as_str()))?,
            fs::read(root.path().join(second_reference.path.as_str()))?
        );
        Ok(())
    }

    #[test]
    fn source_bytes_remain_preserved_and_derived_provenance_verifies() -> Result<(), Box<dyn Error>>
    {
        let root = TestRepository::new("preserved-source")?;
        let source_path = copy_fixture(root.path(), "preserved-source.json")?;
        let before = fs::read(&source_path)?;
        let source = preserved_source(root.path())?;
        let record = fixture_json("partial-write.json")?;
        let derived = write_derived_json(root.path(), &record, &source)?;

        assert_eq!(fs::read(&source_path)?, before);
        let verified = verify_reference(root.path(), &derived)?;
        let provenance = verified
            .json()
            .and_then(Value::as_object)
            .ok_or("derived record must decode as an object")?;
        assert_eq!(
            provenance.get("sourcePath").and_then(Value::as_str),
            Some(source.path.as_str())
        );
        let source_digest = source.sha256.to_string();
        assert_eq!(
            provenance.get("sourceSha256").and_then(Value::as_str),
            Some(source_digest.as_str())
        );
        Ok(())
    }

    #[test]
    fn path_digest_verification_streams_without_decoding_json() -> Result<(), Box<dyn Error>> {
        let root = TestRepository::new("streaming-path-digest")?;
        let path = RepositoryPath::parse("qualification/proof.bin")?;
        let resolved_path = root.path().join(path.as_str());
        let parent = resolved_path
            .parent()
            .ok_or("streaming fixture must have a parent")?;
        fs::create_dir_all(parent)?;
        let bytes = vec![b'x'; 2 * 64 * 1024 + 1];
        let expected_size = u64::try_from(bytes.len())?;
        fs::write(&resolved_path, bytes)?;
        let digest = crate::hash::hash_file(&resolved_path)?;

        let verified = super::verify_path_digest(root.path(), &path, &digest)?;
        assert_eq!(verified.size_bytes(), expected_size);
        assert!(verified.json().is_none());
        Ok(())
    }

    #[test]
    fn bad_digest_and_out_of_root_fixtures_fail_closed() -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let media_type = MediaType::application_json();
        let bad_digest = RepositoryPath::parse("qualification/fixtures/evidence/bad-digest.json")?;
        assert!(matches!(
            verify_file(&root, &bad_digest, &media_type),
            Err(EvidenceError::DigestMismatch { .. })
        ));
        let out_of_root =
            RepositoryPath::parse("qualification/fixtures/evidence/out-of-root.json")?;
        assert!(matches!(
            verify_file(&root, &out_of_root, &media_type),
            Err(EvidenceError::OutsideEvidenceRoot { .. })
        ));
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn verification_rejects_a_symlink_that_leaves_the_evidence_root() -> Result<(), Box<dyn Error>>
    {
        use std::os::unix::fs::symlink;

        let root = TestRepository::new("evidence-root-symlink")?;
        let outside = root.path().join("outside");
        fs::create_dir_all(root.path().join("qualification"))?;
        fs::create_dir_all(&outside)?;
        let outside_file = outside.join("proof.json");
        fs::write(&outside_file, b"{}")?;
        symlink(&outside_file, root.path().join("qualification/proof.json"))?;

        let result = verify_file(
            root.path(),
            &RepositoryPath::parse("qualification/proof.json")?,
            &MediaType::application_json(),
        );
        assert!(matches!(result, Err(EvidenceError::SymlinkEscape { .. })));
        Ok(())
    }

    #[test]
    fn interrupted_write_publishes_no_partial_file_or_reference() -> Result<(), Box<dyn Error>> {
        let root = TestRepository::new("interrupted-write")?;
        let source_path = copy_fixture(root.path(), "preserved-source.json")?;
        let source = preserved_source(root.path())?;
        let record = fixture_json("partial-write.json")?;
        let result = super::publish::write_derived_json_with(
            root.path(),
            &record,
            &source,
            |writer, bytes| {
                let mut failing_writer = FailingWriter::new(writer, bytes.len() / 2);
                failing_writer.write_all(bytes)
            },
        );
        assert!(result.is_err());
        assert_eq!(
            fs::read(&source_path)?,
            fs::read(fixture_path("preserved-source.json"))?
        );

        let derived_directory = root.path().join("qualification/evidence/derived");
        let files = if derived_directory.exists() {
            fs::read_dir(derived_directory)?.collect::<Result<Vec<_>, io::Error>>()?
        } else {
            Vec::new()
        };
        assert!(
            files.is_empty(),
            "interrupted writes must leave no published files"
        );
        Ok(())
    }

    #[test]
    fn publication_rejects_a_destination_created_after_the_writer_starts()
    -> Result<(), Box<dyn Error>> {
        let root = TestRepository::new("no-replace-race")?;
        copy_fixture(root.path(), "preserved-source.json")?;
        let source = preserved_source(root.path())?;
        let record = fixture_json("partial-write.json")?;
        let destination = expected_derived_destination(root.path(), &record, &source)?;
        let result = super::publish::write_derived_json_with(
            root.path(),
            &record,
            &source,
            |writer, bytes| {
                writer.write_all(bytes)?;
                fs::write(&destination, b"competing content")
            },
        );
        assert!(matches!(
            result,
            Err(EvidenceError::ContentAddressCollision { .. })
        ));
        assert_eq!(fs::read(destination)?, b"competing content");
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn publication_rejects_symlinked_destination_parents_before_writing()
    -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::symlink;

        let root = TestRepository::new("symlinked-destination")?;
        copy_fixture(root.path(), "preserved-source.json")?;
        let outside = TestRepository::new("symlink-target")?;
        let evidence_directory = root.path().join("qualification/evidence");
        fs::create_dir_all(&evidence_directory)?;
        symlink(outside.path(), evidence_directory.join("derived"))?;

        let source = preserved_source(root.path())?;
        let record = fixture_json("partial-write.json")?;
        let result = write_derived_json(root.path(), &record, &source);
        assert!(matches!(
            result,
            Err(EvidenceError::UnsafeDestinationParent { .. })
        ));
        assert!(
            fs::read_dir(outside.path())?.next().is_none(),
            "no lock, temporary file, or final evidence may escape through a destination symlink"
        );
        Ok(())
    }

    #[test]
    fn publication_recovers_an_expired_writer_lock_on_every_host() -> Result<(), Box<dyn Error>> {
        let root = TestRepository::new("expired-lock")?;
        copy_fixture(root.path(), "preserved-source.json")?;
        let source = preserved_source(root.path())?;
        let record = fixture_json("partial-write.json")?;
        let destination = expected_derived_destination(root.path(), &record, &source)?;
        let _ = prepare_destination(root.path(), &destination)?;
        let lock = lock_path(&destination)?;
        let expired = WriterLock {
            pid: std::process::id(),
            created_at_unix_seconds: 0,
            process_start_ticks: super::publish::current_process_start_ticks(),
        };
        fs::write(lock, serde_json::to_vec(&expired)?)?;

        let reference = write_derived_json(root.path(), &record, &source)?;
        assert!(root.path().join(reference.path.as_str()).is_file());
        Ok(())
    }

    #[test]
    fn publication_recovers_an_empty_legacy_lock_after_an_interrupted_write()
    -> Result<(), Box<dyn Error>> {
        let root = TestRepository::new("empty-legacy-lock")?;
        copy_fixture(root.path(), "preserved-source.json")?;
        let source = preserved_source(root.path())?;
        let record = fixture_json("partial-write.json")?;
        let destination = expected_derived_destination(root.path(), &record, &source)?;
        let _ = prepare_destination(root.path(), &destination)?;
        let lock = lock_path(&destination)?;
        fs::write(lock, [])?;

        let reference = write_derived_json(root.path(), &record, &source)?;
        assert!(root.path().join(reference.path.as_str()).is_file());
        Ok(())
    }

    #[test]
    fn publication_reports_a_live_writer_lock() -> Result<(), Box<dyn Error>> {
        let root = TestRepository::new("live-lock")?;
        copy_fixture(root.path(), "preserved-source.json")?;
        let source = preserved_source(root.path())?;
        let record = fixture_json("partial-write.json")?;
        let destination = expected_derived_destination(root.path(), &record, &source)?;
        let _ = prepare_destination(root.path(), &destination)?;
        let lock = lock_path(&destination)?;
        acquire_lock(&lock)?;

        let result = write_derived_json(root.path(), &record, &source);
        assert!(matches!(result, Err(EvidenceError::WriteInProgress { .. })));
        release_lock(&lock)?;
        Ok(())
    }

    fn expected_derived_destination(
        root: &Path,
        record: &Value,
        source: &super::EvidenceRef,
    ) -> Result<PathBuf, Box<dyn Error>> {
        let mut record = record
            .as_object()
            .cloned()
            .ok_or("derived record must be an object")?;
        record.insert(
            "sourcePath".to_owned(),
            Value::String(source.path.as_str().to_owned()),
        );
        record.insert(
            "sourceSha256".to_owned(),
            Value::String(source.sha256.to_string()),
        );
        let bytes = canonical_json_bytes(&Value::Object(record))?;
        let digest = crate::hash::hash_reader(std::io::Cursor::new(bytes))?;
        Ok(root.join(format!("qualification/evidence/derived/{digest}.json")))
    }

    fn preserved_source(root: &Path) -> Result<super::EvidenceRef, Box<dyn Error>> {
        Ok(preserve_source(
            root,
            RepositoryPath::parse("qualification/fixtures/evidence/preserved-source.json")?,
            MediaType::application_json(),
        )?)
    }

    fn fixture_json(name: &str) -> Result<Value, Box<dyn Error>> {
        Ok(serde_json::from_slice(&fs::read(fixture_path(name))?)?)
    }

    fn fixture_path(name: &str) -> PathBuf {
        workspace_root_path().join(FIXTURE_DIRECTORY).join(name)
    }

    fn copy_fixture(root: &Path, name: &str) -> Result<PathBuf, Box<dyn Error>> {
        let destination = root.join(FIXTURE_DIRECTORY).join(name);
        let parent = destination
            .parent()
            .ok_or("fixture destination must have a parent")?;
        fs::create_dir_all(parent)?;
        fs::copy(fixture_path(name), &destination)?;
        Ok(destination)
    }

    fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
        Ok(workspace_root_path())
    }

    fn workspace_root_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."))
    }

    struct TestRepository {
        path: PathBuf,
    }

    impl TestRepository {
        fn new(name: &str) -> Result<Self, Box<dyn Error>> {
            let sequence = super::publish::next_temporary_sequence();
            let path = std::env::temp_dir().join(format!(
                "oxyflut-evidence-{name}-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir_all(&path)?;
            Ok(Self { path })
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestRepository {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    struct FailingWriter<'writer> {
        writer: &'writer mut dyn Write,
        remaining: usize,
    }

    impl<'writer> FailingWriter<'writer> {
        fn new(writer: &'writer mut dyn Write, remaining: usize) -> Self {
            Self { writer, remaining }
        }
    }

    impl Write for FailingWriter<'_> {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            if self.remaining == 0 {
                return Err(io::Error::other("simulated interrupted write"));
            }
            let count = buffer.len().min(self.remaining);
            let written = self.writer.write(&buffer[..count])?;
            self.remaining -= written;
            Ok(written)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.writer.flush()
        }
    }
}
