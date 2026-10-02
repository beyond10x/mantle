---
format: aep.planning-md/3
id: story:codex-private-terminal
kind: story
status: archived
title: Paused encrypted Mantle terminal candidate
relations:
- decomposes: epic:codex-parity
- depends_on: story:agent-ready-worker
- informed_by: coordination-blocker:codex-terminal-capture
scope:
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Cargo.toml
- confidence: cited
  path: crates/mantle-launch/Cargo.toml
- confidence: cited
  path: crates/mantle-launch/src/attach.rs
- confidence: cited
  path: crates/mantle-launch/src/cli.rs
- confidence: cited
  path: crates/mantle-launch/src/lib.rs
- confidence: cited
  path: crates/mantle-launch/src/serve.rs
- confidence: cited
  path: crates/mantle-launch/src/session.rs
- confidence: inferred
  path: crates/mantle-launch/src/terminal_protocol.rs
- confidence: cited
  path: crates/mantle-launch/tests/conformance.rs
- confidence: cited
  path: crates/mantle-launch/tests/support/probe.rs
- confidence: cited
  path: crates/mantle/Cargo.toml
- confidence: cited
  path: crates/mantle/src/adapters/conformance.rs
- confidence: cited
  path: crates/mantle/src/adapters/orchestration.rs
- confidence: cited
  path: crates/mantle/src/app/session.rs
- confidence: cited
  path: crates/mantle/src/app/terminal.rs
- confidence: inferred
  path: generated/terminal-model/Cargo.toml
- confidence: inferred
  path: generated/terminal-model/source.schema.json
- confidence: inferred
  path: generated/terminal-model/types-report.json
- confidence: inferred
  path: generated/terminal-model/types.rs
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
  path: spec/domains/orchestration.yaml
- confidence: cited
  path: spec/ess-inputs.yaml
- confidence: cited
  path: spec/scenarios/cli/orchestration-agent-request.yaml
- confidence: cited
  path: spec/scenarios/cli/orchestration-attach-request.yaml
- confidence: cited
  path: spec/scenarios/cli/orchestration-exec-request.yaml
- confidence: inferred
  path: spec/scenarios/cli/orchestration-private-attach-request.yaml
- confidence: cited
  path: spec/scenarios/launch/argument-byte-preservation.yaml
- confidence: cited
  path: spec/scenarios/launch/argument-defaults.yaml
- confidence: cited
  path: spec/scenarios/launch/attach-no-tty.yaml
- confidence: inferred
  path: spec/scenarios/launch/encrypted-attach-arguments.yaml
- confidence: inferred
  path: spec/scenarios/launch/encrypted-attachment-lifecycle.yaml
- confidence: inferred
  path: spec/scenarios/launch/encrypted-budget-end.yaml
- confidence: inferred
  path: spec/scenarios/launch/encrypted-input-controls.yaml
- confidence: inferred
  path: spec/scenarios/launch/encrypted-output-canary.yaml
- confidence: inferred
  path: spec/scenarios/launch/encrypted-record-validation.yaml
- confidence: inferred
  path: spec/scenarios/launch/encrypted-slow-reader.yaml
- confidence: inferred
  path: spec/scenarios/launch/volatile-replay-no-last-output.yaml
revision: 50
transitions:
- {from: "draft", to: "archived", at: "2026-10-02T13:33:21Z", actor: "human:timo", revision: 50, decided_on: {"recorded":{"test_result":1}}}
---
## Disposition

Operator correction on 2026-10-02 pauses the encrypted terminal approach. Archive this unimplemented draft from wave scheduling. Do not implement its codec, key management, ciphertext budgets or reconstructed terminal protocol.

## Reason and replacement direction

Substrate owns the output recording behavior. First assess a policy-controlled Substrate mode that streams without persistent terminal output while retaining metadata and independent resource bounds. The current assessment is design:codex-terminal-confidentiality. The terminal-capture blocker remains open for interactive-start; it is not cleared by archiving this candidate.

## Evidence and recovery

Worker provisioning is complete at b3372f054bc5bf84df31d3e59b508bba074593f6. No encrypted runtime implementation was written. Historical proposed scope remains in this archived artifact's frontmatter; it is not reserved implementation ownership. The prior full body, draft model patch and generated bindings are preserved in ~/.cache/mantle-interactive-probe/paused-encryption-20261002/. The external native scenario patch remains unapplied. .engineering/reports/codex-terminal-age-proof-2026-10-02.md is a size probe for the discarded candidate, not acceptance evidence for the new direction.
