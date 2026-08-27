//! Command dispatcher for qualification-only repository tooling.

use std::process::ExitCode;

mod commands;
mod evidence;
mod toolchain;

/// Runs one qualification command.
fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let outcome = match dispatch(&arguments) {
        Ok(invocation) => execute(invocation),
        Err(error) => CommandOutcome::failed(error),
    };

    if let Some(diagnostic) = outcome.diagnostic() {
        eprintln!("{diagnostic}");
    }

    outcome.exit_code()
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

/// Couples a recognized command route with the arguments owned by its command module.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CommandInvocation<'arguments> {
    route: CommandRoute,
    arguments: &'arguments [String],
}

/// Classifies a command invocation and preserves its command-specific arguments.
fn dispatch(arguments: &[String]) -> Result<CommandInvocation<'_>, CommandError> {
    match arguments {
        [command, action, remaining @ ..] if command == "contracts" && action == "validate" => {
            Ok(CommandInvocation {
                route: CommandRoute::Contracts,
                arguments: remaining,
            })
        }
        [command, action, remaining @ ..] if command == "evidence" && action == "verify" => {
            Ok(CommandInvocation {
                route: CommandRoute::Evidence,
                arguments: remaining,
            })
        }
        [command, action, remaining @ ..]
            if command == "external-contracts" && action == "verify" =>
        {
            Ok(CommandInvocation {
                route: CommandRoute::ExternalContracts,
                arguments: remaining,
            })
        }
        [command, action, remaining @ ..] if command == "baseline" && action == "validate" => {
            Ok(CommandInvocation {
                route: CommandRoute::Baseline,
                arguments: remaining,
            })
        }
        [command, action, remaining @ ..] if command == "measurement" && action == "validate" => {
            Ok(CommandInvocation {
                route: CommandRoute::Measurement,
                arguments: remaining,
            })
        }
        [command, action, remaining @ ..] if command == "environment" && action == "inspect" => {
            Ok(CommandInvocation {
                route: CommandRoute::Environment,
                arguments: remaining,
            })
        }
        [command, action, remaining @ ..] if command == "lock" && action == "status" => {
            Ok(CommandInvocation {
                route: CommandRoute::Lock,
                arguments: remaining,
            })
        }
        [command, action, remaining @ ..] if command == "candidate" && action == "build" => {
            Ok(CommandInvocation {
                route: CommandRoute::Candidate,
                arguments: remaining,
            })
        }
        [command, remaining @ ..] if command == "probe" => Ok(CommandInvocation {
            route: CommandRoute::Probe,
            arguments: remaining,
        }),
        [command, remaining @ ..] if command == "qualify" => Ok(CommandInvocation {
            route: CommandRoute::Qualify,
            arguments: remaining,
        }),
        _ => Err(CommandError::InvalidCommand),
    }
}

/// Routes a recognized command and its arguments to the owning command module.
fn execute(invocation: CommandInvocation<'_>) -> CommandOutcome {
    match invocation.route {
        CommandRoute::Contracts => commands::contracts::run(invocation.arguments),
        CommandRoute::Evidence => commands::evidence::run(invocation.arguments),
        CommandRoute::ExternalContracts => commands::external_contracts::run(invocation.arguments),
        CommandRoute::Baseline => commands::baseline::run(invocation.arguments),
        CommandRoute::Measurement => commands::measurement::run(invocation.arguments),
        CommandRoute::Environment => commands::environment::run(invocation.arguments),
        CommandRoute::Lock => commands::lock::run(invocation.arguments),
        CommandRoute::Candidate => commands::candidate::run(invocation.arguments),
        CommandRoute::Probe => commands::probe::run(invocation.arguments),
        CommandRoute::Qualify => commands::qualify::run(invocation.arguments),
    }
}

/// Describes an exit outcome returned by a qualification command.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum CommandOutcome {
    /// The command completed successfully.
    #[allow(
        dead_code,
        reason = "Later ticket-owned command modules return successful outcomes."
    )]
    Success,
    /// The command failed validation or execution.
    Failed(CommandError),
    /// The lock was valid but the requested readiness gate remains open.
    #[allow(
        dead_code,
        reason = "The OXY-A004 lock command owns this open-gate outcome."
    )]
    ValidButOpen,
}

impl CommandOutcome {
    /// Creates a failed outcome from a command error.
    pub(crate) const fn failed(error: CommandError) -> Self {
        Self::Failed(error)
    }

    /// Creates a named unimplemented-command failure.
    pub(crate) const fn not_implemented(command: &'static str) -> Self {
        Self::failed(CommandError::NotImplemented { command })
    }

    /// Returns the process exit code required by the qualification command contract.
    fn exit_code(&self) -> ExitCode {
        match self {
            Self::Success => ExitCode::SUCCESS,
            Self::Failed(error) => error.exit_code(),
            Self::ValidButOpen => ExitCode::from(2),
        }
    }

    /// Returns a content-free failure diagnostic when one is available.
    const fn diagnostic(&self) -> Option<&CommandError> {
        match self {
            Self::Success => None,
            Self::Failed(error) => Some(error),
            Self::ValidButOpen => None,
        }
    }
}

