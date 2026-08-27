use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

#[allow(clippy::enum_variant_names, unreachable_pub)]
mod authoritative {
    include!(concat!(
        env!("OUT_DIR"),
        "/native-authoritative-contracts.rs"
    ));

    #[allow(
        dead_code,
        missing_docs,
        non_camel_case_types,
        unreachable_pub,
        unsafe_code
    )]
    pub mod sys {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../qualification/fixtures/generated-bindings/oxyflut-substrate.rs"
        ));
    }
}

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
fn generated_bindings_are_byte_stable_under_the_locked_toolchain() -> Result<(), Box<dyn Error>> {
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
fn macro_and_nullability_mutations_fail() -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let tools = NativeTools::load(&root)?;
    let source = fs::read_to_string(root.join(super::HEADER_PATH))?;
    let mutations = [
        (
            "stripped export visibility",
            "#define OXY_EXPORT __attribute__((visibility(\"default\")))",
            "#define OXY_EXPORT",
        ),
        (
            "flipped color-source nullability",
            "OxyColorSource* color_source; /* Nullable; null selects the inline solid color. */",
            "OxyColorSource* color_source; /* Nonnull; an inline color is required. */",
        ),
    ];
    for (name, original, replacement) in mutations {
        let temporary = TemporaryDirectory::new("native-annotation-mutation")?;
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
    let expected = [
        (
            authoritative::sys::OXY_NATIVE_TEXT_INDEX_UTF8_BYTES,
            authoritative::substrate::NativeTextIndexUnit::Utf8Bytes,
            "utf8-bytes",
        ),
        (
            authoritative::sys::OXY_NATIVE_TEXT_INDEX_UTF16_UNITS,
            authoritative::substrate::NativeTextIndexUnit::Utf16Units,
            "utf16-code-units",
        ),
        (
            authoritative::sys::OXY_NATIVE_TEXT_INDEX_UNICODE_SCALARS,
            authoritative::substrate::NativeTextIndexUnit::UnicodeScalars,
            "unicode-scalars",
        ),
    ];
    for (raw, unit, name) in expected {
        assert_eq!(native_index_unit_from_c(raw)?, unit);
        assert_eq!(platform_index_unit(unit), name);
        assert!(platform_contract_defines(&platform, name));
    }
    assert!(matches!(
        convert_native_range(99, 9, 1),
        Err(ContractConversionError::UnknownNativeIndexUnit(99))
    ));
    Ok(())
}

#[test]
fn presentation_statuses_map_and_reject_invalid_timestamps_before_delivery()
-> Result<(), Box<dyn Error>> {
    let expected = [
        (
            authoritative::sys::OXY_STATUS_OK,
            authoritative::substrate::PresentationStatus::Presented,
        ),
        (
            authoritative::sys::OXY_STATUS_INVALID_ARGUMENT,
            authoritative::substrate::PresentationStatus::InvalidArgument,
        ),
        (
            authoritative::sys::OXY_STATUS_INCOMPATIBLE_ABI,
            authoritative::substrate::PresentationStatus::IncompatibleAbi,
        ),
        (
            authoritative::sys::OXY_STATUS_STALE_OWNER,
            authoritative::substrate::PresentationStatus::StaleOwner,
        ),
        (
            authoritative::sys::OXY_STATUS_RESOURCE_LIMIT,
            authoritative::substrate::PresentationStatus::ResourceLimit,
        ),
        (
            authoritative::sys::OXY_STATUS_UNSUPPORTED,
            authoritative::substrate::PresentationStatus::Unsupported,
        ),
        (
            authoritative::sys::OXY_STATUS_SUBSTRATE_FAILURE,
            authoritative::substrate::PresentationStatus::SubstrateFailure,
        ),
        (
            authoritative::sys::OXY_STATUS_CANCELLED,
            authoritative::substrate::PresentationStatus::Cancelled,
        ),
        (
            authoritative::sys::OXY_STATUS_DEADLINE_EXCEEDED,
            authoritative::substrate::PresentationStatus::DeadlineExceeded,
        ),
    ];
    for (raw, expected_status) in expected {
        let timestamp = if raw == 0 { Some(42) } else { None };
        let mut recorder = PresentationRecorder::default();
        deliver_presentation(raw, timestamp, &mut recorder)?;
        assert_eq!(recorder.deliveries, vec![expected_status]);
        let invalid_timestamp = if raw == 0 { None } else { Some(42) };
        assert!(matches!(
            deliver_presentation(raw, invalid_timestamp, &mut recorder),
            Err(ContractConversionError::InvalidPresentationTimestamp)
        ));
    }
    let mut recorder = PresentationRecorder::default();
    assert!(matches!(
        deliver_presentation(99, Some(42), &mut recorder),
        Err(ContractConversionError::UnknownPresentationStatus(99))
    ));
    assert!(recorder.deliveries.is_empty());
    Ok(())
}

#[test]
fn texture_realization_contracts_accept_and_reject_the_same_inputs() -> Result<(), Box<dyn Error>> {
    let pixels = vec![0; 24];
    let c_pixels = borrowed_bytes(&pixels)?;
    let size = authoritative::substrate::PixelSize {
        width: 2,
        height: 3,
    };
    assert_eq!(
        validate_texture_realization(
            size,
            authoritative::substrate::PixelFormat::Rgba8888,
            c_pixels.length,
        ),
        Ok(24)
    );
    assert_eq!(
        validate_c_texture(
            size.width,
            size.height,
            authoritative::sys::OXY_PIXEL_FORMAT_RGBA8888,
            &c_pixels,
        ),
        Ok(24)
    );
    for (width, height, byte_length) in [(0, 3, 0), (2, 0, 0), (2, 3, 23), (u32::MAX, u32::MAX, 0)]
    {
        assert!(
            validate_texture_realization(
                authoritative::substrate::PixelSize { width, height },
                authoritative::substrate::PixelFormat::Rgba8888,
                byte_length,
            )
            .is_err()
        );
        assert!(
            validate_c_texture(
                width,
                height,
                authoritative::sys::OXY_PIXEL_FORMAT_RGBA8888,
                &borrowed_bytes_with_length(byte_length),
            )
            .is_err()
        );
    }
    assert!(validate_c_texture(size.width, size.height, 99, &c_pixels,).is_err());
    Ok(())
}

#[test]
fn semantics_selection_projects_some_and_none_and_rejects_invalid_c_forms()
-> Result<(), Box<dyn Error>> {
    let some = authoritative::public::Selection {
        range: authoritative::public::TextRange {
            start: authoritative::public::TextIndex::Utf16Units(4),
            end: authoritative::public::TextIndex::Utf16Units(12),
        },
        caret_at_start: false,
    };
    let substrate_some = public_selection_to_substrate(Some(&some))?;
    let c_some = selection_to_c_node(substrate_some)?;
    assert_eq!(c_to_substrate_selection(&c_some)?, substrate_some);
    let substrate_none = public_selection_to_substrate(None)?;
    let c_none = selection_to_c_node(substrate_none)?;
    assert_eq!(c_to_substrate_selection(&c_none)?, substrate_none);
    let _selection_field: fn(&authoritative::substrate::SemanticsNode) -> Option<(u32, u32)> =
        substrate_selection_field;
    let mut invalid_presence = selection_to_c_node(None)?;
    invalid_presence.text_selection_base_utf16 = 1;
    assert!(c_to_substrate_selection(&invalid_presence).is_err());
    invalid_presence.text_selection_base_utf16 = 0;
    invalid_presence.has_text_selection = 2;
    assert!(c_to_substrate_selection(&invalid_presence).is_err());
    invalid_presence.has_text_selection = 1;
    invalid_presence.text_selection_reserved = 1;
    assert!(c_to_substrate_selection(&invalid_presence).is_err());
    invalid_presence.text_selection_reserved = 0;
    invalid_presence.text_selection_base_utf16 = -1;
    assert!(c_to_substrate_selection(&invalid_presence).is_err());
    invalid_presence.text_selection_base_utf16 = i64::from(u32::MAX) + 1;
    assert!(c_to_substrate_selection(&invalid_presence).is_err());
    let non_utf16 = authoritative::public::Selection {
        range: authoritative::public::TextRange {
            start: authoritative::public::TextIndex::Utf8Bytes(0),
            end: authoritative::public::TextIndex::Utf8Bytes(1),
        },
        caret_at_start: false,
    };
    assert!(public_selection_to_substrate(Some(&non_utf16)).is_err());
    Ok(())
}

#[test]
fn abi_seven_through_nine_fail_before_callbacks_install() -> Result<(), Box<dyn Error>> {
    let table_size = api_table_size()?;
    for abi_version in 7..=9 {
        let mut callbacks_installed = false;
        assert!(matches!(
            install_callbacks_after_negotiation(abi_version, table_size, &mut callbacks_installed),
            Err(ContractConversionError::IncompatibleAbi(version)) if version == abi_version
        ));
        assert!(!callbacks_installed);
    }
    let mut callbacks_installed = false;
    install_callbacks_after_negotiation(
        authoritative::sys::OXY_SUBSTRATE_ABI_VERSION,
        table_size,
        &mut callbacks_installed,
    )?;
    assert!(callbacks_installed);
    assert!(matches!(
        install_callbacks_after_negotiation(
            authoritative::sys::OXY_SUBSTRATE_ABI_VERSION,
            table_size - 1,
            &mut callbacks_installed,
        ),
        Err(ContractConversionError::ApiTableTooSmall)
    ));
    Ok(())
}

fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "xtask must remain directly below the workspace root".into())
}

