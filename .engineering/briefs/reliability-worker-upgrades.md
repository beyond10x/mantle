# Unit 5: controlled offline worker upgrades

Read [shared invariants](reliability-invariants.md) and
[the story](../planning/story/controlled-worker-upgrades.md). Dispatch follows the green packaging
integration gate; the execution record supplies the exact base and package counts.

- Managed id and branch: `mantle-worker-upgrades`, `unit/mantle-worker-upgrades`.
- Checkout: `$HOME/.local/state/worktree/trees/b10x/mantle/mantle-worker-upgrades`.
- Implementor lease: `codex-worker-upgrades`.
- Target: `/dev/shm/mantle-worker-upgrades-target`.
- Temporary files: `/dev/shm/mantle-worker-upgrades-tmp`.
- Scratch: `$HOME/.cache/mantle-reliability/worker-upgrades`.

Implement `mantle worker upgrade --check|--apply` with a worker-local offline transaction and
shared artifact verification. Read the final unit 4 interface before designing transport.
Keep the manifest's source/digests and exact Substrate compatibility; same version alone does
not establish that the selected bundle is already installed. No Substrate migration or source
implementation dependency. The CLI transports verified inputs and reports typed observations.

Check is bounded and read-only, including for older workers without the new helper command.
Apply may use an explicitly verified temporary helper for first adoption; own and clean that
transport stage. Never provision, invoke agent credentials, or call worker up as an upgrade.
Do not touch a real worker during implementation or testing.

Both services must already be inactive and masked; queued jobs or a populated delegated cgroup
tree refuse. Validate the supported unit/cgroup layout and classify incomplete observations as
unknown. Use a host-wide lock and recheck immediately before commit. No stop, mask, unmask,
restart or session signals belong in this transaction. External administrative changes during
the exclusive maintenance window are outside the contract and must be documented plainly.

Preserve `/opt/mantle/bin` as a real directory and preserve the Codex relative link to
`../agents/codex/current/codex`. Stage a complete sibling directory on the same filesystem with
the three verified helpers, exact prior Claude bytes/mode/ownership, and supported existing
entries. Refuse unsupported entries. Use atomic `renameat2(RENAME_EXCHANGE)` for legacy adoption
and upgrades; unsupported semantics must fail without sequential fallback. Retain the exchanged
directory for rollback. Keep workspace volumes, configuration, secrets and agent generations.

Sync a journal containing directory identities and inventories before exchange. Recovery must
recognize an exchange that happened before its journal update. Under the same lock, inspect
actual directories, verify postconditions, and either complete or restore exact prior contents.
Do not report rollback unless it is observed. Success is `applied-restart-required`; services
remain masked and inactive. Keep interrupted transactions inspectable.

Serialize Codex installation and bundle exchange through one host-wide lock with a documented
lock order. Prevent legacy worker-up reconciliation from overwriting managed helper bundles or
using its shared `.new` names to bypass the transaction. Retain normal fresh provisioning behavior
where it is safe. Avoid moving the established process owner or duplicating artifact parsers.

Before runtime changes, update the existing UpgradeAssessment ESS value: its proposed
`active_execs` field predates the offline design and must not pretend an SDK inventory exists.
Represent the actual maintenance/process observations and outcomes. Add the named acceptance
scenarios and actual filesystem/process fixtures. Test an eligible success, every refusal with
no installed-byte mutation, concurrent installers, preserved Codex/Claude layout, interruption
around exchange/journal updates, and honest rollback failure. Document the maintenance steps
and limits as unreleased behavior. Return package/source evidence for separate adversary review.
