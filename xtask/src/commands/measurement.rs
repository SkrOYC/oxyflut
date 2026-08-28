//! Raw-measurement and staged sample-validity validation.

use std::path::{Path, PathBuf};

use oxyflut_qualification::evidence::{EvidenceError, MediaType, verify_file};
use oxyflut_qualification::hash::hash_file;
use oxyflut_qualification::identifiers::RepositoryPath;
use oxyflut_qualification::measurement::{
    MeasurementError, RAW_MEASUREMENT_SCHEMA, RawMeasurement, SAMPLE_VALIDITY_SCHEMA,
    SampleValidityRecord,
};
use serde_json::Value;
use thiserror::Error;

use super::super::{CommandError, CommandOutcome};
use crate::contracts::{schema, traceability};

const LOCK_PATH: &str = ".constitution/tech-spec/contracts/qualification-lock.json";

/// Validates one raw-measurement or staged sample-validity record without executing a measurement.
pub(crate) fn run(arguments: &[String]) -> CommandOutcome {
    let input = match parse_arguments(arguments) {
        Ok(input) => input,
        Err(()) => {
            return CommandOutcome::failed(CommandError::InvalidInput {
                code: "measurement-validate-arguments",
            });
        }
    };
    let root = match workspace_root() {
        Ok(root) => root,
        Err(()) => {
            return CommandOutcome::failed(CommandError::Execution {
                code: "workspace-root",
                hint: "rerun: measurement validate --input PATH",
            });
        }
    };

    match validate_at(&root, &input) {
        Ok(()) => {
            println!("measurement validate: ok");
            CommandOutcome::Success
        }
        Err(_) => CommandOutcome::failed(CommandError::ValidationFailed {
            code: "measurement-invalid",
            hint: "rerun: measurement validate --input PATH",
        }),
    }
}

fn parse_arguments(arguments: &[String]) -> Result<RepositoryPath, ()> {
    let [flag, input] = arguments else {
        return Err(());
    };
    if flag != "--input" {
        return Err(());
    }
    RepositoryPath::parse(input).map_err(|_| ())
}

fn validate_at(root: &Path, input: &RepositoryPath) -> Result<(), MeasurementCommandError> {
    let verified = verify_file(root, input, &MediaType::application_json())?;
    let value = verified
        .json()
        .cloned()
        .ok_or(MeasurementCommandError::RecordType)?;
    let registry = schema::compile_workspace(root)?;
    let lock_digest =
        hash_file(&root.join(LOCK_PATH)).map_err(|source| MeasurementCommandError::Lock {
            path: root.join(LOCK_PATH),
            source,
        })?;
    let constraints = traceability::constraint_authority(root)?;

    if is_raw_measurement(&value) {
        registry.validate(RAW_MEASUREMENT_SCHEMA, &value)?;
        let raw = RawMeasurement::parse_value(value.clone())?;
        raw.validate(root, lock_digest)?;
        traceability::validate_raw_measurement(&value, &constraints)?;
        return Ok(());
    }
    if is_sample_validity(&value) {
        registry.validate(SAMPLE_VALIDITY_SCHEMA, &value)?;
        let sample_validity = SampleValidityRecord::parse_value(value)?;
        sample_validity.validate(lock_digest)?;
        sample_validity.validate_constraint_authority(&constraints)?;
        return Ok(());
    }
    Err(MeasurementCommandError::RecordType)
}

fn is_raw_measurement(value: &Value) -> bool {
    let Some(object) = value.as_object() else {
        return false;
    };
    ["schemaVersion", "samples", "meterVersion", "lockDigest"]
        .iter()
        .all(|key| object.contains_key(*key))
}

fn is_sample_validity(value: &Value) -> bool {
    let Some(object) = value.as_object() else {
        return false;
    };
    object.contains_key("schemaVersion")
        && (object.contains_key("exclusionCategories") || object.contains_key("rules"))
}

