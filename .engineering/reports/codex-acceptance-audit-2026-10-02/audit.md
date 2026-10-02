# Codex parity acceptance audit

Current implementation subject:66e20bdf8e7252eed7ac21df377e08ddc6f14b98, verified remote main and clean primary. This is a follow-up to release0.1.1 at95573ba; the release tag is unchanged. Pinned Codex0.153.4 and Substrate0.7.8 are retained. No Substrate capture dependency is restored: the operator accepted the same terminal transport/capture behavior as Claude.

The four original stories are not all complete. Qualification and worker preparation have their delivered evidence; interactive start remains active and dual-agent lifecycle parity remains draft. This audit records partial observations and missing evidence. It is not a passing verdict for the epic or either unfinished parent.

## Current session and new observations

The prepared session is codex-ready, idses_01m3z04vyw154h0470x0a5zafs, workspacews_01M3Z04VZ8GN0NZWPBXNHMD1XQ, agentex_01M3Z04Y9BXGP4VKA5FAA26RS8. Source is substrate6af1b91889edf5fa5455c03e68829b56e6b6cc56. Status observes Running, lease ends2026-10-03T03:06:10.091413098Z. The old codex-acceptance session and expired Claude workspace were preserved. The new process receives the corrected GNU compiler defaults; the old process keeps its original environment.

A metadata-only test for the intended auth-cache path returned remote exit1; no auth-file contents or terminal login screen was read. The operator has been asked to complete device login through mantle attach codex-ready. No reply or successful authentication has been observed.

Each Codex workspace received one distinct empty marker. With both markers present, each workspace observed its own marker and neither the other marker nor the other workspace's host path. All four creation/check commands returned remote0. Both exact marker files were then removed, remote0. This is two-Codex file isolation evidence, not authenticated credential/refresh isolation or Claude/Codex matrix completion.

Through the fixed gateway, auth.openai.com and chatgpt.com each returned CONNECT200 and TLS_VERIFY0, then HTTP403 for unauthenticated root requests. Bodies were discarded. This establishes reachability and certificate verification only, not device/model endpoint acceptance or WSS. An unlisted example.com request returned CONNECT403/curl56. A direct no-proxy connection to1.1.1.1:443 returned curl7 without an HTTP response. No destination or confinement policy was changed.

The final printed Substrate ExecExit is the remote result. Mantle's current supplementary-exec client itself exits0 even for a nonzero remote command; that client status is not used as an assertion that a probe passed. Exact logs are adjacent to this audit, with a UTC observation file for this batch.

## Requirement audit

