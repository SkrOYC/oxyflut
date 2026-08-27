use std::error::Error;
use std::path::{Path, PathBuf};

use crate::hash::hash_file;
use crate::identifiers::{CandidateId, ConstraintId, EnvironmentId, RepositoryPath};

use super::{
    ComparisonStatistic, EvidenceBinding, MeasurementError, RawExclusionReason, RawSample,
    RawSampleInput, TemplateParameters, compute_comparison_bounds, generate_templates,
};

const LOCK_PATH: &str = ".constitution/tech-spec/contracts/qualification-lock.json";
const LOG_PATH: &str = "qualification/fixtures/measurements/harness/perf-001-launch-1.log";

#[test]
fn templates_bind_one_identity_and_preserve_all_valid_observations() -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let templates = generate_templates(
        &root.join(LOCK_PATH),
        TemplateParameters::new(
            ConstraintId::parse("CON-PERF-001")?,
            EnvironmentId::Macos,
            CandidateId::Focused,
            "synthetic-meter-v1".to_owned(),
        )?,
    )?;
    let log = EvidenceBinding::new(
        RepositoryPath::parse(LOG_PATH)?,
        hash_file(&root.join(LOG_PATH))?,
    );
    let raw = templates.raw_measurement().build(vec![
        raw_sample(1, 1, 1, 1.0, log.clone())?,
        raw_sample(1, 2, 2, 2.0, log.clone())?,
        raw_sample(1, 3, 3, 100.0, log.clone())?,
        raw_sample(2, 1, 4, 3.0, log.clone())?,
        raw_sample(2, 2, 5, 4.0, log.clone())?,
        raw_sample(2, 3, 6, 200.0, log)?,
    ])?;
    raw.validate(&root, hash_file(&root.join(LOCK_PATH))?)?;
    let bounds = compute_comparison_bounds(&raw, templates.sample_validity())?;

    assert_eq!(bounds.len(), 3);
    assert_eq!(bounds[0].statistic(), ComparisonStatistic::NearestRank);
    assert_eq!(bounds[0].launch(), Some(1));
    assert_eq!(bounds[0].value(), 100.0);
    assert_eq!(bounds[0].valid_observation_count(), 6);
    assert_eq!(bounds[1].launch(), Some(2));
    assert_eq!(bounds[1].value(), 200.0);
    assert_eq!(bounds[2].statistic(), ComparisonStatistic::MaximumBound);
    assert_eq!(bounds[2].value(), 200.0);
    assert_eq!(bounds[2].valid_observation_count(), 6);
    assert_eq!(raw.lock_digest(), templates.sample_validity().lock_digest());
    assert_eq!(
        raw.meter_version(),
        templates.sample_validity().meter_version()
    );
    Ok(())
}

#[test]
fn paired_records_reject_an_altered_meter_version() -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let templates = generate_templates(
        &root.join(LOCK_PATH),
        TemplateParameters::new(
            ConstraintId::parse("CON-PERF-003")?,
            EnvironmentId::Macos,
            CandidateId::Focused,
            "synthetic-meter-v1".to_owned(),
        )?,
    )?;
    let log = EvidenceBinding::new(
        RepositoryPath::parse(LOG_PATH)?,
        hash_file(&root.join(LOG_PATH))?,
    );
    let raw = templates
        .raw_measurement()
        .build(vec![RawSample::new(RawSampleInput {
            constraint_id: ConstraintId::parse("CON-PERF-003")?,
            launch: 1,
            ordinal: 1,
            monotonic_ns: 1,
            value: 10.0,
            unit: "ms".to_owned(),
            valid: true,
            exclusion_reason: None,
            harness_log: log,
        })?])?;
    let mut altered = templates.sample_validity().to_value()?;
    let meter_version = altered
        .pointer_mut("/meterVersion")
        .ok_or("sample-validity record must contain meterVersion")?;
    *meter_version = serde_json::Value::String("altered-meter-v2".to_owned());
    let altered = super::SampleValidityRecord::parse_value(altered)?;

    assert!(matches!(
        compute_comparison_bounds(&raw, &altered),
        Err(MeasurementError::InconsistentBinding)
    ));
    Ok(())
}

#[test]
fn templates_reject_meters_without_prd_comparison_rules() -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let result = generate_templates(
        &root.join(LOCK_PATH),
        TemplateParameters::new(
            ConstraintId::parse("CON-FRM-001")?,
            EnvironmentId::Macos,
            CandidateId::Focused,
            "synthetic-meter-v1".to_owned(),
        )?,
    );
    assert!(matches!(
        result,
        Err(MeasurementError::UnsupportedComparisonMeter)
    ));
    Ok(())
}

#[test]
fn raw_samples_reject_valid_exclusions_before_serialization() -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let log = EvidenceBinding::new(
        RepositoryPath::parse(LOG_PATH)?,
        hash_file(&root.join(LOG_PATH))?,
    );
    let result = RawSample::new(RawSampleInput {
        constraint_id: ConstraintId::parse("CON-PERF-001")?,
        launch: 1,
        ordinal: 1,
        monotonic_ns: 1,
        value: 1.0,
        unit: "ms".to_owned(),
        valid: true,
        exclusion_reason: Some(RawExclusionReason::MeasurementToolFailure),
        harness_log: log,
    });
    assert!(matches!(result, Err(MeasurementError::SampleValidity)));
    Ok(())
}

fn raw_sample(
    launch: u64,
    ordinal: u64,
    monotonic_ns: u64,
    value: f64,
    harness_log: EvidenceBinding,
) -> Result<RawSample, MeasurementError> {
    RawSample::new(RawSampleInput {
        constraint_id: ConstraintId::parse("CON-PERF-001")
            .map_err(|_| MeasurementError::Constraint)?,
        launch,
        ordinal,
        monotonic_ns,
        value,
        unit: "ms".to_owned(),
        valid: true,
        exclusion_reason: None,
        harness_log,
    })
}

fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| "qualification crate must be below the workspace root".into())
}
