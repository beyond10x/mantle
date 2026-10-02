---
format: aep.planning-md/3
id: executable-system-specification:mantle-session
kind: executable-system-specification
status: validated
title: Mantle implemented contracts
summary: Five ESS domains; complete native conformance with zero unresolved mappings
relations:
- specifies: epic:vertical-slice
model_digest: 3a08777de0d97b8f4b6127d959d931d021cd3fbac3a529f3522372ec4169c4ea
revision: 21
transitions:
- {from: "draft", to: "validated", at: "2026-10-02T08:42:41Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"test_result":1}}}
- {from: "validated", to: "conforming", at: "2026-10-02T10:39:51Z", actor: "human:timo", revision: 9, decided_on: {"recorded":{"test_result":1,"ess_conformance_coverage_v1":1}}, executor: "agent:codex-mantle"}
- {from: "conforming", to: "validated", at: "2026-10-02T10:45:11Z", actor: "human:timo", revision: 10, decided_on: {"recorded":{"test_result":1,"ess_conformance_coverage_v1":1}}, executor: "agent:codex-mantle"}
- {from: "validated", to: "conforming", at: "2026-10-02T10:45:12Z", actor: "human:timo", revision: 13, decided_on: {"recorded":{"test_result":1,"ess_conformance_coverage_v1":1}}, executor: "agent:codex-mantle"}
- {from: "conforming", to: "validated", at: "2026-10-02T12:59:23Z", actor: "human:timo", revision: 14, decided_on: {"recorded":{"test_result":1,"ess_conformance_coverage_v1":1}}}
- {from: "validated", to: "conforming", at: "2026-10-02T12:59:47Z", actor: "human:timo", revision: 17, decided_on: {"recorded":{"test_result":1,"ess_conformance_coverage_v1":1}}}
- {from: "conforming", to: "validated", at: "2026-10-02T13:14:34Z", actor: "human:timo", revision: 18, decided_on: {"recorded":{"test_result":1,"ess_conformance_coverage_v1":1}}}
- {from: "validated", to: "conforming", at: "2026-10-02T14:37:29Z", actor: "human:timo", revision: 20, decided_on: {"recorded":{"test_result":1,"ess_conformance_coverage_v1":1}}}
- {from: "conforming", to: "validated", at: "2026-10-02T14:44:18Z", actor: "human:timo", revision: 21, decided_on: {"recorded":{"test_result":1,"ess_conformance_coverage_v1":1}}}
---
# Mantle executable contract

## Sources and scope

Five domains selected by spec/ess-inputs.yaml cover session records, manifest resolution, egress, launcher OS behavior and application orchestration. The worker addition extends those contracts with agent installation facts/outcomes and agent-neutral readiness. spec/README.md records production sources, named native boundaries and external-provider scope.

The original57-scenario reports and upstream304-scenario native sweep reports remain historical evidence. Wave3's final report records the current model and execution; it does not retroactively attribute upstream mutation runs to the changed unit.

## Verification

On integration merge247ed1715ac24bba0c4188354d5aac3c02d6c26a, the final gate passed167 top-level Rust tests and313 complete native ESS scenarios (CLI213, egress66, launcher34), with zero failed/error/skipped/unsupported/outside/refused. ESS0.50.0 validates seven model files and218 authored scenarios. The suite contains95 generated and218 authored scenarios. Formatting, full-workspace clippy, specification validation, planning validation and documentation build also exited0. Each command's own output is retained under .engineering/reports/codex-worker-final-gate-2026-10-02/.

The exact suite, admitted aggregate report and no-op audit are retained under .engineering/reports/codex-worker-final-native-2026-10-02/. Current model digest is8a272638ba1e78a3a382335a397e223d44436d4b3423664c5c1c22ffe8f0f895. The AEP evidence was read from that report/2 with its exact suite; historical model digests remain in Git and prior evidence.

## Boundary closure

SourceKey and native insert/read scenarios hold composite identity, duplicate refusal and retention. All seven lifecycle states exercise live-name uniqueness. Manifest scenarios execute parsing, defaults, exact-byte hashing and validation. Egress scenarios run real TCP transport with controlled DNS/dial IO. Launcher scenarios observe real processes, PTYs, FIFOs, locks, argument bytes, secrets, resize, signals and slow readers. Orchestration scenarios call production start/materialize/stop and provider logic through IO ports, then read SQLite and ordered calls. Request construction and capability/usage presentation are directly observed.

Worker commands call real pin/architecture/install-observation decisions. Separate Rust tests exercise actual bounded transport ownership, immutable activation, concurrent installation, crash/failure preservation, filesystem refusals, generation inspection and private socket allocation. The native suite does not provision cloud resources. Wave3's separate live evidence provisions a fresh KubeVirt worker without Claude credentials and adds Codex alongside an unchanged existing Claude exec. It does not claim live EC2, authenticated Codex or full lifecycle parity.

## Sensitivity evidence

All218 current authored scenarios reject the inert no-op target; six generated optional-return witnesses pass it and do not independently prove behavior. The required baseline retains every304 upstream scenario and adds9 worker/common-readiness scenarios. No counts or required names were lowered.

Historical upstream evidence in native/mutations.json records eight restored mutations and the real CheckDrain defect correction. Those mutation runs predate the worker unit; new worker adversarial red cases and the live tunnel-path red/green evidence are retained separately in this wave's reports and immutable AEP review records.

## Delivery

The public source repository is https://github.com/beyond10x/mantle. This wave is integrated locally under standing approval; it does not publish source, documentation or a release. Prior documentation delivery history remains in Git and the existing delivery stories. Current worker and Codex-parity completion boundaries are tracked in the AEP store.
