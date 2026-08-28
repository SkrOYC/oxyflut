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

## Completion record

Epic B completed its 20 story points as research and coordination inputs only. No ticket edited an active specification, set a readiness flag, or started candidate work. Every outcome below cites the committed deliverable that carries it.

### Ticket outcomes

- **OXY-B001** — `.constitution/spikes/SPK-B001.md`: chose the A/C mix (Option A for the documented interface availability rows, Option C for every behavior, mapping, timing-independence, routing, recovery, and evidence-publication row). Its decision register records 6 KK rows (B001-01, B001-03, B001-06, B001-09, B001-12 as KK not applicable, and B001-15) and 12 KU (gating) rows, and adds the D0 Stage 3 semantic-role registry as a prerequisite for the accessibility maps.
- **OXY-B002** — `.constitution/spikes/SPK-B002.md`: chose a per-row A / A+C / B / C mix, with Option A limited to the documented focused-host interface surface. Its decision table records 0 KK rows and 20 KU (gating) rows, so the Windows environment stays `ku-gating` and no allocation can collect scored evidence from this spike.
- **OXY-B003** — `.constitution/spikes/SPK-B003.md`: chose a mix of A, B, and C — freeze the source-level Wayland core, shell, scale, text-input, clipboard, presentation, GTK 4.20.4, and AT-SPI 2.60.6 floors; adopt the DRM `drm:drm_vblank_event` design as prospective only; retain every reference-session and candidate-specific row. Table 1 records 9 KK rows and 12 KU (gating) rows.
- **OXY-B004** — `.constitution/spikes/SPK-B004.md`: chose Option A for the Ubuntu package and protocol contracts, "A plus C" for the AT-SPI maps and scalar conversion, and Option C for native X server behavior, independent timing, service routing, and recovery. Table 1 records 5 KK rows and 9 KU (gating) rows.
- **OXY-B005** — `.constitution/spikes/SPK-B005.md`: chose Option A for rows 1 through 5 and Option C for row 6, and chose the no-exclusion sample-validity Option B. Table 1 records 5 KK rows (rows 4 and 5 as KK not applicable) and 1 KU (gating) row. `2` is recorded as a derived probe threshold, not a frozen cap; `measurementPolicy.layoutVisitCap` stays `null`.
- **OXY-B006** — `.constitution/spikes/SPK-B006.md`: chose Option B, the predeclared synthetic shared patch `OXY-SYN-SEC-001`, with four frozen post-patch tests and an ingress-complete seed-corpus registry. Table 1 records 5 KK rows and 1 KU (gating) row.
- **OXY-B007** — `.constitution/reports/reference-hardware-access.md`: 2 CONFIRMED rows (Wayland x86-64 and X11 x86-64) and 2 BLOCKED rows (macOS arm64 and Windows x86-64); the answer table records 6 KK and 6 gating KU rows. Both confirmed rows are owner-attested non-reference access, not reference-environment conformance.
- **OXY-B008** — `.constitution/reports/qualification-assessors.md`: CLOSED AS BLOCKED. The ticket pass rule requires two distinct preserved confirmations and only assessor 1 is recorded, so its acceptance pass log remains unmet. The second-assessor confirmation is a named external input routed to OXY-D001; Assessor 2 is unnamed, and assessor 1's disclosed candidate-code and qualification-evidence authorship remains a gating conflict until Stage 1 approves and applies an authorship-independence policy.

##### OXY-B006 Deviations & Justifications

- **Touched Files:** `.constitution/spikes/SPK-B006.md`
- **Justification:** P1 resolved the engine pins for all three frozen Flutter lines and fetched their actual `pngrtran.c` files; both pins already contain the `08da33b` libpng postimage, so the candidate real patch has no remaining preimage and cannot rehearse remediation. The unpinned integrated fork also prevents proving a shared real-patch path. That triggers only the ticket's real-upstream-patch STOP condition, so the spike selected Option B, the predeclared synthetic patch `OXY-SYN-SEC-001`.

##### OXY-B007 Deviations & Justifications

- **Touched Files:** `.constitution/reports/reference-hardware-access.md`
- **Justification:** The ticket's STOP condition requires an environment with no accountable owner or access procedure to be recorded as blocked. The register records B007-Q01, B007-Q02, B007-Q05, B007-Q06, B007-Q09, and B007-Q10 as gating KUs and marks the macOS arm64 and Windows x86-64 rows BLOCKED; its `Spec edits required` section authorizes no Stage 3 edit. The confirmed Wayland and X11 rows are the single physical machine `thinkpadp14s`, an x86_64 NixOS 26.05 host with a Hyprland Wayland session and an Xwayland or Xvfb X11 path, not the Stage 3 Ubuntu 26.04 LTS reference. The same STOP condition forbids treating one machine as two independent hardware configurations, so B007-Q11 and B007-Q12 record no second-configuration score-4 evidence, and B007-Q07 and B007-Q08 record the reference-conformance gap.

##### OXY-B008 Deviations & Justifications

- **Touched Files:** `.constitution/reports/qualification-assessors.md`
- **Justification:** The `hitl_sil` pass log requires two distinct confirmations preserved before candidate implementation. The record preserves one confirmation (Oscar Y. <oscar@ocmasesorias.com>) with conflict disclosures and freezes the second-assessor confirmation, independence, evidence-access, and written-consensus procedures. Its Q4 row is the ticket's STOP condition. OXY-B008 is CLOSED AS BLOCKED: its acceptance pass log remains unmet, and the second-assessor confirmation is a named external input routed to OXY-D001 while Stage 1 owns approval and application of the authorship-independence policy. Its placement in `completed/` records that Epic B has no further in-scope work; it does not complete the assessor gate or permit setting `measurementPolicy.assessors`, `measurementPolicy.scoringAnchors`, or `candidateImplementationReady`.

##### EPIC-B closeout Deviations & Justifications

- **Touched Files:** `.constitution/tasks/critical-path.md`, `.constitution/tasks/changelog.md`, `.constitution/tasks/active/EPIC-D-readiness-reconciliation.md`, and the `.constitution/tasks/active/EPIC-B-readiness-research.md` to `.constitution/tasks/completed/EPIC-B-readiness-research.md` rename.
- **Justification:** Step 5/6 constitution reconciliation records Epic B's completed research inputs, the remaining blocked external inputs, and the resulting OXY-D001 handoff. These closeout changes archive the epic and update task bookkeeping without editing active specifications or changing readiness.

### Stage 3 revisions required — routed to OXY-D001

Every spike's `Spec edits required` or `Downstream impact` section, the hardware register's blocked rows, and the assessor record's blocked second assessor and authorship-independence decision are enumerated in [OXY-D001 inputs from Epic B](../active/EPIC-D-readiness-reconciliation.md#oxy-d001-inputs-from-epic-b). No Epic B ticket applied any of them.
