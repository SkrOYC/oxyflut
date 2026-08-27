//! Native C ABI qualification without candidate compilation or linking.

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use oxyflut_qualification::hash::{Sha256Digest, hash_file};
use serde_json::Value;
use thiserror::Error;

use crate::toolchain::{self, ToolchainManifest};

/// The stable name reported by the contract-validation command.
pub(crate) const FAMILY: &str = "native";

const MANIFEST_PATH: &str = "qualification/tools/native-contract-toolchain.json";
const HEADER_PATH: &str = ".constitution/tech-spec/contracts/oxyflut-substrate.h";
#[cfg(test)]
const COMMON_CONTRACT_PATH: &str = ".constitution/tech-spec/contracts/oxyflut-substrate.rs";
const BINDINGS_PATH: &str = "qualification/fixtures/generated-bindings/oxyflut-substrate.rs";
const BINDINGS_DIGEST_PATH: &str =
    "qualification/fixtures/generated-bindings/oxyflut-substrate.rs.sha256";
const INTERFACE_FIXTURE_PATH: &str = "qualification/fixtures/native/interface.json";
const LAYOUT_PROBE_PATH: &str = "qualification/fixtures/native/layout-probe.c.in";
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Validates the authoritative integrated C ABI using only staged native tools.
///
/// The layout probe is a generated and linked standalone host executable. It neither imports nor
/// links candidate code; linking is necessary only to obtain the compiler's actual host layout.
///
/// # Errors
///
/// Returns an error before header validation when the staged manifest is missing, incomplete, or
/// mismatched. Returns an error when syntax, generated declarations, ABI metadata, or host layout
/// differs from its committed qualification fixture.
pub(crate) fn validate_workspace(root: &Path) -> Result<(), NativeContractError> {
    let tools = NativeTools::load(root)?;
    validate_header(root, &root.join(HEADER_PATH), &tools)
}

fn validate_header(
    root: &Path,
    header: &Path,
    tools: &NativeTools,
) -> Result<(), NativeContractError> {
    let temporary = TemporaryDirectory::new("native-contract")?;
    syntax_check(header, tools)?;

    let bindings = temporary.path().join("oxyflut-substrate.rs");
    generate_bindings(header, &bindings, tools)?;
    validate_bindings(root, &bindings)?;
    validate_interface(root, header)?;
    validate_layout(root, header, tools, &temporary)?;
    Ok(())
}

fn syntax_check(header: &Path, tools: &NativeTools) -> Result<(), NativeContractError> {
    // The Nix compiler wrapper supplies linker-only arguments even for -fsyntax-only. Suppress
    // only that wrapper diagnostic; source warnings remain enabled and errors.
    run_tool(
        &tools.c_header_checker,
        "c-header-checker",
        "C11 syntax check",
        [
            OsStr::new("-Qunused-arguments"),
            OsStr::new("-x"),
            OsStr::new("c"),
            OsStr::new("-std=c11"),
            OsStr::new("-fsyntax-only"),
            OsStr::new("-Wall"),
            OsStr::new("-Wextra"),
            OsStr::new("-Werror"),
            OsStr::new("-pedantic"),
            header.as_os_str(),
        ],
    )?;
    run_tool(
        &tools.cxx_compiler,
        "cxx-compiler",
        "C++17 syntax check",
        [
            OsStr::new("-Qunused-arguments"),
            OsStr::new("-x"),
            OsStr::new("c++"),
            OsStr::new("-std=c++17"),
            OsStr::new("-fsyntax-only"),
            OsStr::new("-Wall"),
            OsStr::new("-Wextra"),
            OsStr::new("-Werror"),
            OsStr::new("-pedantic"),
            header.as_os_str(),
        ],
    )
}

