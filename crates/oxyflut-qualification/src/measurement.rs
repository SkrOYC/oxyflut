//! Typed raw-measurement and sample-validity records for qualification tooling.
//!
//! The durable raw-measurement schema remains the Stage 3 authority. The sample-validity schema in
//! `qualification/schemas/` is a staged proposal that supplies the comparison-bound inputs used by
//! this module. Neither record executes a measurement.

use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::evidence::{EvidenceError, verify_path_digest};
use crate::hash::{Sha256Digest, hash_file};
use crate::identifiers::{CandidateId, ConstraintId, EnvironmentId, RepositoryPath};

/// The Stage 3 raw-measurement schema identity.
pub const RAW_MEASUREMENT_SCHEMA: &str = "urn:oxyflut:schema:raw-measurement:2";
/// The staged sample-validity schema identity.
pub const SAMPLE_VALIDITY_SCHEMA: &str = "urn:oxyflut:staged:sample-validity:1";

const RAW_MEASUREMENT_SCHEMA_VERSION: &str = "2.0.0";
const SAMPLE_VALIDITY_SCHEMA_VERSION: &str = "1.0.0";

/// An exclusion category admitted by the raw-measurement v2 schema.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum RawExclusionReason {
    /// The measurement tool failed to capture the observation.
    #[serde(rename = "measurement-tool-failure")]
    MeasurementToolFailure,
    /// An unrelated operating-system interruption prevented a valid observation.
    #[serde(rename = "unrelated-os-interruption")]
    UnrelatedOperatingSystemInterruption,
    /// A physical connection was interrupted during the observation.
    #[serde(rename = "physical-disconnect")]
    PhysicalDisconnect,
}

/// A closed sample-validity exclusion category proposed for lock binding.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SampleValidityExclusionCategory {
    /// The measurement tool failed to capture the observation.
    #[serde(rename = "measurement-tool-failure")]
    MeasurementToolFailure,
    /// An unrelated operating-system interruption prevented a valid observation.
    #[serde(rename = "unrelated-os-interruption")]
    UnrelatedOsInterruption,
    /// A physical connection was interrupted during the observation.
    #[serde(rename = "physical-disconnect")]
    PhysicalDisconnect,
}

impl SampleValidityExclusionCategory {
    const fn all() -> [Self; 3] {
        [
            Self::MeasurementToolFailure,
            Self::UnrelatedOsInterruption,
            Self::PhysicalDisconnect,
        ]
    }
}

/// An immutable path and digest binding for one harness log.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvidenceBinding {
    path: String,
    sha256: String,
}

impl EvidenceBinding {
    /// Creates a harness-log binding from checked repository identifiers.
    #[must_use]
    pub fn new(path: RepositoryPath, sha256: Sha256Digest) -> Self {
        Self {
            path: path.to_string(),
            sha256: sha256.to_string(),
        }
    }

    fn verify(&self, root: &Path) -> Result<(), MeasurementError> {
        let path = self
            .path
            .parse::<RepositoryPath>()
            .map_err(|_| MeasurementError::HarnessLog)?;
        let sha256 = self
            .sha256
            .parse::<Sha256Digest>()
            .map_err(|_| MeasurementError::HarnessLog)?;
        let _ = verify_path_digest(root, &path, &sha256).map_err(MeasurementError::Evidence)?;
        Ok(())
    }
}

/// Input for one preserved raw observation and its harness-log binding.
#[derive(Clone, Debug)]
pub struct RawSampleInput {
    /// The measured product constraint.
    pub constraint_id: ConstraintId,
    /// The one-based process launch number.
    pub launch: u64,
    /// The one-based observation number within the launch.
    pub ordinal: u64,
    /// The observation time in nanoseconds from a clock that is monotonic for this launch.
    pub monotonic_ns: u64,
    /// The directly observed scalar value.
    pub value: f64,
    /// The frozen scalar unit.
    pub unit: String,
    /// Whether the sample is admitted by the frozen validity rules.
    pub valid: bool,
    /// The exclusion category when the sample is not admitted.
    pub exclusion_reason: Option<RawExclusionReason>,
    /// The immutable harness-log reference for this observation.
    pub harness_log: EvidenceBinding,
}

