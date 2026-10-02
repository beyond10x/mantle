# Agent-ready worker acceptance ledger

Status: implementation correction/review in progress; no live acceptance claimed.
Owner: story:agent-ready-worker, epic:codex-parity. Wave page:
`.engineering/waves/2026-10-02-wave-3.md`.

## Claim to verify

With the exact same no-Claude configuration that the old required-Claude parser
refused, the treatment provisions an isolated KubeVirt worker, starts the unchanged
confinement stack without a Claude secret slot and reports common READY plus coherent
verified Codex0.153.4 installation facts. Repeating up observes AlreadyCurrent.

On the existing worker, the treatment adds Codex with freshly built artifacts while
preserving the exact running Claude exec, daemon/gateway process and start identities,
and the deployed shared executable hashes. Changed shared binaries are explicitly
deferred; they must not be reported as installed. No helper or agent install restarts
the shared services. This scenario is separate from authenticated Codex operation.

## Offline evidence retained so far

Initial implementor report is retained verbatim in
`codex-worker-initial-implementation-2026-10-02.md`, including its actual red runs,
gate output and scope confirmation. Its initial green48-test snapshot is historical;
the following adversary superseded its sufficiency.

Immutable `review-result:codex-worker-adversary-1` holds the full first attack:
7new cases,55top-level package executions,54passed/1failed in the full suite; the
scheduling-dependent directory race was also red in two selected runs. Two introduced
findings: cancellation leaves a bounded child group running, and concurrent first
directory creation can fail before the lock. Both are routed to the same implementor.
No fixed outcome or unit verification is asserted yet.

## Live baseline and fixtures

Read-only10:17UTC baseline is retained in `codex-worker-wave3-before.txt` and
`codex-worker-wave3-claude-before.json`. It is preparation, not the immediate
before/after measurement. The final run must refresh all identities immediately
before mutation and observe them afterward.

Fresh fixture: private no-Claude config under wave3 scratch-int/fresh-worker,
provider kubevirt, context k3d-mantle, isolated namespace mantle-wave3-acceptance,
Ubuntu serial20260926,4vCPU/8GiB memory/20GiB root/8GiB data. It uses explicit
MANTLE_CONFIG and MANTLE_STATE_DIR rather than changing HOME. Namespace was absent
in read-only preparation. Recheck disk, memory and cluster pressure before launch.

Existing fixture: a separate private no-Claude config selects the existing mantle
namespace and existing operator state/key directory. The original VM UID is
4b378e47-faf8-4c13-b248-5bba94af2c41 and runStrategy Always was observed11:13UTC.
This configuration avoids reading or replacing the existing Claude credential.
The daemon's existing slot must remain available.

## Pending evidence

- Correction report, final adversary, retained findings ledger and unit claim verdict.
- Actual fresh worker up/readiness/slot absence, pinned identity and idempotent rerun.
- Actual existing-worker addition with unchanged active identities and Claude exec.
- Integration gate: each step's raw output, exit and actual executed-case count.
- Bot source/merge commits, AEP evidence/move, local-main merge and managed archive/cleanup.

No live EC2 run, real Codex login, model request, refresh or lifecycle-parity result
is asserted by this worker ledger. Those remain required in the downstream stories.
