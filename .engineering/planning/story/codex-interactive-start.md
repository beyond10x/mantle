---
format: aep.planning-md/3
id: story:codex-interactive-start
kind: story
status: active
title: Start an authenticated Codex conversation in a confined workspace
relations:
- decomposes: epic:codex-parity
- depends_on: story:codex-confinement-qualification
- depends_on: story:agent-ready-worker
- depends_on: story:launcher-volatile-replay
- depends_on: story:codex-session-wiring
- depends_on: story:codex-gateway-destinations
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
revision: 40
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T18:17:04Z", actor: "human:timo", revision: 36}
- {from: "proposed", to: "active", at: "2026-10-02T18:17:04Z", actor: "human:timo", revision: 37}
---
## Context

After qualification and worker preparation, Mantle still refuses agent.kind=codex and always starts /opt/mantle/bin/claude with CLAUDE_CODE_OAUTH_TOKEN. This story owns one complete first-conversation path, including credential lifecycle and network policy.

## Acceptance

A valid Codex manifest that the baseline rejects starts a confined, subscription-authenticated interactive Codex conversation using the CS scenario set, without requiring Claude credentials.

## Named conformance scenarios

- CS-01: claude-code and codex resolve to distinct typed launch/auth descriptors; unknown kinds and incompatible auth modes fail before workspace creation. Existing Claude manifests and legacy state records retain their meaning; old optional spec fields resolve to Claude values. Session status and records name the selected agent.
- CS-02: first start creates only that session's private Codex home, device login is completed in the attached terminal, and one model turn reads the checked-out repository at its recorded commit. Start --detach gives an actionable attach/login instruction rather than claiming authenticated readiness.
- CS-03: Codex login-cache and refresh writes are confined to the session home, with safe paths, restrictive ownership/modes, and no cache overwrite on reattach; expired/revoked login returns to a useful authentication flow. Another workspace and the laptop state database cannot obtain the credentials.
- CS-04: fixed source-established auth.openai.com:443 and chatgpt.com:443 destinations are admitted through the gateway; CQ live observations establish the required HTTPS, WSS or supported fallback, refresh, Git and Cargo traffic and whether any additional exact destination is needed. No speculative wildcard/CDN expansion. Direct egress, metadata/reserved addresses and unlisted destinations remain denied.
- CS-05: no secret or device code appears in argv, Mantle DB/manifests, status/errors, evidence or persisted launcher replay/last-output; dummy credential canaries exercise each sink. Protect the interactive login transcript from recording. Explicit private Codex auth-cache files are the only planned persistent Codex credential exception.
- CS-06: start, login cancellation, denied domain, missing binary and early agent exit leave a truthful status and a cleanable workspace; retry does not create duplicate sessions.
- CS-07: Claude still uses only its own secret slot and token; a Codex session receives no Claude token, and a Claude session receives no Codex credential.
- CS-08: repository AGENTS.md is available and observed in a controlled prompt; repository hooks/configuration cannot redirect the generated auth path or relax the outer confinement. No import of the laptop's plugins/configuration.

## Implementation boundaries

Persist selected agent/auth identity, validate manifests, select runtime argv/readiness and add examples/codex.yaml. Device login and the main Codex TUI use the same private session home. Do not label API-key injection as interactive subscription support. The operator has paused encrypted transport. This story remains blocked pending a supported Substrate non-recording mode assessed in design:codex-terminal-confidentiality. Select the admitted mode through the SDK and prove actual Substrate, launcher and Codex diagnostic sink behavior with canaries before real login. Substrate support alone does not discharge the launcher or real-program obligations.

The implemented launcher and wiring use bounded live replay for the attached terminal and forbid persistent launcher last-output for Codex. No cache-existence or previous login success is treated as a reliable TUI authentication-phase signal. The parent's current capture requirement remains until the pending operator clarification is answered; neither local implementation nor planning review clears its blocker or proves full sink privacy.

