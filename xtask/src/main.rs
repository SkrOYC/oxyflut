//! Command dispatcher for qualification-only repository tooling.

use std::process::ExitCode;

mod commands;
mod evidence;
mod toolchain;

/// Runs one qualification command.
fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let outcome = match dispatch(&arguments) {
        Ok(route) => execute(route),
        Err(error) => CommandOutcome::Failed(error),
    };

    if let Some(diagnostic) = outcome.diagnostic() {
        eprintln!("{diagnostic}");
    }

    ExitCode::from(outcome.exit_code())
}

/// Selects a registered qualification command placeholder.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CommandRoute {
    /// The contracts-validation command.
    Contracts,
    /// The evidence-verification command.
    Evidence,
    /// The external-contract validation command.
    ExternalContracts,
    /// The baseline-validation command.
    Baseline,
    /// The raw-measurement validation command.
    Measurement,
    /// The reference-environment inspection command.
    Environment,
    /// The qualification-lock status command.
    Lock,
    /// The candidate-build command.
    Candidate,
    /// The candidate-probe command.
    Probe,
    /// The candidate-qualification command.
    Qualify,
}

/// Classifies a command invocation without inspecting unimplemented arguments.
fn dispatch(arguments: &[String]) -> Result<CommandRoute, DispatchError> {
    match arguments {
        [command, action, ..] if command == "contracts" && action == "validate" => {
            Ok(CommandRoute::Contracts)
        }
        [command, action, ..] if command == "evidence" && action == "verify" => {
            Ok(CommandRoute::Evidence)
        }
        [command, action, ..] if command == "external-contracts" && action == "verify" => {
            Ok(CommandRoute::ExternalContracts)
        }
        [command, action, ..] if command == "baseline" && action == "validate" => {
            Ok(CommandRoute::Baseline)
        }
        [command, action, ..] if command == "measurement" && action == "validate" => {
            Ok(CommandRoute::Measurement)
        }
        [command, action, ..] if command == "environment" && action == "inspect" => {
            Ok(CommandRoute::Environment)
        }
        [command, action, ..] if command == "lock" && action == "status" => Ok(CommandRoute::Lock),
        [command, action, ..] if command == "candidate" && action == "build" => {
            Ok(CommandRoute::Candidate)
        }
        [command, ..] if command == "probe" => Ok(CommandRoute::Probe),
        [command, ..] if command == "qualify" => Ok(CommandRoute::Qualify),
        _ => Err(DispatchError::InvalidCommand),
    }
}

/// Routes a recognized command to its owned placeholder module.
fn execute(route: CommandRoute) -> CommandOutcome {
    match route {
        CommandRoute::Contracts => commands::contracts::run(),
        CommandRoute::Evidence => commands::evidence::run(),
        CommandRoute::ExternalContracts => commands::external_contracts::run(),
        CommandRoute::Baseline => commands::baseline::run(),
        CommandRoute::Measurement => commands::measurement::run(),
        CommandRoute::Environment => commands::environment::run(),
        CommandRoute::Lock => commands::lock::run(),
        CommandRoute::Candidate => commands::candidate::run(),
        CommandRoute::Probe => commands::probe::run(),
        CommandRoute::Qualify => commands::qualify::run(),
    }
}

/// Describes an exit outcome without allowing a command to claim success.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum CommandOutcome {
    /// The command completed successfully.
    #[allow(
        dead_code,
        reason = "Later ticket-owned command modules return successful outcomes."
    )]
    Success,
    /// The command failed validation or execution.
    Failed(DispatchError),
    /// The lock was valid but the requested readiness gate remains open.
    #[allow(
        dead_code,
        reason = "The OXY-A004 lock command owns this open-gate outcome."
    )]
    ValidButOpen,
}

impl CommandOutcome {
    /// Creates a named unimplemented-command failure.
    pub(crate) const fn not_implemented(command: &'static str) -> Self {
        Self::Failed(DispatchError::NotImplemented { command })
    }

    /// Returns the process exit code required by the qualification command contract.
    const fn exit_code(&self) -> u8 {
        match self {
            Self::Success => 0,
            Self::Failed(_) => 1,
            Self::ValidButOpen => 2,
        }
    }

    /// Returns a content-free failure diagnostic when one is available.
    const fn diagnostic(&self) -> Option<&DispatchError> {
        match self {
            Self::Success => None,
            Self::Failed(error) => Some(error),
            Self::ValidButOpen => None,
        }
    }
}

