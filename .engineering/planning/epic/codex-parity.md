---
format: aep.planning-md/3
id: epic:codex-parity
kind: epic
status: active
title: Codex sessions with Claude-equivalent terminal behavior
relations:
- informed_by: epic:vertical-slice
- informed_by: executable-system-specification:mantle-session
revision: 23
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

- story:codex-confinement-qualification and story:agent-ready-worker are implemented; their recorded live/offline limits remain explicit.
- story:launcher-volatile-replay and story:codex-session-wiring are implemented on local main; the latter's wave5 gate passed179Rust/331native scenarios. Codex manifests, persisted identity, start/attach builders, private paths and tmpfs diagnostics are wired; supported capture admission remains an explicit startup refusal.
- story:attach-backpressure-cancellation is implemented by wave6 unitfd87c21/integrationde01750, with183Rust/333native full-gate passes and a measured old/new cancellation result. It supplies the local DP02/DP03 relay behavior without claiming authenticated conversation parity.
- story:codex-gateway-destinations is implemented at wave7 integration710dc32; its full gate passes185Rust tests and336native scenarios, including the two fixed source-established subscription hosts through the existing gateway. Live deployment and traffic sufficiency remain parent acceptance.
- The other session owns Substrate capture support tracked at https://github.com/beyond10x/substrate/issues/112. This effort does not implement Substrate or edit its store. coordination-blocker:codex-terminal-capture remains open while the operator's transport clarification is unanswered; elapsed time is not permission to remove the accepted privacy boundary.
- story:codex-interactive-start retains usable startup, supported SDK consumption where required, real login/model/tools, refresh/revocation and observed network behavior. story:dual-agent-session-parity retains real two-agent lifecycle, rendering, confinement and isolation evidence. Neither parent nor epic is complete.

The encrypted story:codex-private-terminal remains archived and excluded. Local units proceed independently where possible; shared source/provenance edits remain serialized. Standing wave approval covers local commits/merges; source publication and release remain outside it. The separate issue-filing instruction authorized Substrate112, already filed as the bot.

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

## Local completion and pending activation after wave7

All independent source units are implemented: qualification, credential-independent workers, volatile launcher replay, Codex manifest/persisted identity/private runtime/start-attach wiring, bounded attachment cancellation and exact subscription gateway destinations. Latest full integration710dc32b6b565da6878a6c964265adcd3f17e686 passes185Rust tests and336native scenarios; full evidence lives in .engineering/reports/codex-gateway-wave7-2026-10-02. Those observations establish the local contracts, not usable authenticated Codex.

Production still returns the fixed unsupported-capture refusal before login because the pinned SDK cannot enforce the earlier accepted non-recording requirement. The previously asked operator question—use the existing Claude recording transport now, or retain the no-capture startup gate—remains unanswered. No permission is inferred from elapsed time. The latest read-only upstream check still found main6af1b91889edf5fa5455c03e68829b56e6b6cc56 and issue112 open without comments; this is not a claim about the other session's unpushed progress. No Substrate implementation was taken over.

The next substantive step requires either an explicit transport-policy answer or the supported SDK delivery, followed by real operator device authentication and the named CS/DP observations. Do not invent a live pass, silently remove the guard, reopen completed local units, or create more synthetic work merely to appear active. AWS remains paused. The unbudgeted goal is active and incomplete. Source commits are local; publication/release is not part of standing wave approval.

## Current transport and release decision

On 2026-10-02 the operator explicitly removed the Codex-only non-recording capture prerequisite, authorizing the existing Claude Substrate terminal transport, and requested immediate source release with parallel documentation updates. This supersedes all earlier pending-answer and capture-blocked wording. coordination-blocker:codex-terminal-capture is cleared by the policy decision, not by an upstream delivery claim. story:codex-existing-transport owns code, ESS and documentation alignment. Preserve private home, temporary Codex diagnostics, launcher volatile replay and confinement. Source release can describe this implementation without claiming unobserved authenticated CS/DP acceptance; the full epic remains incomplete until those observations exist.

## Released integration and observed live startup

Mantle0.1.1 is published at95573ba9e73f6effd576ab9ffc8646db3ec6c5f4 (bot release402066028; exact candidate Gate37048284479 and common37048285313, tag common37048877606 all successful). The static worker portability correction is implemented and CI now builds that target. All187 top-level Rust tests and336 native ESS scenarios pass. Current source and public setup documentation explain the operator-approved existing Claude capture transport. Documentation index was verified byte-exact live atfd205aa; subsequent release publication is asynchronous.

At2026-10-02T18:35Z the idle KubeVirt worker received the verified0.1.1 static egress/launcher/worker binaries after checking there were no active execution cgroups. The existing expired Claude workspace was preserved. Worker readiness reports pinned Codex0.153.4 and unchanged Substrate0.7.8. Using a configuration without a Claude credential section, a real detached start created disposable codex-acceptance, session ses_01m3yyc60xy3c51rkpw7jrxvjh, workspace ws_01M3YYC62DG3BYPVGTSASHH9Z8 and agent exec ex_01M3YYCA86H8RPCQX3EQFFAY73. Source is substrate main@6af1b91889edf5fa5455c03e68829b56e6b6cc56. Status observes Running and lease ending2026-10-03T02:35:14.566867124Z; host process inventory confirms a real codex process in that execution cgroup. A supplementary mantle exec -- pwd returned/workspace with remote exit0.

This proves production startup and confined supplementary execution, not authentication or a model turn. The operator was given mantle attach codex-acceptance for device login. No login screen, code or credential contents were captured. Real authentication/refresh/model tools and authenticated lifecycle acceptance remain unverified. The local CLI was installed at~/.local/bin/mantle. Private raw runtime logs and status/exit observations remain under~/.cache/mantle-wave9/runtime; release evidence under~/.cache/mantle-wave9/release. Full acceptance remains active and incomplete; no upstream capture dependency is reintroduced.

## Current acceptance audit

At source66e20bdf8e7252eed7ac21df377e08ddc6f14b98, the current requirement-by-requirement audit is.engineering/reports/codex-acceptance-audit-2026-10-02/audit.md. It accounts for everyCQ/AW/LP/CS/DP requirement and distinguishes offline, unauthenticated live and missing authenticated evidence. The prepared login target is codex-ready; its current process has the compiler correction. Old Codex/Claude workspaces remain preserved. Two-Codex file isolation and cleanup, OpenAI-host CONNECT/TLS, unlisted-host refusal and direct-egress refusal are observed; no real login/model/refresh/rendered lifecycle pass is claimed. Metadata-only auth-cache presence check returns remote1. Operator device login remains the required next external action. No new Substrate blocker or end-to-end no-recording prerequisite applies. The parent/epic remains incomplete.

## Post-release acceptance follow-up

Source release0.1.3 and updated public docs are verified, with release evidence integrated by PR4 at1c225a824682574ffb9d451218da8c3b011ab388. Operator confirmed the corrected versioned CLI/profile attachment works. This is human attachment evidence, not an explicit statement that ChatGPT login and a repository/model turn completed; clarification is pending. The completed attach-startup-readiness story records real PTY detach/reconnect and published Substrate0.7.10 acceptance. Earlier codex-ready/wave9 is historical; current handoff is codex-read on the separate updated worker.

At2026-10-03T07:13Z further live checks found an access failure: worker status and strict SSH return exit255, no route to host. Kubernetes reports VMI Ready and launcher pod Running; these do not prove guest SSH or agent liveness. No restart, credential read, conversation inspection or destructive lifecycle probe was attempted. Authenticated CS and DP cases remain incomplete; source release is complete. Preserving the user's session remains required while diagnosing access.
