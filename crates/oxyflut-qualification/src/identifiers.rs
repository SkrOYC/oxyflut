//! Strongly typed qualification identifiers and canonical repository paths.
//!
//! Durable qualification inputs use these types at validation boundaries so an identifier or path cannot be confused with an arbitrary string.

use std::fmt;
use std::str::FromStr;

use thiserror::Error;

/// Explains why a qualification identifier or canonical path is invalid.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum IdentifierError {
    /// The value did not match the identifier's closed syntax.
    #[error("invalid {kind} identifier")]
    InvalidIdentifier {
        /// The rejected identifier family.
        kind: &'static str,
    },
    /// The value was empty where a canonical path requires one or more segments.
    #[error("canonical path is empty")]
    EmptyPath,
    /// The path contains a platform-specific drive prefix.
    #[error("canonical path contains a drive prefix")]
    DrivePrefix,
    /// The path contains a backslash instead of a slash separator.
    #[error("canonical path contains a backslash")]
    Backslash,
    /// The path contains a NUL byte.
    #[error("canonical path contains a NUL byte")]
    Nul,
    /// The path contains an ASCII control character.
    #[error("canonical path contains a control character")]
    ControlCharacter,
    /// The path contains an empty component.
    #[error("canonical path contains an empty segment")]
    EmptySegment,
    /// The path contains a current-directory or parent-directory component.
    #[error("canonical path contains a dot segment")]
    DotSegment,
}

macro_rules! string_identifier {
    ($name:ident, $description:literal, $kind:literal, $check:ident) => {
        #[doc = $description]
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Parses a checked identifier.
            ///
            /// # Errors
            ///
            /// Returns [`IdentifierError::InvalidIdentifier`] when the value does not use the required syntax.
            pub fn parse(value: &str) -> Result<Self, IdentifierError> {
                if $check(value) {
                    Ok(Self(value.to_owned()))
                } else {
                    Err(IdentifierError::InvalidIdentifier { kind: $kind })
                }
            }

            /// Returns the canonical identifier text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl FromStr for $name {
            type Err = IdentifierError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::parse(value)
            }
        }
    };
}

string_identifier!(
    CapabilityId,
    "Identifies one release-blocking product capability.",
    "capability",
    is_capability_id
);
string_identifier!(
    ConstraintId,
    "Identifies one release-blocking product constraint.",
    "constraint",
    is_constraint_id
);
string_identifier!(
    AbsentEventId,
    "Identifies one immutable absent platform event.",
    "absent-event",
    is_absent_event_id
);
string_identifier!(
    EventName,
    "Identifies one registered diagnostic or platform event name.",
    "event",
    is_event_name
);
string_identifier!(
    RegistryVersion,
    "Identifies the semantic version of the diagnostic event registry.",
    "diagnostic-registry-version",
    is_semantic_version
);
string_identifier!(
    SchemaVersion,
    "Identifies the semantic version of a durable data schema.",
    "schema-version",
    is_semantic_version
);
string_identifier!(
    SpecificationVersion,
    "Identifies the active technical specification version.",
    "specification-version",
    is_semantic_version
);
string_identifier!(
    ContractTestId,
    "Identifies one deferred common candidate contract test.",
    "contract-test",
    is_contract_test_id
);

/// Identifies a capability or constraint gate without erasing its namespace.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GateId {
    /// A product capability gate.
    Capability(CapabilityId),
    /// A product constraint gate.
    Constraint(ConstraintId),
}

impl GateId {
    /// Parses a checked capability or constraint gate identifier.
    ///
    /// # Errors
    ///
    /// Returns an identifier error when the value belongs to neither gate family.
    pub fn parse(value: &str) -> Result<Self, IdentifierError> {
        if let Ok(capability) = CapabilityId::parse(value) {
            return Ok(Self::Capability(capability));
        }
        ConstraintId::parse(value).map(Self::Constraint)
    }

    /// Returns the canonical identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Capability(identifier) => identifier.as_str(),
            Self::Constraint(identifier) => identifier.as_str(),
        }
    }
}

impl fmt::Display for GateId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for GateId {
    type Err = IdentifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

/// Selects a fixed Tier 1 qualification environment.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EnvironmentId {
    /// macOS on arm64.
    Macos,
    /// Windows on x86-64.
    Windows,
    /// Linux under Wayland.
    Wayland,
    /// Linux under X11.
    X11,
}