fn platform_contract_defines(contract: &Value, expected: &str) -> bool {
    contract
        .pointer("/$defs/ime/properties/nativeIndexUnit/enum")
        .and_then(Value::as_array)
        .is_some_and(|units| units.iter().any(|unit| unit.as_str() == Some(expected)))
}

fn native_index_unit_from_c(
    value: authoritative::sys::OxyNativeTextIndexUnit,
) -> Result<authoritative::substrate::NativeTextIndexUnit, ContractConversionError> {
    match value {
        value if value == authoritative::sys::OXY_NATIVE_TEXT_INDEX_UTF8_BYTES => {
            Ok(authoritative::substrate::NativeTextIndexUnit::Utf8Bytes)
        }
        value if value == authoritative::sys::OXY_NATIVE_TEXT_INDEX_UTF16_UNITS => {
            Ok(authoritative::substrate::NativeTextIndexUnit::Utf16Units)
        }
        value if value == authoritative::sys::OXY_NATIVE_TEXT_INDEX_UNICODE_SCALARS => {
            Ok(authoritative::substrate::NativeTextIndexUnit::UnicodeScalars)
        }
        value => Err(ContractConversionError::UnknownNativeIndexUnit(value)),
    }
}

fn platform_index_unit(unit: authoritative::substrate::NativeTextIndexUnit) -> &'static str {
    match unit {
        authoritative::substrate::NativeTextIndexUnit::Utf8Bytes => "utf8-bytes",
        authoritative::substrate::NativeTextIndexUnit::Utf16Units => "utf16-code-units",
        authoritative::substrate::NativeTextIndexUnit::UnicodeScalars => "unicode-scalars",
    }
}

