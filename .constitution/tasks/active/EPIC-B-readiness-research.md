# Epic B: Readiness research and coordination inputs

Resolve the decisions that Stage 3 intentionally records as pre-implementation KUs. Spike tickets produce reports only; they don't edit active specifications or implement candidate code.

#### OXY-B001 Resolve the macOS qualification baseline

- **Type:** Spike
- **Effort:** 3
- **Dependencies:** None
- **Category:** Dependency-Upgrade
- **Scope (In-Scope Files):**
  - `.constitution/spikes/SPK-B001.md`
- **Scope (Out-of-Scope Files):**
  - `.constitution/tech-spec/` (report required changes; don't edit them in the spike)
  - `platform/macos/`
  - Candidate source trees
- **Verification Command:** `prettier --prose-wrap never --check '.constitution/**/*.md'`
- **Expected Success Output:** `exit 0` and a completed spike recommendation
- **STOP Conditions:**
  - STOP if official Apple documentation or a controlled probe cannot establish an interface; retain a gating KU.
  - STOP if the recommendation changes a product capability or architecture boundary.
- **Description:** Complete the one-day spike in `.constitution/spikes/SPK-B001.md` covering the minimum macOS deployment target, AppKit and text-input contracts, VoiceOver mapping, view-associated and independent display timing, service routing, and injectable recovery behavior for both allocations.
- **Acceptance:**
  - **Mode:** runbook_probe
  - **Evidence:**

```text
Procedure:
1. Record cited official API and availability evidence for every baseline row.
2. Run minimal noncandidate API probes where documentation is insufficient.
3. Mark each item KK or gating KU and recommend exact Stage 3 edits.
Pass: The report answers every question or explicitly retains it as a gating KU with the next bounded probe.
```

#### OXY-B002 Resolve the Windows qualification baseline

- **Type:** Spike
- **Effort:** 3
- **Dependencies:** None
- **Category:** Dependency-Upgrade
- **Scope (In-Scope Files):**
  - `.constitution/spikes/SPK-B002.md`
- **Scope (Out-of-Scope Files):**
  - `.constitution/tech-spec/` (report required changes; don't edit them in the spike)
  - `platform/windows/`
  - Candidate source trees
- **Verification Command:** `prettier --prose-wrap never --check '.constitution/**/*.md'`
- **Expected Success Output:** `exit 0` and a completed spike recommendation
- **STOP Conditions:**
  - STOP if official Microsoft documentation or a controlled probe cannot establish an interface; retain a gating KU.
  - STOP if the recommendation relies on `DwmGetCompositionTimingInfo` as a per-view opportunity source.
- **Description:** Complete the one-day spike in `.constitution/spikes/SPK-B002.md` covering the minimum Windows build, Win32, TSF and UTF-16 behavior, Narrator and UI Automation mapping, per-output independent timing, service routing, and injectable DXGI recovery for both allocations.
- **Acceptance:**
  - **Mode:** runbook_probe
  - **Evidence:**

```text
Procedure:
1. Record cited official API and availability evidence for every baseline row.
2. Run minimal noncandidate API probes where documentation is insufficient.
3. Mark each item KK or gating KU and recommend exact Stage 3 edits.
Pass: The report answers every question or explicitly retains it as a gating KU with the next bounded probe.
```

#### OXY-B003 Resolve the Wayland qualification baseline

- **Type:** Spike
- **Effort:** 3
- **Dependencies:** None
- **Category:** Dependency-Upgrade
- **Scope (In-Scope Files):**
  - `.constitution/spikes/SPK-B003.md`
- **Scope (Out-of-Scope Files):**
  - `.constitution/tech-spec/` (report required changes; don't edit them in the spike)
  - `platform/linux/`
  - Candidate source trees
- **Verification Command:** `prettier --prose-wrap never --check '.constitution/**/*.md'`
- **Expected Success Output:** `exit 0` and a completed spike recommendation
- **STOP Conditions:**
  - STOP if compositor behavior is assumed from protocol availability; require cited compositor/version evidence.
  - STOP if `wp_presentation` feedback is treated as an independent presentation-opportunity source.
- **Description:** Complete the one-day spike in `.constitution/spikes/SPK-B003.md` covering minimum compositor and protocol versions, GtkIMContext index and input-purpose behavior, the selected Linux assistive technology and complete AT-SPI mapping, Unicode-scalar AT-SPI character-offset conversion fixtures, an observer independent of both candidate callback streams, service routing, and injectable recovery for both allocations.
- **Acceptance:**
  - **Mode:** runbook_probe
  - **Evidence:**

```text
Procedure:
1. Record cited upstream protocol, GTK, AT-SPI, compositor, and graphics evidence.
2. Run minimal noncandidate probes where documentation is insufficient.
3. Prove Unicode-scalar AT-SPI offsets against UTF-8, UTF-16, grapheme, and logical indices with ASCII, multibyte, combining, and bidirectional fixtures.
4. Mark each item KK or gating KU and recommend exact Stage 3 edits.
Pass: The report answers every question or explicitly retains it as a gating KU with the next bounded probe.
```

#### OXY-B004 Resolve the X11 qualification baseline

- **Type:** Spike
- **Effort:** 3
- **Dependencies:** None
- **Category:** Dependency-Upgrade
- **Scope (In-Scope Files):**
  - `.constitution/spikes/SPK-B004.md`
- **Scope (Out-of-Scope Files):**
  - `.constitution/tech-spec/` (report required changes; don't edit them in the spike)
  - `platform/linux/`
  - Candidate source trees
- **Verification Command:** `prettier --prose-wrap never --check '.constitution/**/*.md'`
- **Expected Success Output:** `exit 0` and a completed spike recommendation
- **STOP Conditions:**
  - STOP if X server behavior is assumed from extension availability; require cited server/version evidence.
  - STOP if X Present completion is treated as an independent presentation-opportunity source.
- **Description:** Complete the one-day spike in `.constitution/spikes/SPK-B004.md` covering minimum X server and extension versions, GtkIMContext behavior, the selected Linux assistive technology and complete AT-SPI mapping, Unicode-scalar AT-SPI character-offset conversion fixtures, an observer independent of both candidate callback streams, service routing, and injectable recovery for both allocations.
- **Acceptance:**
  - **Mode:** runbook_probe
  - **Evidence:**

```text
Procedure:
1. Record cited upstream X11, GTK, AT-SPI, graphics, and distribution evidence.
2. Run minimal noncandidate probes where documentation is insufficient.
3. Prove Unicode-scalar AT-SPI offsets against UTF-8, UTF-16, grapheme, and logical indices with ASCII, multibyte, combining, and bidirectional fixtures.
4. Mark each item KK or gating KU and recommend exact Stage 3 edits.
Pass: The report answers every question or explicitly retains it as a gating KU with the next bounded probe.
```

#### OXY-B005 Resolve the common-case layout visit cap

- **Type:** Spike
- **Effort:** 3
- **Dependencies:** None
- **Category:** Perf
- **Scope (In-Scope Files):**
  - `.constitution/spikes/SPK-B005.md`
- **Scope (Out-of-Scope Files):**
  - `.constitution/prd/constraints.md`
  - `.constitution/tech-spec/` (report required changes; don't edit them in the spike)
  - Product layout implementation
- **Verification Command:** `prettier --prose-wrap never --check '.constitution/**/*.md'`
- **Expected Success Output:** `exit 0` and either a defensible finite cap recommendation or a precise Stage 3 blocker
- **STOP Conditions:**
  - STOP if a cap can be obtained only by weakening CAP-LAY-001 or CON-PERF-001.
  - STOP if intrinsic measurement or text work is silently counted as ordinary per-policy visits.
- **Description:** Complete the one-day spike in `.constitution/spikes/SPK-B005.md` to define the candidate-neutral reference layout corpus, counting algorithm, ordinary policy families, finite per-node cap proposal, failure fixtures, and evidence needed to freeze the numeric KU.
- **Acceptance:**
  - **Mode:** runbook_probe
  - **Evidence:**

```text
Probe plan: Define the counting model and corpus, apply it to deep, wide, nested, virtualized, reordered, and failure cases, and preserve each observed or derived visit count.
Decision rule: Recommend a finite cap only when it remains distinct from intrinsic measurement and text work and is compatible with CON-PERF-001's 2.0 ms aggregate goal.
Expected end state: The report justifies the proposed cap from preserved observations or retains a named blocker without guessing.
```

#### OXY-B006 Select the shared security patch and fuzz-corpus policy

- **Type:** Spike
- **Effort:** 3
- **Dependencies:** None
- **Category:** Security
- **Scope (In-Scope Files):**
  - `.constitution/spikes/SPK-B006.md`
- **Scope (Out-of-Scope Files):**
  - Candidate source trees
  - `fuzz/` implementation
  - `.constitution/tech-spec/` (report required changes; don't edit them in the spike)
- **Verification Command:** `prettier --prose-wrap never --check '.constitution/**/*.md'`
- **Expected Success Output:** `exit 0` and a completed patch/corpus recommendation
- **STOP Conditions:**
  - STOP if the patch doesn't apply to code consumed by both candidates.
  - STOP if corpus licensing, provenance, expected tests, or instrumentation cannot be frozen.
- **Description:** Complete the one-day spike in `.constitution/spikes/SPK-B006.md` to select one real shared upstream security patch or a predeclared synthetic patch, define its expected tests, enumerate frozen seed corpora for every architecture ingress category, and specify attribution and immutable evidence.
- **Acceptance:**
  - **Mode:** runbook_probe
  - **Evidence:**

```text
Probe plan: Check patch applicability against all three frozen Flutter lines and both consumption paths; inventory each architecture ingress category; inspect corpus sources, licenses, payload caps, and expected instrumentation.
Decision rule: Select a real patch only when shared applicability is KK; otherwise predeclare one synthetic patch. Admit only attributable immutable corpus inputs.
Expected end state: The report names one patch, its tests, and a corpus registry plan, or retains a precise blocker without starting candidate work.
```

#### OXY-B007 Record reference hardware access and owners

- **Type:** Chore
- **Effort:** 1
- **Dependencies:** None
- **Category:** Docs
- **Scope (In-Scope Files):**
  - `.constitution/reports/reference-hardware-access.md`
- **Scope (Out-of-Scope Files):**
  - `.constitution/tech-spec/contracts/qualification-lock.json` (don't mark readiness from an access promise)
  - Purchasing or provisioning hardware
- **Verification Command:** `prettier --prose-wrap never --check '.constitution/**/*.md'`
- **Expected Success Output:** `exit 0` and an owner-confirmed access matrix for macOS arm64, Windows x86-64, Wayland x86-64, and X11 x86-64
- **STOP Conditions:**
  - STOP if no accountable owner or access procedure exists for an environment; record it as blocked.
  - STOP if one machine/session is described as two independent hardware configurations.
- **Description:** Record the available machines, accountable owners, access procedure, scheduling constraints, hardware/GPU suitability, and whether second-configuration score-4 evidence is feasible. This is an access register, not qualification evidence.
- **Acceptance:**
  - **Mode:** hitl_sil
  - **Evidence:**

```text
Procedure: Each machine owner confirms environment, architecture, GPU, interactive-session availability, administrator requirements, and a repeatable access window.
Pass log: Four Tier 1 rows have a named owner and usable procedure, or the missing row is explicitly blocked without claiming candidateImplementationReady.
```

#### OXY-B008 Freeze the two assessor identities

- **Type:** Chore
- **Effort:** 1
- **Dependencies:** None
- **Category:** Docs
- **Scope (In-Scope Files):**
  - `.constitution/reports/qualification-assessors.md`
- **Scope (Out-of-Scope Files):**
  - Candidate scores
  - Candidate selection
  - `.constitution/tech-spec/contracts/qualification-lock.json` until both assessors accept the role
- **Verification Command:** `prettier --prose-wrap never --check '.constitution/**/*.md'`
- **Expected Success Output:** `exit 0` and two consenting, distinct assessor identities with the frozen consensus procedure
- **STOP Conditions:**
  - STOP if two independent assessors aren't named and available before candidate implementation.
  - STOP if either assessor has already seen candidate score conclusions before recording independent scores.
- **Description:** Record two assessor identities, roles, availability, conflict disclosures, independence rules, evidence-access procedure, and the already specified written consensus process. Don't assign or preview any candidate score.
- **Acceptance:**
  - **Mode:** hitl_sil
  - **Evidence:**

```text
Procedure: Both assessors separately confirm participation, independence, the integer 3–5 scale, the six frozen criteria, and written consensus for disagreements.
Pass log: Two distinct confirmations are preserved before candidate implementation begins.
```