/// Classifies invalid, validation, and execution failures from qualification commands.
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub(crate) enum CommandError {
    /// The command name doesn't match the qualification command contract.
    #[error("invalid command")]
    InvalidCommand,
    /// The command has a registered placeholder but no implementation yet.
    #[error("not implemented: {command}")]
    NotImplemented {
        /// The content-free command name.
        command: &'static str,
    },
    /// Command arguments or their referenced input were invalid.
    #[error("invalid input")]
    #[allow(
        dead_code,
        reason = "Later command modules use this category for invalid command-specific inputs."
    )]
    InvalidInput(String),
    /// Command validation completed but its inputs didn't meet the required contract.
    #[error("validation failed")]
    #[allow(
        dead_code,
        reason = "Later command modules use this category for validation failures."
    )]
    ValidationFailed(String),
    /// The command couldn't complete its local execution.
    #[error("execution failed")]
    #[allow(
        dead_code,
        reason = "Later command modules use this category for local execution failures."
    )]
    Execution(String),
}

impl CommandError {
    /// Returns the failure exit code required by the qualification command contract.
    fn exit_code(&self) -> ExitCode {
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::{Command, ExitCode};

    use super::{CommandError, CommandOutcome, CommandRoute, dispatch};

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

    const BEHAVIOR_FREE_CRATES: &[&str] = &[
        "crates/oxyflut-substrate-impeller",
        "crates/oxyflut-substrate-engine",
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
    ];

    const STACK_ALLOWED_DEPENDENCIES: &[(&str, &str)] = &[
        ("oxyflut-qualification", "0.1.0"),
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
        for crate_path in BEHAVIOR_FREE_CRATES {
            let source = read_file(&root.join(crate_path).join("src/lib.rs"))?;
            assert!(
                is_documentation_only(&source),
                "{crate_path} must not contain product, candidate, or platform behavior"
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
    fn dispatcher_recognizes_every_qualification_command_and_preserves_its_arguments() {
        let cases = [
            (&["contracts", "validate"][..], CommandRoute::Contracts, 2),
            (
                &["evidence", "verify", "PATH"][..],
                CommandRoute::Evidence,
                2,
            ),
            (
                &["external-contracts", "verify"][..],
                CommandRoute::ExternalContracts,
                2,
            ),
            (
                &["baseline", "validate", "--input", "PATH"][..],
                CommandRoute::Baseline,
                2,
            ),
            (
                &["measurement", "validate", "--input", "PATH"][..],
                CommandRoute::Measurement,
                2,
            ),
            (
                &[
                    "environment",
                    "inspect",
                    "--environment",
                    "ENVIRONMENT",
                    "--output",
                    "PATH",
                ][..],
                CommandRoute::Environment,
                2,
            ),
            (
                &["lock", "status", "--gate", "candidate-implementation"][..],
                CommandRoute::Lock,
                2,
            ),
            (
                &["candidate", "build", "--candidate", "focused", "--locked"][..],
                CommandRoute::Candidate,
                2,
            ),
            (
                &[
                    "candidate",
                    "build",
                    "--candidate",
                    "integrated",
                    "--locked",
                    "--dart-disabled",
                ][..],
                CommandRoute::Candidate,
                2,
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
                1,
            ),
            (
                &["qualify", "--all-candidates", "--locked"][..],
                CommandRoute::Qualify,
                1,
            ),
        ];

        for (arguments, route, argument_start) in cases {
            let arguments = arguments
                .iter()
                .map(|value| (*value).to_owned())
                .collect::<Vec<_>>();
            assert_eq!(
                dispatch(&arguments).map(|invocation| invocation.route),
                Ok(route)
            );
            assert_eq!(
                dispatch(&arguments).map(|invocation| invocation.arguments),
                Ok(&arguments[argument_start..])
            );
        }

        assert_eq!(dispatch(&[]), Err(CommandError::InvalidCommand));
        assert_eq!(
            dispatch(&["unknown".to_owned()]),
            Err(CommandError::InvalidCommand)
        );
    }

    #[test]
    fn command_outcomes_follow_the_qualification_exit_code_contract() {
        assert_eq!(CommandOutcome::Success.exit_code(), ExitCode::SUCCESS);
        assert_eq!(CommandOutcome::ValidButOpen.exit_code(), ExitCode::from(2));
        assert_eq!(
            CommandOutcome::not_implemented("placeholder").exit_code(),
            ExitCode::FAILURE
        );
    }

    #[test]
    fn runtime_errors_have_content_free_diagnostics_and_failure_exit_codes() {
        let cases = [
            (
                CommandError::InvalidInput("input context".to_owned()),
                "invalid input",
            ),
            (
                CommandError::ValidationFailed("validation context".to_owned()),
                "validation failed",
            ),
            (
                CommandError::Execution("execution context".to_owned()),
                "execution failed",
            ),
        ];

        for (error, diagnostic) in cases {
            assert_eq!(error.exit_code(), ExitCode::FAILURE);
            assert_eq!(error.to_string(), diagnostic);
        }
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