fn convert_native_range(
    unit: authoritative::sys::OxyNativeTextIndexUnit,
    start: i64,
    end: i64,
) -> Result<(u32, u32), ContractConversionError> {
    let _unit = native_index_unit_from_c(unit)?;
    let start = u32::try_from(start).map_err(|_| ContractConversionError::InvalidRange)?;
    let end = u32::try_from(end).map_err(|_| ContractConversionError::InvalidRange)?;
    if start <= end {
        Ok((start, end))
    } else {
        Err(ContractConversionError::InvalidRange)
    }
}

fn presentation_status_from_c(
    value: authoritative::sys::OxyStatus,
) -> Result<authoritative::substrate::PresentationStatus, ContractConversionError> {
    match value {
        value if value == authoritative::sys::OXY_STATUS_OK => {
            Ok(authoritative::substrate::PresentationStatus::Presented)
        }
        value if value == authoritative::sys::OXY_STATUS_INVALID_ARGUMENT => {
            Ok(authoritative::substrate::PresentationStatus::InvalidArgument)
        }
        value if value == authoritative::sys::OXY_STATUS_INCOMPATIBLE_ABI => {
            Ok(authoritative::substrate::PresentationStatus::IncompatibleAbi)
        }
        value if value == authoritative::sys::OXY_STATUS_STALE_OWNER => {
            Ok(authoritative::substrate::PresentationStatus::StaleOwner)
        }
        value if value == authoritative::sys::OXY_STATUS_RESOURCE_LIMIT => {
            Ok(authoritative::substrate::PresentationStatus::ResourceLimit)
        }
        value if value == authoritative::sys::OXY_STATUS_UNSUPPORTED => {
            Ok(authoritative::substrate::PresentationStatus::Unsupported)
        }
        value if value == authoritative::sys::OXY_STATUS_SUBSTRATE_FAILURE => {
            Ok(authoritative::substrate::PresentationStatus::SubstrateFailure)
        }
        value if value == authoritative::sys::OXY_STATUS_CANCELLED => {
            Ok(authoritative::substrate::PresentationStatus::Cancelled)
        }
        value if value == authoritative::sys::OXY_STATUS_DEADLINE_EXCEEDED => {
            Ok(authoritative::substrate::PresentationStatus::DeadlineExceeded)
        }
        value => Err(ContractConversionError::UnknownPresentationStatus(value)),
    }
}