| Requirement | Evidence and outstanding requirement |
|---|---|
| CQ-01 | Official pinned binary/digest/version and mismatch refusal are recorded in docs/evidence/codex-compatibility.md and the qualification JSON. |
| CQ-02 | Real confined welcome/login screen and forbidden Unix-listener refusal observed in qualification; no authenticated screen claimed. |
| CQ-03 | Outer-confined direct shell/Cargo controls and negative host/socket/network probes observed. Wave10 adds a real native repository build. Model-issued tools and visible approval behavior still require authentication. |
| CQ-04 | Exact fixed gateway hosts, live CONNECT/TLS, unlisted-host refusal, direct-egress refusal and Cargo downloads observed. Device auth/model/refresh traffic and WSS or supported fallback remain unobserved. |
| CQ-05 | Real unauthenticated marker/byte observations and local PTY/slow-reader tests exist. Human-observed rendered authenticated reattach/resize remains missing. |
| CQ-06 | Real expiration/revocation/renewal not run; there is no authenticated cache to exercise. |
| AW-01 | Existing worker records include fresh no-Claude provision up0/READY and selected-Claude missing-config refusal. Wave9 also updated the configured worker using no Claude config. |
| AW-02 | Fresh/updated pinned install identity plus mismatch/architecture/partial-install tests recorded in worker evidence. |
| AW-03 | Existing-worker Installed then AlreadyCurrent preserved exact service identities and live Claude exec; worker live-preservation report records it. |
| AW-04 | Legacy configuration/selected readiness native cases and actual selected-Claude refusal on the fresh no-Claude worker recorded. |
| AW-05 | Prebuilt installer and shared provider rendering are covered; no new Substrate source build used. Live AWS is excluded by the operator's cost pause. |
| LP01–LP06 | Volatile launcher contracts and failure/canary tests recorded by wave4 and retained in current native gate. They do not imply end-to-end non-recording; that prerequisite was removed by the operator. |
| CS-01 | Native manifest/identity migration and request scenarios cover agent/auth selection and compatibility. Live status names Codex/ChatgptDevice. |
| CS-02 | Real detached startup/private home and actionable attach instruction observed. Operator device login and repository-reading model turn still missing. |
| CS-03 | Native safe-path/cache-preservation cases and live0700/tmpfs metadata exist. Auth-cache ownership after real login, refresh/revocation and authenticated cross-workspace isolation remain missing. |
| CS-04 | Fixed policy, native address/budget tests and live CONNECT/TLS/negative probes exist. Auth/model/refresh/WSS behavior not established by an unauthenticated root HTTP403. |
| CS-05 | Launcher/private-runtime synthetic canary tests and metadata exist. Real credential-bearing failure/refresh paths remain unobserved. Substrate terminal recording is explicitly permitted; no end-to-end non-recording claim. |
| CS-06 | Native start/failure/state/cleanup cases exist. Real login cancellation and authenticated retry/expiry observations remain missing. |
| CS-07 | Selected credential routing/native request tests and no-Claude production startup observed. Full authenticated two-agent credential separation remains unobserved. |
| CS-08 | Repository is materialized at the recorded commit and fixed config precedence has local real-Codex evidence. Model-observed AGENTS instructions and authenticated configuration behavior remain missing. |
| DP-01 | No authenticated conversation marker has been established. Same-conversation detach/reattach is not complete. |
| DP-02 | Local real-PTY cancellation/resize/backpressure cases and unauthenticated TUI probes exist. Authenticated child interruption and readable terminal behavior for both agents remain missing. |
| DP-03 | Offline relay/single-attach/tunnel behavior exists. Actual authenticated same-conversation tunnel-loss recovery and concurrent-attach rendering remain missing. |
| DP-04 | Actual Codex supplementary exec, cancelled agent, confirmed workspace destruction, honest repeat-stop refusal and survival of the other Codex session observed. Full Claude/Codex live matrix remains incomplete. |
| DP-05 | Actual two-Codex file namespace isolation observed. Claude/Codex and authenticated login/refresh isolation plus live descendant-kill matrix remain incomplete. |
| DP-06 | Real native host-crate cargo check passed under confinement at the exact source, workerCPU61.05s versus laptop0.13s. It was a supplementary command; model-issued builds for each agent remain unobserved. |
| DP-07 | Existing Claude lease expiry was truthfully observed, and native status handling covers terminal/unknown outcomes. Full live death/unknown/attach-no-silent-resume matrix remains incomplete. |
| DP-08 | Current gate executes336 native scenarios with zero skipped/unsupported/outside/refused and187 top-level Rust tests. Those counters do not replace missing live cases or establish all CS/DP acceptance. |

## Evidence locations and next action

Existing detailed records: docs/evidence/codex-compatibility.md; .engineering/reports/codex-qualification-2026-10-02.json; codex-worker-fresh-up-2026-10-02.log; codex-worker-live-claude-preservation-2026-10-02.json; codex-launcher-wave4-2026-10-02.md; and the wave5 throughwave10 closure/native/gate directories under.engineering/reports. This batch adds only the adjacent production startup, file-isolation, TLS/negative-network and auth-cache-presence logs. It does not rerun or promote older observations.

Next required external action is operator device login in the prepared terminal. After that, record a repository/model tool turn, refresh/revocation, required network transports and the authenticated two-agent lifecycle matrix, including a person's rendered terminal observations. Do not copy laptop credentials, invent a model pass, treat cache presence as authentication proof, or start another synthetic implementation wave merely to avoid this missing evidence. The full epic remains active and incomplete.
