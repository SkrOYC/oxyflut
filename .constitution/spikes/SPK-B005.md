# Spike report: OXY-B005 common-case layout visit cap

## Time box

- **Budget:** 1 focused day.
- **Status:** Completed.
- **Clock start / stop:** 2026-08-28T17:49:09Z / 2026-08-28T19:16:28Z.

## Question

This table answers whether a platform-independent ordinary-layout corpus, counting rule, and finite per-node cap can be frozen without classifying intrinsic measurement or text work as ordinary visits.

Table 1. Decision questions and evidence

| Row | Question | Answer and evidence | Status | Next bounded probe |
| :-- | :-- | :-- | :-- | :-- |
| 1 | Can the corpus define deep, wide, nested, virtualized, reordered, and separation or failure cases without a substrate? | Yes. The [preserved topology probe](#probe-record) builds the 10 declared trees, validates node, edge, and depth counts, replays family-specific parent-child events, and validates every declared counter. It also proves that the lazy tree contains only 64 realized children and zero visits to 9,936 unrealized IDs. | KK | Not applicable. |
| 2 | What is one ordinary visit? | One requested regular child-layout invocation from a policy to a realized direct child in one root transaction. The request increments `attempted_ordinary_visits` before the per-child cap check; a completed invocation increments `node_visits`. The [pinned Flutter source](https://raw.githubusercontent.com/flutter/flutter/4cf24164269a5ebf0c16a028a00727d0e77bbb05/packages/flutter/lib/src/rendering/object.dart) calls `layout` the parent entry point for asking children to update layout. The probe applies this rule only to declared parent-child edges. | KK | Not applicable. |
| 3 | Do the ordinary policy families have finite bounds under the classifier? | Yes. The [preserved topology probe](#probe-record) admits and replays single-pass box, weighted with a definite basis, virtualized lazy, and custom multi-pass families. It verifies at most one completed visit per realized child for the first three and at most two for the custom family. The third custom request fails before invocation. | KK | Not applicable. |
| 4 | Can intrinsic or dry measurement consume an ordinary-policy visit? | No. The [pinned Flutter `getDryLayout` source](https://raw.githubusercontent.com/flutter/flutter/4cf24164269a5ebf0c16a028a00727d0e77bbb05/packages/flutter/lib/src/rendering/box.dart) defines dry layout as distinct from wet layout and states that it doesn't change internal state. The probe records one `intrinsic_queries` event and zero ordinary attempts for `intrinsic-separation`. | KK (not applicable) | Not applicable. |
| 5 | Can text shaping or text layout consume an ordinary-policy visit? | No. The [pinned Yoga source](https://raw.githubusercontent.com/facebook/yoga/bd8fe0d6d243cc7e0334d4cc68864a994f63beae/website/docs/advanced/external-layout-systems.mdx) identifies text as content delegated through a measure function to another layout system. The probe records the ordinary parent-to-realized-text-leaf request separately from one `text_operations` event. | KK (not applicable) | Not applicable. |
| 6 | Can `2` freeze as `measurementPolicy.layoutVisitCap` while establishing compatibility with the 2.0 ms aggregate goal? | No. The [preserved topology probe](#probe-record) establishes count semantics, not application-owned layout or paint-submission duration. The current lock has no durable corpus file, frozen prequalification-probe binding for a complete run and suite, record and suite schemas, counting rules, candidate source and artifact, hardware and driver, or release flags. It also permits an unbound sample-validity policy that could exclude measured frames. | KU (gating) | After Stage 3 creates the digest-bound corpus, adds the record, run, and 48-tuple suite contracts, and implements the fixed-frame no-exclusion suite validator, run the bounded timing procedure in "Next bounded probe" on every tuple of the 2 candidates x 4 environments x 6 passing fixtures matrix. |

## Context and objective

- **Triggering upstream file or section:** `.constitution/prd/constraints.md` retains the numeric CAP-LAY-001 common-case node-visit limit as a gating KU, and `.constitution/tech-spec/contracts/qualification-lock.json` sets `measurementPolicy.layoutVisitCap` to `null`.
- **Target:** Freeze a candidate-neutral corpus and counter semantics, then retain the numeric cap as a precise performance gate until a schema-valid timing probe resolves it.
- **Archetype / surface:** Library and SDK layout policy under system and built-in frame constraints.

## Codebase baseline

- **Status at probe start:** `LayoutResult.node_visits` reports a policy counter, CAP-LAY-001 requires bounded constraint propagation, and CON-PERF-001 limits aggregate application-owned layout plus paint submission to 2.0 ms.
- **Discovered constraints:** The public contract has no `attempted_ordinary_visits` field. The qualification contract has only generic `RawSample`, and `raw-measurement.schema.json` rejects extra counter, identity, fixture, and timing fields because its root and sample objects set `additionalProperties` to `false`.
- **Round-4 correction:** The staged `sample-validity.schema.json` declares `authority: "staged-proposal"` and provides only generic exclusion categories and rules. It contains no layout-frame partition, 20-launch, 300-warmup, or 500-measured-frame contract. The probe therefore uses Option B in "Sample-validity policy": no record can self-assert an exclusion.

The following read-only host inspection produced the correction inputs. The digest identifies the staged proposal only; it isn't a prequalification-lock binding.

```text
53462bd5023dfec25fdedfa3737300e587bea0ba118248d91e488430fed2ef59  qualification/schemas/sample-validity.schema.json
sample_validity_relevant_lines
5:  "authority": "staged-proposal",
15:    "exclusionCategories",
16:    "rules"
25:    "exclusionCategories": {
36:    "rules": {
active_version_dependent_paths=56
```

- **Boundary:** This report defines qualification evidence. It doesn't select a substrate, implement layout, change a capability, or relax CAP-LAY-001 or CON-PERF-001.

## Reference corpus

The root is included in each tree's node count at depth 1. It is the harness-initiated transaction entry and isn't a child visit. Every ordinary leaf has a fixed finite size. Every ordinary container has valid finite constraints. A weighted fixture has explicit finite weights, definite main-axis space, and no content-derived basis.

The canonical corpus is the ASCII-only, UTF-8 JSON serialization of `@FIXTURES` in the embedded probe source, with canonical lexicographic object keys, 2-space indentation, LF line endings, and one trailing LF. Its SHA-256 is `4972e43333984047b5a1d84200d5b89a29c5b59e47c5aca8773379320f2c6c84`. Stage 3 must create the durable `qualification/staged/layout-visit-corpus.json` with the exact bytes in "Durable corpus bytes"; this spike doesn't create that repository file. The command and exact output in "Probe record" and the round-5 preservation probe regenerate and hash those bytes.

### Durable corpus bytes

The following fenced JSON is the complete required byte sequence for `qualification/staged/layout-visit-corpus.json`. Serialize it as UTF-8 with the displayed 2-space indentation, LF line endings, no byte-order mark, and exactly one trailing LF after `]`; no formatter, key reorderer, or newline conversion may run before the SHA-256 check.

<!-- canonical-block: layout-visit-corpus -->
<!-- prettier-ignore -->
```text
[
  {
    "collection": null,
    "depth": 64,
    "description": "One root and 63 one-child boxes.",
    "expected": {
      "attempted_ordinary_visits": 63,
      "intrinsic_queries": 0,
      "maximum_ordinary_visits_per_node": 1,
      "ordinary_visits": 63,
      "outcome": "pass",
      "text_operations": 0
    },
    "family": "single-pass-box",
    "id": "deep-box-064",
    "nodes": 64,
    "operation": "regular-child-layout",
    "passes": 1,
    "topology": {
      "fan_out": 1,
      "kind": "chain"
    }
  },
  {
    "collection": null,
    "depth": 2,
    "description": "One root and 1,024 fixed leaf children.",
    "expected": {
      "attempted_ordinary_visits": 1024,
      "intrinsic_queries": 0,
      "maximum_ordinary_visits_per_node": 1,
      "ordinary_visits": 1024,
      "outcome": "pass",
      "text_operations": 0
    },
    "family": "single-pass-box",
    "id": "wide-box-1024",
    "nodes": 1025,
    "operation": "regular-child-layout",
    "passes": 1,
    "topology": {
      "fan_out": 1024,
      "kind": "star"
    }
  },
  {
    "collection": null,
    "depth": 4,
    "description": "One root, eight weighted columns, 64 weighted rows, and 512 fixed leaves.",
    "expected": {
      "attempted_ordinary_visits": 584,
      "intrinsic_queries": 0,
      "maximum_ordinary_visits_per_node": 1,
      "ordinary_visits": 584,
      "outcome": "pass",
      "text_operations": 0
    },
    "family": "weighted",
    "id": "nested-weighted-8x8x8",
    "nodes": 585,
    "operation": "regular-child-layout",
    "passes": 1,
    "topology": {
      "definite_basis": true,
      "fan_out": [
        8,
        8,
        8
      ],
      "kind": "nested"
    }
  },
  {
    "collection": 10000,
    "depth": 2,
    "description": "A 10,000-item collection realizes [4968,5032): 32 visible items and 16 cached items on each side. The root and 64 realized fixed leaves form the layout tree.",
    "expected": {
      "attempted_ordinary_visits": 64,
      "intrinsic_queries": 0,
      "maximum_ordinary_visits_per_node": 1,
      "ordinary_visits": 64,
      "outcome": "pass",
      "text_operations": 0
    },
    "family": "virtualized-lazy",
    "id": "lazy-10000-realized-64",
    "nodes": 65,
    "operation": "regular-child-layout",
    "passes": 1,
    "topology": {
      "kind": "virtualized-star",
      "realized_range": [
        4968,
        5032
      ],
      "visible_range": [
        4984,
        5016
      ]
    }
  },
  {
    "collection": null,
    "depth": 2,
    "description": "One weighted root reverses 128 fixed keyed leaves from key-000 through key-127 to key-127 through key-000, then lays out the realized tree.",
    "expected": {
      "attempted_ordinary_visits": 128,
      "intrinsic_queries": 0,
      "maximum_ordinary_visits_per_node": 1,
      "ordinary_visits": 128,
      "outcome": "pass",
      "text_operations": 0
    },
    "family": "weighted",
    "id": "reordered-keyed-128",
    "nodes": 129,
    "operation": "regular-child-layout",
    "passes": 1,
    "topology": {
      "definite_basis": true,
      "key_count": 128,
      "key_permutation": "reverse",
      "kind": "keyed-star"
    }
  },
  {
    "collection": null,
    "depth": 2,
    "description": "One custom root issues two declared constraint passes to 256 fixed leaves.",
    "expected": {
      "attempted_ordinary_visits": 512,
      "intrinsic_queries": 0,
      "maximum_ordinary_visits_per_node": 2,
      "ordinary_visits": 512,
      "outcome": "pass",
      "text_operations": 0
    },
    "family": "custom-multi-pass",
    "id": "custom-two-pass-256",
    "nodes": 257,
    "operation": "regular-child-layout",
    "passes": 2,
    "topology": {
      "fan_out": 256,
      "kind": "star"
    }
  },
  {
    "collection": null,
    "depth": 2,
    "description": "One custom root asks each of 16 fixed children for a third layout after two completed passes.",
    "expected": {
      "attempted_ordinary_visits": 33,
      "intrinsic_queries": 0,
      "maximum_ordinary_visits_per_node": 2,
      "ordinary_visits": 32,
      "outcome": "reject-cap-before-invocation-leaf-0000",
      "text_operations": 0
    },
    "family": "custom-multi-pass",
    "id": "three-pass-cap-failure",
    "nodes": 17,
    "operation": "regular-child-layout",
    "passes": 3,
    "topology": {
      "fan_out": 16,
      "kind": "star"
    }
  },
  {
    "collection": null,
    "depth": 2,
    "description": "One box root receives invalid constraints before it can lay out 16 fixed children.",
    "expected": {
      "attempted_ordinary_visits": 0,
      "intrinsic_queries": 0,
      "maximum_ordinary_visits_per_node": 0,
      "ordinary_visits": 0,
      "outcome": "reject-before-child-layout",
      "text_operations": 0
    },
    "family": "single-pass-box",
    "id": "invalid-constraints",
    "nodes": 17,
    "operation": "invalid-constraints",
    "passes": 0,
    "topology": {
      "fan_out": 16,
      "kind": "star"
    }
  },
  {
    "collection": null,
    "depth": 2,
    "description": "One root requests a dry or intrinsic answer from one nondefinite child.",
    "expected": {
      "attempted_ordinary_visits": 0,
      "intrinsic_queries": 1,
      "maximum_ordinary_visits_per_node": 0,
      "ordinary_visits": 0,
      "outcome": "reject-from-ordinary-family",
      "text_operations": 0
    },
    "family": "intrinsic-measure",
    "id": "intrinsic-separation",
    "nodes": 2,
    "operation": "dry-or-intrinsic-query",
    "passes": 0,
    "topology": {
      "fan_out": 1,
      "kind": "star"
    }
  },
  {
    "collection": null,
    "depth": 2,
    "description": "One box root lays out one realized text leaf, which invokes text layout.",
    "expected": {
      "attempted_ordinary_visits": 1,
      "intrinsic_queries": 0,
      "maximum_ordinary_visits_per_node": 1,
      "ordinary_visits": 1,
      "outcome": "separate-counter",
      "text_operations": 1
    },
    "family": "text",
    "id": "text-separation",
    "nodes": 2,
    "operation": "text-layout",
    "passes": 1,
    "topology": {
      "fan_out": 1,
      "kind": "star"
    }
  }
]
```

The checkout's existing staged convention appears in `qualification/fixtures/readiness/staged/` as one JSON file per named input, while `qualification/staged/` doesn't yet exist. Stage 3 must create the required production-stage directory and the named corpus file; it must not substitute a fixture copy or the ephemeral `/tmp` result.

Round-5 preservation command run from the repository root:

```sh
perl /tmp/wf-epic-b/OXY-B005/layout_visit_topology_model.pl /tmp/wf-epic-b/OXY-B005/layout-visit-corpus.json && sha256sum /tmp/wf-epic-b/OXY-B005/layout-visit-corpus.json && wc -c /tmp/wf-epic-b/OXY-B005/layout-visit-corpus.json
```

Exact captured output:

```text
OXY-B005 candidate-neutral topology counter model
cap=2
corpus_sha256=4972e43333984047b5a1d84200d5b89a29c5b59e47c5aca8773379320f2c6c84
fixture|family|nodes|edges|depth|realized|unrealized|ordinary|attempted|intrinsic|text|max_per_node|result
deep-box-064|single-pass-box|64|63|64|63|0|63|63|0|0|1|pass
wide-box-1024|single-pass-box|1025|1024|2|1024|0|1024|1024|0|0|1|pass
nested-weighted-8x8x8|weighted|585|584|4|584|0|584|584|0|0|1|pass
lazy-10000-realized-64|virtualized-lazy|65|64|2|64|9936|64|64|0|0|1|pass
reordered-keyed-128|weighted|129|128|2|128|0|128|128|0|0|1|pass
custom-two-pass-256|custom-multi-pass|257|256|2|256|0|512|512|0|0|2|pass
three-pass-cap-failure|custom-multi-pass|17|16|2|16|0|32|33|0|0|2|reject-cap-before-invocation-leaf-0000
invalid-constraints|single-pass-box|17|16|2|16|0|0|0|0|0|0|reject-before-child-layout
intrinsic-separation|intrinsic-measure|2|1|2|1|0|0|0|1|0|0|reject-from-ordinary-family
text-separation|text|2|1|2|1|0|1|1|0|1|1|separate-counter
topology_and_counter_assertions=passed
4972e43333984047b5a1d84200d5b89a29c5b59e47c5aca8773379320f2c6c84  /tmp/wf-epic-b/OXY-B005/layout-visit-corpus.json
6152 /tmp/wf-epic-b/OXY-B005/layout-visit-corpus.json
```

Table 2. Ordinary success corpus

| Fixture | Topology and declared event order | Tree nodes | Edges | Depth | Realized child IDs | Unrealized collection IDs | Ordinary visits | Attempts | Intrinsic | Text | Maximum per node | Outcome |
| :-- | :-- | --: | --: | --: | --: | --: | --: | --: | --: | --: | --: | :-- |
| `deep-box-064` | A 64-node chain. Preorder replays each actual parent-child edge once. | 64 | 63 | 64 | 63 | 0 | 63 | 63 | 0 | 0 | 1 | Pass. |
| `wide-box-1024` | One root with 1,024 fixed leaves. The root requests each direct child once. | 1,025 | 1,024 | 2 | 1,024 | 0 | 1,024 | 1,024 | 0 | 0 | 1 | Pass. |
| `nested-weighted-8x8x8` | Root -> 8 columns -> 64 rows -> 512 leaves. Preorder replays all 584 actual edges once with definite basis. | 585 | 584 | 4 | 584 | 0 | 584 | 584 | 0 | 0 | 1 | Pass. |
| `lazy-10000-realized-64` | A 10,000-item collection realizes `[4968,5032)`. The root requests only those 64 IDs in ascending range order. | 65 | 64 | 2 | 64 | 9,936 | 64 | 64 | 0 | 0 | 1 | Pass. |
| `reordered-keyed-128` | One definite-basis weighted root with 128 keyed leaves. It requests `key-127` through `key-000`, the declared reverse permutation. | 129 | 128 | 2 | 128 | 0 | 128 | 128 | 0 | 0 | 1 | Pass. |
| `custom-two-pass-256` | One root with 256 leaves. The declared custom sequence requests every direct child twice. | 257 | 256 | 2 | 256 | 0 | 512 | 512 | 0 | 0 | 2 | Pass. |

The virtualized fixture has a 10,000-item collection model, but only 64 realized IDs become tree nodes. The probe asserts every realized ID is in `[4968,5032)`, every other collection ID is unrealized, and no unrealized ID appears in the completed-visit map. Collection indexing and range selection aren't ordinary visits and need separate CAP-SCR-001 and CON-PERF-001 timing evidence.

Table 3. Failure and separation corpus

| Fixture | Topology and operation | Tree nodes | Edges | Depth | Realized child IDs | Unrealized collection IDs | Ordinary visits | Attempts | Intrinsic | Text | Maximum per node | Outcome |
| :-- | :-- | --: | --: | --: | --: | --: | --: | --: | --: | --: | --: | :-- |
| `three-pass-cap-failure` | One root with 16 leaves. The third declared pass rejects `leaf-0000` before invocation. | 17 | 16 | 2 | 16 | 0 | 32 | 33 | 0 | 0 | 2 | Reject before invocation. |
| `invalid-constraints` | One root with 16 leaves. Root validation fails before an ordinary child request. | 17 | 16 | 2 | 16 | 0 | 0 | 0 | 0 | 0 | 0 | Reject before a child request. |
| `intrinsic-separation` | One root and one nondefinite child. The operation is one dry or intrinsic query. | 2 | 1 | 2 | 1 | 0 | 0 | 0 | 1 | 0 | 0 | Exclude from the ordinary family. |
| `text-separation` | One root and one realized text leaf. The root issues one ordinary request and the leaf performs one text operation. | 2 | 1 | 2 | 1 | 0 | 1 | 1 | 0 | 1 | 1 | Record text separately. |

## Counting model

Apply this algorithm to one root layout transaction:

1. Validate root constraints before issuing a child request. If validation fails, return a structured layout failure with zero `ordinary_visits` and zero `attempted_ordinary_visits`.
2. Classify the operation before it runs. An ordinary operation is a regular request from a policy to a realized direct child. Dry or intrinsic requests and text-engine operations use separate counters.
3. Before the per-child cap check, increment the transaction `attempted_ordinary_visits` counter for each requested ordinary child invocation.
4. Compare the target child's completed ordinary-visit count in this transaction with the declaring policy-family cap. If the count equals the cap, reject the request before invocation. Don't increment the completed counter or `LayoutResult.node_visits`.
5. Otherwise, invoke the child. Increment that child's completed count and the issuing policy's `LayoutResult.node_visits`.
6. At transaction end, record `ordinary_visits` as the sum of completed event emissions. The harness must add each policy-local result once when it is emitted, not recursively sum nested `LayoutResult` values.

This model makes attempts equal completed visits when no cap rejects a request. In `three-pass-cap-failure`, the 33rd request increments attempts and then fails. The transaction records 32 completed ordinary visits and 33 attempts.

Table 4. Ordinary-policy classifier

| Policy family | Admission rule | Cap per realized direct child | Replay rule | Excluded work |
| :-- | :-- | --: | :-- | :-- |
| Single-pass box | Regular child layout with a chain or star topology and valid finite constraints. | 1 | Preorder each actual tree edge once. | Dry or intrinsic queries and text operations. |
| `weighted` | Regular child layout with explicit finite weights and a definite basis. | 1 | Preorder nested edges, or use the declared key permutation for keyed children. | Content-derived bases, dry or intrinsic measurement, and text operations. |
| Virtualized lazy | Regular child layout with a declared realized range and at least one unrealized collection ID. | 1 | Request only realized IDs in the declared range. | Offscreen collection work, range selection, intrinsic measurement, and text operations. |
| Custom multi-pass | Regular child layout with a finite declared direct-child event sequence. | 2 | Replay the declared pass sequence; reject the third requested invocation to a child before invocation. | Dry or intrinsic measurement and text operations. |

A policy that needs a dry query, intrinsic query, text operation, or a rejected third ordinary request isn't silently counted as ordinary. The harness must preserve its separate counter and result.

## Probe record

The nonproduction Perl model at `/tmp/wf-epic-b/OXY-B005/layout_visit_topology_model.pl` builds actual tree edges from each fixture's declared topology. It consumes `depth`, `collection`, `family`, `operation`, realized ranges, fan-out, and the reordered key permutation. It validates topology, family admission, declared event sequence, counters, cap ordering, and the absence of unrealized visits. It doesn't measure a layout engine or frame duration.

The following is the complete executed probe source.

<!-- canonical-block: layout-visit-topology-model-source -->

```text
#!/usr/bin/env perl
use strict;
use warnings;
use Digest::SHA qw(sha256_hex);
use JSON::PP;

# Candidate-neutral topology and counter model for OXY-B005. This is not a layout engine.
my $CAP = 2;
my @FIXTURES = (
  {
    id => 'deep-box-064', family => 'single-pass-box', operation => 'regular-child-layout', nodes => 64, depth => 64, passes => 1, collection => undef,
    topology => { kind => 'chain', fan_out => 1 },
    description => 'One root and 63 one-child boxes.',
    expected => { ordinary_visits => 63, attempted_ordinary_visits => 63, intrinsic_queries => 0, text_operations => 0, maximum_ordinary_visits_per_node => 1, outcome => 'pass' },
  },
  {
    id => 'wide-box-1024', family => 'single-pass-box', operation => 'regular-child-layout', nodes => 1025, depth => 2, passes => 1, collection => undef,
    topology => { kind => 'star', fan_out => 1024 },
    description => 'One root and 1,024 fixed leaf children.',
    expected => { ordinary_visits => 1024, attempted_ordinary_visits => 1024, intrinsic_queries => 0, text_operations => 0, maximum_ordinary_visits_per_node => 1, outcome => 'pass' },
  },
  {
    id => 'nested-weighted-8x8x8', family => 'weighted', operation => 'regular-child-layout', nodes => 585, depth => 4, passes => 1, collection => undef,
    topology => { kind => 'nested', fan_out => [8, 8, 8], definite_basis => JSON::PP::true },
    description => 'One root, eight weighted columns, 64 weighted rows, and 512 fixed leaves.',
    expected => { ordinary_visits => 584, attempted_ordinary_visits => 584, intrinsic_queries => 0, text_operations => 0, maximum_ordinary_visits_per_node => 1, outcome => 'pass' },
  },
  {
    id => 'lazy-10000-realized-64', family => 'virtualized-lazy', operation => 'regular-child-layout', nodes => 65, depth => 2, passes => 1, collection => 10000,
    topology => { kind => 'virtualized-star', realized_range => [4968, 5032], visible_range => [4984, 5016] },
    description => 'A 10,000-item collection realizes [4968,5032): 32 visible items and 16 cached items on each side. The root and 64 realized fixed leaves form the layout tree.',
    expected => { ordinary_visits => 64, attempted_ordinary_visits => 64, intrinsic_queries => 0, text_operations => 0, maximum_ordinary_visits_per_node => 1, outcome => 'pass' },
  },
  {
    id => 'reordered-keyed-128', family => 'weighted', operation => 'regular-child-layout', nodes => 129, depth => 2, passes => 1, collection => undef,
    topology => { kind => 'keyed-star', key_count => 128, key_permutation => 'reverse', definite_basis => JSON::PP::true },
    description => 'One weighted root reverses 128 fixed keyed leaves from key-000 through key-127 to key-127 through key-000, then lays out the realized tree.',
    expected => { ordinary_visits => 128, attempted_ordinary_visits => 128, intrinsic_queries => 0, text_operations => 0, maximum_ordinary_visits_per_node => 1, outcome => 'pass' },
  },
  {
    id => 'custom-two-pass-256', family => 'custom-multi-pass', operation => 'regular-child-layout', nodes => 257, depth => 2, passes => 2, collection => undef,
    topology => { kind => 'star', fan_out => 256 },
    description => 'One custom root issues two declared constraint passes to 256 fixed leaves.',
    expected => { ordinary_visits => 512, attempted_ordinary_visits => 512, intrinsic_queries => 0, text_operations => 0, maximum_ordinary_visits_per_node => 2, outcome => 'pass' },
  },
  {
    id => 'three-pass-cap-failure', family => 'custom-multi-pass', operation => 'regular-child-layout', nodes => 17, depth => 2, passes => 3, collection => undef,
    topology => { kind => 'star', fan_out => 16 },
    description => 'One custom root asks each of 16 fixed children for a third layout after two completed passes.',
    expected => { ordinary_visits => 32, attempted_ordinary_visits => 33, intrinsic_queries => 0, text_operations => 0, maximum_ordinary_visits_per_node => 2, outcome => 'reject-cap-before-invocation-leaf-0000' },
  },
  {
    id => 'invalid-constraints', family => 'single-pass-box', operation => 'invalid-constraints', nodes => 17, depth => 2, passes => 0, collection => undef,
    topology => { kind => 'star', fan_out => 16 },
    description => 'One box root receives invalid constraints before it can lay out 16 fixed children.',
    expected => { ordinary_visits => 0, attempted_ordinary_visits => 0, intrinsic_queries => 0, text_operations => 0, maximum_ordinary_visits_per_node => 0, outcome => 'reject-before-child-layout' },
  },
  {
    id => 'intrinsic-separation', family => 'intrinsic-measure', operation => 'dry-or-intrinsic-query', nodes => 2, depth => 2, passes => 0, collection => undef,
    topology => { kind => 'star', fan_out => 1 },
    description => 'One root requests a dry or intrinsic answer from one nondefinite child.',
    expected => { ordinary_visits => 0, attempted_ordinary_visits => 0, intrinsic_queries => 1, text_operations => 0, maximum_ordinary_visits_per_node => 0, outcome => 'reject-from-ordinary-family' },
  },
  {
    id => 'text-separation', family => 'text', operation => 'text-layout', nodes => 2, depth => 2, passes => 1, collection => undef,
    topology => { kind => 'star', fan_out => 1 },
    description => 'One box root lays out one realized text leaf, which invokes text layout.',
    expected => { ordinary_visits => 1, attempted_ordinary_visits => 1, intrinsic_queries => 0, text_operations => 1, maximum_ordinary_visits_per_node => 1, outcome => 'separate-counter' },
  },
);

sub expect {
  my ($condition, $message) = @_;
  die "assertion failed: $message\n" if !$condition;
}

sub new_tree {
  return { nodes => {}, children => {}, unrealized_ids => {}, source_keys => [], layout_keys => [] };
}

sub add_node {
  my ($tree, $id, $parent, $depth, $key, $realized) = @_;
  expect(!exists $tree->{nodes}{$id}, "duplicate node $id");
  $tree->{nodes}{$id} = { parent => $parent, depth => $depth, key => $key, realized => $realized };
  push @{ $tree->{children}{$parent} }, $id if defined $parent;
}

sub make_star {
  my ($fan_out, $prefix, $keys) = @_;
  my $tree = new_tree();
  add_node($tree, 'root', undef, 1, undef, 1);
  for my $index (0 .. $fan_out - 1) {
    my $key = defined $keys ? $keys->[$index] : undef;
    add_node($tree, sprintf('%s-%04d', $prefix, $index), 'root', 2, $key, 1);
  }
  return $tree;
}

sub build_tree {
  my ($fixture) = @_;
  my $topology = $fixture->{topology};
  my $kind = $topology->{kind};
  if ($kind eq 'chain') {
    my $tree = new_tree();
    add_node($tree, 'root', undef, 1, undef, 1);
    my $parent = 'root';
    for my $depth (2 .. $fixture->{depth}) {
      my $id = sprintf('box-%03d', $depth - 1);
      add_node($tree, $id, $parent, $depth, undef, 1);
      $parent = $id;
    }
    return $tree;
  }
  return make_star($topology->{fan_out}, 'leaf', undef) if $kind eq 'star';
  if ($kind eq 'nested') {
    my $tree = new_tree();
    add_node($tree, 'root', undef, 1, undef, 1);
    for my $column (0 .. $topology->{fan_out}[0] - 1) {
      my $column_id = sprintf('column-%02d', $column);
      add_node($tree, $column_id, 'root', 2, undef, 1);
      for my $row (0 .. $topology->{fan_out}[1] - 1) {
        my $row_id = sprintf('row-%02d-%02d', $column, $row);
        add_node($tree, $row_id, $column_id, 3, undef, 1);
        for my $leaf (0 .. $topology->{fan_out}[2] - 1) {
          add_node($tree, sprintf('leaf-%02d-%02d-%02d', $column, $row, $leaf), $row_id, 4, undef, 1);
        }
      }
    }
    return $tree;
  }
  if ($kind eq 'virtualized-star') {
    my ($start, $end) = @{ $topology->{realized_range} };
    expect(defined $fixture->{collection} && 0 <= $start && $start < $end && $end <= $fixture->{collection}, 'realized range');
    my $tree = new_tree();
    add_node($tree, 'root', undef, 1, undef, 1);
    for my $item (0 .. $fixture->{collection} - 1) {
      my $id = sprintf('item-%05d', $item);
      if ($start <= $item && $item < $end) {
        add_node($tree, $id, 'root', 2, sprintf('key-%05d', $item), 1);
      } else {
        $tree->{unrealized_ids}{$id} = 1;
      }
    }
    return $tree;
  }
  if ($kind eq 'keyed-star') {
    my @source_keys = map { sprintf('key-%03d', $_) } 0 .. $topology->{key_count} - 1;
    expect($topology->{key_permutation} eq 'reverse', 'key permutation declaration');
    my $tree = make_star($topology->{key_count}, 'keyed', \@source_keys);
    $tree->{source_keys} = \@source_keys;
    $tree->{layout_keys} = [ reverse @source_keys ];
    return $tree;
  }
  die "unknown topology: $kind\n";
}

sub edge_count {
  my ($tree) = @_;
  my $count = 0;
  $count += scalar @{ $_ } for values %{ $tree->{children} };
  return $count;
}

sub max_depth {
  my ($tree) = @_;
  my $maximum = 0;
  for my $node (values %{ $tree->{nodes} }) {
    $maximum = $node->{depth} if $node->{depth} > $maximum;
  }
  return $maximum;
}

sub assert_topology {
  my ($fixture, $tree) = @_;
  expect(scalar(keys %{ $tree->{nodes} }) == $fixture->{nodes}, "$fixture->{id}: node count");
  expect(edge_count($tree) == $fixture->{nodes} - 1, "$fixture->{id}: edge count");
  expect(max_depth($tree) == $fixture->{depth}, "$fixture->{id}: depth");
  expect(!defined $tree->{nodes}{root}{parent}, "$fixture->{id}: root");
  for my $id (keys %{ $tree->{nodes} }) {
    my $node = $tree->{nodes}{$id};
    next if !defined $node->{parent};
    expect(exists $tree->{nodes}{ $node->{parent} }, "$fixture->{id}: parent edge");
    expect($node->{depth} == $tree->{nodes}{ $node->{parent} }{depth} + 1, "$fixture->{id}: edge depth");
  }
  my $topology = $fixture->{topology};
  if ($topology->{kind} eq 'nested') {
    expect(join(',', @{ $topology->{fan_out} }) eq '8,8,8', 'nested fan-out');
    expect($topology->{definite_basis}, 'nested definite basis');
  }
  if ($topology->{kind} eq 'virtualized-star') {
    my ($start, $end) = @{ $topology->{realized_range} };
    my ($visible_start, $visible_end) = @{ $topology->{visible_range} };
    my @realized = map { sprintf('item-%05d', $_) } $start .. $end - 1;
    my %realized = map { $_ => 1 } @realized;
    my %actual = map { $_ => 1 } grep { $_ ne 'root' } keys %{ $tree->{nodes} };
    expect(join(',', sort keys %actual) eq join(',', sort keys %realized), 'realized IDs');
    expect(scalar(@realized) == 64, 'realized count');
    expect(scalar(keys %{ $tree->{unrealized_ids} }) == $fixture->{collection} - scalar(@realized), 'unrealized count');
    expect($fixture->{collection} - scalar(@realized) == 9936, '9,936 unrealized items');
    expect($start <= $visible_start && $visible_start < $visible_end && $visible_end <= $end && $visible_end - $visible_start == 32, 'visible range');
  }
  if ($topology->{kind} eq 'keyed-star') {
    my @expected = map { sprintf('key-%03d', $_) } reverse 0 .. $topology->{key_count} - 1;
    expect(join(',', @{ $tree->{layout_keys} }) eq join(',', @expected), 'reordered key sequence');
    expect(join(',', sort @{ $tree->{source_keys} }) eq join(',', sort @{ $tree->{layout_keys} }), 'reordered keys preserved');
    expect($topology->{definite_basis}, 'reordered definite basis');
  }
}

sub family_admitted {
  my ($fixture, $tree) = @_;
  my $family = $fixture->{family};
  my $kind = $fixture->{topology}{kind};
  return ($fixture->{operation} eq 'regular-child-layout' || $fixture->{operation} eq 'invalid-constraints') && ($kind eq 'chain' || $kind eq 'star') if $family eq 'single-pass-box';
  return $fixture->{operation} eq 'regular-child-layout' && $fixture->{topology}{definite_basis} if $family eq 'weighted';
  return $fixture->{operation} eq 'regular-child-layout' && scalar(keys %{ $tree->{unrealized_ids} }) > 0 if $family eq 'virtualized-lazy';
  return $fixture->{operation} eq 'regular-child-layout' && $fixture->{passes} >= 1 if $family eq 'custom-multi-pass';
  return 0 if $family eq 'intrinsic-measure' || $family eq 'text';
  die "unknown family: $family\n";
}

sub preorder_edges {
  my ($tree, $parent) = @_;
  my @events;
  for my $child (@{ $tree->{children}{$parent} // [] }) {
    push @events, [$parent, $child], preorder_edges($tree, $child);
  }
  return @events;
}

sub declared_child_layout_events {
  my ($fixture, $tree) = @_;
  my $family = $fixture->{family};
  if ($family eq 'single-pass-box' || $family eq 'weighted') {
    if ($fixture->{topology}{kind} eq 'keyed-star') {
      my %by_key = map { my $node = $tree->{nodes}{$_}; defined $node->{key} ? ($node->{key} => $_) : () } keys %{ $tree->{nodes} };
      return map { ['root', $by_key{$_}] } @{ $tree->{layout_keys} };
    }
    return preorder_edges($tree, 'root');
  }
  if ($family eq 'virtualized-lazy') {
    my ($start, $end) = @{ $fixture->{topology}{realized_range} };
    return map { ['root', sprintf('item-%05d', $_)] } $start .. $end - 1;
  }
  if ($family eq 'custom-multi-pass') {
    my @direct_children = @{ $tree->{children}{root} };
    my @events;
    push @events, map { ['root', $_] } @direct_children for 1 .. $fixture->{passes};
    return @events;
  }
  if ($family eq 'text') {
    return ['root', $tree->{children}{root}[0]];
  }
  die "$fixture->{id}: no ordinary event sequence\n";
}

sub new_counter {
  return { completed_by_node => {}, attempted_ordinary_visits => 0, intrinsic_queries => 0, text_operations => 0, emitted_events => [] };
}

sub ordinary_child_layout {
  my ($counter, $tree, $parent, $child) = @_;
  expect(exists $tree->{nodes}{$child} && defined $tree->{nodes}{$child}{parent} && $tree->{nodes}{$child}{parent} eq $parent, 'event is a declared parent-child edge');
  expect($tree->{nodes}{$child}{realized}, 'unrealized child cannot receive an ordinary visit');
  $counter->{attempted_ordinary_visits}++;
  my $completed = $counter->{completed_by_node}{$child} // 0;
  die "cap:$child:$counter->{attempted_ordinary_visits}\n" if $completed == $CAP;
  $counter->{completed_by_node}{$child} = $completed + 1;
  push @{ $counter->{emitted_events} }, [$parent, $child];
}

sub ordinary_visits {
  my ($counter) = @_;
  my $total = 0;
  $total += $_ for values %{ $counter->{completed_by_node} };
  return $total;
}

sub maximum_ordinary_visits_per_node {
  my ($counter) = @_;
  my $maximum = 0;
  for my $count (values %{ $counter->{completed_by_node} }) {
    $maximum = $count if $count > $maximum;
  }
  return $maximum;
}

sub verify {
  my ($fixture, $tree, $counter) = @_;
  my $expected = $fixture->{expected};
  expect(ordinary_visits($counter) == $expected->{ordinary_visits}, "$fixture->{id}: ordinary visits");
  expect($counter->{attempted_ordinary_visits} == $expected->{attempted_ordinary_visits}, "$fixture->{id}: attempted visits");
  expect($counter->{intrinsic_queries} == $expected->{intrinsic_queries}, "$fixture->{id}: intrinsic queries");
  expect($counter->{text_operations} == $expected->{text_operations}, "$fixture->{id}: text operations");
  expect(maximum_ordinary_visits_per_node($counter) == $expected->{maximum_ordinary_visits_per_node}, "$fixture->{id}: per-node maximum");
  for my $node (keys %{ $counter->{completed_by_node} }) {
    expect(!exists $tree->{unrealized_ids}{$node}, "$fixture->{id}: unrealized visits");
    expect($counter->{completed_by_node}{$node} <= $CAP, "$fixture->{id}: cap");
  }
}

sub replay_fixture {
  my ($fixture) = @_;
  my $tree = build_tree($fixture);
  assert_topology($fixture, $tree);
  my $counter = new_counter();
  my $admitted = family_admitted($fixture, $tree);
  if ($fixture->{operation} eq 'invalid-constraints') {
    expect($admitted, 'invalid constraint fixture family admission');
    verify($fixture, $tree, $counter);
    return ($tree, $counter);
  }
  if ($fixture->{operation} eq 'dry-or-intrinsic-query') {
    expect(!$admitted, 'intrinsic excluded from ordinary family');
    $counter->{intrinsic_queries}++;
    verify($fixture, $tree, $counter);
    return ($tree, $counter);
  }
  if ($fixture->{operation} eq 'text-layout') {
    expect(!$admitted, 'text excluded from ordinary family');
    ordinary_child_layout($counter, $tree, @{ $_ }) for declared_child_layout_events($fixture, $tree);
    $counter->{text_operations}++;
    verify($fixture, $tree, $counter);
    return ($tree, $counter);
  }
  expect($fixture->{operation} eq 'regular-child-layout', 'known operation');
  expect($admitted, 'ordinary family admission');
  my @events = declared_child_layout_events($fixture, $tree);
  if ($fixture->{family} eq 'custom-multi-pass') {
    expect(scalar(@events) == $fixture->{passes} * scalar(@{ $tree->{children}{root} }), 'custom pass sequence');
  }
  if ($fixture->{topology}{kind} eq 'keyed-star') {
    my @emitted_keys = map { $tree->{nodes}{$_->[1]}{key} } @events;
    expect(join(',', @emitted_keys) eq join(',', @{ $tree->{layout_keys} }), 'reordered event sequence');
  }
  if ($fixture->{family} eq 'virtualized-lazy') {
    my %children = map { $_->[1] => 1 } @events;
    my %realized = map { $_ => 1 } grep { $_ ne 'root' } keys %{ $tree->{nodes} };
    expect(join(',', sort keys %children) eq join(',', sort keys %realized), 'virtualized event range');
  }
  my $completed = eval {
    ordinary_child_layout($counter, $tree, @{ $_ }) for @events;
    1;
  };
  if (!$completed) {
    my $error = $@;
    expect($fixture->{expected}{outcome} eq 'reject-cap-before-invocation-leaf-0000', 'unexpected cap rejection');
    expect($error eq "cap:leaf-0000:33\n", 'cap rejection location');
    verify($fixture, $tree, $counter);
    return ($tree, $counter);
  }
  expect($fixture->{expected}{outcome} eq 'pass', 'missing expected cap rejection');
  verify($fixture, $tree, $counter);
  return ($tree, $counter);
}

@ARGV == 1 or die "usage: $0 MANIFEST_PATH\n";
my $json = JSON::PP->new->ascii->canonical->indent(1)->indent_length(2)->space_before(0)->space_after(1);
my $manifest = $json->encode(\@FIXTURES);
open my $manifest_file, '>:raw', $ARGV[0] or die "cannot write $ARGV[0]: $!\n";
print {$manifest_file} $manifest or die "cannot write $ARGV[0]: $!\n";
close $manifest_file or die "cannot close $ARGV[0]: $!\n";

print "OXY-B005 candidate-neutral topology counter model\n";
printf "cap=%d\n", $CAP;
printf "corpus_sha256=%s\n", sha256_hex($manifest);
print "fixture|family|nodes|edges|depth|realized|unrealized|ordinary|attempted|intrinsic|text|max_per_node|result\n";
for my $fixture (@FIXTURES) {
  my ($tree, $counter) = replay_fixture($fixture);
  printf "%s|%s|%d|%d|%d|%d|%d|%d|%d|%d|%d|%d|%s\n",
    $fixture->{id}, $fixture->{family}, scalar(keys %{ $tree->{nodes} }), edge_count($tree), max_depth($tree), scalar(keys %{ $tree->{nodes} }) - 1,
    scalar(keys %{ $tree->{unrealized_ids} }), ordinary_visits($counter), $counter->{attempted_ordinary_visits}, $counter->{intrinsic_queries},
    $counter->{text_operations}, maximum_ordinary_visits_per_node($counter), $fixture->{expected}{outcome};
}
print "topology_and_counter_assertions=passed\n";
```

Command run from the repository root:

```sh
perl /tmp/wf-epic-b/OXY-B005/layout_visit_topology_model.pl /tmp/wf-epic-b/OXY-B005/layout_visit_topology_corpus.json && sha256sum /tmp/wf-epic-b/OXY-B005/layout_visit_topology_corpus.json
```

Exact captured output:

```text
OXY-B005 candidate-neutral topology counter model
cap=2
corpus_sha256=4972e43333984047b5a1d84200d5b89a29c5b59e47c5aca8773379320f2c6c84
fixture|family|nodes|edges|depth|realized|unrealized|ordinary|attempted|intrinsic|text|max_per_node|result
deep-box-064|single-pass-box|64|63|64|63|0|63|63|0|0|1|pass
wide-box-1024|single-pass-box|1025|1024|2|1024|0|1024|1024|0|0|1|pass
nested-weighted-8x8x8|weighted|585|584|4|584|0|584|584|0|0|1|pass
lazy-10000-realized-64|virtualized-lazy|65|64|2|64|9936|64|64|0|0|1|pass
reordered-keyed-128|weighted|129|128|2|128|0|128|128|0|0|1|pass
custom-two-pass-256|custom-multi-pass|257|256|2|256|0|512|512|0|0|2|pass
three-pass-cap-failure|custom-multi-pass|17|16|2|16|0|32|33|0|0|2|reject-cap-before-invocation-leaf-0000
invalid-constraints|single-pass-box|17|16|2|16|0|0|0|0|0|0|reject-before-child-layout
intrinsic-separation|intrinsic-measure|2|1|2|1|0|0|0|1|0|0|reject-from-ordinary-family
text-separation|text|2|1|2|1|0|1|1|0|1|1|separate-counter
topology_and_counter_assertions=passed
4972e43333984047b5a1d84200d5b89a29c5b59e47c5aca8773379320f2c6c84  /tmp/wf-epic-b/OXY-B005/layout_visit_topology_corpus.json
```

## Reference algorithm comparison

The pinned [Flutter `RenderObject.layout` source](https://raw.githubusercontent.com/flutter/flutter/4cf24164269a5ebf0c16a028a00727d0e77bbb05/packages/flutter/lib/src/rendering/object.dart) establishes the parent-to-child request boundary used by row 2. The pinned [Flutter `RenderBox.getDryLayout` source](https://raw.githubusercontent.com/flutter/flutter/4cf24164269a5ebf0c16a028a00727d0e77bbb05/packages/flutter/lib/src/rendering/box.dart) distinguishes dry from wet layout and warns that dry layout can be O(N^2). The probe is a candidate-neutral counting model, not a claim that Oxyflut adopts Flutter behavior.

The dated [CSS Flexible Box Layout Module Level 1 Candidate Recommendation](https://www.w3.org/TR/2018/CR-css-flexbox-1-20181119/) defines order-modified document order. The reordered fixture uses an explicit reverse key permutation to exercise ordering without claiming CSS implementation conformance.

The pinned [Yoga external-layout-systems source](https://raw.githubusercontent.com/facebook/yoga/bd8fe0d6d243cc7e0334d4cc68864a994f63beae/website/docs/advanced/external-layout-systems.mdx) identifies text and externally laid-out views as measure-function content. This supports a separate text-operation counter. The ordinary parent-to-text-leaf invocation remains counted, while text work remains separate.

### Immutable source record

All four sources were fetched successfully. The SHA-256 values are over the fetched source bytes. The Flutter and Yoga excerpts are verbatim. The CSS excerpt is normalized for readability; its digest remains over the unmodified fetched HTML bytes.

| Source | Fetched UTC | Bytes | SHA-256 |
| :-- | :-- | --: | :-- |
| [Flutter `object.dart` at `4cf24164269a5ebf0c16a028a00727d0e77bbb05`](https://raw.githubusercontent.com/flutter/flutter/4cf24164269a5ebf0c16a028a00727d0e77bbb05/packages/flutter/lib/src/rendering/object.dart) | 2026-08-28T16:59:35Z | 263,459 | `292ef7c3a0995054054274827417820556e3afa8211977037460b30e6240aa51` |
| [Flutter `box.dart` at `4cf24164269a5ebf0c16a028a00727d0e77bbb05`](https://raw.githubusercontent.com/flutter/flutter/4cf24164269a5ebf0c16a028a00727d0e77bbb05/packages/flutter/lib/src/rendering/box.dart) | 2026-08-28T16:59:35Z | 142,065 | `66c2e64bb8b508af65178991ac81bde5f0b99bbe052d990dee361ba2f5019beb` |
| [CSS Flexible Box Layout Module Level 1 Candidate Recommendation, 2018-11-19](https://www.w3.org/TR/2018/CR-css-flexbox-1-20181119/) | 2026-08-28T16:59:35Z | 638,802 | `288cdb522418f764d9a312c58d4dc6a76d9bbef7ac0f15344184a7d7b6a5bae8` |
| [Yoga external-layout-systems source at `bd8fe0d6d243cc7e0334d4cc68864a994f63beae`](https://raw.githubusercontent.com/facebook/yoga/bd8fe0d6d243cc7e0334d4cc68864a994f63beae/website/docs/advanced/external-layout-systems.mdx) | 2026-08-28T17:00:27Z | 3,454 | `09053a128470512bbc6767bba3a23811c5d37e9bc2f4008e35178d0a1c502d48` |

Flutter `RenderObject.layout` excerpt:

```dart
  /// This method is the main entry point for parents to ask their children to
  /// update their layout information. The parent passes a constraints object,
  /// which informs the child as to which layouts are permissible. The child is
```

Flutter `RenderBox.getDryLayout` excerpt:

```dart
  /// This layout is called "dry" layout as opposed to the regular "wet" layout
  /// run performed by [performLayout] because it computes the desired size for
  /// the given constraints without changing any internal state.
```

CSS Flexbox excerpt, normalized for readability:

```html
<p>
  A flex container lays out its content in
  <dfn
    class="dfn-paneled"
    data-dfn-type="dfn"
    data-export
    id="order-modified-document-order"
    >order-modified document order</dfn
  >, starting from the lowest numbered ordinal group and going up. Items with
  the same ordinal group are laid out in the order they appear in the source
  document.
</p>
```

Yoga excerpt:

```md
It is typical for applications to have content whose size may be dependent on factors not expressible inside of Yoga. This can often include text, or views which are rendered or laid out using a different system. Yoga allows leaf nodes to delegate to a different layout system via **Measure Functions**.
```

## Options and trade-offs

- Option A: Freeze the corpus, topology replay, counting algorithm, family classifier, and per-family algebraic bounds.
- Option B: Freeze one global numeric cap as a performance-qualified result.
- Option C: Retain the numeric cap as a gating KU until an instrumented candidate probe produces schema-valid timing evidence under CON-PERF-001.

## Recommendation

- **Chosen option by row:** Rows 1 through 5 use Option A. Row 6 uses Option C. Option B is rejected for row 6 because the topology model has no timing observation.
- **Derived threshold, not a freeze:** `2` is the smallest global threshold that admits the declared two-pass custom fixture. It isn't a performance recommendation because the probe records no nanoseconds or paint-submission allowance.
- **Why it fits:** The result preserves CAP-LAY-001's bounded propagation, keeps intrinsic and text work explicit, and doesn't weaken CON-PERF-001. The wide passing fixture has 1,024 ordinary visits, yielding an all-layout arithmetic ceiling of 1.953125 microseconds per visit under 2.0 ms. That ceiling doesn't reserve paint time and isn't performance evidence.
- **Rejected options:** Reject a timing-only rule, an average-count rule, unbounded intrinsic recursion, a hidden text-work exemption, and a numeric cap selected from shallow scenes.

### Sample-validity policy

- **Option A:** Bind a complete sample-validity contract to the prequalification lock and have the validator apply it before aggregation.
- **Option B (chosen):** Forbid exclusions: each run includes all 500 measured frames from each launch, `valid` is always `true`, and `exclusionReason` is disallowed.
- **Reason:** The staged sample-validity schema is a generic, nonauthoritative proposal. The preserved host inspection shows no layout-frame, launch, or ordinal rule. It therefore can't establish a digest-bound rule for this timing procedure. Option B leaves no self-asserted exclusion path and doesn't claim that the staged proposal covers layout frames.

### Next bounded probe

After Stage 3 makes the exact contract edits below and authorizes unscored candidate probes, run both instrumented candidates with `CAP_CANDIDATE=2` on the six passing fixtures in table 2. The suite must enumerate exactly the Cartesian product of `focused` and `integrated`, `macos`, `windows`, `wayland`, and `x11`, and the six passing fixture IDs: 2 x 4 x 6 = 48 unique tuples. For every tuple, run 20 launches, execute 300 warmup frames, and record all 500 fixed measured frames per launch. The probe forbids exclusions; a missing, duplicate, failed, or mismatched tuple retains the KU (gating).

For this probe, `lockDigest` is the SHA-256 digest of the frozen canonical UTF-8 bytes of one `qualification-lock` v6.0.0 prequalification-probe instance. That instance must set `candidateImplementationReady` and `measurementReady` to `false`; it doesn't claim either readiness state.

Before the probe, the prequalification-probe lock must resolve `measurementPolicy.layoutVisitCorpus`, `measurementPolicy.layoutQualificationRecordSchema`, `measurementPolicy.layoutPrequalificationRunSchema`, `measurementPolicy.layoutPrequalificationSuiteSchema`, and `measurementPolicy.layoutVisitCountingRules`; `measurementPolicy.layoutPrequalificationIdentities` for both candidates and all four environments; every `sourcePins` value, including `integratedFork.commit` and `oxyflutAdapter.commit` with `status: "kk"`; every `candidateArtifacts` `sourceRevision`, `httpVerified`, `sha256`, and `sizeBytes`; every `referenceEnvironments` `minimumVersion`, `hardwareId`, `gpuId`, `driverVersion`, and `systemPackageLockDigest`; and `workload.releaseFlags`. The matching `layoutPrequalificationIdentities.<candidate>.<environment>` pair must equal `candidateSource.revision` and `candidateSource.artifactSha256` in every matching run and record. The suite's `lockDigest`, corpus, counting-rules, record-schema, run-schema, and suite-schema digests must equal their lock fields; each run must carry the matching applicable identity digests; each record must carry the matching lock, corpus, counting-rules, candidate source, hardware, and release-flag identities.

Only `workload.referenceApplication`, `scenes`, `interactionScripts`, `fonts`, `assets`, `windowMatrix`, and `cacheStates`; and `measurementPolicy.rawMeasurementSchema`, `capabilityBaseline`, `platformContracts`, `scoringAnchors`, `assessors`, `fuzzCorpora`, `securityPatchRehearsal`, and `externalContractLock` may remain `null`. `measurementPolicy.sampleValidityRules` must be exactly `null` for this probe, and `resolvedTools` may remain an empty array. `measurementPolicy.layoutVisitCap` must remain `null`, and the known-unknown arrays must retain `layout-visit-cap`. No record can claim readiness, contribute a comparative score, select a candidate, or set the numeric cap.

For every root layout transaction in every warmup and measured frame, emit one schema-valid `layout-qualification-record` with the corpus, counting-rules, fixture, candidate source and artifact, hardware, GPU, driver, release-flag, and lock identities; frame partition; separate counters; application-owned layout nanoseconds; paint-submission nanoseconds; and aggregate nanoseconds. The harness must start fixture root layouts only through one instrumented dispatcher. The dispatcher assigns the next `transactionOrdinal` when it starts each transaction and emits exactly one record when that transaction completes. The harness must fail a frame with an unrecorded dispatch, duplicate dispatch record, or no root transaction. `valid` is always `true`, and `exclusionReason` is disallowed. Each tuple's `RECORD_SET_PATH` is one canonical UTF-8 JSON array ordered by `(launch, framePhase, frameOrdinal, transactionOrdinal)`, with lexicographic object keys, 2-space indentation, LF line endings, and one trailing LF. Its cardinality is variable: each run contains 16,000 frame groups and at least 16,000 records, and the complete suite contains 768,000 frame groups and at least 768,000 records. In each frame group, transaction ordinals must be contiguous from 1 through the observed transaction count. The record with `transactionOrdinal: 1` carries the complete frame `paintSubmissionNs`; every later transaction carries `paintSubmissionNs: 0`. Each record calculates `aggregateNs` as application-owned layout plus its paint-submission value. The harness sums every transaction's ordinary, attempted, intrinsic, text, layout, paint-submission, and aggregate values by frame, calculates a nearest-rank 99th percentile per launch from exactly 500 measured frame totals, then takes the maximum of 20 launch percentiles. Each transaction in a passing fixture has the table 2 counters and no cap rejection. The run passes only when that maximum is no greater than 2.0 ms. A failing value retains the KU. It doesn't increase the cap, alter the corpus, emit a score, select a candidate, or claim readiness.

The frozen prequalification-probe lock breaks the evidence-order problem without claiming readiness: the hashing-bound counting rules supply `CAP_CANDIDATE=2` only for the unscored probe, while `measurementPolicy.layoutVisitCap` remains `null`. The validator must load raw bytes from `--corpus CORPUS_PATH`, SHA-256-check them against `measurementPolicy.layoutVisitCorpus` before parsing, and look up every fixture's counters and outcome in that parsed corpus rather than in validator constants. It must also load raw bytes from `--suite-schema SUITE_SCHEMA_PATH`, SHA-256-check them against `measurementPolicy.layoutPrequalificationSuiteSchema` before parsing that schema; then parse it and validate the suite against it, and require the suite's `suiteSchemaDigest` to equal that raw-byte SHA-256 digest. It maps corpus `expected.outcome: "pass"` to record `capOutcome: "completed"` as a generic outcome conversion; it must reject any other outcome for one of the six passing fixtures. The host-only topology model can't close the timing gate.

## Downstream impact

- **ADRs to write or update:** None. This report doesn't change an architecture decision.
- **Tickets unblocked in `tasks/active/`:** None. `OXY-D001` remains blocked by `layout-visit-cap`.
- **Tickets to add or split:** Add one bounded prequalification layout-cost prototype ticket only after Stage 3 authorizes the probe in "Next bounded probe".

### Spec edits required

Stage 3 must apply all of the following edits without setting a numeric cap.

- Create `.constitution/tech-spec/data-models/layout-qualification-record.schema.json` with the exact bytes in the following code block. Its SHA-256 is `09d96af49384e47ee6154f386af2ef771985516a61c843d561835654283bd7b1`.

<!-- canonical-block: layout-qualification-record-schema -->

```text
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "urn:oxyflut:schema:layout-qualification-record:1",
  "title": "Oxyflut layout qualification record",
  "description": "One raw, harness-emitted root layout transaction. Counters and application-owned layout timing describe one transaction in one declared warmup or measured frame. The frame's paint submission is recorded only in transactionOrdinal 1.",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "schemaVersion",
    "candidate",
    "environment",
    "candidateSource",
    "hardware",
    "releaseFlagsDigest",
    "lockDigest",
    "corpusDigest",
    "countingRulesDigest",
    "fixtureId",
    "launch",
    "framePhase",
    "frameOrdinal",
    "transactionOrdinal",
    "monotonicNs",
    "ordinaryVisits",
    "attemptedOrdinaryVisits",
    "intrinsicQueries",
    "textOperations",
    "applicationOwnedLayoutNs",
    "paintSubmissionNs",
    "aggregateNs",
    "capOutcome",
    "aggregation",
    "valid",
    "harnessLog"
  ],
  "properties": {
    "schemaVersion": {
      "const": "1.0.0",
      "description": "Schema version; no unit or aggregation."
    },
    "candidate": {
      "enum": ["focused", "integrated"],
      "description": "Candidate identity; group records by this value before aggregation."
    },
    "environment": {
      "enum": ["macos", "windows", "wayland", "x11"],
      "description": "Tier 1 environment identity; never aggregate records across this value."
    },
    "candidateSource": {
      "type": "object",
      "additionalProperties": false,
      "required": ["revision", "artifactSha256"],
      "properties": {
        "revision": {
          "$ref": "#/$defs/sha40",
          "description": "Candidate source revision; identity only."
        },
        "artifactSha256": {
          "$ref": "#/$defs/sha256",
          "description": "Measured artifact digest; identity only."
        }
      }
    },
    "hardware": {
      "type": "object",
      "additionalProperties": false,
      "required": ["hardwareId", "gpuId", "driverVersion"],
      "properties": {
        "hardwareId": {
          "type": "string",
          "minLength": 1,
          "description": "Reference-machine hardware identity; never aggregate records across this value."
        },
        "gpuId": {
          "type": "string",
          "minLength": 1,
          "description": "Graphics-processor identity; never aggregate records across this value."
        },
        "driverVersion": {
          "type": "string",
          "minLength": 1,
          "description": "Graphics-driver identity; never aggregate records across this value."
        }
      }
    },
    "releaseFlagsDigest": {
      "$ref": "#/$defs/sha256",
      "description": "Digest of candidate release flags; never aggregate records across this value."
    },
    "lockDigest": {
      "$ref": "#/$defs/sha256",
      "description": "Digest of the frozen prequalification-probe qualification lock. That lock has candidateImplementationReady and measurementReady set to false and binds the counting rules, corpus, record schema, candidate source and artifact, matching hardware and driver, and release-flag identities; identity only."
    },
    "corpusDigest": {
      "$ref": "#/$defs/sha256",
      "description": "Digest of the canonical layout corpus manifest; identity only."
    },
    "countingRulesDigest": {
      "$ref": "#/$defs/sha256",
      "description": "Digest of the canonical layout-visit counting-rules document bound by lockDigest; identity only."
    },
    "fixtureId": {
      "enum": [
        "deep-box-064",
        "wide-box-1024",
        "nested-weighted-8x8x8",
        "lazy-10000-realized-64",
        "reordered-keyed-128",
        "custom-two-pass-256",
        "three-pass-cap-failure",
        "invalid-constraints",
        "intrinsic-separation",
        "text-separation"
      ],
      "description": "Canonical corpus fixture identity; aggregate timing only within one fixture."
    },
    "framePhase": {
      "enum": ["warmup", "measured"],
      "description": "Frame partition within a launch; only measured frames contribute to CON-PERF-001 aggregation."
    },
    "launch": {
      "type": "integer",
      "minimum": 1,
      "description": "One-based process launch; group exactly 500 measured frames by this value before calculating a per-launch percentile."
    },
    "frameOrdinal": {
      "type": "integer",
      "minimum": 1,
      "description": "One-based ordinal within framePhase; warmup requires 1 through 300 and measured requires 1 through 500 in the whole-run validator."
    },
    "transactionOrdinal": {
      "type": "integer",
      "minimum": 1,
      "description": "One-based ordinal of a root layout transaction within frameOrdinal; the whole-run validator requires a nonempty contiguous sequence starting at 1."
    },
    "monotonicNs": {
      "type": "integer",
      "minimum": 0,
      "description": "Transaction observation timestamp in monotonic nanoseconds; do not aggregate as a duration."
    },
    "ordinaryVisits": {
      "type": "integer",
      "minimum": 0,
      "description": "Completed ordinary direct-child layout invocations in this transaction; unit: invocations; sum only when reporting a frame total."
    },
    "attemptedOrdinaryVisits": {
      "type": "integer",
      "minimum": 0,
      "description": "Requested ordinary direct-child layout invocations in this transaction, incremented immediately before each per-child cap check; unit: invocations; includes a rejected request and is at least ordinaryVisits."
    },
    "intrinsicQueries": {
      "type": "integer",
      "minimum": 0,
      "description": "Dry or intrinsic queries in this transaction; unit: queries; separate from ordinaryVisits and summed only within a frame total."
    },
    "textOperations": {
      "type": "integer",
      "minimum": 0,
      "description": "Text-engine layout or shaping operations in this transaction; unit: operations; separate from ordinaryVisits and summed only within a frame total."
    },
    "applicationOwnedLayoutNs": {
      "type": "integer",
      "minimum": 0,
      "description": "Application-owned layout duration for this root transaction; unit: nanoseconds; sum every transaction within a frame before percentile aggregation."
    },
    "paintSubmissionNs": {
      "type": "integer",
      "minimum": 0,
      "description": "Application-owned paint-submission duration for the entire frame; unit: nanoseconds; transactionOrdinal 1 carries the value and every later transaction carries 0 before the frame sum."
    },
    "aggregateNs": {
      "type": "integer",
      "minimum": 0,
      "description": "applicationOwnedLayoutNs plus paintSubmissionNs for this root transaction; unit: nanoseconds; sum every transaction within a frame, calculate each launch's nearest-rank 99th percentile from 500 measured frame totals, then take the maximum across 20 launches."
    },
    "capOutcome": {
      "enum": ["completed", "rejected-before-invocation", "not-reached"],
      "description": "Outcome of the transaction's ordinary-visit cap check; identity only, and any rejected-before-invocation record fails the passing corpus expectation."
    },
    "aggregation": {
      "const": "sum-transactions-per-frame;-nearest-rank-p99-per-launch;maximum-20-launches",
      "description": "Fixed CON-PERF-001 aggregation order for measured records with identical candidate, environment, source, hardware, driver, flags, lock, corpus, counting-rules, and fixture identities."
    },
    "valid": {
      "const": true,
      "description": "This prequalification probe admits every fixed frame; the record cannot self-assert exclusion."
    },
    "harnessLog": {
      "$ref": "#/$defs/evidence",
      "description": "Immutable preserved harness output for this record; identity only."
    }
  },
  "allOf": [
    {
      "if": {
        "properties": { "transactionOrdinal": { "const": 1 } },
        "required": ["transactionOrdinal"]
      },
      "else": {
        "properties": {
          "paintSubmissionNs": { "const": 0 }
        }
      }
    },
    {
      "if": {
        "properties": { "fixtureId": { "const": "deep-box-064" } },
        "required": ["fixtureId"]
      },
      "then": {
        "properties": {
          "ordinaryVisits": { "const": 63 },
          "attemptedOrdinaryVisits": { "const": 63 },
          "intrinsicQueries": { "const": 0 },
          "textOperations": { "const": 0 },
          "capOutcome": { "const": "completed" }
        }
      }
    },
    {
      "if": {
        "properties": { "fixtureId": { "const": "wide-box-1024" } },
        "required": ["fixtureId"]
      },
      "then": {
        "properties": {
          "ordinaryVisits": { "const": 1024 },
          "attemptedOrdinaryVisits": { "const": 1024 },
          "intrinsicQueries": { "const": 0 },
          "textOperations": { "const": 0 },
          "capOutcome": { "const": "completed" }
        }
      }
    },
    {
      "if": {
        "properties": { "fixtureId": { "const": "nested-weighted-8x8x8" } },
        "required": ["fixtureId"]
      },
      "then": {
        "properties": {
          "ordinaryVisits": { "const": 584 },
          "attemptedOrdinaryVisits": { "const": 584 },
          "intrinsicQueries": { "const": 0 },
          "textOperations": { "const": 0 },
          "capOutcome": { "const": "completed" }
        }
      }
    },
    {
      "if": {
        "properties": { "fixtureId": { "const": "lazy-10000-realized-64" } },
        "required": ["fixtureId"]
      },
      "then": {
        "properties": {
          "ordinaryVisits": { "const": 64 },
          "attemptedOrdinaryVisits": { "const": 64 },
          "intrinsicQueries": { "const": 0 },
          "textOperations": { "const": 0 },
          "capOutcome": { "const": "completed" }
        }
      }
    },
    {
      "if": {
        "properties": { "fixtureId": { "const": "reordered-keyed-128" } },
        "required": ["fixtureId"]
      },
      "then": {
        "properties": {
          "ordinaryVisits": { "const": 128 },
          "attemptedOrdinaryVisits": { "const": 128 },
          "intrinsicQueries": { "const": 0 },
          "textOperations": { "const": 0 },
          "capOutcome": { "const": "completed" }
        }
      }
    },
    {
      "if": {
        "properties": { "fixtureId": { "const": "custom-two-pass-256" } },
        "required": ["fixtureId"]
      },
      "then": {
        "properties": {
          "ordinaryVisits": { "const": 512 },
          "attemptedOrdinaryVisits": { "const": 512 },
          "intrinsicQueries": { "const": 0 },
          "textOperations": { "const": 0 },
          "capOutcome": { "const": "completed" }
        }
      }
    },
    {
      "if": {
        "properties": { "fixtureId": { "const": "three-pass-cap-failure" } },
        "required": ["fixtureId"]
      },
      "then": {
        "properties": {
          "ordinaryVisits": { "const": 32 },
          "attemptedOrdinaryVisits": { "const": 33 },
          "intrinsicQueries": { "const": 0 },
          "textOperations": { "const": 0 },
          "capOutcome": { "const": "rejected-before-invocation" }
        }
      }
    },
    {
      "if": {
        "properties": { "fixtureId": { "const": "invalid-constraints" } },
        "required": ["fixtureId"]
      },
      "then": {
        "properties": {
          "ordinaryVisits": { "const": 0 },
          "attemptedOrdinaryVisits": { "const": 0 },
          "intrinsicQueries": { "const": 0 },
          "textOperations": { "const": 0 },
          "capOutcome": { "const": "not-reached" }
        }
      }
    },
    {
      "if": {
        "properties": { "fixtureId": { "const": "intrinsic-separation" } },
        "required": ["fixtureId"]
      },
      "then": {
        "properties": {
          "ordinaryVisits": { "const": 0 },
          "attemptedOrdinaryVisits": { "const": 0 },
          "intrinsicQueries": { "const": 1 },
          "textOperations": { "const": 0 },
          "capOutcome": { "const": "not-reached" }
        }
      }
    },
    {
      "if": {
        "properties": { "fixtureId": { "const": "text-separation" } },
        "required": ["fixtureId"]
      },
      "then": {
        "properties": {
          "ordinaryVisits": { "const": 1 },
          "attemptedOrdinaryVisits": { "const": 1 },
          "intrinsicQueries": { "const": 0 },
          "textOperations": { "const": 1 },
          "capOutcome": { "const": "completed" }
        }
      }
    }
  ],
  "$defs": {
    "sha40": {
      "type": "string",
      "pattern": "^[0-9a-f]{40}$"
    },
    "sha256": {
      "type": "string",
      "pattern": "^[0-9a-f]{64}$"
    },
    "evidence": {
      "type": "object",
      "additionalProperties": false,
      "required": ["path", "sha256"],
      "properties": {
        "path": {
          "type": "string",
          "minLength": 1
        },
        "sha256": {
          "$ref": "#/$defs/sha256"
        }
      }
    }
  }
}
```

- The first `allOf` branch uses JSON Schema `if` and `else` to require `paintSubmissionNs: 0` for every `transactionOrdinal` after the first. The remaining fixture branches use `if` and `then` to bind each `fixtureId` to its canonical counters and `capOutcome`. The [JSON Schema conditional-validation guide](https://json-schema.org/understanding-json-schema/reference/conditionals) establishes those keywords. JSON Schema can't compare instance values or recompute sums, so the whole-run validator must enforce transaction contiguity and the semantic rules that follow.
- Create `.constitution/tech-spec/data-models/layout-prequalification-run.schema.json` with the exact bytes in the following code block. Its SHA-256 is `76dfee7dfcdfdd49e2d67afdf83ab43c29dbb6513652a8023b0869a7d59293e2`.

<!-- canonical-block: layout-prequalification-run-schema -->

```text
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "urn:oxyflut:schema:layout-prequalification-run:1",
  "title": "Oxyflut layout prequalification run",
  "description": "One complete, unscored CON-PERF-001 layout-cap probe for one candidate, environment, and passing corpus fixture.",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "schemaVersion",
    "runId",
    "candidate",
    "environment",
    "candidateSource",
    "hardware",
    "releaseFlagsDigest",
    "lockDigest",
    "corpusDigest",
    "countingRulesDigest",
    "recordSchemaDigest",
    "fixtureId",
    "capCandidate",
    "recordSet",
    "aggregation",
    "launches",
    "maximumLaunchP99Ns",
    "conPerf001LimitNs",
    "pass",
    "harnessLog"
  ],
  "properties": {
    "schemaVersion": {
      "const": "1.0.0"
    },
    "runId": {
      "type": "string",
      "minLength": 1
    },
    "candidate": {
      "enum": ["focused", "integrated"]
    },
    "environment": {
      "enum": ["macos", "windows", "wayland", "x11"]
    },
    "candidateSource": {
      "$ref": "#/$defs/candidateSource"
    },
    "hardware": {
      "$ref": "#/$defs/hardware"
    },
    "releaseFlagsDigest": {
      "$ref": "#/$defs/sha256"
    },
    "lockDigest": {
      "$ref": "#/$defs/sha256"
    },
    "corpusDigest": {
      "$ref": "#/$defs/sha256"
    },
    "countingRulesDigest": {
      "$ref": "#/$defs/sha256"
    },
    "recordSchemaDigest": {
      "$ref": "#/$defs/sha256"
    },
    "fixtureId": {
      "enum": [
        "deep-box-064",
        "wide-box-1024",
        "nested-weighted-8x8x8",
        "lazy-10000-realized-64",
        "reordered-keyed-128",
        "custom-two-pass-256"
      ]
    },
    "capCandidate": {
      "const": 2
    },
    "recordSet": {
      "$ref": "#/$defs/evidence"
    },
    "aggregation": {
      "const": "sum-transactions-per-frame;-nearest-rank-p99-per-launch;maximum-20-launches"
    },
    "launches": {
      "type": "array",
      "minItems": 20,
      "maxItems": 20,
      "items": {
        "$ref": "#/$defs/launch"
      }
    },
    "maximumLaunchP99Ns": {
      "type": "integer",
      "minimum": 0
    },
    "conPerf001LimitNs": {
      "const": 2000000
    },
    "pass": {
      "type": "boolean"
    },
    "harnessLog": {
      "$ref": "#/$defs/evidence"
    }
  },
  "$defs": {
    "sha40": {
      "type": "string",
      "pattern": "^[0-9a-f]{40}$"
    },
    "sha256": {
      "type": "string",
      "pattern": "^[0-9a-f]{64}$"
    },
    "evidence": {
      "type": "object",
      "additionalProperties": false,
      "required": ["path", "sha256"],
      "properties": {
        "path": {
          "type": "string",
          "minLength": 1
        },
        "sha256": {
          "$ref": "#/$defs/sha256"
        }
      }
    },
    "candidateSource": {
      "type": "object",
      "additionalProperties": false,
      "required": ["revision", "artifactSha256"],
      "properties": {
        "revision": {
          "$ref": "#/$defs/sha40"
        },
        "artifactSha256": {
          "$ref": "#/$defs/sha256"
        }
      }
    },
    "hardware": {
      "type": "object",
      "additionalProperties": false,
      "required": ["hardwareId", "gpuId", "driverVersion"],
      "properties": {
        "hardwareId": {
          "type": "string",
          "minLength": 1
        },
        "gpuId": {
          "type": "string",
          "minLength": 1
        },
        "driverVersion": {
          "type": "string",
          "minLength": 1
        }
      }
    },
    "frame": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "ordinal",
        "ordinaryVisits",
        "attemptedOrdinaryVisits",
        "intrinsicQueries",
        "textOperations",
        "applicationOwnedLayoutNs",
        "paintSubmissionNs",
        "aggregateNs"
      ],
      "properties": {
        "ordinal": {
          "type": "integer",
          "minimum": 1
        },
        "ordinaryVisits": {
          "type": "integer",
          "minimum": 0
        },
        "attemptedOrdinaryVisits": {
          "type": "integer",
          "minimum": 0
        },
        "intrinsicQueries": {
          "type": "integer",
          "minimum": 0
        },
        "textOperations": {
          "type": "integer",
          "minimum": 0
        },
        "applicationOwnedLayoutNs": {
          "type": "integer",
          "minimum": 0
        },
        "paintSubmissionNs": {
          "type": "integer",
          "minimum": 0
        },
        "aggregateNs": {
          "type": "integer",
          "minimum": 0
        }
      }
    },
    "launch": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "launch",
        "warmupFrames",
        "measuredFrames",
        "nearestRankP99Ns"
      ],
      "properties": {
        "launch": {
          "type": "integer",
          "minimum": 1
        },
        "warmupFrames": {
          "type": "array",
          "minItems": 300,
          "maxItems": 300,
          "items": {
            "$ref": "#/$defs/frame"
          }
        },
        "measuredFrames": {
          "type": "array",
          "minItems": 500,
          "maxItems": 500,
          "items": {
            "$ref": "#/$defs/frame"
          }
        },
        "nearestRankP99Ns": {
          "type": "integer",
          "minimum": 0
        }
      }
    }
  }
}
```

The following local schema-byte check parsed both proposed schemas and hashed the exact fenced bytes after Markdown formatting.

```text
09d96af49384e47ee6154f386af2ef771985516a61c843d561835654283bd7b1  layout-qualification-record.schema.json
76dfee7dfcdfdd49e2d67afdf83ab43c29dbb6513652a8023b0869a7d59293e2  layout-prequalification-run.schema.json
```

- Create `.constitution/tech-spec/data-models/layout-prequalification-suite.schema.json` with the exact bytes in the following code block. Its SHA-256 is `27e3a876f3b8d5e88ad43089a9eff0c7ce225a6d9cece5fcd789f7759c05c924`.

<!-- canonical-block: layout-prequalification-suite-schema -->

```text
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "urn:oxyflut:schema:layout-prequalification-suite:1",
  "title": "Oxyflut layout prequalification suite",
  "description": "The complete unscored layout-cap matrix. Each tuple identifies one candidate, environment, and passing corpus fixture run.",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "schemaVersion",
    "suiteId",
    "lockDigest",
    "corpusDigest",
    "countingRulesDigest",
    "recordSchemaDigest",
    "runSchemaDigest",
    "suiteSchemaDigest",
    "tuples"
  ],
  "properties": {
    "schemaVersion": {
      "const": "1.0.0"
    },
    "suiteId": {
      "const": "layout-prequalification-suite"
    },
    "lockDigest": {
      "$ref": "#/$defs/sha256"
    },
    "corpusDigest": {
      "$ref": "#/$defs/sha256"
    },
    "countingRulesDigest": {
      "$ref": "#/$defs/sha256"
    },
    "recordSchemaDigest": {
      "$ref": "#/$defs/sha256"
    },
    "runSchemaDigest": {
      "$ref": "#/$defs/sha256"
    },
    "suiteSchemaDigest": {
      "$ref": "#/$defs/sha256"
    },
    "tuples": {
      "type": "array",
      "minItems": 48,
      "maxItems": 48,
      "items": {
        "$ref": "#/$defs/tuple"
      }
    }
  },
  "$defs": {
    "sha256": {
      "type": "string",
      "pattern": "^[0-9a-f]{64}$"
    },
    "evidence": {
      "type": "object",
      "additionalProperties": false,
      "required": ["path", "sha256"],
      "properties": {
        "path": {
          "type": "string",
          "minLength": 1
        },
        "sha256": {
          "$ref": "#/$defs/sha256"
        }
      }
    },
    "tuple": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "candidate",
        "environment",
        "fixtureId",
        "runManifest",
        "pass"
      ],
      "properties": {
        "candidate": {
          "enum": ["focused", "integrated"]
        },
        "environment": {
          "enum": ["macos", "windows", "wayland", "x11"]
        },
        "fixtureId": {
          "enum": [
            "deep-box-064",
            "wide-box-1024",
            "nested-weighted-8x8x8",
            "lazy-10000-realized-64",
            "reordered-keyed-128",
            "custom-two-pass-256"
          ]
        },
        "runManifest": {
          "$ref": "#/$defs/evidence"
        },
        "pass": {
          "type": "boolean"
        }
      }
    }
  }
}
```

The suite's `suiteSchemaDigest` binds its exact raw schema bytes. The validator must SHA-256-check those bytes against the lock before parsing the schema or validating the suite. The schema's cardinality is necessary but insufficient: `xtask` must validate the exact tuple set, uniqueness, identities, referenced run-manifest bytes, and all pass values as stated below. The suite's `suiteId` and six identity digests make it one named, lock-bound unit instead of 48 independent submissions.

Round-6 suite-schema probe command and output:

```sh
jq empty /tmp/wf-epic-b/OXY-B005/layout-prequalification-suite.schema.json && sha256sum /tmp/wf-epic-b/OXY-B005/layout-prequalification-suite.schema.json && wc -c /tmp/wf-epic-b/OXY-B005/layout-prequalification-suite.schema.json
```

```text
27e3a876f3b8d5e88ad43089a9eff0c7ce225a6d9cece5fcd789f7759c05c924  /tmp/wf-epic-b/OXY-B005/layout-prequalification-suite.schema.json
2467 /tmp/wf-epic-b/OXY-B005/layout-prequalification-suite.schema.json
```

Round-6 reserialized the two changed schema blocks as UTF-8 JSON with no byte-order mark, 2-space indentation, LF line endings, and one trailing LF. It used Prettier 3.9.6 with `--parser json`; `cmp` confirmed that each exact fenced byte sequence equals the reserialized output.

```sh
prettier --parser json /tmp/wf-epic-b/OXY-B005/round6-record.schema.json > /tmp/wf-epic-b/OXY-B005/round6-record.reserialized.json && prettier --parser json /tmp/wf-epic-b/OXY-B005/round6-suite.schema.json > /tmp/wf-epic-b/OXY-B005/round6-suite.reserialized.json && cmp -s /tmp/wf-epic-b/OXY-B005/round6-record.schema.json /tmp/wf-epic-b/OXY-B005/round6-record.reserialized.json && cmp -s /tmp/wf-epic-b/OXY-B005/round6-suite.schema.json /tmp/wf-epic-b/OXY-B005/round6-suite.reserialized.json && sha256sum /tmp/wf-epic-b/OXY-B005/round6-{record,suite}.reserialized.json && wc -c /tmp/wf-epic-b/OXY-B005/round6-{record,suite}.reserialized.json
```

```text
09d96af49384e47ee6154f386af2ef771985516a61c843d561835654283bd7b1  /tmp/wf-epic-b/OXY-B005/round6-record.reserialized.json
27e3a876f3b8d5e88ad43089a9eff0c7ce225a6d9cece5fcd789f7759c05c924  /tmp/wf-epic-b/OXY-B005/round6-suite.reserialized.json
13109 /tmp/wf-epic-b/OXY-B005/round6-record.reserialized.json
 2467 /tmp/wf-epic-b/OXY-B005/round6-suite.reserialized.json
15576 total
```

- Create `qualification/staged/layout-visit-corpus.json` with the exact canonical UTF-8 bytes in "Durable corpus bytes". Retain the stated UTF-8, LF, no-byte-order-mark, 2-space-indent, and trailing-LF convention. The destination file must SHA-256 to `4972e43333984047b5a1d84200d5b89a29c5b59e47c5aca8773379320f2c6c84`; don't replace it with a generated fixture or a `/tmp` path.
- Create `qualification/staged/layout-visit-counting-rules.json` with the exact canonical UTF-8 bytes below. Its SHA-256 is `6cd0d7c7b06587525d9127f15cceecdd6f9c21b8a62be93c70c9b3756ca459c2`. The `policyCaps` identifier is `weighted`; `intrinsic-measure` and `text` are explicit nonordinary families. `CAP_CANDIDATE=2` is a probe input in this document, not the value of `measurementPolicy.layoutVisitCap`.

<!-- canonical-block: layout-visit-counting-rules -->

```text
{
  "schemaVersion": "1.0.0",
  "ordinaryVisit": "One requested regular child-layout invocation from a policy to a realized direct child in one root transaction.",
  "rootEntry": "The harness-initiated root transaction entry is not a child visit.",
  "attemptOrdering": [
    "Increment attemptedOrdinaryVisits immediately before the per-child cap check.",
    "On cap rejection, record the attempt and reject before invocation.",
    "Increment ordinaryVisits only after the child invocation completes."
  ],
  "policyCaps": {
    "single-pass-box": 1,
    "weighted": 1,
    "virtualized-lazy": 1,
    "custom-multi-pass": 2
  },
  "nonOrdinaryFamilies": ["intrinsic-measure", "text"],
  "excludedOperations": [
    "dry-or-intrinsic-query",
    "text-layout-or-shaping",
    "unrealized-collection-work",
    "collection-range-selection"
  ],
  "transactionAggregation": "Sum each policy-local result once at emission; do not recursively sum nested LayoutResult values."
}
```

- `.constitution/tech-spec/data-models/qualification-lock.schema.json`: apply the schema-semver rule that a new required field on an existing durable document is breaking and therefore needs a major bump. Replace `$id` with `urn:oxyflut:schema:qualification-lock:6` and the `schemaVersion` `const` with `"6.0.0"`.
- `.constitution/tech-spec/data-models/qualification-lock.schema.json` in `$defs.measurementPolicy.required`: add `layoutVisitCorpus`, `layoutQualificationRecordSchema`, `layoutPrequalificationRunSchema`, `layoutPrequalificationSuiteSchema`, `layoutVisitCountingRules`, and `layoutPrequalificationIdentities`.
- `.constitution/tech-spec/data-models/qualification-lock.schema.json` in `$defs.measurementPolicy.properties`: add exactly `"layoutVisitCorpus": { "$ref": "#/$defs/digestOrNull" }`, `"layoutQualificationRecordSchema": { "$ref": "#/$defs/digestOrNull" }`, `"layoutPrequalificationRunSchema": { "$ref": "#/$defs/digestOrNull" }`, `"layoutPrequalificationSuiteSchema": { "$ref": "#/$defs/digestOrNull" }`, `"layoutVisitCountingRules": { "$ref": "#/$defs/digestOrNull" }`, and `"layoutPrequalificationIdentities": { "$ref": "#/$defs/layoutPrequalificationIdentitiesOrNull" }`.
- `.constitution/tech-spec/data-models/qualification-lock.schema.json` in `$defs.resolvedMeasurementPolicy.properties`: add exactly `"layoutVisitCorpus": { "$ref": "#/$defs/sha256" }`, `"layoutQualificationRecordSchema": { "$ref": "#/$defs/sha256" }`, `"layoutPrequalificationRunSchema": { "$ref": "#/$defs/sha256" }`, `"layoutPrequalificationSuiteSchema": { "$ref": "#/$defs/sha256" }`, `"layoutVisitCountingRules": { "$ref": "#/$defs/sha256" }`, and `"layoutPrequalificationIdentities": { "$ref": "#/$defs/layoutPrequalificationIdentities" }`.
- `.constitution/tech-spec/data-models/qualification-lock.schema.json` in `$defs`: add `layoutPrequalificationIdentitiesOrNull` as `{"oneOf":[{"type":"null"},{"$ref":"#/$defs/layoutPrequalificationIdentities"}]}`; add `layoutPrequalificationIdentities` as an object with `additionalProperties: false`, required `focused` and `integrated` properties, and each property referencing `candidateEnvironmentIdentities`; add `candidateEnvironmentIdentities` as an object with `additionalProperties: false`, required `macos`, `windows`, `wayland`, and `x11` properties, and each property referencing `layoutPrequalificationIdentity`; add `layoutPrequalificationIdentity` as an object with `additionalProperties: false`, required `revision` and `artifactSha256` properties, and exactly `"revision": { "$ref": "#/$defs/sha40" }` and `"artifactSha256": { "$ref": "#/$defs/sha256" }`.
- `.constitution/tech-spec/contracts/qualification-lock.json`: migrate `schemaVersion` from `"5.0.0"` to `"6.0.0"`; add to `measurementPolicy` exactly `"layoutVisitCorpus": "4972e43333984047b5a1d84200d5b89a29c5b59e47c5aca8773379320f2c6c84"`, `"layoutQualificationRecordSchema": "09d96af49384e47ee6154f386af2ef771985516a61c843d561835654283bd7b1"`, `"layoutPrequalificationRunSchema": "76dfee7dfcdfdd49e2d67afdf83ab43c29dbb6513652a8023b0869a7d59293e2"`, `"layoutPrequalificationSuiteSchema": "27e3a876f3b8d5e88ad43089a9eff0c7ce225a6d9cece5fcd789f7759c05c924"`, `"layoutVisitCountingRules": "6cd0d7c7b06587525d9127f15cceecdd6f9c21b8a62be93c70c9b3756ca459c2"`, and `"layoutPrequalificationIdentities": null`; retain exactly `"sampleValidityRules": null`, `"layoutVisitCap": null`, `"candidateImplementationReady": false`, and `"measurementReady": false`.
- Create `.constitution/tech-spec/migrations/qualification-lock-v5-to-v6.md` with the title `# Qualification lock v5 to v6` and these required statements: preserve the byte-for-byte v5 input and its computed SHA-256 before writing the derived v6 instance; add the six `measurementPolicy` fields above; don't add migration metadata to the lock because its root rejects additional properties; preserve `sampleValidityRules: null`, `layoutVisitCap: null`, both readiness flags as `false`, and `layout-visit-cap` in both known-unknown arrays; and validate the source and derived documents with their respective schemas. The note must name the preserved v5 path and computed source digest after Stage 3 performs the migration; this report cannot supply an unfetched digest.
- `.constitution/tech-spec/stack.md`: set `Version` to `v0.16.0` and replace both active `v0.15.0` references in the Scope guard with `v0.16.0`. `.constitution/tech-spec/contracts/specification-phase.json`, `.constitution/tech-spec/contracts/platform-contracts.json`, and `.constitution/tech-spec/contracts/capability-traceability.json`: set each `specificationVersion` to exactly `"0.16.0"` in the same change so active cross-document equality remains valid.
- `.constitution/tech-spec/changelog.md`: prepend this exact entry before `## [v0.15.0] - 2026-08-26`.

<!-- canonical-block: qualification-lock-v6-changelog-entry -->

```text
## [v0.16.0] - 2026-08-28

### Added

- Added the versioned layout-qualification record, run, and suite manifests, canonical corpus and counting-rules bindings, and frozen prequalification-probe lock profile for the unscored layout-cap timing probe.

### Changed

- Advanced the qualification-lock schema and instance from v5 to v6 because new required durable fields are breaking.
- Added the `LayoutResult.attempted_ordinary_visits` public field and the `CandidateProbe::run_layout_fixture` qualification contract.
- Kept `measurementPolicy.layoutVisitCap` unresolved and both readiness flags false until the bounded timing probe supplies evidence.
```

- `.constitution/tech-spec/data-models/raw-measurement.schema.json`: make no extension for layout transactions. Its existing `additionalProperties: false` remains valid because the new companion schema owns these records.
- `.constitution/tech-spec/contracts/oxyflut-public.rs` in `LayoutResult`: replace the `node_visits` documentation with `Number of completed ordinary direct-child layout invocations issued by this policy; excludes the root entry, dry or intrinsic measurements, text operations, and rejected attempts.` Then add immediately after `node_visits` exactly `/// Number of ordinary direct-child layout requests issued by this policy; increment immediately before the per-child cap check, include a request rejected before invocation, and exclude the root entry, dry or intrinsic measurements, and text operations.\npub attempted_ordinary_visits: u32,`.
- `.constitution/tech-spec/contracts/oxyflut-public.rs` immediately before `LayoutResult`: add exactly `/// Migration note for v0.15.0 consumers: constructing or destructuring LayoutResult now requires attempted_ordinary_visits. Set it to the number of ordinary direct-child requests issued before the cap check; it can exceed node_visits only when a request is rejected before invocation.` This is the ADR-0003 pre-v1 public-contract migration note and the changelog entry above is its release note.
- `.constitution/tech-spec/contracts/oxyflut-qualification.rs` after `RawSample`: add a public `LayoutTransactionCounters` structure with `ordinary_visits: u64`, `attempted_ordinary_visits: u64`, `intrinsic_queries: u64`, and `text_operations: u64`. Its exact semantics are per root transaction: the harness observes every policy-local `LayoutResult` once as it is emitted, sums the two local ordinary counters without recursively summing child results, increments attempted before the cap check, and records rejected attempts only in attempted. Add `CandidateProbe::run_layout_fixture(&mut self, candidate: Candidate, environment: Environment, fixture_id: &str) -> Result<(GateResult<CapabilityId>, Vec<LayoutTransactionCounters>), Self::Error>;`. During each generated frame, the harness must start root layouts only through one instrumented dispatcher. The dispatcher assigns contiguous one-based transaction ordinals and emits exactly one record for each completed transaction. The schema writer records the complete frame paint-submission duration only on ordinal 1, records zero paint-submission duration on later ordinals, and attaches the required identities and timings to every transaction record.
- `xtask/src/contracts/readiness.rs` and `xtask/src/commands/environment/mod.rs`: replace `urn:oxyflut:schema:qualification-lock:5` with `urn:oxyflut:schema:qualification-lock:6`. Extend `xtask/src/commands/contracts.rs` so `validate_rust_contracts` compiles an external-client assertion that constructs `LayoutResult` with `attempted_ordinary_visits` and type-checks `CandidateProbe::run_layout_fixture`.
- Add `cargo +1.98.0 run -p xtask -- layout-prequalification validate --lock LOCK_PATH --corpus CORPUS_PATH --suite-schema SUITE_SCHEMA_PATH --suite SUITE_PATH --output RESULT_PATH`. The command must validate the v6 lock; read `CORPUS_PATH` as raw bytes, SHA-256-check it against `measurementPolicy.layoutVisitCorpus`, then parse it; read `SUITE_SCHEMA_PATH` as raw bytes, SHA-256-check it against `measurementPolicy.layoutPrequalificationSuiteSchema` before parsing that schema; then parse it and validate the suite against it, require `suiteSchemaDigest` to equal that raw-byte SHA-256 digest, and validate its 48 referenced run manifests and every referenced record; and SHA-256-check every run-manifest and record-set evidence reference against its raw bytes. It must SHA-256-bind every `lockDigest` to the supplied canonical lock bytes; write the canonical validated result to `RESULT_PATH`; print that result's SHA-256; and retain its `lockDigest`, corpus digest, suite-schema digest, suite digest, every run-manifest digest, every record-set digest, and every harness-log digest. It must require both readiness flags to be `false`, `sampleValidityRules` and `layoutVisitCap` to be `null`, and the resolved and nullable field sets in "Next bounded probe". It must compare the suite, every run, and every record identity with the lock, reject score emission and candidate-selection output, and reject a suite that isn't the complete passing matrix.
- `xtask/src/commands/layout_prequalification.rs` must apply these custom rules: SHA-256-check `--corpus CORPUS_PATH` before parsing it; SHA-256-check the exact raw `--suite-schema SUITE_SCHEMA_PATH` bytes against `measurementPolicy.layoutPrequalificationSuiteSchema` before parsing that schema; then parse it, validate the suite against it, and require `suiteSchemaDigest` to equal that raw-byte SHA-256 digest; recover each fixture's four counters and outcome from that parsed digest-bound corpus, never from validator fixture constants; map `expected.outcome: "pass"` to `capOutcome: "completed"` generically; reject a record when `attemptedOrdinaryVisits < ordinaryVisits`; reject it when `aggregateNs != applicationOwnedLayoutNs + paintSubmissionNs`; require a passing-fixture run to use one of the six corpus fixtures whose `expected.outcome` is `"pass"` and `capOutcome: "completed"`; require `valid: true` and no `exclusionReason`; require every `(launch, framePhase, frameOrdinal)` group to have a nonempty contiguous `transactionOrdinal` sequence beginning at 1; reject a missing, duplicate, or noncontiguous transaction ordinal; require ordinal 1 to carry the frame's paint-submission duration and every later ordinal to carry `paintSubmissionNs: 0`; require exactly launches 1 through 20; require exactly once each warmup ordinal 1 through 300 and measured ordinal 1 through 500 for every launch; reject duplicate or missing record keys; recompute every run-manifest frame summary by summing `ordinaryVisits`, `attemptedOrdinaryVisits`, `intrinsicQueries`, `textOperations`, `applicationOwnedLayoutNs`, `paintSubmissionNs`, and `aggregateNs` from every transaction in its complete record-set frame group; sort each launch's 500 measured frame totals, select the 495th smallest value (one-based rank 495 = ceil(0.99 × 500); zero-based index 494) as the nearest-rank p99, then take the maximum of the 20 p99 values, and require `pass` to equal whether that maximum is at most 2,000,000 ns. After validating all run manifests, require one suite with `suiteId: "layout-prequalification-suite"`, lock, corpus, counting-rules, record-schema, run-schema, and suite-schema digests equal to the v6 lock; exactly the 48 unique `(candidate, environment, fixtureId)` tuples in the Cartesian product of the two candidate values, four environment values, and six parsed passing corpus fixtures; a SHA-256-matching run manifest for each tuple; no duplicate tuple or referenced run; tuple values and `pass` equal to the referenced run values; every tuple and run `pass: true`; and every run's candidate source, hardware, GPU, driver, release flags, lock, corpus, counting-rules, and schema identities equal to the corresponding frozen lock identity. Reject any missing, extra, duplicate, failed, mismatched, or cherry-picked tuple. The command must reject a declared frame summary, percentile, maximum, pass value, cap outcome, counter, identity, run-manifest digest, record-set digest, suite-schema digest, or suite digest that differs from the recomputation or raw bytes.

The following pseudocode defines the zero-based rank used after sorting 500 measured frame totals.

```python
def nearest_rank_p99_ns(frame_totals):
    assert len(frame_totals) == 500
    # The 495th smallest value (one-based rank 495 = ceil(0.99 × 500); zero-based index 494).
    return sorted(frame_totals)[494]
```

- Add validator fixtures, outside the JSON Schema fixture directories, at `qualification/fixtures/layout-prequalification/valid/complete-run.json`, `qualification/fixtures/layout-prequalification/valid/complete-suite.json`, and `qualification/fixtures/layout-prequalification/invalid/attempted-less-than-ordinary.json`, `aggregate-arithmetic-mismatch.json`, `duplicate-frame-ordinal.json`, `duplicate-transaction-ordinal.json`, `missing-transaction-ordinal.json`, `noncontiguous-transaction-ordinal.json`, `later-transaction-paint-submission.json`, `missing-warmup-frame-ordinal.json`, `missing-measured-frame-ordinal.json`, `frame-summary-mismatch.json`, `nearest-rank-p99-mismatch.json`, `maximum-launch-p99-mismatch.json`, `pass-threshold-mismatch.json`, `record-set-digest-mismatch.json`, `identity-mismatch.json`, `corpus-digest-mismatch.json`, `suite-schema-lock-digest-mismatch.json`, `suite-schema-digest-mismatch.json`, `suite-missing-tuple.json`, `suite-extra-tuple.json`, `suite-duplicate-tuple.json`, `suite-run-manifest-digest-mismatch.json`, `suite-run-tuple-mismatch.json`, `suite-failed-run.json`, and `suite-shared-identity-mismatch.json`. Each invalid fixture must name the stable custom-validator error code in a matching `.expected.json` sidecar. The `suite-schema-lock-digest-mismatch` fixture supplies raw `--suite-schema` bytes that differ from the lock field. The `suite-schema-digest-mismatch` fixture supplies bytes that match the lock but a suite manifest whose `suiteSchemaDigest` differs. The complete-suite fixture must exercise all 48 tuples and valid SHA-256 evidence references. The `nearest-rank-p99-mismatch.json` fixture must declare a value different from the 495th smallest value (one-based rank 495 = ceil(0.99 × 500); zero-based index 494). Keep fixture-counter and cap-outcome mismatches in the `layout-qualification-record` schema fixture matrix because the `if`, `then`, and `else` branches express them.
- `xtask/src/contracts/schema.rs` automatically requires fixture directories for every schema. Add `qualification/fixtures/contracts/layout-qualification-record/valid/minimal.json`, plus `invalid/additional-properties.json`, `invalid/conditional.json`, `invalid/enum.json`, `invalid/required.json`, `invalid/type.json`, and `invalid/sample-exclusion.json` with a matching `.expected.json` sidecar for every invalid file. The valid fixture must bind all required record identities with `valid: true`; the invalid fixtures must separately reject a missing `countingRulesDigest`, a nonmatching fixture `capOutcome`, an invalid candidate or environment, an extra property, an invalid counter type, `valid: false`, an `exclusionReason`, and nonzero `paintSubmissionNs` after `transactionOrdinal: 1`. Add the same valid and schema-invalid fixture matrix for `qualification/fixtures/contracts/layout-prequalification-run/`; its valid fixture has 20 launches, 300 warmup frames, and 500 measured frames per launch. Add the same valid and schema-invalid fixture matrix for `qualification/fixtures/contracts/layout-prequalification-suite/`; its valid fixture contains exactly 48 syntactically valid tuple entries and a `suiteSchemaDigest`, while custom-validator fixtures prove cross-product completeness, uniqueness, digest binding, pass state, and shared identities.
- Update these existing qualification-lock schema fixtures and their necessary sidecars to v6 and the six new required `measurementPolicy` fields: `qualification/fixtures/contracts/qualification-lock/valid/minimal.json`; `qualification/fixtures/contracts/qualification-lock/invalid/additional-properties.json`; `qualification/fixtures/contracts/qualification-lock/invalid/additional-properties.expected.json`; `qualification/fixtures/contracts/qualification-lock/invalid/conditional.json`; `qualification/fixtures/contracts/qualification-lock/invalid/conditional.expected.json`; `qualification/fixtures/contracts/qualification-lock/invalid/enum.json`; `qualification/fixtures/contracts/qualification-lock/invalid/enum.expected.json`; `qualification/fixtures/contracts/qualification-lock/invalid/required.json`; `qualification/fixtures/contracts/qualification-lock/invalid/required.expected.json`; `qualification/fixtures/contracts/qualification-lock/invalid/type.json`; and `qualification/fixtures/contracts/qualification-lock/invalid/type.expected.json`. Keep `qualification/fixtures/contracts/qualification-lock/invalid/superseded-identity.json` at the v4 identity, but change `qualification/fixtures/contracts/qualification-lock/invalid/superseded-identity.expected.json` to `"supersededBy": "urn:oxyflut:schema:qualification-lock:6"` and change the qualification-lock entry in `qualification/fixtures/contracts/supersession.json` to `"superseded": "urn:oxyflut:schema:qualification-lock:5"` and `"current": "urn:oxyflut:schema:qualification-lock:6"`.
- Migrate the lock-bearing readiness fixtures so `xtask contracts validate` reaches their intended assertions with the six new `measurementPolicy` fields: `qualification/fixtures/contracts/readiness/ready/.constitution/tech-spec/contracts/qualification-lock.json`; `qualification/fixtures/contracts/readiness/production-3b/.constitution/tech-spec/contracts/qualification-lock.json`; `qualification/fixtures/contracts/readiness/ready/negative/missing-candidate-identities-lock.json`; `qualification/fixtures/contracts/readiness/ready/negative/unresolved-readiness-lock.json`; `qualification/fixtures/contracts/readiness/ready/negative/mismatched-typed-reference-lock.json`; `qualification/fixtures/contracts/readiness/ready/negative/synthetic-baseline-lock.json`; `qualification/fixtures/contracts/readiness/ready/negative/mismatched-engine-revision-lock.json`; `qualification/fixtures/contracts/readiness/ready/negative/unresolved-tool-fields-lock.json`; `qualification/fixtures/contracts/readiness/ready/negative/mismatched-tool-lock.json`; and `qualification/fixtures/contracts/readiness/ready/negative/missing-tool-lock.json`. Update their copied active `specificationVersion` values to `"0.16.0"` and regenerate only the digest-bound fixture artifacts that change because of this migration.
- `.constitution/tech-spec/stack.md` in the Scope guard paragraph beginning `The current qualification lock`: append exactly `Before candidateImplementationReady becomes true, Stage 4 may run unscored nonproduction candidate probes only to resolve a pre-implementation gating KU; each probe must use the frozen evidence contract and can't produce comparative scores or select a candidate.`

### Version migration inventory

The preserved `grep` found 56 active-version-dependent paths under `xtask/` and `qualification/`. For every `R` entry, replace the active literal `"0.15.0"` with exactly `"0.16.0"`. For every `R+D` entry, also regenerate every valid digest-bound parent, reference, fixture sidecar, and test assertion affected by the changed bytes. For every `R+N` entry, preserve the intentionally invalid digest or value after updating the active-version literal and revalidate its intended failure.

| File | Required Stage 3 change |
| :-- | :-- |
| `xtask/src/commands/lock_tests.rs` | R: update the corrupted platform-baseline source version and its expected validation path. |
| `xtask/src/contracts/readiness_tests.rs` | R: update both `candidate_input_issues` active-version arguments. |
| `xtask/src/contracts/traceability/fixtures.rs` | R+D: update the synthetic platform-baseline version and regenerate its hard-coded digest references. |
| `xtask/src/contracts/traceability/tests.rs` | R: update the three synthetic and absence-binding version literals. |
| `qualification/fixtures/baselines/approved-without-approval-evidence.json` | R. |
| `qualification/fixtures/baselines/complete.synthetic.json` | R. |
| `qualification/fixtures/baselines/duplicate-key.json` | R. |
| `qualification/fixtures/baselines/empty-evidence.json` | R. |
| `qualification/fixtures/baselines/extra-key.json` | R. |
| `qualification/fixtures/baselines/mismatched-flow.json` | R. |
| `qualification/fixtures/baselines/missing-key.json` | R. |
| `qualification/fixtures/baselines/synthetic-with-approval.json` | R. |
| `qualification/fixtures/contracts/capability-baseline/invalid/additional-properties.json` | R. |
| `qualification/fixtures/contracts/capability-baseline/invalid/conditional.json` | R. |
| `qualification/fixtures/contracts/capability-baseline/invalid/enum.json` | R. |
| `qualification/fixtures/contracts/capability-baseline/invalid/required.json` | R. |
| `qualification/fixtures/contracts/capability-baseline/invalid/type.json` | R. |
| `qualification/fixtures/contracts/capability-baseline/valid/minimal.json` | R. |
| `qualification/fixtures/contracts/capability-traceability/invalid/additional-properties.json` | R. |
| `qualification/fixtures/contracts/capability-traceability/invalid/enum.json` | R. |
| `qualification/fixtures/contracts/capability-traceability/invalid/required.json` | R. |
| `qualification/fixtures/contracts/capability-traceability/invalid/type.json` | R. |
| `qualification/fixtures/contracts/capability-traceability/valid/minimal.json` | R. |
| `qualification/fixtures/contracts/platform-contracts/invalid/additional-properties.json` | R. |
| `qualification/fixtures/contracts/platform-contracts/invalid/conditional.json` | R. |
| `qualification/fixtures/contracts/platform-contracts/invalid/enum.json` | R. |
| `qualification/fixtures/contracts/platform-contracts/invalid/required.json` | R. |
| `qualification/fixtures/contracts/platform-contracts/invalid/type.json` | R. |
| `qualification/fixtures/contracts/platform-contracts/valid/minimal.json` | R. |
| `qualification/fixtures/contracts/qualification-evidence/invalid/contradictory-pass-binding.json` | R. |
| `qualification/fixtures/contracts/qualification-evidence/valid/not-applicable-kk-binding.json` | R. |
| `qualification/fixtures/contracts/readiness/ready/.constitution/tech-spec/contracts/platform-contracts.json` | R+D: update the copied active specification and regenerate the ready-fixture lock/reference bindings. |
| `qualification/fixtures/contracts/readiness/ready/.constitution/tech-spec/contracts/specification-phase.json` | R+D: update the copied active specification and regenerate the ready-fixture lock/reference bindings. |
| `qualification/fixtures/contracts/readiness/ready/baselines/capability.json` | R+D: update the baseline and its valid lock/reference digest. |
| `qualification/fixtures/contracts/readiness/ready/baselines/synthetic.json` | R+D: update the baseline and its valid lock/reference digest. |
| `qualification/fixtures/contracts/readiness/ready/negative/missing-nested-kk-platform.json` | R+N: update the active version while preserving the fixture's intended rejection. |
| `qualification/fixtures/contracts/selection-decision/invalid/additional-properties.json` | R. |
| `qualification/fixtures/contracts/selection-decision/invalid/conditional.json` | R. |
| `qualification/fixtures/contracts/selection-decision/invalid/enum.json` | R. |
| `qualification/fixtures/contracts/selection-decision/invalid/required.json` | R. |
| `qualification/fixtures/contracts/selection-decision/invalid/type.json` | R. |
| `qualification/fixtures/contracts/selection-decision/valid/minimal.json` | R. |
| `qualification/fixtures/contracts/specification-phase/invalid/additional-properties.json` | R. |
| `qualification/fixtures/contracts/specification-phase/invalid/conditional.json` | R. |
| `qualification/fixtures/contracts/specification-phase/invalid/enum.json` | R. |
| `qualification/fixtures/contracts/specification-phase/invalid/required.json` | R. |
| `qualification/fixtures/contracts/specification-phase/invalid/type.json` | R. |
| `qualification/fixtures/contracts/specification-phase/valid/minimal.json` | R. |
| `qualification/fixtures/contracts/traceability/synthetic-capability-baseline-approved.json` | R+D: update the source fixture and every hard-coded digest that binds it. |
| `qualification/fixtures/contracts/traceability/synthetic-capability-baseline-malformed-entry.json` | R. |
| `qualification/fixtures/contracts/traceability/synthetic-capability-baseline-missing-approval-evidence.json` | R. |
| `qualification/fixtures/contracts/traceability/synthetic-capability-baseline-missing-schema-version.json` | R. |
| `qualification/fixtures/contracts/traceability/synthetic-capability-baseline-synthetic.json` | R. |
| `qualification/fixtures/contracts/traceability/synthetic-platform-baseline.json` | R+D: update the source fixture and every hard-coded digest that binds it. |
| `qualification/fixtures/evidence/bad-platform-baseline-reference.json` | R+N: update the embedded active version and retain the deliberately wrong all-zero digest. |
| `qualification/fixtures/evidence/schema-valid.json` | R+D: update the active version and regenerate every valid envelope or reference that hashes these bytes. |

### Layout prequalification additions inventory

These new v6 artifacts must land atomically with the migration above. `N` means a new artifact, `N+D` means a new artifact whose digest is bound by the v6 lock or by another new evidence file, and `N+V` means new custom-validator coverage.

| File or directory | Required Stage 3 change |
| :-- | :-- |
| `qualification/staged/layout-visit-corpus.json` | N+D: create from the exact bytes in "Durable corpus bytes" and verify SHA-256 `4972e43333984047b5a1d84200d5b89a29c5b59e47c5aca8773379320f2c6c84` before the v6 lock binds it. |
| `qualification/staged/layout-visit-counting-rules.json` | N+D: create from the exact fenced bytes and verify SHA-256 `6cd0d7c7b06587525d9127f15cceecdd6f9c21b8a62be93c70c9b3756ca459c2` before the v6 lock binds it. |
| `.constitution/tech-spec/data-models/layout-qualification-record.schema.json` | N+D: create the raw layout-transaction record schema from the exact fenced bytes and verify SHA-256 `09d96af49384e47ee6154f386af2ef771985516a61c843d561835654283bd7b1` before the v6 lock binds it. |
| `.constitution/tech-spec/data-models/layout-prequalification-run.schema.json` | N+D: create the per-candidate, environment, and passing-fixture run schema from the exact fenced bytes and verify SHA-256 `76dfee7dfcdfdd49e2d67afdf83ab43c29dbb6513652a8023b0869a7d59293e2` before the v6 lock binds it. |
| `.constitution/tech-spec/data-models/layout-prequalification-suite.schema.json` | N+D: create from the exact fenced bytes and verify SHA-256 `27e3a876f3b8d5e88ad43089a9eff0c7ce225a6d9cece5fcd789f7759c05c924` before the v6 lock binds it. |
| `.constitution/tech-spec/data-models/qualification-lock.schema.json` and `.constitution/tech-spec/contracts/qualification-lock.json` | R+D: add and bind `layoutPrequalificationSuiteSchema` with the other five new layout fields; preserve the v6 migration requirements already listed. |
| `xtask/src/commands/layout_prequalification.rs` and its command registration | N+V: implement `--corpus CORPUS_PATH` and `--suite-schema SUITE_SCHEMA_PATH` raw-byte digest binding, corpus-derived fixture expectations, variable-cardinality transaction-frame validation, and complete 48-tuple suite validation. |
| `qualification/fixtures/contracts/layout-prequalification-suite/` | N: add the schema-valid and schema-invalid matrix for the suite contract. |
| `qualification/fixtures/layout-prequalification/valid/complete-suite.json` and `qualification/fixtures/layout-prequalification/invalid/suite-*.json` | N+V: add the 48-tuple complete suite and the missing, extra, duplicate, `suite-schema-lock-digest-mismatch.json`, `suite-schema-digest-mismatch.json`, tuple, failed-run, and shared-identity negative cases with stable error-code sidecars. |

After the migration and digest regeneration, `cargo +1.98.0 run -p xtask -- contracts validate` and `cargo +1.98.0 test --workspace --all-features` must both pass.

### Canonical fenced-block integrity proposal

Stage 3 must add an `xtask` or CI check that extracts the exact body after each stable `canonical-block` anchor, including its terminal LF and excluding the fences, then SHA-256-checks the raw bytes before accepting a report change. Run the check after Prettier. The seven protected streams use `text` fences so Markdown formatting can't rewrite their bytes.

The check must cover these anchors and digests:

- `layout-visit-corpus`: `4972e43333984047b5a1d84200d5b89a29c5b59e47c5aca8773379320f2c6c84`.
- `layout-visit-topology-model-source`: `a0774355500de806c118316982dc6b781518f9b1134f6c9239d6f3fcc149ddff`.
- `layout-qualification-record-schema`: `09d96af49384e47ee6154f386af2ef771985516a61c843d561835654283bd7b1`.
- `layout-prequalification-run-schema`: `76dfee7dfcdfdd49e2d67afdf83ab43c29dbb6513652a8023b0869a7d59293e2`.
- `layout-prequalification-suite-schema`: `27e3a876f3b8d5e88ad43089a9eff0c7ce225a6d9cece5fcd789f7759c05c924`.
- `layout-visit-counting-rules`: `6cd0d7c7b06587525d9127f15cceecdd6f9c21b8a62be93c70c9b3756ca459c2`.
- `qualification-lock-v6-changelog-entry`: `7c271171a6cdda4515e7c96e26ac5db79cd05f1d5acc1d62e03ceb37853f2bb9`.

The correction run reserialized the changed counting-rules JSON with Prettier's JSON parser, checked its JSON syntax, and verified all seven fenced blocks after Markdown formatting. The verifier wrote only `/tmp/wf-epic-b/OXY-B005-pr-fix/` files.

```sh
prettier --prose-wrap never --write .constitution/spikes/SPK-B005.md
```

```text
.constitution/spikes/SPK-B005.md 313ms
```

```sh
set -euo pipefail
prettier --prose-wrap never --check .constitution/spikes/SPK-B005.md
perl /tmp/wf-epic-b/OXY-B005-pr-fix/verify-canonical-blocks.pl .constitution/spikes/SPK-B005.md /tmp/wf-epic-b/OXY-B005-pr-fix/canonical-blocks
prettier --parser json /tmp/wf-epic-b/OXY-B005-pr-fix/canonical-blocks/layout-visit-counting-rules > /tmp/wf-epic-b/OXY-B005-pr-fix/layout-visit-counting-rules.after-prettier.json
cmp -s /tmp/wf-epic-b/OXY-B005-pr-fix/canonical-blocks/layout-visit-counting-rules /tmp/wf-epic-b/OXY-B005-pr-fix/layout-visit-counting-rules.after-prettier.json
jq empty /tmp/wf-epic-b/OXY-B005-pr-fix/canonical-blocks/layout-visit-counting-rules
sha256sum /tmp/wf-epic-b/OXY-B005-pr-fix/layout-visit-counting-rules.after-prettier.json
printf 'counting_rules_reserialization=passed\n'
```

```text
Checking formatting...
All matched files use Prettier code style!
layout-visit-corpus|4972e43333984047b5a1d84200d5b89a29c5b59e47c5aca8773379320f2c6c84|6152|ok
layout-visit-topology-model-source|a0774355500de806c118316982dc6b781518f9b1134f6c9239d6f3fcc149ddff|18850|ok
layout-qualification-record-schema|09d96af49384e47ee6154f386af2ef771985516a61c843d561835654283bd7b1|13109|ok
layout-prequalification-run-schema|76dfee7dfcdfdd49e2d67afdf83ab43c29dbb6513652a8023b0869a7d59293e2|5479|ok
layout-prequalification-suite-schema|27e3a876f3b8d5e88ad43089a9eff0c7ce225a6d9cece5fcd789f7759c05c924|2467|ok
layout-visit-counting-rules|6cd0d7c7b06587525d9127f15cceecdd6f9c21b8a62be93c70c9b3756ca459c2|976|ok
qualification-lock-v6-changelog-entry|7c271171a6cdda4515e7c96e26ac5db79cd05f1d5acc1d62e03ceb37853f2bb9|651|ok
canonical_fenced_blocks=7
canonical_fence_assertions=passed
6cd0d7c7b06587525d9127f15cceecdd6f9c21b8a62be93c70c9b3756ca459c2  /tmp/wf-epic-b/OXY-B005-pr-fix/layout-visit-counting-rules.after-prettier.json
counting_rules_reserialization=passed
```

The PR round-2 inventory correction reran the seven-digest check after the inventory update. The verifier wrote only `/tmp/wf-epic-b/OXY-B005/canonical-blocks-round-2/` files.

```sh
set -euo pipefail
rm -rf /tmp/wf-epic-b/OXY-B005/canonical-blocks-round-2
perl /tmp/wf-epic-b/OXY-B005/verify-canonical-blocks-round-2.pl .constitution/spikes/SPK-B005.md /tmp/wf-epic-b/OXY-B005/canonical-blocks-round-2
```

```text
layout-visit-corpus|4972e43333984047b5a1d84200d5b89a29c5b59e47c5aca8773379320f2c6c84|6152|ok
layout-visit-topology-model-source|a0774355500de806c118316982dc6b781518f9b1134f6c9239d6f3fcc149ddff|18850|ok
layout-qualification-record-schema|09d96af49384e47ee6154f386af2ef771985516a61c843d561835654283bd7b1|13109|ok
layout-prequalification-run-schema|76dfee7dfcdfdd49e2d67afdf83ab43c29dbb6513652a8023b0869a7d59293e2|5479|ok
layout-prequalification-suite-schema|27e3a876f3b8d5e88ad43089a9eff0c7ce225a6d9cece5fcd789f7759c05c924|2467|ok
layout-visit-counting-rules|6cd0d7c7b06587525d9127f15cceecdd6f9c21b8a62be93c70c9b3756ca459c2|976|ok
qualification-lock-v6-changelog-entry|7c271171a6cdda4515e7c96e26ac5db79cd05f1d5acc1d62e03ceb37853f2bb9|651|ok
canonical_fenced_blocks=7
canonical_fence_assertions=passed
```

## Sources

- https://raw.githubusercontent.com/flutter/flutter/4cf24164269a5ebf0c16a028a00727d0e77bbb05/packages/flutter/lib/src/rendering/object.dart
- https://raw.githubusercontent.com/flutter/flutter/4cf24164269a5ebf0c16a028a00727d0e77bbb05/packages/flutter/lib/src/rendering/box.dart
- https://www.w3.org/TR/2018/CR-css-flexbox-1-20181119/
- https://raw.githubusercontent.com/facebook/yoga/bd8fe0d6d243cc7e0334d4cc68864a994f63beae/website/docs/advanced/external-layout-systems.mdx
- https://json-schema.org/understanding-json-schema/reference/conditionals
