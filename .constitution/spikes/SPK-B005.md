# Spike report: OXY-B005 common-case layout visit cap

## Time box

- **Budget:** 1 focused day.
- **Clock start / stop:** 2026-08-28T16:22:22Z / 2026-08-28T16:32:32Z.

## Question

The table answers the decision question: what platform-independent ordinary-layout corpus, counting rule, and finite per-node cap can be frozen before candidate implementation without counting intrinsic measurement or text work as ordinary visits?

Table 1. Decision questions and evidence

| Row | Question | Answer and evidence | Status | Next bounded probe |
| :-- | :-- | :-- | :-- | :-- |
| 1 | Can the corpus define deep, wide, nested, virtualized, reordered, and failure cases without a substrate? | Yes. The canonical manifest and the [preserved counter-model output](#probe-record) define 10 exact fixtures, their topology, and all derived counters. The model has no substrate API dependency. | KK | Not applicable. |
| 2 | What is one ordinary visit? | One completed parent-issued regular child-layout invocation for one realized child in one root transaction. The root entry, rejected attempt, intrinsic query, and text operation are not ordinary visits. Flutter [documents `layout`](https://api.flutter.dev/flutter/rendering/RenderObject/layout.html) as the parent-to-child layout entry point, and the [preserved output](#probe-record) validates this counter rule. | KK | Not applicable. |
| 3 | Do the ordinary policy families have finite bounds under the proposed classifier? | Yes. Single-pass box, definite-basis weighted, and realized virtualized policies each permit one visit per direct child. Custom multi-pass policies permit two. The [six passing fixtures](#probe-record) validate the derived bounds. | KK | Not applicable. |
| 4 | Can intrinsic or dry measurement consume an ordinary-policy visit? | No. Flutter [defines dry layout](https://api.flutter.dev/flutter/rendering/RenderBox/getDryLayout.html) as a distinct state-free calculation and warns that it can produce O(N^2) behavior. The intrinsic fixture reports `ordinary=0` and `intrinsic=1`, then rejects the fixture from the ordinary family. | not applicable-with-citation | Not applicable. |
| 5 | Can text shaping or text layout consume an ordinary-policy visit? | No. [Yoga identifies text and externally laid-out views as measure-function work](https://www.yogalayout.dev/docs/advanced/external-layout-systems). The text fixture reports one ordinary child layout and one separate text operation. | not applicable-with-citation | Not applicable. |
| 6 | Can `2` freeze as `measurementPolicy.layoutVisitCap` while establishing compatibility with the 2.0 ms aggregate goal? | No. The counter model establishes counts, not nanosecond cost. The qualification lock has no reference workload, release flags, hardware identifiers, or candidate source identities. No preserved result measures layout plus paint submission, so a numeric freeze would guess at CON-PERF-001. | KU (gating) | After Stage 3 authorizes nonproduction candidate probes, run the bounded timing probe defined in "Next bounded probe" on every locked reference configuration. |

## Context and objective

- **Triggering upstream file or section:** `.constitution/prd/constraints.md` defines the gating common-case node-visit limit, and `.constitution/tech-spec/contracts/qualification-lock.json` has `measurementPolicy.layoutVisitCap: null`.
- **Target:** Freeze the corpus and counter semantics, then either freeze a numeric cap from performance evidence or retain a precise blocker.
- **Archetype / surface:** Library and SDK layout policy under system and built-in frame constraints.

## Codebase baseline

- **Status at probe start:** `LayoutResult.node_visits` reports participating-node visits made by a policy, CAP-LAY-001 requires bounded constraint propagation, and CON-PERF-001 limits aggregate application-owned layout and paint submission to 2.0 ms.
- **Discovered constraints:** The `CAP-LAY-001` flow rejects a policy that exceeds its declared cap. The `CAP-LAY-002` flow also stops custom policies that exceed a cap. The lock keeps `layout-visit-cap` in both known-unknown lists.
- **Boundary:** This report specifies a qualification counter and corpus. It doesn't select a substrate, implement layout, change a capability, or relax CON-PERF-001.

## Reference corpus

The root is included in each node total, has depth 1, and is the harness-initiated transaction entry. It is not a child visit. Every nontext ordinary leaf has a fixed finite size. Every ordinary container receives valid finite constraints. A weighted fixture uses explicit finite weights and a definite main-axis size, so it has no content-derived basis.

The canonical corpus manifest has SHA-256 `b671d11c256a8e65be12313e415c96f6fa83cb701b0007544783dde0341a663e`:

```json
[
  {
    "collection": null,
    "depth": 64,
    "family": "single-pass-box",
    "id": "deep-box-064",
    "nodes": 64,
    "passes": 1
  },
  {
    "collection": null,
    "depth": 2,
    "family": "single-pass-box",
    "id": "wide-box-1024",
    "nodes": 1025,
    "passes": 1
  },
  {
    "collection": null,
    "depth": 4,
    "family": "weighted",
    "id": "nested-weighted-8x8x8",
    "nodes": 585,
    "passes": 1
  },
  {
    "collection": 10000,
    "depth": 2,
    "family": "virtualized-lazy",
    "id": "lazy-10000-realized-64",
    "nodes": 65,
    "passes": 1
  },
  {
    "collection": null,
    "depth": 2,
    "family": "weighted",
    "id": "reordered-keyed-128",
    "nodes": 129,
    "passes": 1
  },
  {
    "collection": null,
    "depth": 2,
    "family": "custom-multi-pass",
    "id": "custom-two-pass-256",
    "nodes": 257,
    "passes": 2
  }
]
```

Table 2. Ordinary success corpus

| Fixture | Exact topology and operation | Nodes | Depth | Ordinary visits | Intrinsic queries | Text operations | Maximum visits per node |
| :-- | :-- | --: | --: | --: | --: | --: | --: |
| `deep-box-064` | One root and 63 one-child boxes. | 64 | 64 | 63 | 0 | 0 | 1 |
| `wide-box-1024` | One root and 1,024 fixed leaf children. | 1,025 | 2 | 1,024 | 0 | 0 | 1 |
| `nested-weighted-8x8x8` | One root, eight weighted columns, 64 weighted rows, and 512 fixed leaves. | 585 | 4 | 584 | 0 | 0 | 1 |
| `lazy-10000-realized-64` | A 10,000-item collection realizes `[4968,5032)`: 32 visible items and 16 cached items on each side. The root and 64 realized fixed leaves form the layout tree. | 65 realized | 2 | 64 | 0 | 0 | 1 |
| `reordered-keyed-128` | One weighted root reverses 128 fixed keyed leaves from `key-000` through `key-127` to `key-127` through `key-000`, then lays out the realized tree. | 129 | 2 | 128 | 0 | 0 | 1 |
| `custom-two-pass-256` | One custom root issues two declared constraint passes to 256 fixed leaves. | 257 | 2 | 512 | 0 | 0 | 2 |

The lazy fixture must issue no child-layout request for the 9,936 unrealized collection items. Collection indexing and range selection are not ordinary visits and need separate timing evidence under CAP-SCR-001 and CON-PERF-001.

Table 3. Failure and separation fixtures

| Fixture | Exact operation | Nodes | Depth | Ordinary visits | Other work | Expected result |
| :-- | :-- | --: | --: | --: | --: | :-- |
| `three-pass-cap-failure` | One custom root asks each of 16 fixed children for a third layout after two completed passes. | 17 | 2 | 32 completed, 33 attempted | 0 intrinsic, 0 text | Reject the third attempted visit to child 1 before invocation. No `LayoutResult` succeeds. |
| `invalid-constraints` | One box root receives invalid constraints before it can lay out 16 fixed children. | 17 | 2 | 0 | 0 intrinsic, 0 text | Reject before a child-layout request. |
| `intrinsic-separation` | One root requests a dry or intrinsic answer from one nondefinite child. | 2 | 2 | 0 | 1 intrinsic query | Reject the fixture from the ordinary family. Record the query only in the intrinsic counter. |
| `text-separation` | One box root lays out one realized text leaf, which invokes text layout. | 2 | 2 | 1 | 1 text operation | Keep the parent-to-leaf layout visit and text operation in separate counters. |

## Counting model

Apply this algorithm to one root layout transaction:

1. Validate the root constraints before issuing a child request. If validation fails, return a structured layout failure with zero ordinary visits.
2. Classify each requested operation before it runs. An ordinary operation is a regular `layout` request from a policy to a realized direct child. A dry or intrinsic request and text-engine work have separate counters.
3. Before invoking an ordinary child, compare that child's completed ordinary-visit count for this transaction with the declaring policy-family cap.
4. If the count equals the cap, increment `attempted_ordinary_visits`, reject the request before invocation, and return a structured cap failure. Do not add the rejected attempt to `node_visits`.
5. Otherwise invoke the child, increment that child's completed count and the issuing policy's `LayoutResult.node_visits`, then continue traversal.
6. At transaction end, the harness records the sum of the emitted ordinary-visit events without recursively summing nested `LayoutResult` values. This prevents double counting.

Table 4. Proposed ordinary-policy classifier

| Policy family | Admission rule | Cap per realized direct child | Excluded work |
| :-- | :-- | --: | :-- |
| Single-pass box | The policy issues one regular request to each participating child under valid finite constraints. | 1 | Intrinsic queries and text operations. |
| Definite-basis weighted | Weights, minimums, maximums, and main-axis space are finite and explicit. The policy issues one regular request after allocation. | 1 | Content-derived bases, dry or intrinsic measurement, and text operations. |
| Virtualized or lazy | The viewport has a declared realized range. Only realized children receive regular requests. | 1 | Offscreen collection work, range selection, intrinsic measurement, and text operations. |
| Custom multi-pass | The registered policy declares at most two regular passes and the harness checks each request before invocation. | 2 | Convergence loops beyond two passes, dry or intrinsic measurement, and text operations. |

A policy that needs content-derived sizing, a dry query, an intrinsic query, text work, or more than two ordinary passes isn't an ordinary fixture. It must enter its dedicated evidence suite and cannot make its work disappear into `node_visits`.

## Probe record

I ran the nonproduction Perl counter model at `/tmp/wf-epic-b/OXY-B005/layout_visit_model.pl`. It validates the arithmetic and cap rejection rules only. It doesn't measure a layout engine or frame time.

The output is similar to the following:

```text
OXY-B005 candidate-neutral counter model
cap=2
corpus_sha256=b671d11c256a8e65be12313e415c96f6fa83cb701b0007544783dde0341a663e
fixture|family|nodes|depth|ordinary|intrinsic|text|max_per_node|result
deep-box-064|single-pass-box|64|64|63|0|0|1|pass
wide-box-1024|single-pass-box|1025|2|1024|0|0|1|pass
nested-weighted-8x8x8|weighted|585|4|584|0|0|1|pass
lazy-10000-realized-64|virtualized-lazy|65|2|64|0|0|1|pass
reordered-keyed-128|weighted|129|2|128|0|0|1|pass
custom-two-pass-256|custom-multi-pass|257|2|512|0|0|2|pass
three-pass-cap-failure|custom-multi-pass|17|2|32|0|0|2|reject node=1 attempted=33
invalid-constraints|single-pass-box|17|2|0|0|0|0|reject-before-child-layout
intrinsic-separation|intrinsic-measure|2|2|0|1|0|0|reject-from-ordinary-family
text-separation|text|2|2|1|0|1|1|separate-counter
assertions=passed
```

The SHA-256 of the executed probe script is `3696d54aaea82c077b7c7670d69f38ce411c0ca1bf345cc4a944b0d6b4a9e33b`.

## Reference algorithm comparison

Flutter [documents `layout`](https://api.flutter.dev/flutter/rendering/RenderObject/layout.html) as a parent request for child layout and says parents call it for all children. This supports counting the parent-issued request, not a geometry calculation or paint operation. Flutter [documents dry layout](https://api.flutter.dev/flutter/rendering/RenderBox/getDryLayout.html) as state-free and potentially O(N^2), which requires a separate intrinsic meter rather than an ordinary-visit exemption.

The [CSS Flexible Box Layout specification](https://www.w3.org/TR/css-flexbox-1/#layout-algorithm) defines a normative flex algorithm, uses order-modified document order, and includes intrinsic-size branches. The weighted corpus avoids those branches by requiring explicit definite inputs. It is a qualification counter model, not a claim that an Oxyflut policy implements the CSS algorithm.

[Yoga documents measure functions](https://www.yogalayout.dev/docs/advanced/external-layout-systems) for text and externally laid-out content when it can't express the size. This supports a separate text counter. The ordinary text-leaf visit remains visible, but its text operation doesn't become another ordinary visit.

## Options and trade-offs

- Option A: Freeze the corpus, counting algorithm, and per-family algebraic bounds. This result supports Option A for counter semantics only.
- Option B: Freeze one global cap for every ordinary policy. The existing lock can store only one integer, but no timing evidence supports freezing `2` as that integer.
- Option C: Retain the numeric cap as a gating KU until an instrumented candidate probe measures the frozen corpus against CON-PERF-001. This preserves the target and prevents an intuition-based number.

## Recommendation

- **Chosen option:** Use a mix of A and C. Freeze the corpus digest and counting rules from this report. Choose C for the numeric `layoutVisitCap`; retain it as `null` and gating.
- **Derived threshold, not a freeze:** The corpus demonstrates that `2` is the smallest global threshold that admits the declared custom two-pass fixture. It isn't a performance recommendation because no result assigns time to a visit or reserves time for paint submission.
- **Why it fits:** The result preserves CAP-LAY-001's bounded propagation, keeps intrinsic and text work explicit, and doesn't weaken CON-PERF-001. The widest passing fixture has 1,024 ordinary visits, which gives an all-layout arithmetic ceiling of 1.953125 microseconds per visit under 2.0 ms. That ceiling leaves no measured paint allowance and isn't performance evidence.
- **Rejected options:** Reject a timing-only rule, an average-count rule, an unbounded intrinsic recursion, a text-work exemption hidden in `node_visits`, and a numeric threshold selected from shallow scenes.

### Next bounded probe

Stage 3 must first authorize unscored, nonproduction candidate probes before `candidateImplementationReady` changes. On each of the four locked reference configurations, run both instrumented candidate prototypes with `CAP_CANDIDATE=2` across the six success fixtures in table 2. For each fixture and prototype, run 20 launches, discard 300 warmup frames, and record 500 measured frames per launch.

Each raw record must contain the corpus digest, fixture ID, candidate source identity, hardware and driver identity, release flags, `ordinary_visits`, `attempted_ordinary_visits`, `intrinsic_queries`, `text_operations`, application-owned layout nanoseconds, paint-submission nanoseconds, and their aggregate. The expected successful output has the table 2 counts, zero intrinsic and text counters for ordinary fixtures, no cap rejection, and a maximum of the 20 per-launch nearest-rank 99th percentiles at or below 2.0 ms. A failed value retains the KU and rejects the candidate; it doesn't increase the cap or change the corpus.

Without the Stage 3 authorization, the required timing result is circular: candidate implementation needs the numeric cap, and numeric compatibility needs candidate layout and paint-submission code. A host-only counter model cannot close that evidence gap.

## Downstream impact

- **ADRs to write or update:** None. This report doesn't change an architecture decision.
- **Tickets unblocked in `tasks/active/`:** None. `OXY-D001` remains blocked by `layout-visit-cap`.
- **Tickets to add or split:** Add one bounded prequalification layout-cost prototype ticket only after Stage 3 authorizes the probe described in "Next bounded probe."
- **Spec edits required:** Stage 3 must apply the following exact edits without setting a numeric cap.
  - `.constitution/tech-spec/data-models/qualification-lock.schema.json` in `$defs.measurementPolicy.required`: add `layoutVisitCorpus`.
  - `.constitution/tech-spec/data-models/qualification-lock.schema.json` in `$defs.measurementPolicy.properties`: add `"layoutVisitCorpus": { "$ref": "#/$defs/digestOrNull" }`.
  - `.constitution/tech-spec/data-models/qualification-lock.schema.json` in `$defs.resolvedMeasurementPolicy.properties`: add `"layoutVisitCorpus": { "$ref": "#/$defs/sha256" }`.
  - `.constitution/tech-spec/contracts/qualification-lock.json` in `measurementPolicy`: add `"layoutVisitCorpus": "b671d11c256a8e65be12313e415c96f6fa83cb701b0007544783dde0341a663e"` and retain `"layoutVisitCap": null`.
  - `.constitution/tech-spec/contracts/oxyflut-public.rs` in the `LayoutResult.node_visits` documentation: replace the field description with `Number of completed ordinary direct-child layout invocations issued by this policy; excludes the root entry, dry or intrinsic measurements, text operations, and rejected attempts.`
  - `.constitution/tech-spec/stack.md` in the Scope guard paragraph that limits Stage 4 before `candidateImplementationReady`: append `Before candidateImplementationReady becomes true, Stage 4 may run unscored nonproduction candidate probes only to resolve a pre-implementation gating KU; the probes must use the frozen evidence contract and can't produce comparative scores or select a candidate.`

## Sources

All sources in this list were fetched successfully through the Jina reader proxy during this spike.

- [Flutter `RenderObject.layout` API documentation](https://api.flutter.dev/flutter/rendering/RenderObject/layout.html)
- [Flutter `RenderBox.getDryLayout` API documentation](https://api.flutter.dev/flutter/rendering/RenderBox/getDryLayout.html)
- [CSS Flexible Box Layout Module Level 1](https://www.w3.org/TR/css-flexbox-1/#layout-algorithm)
- [Yoga: Integrating with external layout systems](https://www.yogalayout.dev/docs/advanced/external-layout-systems)