fn generate_bindings(
    header: &Path,
    output: &Path,
    tools: &NativeTools,
) -> Result<(), NativeContractError> {
    run_tool(
        &tools.bindgen,
        "bindgen",
        "binding generation",
        [
            header.as_os_str(),
            OsStr::new("--rust-target"),
            OsStr::new("1.98"),
            OsStr::new("--rust-edition"),
            OsStr::new("2024"),
            OsStr::new("--no-layout-tests"),
            OsStr::new("--no-doc-comments"),
            OsStr::new("--disable-header-comment"),
            OsStr::new("--use-core"),
            OsStr::new("--ctypes-prefix"),
            OsStr::new("core::ffi"),
            OsStr::new("--allowlist-type"),
            OsStr::new("^Oxy.*"),
            OsStr::new("--allowlist-var"),
            OsStr::new("^OXY_.*"),
            OsStr::new("--allowlist-function"),
            OsStr::new("^OxySubstrateGetApi$"),
            OsStr::new("-o"),
            output.as_os_str(),
            OsStr::new("--"),
            OsStr::new("-x"),
            OsStr::new("c"),
            OsStr::new("-std=c11"),
        ],
    )
}

fn validate_bindings(root: &Path, generated: &Path) -> Result<(), NativeContractError> {
    let golden = root.join(BINDINGS_PATH);
    let golden_digest = root.join(BINDINGS_DIGEST_PATH);
    let expected_digest = fs::read_to_string(golden_digest)?.trim().to_owned();
    if expected_digest.parse::<Sha256Digest>().is_err() {
        return Err(NativeContractError::InvalidFixture {
            fixture: "generated binding SHA-256",
        });
    }
    if hash_file(&golden)?.to_string() != expected_digest {
        return Err(NativeContractError::GoldenDigestMismatch);
    }
    if hash_file(generated)?.to_string() != expected_digest {
        return Err(NativeContractError::GeneratedBindingsMismatch);
    }
    if fs::read(generated)? != fs::read(golden)? {
        return Err(NativeContractError::GeneratedBindingsMismatch);
    }
    Ok(())
}

fn validate_interface(root: &Path, header: &Path) -> Result<(), NativeContractError> {
    let fixture = read_json(&root.join(INTERFACE_FIXTURE_PATH), "native interface")?;
    let source = fs::read_to_string(header)?;
    let compact = source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>();

    let abi_version = required_u64(&fixture, "abiVersion", "native interface")?;
    if !compact.contains(&format!("#defineOXY_SUBSTRATE_ABI_VERSION{abi_version}u")) {
        return Err(NativeContractError::InterfaceMismatch {
            item: "ABI version".to_owned(),
        });
    }
    let symbol = required_string(&fixture, "acquisitionSymbol", "native interface")?;
    let calling_convention = required_object(&fixture, "callingConvention", "native interface")?;
    let export = required_object_string(calling_convention, "exportMacro", "native interface")?;
    let call = required_object_string(calling_convention, "callMacro", "native interface")?;
    for macro_name in [&export, &call] {
        if !source.contains(&format!("#define {macro_name}")) {
            return Err(NativeContractError::InterfaceMismatch {
                item: macro_name.to_owned(),
            });
        }
    }
    if !compact.contains(&format!("{export}OxyStatus{call}{symbol}(")) {
        return Err(NativeContractError::InterfaceMismatch {
            item: symbol.to_owned(),
        });
    }

    for handle in required_string_array(&fixture, "opaqueHandles", "native interface")? {
        let declaration = format!("typedefstruct{handle}{handle};");
        if !compact.contains(&declaration) {
            return Err(NativeContractError::InterfaceMismatch { item: handle });
        }
    }
    for struct_name in required_string_array(&fixture, "extensibleStructs", "native interface")? {
        let prefix =
            format!("typedefstruct{struct_name}{{uint32_tstruct_size;uint32_tabi_version;");
        if !compact.contains(&prefix) {
            return Err(NativeContractError::InterfaceMismatch { item: struct_name });
        }
    }
    for signature in required_string_array(&fixture, "callbackSignatures", "native interface")? {
        if !compact.contains(&signature) {
            return Err(NativeContractError::InterfaceMismatch { item: signature });
        }
    }
    let pointer_rule = required_string(&fixture, "pointerRule", "native interface")?;
    if !source.contains(&pointer_rule) {
        return Err(NativeContractError::InterfaceMismatch {
            item: "pointer nullability rule".to_owned(),
        });
    }

    let api_start = compact
        .find("typedefstructOxySubstrateApi{")
        .ok_or_else(|| NativeContractError::InterfaceMismatch {
            item: "OxySubstrateApi".to_owned(),
        })?;
    let api_end = compact[api_start..]
        .find("}OxySubstrateApi;")
        .map(|offset| api_start + offset)
        .ok_or_else(|| NativeContractError::InterfaceMismatch {
            item: "OxySubstrateApi terminator".to_owned(),
        })?;
    let api = &compact[api_start..api_end];
    if api
        .split(';')
        .any(|declaration| declaration.contains("(*") && !declaration.contains(&call))
    {
        return Err(NativeContractError::InterfaceMismatch {
            item: "OxySubstrateApi calling convention".to_owned(),
        });
    }
    let expected_api_functions = usize::try_from(required_u64(
        &fixture,
        "apiFunctionCount",
        "native interface",
    )?)
    .map_err(|_| NativeContractError::InvalidFixture {
        fixture: "native interface",
    })?;
    if api.matches(&format!("{call}*")).count() != expected_api_functions {
        return Err(NativeContractError::InterfaceMismatch {
            item: "OxySubstrateApi function table".to_owned(),
        });
    }
    Ok(())
}

