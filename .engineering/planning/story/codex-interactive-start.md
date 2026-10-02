---
format: aep.planning-md/3
id: story:codex-interactive-start
kind: story
status: draft
title: Start an authenticated Codex conversation in a confined workspace
relations:
- decomposes: epic:codex-parity
- depends_on: story:codex-confinement-qualification
- depends_on: story:agent-ready-worker
- depends_on: story:launcher-volatile-replay
scope:
- confidence: cited
  path: AGENTS.md
- confidence: cited
  path: README.md
- confidence: cited
  path: Taskfile.yml
- confidence: inferred
  path: crates/mantle-egress/src/allow.rs
- confidence: inferred
  path: crates/mantle-launch/src/cli.rs
- confidence: inferred
  path: crates/mantle-launch/src/secret.rs
- confidence: inferred
  path: crates/mantle-launch/src/serve.rs
- confidence: inferred
  path: crates/mantle/src/adapters/state.rs
- confidence: inferred
  path: crates/mantle/src/adapters/substrate.rs
- confidence: inferred
  path: crates/mantle/src/app/session.rs
- confidence: inferred
  path: crates/mantle/src/app/worker.rs
- confidence: inferred
  path: crates/mantle/src/config.rs
- confidence: inferred
  path: crates/mantle/src/domain/manifest.rs
- confidence: inferred
  path: crates/mantle/src/domain/session.rs
- confidence: inferred
  path: crates/mantle/tests/codex_start.rs
- confidence: inferred
  path: deploy/substrate.service
- confidence: inferred
  path: examples/codex.yaml
- confidence: inferred
  path: examples/config.toml
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
- confidence: cited
  path: spec/domains/manifest.yaml
- confidence: cited
  path: spec/domains/orchestration.yaml
- confidence: inferred
  path: spec/domains/session.yaml
- confidence: inferred
  path: spec/scenarios/cli/codex-start.yaml
revision: 27
---
## Context

After qualification and worker preparation, Mantle still refuses agent.kind=codex and always starts /opt/mantle/bin/claude with CLAUDE_CODE_OAUTH_TOKEN. This story owns one complete first-conversation path, including credential lifecycle and network policy.

## Acceptance

A valid Codex manifest that the baseline rejects starts a confined, subscription-authenticated interactive Codex conversation using the CS scenario set, without requiring Claude credentials.

## Named conformance scenarios

- CS-01: claude-code and codex resolve to distinct typed launch/auth descriptors; unknown kinds and incompatible auth modes fail before workspace creation. Existing Claude manifests and legacy state records retain their meaning; old optional spec fields resolve to Claude values. Session status and records name the selected agent.
- CS-02: first start creates only that session's private Codex home, device login is completed in the attached terminal, and one model turn reads the checked-out repository at its recorded commit. Start --detach gives an actionable attach/login instruction rather than claiming authenticated readiness.
- CS-03: Codex login-cache and refresh writes are confined to the session home, with safe paths, restrictive ownership/modes, and no cache overwrite on reattach; expired/revoked login returns to a useful authentication flow. Another workspace and the laptop state database cannot obtain the credentials.
- CS-04: only the CQ-observed required hosts are admitted; HTTPS inference, required WSS traffic or a supported tested fallback, refresh, Git and Cargo all use the gateway. Direct egress, metadata/reserved addresses and unlisted destinations stay denied.
- CS-05: no secret or device code appears in argv, Mantle DB/manifests, status/errors, evidence or persisted launcher replay/last-output; dummy credential canaries exercise each sink. Protect the interactive login transcript from recording. Explicit private Codex auth-cache files are the only planned persistent Codex credential exception.
- CS-06: start, login cancellation, denied domain, missing binary and early agent exit leave a truthful status and a cleanable workspace; retry does not create duplicate sessions.
- CS-07: Claude still uses only its own secret slot and token; a Codex session receives no Claude token, and a Claude session receives no Codex credential.
- CS-08: repository AGENTS.md is available and observed in a controlled prompt; repository hooks/configuration cannot redirect the generated auth path or relax the outer confinement. No import of the laptop's plugins/configuration.

## Implementation boundaries

Persist selected agent/auth identity, validate manifests, select runtime argv/readiness and add examples/codex.yaml. Device login and the main Codex TUI use the same private session home. Do not label API-key injection as interactive subscription support. The operator has paused encrypted transport. This story remains blocked pending a supported Substrate non-recording mode assessed in design:codex-terminal-confidentiality. Select the admitted mode through the SDK and prove actual Substrate, launcher and Codex diagnostic sink behavior with canaries before real login. Substrate support alone does not discharge the launcher or real-program obligations.

This draft proposes replacing the earlier unsupported phase-suppression mechanism: bounded live replay is permitted for the authorized attached terminal, while every persistent plaintext transcript/diagnostic/state sink remains forbidden. Codex never writes launcher last-output; no cache-existence or previous login success is treated as a reliable TUI authentication-phase signal. This prose is a proposal for the forthcoming critic round, not approval or blocker clearance.

