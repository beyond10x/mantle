---
format: aep.planning-md/3
id: executable-system-specification:mantle-session
kind: executable-system-specification
status: validated
title: Mantle implemented contracts
summary: Five ESS domains; complete native conformance with zero unresolved mappings
relations:
- specifies: epic:vertical-slice
model_digest: 2dc5342bfece562e4b6ac7402c8f029d37cb77225cef07625c9af65b7718ef37
revision: 29
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
- {from: "validated", to: "conforming", at: "2026-10-02T15:55:05Z", actor: "human:timo", revision: 25, decided_on: {"recorded":{"test_result":2,"ess_conformance_coverage_v1":1}}, executor: "agent:codex-mantle"}
- {from: "conforming", to: "validated", at: "2026-10-02T16:02:44Z", actor: "human:timo", revision: 26, decided_on: {"recorded":{"test_result":2,"ess_conformance_coverage_v1":1}}, executor: "agent:codex-mantle"}
- {from: "validated", to: "conforming", at: "2026-10-02T16:38:46Z", actor: "human:timo", revision: 28, decided_on: {"recorded":{"test_result":2,"ess_conformance_coverage_v1":1}}}
- {from: "conforming", to: "validated", at: "2026-10-02T16:44:06Z", actor: "human:timo", revision: 29, decided_on: {"recorded":{"test_result":2,"ess_conformance_coverage_v1":1}}}
---
# Mantle executable contract

## Sources and scope

Five domains selected by spec/ess-inputs.yaml cover session records, manifest resolution, egress, launcher OS behavior and application orchestration. The worker addition extends those contracts with agent installation facts/outcomes and agent-neutral readiness. spec/README.md records production sources, named native boundaries and external-provider scope.

The original57-scenario reports and upstream304-scenario native sweep reports remain historical evidence. Wave3's final report records the current model and execution; it does not retroactively attribute upstream mutation runs to the changed unit.

## Verification

Integration merge8d05b7a8b6ffa41fef3c4eb1f43292e054079624 passes the complete repository gate:179 top-level Rust tests and331 native ESS scenarios, comprising227 authored and104 generated, with zero failed/error/skipped/unsupported/outside/refused. Components execute CLI218, egress66 and launcher47. ESS0.50 validates seven source files and227 authored scenarios. Formatting, full-workspace clippy, specification/planning validation and documentation build each exit0. Per-step runner output is retained under .engineering/reports/codex-session-wave5-2026-10-02/gate/ with personal home prefixes normalized; no step was skipped.

Exact suite, report, results and no-op audit are under .engineering/reports/codex-session-wave5-2026-10-02/native/. Current model digest is45700f1d2dc4e3c23ba9a26824b43bef5e98d2623981472ab852f3b57951dcac. AEP evidence is admitted from that report/2 beside its exact suite. Prior qualification, worker and launcher reports remain historical; their outcomes are not silently attributed to this changed model.

Wave5 adds actual Codex manifest/SQLite identity and migration, production request/preflight/attachment selection, launcher filesystem checks and pinned-program synthetic diagnostic-path evidence. It explicitly refuses unsupported Substrate capture. No authenticated session, token refresh, complete network use or live two-agent parity is claimed by this local contract.

## Boundary closure

SourceKey and native insert/read scenarios hold composite identity, duplicate refusal and retention. All seven lifecycle states exercise live-name uniqueness. Manifest scenarios execute parsing, defaults, exact-byte hashing and validation. Egress scenarios run real TCP transport with controlled DNS/dial IO. Launcher scenarios observe real processes, PTYs, FIFOs, locks, argument bytes, secrets, resize, signals and slow readers. Orchestration scenarios call production start/materialize/stop and provider logic through IO ports, then read SQLite and ordered calls. Request construction and capability/usage presentation are directly observed.

Worker commands call real pin/architecture/install-observation decisions. Separate Rust tests exercise actual bounded transport ownership, immutable activation, concurrent installation, crash/failure preservation, filesystem refusals, generation inspection and private socket allocation. The native suite does not provision cloud resources. Wave3's separate live evidence provisions a fresh KubeVirt worker without Claude credentials and adds Codex alongside an unchanged existing Claude exec. It does not claim live EC2, authenticated Codex or full lifecycle parity.

## Sensitivity evidence

All227 current authored scenarios reject the deliberately inert adapter. The same six schema-only generated optional-return witnesses pass it and do not independently prove behavior. The required baseline retains prior named scenarios and assertions; neither the inventory floor nor expected values were weakened to admit the added identity fields. Current no-op audit is retained with the exact suite/report in .engineering/reports/codex-session-wave5-2026-10-02/native/.

Historical upstream native/mutations.json records eight restored mutations and the real CheckDrain correction; those runs predate these Codex changes. Separate test-first public CLI/private-path failures, actual legacy-database tests, five independent adversary additions and synthetic pinned-Codex sink observations are recorded in this wave's reports and immutable review-result:codex-session-adversary-1. The runtime scanner's deliberately planted persistent positive control was detected; it does not claim a real token-refresh test.

## Delivery

The public source repository is https://github.com/beyond10x/mantle. This wave is integrated locally under standing approval; it does not publish source, documentation or a release. Prior documentation delivery history remains in Git and the existing delivery stories. Current worker and Codex-parity completion boundaries are tracked in the AEP store.