impl EnvironmentId {
    /// Returns the canonical lower-case environment identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Macos => "macos",
            Self::Windows => "windows",
            Self::Wayland => "wayland",
            Self::X11 => "x11",
        }
    }

    /// Returns all Tier 1 environment identifiers in stable order.
    #[must_use]
    pub const fn tier_one() -> [Self; 4] {
        [Self::Macos, Self::Windows, Self::Wayland, Self::X11]
    }
}

impl fmt::Display for EnvironmentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EnvironmentId {
    type Err = IdentifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "macos" => Ok(Self::Macos),
            "windows" => Ok(Self::Windows),
            "wayland" => Ok(Self::Wayland),
            "x11" => Ok(Self::X11),
            _ => Err(IdentifierError::InvalidIdentifier {
                kind: "environment",
            }),
        }
    }
}

/// Selects one of the two fixed substrate candidates.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CandidateId {
    /// The focused drawing-and-text candidate.
    Focused,
    /// The integrated engine candidate.
    Integrated,
}

impl CandidateId {
    /// Returns the canonical lower-case candidate identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Focused => "focused",
            Self::Integrated => "integrated",
        }
    }

    /// Returns every admitted candidate in stable order.
    #[must_use]
    pub const fn all() -> [Self; 2] {
        [Self::Focused, Self::Integrated]
    }
}

impl fmt::Display for CandidateId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CandidateId {
    type Err = IdentifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "focused" => Ok(Self::Focused),
            "integrated" => Ok(Self::Integrated),
            _ => Err(IdentifierError::InvalidIdentifier { kind: "candidate" }),
        }
    }
}

/// A canonical slash-separated path relative to the repository root.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RepositoryPath(String);

impl RepositoryPath {
    /// Parses a canonical repository-relative path.
    ///
    /// The path has no drive prefix, backslash, NUL byte, control character, empty segment, or `.` or `..` segment.
    ///
    /// # Errors
    ///
    /// Returns an [`IdentifierError`] that identifies the rejected canonical-path rule.
    pub fn parse(value: &str) -> Result<Self, IdentifierError> {
        validate_canonical_path(value)?;
        Ok(Self(value.to_owned()))
    }

    /// Returns the canonical slash-separated path.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RepositoryPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for RepositoryPath {
    type Err = IdentifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

/// A canonical artifact-root-relative symlink or hardlink target.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LinkTarget(RepositoryPath);

impl LinkTarget {
    /// Parses a canonical artifact-root-relative link target.
    ///
    /// # Errors
    ///
    /// Returns an [`IdentifierError`] when the target can escape the artifact root or violates canonical path rules.
    pub fn parse(value: &str) -> Result<Self, IdentifierError> {
        RepositoryPath::parse(value).map(Self)
    }

