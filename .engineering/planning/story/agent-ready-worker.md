---
format: aep.planning-md/3
id: story:agent-ready-worker
kind: story
status: implemented
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
- confidence: cited
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
- confidence: cited
  path: spec/conformance-baseline.json
- confidence: cited
  path: spec/domains/orchestration.yaml
- confidence: cited
  path: spec/domains/session.yaml
- confidence: cited
  path: spec/ess-inputs.yaml
- confidence: cited
  path: spec/scenarios/cli/agent-worker.yaml
- confidence: cited
  path: spec/scenarios/cli/orchestration-observe-common-without-claude.yaml
revision: 52
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T10:13:54Z", actor: "human:timo", revision: 24}
- {from: "proposed", to: "active", at: "2026-10-02T10:13:54Z", actor: "human:timo", revision: 25}
- {from: "active", to: "implemented", at: "2026-10-02T13:00:17Z", actor: "human:timo", revision: 52, decided_on: {"recorded":{"test_result":4,"review_outcome":2,"verification":3}}}
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

## Scope

All31 changed paths in final unit dc71f10d32451f9ed57365685b702c4c5ee8c6a5 match the typed scope exactly, with cited confidence. Coordinator comparison against b3a9c456 found no missing or extra scope paths; the final tunnel correction touches only three already-owned SSH/Cargo paths. Scope confirmation sources are the initial implementation, correction1, rebase reconciliation and tunnel-correction reports under .engineering/reports/.

- Worker/session orchestration, capability mapping, configuration and explicit config/state paths retain their confirmed ownership. Common readiness excludes Claude; selected Claude checks credential/executable/slot before workspace creation. Native Observe preserves the legacy selected-Claude default and adds common-only capability observation.
- Rust worker library/CLI, bounded SSH delivery, startup cancellation handling and private ephemeral tunnel allocation are confirmed. The last correction leaves persistent state/key paths unchanged. Both independent-review findings are resolved; all retained assertions remain. Fixture isolation resolves the observed inherited-writer ETXTBSY mechanism without a production retry or weakened installer-concurrency test.
- Cargo, deploy, example configuration and Taskfile ownership is confirmed. Correction to initial inference: build-worker includes the new helper; plain cargo fmt --check selects all six workspace packages while generated-byte drift independently checks the excluded generated crate.
- Native ESS paths retain all304 incoming scenario names and add9, giving313 complete scenarios. The worker sequence's final path is spec/scenarios/cli/agent-worker.yaml, superseding its original root-level placement; the common-only case is spec/scenarios/cli/orchestration-observe-common-without-claude.yaml. No generic mantle-conformance runner edit was needed. All218 authored scenarios reject the no-op target.
- Exactly four portable worker-model outputs are committed. Operational .ess-output state remains ignored, correcting the initial five-file generated scope; provenance regeneration and drift verification passed.

The rebase additions were inferred when scheduled, then changed to cited from actual diff and execution. Interactive-start remains serialized against these worker/session/configuration/ESS/adapter surfaces. No Substrate source change is part of this unit. Live AW acceptance now passes on fresh and existing workers; final integration gate results are recorded separately.

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

## Live acceptance tunnel-path correction

Existing-worker live acceptance succeeded: first worker up exit0 Installed0.153.4, repeat exit0 AlreadyCurrent; observed old service PIDs/start times/shared hashes unchanged and Claude exec still running. Fresh worker21c01ed2-df0f-4ff1-a79a-0c397c186073 in namespace mantle-wave3-acceptance bootstrapped, rebooted into the required kernel and installed the same candidate, but worker up exited1 at its final local SSH tunnel. Exact source: scratch-int/fresh-worker-up.log; OpenSSH rejected the local forwarding specification because the long isolated state directory plus worker UUID/PID socket name exceeds the Unix-socket path limit. Fresh AW-01 is NOT VERIFIED until the same CLI completes after correction.

Coordinator authorized a bounded correction in already-owned SSH/Cargo surfaces: allocate a unique short private ephemeral socket directory independently of persistent state/key paths; retain it for Tunnel lifetime and clean it after child cleanup. A standard secure temporary directory under an explicit short root is acceptable; ambient TMPDIR must not recreate the length defect. Reproduce the actual allocation/bind failure before fixing, preserve all existing assertions, and cover path length, delimiter/nonASCII inputs, concurrency, directory privacy and cleanup. No state/key relocation, fixture shortening or VM recreation. The fresh provisioned worker is retained for the retry.

This live-discovered defect followed two completed adversarial passes; it does not reset the attack budget. The coordinator reviews the correction diff and regression assertions and reruns relevant gates/live acceptance. No third full adversary is authorized or required. Add exact native ESS scope if needed before editing outside the current paths.

## Live acceptance and tunnel correction verified

## Live acceptance and tunnel correction verified

The exact fresh VM21c01ed2-df0f-4ff1-a79a-0c397c186073, no-Claude configuration and original long MANTLE_STATE_DIR were retained. Initial up installed Codex but exited1 at local SSH forwarding; the actual allocation/bind regression reproduced that failure before the correction. Socket allocation now uses unique private0700 directories below explicit /tmp, retains them until SSH cleanup and leaves persistent state/key paths unchanged. Three retained regressions cover the original length, unusual state/TMPDIR paths, concurrent allocations, modes/ownership and success/error cleanup. No third adversarial pass occurred; the coordinator inspected the narrow correction and all assertions.

Corrected CLI SHA2560762c0584d9c1043d7f398f06ca4ee2712c514e54b8e7c219a89b7d667bb4390 completed the same fresh-worker up with exit0, AlreadyCurrent, coherent version/archive/binary digests and common READY. Root tested Claude token absence; running service command and machine facts expose no Claude slot. Selected Claude start separately exited1 with its actionable missing-config message and zero session rows. No auth/model call is claimed. Sources: codex-worker-fresh-up-2026-10-02.log, codex-worker-fresh-install-2026-10-02.txt, codex-worker-fresh-machine-2026-10-02.json and codex-worker-tunnel-correction-2026-10-02.md under .engineering/reports/.

Existing-worker up exited0 Installed, then exit0 AlreadyCurrent, while exact daemon/gateway PIDs/start times, shared executable hashes and running Claude exec remained unchanged. Changed shared binaries were reported deferred. Sources: existing-before-up.txt, existing-after-up.txt, existing-worker-up.log, codex-worker-existing-repeat-2026-10-02.log and codex-worker-live-claude-preservation-2026-10-02.json under .engineering/reports/.

The complete final worker unit is dc71f10d32451f9ed57365685b702c4c5ee8c6a5, merged by bot as247ed1715ac24bba0c4188354d5aac3c02d6c26a. Both author and committer were verified. Offline correction gates report167 Rust tests and313 complete native scenarios. The live AW claim is VERIFIED; closing integration gates remain a separate required observation.
