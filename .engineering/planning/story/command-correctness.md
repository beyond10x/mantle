---
format: aep.planning-md/3
id: story:command-correctness
kind: story
status: active
title: Propagate remote command outcomes and resolve repository tags
relations:
- decomposes: epic:reliability-and-usability
scope:
- confidence: inferred
  path: README.md
- confidence: cited
  path: crates/mantle/src/adapters/orchestration.rs
- confidence: cited
  path: crates/mantle/src/app/session.rs
- confidence: cited
  path: crates/mantle/src/main.rs
- confidence: inferred
  path: crates/mantle/tests/command_correctness.rs
- confidence: inferred
  path: crates/mantle/tests/support/ssh_fixture.rs
- confidence: inferred
  path: generated/worker-model
- confidence: inferred
  path: spec/README.md
- confidence: inferred
  path: spec/conformance-baseline.json
- confidence: inferred
  path: spec/domains/orchestration.yaml
- confidence: inferred
  path: spec/ess-inputs.yaml
- confidence: inferred
  path: spec/scenarios/cli
- confidence: inferred
  path: website/index.html
revision: 19
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T08:26:55Z", actor: "human:timo", revision: 13}
- {from: "proposed", to: "active", at: "2026-10-03T08:26:55Z", actor: "human:timo", revision: 14}
---
## Problem
Source inspection: crates/mantle/src/app/session.rs:470 returns Ok after any SDK execution result, and main.rs:142 forwards it, so remote nonzero exits report laptop success. materialize at session.rs:303 promises manifest branch/tag/commit resolution but clones --no-tags --branch for symbolic references. Existing adapters/orchestration.rs:55 merely supplies canned Git success, so real Git must establish the tag defect.

## Acceptance
Named ESS cases: exec-exit-zero, exec-exit-nonzero, exec-signal, exec-indeterminate, exec-output, source-branch, source-lightweight-tag, source-annotated-tag, source-pinned-commit, source-missing-ref. Remote normal exit0/1/42/255 becomes the same laptop code. POSIX signals map to128+signal within1..255; invalid codes, missing/contradictory results, refusal, Unknown/Expired/Cancelled and transport errors never become success and retain truthful diagnostics. Output streams retain exact bytes. A subprocess regression covers the actual CLI termination boundary, not only a pure mapping. Real local Git fixtures execute production materialization argv and verify resolved/persisted commit for branch, lightweight/annotated tag and exact commit; missing ref never starts agent or invents source. Preserve proxy, aperture, argument validation and secret-free supplementary requests.

## Scope
Cited from read-only story-scoper: session.rs, main.rs, adapters/orchestration.rs, website/index.html and spec/README.md. Inferred: orchestration domain/scenarios/inputs/baseline, real-process test file and generated worker-model provenance when specification changes. README success-warning is removed only after verified fix. Scope overlaps subsequent profiles/doctor main and conformance edits, so units are serial.
