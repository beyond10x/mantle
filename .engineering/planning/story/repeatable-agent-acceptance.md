---
format: aep.planning-md/3
id: story:repeatable-agent-acceptance
kind: story
status: proposed
title: Run and record a bounded real-agent lifecycle acceptance suite
relations:
- decomposes: epic:reliability-and-usability
- depends_on: story:retained-workspace-lifecycle
scope:
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Cargo.toml
- confidence: inferred
  path: README.md
- confidence: inferred
  path: crates/mantle-acceptance
- confidence: inferred
  path: crates/mantle-conformance
- confidence: cited
  path: crates/mantle/src/main.rs
- confidence: cited
  path: crates/mantle/tests/codex_preflight.rs
- confidence: inferred
  path: docs/evidence
- confidence: inferred
  path: generated/worker-model
- confidence: inferred
  path: spec/components.yaml
- confidence: cited
  path: spec/domains/operator.yaml
- confidence: inferred
  path: spec/ess-inputs.yaml
- confidence: inferred
  path: spec/scenarios/cli
- confidence: inferred
  path: website/index.html
revision: 19
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T08:57:35Z", actor: "human:timo", revision: 19}
---
## Outcome
Provide a Rust/clap acceptance runner plus operator procedure for disposable Claude and Codex sessions through the installed Mantle CLI, not duplicated orchestration. AcceptanceResult is typed in spec/domains/operator.yaml. Bound duration/output/disk; keep only non-secret metadata, exact version/source identities and explicit case outcomes.

## Acceptance
Named scenarios acceptance-case-inventory, acceptance-real-command-path, acceptance-auth-required, acceptance-no-raw-transcript, acceptance-failure-exit, acceptance-resume-evidence, acceptance-owned-cleanup and acceptance-no-fabricated-pass. Cases cover create/login handoff, repository/model tool turn, detach/reconnect, resize/interrupt, transport loss, status/exec, retained stop/restart and explicit destroy for both agents. User completes real login/model steps in their terminal; runner records operator assertions distinctly from machine observations and never reads credentials/transcripts or treats cache presence as authentication. Missing manual action is operator-required/not-run and aggregate incomplete, never green. Run-id and names isolate disposable sessions; cleanup only exact resources it created, never user sessions. Controlled CLI fixtures prove progression, timeouts, interruptions and failure counts; an opt-in real mode uses the actual installed CLI. Existing full native conformance gate remains mandatory. Documentation gives a short repeatable procedure and clearly separates full live qualification from runner implementation.

## Scope
Inferred: Rust acceptance crate/CLI, Cargo workspace/lock, ESS operator commands/scenarios, fixtures, docs and CI checks. Uses selected named profile; no new auth/agent protocol, no automated credential entry or API billing fallback. Does not claim missing old-epic live cases passed.