fn validate_layout(
    root: &Path,
    header: &Path,
    tools: &NativeTools,
    temporary: &TemporaryDirectory,
) -> Result<(), NativeContractError> {
    let probe_source = temporary.path().join("layout-probe.c");
    let probe_binary = temporary.path().join("layout-probe");
    fs::write(&probe_source, fs::read(root.join(LAYOUT_PROBE_PATH))?)?;
    let include_directory = header
        .parent()
        .ok_or(NativeContractError::InvalidHeaderPath)?;
    let linker = format!("--ld-path={}", tools.linker.display());
    run_tool(
        &tools.c_header_checker,
        "c-header-checker",
        "layout probe compilation",
        [
            OsStr::new("-Qunused-arguments"),
            OsStr::new("-std=c11"),
            OsStr::new("-Wall"),
            OsStr::new("-Wextra"),
            OsStr::new("-Werror"),
            OsStr::new("-pedantic"),
            OsStr::new("-I"),
            include_directory.as_os_str(),
            OsStr::new(&linker),
            OsStr::new("-o"),
            probe_binary.as_os_str(),
            probe_source.as_os_str(),
        ],
    )?;
    let output = Command::new(&probe_binary).output().map_err(|source| {
        NativeContractError::ToolExecution {
            tool: "layout-probe",
            operation: "layout probe execution",
            source,
        }
    })?;
    if !output.status.success() {
        return Err(NativeContractError::ToolFailed {
            tool: "layout-probe",
            operation: "layout probe execution",
        });
    }
    let actual: Value = serde_json::from_slice(&output.stdout)?;
    let expected = read_json(
        &root.join(format!(
            "qualification/fixtures/native/layout.{}.json",
            tools.host_triple
        )),
        "native layout",
    )?;
    if actual != expected {
        return Err(NativeContractError::LayoutMismatch);
    }
    Ok(())
}

fn run_tool<'arguments, I>(
    tool: &Path,
    name: &'static str,
    operation: &'static str,
    arguments: I,
) -> Result<(), NativeContractError>
where
    I: IntoIterator<Item = &'arguments OsStr>,
{
    let status = Command::new(tool)
        .args(arguments)
        .status()
        .map_err(|source| NativeContractError::ToolExecution {
            tool: name,
            operation,
            source,
        })?;
    if status.success() {
        Ok(())
    } else {
        Err(NativeContractError::ToolFailed {
            tool: name,
            operation,
        })
    }
}

struct NativeTools {
    c_header_checker: PathBuf,
    cxx_compiler: PathBuf,
    bindgen: PathBuf,
    linker: PathBuf,
    host_triple: String,
}

impl NativeTools {
    fn load(root: &Path) -> Result<Self, NativeContractError> {
        let manifest_path = root.join(MANIFEST_PATH);
        let bytes = fs::read(&manifest_path).map_err(|source| {
            if source.kind() == io::ErrorKind::NotFound {
                NativeContractError::MissingToolchainManifest
            } else {
                NativeContractError::Io(source)
            }
        })?;
        let manifest =
            ToolchainManifest::from_json(&bytes).map_err(NativeContractError::Toolchain)?;
        toolchain::verify(&manifest).map_err(NativeContractError::Toolchain)?;
        let value: Value = serde_json::from_slice(&bytes)?;
        let (c_header_checker, host_triple) = manifest_tool(&value, "c-header-checker")?;
        Ok(Self {
            c_header_checker,
            cxx_compiler: manifest_tool(&value, "cxx-compiler")?.0,
            bindgen: manifest_tool(&value, "bindgen")?.0,
            linker: manifest_tool(&value, "linker")?.0,
            host_triple,
        })
    }
}

