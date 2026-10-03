---
format: aep.planning-md/3
id: task:reliability-hardening-design
kind: task
status: implemented
title: Harden the green reliability specification against accepted design
relations:
- delivers: epic:reliability-and-usability
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T13:50:43Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-03T13:50:43Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-03T14:24:57Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"review_outcome":10,"verification":1}}}
---
## Outcome
Run ESS hardening design/spec review of the already-green reliability slice, explicitly requested by the operator during implementation. Use the accepted command-correctness, named-profiles, worker-doctor and prebuilt-release-artifacts stories as the design contracts, their documented current behavior, and spec/README.md as the existing mapping. Broader architecture proposals are context, not silently promoted into current deliverables. Worker upgrades, retained lifecycle and acceptance remain governed by their own delivery reviews.

## Acceptance
Validate the pinned specification, demonstrate the review catches a deliberately deleted declared rule on an isolated copy with both design and spec citations, restore and review the genuine baseline. Return findings classified missing/contradicts/stale mapping/spec-only/unclear with both citations, separately label spec gaps/implementation defects/technique false positives, and list unreviewed sections. No clean-review claim without the planted failure. Root records and routes findings before final PR. This task adds verification, not a new product capability or extra PR.

## Scope
Read-only review on a managed exact integration checkpoint with source equivalent to the last green381-case gate. All mutations occur only on scratch copies. No live resources, auth data, AEP writes, implementation edits or publication. Report and small machine-readable evidence are integrated by root.

## Disposition
The complete unchanged report and negative control are recorded in
review-result:reliability-hardening-design-structured (the prose-only predecessor is archived).
HD-01 through HD-09 remain open in proposed task:reliability-conformance-boundary-expansion,
with exact source seams and acceptance; this audit does not claim to have closed them.
They require a broader shared subprocess conformance seam and are follow-up work, while existing
package coverage stays in the repository gate. HD-10 is assigned to unit7's final documentation
reconciliation. HD-11 is clarified in the command-correctness story and README: exact bytes follow
successful SDK-envelope decoding; malformed wire responses fail without raw-response salvage.
The separate mutation audit corrects its measured checksum survivor in this integration.
