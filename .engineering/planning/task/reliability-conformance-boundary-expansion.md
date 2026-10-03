---
format: aep.planning-md/3
id: task:reliability-conformance-boundary-expansion
kind: task
status: proposed
title: Represent existing reliability guarantees at executable ESS boundaries
relations:
- informed_by: review-result:reliability-hardening-design-structured
revision: 2
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T14:12:53Z", actor: "human:timo", revision: 2}
---
## Outcome
Strengthen ESS observations for existing Mantle reliability behavior identified by hardening
findings HD-01 through HD-09. The complete source-cited review is
review-result:reliability-hardening-design-structured; its implementation-seam proposal is
.engineering/reports/reliability/hardening-design-remediation.md. These are specification gaps,
not established runtime defects. Existing package checks remain in the required workspace gate.

## Scope and disposition
This is proposed follow-up work, outside the bounded design/mutation audit and the five-roadmap-item
integration delivery. Do not call these gaps fixed by filing this task. The common dependency is
an explicit exact-candidate executable seam for subprocess-backed native conformance; current
native ESS runs in a unit-test binary, while subprocess fixtures use Cargo integration-test
executable paths. Reuse fixtures; do not run Cargo recursively, choose an arbitrary PATH binary,
or report a package test's exit code as fabricated ESS observations. No new public capabilities,
live calls, credentials, publication or new parallel model are required.

## Acceptance
Extend the existing ESS boundaries before adapter changes, with named cases and measured outputs:
- HD-01: binary exec streams and malformed-envelope failure, preserving the clarified decode boundary.
- HD-02: selected SSH key/known-host and private short socket isolation through unusual state paths.
- HD-03 through HD-05: shared actual CLI fixture for bounded local IO and descendant retirement,
  safe reports/process status/selection, independent installed/live/wire identity and committed WAL.
- HD-06: real clean exact-revision and export byte/inventory/mode checks through production Git seams.
- HD-07: successful complete installation, concurrent selection and the actual interrupted lock-wait
  phase, with before/after generation identities; do not generalize to untested crash phases.
- HD-08: controlled bot transport proving source/tag authority, immutability, verified remote assets
  and honest partial failure without publishing anything.
- HD-09: entry safety, bounded input, executable/static/runtime checks and recognized notice provenance.
  Preserve actual locked notice generation and fresh-build CI evidence separately; tiny fixture
  notices do not prove complete license grants or a real source build.
Run each named ESS case against production seams, demonstrate representative planted defects fail,
restore green and pass the complete repository gate. Keep tests already protecting these rules.

## Routing
HD-10 remains with the already planned unit7 documentation reconciliation. HD-11 was clarified in
the command-correctness story and README; it adds no raw-response salvage behavior. The independently
measured checksum survivor is corrected in the current audit and is separate from these nine gaps.
This is one proposed follow-up item; no multi-item decomposition/critic panel was performed.

## Scope
Cited: spec/domains/orchestration.yaml, spec/domains/operator.yaml, spec/scenarios/cli,
spec/ess-inputs.yaml, spec/conformance-baseline.json, crates/mantle/src/adapters/orchestration.rs,
crates/mantle/src/adapters/conformance.rs, crates/mantle/tests, crates/mantle-release/tests,
crates/mantle-release/src/build.rs, crates/mantle-release/src/publish.rs,
crates/mantle-release/src/licenses.rs, Taskfile.yml and .github/workflows/ci.yml.
Inferred: shared Rust test-support extraction and native report aggregation wiring.