fn manifest_tool(value: &Value, name: &str) -> Result<(PathBuf, String), NativeContractError> {
    let tools = value.get("resolvedTools").and_then(Value::as_array).ok_or(
        NativeContractError::InvalidFixture {
            fixture: "staged native toolchain",
        },
    )?;
    let tool = tools
        .iter()
        .find(|tool| tool.get("name").and_then(Value::as_str) == Some(name))
        .ok_or_else(|| NativeContractError::MissingManifestTool {
            name: name.to_owned(),
        })?;
    if tool.get("pathRoot").is_some() {
        return Err(NativeContractError::ManifestToolPath {
            name: name.to_owned(),
        });
    }
    let executable = tool
        .get("executablePath")
        .and_then(Value::as_str)
        .filter(|path| !path.is_empty())
        .ok_or_else(|| NativeContractError::ManifestToolPath {
            name: name.to_owned(),
        })?;
    let path = PathBuf::from(executable);
    if !path.is_absolute() || !path.is_file() {
        return Err(NativeContractError::ManifestToolPath {
            name: name.to_owned(),
        });
    }
    let host_triple = tool
        .get("hostTriple")
        .and_then(Value::as_str)
        .filter(|triple| !triple.is_empty())
        .map(str::to_owned)
        .ok_or(NativeContractError::InvalidFixture {
            fixture: "staged native toolchain host triple",
        })?;
    Ok((path, host_triple))
}

fn read_json(path: &Path, fixture: &'static str) -> Result<Value, NativeContractError> {
    let value: Value = serde_json::from_slice(&fs::read(path)?)?;
    if value.is_object() {
        Ok(value)
    } else {
        Err(NativeContractError::InvalidFixture { fixture })
    }
}

fn required_u64(
    value: &Value,
    key: &str,
    fixture: &'static str,
) -> Result<u64, NativeContractError> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .ok_or(NativeContractError::InvalidFixture { fixture })
}

fn required_string(
    value: &Value,
    key: &str,
    fixture: &'static str,
) -> Result<String, NativeContractError> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|item| !item.is_empty())
        .map(str::to_owned)
        .ok_or(NativeContractError::InvalidFixture { fixture })
}

fn required_object<'value>(
    value: &'value Value,
    key: &str,
    fixture: &'static str,
) -> Result<&'value serde_json::Map<String, Value>, NativeContractError> {
    value
        .get(key)
        .and_then(Value::as_object)
        .ok_or(NativeContractError::InvalidFixture { fixture })
}

fn required_object_string(
    object: &serde_json::Map<String, Value>,
    key: &str,
    fixture: &'static str,
) -> Result<String, NativeContractError> {
    object
        .get(key)
        .and_then(Value::as_str)
        .filter(|item| !item.is_empty())
        .map(str::to_owned)
        .ok_or(NativeContractError::InvalidFixture { fixture })
}

