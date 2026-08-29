# Product constraints

The meters in this file apply to every eligible substrate candidate. Before measurement, freeze the reference application, environments, hardware, workloads, tools, sample-validity rules, and raw-result format.

## Performance and resource constraints

| Constraint ID | Scale | Meter | Goal | Stretch | Fail |
| :-- | :-- | :-- | :-- | :-- | :-- |
| CON-PERF-001 | Application-owned layout and paint-submission time per measured frame. | Maximum of 20 per-launch nearest-rank 99th percentiles after 300 warmup and 500 measured frames per launch. | At most 2.0 ms. | At least 30% headroom. | More than 2.0 ms on any reference configuration. |
| CON-PERF-002 | Global heap allocations during steady-state application-owned paint traversal. | Count allocations across 10,000 measured frames after cache warmup. | Zero allocations in every frame. | Not applicable. | Any allocation in a measured frame. |
| CON-PERF-003 | Cold launch to first complete acknowledged frame. | Maximum of 60 independent cold launches from process entry to presentation feedback. | At most 50 ms. | At least 30% headroom. | More than 50 ms on any reference configuration. |
| CON-MEM-001 | Idle resident memory for one-view and two-view processes. | Maximum of 10 per-launch nearest-rank 95th percentiles from 60 samples taken once per second after a 10-second idle wait. | At most 25 MiB for each baseline. | At least 30% headroom. | More than 25 MiB for either baseline on any reference configuration. |
| CON-SIZE-001 | Compressed canonical unsigned runtime payload. | Direct byte count of the reproducible release archive, excluding application assets and separate debug symbols. | At most 75 MiB. | At least 30% headroom. | More than 75 MiB on any Tier 1 environment. |
| CON-SIZE-002 | Installed canonical unsigned runtime payload. | Sum of regular-file payload bytes from the canonical manifest. | At most 300 MiB. | At least 30% headroom. | More than 300 MiB on any Tier 1 environment. |

The numeric common-case node-visit limit for CAP-LAY-001 remains a gating known unknown until the prequalification lock binds candidate and environment identities and the 48-tuple timing probe supplies schema-valid evidence under CON-PERF-001 on unblocked reference hardware.

## Rendering and recovery constraints

| Constraint ID | Scale | Meter | Goal | Stretch | Fail |
| :-- | :-- | :-- | :-- | :-- | :-- |
| CON-FRM-001 | Presented frames relative to independently observed presentation opportunities for each continuously animated view. | One-to-one causal matching during a frozen 10-second display epoch after settling. | Match 95% to 100% of eligible opportunities with a 95th-percentile interval error no greater than 10%. | Match 100% with a 95th-percentile interval error no greater than 5%. | Less than 95%, more than 100%, an unmatched presentation, or interval error above 10%. |
| CON-FRM-002 | Rendering by an idle peer view. | Count frames after settling and before explicit invalidation. | Zero frames. | Not applicable. | Any rendered frame. |
| CON-REC-001 | Resize recovery. | From the later of the final resize event and resource availability to acknowledged correctly sized output. | At most two destination-display refresh intervals. | One refresh interval. | More than two refresh intervals. |
| CON-REC-002 | Surface-loss recovery. | From an externally observed surface-loss event to acknowledged valid output. | At most 250 ms. | At most 125 ms. | More than 250 ms. |
| CON-REC-003 | Resume or display-topology recovery. | From the operating-system event to acknowledged valid output. | At most 500 ms. | At most 250 ms. | More than 500 ms. |
| CON-REC-004 | Recoverable graphics-device-loss recovery. | From the external device-loss event to acknowledged valid output. | At most 2 seconds. | At most 1 second. | More than 2 seconds. |
| CON-REC-005 | Transient recovery memory. | Maximum recovery allocation relative to steady-state graphics allocation. | At most 2x steady state. | At most 1.5x steady state. | More than 2x steady state. |
| CON-REC-006 | Consecutive recreation attempts for any recoverable fault. | Count attempts from the first fault event through success or terminal failure. | At most three attempts, followed by a structured terminal error if recovery fails. | Recovery on the first attempt. | A fourth attempt or no structured terminal error. |
| CON-REC-007 | Lifetime of resources superseded during recovery. | Measure from acknowledged recovery success or terminal failure until release. | At most 500 ms. | At most 250 ms. | More than 500 ms. |

## Determinism and compatibility constraints

| Constraint ID | Scale | Meter | Goal | Stretch | Fail |
| :-- | :-- | :-- | :-- | :-- | :-- |
| CON-DET-001 | Rendering repeatability in a pinned reference environment. | Compare encoded raster output from 20 repeated runs. A targeted pixel invariant can replace a whole-view baseline only when preserved evidence shows equal regression sensitivity. | Byte-identical output or exact satisfaction of an approved targeted invariant. | Not applicable. | Any byte difference outside an approved invariant or any invariant failure. |
| CON-DET-002 | Rendering agreement across platforms or rendering families. | Apply a predeclared channel threshold or perceptual metric to platform-specific baselines. | Every image stays within the frozen threshold. | At least 30% threshold headroom. | Any image exceeds the threshold. |
| CON-UPG-001 | Maintenance across two consecutive substrate upgrades. | Sum attributable engineering time and manually resolved non-generated files across the same two transitions. | At most 10 person-days and 40 files. | At most 2 person-days and 10 files. | Either goal is exceeded. |
| CON-COMP-001 | Production application dependence on a secondary application runtime. | Inspect process startup, the complete linked production payload, and executed application code. | The production payload omits the runtime, the runtime doesn't start, and application code doesn't execute through it. | Not applicable. | The payload contains the runtime, the runtime starts, or application code executes through it. |