Use credentials only inside the trusted session boundary; generated child commands can read the same user's files, as with today's Claude environment. Do not promise a broker or same-uid secret isolation. Keep all secrets out of non-secret state/errors. Amend AGENTS.md's credential rule explicitly for the private Codex auth-cache exception while preserving Claude's slot route. Generated Codex configuration, environment and home paths must prevent real Codex logging/config precedence from creating credential transcripts; restrictive file modes alone are insufficient.

Consume story:codex-gateway-destinations for the two fixed source-established endpoints; use CQ observations to justify any additional exact destination, with fail-closed matching and existing DoS/address protections. The gateway is worker-shared; document its combined allowlist's scope rather than claiming per-agent destination separation. Transport settings and any exposed model override are validated non-secret config, not arbitrary argv injection.

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

## Independent integration implementation

story:codex-session-wiring extracts the offline-checkable manifest, state migration, credential routing and real request-builder implementation. It follows launcher-volatile-replay. This parent depends on that result and owns its supported SDK activation, actual Codex home/config/diagnostic setup, chosen confinement profile, network and authenticated/live CS acceptance. The substory's mandatory unavailable-capability refusal is a temporary honest boundary, not completed Codex support. The user's instruction is to finish Mantle integration while the other session owns Substrate; do all local work while that capability develops.

## Pinned Codex diagnostic configuration findings

Pinned Codex 0.153.4 independently installs its SQLite diagnostic sink at TRACE; RUST_LOG=off suppresses direct-login and optional TUI text layers but is insufficient alone (tui/src/startup_orchestration.rs:528, state/src/log_db.rs:57). Token refresh failure can log a backend response body (login/src/auth/manager.rs:1612). These are source findings, not a live credential observation.

Existing Substrate 05695970 process.rs:1908–1937 supplies a fresh private /tmp tmpfs to each confined exec. Its memory.max and memory.swap.max=0 bound memory-backed files with the session; this is not a separate diagnostic quota. Configure pinned Codex sqlite_home=/tmp/mantle-codex/sqlite and log_dir=/tmp/mantle-codex/log, initialize those private directories inside the long-lived launcher exec, and verify tmpfs before dispatch. Both state and log SQLite move there; auth.json and intended conversation rollouts remain under the private workspace CODEX_HOME. Same-process detach/reattach preserves tmpfs; process-tree termination retires it. Restart/resume across a new exec is not established.

Supported TUI configuration includes forced_login_method=chatgpt, cli_auth_credentials_store=file, history.persistence=none, analytics.enabled=false, feedback.enabled=false and check_for_update_on_startup=false. TUI onboarding permits device code without an unconditional login preflight that clears existing auth. No supported TUI external-sandbox selector was found; retain the reviewed outer-confinement candidate with on-request approvals for qualification, never the bypass flag. These findings define the next local configuration/private-home implementation and its synthetic canary tests; they do not claim authenticated operation or waive effective configuration validation.

Source scoper performed read-only inspection only. No Substrate changes, credentials, login or model requests. The other session remains owner of substrate#112.

## Remaining gateway source wiring

Read-only story-scoper at79e1ee6 confirmed the existing worker route: worker::up/install_binaries installs and starts mantle-egress; its service binds127.0.0.1:3128 using compiled defaults; Substrate declares the named egress aperture; Mantle's agent request supplies that aperture and the launcher's fixed proxy. This is configured source behavior, not a fresh observation of live gateway health.

The default eight-host allowlist in crates/mantle-egress/src/allow.rs lacks auth.openai.com:443 and chatgpt.com:443. Pinned Codex0.153.4 source selects the former for device code/polling/token exchange/refresh and the latter for the built-in ChatGPT backend and subscription model endpoint. These two exact destinations are source-established local integration requirements, not an observed complete traffic inventory. No evidence warrants wildcard/CDN/API-key endpoint additions. Browser-side authorization dependencies do not establish worker egress requirements.

