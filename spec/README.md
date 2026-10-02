# Mantle executable contract

Five ESS domains describe Mantle's implemented behavior: sessions, manifest resolution, egress,
launcher OS behavior, and application orchestration. The specification uses ESS 0.50.x, `ess/17`
and authored `ess-scenario/4` documents. `ess-inputs.yaml` explicitly selects every model and
scenario input. Design proposals for scheduling, snapshots, peer buses and shared caches remain
future scope.

## Executable boundaries

| Domain | Production boundary | Evidence |
| --- | --- | --- |
| `mantle.session` | `crates/mantle/src/adapters/state.rs` | Real SQLite inserts, transitions and reads; full session rows, source composite keys, partial live-name uniqueness in all seven states |
| `mantle.manifest` | `crates/mantle/src/domain/manifest.rs` | Actual YAML parser and validators: strict fields, unsigned bounds, defaults, paths, refs, unique mounts and original-byte SHA-256 |
| `mantle.egress` | `crates/mantle-egress/src/{allow,head,addr,proxy}.rs` | Real TCP CONNECT requests, binary tunnel traffic, DNS/address filtering, deadlines, connection budgets and shutdown |
| `mantle.launch` | `crates/mantle-launch/src/` | Real subprocesses, PTYs, FIFOs, locks and descriptor ownership; OS argument bytes, detach/replay, resize, process-group signals, bounded slow-reader output and filesystem safety |
| `mantle.orchestration` | `crates/mantle/src/app/{session,worker}.rs`, `adapters/substrate.rs` | Production start/materialization/stop and provider flows over controlled IO ports; exact agent/attach/exec request construction, capability refusal and usage presentation |

The test interfaces substitute external IO, not Mantle's decisions. DNS fixtures return addresses;
the production proxy filters and dials them. Provider fixtures return typed observations and record
calls; the production application orders those calls and writes SQLite. A stop with an unknown
destroy outcome must read back absence before recording Stopped. The five-minute deadline runs
under Tokio's virtual clock. These tests do not provision AWS or KubeVirt, run a Substrate daemon,
or attest to a live deployment's capabilities. Those upstream systems retain their own contracts.

The SDK is pinned to Substrate `05695970b069f79e6678f2f02cbd78bbe5fa2a56` (0.7.8).
Its producer declarations in `crates/substrate-wire/src/lib.rs` define `CapabilityFacts`,
`ExecUsage`, `ResourceUsage` and `ErrorDetail`; `crates/substrate-daemon/src/app/responses.rs`
defines unknown-outcome refusals. Observation fixtures retain metadata and metrics that Mantle
ignores. They use the owner's JSON vocabulary instead of declaring a competing local wire model.

## Record semantics

- Sessions start at Materializing. The seven recorded states and six transitions describe local
  intent; Running does not assert that an agent is currently alive. Status reports Substrate's
  observation, including unavailable or absent usage, separately.
- Session workspace, agent exec and failure fields are nullable. `SessionRecords` observes the
  internal `session_by_id` read; `LiveSessions` excludes Stopped. Logical ESS events describe
  database writes. Mantle has no event bus.
- Source identity is the actual `(session_id, mount)` key, represented by `SourceKey` and native
  `PutSource`/`ReadSources` boundaries. Inserts do not upsert: a duplicate key is refused and its
  original row survives. Source rows require an existing session. Stop deletes neither sources
  nor session records; there is no invented source identifier or ownership cascade.
- A live session name is unique across records in every state except Stopped. Stopping still owns
  the name. The worker relation identifies a retained record; SQLite does not enforce that relation.
- Worker records are upserted by name and contain instance, region/location and optional data
  volume. Recorded is a structural ESS state, not provider liveness. Worker down retains storage.

## Manifest and confinement

The API is `mantle.beyond10x.dev/v1alpha1`, kind Session, agent `claude-code`, runtime `rust-dev`,
root `/workspace`, and network default deny. Unknown fields are refused. Sources use HTTPS URLs,
unique simple mounts and safe refs; cwd stays within `/workspace` or a listed mount. Defaults are
CPU 4 (clamped to at least 1), memory 16 GiB, pids 4096, no storage quota request and retention
8 hours. Memory must be at least 512 MiB, pids 1–4096 and retention positive through 24 hours.
Hashing uses the original manifest bytes. CPU controls `CARGO_BUILD_JOBS`, not CPU bandwidth.