/// Reports invalid or unimplemented qualification command invocations.
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub(crate) enum DispatchError {
    /// The command name doesn't match the qualification command contract.
    #[error("invalid command")]
    InvalidCommand,
    /// The command has a registered placeholder but no implementation yet.
    #[error("not implemented: {command}")]
    NotImplemented {
        /// The content-free command name.
        command: &'static str,
    },
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    use super::{CommandOutcome, CommandRoute, DispatchError, dispatch, execute};

    const WORKSPACE_MEMBERS: &[&str] = &[
        "crates/oxyflut",
        "crates/oxyflut-runtime",
        "crates/oxyflut-layout",
        "crates/oxyflut-scene",
        "crates/oxyflut-assets",
        "crates/oxyflut-view",
        "crates/oxyflut-input",
        "crates/oxyflut-text",
        "crates/oxyflut-semantics",
        "crates/oxyflut-platform",
        "crates/oxyflut-diagnostics",
        "crates/oxyflut-qualification",
        "crates/oxyflut-substrate",
        "crates/oxyflut-substrate-impeller",
        "crates/oxyflut-substrate-engine",
        "xtask",
    ];

    const DOCUMENTATION_ONLY_CRATES: &[&str] = &[
        "crates/oxyflut",
        "crates/oxyflut-runtime",
        "crates/oxyflut-layout",
        "crates/oxyflut-scene",
        "crates/oxyflut-assets",
        "crates/oxyflut-view",
        "crates/oxyflut-input",
        "crates/oxyflut-text",
        "crates/oxyflut-semantics",
        "crates/oxyflut-platform",
        "crates/oxyflut-diagnostics",
        "crates/oxyflut-substrate",
        "crates/oxyflut-substrate-impeller",
        "crates/oxyflut-substrate-engine",
    ];

    const QUALIFICATION_MODULES: &[&str] = &[
        "schema",
        "identifiers",
        "readiness",
        "evidence",
        "hash",
        "baseline",
        "measurement",
        "environment",
    ];

    const STACK_ALLOWED_DEPENDENCIES: &[(&str, &str)] = &[
        ("thiserror", "2.0.20"),
        ("slotmap", "1.1.1"),
        ("smallvec", "1.15.1"),
        ("crossbeam-channel", "0.5.16"),
        ("bitflags", "2.13.1"),
        ("unicode-segmentation", "1.13.3"),
        ("image", "0.25.10"),
        ("serde", "1.0.229"),
        ("serde_json", "1.0.151"),
        ("sha2", "0.11.0"),
        ("jsonschema", "0.51.0"),
        ("proptest", "1.11.0"),
        ("criterion", "0.8.2"),
        ("bindgen", "0.72.1"),
        ("cbindgen", "0.29.4"),
        ("objc2", "0.6.4"),
        ("objc2-app-kit", "0.3.2"),
        ("windows", "0.62.2"),
        ("gtk4", "0.11.4"),
        ("glib", "0.22.8"),
        ("wayland-client", "0.31.15"),
        ("wayland-protocols", "0.32.13"),
        ("x11rb", "0.14.0"),
    ];

    #[test]
    fn cargo_metadata_lists_each_stage_three_workspace_member_once() -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let output = Command::new(env!("CARGO"))
            .current_dir(&root)
            .args([
                "metadata",
                "--format-version",
                "1",
                "--no-deps",
                "--offline",
            ])
            .output()?;
        assert!(output.status.success());

        let metadata = String::from_utf8(output.stdout)?;
        for member in WORKSPACE_MEMBERS {
            let manifest_path = root.join(member).join("Cargo.toml");
            let manifest_field = format!("\"manifest_path\":\"{}\"", json_path(&manifest_path));
            assert_eq!(
                metadata.matches(&manifest_field).count(),
                1,
                "metadata must contain {member} exactly once"
            );
        }
        assert_eq!(
            metadata.matches("\"manifest_path\":").count(),
            WORKSPACE_MEMBERS.len(),
            "metadata must not contain an unlisted workspace member"
        );

        Ok(())
    }

    #[test]
    fn packages_inherit_the_workspace_edition_rust_version_and_lints() -> Result<(), Box<dyn Error>>
    {
        let root = workspace_root()?;
        let workspace_manifest = read_file(&root.join("Cargo.toml"))?;
        assert!(workspace_manifest.contains("edition = \"2024\""));
        assert!(workspace_manifest.contains("rust-version = \"1.98.0\""));
        assert!(workspace_manifest.contains("[workspace.lints.rust]"));
        assert!(workspace_manifest.contains("[workspace.lints.clippy]"));

        for member in WORKSPACE_MEMBERS {
            let package_manifest = read_file(&root.join(member).join("Cargo.toml"))?;
            assert!(package_manifest.contains("edition.workspace = true"));
            assert!(package_manifest.contains("rust-version.workspace = true"));
            assert!(package_manifest.contains("[lints]\nworkspace = true"));
        }

        Ok(())
    }

    #[test]
    fn candidate_platform_and_product_crates_contain_documentation_only()
    -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        for crate_path in DOCUMENTATION_ONLY_CRATES {
            let source = read_file(&root.join(crate_path).join("src/lib.rs"))?;
            assert!(
                is_documentation_only(&source),
                "{crate_path} must not contain product, candidate, or platform behavior"
            );
        }

        for module in QUALIFICATION_MODULES {
            let source = read_file(
                &root
                    .join("crates/oxyflut-qualification/src")
                    .join(format!("{module}.rs")),
            )?;
            assert!(
                is_documentation_only(&source),
                "qualification placeholder {module} must not contain behavior"
            );
        }

        for directory in [
            "native/engine-bridge",
            "platform/macos",
            "platform/windows",
            "platform/linux",
        ] {
            assert_directory_contains_only_readme(&root.join(directory))?;
        }

        Ok(())
    }

    #[test]
    fn dispatcher_routes_every_qualification_command_to_a_named_failure() {
        let cases = [
            (
                &["contracts", "validate"][..],
                CommandRoute::Contracts,
                "contracts validate",
            ),
            (
                &["evidence", "verify", "PATH"][..],
                CommandRoute::Evidence,
                "evidence verify",
            ),
            (
                &["external-contracts", "verify"][..],
                CommandRoute::ExternalContracts,
                "external-contracts verify",
            ),
            (
                &["baseline", "validate", "--input", "PATH"][..],
                CommandRoute::Baseline,
                "baseline validate",
            ),
            (
                &["measurement", "validate", "--input", "PATH"][..],
                CommandRoute::Measurement,
                "measurement validate",
            ),
            (
                &[
                    "environment",
                    "inspect",
                    "--environment",
                    "E",
                    "--output",
                    "P",
                ][..],
                CommandRoute::Environment,
                "environment inspect",
            ),
            (
                &["lock", "status", "--gate", "G"][..],
                CommandRoute::Lock,
                "lock status",
            ),
            (
                &["candidate", "build", "--candidate", "focused", "--locked"][..],
                CommandRoute::Candidate,
                "candidate build",
            ),
            (
                &[
                    "probe",
                    "--candidate",
                    "CANDIDATE",
                    "--environment",
                    "ENVIRONMENT",
                ][..],
                CommandRoute::Probe,
                "probe",
            ),
            (
                &["qualify", "--all-candidates", "--locked"][..],
                CommandRoute::Qualify,
                "qualify",
            ),
        ];

        for (arguments, route, command) in cases {
            let arguments = arguments
                .iter()
                .map(|value| (*value).to_owned())
                .collect::<Vec<_>>();
            assert_eq!(dispatch(&arguments), Ok(route));
            assert_eq!(
                execute(route),
                CommandOutcome::not_implemented(command),
                "{command} must route to its named placeholder"
            );
            assert_eq!(execute(route).exit_code(), 1);
        }

        assert_eq!(dispatch(&[]), Err(DispatchError::InvalidCommand));
    }

    #[test]
    fn lockfile_direct_workspace_dependencies_are_pinned_stack_dependencies()
    -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let lockfile = read_file(&root.join("Cargo.lock"))?;

        for member in WORKSPACE_MEMBERS {
            let package_name = member
                .rsplit('/')
                .next()
                .ok_or("workspace member must have a name")?;
            let package = lock_package(&lockfile, package_name)
                .ok_or("every workspace package must have a Cargo.lock entry")?;
            for dependency in lock_dependencies(package) {
                let (_, expected_version) = STACK_ALLOWED_DEPENDENCIES
                    .iter()
                    .find(|(allowed_name, _)| *allowed_name == dependency)
                    .ok_or("workspace dependency must be allowlisted by stack.md")?;
                let dependency_package = lock_package(&lockfile, dependency)
                    .ok_or("workspace dependency must have a Cargo.lock entry")?;
                let actual_version = package_version(dependency_package)
                    .ok_or("Cargo.lock package entries must state a version")?;
                assert_eq!(actual_version, *expected_version);
            }
        }

        Ok(())
    }

    fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| "xtask must remain directly below the workspace root".into())
    }

    fn read_file(path: &Path) -> Result<String, Box<dyn Error>> {
        Ok(fs::read_to_string(path)?)
    }

    fn json_path(path: &Path) -> String {
        path.to_string_lossy().replace('\\', "\\\\")
    }

    fn is_documentation_only(source: &str) -> bool {
        source
            .lines()
            .all(|line| line.is_empty() || line.starts_with("//!"))
    }

    fn assert_directory_contains_only_readme(directory: &Path) -> Result<(), Box<dyn Error>> {
        let entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, std::io::Error>>()?;
        assert_eq!(entries.len(), 1);
        let file_name = entries[0].file_name();
        assert_eq!(file_name, "README.md");
        Ok(())
    }

    fn lock_package<'a>(lockfile: &'a str, name: &str) -> Option<&'a str> {
        let name_line = format!("name = \"{name}\"");
        lockfile
            .split("[[package]]")
            .find(|package| package.lines().any(|line| line == name_line))
    }

    fn lock_dependencies(package: &str) -> Vec<&str> {
        let Some((_, dependencies)) = package.split_once("dependencies = [") else {
            return Vec::new();
        };
        let Some((dependencies, _)) = dependencies.split_once(']') else {
            return Vec::new();
        };

        dependencies
            .lines()
            .filter_map(|line| {
                let dependency = line.trim().trim_end_matches(',').trim_matches('"');
                if dependency.is_empty() {
                    None
                } else {
                    dependency.split_whitespace().next()
                }
            })
            .collect()
    }

    fn package_version(package: &str) -> Option<&str> {
        package
            .lines()
            .find_map(|line| line.strip_prefix("version = \"")?.strip_suffix('"'))
    }
}
