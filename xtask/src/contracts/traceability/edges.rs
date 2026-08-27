//! Required capability-to-contract and capability-to-symbol edge tables.

use super::*;

pub(super) fn validate_required_symbol_edges(
    traceability: &Value,
) -> Result<(), TraceabilityError> {
    for (capability, contract, symbol) in REQUIRED_SYMBOL_EDGES {
        let mapping = array_field(traceability, "mappings")?
            .iter()
            .find(|mapping| {
                mapping.get("capabilityId").and_then(Value::as_str) == Some(*capability)
            })
            .ok_or_else(|| error("edge-matrix-capability"))?;
        let has_symbol = array_field(mapping, "bindings")?.iter().any(|binding| {
            binding.get("contract").and_then(Value::as_str) == Some(*contract)
                && binding
                    .get("symbols")
                    .and_then(Value::as_array)
                    .is_some_and(|symbols| {
                        symbols.iter().any(|item| item.as_str() == Some(*symbol))
                    })
        });
        if !has_symbol {
            return fail("capability-symbol-edge");
        }
    }
    Ok(())
}

pub(super) const EXPECTED_CONSTRAINTS: [&str; CONSTRAINT_COUNT] = [
    "CON-PERF-001",
    "CON-PERF-002",
    "CON-PERF-003",
    "CON-MEM-001",
    "CON-SIZE-001",
    "CON-SIZE-002",
    "CON-FRM-001",
    "CON-FRM-002",
    "CON-REC-001",
    "CON-REC-002",
    "CON-REC-003",
    "CON-REC-004",
    "CON-REC-005",
    "CON-REC-006",
    "CON-REC-007",
    "CON-DET-001",
    "CON-DET-002",
    "CON-UPG-001",
    "CON-COMP-001",
    "CON-SAFE-001",
    "CON-SEC-001",
    "CON-SEC-002",
    "CON-SEC-003",
    "CON-PRV-001",
    "CON-DIA-001",
    "CON-DST-001",
    "CON-LIC-001",
];

pub(super) const REQUIRED_ACCESSIBILITY_CATEGORIES: &[&str] = &[
    "roles",
    "states",
    "actions",
    "values",
    "labels",
    "accessibleNames",
    "descriptions",
    "hints",
    "helpOrFullDescriptions",
    "tooltips",
    "attributedText",
    "identifiers",
    "bounds",
    "transforms",
    "traversal",
    "labelledByRelations",
    "describedByRelations",
    "roleApplicableRelations",
    "accessibilityFocus",
    "inputFocus",
    "hitTesting",
    "textRanges",
    "selection",
    "scrollExtents",
    "language",
    "direction",
    "headingLevels",
    "liveRegions",
    "hidden",
    "disabled",
    "secureFieldRedaction",
    "multiViewIsolation",
];

const REQUIRED_SYMBOL_EDGES: &[(&str, &str, &str)] = &[
    (
        "CAP-REN-002",
        "contracts/oxyflut-public.rs",
        "Canvas::draw_texture",
    ),
    (
        "CAP-REN-002",
        "contracts/oxyflut-substrate.rs",
        "SceneBuilder::draw_texture",
    ),
    (
        "CAP-REN-002",
        "contracts/oxyflut-substrate.h",
        "OxySubstrateApi.scene_builder_draw_texture",
    ),
    (
        "CAP-SEM-002",
        "contracts/oxyflut-public.rs",
        "SemanticsBridge::perform_action",
    ),
    (
        "CAP-SEM-002",
        "contracts/oxyflut-substrate.rs",
        "SubstrateEvents::semantics_action",
    ),
    (
        "CAP-SEM-002",
        "contracts/oxyflut-substrate.rs",
        "SubstrateAdapter::respond_semantics_action",
    ),
    (
        "CAP-SEM-002",
        "contracts/oxyflut-substrate.h",
        "OxySubstrateCallbacks.on_semantics_action",
    ),
    (
        "CAP-SEM-002",
        "contracts/oxyflut-substrate.h",
        "OxySubstrateApi.respond_semantics_action",
    ),
];

pub(super) fn edge_matrix(capability: &str) -> Option<&'static [&'static str]> {
    CAPABILITY_CONTRACT_EDGES
        .iter()
        .find(|(id, _)| *id == capability)
        .map(|(_, contracts)| *contracts)
}

