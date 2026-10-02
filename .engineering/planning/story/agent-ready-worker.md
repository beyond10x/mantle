---
format: aep.planning-md/3
id: story:agent-ready-worker
kind: story
status: draft
title: Provision an agent-ready worker without Claude credentials
relations:
- decomposes: epic:codex-parity
scope:
- confidence: cited
  path: Taskfile.yml
- confidence: inferred
  path: crates/mantle/src/adapters/conformance.rs
- confidence: cited
  path: crates/mantle/src/adapters/substrate.rs
- confidence: inferred
  path: crates/mantle/src/app/session.rs
- confidence: cited
  path: crates/mantle/src/app/worker.rs
- confidence: cited
  path: crates/mantle/src/config.rs
- confidence: inferred
  path: crates/mantle/tests/agent_worker.rs
- confidence: cited
  path: deploy/cloud-init.yaml
- confidence: cited
  path: deploy/substrate.service
- confidence: cited
  path: examples/config.toml
- confidence: inferred
  path: generated/
- confidence: inferred
  path: spec/components.yaml
- confidence: cited
  path: spec/domains/session.yaml
- confidence: inferred
  path: spec/ess-inputs.yaml
- confidence: inferred
  path: spec/scenarios/agent-worker.yaml
revision: 16
---
## Context

Worker bootstrap currently installs Claude only; Config requires [claude], missing_facts requires its slot, and substrate.service cannot start without its token file. A Codex-only installation therefore fails before agent selection is relevant.

## Acceptance

Given a fresh KubeVirt worker configuration with no Claude credential, worker up reaches credential-independent readiness with the verified Codex candidate installed, as demonstrated by the AW scenario set.

## Named conformance scenarios

- AW-01: no Claude config/secret is required to provision a Codex-capable worker; READY means confinement/toolchain ready, not authenticated agent.
- AW-02: fresh provisioning installs the exact prebuilt Codex version/digest, records it in bootstrap facts, and refuses checksum/version mismatch or unsupported architecture before advertising capability.
- AW-03: re-running worker up installs an additional agent without disrupting an existing Claude session; failed/partial download preserves the previous valid binary and capability.
- AW-04: legacy Claude configuration remains parseable and can provision its current worker. Agent-specific readiness reports the missing executable/credential only when that agent is selected.
- AW-05: installing binaries requires no Substrate source compilation. Shared EC2 and KubeVirt bootstrap rendering includes the same version evidence; live EC2 execution is not claimed.

## Implementation boundaries

Separate common readiness from per-agent readiness, retaining existing fact checks for confinement. Keep Claude's secret-slot route; do not invent an empty Claude token to satisfy the unit. Move token installation and optional service-slot configuration to explicit selected-agent setup while keeping worker up useful without a secret. The later start story owns the concrete selected Codex auth flow. Preserve the service's existing Claude slot when installed and avoid restarting the shared daemon during active sessions; if adding a slot requires restart, report the prerequisite rather than disrupting sessions.

Use 0.153.4 as the initial candidate, verified against official release metadata before implementation; qualification may reject it, so the final runtime pin requires CQ results. Declare actual available versions in worker status. No silently copied host binary.

Extend the existing Worker ESS contract with provisioning outcomes/views, author AW scenarios and synthesize a Rust implementation/conformance adapter before claiming this behavior. Add a bounded offline conformance gate to task check; the real worker run is separate evidence. Do not assert that a reference-model run tested the worker.

## Scope

Derived 2026-10-02 by `story-scoper`; this is a future implementation unit after the overlapping ESS work is reconciled — inferred.

- **Primary surface:** `crates/mantle/src/app/worker.rs` — cited; `up`, `install_binaries`, `report_machine`, `status` and `render_user_data` own provisioning, readiness, installation and version reporting.
- **Configuration:** `crates/mantle/src/config.rs`, `examples/config.toml` — cited; `Config.claude` is currently required and `claude_token` supplies the credential.
- **Readiness:** `crates/mantle/src/adapters/substrate.rs` — cited; `missing_facts` combines confinement checks with the required Claude secret slot.
- **Selected-agent caller:** `crates/mantle/src/app/session.rs` — inferred; its existing calls to `claude_token`, `missing_facts` and `install_token` must retain Claude-specific checks when worker readiness becomes credential-independent.
- **Bootstrap and service:** `deploy/cloud-init.yaml`, `deploy/substrate.service` — cited; bootstrap installs Claude and records its version, while the service requires and declares its token slot.
- **ESS contract and gate:** `spec/domains/session.yaml`, `Taskfile.yml` — cited; AW requires Worker provisioning outcomes/views and an offline implementation conformance gate.
- **AW scenario and test paths:** `spec/scenarios/agent-worker.yaml`, `crates/mantle/tests/agent_worker.rs` — inferred; reserved locations from the existing scope, with test placement to be confirmed against the reconciled conformance harness.
- **ESS integration after reconciliation:** `spec/ess-inputs.yaml`, `spec/components.yaml`, `crates/mantle/src/adapters/conformance.rs` — inferred; the active ESS tree introduces explicit scenario inputs, component command ownership and a Rust implementation adapter that AW may need to extend.
- **Generated outputs:** `generated/` — inferred; retained reservation only, with exact output paths unresolved and required before implementation dispatch.
- **Documents:** the configuration example above; no additional documentation surface established — inferred.
- **Confidence:** medium — inferred; production ownership is directly located, but the incoming ESS contract and AW adapter/output placement are not reconciled.
- **Would collide with:** worker/config/session readiness changes, bootstrap/service changes, and ESS contract/conformance integration; the active `spec/mantle-ess-contracts` tree already changes the contract and gate and introduces the likely ESS integration files above — cited.
- **Scheduling boundary:** reconcile the active ESS work, select exact AW adapter/generated paths and refresh machine-readable scopes before dispatching this story; it can then be a separate future unit from qualification, while acceptance of its runtime candidate remains conditional on CQ results — inferred.
- **Safety fact:** the current readiness helper is shared by worker readiness and session start; removing its Claude-slot check without retaining selected-agent validation would change both callers (`crates/mantle/src/app/worker.rs:335`, `crates/mantle/src/app/session.rs:78`) — cited; proof level 2, unproven.
- **Safety fact:** re-running provisioning currently calls `install_binaries`, which unconditionally restarts the shared egress service (`crates/mantle/src/app/worker.rs:274`); AW-03 must address that disruption risk as well as avoiding a Substrate restart — cited; proof level 2, unproven.

## Pinned daemon readiness evidence

Pinned Substrate source `05695970b069f79e6678f2f02cbd78bbe5fa2a56`,
`crates/substrate-daemon/src/runtime.rs:554,745-776`, checks each declared secret-slot file
at daemon startup: it must exist, be nonempty, bounded, regular, workload-owned and private.
Therefore simply removing systemd ConditionFileNotEmpty while retaining an absent `--secret-slot`
cannot satisfy AW-01. `crates/substrate-host/src/secrets.rs:384-400` advertises configured names
after sealing/descriptor probes; it does not make a configured missing file optional.

Implementation consequence (inference): a credential-free daemon configuration must omit the
Claude slot. Installing a real Claude slot later needs an explicit daemon configuration change;
preserve an already-installed slot and refuse a disruptive restart while another session is live.
Never create an empty or dummy credential. The existing selected-agent start path must keep
Claude-specific credential checks when common worker readiness stops requiring that slot.