    /// Returns the canonical target relative to the artifact root.
    #[must_use]
    pub fn as_path(&self) -> &RepositoryPath {
        &self.0
    }
}

impl fmt::Display for LinkTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for LinkTarget {
    type Err = IdentifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

fn is_capability_id(value: &str) -> bool {
    is_prefixed_numeric_id(value, "CAP-")
}

fn is_constraint_id(value: &str) -> bool {
    is_prefixed_numeric_id(value, "CON-")
}

fn is_absent_event_id(value: &str) -> bool {
    is_prefixed_numeric_id(value, "ABS-")
}

fn is_prefixed_numeric_id(value: &str, prefix: &str) -> bool {
    let Some(remainder) = value.strip_prefix(prefix) else {
        return false;
    };
    let Some((category, ordinal)) = remainder.rsplit_once('-') else {
        return false;
    };
    !category.is_empty()
        && category
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
        && ordinal.len() == 3
        && ordinal.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_event_name(value: &str) -> bool {
    !value.is_empty()
        && value.split('.').all(|segment| {
            let mut characters = segment.bytes();
            matches!(characters.next(), Some(first) if first.is_ascii_lowercase())
                && characters.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        })
}

fn is_semantic_version(value: &str) -> bool {
    let mut components = value.split('.');
    matches!(
        (components.next(), components.next(), components.next(), components.next()),
        (Some(major), Some(minor), Some(patch), None)
            if is_version_component(major)
                && is_version_component(minor)
                && is_version_component(patch)
    )
}

fn is_version_component(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_contract_test_id(value: &str) -> bool {
    let Some(identifier) = value.strip_prefix("contract::cap_") else {
        return false;
    };
    !identifier.is_empty()
        && identifier
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn validate_canonical_path(value: &str) -> Result<(), IdentifierError> {
    if value.is_empty() {
        return Err(IdentifierError::EmptyPath);
    }
    if value.len() >= 2 && value.as_bytes()[0].is_ascii_alphabetic() && value.as_bytes()[1] == b':'
    {
        return Err(IdentifierError::DrivePrefix);
    }
    if value.contains('\\') {
        return Err(IdentifierError::Backslash);
    }
    if value.contains('\0') {
        return Err(IdentifierError::Nul);
    }
    if value
        .chars()
        .any(|character| character.is_control() || character == '\u{7f}')
    {
        return Err(IdentifierError::ControlCharacter);
    }
    for segment in value.split('/') {
        if segment.is_empty() {
            return Err(IdentifierError::EmptySegment);
        }
        if matches!(segment, "." | "..") {
            return Err(IdentifierError::DotSegment);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::{
        AbsentEventId, CandidateId, CapabilityId, ConstraintId, ContractTestId, EnvironmentId,
        EventName, GateId, IdentifierError, LinkTarget, RegistryVersion, RepositoryPath,
        SchemaVersion, SpecificationVersion,
    };

    #[test]
    fn checked_identifier_types_accept_only_their_closed_forms() -> Result<(), Box<dyn Error>> {
        assert_eq!(CapabilityId::parse("CAP-CMP-001")?.as_str(), "CAP-CMP-001");
        assert_eq!(
            ConstraintId::parse("CON-PERF-001")?.as_str(),
            "CON-PERF-001"
        );
        assert_eq!(AbsentEventId::parse("ABS-IME-001")?.as_str(), "ABS-IME-001");
        assert_eq!(
            EventName::parse("frame.presented")?.as_str(),
            "frame.presented"
        );
        assert_eq!(RegistryVersion::parse("2.0.0")?.as_str(), "2.0.0");
        assert_eq!(SchemaVersion::parse("5.0.0")?.as_str(), "5.0.0");
        assert_eq!(SpecificationVersion::parse("0.15.0")?.as_str(), "0.15.0");
        assert_eq!(
            ContractTestId::parse("contract::cap_cmp_001")?.as_str(),
            "contract::cap_cmp_001"
        );
        assert!(CapabilityId::parse("CAP-cmp-001").is_err());
        assert!(ConstraintId::parse("CAP-CMP-001").is_err());
        assert!(EventName::parse("Frame.Presented").is_err());
        assert!(RegistryVersion::parse("2.0").is_err());
        Ok(())
    }

    #[test]
    fn gates_and_closed_candidate_sets_are_typed() -> Result<(), Box<dyn Error>> {
        assert!(matches!(
            GateId::parse("CAP-CMP-001")?,
            GateId::Capability(_)
        ));
        assert!(matches!(
            GateId::parse("CON-PERF-001")?,
            GateId::Constraint(_)
        ));
        assert_eq!(EnvironmentId::tier_one().len(), 4);
        assert_eq!("wayland".parse::<EnvironmentId>()?, EnvironmentId::Wayland);
        assert_eq!(CandidateId::all().len(), 2);
        assert_eq!("focused".parse::<CandidateId>()?, CandidateId::Focused);
        assert!("remote".parse::<CandidateId>().is_err());
        Ok(())
    }

    #[test]
    fn canonical_paths_reject_all_forbidden_forms() -> Result<(), Box<dyn Error>> {
        let path = RepositoryPath::parse("qualification/evidence/sample.json")?;
        assert_eq!(path.as_str(), "qualification/evidence/sample.json");
        assert_eq!(
            LinkTarget::parse("bin/oxyflut")?.as_path().as_str(),
            "bin/oxyflut"
        );
        for value in [
            "",
            "/absolute",
            "C:/drive",
            "dir\\file",
            "dir/../file",
            "dir/./file",
            "dir//file",
            "dir/",
            "dir/\u{0001}file",
            "dir/\0file",
        ] {
            assert!(RepositoryPath::parse(value).is_err(), "{value:?} must fail");
        }
        assert_eq!(
            RepositoryPath::parse("dir//file"),
            Err(IdentifierError::EmptySegment)
        );
        Ok(())
    }
}