#[derive(Default)]
struct PresentationRecorder {
    deliveries: Vec<authoritative::substrate::PresentationStatus>,
}

fn deliver_presentation(
    raw_status: authoritative::sys::OxyStatus,
    timestamp: Option<u64>,
    recorder: &mut PresentationRecorder,
) -> Result<(), ContractConversionError> {
    let status = presentation_status_from_c(raw_status)?;
    if matches!(
        status,
        authoritative::substrate::PresentationStatus::Presented
    ) != timestamp.is_some()
    {
        return Err(ContractConversionError::InvalidPresentationTimestamp);
    }
    recorder.deliveries.push(status);
    Ok(())
}

fn pixel_format_from_c(
    value: authoritative::sys::OxyPixelFormat,
) -> Result<authoritative::substrate::PixelFormat, ContractConversionError> {
    if value == authoritative::sys::OXY_PIXEL_FORMAT_RGBA8888 {
        Ok(authoritative::substrate::PixelFormat::Rgba8888)
    } else {
        Err(ContractConversionError::UnknownPixelFormat(value))
    }
}

fn validate_texture_realization(
    size: authoritative::substrate::PixelSize,
    pixel_format: authoritative::substrate::PixelFormat,
    byte_length: u64,
) -> Result<u64, ContractConversionError> {
    if size.width == 0 || size.height == 0 {
        return Err(ContractConversionError::ZeroTextureDimension);
    }
    let bytes_per_pixel = match pixel_format {
        authoritative::substrate::PixelFormat::Rgba8888 => 4_u64,
    };
    let expected = u64::from(size.width)
        .checked_mul(u64::from(size.height))
        .and_then(|pixels| pixels.checked_mul(bytes_per_pixel))
        .ok_or(ContractConversionError::TextureLengthOverflow)?;
    if byte_length == expected {
        Ok(expected)
    } else {
        Err(ContractConversionError::TextureLengthMismatch)
    }
}