/// One preserved raw observation and its harness-log binding.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RawSample {
    constraint_id: String,
    launch: u64,
    ordinal: u64,
    monotonic_ns: u64,
    value: f64,
    unit: String,
    valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_reason: Option<RawExclusionReason>,
    harness_log: EvidenceBinding,
}

impl RawSample {
    /// Creates one raw sample with an explicit admission decision and harness-log binding.
    ///
    /// # Errors
    ///
    /// Returns an error when the launch or ordinal is zero, the unit is blank, or the validity and
    /// exclusion fields contradict each other.
    pub fn new(input: RawSampleInput) -> Result<Self, MeasurementError> {
        if input.launch == 0 || input.ordinal == 0 {
            return Err(MeasurementError::SampleOrdinal);
        }
        if input.unit.trim().is_empty() {
            return Err(MeasurementError::Unit);
        }
        if !input.value.is_finite() {
            return Err(MeasurementError::Value);
        }
        validate_validity(input.valid, input.exclusion_reason)?;
        Ok(Self {
            constraint_id: input.constraint_id.to_string(),
            launch: input.launch,
            ordinal: input.ordinal,
            monotonic_ns: input.monotonic_ns,
            value: input.value,
            unit: input.unit,
            valid: input.valid,
            exclusion_reason: input.exclusion_reason,
            harness_log: input.harness_log,
        })
    }
}

/// A complete raw-measurement record.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RawMeasurement {
    schema_version: String,
    candidate: String,
    environment: String,
    lock_digest: String,
    meter_version: String,
    samples: Vec<RawSample>,
}

impl RawMeasurement {
    /// Parses a raw-measurement JSON value.
    ///
    /// # Errors
    ///
    /// Returns an error when the value cannot be represented as one raw-measurement record.
    pub fn parse_value(value: Value) -> Result<Self, MeasurementError> {
        serde_json::from_value(value).map_err(MeasurementError::Json)
    }

    /// Returns the bound qualification-lock digest text.
    #[must_use]
    pub fn lock_digest(&self) -> &str {
        &self.lock_digest
    }

    /// Returns the bound meter-version text.
    #[must_use]
    pub fn meter_version(&self) -> &str {
        &self.meter_version
    }

    /// Converts the record to a JSON value for schema validation or durable output.
    ///
    /// # Errors
    ///
    /// Returns an error only when serialization fails.
    pub fn to_value(&self) -> Result<Value, MeasurementError> {
        serde_json::to_value(self).map_err(MeasurementError::Serialization)
    }

    /// Validates raw-record invariants, each harness-log digest, and the current lock binding.
    ///
    /// # Errors
    ///
    /// Returns an error when a raw sample is incomplete, reordered, duplicated, has an invalid
    /// admission decision, or is bound to a missing, changed, or different-lock harness input.
    pub fn validate(
        &self,
        root: &Path,
        expected_lock_digest: Sha256Digest,
    ) -> Result<(), MeasurementError> {
        self.validate_structure(expected_lock_digest)?;
        for sample in &self.samples {
            sample.harness_log.verify(root)?;
        }
        Ok(())
    }

    fn validate_structure(
        &self,
        expected_lock_digest: Sha256Digest,
    ) -> Result<(), MeasurementError> {
        if self.schema_version != RAW_MEASUREMENT_SCHEMA_VERSION {
            return Err(MeasurementError::RawSchemaVersion);
        }
        let _ = self
            .candidate
            .parse::<CandidateId>()
            .map_err(|_| MeasurementError::Candidate)?;
        let _ = self
            .environment
            .parse::<EnvironmentId>()
            .map_err(|_| MeasurementError::Environment)?;
        validate_lock_digest(&self.lock_digest, expected_lock_digest)?;
        validate_meter_version(&self.meter_version)?;
        if self.samples.is_empty() {
            return Err(MeasurementError::NoSamples);
        }

        let mut keys = BTreeSet::new();
        let mut previous_times = BTreeMap::new();
        for sample in &self.samples {
            let constraint = ConstraintId::parse(&sample.constraint_id)
                .map_err(|_| MeasurementError::Constraint)?;
            if sample.launch == 0 || sample.ordinal == 0 {
                return Err(MeasurementError::SampleOrdinal);
            }
            if !keys.insert((constraint.clone(), sample.launch, sample.ordinal)) {
                return Err(MeasurementError::DuplicateSampleKey);
            }
            let key = (constraint, sample.launch);
            if previous_times
                .get(&key)
                .is_some_and(|time| sample.monotonic_ns < *time)
            {
                return Err(MeasurementError::NonMonotonicTime);
            }
            previous_times.insert(key, sample.monotonic_ns);
            if sample.unit.trim().is_empty() {
                return Err(MeasurementError::Unit);
            }
            validate_validity(sample.valid, sample.exclusion_reason)?;
        }
        Ok(())
    }
}