The smallest local source unit is two default entries plus the existing ESS CheckDefaultDestination guard and authored native cases through the existing Exchange fixture: both exact443 authorities accepted, opaque binary bytes retained, fully qualified resolution and pinned public dialing, wrong ports and lookalike/suffix authorities rejected before DNS/dial. Existing private/mixed-DNS, exact matching, limits and shutdown assertions stay intact. Scope: crates/mantle-egress/src/allow.rs; spec/domains/egress.yaml; inferred spec/scenarios/egress/codex-{default-destinations,connect-destinations,destination-refusals}.yaml; spec/ess-inputs.yaml; spec/conformance-baseline.json; spec/README.md; existing generated/worker-model provenance as actually required. Existing conformance.rs:42–142 already exposes the real handler with controlled DNS/dial; no new harness abstraction or deployment files are indicated.

Active gateway upgrades are deliberately deferred by worker.rs:329–342 to preserve shared sessions. Updating source therefore does not activate the two entries on an already-running gateway; controlled deployment and real TLS/login/model/refresh/network acceptance remain this parent's work. No worker or network policy was mutated during scoping. This is preparation for the next selected local unit, not an implemented or live-qualified claim.

Pinned source citations: https://github.com/openai/codex/blob/rust-v0.153.4/codex-rs/login/src/server.rs#L59 ; https://github.com/openai/codex/blob/rust-v0.153.4/codex-rs/login/src/device_code_auth.rs#L166 ; https://github.com/openai/codex/blob/rust-v0.153.4/codex-rs/model-provider-info/src/lib.rs#L290 .

## Local completion and pending activation after wave7

All independent source units are implemented: qualification, credential-independent workers, volatile launcher replay, Codex manifest/persisted identity/private runtime/start-attach wiring, bounded attachment cancellation and exact subscription gateway destinations. Latest full integration710dc32b6b565da6878a6c964265adcd3f17e686 passes185Rust tests and336native scenarios; full evidence lives in .engineering/reports/codex-gateway-wave7-2026-10-02. Those observations establish the local contracts, not usable authenticated Codex.

Production still returns the fixed unsupported-capture refusal before login because the pinned SDK cannot enforce the earlier accepted non-recording requirement. The previously asked operator question—use the existing Claude recording transport now, or retain the no-capture startup gate—remains unanswered. No permission is inferred from elapsed time. The latest read-only upstream check still found main6af1b91889edf5fa5455c03e68829b56e6b6cc56 and issue112 open without comments; this is not a claim about the other session's unpushed progress. No Substrate implementation was taken over.

The next substantive step requires either an explicit transport-policy answer or the supported SDK delivery, followed by real operator device authentication and the named CS/DP observations. Do not invent a live pass, silently remove the guard, reopen completed local units, or create more synthetic work merely to appear active. AWS remains paused. The unbudgeted goal is active and incomplete. Source commits are local; publication/release is not part of standing wave approval.

## Current transport and release decision

The operator explicitly removed the Substrate no-recording prerequisite and authorized source release on 2026-10-02. Codex uses the same existing Substrate terminal capture behavior as Claude; no end-to-end no-recording claim applies. This supersedes the earlier gate, pending clarification, SDK activation dependency and the Substrate-capture portion of CS-05. Private auth/home, tmpfs diagnostic placement, volatile launcher replay, credential separation and confinement remain required. story:codex-existing-transport implements the correction; actual operator device authentication and live CS/DP observations remain unproven. Substrate issue112 belongs to another session and no longer blocks this integration.

## Live acceptance after release0.1.0

Release0.1.0 is published atfd205aac9bbe440698c6863365c11fc8caca02d3 after187Rust/336native and exact GitHub Gate/security success. The capture blocker is cleared by explicit operator direction. Current read-only Mantle worker status observes the KubeVirt worker READY, installed Codex0.153.4 with expected binary/archive digests, Substrate0.7.8, and the preserved substrate-work Claude exec Expired with SIGKILL at2026-10-02T14:17:03.863210294Z. That observation is not permission to delete its workspace.

