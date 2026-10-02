---
format: aep.planning-md/3
id: story:agent-ready-worker
kind: story
status: active
title: Provision an agent-ready worker without Claude credentials
relations:
- decomposes: epic:codex-parity
scope:
- confidence: cited
  path: .gitignore
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Cargo.toml
- confidence: cited
  path: Taskfile.yml
- confidence: cited
  path: crates/mantle-worker/Cargo.toml
- confidence: cited
  path: crates/mantle-worker/src/lib.rs
- confidence: cited
  path: crates/mantle-worker/src/main.rs
- confidence: cited
  path: crates/mantle/Cargo.toml
- confidence: cited
  path: crates/mantle/src/adapters/conformance.rs
- confidence: inferred
  path: crates/mantle/src/adapters/orchestration.rs
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
- confidence: cited
  path: generated/worker-model/Cargo.toml
- confidence: cited
  path: generated/worker-model/source.schema.json
- confidence: cited
  path: generated/worker-model/types-report.json
- confidence: cited
  path: generated/worker-model/types.rs
- confidence: cited
  path: spec/README.md
- confidence: cited
  path: spec/components.yaml
- confidence: inferred
  path: spec/conformance-baseline.json
- confidence: inferred
  path: spec/domains/orchestration.yaml
- confidence: cited
  path: spec/domains/session.yaml
- confidence: cited
  path: spec/ess-inputs.yaml
- confidence: inferred
  path: spec/scenarios/cli/agent-worker.yaml
- confidence: inferred
  path: spec/scenarios/cli/orchestration-observe-common-without-claude.yaml
revision: 46
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T10:13:54Z", actor: "human:timo", revision: 24}
- {from: "proposed", to: "active", at: "2026-10-02T10:13:54Z", actor: "human:timo", revision: 25}
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

Scope reconciled from the implementor's confirmation table and the coordinator's staged-diff inspection on 2026-10-02. This records observed source ownership, not completion; adversary corrections and live acceptance remain pending.

- **Worker orchestration and selected readiness:** `crates/mantle/src/app/worker.rs`, `app/session.rs`, `adapters/substrate.rs` — cited; common readiness no longer requires Claude, selected Claude retains token/executable/slot checks, shared active upgrades defer individually, and independent installation continues.
- **Configuration and explicit isolation:** `crates/mantle/src/config.rs`, `examples/config.toml` — cited; optional Claude table and explicit absolute config/state paths, preserving ordinary defaults.
- **Bounded transport:** `crates/mantle/src/adapters/ssh.rs`, `crates/mantle-worker/src/lib.rs` — cited; concurrent bounded IO and owned process-group cleanup. Attack1 found cancellation and first-directory concurrency defects; review-result:codex-worker-adversary-1 records them. Ownership is confirmed, correctness still requires correction evidence.
- **Installer library and CLI:** `crates/mantle-worker/Cargo.toml`, `src/lib.rs`, `src/main.rs` — cited; previously inferred new placement now exists and implements the single pinned installer shared by both providers. No second cloud-init installer.
- **Workspace integration:** `Cargo.toml`, `Cargo.lock`, `crates/mantle/Cargo.toml`, `crates/mantle/src/main.rs`, `Taskfile.yml` — cited. Correction to original scoping: Taskfile does change because build-worker must include the new helper; existing cargo-test gate already runs conformance.
- **Bootstrap and service:** `deploy/cloud-init.yaml`, `deploy/substrate.service` — cited; optional Claude slot, daemon start after gateway/helper and optional real token, and preserved aperture/CA arguments.
- **ESS and real adapter:** `spec/domains/session.yaml`, `spec/components.yaml`, `spec/ess-inputs.yaml`, `spec/scenarios/agent-worker.yaml`, `spec/README.md`, `crates/mantle/src/adapters/conformance.rs` — cited. New authored sequence is actually selected by synthesis. WorkerRecord remains a local record, not invented remote liveness.
- **Generated portable values:** `generated/worker-model/Cargo.toml`, `types.rs`, `source.schema.json`, `types-report.json` — cited; actual generated crate with path dependency and byte-drift test. Previously inferred placement is confirmed.
- **Generated operational state:** `.gitignore` — cited. Correction to the provisional generation assumption: `.ess-output/state.json` contains machine/location/inode metadata and is ignored, not committed. The typed source scope contains only the four portable outputs above.
- **Tests:** existing owned Rust modules and conformance adapter — cited; no provisional `crates/mantle/tests/agent_worker.rs` or broad `generated/` reservation remains.
- **Confidence:** high for source placement; live AW behavior remains unverified until actual worker runs. Sources: scratch-worker/implementor-report.md section1 confirmation table, staged patch SHA256 `5950cc2b18602be4765305e4f4e52ef4301221ad859abed596c7264155517d83`, and immutable attack1 review.
- **Collision boundary:** configuration, worker/session readiness, bootstrap/service, session ESS domain and conformance adapter remain serialized with interactive-start. No scope claim authorizes a Substrate change.

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

