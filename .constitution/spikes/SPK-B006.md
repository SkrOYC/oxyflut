# Spike report: OXY-B006 shared security patch and fuzz corpora

## Time box

- Budget: 1 focused day.
- Clock start / stop: 2026-08-28T16:22:21Z / 2026-08-28T16:46:01Z.

## Question

This spike decides which patch rehearsal and attributable seed corpus policy exercise both candidates before implementation.

Table 1. Decision answers

| Question | Status | Answer and evidence | Next bounded probe for a KU |
| :-- | :-- | :-- | :-- |
| Can a disclosed upstream engine patch apply to every frozen Flutter line and both consumption paths? | KU (gating) | No real patch is admissible. Probe P1 resolved the three framework-to-engine pins, found published focused SDK archives for all three engine revisions, and could retrieve engine source only for the middle revision. A patch cannot be checked against the missing first and third sources or the unpinned integrated fork. | At an authoritative source endpoint that serves all three engine revisions, fetch a disclosed fix commit, its parent, and each touched file. Run `git apply --check` at the three revisions, rebuild the focused SDK path, and inspect the integrated fork build graph. Expect six successful applicability checks and two build-graph records before replacing the synthetic policy. |
| Which shared patch rehearsal applies before implementation? | KK | Select `OXY-SYN-SEC-001`, a synthetic shared image-decoder hardening patch. Probe P2 confirms that Stage 3 assigns both candidates one bounded Rust decoder above the substrate boundary. The patch replaces unchecked RGBA byte-count multiplication with checked `u64` arithmetic and rejects overflow or more than 67,108,864 decoded bytes before allocation or adapter entry. | Not applicable. |
| What tests establish the synthetic patch result? | KK | The frozen tests are `checked_rgba_bytes_accepts_4096_by_4096_rgba`, `checked_rgba_bytes_rejects_4097_by_4096_rgba`, `checked_rgba_bytes_rejects_u32_max_square_without_decoder_or_adapter_call`, and `asset_decode_replays_image_registry`. Both candidates run the same shared Rust tests and image corpus. | Not applicable. |
| Can every architecture ingress receive attributable, licensed, capped seed material? | KK | The registry maps all eight architecture ingress categories to five immutable source sets. Every retained source has an exact URL, SHA-256, SPDX-compatible license identifier, license URL, observed size, and cap. Probe P3 records the architecture categories. Probe P4 records the source digests and sizes. | Not applicable. |
| Can the required memory, undefined-behavior, and concurrency instrumentation be frozen? | KK | `cargo-fuzz` 0.13.2 enables AddressSanitizer by default and exposes `--careful` and `--sanitizer thread`. A controlled `/tmp` probe ran `--careful` on `unreachable_unchecked` and stopped with the checked unsafe-precondition error. Miri is installed on the pinned nightly and is the required replay checker for every minimized finding. | Not applicable. |
| How is the policy made immutable and attributable? | KK | Stage 3 must copy the two canonical byte streams in this report, preserve their SHA-256 values, retain every source license notice, hash every fetched byte stream, and reject any source, license, size, or digest mismatch before a campaign starts. | Not applicable. |

## Context and objective

- Triggering requirements: `CON-SEC-001` through `CON-SEC-003`, `CAP-SEC-001`, and the `measurementPolicy.fuzzCorpora` and `measurementPolicy.securityPatchRehearsal` known unknowns.
- Target: One shared synthetic patch, exact tests, an ingress-complete seed registry, source attribution, immutable digests, payload caps, and instrumentation commands.
- Surface: Library/SDK and System/Native parser, artifact, callback, and unsafe-boundary inputs.

## Codebase baseline

- Stage 3 pins `cargo-fuzz` 0.13.2 and the common Rust `image` decoder with `gif`, `jpeg`, `png`, and `webp` features.
- The architecture registers eight invariant ingress categories and expands them only after candidate implementation identifies the actual parser surface.
- The focused and integrated adapter crates are qualification scaffolds. The integrated fork and Oxyflut adapter commits remain `ku-gating` in the qualification lock.
- Each implemented untrusted parser requires 24 CPU-hours with a 5-second timeout. Callback and teardown targets require 8 CPU-hours with thread instrumentation where the environment supports it.

## Options and trade-offs

