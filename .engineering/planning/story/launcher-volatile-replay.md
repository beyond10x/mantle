---
format: aep.planning-md/3
id: story:launcher-volatile-replay
kind: story
status: implemented
title: Keep launcher replay in memory without writing last-output
relations:
- decomposes: epic:codex-parity
- depends_on: story:agent-ready-worker
- informed_by: design:codex-terminal-confidentiality
scope:
- confidence: cited
  path: crates/mantle-launch/src/cli.rs
- confidence: cited
  path: crates/mantle-launch/src/serve.rs
- confidence: cited
  path: crates/mantle-launch/tests/conformance.rs
- confidence: cited
  path: crates/mantle-launch/tests/support/probe.rs
- confidence: cited
  path: generated/worker-model/Cargo.toml
- confidence: cited
  path: generated/worker-model/source.schema.json
- confidence: cited
  path: generated/worker-model/types-report.json
- confidence: cited
  path: generated/worker-model/types.rs
- confidence: cited
  path: spec/README.md
- confidence: cited
  path: spec/components.yaml
- confidence: cited
  path: spec/conformance-baseline.json
- confidence: cited
  path: spec/domains/launch.yaml
- confidence: cited
  path: spec/ess-inputs.yaml
- confidence: cited
  path: spec/scenarios/launch/argument-byte-preservation.yaml
- confidence: cited
  path: spec/scenarios/launch/argument-defaults.yaml
- confidence: inferred
  path: spec/scenarios/launch/persistent-replay-compatibility.yaml
- confidence: inferred
  path: spec/scenarios/launch/replay-policy-arguments.yaml
- confidence: inferred
  path: spec/scenarios/launch/volatile-replay-bounds.yaml
- confidence: inferred
  path: spec/scenarios/launch/volatile-replay-exit-paths.yaml
- confidence: inferred
  path: spec/scenarios/launch/volatile-replay-lifecycle.yaml
- confidence: inferred
  path: spec/scenarios/launch/volatile-replay-preflight.yaml
revision: 8
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T13:50:24Z", actor: "human:timo", revision: 4}
- {from: "proposed", to: "active", at: "2026-10-02T13:50:24Z", actor: "human:timo", revision: 5}
- {from: "active", to: "implemented", at: "2026-10-02T14:37:29Z", actor: "human:timo", revision: 8, decided_on: {"recorded":{"test_result":1,"review_outcome":1,"verification":1,"ess_conformance_coverage_v1":1}}}
---
## Context

The upstream non-recording capability is tracked in beyond10x/substrate#112. Independently, Mantle's launcher unconditionally persists its bounded replay tail on child exit at crates/mantle-launch/src/serve.rs:151-156. This story removes that local sink when explicitly requested while preserving live replay and existing defaults. It does not make a whole session confidential, clear the upstream blocker, or allow real Codex login.

Typed home precedes this story: spec/domains/launch.yaml, existing mantle.launch.ServeArgs gains Boolean volatile_replay, normalized false when the new --volatile-replay switch is absent. ESS validates the seven-domain input and all 218 prior authored scenarios after the two complete normalized ServeArgs expectations gain false. No new transport model, generated crate or framing protocol is needed.

## Acceptance

Starting from a launcher that always writes its final replay tail, --volatile-replay preserves bounded live relay/detach/reattach and exit behavior while never creating or updating last-output, refuses any preexisting last-output entry before child dispatch/readiness, and preserves the default persistent behavior, as observed by LP01–LP06 through real launcher processes and native ESS conformance.

## Named conformance scenarios

