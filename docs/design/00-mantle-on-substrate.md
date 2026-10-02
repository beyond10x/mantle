# Mantle — Portable Cloud Development Sessions on Substrate

**Status:** Refined design proposal  
**Working product / CLI name:** `mantle`  
**Foundation:** `beyond10x/substrate`  
**Primary initial agent:** Claude Code  
**Primary initial use case:** many isolated coding sessions for Rust / ROS / CMake projects without consuming local CPU  
**Cloud target:** AWS EC2  
**Source of truth during a cloud session:** the remote Substrate workspace  
**Reviewed against Substrate:** 2026-10-01

> **One sentence:** Mantle turns a development session into a portable, confined Unix environment whose placement can be local or remote, while Substrate remains the per-host execution data plane.

This document supersedes the earlier `rbuild` / local-source-sync design.

> **Corrections against Substrate `0.7.8` (`0569597`), checked 2026-10-02.** Three statements below
> do not hold for the tagged source, and the first vertical slice works around them:
>
> 1. **A PTY session cannot be detached and reattached** (§10). It has exactly one attachment;
>    disconnect, protocol failure or send timeout cancels the whole process tree
>    (`adr/0008-pipe-sessions-have-distinct-durable-identity.md`, "Upgrade failure, disconnect …
>    triggers whole-tree cancellation"). The confinement seccomp profile also refuses
>    `socket(AF_UNIX, …)` with `EACCES` and datagram socketpairs (`substrate-host/src/seccomp.rs`,
>    `DENIED_FAMILIES`), so `tmux`, `screen` and any tool that serves a Unix socket cannot run in a
>    session. The slice runs `mantle-launch serve` as a long-lived exec that holds the agent on a
>    pseudo-terminal, and `mantle attach` opens a fresh PTY session running `mantle-launch attach`,
>    which relays to it through FIFOs in `/workspace/.mantle/agent`; `Ctrl-] d` detaches. A
>    reattachable session mode is a candidate product-neutral Substrate extension.
> 2. **The remote SDK transport requires a hosted Identity authority** (§11, §12). `ClientBuilder`
>    accepts a Unix socket, or HTTPS with trust roots, a DNS identity and an access-token provider;
>    there is no static-bearer remote client. The slice forwards the daemon's Unix socket over SSH
>    carried by AWS SSM, and the TLS/Identity path is a later milestone.
> 3. **`/runtime` is the execution-capsule mount** (`EXECUTION_CAPSULE_MOUNT`), and a run may
>    declare at most four read-only roots. The slice projects its toolchain at `/opt/mantle`. Host
>    `/usr`, `/bin`, `/lib` and `/lib64` are already bound read-only into every sandbox, so the
>    worker's installed packages are the slice's runtime image.
>
> Exec timeouts and leases are capped at 24 hours (`MAX_LEASE_TTL_MS`), which bounds one agent run.

---

## 1. Why the design changed

The earlier design kept Claude Code and the canonical source tree on the laptop, mirrored source to EC2, and transparently proxied build commands.

That works, but after reviewing the current `beyond10x/substrate` implementation and the desired experience, it is no longer the preferred architecture.

The cleaner model is:

```text
the laptop owns the experience
the remote worker owns the session
Substrate owns confinement and process truth
Mantle owns session intent and placement
```

The Claude Code process itself runs inside the remote confined environment.

There is no network-mounted source tree and no continuous local-to-remote source synchronization in the normal path.

From the agent's perspective it simply runs on Linux:

```text
/
├── runtime/       read-only runtime/toolchain
├── workspace/     writable session workspace
├── run/           session-local sockets/capabilities
└── ...
```

From the user's perspective it still feels local:

```bash
mantle start
mantle attach my-session
```

The terminal is local; the PTY and work happen remotely.

---

## 2. The name: Mantle

`b10x` is already an internal repository-management CLI and should remain that.

The proposed tool is named **Mantle**.

The metaphor is deliberate:

```text
Mantle
   ↓
Substrate
   ↓
Linux / EC2
```

Substrate is the execution foundation. Mantle is the session and placement layer above it.

The command line is concise:

```bash
mantle start
mantle attach
mantle list
mantle stop
mantle status
mantle doctor
```

The name is a working name; perform normal package/domain/trademark checks before an external release.

---

## 3. Current Substrate boundary

The design intentionally follows Substrate's current architecture rather than turning Substrate into a development product.

As reviewed on 2026-10-01, `beyond10x/substrate` describes itself as the b10x execution data plane for one Linux host. It currently owns:

- confined workspaces,
- bounded argv execution,
- raw-pipe sessions,
- PTY sessions,
- cgroup-backed resource constraints,
- durable operation reservation and observations,
- leases and cancellation,
- guarded workspace file operations,
- authorized HTTPS Git materialization,
- secret slots,
- no-egress execution plus explicit egress apertures,
- local Unix-socket transport,
- production TLS 1.3 HTTPS/WSS transport,
- a typed Rust SDK.

It explicitly does **not** own product policy, agent loops, fleet scheduling, billing, or product quotas.

Its current roadmap also keeps fleet scheduling and product policy outside Substrate.

That is the correct boundary for Mantle.

---

## 4. Architecture principles

### 4.1 A session is the unit of placement

Do not remotely place individual `cargo` invocations.

Place an entire development session.

A session contains:

- one agent process,
- one workspace,
- one runtime/toolchain,
- source repositories,
- a filesystem namespace,
- a process namespace,
- resource limits,
- a network policy,
- session-local credentials/capabilities,
- optional peer communication endpoints,
- caches,
- durable session metadata.

Placement is an attribute:

```text
local-linux
aws:dev-small
aws:dev-large
aws:dev-burst
```

The session semantics do not change with placement.

### 4.2 Substrate remains host-local execution truth

Substrate answers:

> What actually ran on this machine, under which bounds, and what was observed?

Mantle answers:

> What development session does the user want, and where should it run?

### 4.3 The terminal is not the session

An attached terminal is only a view onto a session.

```text
session lifecycle != terminal attachment lifecycle
```

Closing the laptop should not necessarily kill the session.

The user can detach and later reattach.

### 4.4 Remote source becomes canonical for that session

Once the cloud session starts, the workspace inside Substrate is the active source tree for that session.

Normal edits do not continuously mirror to the laptop.

Publication happens through Git or an explicit export.

### 4.5 Isolation is real, not convention

Two sessions on the same EC2 host should be isolated by Substrate, not merely by directory names.

Each session receives its own:

- writable workspace,
- confined process tree,
- resource accounting,
- network capability,
- credentials,
- peer-channel authority.

### 4.6 Capabilities, not ambient authority

A session should not inherit:

- the host filesystem,
- arbitrary host sockets,
- the EC2 instance role,
- unrestricted VPC access,
- unrestricted internet,
- production credentials.

It gets explicit capabilities.

### 4.7 Mantle depends on Substrate; never the reverse

```text
mantle
   ↓
b10x-substrate-sdk / released contract
   ↓
substrate-daemon
```

No Mantle crate, type, product concept, or AWS policy enters Substrate.

If Mantle needs a missing primitive, the primitive is added to Substrate only if it is genuinely product-neutral execution-data-plane functionality.

---

## 5. User experience

The target experience is intentionally simple.

Given:

```yaml
# mantle.yaml
apiVersion: mantle.beyond10x.dev/v1alpha1
kind: Session

metadata:
  name: substrate-work

placement:
  class: aws-dev-large

runtime:
  profile: rust-dev

workspace:
  root: /workspace
  repositories:
    - name: substrate
      repository: https://github.com/beyond10x/substrate.git
      ref: main
      mount: substrate

    - name: connectors
      repository: https://github.com/beyond10x/connectors.git
      ref: main
      mount: connectors

agent:
  kind: claude-code
  cwd: /workspace/substrate

resources:
  cpu: 16
  memory: 48GiB
  storage: 200GiB

network:
  profile: developer-rust
```

the user runs:

```bash
mantle start
```

and sees:

```text
Session        substrate-work
Placement      aws-dev-large
Worker         aws/eu-central-1/i-012345...
Runtime        rust-dev@sha256:...
Workspace      /workspace
Resources      16 CPU / 48 GiB / 200 GiB

Materializing:
  /workspace/substrate
  /workspace/connectors

Network:
  model inference        allowed
  GitHub HTTPS           allowed
  Rust package sources   allowed
  everything else        denied

Starting Claude Code...

╭──────────────────────────────────╮
│ Claude Code                      │
│ /workspace/substrate             │
╰──────────────────────────────────╯
>
```

Claude runs:

```bash
cargo test
```

on EC2.

The laptop only:

- renders the terminal,
- sends keystrokes,
- receives output,
- runs the Mantle client.

---

## 6. System overview

```text
                              USER LAPTOP
┌──────────────────────────────────────────────────────────────────┐
│ mantle CLI                                                       │
│                                                                  │
│ manifest resolution                                              │
│ session lifecycle UI                                             │
│ placement request                                                │
│ Identity authentication                                          │
│ PTY attach / detach                                              │
│ session list/status                                              │
└──────────────────────────────┬───────────────────────────────────┘
                               │
                               │ control
                               ▼
                    ┌──────────────────────┐
                    │ Mantle Coordinator   │
                    │                      │
                    │ session metadata     │
                    │ placement            │
                    │ AWS worker lifecycle │
                    │ runtime resolution   │
                    │ capability policy    │
                    └──────────┬───────────┘
                               │
                     choose/provision worker
                               │
                               ▼
                            AWS EC2
┌──────────────────────────────────────────────────────────────────┐
│ Worker host                                                      │
│                                                                  │
│ substrate-daemon                                                 │
│ Mantle egress gateway endpoint                                  │
│ optional Mantle peer relay endpoint                             │
│ runtime/image/cache assets                                      │
│                                                                  │
│  ┌──────────── Substrate workspace/session A ─────────────────┐  │
│  │ /runtime        read only                                  │  │
│  │ /workspace      writable                                   │  │
│  │ /run/mantle     scoped local endpoints                     │  │
│  │                                                           │  │
│  │ Claude Code                                                │  │
│  │ cargo / rustc / tests                                      │  │
│  │ private build state                                        │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                  │
│  ┌──────────── Substrate workspace/session B ─────────────────┐  │
│  │ isolated filesystem / process / network / resource domain  │  │
│  └───────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
                               │
                               ▼
                    optional shared services
             ┌───────────────┬──────────────┐
             │ cache backend │ Git broker   │
             │ S3 / service  │ credential   │
             └───────────────┴──────────────┘
```

---

## 7. What Mantle owns

Mantle owns product/session policy.

### Session intent

- session manifest,
- selected agent,
- selected runtime profile,
- repository set,
- requested resources,
- requested network capabilities,
- requested peer communication.

### Placement

- local vs cloud,
- provider,
- region,
- worker class,
- worker selection,
- capacity,
- affinity,
- scale-out,
- scale-in.

### Provisioning

- EC2 Launch Templates / Auto Scaling integration,
- worker bootstrap,
- worker certificate identity,
- worker registration,
- worker health.

### Development environment composition

- toolchain profile,
- Claude Code adapter,
- Rust/ROS/CMake profiles,
- setup hooks,
- credential adapters,
- package-source configuration.

### Source-set composition

- multiple repositories,
- exact commits,
- branches,
- local dirty-state handoff,
- publication policy.

### User-facing session lifecycle

- start,
- attach,
- detach,
- suspend/idle policy,
- resume,
- stop,
- export,
- publish.

### Session-to-session communication

- logical peer identities,
- authorization,
- routing,
- discovery,
- local socket projection.

### Cloud costs and policy

- worker size,
- allowed instance families,
- On-Demand baseline,
- Spot use,
- idle shutdown,
- budget policy.

---

## 8. What Substrate owns

Substrate continues to own mechanism.

- create confined workspace,
- guarded file operations,
- execute process,
- create PTY,
- durable operation identity,
- output observation,
- cancellation,
- leases,
- process-tree cleanup,
- cgroup facts,
- filesystem quota facts,
- egress mechanism facts,
- secret-slot mechanism,
- per-run applied facts,
- authenticated host API.

Mantle does not claim an isolation guarantee Substrate did not report.

Example:

```text
Mantle asks for PTY
       ↓
Substrate machine facts do not advertise PTY
       ↓
session start fails by named capability mismatch
```

Never silently degrade to an unconfined shell.

---

## 9. Session domain model

```text
Session
├── id
├── name
├── manifest_digest
├── requested_placement
├── resolved_placement
├── worker
├── substrate_workspace_ref
├── runtime
├── sources[]
├── resources
├── network_policy
├── peer_policy
├── agent
├── status
├── created_at
├── last_activity
└── publication_state
```

Recommended lifecycle:

```text
REQUESTED
    ↓
PLACING
    ↓
PROVISIONING
    ↓
MATERIALIZING
    ↓
STARTING
    ↓
READY
    ↓
RUNNING
   ↙   ↘
DETACHED  ATTACHED
    \     /
     IDLE
      ↓
   STOPPING
      ↓
    STOPPED
```

Failure is explicit:

```text
FAILED_PLACEMENT
FAILED_CAPABILITY
FAILED_MATERIALIZATION
FAILED_RUNTIME
FAILED_AGENT_START
LOST
```

---

## 10. Attachments and session durability

An agent process may outlive a terminal.

Substrate already has a distinct session identity and PTY attachment model. Mantle should use that directly.

The local CLI stores only enough information to re-resolve the session:

```text
session id
coordinator endpoint
```

Attach:

```bash
mantle attach substrate-work
```

Detach:

```text
Ctrl-] d
```

Kill:

```bash
mantle stop substrate-work
```

Closing a terminal is not equivalent to kill.

---

## 11. Direct data path

Avoid routing terminal bytes through a central Mantle service if possible.

Preferred path:

```text
CLI
 ↓ asks coordinator
Coordinator
 ↓ returns worker address + authority metadata
CLI
 ↓ obtains Identity authority
Substrate TLS/WSS
 ↓
PTY
```

The coordinator is not in the byte path.

Benefits:

- lower latency,
- less central bandwidth,
- fewer failure modes,
- Substrate remains the authoritative execution endpoint.

---

## 12. Authentication

Use Substrate's current production remote transport instead of inventing another worker auth protocol.

Current Substrate source supports:

- TLS 1.3 HTTPS/WSS,
- explicit trust roots,
- expected DNS identity,
- short-lived hosted Identity authority,
- route-scoped authorization,
- one-use proof-bound session attachment authority.

Mantle should act as a client of that contract.

The worker does not trust a caller-provided session identity.

---

## 13. AWS network path

### MVP: no inbound security-group ports

For the first AWS implementation:

```text
Laptop
  ↓
AWS SSM port-forward
  ↓
worker localhost / private listener
  ↓
Substrate TLS
```

The Mantle client can establish the SSM tunnel automatically.

Substrate still performs its own TLS and Identity checks end-to-end.

This keeps:

```text
EC2 security group ingress = none
```

while testing the real remote SDK and WSS attachment path.

### Later: direct private path

When multiple workers become routine, use one of:

- corporate/private VPN into the VPC,
- managed overlay,
- a Mantle L4 CONNECT relay,
- another private routed path.

Substrate TLS remains end-to-end.

Do not terminate Substrate trust at an HTTP reverse proxy unless the Substrate trust model explicitly allows it.

---

## 14. Workspace source model

A session commonly needs more than one repository.

Example:

```text
/workspace
├── substrate
├── connectors
├── application
└── docs
```

Current Substrate Git materialization handles an empty workspace or an authorized configured HTTPS Git source at an exact commit.

Mantle needs a multi-source composition model.

There are two implementation stages.

### Stage A — works with current Substrate

For the first vertical slice:

1. create an empty Substrate workspace;
2. run a dedicated initializer process;
3. give that process only:
   - the Git egress capability,
   - temporary Git credential capability,
   - the workspace;
4. materialize the declared repositories;
5. remove initializer-only authority;
6. start the agent with the normal runtime network policy.

This keeps multi-repo policy in Mantle.

The initializer must resolve every ref to an exact commit and record the result:

```text
declared:
  substrate@main

resolved:
  substrate@4b7f...40hex
```

The resolved source set becomes part of the session record.

### Stage B — proposed Substrate extension

Long-term, generalize Git workspace materialization to multiple source roots.

This is a reasonable Substrate primitive because it is product-neutral:

```text
WorkspaceMaterialization
  sources:
    - source authority
      locator
      exact commit
      mount path
    - ...
```

Required invariants:

- every mount path is relative and beneath `/workspace`,
- no overlap,
- no symlink escape,
- each source authority is independently scoped,
- exact commit recorded,
- source count bounded,
- total bytes/inodes bounded,
- failure is atomic or explicitly reports partial state,
- `.git` exposure follows one consistent rule.

This should be added as a new versioned Substrate contract, not as a Mantle-specific route.

---

## 15. Starting from a local checkout

A cloud session should be easy to start from the code currently on the laptop.

Example:

```bash
cd ~/src/substrate
mantle start --from .
```

Mantle resolves:

```text
remote URL
current branch
HEAD commit
dirty tracked changes
untracked files
```

Three cases:

### Clean and reachable

Use exact Git commit.

### Local commits not on remote

Do **not** force a GitHub branch push.

Mantle may send an ephemeral Git bundle or equivalent source snapshot to the initializer.

### Dirty worktree

Materialize the exact base commit, then apply an explicit bounded local overlay.

After handoff, the cloud session is canonical.

There is no continuous sync.

Mantle records:

```text
base commit
overlay digest
overlay file count
overlay bytes
```

A local source handoff is a bootstrap mechanism, not a second source of truth.

---

## 16. Publishing code back

Normal cloud-session output should be Git-native.

Inside the session:

```text
edit
test
commit
publish branch
```

The preferred model is **not** to store a long-lived GitHub token in the session.

Use a Git credential broker.

### Git credential broker

Provide a runtime helper:

```text
git-credential-mantle
```

Git invokes it using the standard credential-helper protocol.

The helper talks to a session-local endpoint:

```text
/run/mantle/credentials.sock
```

The endpoint asks a trusted Mantle broker for a short-lived repository-scoped credential.

Properties:

- no durable GitHub token in `/workspace`,
- no EC2 instance-wide GitHub token,
- credentials scoped to repository/action/session,
- revocation when session stops,
- audit by session identity.

For the first implementation, a Substrate secret slot can deliver a short-lived credential to the initializer or launcher.

The brokered model is the target architecture.

---

## 17. Agent credentials

Agent runtimes have different credential expectations.

Do not bake credential logic into Substrate.

Mantle defines agent adapters.

Example:

```rust
trait AgentAdapter {
    fn command(&self, session: &ResolvedSession) -> CommandSpec;
    fn runtime_requirements(&self) -> RuntimeRequirements;
    fn credential_requirements(&self) -> Vec<CredentialRequest>;
    fn prepare(&self, capability: AgentCapability) -> PreparedAgent;
}
```

Initial adapter:

```text
ClaudeCodeAdapter
```

A credential should enter the confined environment through a controlled mechanism such as:

- a Substrate sealed secret slot consumed by a launcher,
- a session-local credential broker,
- a short-lived private configuration file created outside `/workspace`.

It should not become durable EC2 host state.

---

## 18. Runtime environment

A useful development session needs more than one executable.

Typical environment:

```text
shell
git
Claude Code
rustup / rustc / cargo
clang / linker
cmake
ninja
Python
ROS tooling
package-manager clients
debuggers
project-specific utilities
```

This reveals the most important gap between today's Substrate and the ideal Mantle environment.

Substrate currently has exact execution capsules and a read-only `/runtime`, while images are still a future resource family.

A practical development session wants a reusable, versioned runtime image.

### Proposed Substrate extension: Runtime Image resource

Add a product-neutral runtime/image resource.

Conceptually:

```text
RuntimeImage
├── digest
├── architecture
├── filesystem image
├── metadata
├── entrypoint compatibility
└── verified facts
```

A session requests:

```yaml
runtime:
  image: ghcr.io/beyond10x/dev-rust@sha256:...
```

Substrate resolves the immutable image and projects it read-only as `/runtime` / rootfs according to the selected driver.

Required properties:

- digest-pinned,
- immutable,
- bounded materialization,
- no mutable tag as authoritative identity,
- applied digest reported,
- unsupported driver refuses,
- no ambient host paths,
- reproducible environment.

Mantle selects images.

Substrate only materializes and reports them.

This is a higher-priority Substrate extension for Mantle than Kubernetes or Firecracker.

### Runtime profiles

Mantle offers named profiles that resolve to immutable images.

Examples:

```text
rust-dev
ros2-jazzy
cpp-dev
general-dev
```

Manifest:

```yaml
runtime:
  profile: rust-dev
```

Resolution:

```text
rust-dev
  → ghcr.io/beyond10x/mantle-runtime-rust@sha256:abc...
```

Profiles are Mantle policy and may change.

Session records preserve the resolved immutable digest.

---

## 19. Project setup

The runtime should contain generic tooling.

Project-specific setup belongs in the manifest:

```yaml
setup:
  - command: rustup show
  - command: cargo fetch --locked
```

or a repository-owned file:

```text
.mantle/setup.yaml
```

Setup runs as a separately observed phase.

A setup failure must not be reported as an agent failure.

---

## 20. Network: the important current mismatch

Current Substrate ordinary execution has no egress.

Its current aperture model deliberately grants one operator-declared, pinned TCP destination to a run.

That is excellent for a bounded tool call.

A development agent, however, may simultaneously need:

```text
model API
GitHub
crates.io
static.crates.io
ROS repositories
apt repositories
private artifact services
documentation sites
```

Do not weaken Substrate into "sandbox has internet."

Instead add a Mantle egress gateway.

### Mantle egress gateway

Each session gets one Substrate aperture to a trusted gateway:

```text
confined session
     │
     │ one existing Substrate aperture
     ▼
Mantle egress gateway
     │
     ├── api.anthropic.com
     ├── github.com
     ├── crates.io
     ├── static.crates.io
     └── allowed internal services
```

The session sees a normal HTTP(S) proxy:

```text
HTTPS_PROXY=http://mantle-egress.local:3128
HTTP_PROXY=http://mantle-egress.local:3128
```

Substrate pins only the gateway endpoint.

Mantle's gateway evaluates the higher-level destination policy.

This preserves the strong Substrate statement:

```text
the sandbox cannot choose an arbitrary network destination
```

while making real developer sessions practical.

### Network policy

Manifest:

```yaml
network:
  default: deny
  capabilities:
    - model.anthropic
    - git.github
    - rust.crates
```

Mantle resolves capabilities to gateway rules.

For example:

```text
rust.crates
  crates.io:443
  static.crates.io:443
  index.crates.io:443
```

The session does not provide arbitrary hostnames in the Substrate request.

Destination policy remains operator-controlled.

### Egress gateway requirements

- CONNECT support for HTTPS,
- destination allowlist,
- DNS performed by gateway, not sandbox,
- per-session identity,
- per-session byte accounting,
- connection limits,
- audit events,
- optional bandwidth limit,
- destination-category metrics,
- no credential logging,
- explicit refusal reason.

Do not MITM TLS by default.

SNI / CONNECT hostname policy is sufficient for the initial development use case.

---

## 21. Session-to-session communication

Multiple Claude sessions may need to communicate.

Do not give them arbitrary access to each other's network namespace.

Do not let confined processes connect to host Unix sockets.

Expose an explicit peer-channel capability.

The desired agent-facing interface can still be socket-based:

```text
/run/mantle/bus.sock
```

but the socket is session-scoped.

### Mantle peer bus

Logical model:

```text
Claude A
  │
  │ /run/mantle/bus.sock
  ▼
session endpoint A
  │
  ▼
Mantle Relay
  │
  ├── Session B
  ├── Session C
  └── local controller
```

Authorization examples:

```text
parent -> child           allowed
child -> parent           allowed
siblings                  manifest-dependent
unrelated sessions        denied
broadcast                 explicit
```

Application messages have:

```text
source session
destination session
channel
message id
payload
timestamp
```

### Proposed Substrate extension: Endpoint resource

A clean implementation deserves a first-class Substrate endpoint primitive.

Substrate already lists endpoints as a future resource family.

Proposed abstraction:

```text
Endpoint
├── identity
├── direction
├── protocol
├── bound workspace/session
├── backing provider handle
└── applied facts
```

A session can request a named endpoint:

```text
mantle.peer_bus
```

Substrate projects it into the sandbox as a local Unix socket or loopback endpoint without exposing the host socket namespace.

Important:

- caller cannot choose arbitrary host path,
- endpoint is an opaque operator-declared handle,
- lifecycle follows session,
- session identity is attached out-of-band,
- bytes/connections bounded,
- endpoint policy reported.

This is useful beyond Mantle and therefore belongs in Substrate.

### MVP peer bus without Endpoint resource

Do not block the first prototype on the new resource.

MVP option:

1. use the single egress gateway aperture;
2. expose the peer relay as one allowed internal destination;
3. run a small session-local proxy in the runtime;
4. proxy offers `/run/mantle/bus.sock`;
5. proxy tunnels to Mantle Relay.

The eventual Endpoint resource removes this adapter.

---

## 22. Resource isolation

A session asks for:

```yaml
resources:
  cpu: 16
  memory: 48GiB
  pids: 4096
  storage: 200GiB
```

Mantle verifies the target worker's Substrate machine facts before admission.

No fact → no session.

Do not interpret "requested" as "enforced."

The session status should show:

```text
Requested:
  CPU       16
  Memory    48 GiB

Applied:
  CPU       ...
  Memory    ...
  PIDs      ...
  Storage   ...
  Mechanism cgroup v2 / project quota / ...
```

The applied values come from Substrate observations.

---

## 23. Worker-level scheduling

Substrate confines one host but should not decide product placement.

Mantle tracks worker capacity.

Example:

```text
Worker A
  allocatable CPU     60
  allocatable memory  220 GiB
  disk free           1.4 TiB
  sessions            8

Worker B
  allocatable CPU     28
  allocatable memory  100 GiB
  sessions            3
```

Placement score:

```text
+ existing session affinity
+ runtime image already warm
+ source/cache locality
+ sufficient free disk
+ sufficient memory
+ region preference
- CPU pressure
- memory pressure
- disk pressure
- Spot interruption risk
```

### CPU sharing

There are two different controls:

1. session admission / placement;
2. kernel enforcement on a host.

Mantle decides how many sessions enter a worker.

Substrate/cgroups enforce each session's process-tree bounds.

For interactive development, prefer:

- a guaranteed/fair CPU weight,
- a maximum burst ceiling,
- memory hard bound,
- PID hard bound,

rather than giving every session an independently optimistic `cargo -j <host CPU count>`.

A future Cargo adapter inside the runtime may set:

```text
CARGO_BUILD_JOBS
NINJAFLAGS
MAKEFLAGS
```

from the session's CPU allocation.

This is runtime configuration, not Substrate policy.

---

## 24. Build caching model

The full remote-session model simplifies caching.

The hottest cache is simply the persistent session workspace:

```text
/workspace/repo/target
```

A one-character edit hits the same machine and the same target tree.

No local/remote sync occurs.

Cache hierarchy:

```text
1. session-private build state
2. worker/user compiler cache
3. fleet-wide shared compiler cache
4. upstream package registry
```

### Phase 1 cache: session-private only

First prove:

```text
edit
cargo check
edit
cargo check
```

inside one persistent session.

This alone solves a large part of the original problem.

Do not add shared cache complexity before measuring it.

### Phase 2 cache: compiler cache

Benchmark:

- `sccache`,
- `kache`,
- persistent target only.

Measure:

- p50 edit → diagnostics,
- p95 edit → diagnostics,
- CPU seconds,
- disk growth,
- network bytes,
- weighted hit rate.

Do not choose on raw hit percentage.

### Cache authority problem

A confined session should not receive the EC2 instance role simply so `sccache` can write S3.

Preferred options, in order:

1. a user/session-scoped cache service,
2. a Substrate-managed cache volume/resource,
3. short-lived tightly scoped cloud credentials injected through Mantle.

Avoid exposing IMDS to the sandbox.

### Proposed Substrate extension: Volume / cache resource

Substrate already identifies volumes as a future resource family.

Mantle has a concrete use case:

```text
session-private workspace volume
user-scoped compiler cache
ephemeral high-speed build volume
```

Proposed generic resource:

```text
Volume
├── opaque id
├── class
├── scope
├── byte/inode limit
├── lifecycle
├── mount target
└── applied backing facts
```

Examples:

```text
scope=session
scope=principal
scope=project
```

Mantle might request:

```yaml
volumes:
  - name: compiler-cache
    class: cache
    scope: principal
    mount: /cache/compiler
```

Substrate owns safe mount/materialization semantics.

Mantle owns which cache the user gets.

Do not implement "mount arbitrary host path."

---

## 25. Cargo configuration example

Inside a Rust runtime:

```text
RUSTC_WRAPPER=sccache
CARGO_INCREMENTAL=0
SCCACHE_DIR=/cache/compiler
CARGO_HOME=/workspace/.mantle/home/cargo
```

or an equivalent kache configuration.

The exact cache implementation remains a Mantle runtime-profile choice.

---

## 26. ROS and non-Cargo workloads

Nothing in Mantle's session model is Cargo-specific.

Example ROS profile:

```yaml
runtime:
  profile: ros2-jazzy

resources:
  cpu: 24
  memory: 64GiB

network:
  capabilities:
    - model.anthropic
    - git.github
    - ros.packages
    - apt.ubuntu
```

Inside:

```bash
colcon build
colcon test
```

Build outputs stay inside the session workspace.

Likewise:

```text
CMake
Ninja
Make
Bazel
Python
Node
custom generators
simulation tools
```

The heavy-compute problem is solved by session placement, not command interception.

---

## 27. Workspace persistence

A cloud development session cannot casually lose source changes when a worker disappears.

Current Substrate explicitly notes that general workspace backup/restore snapshots are absent.

Therefore do not enable disposable Spot workers before a durability story exists.

### MVP durability

Use one On-Demand EC2 instance.

Put Substrate workspace state on durable EBS, not instance-store NVMe.

Example:

```text
root EBS
  OS

workspace EBS / gp3 or io2
  /var/lib/substrate/state
  /var/lib/substrate/workspaces

optional local NVMe
  disposable runtime/download/cache data
```

This trades some peak build I/O performance for safer early development.

Benchmark before moving workspace build trees to local NVMe.

### Proposed Substrate extension: workspace snapshots

Before Spot/fleet migration becomes a normal session behavior, add a product-neutral snapshot/export primitive.

Conceptually:

```text
WorkspaceSnapshot
├── workspace id
├── source generation
├── content digest
├── byte count
├── created at
└── backing object reference
```

Operations:

```text
snapshot workspace
restore snapshot into new workspace
```

Required properties:

- atomic snapshot point,
- exact digest,
- bounded size,
- no symlink escape,
- explicit inclusion/exclusion semantics,
- encrypted backing,
- restore reports exact applied identity.

Mantle can then store snapshots in S3.

Substrate remains unaware of "Claude" or "session migration."

### Git checkpoints as a complementary layer

Even with snapshots, source should be recoverable through Git.

Mantle can optionally auto-checkpoint:

```text
refs/mantle/sessions/<session-id>/checkpoint
```

or a separate snapshot repository.

Do not force user-visible commits into the development branch.

A session can publish a clean branch when ready.

---

## 28. AWS worker design

Start with a simple worker image.

Installed/configured:

- Linux,
- systemd,
- cgroup v2 delegation,
- verified bubblewrap version/profile,
- `socat` as required by current Substrate probes,
- `substrate-daemon`,
- SSM Agent,
- Mantle egress gateway client/endpoint if host-local,
- runtime image cache,
- metrics agent,
- certificate/trust material.

Worker boot performs:

```text
1. verify cgroup delegation
2. verify workspace filesystem / quotas if required
3. start Substrate
4. inspect GET /v1/machine
5. register observed facts with Mantle
6. enter READY only if required baseline facts are present
```

### Initial instance family

For the first serious test, prefer an On-Demand memory-balanced host with local NVMe available for later cache experiments.

Examples to benchmark:

```text
M8id class
C8id class
```

Do not assume the compute-optimized family wins.

Rust and ROS builds can be memory- and I/O-sensitive.

Start with enough RAM that simultaneous compilers do not force reclaim/swap.

---

## 29. AWS IAM

The worker EC2 role should be minimal.

Host-level permissions might include:

- SSM managed-instance operations,
- read of immutable runtime artifacts,
- optional write/read to Mantle snapshot/cache prefixes,
- metrics/log publishing,
- certificate bootstrap if required.

The confined session does not inherit that role.

---

## 30. AWS network

Initial security group:

```text
ingress: none
egress:
  SSM/AWS control requirements
  runtime/package infrastructure as needed by host services
```

The session does not receive host network access.

For the initial client path, Mantle starts SSM port forwarding.

Later private direct paths may reduce tunnel overhead.

---

## 31. Worker identity and TLS

Each worker receives:

```text
worker id
DNS identity
TLS certificate/key
explicit trust chain
```

Substrate's production TLS listener remains authoritative.

The Mantle client knows:

```text
connect origin
expected DNS identity
trust roots
Identity token provider
```

A tunnel changes routing, not identity.

---

## 32. Mantle coordinator

The coordinator can begin as a local daemon:

```text
mantled
```

Local-only first:

```text
~/.local/state/mantle/state.db
```

It owns:

- session records,
- worker records,
- AWS instance ids,
- placement,
- manifest resolutions,
- source resolutions,
- lifecycle.

For a single developer, no hosted control plane is necessary initially.

Later the same application layer can become a service for multiple developers.

---

## 33. Coordinator domain

```text
SessionRepository
WorkerRepository
PlacementPort
SubstratePort
CloudProviderPort
IdentityPort
SourceResolverPort
RuntimeResolverPort
Clock
```

AWS and Substrate are adapters.

Do not put AWS SDK calls in session-domain decisions.

### Cloud provider abstraction

```rust
trait CloudProvider {
    async fn ensure_capacity(
        &self,
        request: CapacityRequest
    ) -> Result<Vec<WorkerCandidate>>;

    async fn start_worker(
        &self,
        class: WorkerClass
    ) -> Result<ProvisionedWorker>;

    async fn stop_worker(
        &self,
        worker: WorkerId
    ) -> Result<()>;

    async fn open_transport(
        &self,
        worker: WorkerId
    ) -> Result<WorkerTransport>;
}
```

Implementations:

```text
LocalLinuxProvider
StaticVmProvider
AwsEc2Provider
```

Substrate is not a `CloudProvider`.

---

## 34. Substrate adapter

Use `b10x-substrate-sdk` from an exact reviewed source/release as Substrate itself recommends for consumers.

Responsibilities:

- query machine facts,
- create workspace,
- materialize source,
- start raw/PTY session,
- attach WSS,
- cancel,
- observe,
- query output,
- file operations where needed.

Mantle stores Substrate resource refs as opaque references.

---

## 35. Manifest

Proposed full shape:

```yaml
apiVersion: mantle.beyond10x.dev/v1alpha1
kind: Session

metadata:
  name: substrate-feature

placement:
  class: aws-dev-large
  region: eu-central-1
  affinity: sticky

runtime:
  profile: rust-dev

workspace:
  root: /workspace

  repositories:
    - name: substrate
      repository: https://github.com/beyond10x/substrate.git
      ref: main
      mount: substrate
      mode: read-write

    - name: docs
      repository: https://github.com/beyond10x/docs-system.git
      ref: main
      mount: docs
      mode: read-only

resources:
  cpu: 16
  memory: 48GiB
  pids: 4096
  storage: 200GiB

network:
  default: deny
  capabilities:
    - model.anthropic
    - git.github
    - rust.crates

peers:
  allow:
    - selector: project

agent:
  kind: claude-code
  cwd: /workspace/substrate

lifecycle:
  detachKeepsRunning: true
  idleAfter: 30m
  retainFor: 8h

caches:
  - profile: rust-compiler
```

---

## 36. Manifest resolution

Never run directly from unresolved names.

Persist a resolved session specification:

```text
placement class
→ worker class constraints

runtime profile
→ immutable runtime digest

repository ref
→ exact commit

network capability
→ concrete gateway policy version

cache profile
→ concrete cache backend

agent kind
→ exact launcher/runtime version
```

The resolved spec gets a digest.

The session status displays it.

---

## 37. CLI

Suggested command surface:

```text
mantle init

mantle start [manifest]
mantle start --from .
mantle attach <session>
mantle detach
mantle stop <session>
mantle restart <session>

mantle list
mantle status <session>
mantle inspect <session>
mantle logs <session>

mantle exec <session> -- <argv...>
mantle shell <session>

mantle publish <session>
mantle export <session>

mantle worker list
mantle worker inspect <worker>

mantle doctor
mantle benchmark
```

Avoid exposing Substrate implementation terminology in the common UX unless inspecting/debugging.

---

## 38. Starting multiple sessions

Example:

```bash
mantle start feature-a.yaml
mantle start feature-b.yaml
mantle start feature-c.yaml
```

Mantle may place them:

```text
feature-a → worker-01
feature-b → worker-01
feature-c → worker-02
```

Each has an independent Substrate workspace and process boundary.

The user does not need one EC2 instance per agent.

---

## 39. Session peer communication example

Manifest:

```yaml
peers:
  identity: parent

  allow:
    - to: child:*
      channels:
        - coordination
        - review
```

A child sees:

```text
/run/mantle/bus.sock
```

and sends:

```json
{
  "to": "parent",
  "channel": "coordination",
  "payload": {
    "status": "tests-pass"
  }
}
```

The relay authenticates the sending session from the endpoint binding.

The payload cannot claim a different sender.

### Session network versus peer bus

These are separate authorities.

```text
external network capability
!=
session peer capability
```

A session may have:

```text
no external internet
yes peer bus
```

or:

```text
GitHub + crates
no peer bus
```

Do not route peer communication through general internet permission.

---

## 40. Local placement

Mantle should retain a local mode for testing and smaller tasks.

On a Linux machine with valid Substrate confinement:

```bash
mantle start --placement local
```

The coordinator starts or uses a local `substrate-daemon`.

On macOS, local placement requires a Linux VM because the Substrate execution guarantees are Linux-specific.

The manifest remains unchanged.

---

## 41. VM-first test plan

Before AWS automation, validate the architecture on one ordinary Linux VM.

The laptop runs Mantle.

The VM runs Substrate.

Use the real remote/PTTY path as early as possible.

Acceptance:

1. Mantle creates a workspace.
2. Source materializes.
3. Claude Code starts inside PTY.
4. terminal resizes correctly.
5. detach does not kill session.
6. reattach restores interaction.
7. `cargo check` burns VM CPU, not laptop CPU.
8. second build reuses remote build state.
9. second session cannot read first session's workspace.
10. second session cannot connect to first session except via granted peer bus.
11. external network default is denied.
12. declared gateway destinations work.
13. undeclared destinations fail.
14. Ctrl-C kills the remote process tree as expected.
15. resource limits are observed and displayed.
16. killing Mantle CLI does not imply successful remote cancellation.
17. status distinguishes unknown outcome from success.

---

## 42. AWS Phase 1: one static On-Demand worker

After VM success:

```text
one EC2 worker
one EBS workspace volume
one Substrate daemon
SSM access
no Spot
no ASG
no public listener
```

Mantle provisions or references the instance.

All real daily sessions can use it.

This immediately moves CPU off the laptop.

### Success criteria

- 10+ simultaneous sessions can exist.
- idle Claude sessions consume little CPU.
- active builds are confined.
- one session cannot see another workspace.
- local CPU remains low during builds.
- attach latency is acceptable.
- repeated build latency is comparable to normal remote-local development.
- stopping one session does not affect others.
- worker restart behavior is understood.
- workspace durability is measured.
- total EBS IOPS/throughput is measured.
- host memory pressure is measured.

---

## 43. AWS Phase 2: development capabilities

Implement/prove:

- Mantle egress gateway,
- Git credential flow,
- local dirty-state handoff,
- multi-repo initializer,
- runtime profile,
- Claude adapter hardening,
- peer bus.

Exit:

> normal multi-repo work needs no manual SSH/setup.

---

## 44. AWS Phase 3: Substrate product-neutral extensions

Prioritize:

1. Runtime Image.
2. Multi-source workspace materialization.
3. Endpoint.
4. Volume/cache resource as justified by measurement.

Exit:

> initializer/proxy workarounds shrink substantially.

---

## 45. AWS Phase 4: caching and density

Add:

- compiler-cache benchmark,
- worker resource scheduler,
- many sessions per host,
- runtime/cache locality placement.

Exit:

> aggregate 20-session throughput is predictably higher than local baseline.

---

## 46. AWS Phase 5: durability and fleet

Add:

- workspace snapshots,
- multi-worker placement,
- failure restoration,
- autoscaling.

Exit:

> sessions survive normal worker replacement without source loss.

---

## 47. AWS Phase 6: Spot burst

Only after snapshot/restore is proven:

```text
On-Demand baseline
+
Spot burst
```

Worker receives interruption signal:

```text
READY
  ↓
DRAINING
```

No new sessions.

Existing sessions:

- snapshot if policy allows,
- auto-checkpoint source,
- detach,
- restore elsewhere.

Do not pretend live process memory migrates unless Substrate actually supports it.

Source/workspace migration and process-memory migration are different capabilities.

---

## 48. Session recovery semantics

Explicitly distinguish:

```text
reattach
restart process in same workspace
restore workspace on another worker
resume exact process memory
```

Initial product supports:

```text
reattach                  yes
restart in same workspace yes
restore workspace         later
resume process memory     no
```

Do not call a restarted Claude process "resumed" if conversation/process memory was not actually restored.

Agent-specific continuation can use the agent's own session semantics where available.

---

## 49. Observability

`mantle status` should show three layers.

Example:

```text
Session
  id             ses_01...
  state          RUNNING
  attached       yes
  age            2h13m
  manifest       sha256:...

Placement
  provider       aws
  region         eu-central-1
  worker         i-0123...
  class          aws-dev-large

Substrate
  workspace      ws_...
  process        sess_...
  transport      TLS/WSS via SSM tunnel
  confinement    available
  PTY            available
  storage quota  available

Applied resources
  memory         48 GiB
  pids           4096
  ...

Network
  model.anthropic allowed
  git.github      allowed
  rust.crates     allowed
  default         deny

Sources
  substrate      4b7f...
  connectors     92a1...

Runtime
  rust-dev       sha256:...
```

Mantle must label requested vs applied values.

### Metrics

Per session:

- start latency,
- source-materialization latency,
- agent-start latency,
- attach latency,
- CPU usage,
- memory usage,
- network bytes by capability,
- workspace bytes,
- build command durations where observable,
- cache metrics,
- idle time.

Per worker:

- active sessions,
- runnable CPU,
- memory pressure,
- storage pressure,
- runtime image locality,
- cache size/hits,
- placement failures.

---

## 50. Security invariants

1. No session gets host filesystem paths.
2. No session gets host Unix-socket namespace.
3. No session gets EC2 instance-role credentials.
4. External network defaults to deny.
5. Peer communication defaults to deny.
6. Git/model credentials are short-lived or brokered.
7. A session cannot request arbitrary Substrate egress destinations.
8. Mantle never reports confinement that Substrate did not prove.
9. Runtime identity is immutable/digest-based.
10. Source exact commits are recorded.
11. A terminal attachment is authorized independently of session existence.
12. Worker loss is never reported as a successful agent exit.
13. Unknown execution outcome remains unknown.
14. Cross-session shared caches must not become writable ambient host authority.

---

## 51. Threat scenarios

### Malicious repository build script

It runs inside the same session confinement.

It can access only the session's declared capabilities.

### Prompt injection asks agent to read another session

No filesystem authority exists.

### Prompt injection asks agent to curl production

Egress gateway denies undeclared destination.

### Agent tries `/var/run/docker.sock`

Host Unix sockets are unavailable.

### Agent probes EC2 metadata

Sandbox has no host/VPC network path.

### Agent steals long-lived GitHub token

No long-lived token should exist in the workspace.

### One build fork-bombs

PID and memory/CPU controls contain it.

### One session fills disk

Workspace quota must bound it where hard quota capability is required.

---

## 52. Failure semantics

### Worker unreachable

Session state:

```text
UNREACHABLE
```

not:

```text
STOPPED
```

### PTY attachment lost

Agent/process may remain running.

### Agent process exited

Substrate observation provides terminal outcome.

### Coordinator crashes

Rebuild state by reconciling:

- stored session records,
- AWS worker inventory,
- Substrate observations.

### AWS says worker terminated

Mark session lost unless a restorable snapshot exists.

### Network policy service unavailable

Fail closed for new egress.

Do not open internet as fallback.

---

## 53. Runtime adapter structure

```text
mantle-runtime
├── generic
├── claude-code
├── rust
├── ros2
└── cpp
```

An agent adapter is not a cloud provider.

A runtime profile is not an agent.

Example:

```text
runtime: rust-dev
agent: claude-code
```

could later become:

```text
runtime: rust-dev
agent: codex
```

without changing placement or Substrate.

---

## 54. Repository layout proposal

New repository:

```text
beyond10x/mantle
```

Possible Rust workspace:

```text
crates/
├── mantle-domain/
├── mantle/
├── mantle-cli/
├── mantle-state/
├── mantle-substrate/
├── mantle-provider-local/
├── mantle-provider-aws/
├── mantle-runtime/
├── mantle-agent-claude/
├── mantle-egress/
├── mantle-relay/
└── mantle-testkit/
```

Do not create all crates immediately.

Preserve logical boundaries first.

### Minimum implementation layout

Initial repository can be:

```text
src/
├── domain/
│   ├── session.rs
│   ├── worker.rs
│   ├── manifest.rs
│   └── placement.rs
├── app/
│   ├── start_session.rs
│   ├── attach_session.rs
│   └── stop_session.rs
├── adapters/
│   ├── substrate.rs
│   ├── aws.rs
│   ├── local.rs
│   └── state.rs
├── runtime/
│   └── claude.rs
└── main.rs
```

Split only as interfaces stabilize.

---

## 55. Dependency rule

A build check should enforce:

```text
Substrate has no Mantle dependency.
```

Mantle may depend on:

```text
b10x-substrate-sdk
substrate contract artifacts
```

using the distribution/pinning pattern Substrate itself defines.

No source import from internal Substrate implementation crates unless explicitly adopting its in-process consumer seam for local-only mode.

Remote mode should exercise the public daemon contract.

---

## 56. Substrate changes prioritized for Mantle

### P0 — no Substrate change

Enough for a vertical slice:

- one workspace,
- confined PTY,
- current workspace/file APIs,
- existing TLS/WSS,
- current secret slot,
- one aperture to Mantle egress gateway.

### P1 — Runtime Image resource

Required for a polished, reproducible development environment.

### P2 — multi-source workspace materialization

Removes the initializer workaround.

### P3 — Endpoint resource

Clean session bus / credential broker / local services.

### P4 — Volume/cache resource

Safe persistent/shared cache composition without arbitrary host mounts.

### P5 — Workspace snapshot/export

Required before robust migration and Spot.

These are generic execution-data-plane resources and should remain free of Mantle policy.

---

## 57. Things that should NOT be added to Substrate

Do not add:

- AWS instance selection,
- EC2 APIs,
- Claude session semantics,
- Cargo knowledge,
- ROS knowledge,
- GitHub branch publication policy,
- peer topology rules,
- idle-cost policy,
- user billing,
- agent coordination.

Those belong in Mantle or another consumer.

---

## 58. Testing strategy

Three suites.

### Domain tests

No OS/network.

Test:

- placement decisions,
- lifecycle transitions,
- manifest validation,
- capability resolution,
- recovery decisions.

### Adapter contract tests

Test:

- AWS provider emulator/fakes,
- Substrate public contract,
- Identity authority flow,
- runtime adapters,
- egress policies.

### Live conformance tests

Provision real Linux environments and assert:

- actual cgroup confinement,
- actual PTY behavior,
- actual network denial,
- actual egress policy,
- actual Git materialization,
- actual multi-session isolation.

A mocked sandbox is not evidence of isolation.

---

## 59. Critical end-to-end tests

### Isolation

Session A writes a secret file.

Session B attempts:

- guessed path,
- `/proc`,
- symlink traversal,
- socket access.

Must fail.

### Network

Agent can reach:

```text
api.anthropic.com
github.com
crates sources
```

only when the manifest allows the respective capability.

A random internet host fails.

### CPU offload

Run 10 builds.

Laptop CPU remains near terminal/client baseline.

### Hot rebuild

Build same repo twice with one-line edit.

Second build uses persistent remote state.

### Concurrent builds

20 agent sessions compile simultaneously.

Worker remains within defined admission/resource policy.

### Detach

Detach local client during build.

Session lifecycle follows configured policy, not accidental client death.

### Reattach

Attach from a second terminal.

Same PTY/session identity.

### Worker death

Hard terminate VM.

Mantle reports lost/unreachable honestly.

After snapshot support, restore and prove source state.

---

## 60. Performance benchmarks

For Rust:

```text
cold session start
first cargo check
one-line edit cargo check
dependency edit cargo check
10 sessions parallel
20 sessions parallel
```

For ROS:

```text
cold colcon build
one-package edit
shared-package edit
parallel workspaces
```

Metrics:

- local CPU,
- remote CPU,
- remote RAM,
- p50/p95 command latency,
- start latency,
- cache hit,
- disk growth,
- bytes egressed,
- cost/hour,
- cost per completed build.

---

## 61. Milestone plan

### M0 — prove remote whole-session UX

- Mantle CLI skeleton.
- Static Linux VM.
- Current Substrate.
- One repository.
- PTY start/attach.
- Claude Code process remote.
- No fancy caching.
- No multi-worker.
- No Spot.

**Exit:** user can spend a day developing remotely and forget the process is not local.

### M1 — one EC2 worker

- AWS provider.
- On-Demand EC2.
- SSM tunnel.
- Substrate production TLS/WSS.
- EBS workspace storage.
- session list/attach/stop.
- measured build performance.

**Exit:** local machine stays cool during many builds.

### M2 — development capabilities

- Mantle egress gateway.
- Git credential flow.
- local dirty-state handoff.
- multi-repo initializer.
- runtime profile.
- Claude adapter hardening.
- peer bus MVP.

**Exit:** normal multi-repo work needs no manual SSH/setup.

### M3 — Substrate product-neutral extensions

- Runtime Image.
- multi-source workspace.
- Endpoint.
- Volume/cache resource as justified by measurement.

**Exit:** initializer/proxy workarounds shrink substantially.

### M4 — cache + density

- compiler-cache benchmark,
- worker resource scheduler,
- many sessions per host,
- runtime/cache locality placement.

**Exit:** aggregate 20-session throughput is predictably higher than local baseline.

### M5 — durability and fleet

- workspace snapshots,
- multi-worker placement,
- failure restoration,
- autoscaling.

**Exit:** sessions survive normal worker replacement without source loss.

### M6 — Spot

- drain,
- snapshot/checkpoint,
- restore elsewhere,
- capacity rebalance.

**Exit:** Spot loss is routine, not exceptional.

---

## 62. First concrete experiment

Do **not** start by building the AWS fleet.

Use one Linux VM with enough CPU.

1. Install the current signed/reviewed Substrate daemon.
2. Configure real cgroup delegation and its required runtime probes.
3. Start Substrate with a workspace root on a large filesystem.
4. Build a tiny Mantle CLI using `b10x-substrate-sdk`.
5. Create one workspace.
6. Launch a PTY shell.
7. Launch Claude Code.
8. clone/materialize one Rust repository.
9. run `cargo check`.
10. detach.
11. reattach.
12. start five more sessions.
13. build concurrently.
14. inspect Substrate applied resource observations.

Only after this works should AWS enter the picture.

---

## 63. First AWS experiment

Use:

```text
one On-Demand EC2 instance
one EBS workspace volume
SSM enabled
no inbound SG rules
no Auto Scaling
no Spot
```

Test through an automated SSM tunnel.

The same Mantle CLI should see only a provider change:

```text
local-vm
→
aws-static
```

No session-domain code should change.

---

## 64. Why not keep Claude local?

A local Claude process with remote filesystem/process/network would require virtualizing almost its whole Unix environment:

```text
filesystem RPC
process RPC
network proxy
socket proxy
credential forwarding
cwd/path mapping
watchers
signals
PTY
```

It would also reintroduce network-filesystem latency and subtle split-brain behavior.

Running the whole session remotely gives the agent normal local Unix semantics.

The local client only needs a control/terminal protocol.

This is a much smaller distributed system.

---

## 65. Why not one VM per session?

Because idle agent sessions are common and expensive.

Substrate already exists to confine multiple workloads on one Linux host.

The intended model is:

```text
many sessions
   ↓
fewer large workers
```

Scale worker count when:

- CPU admission is saturated,
- memory is saturated,
- storage is saturated,
- isolation policy requires dedicated placement.

Not because session count equals VM count.

---

## 66. Why not Kubernetes first?

It solves a different layer.

Mantle needs:

- session semantics,
- source composition,
- PTY attachment,
- capability resolution,
- developer UX.

Substrate already has the key host boundary.

Start on EC2 hosts.

Kubernetes can later be a Mantle worker provider or a Substrate deployment/driver when its own gated track is ready.

Do not make it an initial dependency.

---

## 67. Why this is useful beyond current development

The same primitives later support the software factory.

Today:

```text
human
  ↓
Mantle
  ↓
Substrate session
  ↓
Claude Code
```

Later:

```text
software factory
  ↓
Mantle session API
  ↓
Substrate session
  ↓
agent runtime
```

The user-facing controller can change.

The data plane remains Substrate.

This is a better convergence than building a one-off remote-build system.

---

## 68. Recommended immediate decisions

1. Use **Mantle** as the working name.
2. Make **whole-session remote placement** the primary architecture.
3. Keep **Substrate unchanged for the first vertical slice**.
4. Use the **public Substrate daemon/SDK seam**, not internal implementation coupling.
5. Test on **one Linux VM first**.
6. Then use **one On-Demand EC2 worker with EBS**.
7. Use **SSM tunnel + Substrate TLS/WSS** initially.
8. Build **one egress gateway aperture** rather than broadening sandbox internet.
9. Treat **Runtime Image** as the first major Substrate extension.
10. Keep **fleet scheduling in Mantle**.
11. Delay **Spot** until workspace snapshot/restore exists.
12. Make **peer communication an explicit capability**, never ambient host networking.
13. Keep the manifest **tool-agnostic**; Claude Code is just the first adapter.
14. Measure before selecting sccache/kache/cache architecture.

---

## 69. Key design conclusion

The core abstraction is:

> **A Mantle session is a portable confined Unix development environment. Mantle decides what the session is and where it runs. Substrate decides what that host can safely execute and reports what actually happened.**

The laptop becomes a control surface.

The remote session gets:

```text
normal filesystem semantics
normal process semantics
normal local compilation
confined network
confined credentials
confined resources
explicit peer channels
```

Moving from local to EC2 is a placement change, not a workflow rewrite.

---

## 70. References reviewed

### beyond10x Substrate

Repository:

https://github.com/beyond10x/substrate

Current README / execution boundary:

https://github.com/beyond10x/substrate/blob/main/README.md

Current status:

https://github.com/beyond10x/substrate/blob/main/STATUS.md

Roadmap:

https://github.com/beyond10x/substrate/blob/main/ROADMAP.md

Public docs:

https://beyond10x.github.io/docs/substrate/

The design above specifically relies on current documented facts including:

- one-host execution-data-plane boundary,
- confined workspaces,
- bounded process/session execution,
- PTY and raw pipes,
- durable observations and leases,
- TLS 1.3 HTTPS/WSS remote SDK transport,
- authorized HTTPS Git source support,
- sealed secret slots,
- default no-egress plus named destination aperture,
- `/runtime` read-only and `/workspace` writable confinement,
- product/fleet policy remaining outside Substrate,
- general workspace backup/restore snapshots currently being absent.

### AWS

Systems Manager:

https://docs.aws.amazon.com/systems-manager/

EC2 instance store:

https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/storage-instance-store.html

EBS:

https://docs.aws.amazon.com/ebs/

Auto Scaling / Spot should be introduced only after the durability milestones above.

---

# Appendix A — Example first manifest

```yaml
apiVersion: mantle.beyond10x.dev/v1alpha1
kind: Session

metadata:
  name: substrate-caching

placement:
  class: aws-dev-large
  region: eu-central-1

runtime:
  profile: rust-dev

workspace:
  repositories:
    - name: substrate
      repository: https://github.com/beyond10x/substrate.git
      ref: main
      mount: substrate

resources:
  cpu: 16
  memory: 48GiB
  pids: 4096
  storage: 200GiB

network:
  default: deny
  capabilities:
    - model.anthropic
    - git.github
    - rust.crates

agent:
  kind: claude-code
  cwd: /workspace/substrate

lifecycle:
  detachKeepsRunning: true
  idleAfter: 30m
  retainFor: 8h
```

---

# Appendix B — Example multi-repo manifest

```yaml
apiVersion: mantle.beyond10x.dev/v1alpha1
kind: Session

metadata:
  name: software-factory

placement:
  class: aws-dev-xlarge

runtime:
  profile: rust-dev

workspace:
  repositories:
    - name: substrate
      repository: https://github.com/beyond10x/substrate.git
      ref: main
      mount: substrate

    - name: connectors
      repository: https://github.com/beyond10x/connectors.git
      ref: main
      mount: connectors

    - name: app
      repository: https://github.com/example/application.git
      ref: feature/foo
      mount: app

resources:
  cpu: 32
  memory: 96GiB
  pids: 8192
  storage: 400GiB

network:
  default: deny
  capabilities:
    - model.anthropic
    - git.github
    - rust.crates
    - apt.ubuntu

peers:
  allow:
    - selector: project

agent:
  kind: claude-code
  cwd: /workspace/app
```

---

# Appendix C — Proposed Substrate extension summary

```text
CURRENT SUBSTRATE
────────────────────────────────────────────
Workspace
Exec
Raw-pipe session
PTY session
File API
Git source
Secret slot
Single pinned egress aperture
Durable ops / leases / observations
TLS/WSS remote transport

MANTLE CAN START HERE
────────────────────────────────────────────
single repo
Claude PTY
one EC2 worker
SSM tunnel
egress gateway
persistent workspace

PROPOSED GENERIC SUBSTRATE EXTENSIONS
────────────────────────────────────────────
Runtime Image resource
Multi-source workspace materialization
Endpoint resource
Volume/cache resource
Workspace snapshot/export
```

The extensions are valuable only if implemented with Substrate's existing fail-closed and observed-facts discipline.

---

# Appendix D — Architecture invariant checklist

Before merging a Mantle architectural change, ask:

- Does this belong to session/product policy? → Mantle.
- Does this describe a generic host execution primitive? → possibly Substrate.
- Does Substrate need to know "Claude", "AWS", "Cargo", or "GitHub workflow"? → probably wrong layer.
- Does the session receive ambient authority? → redesign.
- Is a requested capability being confused with an applied fact? → redesign.
- Can the user detach without changing the execution outcome? → should be yes.
- Can a worker die without Mantle inventing a successful result? → must be yes.
- Can two sessions on one host access each other without explicit policy? → must be no.
- Does moving local → EC2 change the manifest semantics? → should be no.
- Does the design require a network-mounted development filesystem? → should be no.