fn workspace_root() -> Result<PathBuf, ()> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or(())
}

#[derive(Debug, Error)]
enum MeasurementCommandError {
    #[error("measurement input does not identify a supported record type")]
    RecordType,
    #[error("measurement input failed immutable evidence verification")]
    Evidence(#[from] EvidenceError),
    #[error("measurement schema registry failed")]
    SchemaRegistry(#[from] schema::ContractSchemaError),
    #[error("measurement schema validation failed")]
    Schema(#[from] oxyflut_qualification::schema::SchemaError),
    #[error("measurement record validation failed")]
    Measurement(#[from] MeasurementError),
    #[error("measurement constraint authority failed")]
    Traceability(#[from] traceability::TraceabilityError),
    #[error("qualification lock could not be hashed")]
    Lock {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::path::{Path, PathBuf};

    use super::{run, validate_at};
    use crate::CommandOutcome;

    const RAW_COMPLETE: &str = "qualification/fixtures/measurements/complete.synthetic.json";
    const RAW_ALL_EXCLUDED: &str = "qualification/fixtures/measurements/all-excluded.json";
    const SAMPLE_VALIDITY_COMPLETE: &str =
        "qualification/fixtures/sample-validity/complete.synthetic.json";

    #[test]
    fn measurement_validates_complete_and_all_excluded_raw_templates() -> Result<(), Box<dyn Error>>
    {
        assert_eq!(
            run(&["--input".to_owned(), RAW_COMPLETE.to_owned()]),
            CommandOutcome::Success
        );
        assert_eq!(
            run(&["--input".to_owned(), RAW_ALL_EXCLUDED.to_owned()]),
            CommandOutcome::Success
        );
        assert_eq!(
            run(&["--input".to_owned(), SAMPLE_VALIDITY_COMPLETE.to_owned()]),
            CommandOutcome::Success
        );
        Ok(())
    }

    #[test]
    fn measurement_rejects_every_required_negative_fixture() {
        let fixtures = [
            "qualification/fixtures/measurements/unapproved-exclusion.json",
            "qualification/fixtures/measurements/missing-raw-sample.json",
            "qualification/fixtures/measurements/missing-harness-log.json",
            "qualification/fixtures/measurements/altered-meter.json",
            "qualification/fixtures/measurements/duplicate-ordinal.json",
            "qualification/fixtures/measurements/valid-with-exclusion.json",
            "qualification/fixtures/measurements/non-monotonic-time.json",
            "qualification/fixtures/measurements/wrong-lock-digest.json",
            "qualification/fixtures/sample-validity/unstated-percentile.json",
            "qualification/fixtures/sample-validity/wrong-unit.json",
            "qualification/fixtures/sample-validity/missing-maximum-bound.json",
            "qualification/fixtures/sample-validity/unsupported-meter.json",
        ];
        for fixture in fixtures {
            assert!(matches!(
                run(&["--input".to_owned(), fixture.to_owned()]),
                CommandOutcome::Failed(_)
            ));
        }
    }

    #[test]
    fn measurement_rejects_unknown_records_and_invalid_arguments() {
        let unknown = run(&[
            "--input".to_owned(),
            "qualification/fixtures/measurements/unknown-record.json".to_owned(),
        ]);

        assert!(matches!(unknown, CommandOutcome::Failed(_)));
        assert!(matches!(run(&[]), CommandOutcome::Failed(_)));
        assert!(matches!(
            run(&["--output".to_owned(), RAW_COMPLETE.to_owned()]),
            CommandOutcome::Failed(_)
        ));
    }

    #[test]
    fn measurement_confines_input_to_the_qualification_root() -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let input = ".constitution/tech-spec/contracts/qualification-lock.json"
            .parse()
            .map_err(|_| "test input path must be canonical")?;
        assert!(validate_at(&root, &input).is_err());
        Ok(())
    }

    fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| "xtask must be directly below the workspace root".into())
    }
}