- Option A: Select a disclosed renderer, text, image, or shared-dependency fix only after it applies to all three frozen lines and both consumption paths.
- Option B: Predeclare a minimal synthetic patch in the common Rust decoder and run the same tests and corpus through both candidates.
- Option C: Delay the policy until candidate ingress inventories exist.

## Recommendation

- Chosen option: B.
- Why it fits: `OXY-SYN-SEC-001` tests a safety boundary assigned above both adapters. It does not depend on a missing engine source tree, an unpinned fork, or a candidate-specific parser.
- Option A: Rejected for this rehearsal because P1 leaves two source revisions unavailable for patch application. This is the triggered STOP condition for the real-upstream-patch row only.
- Option C: Rejected because it leaves the required preimplementation corpus and rehearsal policy unfrozen.
- Rejected inputs: Candidate-specific patches, mutable branch references, source files without a license notice, unbounded corpus files, raw private content, and derived fixtures whose source digest is absent.

### Synthetic patch and expected result

`OXY-SYN-SEC-001` introduces `oxyflut_assets::decode::checked_rgba_bytes`. The function computes `width * height * 4` with `u64::checked_mul`, rejects overflow, rejects totals greater than `67_108_864`, converts to `usize` only after both checks, and runs before allocation or an adapter call.

The preimage and postimage must apply in the common Rust asset-decoder module, not either adapter. The patch file must contain only this guard and its four listed tests. The rehearsal must fail if the patch changes a candidate-specific file, touches unrelated code, permits either oversized input, or reaches an adapter for a rejected image.