/// A nearest-rank or maximum-bound comparison input.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonStatistic {
    /// Selects the nearest-ranked observation at the declared percentile.
    NearestRank,
    /// Selects the greatest bound value.
    MaximumBound,
}

/// A staged sample-validity record bound to one measurement identity.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SampleValidityRecord {
    #[serde(rename = "$schema")]
    schema: String,
    schema_version: String,
    lock_digest: String,
    environment: String,
    candidate: String,
    meter_version: String,
    exclusion_categories: Vec<SampleValidityExclusionCategory>,
    rules: Vec<ComparisonBoundInput>,
}

impl SampleValidityRecord {
    /// Parses a staged sample-validity JSON value.
    ///
    /// # Errors
    ///
    /// Returns an error when the value cannot be represented as one sample-validity record.
    pub fn parse_value(value: Value) -> Result<Self, MeasurementError> {
        serde_json::from_value(value).map_err(MeasurementError::Json)
    }

    /// Returns the bound qualification-lock digest text.
    #[must_use]
    pub fn lock_digest(&self) -> &str {
        &self.lock_digest
    }

    /// Returns the bound meter-version text.
    #[must_use]
    pub fn meter_version(&self) -> &str {
        &self.meter_version
    }

    /// Converts the record to a JSON value for schema validation or durable output.
    ///
    /// # Errors
    ///
    /// Returns an error only when serialization fails.
    pub fn to_value(&self) -> Result<Value, MeasurementError> {
        serde_json::to_value(self).map_err(MeasurementError::Serialization)
    }

    /// Validates staged sample-validity structure and the current qualification-lock binding.
    ///
    /// # Errors
    ///
    /// Returns an error when categories, comparison rules, identity fields, or the lock digest are
    /// invalid.
    pub fn validate(&self, expected_lock_digest: Sha256Digest) -> Result<(), MeasurementError> {
        if self.schema != SAMPLE_VALIDITY_SCHEMA {
            return Err(MeasurementError::SampleValiditySchema);
        }
        if self.schema_version != SAMPLE_VALIDITY_SCHEMA_VERSION {
            return Err(MeasurementError::SampleValiditySchemaVersion);
        }
        let _ = self
            .candidate
            .parse::<CandidateId>()
            .map_err(|_| MeasurementError::Candidate)?;
        let _ = self
            .environment
            .parse::<EnvironmentId>()
            .map_err(|_| MeasurementError::Environment)?;
        validate_lock_digest(&self.lock_digest, expected_lock_digest)?;
        validate_meter_version(&self.meter_version)?;
        if self.exclusion_categories.as_slice() != SampleValidityExclusionCategory::all() {
            return Err(MeasurementError::ExclusionCategories);
        }
        if self.rules.is_empty() {
            return Err(MeasurementError::NoComparisonRules);
        }

        let mut seen_rules = BTreeSet::new();
        let mut constraints = BTreeSet::new();
        for rule in &self.rules {
            let constraint = ConstraintId::parse(&rule.constraint_id)
                .map_err(|_| MeasurementError::Constraint)?;
            if !seen_rules.insert((constraint.clone(), rule.statistic)) {
                return Err(MeasurementError::DuplicateComparisonRule);
            }
            if rule.unit.trim().is_empty() || !rule.retain_all_valid_observations {
                return Err(MeasurementError::ComparisonRule);
            }
            match (rule.statistic, rule.percentile) {
                (ComparisonStatistic::NearestRank, Some(percentile)) if percentile > 0 => {}
                (ComparisonStatistic::NearestRank, Some(_))
                | (ComparisonStatistic::NearestRank, None)
                | (ComparisonStatistic::MaximumBound, Some(_)) => {
                    return Err(MeasurementError::ComparisonRule);
                }
                (ComparisonStatistic::MaximumBound, None) => {}
            }
            if prd_meter_rules_for(&constraint).next().is_none() {
                return Err(MeasurementError::UnsupportedComparisonMeter);
            }
            if !prd_meter_rules_for(&constraint).any(|expected| expected.matches(rule)) {
                return Err(MeasurementError::ComparisonRule);
            }
            constraints.insert(constraint);
        }
        for constraint in constraints {
            for expected in prd_meter_rules_for(&constraint) {
                if !self
                    .rules
                    .iter()
                    .any(|rule| rule.constraint_id == constraint.as_str() && expected.matches(rule))
                {
                    return Err(MeasurementError::ComparisonRule);
                }
            }
        }
        Ok(())
    }

