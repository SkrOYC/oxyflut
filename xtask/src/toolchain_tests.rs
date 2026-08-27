//! Staged native toolchain tests.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use super::{STAGED_HOST, ToolchainError, ToolchainManifest, resolve, verify};

#[test]
fn every_required_tool_has_complete_staged_metadata() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let manifest = resolve()?;
    assert_eq!(manifest.authority, "staged-proposal");
    assert!(manifest.note.contains("Stage 3 reconciliation"));
    assert_eq!(manifest.resolved_tools.len(), super::TOOL_SPECS.len());
    assert_eq!(manifest.sdk_utilities.len(), 1);

    for tool in &manifest.resolved_tools {
        assert!(!tool.name.is_empty());
        assert!(!tool.version.is_empty());
        assert!(!tool.source_identity.is_empty());
        assert_eq!(tool.host_triple, STAGED_HOST);
        assert!(!tool.license_id.is_empty());
        assert!(super::resolve_manifest_executable_path(tool)?.is_file());
        assert!(tool.sha256.parse::<super::Sha256Digest>().is_ok());
    }

    let sdk = &manifest.sdk_utilities[0];
    assert_eq!(sdk.name, "linux-libc-headers");
    assert!(!sdk.version.is_empty());
    assert!(!sdk.source_identity.is_empty());
    assert_eq!(sdk.host_triple, STAGED_HOST);
    assert!(!sdk.license_id.is_empty());
    assert!(Path::new(&sdk.path).is_dir());
    assert!(sdk.sha256.parse::<super::Sha256Digest>().is_ok());
    assert!(sdk.purpose.contains("syntax-only"));
    Ok(())
}

#[test]
fn prettier_is_a_declared_immutable_executable() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let manifest = resolve()?;
    let prettier = manifest
        .resolved_tools
        .iter()
        .find(|tool| tool.name == "prettier")
        .ok_or("the staged manifest must resolve Prettier")?;

    assert_eq!(prettier.version, "3.9.6");
    assert!(
        prettier.executable_path.ends_with("/bin/prettier"),
        "the manifest must record the Prettier executable rather than a package launcher"
    );
    assert!(
        prettier.source_identity.contains("prettier-3.9.6"),
        "the manifest must bind Prettier to its immutable Nix derivation"
    );
    assert!(
        prettier.executable_path.starts_with("/nix/store/"),
        "the manifest must not resolve Prettier from a mutable package cache"
    );
    assert_eq!(
        super::resolve_manifest_executable_path(prettier)?,
        super::executable_path(super::tool_specification("prettier")?)?
    );
    Ok(())
}

#[test]
fn rustfmt_identity_is_root_relative_and_digest_bound() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let manifest = resolve()?;
    let rustfmt = manifest
        .resolved_tools
        .iter()
        .find(|tool| tool.name == "rustfmt")
        .ok_or("the staged manifest must resolve rustfmt")?;

    assert_eq!(rustfmt.path_root.as_deref(), Some("rustup-home"));
    assert_eq!(rustfmt.executable_path, super::RUSTFMT_RELATIVE_PATH);
    assert!(!rustfmt.executable_path.contains("/home/"));
    assert_eq!(
        rustfmt.source_identity,
        format!(
            "rustup-toolchain: {}; commit: {}",
            super::RUST_TOOLCHAIN_NAME,
            super::RUST_TOOLCHAIN_COMMIT
        )
    );

    let resolved_path = super::resolve_manifest_executable_path(rustfmt)?;
    assert_eq!(
        resolved_path,
        super::rustup_component_path("rustfmt", super::RUSTFMT_RELATIVE_PATH)?
    );
    assert_eq!(
        rustfmt.sha256,
        super::hash_file(&resolved_path)?.to_string()
    );
    verify(&manifest)?;
    Ok(())
}

#[test]
fn rustc_identity_is_root_relative_and_individually_verified() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let manifest = resolve()?;
    let rustc = manifest
        .resolved_tools
        .iter()
        .find(|tool| tool.name == "rustc")
        .ok_or("the staged manifest must resolve rustc")?;

    assert_eq!(rustc.path_root.as_deref(), Some("rustup-home"));
    assert_eq!(rustc.executable_path, super::RUSTC_RELATIVE_PATH);
    assert!(!rustc.executable_path.contains("/home/"));
    assert_eq!(
        super::resolve_manifest_executable_path(rustc)?,
        super::rustup_component_path("rustc", super::RUSTC_RELATIVE_PATH)?
    );
    assert_eq!(
        rustc.sha256,
        super::hash_file(&super::rustup_component_path(
            "rustc",
            super::RUSTC_RELATIVE_PATH,
        )?)?
        .to_string()
    );
    assert_eq!(
        manifest.verified_executable_path("rustc")?,
        super::rustup_component_path("rustc", super::RUSTC_RELATIVE_PATH)?
    );
    Ok(())
}

#[test]
fn re_resolution_is_byte_identical_on_the_locked_host() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let first = resolve()?.canonical_bytes()?;
    let second = resolve()?.canonical_bytes()?;
    assert_eq!(first, second);
    Ok(())
}

#[test]
fn committed_manifest_matches_this_locked_host_with_a_clear_cross_host_skip()
-> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let generated = resolve()?.canonical_bytes()?;
    let committed =
        fs::read(workspace_root()?.join("qualification/tools/native-contract-toolchain.json"))?;
    assert_eq!(
        generated, committed,
        "the committed staged proposal differs from this locked host; regenerate it only after reviewing the immutable tool identities"
    );
    Ok(())
}

#[test]
fn malformed_toolchain_fixtures_fail_before_native_validation() -> Result<(), Box<dyn Error>> {
    if skip_on_unsupported_host()? {
        return Ok(());
    }
    let root = workspace_root()?.join("qualification/fixtures/toolchain");
    let cases = [
        ("missing-tool.json", "missing"),
        ("substituted-executable.json", "substituted"),
        (
            "prettier-substituted-executable.json",
            "prettier-substituted",
        ),
        ("digest-mismatch.json", "digest"),
        ("wrong-version.json", "version"),
    ];

    for (fixture, reason) in cases {
        let manifest = ToolchainManifest::from_json(&fs::read(root.join(fixture))?)?;
        let error = verify(&manifest)
            .err()
            .ok_or("fixture unexpectedly verified")?;
        match (reason, error) {
            ("missing", ToolchainError::MissingTool { .. })
            | ("substituted", ToolchainError::ExecutableSubstitution { .. })
            | ("prettier-substituted", ToolchainError::ExecutableSubstitution { .. })
            | ("digest", ToolchainError::DigestMismatch { .. })
            | ("version", ToolchainError::VersionMismatch { .. }) => {}
            (_, error) => return Err(format!("fixture {fixture} failed for {error}").into()),
        }
    }
    Ok(())
}

#[test]
fn staged_manifest_rejects_readiness_declarations() -> Result<(), Box<dyn Error>> {
    let path =
        workspace_root()?.join("qualification/fixtures/toolchain/readiness-declaration.json");
    let result = ToolchainManifest::from_json(&fs::read(path)?);
    assert!(matches!(result, Err(ToolchainError::ReadinessField { .. })));
    Ok(())
}

fn skip_on_unsupported_host() -> Result<bool, Box<dyn Error>> {
    if super::is_staged_host()? {
        Ok(false)
    } else {
        eprintln!("skipped: staged toolchain host is x86_64-unknown-linux-gnu");
        Ok(true)
    }
}

fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "xtask must remain directly below the workspace root".into())
}
