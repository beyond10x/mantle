---
format: aep.planning-md/3
id: story:retained-workspace-lifecycle
kind: story
status: proposed
title: Stop agents while retaining workspaces and destroy only explicitly
relations:
- decomposes: epic:reliability-and-usability
- depends_on: story:controlled-worker-upgrades
scope:
- confidence: inferred
  path: README.md
- confidence: cited
  path: crates/mantle/src/adapters/orchestration.rs
- confidence: cited
  path: crates/mantle/src/adapters/state.rs
- confidence: cited
  path: crates/mantle/src/app/session.rs
- confidence: cited
  path: crates/mantle/src/domain/session.rs
- confidence: cited
  path: crates/mantle/src/main.rs
- confidence: inferred
  path: generated/session-model
- confidence: inferred
  path: generated/worker-model
- confidence: cited
  path: spec/domains/orchestration.yaml
- confidence: cited
  path: spec/domains/session.yaml
- confidence: inferred
  path: spec/ess-inputs.yaml
- confidence: inferred
  path: spec/scenarios/cli
- confidence: inferred
  path: website/index.html
revision: 26
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T08:57:35Z", actor: "human:timo", revision: 23}
---
## Outcome
Separate process lifetime from workspace destruction using the existing mantle.session.Session entity and Workspace SDK handle. Extend ESS states/commands before implementation. Proposed CLI: stop NAME ends the agent but preserves files and login state; destroy NAME --yes is explicit irreversible deletion; restart NAME starts a fresh agent execution in a retained workspace without cloning over files. Do not call a fresh process a same-conversation resume. Reattach alone never restarts. Document the intentional stop-semantics compatibility change prominently.

## Acceptance
Named scenarios retain-stop, retain-repeat-stop, retain-interrupted-stop, retain-files-auth, retain-restart, retain-restart-failure, retain-attach-refusal, destroy-explicit-confirmation, destroy-readback, destroy-repeat, retain-legacy-stopped and retain-other-session-survives. Kill/wait/retire descendants before marking retained; unknown remains incomplete and retryable. Explicit destroy has existing unknown-outcome read-back and no silent success. Existing STOPPED rows retain their historical destroyed meaning; no fabricated recoverability. Names remain reserved while workspace retained. Restart uses persisted selected agent/cwd/resources and existing private paths, never overwrites auth cache or user files; missing workspace is actionable, no implicit recreation. All local state migration and ESS native tests cover old/new schema. Controlled actual filesystem tests prove retained markers survive and deletion affects only selected workspace; live authenticated credentials need not be copied or inspected.

## Scope
Cited: domain/session.rs, app/session.rs, adapters/state.rs, Session ESS lifecycle, orchestration Stop command and native tests. Inferred: restart request context persistence/migration, CLI/help/docs and lifecycle cases. Serialized after earlier session/worker changes. No VM-loss recovery guarantee or agent-specific conversation-resume protocol.

## Migration and interrupted intent

Legacy STOPPED records represent destroyed workspaces and stay terminal. Legacy STOPPING records may already have a destructive operation in flight; a migration must preserve their destructive intent instead of silently reinterpreting them as the new retain-only stop. New retain-stop and destroy intent must be distinguishable durably across interruption/reopen. Observe absent workspaces honestly. Older rows lacking a validated restart context must not fabricate agent settings or silently reconstruct a different session from current config. Cover these migration/retry boundaries with real SQLite reopen tests and named conformance scenarios, alongside fresh-session stop/restart/destroy. Restart reuses verified persisted context and workspace, with a new exec id; source files and private agent home are retained, while a same-conversation resume is not promised.

## Restart admission and interrupted outcomes

Pinned SDK revision 65304edf already provides CommandBuilder::operation_id, Client::operation and Client::get_exec; no new Substrate capability is needed. Before submitting a restart, durably record its operation id, immutable restart request/context and selected worker/workspace binding. Persist the returned exec id before any readiness sleep or refresh (current start_agent waits three seconds before start_recorded can persist its id). Recovery queries the recorded operation first and reconciles its resource through get_exec, validating identity/workspace. An accepted or unresolved operation never triggers another start with a fresh id. Operation terminal means that the admission operation answered, not that the agent process exited.

Read-only SDK/source assessment by substrate_output_queue found exec.start stores its operation resource and provisional exec atomically before dispatch (Substrate daemon execs.rs and store execs.rs). Operation ids are retained within a deployment epoch; a changed deployment/subject/database can change lookup scope. Do not interpret a bare not-found after an unqualified reconnect as proof that no exec was admitted. If worker/deployment identity or recovery outcome cannot be established, leave restart incomplete with an actionable diagnostic rather than risking a duplicate. Add named interruption/reopen scenarios before submission, after accepted response and before exec-id persistence, plus unknown-outcome/no-duplicate observation. A normal eligible first restart still succeeds.

## Concurrent restart ownership

Concurrent restart callers must claim the same pending attempt atomically in SQLite rather than minting independent operation ids. On interrupted recovery, observe that attempt; do not rebuild and resubmit its request with a refreshed capability snapshot. The SDK's internal byte-identical transport retry differs from reconstructing a builder after reconnect. Test that two callers cannot start two agents for one retained session and that missing/conflicting operation observations leave the recorded attempt unresolved.

## Expected identity guards

Provide an optional expected-session-id guard on stop, restart and destroy so automation can refuse a reused name before any remote mutation. Resolve a recorded session once, compare the expected immutable id, and carry that record's workspace/exec ids through the operation. Test stale-id refusal with zero remote mutation calls and preservation of a newer session using the same name. The acceptance runner requires these guards for all cleanup/mutation commands; a run-name prefix alone is not ownership proof.
