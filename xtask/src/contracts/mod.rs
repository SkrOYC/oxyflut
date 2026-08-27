//! Contract-validation families owned by the qualification command.

pub(crate) mod digests;
pub(crate) mod native;
pub(crate) mod readiness;
pub(crate) mod registries;
pub(crate) mod schema;
pub(crate) mod traceability;

/// Returns validation families that are deliberately fail-closed until later tickets implement them.
pub(crate) const fn unimplemented_families() -> [&'static str; 3] {
    [readiness::FAMILY, digests::FAMILY, native::FAMILY]
}