## Safety, security, privacy, and operations constraints

| Constraint ID | Scale | Meter | Goal | Stretch | Fail |
| :-- | :-- | :-- | :-- | :-- | :-- |
| CON-SAFE-001 | Unsafe-boundary memory and thread safety. | Layout conformance, ownership, reentrancy, teardown, sanitizer, and stress evidence for every boundary. | No unresolved panic, exception crossing, sanitizer report, deadlock, use-after-free, or double release. | Independent audit finds no unresolved medium-or-higher issue. | Any unresolved boundary failure. |
| CON-SEC-001 | Robustness of each implemented untrusted parser ingress. | At least 24 CPU-hours of frozen-corpus fuzzing per ingress with memory and undefined-behavior instrumentation. | No unresolved crash, timeout beyond 5 seconds, resource-cap breach, or instrumentation report. | Repeat the campaign independently. | Any unresolved finding. |
| CON-SEC-002 | Concurrent callback and teardown robustness. | At least 8 CPU-hours of concurrency instrumentation where the environment supports it. | No unresolved race, deadlock, or lifecycle failure. | Repeat the campaign independently. | Any unresolved finding. |
| CON-SEC-003 | Dependency vulnerability response. | Measure time from disclosure or detection to applicability triage and from applicability confirmation to remediation or risk acceptance. | Triage critical issues within 1 business day, high issues within 3 business days, and medium issues within 30 calendar days. Remediate critical issues within 7 calendar days and high issues within 14 calendar days. Remediate or accept medium risk within 90 calendar days. | Remediate critical issues within 3 days and high issues within 7 days. | A critical or high issue misses either deadline, or a medium issue has no disposition within 90 days. |
| CON-PRV-001 | Raw private content in production diagnostics. | Inspect schemas, build variants, and captured records for clipboard, entered text, composition, and accessibility content. | Zero raw private-content fields or collection paths. | Not applicable. | Any raw private content is collected. |
| CON-DIA-001 | Privacy-safe local-diagnostics overhead. | Maximum of 20 matched-pair differences over a frozen 60-second workload. | Less than 1 percentage point CPU, 1 MiB peak resident memory, and 0.05 ms 99th-percentile frame time. | Half each limit. | Any goal is met or exceeded. |
| CON-DST-001 | Reproducibility of unsigned release artifacts and metadata. | Compare cryptographic hashes from two independent builders for the canonical payload, software bill of materials, notices, and provenance subject. | Exact equality. | An external builder also reproduces every item. | Any required item differs or fails validation. |
| CON-LIC-001 | Fulfillment of release license obligations. | Trace every packaged source, binary, font, data file, shader, and generated tool to its license and required notices or offers. | 100% coverage. | Independent review finds no omission. | Any packaged item lacks a resolved obligation. |

## Substrate selection policy

After hard-gate eligibility, two assessors independently assign an integer score from 3 through 5 to each criterion from cited evidence. A person who authors candidate implementation or qualification evidence for a candidate must not serve as an independent scorer for that candidate. They must record one consensus score for every disagreement. Multiply each consensus score by its weight and divide by 5 to produce a 100-point result.

| Criterion                                       | Weight |
| :---------------------------------------------- | -----: |
| Demonstrated P0 platform coverage               |     30 |
| Two-transition upgrade-maintenance cost         |     20 |
| Performance, startup, memory, and artifact size |     15 |
| Boundary safety, security, and privacy          |     15 |
| Distribution, licensing, and provenance         |     10 |
| Testing, diagnostics, and operational clarity   |     10 |

The scoring anchors must be frozen before either candidate implementation begins. CAP-SUB-002 through CAP-SUB-004 govern eligibility, zero-candidate and one-candidate outcomes, weighted selection, and the maintenance-first tie-break.

Qualification uses a declared sequence. The integrated candidate enters the frozen suite first; build and qualify the focused candidate only if the integrated candidate fails hard-gate eligibility in the first Tier 1 environment. CAP-SUB-001 requires the same complete frozen suite for every candidate that enters qualification.

Qualify Tier 1 environments in this order: Wayland, X11, macOS, then Windows. Apply readiness gates separately to each environment on pinned reference hardware the project controls. An environment without accountable reference hardware remains blocked. A selection supported by complete evidence from the first environment is provisional and becomes final only after every Tier 1 environment passes.

CAP-SUB-003 selects the sole eligible candidate, subject to the provisional-to-final rule. The two-assessor weighted comparison applies only when two candidates are eligible.

Sequencing leaves CAP-SUB-001 through CAP-SUB-004 and CAP-PLT-001 unchanged.