    /// Checks that every comparison rule names an exact authoritative product constraint.
    ///
    /// # Errors
    ///
    /// Returns an error when a rule names a syntactically valid but nonauthoritative constraint.
    pub fn validate_constraint_authority(
        &self,
        constraints: &BTreeSet<ConstraintId>,
    ) -> Result<(), MeasurementError> {
        for rule in &self.rules {
            let constraint = ConstraintId::parse(&rule.constraint_id)
                .map_err(|_| MeasurementError::Constraint)?;
            if !constraints.contains(&constraint) {
                return Err(MeasurementError::UnknownConstraint);
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ComparisonBoundInput {
    constraint_id: String,
    statistic: ComparisonStatistic,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentile: Option<u8>,
    unit: String,
    retain_all_valid_observations: bool,
}

/// Immutable generation inputs for one measurement template.
#[derive(Clone, Debug)]
pub struct TemplateParameters {
    constraint_id: ConstraintId,
    environment: EnvironmentId,
    candidate: CandidateId,
    meter_version: String,
}

impl TemplateParameters {
    /// Creates template parameters with a nonblank meter version.
    ///
    /// # Errors
    ///
    /// Returns an error when the meter version is blank.
    pub fn new(
        constraint_id: ConstraintId,
        environment: EnvironmentId,
        candidate: CandidateId,
        meter_version: String,
    ) -> Result<Self, MeasurementError> {
        validate_meter_version(&meter_version)?;
        Ok(Self {
            constraint_id,
            environment,
            candidate,
            meter_version,
        })
    }
}

/// A raw-measurement builder constrained to one template identity and constraint.
#[derive(Clone, Debug)]
pub struct RawMeasurementTemplate {
    constraint_id: ConstraintId,
    environment: EnvironmentId,
    candidate: CandidateId,
    meter_version: String,
    lock_digest: Sha256Digest,
}

impl RawMeasurementTemplate {
    /// Builds one raw-measurement record from preserved samples for this template's constraint.
    ///
    /// # Errors
    ///
    /// Returns an error when no samples are supplied or a sample names a different constraint.
    pub fn build(&self, samples: Vec<RawSample>) -> Result<RawMeasurement, MeasurementError> {
        if samples.is_empty() {
            return Err(MeasurementError::NoSamples);
        }
        if samples
            .iter()
            .any(|sample| sample.constraint_id != self.constraint_id.as_str())
        {
            return Err(MeasurementError::TemplateConstraint);
        }
        Ok(RawMeasurement {
            schema_version: RAW_MEASUREMENT_SCHEMA_VERSION.to_owned(),
            candidate: self.candidate.to_string(),
            environment: self.environment.to_string(),
            lock_digest: self.lock_digest.to_string(),
            meter_version: self.meter_version.clone(),
            samples,
        })
    }
}

/// A paired raw-measurement and sample-validity template.
#[derive(Clone, Debug)]
pub struct MeasurementTemplates {
    raw_measurement: RawMeasurementTemplate,
    sample_validity: SampleValidityRecord,
}

impl MeasurementTemplates {
    /// Returns the raw-measurement builder for this identity and constraint.
    #[must_use]
    pub fn raw_measurement(&self) -> &RawMeasurementTemplate {
        &self.raw_measurement
    }

    /// Returns the staged sample-validity record for this identity and constraint.
    #[must_use]
    pub fn sample_validity(&self) -> &SampleValidityRecord {
        &self.sample_validity
    }
}

/// Produces typed templates bound to the SHA-256 of one committed qualification lock.
///
/// The generator supports only the PRD meters in the authoritative nearest-rank and maximum-bound
/// table.
///
/// # Errors
///
/// Returns an error when the lock cannot be hashed, the meter version is blank, or the selected
/// constraint has no PRD-defined comparison-bound statistic.
pub fn generate_templates(
    qualification_lock_path: &Path,
    parameters: TemplateParameters,
) -> Result<MeasurementTemplates, MeasurementError> {
    let lock_digest =
        hash_file(qualification_lock_path).map_err(|source| MeasurementError::Lock {
            path: qualification_lock_path.to_path_buf(),
            source,
        })?;
    let rules = prd_meter_rules_for(&parameters.constraint_id)
        .map(|rule| ComparisonBoundInput {
            constraint_id: parameters.constraint_id.to_string(),
            statistic: rule.statistic,
            percentile: rule.percentile,
            unit: rule.unit.to_owned(),
            retain_all_valid_observations: true,
        })
        .collect::<Vec<_>>();
    if rules.is_empty() {
        return Err(MeasurementError::UnsupportedComparisonMeter);
    }
    let sample_validity = SampleValidityRecord {
        schema: SAMPLE_VALIDITY_SCHEMA.to_owned(),
        schema_version: SAMPLE_VALIDITY_SCHEMA_VERSION.to_owned(),
        lock_digest: lock_digest.to_string(),
        environment: parameters.environment.to_string(),
        candidate: parameters.candidate.to_string(),
        meter_version: parameters.meter_version.clone(),
        exclusion_categories: SampleValidityExclusionCategory::all().to_vec(),
        rules,
    };
    sample_validity.validate(lock_digest)?;
    Ok(MeasurementTemplates {
        raw_measurement: RawMeasurementTemplate {
            constraint_id: parameters.constraint_id,
            environment: parameters.environment,
            candidate: parameters.candidate,
            meter_version: parameters.meter_version,
            lock_digest,
        },
        sample_validity,
    })
}

/// One computed comparison bound with its preserved valid-observation count.
#[derive(Clone, Debug, PartialEq)]
pub struct ComparisonBound {
    constraint_id: ConstraintId,
    statistic: ComparisonStatistic,
    launch: Option<u64>,
    value: f64,
    valid_observation_count: usize,
}

impl ComparisonBound {
    /// Returns the constraint whose observations produced this bound.
    #[must_use]
    pub fn constraint_id(&self) -> &ConstraintId {
        &self.constraint_id
    }

    /// Returns the statistic used to calculate this bound.
    #[must_use]
    pub const fn statistic(&self) -> ComparisonStatistic {
        self.statistic
    }

    /// Returns the launch for a per-launch nearest-rank result.
    #[must_use]
    pub const fn launch(&self) -> Option<u64> {
        self.launch
    }

    /// Returns the calculated scalar bound.
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }

    /// Returns the number of valid raw observations preserved for this constraint.
    #[must_use]
    pub const fn valid_observation_count(&self) -> usize {
        self.valid_observation_count
    }
}

/// Computes nearest-rank and maximum bounds without deleting valid observations.
///
/// When a constraint declares both rules, nearest rank is calculated for each launch and
/// maximum-bound selects the maximum of those launch results. A maximum-only rule selects the
/// maximum of every valid raw observation for that constraint.
///
/// # Errors
///
/// Returns an error when records bind different environment, candidate, meter-version, or lock
/// values; a raw sample lacks a rule; or a rule lacks an admitted observation.
pub fn compute_comparison_bounds(
    raw_measurement: &RawMeasurement,
    sample_validity: &SampleValidityRecord,
) -> Result<Vec<ComparisonBound>, MeasurementError> {
    let lock_digest = raw_measurement
        .lock_digest
        .parse::<Sha256Digest>()
        .map_err(|_| MeasurementError::LockDigest)?;
    raw_measurement.validate_structure(lock_digest)?;
    sample_validity.validate(lock_digest)?;
    if raw_measurement.candidate != sample_validity.candidate
        || raw_measurement.environment != sample_validity.environment
        || raw_measurement.meter_version != sample_validity.meter_version
        || raw_measurement.lock_digest != sample_validity.lock_digest
    {
        return Err(MeasurementError::InconsistentBinding);
    }

    let mut rules_by_constraint = BTreeMap::<ConstraintId, Vec<&ComparisonBoundInput>>::new();
    for rule in &sample_validity.rules {
        let constraint =
            ConstraintId::parse(&rule.constraint_id).map_err(|_| MeasurementError::Constraint)?;
        rules_by_constraint
            .entry(constraint)
            .or_default()
            .push(rule);
    }
    let mut observations = BTreeMap::<ConstraintId, BTreeMap<u64, Vec<f64>>>::new();
    for sample in &raw_measurement.samples {
        let constraint =
            ConstraintId::parse(&sample.constraint_id).map_err(|_| MeasurementError::Constraint)?;
        let rules = rules_by_constraint
            .get(&constraint)
            .ok_or(MeasurementError::MissingComparisonRule)?;
        if rules.iter().any(|rule| rule.unit != sample.unit) {
            return Err(MeasurementError::Unit);
        }
        if sample.valid {
            observations
                .entry(constraint)
                .or_default()
                .entry(sample.launch)
                .or_default()
                .push(sample.value);
        }
    }

    let mut bounds = Vec::new();
    for (constraint, rules) in rules_by_constraint {
        let launches = observations
            .get(&constraint)
            .ok_or(MeasurementError::NoValidObservations)?;
        let valid_observation_count = launches.values().try_fold(0_usize, |count, values| {
            count
                .checked_add(values.len())
                .ok_or(MeasurementError::Arithmetic)
        })?;
        let nearest_rank_rule = rules
            .iter()
            .find(|rule| rule.statistic == ComparisonStatistic::NearestRank)
            .copied();
        let nearest_rank_values = match nearest_rank_rule {
            Some(rule) => nearest_rank_values(
                launches,
                rule.percentile.ok_or(MeasurementError::ComparisonRule)?,
            )?,
            None => Vec::new(),
        };
        let mut rules = rules;
        rules.sort_by_key(|rule| rule.statistic);
        for rule in rules {
            match rule.statistic {
                ComparisonStatistic::NearestRank => {
                    for (launch, value) in &nearest_rank_values {
                        bounds.push(ComparisonBound {
                            constraint_id: constraint.clone(),
                            statistic: ComparisonStatistic::NearestRank,
                            launch: Some(*launch),
                            value: *value,
                            valid_observation_count,
                        });
                    }
                }
                ComparisonStatistic::MaximumBound => {
                    let value = if nearest_rank_rule.is_some() {
                        maximum_bound(nearest_rank_values.iter().map(|(_, value)| *value))?
                    } else {
                        maximum_bound(launches.values().flatten().copied())?
                    };
                    bounds.push(ComparisonBound {
                        constraint_id: constraint.clone(),
                        statistic: ComparisonStatistic::MaximumBound,
                        launch: None,
                        value,
                        valid_observation_count,
                    });
                }
            }
        }
    }
    Ok(bounds)
}

/// Reports why a measurement record, template, or comparison input is invalid.
#[derive(Debug, Error)]
pub enum MeasurementError {
    /// A JSON value could not be decoded into a typed measurement record.
    #[error("measurement JSON is invalid")]
    Json(#[source] serde_json::Error),
    /// A typed measurement record could not be converted back to JSON.
    #[error("measurement JSON could not be serialized")]
    Serialization(#[source] serde_json::Error),
    /// The raw record did not declare schema version 2.0.0.
    #[error("raw measurement schema version is invalid")]
    RawSchemaVersion,
    /// The sample-validity record did not declare its staged schema identity.
    #[error("sample-validity schema identity is invalid")]
    SampleValiditySchema,
    /// The sample-validity record did not declare schema version 1.0.0.
    #[error("sample-validity schema version is invalid")]
    SampleValiditySchemaVersion,
    /// The candidate identifier was outside the closed candidate set.
    #[error("measurement candidate is invalid")]
    Candidate,
    /// The environment identifier was outside the Tier 1 environment set.
    #[error("measurement environment is invalid")]
    Environment,
    /// A constraint identifier did not use the closed constraint syntax.
    #[error("measurement constraint is invalid")]
    Constraint,
    /// A syntactically valid constraint was absent from the authoritative PRD set.
    #[error("measurement constraint is not authoritative")]
    UnknownConstraint,
    /// The record's lock digest was malformed or differed from the expected lock digest.
    #[error("measurement lock digest is invalid")]
    LockDigest,
    /// The meter version was empty or whitespace only.
    #[error("measurement meter version is invalid")]
    MeterVersion,
    /// A sample's launch or ordinal was zero.
    #[error("measurement sample launch or ordinal is invalid")]
    SampleOrdinal,
    /// A sample's unit was empty or whitespace only.
    #[error("measurement sample unit is invalid")]
    Unit,
    /// A sample value was not finite.
    #[error("measurement sample value is invalid")]
    Value,
    /// A sample's admission value contradicted its exclusion reason.
    #[error("measurement sample validity and exclusion are inconsistent")]
    SampleValidity,
    /// A raw measurement contained no samples.
    #[error("raw measurement has no samples")]
    NoSamples,
    /// A raw measurement repeated a constraint, launch, and ordinal tuple.
    #[error("raw measurement has a duplicate sample key")]
    DuplicateSampleKey,
    /// Raw sample timestamps decreased in record order.
    #[error("raw measurement timestamps are not monotonic")]
    NonMonotonicTime,
    /// A sample's harness-log path or digest was malformed.
    #[error("measurement harness-log binding is invalid")]
    HarnessLog,
    /// A harness-log reference failed repository confinement or digest verification.
    #[error("measurement harness-log verification failed")]
    Evidence(#[source] EvidenceError),
    /// The staged record did not contain the exact closed exclusion-category list.
    #[error("sample-validity exclusion categories are invalid")]
    ExclusionCategories,
    /// The staged record did not declare at least one comparison rule.
    #[error("sample-validity comparison rules are missing")]
    NoComparisonRules,
    /// The staged record repeated a constraint and statistic pair.
    #[error("sample-validity comparison rule is duplicated")]
    DuplicateComparisonRule,
    /// A staged comparison rule was incomplete or could discard valid observations.
    #[error("sample-validity comparison rule is invalid")]
    ComparisonRule,
    /// The selected constraint has no PRD-defined nearest-rank or maximum-bound rule.
    #[error("measurement template needs an unstated comparison rule")]
    UnsupportedComparisonMeter,
    /// A template was given samples for another constraint.
    #[error("raw-measurement template contains a different constraint")]
    TemplateConstraint,
    /// Paired raw-measurement and sample-validity records had different identity bindings.
    #[error("measurement records do not share one identity binding")]
    InconsistentBinding,
    /// A raw sample had no corresponding staged comparison rule.
    #[error("raw sample has no comparison rule")]
    MissingComparisonRule,
    /// A comparison rule had no admitted observations.
    #[error("comparison rule has no admitted observations")]
    NoValidObservations,
    /// A checked arithmetic operation could not represent the result.
    #[error("measurement arithmetic overflowed")]
    Arithmetic,
    /// The qualification lock could not be read and hashed during template generation.
    #[error("qualification lock could not be hashed")]
    Lock {
        /// The lock file that could not be hashed.
        path: PathBuf,
        /// The local I/O failure.
        #[source]
        source: io::Error,
    },
}

fn validate_validity(
    valid: bool,
    exclusion_reason: Option<RawExclusionReason>,
) -> Result<(), MeasurementError> {
    match (valid, exclusion_reason) {
        (true, None) | (false, Some(_)) => Ok(()),
        (true, Some(_)) | (false, None) => Err(MeasurementError::SampleValidity),
    }
}

fn validate_lock_digest(declared: &str, expected: Sha256Digest) -> Result<(), MeasurementError> {
    let declared = declared
        .parse::<Sha256Digest>()
        .map_err(|_| MeasurementError::LockDigest)?;
    if declared == expected {
        Ok(())
    } else {
        Err(MeasurementError::LockDigest)
    }
}

fn validate_meter_version(meter_version: &str) -> Result<(), MeasurementError> {
    if meter_version.trim().is_empty() {
        Err(MeasurementError::MeterVersion)
    } else {
        Ok(())
    }
}

fn nearest_rank_values(
    launches: &BTreeMap<u64, Vec<f64>>,
    percentile: u8,
) -> Result<Vec<(u64, f64)>, MeasurementError> {
    launches
        .iter()
        .map(|(launch, values)| {
            let mut values = values.clone();
            values.sort_by(f64::total_cmp);
            let count = u64::try_from(values.len()).map_err(|_| MeasurementError::Arithmetic)?;
            let rank = u64::from(percentile)
                .checked_mul(count)
                .ok_or(MeasurementError::Arithmetic)?
                .div_ceil(100);
            let index = usize::try_from(rank.checked_sub(1).ok_or(MeasurementError::Arithmetic)?)
                .map_err(|_| MeasurementError::Arithmetic)?;
            let value = values
                .get(index)
                .copied()
                .ok_or(MeasurementError::NoValidObservations)?;
            Ok((*launch, value))
        })
        .collect()
}

fn maximum_bound(values: impl Iterator<Item = f64>) -> Result<f64, MeasurementError> {
    values
        .max_by(f64::total_cmp)
        .ok_or(MeasurementError::NoValidObservations)
}

/// One PRD-defined comparison rule for a meter that has explicit sample-validity inputs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PrdMeterRule {
    constraint_id: &'static str,
    statistic: ComparisonStatistic,
    percentile: Option<u8>,
    unit: &'static str,
}

impl PrdMeterRule {
    fn matches_constraint(self, constraint: &ConstraintId) -> bool {
        self.constraint_id == constraint.as_str()
    }

    fn matches(self, rule: &ComparisonBoundInput) -> bool {
        self.statistic == rule.statistic
            && self.percentile == rule.percentile
            && self.unit == rule.unit
    }
}

/// The complete set of PRD meters whose explicit nearest-rank or maximum-bound inputs can be
/// represented by a sample-validity rule.
const PRD_METER_TABLE: &[PrdMeterRule] = &[
    PrdMeterRule {
        constraint_id: "CON-PERF-001",
        statistic: ComparisonStatistic::NearestRank,
        percentile: Some(99),
        unit: "ms",
    },
    PrdMeterRule {
        constraint_id: "CON-PERF-001",
        statistic: ComparisonStatistic::MaximumBound,
        percentile: None,
        unit: "ms",
    },
    PrdMeterRule {
        constraint_id: "CON-PERF-003",
        statistic: ComparisonStatistic::MaximumBound,
        percentile: None,
        unit: "ms",
    },
    PrdMeterRule {
        constraint_id: "CON-MEM-001",
        statistic: ComparisonStatistic::NearestRank,
        percentile: Some(95),
        unit: "MiB",
    },
    PrdMeterRule {
        constraint_id: "CON-MEM-001",
        statistic: ComparisonStatistic::MaximumBound,
        percentile: None,
        unit: "MiB",
    },
];

fn prd_meter_rules_for(constraint: &ConstraintId) -> impl Iterator<Item = &'static PrdMeterRule> {
    PRD_METER_TABLE
        .iter()
        .filter(move |rule| rule.matches_constraint(constraint))
}

#[cfg(test)]
#[path = "measurement_tests.rs"]
mod tests;
