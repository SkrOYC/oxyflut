//! Candidate-neutral capability-baseline validation and canonical authoring.

use std::fs;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};

use oxyflut_qualification::baseline::{BaselineAuthority, BaselineError, CapabilityBaseline};
use oxyflut_qualification::evidence::{
    EvidenceError, MediaType, preserve_source, write_canonical_json_to_directory,
    write_canonical_json_to_path,
};
use oxyflut_qualification::hash::hash_reader;
use oxyflut_qualification::identifiers::RepositoryPath;
use oxyflut_qualification::schema::SchemaError;
use serde_json::Value;
use thiserror::Error;

use super::super::{CommandError, CommandOutcome};
use crate::contracts::{schema, traceability};

const BASELINE_SCHEMA: &str = "urn:oxyflut:schema:capability-baseline:4";

/// Validates one candidate-neutral baseline and optionally writes its canonical derived form.
pub(crate) fn run(arguments: &[String]) -> CommandOutcome {
    let root = match workspace_root() {
        Ok(root) => root,
        Err(()) => {
            return CommandOutcome::failed(CommandError::Execution {
                code: "workspace-root",
                hint: "rerun: baseline validate --input PATH",
            });
        }
    };
    run_at_root(&root, arguments)
}

fn run_at_root(root: &Path, arguments: &[String]) -> CommandOutcome {
    let (input, output) = match parse_arguments(arguments) {
        Ok(arguments) => arguments,
        Err(()) => {
            return CommandOutcome::failed(CommandError::InvalidInput {
                code: "baseline-validate-arguments",
            });
        }
    };

    match validate_at(root, &input, output.as_ref()) {
        Ok(Some(reference)) => {
            println!("baseline validate: ok ({})", reference.path.as_str());
            CommandOutcome::Success
        }
        Ok(None) => {
            println!("baseline validate: ok");
            CommandOutcome::Success
        }
        Err(_) => CommandOutcome::failed(CommandError::ValidationFailed {
            code: "baseline-invalid",
            hint: "rerun: baseline validate --input PATH",
        }),
    }
}

fn parse_arguments(arguments: &[String]) -> Result<(RepositoryPath, Option<RepositoryPath>), ()> {
    let (input, output) = match arguments {
        [input_flag, input] if input_flag == "--input" => (input, None),
        [input_flag, input, output_flag, output]
            if input_flag == "--input" && output_flag == "--output" =>
        {
            (input, Some(output))
        }
        _ => return Err(()),
    };
    let input = RepositoryPath::parse(input).map_err(|_| ())?;
    let output = output
        .map(|path| RepositoryPath::parse(path).map_err(|_| ()))
        .transpose()?;
    Ok((input, output))
}

fn validate_at(
    root: &Path,
    input: &RepositoryPath,
    output: Option<&RepositoryPath>,
) -> Result<Option<oxyflut_qualification::evidence::EvidenceRef>, BaselineCommandError> {
    validate_at_with_before_publication(root, input, output, || Ok(()))
}

fn validate_at_with_before_publication<F>(
    root: &Path,
    input: &RepositoryPath,
    output: Option<&RepositoryPath>,
    before_publication: F,
) -> Result<Option<oxyflut_qualification::evidence::EvidenceRef>, BaselineCommandError>
where
    F: FnOnce() -> Result<(), BaselineCommandError>,
{
    let bytes = read_confined_file(root, input)?;
    let baseline = CapabilityBaseline::parse_json(&bytes)?;
    let raw: Value = serde_json::from_slice(&bytes).map_err(BaselineCommandError::Json)?;

    let registry = schema::compile_workspace(root)?;
    registry
        .validate(BASELINE_SCHEMA, &raw)
        .map_err(BaselineCommandError::Schema)?;
    let (specification_version, capabilities) = traceability::capability_baseline_authority(root)?;
    let authority = BaselineAuthority::new(specification_version.clone(), capabilities.clone())?;
    baseline.validate_structure(root, &authority)?;
    traceability::validate_capability_baseline(root, &raw, &specification_version, &capabilities)?;

    let output = match output {
        Some(output) => {
            let validated_digest =
                hash_reader(Cursor::new(&bytes)).map_err(|source| BaselineCommandError::Hash {
                    path: root.join(input.as_str()),
                    source,
                })?;
            before_publication()?;
            let source = preserve_source(root, input.clone(), MediaType::application_json())?;
            if source.sha256 != validated_digest {
                return Err(BaselineCommandError::SourceSnapshotMismatch);
            }

            let canonical = baseline.canonical_value()?;
            let draft = write_canonical_json_to_directory(root, output, &canonical)?;
            let sidecar_path_text = format!("{}/{}.provenance.json", output.as_str(), draft.sha256);
            let sidecar_path = RepositoryPath::parse(&sidecar_path_text).map_err(|source| {
                EvidenceError::InvalidPath {
                    path: sidecar_path_text,
                    source,
                }
            })?;
            let sidecar = serde_json::json!({
                "path": draft.path.as_str(),
                "sha256": draft.sha256.to_string(),
                "mediaType": draft.media_type.as_str(),
                "sourcePath": source.path.as_str(),
                "sourceSha256": source.sha256.to_string(),
            });
            let _ = write_canonical_json_to_path(root, &sidecar_path, &sidecar)?;
            Some(draft)
        }
        None => None,
    };
    Ok(output)
}

