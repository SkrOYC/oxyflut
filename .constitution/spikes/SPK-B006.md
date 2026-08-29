# Spike report: OXY-B006 shared security patch and fuzz corpora

## Time box

- Status: Completed.
- Budget: 1 focused day.
- Clock start / stop: 2026-08-28T20:23:54Z / 2026-08-28T20:33:24Z.
- Round-6 evidence clock start / stop: 2026-08-28T22:32:41Z / 2026-08-28T22:38:44Z.
- CHANGES: `fuzz-corpora.json` SHA-256 `59f239e1e9dffbca7eb9d15be6cb69139435a74d4d86b0c6d8e0ddcc1b93b80d`, 15,991 bytes; `security-patch-rehearsal.json` SHA-256 `82037d5fd08495aee0ff2a2c2e7e8a4b9ade4c2f76b65f966586a5872667d9bd`, 2,000 bytes.
- Round-9 correction clock start / stop: 2026-08-29T01:02:59Z / 2026-08-29T01:08:30Z.
- Round-9 correction: Remove `fuzz-corpora` and `security-patch-rehearsal`, add `campaign-host-tool-records`, and reduce each affected KU array by one entry.
- Round-12 correction clock start / stop: 2026-08-29T02:32:25Z / 2026-08-29T02:39:48Z.
- Round-12 correction: Pin GNU Time output parsing to `LC_ALL=C` and re-freeze `fuzz-corpora`.
- Round-13 correction clock start / stop: 2026-08-29T03:04:16Z / 2026-08-29T03:10:15Z.
- Round-13 correction: P20 adds ordered, context-preserving preimage and postimage hunk assertions for the three frozen engine lines without changing either canonical staged block.
- Round-14 correction clock start / stop: 2026-08-29T03:36:04Z / 2026-08-29T03:41:44Z.
- Round-14 correction: Order the two expected KU vectors lexicographically, make prescribed Rustup resolution host-neutral, and re-freeze `security-patch-rehearsal` with `LC_ALL=C` before GNU Time parses its English field labels.

## Question

This spike decides which patch rehearsal and attributable seed corpus policy exercise both candidates before implementation.

Table 1. Decision answers

