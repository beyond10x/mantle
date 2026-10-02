---
format: aep.planning-md/3
id: story:dual-agent-session-parity
kind: story
status: draft
title: Verify and complete terminal-session parity for Codex and Claude
relations:
- decomposes: epic:codex-parity
- depends_on: story:codex-interactive-start
scope:
- confidence: inferred
  path: .engineering/planning/story/session-lifecycle.md
- confidence: inferred
  path: .engineering/planning/story/slice-evidence.md
- confidence: cited
  path: README.md
- confidence: cited
  path: Taskfile.yml
- confidence: inferred
  path: crates/mantle-launch/src/attach.rs
- confidence: inferred
  path: crates/mantle-launch/src/serve.rs
- confidence: inferred
  path: crates/mantle-launch/tests/agent_parity.rs
- confidence: inferred
  path: crates/mantle/src/app/session.rs
- confidence: inferred
  path: crates/mantle/src/app/terminal.rs
- confidence: inferred
  path: crates/mantle/tests/agent_parity.rs
- confidence: inferred
  path: docs/evidence/agent-parity.md
- confidence: inferred
  path: generated/session-model/Cargo.toml
- confidence: inferred
  path: generated/session-model/source.schema.json
- confidence: inferred
  path: generated/session-model/types-report.json
- confidence: inferred
  path: generated/session-model/types.rs
- confidence: cited
  path: generated/worker-model/Cargo.toml
- confidence: cited
  path: generated/worker-model/source.schema.json
- confidence: cited
  path: generated/worker-model/types-report.json
- confidence: cited
  path: generated/worker-model/types.rs
- confidence: inferred
  path: spec/domains/session.yaml
- confidence: inferred
  path: spec/scenarios/cli/agent-parity.yaml
revision: 22
---
## Context

An initial model response does not establish the previously intended Claude experience. The 200 ms redraw change has regression coverage but still lacks recorded live Claude reattach acceptance; Codex is a different TUI. This story proves and fixes lifecycle parity without claiming VM-loss recovery.

## Acceptance

For each supported agent, a live session that has completed a repository task retains its process and conversation across detach and reconnect, as established by the DP scenario matrix.

## Named conformance scenarios

- DP-01: after a distinct conversation marker and a repository command, Ctrl-] d returns to the laptop while the same agent exec remains running; a same-size attach returns the same conversation and a usable screen without a second login or agent process.
- DP-02: resize, Ctrl-C during a child command, slow terminal output and output overflow leave a responsive agent and readable terminal; the PTY restores the laptop terminal settings after interruption.
- DP-03: an interrupted SSH/tunnel connection does not end the detached agent; after connectivity returns, attach returns to the same exec/conversation. A second simultaneous attach is refused without corrupting the first.
- DP-04: status/exec/stop have the same contract for both agents; unknown stays unknown, stop handles slow destruction/read-back, repeat stop is honest, workspace and child processes are gone, and the other session survives.
- DP-05: two simultaneous workspaces (one Codex, one Claude, then two Codex sessions) cannot read each other's files/credentials; concurrent login/refresh state is not shared. The worker's cgroup kill covers descendants that leave a process group.
- DP-06: a real cargo check runs on the worker for each agent with timestamped worker and laptop CPU measurements, command exit status and exact source commit, while the fixed egress policy remains enforced.
- DP-07: the existing eight-hour maximum lease and process/VM death are reported as ended or unknown as observed; attach must not silently create a new conversation and call it a resume.
- DP-08: the deterministic conformance target runs all named AW/CS/DP offline scenarios, reports executed/skipped counts, and fails task check on a regression or unaccounted skip; live cases retain dated evidence and cannot be replaced by a reference-only or fake-agent pass.

## Evidence and existing work

Add a matrix with command, version/digest, date, before/after exec identity, observed screen outcome and pass/fail/not-run for every agent/scenario. Redact terminal captures and omit authentication screens. Account for every CQ observation and any remaining limitation. Update README examples with Ctrl-] d; correct the old session-lifecycle acceptance's stale Ctrl-b d through AEP at implementation time. Link results to the existing session-lifecycle and slice-evidence artifacts; do not bulk-close those stories or claim their broader acceptance automatically.

Preserve the user's current session. Use disposable sessions and bounded caches; check free space first. AWS remains paused. A tester's terminal result is required for real rendering; lack of access is an explicit missing record. Recovery beyond the lifetime of the workspace is excluded.

## Scope

Exact files below are proposed ownership, with confidence retained from the current source review. New generated bindings and scenario paths are inferred until implementation confirms them. Shared launcher/session/ESS/generated provenance surfaces serialize interactive-start and lifecycle parity after the Substrate contract is reviewed. Future stories are rescoped again before selection; no broad directory reservation remains.

- inferred: `.engineering/planning/story/session-lifecycle.md`
- inferred: `.engineering/planning/story/slice-evidence.md`
- cited: `README.md`
- cited: `Taskfile.yml`
- inferred: `crates/mantle-launch/src/attach.rs`
- inferred: `crates/mantle-launch/src/serve.rs`
- inferred: `crates/mantle-launch/tests/agent_parity.rs`
- inferred: `crates/mantle/src/app/session.rs`
- inferred: `crates/mantle/src/app/terminal.rs`
- inferred: `crates/mantle/tests/agent_parity.rs`
- inferred: `docs/evidence/agent-parity.md`
- inferred: `generated/session-model/Cargo.toml`
- inferred: `generated/session-model/source.schema.json`
- inferred: `generated/session-model/types-report.json`
- inferred: `generated/session-model/types.rs`
- cited: `generated/worker-model/Cargo.toml`
- cited: `generated/worker-model/source.schema.json`
- cited: `generated/worker-model/types-report.json`
- cited: `generated/worker-model/types.rs`
- inferred: `spec/domains/session.yaml`
- inferred: `spec/scenarios/cli/agent-parity.yaml`
