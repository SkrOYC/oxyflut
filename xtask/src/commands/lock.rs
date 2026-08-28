//! Qualification-lock readiness status reporting.

use std::fs;
use std::path::{Path, PathBuf};

use oxyflut_qualification::readiness::{
    ReadinessBlocking, ReadinessReport, ReadinessStatus, candidate_implementation_report,
};
use serde_json::Value;

use super::super::{CommandError, CommandOutcome};
use crate::contracts as validators;
use validators::readiness::GateStatus;

const LOCK_PATH: &str = ".constitution/tech-spec/contracts/qualification-lock.json";

/// Validates one requested readiness gate without modifying the qualification lock.
///
/// The command runs the schema, instance, exact-set, registry, and digest families before the
/// readiness gate. A failed family returns exit code 1 as `lock status: invalid (FAMILY; PATH)`.
pub(crate) fn run(arguments: &[String]) -> CommandOutcome {
    let gate = match arguments {
        [flag, gate] if flag == "--gate" => gate.as_str(),
        _ => {
            return CommandOutcome::failed(CommandError::InvalidInput {
                code: "lock-status-arguments",
            });
        }
    };
    let root = match workspace_root() {
        Ok(root) => root,
        Err(()) => {
            return CommandOutcome::failed(CommandError::Execution {
                code: "workspace-root",
                hint: "rerun: lock status --gate GATE",
            });
        }
    };
    run_at_root(&root, gate)
}

fn run_at_root(root: &Path, gate: &str) -> CommandOutcome {
    if let Some(failure) = super::contracts::first_pre_implementation_input_failure(root) {
        println!("{}", invalid_status_line(&failure));
        return CommandOutcome::failed(CommandError::ValidationFailed {
            code: "lock-invalid",
            hint: "rerun: lock status --gate GATE",
        });
    }

    let validated = match validators::readiness::validate_workspace(root) {
        Ok(report) => report,
        Err(_) => {
            println!("lock status: invalid (readiness)");
            return CommandOutcome::failed(CommandError::ValidationFailed {
                code: "lock-invalid",
                hint: "rerun: lock status --gate GATE",
            });
        }
    };

    match gate {
        "candidate-implementation" => report_candidate_implementation_gate(root),
        "measurement" => report_gate(gate, &validated.measurement),
        _ => CommandOutcome::failed(CommandError::InvalidInput {
            code: "lock-status-gate",
        }),
    }
}

fn report_candidate_implementation_gate(root: &Path) -> CommandOutcome {
    let report = match candidate_report_at(root) {
        Ok(report) => report,
        Err(()) => {
            println!("lock status: invalid (readiness-report)");
            return CommandOutcome::failed(CommandError::ValidationFailed {
                code: "lock-invalid",
                hint: "rerun: lock status --gate GATE",
            });
        }
    };
    emit_candidate_report(&report)
}

fn emit_candidate_report(report: &ReadinessReport) -> CommandOutcome {
    for line in candidate_report_lines(report) {
        println!("{line}");
    }
    match report.status {
        ReadinessStatus::Ready => CommandOutcome::Success,
        ReadinessStatus::Open => CommandOutcome::ValidButOpen,
    }
}

fn candidate_report_at(root: &Path) -> Result<ReadinessReport, ()> {
    let bytes = fs::read(root.join(LOCK_PATH)).map_err(|_| ())?;
    let lock: Value = serde_json::from_slice(&bytes).map_err(|_| ())?;
    candidate_implementation_report(&lock).map_err(|_| ())
}

fn candidate_report_lines(report: &ReadinessReport) -> Vec<String> {
    let mut lines = Vec::with_capacity(report.blocking.len().saturating_add(1));
    lines.push(format!(
        "lock status: {} ({})",
        report.status.as_str(),
        report.gate.as_str()
    ));
    lines.extend(report.blocking.iter().map(blocking_line));
    lines
}

fn blocking_line(blocking: &ReadinessBlocking) -> String {
    let mut line = format!(
        "blocking: field-path={} kind={}",
        blocking.field_path,
        blocking.kind.as_str()
    );
    if let Some(evidence_path) = &blocking.evidence_path {
        line.push_str(" evidence-path=");
        line.push_str(evidence_path);
    }
    if let Some(upstream_owner) = &blocking.upstream_owner {
        line.push_str(" upstream-owner=");
        line.push_str(upstream_owner);
    }
    line
}

fn invalid_status_line(failure: &super::contracts::ValidationFamilyFailure) -> String {
    format!(
        "lock status: invalid ({}; {})",
        failure.family, failure.contract_path
    )
}

fn report_gate(gate: &str, status: &GateStatus) -> CommandOutcome {
    match status {
        GateStatus::Ready => {
            println!("lock status: ready ({gate})");
            CommandOutcome::Success
        }
        GateStatus::Open(_) => {
            println!("lock status: open ({gate})");
            for known_unknown in status.remaining_known_unknowns() {
                println!("remaining-ku: {known_unknown}");
            }
            CommandOutcome::ValidButOpen
        }
    }
}

