use std::error::Error;
use std::fs;
use std::path::Path;

use oxyflut_qualification::hash::hash_file;

use super::{
    ExternalContractsError, PROPOSAL_PATH, ProposalCode, SNAPSHOTS, SPDX_SCHEMA_PATH,
    TemporaryDirectory, decode_base64, dsse_pae, outcome_at, read_json, run, verify_at,
    verify_publication_source_bytes, workspace_root,
};
use crate::CommandOutcome;

#[test]
fn verifies_authoritative_snapshots_and_local_semantics_without_network()
-> Result<(), Box<dyn Error>> {
    let root = workspace_root().map_err(|_| "xtask must remain below the workspace root")?;
    verify_at(&root)?;
    assert_eq!(run(&[]), CommandOutcome::Success);
    Ok(())
}

#[test]
fn mutated_snapshot_bytes_fail_the_recorded_digest() -> Result<(), Box<dyn Error>> {
    let temporary = staged_root()?;
    let fixture = temporary
        .path()
        .join("qualification/fixtures/external-contracts/negative/mutated-schema.bytes");
    let snapshot = SNAPSHOTS
        .iter()
        .find(|snapshot| snapshot.artifact_path == SPDX_SCHEMA_PATH)
        .ok_or("SPDX schema snapshot must be registered")?;
    fs::copy(&fixture, temporary.path().join(snapshot.artifact_path))?;

    let error = verify_at(temporary.path())
        .err()
        .ok_or("mutation must fail")?;
    assert!(matches!(error, ExternalContractsError::Snapshot { .. }));
    Ok(())
}

#[test]
fn mutated_statement_predicate_and_envelope_fail_local_semantics() -> Result<(), Box<dyn Error>> {
    for (fixture_name, target_name) in [
        ("mutated-statement.json", "statement.json"),
        ("mutated-predicate.json", "provenance.json"),
        ("mutated-envelope.json", "envelope.json"),
        ("invalid-resolved-dependency.json", "provenance.json"),
        ("invalid-provenance-started-on.json", "provenance.json"),
        ("invalid-provenance-timestamp.json", "provenance.json"),
    ] {
        let temporary = staged_root()?;
        let fixture = temporary
            .path()
            .join("qualification/fixtures/external-contracts/negative")
            .join(fixture_name);
        let target = temporary
            .path()
            .join("qualification/fixtures/external-contracts/positive")
            .join(target_name);
        fs::copy(fixture, target)?;

        let error = verify_at(temporary.path())
            .err()
            .ok_or("mutated semantic fixture must fail")?;
        assert!(matches!(error, ExternalContractsError::Fixture { .. }));
    }
    Ok(())
}

#[test]
fn invalid_spdx_documents_fail_the_authoritative_schema_or_context_check()
-> Result<(), Box<dyn Error>> {
    for (fixture_name, expected_context_error) in [
        ("empty-spdx-creation-info.json", false),
        ("wrong-spdx-context.json", true),
        ("unknown-spdx-type.json", false),
    ] {
        let temporary = staged_root()?;
        let fixture = temporary
            .path()
            .join("qualification/fixtures/external-contracts/negative")
            .join(fixture_name);
        let target = temporary
            .path()
            .join("qualification/fixtures/external-contracts/positive/spdx-document.json");
        fs::copy(fixture, target)?;

        let error = verify_at(temporary.path())
            .err()
            .ok_or("invalid SPDX document must fail")?;
        if expected_context_error {
            assert!(matches!(error, ExternalContractsError::SpdxContext { .. }));
        } else {
            assert!(matches!(error, ExternalContractsError::SpdxSchema { .. }));
        }
    }
    Ok(())
}

#[test]
fn proposal_failures_report_the_mutated_condition_without_touching_active_lock()
-> Result<(), Box<dyn Error>> {
    for (fixture_name, expected_code) in [
        (
            "wrong-registry-digest.json",
            ProposalCode::RegistryDigestMismatch,
        ),
        (
            "incomplete-proposed-lock.json",
            ProposalCode::MissingDsseEnvelopeEntry,
        ),
    ] {
        let temporary = staged_root()?;
        let active_lock = temporary
            .path()
            .join(".constitution/tech-spec/contracts/external-contract-lock.json");
        let before = fs::read(&active_lock)?;
        let fixture = temporary
            .path()
            .join("qualification/fixtures/external-contracts/negative")
            .join(fixture_name);
        fs::copy(fixture, temporary.path().join(PROPOSAL_PATH))?;

        let error = verify_at(temporary.path())
            .err()
            .ok_or("invalid proposal fixture must fail")?;
        assert!(
            matches!(error, ExternalContractsError::Proposal { code, .. } if code == expected_code),
            "unexpected proposal failure: {error:?}"
        );
        assert!(matches!(
            outcome_at(temporary.path()),
            CommandOutcome::Failed(_)
        ));
        assert_eq!(fs::read(active_lock)?, before);
    }
    Ok(())
}