Network capabilities can name `model.anthropic`, `git.github` and `rust.crates`; a subset does not
narrow the gateway's fixed allowlist. `placement`, `network.profile` and `lifecycle.idleAfter` are
accepted values that do not drive per-session scheduling or policy. `detachKeepsRunning: false`
is refused. Agent requests alone carry the `claude` secret slot on descriptor 3. Attach and exec
requests carry no credential; all use the intended toolchain root and their explicit resource bounds.

The proxy admits ten exact normalized host/port pairs. Its worker-shared default list includes
`auth.openai.com:443` and `chatgpt.com:443` alongside the eight existing destinations; it does not
grant API-key endpoints, wildcard hosts or other ports. It discards unsafe resolved addresses and
pins dialing to an admitted public address. A mixed DNS answer may connect to its public member;
it does not make the private member dialable. Permitting a name is not evidence of a tunnel.
Shutdown owns and joins its connection tasks, aborting remaining tunnels after the drain deadline.

## Gate and evidence

`task conformance` synthesizes and executes **336 scenarios: 105 generated and 231 authored**.
The component counts are CLI 218, egress 69 and launcher 49. The complete inventory requires zero
failed, skipped, unsupported, outside or refused scenarios. `task check` also runs the
existing unit/integration/adversary tests, clippy, formatting, ESS validation, AEP validation and
website generation.

GW01–GW04 are exercised by `codex-default-destinations`, `codex-connect-destinations` and
`codex-destination-refusals`, together with every retained gateway defense scenario. The existing
real CONNECT fixture observes fully qualified resolver names, pinned public dial addresses and
unchanged binary payloads for both new hosts, including mixed DNS and normalized authorities.
Wrong ports, suffixes, subdomains, lookalikes, `api.openai.com` and `cdn.openai.com` refuse before
DNS or dialing. These are local handler observations with controlled upstream IO, not live
authentication or traffic evidence. Deployment and authenticated Codex acceptance remain separate.

`crates/mantle-conformance` adapts native observations to ESS's own Rust runner, pinned to
`8700d0808e8f3b19711629d8a17afc5281680f58` (0.50.0). It does not evaluate guards or copy an ESS
state machine. Integer observations preserve full unsigned widths; OS arguments and tunnel data
use Bytes. Read-your-writes tokens follow completed operations on the same native boundary.

Each component emits an official `ess-conformance-report/2`, detailed `ess-conformance-run/2`
and its exact suite. The aggregation command admits those reports with ESS, compares each executed
scenario definition to the complete suite, and rejects missing, duplicate or changed scenarios.
`spec/conformance-baseline.json` holds scenario names and answered/total floors; any failure, skip
or unsupported result fails the gate. The aggregate is explicitly an ESS external-results report
assembled from the three native runs, not a single-process Rust execution report.

A no-op audit runs all scenarios against accepted commands with empty observations. Every one
of the authored scenarios must fail that target. Six automatic acceptance/optional-return witnesses
pass it; their substantive behaviors are checked by the authored scenarios. Those six alone are
not evidence of behavior. The audit is retained alongside the native reports.

The incoming 304-scenario evidence lives in `.engineering/reports/native/`; fresh local runs
are written under `.engineering/drafts/`, and CI retains fresh suites,
runs and reports as the `mantle-native-conformance` artifact. The earlier 57-scenario report in
`.engineering/reports/mantle-ess-local.json` is historical evidence, superseded by the complete suite.
The coverage expansion found a real shutdown defect: detached proxy tasks survived the drain
boundary. `CheckDrain` failed before connection ownership was fixed. Mutation evidence also records
named failures after deliberately breaking production boundaries; every mutation was restored.

## Agent-ready worker decision seam

`AssessWorkerReadiness` calls `mantle_worker::worker_ready` over observed common readiness,
selected agent executable and declared Claude secret slot. Common readiness ignores Claude
credentials; selected Claude requires both its executable and slot. `AssessAgentInstallation`
calls the production installer decision over architecture and verification observations. The
installer obtains these facts through bounded download, digest and version probes; neither
command infers observations from `WorkerRecord`.