fn workspace_root() -> Result<PathBuf, ()> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::error::Error;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::ExitCode;

    use oxyflut_qualification::hash::hash_file;
    use oxyflut_qualification::readiness::candidate_implementation_report;
    use serde_json::Value;

    use super::{
        candidate_report_at, candidate_report_lines, emit_candidate_report, invalid_status_line,
        run, run_at_root, workspace_root,
    };
    use crate::CommandOutcome;

    const COMPLETE_SYNTHETIC: &str = "qualification/fixtures/readiness/complete.synthetic.json";
    const INVALID: &str = "qualification/fixtures/readiness/invalid.json";
    const CLEARED_WITHOUT_EVIDENCE: &str =
        "qualification/fixtures/readiness/cleared-without-evidence.json";
    #[test]
    fn committed_candidate_gate_is_valid_but_open_with_the_exact_ku_set()
    -> Result<(), Box<dyn Error>> {
        let root = source_root()?;
        let report = candidate_report_at(&root).map_err(|_| "committed report must parse")?;
        let known_unknowns = report
            .blocking
            .iter()
            .filter(|blocking| blocking.kind == oxyflut_qualification::readiness::BlockingKind::Ku)
            .map(|blocking| {
                blocking
                    .field_path
                    .strip_prefix("preImplementationKnownUnknowns.")
                    .ok_or("KU paths must name the lock array member")
            })
            .collect::<Result<Vec<_>, _>>()?;

        assert_eq!(
            run(&["--gate".to_owned(), "candidate-implementation".to_owned()]),
            CommandOutcome::ValidButOpen
        );
        assert_eq!(
            known_unknowns,
            vec![
                "capability-and-platform-baselines",
                "complete-ime-editing-geometry-and-accessibility-maps",
                "external-distribution-schema-snapshots-and-verifiers",
                "fuzz-corpora",
                "hardware-gpu-driver-and-system-package-locks",
                "independent-presentation-opportunity-sources",
                "layout-visit-cap",
                "minimum-platform-and-protocol-versions",
                "raw-measurement-and-sample-validity-contracts",
                "reference-application-scenes-scripts-fonts-assets-windows-cache-and-flags",
                "resolved-tool-digests",
                "scoring-anchors-and-two-assessors",
                "security-patch-rehearsal",
            ]
        );
        Ok(())
    }

    #[test]
    fn complete_synthetic_fixture_returns_exit_zero() -> Result<(), Box<dyn Error>> {
        let lock: Value =
            serde_json::from_slice(&fs::read(source_root()?.join(COMPLETE_SYNTHETIC))?)?;
        let report = candidate_implementation_report(&lock)?;
        let outcome = emit_candidate_report(&report);

        assert_eq!(outcome, CommandOutcome::Success);
        assert_eq!(outcome.exit_code(), ExitCode::SUCCESS);
        Ok(())
    }

    #[test]
    fn invalid_referenced_input_fixture_returns_exit_one() -> Result<(), Box<dyn Error>> {
        let root = open_fixture_root("invalid")?;
        fs::copy(source_root()?.join(INVALID), root.join(super::LOCK_PATH))?;
        assert!(super::super::contracts::first_pre_implementation_input_failure(&root).is_none());
        let outcome = run_at_root(&root, "candidate-implementation");
        fs::remove_dir_all(&root)?;

        assert!(matches!(outcome, CommandOutcome::Failed(_)));
        assert_eq!(outcome.exit_code(), ExitCode::FAILURE);
        Ok(())
    }

    #[test]
    fn cleared_ku_without_evidence_remains_open_with_the_exact_remaining_ku_set()
    -> Result<(), Box<dyn Error>> {
        let root = open_fixture_root("cleared")?;
        let report =
            candidate_report_at(&root).map_err(|_| "candidate readiness report must parse")?;
        let known_unknowns = report
            .blocking
            .iter()
            .filter(|blocking| blocking.kind == oxyflut_qualification::readiness::BlockingKind::Ku)
            .map(|blocking| {
                blocking
                    .field_path
                    .strip_prefix("preImplementationKnownUnknowns.")
                    .ok_or("KU paths must name the lock array member")
            })
            .collect::<Result<Vec<_>, _>>()?;
        let outcome = run_at_root(&root, "candidate-implementation");
        fs::remove_dir_all(&root)?;

        assert_eq!(outcome, CommandOutcome::ValidButOpen);
        assert_eq!(outcome.exit_code(), ExitCode::from(2));
        assert_eq!(
            known_unknowns,
            vec![
                "capability-and-platform-baselines",
                "complete-ime-editing-geometry-and-accessibility-maps",
                "external-distribution-schema-snapshots-and-verifiers",
                "fuzz-corpora",
                "hardware-gpu-driver-and-system-package-locks",
                "independent-presentation-opportunity-sources",
                "layout-visit-cap",
                "minimum-platform-and-protocol-versions",
                "raw-measurement-and-sample-validity-contracts",
                "reference-application-scenes-scripts-fonts-assets-windows-cache-and-flags",
                "scoring-anchors-and-two-assessors",
                "security-patch-rehearsal",
            ]
        );
        Ok(())
    }

    #[test]
    fn candidate_report_lines_are_stable_and_content_free() -> Result<(), Box<dyn Error>> {
        let root = open_fixture_root("report-lines")?;
        let report =
            candidate_report_at(&root).map_err(|_| "candidate readiness report must parse")?;
        let lines = candidate_report_lines(&report);
        fs::remove_dir_all(&root)?;

        assert_eq!(
            lines.first().map(String::as_str),
            Some("lock status: open (candidate-implementation)")
        );
        assert!(lines.iter().all(|line| {
            !line.contains("macOS with Xcode")
                && !line.contains("https://")
                && !line.contains("Apple")
        }));
        assert!(lines.iter().any(|line| {
            line == "blocking: field-path=resolvedTools kind=missing evidence-path=qualification/tools/native-contract-toolchain.json upstream-owner=OXY-A008"
        }));
        Ok(())
    }

    #[test]
    fn lock_status_never_mutates_constitution() -> Result<(), Box<dyn Error>> {
        let root = source_root()?;
        let before = constitution_digest(&root)?;
        let outcome = run_at_root(&root, "candidate-implementation");
        let after = constitution_digest(&root)?;

        assert_eq!(outcome, CommandOutcome::ValidButOpen);
        assert_eq!(before, after);
        Ok(())
    }

    #[test]
    fn corrupt_platform_baseline_fails_before_reporting_an_open_gate() -> Result<(), Box<dyn Error>>
    {
        let source = workspace_root().map_err(|_| "xtask must remain below the workspace root")?;
        let root = temporary_directory("corrupt-platform-baseline");
        copy_directory(&source.join(".constitution"), &root.join(".constitution"))?;
        copy_directory(&source.join("qualification"), &root.join("qualification"))?;

        let platform_path = root.join(".constitution/tech-spec/contracts/platform-contracts.json");
        let original = fs::read_to_string(&platform_path)?;
        let corrupted = original.replacen(
            "\"specificationVersion\": \"0.15.0\"",
            "\"specificationVersion\": \"0.15.1\"",
            1,
        );
        assert_ne!(corrupted, original);
        fs::write(platform_path, corrupted)?;

        let outcome = run_at_root(&root, "candidate-implementation");
        assert!(matches!(&outcome, CommandOutcome::Failed(_)));
        assert_eq!(outcome.exit_code(), ExitCode::FAILURE);
        let failure = super::super::contracts::first_pre_implementation_input_failure(&root)
            .ok_or("the corrupt platform baseline must fail a pre-implementation family")?;
        assert_eq!(
            invalid_status_line(&failure),
            "lock status: invalid (exact-set; .constitution/tech-spec/contracts/capability-traceability.json)"
        );
        fs::remove_dir_all(root)?;
        Ok(())
    }

    #[test]
    fn reports_measurement_gate_and_rejects_invalid_arguments() {
        assert_eq!(
            run(&["--gate".to_owned(), "measurement".to_owned()]),
            CommandOutcome::ValidButOpen
        );
        assert!(matches!(run(&[]), CommandOutcome::Failed(_)));
        assert!(matches!(
            run(&["--gate".to_owned(), "production".to_owned()]),
            CommandOutcome::Failed(_)
        ));
    }

    fn open_fixture_root(name: &str) -> Result<PathBuf, Box<dyn Error>> {
        let source = source_root()?;
        let root = temporary_directory(name);
        copy_directory(&source.join(".constitution"), &root.join(".constitution"))?;
        copy_directory(&source.join("qualification"), &root.join("qualification"))?;
        fs::copy(
            source.join(CLEARED_WITHOUT_EVIDENCE),
            root.join(super::LOCK_PATH),
        )?;
        Ok(root)
    }

    fn source_root() -> Result<PathBuf, Box<dyn Error>> {
        workspace_root().map_err(|_| "xtask must remain directly below the workspace root".into())
    }

    fn constitution_digest(root: &Path) -> Result<BTreeMap<PathBuf, String>, Box<dyn Error>> {
        let directory = root.join(".constitution");
        let mut digests = BTreeMap::new();
        collect_file_digests(&directory, &mut digests)?;
        Ok(digests)
    }

    fn collect_file_digests(
        directory: &Path,
        digests: &mut BTreeMap<PathBuf, String>,
    ) -> Result<(), Box<dyn Error>> {
        let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, std::io::Error>>()?;
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                collect_file_digests(&path, digests)?;
            } else {
                let digest = hash_file(&path)?.to_string();
                digests.insert(path, digest);
            }
        }
        Ok(())
    }

    fn temporary_directory(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("oxyflut-c005-{name}-{}", std::process::id()))
    }

    fn copy_directory(source: &Path, destination: &Path) -> Result<(), std::io::Error> {
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
}
