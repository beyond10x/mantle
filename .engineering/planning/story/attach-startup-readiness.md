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
  path: Cargo.lock
- confidence: cited
  path: crates/mantle-launch/src/attach.rs
- confidence: cited
  path: crates/mantle-launch/src/cli.rs
- confidence: cited
  path: crates/mantle-launch/tests/conformance.rs
- confidence: cited
  path: crates/mantle/Cargo.toml
- confidence: cited
  path: crates/mantle/src/app/session.rs
- confidence: cited
  path: crates/mantle/src/app/terminal.rs
- confidence: cited
  path: crates/mantle/src/app/worker.rs
- confidence: cited
  path: spec/scenarios/cli/codex-start.yaml
- confidence: cited
  path: spec/scenarios/cli/orchestration-attach-request.yaml
revision: 9
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

## Upstream queue dependency

The 500ms delayed real-PTY candidate completed READY/ACK but still failed after 77,822 output bytes on worker0.7.8. Readiness remains partial. Substrate issue115 and its separately accepted story:live-output-stall own bounded asynchronous queue reservation. That cross-repository correction does not change Mantle transport bounds or implement the separate capture feature. Local candidate20c96f79 has a red-to-green host regression; independent review, full gates and combined transport proof remain pending. Installed SDK/runtime promotion requires a compatible, verified upstream release; no deployment claim is made here.

## Runtime adoption

Continue the original release intent after operator instruction to keep working. Both source corrections are merged (Mantle PR2, Substrate PR116). Adopt the verified Substrate0.7.10 release: pin SDK to its exact tagged source revision, worker daemon image and layer digests to its signed released artifacts. CurrentSDK0.16/daemon0.7.8 are incompatible with the new0.17 discovery bundle. Preserve queue limits and capture policy. Validate full Mantle gate and docs, publish a patch release, and prove real installed attach before reporting this story complete. A source release does not by itself authorize ending existing agent processes or destroying workspaces; prefer an isolated updated worker/runtime for acceptance while preserving them.

## Installed verification

The public installed-runtime-proof report records the published signed Substrate0.7.10 daemon, exact source/image/layer/binary identities and real Codex0.153.4 attachment with Mantle implementationfac3b79. Two100x50PTY runs drained329157 and346630bytes with exit0, normal detach, zero CLIstderr and the same observed Running agent. Original worker/sessions were preserved. Full integration gate and336nativeESS scenarios passed. Independent reviewer found no source or artifact-binding defect. Authentication and terminal-app visual rendering remain unverified. Patch release0.1.3 publication is the remaining story acceptance step.
