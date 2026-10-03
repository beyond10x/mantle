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
  path: crates/mantle/src/adapters/orchestration.rs
- confidence: cited
  path: crates/mantle/src/adapters/state.rs
- confidence: cited
  path: crates/mantle/src/app/session.rs
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
revision: 21
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T08:57:35Z", actor: "human:timo", revision: 19}
---
## Outcome
Provide a Rust/clap acceptance runner plus operator procedure for disposable Claude and Codex sessions through the installed Mantle CLI, not duplicated orchestration. AcceptanceResult is typed in spec/domains/operator.yaml. Bound duration/output/disk; keep only non-secret metadata, exact version/source identities and explicit case outcomes.

## Acceptance
Named scenarios acceptance-case-inventory, acceptance-real-command-path, acceptance-auth-required, acceptance-no-raw-transcript, acceptance-failure-exit, acceptance-resume-evidence, acceptance-owned-cleanup and acceptance-no-fabricated-pass. Cases cover create/login handoff, repository/model tool turn, detach/reconnect, resize/interrupt, transport loss, status/exec, retained stop/restart and explicit destroy for both agents. User completes real login/model steps in their terminal; runner records operator assertions distinctly from machine observations and never reads credentials/transcripts or treats cache presence as authentication. Missing manual action is operator-required/not-run and aggregate incomplete, never green. Run-id and names isolate disposable sessions; cleanup only exact resources it created, never user sessions. Controlled CLI fixtures prove progression, timeouts, interruptions and failure counts; an opt-in real mode uses the actual installed CLI. Existing full native conformance gate remains mandatory. Documentation gives a short repeatable procedure and clearly separates full live qualification from runner implementation.

## Scope
Inferred: Rust acceptance crate/CLI, Cargo workspace/lock, ESS operator commands/scenarios, fixtures, docs and CI checks. Uses selected named profile; no new auth/agent protocol, no automated credential entry or API billing fallback. Does not claim missing old-epic live cases passed.

## Metadata-only CLI observations

Read-only assessment found current human status fetches and prints terminal stderr (app/session.rs), so the runner must never capture or parse that path. Add versioned metadata-only JSON status and list output, including lookup of a recorded terminal session by exact session id. Status reports recorded versus observed states, session/workspace/agent-exec identity, exit/refusal/connectivity status, source commits and observation time; it must never request terminal output pages or serialize raw diagnostic payloads. Unavailable observations remain explicit and cannot produce a successful acceptance case. Authentication method is not an authenticated-state observation.

Reuse verified release manifest/install provenance where available; an unavailable source identity is unknown, never inferred from package version. Extend the existing AcceptanceResult ESS value with run/session/workspace identity, provenance and machine-versus-operator evidence origin before runtime implementation. The runner starts detached with a unique owned name, discards human startup output, records the resulting identity and uses the metadata path for subsequent checks.

Login/model turns run through an exact printed attach command in the operator's own terminal, outside runner pipes. Resume takes explicit operator attestations for login, model/tool action and visual terminal behavior; label these distinctly from machine observations. Missing attestations remain incomplete. Existing codex_qualification example supplies useful bounded scratch/digest patterns but its direct SDK path does not qualify installed Mantle CLI behavior. No raw terminal payload is retained.