The real upstream candidate inspected was [libpng commit `08da33b`](https://github.com/pnggroup/libpng/commit/08da33b4c88cfcd36e5a706558a8d7e0e4773643), titled "Fix a buffer overflow in `png_init_read_transformations`." P1 could not apply it to every required Flutter revision, and the accessible middle revision did not expose the tested libpng paths.

```text
$ curl raw libpng paths at engine 4c525dac5ebe5971c5708ef73558ed8edcf4a362
third_party/libpng/pngrutil.c 404
third_party/libpng/png.c 404
third_party/libpng/CHANGES 404
engine/src/third_party/libpng/pngrutil.c 404
```

The output of P1 follows:

```text
$ curl raw engine.version for every frozen framework commit
44a626f4f0027bc38a46dc68aed5964b05a83c18 -> 3452d735bd38224ef2db85ca763d862d6326b17f
559ffa3f75e7402d65a8def9c28389a9b2e6fe42 -> 4c525dac5ebe5971c5708ef73558ed8edcf4a362
4cf24164269a5ebf0c16a028a00727d0e77bbb05 -> 5f77625673248ee5846fbcaf5d3e1a3878386fd7
$ curl raw README for every resolved engine commit
3452d735bd38224ef2db85ca763d862d6326b17f 404 https://raw.githubusercontent.com/flutter-team-archive/engine/3452d735bd38224ef2db85ca763d862d6326b17f/README.md
4c525dac5ebe5971c5708ef73558ed8edcf4a362 200 https://raw.githubusercontent.com/flutter-team-archive/engine/4c525dac5ebe5971c5708ef73558ed8edcf4a362/README.md
5f77625673248ee5846fbcaf5d3e1a3878386fd7 404 https://raw.githubusercontent.com/flutter-team-archive/engine/5f77625673248ee5846fbcaf5d3e1a3878386fd7/README.md
$ curl -I focused Impeller SDK for every resolved engine commit
3452d735bd38224ef2db85ca763d862d6326b17f 200 https://storage.googleapis.com/flutter_infra_release/flutter/3452d735bd38224ef2db85ca763d862d6326b17f/linux-x64/impeller_sdk.zip
4c525dac5ebe5971c5708ef73558ed8edcf4a362 200 https://storage.googleapis.com/flutter_infra_release/flutter/4c525dac5ebe5971c5708ef73558ed8edcf4a362/linux-x64/impeller_sdk.zip
5f77625673248ee5846fbcaf5d3e1a3878386fd7 200 https://storage.googleapis.com/flutter_infra_release/flutter/5f77625673248ee5846fbcaf5d3e1a3878386fd7/linux-x64/impeller_sdk.zip
```

The output of P2 follows:

```text
$ grep shared decoder and candidate scaffold state
.constitution/tech-spec/stack.md:32:| Image decoding | `image` 0.25.10 with default features disabled and `gif`, `jpeg`, `png`, and `webp` enabled | Trial | Gives both candidates one shared bounded Rust decoder above the substrate boundary; adapters receive identical validated decoded pixels and never own image decoding. |
crates/oxyflut-assets/src/lib.rs:1://! Qualification-only scaffold for the asset and resource-manager boundary.
$ sha256sum source contract
0001d4812bbf5c39863e401db3fcb027f4dc3770b2ca506c65920c59aa0af27b  .constitution/tech-spec/contracts/oxyflut-substrate.h
```

### Instrumentation and campaign procedure

Use one physical core for each parser campaign. Require at least `86_400` process CPU seconds, a 5-second libFuzzer timeout, the ingress cap as `-max_len`, and a zero unresolved-report result. In the commands, `TARGET` is the implemented ingress fuzz target, `CORPUS` is its admitted corpus directory, `CAP` is the row's byte cap, `PACKAGE` owns the replay test, and `TEST_FILTER` selects that replay. Use `cargo +nightly fuzz run --sanitizer address --careful TARGET CORPUS -- -max_total_time=86400 -timeout=5 -max_len=CAP` for each implemented untrusted parser. Replay every minimized crash and every retained seed with `cargo +nightly miri test -p PACKAGE TEST_FILTER`. Run `cargo +nightly fuzz run --sanitizer thread TARGET CORPUS -- -max_total_time=28800 -timeout=5 -max_len=CAP` for callback and teardown targets where thread instrumentation runs.

The frozen tool identity is `cargo-fuzz` 0.13.2 with SHA-256 `db150590a2f9fa003fb167bc0eec3f90ba5574fcdd01f78110e6f397dda56582`, Rust nightly commit `3d6c19bb9ab4798ecfb2ee943df01a811720fc27` with SHA-256 `7de94a5c099c8d7ee4cafb905e36d882325faa480d8cff6513dd8c0887fac0c5`, and Miri SHA-256 `e779a4a85e5491ffa59ebff34e4af851c9e67dfa9f424212bd15f8fea0ca8bb8`. The Stage 3 tool lock must bind equivalent executables for every campaign host.

The relevant instrumentation output follows:

```text
$ cargo fuzz run --help
By default fuzz targets are built with debug assertions and overflow checks enabled.
Address Sanitizer is also enabled by default.
--sanitizer <SANITIZER>
[possible values: address, leak, memory, thread, none]
--careful
enable "careful" mode ... with debug assertions and extra const UB and init checks
$ LD_LIBRARY_PATH=LIBSTDCXX_DIR cargo +nightly fuzz run --careful careful_ub corpus -- -runs=1
unsafe precondition(s) violated: hint::unreachable_unchecked must never be reached
exit=1
```

### Frozen corpus sources

Table 2. Admitted source sets

| Set | Immutable origin and evidence | License and attribution | Cap | Observed maximum seed size |
| :-- | :-- | :-- | --: | --: |
| `image` | `image` v0.25.10 commit `76e57184f22772dad1138e96954e57945406b15e`; PNG, progressive JPEG, interlaced GIF, and animated alpha WebP digests appear in the canonical registry. | MIT OR Apache-2.0 under the fetched [Apache license](https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/LICENSE-APACHE) and [MIT license](https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/LICENSE-MIT). | 1,048,576 bytes | 52,286 bytes |
| `font` | Noto commit `ffebf8c1ee449e544955a7e813c54f9b73848eac`; Noto Sans Regular and Noto Sans Arabic Regular digests appear in the canonical registry. | OFL-1.1 under the fetched [Noto license](https://raw.githubusercontent.com/notofonts/noto-fonts/ffebf8c1ee449e544955a7e813c54f9b73848eac/LICENSE). | 1,048,576 bytes | 509,848 bytes |
| `unicode-text` | Unicode 16.0.0 `GraphemeBreakTest.txt` and `BidiTest.txt`; digests appear in the canonical registry. | Unicode-DFS-2016 under the fetched [Unicode license](https://www.unicode.org/license.txt). | 8,388,608 bytes | 7,959,988 bytes |
| `json` | JSONTestSuite commit `1ef36fa01286573e846ac449e8683f8833c5b26a`; valid, invalid-UTF-8, and missing-colon inputs appear in the canonical registry. | MIT under the fetched [JSONTestSuite license](https://raw.githubusercontent.com/nst/JSONTestSuite/1ef36fa01286573e846ac449e8683f8833c5b26a/LICENSE). | 65,536 bytes | 7 bytes |
| `wpt-events` | Web Platform Tests commit `461f7e8515940598535c71ae334e188eadde27a3`; clipboard, key, input, pointer, accessibility-property, and accessibility-action inputs appear in the canonical registry. | BSD-3-Clause under the fetched [Web Platform Tests license](https://raw.githubusercontent.com/web-platform-tests/wpt/461f7e8515940598535c71ae334e188eadde27a3/LICENSE.md). | 65,536 bytes | 7,783 bytes |

The output of P3 follows:

```text
Application assets, fonts, and images
Pointer, touch, keyboard, window, display, and lifecycle events
Input method editor, clipboard, and platform-message content
Accessibility properties and actions
Candidate callbacks, resources, and errors
Local-sink acknowledgement or failure
Candidate artifacts and evidence
Independent verification result
```

Table 3. Ingress-to-corpus and expected-test map

| Architecture ingress | Seed sets | Expected parser or boundary test | Cap |
| :-- | :-- | :-- | --: |
| Application assets, fonts, and images | `image`, `font`, `json` | `asset_decode`, `font_registration`, and `asset_manifest_json` reject malformed or oversized input before allocation. | 1,048,576 bytes, except `json` at 65,536 bytes |
| Operating-environment events | `wpt-events` | `platform_event_normalization` rejects invalid shape, stale identity, invalid order, and cap breach. | 65,536 bytes |
| Input method editor, clipboard, and platform-message content | `unicode-text`, `wpt-events` | `ime_transaction`, `clipboard_transaction`, and `platform_message` reject invalid range, index unit, sensitive-field, and payload size without recording content. | 8,388,608 bytes for Unicode seed files; 65,536 bytes for event seeds |
| Accessibility properties and actions | `unicode-text`, `wpt-events` | `semantics_update` and `semantics_action` reject invalid node, action, range, and private payload without retargeting. | 8,388,608 bytes for Unicode seed files; 65,536 bytes for event seeds |
| Candidate callbacks, resources, and errors | `json`, `wpt-events` | `abi_callback_decoder` and `abi_resource_validator` reject unknown status, invalid length, invalid enum, stale generation, and buffer overflow before candidate work. | 65,536 bytes |
| Local-sink acknowledgement or failure | `json` | `diagnostic_record` and `sink_ack` reject malformed records and failed acknowledgements without blocking producers. | 65,536 bytes |
| Candidate artifacts and evidence | `json` | `artifact_manifest`, provenance, and software-bill-of-materials parsers reject malformed JSON before qualification. | 65,536 bytes |
| Independent verification result | `json` | `verification_result` rejects an incomplete, malformed, or artifact-mismatched result and keeps qualification open. | 65,536 bytes |

The corpus importer may derive target-specific encodings only from a listed source after preserving the source SHA-256, the importer revision, the derived-file SHA-256, the license notice, and the target cap. It must reject a derived input that contains raw private content. A source branch, a release tag without a resolved commit, or a digest-less file is not admissible.

## Downstream impact

- ADRs to write or update: None.
- Tickets unblocked in `tasks/active/`: `OXY-D001` can materialize the two staged policy records. Candidate fuzz targets remain contingent on candidate implementation.
- Tickets to add or split: Create one implementation ticket for each actual parser ingress that the implementation inventory adds beyond table 3. Each ticket must inherit the same registry admission and campaign rules.
- Spec edits required:
  - Create the `qualification/staged/fuzz-corpora.json` file with the exact canonical bytes in the next section. Set `.constitution/tech-spec/contracts/qualification-lock.json` at `measurementPolicy.fuzzCorpora` to `a50008c4a320a6d9a4fe75c9e71d5db1b1365876e6c3aa224cd8bca5fa8e4a28`.
  - Create the `qualification/staged/security-patch-rehearsal.json` file with the exact canonical bytes in the next section. Set `.constitution/tech-spec/contracts/qualification-lock.json` at `measurementPolicy.securityPatchRehearsal` to `7dff0e97afaf5b6b4590aa4bf5bf9d28aadae4586d8d2c8f4707e2b52035e0dd`.
  - Add the three frozen tool identities from the instrumentation section to `.constitution/tech-spec/contracts/qualification-lock.json` at `resolvedTools`, retaining the field's existing gate until every campaign host has an equivalent pinned record.
  - After the two records and their source bytes pass admission, remove only `fuzz-corpora` and `security-patch-rehearsal` from `preImplementationKnownUnknowns` and `gatingKnownUnknowns`. Leave all unrelated readiness gates unchanged.

### Canonical staged inputs

The following source bytes produced the stated digests:

```text
a50008c4a320a6d9a4fe75c9e71d5db1b1365876e6c3aa224cd8bca5fa8e4a28  fuzz-corpora.json
7dff0e97afaf5b6b4590aa4bf5bf9d28aadae4586d8d2c8f4707e2b52035e0dd  security-patch-rehearsal.json
```

```json
{
  "schemaVersion": "1.0.0",
  "policyId": "OXY-B006-fuzz-corpora-v1",
  "admission": {
    "requireExactUrl": true,
    "requireSha256": true,
    "requireLicenseId": true,
    "requireLicenseUrl": true,
    "requireSizeAtMostCap": true,
    "rejectPrivateContent": true
  },
  "instrumentation": {
    "command": "cargo +nightly fuzz run --sanitizer address --careful TARGET CORPUS -- -max_total_time=86400 -timeout=5 -max_len=CAP",
    "address": "cargo-fuzz 0.13.2 default address sanitizer",
    "undefinedBehavior": "cargo-fuzz --careful plus cargo +nightly miri test replay of every minimized finding",
    "concurrency": "cargo +nightly fuzz run --sanitizer thread TARGET CORPUS -- -max_total_time=28800 -timeout=5 -max_len=CAP",
    "toolchain": {
      "cargoFuzzSha256": "db150590a2f9fa003fb167bc0eec3f90ba5574fcdd01f78110e6f397dda56582",
      "rustcCommit": "3d6c19bb9ab4798ecfb2ee943df01a811720fc27",
      "rustcSha256": "7de94a5c099c8d7ee4cafb905e36d882325faa480d8cff6513dd8c0887fac0c5",
      "miriSha256": "e779a4a85e5491ffa59ebff34e4af851c9e67dfa9f424212bd15f8fea0ca8bb8"
    }
  },
  "corpusSets": [
    {
      "id": "image",
      "capBytes": 1048576,
      "licenseId": "MIT OR Apache-2.0",
      "licenseUrl": "https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/LICENSE-APACHE",
      "sources": [
        [
          "png",
          "https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/tests/images/png/interlaced/basi2c08.png",
          "b2690d4475cdc39faf5a7d2de20de4e14eb96c6360fa791ec2003b4085ecacde"
        ],
        [
          "jpeg",
          "https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/tests/images/jpg/progressive/test.jpg",
          "4014ce154e09f4bc480f9cf6b00745ff76a86e37af8f94e694eea1b6e770edbe"
        ],
        [
          "gif",
          "https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/tests/images/gif/anim/interlaced.gif",
          "e5771812f4575268f9c1d84681254d0e5c59c37754c0cb1c94b6cfef0d861f44"
        ],
        [
          "webp",
          "https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/tests/images/webp/extended_images/advertises_rgba_but_frames_are_rgb.webp",
          "5fd83430e1dca3ffcced5bcac452147f30296ab83cae00d84603959ba96abfb6"
        ]
      ]
    },
    {
      "id": "font",
      "capBytes": 1048576,
      "licenseId": "OFL-1.1",
      "licenseUrl": "https://raw.githubusercontent.com/notofonts/noto-fonts/ffebf8c1ee449e544955a7e813c54f9b73848eac/LICENSE",
      "sources": [
        [
          "NotoSans-Regular.ttf",
          "https://raw.githubusercontent.com/notofonts/noto-fonts/ffebf8c1ee449e544955a7e813c54f9b73848eac/archive/hinted/NotoSans/NotoSans-Regular.ttf",
          "d78a4640e19e06c128e2041d480d5ddfd8db4fdecb3d582ca12b26aef1548bf9"
        ],
        [
          "NotoSansArabic-Regular.ttf",
          "https://raw.githubusercontent.com/notofonts/noto-fonts/ffebf8c1ee449e544955a7e813c54f9b73848eac/archive/hinted/NotoSansArabic/NotoSansArabic-Regular.ttf",
          "7ff958cd705f9e56a9e2755b9405dbb45b52cdd8a387a2bb2050083b0e488830"
        ]
      ]
    },
    {
      "id": "unicode-text",
      "capBytes": 8388608,
      "licenseId": "Unicode-DFS-2016",
      "licenseUrl": "https://www.unicode.org/license.txt",
      "sources": [
        [
          "GraphemeBreakTest.txt",
          "https://www.unicode.org/Public/16.0.0/ucd/auxiliary/GraphemeBreakTest.txt",
          "ee2b9354d270ac061b29f09662cafea06341d77e704b8cc6bd72aaeeda363cb5"
        ],
        [
          "BidiTest.txt",
          "https://www.unicode.org/Public/16.0.0/ucd/BidiTest.txt",
          "93e5eb9d88ca89dcf895f5576486a3363762ad2aa8f2db2fa56fe60cb82b9520"
        ]
      ]
    },
    {
      "id": "json",
      "capBytes": 65536,
      "licenseId": "MIT",
      "licenseUrl": "https://raw.githubusercontent.com/nst/JSONTestSuite/1ef36fa01286573e846ac449e8683f8833c5b26a/LICENSE",
      "sources": [
        [
          "valid",
          "https://raw.githubusercontent.com/nst/JSONTestSuite/1ef36fa01286573e846ac449e8683f8833c5b26a/test_parsing/y_structure_true_in_array.json",
          "1c28f2eb0958c3d15db1f0f0e7f2b8998ca2b8f67ab426a1fbb3d561fe76fad9"
        ],
        [
          "invalid-utf8",
          "https://raw.githubusercontent.com/nst/JSONTestSuite/1ef36fa01286573e846ac449e8683f8833c5b26a/test_parsing/n_array_invalid_utf8.json",
          "379af949f1f0fe32439c2c960df7adf60d3a858b8c640c858fb780fb79bf5c94"
        ],
        [
          "missing-colon",
          "https://raw.githubusercontent.com/nst/JSONTestSuite/1ef36fa01286573e846ac449e8683f8833c5b26a/test_parsing/n_object_missing_colon.json",
          "f1a260a986fa42f6c6c33dc3f130bfd3af488acd84ce8573095f30d23eed1861"
        ]
      ]
    },
    {
      "id": "wpt-events",
      "capBytes": 65536,
      "licenseId": "BSD-3-Clause",
      "licenseUrl": "https://raw.githubusercontent.com/web-platform-tests/wpt/461f7e8515940598535c71ae334e188eadde27a3/LICENSE.md",
      "sources": [
        [
          "clipboard-write",
          "https://raw.githubusercontent.com/web-platform-tests/wpt/461f7e8515940598535c71ae334e188eadde27a3/clipboard-apis/async-navigator-clipboard-write-domstring.https.html",
          "05117804a0ae232bc63daa52e36e5087692a427534f1cb9fa15724007f01a6be"
        ],
        [
          "clipboard-event",
          "https://raw.githubusercontent.com/web-platform-tests/wpt/461f7e8515940598535c71ae334e188eadde27a3/clipboard-apis/clipboard-events-synthetic.html",
          "2a435d3264a69d9a117e7bbb7f7d5185059a45baa64d17fd927b2610e48fbf41"
        ],
        [
          "key",
          "https://raw.githubusercontent.com/web-platform-tests/wpt/461f7e8515940598535c71ae334e188eadde27a3/uievents/keyboard/keyboardevent-composed.html",
          "85896d0f923d0d8512034f13eccd75baf6dddbbc3f4e2044212b57a90b2f7033"
        ],
        [
          "input",
          "https://raw.githubusercontent.com/web-platform-tests/wpt/461f7e8515940598535c71ae334e188eadde27a3/uievents/constructors/inputevent-constructor.html",
          "19529c6d1b4d8598ba09009b9a4808a4dd5a778356fb2e081144a324c9971cde"
        ],
        [
          "pointer",
          "https://raw.githubusercontent.com/web-platform-tests/wpt/461f7e8515940598535c71ae334e188eadde27a3/pointerevents/pointerevent_attributes.html",
          "026095d92a46116740f7c8c354bccda7c90ef79fc6c8b0eeddd8a26511e7f8ed"
        ],
        [
          "accessibility-properties",
          "https://raw.githubusercontent.com/web-platform-tests/wpt/461f7e8515940598535c71ae334e188eadde27a3/wai-aria/accessibility_properties_basic.tentative.html",
          "b2910a4661ae5046991c6ed5d301faae243ad89669d4af621549ca5af75faf69"
        ],
        [
          "accessibility-action",
          "https://raw.githubusercontent.com/web-platform-tests/wpt/461f7e8515940598535c71ae334e188eadde27a3/wai-aria/aria-actions/aria-actions-target-accname-from-aria-label.tentative.html",
          "965ab73007f1ad6dde5a363e6dab141079421d55acb01b3d6ba0c6bf32c3584e"
        ]
      ]
    }
  ],
  "ingressMapping": {
    "application-assets": ["image", "font", "json"],
    "operating-environment-events": ["wpt-events"],
    "private-platform-content": ["unicode-text", "wpt-events"],
    "accessibility": ["unicode-text", "wpt-events"],
    "candidate-boundary": ["json", "wpt-events"],
    "local-sink": ["json"],
    "candidate-artifacts": ["json"],
    "independent-verification": ["json"]
  }
}
```

```json
{
  "schemaVersion": "1.0.0",
  "policyId": "OXY-SYN-SEC-001",
  "kind": "synthetic",
  "scope": "candidate-neutral Rust image decoder above the rendering-substrate boundary",
  "function": "oxyflut_assets::decode::checked_rgba_bytes",
  "change": "replace unchecked width * height * 4 arithmetic with u64 checked multiplication, reject overflow and byte totals greater than 67108864 before allocation or adapter call",
  "tests": [
    "checked_rgba_bytes_accepts_4096_by_4096_rgba",
    "checked_rgba_bytes_rejects_4097_by_4096_rgba",
    "checked_rgba_bytes_rejects_u32_max_square_without_decoder_or_adapter_call",
    "asset_decode_replays_image_registry"
  ],
  "rehearsal": [
    "git apply --check OXY-SYN-SEC-001.patch in the shared Rust workspace",
    "cargo test -p oxyflut-assets checked_rgba_bytes",
    "cargo +nightly fuzz run --sanitizer address --careful asset_decode CORPUS -- -max_total_time=86400 -timeout=5 -max_len=1048576",
    "cargo +nightly miri test -p oxyflut-assets minimized_asset_decode_findings"
  ],
  "acceptance": "both focused and integrated builds execute the identical shared Rust tests and replay the identical corpus; neither adapter receives an allocation request for the rejected dimensions",
  "evidence": [
    "patch SHA-256",
    "preimage and postimage SHA-256",
    "focused and integrated command logs",
    "minimized findings or explicit none-found record",
    "tool identity digest"
  ]
}
```

## Sources

- [Flutter 3.41.0 engine pin](https://raw.githubusercontent.com/flutter/flutter/44a626f4f0027bc38a46dc68aed5964b05a83c18/bin/internal/engine.version)
- [Flutter 3.44.0 engine pin](https://raw.githubusercontent.com/flutter/flutter/559ffa3f75e7402d65a8def9c28389a9b2e6fe42/bin/internal/engine.version)
- [Flutter 3.47.0 engine pin](https://raw.githubusercontent.com/flutter/flutter/4cf24164269a5ebf0c16a028a00727d0e77bbb05/bin/internal/engine.version)
- [cargo-fuzz README](https://raw.githubusercontent.com/rust-fuzz/cargo-fuzz/main/README.md)
- [Miri README](https://raw.githubusercontent.com/rust-lang/miri/master/README.md)
- [image v0.25.10 annotated tag](https://api.github.com/repos/image-rs/image/git/tags/cf06bc9f66bb9423b1d5231af29cb5dd02bb4fa1)
- [Noto source commit](https://api.github.com/repos/googlefonts/noto-fonts/commits/ffebf8c1ee449e544955a7e813c54f9b73848eac)
- [JSONTestSuite source commit](https://api.github.com/repos/nst/JSONTestSuite/commits/1ef36fa01286573e846ac449e8683f8833c5b26a)
- [Web Platform Tests source commit](https://api.github.com/repos/web-platform-tests/wpt/commits/461f7e8515940598535c71ae334e188eadde27a3)