Use credentials only inside the trusted session boundary; generated child commands can read the same user's files, as with today's Claude environment. Do not promise a broker or same-uid secret isolation. Keep all secrets out of non-secret state/errors. Amend AGENTS.md's credential rule explicitly for the private Codex auth-cache exception while preserving Claude's slot route. Generated Codex configuration, environment and home paths must prevent real Codex logging/config precedence from creating credential transcripts; restrictive file modes alone are insufficient.

Add exact-host network policy from CQ, with fail-closed matching and existing DoS/address protections. The gateway is worker-shared; document its combined allowlist's scope rather than claiming per-agent destination separation. Transport settings and any exposed model override are validated non-secret config, not arbitrary argv injection.

Extend Session ESS commands/outcomes/views and authored CS scenarios before implementation; generate model bindings and run the real Rust adapter in task check. A mocked provider can exercise refusal/cleanup/canary paths, but the actual operator-completed device login, model/tool turn, refresh/revocation and required network observations are separate mandatory acceptance evidence. Missing operator participation can block live completion, never justify claiming success from offline tests.

## Scope

Exact files below are proposed ownership, with confidence retained from the current source review. New generated bindings and scenario paths are inferred until implementation confirms them. Shared launcher/session/ESS/generated provenance surfaces serialize interactive-start and lifecycle parity after the Substrate contract is reviewed. Future stories are rescoped again before selection; no broad directory reservation remains.

- cited: `AGENTS.md`
- cited: `README.md`
- cited: `Taskfile.yml`
- inferred: `crates/mantle-egress/src/allow.rs`
- inferred: `crates/mantle-launch/src/cli.rs`
- inferred: `crates/mantle-launch/src/secret.rs`
- inferred: `crates/mantle-launch/src/serve.rs`
- inferred: `crates/mantle/src/adapters/state.rs`
- inferred: `crates/mantle/src/adapters/substrate.rs`
- inferred: `crates/mantle/src/app/session.rs`
- inferred: `crates/mantle/src/app/worker.rs`
- inferred: `crates/mantle/src/config.rs`
- inferred: `crates/mantle/src/domain/manifest.rs`
- inferred: `crates/mantle/src/domain/session.rs`
- inferred: `crates/mantle/tests/codex_start.rs`
- inferred: `deploy/substrate.service`
- inferred: `examples/codex.yaml`
- inferred: `examples/config.toml`
- inferred: `generated/session-model/Cargo.toml`
- inferred: `generated/session-model/source.schema.json`
- inferred: `generated/session-model/types-report.json`
- inferred: `generated/session-model/types.rs`
- cited: `generated/worker-model/Cargo.toml`
- cited: `generated/worker-model/source.schema.json`
- cited: `generated/worker-model/types-report.json`
- cited: `generated/worker-model/types.rs`
- cited: `spec/domains/manifest.yaml`
- cited: `spec/domains/orchestration.yaml`
- inferred: `spec/domains/session.yaml`
- inferred: `spec/scenarios/cli/codex-start.yaml`

## Reconciliation with implemented ESS contracts

Integration commit `4bb052c` reconciles incoming published source `2e6b116` with the Codex plan.
The actual Session entity, constructor and views keep their implemented SQLite shape; proposed
AgentKind and AuthenticationMethod remain enum vocabulary only. The earlier provisional optional
fields are superseded. `story:codex-interactive-start` adds real agent/auth fields, DB migration,
constructor inputs, command assignments, both views and implementation adapter snapshots together.
Legacy Claude values are resolved by that migration, not invented by today's conformance adapter.

Qualification's completed harness/report may contain exact refusals and not-run cases. Missing
operator authentication blocks live acceptance and completion of interactive start, not construction
of the login plumbing needed to resolve it. A demonstrated confinement violation or mandatory
unavailable socket remains a design blocker. Final epic acceptance still requires the full named
CQ/AW/CS/DP behavior, including actual authentication, model tools, approval prompts and lifecycle
observations. The current direct outer-profile controls do not discharge those obligations.

Source: read-only story-scoper report against `2e6b116`; incoming conformance.rs complete-subject
snapshot checks; qualification live JSON and docs/evidence/codex-compatibility.md. This sequencing
clarification is the coordinator's inference and does not weaken final acceptance.

## Launcher prerequisite ownership

story:launcher-volatile-replay owns the launcher Boolean --volatile-replay policy and native LP01–LP06 proof. This interactive-start story consumes that policy for every Codex launcher invocation and owns safe production request construction, exact supported Substrate SDK pin, effective capture-mode verification/refusal, actual Codex diagnostics and all CS evidence. It must not reimplement the launcher primitive or infer end-to-end privacy from its local tests. Shared files are serialized by depends_on:story:launcher-volatile-replay; dual-agent-session-parity follows this story. Synthetic local preparation may precede upstream delivery, but real login and the story's completion cannot.
