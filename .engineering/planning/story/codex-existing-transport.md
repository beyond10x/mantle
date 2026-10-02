---
format: aep.planning-md/3
id: story:codex-existing-transport
kind: story
status: active
title: Enable Codex through the existing Claude terminal transport
relations:
- decomposes: story:codex-interactive-start
scope:
- confidence: cited
  path: AGENTS.md
- confidence: cited
  path: README.md
- confidence: cited
  path: crates/mantle/src/adapters/orchestration.rs
- confidence: cited
  path: crates/mantle/src/adapters/substrate.rs
- confidence: cited
  path: crates/mantle/src/app/session.rs
- confidence: cited
  path: crates/mantle/tests/codex_preflight.rs
- confidence: cited
  path: docs/design/00-mantle-on-substrate.md
- confidence: cited
  path: docs/evidence/codex-compatibility.md
- confidence: cited
  path: examples/codex.yaml
- confidence: inferred
  path: generated/worker-model/source.schema.json
- confidence: inferred
  path: generated/worker-model/types-report.json
- confidence: inferred
  path: generated/worker-model/types.rs
- confidence: cited
  path: spec/README.md
- confidence: cited
  path: spec/domains/orchestration.yaml
- confidence: cited
  path: spec/scenarios/cli/codex-private-runtime.yaml
- confidence: cited
  path: spec/scenarios/cli/codex-start.yaml
- confidence: cited
  path: website/index.html
revision: 20
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T17:43:30Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-02T17:43:30Z", actor: "human:timo", revision: 3}
---
## Acceptance

Codex start and attach pass the existing production request builders without a Codex-only capture refusal, retaining agent credential separation, private auth storage, tmpfs diagnostics, volatile launcher replay, and existing confinement, as observed by updated CW capture scenarios and CLI preflight regression tests.

## Implementation

The operator explicitly removes the non-recording startup prerequisite on 2026-10-02. Remove the unsupported capture guard and requires_non_recording plumbing. Amend AGENTS.md and current ESS scenarios to require existing shared Substrate transport. Replace refusal tests with positive construction and next-real-preflight assertions, preserving Claude and credential/path defenses. Update current public docs to state capture behavior truthfully; historical reports stay historical. This unit proves local startup routing, not actual authenticated model or lifecycle acceptance. Substrate implementation and encrypted transport are excluded.

## Scope

Source scope is being confirmed by an independent read-only scoper before dispatch. Root owns planning records; docs are delegated separately with disjoint paths.
