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
  path: Cargo.lock
- confidence: cited
  path: Cargo.toml
- confidence: cited
  path: Taskfile.yml
- confidence: inferred
  path: crates/mantle-worker/Cargo.toml
- confidence: inferred
  path: crates/mantle-worker/src/lib.rs
- confidence: inferred
  path: crates/mantle-worker/src/main.rs
- confidence: cited
  path: crates/mantle/Cargo.toml
- confidence: cited
  path: crates/mantle/src/adapters/conformance.rs
- confidence: cited
  path: crates/mantle/src/adapters/ssh.rs
- confidence: cited
  path: crates/mantle/src/adapters/substrate.rs
- confidence: cited
  path: crates/mantle/src/app/session.rs
- confidence: cited
  path: crates/mantle/src/app/worker.rs
- confidence: cited
  path: crates/mantle/src/config.rs
- confidence: cited
  path: crates/mantle/src/main.rs
- confidence: cited
  path: deploy/cloud-init.yaml
- confidence: cited
  path: deploy/substrate.service
- confidence: cited
  path: examples/config.toml
- confidence: inferred
  path: generated/worker-model/.ess-output/state.json
- confidence: inferred
  path: generated/worker-model/Cargo.toml
- confidence: inferred
  path: generated/worker-model/source.schema.json
- confidence: inferred
  path: generated/worker-model/types-report.json
- confidence: inferred
  path: generated/worker-model/types.rs
- confidence: inferred
  path: spec/README.md
- confidence: cited
  path: spec/components.yaml
- confidence: cited
  path: spec/domains/session.yaml
- confidence: cited
  path: spec/ess-inputs.yaml
- confidence: inferred
  path: spec/scenarios/agent-worker.yaml
revision: 23
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

Derived 2026-10-02 by `story-scoper` against incoming ESS commit `2e6b116`; every entry distinguishes inspected source from proposed placement — cited.

- **Primary surface:** `crates/mantle/src/app/worker.rs` — cited; `up`, `install_binaries`, `install_token`, `report_identity`, `report_machine`, `render_user_data` and their tests own this behavior.
- **Configuration:** `crates/mantle/src/config.rs`, `examples/config.toml` — cited; make common provisioning independent of the required Claude table while preserving selected-token errors and legacy parsing.
- **Common and selected readiness:** `crates/mantle/src/adapters/substrate.rs`, `crates/mantle/src/app/session.rs` — cited; split shared `missing_facts` and preserve Claude slot validation before workspace creation.
- **Bootstrap and daemon:** `deploy/cloud-init.yaml`, `deploy/substrate.service` — cited; render credential-free versus selected-Claude startup, verified pinned candidate installation and observed bootstrap identity.
- **ESS contract:** `spec/domains/session.yaml`, `spec/components.yaml`, `spec/ess-inputs.yaml` — cited; incoming files own worker vocabulary, command ownership and explicit authored scenario inputs.
- **Authored AW scenarios:** `spec/scenarios/agent-worker.yaml` — inferred; new named obligations for readiness/provisioning decisions, with OS/live cases represented honestly as external evidence.
- **Actual adapter:** `crates/mantle/src/adapters/conformance.rs` — cited; extend the incoming adapter's dispatch, actual Rust observations, exact count/refusal accounting and scenario coverage.
- **Contract coverage documentation:** `spec/README.md` — inferred; incoming mapped/unmapped coverage frontier must explain the added decision seam and remaining live provisioning effects.
- **Tests and generated artifacts:** tests remain in the owned Rust modules and existing conformance adapter; synthesis remains in the existing ignored drafts directory — inferred; remove provisional `crates/mantle/tests/agent_worker.rs` and `generated/` reservations.
- **Gate:** existing workspace cargo-test already executes the conformance adapter; no Taskfile change required for this implementation design — cited.
- **Confidence:** high — cited; all production ownership and the incoming runner are inspected; only the authored AW scenario/documentation placement is proposed.
- **Would collide with:** worker/config/session readiness, bootstrap/service configuration, and session-domain/conformance input/adapter changes — cited.
- **Safety fact:** `worker.rs:269–274` restarts a shared gateway on every rerun, while `session.rs:78` shares the credential-dependent readiness function with worker readiness; both must be changed together before AW-01/AW-03 can pass — cited; proof level2, unproven by this read-only pass.
- **Safety fact:** incoming Worker is a local stored record and its specification explicitly excludes provider/daemon liveness; new observed readiness must not be inferred from that row — cited; proof level2, unproven by this pass.

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

## Rust installer boundary and scope refinement

The coordinator selects a small `mantle-worker` Rust/clap helper so both fresh and existing workers
use one bounded installation transaction after common bootstrap. No new embedded shell installer.
The helper verifies the pinned official archive and executable digests and exact version before
activation, rejects unsupported architecture before download, and preserves current binary/facts
on every failure. Publish a verified immutable generation containing binary and facts through one
atomic current pointer; a separate binary rename and unrelated facts rewrite would have a crash
window. No daemon/gateway restart belongs to this installer. Inspect reports verified installed
candidate identity, never authentication or completed support.

Additional inferred source: `crates/mantle-worker/Cargo.toml`, `src/lib.rs`, `src/main.rs`.
Additional cited integration ownership: workspace `Cargo.toml`, `Cargo.lock`,
`crates/mantle/Cargo.toml`, `crates/mantle/src/main.rs`, `crates/mantle/src/adapters/ssh.rs`,
and `Taskfile.yml` (static worker build list, correcting the earlier no-Taskfile estimate).
Worker SSH delivery needs bounded input/output/deadlines and owned-child cleanup, not the existing
unbounded accumulated-output helper. Only fixed bootstrap primitives deliver the Rust helper.

`AgentInstallationFacts` and `AgentInstallationOutcome` were first drafted as ESS value types and
validated; they do not add fictitious Worker database fields. A generator probe produced the exact
standalone Rust model layout with no runtime obligations. Generate, do not hand-copy, the two types:
`ess generate types --path spec --root mantle.session.AgentInstallationFacts --root mantle.session.AgentInstallationOutcome --target rust --package mantle-worker-model --out generated/worker-model`.
The five inferred generated paths are Cargo.toml, types.rs, source.schema.json, types-report.json,
and .ess-output/state.json. Keep the generated standalone crate excluded from the parent workspace,
use it as a path dependency, and verify regeneration without editing generated output. The model
and any extra decision contracts/scenarios precede their implementation. Actual readiness/installer
conformance calls real Rust decisions, with remote side effects verified separately.

AW-02's bootstrap facts mean coherent observed agent-installation facts beside the existing common
image/toolchain bootstrap record; status reads both. Cloud-init may describe the desired pin, but
must not call it installed or perform a second installer path. The exact host/worker run still must
prove fresh no-Claude readiness and non-disruptive installation beside the live Claude exec.

Source: `.engineering/reports/codex-worker-scope-2026-10-02.md`; these architecture choices are
coordinator inference from the actual installation and shared-service boundaries, selected under
standing wave approval. Dependencies and low-level API details remain implementor-confirmed.