fn read_confined_file(root: &Path, path: &RepositoryPath) -> Result<Vec<u8>, BaselineCommandError> {
    let canonical_root = fs::canonicalize(root).map_err(|source| BaselineCommandError::Io {
        path: root.to_path_buf(),
        source,
    })?;
    let unresolved = root.join(path.as_str());
    let resolved = fs::canonicalize(&unresolved).map_err(|source| BaselineCommandError::Io {
        path: unresolved,
        source,
    })?;
    if !resolved.starts_with(&canonical_root) {
        return Err(BaselineCommandError::InputPath);
    }
    let metadata = fs::metadata(&resolved).map_err(|source| BaselineCommandError::Io {
        path: resolved.clone(),
        source,
    })?;
    if !metadata.is_file() {
        return Err(BaselineCommandError::InputPath);
    }
    fs::read(&resolved).map_err(|source| BaselineCommandError::Io {
        path: resolved,
        source,
    })
}

fn workspace_root() -> Result<PathBuf, ()> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or(())
}

#[derive(Debug, Error)]
enum BaselineCommandError {
    #[error("baseline input path is invalid")]
    InputPath,
    #[error("baseline input could not be read")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("baseline input could not be hashed")]
    Hash {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("baseline input changed between validation and publication")]
    SourceSnapshotMismatch,
    #[error("baseline input is not JSON")]
    Json(#[source] serde_json::Error),
    #[error("baseline schema validation failed")]
    Schema(#[source] SchemaError),
    #[error("baseline schema registry failed")]
    SchemaRegistry(#[from] schema::ContractSchemaError),
    #[error("baseline exact-set validation failed")]
    Traceability(#[from] traceability::TraceabilityError),
    #[error("baseline structural validation failed")]
    Baseline(#[from] BaselineError),
    #[error("baseline evidence publication failed")]
    Evidence(#[from] EvidenceError),
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::path::{Path, PathBuf};

    use oxyflut_qualification::evidence::{
        MediaType, canonical_json_bytes, verify_file, verify_path_digest,
    };
    use oxyflut_qualification::hash::hash_file;
    use oxyflut_qualification::identifiers::RepositoryPath;
    use serde_json::Value;

    use super::{BaselineCommandError, run, run_at_root, validate_at_with_before_publication};
    use crate::CommandOutcome;
    use crate::contracts::{
        readiness::{ReadinessError, ReadinessValidationError, validate_workspace},
        schema,
    };

    const COMPLETE_FIXTURE: &str = "qualification/fixtures/baselines/complete.synthetic.json";

    #[test]
    fn validates_the_complete_synthetic_baseline_without_writing() -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let output = root.join("qualification/fixtures/baselines/no-output-test");
        if output.exists() {
            fs::remove_dir_all(&output)?;
        }

        assert_eq!(
            run(&["--input".to_owned(), COMPLETE_FIXTURE.to_owned()]),
            CommandOutcome::Success
        );
        assert!(!output.exists());
        Ok(())
    }

    #[test]
    fn rejects_every_required_invalid_baseline_fixture() {
        for fixture in [
            "missing-key.json",
            "duplicate-key.json",
            "mismatched-flow.json",
            "empty-evidence.json",
            "synthetic-with-approval.json",
            "approved-without-approval-evidence.json",
            "extra-key.json",
        ] {
            assert!(matches!(
                run(&[
                    "--input".to_owned(),
                    format!("qualification/fixtures/baselines/{fixture}"),
                ]),
                CommandOutcome::Failed(_)
            ));
        }
    }

    #[test]
    fn approved_provenance_requires_digest_bound_approval_evidence() -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let input = root.join(format!(
            "qualification/fixtures/baselines/approved-test-{}.json",
            std::process::id()
        ));
        let approval = root.join("qualification/fixtures/evidence/preserved-source.json");
        let mut baseline: Value = serde_json::from_slice(&fs::read(root.join(COMPLETE_FIXTURE))?)?;
        let provenance = baseline
            .pointer_mut("/provenance")
            .and_then(Value::as_object_mut)
            .ok_or("baseline fixture must contain provenance")?;
        provenance.insert("kind".to_owned(), Value::String("approved".to_owned()));
        provenance.insert(
            "approvalEvidence".to_owned(),
            serde_json::json!({
                "path": "qualification/fixtures/evidence/preserved-source.json",
                "sha256": hash_file(&approval)?.to_string(),
            }),
        );
        fs::write(
            &input,
            format!("{}\n", serde_json::to_string_pretty(&baseline)?),
        )?;
        let input_argument = input
            .strip_prefix(&root)?
            .to_str()
            .ok_or("approved baseline input must be UTF-8")?
            .to_owned();
        let valid = run(&["--input".to_owned(), input_argument.clone()]);

        let approval_evidence = baseline
            .pointer_mut("/provenance/approvalEvidence/sha256")
            .ok_or("approved baseline must contain approval digest")?;
        *approval_evidence = Value::String("0".repeat(64));
        fs::write(
            &input,
            format!("{}\n", serde_json::to_string_pretty(&baseline)?),
        )?;
        let invalid = run(&["--input".to_owned(), input_argument]);
        fs::remove_file(input)?;

        assert_eq!(valid, CommandOutcome::Success);
        assert!(matches!(invalid, CommandOutcome::Failed(_)));
        Ok(())
    }

    #[test]
    fn writes_deterministic_schema_valid_canonical_output_with_sidecar_provenance()
    -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let output = "qualification/fixtures/baselines/output-test";
        let output_path = root.join(output);
        if output_path.exists() {
            fs::remove_dir_all(&output_path)?;
        }

        let arguments = [
            "--input".to_owned(),
            COMPLETE_FIXTURE.to_owned(),
            "--output".to_owned(),
            output.to_owned(),
        ];
        assert_eq!(run_at_root(&root, &arguments), CommandOutcome::Success);
        let first = output_files(&output_path)?;
        assert_eq!(first.len(), 2);
        let first_draft = first
            .iter()
            .find(|path| {
                !path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.ends_with(".provenance.json"))
            })
            .ok_or("output must contain the baseline draft")?;
        let first_sidecar = first
            .iter()
            .find(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.ends_with(".provenance.json"))
            })
            .ok_or("output must contain the provenance sidecar")?;
        let first_bytes = fs::read(first_draft)?;

        assert_eq!(run_at_root(&root, &arguments), CommandOutcome::Success);
        let second = output_files(&output_path)?;
        assert_eq!(second, first);
        assert_eq!(fs::read(first_draft)?, first_bytes);
        let output_reference = first_draft
            .strip_prefix(&root)?
            .to_str()
            .ok_or("output reference must be UTF-8")?
            .parse::<RepositoryPath>()?;
        let _ = verify_file(&root, &output_reference, &MediaType::application_json())?;
        assert!(
            first_draft
                .file_stem()
                .and_then(|stem| stem.to_str())
                .is_some_and(|stem| stem.len() == 64)
        );

        assert_eq!(
            run_at_root(
                &root,
                &["--input".to_owned(), output_reference.as_str().to_owned()],
            ),
            CommandOutcome::Success
        );
        let draft: Value = serde_json::from_slice(&first_bytes)?;
        let original: Value = serde_json::from_slice(&fs::read(root.join(COMPLETE_FIXTURE))?)?;
        let registry = schema::compile_workspace(&root)?;
        registry.validate(super::BASELINE_SCHEMA, &draft)?;
        assert_eq!(draft, original);
        assert_eq!(first_bytes, canonical_json_bytes(&original)?);
        assert!(draft.get("sourcePath").is_none());
        assert!(draft.get("sourceSha256").is_none());

        let sidecar_reference = first_sidecar
            .strip_prefix(&root)?
            .to_str()
            .ok_or("sidecar reference must be UTF-8")?
            .parse::<RepositoryPath>()?;
        let sidecar_verified =
            verify_file(&root, &sidecar_reference, &MediaType::application_json())?;
        let sidecar = sidecar_verified
            .json()
            .cloned()
            .ok_or("sidecar must be JSON")?;
        let sidecar_path = sidecar
            .get("path")
            .and_then(Value::as_str)
            .ok_or("sidecar must name the draft path")?
            .parse::<RepositoryPath>()?;
        let sidecar_digest = sidecar
            .get("sha256")
            .and_then(Value::as_str)
            .ok_or("sidecar must name the draft digest")?
            .parse()?;
        let resolved_draft = verify_path_digest(&root, &sidecar_path, &sidecar_digest)?;
        assert_eq!(
            first_sidecar.file_name().and_then(|name| name.to_str()),
            Some(format!("{}.provenance.json", resolved_draft.sha256()).as_str())
        );
        let source_digest = hash_file(&root.join(COMPLETE_FIXTURE))?.to_string();
        assert_eq!(resolved_draft.sha256(), hash_file(first_draft)?);
        assert_eq!(
            sidecar,
            serde_json::json!({
                "path": output_reference.as_str(),
                "sha256": resolved_draft.sha256().to_string(),
                "mediaType": "application/json",
                "sourcePath": COMPLETE_FIXTURE,
                "sourceSha256": source_digest,
            })
        );

        fs::remove_dir_all(output_path)?;
        Ok(())
    }