const CAPABILITY_CONTRACT_EDGES: &[(&str, &[&str])] = &[
    ("CAP-CMP-001", &["contracts/oxyflut-public.rs"]),
    ("CAP-CMP-002", &["contracts/oxyflut-public.rs"]),
    ("CAP-CMP-003", &["contracts/oxyflut-public.rs"]),
    ("CAP-CMP-004", &["contracts/oxyflut-public.rs"]),
    ("CAP-CMP-005", &["contracts/oxyflut-public.rs"]),
    ("CAP-CMP-006", &["contracts/oxyflut-public.rs"]),
    ("CAP-CMP-007", &["contracts/oxyflut-public.rs"]),
    ("CAP-LAY-001", &["contracts/oxyflut-public.rs"]),
    ("CAP-LAY-002", &["contracts/oxyflut-public.rs"]),
    ("CAP-SCR-001", &["contracts/oxyflut-public.rs"]),
    ("CAP-SCR-002", &["contracts/oxyflut-public.rs"]),
    (
        "CAP-REN-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-REN-002",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-REN-003",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    ("CAP-AST-001", &["contracts/oxyflut-public.rs"]),
    ("CAP-AST-002", &["contracts/oxyflut-public.rs"]),
    ("CAP-AST-003", &["contracts/oxyflut-public.rs"]),
    (
        "CAP-AST-004",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-VIEW-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-VIEW-002",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-VIEW-003",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-VIEW-004",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-VIEW-005",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-REC-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    ("CAP-INP-001", &["contracts/oxyflut-public.rs"]),
    ("CAP-INP-002", &["contracts/oxyflut-public.rs"]),
    ("CAP-FOC-001", &["contracts/oxyflut-public.rs"]),
    (
        "CAP-TXT-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-TXT-002",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-TXT-003",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-IME-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-CLP-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-I18N-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-SEM-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
            "contracts/platform-contracts.json",
            "data-models/accessibility-map.schema.json",
        ],
    ),
    (
        "CAP-SEM-002",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/oxyflut-substrate.rs",
            "contracts/oxyflut-substrate.h",
            "contracts/platform-contracts.json",
            "data-models/accessibility-map.schema.json",
        ],
    ),
    (
        "CAP-PLT-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/platform-contracts.json",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-OS-001",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/platform-contracts.json",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-OS-002",
        &[
            "contracts/oxyflut-public.rs",
            "contracts/platform-contracts.json",
            "contracts/oxyflut-substrate.h",
        ],
    ),
    (
        "CAP-TST-001",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "data-models/qualification-evidence.schema.json",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-TST-002",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "data-models/qualification-evidence.schema.json",
        ],
    ),
    (
        "CAP-TST-003",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "data-models/qualification-evidence.schema.json",
        ],
    ),
    (
        "CAP-TST-004",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "data-models/qualification-evidence.schema.json",
        ],
    ),
    (
        "CAP-DST-001",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "data-models/artifact-manifest.schema.json",
            "data-models/qualification-evidence.schema.json",
            "data-models/release-evidence-bundle.schema.json",
            "data-models/ci-invocation.schema.json",
        ],
    ),
    (
        "CAP-SEC-001",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "data-models/ingress-inventory.schema.json",
            "data-models/qualification-evidence.schema.json",
        ],
    ),
    (
        "CAP-DIA-001",
        &[
            "contracts/diagnostic-event-registry.json",
            "data-models/diagnostic-event.schema.json",
        ],
    ),
    (
        "CAP-DIA-002",
        &[
            "contracts/diagnostic-event-registry.json",
            "data-models/diagnostic-event.schema.json",
        ],
    ),
    (
        "CAP-DIA-003",
        &[
            "contracts/diagnostic-event-registry.json",
            "data-models/diagnostic-event.schema.json",
        ],
    ),
    (
        "CAP-DIA-004",
        &[
            "contracts/diagnostic-event-registry.json",
            "data-models/diagnostic-event.schema.json",
            "contracts/oxyflut-public.rs",
        ],
    ),
    (
        "CAP-SUB-001",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "contracts/specification-phase.json",
            "data-models/qualification-evidence.schema.json",
        ],
    ),
    (
        "CAP-SUB-002",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "contracts/specification-phase.json",
            "data-models/qualification-evidence.schema.json",
            "contracts/platform-contracts.json",
        ],
    ),
    (
        "CAP-SUB-003",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "contracts/specification-phase.json",
            "data-models/qualification-evidence.schema.json",
            "data-models/selection-decision.schema.json",
        ],
    ),
    (
        "CAP-SUB-004",
        &[
            "contracts/oxyflut-qualification.rs",
            "contracts/qualification-lock.json",
            "contracts/specification-phase.json",
            "data-models/qualification-evidence.schema.json",
            "data-models/selection-decision.schema.json",
        ],
    ),
];