#[test]
fn proposal_negative_fixtures_have_one_semantic_mutation() -> Result<(), Box<dyn Error>> {
    let root = workspace_root().map_err(|_| "xtask must remain below the workspace root")?;
    let proposal = read_json(&root.join(PROPOSAL_PATH))?;

    let mut incomplete = proposal.clone();
    incomplete
        .get_mut("contracts")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or("proposal contracts must be an object")?
        .remove("dsse-envelope-v1");
    assert_eq!(
        read_json(&root.join(
            "qualification/fixtures/external-contracts/negative/incomplete-proposed-lock.json"
        ))?,
        incomplete
    );

    let mut wrong_digest = proposal;
    *wrong_digest
        .get_mut("contracts")
        .and_then(serde_json::Value::as_object_mut)
        .and_then(|contracts| contracts.get_mut("spdx-3.0.1"))
        .and_then(serde_json::Value::as_object_mut)
        .and_then(|contract| contract.get_mut("sha256"))
        .ok_or("SPDX proposal digest must be present")? = serde_json::Value::String(
        "0000000000000000000000000000000000000000000000000000000000000000".to_owned(),
    );
    assert_eq!(
        read_json(&root.join(
            "qualification/fixtures/external-contracts/negative/wrong-registry-digest.json"
        ))?,
        wrong_digest
    );
    Ok(())
}

#[test]
fn dsse_pae_matches_the_pinned_protocol_example() -> Result<(), Box<dyn Error>> {
    let pae = dsse_pae(b"http://example.com/HelloWorld", b"hello world")?;
    assert_eq!(
        pae,
        b"DSSEv1 29 http://example.com/HelloWorld 11 hello world"
    );
    Ok(())
}

#[test]
fn base64_decoder_requires_the_dsse_standard_padded_alphabet() {
    assert_eq!(decode_base64("AA=="), Some(vec![0]));
    assert_eq!(decode_base64("+/8="), Some(vec![251, 255]));
    assert_eq!(decode_base64("-_8="), None);
    assert_eq!(decode_base64("AA"), None);
    assert_eq!(decode_base64("A"), None);
    assert_eq!(decode_base64("A=AA"), None);
}

#[test]
fn published_snapshot_requires_a_digest_verified_source_url_reference() -> Result<(), Box<dyn Error>>
{
    let temporary = TemporaryDirectory::new()?;
    let source_path = temporary.path().join("publication.source");
    let metadata_path = temporary.path().join("source.json");
    let retrieval_url = "https://spdx.org/schema/3.0.1/spdx-json-schema.json";

    fs::write(&source_path, format!("published at {retrieval_url}\n"))?;
    let digest = hash_file(&source_path)?.to_string();
    verify_publication_source_bytes(&source_path, &digest, retrieval_url, &metadata_path)?;

    fs::write(&source_path, "published source without the schema URL\n")?;
    let digest = hash_file(&source_path)?.to_string();
    let error =
        verify_publication_source_bytes(&source_path, &digest, retrieval_url, &metadata_path)
            .err()
            .ok_or("a source without the published retrieval URL must fail")?;
    assert!(matches!(
        error,
        ExternalContractsError::SourceIdentity { .. }
    ));
    Ok(())
}

#[test]
fn published_spdx_schema_rejects_a_fabricated_commit_pin() -> Result<(), Box<dyn Error>> {
    let temporary = staged_root()?;
    let path = temporary
        .path()
        .join("qualification/schemas/external/spdx-3.0.1/schema/source.json");
    let mut metadata = read_json(&path)?;
    let object = metadata
        .as_object_mut()
        .ok_or("SPDX source metadata must be an object")?;
    object.insert(
        "repository".to_owned(),
        serde_json::Value::String("https://github.com/spdx/spdx-spec".to_owned()),
    );
    object.insert(
        "commit".to_owned(),
        serde_json::Value::String("61a649da8ca27924ac1ca8d2a061cb228839b24c".to_owned()),
    );
    object.insert(
        "path".to_owned(),
        serde_json::Value::String("schema/3.0.1/spdx-json-schema.json".to_owned()),
    );
    fs::write(
        path,
        format!("{}\n", serde_json::to_string_pretty(&metadata)?),
    )?;

    let error = verify_at(temporary.path())
        .err()
        .ok_or("a fabricated SPDX pin must fail")?;
    assert!(matches!(
        error,
        ExternalContractsError::SourceIdentity { .. }
    ));
    Ok(())
}

fn staged_root() -> Result<TemporaryDirectory, Box<dyn Error>> {
    let source = workspace_root().map_err(|_| "xtask must remain below the workspace root")?;
    let temporary = TemporaryDirectory::new()?;
    copy_directory(
        &source.join("qualification"),
        &temporary.path().join("qualification"),
    )?;
    copy_directory(
        &source.join(".constitution/tech-spec/data-models"),
        &temporary.path().join(".constitution/tech-spec/data-models"),
    )?;
    copy_directory(
        &source.join(".constitution/tech-spec/contracts"),
        &temporary.path().join(".constitution/tech-spec/contracts"),
    )?;
    Ok(temporary)
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