fn validate_c_texture(
    width: u32,
    height: u32,
    pixel_format: authoritative::sys::OxyPixelFormat,
    pixels: &authoritative::sys::OxyBorrowedBytes,
) -> Result<u64, ContractConversionError> {
    if pixels.data.is_null() != (pixels.length == 0) {
        return Err(ContractConversionError::InvalidBorrowedBytes);
    }
    validate_texture_realization(
        authoritative::substrate::PixelSize { width, height },
        pixel_format_from_c(pixel_format)?,
        pixels.length,
    )
}

fn borrowed_bytes(
    pixels: &[u8],
) -> Result<authoritative::sys::OxyBorrowedBytes, ContractConversionError> {
    let length =
        u64::try_from(pixels.len()).map_err(|_| ContractConversionError::TextureLengthOverflow)?;
    let data = if pixels.is_empty() {
        std::ptr::null()
    } else {
        pixels.as_ptr()
    };
    Ok(authoritative::sys::OxyBorrowedBytes { data, length })
}

fn borrowed_bytes_with_length(length: u64) -> authoritative::sys::OxyBorrowedBytes {
    let data = if length == 0 {
        std::ptr::null()
    } else {
        std::ptr::NonNull::<u8>::dangling().as_ptr()
    };
    authoritative::sys::OxyBorrowedBytes { data, length }
}

fn public_selection_to_substrate(
    selection: Option<&authoritative::public::Selection>,
) -> Result<Option<(u32, u32)>, ContractConversionError> {
    let Some(selection) = selection else {
        return Ok(None);
    };
    Ok(Some((
        public_utf16_offset(selection.range.start)?,
        public_utf16_offset(selection.range.end)?,
    )))
}

fn public_utf16_offset(
    index: authoritative::public::TextIndex,
) -> Result<u32, ContractConversionError> {
    match index {
        authoritative::public::TextIndex::Utf8Bytes(_) => {
            Err(ContractConversionError::PublicSelectionIsNotUtf16)
        }
        authoritative::public::TextIndex::Utf16Units(offset) => Ok(offset),
        authoritative::public::TextIndex::Grapheme(_) => {
            Err(ContractConversionError::PublicSelectionIsNotUtf16)
        }
        authoritative::public::TextIndex::Logical(_) => {
            Err(ContractConversionError::PublicSelectionIsNotUtf16)
        }
    }
}

fn substrate_selection_field(node: &authoritative::substrate::SemanticsNode) -> Option<(u32, u32)> {
    node.selection_utf16
}

fn selection_to_c_node(
    selection: Option<(u32, u32)>,
) -> Result<authoritative::sys::OxySemanticsNode, ContractConversionError> {
    let (base, extent, has_selection) = match selection {
        Some((base, extent)) => (i64::from(base), i64::from(extent), 1),
        None => (0, 0, 0),
    };
    let empty = authoritative::sys::OxyBorrowedBytes {
        data: std::ptr::null(),
        length: 0,
    };
    let struct_size = u32::try_from(std::mem::size_of::<authoritative::sys::OxySemanticsNode>())
        .map_err(|_| ContractConversionError::StructSizeOverflow)?;
    Ok(authoritative::sys::OxySemanticsNode {
        struct_size,
        abi_version: authoritative::sys::OXY_SUBSTRATE_ABI_VERSION,
        node_generation: 0,
        flags: 0,
        actions: 0,
        role: 0,
        live_region: 0,
        bounds: authoritative::sys::OxyRect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        transform: authoritative::sys::OxyTransform {
            column_major: [0.0; 16],
        },
        label_utf8: empty,
        accessible_name_utf8: empty,
        description_utf8: empty,
        value_utf8: empty,
        hint_utf8: empty,
        help_utf8: empty,
        tooltip_utf8: empty,
        attributed_text_index_unit: authoritative::sys::OXY_NATIVE_TEXT_INDEX_UTF8_BYTES,
        attributed_text_reserved: 0,
        attributed_text_segments: std::ptr::null(),
        attributed_text_segment_count: 0,
        identifier_utf8: empty,
        language_bcp47_utf8: empty,
        traversal_children: std::ptr::null(),
        traversal_child_count: 0,
        hit_test_children: std::ptr::null(),
        hit_test_child_count: 0,
        relations: std::ptr::null(),
        relation_count: 0,
        text_selection_base_utf16: base,
        text_selection_extent_utf16: extent,
        has_text_selection: has_selection,
        text_selection_reserved: 0,
        text_layout_generation: u64::from(has_selection),
        has_text_layout: has_selection,
        text_layout_reserved: 0,
        scroll_position: 0.0,
        scroll_minimum: 0.0,
        scroll_maximum: 0.0,
        has_scroll: 0,
        heading_level: 0,
        text_direction: 0,
        has_input_focus: 0,
        has_accessibility_focus: 0,
        is_hidden: 0,
        is_disabled: 0,
        is_secure_field: 0,
    })
}

