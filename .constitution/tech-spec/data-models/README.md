# Durable qualification data

Oxyflut has no database in Phase 3A. The JSON Schema files in this directory govern durable or exchanged qualification data.

| Schema | Purpose | Compatibility rule |
| :-- | :-- | :-- |
| `accessibility-map.schema.json` | Records one candidate's complete forward property map and reverse action map for one environment. | Any native mapping change reruns API inspection and end-to-end assistive-technology tests. |
| `artifact-manifest.schema.json` | Records canonical release payload files, hashes, modes, links, and licenses. | Preserve old manifests; breaking changes require a major schema version. |
| `capability-baseline.schema.json` | Freezes the exact test vectors and evidence expected for all 52 P0 capabilities. | A baseline change creates a new lock and prevents comparison with the old corpus. |
| `capability-traceability.schema.json` | Maps every P0 capability to its architecture flow, target crate, and physical contracts. | Every Stage 3 revision must keep exactly one mapping per P0 capability. |
| `ci-invocation.schema.json` | Records the project-owned build command, builder, materials, timing, and outputs. | Reproducibility comparison requires independently produced records bound to one lock. |
| `diagnostic-event-registry.schema.json` | Freezes stable event names, classifications, and field contracts. | Event additions require registry review; field meaning changes require a major version. |
| `diagnostic-event.schema.json` | Defines durable machine-local diagnostic records. | Additive classified fields can use a minor version; private raw content is always forbidden. |
| `external-contract-lock.schema.json` | Pins local snapshots and verifiers for SPDX, in-toto, SLSA, and DSSE. | Network resolution is forbidden; a source or digest change creates a new qualification lock. |
| `ingress-inventory.schema.json` | Records candidate-specific ingresses, owners, limits, validation, privacy, and evidence. | A missing or changed ingress requires renewed security qualification. |
| `platform-contracts.schema.json` | Records the platform mechanisms used by each qualification allocation. | A mechanism change requires the full affected Tier 1 row. |
| `qualification-evidence.schema.json` | Records capability and constraint results, eligibility, and consensus scores. | Preserve original evidence; corrected evidence must reference a new lock digest. |
| `qualification-lock.schema.json` | Pins source, tools, environments, and measurement readiness. | Any changed pin creates a new lock and invalidates cross-lock comparisons. |
| `raw-measurement.schema.json` | Preserves every admitted or excluded raw sample and harness log. | Never delete outliers; exclusions use only the three predeclared categories. |
| `release-evidence-bundle.schema.json` | Binds the manifest, SBOM, notices, provenance, DSSE envelope, and CI invocation. | Every member is immutable and bound by SHA-256 to one external-contract lock. |
| `selection-decision.schema.json` | Records zero-, one-, and two-eligible-candidate outcomes and the maintenance tie-break. | Recompute the outcome from immutable candidate evidence; never edit a decision in place. |
| `specification-phase.schema.json` | Prevents Phase 3A from authorizing production planning. | Only Phase 3B can set production readiness after satisfying its conditional rules. |

All JSON uses UTF-8. Validation disables network resolution. Evidence migration creates a derived document and preserves the source bytes and digest.

Schema validation is necessary but not sufficient. The `xtask contracts validate` command must resolve only the local schema registry and verify file existence and SHA-256 bindings; the exact PRD and architecture ID sets; traceability symbols and contract tests; diagnostic names, registry versions, registered privacy classes, field kinds, bounds, and closed values; unique canonical artifact paths; unique `(constraintId, launch, ordinal)` raw-sample keys; every capability baseline; every external schema snapshot; score arithmetic and assessor consensus; zero-, one-, and two-candidate selection arithmetic; and every Phase 3B promotion reference. Diagnostic event files cannot declare or override privacy metadata. Promotion validation must bind the selected candidate, qualification evidence, decision, layout corpus and result, accepted ADR-0010 bytes, final contract set, target matrix, all-Tier-1 results, losing-candidate removal, production bill of materials, and release qualification to the same qualification lock and Stage 3 version. A schema-valid document with a missing, duplicated, mismatched, or fabricated reference fails validation.

Artifact paths use canonical repository-relative slash separators. They cannot contain drive prefixes, backslashes, null bytes, `.` segments, or `..` segments. Raw-measurement records cannot repeat a `(constraintId, launch, ordinal)` tuple. A valid sample cannot include `exclusionReason`.

Link targets are artifact-root-relative canonical paths. For a regular file or hardlink, `size` and `sha256` describe the referenced content bytes. A hardlink's size and digest must equal its target regular file. For a symlink, they describe the UTF-8 bytes of `linkTarget`, not dereferenced content. Every hardlink target must equal another regular-file path in the same manifest. Every symlink target must resolve within the artifact root. Canonical link targets cannot use absolute paths, drive prefixes, backslashes, null bytes, `.` segments, or `..` segments. A regular file cannot declare `linkTarget`.

The artifact-manifest v3, raw-measurement v2, platform-contracts v3, and qualification-lock v4 identities supersede their earlier pre-evidence contracts. No durable qualification instance was produced under the superseded identities because the implementation workspace and qualification commands don't exist. Git history preserves the earlier schema bytes. OXY-A002 must include old-reader rejection and explicit supersession fixtures; no evidence migration is required until an instance exists.
