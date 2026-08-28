use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::hash::hash_file;
use crate::identifiers::{CandidateId, ConstraintId, EnvironmentId, RepositoryPath};
use crate::schema::SchemaRegistry;

use super::{
    ComparisonStatistic, EvidenceBinding, MeasurementError, RawExclusionReason, RawSample,
    RawSampleInput, TemplateParameters, compute_comparison_bounds, generate_templates,
};

const LOCK_PATH: &str = ".constitution/tech-spec/contracts/qualification-lock.json";
const CONSTRAINTS_PATH: &str = ".constitution/prd/constraints.md";
const LOG_PATH: &str = "qualification/fixtures/measurements/harness/perf-001-launch-1.log";
const SAMPLE_VALIDITY_PATH: &str = "qualification/fixtures/sample-validity/complete.synthetic.json";

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
fn template_serializations_omit_absent_conditional_fields_and_validate_their_schemas()
-> Result<(), Box<dyn Error>> {
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
    let raw = templates
        .raw_measurement()
        .build(vec![raw_sample(1, 1, 1, 1.0, log)?])?;
    let registry = SchemaRegistry::from_directories(&[
        root.join(".constitution/tech-spec/data-models"),
        root.join("qualification/schemas"),
    ])?;

    let raw_value = raw.to_value()?;
    registry.validate(super::RAW_MEASUREMENT_SCHEMA, &raw_value)?;
    assert!(raw_value.pointer("/samples/0/exclusionReason").is_none());

    let sample_validity_value = templates.sample_validity().to_value()?;
    registry.validate(super::SAMPLE_VALIDITY_SCHEMA, &sample_validity_value)?;
    assert!(
        sample_validity_value
            .pointer("/rules/1/percentile")
            .is_none()
    );
    Ok(())
}

#[test]
fn prd_meter_table_matches_the_constraints_document() -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let source_rules = parse_prd_meter_rules(&root.join(CONSTRAINTS_PATH))?;
    let table_rules = super::PRD_METER_TABLE
        .iter()
        .map(|rule| DocumentMeterRule {
            constraint_id: rule.constraint_id.to_owned(),
            statistic: rule.statistic,
            percentile: rule.percentile,
            unit: rule.unit.to_owned(),
        })
        .collect::<BTreeSet<_>>();

    assert_eq!(table_rules, source_rules);
    Ok(())
}

#[test]
fn sample_validity_rules_require_the_complete_prd_meter_rule_set() -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let complete =
        serde_json::from_slice::<serde_json::Value>(&fs::read(root.join(SAMPLE_VALIDITY_PATH))?)?;
    let lock_digest = hash_file(&root.join(LOCK_PATH))?;
    super::SampleValidityRecord::parse_value(complete.clone())?.validate(lock_digest)?;

    let mut unstated_percentile = complete.clone();
    *unstated_percentile
        .pointer_mut("/rules/0/percentile")
        .ok_or("nearest-rank rule must declare a percentile")? = serde_json::Value::from(98);
    assert!(matches!(
        super::SampleValidityRecord::parse_value(unstated_percentile)?.validate(lock_digest),
        Err(MeasurementError::ComparisonRule)
    ));

    let mut wrong_unit = complete.clone();
    *wrong_unit
        .pointer_mut("/rules/0/unit")
        .ok_or("nearest-rank rule must declare a unit")? =
        serde_json::Value::String("us".to_owned());
    assert!(matches!(
        super::SampleValidityRecord::parse_value(wrong_unit)?.validate(lock_digest),
        Err(MeasurementError::ComparisonRule)
    ));

    let mut omitted_rule = complete.clone();
    omitted_rule
        .get_mut("rules")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or("sample-validity record must declare rules")?
        .pop()
        .ok_or("sample-validity fixture must contain a maximum-bound rule")?;
    assert!(matches!(
        super::SampleValidityRecord::parse_value(omitted_rule)?.validate(lock_digest),
        Err(MeasurementError::ComparisonRule)
    ));

    let mut unsupported_meter = complete;
    for pointer in ["/rules/0/constraintId", "/rules/1/constraintId"] {
        *unsupported_meter
            .pointer_mut(pointer)
            .ok_or("sample-validity rule must declare a constraint")? =
            serde_json::Value::String("CON-FRM-001".to_owned());
    }
    assert!(matches!(
        super::SampleValidityRecord::parse_value(unsupported_meter)?.validate(lock_digest),
        Err(MeasurementError::UnsupportedComparisonMeter)
    ));
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
fn raw_records_preserve_all_excluded_samples_until_bound_calculation() -> Result<(), Box<dyn Error>>
{
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
            value: 0.0,
            unit: "ms".to_owned(),
            valid: false,
            exclusion_reason: Some(RawExclusionReason::PhysicalDisconnect),
            harness_log: log,
        })?])?;

    raw.validate(&root, hash_file(&root.join(LOCK_PATH))?)?;
    assert!(matches!(
        compute_comparison_bounds(&raw, templates.sample_validity()),
        Err(MeasurementError::NoValidObservations)
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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct DocumentMeterRule {
    constraint_id: String,
    statistic: ComparisonStatistic,
    percentile: Option<u8>,
    unit: String,
}

fn parse_prd_meter_rules(path: &Path) -> Result<BTreeSet<DocumentMeterRule>, Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    let mut constraints = BTreeSet::new();
    let mut rules = BTreeSet::new();
    for line in source.lines().filter(|line| line.starts_with("| CON-")) {
        let columns = line.split('|').map(str::trim).collect::<Vec<_>>();
        let constraint_id = *columns
            .get(1)
            .ok_or("constraint row must have an identifier")?;
        let _ = ConstraintId::parse(constraint_id)?;
        constraints.insert(constraint_id);
        let meter = *columns.get(3).ok_or("constraint row must have a meter")?;
        let goal = *columns.get(4).ok_or("constraint row must have a goal")?;

        if let Some((_, percentile)) = meter.split_once("nearest-rank ") {
            let percentile = percentile
                .split_once("th percentile")
                .ok_or("nearest-rank meter must name a percentile")?
                .0
                .parse::<u8>()?;
            let unit = comparison_unit(goal)?;
            rules.insert(DocumentMeterRule {
                constraint_id: constraint_id.to_owned(),
                statistic: ComparisonStatistic::NearestRank,
                percentile: Some(percentile),
                unit: unit.to_owned(),
            });
            rules.insert(DocumentMeterRule {
                constraint_id: constraint_id.to_owned(),
                statistic: ComparisonStatistic::MaximumBound,
                percentile: None,
                unit: unit.to_owned(),
            });
        } else if meter.starts_with("Maximum of ") && meter.contains("independent cold launches") {
            rules.insert(DocumentMeterRule {
                constraint_id: constraint_id.to_owned(),
                statistic: ComparisonStatistic::MaximumBound,
                percentile: None,
                unit: comparison_unit(goal)?.to_owned(),
            });
        }
    }
    assert_eq!(constraints.len(), 27);
    Ok(rules)
}

fn comparison_unit(goal: &str) -> Result<&str, Box<dyn Error>> {
    let remainder = goal
        .strip_prefix("At most ")
        .ok_or("comparison-bound goal must state an upper bound")?;
    remainder
        .split_whitespace()
        .nth(1)
        .map(|unit| unit.trim_end_matches('.'))
        .ok_or_else(|| "comparison-bound goal must state a unit".into())
}

fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| "qualification crate must be below the workspace root".into())
}
