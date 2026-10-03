---
format: aep.planning-md/3
id: story:retained-workspace-lifecycle
kind: story
status: active
title: Stop agents while retaining workspaces and destroy only explicitly
relations:
- decomposes: epic:reliability-and-usability
- depends_on: story:controlled-worker-upgrades
scope:
- confidence: inferred
  path: README.md
- confidence: cited
  path: crates/mantle/src/adapters/conformance.rs
- confidence: cited
  path: crates/mantle/src/adapters/orchestration.rs
- confidence: cited
  path: crates/mantle/src/adapters/state.rs
- confidence: inferred
  path: crates/mantle/src/adapters/state/lifecycle.rs
- confidence: inferred
  path: crates/mantle/src/app/lifecycle.rs
- confidence: cited
  path: crates/mantle/src/app/session.rs
- confidence: cited
  path: crates/mantle/src/domain/session.rs
- confidence: cited
  path: crates/mantle/src/main.rs
- confidence: inferred
  path: crates/mantle/tests/retained_workspaces.rs
- confidence: inferred
  path: crates/mantle/tests/support/lifecycle_ssh.rs
- confidence: inferred
  path: generated/session-model
- confidence: inferred
  path: generated/worker-model
- confidence: inferred
  path: spec/components.yaml
- confidence: inferred
  path: spec/conformance-baseline.json
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
revision: 39
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T08:57:35Z", actor: "human:timo", revision: 23}
- {from: "proposed", to: "active", at: "2026-10-03T14:48:25Z", actor: "human:timo", revision: 30}
---
## Outcome
Separate process lifetime from workspace destruction using the existing mantle.session.Session entity and Workspace SDK handle. Extend ESS states/commands before implementation. Proposed CLI: stop NAME ends the agent but preserves files and login state; destroy NAME --yes is explicit irreversible deletion; restart NAME starts a fresh agent execution in a retained workspace without cloning over files. Do not call a fresh process a same-conversation resume. Reattach alone never restarts. Document the intentional stop-semantics compatibility change prominently.

## Acceptance
Named scenarios retain-stop, retain-repeat-stop, retain-interrupted-stop, retain-files-auth, retain-restart, retain-restart-failure, retain-attach-refusal, destroy-explicit-confirmation, destroy-readback, destroy-repeat, retain-legacy-stopped and retain-other-session-survives. Kill/wait/retire descendants before marking retained; unknown remains incomplete and retryable. Explicit destroy has existing unknown-outcome read-back and no silent success. Existing STOPPED rows retain their historical destroyed meaning; no fabricated recoverability. Names remain reserved while workspace retained. Restart uses persisted selected agent/cwd/resources and existing private paths, never overwrites auth cache or user files; missing workspace is actionable, no implicit recreation. All local state migration and ESS native tests cover old/new schema. Controlled actual filesystem tests prove retained markers survive and deletion affects only selected workspace; live authenticated credentials need not be copied or inspected.

## Scope

Cited: domain/session.rs, app/session.rs, adapters/state.rs, Session ESS lifecycle, orchestration Stop command and native tests. Inferred: restart request context persistence/migration, CLI/help/docs and lifecycle cases. Serialized after earlier session/worker changes. No VM-loss recovery guarantee or agent-specific conversation-resume protocol.

Implementation preparation adds inferred module boundaries crates/mantle/src/app/lifecycle.rs and crates/mantle/src/adapters/state/lifecycle.rs, with actual CLI fixtures in crates/mantle/tests/retained_workspaces.rs and crates/mantle/tests/support/lifecycle_ssh.rs. These separate lifecycle orchestration and atomic state claims from already substantial session.rs. Existing destructive Stop scenarios retain their purpose as explicit Destroy or legacy migration coverage; new Stop scenarios assert retention. No acceptance case is dropped by renaming the command.

Inferred addition during ESS validation: spec/components.yaml must admit the renamed destructive Destroy command and new lifecycle commands. The previous admission reference to orchestration.Stop cannot be left after its destructive contract becomes Destroy.

Inferred: spec/conformance-baseline.json must reconcile the required generated identity mantle.orchestration.Stop/outcome/returned to Destroy while preserving its destruction purpose, and retain the new lifecycle obligations. Root found the old required identity during aggregate-gate preparation; package-only checks do not read this baseline.

## Migration and interrupted intent

Legacy STOPPED records represent destroyed workspaces and stay terminal. Legacy STOPPING records may already have a destructive operation in flight; a migration must preserve their destructive intent instead of silently reinterpreting them as the new retain-only stop. New retain-stop and destroy intent must be distinguishable durably across interruption/reopen. Observe absent workspaces honestly. Older rows lacking a validated restart context must not fabricate agent settings or silently reconstruct a different session from current config. Cover these migration/retry boundaries with real SQLite reopen tests and named conformance scenarios, alongside fresh-session stop/restart/destroy. Restart reuses verified persisted context and workspace, with a new exec id; source files and private agent home are retained, while a same-conversation resume is not promised.

