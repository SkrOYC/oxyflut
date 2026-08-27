// Test-only mirrors for native/common contract boundary assertions. This file is included by
// xtask/src/contracts/native.rs so it is not candidate adapter code.

fn platform_contract_defines(contract: &Value, expected: &str) -> bool {
    contract
        .pointer("/$defs/ime/properties/nativeIndexUnit/enum")
        .and_then(Value::as_array)
        .is_some_and(|units| units.iter().any(|unit| unit.as_str() == Some(expected)))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NativeTextIndexUnit {
    Utf8Bytes,
    Utf16Units,
    UnicodeScalars,
}

impl NativeTextIndexUnit {
    const fn platform_name(self) -> &'static str {
        match self {
            Self::Utf8Bytes => "utf8-bytes",
            Self::Utf16Units => "utf16-code-units",
            Self::UnicodeScalars => "unicode-scalars",
        }
    }
}

impl TryFrom<u32> for NativeTextIndexUnit {
    type Error = MirrorError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Utf8Bytes),
            2 => Ok(Self::Utf16Units),
            3 => Ok(Self::UnicodeScalars),
            value => Err(MirrorError::UnknownNativeIndexUnit(value)),
        }
    }
}

fn convert_native_range(unit: u32, start: i64, end: i64) -> Result<(u32, u32), MirrorError> {
    let _unit = NativeTextIndexUnit::try_from(unit)?;
    let start = u32::try_from(start).map_err(|_| MirrorError::InvalidRange)?;
    let end = u32::try_from(end).map_err(|_| MirrorError::InvalidRange)?;
    if start <= end {
        Ok((start, end))
    } else {
        Err(MirrorError::InvalidRange)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PresentationStatus {
    Presented,
    InvalidArgument,
    IncompatibleAbi,
    StaleOwner,
    ResourceLimit,
    Unsupported,
    SubstrateFailure,
    Cancelled,
    DeadlineExceeded,
}

fn presentation_status(status: u32) -> Result<PresentationStatus, MirrorError> {
    match status {
        0 => Ok(PresentationStatus::Presented),
        1 => Ok(PresentationStatus::InvalidArgument),
        2 => Ok(PresentationStatus::IncompatibleAbi),
        3 => Ok(PresentationStatus::StaleOwner),
        4 => Ok(PresentationStatus::ResourceLimit),
        5 => Ok(PresentationStatus::Unsupported),
        6 => Ok(PresentationStatus::SubstrateFailure),
        7 => Ok(PresentationStatus::Cancelled),
        8 => Ok(PresentationStatus::DeadlineExceeded),
        status => Err(MirrorError::UnknownPresentationStatus(status)),
    }
}

#[derive(Default)]
struct PresentationRecorder {
    deliveries: Vec<PresentationStatus>,
}

fn deliver_presentation(
    raw_status: u32,
    timestamp: Option<u64>,
    recorder: &mut PresentationRecorder,
) -> Result<(), MirrorError> {
    let status = presentation_status(raw_status)?;
    if (status == PresentationStatus::Presented) != timestamp.is_some() {
        return Err(MirrorError::InvalidPresentationTimestamp);
    }
    recorder.deliveries.push(status);
    Ok(())
}

#[derive(Clone, Copy)]
enum CommonPixelFormat {
    Rgba8888,
}

impl TryFrom<u32> for CommonPixelFormat {
    type Error = MirrorError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Rgba8888),
            value => Err(MirrorError::UnknownPixelFormat(value)),
        }
    }
}

fn validate_common_texture_from_native(
    width: u32,
    height: u32,
    format: u32,
    bytes: u64,
) -> Result<u64, MirrorError> {
    validate_common_texture(width, height, CommonPixelFormat::try_from(format)?, bytes)
}

fn validate_common_texture(
    width: u32,
    height: u32,
    pixel_format: CommonPixelFormat,
    bytes: u64,
) -> Result<u64, MirrorError> {
    match pixel_format {
        CommonPixelFormat::Rgba8888 => validate_texture_dimensions(width, height, bytes),
    }
}

fn validate_c_texture(width: u32, height: u32, format: u32, bytes: u64) -> Result<u64, MirrorError> {
    match format {
        1 => validate_texture_dimensions(width, height, bytes),
        format => Err(MirrorError::UnknownPixelFormat(format)),
    }
}

fn validate_texture_dimensions(width: u32, height: u32, bytes: u64) -> Result<u64, MirrorError> {
    if width == 0 || height == 0 {
        return Err(MirrorError::ZeroTextureDimension);
    }
    let expected = u64::from(width)
        .checked_mul(u64::from(height))
        .and_then(|pixels| pixels.checked_mul(4))
        .ok_or(MirrorError::TextureLengthOverflow)?;
    if bytes == expected {
        Ok(expected)
    } else {
        Err(MirrorError::TextureLengthMismatch)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CommonSelection {
    Some(u32, u32),
    None,
}

#[derive(Clone, Copy)]
struct CSelection {
    base: i64,
    extent: i64,
    has_selection: u32,
    reserved: u32,
}

fn common_to_c_selection(selection: CommonSelection) -> Result<CSelection, MirrorError> {
    match selection {
        CommonSelection::Some(base, extent) => Ok(CSelection {
            base: i64::from(base),
            extent: i64::from(extent),
            has_selection: 1,
            reserved: 0,
        }),
        CommonSelection::None => Ok(CSelection {
            base: 0,
            extent: 0,
            has_selection: 0,
            reserved: 0,
        }),
    }
}

fn c_to_common_selection(selection: CSelection) -> Result<CommonSelection, MirrorError> {
    if selection.reserved != 0 {
        return Err(MirrorError::ReservedSelectionField);
    }
    match selection.has_selection {
        0 if selection.base == 0 && selection.extent == 0 => Ok(CommonSelection::None),
        0 => Err(MirrorError::AbsentSelectionHasEndpoints),
        1 => Ok(CommonSelection::Some(
            u32::try_from(selection.base).map_err(|_| MirrorError::InvalidSelectionEndpoint)?,
            u32::try_from(selection.extent).map_err(|_| MirrorError::InvalidSelectionEndpoint)?,
        )),
        value => Err(MirrorError::InvalidSelectionPresence(value)),
    }
}

fn negotiate_abi(implementation_abi: u32, callbacks_installed: &mut bool) -> Result<(), MirrorError> {
    if implementation_abi != 10 {
        return Err(MirrorError::IncompatibleAbi(implementation_abi));
    }
    *callbacks_installed = true;
    Ok(())
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
enum MirrorError {
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
    #[error("selection reserved field is nonzero")]
    ReservedSelectionField,
    #[error("absent selection has endpoints")]
    AbsentSelectionHasEndpoints,
    #[error("selection presence is invalid")]
    InvalidSelectionPresence(u32),
    #[error("selection endpoint is invalid")]
    InvalidSelectionEndpoint,
    #[error("ABI is incompatible")]
    IncompatibleAbi(u32),
}