| Question | Status | Answer and evidence | Next bounded probe for a KU |
| :-- | :-- | :-- | :-- |
| Can a disclosed upstream engine patch apply to every frozen Flutter line and both consumption paths? | KU (gating) | P15 traces the lock's framework commits to engine commits: 3.41.0 `44a626f4...` -> `3452d735...`, 3.44.0 `559ffa3f...` -> `4c525dac...`, and 3.47.0 `4cf24164...` -> `5f776256...`. It then resolves `f139fd5d...` for 3.41.0 and `b6004397...` for 3.44.0 and 3.47.0 from the Flutter monorepo's engine `DEPS` files. The official `08da33b` patch adds 39 nonblank and removes 10 nonblank `pngrtran.c` lines. P15's whitespace-normalized line-membership check finds zero added lines absent, which is supplementary evidence only. P20 extracts the ordered, context-preserving preimage and postimage hunks. Each vendored file omits the exact 826-byte preimage hunk and its ASCII-whitespace-normalized form, and contains the exact ordered postimage hunk and its normalized form. This establishes that every lock-traceable upstream engine source has the `08da33b` hunk's postimage and not its preimage; it doesn't establish historical patch application. P1 preserves the 3.47.0 focused SDK and full-engine libpng consumption evidence. The Oxyflut integrated fork still has no source identity, so its actual consumption remains unverified. | Pin the integrated-fork commit and fetch its `DEPS`, `build/secondary/third_party/libpng/BUILD.gn`, `impeller/toolkit/interop/BUILD.gn`, and final GN dependency graph. Expect the fork revision, its libpng pin, and both focused and integrated `pngrtran.c` object paths before a real patch can replace the synthetic rehearsal. |
| Which shared patch rehearsal applies before implementation? | KK | Select `OXY-SYN-SEC-001`, a synthetic shared image-decoder hardening patch. The pinned stack assigns both candidates one bounded Rust decoder above the substrate boundary. The patch replaces unchecked RGBA byte-count multiplication with checked `u64` arithmetic and rejects overflow or more than 67,108,864 decoded bytes before allocation or adapter entry. | Not applicable. |
| What tests establish the synthetic patch result? | KK | The frozen post-patch tests are `checked_rgba_bytes_accepts_4096_by_4096_rgba`, `checked_rgba_bytes_rejects_4097_by_4096_rgba`, `checked_rgba_bytes_rejects_u32_max_square_without_decoder_or_adapter_call`, and `asset_decode_replays_image_registry`. P7 confirms the qualification scaffold does not yet define these functions, so the patch must add them before rehearsal; the frozen rehearsal runs all four. | Not applicable. |
| Can every architecture ingress receive attributable, licensed, capped seed material? | KK | P3 maps all eight architecture ingress categories to five immutable source sets. P4 and P13a SHA-256-verified all 18 retained seed bytes and six retained license notices, including both Apache-2.0 and MIT notices for `image`. The normalized registry requires each set to carry a `licenses` array of objects with `licenseId`, `licenseUrl`, and `licenseSha256`; it applies each set's `capBytes` at ingestion and derives each ingress's `maxLenBytes` from its mapped sets. The Unicode 16.0.0 ReadMe is dated 2024-08-25, and the same-day immutable License V3 snapshot hashes to `f5062c9a...`; Unicode documents the SPDX identifier as `Unicode-3.0`. | Not applicable. |
| Can the required memory, undefined-behavior, and concurrency instrumentation be frozen? | KK | The [LLVM libFuzzer documentation](https://llvm.org/docs/LibFuzzer.html) defines `-max_total_time` as a maximum run time, not CPU accounting. The [cargo-fuzz README](https://raw.githubusercontent.com/rust-fuzz/cargo-fuzz/0.13.2/README.md) directs users to the command help; P6 and P8 verify GNU Time accounting and that `--careful` is a `cargo fuzz build` option. The policy requires cumulative process CPU across resumed corpus shards, a 5-second timeout, dated `nightly-2026-08-12`, and executable-hash preflight. | Not applicable. |
| How is the policy made immutable and attributable? | KK | P13a writes the exact displayed UTF-8, 2-space JSON byte streams with their displayed key order and one trailing LF, then verifies their SHA-256 values. Stage 3 must copy those bytes, retain and hash every license notice and seed byte stream, require a host tool record before each campaign, and reject any source, license, size, tool, or digest mismatch. | Not applicable. |

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
- Why it fits: `OXY-SYN-SEC-001` tests a safety boundary assigned above both adapters. P20 establishes that the candidate `08da33b` hunk's postimage, not its ordered preimage, is in every frozen upstream engine source. It can't serve as a remediation rehearsal, and the unpinned integrated fork prevents confirming a real patch path.
- Option A: Rejected for this rehearsal because P20 finds no candidate `08da33b` preimage in the lock-traceable 3.41.0, 3.44.0, or 3.47.0 engine sources, and the integrated fork has no source identity. This triggers the real-upstream-patch STOP condition only.
- Option C: Rejected because it leaves the required preimplementation corpus and rehearsal policy unfrozen.
- Rejected inputs: Candidate-specific patches, mutable branch references, source files without a license notice, unbounded corpus files, raw private content, and derived fixtures whose source digest is absent.

### Synthetic patch and expected result

`OXY-SYN-SEC-001` introduces `oxyflut_assets::decode::checked_rgba_bytes`. The function computes `width * height * 4` with `u64::checked_mul`, rejects overflow, rejects totals greater than `67_108_864`, converts to `usize` only after both checks, and runs before allocation or an adapter call.

The preimage and postimage must apply in the common Rust asset-decoder module, not either adapter. The patch file must contain only this guard and the four listed tests; P7 confirms that the qualification scaffold does not yet define the post-patch test functions. The rehearsal must fail if the patch changes a candidate-specific file, touches unrelated code, omits any listed test, permits either oversized input, or reaches an adapter for a rejected image.

The real upstream candidate was [libpng commit `08da33b`](https://github.com/pnggroup/libpng/commit/08da33b4c88cfcd36e5a706558a8d7e0e4773643), titled "Fix a buffer overflow in `png_init_read_transformations`." P15 closes the lock chain before testing containment: framework 3.41.0 commit `44a626f4f0027bc38a46dc68aed5964b05a83c18` pins engine `3452d735bd38224ef2db85ca763d862d6326b17f`; 3.44.0 `559ffa3f75e7402d65a8def9c28389a9b2e6fe42` pins `4c525dac5ebe5971c5708ef73558ed8edcf4a362`; and 3.47.0 `4cf24164269a5ebf0c16a028a00727d0e77bbb05` pins `5f77625673248ee5846fbcaf5d3e1a3878386fd7`. It fetched `bin/internal/engine.version` at each framework commit, then used `flutter/flutter` for the engine `DEPS` files because Flutter Engine has merged into that monorepo. Those `DEPS` files resolve `f139fd5d...` for 3.41.0 and `b6004397...` for 3.44.0 and 3.47.0. P15 fetched the official commit patch and each vendored [`pngrtran.c` postimage](https://flutter.googlesource.com/third_party/libpng/+/b6004397d2ab98f0250376d9b357337b7f422d13/pngrtran.c?format=TEXT); all 39 nonblank added lines occur after ASCII-whitespace normalization. That line-membership result alone cannot distinguish the `08da33b` hunk from pre-existing lines. P20 adds an ordered, context-preserving hunk check: every lock-traceable source omits the exact preimage and contains the exact postimage. This establishes the source state required to reject `08da33b` as a rehearsal patch; it doesn't prove a historical application of that commit.

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

P15 corrects the lock trace for the engine pins used by P1. It fetches each framework commit's `bin/internal/engine.version`, then follows the engine revision's `DEPS` in `flutter/flutter`; the older `flutter/engine` path is not used because the engine source is in the Flutter monorepo. For containment, it fetches the official `08da33b` commit metadata and its `pngrtran.c` patch through the GitHub API, then fetches the vendored postimages through `flutter.googlesource.com`. The check extracts the 39 nonblank added lines, replaces each run of ASCII whitespace with one space, trims leading and trailing spaces, and asks whether every resulting line occurs in each normalized vendored postimage. The `4546e144...` digest is only the preserved normalized 39-line excerpt, not a digest of a proxied page. This set-membership check is supplementary: it cannot distinguish the patch hunk from added lines that pre-existed in a vendored file. P20 independently extracts the patch hunk's context and removed lines in original order, then searches the entire vendored postimage for that exact sequence and for an ASCII-whitespace-normalized sequence. It also searches for the ordered postimage hunk. This checks sequence and context rather than individual-line membership.

The output of P15 follows:

```text
$ curl -fsSL https://raw.githubusercontent.com/flutter/flutter/FRAMEWORK/bin/internal/engine.version
44a626f4f0027bc38a46dc68aed5964b05a83c18 3452d735bd38224ef2db85ca763d862d6326b17f sha256=66b2f8154c073765ef3b490aebcd742be3be699f044cf39dbe4b7de58c94fea1
559ffa3f75e7402d65a8def9c28389a9b2e6fe42 4c525dac5ebe5971c5708ef73558ed8edcf4a362 sha256=dfee60c3cf3adc7aa72966ff419b328fef984aa9b663f8f622c9043e46977aaa
4cf24164269a5ebf0c16a028a00727d0e77bbb05 5f77625673248ee5846fbcaf5d3e1a3878386fd7 sha256=1fa9654ffa28f11071dcf4dcfdc8d75753446033985412cc3483a207097117c6
$ GET https://api.github.com/repos/pnggroup/libpng/commits/08da33b4c88cfcd36e5a706558a8d7e0e4773643
sha=08da33b4c88cfcd36e5a706558a8d7e0e4773643
parents=83b23a888b4395c3ae0af3f6d484fce3e4a81155
message=Fix a buffer overflow in `png_init_read_transformations`
file=pngrtran.c status=modified
$ resolve Flutter-monorepo engine DEPS and fetch vendored pngrtran.c postimages
3452d735bd38224ef2db85ca763d862d6326b17f libpng=f139fd5d80944f5453b079672e50f32ca98ef076 deps_sha256=34e98c6f0d1caa46ace21913bb587eafe961d6f7fc9d7086961d232cd4a2179b pngrtran_sha256=185fceeb5b00b8ef55f571cbc7ea5a04a87a146d9a8eabcb9ad9b91b43d80fad
  1784:                     if ((png_ptr->flags & PNG_FLAG_OPTIMIZE_ALPHA) != 0)
  1793:                            (component * png_ptr->trans_alpha[i] + 128) / 255;
  1798:                            (component * png_ptr->trans_alpha[i] + 128) / 255;
  1803:                            (component * png_ptr->trans_alpha[i] + 128) / 255;
4c525dac5ebe5971c5708ef73558ed8edcf4a362 libpng=b6004397d2ab98f0250376d9b357337b7f422d13 deps_sha256=c5a8557661cc4d4a76a612bfb1fd81a11814b553566cbbd4b847b51e685ec93c pngrtran_sha256=1462c8f9097342782b2180b791a83f7cc03b64b3554a8f13bd78cf6bc261c469
  1784:                     if ((png_ptr->flags & PNG_FLAG_OPTIMIZE_ALPHA) != 0)
  1793:                            (component * png_ptr->trans_alpha[i] + 128) / 255;
  1798:                            (component * png_ptr->trans_alpha[i] + 128) / 255;
  1803:                            (component * png_ptr->trans_alpha[i] + 128) / 255;
5f77625673248ee5846fbcaf5d3e1a3878386fd7 libpng=b6004397d2ab98f0250376d9b357337b7f422d13 deps_sha256=6844fe02248df123c7aa4823e2e50230ee62c853043211f38938033792544ea0 pngrtran_sha256=1462c8f9097342782b2180b791a83f7cc03b64b3554a8f13bd78cf6bc261c469
  1784:                     if ((png_ptr->flags & PNG_FLAG_OPTIMIZE_ALPHA) != 0)
  1793:                            (component * png_ptr->trans_alpha[i] + 128) / 255;
  1798:                            (component * png_ptr->trans_alpha[i] + 128) / 255;
  1803:                            (component * png_ptr->trans_alpha[i] + 128) / 255;
$ compare all 39 nonblank added lines from 08da33b with each vendored postimage after ASCII-whitespace normalization
f139fd5d80944f5453b079672e50f32ca98ef076 added_lines=39 missing_from_vendored_postimage=0
b6004397d2ab98f0250376d9b357337b7f422d13 added_lines=39 missing_from_vendored_postimage=0
4546e1448adbbafe56cfa753254932f535bd143ccc132ade54ad76920c528916  08da33b-added-normalized.txt
```

P20 re-fetches the [official `08da33b` patch](https://github.com/pnggroup/libpng/commit/08da33b4c88cfcd36e5a706558a8d7e0e4773643.patch), resolves each engine `DEPS` pin, and base64-decodes each canonical vendored source. It reconstructs the preimage from every context and removal line in the diff hunk, preserving source bytes and order. It reconstructs the postimage from every context and addition line. For each vendored `pngrtran.c`, P20 requires the preimage to be absent and the postimage to be present, both as exact source bytes and after replacing each ASCII-whitespace run with one space and trimming it. `patch_sha256` is a full direct-patch digest. The source-byte and normalized-hunk SHA-256 values are hashes of preserved excerpts, and none hashes a proxied page. This establishes postimage source state, not historical commit application.

The output of P20 follows:

```text
$ P20 extract the ordered, context-preserving 08da33b preimage and postimage hunks
patch_sha256=608c2c5f61c624a0c57185191daadf7280dbcefe14ea19457d5b5d8cdbf1c488
patch_removed_nonblank_lines=10 patch_added_nonblank_lines=39
preimage_hunk_bytes=826 sha256=48464c3c407c0bab1dece77e3e30a8e733aa4b7c865eac6f115cc3d819c540b1
preimage_hunk_ascii_whitespace_normalized_bytes=511 sha256=2a2b195917a54e601962d705d511f4ee58d1adf79a1b7da5e329095b34f025df
postimage_hunk_bytes=2324 sha256=12eb3d8682bda6265bf447ca6b5e9bce19dff06bf59c2476eb02fda747905e0d
postimage_hunk_ascii_whitespace_normalized_bytes=1266 sha256=b297940eb3c6b7c41ae8f06ce22dc92c439e5cb8a723d1f6b9f055cd381836e6
$ P20 resolve each engine DEPS and search its vendored pngrtran.c postimage
flutter=3.41.0 engine=3452d735bd38224ef2db85ca763d862d6326b17f libpng=f139fd5d80944f5453b079672e50f32ca98ef076 deps_sha256=34e98c6f0d1caa46ace21913bb587eafe961d6f7fc9d7086961d232cd4a2179b pngrtran_sha256=185fceeb5b00b8ef55f571cbc7ea5a04a87a146d9a8eabcb9ad9b91b43d80fad
  preimage_hunk_exact=absent preimage_hunk_ascii_whitespace_normalized=absent
  postimage_hunk_exact=present postimage_hunk_ascii_whitespace_normalized=present
flutter=3.44.0 engine=4c525dac5ebe5971c5708ef73558ed8edcf4a362 libpng=b6004397d2ab98f0250376d9b357337b7f422d13 deps_sha256=c5a8557661cc4d4a76a612bfb1fd81a11814b553566cbbd4b847b51e685ec93c pngrtran_sha256=1462c8f9097342782b2180b791a83f7cc03b64b3554a8f13bd78cf6bc261c469
  preimage_hunk_exact=absent preimage_hunk_ascii_whitespace_normalized=absent
  postimage_hunk_exact=present postimage_hunk_ascii_whitespace_normalized=present
flutter=3.47.0 engine=5f77625673248ee5846fbcaf5d3e1a3878386fd7 libpng=b6004397d2ab98f0250376d9b357337b7f422d13 deps_sha256=6844fe02248df123c7aa4823e2e50230ee62c853043211f38938033792544ea0 pngrtran_sha256=1462c8f9097342782b2180b791a83f7cc03b64b3554a8f13bd78cf6bc261c469
  preimage_hunk_exact=absent preimage_hunk_ascii_whitespace_normalized=absent
  postimage_hunk_exact=present postimage_hunk_ascii_whitespace_normalized=present
P20=passed
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

Use one physical core for each parser campaign. Require at least `86_400` process CPU seconds per implemented untrusted parser ingress and `28_800` process CPU seconds for each supported thread-instrumented callback or teardown target. Require a 5-second libFuzzer timeout, `ingressMapping[INGRESS].maxLenBytes` as `-max_len`, and a zero unresolved-report result. Admission checks every source and derived input against its own `corpusSets[SET].capBytes` before it enters a corpus. `maxLenBytes` is the maximum `capBytes` of the corpus sets mapped to that ingress, so it bounds fuzz mutations without weakening per-set admission. In the commands, `INGRESS` is the architecture ingress, `TARGET` is its implemented fuzz target, `CORPUS` is its admitted persistent corpus directory, `MAX_LEN_BYTES` is `ingressMapping[INGRESS].maxLenBytes`, `FUZZ_EXE` is the built target executable, `CPU_LOG` is the GNU Time output, `PACKAGE` owns the replay test, and `TEST_FILTER` selects that replay. The timed command and preflight entries that parse tool output export `LC_ALL=C`, so the required GNU Time field names remain stable on a host with any locale.

[`-max_total_time`](https://llvm.org/docs/LibFuzzer.html) is an elapsed-time maximum for one fuzzer invocation. It is only an operational bound. It cannot establish CON-SEC-001 or CON-SEC-002 process-CPU coverage. The [cargo-fuzz README](https://raw.githubusercontent.com/rust-fuzz/cargo-fuzz/0.13.2/README.md) directs users to command help, and P8 verifies that `--careful` is a `cargo fuzz build` option. Before a campaign, select `nightly-2026-08-12`, select exactly one `hostToolRecords` entry by both `hostname` and Rust host triple, then run this preflight. It resolves Rust tools through `rustup which --toolchain`, resolves non-Rust tools through `command -v`, compares every resolved executable hash with that selected record, and fails before build or execution on no match or any mismatch.

For Rustup-resolved `rustc`, `cargo`, and `cargo-miri`, `executablePath` and `pathRoot` are provenance-only fields. `rustup which` determines their paths, and the preflight checks their hashes and Rust commit. Only `time` has a path assertion: the record supplies `executablePath` to `command -v`, then the preflight checks the resolved path's hash and version.

```sh
POLICY=qualification/staged/fuzz-corpora.json
TOOLCHAIN=nightly-2026-08-12
export LC_ALL=C
export RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
HOST_NAME="$(hostname)"
HOST_TRIPLE="$(rustc +"$TOOLCHAIN" -vV | awk '/^host:/ {print $2}')"
HOST_RECORD="$(jq -cer --arg hostname "$HOST_NAME" --arg triple "$HOST_TRIPLE" '[.instrumentation.campaignToolchain.hostToolRecords[] | select(.hostname == $hostname and .hostTriple == $triple)] | if length == 1 then .[0] else error("expected exactly one host record") end' "$POLICY")"
RUSTC_BIN="$(rustup which --toolchain "$TOOLCHAIN" rustc)"
CARGO_BIN="$(rustup which --toolchain "$TOOLCHAIN" cargo)"
CARGO_MIRI_BIN="$(rustup which --toolchain "$TOOLCHAIN" cargo-miri)"
CARGO_FUZZ_BIN="$(command -v cargo-fuzz)"
TIME_BIN="$(command -v "$(printf '%s' "$HOST_RECORD" | jq -er '.tools[] | select(.name == "time") | .executablePath')")"
printf '%s  %s\n' \
  "$(printf '%s' "$HOST_RECORD" | jq -er '.tools[] | select(.name == "rustc") | .sha256')" "$RUSTC_BIN" \
  "$(printf '%s' "$HOST_RECORD" | jq -er '.tools[] | select(.name == "cargo") | .sha256')" "$CARGO_BIN" \
  "$(printf '%s' "$HOST_RECORD" | jq -er '.tools[] | select(.name == "cargo-miri") | .sha256')" "$CARGO_MIRI_BIN" \
  "$(printf '%s' "$HOST_RECORD" | jq -er '.tools[] | select(.name == "cargo-fuzz") | .sha256')" "$CARGO_FUZZ_BIN" \
  "$(printf '%s' "$HOST_RECORD" | jq -er '.tools[] | select(.name == "time") | .sha256')" "$TIME_BIN" | sha256sum -c -
test "$(rustc +"$TOOLCHAIN" -vV | awk '/^commit-hash:/ {print $2}')" = "$(printf '%s' "$HOST_RECORD" | jq -er '.rustcCommit')"
test "$(cargo +"$TOOLCHAIN" fuzz --version)" = "$(printf '%s' "$HOST_RECORD" | jq -er '.tools[] | select(.name == "cargo-fuzz") | .version')"
test "$("$TIME_BIN" --version | head -n 1)" = "$(printf '%s' "$HOST_RECORD" | jq -er '.tools[] | select(.name == "time") | .version')"
```

Build every AddressSanitizer target with `cargo +nightly-2026-08-12 fuzz build --sanitizer address --careful TARGET`, and build every ThreadSanitizer target with `cargo +nightly-2026-08-12 fuzz build --sanitizer thread --careful TARGET`. For every successful shard, resolve `MAX_LEN_BYTES` from `ingressMapping[INGRESS].maxLenBytes`, then run `LC_ALL=C "$TIME_BIN" -v -o CPU_LOG FUZZ_EXE CORPUS -max_total_time=28800 -timeout=5 -max_len=MAX_LEN_BYTES`. Preserve `User time (seconds)`, `System time (seconds)`, and elapsed wall time from `CPU_LOG`, and record user plus system seconds in the shard ledger. Add only successful shards with the same target, sanitizer, and persistent `CORPUS`; resume until the applicable threshold is reached. `-max_total_time=28800` bounds one operational invocation only. Keep `CORPUS` as the first libFuzzer corpus directory. Before admitting another input directory, use `FUZZ_EXE -merge=1 CORPUS NEW_INPUTS`, then resume the timed target. The LLVM documentation states that the first corpus directory receives new inputs and describes `-merge=1` and resumable merge control files. Replay every minimized crash and retained seed with `cargo +nightly-2026-08-12 miri test -p PACKAGE TEST_FILTER`.

P6 proves this host's `command time -v -o CPU_LOG` records the required three fields. P7 confirms the four post-patch test functions do not yet exist in the qualification scaffold, so OXY-SYN-SEC-001 must add them before rehearsal. P8 proves `nightly-2026-08-11` resolves commit `12c36e2539c54397c51d6ea4401defd8768a4f5b`, while `nightly-2026-08-12` resolves the required `3d6c19bb9ab4798ecfb2ee943df01a811720fc27`. P8 also re-hashes the executables from the dated selector. The captured host record uses only `nightly-2026-08-12`; the previous rolling-`nightly` record is inadmissible. This host records SHA-256 values for `rustc` `7de94a5c099c8d7ee4cafb905e36d882325faa480d8cff6513dd8c0887fac0c5`, `cargo` `1cf1cd7feded113706026c5f04fad33e45364546e3c0d92ddee0c1a4c8277296`, `cargo-miri` `40a69668c9ff4e5df3e6a87531f2b87dcc5c84e705ee5b06f915fb76383c94af`, `cargo-fuzz` 0.13.2 `db150590a2f9fa003fb167bc0eec3f90ba5574fcdd01f78110e6f397dda56582`, and GNU Time 1.10 `e8b9f5440e01a81e0692e68d07dfacb8059c434cae100c1fbb60b7ec52848480`. Stage 3 must stage an equivalent complete record in `qualification/staged/fuzz-corpora.json` for every campaign host and retain `campaign-host-tool-records` as a gate until it does. `resolved-tool-digests` remains independently bound to `resolvedTools` and `qualification/tools/native-contract-toolchain.json`, which OXY-A008 owns. P16 establishes that campaign-host tools must remain only in `instrumentation.campaignToolchain.hostToolRecords`; they must never enter `qualification-lock.json` `resolvedTools`. The lock binds the campaign toolchain only through `measurementPolicy.fuzzCorpora`, which holds the staged file digest.

The relevant instrumentation output follows:

```text
$ rustc +nightly-2026-08-11 -vV
rustc 1.99.0-nightly (12c36e253 2026-08-10)
commit-hash: 12c36e2539c54397c51d6ea4401defd8768a4f5b
commit-date: 2026-08-10
host: x86_64-unknown-linux-gnu
$ rustc +nightly-2026-08-12 -vV
rustc 1.99.0-nightly (3d6c19bb9 2026-08-11)
commit-hash: 3d6c19bb9ab4798ecfb2ee943df01a811720fc27
commit-date: 2026-08-11
host: x86_64-unknown-linux-gnu
$ rustup toolchain install nightly-2026-08-12 --component miri --profile minimal
info: syncing channel updates for 'nightly-2026-08-12-x86_64-unknown-linux-gnu'
info: downloading component 'miri'
info: installing component 'miri'
nightly-2026-08-12-x86_64-unknown-linux-gnu updated - rustc 1.99.0-nightly (3d6c19bb9 2026-08-11)
$ rustup which --toolchain nightly-2026-08-12 rustc
/home/oscar/.rustup/toolchains/nightly-2026-08-12-x86_64-unknown-linux-gnu/bin/rustc
$ rustup which --toolchain nightly-2026-08-12 cargo
/home/oscar/.rustup/toolchains/nightly-2026-08-12-x86_64-unknown-linux-gnu/bin/cargo
$ rustup which --toolchain nightly-2026-08-12 cargo-miri
/home/oscar/.rustup/toolchains/nightly-2026-08-12-x86_64-unknown-linux-gnu/bin/cargo-miri
$ command -v cargo-fuzz
/nix/store/w6g92cm021l24m5815ry1qf57n00k5j2-cargo-fuzz-0.13.2/bin/cargo-fuzz
$ sha256sum rustc cargo cargo-miri cargo-fuzz time
7de94a5c099c8d7ee4cafb905e36d882325faa480d8cff6513dd8c0887fac0c5  /home/oscar/.rustup/toolchains/nightly-2026-08-12-x86_64-unknown-linux-gnu/bin/rustc
1cf1cd7feded113706026c5f04fad33e45364546e3c0d92ddee0c1a4c8277296  /home/oscar/.rustup/toolchains/nightly-2026-08-12-x86_64-unknown-linux-gnu/bin/cargo
40a69668c9ff4e5df3e6a87531f2b87dcc5c84e705ee5b06f915fb76383c94af  /home/oscar/.rustup/toolchains/nightly-2026-08-12-x86_64-unknown-linux-gnu/bin/cargo-miri
db150590a2f9fa003fb167bc0eec3f90ba5574fcdd01f78110e6f397dda56582  /nix/store/w6g92cm021l24m5815ry1qf57n00k5j2-cargo-fuzz-0.13.2/bin/cargo-fuzz
e8b9f5440e01a81e0692e68d07dfacb8059c434cae100c1fbb60b7ec52848480  /run/current-system/sw/bin/time
$ cargo +nightly-2026-08-12 fuzz --version
cargo-fuzz 0.13.2
$ cargo +nightly-2026-08-12 fuzz build --help | grep -- --careful
  -c, --careful
$ command time -v -o CPU_LOG true
User time (seconds): 0.00
System time (seconds): 0.00
Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.00
$ grep -rn "fn asset_decode_replays_image_registry\\|fn checked_rgba_bytes" crates/oxyflut-assets/src
NO_MATCH: the qualification scaffold does not yet contain the post-patch tests.
```

P11 rechecked the first host record. The [Rust COPYRIGHT notice](https://raw.githubusercontent.com/rust-lang/rust/3d6c19bb9ab4798ecfb2ee943df01a811720fc27/COPYRIGHT) licenses Rust under Apache-2.0 or MIT at the recipient's option; it supports `MIT OR Apache-2.0` for `rustc`, `cargo`, and `cargo-miri`. The [cargo-fuzz 0.13.2 package metadata](https://raw.githubusercontent.com/rust-fuzz/cargo-fuzz/0.13.2/Cargo.toml) declares `MIT OR Apache-2.0`, and its fetched Apache and MIT notices match that declaration. The fetched [GNU Time 1.10 source distribution](https://ftp.gnu.org/gnu/time/time-1.10.tar.gz) contains the GPLv3 `COPYING` notice and a source header permitting version 3 or any later version, establishing `GPL-3.0-or-later` for GNU `time`.

The output of P11 follows:

```text
$ hostname; . /etc/os-release; printf 'os=%s %s\n' "$ID" "$VERSION_ID"; rustc -vV | awk '/^host:/ {print $2}'
thinkpadp14s
os=nixos 26.05
x86_64-unknown-linux-gnu
$ rustc +nightly-2026-08-12 -vV | grep -E '^(rustc|commit-hash:|host:)'
rustc 1.99.0-nightly (3d6c19bb9 2026-08-11)
commit-hash: 3d6c19bb9ab4798ecfb2ee943df01a811720fc27
host: x86_64-unknown-linux-gnu
$ cargo +nightly-2026-08-12 --version; cargo +nightly-2026-08-12 miri --version; cargo +nightly-2026-08-12 fuzz --version
cargo 1.99.0-nightly (b07e5a086 2026-08-07)
miri 0.1.0 (3d6c19bb9a 2026-08-11)
cargo-fuzz 0.13.2
$ command -v time; command -V time; command -v /run/current-system/sw/bin/time; /run/current-system/sw/bin/time --version | head -n 1
time
time is a shell keyword
/run/current-system/sw/bin/time
time (GNU Time) 1.10
$ sha256sum "$(rustup which --toolchain nightly-2026-08-12 rustc)" "$(rustup which --toolchain nightly-2026-08-12 cargo)" "$(rustup which --toolchain nightly-2026-08-12 cargo-miri)" "$(command -v cargo-fuzz)" /run/current-system/sw/bin/time
7de94a5c099c8d7ee4cafb905e36d882325faa480d8cff6513dd8c0887fac0c5  /home/oscar/.rustup/toolchains/nightly-2026-08-12-x86_64-unknown-linux-gnu/bin/rustc
1cf1cd7feded113706026c5f04fad33e45364546e3c0d92ddee0c1a4c8277296  /home/oscar/.rustup/toolchains/nightly-2026-08-12-x86_64-unknown-linux-gnu/bin/cargo
40a69668c9ff4e5df3e6a87531f2b87dcc5c84e705ee5b06f915fb76383c94af  /home/oscar/.rustup/toolchains/nightly-2026-08-12-x86_64-unknown-linux-gnu/bin/cargo-miri
db150590a2f9fa003fb167bc0eec3f90ba5574fcdd01f78110e6f397dda56582  /nix/store/w6g92cm021l24m5815ry1qf57n00k5j2-cargo-fuzz-0.13.2/bin/cargo-fuzz
e8b9f5440e01a81e0692e68d07dfacb8059c434cae100c1fbb60b7ec52848480  /run/current-system/sw/bin/time
$ fetch Rust, cargo-fuzz, and GNU Time license metadata to /tmp/wf-epic-b/OXY-B006-pr-round2/licenses
rust COPYRIGHT sha256=172020dbfd5b53a226dfde77616190a48dcff519b0bc0e6deb91a8450782c4af: Apache License, Version 2.0 or MIT license, at your option
cargo-fuzz Cargo.toml sha256=26132b1acda063cc70364cee6fbefc4dbc7bad80f99e43d550dfd0a0534e6174: license = "MIT OR Apache-2.0"
rust LICENSE-APACHE sha256=62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a; LICENSE-MIT sha256=b71bd43a069ca0641a9ecfe585ca7b3c53b5cc1608f8b68321168698e28b5ea1
cargo-fuzz LICENSE-APACHE sha256=a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2; LICENSE-MIT sha256=0621878e61f0d0fda054bcbe02df75192c28bde1ecc8289cbd86aeba2dd72720
GNU time 1.10 tarball sha256=e8c29fb4ab599d8478e41e8618f50db8aede9c90af27d0d2ef28ae50d5de09c3; COPYING sha256=8ceb4b9ee5adedde47b31e975c1d90c73ad27b6b165a1dcd80c7c545eb65b903
src/time.c: GNU Time is free software under GPL version 3 or (at your option) any later version.
```

P12 ran the host-neutral preflight against a temporary copy of the canonical policy. The record selector returned exactly the non-reference NixOS record, and every resolved executable matched its recorded SHA-256. This confirms the procedure's behavior on this host only; it doesn't create an Ubuntu reference-host record.

The output of P12 follows:

```text
$ POLICY=/tmp/wf-epic-b/OXY-B006-pr-round2/p12/fuzz-corpora.json /tmp/wf-epic-b/OXY-B006-pr-round2/run-preflight.sh
selected hostname=thinkpadp14s hostTriple=x86_64-unknown-linux-gnu reference=false
rustc: OK
cargo: OK
cargo-miri: OK
cargo-fuzz: OK
time: OK
preflight=passed
```

This reconciles the record with `reference-hardware-access.md`: non-reference campaign-host tool records with `reference: false` are a deliberate exemption from its prohibition on adding `thinkpadp14s` hardware, GPU, driver, or access information to the lock. They identify a toolchain, not a qualification environment, and carry no GPU, driver, or access data. Because `requireHostToolRecordForEveryCampaign` is `true`, every new campaign host changes the lock-bound policy digest and requires a new lock re-bind.

P16 re-read the repository validator to correct the rejected campaign-tool proposal. `verify_resolved_tools` delegates every nonempty lock array to the staged-manifest comparison at `xtask/src/contracts/readiness.rs:409-419`. `LockResolvedTool::from_value` allowlists exactly seven properties at `xtask/src/toolchain/lock.rs:238-262`. `verify_lock_resolved_tools` rejects duplicate names, resolves each name against the staged manifest, and requires every `TOOL_SPECS` name at `xtask/src/toolchain/lock.rs:50-75`; `verify_lock_tool` requires the staged absolute executable path at `xtask/src/toolchain/lock.rs:296-320`. Consequently, campaign-host records belong only in the staged fuzz policy. The immutable lock reference is `measurementPolicy.fuzzCorpora`, its digest of that staged file. No canonical staged block changes: it already records each host tool's `licenseId` and other campaign metadata.

The output of P16 follows:

```text
$ nl -ba xtask/src/contracts/readiness.rs | sed -n "409,421p"
   409 fn verify_resolved_tools(root: &Path, tools: &[Value]) -> Result<(), ReadinessError> {
   410     let manifest_path = root.join(TOOLCHAIN_MANIFEST_PATH);
   411     let manifest = crate::toolchain::ToolchainManifest::from_json(
   412         &fs::read(&manifest_path).map_err(|source| ReadinessError::Io {
   413             path: manifest_path,
   414             source,
   415         })?,
   416     )
   417     .map_err(|_| invariant("resolved-tool-manifest"))?;
   418     crate::toolchain::lock::verify_lock_resolved_tools_classified(&manifest, tools)
   419         .map_err(|failure| invariant(failure.code()))
   420 }
$ nl -ba xtask/src/toolchain/lock.rs | sed -n "50,76p;238,262p;296,320p"
    50 pub(crate) fn verify_lock_resolved_tools(
    51     manifest: &ToolchainManifest,
    52     lock_tools: &[Value],
    53 ) -> Result<(), ToolchainError> {
    54     verify(manifest)?;
    56     let mut names = BTreeSet::new();
    57     for value in lock_tools {
    58         let lock_tool = LockResolvedTool::from_value(value)?;
    59         if !names.insert(lock_tool.name.clone()) {
    60             return Err(ToolchainError::DuplicateTool {
    61                 name: lock_tool.name,
    62             });
    63         }
    64         let staged_tool = manifest.tool(&lock_tool.name)?;
    65         verify_lock_tool(&lock_tool, staged_tool)?;
    66     }
    68     for specification in TOOL_SPECS {
    69         if !names.contains(specification.name) {
    70             return Err(ToolchainError::MissingTool {
    71                 name: specification.name.to_owned(),
    72             });
    73         }
    74     }
    76     Ok(())
   238     fn from_value(value: &Value) -> Result<Self, ToolchainError> {
   239         let object = value.as_object().ok_or(ToolchainError::InvalidManifest {
   240             reason: "a qualification-lock resolvedTools entry must be an object".to_owned(),
   241         })?;
   242         reject_unknown_fields(
   243             object,
   244             &[
   245                 "name",
   246                 "version",
   247                 "sourceIdentity",
   248                 "hostTriple",
   249                 "licenseId",
   250                 "executablePath",
   251                 "sha256",
   252             ],
   253         )?;
   254         Ok(Self {
   255             name: required_string(object, "name")?,
   256             version: required_string(object, "version")?,
   257             source_identity: required_string(object, "sourceIdentity")?,
   258             host_triple: required_string(object, "hostTriple")?,
   259             license_id: required_string(object, "licenseId")?,
   260             executable_path: required_string(object, "executablePath")?,
   261             sha256: required_string(object, "sha256")?,
   262         })
   296 fn verify_lock_tool(
   297     lock_tool: &LockResolvedTool,
   298     staged_tool: &ResolvedTool,
   299 ) -> Result<(), ToolchainError> {
   300     if lock_tool.version != staged_tool.version
   301         || lock_tool.source_identity != staged_tool.source_identity
   302         || lock_tool.host_triple != staged_tool.host_triple
   303         || lock_tool.license_id != staged_tool.license_id
   304     {
   305         return Err(ToolchainError::LockEntryMismatch {
   306             name: lock_tool.name.clone(),
   307         });
   308     }
   309     if lock_tool.sha256.parse::<Sha256Digest>().is_err() || lock_tool.sha256 != staged_tool.sha256 {
   310         return Err(ToolchainError::DigestMismatch {
   311             name: lock_tool.name.clone(),
   312         });
   313     }
   315     let expected_path = resolve_manifest_executable_path(staged_tool)?;
   316     let actual_path = PathBuf::from(&lock_tool.executable_path);
   317     if !actual_path.is_absolute() || actual_path != expected_path {
   318         return Err(ToolchainError::ExecutableSubstitution {
   319             name: lock_tool.name.clone(),
   320         });
$ nl -ba xtask/src/toolchain/specs.rs | grep -E "TOOL_SPECS|name:"
    23 pub(super) const TOOL_SPECS: &[ToolSpec] = &[
    25         name: "c-compiler",
    34         name: "cxx-compiler",
    43         name: "c-header-checker",
    52         name: "linker",
    61         name: "archiver",
    70         name: "symbol-inspector",
    79         name: "bindgen",
    88         name: "cbindgen",
    97         name: "prettier",
   106         name: "rustfmt",
   115         name: "rustc",
```

P18 verified the readiness binding and its tests and fixtures. `resolved-tool-digests` maps only to `resolvedTools` and `qualification/tools/native-contract-toolchain.json`, which OXY-A008 owns. `collect_known_unknowns` maps each KU string to its own binding and requires the binding's field. No `hostToolRecords` or `campaignToolchain` implementation exists in the readiness crate or `xtask`. Therefore, clearing the OXY-A008 gate cannot establish campaign-host records. `campaign-host-tool-records` must remain a separate KU until every campaign host that produced evidence has a complete selected `hostToolRecords` entry.

The output of P18 follows:

```text
$ nl -ba crates/oxyflut-qualification/src/readiness.rs | sed -n "185,215p;509,536p"
   185    KnownUnknownBinding {
   186        known_unknown: "fuzz-corpora",
   187        required_field: "measurementPolicy.fuzzCorpora",
   188        evidence_path: Some("qualification/staged/fuzz-corpora.json"),
   189        upstream_owner: "OXY-D001",
   190    },
   191    KnownUnknownBinding {
   192        known_unknown: "security-patch-rehearsal",
   193        required_field: "measurementPolicy.securityPatchRehearsal",
   194        evidence_path: Some("qualification/staged/security-patch-rehearsal.json"),
   195        upstream_owner: "OXY-D001",
   196    },
   197    KnownUnknownBinding {
   198        known_unknown: "layout-visit-cap",
   199        required_field: "measurementPolicy.layoutVisitCap",
   200        evidence_path: None,
   201        upstream_owner: "OXY-D001",
   202    },
   203    KnownUnknownBinding {
   204        known_unknown: EXTERNAL_CONTRACT_LOCK_KNOWN_UNKNOWN,
   205        required_field: "measurementPolicy.externalContractLock",
   206        evidence_path: None,
   207        upstream_owner: "OXY-C001",
   208    },
   209    KnownUnknownBinding {
   210        known_unknown: "resolved-tool-digests",
   211        required_field: "resolvedTools",
   212        evidence_path: Some("qualification/tools/native-contract-toolchain.json"),
   213        upstream_owner: "OXY-A008",
   214    },
   215 ];
   509    for known_unknown in known_unknowns {
   510        let known_unknown = known_unknown.as_str().ok_or(ReadinessError::InvalidLock {
   511            code: "pre-implementation-known-unknown",
   512        })?;
   513        let binding = KNOWN_UNKNOWN_BINDINGS
   514            .iter()
   515            .find(|binding| binding.known_unknown == known_unknown)
   516            .ok_or(ReadinessError::UnmappedKnownUnknown)?;
   517        if !required_field_is_present(lock, binding.required_field) {
   518            return Err(ReadinessError::InvalidLock {
   519                code: "ku-required-field",
   520            });
   521        }
   522        let field_path = format!("preImplementationKnownUnknowns.{known_unknown}");
   523        let referent = (known_unknown == "external-distribution-schema-snapshots-and-verifiers")
   524            .then(|| external_contract_lock_referent(active_external_lock))
   525            .transpose()?;
   526        let evidence_path = referent
   527            .map(ExternalContractLockReferent::evidence_path)
   528            .or(binding.evidence_path);
   529        push_block_with_referent(
   530            blocking,
   531            &field_path,
   532            BlockingKind::Ku,
   533            evidence_path,
   534            referent,
   535            Some(binding.upstream_owner),
$ rg -n "hostToolRecords|campaignToolchain" crates/oxyflut-qualification xtask || true
$ for file in qualification/fixtures/readiness/invalid.json qualification/fixtures/readiness/cleared-without-evidence.json; do jq -r --arg file "$file" "$file + \" pre=\" + ([.preImplementationKnownUnknowns[] | select(. == \"fuzz-corpora\" or . == \"security-patch-rehearsal\" or . == \"campaign-host-tool-records\")] | join(\",\")) + \" gating=\" + ([.gatingKnownUnknowns[] | select(. == \"fuzz-corpora\" or . == \"security-patch-rehearsal\" or . == \"campaign-host-tool-records\")] | join(\",\"))" "$file"; done
qualification/fixtures/readiness/invalid.json pre=fuzz-corpora,security-patch-rehearsal gating=fuzz-corpora,security-patch-rehearsal
qualification/fixtures/readiness/cleared-without-evidence.json pre=fuzz-corpora,security-patch-rehearsal gating=fuzz-corpora,security-patch-rehearsal
$ nl -ba xtask/src/commands/lock_tests.rs | sed -n "51,68p;268,284p;311,317p"
    51    assert_eq!(
    52        known_unknowns,
    53        vec![
    54            "capability-and-platform-baselines",
    55            "complete-ime-editing-geometry-and-accessibility-maps",
    56            "external-distribution-schema-snapshots-and-verifiers",
    57            "fuzz-corpora",
    58            "hardware-gpu-driver-and-system-package-locks",
    59            "independent-presentation-opportunity-sources",
    60            "layout-visit-cap",
    61            "minimum-platform-and-protocol-versions",
    62            "raw-measurement-and-sample-validity-contracts",
    63            "reference-application-scenes-scripts-fonts-assets-windows-cache-and-flags",
    64            "resolved-tool-digests",
    65            "scoring-anchors-and-two-assessors",
    66            "security-patch-rehearsal",
    67        ]
    68    );
   268    assert_eq!(
   269        known_unknowns,
   270        vec![
   271            "capability-and-platform-baselines",
   272            "complete-ime-editing-geometry-and-accessibility-maps",
   273            "external-distribution-schema-snapshots-and-verifiers",
   274            "fuzz-corpora",
   275            "hardware-gpu-driver-and-system-package-locks",
   276            "independent-presentation-opportunity-sources",
   277            "layout-visit-cap",
   278            "minimum-platform-and-protocol-versions",
   279            "raw-measurement-and-sample-validity-contracts",
   280            "reference-application-scenes-scripts-fonts-assets-windows-cache-and-flags",
   281            "scoring-anchors-and-two-assessors",
   282            "security-patch-rehearsal",
   283        ]
   284    );
   311    for line in [
   312        "blocking: field-path=preImplementationKnownUnknowns.capability-and-platform-baselines kind=ku evidence-path=.constitution/tech-spec/contracts/platform-contracts.json upstream-owner=OXY-C002,OXY-C004",
   313        "blocking: field-path=preImplementationKnownUnknowns.scoring-anchors-and-two-assessors kind=ku evidence-path=qualification/staged/scoring-anchors.json upstream-owner=OXY-D001",
   314        "blocking: field-path=preImplementationKnownUnknowns.fuzz-corpora kind=ku evidence-path=qualification/staged/fuzz-corpora.json upstream-owner=OXY-D001",
   315        "blocking: field-path=preImplementationKnownUnknowns.security-patch-rehearsal kind=ku evidence-path=qualification/staged/security-patch-rehearsal.json upstream-owner=OXY-D001",
   316    ] {
   317        assert!(lines.iter().any(|actual| actual == line), "{line}");
```

### Frozen corpus sources

Table 2. Admitted source sets

| Set | Immutable origin and evidence | License and attribution | Cap | Observed maximum seed size |
| :-- | :-- | :-- | --: | --: |
| `image` | `image` v0.25.10 commit `76e57184f22772dad1138e96954e57945406b15e`; PNG, progressive JPEG, interlaced GIF, and animated alpha WebP digests appear in the canonical registry. | MIT OR Apache-2.0 under the fetched [Apache license](https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/LICENSE-APACHE), SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`, and [MIT license](https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/LICENSE-MIT), SHA-256 `c77a4cf9da729987d0fe7ccd811e3bd27393914ddf3d23467c18cc22954513b3`. | 1,048,576 bytes | 52,286 bytes |
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
$ curl -fsSL https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/LICENSE-MIT | sha256sum
c77a4cf9da729987d0fe7ccd811e3bd27393914ddf3d23467c18cc22954513b3  -
$ fetch versioned UCD ReadMe and dated license snapshot
UCD 16.0.0 ReadMe 2024-08-25 sha256=14cafa23788d3a20dd21d6b0cdcb8d6dab520781fcd9ad9392f3b88ea607e633
Unicode License V3 snapshot 2024-08-25 1995 bytes sha256=f5062c9a188d81dfe66b56db4182dcf9e4b17c0d9b0d311a8e20b3a1b075c443
```

The `unicode-text` sources are live, versioned UCD publication paths rather than commit-pinned archives. This is a deliberate fail-closed trade: the frozen SHA-256 values make an in-place source revision a rebuild failure, not a silent input difference. Unicode uses versioned UCD paths as its stable published form; the [UCD 16.0.0 ReadMe](https://www.unicode.org/Public/16.0.0/ucd/ReadMe.txt) identifies the published database version and date. Stage 3 can add an archive-snapshot mirror URL as a secondary source only when it creates the file through a canonical re-freeze with a new digest.

P17 re-fetched the UCD ReadMe and both live source URLs through the Jina reader proxy. The byte counts are transport results, not fixture digests.

```text
PASS url=https://www.unicode.org/Public/16.0.0/ucd/ReadMe.txt jina_bytes=833
PASS url=https://www.unicode.org/Public/16.0.0/ucd/auxiliary/GraphemeBreakTest.txt jina_bytes=172088
PASS url=https://www.unicode.org/Public/16.0.0/ucd/BidiTest.txt jina_bytes=7960131
$ grep -E -m 4 "Unicode Character Database|Date|Version" ReadMe.txt
# Unicode Character Database
# Date: 2024-08-25
# UAX #44, "Unicode Character Database"
for the Unicode Character Database, for Version 16.0.0 of the Unicode Standard.
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

| Architecture ingress | Seed sets | Expected parser or boundary test | Per-set admission caps; ingress `maxLenBytes` |
| :-- | :-- | :-- | --: |
| Application assets, fonts, and images | `image`, `font`, `json` | `asset_decode`, `font_registration`, and `asset_manifest_json` reject malformed or oversized input before allocation. | `image`: 1,048,576; `font`: 1,048,576; `json`: 65,536; `maxLenBytes`: 1,048,576 |
| Pointer, touch, keyboard, window, display, and lifecycle events | `wpt-events` | `platform_event_normalization` rejects invalid shape, stale identity, invalid order, and cap breach. | `wpt-events`: 65,536; `maxLenBytes`: 65,536 |
| Input method editor, clipboard, and platform-message content | `unicode-text`, `wpt-events` | `ime_transaction`, `clipboard_transaction`, and `platform_message` reject invalid range, index unit, sensitive-field, and payload size without recording content. | `unicode-text`: 8,388,608; `wpt-events`: 65,536; `maxLenBytes`: 8,388,608 |
| Accessibility properties and actions | `unicode-text`, `wpt-events` | `semantics_update` and `semantics_action` reject invalid node, action, range, and private payload without retargeting. | `unicode-text`: 8,388,608; `wpt-events`: 65,536; `maxLenBytes`: 8,388,608 |
| Candidate callbacks, resources, and errors | `json`, `wpt-events` | `abi_callback_decoder` and `abi_resource_validator` reject unknown status, invalid length, invalid enum, stale generation, and buffer overflow before candidate work. | `json`: 65,536; `wpt-events`: 65,536; `maxLenBytes`: 65,536 |
| Local-sink acknowledgement or failure | `json` | `diagnostic_record` and `sink_ack` reject malformed records and failed acknowledgements without blocking producers. | `json`: 65,536; `maxLenBytes`: 65,536 |
| Candidate artifacts and evidence | `json` | `artifact_manifest`, provenance, and software-bill-of-materials parsers reject malformed JSON before qualification. | `json`: 65,536; `maxLenBytes`: 65,536 |
| Independent verification result | `json` | `verification_result` rejects an incomplete, malformed, or artifact-mismatched result and keeps qualification open. | `json`: 65,536; `maxLenBytes`: 65,536 |

The corpus importer may derive target-specific encodings only from a listed source after preserving the source SHA-256, the importer revision, the derived-file SHA-256, the license notice, the source set, and the source set `capBytes`. It must reject each source or derived input larger than that set's `capBytes` and any derived input that contains raw private content. A target selects `-max_len` only from its ingress `maxLenBytes`, the maximum across its mapped sets. A source branch, a release tag without a resolved commit, or a digest-less file is not admissible.

## Downstream impact

- ADRs to write or update: None.
- Tickets unblocked in `tasks/active/`: `OXY-D001` can materialize the two staged policy records. Candidate fuzz targets remain contingent on candidate implementation.
- Tickets to add or split: Create one implementation ticket for each actual parser ingress that the implementation inventory adds beyond table 3. Each ticket must inherit the same registry admission and campaign rules.

### Spec edits required

- `qualification/staged/fuzz-corpora.json`: create this file with the exact canonical bytes in the next section. In `admission`, set `requireLicenses` to `true` and `requiredLicenseEntryKeys` to `["licenseId", "licenseUrl", "licenseSha256"]`; don't use flat license fields. In every `corpusSets[*].licenses`, use objects with exactly those three keys. In every `ingressMapping[INGRESS]`, use `corpusSets` and `maxLenBytes`, where `maxLenBytes` is the maximum mapped `capBytes`; enforce `requireSizeAtMostCorpusSetCap` against each source set's `capBytes` at ingestion and pass only the ingress `maxLenBytes` to `-max_len`. Set `instrumentation.runCommand` to the canonical `LC_ALL=C; export LC_ALL;`-prefixed command and prefix `instrumentation.campaignToolchain.preflight[0]` and `[4]` with `export LC_ALL=C;`. `.constitution/tech-spec/contracts/qualification-lock.json`, `measurementPolicy.fuzzCorpora`: set the value to `59f239e1e9dffbca7eb9d15be6cb69139435a74d4d86b0c6d8e0ddcc1b93b80d`.
- `qualification/staged/security-patch-rehearsal.json`: create this file with the exact canonical bytes in the next section. In `rehearsal[4]`, resolve `ingressMapping["application-assets"].maxLenBytes` from `qualification/staged/fuzz-corpora.json`, pass it to `-max_len`, and invoke GNU Time as `LC_ALL=C "$TIME_BIN"`; this resolves `maxLenBytes` to `1048576` and keeps the parsed GNU Time field labels English. `.constitution/tech-spec/contracts/qualification-lock.json`, `measurementPolicy.securityPatchRehearsal`: set the value to `82037d5fd08495aee0ff2a2c2e7e8a4b9ade4c2f76b65f966586a5872667d9bd`.
- `.constitution/tech-spec/data-models/qualification-lock.schema.json`, `$defs.tool.properties`: make no `pathRoot` edit. `xtask/src/toolchain/lock.rs:238-262` accepts only `name`, `version`, `sourceIdentity`, `hostTriple`, `licenseId`, `executablePath`, and `sha256` for a `resolvedTools` entry. The `pathRoot` fields already present in the canonical campaign record remain staged-policy data only; they are not `$defs.tool` properties.
- `.constitution/tech-spec/contracts/qualification-lock.json`, `resolvedTools`: make no campaign-host append or transformation. `xtask/src/contracts/readiness.rs:409-419` delegates nonempty entries to the staged-manifest validator. `xtask/src/toolchain/lock.rs:50-75` rejects duplicate names, resolves every entry against the staged manifest, and requires every `TOOL_SPECS` name; `xtask/src/toolchain/lock.rs:296-320` requires an exact absolute staged path. Thus `pathRoot` yields `InvalidManifest`, Rustup-relative paths yield `ExecutableSubstitution`, campaign `cargo`, `cargo-miri`, `cargo-fuzz`, and `time` yield `MissingTool`, a campaign `rustc` either duplicates or mismatches the staged `rustc`, and multiple campaign hosts yield `DuplicateTool`. `resolvedTools` must not represent a campaign host. The lock binds campaign tools only through `measurementPolicy.fuzzCorpora`, the SHA-256 digest of `qualification/staged/fuzz-corpora.json`.
- `qualification/staged/fuzz-corpora.json`, `instrumentation.campaignToolchain.hostToolRecords`: before each campaign, capture hostname, host triple, reference status, operating system, selector, Rust commit, CPU-accounting fields, and each executable's `name`, `version`, `sourceIdentity`, `hostTriple`, `licenseId`, `executablePath`, and `sha256`; retain an existing `pathRoot` only as campaign-record data. Capture `licenseId` from the package's authoritative license metadata or notice; reject an absent or non-SPDX value. The existing canonical block already holds these host records, including `licenseId`, so this correction requires no canonical-block change. Require a complete selected record, including license ID, executable hashes, and CPU-accounting fields, for every campaign host. Add `campaign-host-tool-records` to both known-unknown arrays. Clear it only when every campaign host that produced evidence has a complete selected `hostToolRecords` entry. This rule must not alter `resolved-tool-digests`, which remains OXY-A008's `resolvedTools` gate.
- `.constitution/tech-spec/contracts/qualification-lock.json`, `preImplementationKnownUnknowns` and `gatingKnownUnknowns`: after both staged records and every listed source and license byte pass admission, remove `fuzz-corpora` and `security-patch-rehearsal` and add `campaign-host-tool-records` to both arrays. This is a net reduction of one entry in each array. Clear `campaign-host-tool-records` only when every campaign host that produced evidence has a complete selected `hostToolRecords` entry. Leave all unrelated readiness gates, including `resolved-tool-digests`, unchanged.
- `crates/oxyflut-qualification/src/readiness.rs`, `KNOWN_UNKNOWN_BINDINGS`: remove the existing `fuzz-corpora` row (`required_field: "measurementPolicy.fuzzCorpora"`, `evidence_path: Some("qualification/staged/fuzz-corpora.json")`, upstream owner `OXY-D001`) and `security-patch-rehearsal` row (`required_field: "measurementPolicy.securityPatchRehearsal"`, `evidence_path: Some("qualification/staged/security-patch-rehearsal.json")`, upstream owner `OXY-D001`) when their KU strings leave the arrays. This spike adds `campaign-host-tool-records`; add this exact binding row:

```rust
KnownUnknownBinding {
    known_unknown: "campaign-host-tool-records",
    required_field: "measurementPolicy.fuzzCorpora",
    evidence_path: Some("qualification/staged/fuzz-corpora.json"),
    upstream_owner: "OXY-D001",
},
```

Clear that KU only when every campaign host that produced evidence has a complete selected `hostToolRecords` entry. Keep both `POLICY_FIELDS` rows: they bind and verify the staged policy digests even after the two policy KU blockers clear. Update the module's `clearing_a_ku_string_without_its_evidence_keeps_the_gate_open` expected KU set and its KU evidence-path loop by removing the two policy KUs and adding `campaign-host-tool-records` with evidence path `qualification/staged/fuzz-corpora.json`.

- `qualification/fixtures/readiness/invalid.json` and `qualification/fixtures/readiness/cleared-without-evidence.json`, `preImplementationKnownUnknowns` and `gatingKnownUnknowns`: remove `fuzz-corpora` and `security-patch-rehearsal` and add `campaign-host-tool-records` to all four fixture arrays. This is a net reduction of one entry in each array. Without this fixture update, `collect_known_unknowns` returns `ReadinessError::UnmappedKnownUnknown` before the intended fixture assertions run.
- `xtask/src/commands/lock_tests.rs`, `committed_candidate_gate_is_valid_but_open_with_the_exact_ku_set`: remove `fuzz-corpora` and `security-patch-rehearsal`, add `campaign-host-tool-records`, and assert exactly `campaign-host-tool-records`, `capability-and-platform-baselines`, `complete-ime-editing-geometry-and-accessibility-maps`, `external-distribution-schema-snapshots-and-verifiers`, `hardware-gpu-driver-and-system-package-locks`, `independent-presentation-opportunity-sources`, `layout-visit-cap`, `minimum-platform-and-protocol-versions`, `raw-measurement-and-sample-validity-contracts`, `reference-application-scenes-scripts-fonts-assets-windows-cache-and-flags`, `resolved-tool-digests`, and `scoring-anchors-and-two-assessors`. In `cleared_ku_without_evidence_remains_open_with_the_exact_remaining_ku_set`, remove the same two names, add `campaign-host-tool-records`, and assert exactly `campaign-host-tool-records`, `capability-and-platform-baselines`, `complete-ime-editing-geometry-and-accessibility-maps`, `external-distribution-schema-snapshots-and-verifiers`, `hardware-gpu-driver-and-system-package-locks`, `independent-presentation-opportunity-sources`, `layout-visit-cap`, `minimum-platform-and-protocol-versions`, `raw-measurement-and-sample-validity-contracts`, `reference-application-scenes-scripts-fonts-assets-windows-cache-and-flags`, and `scoring-anchors-and-two-assessors`. Both expected `known_unknowns` vectors are in lexicographic `field_path` order, so `campaign-host-tool-records` precedes `capability-and-platform-baselines`. In `candidate_report_lines_are_stable_and_content_free`, replace the two policy KU output-line assertions with `blocking: field-path=preImplementationKnownUnknowns.campaign-host-tool-records kind=ku evidence-path=qualification/staged/fuzz-corpora.json upstream-owner=OXY-D001`. Retain the staged-file digest and missing-file assertions because `POLICY_FIELDS` still verifies both staged inputs.

### Canonical staged inputs

Each displayed JSON block is UTF-8, uses the displayed 2-space indentation and key order, and ends with exactly one LF. The stable canonical-block anchor, `prettier-ignore` directive, and `text` fence protect each byte stream from Markdown formatting. P13b extracts both blocks after Prettier, JSON-reserializes each with Prettier's JSON parser, compares the byte streams, and SHA-256-checks the result.

The campaign policy is host-neutral except for `hostToolRecords`. Its first record captures this NixOS 26.05 host as non-reference. A campaign must select an exact hostname-and-triple match, resolve Rust through the selected dated Rustup toolchain, resolve `cargo-fuzz` and the selected GNU Time executable with `command -v`, and compare hashes against that record. `command -v time` alone yields a shell keyword on this host, so the selected record supplies the executable path to `command -v`; no Nix path or `/home/oscar` path appears in the generic procedure. P11 verifies the five records, their license IDs, and the shell-keyword result. P16 confirms the records remain only in this staged policy; no campaign-host record or tool enters the qualification lock's `resolvedTools`. The `LC_ALL=C` export makes the canonical English `cpuAccounting.requiredFields` labels reproducible before the campaign parser reads `CPU_LOG`.

The corrected current source bytes produce these declared digests and byte counts:

```text
59f239e1e9dffbca7eb9d15be6cb69139435a74d4d86b0c6d8e0ddcc1b93b80d  fuzz-corpora.json  15991 bytes
82037d5fd08495aee0ff2a2c2e7e8a4b9ade4c2f76b65f966586a5872667d9bd  security-patch-rehearsal.json  2000 bytes
```

<!-- canonical-block: fuzz-corpora -->
<!-- prettier-ignore -->
```text
{
  "schemaVersion": "1.0.0",
  "policyId": "OXY-B006-fuzz-corpora-v1",
  "admission": {
    "requireExactUrl": true,
    "requireSha256": true,
    "requireLicenses": true,
    "requiredLicenseEntryKeys": ["licenseId", "licenseUrl", "licenseSha256"],
    "requireSizeAtMostCorpusSetCap": true,
    "rejectPrivateContent": true
  },
  "instrumentation": {
    "addressRequiredProcessCpuSeconds": 86400,
    "concurrencyRequiredProcessCpuSeconds": 28800,
    "addressBuildCommand": "cargo +nightly-2026-08-12 fuzz build --sanitizer address --careful TARGET",
    "concurrencyBuildCommand": "cargo +nightly-2026-08-12 fuzz build --sanitizer thread --careful TARGET",
    "runCommand": "LC_ALL=C; export LC_ALL; MAX_LEN_BYTES=\"$(jq -er --arg ingress \"$INGRESS\" '.ingressMapping[$ingress].maxLenBytes' qualification/staged/fuzz-corpora.json)\"; \"$TIME_BIN\" -v -o CPU_LOG FUZZ_EXE CORPUS -max_total_time=28800 -timeout=5 -max_len=$MAX_LEN_BYTES",
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
    "address": "cargo-fuzz 0.13.2 AddressSanitizer build with --careful",
    "undefinedBehavior": "cargo-fuzz --careful plus cargo +nightly-2026-08-12 miri test replay of every minimized finding",
    "concurrency": "Build TARGET with --sanitizer thread --careful, then use the same timed direct-executable procedure for callback and teardown targets where the environment supports it.",
    "campaignToolchain": {
      "name": "nightly-2026-08-12",
      "rustcCommit": "3d6c19bb9ab4798ecfb2ee943df01a811720fc27",
      "requiredCargoFuzzVersion": "0.13.2",
      "requireHostToolRecordForEveryCampaign": true,
      "preflight": [
        "export LC_ALL=C; TOOLCHAIN=nightly-2026-08-12; HOST_NAME=\"$(hostname)\"; HOST_TRIPLE=\"$(rustc +$TOOLCHAIN -vV | awk '/^host:/ {print $2}')\"",
        "HOST_RECORD=\"$(jq -cer --arg hostname \"$HOST_NAME\" --arg triple \"$HOST_TRIPLE\" '[.instrumentation.campaignToolchain.hostToolRecords[] | select(.hostname == $hostname and .hostTriple == $triple)] | if length == 1 then .[0] else error(\"expected exactly one host record\") end' qualification/staged/fuzz-corpora.json)\"; test -n \"$HOST_RECORD\"",
        "RUSTC_BIN=\"$(rustup which --toolchain $TOOLCHAIN rustc)\"; CARGO_BIN=\"$(rustup which --toolchain $TOOLCHAIN cargo)\"; CARGO_MIRI_BIN=\"$(rustup which --toolchain $TOOLCHAIN cargo-miri)\"; CARGO_FUZZ_BIN=\"$(command -v cargo-fuzz)\"; TIME_BIN=\"$(command -v \"$(printf '%s' \"$HOST_RECORD\" | jq -er '.tools[] | select(.name == \"time\") | .executablePath')\")\"",
        "printf '%s  %s\\n' \"$(printf '%s' \"$HOST_RECORD\" | jq -er '.tools[] | select(.name == \"rustc\") | .sha256')\" \"$RUSTC_BIN\" \"$(printf '%s' \"$HOST_RECORD\" | jq -er '.tools[] | select(.name == \"cargo\") | .sha256')\" \"$CARGO_BIN\" \"$(printf '%s' \"$HOST_RECORD\" | jq -er '.tools[] | select(.name == \"cargo-miri\") | .sha256')\" \"$CARGO_MIRI_BIN\" \"$(printf '%s' \"$HOST_RECORD\" | jq -er '.tools[] | select(.name == \"cargo-fuzz\") | .sha256')\" \"$CARGO_FUZZ_BIN\" \"$(printf '%s' \"$HOST_RECORD\" | jq -er '.tools[] | select(.name == \"time\") | .sha256')\" \"$TIME_BIN\" | sha256sum -c -",
        "export LC_ALL=C; test \"$(rustc +$TOOLCHAIN -vV | awk '/^commit-hash:/ {print $2}')\" = \"$(printf '%s' \"$HOST_RECORD\" | jq -er '.rustcCommit')\"; test \"$(cargo +$TOOLCHAIN fuzz --version)\" = \"$(printf '%s' \"$HOST_RECORD\" | jq -er '.tools[] | select(.name == \"cargo-fuzz\") | .version')\"; test \"$(\"$TIME_BIN\" --version | head -n 1)\" = \"$(printf '%s' \"$HOST_RECORD\" | jq -er '.tools[] | select(.name == \"time\") | .version')\""
      ],
      "hostToolRecords": [
        {
          "hostname": "thinkpadp14s",
          "hostTriple": "x86_64-unknown-linux-gnu",
          "reference": false,
          "operatingSystem": "NixOS 26.05",
          "toolchainSelector": "nightly-2026-08-12",
          "rustcCommit": "3d6c19bb9ab4798ecfb2ee943df01a811720fc27",
          "tools": [
            {
              "name": "rustc",
              "version": "rustc 1.99.0-nightly (3d6c19bb9 2026-08-11)",
              "sourceIdentity": "rustup-toolchain: nightly-2026-08-12-x86_64-unknown-linux-gnu; rust-lang/rust commit: 3d6c19bb9ab4798ecfb2ee943df01a811720fc27",
              "hostTriple": "x86_64-unknown-linux-gnu",
              "licenseId": "MIT OR Apache-2.0",
              "executablePath": "toolchains/nightly-2026-08-12-x86_64-unknown-linux-gnu/bin/rustc",
              "pathRoot": "rustup-home",
              "sha256": "7de94a5c099c8d7ee4cafb905e36d882325faa480d8cff6513dd8c0887fac0c5"
            },
            {
              "name": "cargo",
              "version": "cargo 1.99.0-nightly (b07e5a086 2026-08-07)",
              "sourceIdentity": "rustup-toolchain: nightly-2026-08-12-x86_64-unknown-linux-gnu; rust-lang/rust commit: 3d6c19bb9ab4798ecfb2ee943df01a811720fc27",
              "hostTriple": "x86_64-unknown-linux-gnu",
              "licenseId": "MIT OR Apache-2.0",
              "executablePath": "toolchains/nightly-2026-08-12-x86_64-unknown-linux-gnu/bin/cargo",
              "pathRoot": "rustup-home",
              "sha256": "1cf1cd7feded113706026c5f04fad33e45364546e3c0d92ddee0c1a4c8277296"
            },
            {
              "name": "cargo-miri",
              "version": "miri 0.1.0 (3d6c19bb9a 2026-08-11)",
              "sourceIdentity": "rustup component: miri; nightly-2026-08-12-x86_64-unknown-linux-gnu; rust-lang/rust commit: 3d6c19bb9ab4798ecfb2ee943df01a811720fc27",
              "hostTriple": "x86_64-unknown-linux-gnu",
              "licenseId": "MIT OR Apache-2.0",
              "executablePath": "toolchains/nightly-2026-08-12-x86_64-unknown-linux-gnu/bin/cargo-miri",
              "pathRoot": "rustup-home",
              "sha256": "40a69668c9ff4e5df3e6a87531f2b87dcc5c84e705ee5b06f915fb76383c94af"
            },
            {
              "name": "cargo-fuzz",
              "version": "cargo-fuzz 0.13.2",
              "sourceIdentity": "cargo-fuzz package 0.13.2; tag: 0.13.2; Cargo.toml SHA-256: 26132b1acda063cc70364cee6fbefc4dbc7bad80f99e43d550dfd0a0534e6174",
              "hostTriple": "x86_64-unknown-linux-gnu",
              "licenseId": "MIT OR Apache-2.0",
              "executablePath": "/nix/store/w6g92cm021l24m5815ry1qf57n00k5j2-cargo-fuzz-0.13.2/bin/cargo-fuzz",
              "sha256": "db150590a2f9fa003fb167bc0eec3f90ba5574fcdd01f78110e6f397dda56582"
            },
            {
              "name": "time",
              "version": "time (GNU Time) 1.10",
              "sourceIdentity": "GNU time package 1.10; https://ftp.gnu.org/gnu/time/time-1.10.tar.gz; tarball SHA-256: e8c29fb4ab599d8478e41e8618f50db8aede9c90af27d0d2ef28ae50d5de09c3",
              "hostTriple": "x86_64-unknown-linux-gnu",
              "licenseId": "GPL-3.0-or-later",
              "executablePath": "/run/current-system/sw/bin/time",
              "sha256": "e8b9f5440e01a81e0692e68d07dfacb8059c434cae100c1fbb60b7ec52848480"
            }
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
      "licenses": [
        {
          "licenseId": "Apache-2.0",
          "licenseUrl": "https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/LICENSE-APACHE",
          "licenseSha256": "0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594"
        },
        {
          "licenseId": "MIT",
          "licenseUrl": "https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/LICENSE-MIT",
          "licenseSha256": "c77a4cf9da729987d0fe7ccd811e3bd27393914ddf3d23467c18cc22954513b3"
        }
      ],
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
      "licenses": [
        {
          "licenseId": "OFL-1.1",
          "licenseUrl": "https://raw.githubusercontent.com/notofonts/noto-fonts/ffebf8c1ee449e544955a7e813c54f9b73848eac/LICENSE",
          "licenseSha256": "0dab92d0544f7b233403f14b84a663bdbfa746982eda629e7f4f9ffe1b036feb"
        }
      ],
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
      "licenses": [
        {
          "licenseId": "Unicode-3.0",
          "licenseUrl": "https://web.archive.org/web/20240825031908id_/https://www.unicode.org/license.txt",
          "licenseSha256": "f5062c9a188d81dfe66b56db4182dcf9e4b17c0d9b0d311a8e20b3a1b075c443"
        }
      ],
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
      "licenses": [
        {
          "licenseId": "MIT",
          "licenseUrl": "https://raw.githubusercontent.com/nst/JSONTestSuite/1ef36fa01286573e846ac449e8683f8833c5b26a/LICENSE",
          "licenseSha256": "8bd0e0578be788c617ea01d18b2a8146e3746ae50bddadc65a5f9d3aad08ad49"
        }
      ],
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
      "licenses": [
        {
          "licenseId": "BSD-3-Clause",
          "licenseUrl": "https://raw.githubusercontent.com/web-platform-tests/wpt/461f7e8515940598535c71ae334e188eadde27a3/LICENSE.md",
          "licenseSha256": "5fac07febb0e2a97fb0d7b0def149ec08b642e1ba4b9c345283ab1cbd2af6570"
        }
      ],
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
    "application-assets": {
      "corpusSets": ["image", "font", "json"],
      "maxLenBytes": 1048576
    },
    "operating-environment-events": {
      "corpusSets": ["wpt-events"],
      "maxLenBytes": 65536
    },
    "private-platform-content": {
      "corpusSets": ["unicode-text", "wpt-events"],
      "maxLenBytes": 8388608
    },
    "accessibility": {
      "corpusSets": ["unicode-text", "wpt-events"],
      "maxLenBytes": 8388608
    },
    "candidate-boundary": {
      "corpusSets": ["json", "wpt-events"],
      "maxLenBytes": 65536
    },
    "local-sink": {
      "corpusSets": ["json"],
      "maxLenBytes": 65536
    },
    "candidate-artifacts": {
      "corpusSets": ["json"],
      "maxLenBytes": 65536
    },
    "independent-verification": {
      "corpusSets": ["json"],
      "maxLenBytes": 65536
    }
  }
}
```

<!-- canonical-block: security-patch-rehearsal -->
<!-- prettier-ignore -->
```text
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
    "cargo test -p oxyflut-assets asset_decode_replays_image_registry",
    "cargo +nightly-2026-08-12 fuzz build --sanitizer address --careful asset_decode",
    "MAX_LEN_BYTES=\"$(jq -er '.ingressMapping[\"application-assets\"].maxLenBytes' qualification/staged/fuzz-corpora.json)\"; LC_ALL=C \"$TIME_BIN\" -v -o CPU_LOG FUZZ_EXE CORPUS -max_total_time=28800 -timeout=5 -max_len=$MAX_LEN_BYTES",
    "record User time (seconds) plus System time (seconds) for the successful address shard and resume timed shards with the same corpus until the cumulative process CPU seconds are at least 86400",
    "cargo +nightly-2026-08-12 miri test -p oxyflut-assets checked_rgba_bytes",
    "cargo +nightly-2026-08-12 miri test -p oxyflut-assets asset_decode_replays_image_registry"
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

The output of P13a follows:

```text
$ perl /tmp/wf-epic-b/OXY-B006-pr-round-12/verify-canonical-blocks.pl .constitution/spikes/SPK-B006.md /tmp/wf-epic-b/OXY-B006-pr-round-12/p13a
fuzz-corpora|59f239e1e9dffbca7eb9d15be6cb69139435a74d4d86b0c6d8e0ddcc1b93b80d|15991|ok
security-patch-rehearsal|27b6e4525723e2501d08e72169f8194bb070d4a79b32825f3cc70fe9e66fc14c|1991|ok
$ jq -e . /tmp/wf-epic-b/OXY-B006-pr-round-12/p13a/fuzz-corpora.json >/dev/null
$ jq -e . /tmp/wf-epic-b/OXY-B006-pr-round-12/p13a/security-patch-rehearsal.json >/dev/null
$ sha256sum /tmp/wf-epic-b/OXY-B006-pr-round-12/p13a/fuzz-corpora.json /tmp/wf-epic-b/OXY-B006-pr-round-12/p13a/security-patch-rehearsal.json
59f239e1e9dffbca7eb9d15be6cb69139435a74d4d86b0c6d8e0ddcc1b93b80d  /tmp/wf-epic-b/OXY-B006-pr-round-12/p13a/fuzz-corpora.json
27b6e4525723e2501d08e72169f8194bb070d4a79b32825f3cc70fe9e66fc14c  /tmp/wf-epic-b/OXY-B006-pr-round-12/p13a/security-patch-rehearsal.json
```

The output of P13b follows:

```text
$ perl /tmp/wf-epic-b/OXY-B006-pr-round-12/verify-canonical-blocks.pl .constitution/spikes/SPK-B006.md /tmp/wf-epic-b/OXY-B006-pr-round-12/p13b
fuzz-corpora|59f239e1e9dffbca7eb9d15be6cb69139435a74d4d86b0c6d8e0ddcc1b93b80d|15991|ok
security-patch-rehearsal|27b6e4525723e2501d08e72169f8194bb070d4a79b32825f3cc70fe9e66fc14c|1991|ok
$ prettier --parser json /tmp/wf-epic-b/OXY-B006-pr-round-12/p13b/fuzz-corpora.json > /tmp/wf-epic-b/OXY-B006-pr-round-12/p13b/fuzz-corpora.reserialized.json
$ prettier --parser json /tmp/wf-epic-b/OXY-B006-pr-round-12/p13b/security-patch-rehearsal.json > /tmp/wf-epic-b/OXY-B006-pr-round-12/p13b/security-patch-rehearsal.reserialized.json
$ cmp -s /tmp/wf-epic-b/OXY-B006-pr-round-12/p13b/fuzz-corpora.json /tmp/wf-epic-b/OXY-B006-pr-round-12/p13b/fuzz-corpora.reserialized.json && cmp -s /tmp/wf-epic-b/OXY-B006-pr-round-12/p13b/security-patch-rehearsal.json /tmp/wf-epic-b/OXY-B006-pr-round-12/p13b/security-patch-rehearsal.json
canonical_json_reserialization=passed
$ sha256sum /tmp/wf-epic-b/OXY-B006-pr-round-12/p13b/fuzz-corpora.json /tmp/wf-epic-b/OXY-B006-pr-round-12/p13b/security-patch-rehearsal.json
59f239e1e9dffbca7eb9d15be6cb69139435a74d4d86b0c6d8e0ddcc1b93b80d  /tmp/wf-epic-b/OXY-B006-pr-round-12/p13b/fuzz-corpora.json
27b6e4525723e2501d08e72169f8194bb070d4a79b32825f3cc70fe9e66fc14c  /tmp/wf-epic-b/OXY-B006-pr-round-12/p13b/security-patch-rehearsal.json
```

P14 rechecked both protected streams after the round-4 readiness-binding correction and the round-12 locale re-freeze.

```text
$ perl /tmp/wf-epic-b/OXY-B006-pr-round-12/verify-canonical-blocks.pl .constitution/spikes/SPK-B006.md /tmp/wf-epic-b/OXY-B006-pr-round-12/final
fuzz-corpora|59f239e1e9dffbca7eb9d15be6cb69139435a74d4d86b0c6d8e0ddcc1b93b80d|15991|ok
security-patch-rehearsal|27b6e4525723e2501d08e72169f8194bb070d4a79b32825f3cc70fe9e66fc14c|1991|ok
```

P13a, P13b, and P14 preserve their round-12 verification output. P21 re-freezes the current canonical block after adding the GNU Time locale assignment and verifies both protected streams without changing `fuzz-corpora`.

The output of P21 follows:

```text
$ perl /tmp/wf-epic-b/OXY-B006-pr-round-14/verify-canonical-blocks.pl .constitution/spikes/SPK-B006.md /tmp/wf-epic-b/OXY-B006-pr-round-14/p21
fuzz-corpora|59f239e1e9dffbca7eb9d15be6cb69139435a74d4d86b0c6d8e0ddcc1b93b80d|15991|ok
security-patch-rehearsal|82037d5fd08495aee0ff2a2c2e7e8a4b9ade4c2f76b65f966586a5872667d9bd|2000|ok
$ jq -e . /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/fuzz-corpora.json >/dev/null
$ jq -e . /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/security-patch-rehearsal.json >/dev/null
$ prettier --parser json /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/fuzz-corpora.json > /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/fuzz-corpora.reserialized.json
$ prettier --parser json /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/security-patch-rehearsal.json > /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/security-patch-rehearsal.reserialized.json
$ cmp -s /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/fuzz-corpora.json /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/fuzz-corpora.reserialized.json
$ cmp -s /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/security-patch-rehearsal.json /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/security-patch-rehearsal.reserialized.json
canonical_json_reserialization=passed
$ sha256sum /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/fuzz-corpora.json /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/security-patch-rehearsal.json
59f239e1e9dffbca7eb9d15be6cb69139435a74d4d86b0c6d8e0ddcc1b93b80d  /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/fuzz-corpora.json
82037d5fd08495aee0ff2a2c2e7e8a4b9ade4c2f76b65f966586a5872667d9bd  /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/security-patch-rehearsal.json
$ wc -c /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/fuzz-corpora.json /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/security-patch-rehearsal.json
15991 /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/fuzz-corpora.json
 2000 /tmp/wf-epic-b/OXY-B006-pr-round-14/p21/security-patch-rehearsal.json
17991 total
```

P19 confirmed that the recorded host uses `LANG=es_CL.UTF-8` and that the locale pin produces the exact English labels. The [GNU Time 1.10 manual](https://www.gnu.org/software/time/manual/time.html) documents the same verbose labels; P19 also found the upstream 1.10 source's explicit no-gettext declaration, so this probe doesn't assume that an Ubuntu package has no downstream localization patch.

```text
$ printf '%s\n' "LANG=$LANG" "LC_ALL=${LC_ALL-}"
LANG=es_CL.UTF-8
LC_ALL=
$ LC_ALL=C /run/current-system/sw/bin/time -v -o time-C.log true; sed -n '1,4p' time-C.log
	Command being timed: "true"
	User time (seconds): 0.00
	System time (seconds): 0.00
	Percent of CPU this job got: 50%
$ grep -n -A 1 -B 1 -F 'No gettext support for now.' time-1.10/src/system.h
22-
23:/* No gettext support for now.  */
24-#define _(x) (x)
```

### Canonical fenced-block integrity proposal

Stage 3 must add an `xtask` or continuous-integration check that extracts the exact body after each stable `canonical-block` anchor, includes its terminal LF, excludes the fences, and SHA-256-checks the raw bytes before accepting a report change. Run the check after Prettier. The protected streams use `text` fences so Markdown formatting can't rewrite their bytes.

The check must cover these anchors and digests:

- `fuzz-corpora`: `59f239e1e9dffbca7eb9d15be6cb69139435a74d4d86b0c6d8e0ddcc1b93b80d` (15,991 bytes).
- `security-patch-rehearsal`: `82037d5fd08495aee0ff2a2c2e7e8a4b9ade4c2f76b65f966586a5872667d9bd` (2,000 bytes).

## Sources

- [Flutter 3.41.0 framework engine version](https://raw.githubusercontent.com/flutter/flutter/44a626f4f0027bc38a46dc68aed5964b05a83c18/bin/internal/engine.version)
- [Flutter 3.44.0 framework engine version](https://raw.githubusercontent.com/flutter/flutter/559ffa3f75e7402d65a8def9c28389a9b2e6fe42/bin/internal/engine.version)
- [Flutter 3.47.0 framework engine version](https://raw.githubusercontent.com/flutter/flutter/4cf24164269a5ebf0c16a028a00727d0e77bbb05/bin/internal/engine.version)
- [Flutter 3.41.0 engine `DEPS`](https://raw.githubusercontent.com/flutter/flutter/3452d735bd38224ef2db85ca763d862d6326b17f/DEPS)
- [Flutter 3.44.0 engine `DEPS`](https://raw.githubusercontent.com/flutter/flutter/4c525dac5ebe5971c5708ef73558ed8edcf4a362/DEPS)
- [Flutter 3.47.0 engine `DEPS`](https://raw.githubusercontent.com/flutter/flutter/5f77625673248ee5846fbcaf5d3e1a3878386fd7/DEPS)
- [libpng `08da33b` fix](https://github.com/pnggroup/libpng/commit/08da33b4c88cfcd36e5a706558a8d7e0e4773643)
- [libpng `08da33b` patch](https://github.com/pnggroup/libpng/commit/08da33b4c88cfcd36e5a706558a8d7e0e4773643.patch)
- [libpng `f139fd5d` `pngrtran.c`](https://flutter.googlesource.com/third_party/libpng/+/f139fd5d80944f5453b079672e50f32ca98ef076/pngrtran.c?format=TEXT)
- [libpng `b6004397` `pngrtran.c`](https://flutter.googlesource.com/third_party/libpng/+/b6004397d2ab98f0250376d9b357337b7f422d13/pngrtran.c?format=TEXT)
- [Impeller interop GN target](https://raw.githubusercontent.com/flutter/flutter/5f77625673248ee5846fbcaf5d3e1a3878386fd7/engine/src/flutter/impeller/toolkit/interop/BUILD.gn)
- [Skia libpng GN target](https://raw.githubusercontent.com/flutter/flutter/5f77625673248ee5846fbcaf5d3e1a3878386fd7/engine/src/flutter/skia/BUILD.gn)
- [full-engine shell GN target](https://raw.githubusercontent.com/flutter/flutter/5f77625673248ee5846fbcaf5d3e1a3878386fd7/engine/src/flutter/shell/common/BUILD.gn)
- [Unicode 16.0.0 UCD ReadMe](https://www.unicode.org/Public/16.0.0/ucd/ReadMe.txt)
- [Unicode 16.0.0 GraphemeBreakTest](https://www.unicode.org/Public/16.0.0/ucd/auxiliary/GraphemeBreakTest.txt)
- [Unicode 16.0.0 BidiTest](https://www.unicode.org/Public/16.0.0/ucd/BidiTest.txt)
- [Unicode License V3 dated snapshot](https://web.archive.org/web/20240825031908id_/https://www.unicode.org/license.txt)
- [Unicode CLDR 45 release note and SPDX mapping](https://cldr.unicode.org/downloads/cldr-45)
- [LLVM libFuzzer documentation](https://llvm.org/docs/LibFuzzer.html)
- [cargo-fuzz 0.13.2 README](https://raw.githubusercontent.com/rust-fuzz/cargo-fuzz/0.13.2/README.md)
- [Rust 3d6c19b COPYRIGHT notice](https://raw.githubusercontent.com/rust-lang/rust/3d6c19bb9ab4798ecfb2ee943df01a811720fc27/COPYRIGHT)
- [Rust 3d6c19b Apache-2.0 notice](https://raw.githubusercontent.com/rust-lang/rust/3d6c19bb9ab4798ecfb2ee943df01a811720fc27/LICENSE-APACHE)
- [Rust 3d6c19b MIT notice](https://raw.githubusercontent.com/rust-lang/rust/3d6c19bb9ab4798ecfb2ee943df01a811720fc27/LICENSE-MIT)
- [cargo-fuzz 0.13.2 package metadata](https://raw.githubusercontent.com/rust-fuzz/cargo-fuzz/0.13.2/Cargo.toml)
- [cargo-fuzz 0.13.2 Apache-2.0 notice](https://raw.githubusercontent.com/rust-fuzz/cargo-fuzz/0.13.2/LICENSE-APACHE)
- [cargo-fuzz 0.13.2 MIT notice](https://raw.githubusercontent.com/rust-fuzz/cargo-fuzz/0.13.2/LICENSE-MIT)
- [GNU Time 1.10 manual](https://www.gnu.org/software/time/manual/time.html)
- [GNU Time 1.10 source distribution](https://ftp.gnu.org/gnu/time/time-1.10.tar.gz)
- [image Apache-2.0 notice](https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/LICENSE-APACHE)
- [image MIT notice](https://raw.githubusercontent.com/image-rs/image/76e57184f22772dad1138e96954e57945406b15e/LICENSE-MIT)
- The canonical registry names every fetched immutable seed and license URL; P4 preserves the corresponding SHA-256 verification output.
