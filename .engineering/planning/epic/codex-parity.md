---
format: aep.planning-md/3
id: epic:codex-parity
kind: epic
status: active
title: Codex sessions with Claude-equivalent terminal behavior
relations:
- informed_by: epic:vertical-slice
- informed_by: executable-system-specification:mantle-session
revision: 16
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T08:54:04Z", actor: "human:timo", revision: 3, decided_on: {"recorded":{"review_outcome":1}}}
- {from: "proposed", to: "active", at: "2026-10-02T08:54:04Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"review_outcome":1}}}
---
# Codex sessions with the existing Mantle terminal experience

## Outcome

Support `agent.kind: codex` alongside `claude-code`: the agent and its development commands run inside the worker's Substrate workspace, while the laptop supplies a terminal through the existing Mantle commands. Start, attach, resize, detach, reconnect, status, exec and stop must retain their meaning for both agents.

## Baseline and evidence (2026-10-02)

Read at Mantle `11b69db`; this is a proposal, not delivered Codex support.
- `crates/mantle/src/domain/manifest.rs:173` refuses every kind except claude-code; `Resolved` currently retains cwd but not agent kind.
- `crates/mantle/src/app/session.rs:74,300` loads a Claude token and hard-codes the executable, slot and environment name.
- `crates/mantle-launch/src/serve.rs:160` already launches an arbitrary argv under a PTY and exports proxy variables; its secret path only supports an FD-to-environment string.
- `crates/mantle/src/adapters/substrate.rs:57` requires the Claude secret slot in readiness. `deploy/substrate.service:4,22` requires the token file and declares that slot. `crates/mantle/src/config.rs` requires a Claude configuration section.
- `deploy/cloud-init.yaml:103` installs only Claude; `crates/mantle-egress/src/allow.rs` admits Anthropic/Git/Rust destinations, not Codex's service destinations.
- `spec/domains/session.yaml` contains Session, Worker and SourceResolution, with placeholder single-state lifecycles and no commands or executable scenarios. The draft now adds optional agent/auth enum fields to Session before these stories. No new entity or guessed relation is introduced.
- `Taskfile.yml:check` validates ESS but does not execute an ESS conformance suite. Each implementation story must supply its named scenarios; a green existing gate is insufficient.
- Installed local CLI: `codex-cli 0.153.4`; inspected `codex --help` and `codex login --help`. The installed CLI has interactive mode, resume, device login and access-token/API-key login. This version is a qualification candidate, not a proven worker pin.
- Older local source at `~/.cache/codex-rust-v0.145.0/codex-rs/tui/src/lib.rs` has embedded and socket-backed app-server paths. It is only a reason to test current-version socket behavior, not proof that 0.153.4 works or fails.
- Claude reached a working initial attach; the corrected launcher was deployed at 2026-10-02 06:17 UTC. Live reattach verification is still absent. Do not use that absence to claim either agent already meets parity.

## Proposed decisions

1. Reuse the PTY/FIFO launcher and SSH/provider adapters. Codex TUI runs on the worker, not a local TUI connected to a new public app-server. Do not replace it with `codex exec`.
2. First authentication mode is ChatGPT subscription device login, subject to the operator's preference. Official docs describe headless device login, cached auth and automatic refresh; API billing is a separate choice. Never fall back silently to API billing. Enterprise access tokens are not assumed available.
3. Give each workspace a private, writable Codex home below its own `/workspace/.mantle/home`; use that HOME's default `.codex`, avoiding shared worker or laptop state. Device login happens through the user's attached terminal; do not pipe its code, auth cache or login transcript into evidence. Do not export the user's existing laptop credentials as part of planning.
4. Device login uses a private Codex credential cache (directory 0700, file 0600, restrictive umask), separate from Mantle's non-secret state DB and diagnostics. Token refresh writes stay within the same workspace; never restore an old credential snapshot over a refreshed one. No worker-global Codex credential. The auth implementation must verify the cache's actual modes and safe path handling. Revocation/expiry must lead to an actionable login flow, not repeated failing restarts.
5. Use a pinned, verified prebuilt Linux Codex binary. No compile on every new worker, no global npm dependency, no automatic version drift. Existing worker updates must not restart another agent merely to install Codex.
6. Prove the pinned CLI's app-server/IPC, child commands and network stack work under Substrate's AF_UNIX denial. Keep the outer confinement. Begin with supported Codex defaults; any proposed outer-sandbox-only profile must be explicit, justified by confinement tests, and retain approval prompts. No automatic bypass flags or relaxed Substrate policy.
7. Use the existing CONNECT gateway. Candidate hosts to investigate are `auth.openai.com:443` and `chatgpt.com:443`; `api.openai.com:443` is conditional on observed required traffic, not automatically needed for subscription use. Record the exact required hosts from the pinned version; no wildcard. Check HTTPS and WSS separately, including refresh and child-tool traffic. Disable unnecessary update/telemetry traffic where supported, rather than opening destinations speculatively.
8. Agent selection is immutable for a live session; detach/attach reconnects to the same process and conversation. Process death or the existing eight-hour maximum lease is not detach and must be reported honestly. Restart/resume across stop, VM loss or workspace destruction is excluded.

## Ownership and sequence

- story:codex-confinement-qualification and story:agent-ready-worker are implemented. Worker integration b3372f054bc5bf84df31d3e59b508bba074593f6 includes the ESS sweep, 167 Rust tests and 313 native scenarios. Wave 3 VM/worktree cleanup completed.
- Substrate capture support is an upstream dependency tracked in https://github.com/beyond10x/substrate/issues/112. This Mantle effort does not implement Substrate or edit its store. coordination-blocker:codex-terminal-capture remains open.
- story:launcher-volatile-replay is the independent next Mantle candidate: explicit bounded live replay without last-output. It covers only the launcher sink and does not clear the upstream blocker.
- story:codex-interactive-start follows launcher replay and supported upstream capability: exact SDK consumption, manifest/DB selection, safe home/config/diagnostics, real login/model/tool, refresh/revocation and observed network behavior. It remains blocked.
- story:dual-agent-session-parity follows interactive-start: real two-agent lifecycle, confinement and isolation evidence.

The encrypted story:codex-private-terminal is archived and excluded. Its ciphertext budget and manual-reattach proposal are not selected. Shared launcher, session, ESS and generated-provenance files require serialized implementation in the sequence above. Scope is rescoped before each wave. Standing approval covers local commits/merges; source publication and release remain outside it. The separate explicit issue-filing instruction authorized Substrate #112.

## Acceptance

The completed change passes every required CQ, AW, LP, CS and DP case in the two-agent matrix, with no mandatory live case replaced by a skip or reference-only result. LP01–LP06 isolate the launcher sink; their success cannot substitute for supported upstream capture behavior, Codex diagnostic privacy or real login/conversation. The upstream issue's sink and resource-bound evidence must be verified at the exact consumed revision. The archived encryption PT cases are excluded. Terminal privacy remains required across Substrate, launcher and Codex sinks, including failures.

## Completion and exclusions

A new Codex session can perform a real model turn and a remote Rust build, then retain its process and conversation through detach and transport interruption; a same-size reattach and resize render correctly; stop removes only that session. The same lifecycle matrix is demonstrated for Claude. Two simultaneous workspaces cannot read each other's files or credentials. Unknown execution state stays unknown. Evidence includes worker/laptop CPU measurements, exact versions/digests, endpoints, commands, timestamps and pass/fail/not-run results without credentials.

Local KubeVirt is the live acceptance target. AWS remains paused for cost approval; shared provider-independent behavior and EC2 rendering get offline coverage, with no new EC2 spend. Assess a Substrate capture capability before Mantle integration. No new cloud provider, generic agent plugin architecture, desktop/IDE integration, fleet management, auth broker, VM recovery, cloud tasks, arbitrary MCP/plugin/network parity, or migration of the laptop's entire Codex configuration. Repository AGENTS.md and ordinary Rust/Git tools are included. API-key mode is deferred unless the operator chooses it.

## Sources and confidence

- [OpenAI authentication](https://learn.chatgpt.com/docs/auth): supported login paths and cache behavior, read 2026-10-02.
- [OpenAI environment variables](https://learn.chatgpt.com/docs/config-file/environment-variables): Codex state location and distinction between interactive access tokens and non-interactive API-key variables.
- [OpenAI configuration reference](https://learn.chatgpt.com/docs/config-file/config-reference): authentication-store, centrally managed update and network settings.
- [OpenAI agent approvals and security](https://learn.chatgpt.com/docs/agent-approvals-security): approvals and sandbox policy.
- Local source citations above establish Mantle's gaps. Proxy compatibility, exact Codex domains, current socket behavior and live redraw are unverified; the qualification story owns those unknowns.

## Specification

`spec/domains/session.yaml` is the typed home. The optional fields preserve old-record interpretation as ClaudeCode/ClaudeOauth; `agent.kind` in a manifest is still explicit. Implementations must extend ESS commands/outcomes/views before changing behavior, synthesize rather than hand-transcribe the model, and run named conformance scenarios against the actual Rust implementation. Scenario names below are obligations to author, not tests that currently exist.

## Planning validation and review procedure

On 2026-10-02, `task check` exited 0 with 112 existing Rust tests plus fmt, clippy, ESS and AEP validation. `ess verify conform synthesize --path spec` produced 0 scenarios and 0 refusals: this validates the proposal's shape only and is not Codex conformance evidence. The implementations above own the missing executable contracts.

The four aep:plan-critic roles run read-only through generic agents following the packaged role procedures; this host has no native named-role dispatcher or Sonnet model. They use the inherited session model, with three concurrent worker slots, so the fourth review begins when a slot opens. Reviewers do not receive one another's findings. Results are immutable review-result artifacts; at most two rounds are run. The acceptance correction from round 1 adds the single matrix-level acceptance above; it does not change runtime scope.

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

## Current delivery goal and upstream ownership

Deliver Mantle Codex parity through reviewed AEP waves, preserving confinement and Claude-equivalent start, attach, resize, detach, reconnect, status, exec and stop. The other session owns Substrate issue112; do not implement a Mantle encryption workaround or take over Substrate implementation. Complete independent Mantle code and record actual authentication, model/tool and lifecycle evidence before claiming the full experience works. No token budget is imposed.

The existing goal service is now active and unbudgeted, verified2026-10-02. Its objective remains implementation of epic:codex-parity with qualification, worker preparation, authenticated sessions and lifecycle parity. The earlier paused-state/create_goal refusal is historical; the existing unfinished goal was not falsely completed or replaced. The accepted capture/privacy boundary remains in the owning stories while the operator clarification about the latest separation instruction is pending. Continue all independent implementation and review meanwhile.