fn c_to_substrate_selection(
    node: &authoritative::sys::OxySemanticsNode,
) -> Result<Option<(u32, u32)>, ContractConversionError> {
    if node.text_selection_reserved != 0 {
        return Err(ContractConversionError::ReservedSelectionField);
    }
    match node.has_text_selection {
        0 if node.text_selection_base_utf16 == 0 && node.text_selection_extent_utf16 == 0 => {
            Ok(None)
        }
        0 => Err(ContractConversionError::AbsentSelectionHasEndpoints),
        1 => Ok(Some((
            u32::try_from(node.text_selection_base_utf16)
                .map_err(|_| ContractConversionError::InvalidSelectionEndpoint)?,
            u32::try_from(node.text_selection_extent_utf16)
                .map_err(|_| ContractConversionError::InvalidSelectionEndpoint)?,
        ))),
        value => Err(ContractConversionError::InvalidSelectionPresence(value)),
    }
}

fn api_table_size() -> Result<u32, ContractConversionError> {
    u32::try_from(std::mem::size_of::<authoritative::sys::OxySubstrateApi>())
        .map_err(|_| ContractConversionError::StructSizeOverflow)
}

fn install_callbacks_after_negotiation(
    implementation_abi: u32,
    implementation_table_size: u32,
    callbacks_installed: &mut bool,
) -> Result<(), ContractConversionError> {
    if implementation_abi != authoritative::sys::OXY_SUBSTRATE_ABI_VERSION {
        return Err(ContractConversionError::IncompatibleAbi(implementation_abi));
    }
    if implementation_table_size < api_table_size()? {
        return Err(ContractConversionError::ApiTableTooSmall);
    }
    *callbacks_installed = true;
    Ok(())
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
enum ContractConversionError {
    #[error("unknown native text index unit")]
    UnknownNativeIndexUnit(u32),
    #[error("invalid native range")]
    InvalidRange,
    #[error("unknown presentation status")]
    UnknownPresentationStatus(u32),
    #[error("invalid presentation timestamp")]
    InvalidPresentationTimestamp,
    #[error("unknown pixel format")]
    UnknownPixelFormat(u32),
    #[error("texture dimension is zero")]
    ZeroTextureDimension,
    #[error("texture length overflow")]
    TextureLengthOverflow,
    #[error("texture length mismatch")]
    TextureLengthMismatch,
    #[error("borrowed bytes violate their nullability contract")]
    InvalidBorrowedBytes,
    #[error("public selection is not expressed in UTF-16 units")]
    PublicSelectionIsNotUtf16,
    #[error("selection reserved field is nonzero")]
    ReservedSelectionField,
    #[error("absent selection has endpoints")]
    AbsentSelectionHasEndpoints,
    #[error("selection presence is invalid")]
    InvalidSelectionPresence(u32),
    #[error("selection endpoint is invalid")]
    InvalidSelectionEndpoint,
    #[error("a generated ABI table size cannot fit in u32")]
    StructSizeOverflow,
    #[error("ABI is incompatible")]
    IncompatibleAbi(u32),
    #[error("the ABI table is smaller than the generated OxySubstrateApi shape")]
    ApiTableTooSmall,
}
