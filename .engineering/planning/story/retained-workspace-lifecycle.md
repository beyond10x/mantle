---
format: aep.planning-md/3
id: story:retained-workspace-lifecycle
kind: story
status: draft
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
revision: 22
---
## Outcome
Separate process lifetime from workspace destruction using the existing mantle.session.Session entity and Workspace SDK handle. Extend ESS states/commands before implementation. Proposed CLI: stop NAME ends the agent but preserves files and login state; destroy NAME --yes is explicit irreversible deletion; restart NAME starts a fresh agent execution in a retained workspace without cloning over files. Do not call a fresh process a same-conversation resume. Reattach alone never restarts. Document the intentional stop-semantics compatibility change prominently.

## Acceptance
Named scenarios retain-stop, retain-repeat-stop, retain-interrupted-stop, retain-files-auth, retain-restart, retain-restart-failure, retain-attach-refusal, destroy-explicit-confirmation, destroy-readback, destroy-repeat, retain-legacy-stopped and retain-other-session-survives. Kill/wait/retire descendants before marking retained; unknown remains incomplete and retryable. Explicit destroy has existing unknown-outcome read-back and no silent success. Existing STOPPED rows retain their historical destroyed meaning; no fabricated recoverability. Names remain reserved while workspace retained. Restart uses persisted selected agent/cwd/resources and existing private paths, never overwrites auth cache or user files; missing workspace is actionable, no implicit recreation. All local state migration and ESS native tests cover old/new schema. Controlled actual filesystem tests prove retained markers survive and deletion affects only selected workspace; live authenticated credentials need not be copied or inspected.

## Scope
Cited: domain/session.rs, app/session.rs, adapters/state.rs, Session ESS lifecycle, orchestration Stop command and native tests. Inferred: restart request context persistence/migration, CLI/help/docs and lifecycle cases. Serialized after earlier session/worker changes. No VM-loss recovery guarantee or agent-specific conversation-resume protocol.

## Migration and interrupted intent

Legacy STOPPED records represent destroyed workspaces and stay terminal. Legacy STOPPING records may already have a destructive operation in flight; a migration must preserve their destructive intent instead of silently reinterpreting them as the new retain-only stop. New retain-stop and destroy intent must be distinguishable durably across interruption/reopen. Observe absent workspaces honestly. Older rows lacking a validated restart context must not fabricate agent settings or silently reconstruct a different session from current config. Cover these migration/retry boundaries with real SQLite reopen tests and named conformance scenarios, alongside fresh-session stop/restart/destroy. Restart reuses verified persisted context and workspace, with a new exec id; source files and private agent home are retained, while a same-conversation resume is not promised.