fn required_string_array(
    value: &Value,
    key: &str,
    fixture: &'static str,
) -> Result<Vec<String>, NativeContractError> {
    value
        .get(key)
        .and_then(Value::as_array)
        .filter(|items| !items.is_empty())
        .map(|items| {
            items
                .iter()
                .map(|item| {
                    item.as_str()
                        .filter(|item| !item.is_empty())
                        .map(str::to_owned)
                        .ok_or(NativeContractError::InvalidFixture { fixture })
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .ok_or(NativeContractError::InvalidFixture { fixture })?
}

struct TemporaryDirectory {
    path: PathBuf,
}

impl TemporaryDirectory {
    fn new(prefix: &str) -> Result<Self, NativeContractError> {
        let temporary_root = std::env::temp_dir();
        for _ in 0..128 {
            let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = temporary_root.join(format!(
                "oxyflut-{prefix}-{}-{sequence}",
                std::process::id()
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self { path }),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(NativeContractError::Io(error)),
            }
        }
        Err(NativeContractError::TemporaryDirectory)
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// Reports deterministic native-contract validation failures.
#[derive(Debug, Error)]
pub(crate) enum NativeContractError {
    #[error("the staged native toolchain manifest is missing")]
    MissingToolchainManifest,
    #[error("the staged native toolchain manifest failed verification")]
    Toolchain(#[source] toolchain::ToolchainError),
    #[error("the staged native toolchain is missing {name}")]
    MissingManifestTool { name: String },
    #[error("the staged native toolchain has an invalid executable path for {name}")]
    ManifestToolPath { name: String },
    #[error("native contract I/O failed")]
    Io(#[from] io::Error),
    #[error("native contract JSON failed")]
    Json(#[from] serde_json::Error),
    #[error("native contract fixture is invalid: {fixture}")]
    InvalidFixture { fixture: &'static str },
    #[error("native contract header path is invalid")]
    InvalidHeaderPath,
    #[error("native contract tool execution failed: {tool} during {operation}")]
    ToolExecution {
        tool: &'static str,
        operation: &'static str,
        #[source]
        source: io::Error,
    },
    #[error("native contract tool failed: {tool} during {operation}")]
    ToolFailed {
        tool: &'static str,
        operation: &'static str,
    },
    #[error("the committed generated-binding SHA-256 doesn't match its bytes")]
    GoldenDigestMismatch,
    #[error("generated native bindings differ from the committed golden")]
    GeneratedBindingsMismatch,
    #[error("native ABI metadata differs: {item}")]
    InterfaceMismatch { item: String },
    #[error("native ABI layout differs from the host fixture")]
    LayoutMismatch,
    #[error("could not create a temporary native-contract directory")]
    TemporaryDirectory,
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::path::{Path, PathBuf};

    use serde_json::Value;

    use super::{
        BINDINGS_PATH, NativeContractError, NativeTools, TemporaryDirectory, generate_bindings,
        validate_header, validate_workspace,
    };

    #[test]
    fn authoritative_header_passes_strict_c11_cpp17_interface_and_layout_checks()
    -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        validate_workspace(&root)?;
        Ok(())
    }

    #[test]
    fn generated_bindings_are_byte_stable_under_the_locked_toolchain() -> Result<(), Box<dyn Error>>
    {
        let root = workspace_root()?;
        let tools = NativeTools::load(&root)?;
        let temporary = TemporaryDirectory::new("native-bindings")?;
        let first = temporary.path().join("first.rs");
        let second = temporary.path().join("second.rs");
        let header = root.join(super::HEADER_PATH);
        generate_bindings(&header, &first, &tools)?;
        generate_bindings(&header, &second, &tools)?;
        assert_eq!(fs::read(&first)?, fs::read(&second)?);
        assert_eq!(fs::read(&first)?, fs::read(root.join(BINDINGS_PATH))?);
        Ok(())
    }

    #[test]
    fn native_toolchain_failure_stops_before_header_validation() -> Result<(), Box<dyn Error>> {
        let temporary = TemporaryDirectory::new("native-manifest")?;
        let result = NativeTools::load(temporary.path());
        assert!(matches!(
            result,
            Err(NativeContractError::MissingToolchainManifest)
        ));
        Ok(())
    }

    #[test]
    fn deliberate_layout_type_and_symbol_mutations_fail() -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let tools = NativeTools::load(&root)?;
        let source = fs::read_to_string(root.join(super::HEADER_PATH))?;
        let mutations = [
            (
                "reordered prefix",
                "  uint32_t struct_size;\n  uint32_t abi_version;",
                "  uint32_t abi_version;\n  uint32_t struct_size;",
            ),
            (
                "changed type",
                "  uint64_t view_generation;\n  uint64_t display_epoch;",
                "  uint32_t view_generation;\n  uint64_t display_epoch;",
            ),
            (
                "renamed symbol",
                "OxySubstrateGetApi",
                "OxySubstrateGetApiMutated",
            ),
        ];
        for (name, original, replacement) in mutations {
            let temporary = TemporaryDirectory::new("native-mutation")?;
            let header = temporary.path().join("oxyflut-substrate.h");
            let mutated = source.replacen(original, replacement, 1);
            assert_ne!(mutated, source, "{name} must alter the header fixture");
            fs::write(&header, mutated)?;
            assert!(
                validate_header(&root, &header, &tools).is_err(),
                "{name} must fail"
            );
        }
        Ok(())
    }

    #[test]
    fn native_index_units_match_rust_and_platform_contracts_before_range_conversion()
    -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let platform: Value = serde_json::from_slice(&fs::read(
            root.join(".constitution/tech-spec/data-models/platform-contracts.schema.json"),
        )?)?;
        let header = fs::read_to_string(root.join(super::HEADER_PATH))?;
        let common = fs::read_to_string(root.join(super::COMMON_CONTRACT_PATH))?;
        for item in [
            "pub enum NativeTextIndexUnit",
            "Utf8Bytes",
            "Utf16Units",
            "UnicodeScalars",
        ] {
            assert!(common.contains(item));
        }
        let expected = [
            (
                1,
                "OXY_NATIVE_TEXT_INDEX_UTF8_BYTES = 1u",
                NativeTextIndexUnit::Utf8Bytes,
                "utf8-bytes",
            ),
            (
                2,
                "OXY_NATIVE_TEXT_INDEX_UTF16_UNITS = 2u",
                NativeTextIndexUnit::Utf16Units,
                "utf16-code-units",
            ),
            (
                3,
                "OXY_NATIVE_TEXT_INDEX_UNICODE_SCALARS = 3u",
                NativeTextIndexUnit::UnicodeScalars,
                "unicode-scalars",
            ),
        ];
        for (raw, c_constant, unit, name) in expected {
            assert!(header.contains(c_constant));
            assert_eq!(NativeTextIndexUnit::try_from(raw)?, unit);
            assert_eq!(unit.platform_name(), name);
            assert!(platform_contract_defines(&platform, name));
        }
        assert!(matches!(
            convert_native_range(99, 9, 1),
            Err(MirrorError::UnknownNativeIndexUnit(99))
        ));
        Ok(())
    }

    #[test]
    fn presentation_statuses_map_and_reject_invalid_timestamps_before_delivery()
    -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let header = fs::read_to_string(root.join(super::HEADER_PATH))?;
        let common = fs::read_to_string(root.join(super::COMMON_CONTRACT_PATH))?;
        assert!(common.contains("pub enum PresentationStatus"));
        let expected = [
            (0, "OXY_STATUS_OK = 0u", PresentationStatus::Presented),
            (
                1,
                "OXY_STATUS_INVALID_ARGUMENT = 1u",
                PresentationStatus::InvalidArgument,
            ),
            (
                2,
                "OXY_STATUS_INCOMPATIBLE_ABI = 2u",
                PresentationStatus::IncompatibleAbi,
            ),
            (
                3,
                "OXY_STATUS_STALE_OWNER = 3u",
                PresentationStatus::StaleOwner,
            ),
            (
                4,
                "OXY_STATUS_RESOURCE_LIMIT = 4u",
                PresentationStatus::ResourceLimit,
            ),
            (
                5,
                "OXY_STATUS_UNSUPPORTED = 5u",
                PresentationStatus::Unsupported,
            ),
            (
                6,
                "OXY_STATUS_SUBSTRATE_FAILURE = 6u",
                PresentationStatus::SubstrateFailure,
            ),
            (
                7,
                "OXY_STATUS_CANCELLED = 7u",
                PresentationStatus::Cancelled,
            ),
            (
                8,
                "OXY_STATUS_DEADLINE_EXCEEDED = 8u",
                PresentationStatus::DeadlineExceeded,
            ),
        ];
        for (raw, c_status, expected_status) in expected {
            assert!(header.contains(c_status));
            assert!(common.contains(&format!("{expected_status:?}")));
            let timestamp = if raw == 0 { Some(42) } else { None };
            let mut recorder = PresentationRecorder::default();
            deliver_presentation(raw, timestamp, &mut recorder)?;
            assert_eq!(recorder.deliveries, vec![expected_status]);
            let invalid_timestamp = if raw == 0 { None } else { Some(42) };
            assert!(matches!(
                deliver_presentation(raw, invalid_timestamp, &mut recorder),
                Err(MirrorError::InvalidPresentationTimestamp)
            ));
        }
        let mut recorder = PresentationRecorder::default();
        assert!(matches!(
            deliver_presentation(99, Some(42), &mut recorder),
            Err(MirrorError::UnknownPresentationStatus(99))
        ));
        assert!(recorder.deliveries.is_empty());
        Ok(())
    }

    #[test]
    fn texture_realization_contracts_accept_and_reject_the_same_inputs()
    -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let common = fs::read_to_string(root.join(super::COMMON_CONTRACT_PATH))?;
        let header = fs::read_to_string(root.join(super::HEADER_PATH))?;
        assert!(common.contains("pub enum PixelFormat"));
        assert!(common.contains("Rgba8888"));
        assert!(common.contains("fn realize_texture"));
        assert!(header.contains("OXY_PIXEL_FORMAT_RGBA8888 = 1u"));
        let accepted = (2, 3, 1, 24);
        assert_eq!(
            validate_common_texture(
                accepted.0,
                accepted.1,
                CommonPixelFormat::Rgba8888,
                accepted.3
            ),
            Ok(24)
        );
        assert_eq!(
            validate_common_texture_from_native(accepted.0, accepted.1, accepted.2, accepted.3),
            Ok(24)
        );
        assert_eq!(
            validate_c_texture(accepted.0, accepted.1, accepted.2, accepted.3),
            Ok(24)
        );
        for (width, height, format, bytes) in [
            (0, 3, 1, 0),
            (2, 0, 1, 0),
            (2, 3, 99, 24),
            (2, 3, 1, 23),
            (u32::MAX, u32::MAX, 1, 0),
        ] {
            assert!(validate_common_texture_from_native(width, height, format, bytes).is_err());
            assert!(validate_c_texture(width, height, format, bytes).is_err());
        }
        for (width, height, bytes) in [(0, 3, 0), (2, 0, 0), (2, 3, 23), (u32::MAX, u32::MAX, 0)] {
            assert!(
                validate_common_texture(width, height, CommonPixelFormat::Rgba8888, bytes).is_err()
            );
        }
        Ok(())
    }

    #[test]
    fn semantics_selection_projects_some_and_none_and_rejects_invalid_c_forms()
    -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let common = fs::read_to_string(root.join(super::COMMON_CONTRACT_PATH))?;
        let header = fs::read_to_string(root.join(super::HEADER_PATH))?;
        assert!(common.contains("selection_utf16: Option<(u32, u32)>"));
        for field in [
            "text_selection_base_utf16",
            "text_selection_extent_utf16",
            "has_text_selection",
            "text_selection_reserved",
        ] {
            assert!(header.contains(field));
        }
        let some = CommonSelection::Some(4, 12);
        assert_eq!(c_to_common_selection(common_to_c_selection(some)?)?, some);
        let none = CommonSelection::None;
        assert_eq!(c_to_common_selection(common_to_c_selection(none)?)?, none);
        for c_selection in [
            CSelection {
                base: 1,
                extent: 0,
                has_selection: 0,
                reserved: 0,
            },
            CSelection {
                base: 0,
                extent: 0,
                has_selection: 2,
                reserved: 0,
            },
            CSelection {
                base: 0,
                extent: 0,
                has_selection: 1,
                reserved: 1,
            },
            CSelection {
                base: -1,
                extent: 0,
                has_selection: 1,
                reserved: 0,
            },
            CSelection {
                base: i64::from(u32::MAX) + 1,
                extent: 0,
                has_selection: 1,
                reserved: 0,
            },
        ] {
            assert!(c_to_common_selection(c_selection).is_err());
        }
        Ok(())
    }

    #[test]
    fn abi_seven_through_nine_fail_before_callbacks_install() -> Result<(), Box<dyn Error>> {
        let root = workspace_root()?;
        let common = fs::read_to_string(root.join(super::COMMON_CONTRACT_PATH))?;
        let header = fs::read_to_string(root.join(super::HEADER_PATH))?;
        assert!(common.contains("fn check_compatibility"));
        assert!(header.contains("#define OXY_SUBSTRATE_ABI_VERSION 10u"));
        for abi_version in 7..=9 {
            let mut callbacks_installed = false;
            assert!(matches!(
                negotiate_abi(abi_version, &mut callbacks_installed),
                Err(MirrorError::IncompatibleAbi(version)) if version == abi_version
            ));
            assert!(!callbacks_installed);
        }
        let mut callbacks_installed = false;
        negotiate_abi(10, &mut callbacks_installed)?;
        assert!(callbacks_installed);
        Ok(())
    }

    fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| "xtask must remain directly below the workspace root".into())
    }

    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../qualification/fixtures/native/contract-mirrors.rs"
    ));
}