    #[test]
    fn rejects_publication_when_the_source_changes_after_validation() -> Result<(), Box<dyn Error>>
    {
        let root = workspace_root()?;
        let input_text = format!(
            "qualification/fixtures/baselines/snapshot-source-test-{}.json",
            std::process::id()
        );
        let input = input_text.parse::<RepositoryPath>()?;
        let input_path = root.join(&input_text);
        let output_text = format!(
            "qualification/fixtures/baselines/snapshot-output-test-{}",
            std::process::id()
        );
        let output = output_text.parse::<RepositoryPath>()?;
        let output_path = root.join(&output_text);
        if output_path.exists() {
            fs::remove_dir_all(&output_path)?;
        }
        fs::copy(root.join(COMPLETE_FIXTURE), &input_path)?;

        let changed_input = input_path.clone();
        let result = validate_at_with_before_publication(&root, &input, Some(&output), || {
            fs::write(&changed_input, b"{}").map_err(|source| BaselineCommandError::Io {
                path: changed_input.clone(),
                source,
            })
        });
        let output_exists = output_path.exists();
        fs::remove_file(input_path)?;
        if output_exists {
            fs::remove_dir_all(&output_path)?;
        }

        assert!(matches!(
            result,
            Err(BaselineCommandError::SourceSnapshotMismatch)
        ));
        assert!(!output_exists);
        Ok(())
    }