## Isolated operator paths

The actual CLI must support explicit MANTLE_CONFIG (configuration file) and MANTLE_STATE_DIR
(private local state directory) environment overrides, preserving current defaults. This lets
AW-01 run with an isolated no-Claude config and separate KubeVirt namespace, database and SSH
key without changing the operator's HOME or existing session state. The coordinator selected
this narrow implementation detail from the existing Config::load/state_dir hardcoded paths;
source ownership remains the already assigned config.rs/main.rs and example/help surfaces.
No global environment mutation or credential copying belongs to this change. Test default and
explicit paths through pure selection or subprocess boundaries.

## Generated model output correction

Implementation observed ESS0.50 .ess-output/state.json is an operational ownership envelope:
root components, device/inode, random anchor, sequence and checksum. It is not a portable
model artifact. The coordinator inspected its actual JSON and corrects the earlier five-file
scope: commit only Cargo.toml, types.rs, source.schema.json and types-report.json, all generated
unchanged. Add .gitignore ownership for generated/**/.ess-output/; leave local generated state
untouched and ignored. Regeneration drift compares every byte of the four portable artifacts
from a fresh output directory. Any operational-envelope validation uses that freshly produced
state, not byte equality with another location. The initial new state-byte-equality test was
invalid and its red evidence is retained; correcting it does not waive model drift checking.

## Rerun with newly built shared artifacts

AW-03's ordinary workflow supplies the current task build-worker output, whose shared-binary
digests can differ from the running deployment even while only adding Codex. Defer/refuse the
changed shared upgrade means leave those active bytes and processes untouched, print which
upgrade requires maintenance, and continue the independent helper/Codex installation. Do not
refuse all worker up solely because an unrelated shared upgrade was deferred, and do not claim
that deferred bytes were installed. This worker story's installer needs no changed launcher or
gateway; a demonstrated dependency would require an explicit prerequisite instead. Primary live
AW-03 evidence uses freshly built artifacts and observes preserved live process identities plus
new Codex capability; an exact-old-binaries fixture alone is narrower than the normal workflow.

Source: coordinator inspection of worker.rs install_binaries during implementation, which
currently bails before helper upload on any active shared digest difference. This clarification
preserves the original acceptance and makes its ordinary caller explicit.

## Rebase onto the complete native ESS sweep

Operator request2026-10-02: "btw, make sure you rebae from main, we did a bigger ESS sweep in case you missed it". Fresh git fetch advanced origin/main from2e6b116 to a7b26668a20dce232c5e068989b6818bdefa4171. The incoming commit is bot-authored and bot-committed.

The sweep changes production worker/session orchestration seams, native component adapters and the complete inventory gate. Published baseline is304scenarios (CLI204,egress66,launcher34), with zero failures/skips/unsupported/outside/refused; all216authored scenarios reject the no-op target. These are the incoming report's observations, not a run of this unrebased worker unit.

