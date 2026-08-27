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
const BINDINGS_PATH: &str = "qualification/fixtures/generated-bindings/oxyflut-substrate.rs";
const BINDINGS_DIGEST_PATH: &str =
    "qualification/fixtures/generated-bindings/oxyflut-substrate.rs.sha256";
const INTERFACE_FIXTURE_PATH: &str = "qualification/fixtures/native/interface.json";
const LAYOUT_PROBE_PATH: &str = "qualification/fixtures/native/layout-probe.c.in";
const MACROS_FIXTURE_DIRECTORY: &str = "qualification/fixtures/native";
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
    validate_interface(root, header, tools)?;
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

fn validate_interface(
    root: &Path,
    header: &Path,
    tools: &NativeTools,
) -> Result<(), NativeContractError> {
    let fixture = read_json(&root.join(INTERFACE_FIXTURE_PATH), "native interface")?;
    let source = fs::read_to_string(header)?;
    validate_macro_expansions(root, header, tools)?;
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

fn validate_macro_expansions(
    root: &Path,
    header: &Path,
    tools: &NativeTools,
) -> Result<(), NativeContractError> {
    let fixture = read_json(
        &root.join(format!(
            "{MACROS_FIXTURE_DIRECTORY}/macros.{}.json",
            tools.host_triple
        )),
        "native macro expansions",
    )?;
    let target = required_string(&fixture, "targetTriple", "native macro expansions")?;
    if target != tools.host_triple {
        return Err(NativeContractError::MacroFixtureTargetMismatch {
            expected: tools.host_triple.clone(),
            actual: target,
        });
    }
    let expected = required_object(&fixture, "definitions", "native macro expansions")?;
    let actual = preprocessed_macro_definitions(header, tools)?;
    for name in ["OXY_EXPORT", "OXY_CALL"] {
        let expected_expansion = expected
            .get(name)
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or(NativeContractError::InvalidFixture {
                fixture: "native macro expansions",
            })?;
        let actual_expansion =
            actual
                .get(name)
                .ok_or_else(|| NativeContractError::MacroExpansionMismatch {
                    name: name.to_owned(),
                    expected: expected_expansion.clone(),
                    actual: None,
                })?;
        if actual_expansion != &expected_expansion {
            return Err(NativeContractError::MacroExpansionMismatch {
                name: name.to_owned(),
                expected: expected_expansion,
                actual: Some(actual_expansion.clone()),
            });
        }
    }
    Ok(())
}

fn preprocessed_macro_definitions(
    header: &Path,
    tools: &NativeTools,
) -> Result<std::collections::BTreeMap<String, String>, NativeContractError> {
    let output = run_tool_output(
        &tools.c_header_checker,
        "c-header-checker",
        "macro expansion preprocessing",
        [
            OsStr::new("-Qunused-arguments"),
            OsStr::new("-dM"),
            OsStr::new("-E"),
            OsStr::new("-x"),
            OsStr::new("c"),
            OsStr::new("-std=c11"),
            header.as_os_str(),
        ],
    )?;
    let definitions = String::from_utf8(output.stdout).map_err(|source| {
        NativeContractError::ToolOutputEncoding {
            tool: "c-header-checker",
            operation: "macro expansion preprocessing",
            source,
        }
    })?;
    let mut macros = std::collections::BTreeMap::new();
    for line in definitions.lines() {
        let Some(definition) = line.strip_prefix("#define ") else {
            continue;
        };
        let mut parts = definition.splitn(2, char::is_whitespace);
        let Some(name) = parts.next() else {
            continue;
        };
        let expansion = parts.next().map(str::trim_start).unwrap_or_default();
        macros.insert(name.to_owned(), expansion.to_owned());
    }
    Ok(macros)
}

fn derive_header_nullability(
    header: &Path,
    tools: &NativeTools,
    actual: &mut Value,
) -> Result<(), NativeContractError> {
    let source = fs::read_to_string(header)?;
    let records = parse_header_records(header, tools)?;
    if !source.contains("Unless a field comment states otherwise, every pointer passed to or returned from this ABI is nonnull. An array pointer is null if and only if its count is zero.") {
        return Err(NativeContractError::MissingNullabilityAnnotation {
            annotation: "default pointer and array rule",
        });
    }

    let actual_structs = actual
        .get_mut("structs")
        .and_then(Value::as_array_mut)
        .ok_or(NativeContractError::InvalidFixture {
            fixture: "native layout probe",
        })?;
    let mut probe_fields = std::collections::BTreeMap::new();
    for layout in actual_structs {
        let layout_object = layout
            .as_object_mut()
            .ok_or(NativeContractError::InvalidFixture {
                fixture: "native layout probe",
            })?;
        let name = layout_object
            .get("name")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or(NativeContractError::InvalidFixture {
                fixture: "native layout probe",
            })?;
        let fields = layout_object
            .get_mut("fields")
            .and_then(Value::as_array_mut)
            .ok_or(NativeContractError::InvalidFixture {
                fixture: "native layout probe",
            })?;
        let record = records
            .get(&name)
            .ok_or_else(|| NativeContractError::HeaderAstCoverageMismatch { item: name.clone() })?;
        let mut names = std::collections::BTreeSet::new();
        for layout_field in fields {
            let field_object =
                layout_field
                    .as_object_mut()
                    .ok_or(NativeContractError::InvalidFixture {
                        fixture: "native layout probe",
                    })?;
            let field_name = field_object
                .get("name")
                .and_then(Value::as_str)
                .map(str::to_owned)
                .ok_or(NativeContractError::InvalidFixture {
                    fixture: "native layout probe",
                })?;
            let field = record
                .iter()
                .find(|field| field.name == field_name)
                .ok_or_else(|| NativeContractError::HeaderAstCoverageMismatch {
                    item: format!("{name}.{field_name}"),
                })?;
            names.insert(field_name);
            field_object.insert(
                "nullability".to_owned(),
                Value::String(header_field_nullability(&source, &name, field, record)?.to_owned()),
            );
        }
        probe_fields.insert(name, names);
    }

    let header_fields = records
        .iter()
        .filter(|(_, fields)| !fields.is_empty())
        .map(|(name, fields)| {
            (
                name.clone(),
                fields.iter().map(|field| field.name.clone()).collect(),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    if probe_fields != header_fields {
        return Err(NativeContractError::HeaderAstCoverageMismatch {
            item: "layout probe fields".to_owned(),
        });
    }
    Ok(())
}

fn header_field_nullability(
    source: &str,
    record_name: &str,
    field: &HeaderField,
    record: &[HeaderField],
) -> Result<&'static str, NativeContractError> {
    if !field.is_pointer {
        return Ok("not-pointer");
    }
    if field.comment.contains("Nullable;") {
        return Ok("nullable");
    }
    if field.comment.contains("Nonnull;") {
        return Ok("nonnull");
    }
    if record_name == "OxyBorrowedBytes" && field.name == "data" {
        if source.contains("data is null if and only if length is zero.") {
            return Ok("null-if-length-zero");
        }
        return Err(NativeContractError::MissingNullabilityAnnotation {
            annotation: "OxyBorrowedBytes.data length rule",
        });
    }
    if record_name == "OxyOwnedBytes" && matches!(field.name.as_str(), "data" | "release") {
        if source.contains("Empty values have null data, zero length, and null release.") {
            return Ok("null-if-length-zero");
        }
        return Err(NativeContractError::MissingNullabilityAnnotation {
            annotation: "OxyOwnedBytes empty-value rule",
        });
    }
    if record.iter().any(|candidate| {
        candidate
            .name
            .strip_suffix("_count")
            .is_some_and(|prefix| field.name.starts_with(prefix))
    }) {
        return Ok("null-if-count-zero");
    }
    Ok("nonnull")
}

fn parse_header_records(
    header: &Path,
    tools: &NativeTools,
) -> Result<std::collections::BTreeMap<String, Vec<HeaderField>>, NativeContractError> {
    let output = run_tool_output(
        &tools.c_header_checker,
        "c-header-checker",
        "nullability AST inspection",
        [
            OsStr::new("-Qunused-arguments"),
            OsStr::new("-Xclang"),
            OsStr::new("-ast-dump=json"),
            OsStr::new("-fparse-all-comments"),
            OsStr::new("-fsyntax-only"),
            OsStr::new("-x"),
            OsStr::new("c"),
            OsStr::new("-std=c11"),
            header.as_os_str(),
        ],
    )?;
    let ast: Value = serde_json::from_slice(&output.stdout)?;
    let mut records = std::collections::BTreeMap::new();
    collect_header_records(&ast, &mut records);
    Ok(records)
}

fn collect_header_records(
    value: &Value,
    records: &mut std::collections::BTreeMap<String, Vec<HeaderField>>,
) {
    if value.get("kind").and_then(Value::as_str) == Some("RecordDecl")
        && value.get("completeDefinition").and_then(Value::as_bool) == Some(true)
        && let Some(name) = value.get("name").and_then(Value::as_str)
        && name.starts_with("Oxy")
    {
        let fields = value
            .get("inner")
            .and_then(Value::as_array)
            .map(|children| {
                children
                    .iter()
                    .filter(|child| child.get("kind").and_then(Value::as_str) == Some("FieldDecl"))
                    .filter_map(header_field)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        records.insert(name.to_owned(), fields);
    }
    if let Some(children) = value.get("inner").and_then(Value::as_array) {
        for child in children {
            collect_header_records(child, records);
        }
    }
}

fn header_field(value: &Value) -> Option<HeaderField> {
    let name = value.get("name")?.as_str()?.to_owned();
    let type_name = value
        .get("type")
        .and_then(Value::as_object)
        .and_then(|value| value.get("qualType"))
        .and_then(Value::as_str)?;
    Some(HeaderField {
        name,
        is_pointer: type_name.contains('*'),
        comment: comment_text(value),
    })
}

fn comment_text(value: &Value) -> String {
    let mut text = String::new();
    collect_comment_text(value, &mut text);
    text
}

fn collect_comment_text(value: &Value, text: &mut String) {
    if let Some(comment) = value.get("text").and_then(Value::as_str) {
        text.push_str(comment);
    }
    if let Some(children) = value.get("inner").and_then(Value::as_array) {
        for child in children {
            collect_comment_text(child, text);
        }
    }
}

struct HeaderField {
    name: String,
    is_pointer: bool,
    comment: String,
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
    let mut actual: Value = serde_json::from_slice(&output.stdout)?;
    derive_header_nullability(header, tools, &mut actual)?;
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

fn run_tool_output<'arguments, I>(
    tool: &Path,
    name: &'static str,
    operation: &'static str,
    arguments: I,
) -> Result<std::process::Output, NativeContractError>
where
    I: IntoIterator<Item = &'arguments OsStr>,
{
    let output = Command::new(tool)
        .args(arguments)
        .output()
        .map_err(|source| NativeContractError::ToolExecution {
            tool: name,
            operation,
            source,
        })?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(NativeContractError::ToolFailed {
            tool: name,
            operation,
        })
    }
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
    #[error("native contract tool emitted non-UTF-8 output: {tool} during {operation}")]
    ToolOutputEncoding {
        tool: &'static str,
        operation: &'static str,
        #[source]
        source: std::string::FromUtf8Error,
    },
    #[error("the native macro fixture targets {actual}, not {expected}")]
    MacroFixtureTargetMismatch { expected: String, actual: String },
    #[error(
        "native macro {name} expands differently from its fixture: expected {expected:?}, got {actual:?}"
    )]
    MacroExpansionMismatch {
        name: String,
        expected: String,
        actual: Option<String>,
    },
    #[error("the header is missing a documented nullability annotation: {annotation}")]
    MissingNullabilityAnnotation { annotation: &'static str },
    #[error("the header AST and layout probe disagree about {item}")]
    HeaderAstCoverageMismatch { item: String },
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
#[path = "native_tests.rs"]
mod tests;
