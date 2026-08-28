# Spike report: OXY-B006 shared security patch and fuzz corpora

## Time box

- Status: Completed.
- Budget: 1 focused day.
- Clock start / stop: 2026-08-28T17:02:39Z / 2026-08-28T17:23:34Z.

## Question

This spike decides which patch rehearsal and attributable seed corpus policy exercise both candidates before implementation.

Table 1. Decision answers

| Question | Status | Answer and evidence | Next bounded probe for a KU |
| :-- | :-- | :-- | :-- |
| Can a disclosed upstream engine patch apply to every frozen Flutter line and both consumption paths? | KU (gating) | P1 tried both repositories: `flutter/flutter` resolves all three engine-revision [`DEPS` files](https://raw.githubusercontent.com/flutter/flutter/5f77625673248ee5846fbcaf5d3e1a3878386fd7/DEPS), while `flutter/engine` resolves only 3.44.0 with identical bytes. The resolved pins are `f139fd5d...` for 3.41.0 and `b6004397...` for 3.44.0 and 3.47.0; P1 fetched their actual `pngrtran.c` files. Both pins contain the `08da33b` postimage, so that real patch is already incorporated and cannot rehearse remediation. The 3.47.0 upstream focused SDK and full-engine graphs consume libpng; P1 preserves the GN chain and SDK archive evidence. The Oxyflut integrated fork has no source identity, so its actual consumption remains unverified. | Pin the integrated-fork commit and fetch its `DEPS`, `build/secondary/third_party/libpng/BUILD.gn`, `impeller/toolkit/interop/BUILD.gn`, and final GN dependency graph. Expect the fork revision, its libpng pin, and both focused and integrated `pngrtran.c` object paths before a real patch can replace the synthetic rehearsal. |
| Which shared patch rehearsal applies before implementation? | KK | Select `OXY-SYN-SEC-001`, a synthetic shared image-decoder hardening patch. The pinned stack assigns both candidates one bounded Rust decoder above the substrate boundary. The patch replaces unchecked RGBA byte-count multiplication with checked `u64` arithmetic and rejects overflow or more than 67,108,864 decoded bytes before allocation or adapter entry. | Not applicable. |
| What tests establish the synthetic patch result? | KK | The frozen tests are `checked_rgba_bytes_accepts_4096_by_4096_rgba`, `checked_rgba_bytes_rejects_4097_by_4096_rgba`, `checked_rgba_bytes_rejects_u32_max_square_without_decoder_or_adapter_call`, and `asset_decode_replays_image_registry`. Both candidates must run the same shared Rust tests and image corpus. | Not applicable. |
| Can every architecture ingress receive attributable, licensed, capped seed material? | KK | P3 maps all eight architecture ingress categories to five immutable source sets. P4 re-fetched and SHA-256-verified all 18 retained seed bytes and six license notices. The Unicode 16.0.0 ReadMe is dated 2024-08-25, and the same-day immutable License V3 snapshot hashes to `f5062c9a...`; Unicode documents the SPDX identifier as `Unicode-3.0`. | Not applicable. |
| Can the required memory, undefined-behavior, and concurrency instrumentation be frozen? | KK | P5 preserves the LLVM definition of `-max_total_time` as a maximum run time, not CPU accounting. P6 records GNU Time's user and system fields. The campaign policy requires cumulative process CPU across resumed corpus shards, a 5-second timeout, dated `nightly-2026-08-11`, and executable-hash preflight. | Not applicable. |
| How is the policy made immutable and attributable? | KK | Stage 3 must copy the two canonical byte streams, preserve their SHA-256 values, retain and hash every license notice and seed byte stream, require a host tool record before each campaign, and reject any source, license, size, tool, or digest mismatch. | Not applicable. |

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
- Why it fits: `OXY-SYN-SEC-001` tests a safety boundary assigned above both adapters. The candidate `08da33b` libpng fix is already incorporated by every frozen upstream engine pin, so it cannot be a remediation rehearsal; the unpinned integrated fork also prevents proving a real patch path.
- Option A: Rejected for this rehearsal because P1 proves the candidate patch has no remaining preimage on 3.41.0, 3.44.0, or 3.47.0, and the integrated fork has no source identity. This triggers the real-upstream-patch STOP condition only.
- Option C: Rejected because it leaves the required preimplementation corpus and rehearsal policy unfrozen.
- Rejected inputs: Candidate-specific patches, mutable branch references, source files without a license notice, unbounded corpus files, raw private content, and derived fixtures whose source digest is absent.

### Synthetic patch and expected result

`OXY-SYN-SEC-001` introduces `oxyflut_assets::decode::checked_rgba_bytes`. The function computes `width * height * 4` with `u64::checked_mul`, rejects overflow, rejects totals greater than `67_108_864`, converts to `usize` only after both checks, and runs before allocation or an adapter call.

The preimage and postimage must apply in the common Rust asset-decoder module, not either adapter. The patch file must contain only this guard and its four listed tests. The rehearsal must fail if the patch changes a candidate-specific file, touches unrelated code, permits either oversized input, or reaches an adapter for a rejected image.

The real upstream candidate was [libpng commit `08da33b`](https://github.com/pnggroup/libpng/commit/08da33b4c88cfcd36e5a706558a8d7e0e4773643), titled "Fix a buffer overflow in `png_init_read_transformations`." P1 used the actual Flutter monorepo, not the obsolete archive. The root [`DEPS` files](https://raw.githubusercontent.com/flutter/flutter/3452d735bd38224ef2db85ca763d862d6326b17f/DEPS) map `3452d...` to `f139fd5d...` and `4c525...` and `5f776...` to `b6004397...`. Both fetched [`pngrtran.c` postimages](https://flutter.googlesource.com/third_party/libpng/+/b6004397d2ab98f0250376d9b357337b7f422d13/pngrtran.c?format=TEXT) contain the `PNG_FLAG_OPTIMIZE_ALPHA` branch and checked component arithmetic introduced by `08da33b`. The real patch is therefore already incorporated on all three frozen lines and is unusable as a rehearsal patch.

For the focused 3.47.0 SDK, [`interop_base`](https://raw.githubusercontent.com/flutter/flutter/5f77625673248ee5846fbcaf5d3e1a3878386fd7/engine/src/flutter/impeller/toolkit/interop/BUILD.gn) depends on the Flutter display list, which depends on Skia; [`png_decode_libpng`](https://raw.githubusercontent.com/flutter/flutter/5f77625673248ee5846fbcaf5d3e1a3878386fd7/engine/src/flutter/skia/BUILD.gn) in turn depends on `//flutter/third_party/libpng`. The fetched Linux SDK static archive contains `libpng.pngrtran.o` and `skia_png_init_read_transformations`. For the upstream full engine, [`shell/common`](https://raw.githubusercontent.com/flutter/flutter/5f77625673248ee5846fbcaf5d3e1a3878386fd7/engine/src/flutter/shell/common/BUILD.gn) depends on the same Skia target. The libpng [`BUILD.gn`](https://flutter.googlesource.com/third_party/libpng/+/b6004397d2ab98f0250376d9b357337b7f422d13/BUILD.gn?format=TEXT) compiles `pngrtran.c`. This is KK for the pinned upstream focused SDK and full-engine source graph, not for the unspecified integrated fork.

The output of P1 follows:

```text
$ try flutter/flutter and flutter/engine <engine-revision>/DEPS
3452d735bd38224ef2db85ca763d862d6326b17f flutter/flutter=200 flutter/engine=404
4c525dac5ebe5971c5708ef73558ed8edcf4a362 flutter/flutter=200 flutter/engine=200 identical sha256=c5a8557661cc4d4a76a612bfb1fd81a11814b553566cbbd4b847b51e685ec93c
5f77625673248ee5846fbcaf5d3e1a3878386fd7 flutter/flutter=200 flutter/engine=404
$ use flutter/flutter DEPS for every frozen engine revision
3452d735bd38224ef2db85ca763d862d6326b17f 200 sha256=34e98c6f0d1caa46ace21913bb587eafe961d6f7fc9d7086961d232cd4a2179b
engine/src/flutter/third_party/libpng @ f139fd5d80944f5453b079672e50f32ca98ef076
4c525dac5ebe5971c5708ef73558ed8edcf4a362 200 sha256=c5a8557661cc4d4a76a612bfb1fd81a11814b553566cbbd4b847b51e685ec93c
engine/src/flutter/third_party/libpng @ b6004397d2ab98f0250376d9b357337b7f422d13
5f77625673248ee5846fbcaf5d3e1a3878386fd7 200 sha256=6844fe02248df123c7aa4823e2e50230ee62c853043211f38938033792544ea0
engine/src/flutter/third_party/libpng @ b6004397d2ab98f0250376d9b357337b7f422d13
$ fetch and base64-decode flutter.googlesource.com/third_party/libpng/+/PIN/pngrtran.c?format=TEXT
f139fd5d80944f5453b079672e50f32ca98ef076 sha256=185fceeb5b00b8ef55f571cbc7ea5a04a87a146d9a8eabcb9ad9b91b43d80fad lines: 1784 PNG_FLAG_OPTIMIZE_ALPHA; 1786 Premultiply only; 1793 component * png_ptr->trans_alpha[i]
b6004397d2ab98f0250376d9b357337b7f422d13 sha256=1462c8f9097342782b2180b791a83f7cc03b64b3554a8f13bd78cf6bc261c469 lines: 1784 PNG_FLAG_OPTIMIZE_ALPHA; 1786 Premultiply only; 1793 component * png_ptr->trans_alpha[i]
$ ar t linux-x64 Impeller SDK 5f77625673248ee5846fbcaf5d3e1a3878386fd7/lib/libimpeller.a | grep libpng.pngrtran
libpng.pngrtran.o
$ nm -a libimpeller.a | grep skia_png_init_read_transformations
0000000000000000 T skia_png_init_read_transformations
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

Use one physical core for each parser campaign. Require at least `86_400` process CPU seconds per implemented untrusted parser ingress and `28_800` process CPU seconds for each supported thread-instrumented callback or teardown target. Require a 5-second libFuzzer timeout, the ingress cap as `-max_len`, and a zero unresolved-report result. In the commands, `TARGET` is the implemented ingress fuzz target, `CORPUS` is its admitted persistent corpus directory, `CAP` is the row's byte cap, `FUZZ_EXE` is the built target executable, `CPU_LOG` is the GNU Time output, `PACKAGE` owns the replay test, and `TEST_FILTER` selects that replay.

[`-max_total_time`](https://llvm.org/docs/LibFuzzer.html) is an elapsed-time maximum for one fuzzer invocation. It is only an operational bound. It cannot establish CON-SEC-001 or CON-SEC-002 process-CPU coverage. Before a campaign, install and select `nightly-2026-08-11`, then run this preflight; a mismatch fails before build or execution:

```sh
TOOLCHAIN=nightly-2026-08-11
RUSTC_BIN="$(rustup which rustc --toolchain "$TOOLCHAIN")"
CARGO_BIN="$(rustup which cargo --toolchain "$TOOLCHAIN")"
CARGO_MIRI_BIN="$(rustup which cargo-miri --toolchain "$TOOLCHAIN")"
CARGO_FUZZ_BIN="$(command -v cargo-fuzz)"
test "$(rustc +"$TOOLCHAIN" -vV | awk '/^commit-hash:/ {print $2}')" = "3d6c19bb9ab4798ecfb2ee943df01a811720fc27"
printf '%s  %s\n' \
  '7de94a5c099c8d7ee4cafb905e36d882325faa480d8cff6513dd8c0887fac0c5' "$RUSTC_BIN" \
  '1cf1cd7feded113706026c5f04fad33e45364546e3c0d92ddee0c1a4c8277296' "$CARGO_BIN" \
  '40a69668c9ff4e5df3e6a87531f2b87dcc5c84e705ee5b06f915fb76383c94af' "$CARGO_MIRI_BIN" \
  'db150590a2f9fa003fb167bc0eec3f90ba5574fcdd01f78110e6f397dda56582' "$CARGO_FUZZ_BIN" | sha256sum -c -
test "$(cargo +"$TOOLCHAIN" fuzz --version)" = 'cargo-fuzz 0.13.2'
```

Build each sanitizer target with `cargo +nightly-2026-08-11 fuzz build --sanitizer address TARGET` or `cargo +nightly-2026-08-11 fuzz build --sanitizer thread TARGET`. Time the resulting executable directly with a host-recorded GNU Time path: `TIME_BIN -v -o CPU_LOG FUZZ_EXE CORPUS -max_total_time=28800 -timeout=5 -max_len=CAP`. Preserve `User time (seconds)`, `System time (seconds)`, and elapsed wall time from each `CPU_LOG`. Add user plus system seconds only for successful shards using the same target, sanitizer, and persistent `CORPUS`; resume until the applicable threshold is reached. Keep `CORPUS` as the first libFuzzer corpus directory. Before admitting another input directory, use `FUZZ_EXE -merge=1 CORPUS NEW_INPUTS`, then resume the timed target. The same LLVM documentation states that the first corpus directory receives new inputs and describes `-merge=1` and resumable merge control files. Replay every minimized crash and retained seed with `cargo +nightly-2026-08-11 miri test -p PACKAGE TEST_FILTER`.

The captured host record identifies `nightly` only as the probe selector, not as an admissible campaign selector: `rustc 1.99.0-nightly` reports commit `3d6c19bb9ab4798ecfb2ee943df01a811720fc27` and date `2026-08-11`. The dated selector is required for campaigns. This host records SHA-256 values for `rustc` `7de94a5c099c8d7ee4cafb905e36d882325faa480d8cff6513dd8c0887fac0c5`, `cargo` `1cf1cd7feded113706026c5f04fad33e45364546e3c0d92ddee0c1a4c8277296`, `cargo-miri` `40a69668c9ff4e5df3e6a87531f2b87dcc5c84e705ee5b06f915fb76383c94af`, `cargo-fuzz` 0.13.2 `db150590a2f9fa003fb167bc0eec3f90ba5574fcdd01f78110e6f397dda56582`, and GNU Time 1.10 `e8b9f5440e01a81e0692e68d07dfacb8059c434cae100c1fbb60b7ec52848480`. Stage 3 must stage an equivalent complete record for every campaign host and retain `resolved-tool-digests` as a gate until it does.

The relevant instrumentation output follows:

```text
$ rustc +nightly -vV
rustc 1.99.0-nightly (3d6c19bb9 2026-08-11)
commit-hash: 3d6c19bb9ab4798ecfb2ee943df01a811720fc27
commit-date: 2026-08-11
host: x86_64-unknown-linux-gnu
$ sha256sum cargo-fuzz rustc cargo cargo-miri
cargo-fuzz  db150590a2f9fa003fb167bc0eec3f90ba5574fcdd01f78110e6f397dda56582
rustc       7de94a5c099c8d7ee4cafb905e36d882325faa480d8cff6513dd8c0887fac0c5
cargo       1cf1cd7feded113706026c5f04fad33e45364546e3c0d92ddee0c1a4c8277296
cargo-miri  40a69668c9ff4e5df3e6a87531f2b87dcc5c84e705ee5b06f915fb76383c94af
$ /run/current-system/sw/bin/time -v true
User time (seconds): 0.00
System time (seconds): 0.00
Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.00
```

### Frozen corpus sources

Table 2. Admitted source sets

| Set | Immutable origin and evidence | License and attribution | Cap | Observed maximum seed size |
| :-- | :-- | :-- | --: | --: |
| `image` | `image` v0.25.10 commit `76e57184f22772dad1138e96954e57945406b15e`; PNG, progressive JPEG, interlaced GIF, and animated alpha WebP digests appear in the canonical registry. | MIT OR Apache-2.0 under the fetched [Apache license](https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/LICENSE-APACHE), SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`, and [MIT license](https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/LICENSE-MIT). | 1,048,576 bytes | 52,286 bytes |
| `font` | Noto commit `ffebf8c1ee449e544955a7e813c54f9b73848eac`; Noto Sans Regular and Noto Sans Arabic Regular digests appear in the canonical registry. | OFL-1.1 under the fetched [Noto license](https://raw.githubusercontent.com/notofonts/noto-fonts/ffebf8c1ee449e544955a7e813c54f9b73848eac/LICENSE), SHA-256 `0dab92d0544f7b233403f14b84a663bdbfa746982eda629e7f4f9ffe1b036feb`. | 1,048,576 bytes | 509,848 bytes |
| `unicode-text` | Unicode 16.0.0 `GraphemeBreakTest.txt` and `BidiTest.txt`; digests appear in the canonical registry. | Unicode-3.0 under the [dated 2024-08-25 Unicode License V3 snapshot](https://web.archive.org/web/20240825031908id_/https://www.unicode.org/license.txt), SHA-256 `f5062c9a188d81dfe66b56db4182dcf9e4b17c0d9b0d311a8e20b3a1b075c443`. | 8,388,608 bytes | 7,959,988 bytes |
| `json` | JSONTestSuite commit `1ef36fa01286573e846ac449e8683f8833c5b26a`; valid, invalid-UTF-8, and missing-colon inputs appear in the canonical registry. | MIT under the fetched [JSONTestSuite license](https://raw.githubusercontent.com/nst/JSONTestSuite/1ef36fa01286573e846ac449e8683f8833c5b26a/LICENSE), SHA-256 `8bd0e0578be788c617ea01d18b2a8146e3746ae50bddadc65a5f9d3aad08ad49`. | 65,536 bytes | 7 bytes |
| `wpt-events` | Web Platform Tests commit `461f7e8515940598535c71ae334e188eadde27a3`; clipboard, key, input, pointer, accessibility-property, and accessibility-action inputs appear in the canonical registry. | BSD-3-Clause under the fetched [Web Platform Tests license](https://raw.githubusercontent.com/web-platform-tests/wpt/461f7e8515940598535c71ae334e188eadde27a3/LICENSE.md), SHA-256 `5fac07febb0e2a97fb0d7b0def149ec08b642e1ba4b9c345283ab1cbd2af6570`. | 65,536 bytes | 7,783 bytes |

The output of P4 follows:

```text
$ fetch every canonical seed URL and compare SHA-256
PASS image-png bytes=315 sha256=b2690d4475cdc39faf5a7d2de20de4e14eb96c6360fa791ec2003b4085ecacde
PASS image-jpeg bytes=3744 sha256=4014ce154e09f4bc480f9cf6b00745ff76a86e37af8f94e694eea1b6e770edbe
PASS image-gif bytes=1526 sha256=e5771812f4575268f9c1d84681254d0e5c59c37754c0cb1c94b6cfef0d861f44
PASS image-webp bytes=52286 sha256=5fd83430e1dca3ffcced5bcac452147f30296ab83cae00d84603959ba96abfb6
PASS font-noto-sans bytes=509848 sha256=d78a4640e19e06c128e2041d480d5ddfd8db4fdecb3d582ca12b26aef1548bf9
PASS font-noto-arabic bytes=260740 sha256=7ff958cd705f9e56a9e2755b9405dbb45b52cdd8a387a2bb2050083b0e488830
PASS unicode-grapheme bytes=171927 sha256=ee2b9354d270ac061b29f09662cafea06341d77e704b8cc6bd72aaeeda363cb5
PASS unicode-bidi bytes=7959988 sha256=93e5eb9d88ca89dcf895f5576486a3363762ad2aa8f2db2fa56fe60cb82b9520
PASS json-valid bytes=6 sha256=1c28f2eb0958c3d15db1f0f0e7f2b8998ca2b8f67ab426a1fbb3d561fe76fad9
PASS json-invalid-utf8 bytes=3 sha256=379af949f1f0fe32439c2c960df7adf60d3a858b8c640c858fb780fb79bf5c94
PASS json-missing-colon bytes=7 sha256=f1a260a986fa42f6c6c33dc3f130bfd3af488acd84ce8573095f30d23eed1861
PASS wpt-clipboard-write bytes=4924 sha256=05117804a0ae232bc63daa52e36e5087692a427534f1cb9fa15724007f01a6be
PASS wpt-clipboard-event bytes=1481 sha256=2a435d3264a69d9a117e7bbb7f7d5185059a45baa64d17fd927b2610e48fbf41
PASS wpt-key bytes=1575 sha256=85896d0f923d0d8512034f13eccd75baf6dddbbc3f4e2044212b57a90b2f7033
PASS wpt-input bytes=967 sha256=19529c6d1b4d8598ba09009b9a4808a4dd5a778356fb2e081144a324c9971cde
PASS wpt-pointer bytes=7783 sha256=026095d92a46116740f7c8c354bccda7c90ef79fc6c8b0eeddd8a26511e7f8ed
PASS wpt-accessibility-properties bytes=2490 sha256=b2910a4661ae5046991c6ed5d301faae243ad89669d4af621549ca5af75faf69
PASS wpt-accessibility-action bytes=1441 sha256=965ab73007f1ad6dde5a363e6dab141079421d55acb01b3d6ba0c6bf32c3584e
$ fetch versioned UCD ReadMe and dated license snapshot
UCD 16.0.0 ReadMe 2024-08-25 sha256=14cafa23788d3a20dd21d6b0cdcb8d6dab520781fcd9ad9392f3b88ea607e633
Unicode License V3 snapshot 2024-08-25 1995 bytes sha256=f5062c9a188d81dfe66b56db4182dcf9e4b17c0d9b0d311a8e20b3a1b075c443
```

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
  - `qualification/staged/fuzz-corpora.json`: create this file with the exact canonical bytes in the next section. `.constitution/tech-spec/contracts/qualification-lock.json`, `measurementPolicy.fuzzCorpora`: set the value to `863f8e25ac176fb5cc68e254e281ca94df6eff62a800418cc6fd15a8c30fe83e`.
  - `qualification/staged/security-patch-rehearsal.json`: create this file with the exact canonical bytes in the next section. `.constitution/tech-spec/contracts/qualification-lock.json`, `measurementPolicy.securityPatchRehearsal`: set the value to `eb3b364063704fcec77363d5fb50bf052ac9018e146e02205eef350c07eef3d9`.
  - `.constitution/tech-spec/contracts/qualification-lock.json`, `resolvedTools`: append the exact `instrumentation.campaignToolchain.hostToolRecords[0]` record from `qualification/staged/fuzz-corpora.json`; require an equivalent complete record, including executable hashes and CPU-accounting fields, for every campaign host. Retain `resolved-tool-digests` in both known-unknown arrays until that condition holds.
  - `.constitution/tech-spec/contracts/qualification-lock.json`, `preImplementationKnownUnknowns` and `gatingKnownUnknowns`: after both staged records and every listed source and license byte pass admission, remove only `fuzz-corpora` and `security-patch-rehearsal`. Leave all unrelated readiness gates unchanged.

### Canonical staged inputs

The following source bytes produced the stated digests:

```text
863f8e25ac176fb5cc68e254e281ca94df6eff62a800418cc6fd15a8c30fe83e  fuzz-corpora.json
eb3b364063704fcec77363d5fb50bf052ac9018e146e02205eef350c07eef3d9  security-patch-rehearsal.json
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
    "requireLicenseSha256": true,
    "requireSizeAtMostCap": true,
    "rejectPrivateContent": true
  },
  "instrumentation": {
    "addressRequiredProcessCpuSeconds": 86400,
    "concurrencyRequiredProcessCpuSeconds": 28800,
    "buildCommand": "cargo +nightly-2026-08-11 fuzz build TARGET",
    "runCommand": "TIME_BIN -v -o CPU_LOG FUZZ_EXE CORPUS -max_total_time=28800 -timeout=5 -max_len=CAP",
    "cpuAccounting": {
      "requiredFields": [
        "User time (seconds)",
        "System time (seconds)",
        "Elapsed (wall clock) time (h:mm:ss or m:ss)"
      ],
      "acceptance": "Sum User time (seconds) and System time (seconds) from every successful resumed shard for the same target, sanitizer, and corpus; the sum must meet the required process-CPU seconds. Elapsed time is operational evidence only.",
      "operationalBound": "max_total_time limits a single libFuzzer process run and never establishes process-CPU coverage."
    },
    "resume": "Keep CORPUS as the first corpus directory for every shard. Run FUZZ_EXE -merge=1 CORPUS NEW_INPUTS before admitting a new input directory, then resume timed runs until the accumulated process CPU threshold is met.",
    "address": "cargo-fuzz 0.13.2 address sanitizer with --careful",
    "undefinedBehavior": "cargo-fuzz --careful plus cargo +nightly-2026-08-11 miri test replay of every minimized finding",
    "concurrency": "Build TARGET with --sanitizer thread, then use the same timed direct-executable procedure for callback and teardown targets where the environment supports it.",
    "campaignToolchain": {
      "name": "nightly-2026-08-11",
      "rustcCommit": "3d6c19bb9ab4798ecfb2ee943df01a811720fc27",
      "requiredCargoFuzzVersion": "0.13.2",
      "requireHostToolRecordForEveryCampaign": true,
      "preflight": [
        "TOOLCHAIN=nightly-2026-08-11",
        "test \"$(rustc +$TOOLCHAIN -vV | awk '/^commit-hash:/ {print $2}')\" = \"3d6c19bb9ab4798ecfb2ee943df01a811720fc27\"",
        "RUSTC_BIN=\"$(rustup which rustc --toolchain $TOOLCHAIN)\"; CARGO_BIN=\"$(rustup which cargo --toolchain $TOOLCHAIN)\"; CARGO_MIRI_BIN=\"$(rustup which cargo-miri --toolchain $TOOLCHAIN)\"; CARGO_FUZZ_BIN=\"$(command -v cargo-fuzz)\"",
        "printf \"%s  %s\\n\" \"7de94a5c099c8d7ee4cafb905e36d882325faa480d8cff6513dd8c0887fac0c5\" \"$RUSTC_BIN\" \"1cf1cd7feded113706026c5f04fad33e45364546e3c0d92ddee0c1a4c8277296\" \"$CARGO_BIN\" \"40a69668c9ff4e5df3e6a87531f2b87dcc5c84e705ee5b06f915fb76383c94af\" \"$CARGO_MIRI_BIN\" \"db150590a2f9fa003fb167bc0eec3f90ba5574fcdd01f78110e6f397dda56582\" \"$CARGO_FUZZ_BIN\" | sha256sum -c -",
        "test \"$(cargo +$TOOLCHAIN fuzz --version)\" = \"cargo-fuzz 0.13.2\""
      ],
      "hostToolRecords": [
        {
          "host": "x86_64-unknown-linux-gnu",
          "observedToolchainSelector": "nightly",
          "observedRustcRelease": "1.99.0-nightly",
          "rustcCommit": "3d6c19bb9ab4798ecfb2ee943df01a811720fc27",
          "rustcCommitDate": "2026-08-11",
          "executables": [
            [
              "rustc",
              "/home/oscar/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc",
              "7de94a5c099c8d7ee4cafb905e36d882325faa480d8cff6513dd8c0887fac0c5"
            ],
            [
              "cargo",
              "/home/oscar/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo",
              "1cf1cd7feded113706026c5f04fad33e45364546e3c0d92ddee0c1a4c8277296"
            ],
            [
              "cargo-miri",
              "/home/oscar/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo-miri",
              "40a69668c9ff4e5df3e6a87531f2b87dcc5c84e705ee5b06f915fb76383c94af"
            ],
            [
              "cargo-fuzz",
              "/nix/store/w6g92cm021l24m5815ry1qf57n00k5j2-cargo-fuzz-0.13.2/bin/cargo-fuzz",
              "db150590a2f9fa003fb167bc0eec3f90ba5574fcdd01f78110e6f397dda56582"
            ],
            [
              "time",
              "/run/current-system/sw/bin/time",
              "e8b9f5440e01a81e0692e68d07dfacb8059c434cae100c1fbb60b7ec52848480"
            ]
          ],
          "cpuAccounting": [
            "GNU Time 1.10",
            "User time (seconds)",
            "System time (seconds)",
            "Elapsed (wall clock) time (h:mm:ss or m:ss)"
          ]
        }
      ]
    }
  },
  "corpusSets": [
    {
      "id": "image",
      "capBytes": 1048576,
      "licenseId": "MIT OR Apache-2.0",
      "licenseUrl": "https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/LICENSE-APACHE",
      "licenseSha256": "0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594",
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
      "licenseSha256": "0dab92d0544f7b233403f14b84a663bdbfa746982eda629e7f4f9ffe1b036feb",
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
      "licenseId": "Unicode-3.0",
      "licenseUrl": "https://web.archive.org/web/20240825031908id_/https://www.unicode.org/license.txt",
      "licenseSha256": "f5062c9a188d81dfe66b56db4182dcf9e4b17c0d9b0d311a8e20b3a1b075c443",
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
      "licenseSha256": "8bd0e0578be788c617ea01d18b2a8146e3746ae50bddadc65a5f9d3aad08ad49",
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
      "licenseSha256": "5fac07febb0e2a97fb0d7b0def149ec08b642e1ba4b9c345283ab1cbd2af6570",
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
    "cargo +nightly-2026-08-11 fuzz run --sanitizer address --careful asset_decode CORPUS -- -max_total_time=28800 -timeout=5 -max_len=1048576",
    "cargo +nightly-2026-08-11 miri test -p oxyflut-assets minimized_asset_decode_findings"
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

- [Flutter 3.41.0 engine `DEPS`](https://raw.githubusercontent.com/flutter/flutter/3452d735bd38224ef2db85ca763d862d6326b17f/DEPS)
- [Flutter 3.44.0 engine `DEPS`](https://raw.githubusercontent.com/flutter/flutter/4c525dac5ebe5971c5708ef73558ed8edcf4a362/DEPS)
- [Flutter 3.47.0 engine `DEPS`](https://raw.githubusercontent.com/flutter/flutter/5f77625673248ee5846fbcaf5d3e1a3878386fd7/DEPS)
- [libpng `08da33b` fix](https://github.com/pnggroup/libpng/commit/08da33b4c88cfcd36e5a706558a8d7e0e4773643)
- [libpng `f139fd5d` `pngrtran.c`](https://flutter.googlesource.com/third_party/libpng/+/f139fd5d80944f5453b079672e50f32ca98ef076/pngrtran.c?format=TEXT)
- [libpng `b6004397` `pngrtran.c`](https://flutter.googlesource.com/third_party/libpng/+/b6004397d2ab98f0250376d9b357337b7f422d13/pngrtran.c?format=TEXT)
- [Impeller interop GN target](https://raw.githubusercontent.com/flutter/flutter/5f77625673248ee5846fbcaf5d3e1a3878386fd7/engine/src/flutter/impeller/toolkit/interop/BUILD.gn)
- [Skia libpng GN target](https://raw.githubusercontent.com/flutter/flutter/5f77625673248ee5846fbcaf5d3e1a3878386fd7/engine/src/flutter/skia/BUILD.gn)
- [full-engine shell GN target](https://raw.githubusercontent.com/flutter/flutter/5f77625673248ee5846fbcaf5d3e1a3878386fd7/engine/src/flutter/shell/common/BUILD.gn)
- [Unicode 16.0.0 UCD ReadMe](https://www.unicode.org/Public/16.0.0/ucd/ReadMe.txt)
- [Unicode License V3 dated snapshot](https://web.archive.org/web/20240825031908id_/https://www.unicode.org/license.txt)
- [Unicode CLDR 45 release note and SPDX mapping](https://cldr.unicode.org/downloads/cldr-45)
- [LLVM libFuzzer documentation](https://llvm.org/docs/LibFuzzer.html)
- The canonical registry names every fetched immutable seed and license URL; P4 preserves the corresponding SHA-256 verification output.
