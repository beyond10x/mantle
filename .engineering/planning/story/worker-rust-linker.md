---
format: aep.planning-md/3
id: story:worker-rust-linker
kind: story
status: implemented
title: Select the available Rust linker inside confinement
relations:
- decomposes: story:codex-interactive-start
scope:
- confidence: cited
  path: crates/mantle/src/adapters/substrate.rs
- confidence: cited
  path: spec/scenarios/cli/codex-private-runtime.yaml
- confidence: cited
  path: spec/scenarios/cli/orchestration-agent-request.yaml
- confidence: cited
  path: spec/scenarios/cli/orchestration-exec-request.yaml
revision: 9
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T18:51:45Z", actor: "human:timo", revision: 6}
- {from: "proposed", to: "active", at: "2026-10-02T18:51:45Z", actor: "human:timo", revision: 7}
- {from: "active", to: "implemented", at: "2026-10-02T19:05:26Z", actor: "human:timo", revision: 9, decided_on: {"recorded":{"test_result":3,"review_outcome":1,"ess_conformance_coverage_v1":1}}}
---
## Acceptance

Existing named ESS scenarios orchestration-agent-request, codex-private-runtime and orchestration-exec-request require the x86_64 GNU Cargo linker to be gcc in the actual constructed environment. Both agents and supplementary commands inherit that selection. A real cargo check of b10x-substrate-wire at6af1b91889edf5fa5455c03e68829b56e6b6cc56 inside the existing Substrate confinement succeeds without a per-invocation linker override. Explicit gateway configuration is retained for standalone mantle exec, whose proxy variables are intentionally not admitted by Substrate; agent child commands already receive the launcher proxy.

## Reproduction and causal probe

Actual confined cargo check reached crates.io through the fixed gateway but exited101: linker cc not found. Host/proc-root metadata shows/usr/bin/cc points to/etc/alternatives/cc, absent inside the confined root, while/usr/bin/gcc resolves togcc-13 and executes there. Changing only Cargo target linker togcc makes the same build succeed: remote exit0, workeruser28.98s/system4.94s; laptopuser0.08s/system0.04s. This is a supplementary command, not an authenticated model-tool turn. Raw commands/timestamps/logs are retained in private live-acceptance scratch. No credential or terminal login output was read.

## Scope

Cited: crates/mantle/src/adapters/substrate.rs common environment; spec/scenarios/cli/orchestration-agent-request.yaml; spec/scenarios/cli/orchestration-exec-request.yaml; spec/scenarios/cli/codex-private-runtime.yaml. Existing Request/EnvironmentEntry model already types this output; no new noun or field. No Substrate changes, additional mounts, gateway destinations, global host symlink edits or privilege relaxation. No release version bump in this unit;0.1.1 remains immutable. Existing sessions retain their old environment; verify the changed CLI against a new disposable session.

## Delivery

Standing wave approval applies. Host subagent thread capacity was exhausted in wave9; root executes implementor then adversary roles in separate passes, without claiming independent-agent review. Exact source red/green native scenarios and final full gate are required before merge. Authentication and full parent parity stay incomplete.

## Native dependency compiler selection

The local adversary pass at9b83856 ran b10x-substrate-host after the wire-only crate passed and observed remote exit101 in libz-sys/cc-rs: compiler cc absent. Both cc and c++ use unmounted/etc/alternatives paths; gcc/g++ resolve entirely inside the existing/usr mount. Acceptance additionally requires CC=gcc and CXX=g++ in the same three named Request scenarios, and a real host-crate cargo check without per-command compiler/linker overrides. The source/fixture scope is unchanged. No mounts, Substrate policy or host alternatives are changed. This corrects the incomplete first unit before merge.
