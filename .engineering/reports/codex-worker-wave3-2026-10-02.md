# Agent-ready worker acceptance ledger

Status: worker story implemented. Both independent reviews, both live acceptance
claims and the final integration gate passed. Source is integrated locally; no publication.
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
The same implementor fixed both findings and retained their assertions. The immutable
second review reports no new findings; the explicit adversary comparison records two
resolved findings and zero carried/new findings in
`codex-worker-adversary-ledger-2026-10-02.json`. Its final snapshot passed 164 Rust tests
and all 313 native ESS scenarios after rebasing onto current main. These are offline
results; the later live tunnel correction needs its own validation.

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

## Live observations

The actual existing-worker CLI exited 0 with Installed, then exited 0 on its repeat
with AlreadyCurrent. Both runs reported common READY and explicitly left authentication
unknown. `existing-before-up.txt`, `existing-after-up.txt`, `existing-worker-up.log`
and `codex-worker-live-claude-preservation-2026-10-02.json` retain the observed unchanged
service process/start identities, deployed shared hashes and running original Claude
exec. The CLI explicitly deferred changed shared executables while installing Codex.

The fresh VM bootstrapped, rebooted and installed the verified candidate, then CLI up
exited 1 because its local Unix-socket address exceeded the platform limit. The exact
private state prefix is retained for the retry; no key relocation or VM recreation.
Read-only inspection afterward observed no Claude token file (tested as root), no
Claude slot in the running daemon command or machine facts, and exact Codex version
and both digests. An initial diagnostic used the wrong executable prefix and exited
127; the corrected `/opt/mantle/bin` inspection exited 0. This diagnostic correction
does not turn the failed CLI readiness into a pass.

Selected Claude start with the fresh no-Claude configuration separately exited 1
with the actionable missing-configuration message and created zero session rows.

## Final verification

The corrected CLI retried the exact fresh worker/configuration/state, exited 0 and
reported AlreadyCurrent plus common READY. The root-checked token absence, daemon
command without a Claude slot, exact version and both digests are retained in
`codex-worker-fresh-install-2026-10-02.txt`; machine facts and CLI output are adjacent
`codex-worker-fresh-machine-2026-10-02.json` and `codex-worker-fresh-up-2026-10-02.log`.
The narrow tunnel correction and its original red/unchanged green regression are in
`codex-worker-tunnel-correction-2026-10-02.md`. Coordinator diff/assertion review found
no weakened retained assertions; no third adversary was run. Final AW claim: VERIFIED.

Final unit dc71f10d32451f9ed57365685b702c4c5ee8c6a5 merged as
247ed1715ac24bba0c4188354d5aac3c02d6c26a, both author and committer verified bot.
The integration gate passed167 top-level Rust tests and313 native ESS scenarios;
format, clippy, ESS/AEP validation and documentation build each exited0. No gate step
was skipped. Every gate's own output and exit are under
`codex-worker-final-gate-2026-10-02/` (home prefixes normalized in text logs only).
The exact native suite/report/no-op audit are under `codex-worker-final-native-2026-10-02/`.
All304 incoming scenarios remain,9 added, and all218 authored scenarios reject the
no-op target. Six generated optional-witness passes remain explicitly disclosed.

The ESS artifact now records the current8a272638ba1e78a3a382335a397e223d44436d4b3423664c5c1c22ffe8f0f895
model and its exact report evidence. Story revision52 is implemented. AEP validation:
41 artifacts, valid. Closing evidence commit/local-main merge and managed retirement
are coordinator-owned final operations; exact paths and ids are in the wave page.

The temporary fresh-worker namespace, VM and PVCs were removed and absence observed;
the original operator worker remained Running. See `codex-worker-live-cleanup-2026-10-02.md`.
The host does not report per-agent token/tool/duration totals through its available
agent status interface, so those costs are unavailable rather than estimated.

No live EC2 run, real Codex login, model request, refresh or lifecycle-parity result
is asserted by this worker ledger. Those remain required in the downstream stories.
