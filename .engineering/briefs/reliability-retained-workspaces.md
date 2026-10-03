# Unit 6: retained workspace lifecycle

Read [shared invariants](reliability-invariants.md) and
[the story](../planning/story/retained-workspace-lifecycle.md). Dispatch follows the green upgrade
integration gate; the execution record supplies the exact base and measured counts.

- Managed id and branch: `mantle-retained-workspaces`, `unit/mantle-retained-workspaces`.
- Checkout: `$HOME/.local/state/worktree/trees/b10x/mantle/mantle-retained-workspaces`.
- Implementor lease: `codex-retained-workspaces`.
- Target: `/dev/shm/mantle-retained-workspaces-target`.
- Temporary files: `/dev/shm/mantle-retained-workspaces-tmp`.
- Scratch: `$HOME/.cache/mantle-reliability/retained-workspaces`.

Update the existing Session ESS lifecycle before runtime changes. Stop retains the workspace,
destroy requires explicit `--yes`, and restart creates a fresh agent execution in the retained
workspace. Reattach never restarts. Do not promise same-conversation resume, worker-loss recovery,
or authenticated model behavior. Existing private agent homes and user files remain untouched.

Preserve historical intent in migration: old STOPPED means destroyed and is terminal; old
STOPPING may already be destroying and cannot become retain-only. New retain and destroy intents
are durable and distinct. Retained names stay reserved. A missing workspace is not recreated.
Rows lacking validated restart context cannot invent settings from today's configuration.
Exercise real SQLite migration and reopen, including source rows and indexes.

Persist validated immutable launch context and selected worker/workspace binding. Before restart
admission, atomically claim a single attempt and persist its operation id. The pinned SDK already
supports operation_id, operation lookup and get_exec. Persist a returned exec id before readiness
sleep/refresh. Reconcile the recorded operation and exec after interruption, verifying binding.
An accepted or unknown attempt must not trigger a fresh start id. An operation's terminal state
is admission completion, not proof the process exited. Missing observations after an unqualified
reconnect remain unresolved; do not assume the operation was never accepted.

Serialize conflicting stop/restart/destroy intents with atomic SQLite state/attempt claims, without
holding transactions over network waits. Two callers must not admit two agents, and a stale stop
must not act on a newly started execution of the same session. Cover interruption before dispatch,
after admission but before exec persistence, and around readiness. Keep initial-start uncertainty
honest too; retain success requires observed termination of any possibly admitted execution.

Add optional expected-session-id guards to stop, restart and destroy. Resolve once, check the
immutable id before remote mutation, and carry the resolved resource identities throughout.
Stale guards must produce zero remote mutation calls and preserve a newer same-name session.
The final acceptance runner relies on these guards for owned cleanup.

Use production orchestration with controlled IO plus actual local filesystem fixtures to prove
markers/private-home bytes survive stop/restart and only selected workspace data is destroyed.
Retain unknown-outcome destroy readback and explicit incomplete states. Test every named story
scenario and concurrent/interrupted operations with SQLite reopen. No real credentials or workers.

Document the intentional stop compatibility change prominently in README and website, including
command tables and troubleshooting. Clearly label historical 0.1.4 destructive-stop guidance so
it cannot be mistaken for unreleased semantics. Return source/package evidence for separate review.

Read the story's Durable completion and initialization ownership section. Persist terminal proof
and retirement identity before retiring; missing execution alone is not proof. Initial ownership
covers materialization executions. Fence completion writes as well as claims, version migrations
and the launch-policy contract, and recover through Operation.resource plus get_exec. Preserve
unknown outcomes rather than inventing a recoverable idle workspace.
