# Spike report: OXY-B005 common-case layout visit cap

## Time box

- **Budget:** 1 focused day.
- **Clock start / stop:** 2026-08-28T16:43:22Z / 2026-08-28T16:52:52Z.

## Question

This table answers whether a platform-independent ordinary-layout corpus, counting rule, and finite per-node cap can be frozen without classifying intrinsic measurement or text work as ordinary visits.

Table 1. Decision questions and evidence

| Row | Question | Answer and evidence | Status | Next bounded probe |
| :-- | :-- | :-- | :-- | :-- |
| 1 | Can the corpus define deep, wide, nested, virtualized, reordered, and separation or failure cases without a substrate? | Yes. The canonical manifest contains 10 fixtures with topology, expected counters, and outcome. The preserved counter-model output validates each manifest row without a substrate API. | KK | Not applicable. |
| 2 | What is one ordinary visit? | One requested regular child-layout invocation from a policy to a realized direct child in one root transaction. The transaction entry is not a child request. Each requested ordinary invocation increments `attempted_ordinary_visits` before the cap check. A completed invocation also increments `node_visits`; a rejected request does not. Flutter documents [`RenderObject.layout`](https://api.flutter.dev/flutter/rendering/RenderObject/layout.html) as the parent-to-child layout entry point. The preserved output validates the rule. | KK | Not applicable. |
| 3 | Do the ordinary policy families have finite bounds under the classifier? | Yes. Single-pass box, definite-basis weighted, and realized virtualized policies complete at most one ordinary visit per realized child. A custom multi-pass policy completes at most two. The six passing manifest rows validate the derived bounds. | KK | Not applicable. |
| 4 | Can intrinsic or dry measurement consume an ordinary-policy visit? | No. Flutter documents [`RenderBox.getDryLayout`](https://api.flutter.dev/flutter/rendering/RenderBox/getDryLayout.html) as a state-free dry calculation and warns that it can produce O(N^2) behavior. The manifest and exact output record `ordinary=0`, `attempted=0`, and `intrinsic=1`, then reject the fixture from the ordinary family. | not applicable-with-citation | Not applicable. |
| 5 | Can text shaping or text layout consume an ordinary-policy visit? | No. Yoga documents [measure functions for external layout systems](https://www.yogalayout.dev/docs/advanced/external-layout-systems), including text. The manifest and exact output record one ordinary parent-to-text-leaf visit, one attempt, and one separate text operation. | not applicable-with-citation | Not applicable. |
| 6 | Can `2` freeze as `measurementPolicy.layoutVisitCap` while establishing compatibility with the 2.0 ms aggregate goal? | No. The counter model establishes counts, not nanosecond cost. The qualification lock lacks a reference workload, release flags, hardware identifiers, and candidate source identities. No preserved result measures application-owned layout plus paint submission. | KU (gating) | After Stage 3 authorizes nonproduction candidate probes, run the bounded timing probe in "Next bounded probe" on each locked reference configuration. |

## Context and objective

- **Triggering upstream file or section:** `.constitution/prd/constraints.md` defines the gating common-case node-visit limit, and `.constitution/tech-spec/contracts/qualification-lock.json` sets `measurementPolicy.layoutVisitCap` to `null`.
- **Target:** Freeze the corpus and counter semantics, then either freeze a numeric cap from performance evidence or retain a precise blocker.
- **Archetype / surface:** Library and SDK layout policy under system and built-in frame constraints.

## Codebase baseline

- **Status at probe start:** `LayoutResult.node_visits` reports participating-node visits made by a policy, CAP-LAY-001 requires bounded constraint propagation, and CON-PERF-001 limits aggregate application-owned layout and paint submission to 2.0 ms.
- **Discovered constraints:** The CAP-LAY-001 flow rejects a policy that exceeds its declared cap. The CAP-LAY-002 flow also stops custom policies that exceed a cap. The qualification lock retains `layout-visit-cap` in both known-unknown lists.
- **Boundary:** This report defines a qualification counter and corpus. It doesn't select a substrate, implement layout, change a capability, or relax CON-PERF-001.

## Reference corpus

The root is in each node total, has depth 1, and is the harness-initiated transaction entry. It isn't a child visit. Every nontext ordinary leaf has a fixed finite size. Every ordinary container receives valid finite constraints. A weighted fixture uses explicit finite weights and a definite main-axis size, so it has no content-derived basis.

The canonical corpus manifest contains all 10 fixtures. It is the sole source for fixture identity, topology, expected counters, and expected outcome. The tables transcribe the manifest for review; they don't add fixture data.

The manifest SHA-256 is `502be034a2795302eda483c471b71d82025513e497da842e7c672f80eceeb766`. Its exact byte serialization is UTF-8 encoded ASCII-only JSON, with keys in ASCII lexicographic order at every object level, 2-space indentation, a colon followed by one space, LF line endings, no byte-order mark, and one trailing LF byte after the final `]`. The following code block displays exactly those hashed bytes:

```json
[
  {
    "collection": null,
    "depth": 64,
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
    "operation": "One root and 63 one-child boxes.",
    "passes": 1
  },
  {
    "collection": null,
    "depth": 2,
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
    "operation": "One root and 1,024 fixed leaf children.",
    "passes": 1
  },
  {
    "collection": null,
    "depth": 4,
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
    "operation": "One root, eight weighted columns, 64 weighted rows, and 512 fixed leaves.",
    "passes": 1
  },
  {
    "collection": 10000,
    "depth": 2,
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
    "operation": "A 10,000-item collection realizes [4968,5032): 32 visible items and 16 cached items on each side. The root and 64 realized fixed leaves form the layout tree.",
    "passes": 1
  },
  {
    "collection": null,
    "depth": 2,
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
    "operation": "One weighted root reverses 128 fixed keyed leaves from key-000 through key-127 to key-127 through key-000, then lays out the realized tree.",
    "passes": 1
  },
  {
    "collection": null,
    "depth": 2,
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
    "operation": "One custom root issues two declared constraint passes to 256 fixed leaves.",
    "passes": 2
  },
  {
    "collection": null,
    "depth": 2,
    "expected": {
      "attempted_ordinary_visits": 33,
      "intrinsic_queries": 0,
      "maximum_ordinary_visits_per_node": 2,
      "ordinary_visits": 32,
      "outcome": "reject-cap-before-invocation-node-1",
      "text_operations": 0
    },
    "family": "custom-multi-pass",
    "id": "three-pass-cap-failure",
    "nodes": 17,
    "operation": "One custom root asks each of 16 fixed children for a third layout after two completed passes.",
    "passes": 3
  },
  {
    "collection": null,
    "depth": 2,
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
    "operation": "One box root receives invalid constraints before it can lay out 16 fixed children.",
    "passes": 0
  },
  {
    "collection": null,
    "depth": 2,
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
    "operation": "One root requests a dry or intrinsic answer from one nondefinite child.",
    "passes": 0
  },
  {
    "collection": null,
    "depth": 2,
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
    "operation": "One box root lays out one realized text leaf, which invokes text layout.",
    "passes": 1
  }
]
```

Table 2. Ordinary success corpus

| Fixture | Exact topology and operation | Nodes | Depth | Ordinary visits | Attempted ordinary visits | Intrinsic queries | Text operations | Maximum visits per node | Expected outcome |
| :-- | :-- | --: | --: | --: | --: | --: | --: | --: | :-- |
| `deep-box-064` | One root and 63 one-child boxes. | 64 | 64 | 63 | 63 | 0 | 0 | 1 | Pass. |
| `wide-box-1024` | One root and 1,024 fixed leaf children. | 1,025 | 2 | 1,024 | 1,024 | 0 | 0 | 1 | Pass. |
| `nested-weighted-8x8x8` | One root, eight weighted columns, 64 weighted rows, and 512 fixed leaves. | 585 | 4 | 584 | 584 | 0 | 0 | 1 | Pass. |
| `lazy-10000-realized-64` | A 10,000-item collection realizes `[4968,5032)`: 32 visible items and 16 cached items on each side. The root and 64 realized fixed leaves form the layout tree. | 65 realized | 2 | 64 | 64 | 0 | 0 | 1 | Pass. |
| `reordered-keyed-128` | One weighted root reverses 128 fixed keyed leaves from `key-000` through `key-127` to `key-127` through `key-000`, then lays out the realized tree. | 129 | 2 | 128 | 128 | 0 | 0 | 1 | Pass. |
| `custom-two-pass-256` | One custom root issues two declared constraint passes to 256 fixed leaves. | 257 | 2 | 512 | 512 | 0 | 0 | 2 | Pass. |

The lazy fixture issues no child-layout request for the 9,936 unrealized collection items. Collection indexing and range selection aren't ordinary visits and require separate timing evidence under CAP-SCR-001 and CON-PERF-001.

Table 3. Failure and separation corpus

| Fixture | Exact operation | Nodes | Depth | Ordinary visits | Attempted ordinary visits | Intrinsic queries | Text operations | Maximum visits per node | Expected outcome |
| :-- | :-- | --: | --: | --: | --: | --: | --: | --: | :-- |
| `three-pass-cap-failure` | One custom root asks each of 16 fixed children for a third layout after two completed passes. | 17 | 2 | 32 | 33 | 0 | 0 | 2 | Reject the third request to child 1 before invocation. |
| `invalid-constraints` | One box root receives invalid constraints before it can lay out 16 fixed children. | 17 | 2 | 0 | 0 | 0 | 0 | 0 | Reject before a child-layout request. |
| `intrinsic-separation` | One root requests a dry or intrinsic answer from one nondefinite child. | 2 | 2 | 0 | 0 | 1 | 0 | 0 | Reject from the ordinary family. |
| `text-separation` | One box root lays out one realized text leaf, which invokes text layout. | 2 | 2 | 1 | 1 | 0 | 1 | 1 | Record the text operation in a separate counter. |

## Counting model

Apply this algorithm to one root layout transaction:

1. Validate root constraints before issuing a child request. If validation fails, return a structured layout failure with zero `ordinary_visits` and zero `attempted_ordinary_visits`.
2. Classify each requested operation before it runs. An ordinary operation is a regular `layout` request from a policy to a realized direct child. A dry or intrinsic request and text-engine work use separate counters.
3. For every requested ordinary child invocation, increment the transaction's `attempted_ordinary_visits` before the cap check.
4. Compare the target child's completed ordinary-visit count for this transaction with the declaring policy-family cap. If the count equals the cap, reject the request before invocation and return a structured cap failure. Don't add the rejected request to `node_visits` or the completed ordinary count.
5. Otherwise, invoke the child. Increment that child's completed count and the issuing policy's `LayoutResult.node_visits`.
6. At transaction end, record `ordinary_visits` as the sum of emitted completed ordinary-visit events. Don't recursively sum nested `LayoutResult` values, because that double counts descendants.

This model makes `attempted_ordinary_visits` equal `ordinary_visits` for a transaction with no cap rejection. For `three-pass-cap-failure`, the 33rd requested ordinary invocation increments `attempted_ordinary_visits` and then fails the cap check. The transaction therefore records 32 completed ordinary visits and 33 attempts.

Table 4. Proposed ordinary-policy classifier

| Policy family | Admission rule | Cap per realized direct child | Excluded work |
| :-- | :-- | --: | :-- |
| Single-pass box | The policy issues one regular request to each participating child under valid finite constraints. | 1 | Intrinsic queries and text operations. |
| Definite-basis weighted | Weights, minimums, maximums, and main-axis space are finite and explicit. The policy issues one regular request after allocation. | 1 | Content-derived bases, dry or intrinsic measurement, and text operations. |
| Virtualized or lazy | The viewport has a declared realized range. Only realized children receive regular requests. | 1 | Offscreen collection work, range selection, intrinsic measurement, and text operations. |
| Custom multi-pass | The registered policy declares at most two regular passes and the harness checks each request before invocation. | 2 | Convergence loops beyond two passes, dry or intrinsic measurement, and text operations. |

A policy that needs content-derived sizing, a dry query, an intrinsic query, text work, or more than two ordinary passes isn't an ordinary fixture. It enters a dedicated evidence suite and can't omit its work from `node_visits`.

## Probe record

The nonproduction Perl counter model runs at `/tmp/wf-epic-b/OXY-B005/layout_visit_model.pl`. It validates only the manifest arithmetic, attempt-before-cap ordering, and cap-rejection rules. It doesn't measure a layout engine or frame time. The following is the complete executed probe source. It generates the canonical manifest with the serialization stated in "Reference corpus", so the report remains reproducible from committed Markdown alone.

```perl
#!/usr/bin/env perl
use strict;
use warnings;
use Digest::SHA qw(sha256_hex);
use JSON::PP;

# Candidate-neutral counter model for OXY-B005. This is not a layout engine.
my $CAP = 2;
my @fixtures = (
  {
    id => 'deep-box-064',
    family => 'single-pass-box',
    nodes => 64,
    depth => 64,
    passes => 1,
    collection => undef,
    operation => 'One root and 63 one-child boxes.',
    expected => {
      ordinary_visits => 63,
      attempted_ordinary_visits => 63,
      intrinsic_queries => 0,
      text_operations => 0,
      maximum_ordinary_visits_per_node => 1,
      outcome => 'pass',
    },
  },
  {
    id => 'wide-box-1024',
    family => 'single-pass-box',
    nodes => 1025,
    depth => 2,
    passes => 1,
    collection => undef,
    operation => 'One root and 1,024 fixed leaf children.',
    expected => {
      ordinary_visits => 1024,
      attempted_ordinary_visits => 1024,
      intrinsic_queries => 0,
      text_operations => 0,
      maximum_ordinary_visits_per_node => 1,
      outcome => 'pass',
    },
  },
  {
    id => 'nested-weighted-8x8x8',
    family => 'weighted',
    nodes => 585,
    depth => 4,
    passes => 1,
    collection => undef,
    operation => 'One root, eight weighted columns, 64 weighted rows, and 512 fixed leaves.',
    expected => {
      ordinary_visits => 584,
      attempted_ordinary_visits => 584,
      intrinsic_queries => 0,
      text_operations => 0,
      maximum_ordinary_visits_per_node => 1,
      outcome => 'pass',
    },
  },
  {
    id => 'lazy-10000-realized-64',
    family => 'virtualized-lazy',
    nodes => 65,
    depth => 2,
    passes => 1,
    collection => 10000,
    operation => 'A 10,000-item collection realizes [4968,5032): 32 visible items and 16 cached items on each side. The root and 64 realized fixed leaves form the layout tree.',
    expected => {
      ordinary_visits => 64,
      attempted_ordinary_visits => 64,
      intrinsic_queries => 0,
      text_operations => 0,
      maximum_ordinary_visits_per_node => 1,
      outcome => 'pass',
    },
  },
  {
    id => 'reordered-keyed-128',
    family => 'weighted',
    nodes => 129,
    depth => 2,
    passes => 1,
    collection => undef,
    operation => 'One weighted root reverses 128 fixed keyed leaves from key-000 through key-127 to key-127 through key-000, then lays out the realized tree.',
    expected => {
      ordinary_visits => 128,
      attempted_ordinary_visits => 128,
      intrinsic_queries => 0,
      text_operations => 0,
      maximum_ordinary_visits_per_node => 1,
      outcome => 'pass',
    },
  },
  {
    id => 'custom-two-pass-256',
    family => 'custom-multi-pass',
    nodes => 257,
    depth => 2,
    passes => 2,
    collection => undef,
    operation => 'One custom root issues two declared constraint passes to 256 fixed leaves.',
    expected => {
      ordinary_visits => 512,
      attempted_ordinary_visits => 512,
      intrinsic_queries => 0,
      text_operations => 0,
      maximum_ordinary_visits_per_node => 2,
      outcome => 'pass',
    },
  },
  {
    id => 'three-pass-cap-failure',
    family => 'custom-multi-pass',
    nodes => 17,
    depth => 2,
    passes => 3,
    collection => undef,
    operation => 'One custom root asks each of 16 fixed children for a third layout after two completed passes.',
    expected => {
      ordinary_visits => 32,
      attempted_ordinary_visits => 33,
      intrinsic_queries => 0,
      text_operations => 0,
      maximum_ordinary_visits_per_node => 2,
      outcome => 'reject-cap-before-invocation-node-1',
    },
  },
  {
    id => 'invalid-constraints',
    family => 'single-pass-box',
    nodes => 17,
    depth => 2,
    passes => 0,
    collection => undef,
    operation => 'One box root receives invalid constraints before it can lay out 16 fixed children.',
    expected => {
      ordinary_visits => 0,
      attempted_ordinary_visits => 0,
      intrinsic_queries => 0,
      text_operations => 0,
      maximum_ordinary_visits_per_node => 0,
      outcome => 'reject-before-child-layout',
    },
  },
  {
    id => 'intrinsic-separation',
    family => 'intrinsic-measure',
    nodes => 2,
    depth => 2,
    passes => 0,
    collection => undef,
    operation => 'One root requests a dry or intrinsic answer from one nondefinite child.',
    expected => {
      ordinary_visits => 0,
      attempted_ordinary_visits => 0,
      intrinsic_queries => 1,
      text_operations => 0,
      maximum_ordinary_visits_per_node => 0,
      outcome => 'reject-from-ordinary-family',
    },
  },
  {
    id => 'text-separation',
    family => 'text',
    nodes => 2,
    depth => 2,
    passes => 1,
    collection => undef,
    operation => 'One box root lays out one realized text leaf, which invokes text layout.',
    expected => {
      ordinary_visits => 1,
      attempted_ordinary_visits => 1,
      intrinsic_queries => 0,
      text_operations => 1,
      maximum_ordinary_visits_per_node => 1,
      outcome => 'separate-counter',
    },
  },
);

sub expect {
  my ($condition, $message) = @_;
  die "assertion failed: $message\n" if !$condition;
}

sub new_counter {
  return {
    cap => $CAP,
    completed_by_node => {},
    attempted_ordinary_visits => 0,
    intrinsic_queries => 0,
    text_operations => 0,
  };
}

sub ordinary_child_layout {
  my ($counter, $node) = @_;
  $counter->{attempted_ordinary_visits}++;
  my $completed = $counter->{completed_by_node}{$node} // 0;
  return 0 if $completed == $counter->{cap};
  $counter->{completed_by_node}{$node} = $completed + 1;
  return 1;
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
  my ($counter, $fixture) = @_;
  my $expected = $fixture->{expected};
  expect(ordinary_visits($counter) == $expected->{ordinary_visits}, 'ordinary visits');
  expect($counter->{attempted_ordinary_visits} == $expected->{attempted_ordinary_visits}, 'attempted ordinary visits');
  expect($counter->{intrinsic_queries} == $expected->{intrinsic_queries}, 'intrinsic queries');
  expect($counter->{text_operations} == $expected->{text_operations}, 'text operations');
  expect(maximum_ordinary_visits_per_node($counter) == $expected->{maximum_ordinary_visits_per_node}, 'maximum ordinary visits per node');
}

sub run_success {
  my ($fixture) = @_;
  my $counter = new_counter();
  for (1 .. $fixture->{passes}) {
    for my $node (1 .. ($fixture->{nodes} - 1)) {
      expect(ordinary_child_layout($counter, $node), 'success fixture exceeded cap');
    }
  }
  verify($counter, $fixture);
  return $counter;
}

sub run_three_pass_cap_failure {
  my ($fixture) = @_;
  my $counter = new_counter();
  my $failed_node;
  OUTER: for (1 .. $fixture->{passes}) {
    for my $node (1 .. ($fixture->{nodes} - 1)) {
      if (!ordinary_child_layout($counter, $node)) {
        $failed_node = $node;
        last OUTER;
      }
    }
  }
  expect($failed_node == 1, 'failed node');
  verify($counter, $fixture);
  return $counter;
}

sub run_invalid_constraints {
  my ($fixture) = @_;
  my $counter = new_counter();
  my $root_constraints_are_invalid = 1;
  expect($root_constraints_are_invalid, 'invalid root constraints');
  verify($counter, $fixture);
  return $counter;
}

sub run_intrinsic_separation {
  my ($fixture) = @_;
  my $counter = new_counter();
  $counter->{intrinsic_queries}++;
  verify($counter, $fixture);
  return $counter;
}

sub run_text_separation {
  my ($fixture) = @_;
  my $counter = new_counter();
  expect(ordinary_child_layout($counter, 1), 'text leaf layout');
  $counter->{text_operations}++;
  verify($counter, $fixture);
  return $counter;
}

sub run_fixture {
  my ($fixture) = @_;
  my $outcome = $fixture->{expected}{outcome};
  return run_success($fixture) if $outcome eq 'pass';
  return run_three_pass_cap_failure($fixture) if $outcome eq 'reject-cap-before-invocation-node-1';
  return run_invalid_constraints($fixture) if $outcome eq 'reject-before-child-layout';
  return run_intrinsic_separation($fixture) if $outcome eq 'reject-from-ordinary-family';
  return run_text_separation($fixture) if $outcome eq 'separate-counter';
  die "unknown outcome: $outcome\n";
}

@ARGV == 1 or die "usage: $0 MANIFEST_PATH\n";
my $manifest_json = JSON::PP->new->ascii->canonical->indent(1)->indent_length(2)->space_before(0)->space_after(1);
my $manifest = $manifest_json->encode(\@fixtures);
open my $manifest_file, '>:raw', $ARGV[0] or die "cannot write $ARGV[0]: $!\n";
print {$manifest_file} $manifest or die "cannot write $ARGV[0]: $!\n";
close $manifest_file or die "cannot close $ARGV[0]: $!\n";

print "OXY-B005 candidate-neutral counter model\n";
printf "cap=%d\n", $CAP;
printf "corpus_sha256=%s\n", sha256_hex($manifest);
print "fixture|family|nodes|depth|ordinary|attempted|intrinsic|text|max_per_node|result\n";
for my $fixture (@fixtures) {
  my $counter = run_fixture($fixture);
  my $expected = $fixture->{expected};
  printf "%s|%s|%d|%d|%d|%d|%d|%d|%d|%s\n", $fixture->{id}, $fixture->{family}, $fixture->{nodes}, $fixture->{depth}, ordinary_visits($counter), $counter->{attempted_ordinary_visits}, $counter->{intrinsic_queries}, $counter->{text_operations}, maximum_ordinary_visits_per_node($counter), $expected->{outcome};
}
print "assertions=passed\n";
```

Command run from the repository root:

```sh
perl /tmp/wf-epic-b/OXY-B005/layout_visit_model.pl /tmp/wf-epic-b/OXY-B005/layout_visit_corpus.json && sha256sum /tmp/wf-epic-b/OXY-B005/layout_visit_corpus.json
```

Exact captured output:

```text
OXY-B005 candidate-neutral counter model
cap=2
corpus_sha256=502be034a2795302eda483c471b71d82025513e497da842e7c672f80eceeb766
fixture|family|nodes|depth|ordinary|attempted|intrinsic|text|max_per_node|result
deep-box-064|single-pass-box|64|64|63|63|0|0|1|pass
wide-box-1024|single-pass-box|1025|2|1024|1024|0|0|1|pass
nested-weighted-8x8x8|weighted|585|4|584|584|0|0|1|pass
lazy-10000-realized-64|virtualized-lazy|65|2|64|64|0|0|1|pass
reordered-keyed-128|weighted|129|2|128|128|0|0|1|pass
custom-two-pass-256|custom-multi-pass|257|2|512|512|0|0|2|pass
three-pass-cap-failure|custom-multi-pass|17|2|32|33|0|0|2|reject-cap-before-invocation-node-1
invalid-constraints|single-pass-box|17|2|0|0|0|0|0|reject-before-child-layout
intrinsic-separation|intrinsic-measure|2|2|0|0|1|0|0|reject-from-ordinary-family
text-separation|text|2|2|1|1|0|1|1|separate-counter
assertions=passed
502be034a2795302eda483c471b71d82025513e497da842e7c672f80eceeb766  /tmp/wf-epic-b/OXY-B005/layout_visit_corpus.json
```

## Reference algorithm comparison

Flutter documents [`RenderObject.layout`](https://api.flutter.dev/flutter/rendering/RenderObject/layout.html) as the parent request for child layout and says that a parent's `performLayout` calls `layout` on each child. This supports counting the parent-issued request, not a geometry calculation or paint operation. Flutter documents [`RenderBox.getDryLayout`](https://api.flutter.dev/flutter/rendering/RenderBox/getDryLayout.html) as state-free and potentially O(N^2). The counter therefore records dry and intrinsic work separately.

The [CSS Flexible Box Layout Module Level 1](https://www.w3.org/TR/css-flexbox-1/#layout-algorithm) defines a flex layout algorithm with order-modified document order and intrinsic-size branches. The weighted corpus requires explicit definite inputs to avoid those branches. The corpus is a qualification counter model, not a claim that an Oxyflut policy implements the CSS algorithm.

Yoga documents [measure functions for external layout systems](https://www.yogalayout.dev/docs/advanced/external-layout-systems) for text and externally laid-out content. This supports a separate text counter. The ordinary text-leaf visit remains visible, but its text operation doesn't become another ordinary visit.

## Options and trade-offs

- Option A: Freeze the corpus, counting algorithm, and per-family algebraic bounds. This result supports Option A for counter semantics.
- Option B: Freeze one global cap for every ordinary policy. The lock can store one integer, but no timing evidence supports freezing `2` as that integer.
- Option C: Retain the numeric cap as a gating KU until an instrumented candidate probe measures the frozen corpus against CON-PERF-001. This preserves the target and prevents an intuition-based number.

## Recommendation

- **Chosen option:** Use a mix of A and C. Freeze the corpus digest and counting rules from this report. Choose C for the numeric `layoutVisitCap`; retain it as `null` and gating.
- **Derived threshold, not a freeze:** The corpus establishes that `2` is the smallest global threshold that admits the declared custom two-pass fixture. It isn't a performance recommendation because no result assigns time to a visit or reserves time for paint submission.
- **Why it fits:** The result preserves CAP-LAY-001's bounded propagation, keeps intrinsic and text work explicit, and doesn't weaken CON-PERF-001. The widest passing fixture has 1,024 ordinary visits, which gives an all-layout arithmetic ceiling of 1.953125 microseconds per visit under 2.0 ms. That ceiling leaves no measured paint allowance and isn't performance evidence.
- **Rejected options:** Reject a timing-only rule, an average-count rule, an unbounded intrinsic recursion, a text-work exemption hidden in `node_visits`, and a numeric threshold selected from shallow scenes.

### Next bounded probe

Stage 3 must first authorize unscored, nonproduction candidate probes before `candidateImplementationReady` changes. On each of the four locked reference configurations, run both instrumented candidate prototypes with `CAP_CANDIDATE=2` across the six success fixtures in table 2. For each fixture and prototype, run 20 launches, discard 300 warmup frames, and record 500 measured frames per launch.

Each raw record must contain the corpus digest, fixture ID, candidate source identity, hardware and driver identity, release flags, `ordinary_visits`, `attempted_ordinary_visits`, `intrinsic_queries`, `text_operations`, application-owned layout nanoseconds, paint-submission nanoseconds, and their aggregate. The expected successful output has the table 2 counters, no cap rejection, and a maximum of the 20 per-launch nearest-rank 99th percentiles at or below 2.0 ms. A failed value retains the KU and rejects the candidate. It doesn't increase the cap or change the corpus.

Without Stage 3 authorization, the timing result is circular: candidate implementation needs the numeric cap, and numeric compatibility needs candidate layout and paint-submission code. A host-only counter model can't close that evidence gap.

## Downstream impact

- **ADRs to write or update:** None. This report doesn't change an architecture decision.
- **Tickets unblocked in `tasks/active/`:** None. `OXY-D001` remains blocked by `layout-visit-cap`.
- **Tickets to add or split:** Add one bounded prequalification layout-cost prototype ticket only after Stage 3 authorizes the probe in "Next bounded probe".
- **Spec edits required:** Stage 3 must apply these exact edits without setting a numeric cap.
  - `.constitution/tech-spec/data-models/qualification-lock.schema.json` in `$defs.measurementPolicy.required`: add `layoutVisitCorpus`.
  - `.constitution/tech-spec/data-models/qualification-lock.schema.json` in `$defs.measurementPolicy.properties`: add `"layoutVisitCorpus": { "$ref": "#/$defs/digestOrNull" }`.
  - `.constitution/tech-spec/data-models/qualification-lock.schema.json` in `$defs.resolvedMeasurementPolicy.properties`: add `"layoutVisitCorpus": { "$ref": "#/$defs/sha256" }`.
  - `.constitution/tech-spec/contracts/qualification-lock.json` in `measurementPolicy`: add `"layoutVisitCorpus": "502be034a2795302eda483c471b71d82025513e497da842e7c672f80eceeb766"` and retain `"layoutVisitCap": null`.
  - `.constitution/tech-spec/contracts/oxyflut-public.rs` in the `LayoutResult.node_visits` documentation: replace the field description with `Number of completed ordinary direct-child layout invocations issued by this policy; excludes the root entry, dry or intrinsic measurements, text operations, and rejected attempts. Every requested ordinary invocation increments the transaction's attempted_ordinary_visits counter before the per-child cap check.`
  - `.constitution/tech-spec/stack.md` in the Scope guard paragraph that limits Stage 4 before `candidateImplementationReady`: append `Before candidateImplementationReady becomes true, Stage 4 may run unscored nonproduction candidate probes only to resolve a pre-implementation gating KU; the probes must use the frozen evidence contract and can't produce comparative scores or select a candidate.`

## Sources

All sources were fetched successfully through the Jina reader proxy during this spike.

- [Flutter `RenderObject.layout` API documentation](https://api.flutter.dev/flutter/rendering/RenderObject/layout.html)
- [Flutter `RenderBox.getDryLayout` API documentation](https://api.flutter.dev/flutter/rendering/RenderBox/getDryLayout.html)
- [CSS Flexible Box Layout Module Level 1](https://www.w3.org/TR/css-flexbox-1/#layout-algorithm)
- [Yoga: Integrating with external layout systems](https://www.yogalayout.dev/docs/advanced/external-layout-systems)