## Restart admission and interrupted outcomes

Pinned SDK revision 65304edf already provides CommandBuilder::operation_id, Client::operation and Client::get_exec; no new Substrate capability is needed. Before submitting a restart, durably record its operation id, immutable restart request/context and selected worker/workspace binding. Persist the returned exec id before any readiness sleep or refresh (current start_agent waits three seconds before start_recorded can persist its id). Recovery queries the recorded operation first and reconciles its resource through get_exec, validating identity/workspace. An accepted or unresolved operation never triggers another start with a fresh id. Operation terminal means that the admission operation answered, not that the agent process exited.

Read-only SDK/source assessment by substrate_output_queue found exec.start stores its operation resource and provisional exec atomically before dispatch (Substrate daemon execs.rs and store execs.rs). Operation ids are retained within a deployment epoch; a changed deployment/subject/database can change lookup scope. Do not interpret a bare not-found after an unqualified reconnect as proof that no exec was admitted. If worker/deployment identity or recovery outcome cannot be established, leave restart incomplete with an actionable diagnostic rather than risking a duplicate. Add named interruption/reopen scenarios before submission, after accepted response and before exec-id persistence, plus unknown-outcome/no-duplicate observation. A normal eligible first restart still succeeds.

## Concurrent restart ownership

Concurrent restart callers must claim the same pending attempt atomically in SQLite rather than minting independent operation ids. On interrupted recovery, observe that attempt; do not rebuild and resubmit its request with a refreshed capability snapshot. The SDK's internal byte-identical transport retry differs from reconstructing a builder after reconnect. Test that two callers cannot start two agents for one retained session and that missing/conflicting operation observations leave the recorded attempt unresolved.

## Expected identity guards

Provide an optional expected-session-id guard on stop, restart and destroy so automation can refuse a reused name before any remote mutation. Resolve a recorded session once, compare the expected immutable id, and carry that record's workspace/exec ids through the operation. Test stale-id refusal with zero remote mutation calls and preservation of a newer session using the same name. The acceptance runner requires these guards for all cleanup/mutation commands; a run-name prefix alone is not ownership proof.

## Conflicting lifecycle commands

Use atomic state/attempt claims for all lifecycle mutations, not only two restart callers. Stop or destroy must not act from a stale retained/running record while another caller admits a new exec for the same session id. A pending restart is reconciled before another destructive or retain transition proceeds; a pending stop/destroy prevents new restart admission. Re-read and compare the recorded generation/intent within the state transaction before remote work, without holding a SQLite transaction across network waits. Test stop-versus-restart and destroy-versus-restart using controlled interleavings and SQLite reopen; refusal or reconciliation must preserve the one recorded immutable attempt. Session-id guards protect name reuse, while durable intent protects concurrent operations on the same id. Keep interrupted initial starts honest too: no retained success when an admitted exec may be unrecorded and its termination cannot be observed.

## Durable completion and initialization ownership

Read-only preparation against c63d5a6 identified implementation details of the accepted concurrency/recovery contract. These are source observations, not reproduced findings or live qualification.

1. Persist binding-checked terminal observation and retirement operation ID before retire_with_operation_id, and verify the returned absent bool. Current app/session.rs:723 retires before its SQLite completion and treats resource.not-found as retired; a crash loses terminal proof. Missing exec without durable proof remains unresolved. SDK65304edf lib.rs:1816 exposes idempotent retirement; use it without importing internals.
2. Initial-start ownership covers materialization as well as agent admission. The row is visible at app/session.rs:106 before materialization at182; initialization at259 admits separate executions. A retain-stop cannot see agent_exec=None and succeed while the original caller may continue initialization/admission. Unproven interrupted initialization remains incomplete.
3. Fence every completion mutation with the expected attempt/generation and verify affected rows. Existing adapters/state.rs:228 move_session reads then updates by ID only; set_agent_exec and set_workspace are also unconditional. Use short BEGIN IMMEDIATE transactions for atomic claims and completions, never across remote IO. Add an explicit migration version so legacy STOPPING conversion cannot repeat against newly written intent on reopen.
4. Version the complete persisted launch contract: app/session.rs:842 rebuilding RunRequest and adapters/substrate.rs policy defaults can otherwise change the effective launch configuration. Persist effective settings or bind to an explicitly supported launch-policy version and refuse incompatible reconstruction.
5. Recover via Operation.resource followed by get_exec, checking operation ID/kind/resource and exec ID/workspace binding. Operation.result is wire JSON; do not deserialize it as SDK ExecObservation whose derived enum names differ (SDK model.rs:602 and :195).

Fixtures must cover interrupted retirement, initial materialization interleaving with stop, stale completion after a newer attempt, migration reopen and unsupported launch-policy versions, in addition to the already accepted restart-admission cases. No new Substrate capability or authenticated qualification requirement is introduced.