    #[test]
    fn synthetic_baseline_cannot_satisfy_an_approved_lock_reference() -> Result<(), Box<dyn Error>>
    {
        let source = workspace_root()?;
        let root = temporary_directory("synthetic-lock-reference");
        copy_directory(
            &source.join("qualification/fixtures/contracts/readiness/ready"),
            &root,
        )?;
        copy_directory(
            &source.join(".constitution/tech-spec/data-models"),
            &root.join(".constitution/tech-spec/data-models"),
        )?;
        copy_directory(
            &source.join("qualification/schemas"),
            &root.join("qualification/schemas"),
        )?;
        let baseline_path = root.join(COMPLETE_FIXTURE);
        let parent = baseline_path
            .parent()
            .ok_or("baseline fixture must have a parent directory")?;
        fs::create_dir_all(parent)?;
        fs::copy(source.join(COMPLETE_FIXTURE), &baseline_path)?;

        let lock_path = root.join(".constitution/tech-spec/contracts/qualification-lock.json");
        let mut lock: Value = serde_json::from_slice(&fs::read(&lock_path)?)?;
        let reference = lock
            .pointer_mut("/measurementPolicy/capabilityBaseline")
            .and_then(Value::as_object_mut)
            .ok_or("ready fixture must contain an approved baseline reference")?;
        reference.insert(
            "path".to_owned(),
            Value::String(COMPLETE_FIXTURE.to_owned()),
        );
        reference.insert(
            "sha256".to_owned(),
            Value::String(hash_file(&baseline_path)?.to_string()),
        );
        fs::write(
            &lock_path,
            format!("{}\n", serde_json::to_string_pretty(&lock)?),
        )?;

        let result = validate_workspace(&root);
        fs::remove_dir_all(&root)?;
        assert!(matches!(
            result,
            Err(ReadinessValidationError::Readiness(
                ReadinessError::Invariant {
                    code: "capability-baseline-provenance"
                }
            ))
        ));
        Ok(())
    }

    #[test]
    fn rejects_invalid_command_arguments() {
        assert!(matches!(run(&[]), CommandOutcome::Failed(_)));
        assert!(matches!(
            run(&[
                "--input".to_owned(),
                COMPLETE_FIXTURE.to_owned(),
                "--output".to_owned(),
            ]),
            CommandOutcome::Failed(_)
        ));
    }

    fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| "xtask must remain directly below the workspace root".into())
    }

    fn temporary_directory(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("oxyflut-baseline-{name}-{}", std::process::id()))
    }

    fn copy_directory(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let destination_path = destination.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                copy_directory(&entry.path(), &destination_path)?;
            } else {
                fs::copy(entry.path(), destination_path)?;
            }
        }
        Ok(())
    }

    fn output_files(directory: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut files = fs::read_dir(directory)?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, _>>()?;
        files.sort();
        Ok(files)
    }
}
