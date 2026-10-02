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
  path: generated/
- confidence: inferred
  path: spec/domains/session.yaml
- confidence: inferred
  path: spec/scenarios/codex-start.yaml
revision: 3
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

Persist selected agent/auth identity, validate manifests, select runtime argv and readiness, and add `examples/codex.yaml`. Keep the relay agent-neutral. Device login must run in the same private HOME as Codex; the current FD-to-env launcher hook is not an OAuth-cache adapter. Do not label CODEX_API_KEY injection as interactive subscription support. Login-screen suppression/redaction must prevent the generic scrollback and last-output facilities from retaining device codes or credentials; resume ordinary replay only after authentication.

Use credentials only inside the trusted session boundary; generated child commands can read the same user's files, as with today's Claude environment. Do not promise a broker or same-uid secret isolation. Keep all secrets out of the non-secret state database and errors. Amend AGENTS.md's credential rule explicitly to describe the Codex private auth-cache exception while preserving the Claude slot requirement.

Add exact-host network policy from CQ, with fail-closed matching and existing DoS/address protections. The gateway is worker-shared; document the combined allowlist's scope rather than claiming per-agent destination separation. Transport settings and a model override, if exposed, are validated non-secret config, not arbitrary argv injection.

Extend Session ESS commands/outcomes/views and authored CS scenarios before implementation; generate model bindings and run the actual Rust adapter in task check. A mocked provider can exercise refusal/cleanup and canary paths, but a successful real model turn is separately required for live acceptance.

## Scope

Cited: `AGENTS.md`, `crates/mantle/src/domain/manifest.rs`, `crates/mantle/src/domain/session.rs`, `crates/mantle/src/config.rs`, `crates/mantle/src/app/session.rs`, `crates/mantle/src/app/worker.rs`, `crates/mantle/src/adapters/state.rs`, `crates/mantle/src/adapters/substrate.rs`, `crates/mantle-launch/src/cli.rs`, `crates/mantle-launch/src/serve.rs`, `crates/mantle-launch/src/secret.rs`, `crates/mantle-egress/src/allow.rs`, `deploy/substrate.service`, `examples/config.toml`, `spec/domains/session.yaml`, `Taskfile.yml`, `README.md`.
Inferred new: `examples/codex.yaml`, `spec/scenarios/codex-start.yaml`, `crates/mantle/tests/codex_start.rs`, `generated/`.
Depends on qualification and worker readiness. The lifecycle story follows this one because its fixes and conformance cases share session.rs, serve.rs, the ESS domain and the gate.
