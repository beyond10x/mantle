---
format: aep.planning-md/3
id: story:attach-startup-readiness
kind: story
status: active
title: Keep attachment replay behind client readiness
relations:
- informed_by: story:codex-interactive-start
scope:
- confidence: cited
  path: crates/mantle-launch/src/attach.rs
- confidence: cited
  path: crates/mantle-launch/src/cli.rs
- confidence: cited
  path: crates/mantle-launch/tests/conformance.rs
- confidence: cited
  path: crates/mantle/src/app/session.rs
- confidence: cited
  path: crates/mantle/src/app/terminal.rs
- confidence: cited
  path: spec/scenarios/cli/orchestration-attach-request.yaml
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T21:46:38Z", actor: "human:timo", revision: 3, decided_on: {"recorded":{"test_result":1}}}
- {from: "proposed", to: "active", at: "2026-10-02T21:46:38Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Problem
Real Codex attach fails with session.output-backpressure before device login. A continuously drained100x50 PTY reproduced it twice; metadata-only evidence is in .engineering/reports/attach-startup-2026-10-02/diagnosis.md. Agent/workspace survive but the terminal cannot be used.

## Acceptance
Named existing attach-request and CodexSessionWiring conformance must require the corrected production attachment path for both agents. Native real-PTY regression must prove no replay before client readiness, bounded missing/invalid readiness handling, signal and terminal-state cleanup, and exact forwarding of first real keyboard bytes. Live production attach must survive the original replay burst and detach/reconnect with the same agent exec. If readiness alone does not fix the live red, retain the red and diagnose the next hypothesis rather than claiming completion. No authentication claim from this test.

## Scope
Cited: crates/mantle/src/app/{session,terminal}.rs; crates/mantle-launch/src/{cli,attach}.rs; crates/mantle-launch/tests/conformance.rs; spec/scenarios/cli/orchestration-attach-request.yaml. Inferred additional focused Rust regression tests and existing Codex wiring expected request fixtures; specification changes precede implementation where required. No Substrate changes, queue-limit increase, credential copy, terminal capture feature or existing agent restart.

## Delivery
Standing operator approval covers this corrective wave. Root owns the AEP store and integration; implementor uses an isolated managed tree; independent adversary reviews the implementation before integration. Actual source release tags remain unchanged until a separately authorized release.