- LP01 replay-policy-arguments: omitted flag normalizes volatile_replay=false; explicit flag true; malformed flag values are refused by clap. Existing byte-preserving argv assertions remain intact.
- LP02 volatile-replay-lifecycle: synthetic output is observed at an attached terminal, replayed after detach, the same agent survives and resize works; last-output is absent during the run and after clean exit. The collector has finite byte and time bounds.
- LP03 volatile-replay-exit-paths: clean and nonzero child exit, handled server SIGTERM, spawn failure and forced server death leave no last-output; inspect launcher diagnostics for the synthetic canary, assert observed statuses and reap fixture processes. SIGKILL is not claimed to provide normal cleanup; the fixture must kill and reap the isolated child group explicitly.
- LP04 volatile-replay-preflight: regular file, live symlink, dangling symlink and directory at last-output cause refusal before a child dispatch marker or ctl readiness; all entries and symlink targets are unchanged. Other metadata inspection errors fail closed. No path is opened, followed or deleted to establish absence.
- LP05 volatile-replay-bounds: a coordinated finite flood exceeds the declared 1024-byte replay capacity while detached, stops at a known suffix and keeps the child alive; a new attachment observes only the bounded newest suffix before any redraw output. An unread attachment does not prevent handled server termination within the existing 10-second grace plus 1-second final-flush budget and a 3-second fixture allowance; no last-output appears. Collectors remain bounded and their overflow is a failure.
- LP06 persistent-replay-compatibility: default behavior retains the bounded tail in last-output with mode 0600; all prior 218 authored/313 complete scenarios remain obligations, including existing lock, file/link and terminal cases. Only the two normalized ServeArgs expectations gain the false field; do not remove assertions.

## Implementation boundary

Use clap derive on ServeArgs for --volatile-replay, default false. Keep the current PTY/FIFO attachment, bounded Ring, scrollback limits and pending-output behavior. Under the existing private directory/server lock, use symlink_metadata before Pipes::create and child spawn: only NotFound permits volatile startup. Keep this within the existing cleanup structure, preserving server-lock cleanup. Skip the sole final output write in volatile mode, including nonzero child exits and handled termination. No output-bearing diagnostics.

No attach policy marker, SDK change, Substrate code, encryption, keys, wire framing, reconnect loop or agent selection is in this story. A malicious process with the same UID mutating the private session directory is outside this boundary, as in the existing launcher contract. Substrate and Codex diagnostics can still record content; synthetic fixture output only.

Native ESS commands expose actual observations (bytes/counts, paths present, statuses, dispatch/readiness markers and unchanged targets), not a blanket privacy boolean. The implementor must draft and validate the named command/scenario contracts before runtime changes. Extend only the Rust/clap fixture. Regenerate existing worker-model artifacts with ESS 0.50.0 for whole-spec provenance; never hand-edit generated output.

## Scope

Read-only story-scoper source review on 2026-10-02 identified these exact paths. The coordinator selected one Boolean switch rather than the scoper's two-value enum to avoid an unnecessary second type for a launcher-local on/off policy. Confidence high for existing seams, inferred for new scenario files. Source proof only; runtime acceptance is not yet observed.

Cited: spec/domains/launch.yaml; crates/mantle-launch/src/cli.rs; crates/mantle-launch/src/serve.rs; crates/mantle-launch/tests/conformance.rs; crates/mantle-launch/tests/support/probe.rs; spec/scenarios/launch/argument-defaults.yaml; spec/scenarios/launch/argument-byte-preservation.yaml; spec/conformance-baseline.json; spec/README.md; generated/worker-model/Cargo.toml; generated/worker-model/types.rs; generated/worker-model/source.schema.json; generated/worker-model/types-report.json.

Inferred new scenario files: spec/scenarios/launch/replay-policy-arguments.yaml; spec/scenarios/launch/volatile-replay-lifecycle.yaml; spec/scenarios/launch/volatile-replay-exit-paths.yaml; spec/scenarios/launch/volatile-replay-preflight.yaml; spec/scenarios/launch/volatile-replay-bounds.yaml; spec/scenarios/launch/persistent-replay-compatibility.yaml.

Collides with interactive-start and dual-agent-session-parity on launcher/ESS/generated provenance; run this story first, then interactive-start, then parity. It has no dependency on the upstream capture issue because its acceptance is limited to this local sink. Safety fact (source level 3, unproven at runtime): attach reads only live FIFO output and serve has one last-output write; no transport redesign is required.

## Scope correction before implementation

Implementation preparation found spec/ess-inputs.yaml explicitly enumerates authored scenario files and spec/components.yaml enumerates admitted native commands. Both are cited required scope, added before the implementor edits them; without those changes new tests could silently be excluded. Existing count218 is the baseline and must increase when six LP files are admitted. No acceptance changed.