Preserve every incoming scenario and coverage floor. Rebase the wave onto a base containing this main, adapt the actual worker decision adapter to the native runner, extend the complete inventory and rerun the exact current gate. The earlier65-scenario worker run is historical and cannot stand in for that verification. Preserve all retained adversarial cases and both correction classes. Final full attack2 runs only on the reconciled source. No live acceptance or implementation completion is claimed until the rebased source and current gate pass.

Sources: git fetch/log/diff, incoming spec/README.md, spec/conformance-baseline.json, Taskfile.yml and crates/mantle-conformance. Reconciliation is within the approved wave; no push/tag/release is authorized.

## Native ESS reconciliation scope

Coordinator decision from the implementor's source inspection after the main rebase: preserve Observe's existing default meaning as selected-Claude capability observation. Add optional claude_selected input, absent meaning true; false requests common confinement capabilities. The common production helper remains agent-neutral; selected-Claude checks reuse the same mapping plus a separate executable check. This preserves every existing named scenario and expected response while adding a common-ready observation with no Claude slot. Observe must not invent an executable observation.

Additional paths, inferred until the adapted diff is checked: crates/mantle/src/adapters/orchestration.rs (real native adapter); spec/domains/orchestration.yaml (explicit optional command input); spec/scenarios/cli/orchestration-observe-common-without-claude.yaml (new authored case); spec/conformance-baseline.json (retain all existing names/floors and extend the actual complete inventory). Existing owned substrate/session/worker paths retain production behavior; no mantle-conformance runner change is anticipated. ESS command input changes precede adapter edits. This is a compatibility-preserving split of capability observations, not a deletion or relaxation of the missing-Claude-slot cases.

Source: wave3_worker_implementor's reconciliation report, incoming adapters/orchestration.rs:249-265, spec/scenarios/cli/orchestration-observe-missing-secrets-slots.yaml and the selected/common worker contract. Actual case counts, generated optional-witness changes and complete native gate remain to be measured. Correction1's 61 passing package tests and both fixed review outcomes were recorded against pre-rebase f8565cc; they are historical evidence only until this reconciliation is tested.

## Native worker scenario placement

The native runner selects authored CLI cases under spec/scenarios/cli. Move the unchanged agent-worker sequence from the provisional root-level spec/scenarios/agent-worker.yaml to spec/scenarios/cli/agent-worker.yaml and update the input manifest. The root-level placement would reach aggregate synthesis but not the CLI component, so it cannot pass complete-inventory reconciliation. This supersedes the old root-level path in the historical scope confirmation. Source: implementor inspection of the incoming native component runner during rebase reconciliation; selected path remains inferred until the actual diff and run confirm it.

## Rebase installer test diagnosis

The first full workspace run after rebase failed the retained interrupted-activation installer case with ETXTBSY and the generated provenance drift check (rebase-workspace-first.log, exit101). Regeneration addressed changed specification provenance. An unchanged isolated installer case and four later selected/full/traced probes passing did not erase the intermittent failure.

Ranked hypotheses from implementor inspection: (1) a sibling test's fork temporarily inherits another installer thread's writable staging descriptor; (2) the installer retains its own writer; (3) another installer writes the same inode. The local writer is explicitly dropped before execution; private staging and per-root locking oppose the latter two. A deterministic isolated Rust probe invoked actual run_bounded: it returned ETXTBSY after the parent closed its writer while a fork child retained that descriptor, and the same executable succeeded after that child exited. This proves the inherited-writer mechanism; attribution of the original intermittent scheduling interleaving remains inferred. Evidence: scratch-worker/rebase-inherited-writer.log and the retained trace/report.

Actual production topology is one synchronous Installer invocation in standalone mantle-worker main. Concurrent worker-up operations create independent helper processes and cannot inherit each other's writers. The parallel test process combines several independent installers with subprocess/cancellation fixtures. Coordinator authorized test-process isolation for each installer case, preserving all assertions and explicit concurrent-install threads within their isolated child. No production retry, sleep, widened deadline, blanket test serialization or weakened assertion is authorized. Rerun the original full suite and native aggregate after this correction; independent attack2 remains pending.
