# Mantle executable contract

This specification describes the implemented slice at base commit
`11b69db7b86076299a9dd92cbbc2c0196148c007`, not the complete future system in the design document.
It is maintained with ESS 0.50.x (`ess-inputs.yaml`). The manifest enumerates authored inputs so
reports and generated files never become specification inputs.

## Boundaries and sources

| Domain | Authoritative source | Modeled behavior |
| --- | --- | --- |
| `mantle.session` | `crates/mantle/src/domain/session.rs:10`, `adapters/state.rs:30` | Seven recorded states, six transitions, insertion, workspace/exec updates, missing-record and wrong-state answers, internal record and live-session views |
| `mantle.manifest` | `crates/mantle/src/domain/manifest.rs:23` | Strict manifest shape, resolved values and resource bounds |
| `mantle.egress` | `crates/mantle-egress/src/allow.rs:6`, `proxy.rs:245` | The eight default host/port pairs after normalization, before DNS and connection establishment |
| `mantle.launch` | `crates/mantle-launch/src/cli.rs:29`, `ctl.rs:7` | Serve/attach arguments, bounded scrollback and terminal dimensions as values |

Paths abbreviated as `adapters/` are relative to `crates/mantle/src/`. Commands in the session
domain correspond to calls to `Store`; they are not new public CLI commands. `Materialized`, for
example, calls `move_session(id, Starting, None)` after materialization. The logical events name
local database writes. No component publishes them: Mantle has no event bus.

`SessionRecords` exposes the internal `session_by_id` read; the test adapter enumerates IDs to
check it. `LiveSessions` reflects `live_sessions`, excluding Stopped records. The full row in the
former is not a promise that the CLI prints every field. The worker relation identifies a retained
worker record; it does not assert that SQLite enforces a foreign key or that a provider is live.

## Corrections to the original seed

- The stored initial state is `Materializing`, not `Requested`. ESS uses CamelCase; the SQLite
  representation uses `MATERIALIZING`, `FAILED_AGENT_START`, and the other `Display` spellings.
  The eleven design states are future scope, not additional states silently accepted by the code.
- Workspace, agent exec and failure are nullable. A recorded `Running` state is historical intent;
  `status` reads Substrate separately and prints unknown or unreachable observations honestly.
- `WorkerRecord` stores name, instance, region and optional data volume. There is no persisted
  provider field, capability snapshot or provisioning lifecycle. `Recorded` is a structural ESS
  state only. `worker down` retains the record and storage.
- Source rows have composite identity `(session_id, mount)`. The old standalone `source_id` and
  inferred ownership cascade are removed. `SourceResolution` retains the actual value shape;
  no source or session records are deleted by `stop`.

The design describes future peer buses, scheduling, snapshots and shared caches. This contract
makes no claim that those features are implemented. `placement`, `network.profile` and
`lifecycle.idleAfter` are accepted manifest values but do not drive per-session scheduling or policy.

## Manifest semantics retained in code

The accepted API is `mantle.beyond10x.dev/v1alpha1`, kind `Session`, agent `claude-code`, runtime
`rust-dev`, root `/workspace`, and network default `deny`. Unknown fields are refused. A nonempty
source list uses HTTPS URLs, unique simple mounts, safe refs and a cwd within `/workspace` or a
listed mount. Defaults are CPU 4 (clamped to at least 1), memory 16 GiB, pids 4096, no storage
quota request and retention 8 hours (positive and at most 24 hours). Memory must be at least
512 MiB; pids 1–4096. Manifest bytes are SHA-256 hashed before normalization. The resolved
`retain_for_secs` field is the seconds projection of Rust `Duration` (see `app/session.rs:166`).
Network capabilities can only name `model.anthropic`, `git.github`, and `rust.crates`; a subset
does not narrow the gateway's fixed allowlist. CPU controls `CARGO_BUILD_JOBS`, not CPU bandwidth.
Requested resources and observed enforcement are distinct. `detachKeepsRunning: false` is refused.

## Verification and its limits

`task spec` validates all six authored ESS files. `task conformance` synthesizes the current suite
and executes all **57 scenarios, with zero skipped**, against production SQLite methods and
`Allowlist::default_list().permits()`. `task check` includes this through `cargo test --workspace`.
The Rust adapter does not evaluate guards from the specification. It maps actual returned errors
and reads stored rows after writes, including complete before/after snapshots on refused commands.
The allowlist command accepts already-normalized host and integer port values in the Rust u16
range; it is not the raw CONNECT interface.

The adapter pins the supported suite contract (`ess-conformance/22`), scenario count and exact
synthesis refusal set. New step/value/expectation forms fail closed. The report in
`.engineering/drafts/mantle-conformance-report.json` names every scenario and the model digest.
It is explicitly `mantle-local-conformance/1`, not an ESS report asserting whole-system conformance.
There is no Rust suite runner generated by ESS 0.50; this small adapter implements the exercised
IR subset, with no service model or lifecycle table copied from the specification.

Two synthesis refusals are deliberately visible rather than weakening the declared invariants:

- `ESS-SYNTH-013`: `mantle.launch.ServeArgs` has no scenario; no view publishes its scrollback bounds.
- `ESS-SYNTH-013`: `mantle.manifest.Resolved` has no scenario; no view publishes its resource bounds.

These obligations are **not** included in the 57 passing scenarios. Existing manifest, launcher,
proxy and adversary tests still run, but are not represented as generated ESS conformance.
The AEP specification may be `validated`; it must not be moved to `conforming` on this evidence.

## Unmapped semantics

Each marker also appears next to its source declaration:

| Marker | What would close it |
| --- | --- |
| `source-identity` | An ESS composite-key representation and source-write/query scenarios; verify actual deletion semantics before declaring ownership |
| `live-name-uniqueness` | Cross-record partial uniqueness scenarios for `sessions_live_name`; current SQLite tests cover reuse only after stop |
| `manifest-resolution` | Parser/default/hash/path-rule scenarios and an observable resolved-value boundary; preserve unsigned bounds and exact original input bytes |
| `egress-transport` | A target driving raw CONNECT, DNS/address safety, tunnel budgets, timing, forwarding and shutdown; retain the existing network adversary tests |
| `launcher-os` | A target observing real PTYs, FIFOs, locks, secrets, byte-preserving OS arguments, detach/replay, resize and signals |
| `orchestration` | Provider/Substrate adapter scenarios for worker upsert/start/stop, source writes, start/attach/exec, resource observations and interrupted cleanup |

A CLI stop already in `Stopping` resumes cleanup without calling `BeginStop` again. An absent
Substrate exec/workspace is tolerated during cleanup; an unknown destroy outcome is confirmed by
read-back before the local transition to Stopped. These are orchestration obligations, not
properties proved by the local lifecycle suite. No AWS account, Kubernetes cluster, secret or
network deployment was used to produce this specification.