`mantle.orchestration.Observe` accepts optional `claude_selected`: absent retains the legacy
selected-Claude capability observation, while false requests common capabilities only. Both
use the production capability mapper; executable presence remains a separate selected-agent
probe. Every incoming observation case remains unchanged, including the missing-slot and
all-facts-absent assertions. The common-without-Claude case checks the new explicit selection.

Worker decision obligations and the authored `aw-01-through-aw-05-worker-decisions` sequence
run through the same native Boundary/Reply adapter and inventory as the incoming complete suite.
These are decisions only. AW-01 fresh KubeVirt provisioning, AW-03 preservation of an existing
Claude session and actual helper delivery require separate live evidence; EC2 shares rendering
but is not claimed as live-tested. Rust transaction tests additionally exercise real filesystem
publication, corruption, interrupted fetches, process limits and idempotence using local fixtures.

The installation and selected agent/authentication value types are generated by ESS into `generated/worker-model`; regeneration
compares all four portable artifacts byte for byte. `.ess-output/` is ignored because it records
machine-local output ownership, inode and directory state. Generated bytes are never formatted or
hand-edited. A coherent root-owned generation contains both executable and installation facts;
status verifies current bytes before reporting an installed candidate. READY is common readiness,
not an authentication assertion. Cloud-init's base bootstrap record remains separate.

The launcher `--volatile-replay` switch keeps the same bounded live scrollback and FIFO
attachment behavior, but refuses a preexisting `last-output` entry before readiness/child
dispatch and skips the final file write. Omission preserves persistent replay. LP01–LP06
are the six named `replay-policy-arguments`, `volatile-replay-lifecycle`,
`volatile-replay-exit-paths`, `volatile-replay-preflight`, `volatile-replay-bounds`, and
`persistent-replay-compatibility` scenarios. They execute real launcher/probe processes;
the adapter reports file presence, observed exit statuses, relayed bytes, replay bounds,
readiness creation events and synthetic canary presence in launcher diagnostics. Collectors
are bounded; a server killed with SIGKILL has its fixture child group explicitly killed
and reaped, with its actual exit status reported rather than an assumed signal.

The replay-policy change added 11 complete scenarios to the previous 34 obligations.
This is launcher-local persistence behavior. It does not prevent recording by Substrate,
Codex diagnostic files or the terminal client, and it does not enable confidential login.

CW01–CW09 extend those retained assertions with `codex-start`, `codex-private-runtime` and
`private-path-initialization`. The first runs actual legacy SQLite migration, identity validation,
the production preflight and manifest parser; the second observes the complete production request.
InsertSession's guarded InvalidIdentity refusal precedes accepting/external outcomes in ESS and
production, preventing the invalid agent/auth cross-product. Every persisted view includes the
generated identities. The third runs the launcher over real path/symlink/hardlink/mode fixtures,
including refusal on ordinary disk, and checks child dispatch/readiness and preserved auth bytes.
Existing default Claude and launcher snapshots gain only their new explicit fields.

The public CLI test also checks stored Codex attachment after the manifest changes: worker lookup refuses when no worker is configured and leaves stored exec/workspace unchanged. The bounded
`codex_qualification runtime-sinks` example separately verifies the pinned binary's actual tmpfs
SQLite/WAL and text-log placement with conflicting ordinary settings and isolated synthetic homes.
Its SQLite canary fixture and persistent positive control exercise the scanner while WAL is live;
they do not claim to exercise real token refresh. Managed policy and authenticated lifecycle
acceptance remain with the parent Codex integration story.

AB01–AB04 (`attach-backpressure-cancellation`) hold real attachment clients to a 1500ms
handled-cancellation deadline under unread terminal output and independently paused agent input.
The output producer is finite; the drained-output control must also exit. Blocking/nonblocking
PTY and aliased stdin/stdout fixtures compare original termios and file-status flags after exit.
Every case reattaches to the same surviving server/agent and requests a fresh response. A separate
24 KiB burst and 128 KiB bidirectional transfer use 4096-byte pipes and bounded draining to exercise
partial progress without invoking the server's intentional overflow/redraw behavior.

The client holds one 16 KiB pending buffer per direction, polls writes only while bytes are pending,
and stops reading a direction until its pending bytes are written. Both inherited descriptors are
snapshotted before either is made nonblocking, so alias restoration preserves the original flags.
Signal and resize handling run between bounded operations. These synthetic launcher observations
do not establish authenticated Claude/Codex conversation continuity.
