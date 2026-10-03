# Unit 3: worker doctor

Read [shared invariants](reliability-invariants.md) and
[the story](../planning/story/worker-doctor.md). This file captures the dispatch already underway;
it does not claim to precede that dispatch. Source base: `4d5af8c00e38378080912a4d22e76d3159c5d5e0`.

- Managed id and branch: `mantle-worker-doctor`, `unit/mantle-worker-doctor`.
- Checkout: `$HOME/.local/state/worktree/trees/b10x/mantle/mantle-worker-doctor`.
- Implementor lease: `codex-worker-doctor`.
- Target: `/dev/shm/mantle-worker-doctor-target`.
- Temporary files: `/dev/shm/mantle-worker-doctor-tmp`.
- Scratch: `$HOME/.cache/mantle-reliability/worker-doctor`.
- Baseline: 55 Mantle package tests, 28 worker tests, 243 CLI conformance scenarios.

Implement an early read-only `worker doctor` path and structured JSON even when selection fails.
Configuration reads are bounded and reject nonregular files. SQLite opens read-only without
creation/migration; ordinary locking/SHM coordination is permitted and committed WAL data must
remain visible. No custom snapshot protocol is needed.

Run checks in order: configuration, state/placement, provider, strict SSH, services/socket,
installed and live compatibility, common facts. Suppress dependent probes after failure.
Return fixed safe diagnostics, never raw provider stderr or configuration excerpts. No auth claim.
Use existing bounded-process ownership and extend its live guard for the tunnel; a timeout wrapped
around the old blocking tunnel is insufficient. Retain short private socket paths and strict
known-host validation, with no host-key updates. Quote provider ProxyCommand literals safely.

Observe installed worker/launcher/egress versions, live Substrate driver version and negotiated
wire compatibility. Reuse the common facts checks. Test the actual CLI against controlled
provider/SSH/HTTP fixtures, including timeout cleanup, downstream suppression, WAL read-only
state, malformed inputs and version mismatches. No real provider or worker calls.

Record baseline, red run, implementation, package results, conformance and generated drift.
Hand the source to a separate adversary before integration. The coordinator performs the full gate.