Proceed with bounded disposable live Codex acceptance using the released source, before claiming CS/DP completion. Inspect current worker binaries and sessions, build current static worker binaries, preserve existing workspaces, then select a safe isolated deployment/session. Never print auth files or device codes into evidence; operator device login happens in their own terminal. An asynchronous question asks operator availability for that login. AWS remains paused; only the configured local KubeVirt environment is in scope. Full parent acceptance requires actual model/tool/auth/lifecycle observations, not the already-passing local tests.

## Released integration and observed live startup

Mantle0.1.1 is published at95573ba9e73f6effd576ab9ffc8646db3ec6c5f4 (bot release402066028; exact candidate Gate37048284479 and common37048285313, tag common37048877606 all successful). The static worker portability correction is implemented and CI now builds that target. All187 top-level Rust tests and336 native ESS scenarios pass. Current source and public setup documentation explain the operator-approved existing Claude capture transport. Documentation index was verified byte-exact live atfd205aa; subsequent release publication is asynchronous.

At2026-10-02T18:35Z the idle KubeVirt worker received the verified0.1.1 static egress/launcher/worker binaries after checking there were no active execution cgroups. The existing expired Claude workspace was preserved. Worker readiness reports pinned Codex0.153.4 and unchanged Substrate0.7.8. Using a configuration without a Claude credential section, a real detached start created disposable codex-acceptance, session ses_01m3yyc60xy3c51rkpw7jrxvjh, workspace ws_01M3YYC62DG3BYPVGTSASHH9Z8 and agent exec ex_01M3YYCA86H8RPCQX3EQFFAY73. Source is substrate main@6af1b91889edf5fa5455c03e68829b56e6b6cc56. Status observes Running and lease ending2026-10-03T02:35:14.566867124Z; host process inventory confirms a real codex process in that execution cgroup. A supplementary mantle exec -- pwd returned/workspace with remote exit0.

This proves production startup and confined supplementary execution, not authentication or a model turn. The operator was given mantle attach codex-acceptance for device login. No login screen, code or credential contents were captured. Real authentication/refresh/model tools and authenticated lifecycle acceptance remain unverified. The local CLI was installed at~/.local/bin/mantle. Private raw runtime logs and status/exit observations remain under~/.cache/mantle-wave9/runtime; release evidence under~/.cache/mantle-wave9/release. Full acceptance remains active and incomplete; no upstream capture dependency is reintroduced.

## Current build observations

Supplementary execution needs Cargo's gateway explicitly configured: without it direct DNS is denied. After http.proxy is set to the existing fixed gateway, crates download successfully. The baseline GNU linker lookup fails because/usr/bin/cc points into unmounted/etc/alternatives. Setting target.x86_64-unknown-linux-gnu.linker=gcc makes the same real b10x-substrate-wire check pass under unchanged confinement. story:worker-rust-linker owns default selection for agent and supplementary requests. This causal probe is not an authenticated model/tool turn and does not close DP06 for either agent.

Metadata-only inspection of the actual Codex process root observed its private home as0700 owned900:900, both configured diagnostic filesystems astmpfs, and auth.json absent. No secret file contents or login screen was read. Real authentication therefore remains unavailable for acceptance until the operator completes device login. Documentation0.1.1 provenance is now verified live at95573ba9e73f6effd576ab9ffc8646db3ec6c5f4; main's repeated checks and documentation pipeline are also successful.

## Current acceptance audit

At source66e20bdf8e7252eed7ac21df377e08ddc6f14b98, the current requirement-by-requirement audit is.engineering/reports/codex-acceptance-audit-2026-10-02/audit.md. It accounts for everyCQ/AW/LP/CS/DP requirement and distinguishes offline, unauthenticated live and missing authenticated evidence. The prepared login target is codex-ready; its current process has the compiler correction. Old Codex/Claude workspaces remain preserved. Two-Codex file isolation and cleanup, OpenAI-host CONNECT/TLS, unlisted-host refusal and direct-egress refusal are observed; no real login/model/refresh/rendered lifecycle pass is claimed. Metadata-only auth-cache presence check returns remote1. Operator device login remains the required next external action. No new Substrate blocker or end-to-end no-recording prerequisite applies. The parent/epic remains incomplete.
