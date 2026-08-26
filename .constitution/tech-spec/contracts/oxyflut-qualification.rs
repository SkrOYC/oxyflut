#![deny(missing_docs, unsafe_code)]
#![allow(dead_code)]

//! Qualification-driver contract for the two substrate candidates.

use std::error::Error;

/// Selects one qualification candidate.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Candidate {
    /// Focused standalone drawing-and-text allocation.
    Focused,
    /// Integrated engine allocation.
    Integrated,
}

/// Selects one Tier 1 reference environment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Environment {
    /// macOS on arm64.
    MacOs,
    /// Windows on x86-64.
    Windows,
    /// Wayland on Linux x86-64.
    Wayland,
    /// X11 on Linux x86-64.
    X11,
}

/// Identifies one P0 capability gate.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CapabilityId {
    /// CAP-CMP-001.
    Cmp001,
    /// CAP-CMP-002.
    Cmp002,
    /// CAP-CMP-003.
    Cmp003,
    /// CAP-CMP-004.
    Cmp004,
    /// CAP-CMP-005.
    Cmp005,
    /// CAP-CMP-006.
    Cmp006,
    /// CAP-CMP-007.
    Cmp007,
    /// CAP-LAY-001.
    Lay001,
    /// CAP-LAY-002.
    Lay002,
    /// CAP-SCR-001.
    Scr001,
    /// CAP-SCR-002.
    Scr002,
    /// CAP-REN-001.
    Ren001,
    /// CAP-REN-002.
    Ren002,
    /// CAP-REN-003.
    Ren003,
    /// CAP-AST-001.
    Ast001,
    /// CAP-AST-002.
    Ast002,
    /// CAP-AST-003.
    Ast003,
    /// CAP-AST-004.
    Ast004,
    /// CAP-VIEW-001.
    View001,
    /// CAP-VIEW-002.
    View002,
    /// CAP-VIEW-003.
    View003,
    /// CAP-VIEW-004.
    View004,
    /// CAP-VIEW-005.
    View005,
    /// CAP-REC-001.
    Rec001,
    /// CAP-INP-001.
    Inp001,
    /// CAP-INP-002.
    Inp002,
    /// CAP-FOC-001.
    Foc001,
    /// CAP-TXT-001.
    Txt001,
    /// CAP-TXT-002.
    Txt002,
    /// CAP-TXT-003.
    Txt003,
    /// CAP-IME-001.
    Ime001,
    /// CAP-CLP-001.
    Clp001,
    /// CAP-I18N-001.
    I18n001,
    /// CAP-SEM-001.
    Sem001,
    /// CAP-SEM-002.
    Sem002,
    /// CAP-PLT-001.
    Plt001,
    /// CAP-OS-001.
    Os001,
    /// CAP-OS-002.
    Os002,
    /// CAP-TST-001.
    Tst001,
    /// CAP-TST-002.
    Tst002,
    /// CAP-TST-003.
    Tst003,
    /// CAP-TST-004.
    Tst004,
    /// CAP-DST-001.
    Dst001,
    /// CAP-SEC-001.
    Sec001,
    /// CAP-DIA-001.
    Dia001,
    /// CAP-DIA-002.
    Dia002,
    /// CAP-DIA-003.
    Dia003,
    /// CAP-DIA-004.
    Dia004,
    /// CAP-SUB-001.
    Sub001,
    /// CAP-SUB-002.
    Sub002,
    /// CAP-SUB-003.
    Sub003,
    /// CAP-SUB-004.
    Sub004,
}

/// Identifies one nonfunctional constraint gate.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConstraintId {
    /// CON-PERF-001.
    Perf001,
    /// CON-PERF-002.
    Perf002,
    /// CON-PERF-003.
    Perf003,
    /// CON-MEM-001.
    Mem001,
    /// CON-SIZE-001.
    Size001,
    /// CON-SIZE-002.
    Size002,
    /// CON-FRM-001.
    Frm001,
    /// CON-FRM-002.
    Frm002,
    /// CON-REC-001.
    Rec001,
    /// CON-REC-002.
    Rec002,
    /// CON-REC-003.
    Rec003,
    /// CON-REC-004.
    Rec004,
    /// CON-REC-005.
    Rec005,
    /// CON-REC-006.
    Rec006,
    /// CON-REC-007.
    Rec007,
    /// CON-DET-001.
    Det001,
    /// CON-DET-002.
    Det002,
    /// CON-UPG-001.
    Upg001,
    /// CON-COMP-001.
    Comp001,
    /// CON-SAFE-001.
    Safe001,
    /// CON-SEC-001.
    Sec001,
    /// CON-SEC-002.
    Sec002,
    /// CON-SEC-003.
    Sec003,
    /// CON-PRV-001.
    Prv001,
    /// CON-DIA-001.
    Dia001,
    /// CON-DST-001.
    Dst001,
    /// CON-LIC-001.
    Lic001,
}

/// Selects a gate result without allowing an implementation plan to count as evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GateStatus {
    /// The frozen gate passed.
    Pass,
    /// The frozen gate failed.
    Fail,
    /// A named unanswered question keeps eligibility open.
    GatingKnownUnknown,
    /// The behavior is inapplicable and cited evidence proves that fact.
    NotApplicableKnownKnown,
}

/// Identifies immutable evidence by repository-relative path and digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceRef {
    /// Repository-relative evidence path.
    pub path: String,
    /// Lowercase SHA-256 digest.
    pub sha256: [u8; 32],
    /// Media type of the preserved bytes.
    pub media_type: String,
}

/// Reports one capability or constraint gate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GateResult<I> {
    /// Gate identity.
    pub id: I,
    /// Gate outcome.
    pub status: GateStatus,
    /// Immutable supporting evidence.
    pub evidence: Vec<EvidenceRef>,
}

/// Describes one raw scalar measurement before aggregation.
#[derive(Clone, Debug, PartialEq)]
pub struct RawSample {
    /// Measured constraint.
    pub constraint: ConstraintId,
    /// One-based process launch.
    pub launch: u32,
    /// One-based sample ordinal.
    pub ordinal: u64,
    /// Monotonic observation timestamp.
    pub monotonic_ns: u64,
    /// Directly observed scalar.
    pub value: f64,
    /// Frozen unit name.
    pub unit: String,
    /// True when the sample is admitted by the predeclared validity rules.
    pub valid: bool,
    /// Allowed exclusion category when invalid.
    pub exclusion_reason: Option<String>,
    /// Harness log containing the observation.
    pub harness_log: EvidenceRef,
}

/// Runs one candidate through the shared qualification corpus.
pub trait CandidateProbe {
    /// Structured probe failure type.
    type Error: Error + Send + Sync + 'static;

    /// Verifies source, binary, configuration, and contract identity before execution.
    fn verify_lock(
        &mut self,
        candidate: Candidate,
        lock_sha256: [u8; 32],
    ) -> Result<(), Self::Error>;

    /// Runs one frozen P0 capability vector and preserves all output.
    fn run_capability(
        &mut self,
        candidate: Candidate,
        environment: Environment,
        capability: CapabilityId,
    ) -> Result<GateResult<CapabilityId>, Self::Error>;

    /// Runs one frozen nonfunctional meter without dropping raw samples.
    fn run_constraint(
        &mut self,
        candidate: Candidate,
        environment: Environment,
        constraint: ConstraintId,
    ) -> Result<(GateResult<ConstraintId>, Vec<RawSample>), Self::Error>;

    /// Runs one implemented-ingress fuzz campaign for the frozen duration and corpus.
    fn fuzz_ingress(
        &mut self,
        candidate: Candidate,
        environment: Environment,
        ingress_id: &str,
    ) -> Result<GateResult<ConstraintId>, Self::Error>;

    /// Builds and inspects one canonical unsigned release artifact and metadata bundle.
    fn qualify_artifact(
        &mut self,
        candidate: Candidate,
        environment: Environment,
    ) -> Result<Vec<EvidenceRef>, Self::Error>;

    /// Runs one of the two frozen consecutive upgrade transitions.
    fn rehearse_upgrade(
        &mut self,
        candidate: Candidate,
        from_commit: [u8; 20],
        to_commit: [u8; 20],
    ) -> Result<Vec<EvidenceRef>, Self::Error>;

    /// Applies the frozen security patch without accepting unrelated changes.
    fn rehearse_security_patch(
        &mut self,
        candidate: Candidate,
    ) -> Result<Vec<EvidenceRef>, Self::Error>;
}

/// Validates evidence completeness and performs the deterministic selection algorithm.
pub trait QualificationDecision {
    /// Structured validation failure type.
    type Error: Error + Send + Sync + 'static;

    /// Rejects missing environments, capabilities, constraints, evidence, or unresolved gates.
    fn validate_candidate_evidence(
        &self,
        candidate: Candidate,
        evidence: &EvidenceRef,
    ) -> Result<(), Self::Error>;

    /// Recomputes every weighted score from two assessor records and cited evidence.
    fn validate_scores(
        &self,
        candidate: Candidate,
        evidence: &EvidenceRef,
    ) -> Result<(), Self::Error>;

    /// Applies zero-, one-, or two-eligible-candidate rules and the maintenance tie-break.
    fn select(
        &self,
        focused: &EvidenceRef,
        integrated: &EvidenceRef,
    ) -> Result<EvidenceRef, Self::Error>;

    /// Validates every Phase 3B promotion reference and its binding to the lock and version.
    fn validate_promotion(&self, phase_record: &EvidenceRef) -> Result<(), Self::Error>;
}
