---
format: aep.planning-md/3
id: executable-system-specification:mantle-session
kind: executable-system-specification
status: conforming
title: Mantle implemented contracts
summary: Five ESS domains; complete native conformance with zero unresolved mappings
relations:
- specifies: epic:vertical-slice
model_digest: 8336f2747da565d5095d1469d79302729d8f9583382a012cdef92b09840e9ece
revision: 13
transitions:
- {from: "draft", to: "validated", at: "2026-10-02T08:42:41Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"test_result":1}}}
- {from: "validated", to: "conforming", at: "2026-10-02T10:39:51Z", actor: "human:timo", revision: 9, decided_on: {"recorded":{"test_result":1,"ess_conformance_coverage_v1":1}}, executor: "agent:codex-mantle"}
- {from: "conforming", to: "validated", at: "2026-10-02T10:45:11Z", actor: "human:timo", revision: 10, decided_on: {"recorded":{"test_result":1,"ess_conformance_coverage_v1":1}}, executor: "agent:codex-mantle"}
- {from: "validated", to: "conforming", at: "2026-10-02T10:45:12Z", actor: "human:timo", revision: 13, decided_on: {"recorded":{"test_result":1,"ess_conformance_coverage_v1":1}}, executor: "agent:codex-mantle"}
---
# Mantle executable contract

## Sources and scope

Five domains selected by spec/ess-inputs.yaml cover session records, manifest resolution, egress,
launcher OS behavior and application orchestration. spec/README.md records production sources,
named native boundaries and external-provider scope. The original 57-scenario report is retained
as historical evidence; the complete native reports supersede it.

## Verification

ESS 0.50.0 validates seven model files and 216 authored scenarios. The complete declared suite
contains 304 scenarios (88 generated, 216 authored): CLI 204, egress 66 and launcher 34. All pass;
there are zero failed, skipped, unsupported, outside or refused scenarios. Three consecutive
restored-implementation runs hold these counts. The full task check passes formatting, clippy,
unit/integration/adversary tests, native conformance, ESS/AEP validation and documentation build.

The official component Rust reports, detailed runs and exact suites are in
.engineering/reports/native/. mantle-report.json is an external-results aggregation admitted by
ESS against mantle-suite.json; the gate compares every executed scenario definition and refuses
missing or duplicate results. spec/conformance-baseline.json holds named scenarios and counts.
The current compiled model digest is 8336f2747da565d5095d1469d79302729d8f9583382a012cdef92b09840e9ece.

## Boundary closure

SourceKey and native insert/read scenarios hold composite identity, duplicate refusal and retention.
All seven lifecycle states exercise live-name uniqueness. Manifest scenarios execute parsing,
defaults, exact-byte hashing and validation. Egress scenarios run real TCP transport with controlled
DNS/dial IO. Launcher scenarios observe real processes, PTYs, FIFOs, locks, argument bytes, secrets,
resize, signals and slow readers. Orchestration scenarios call production start/materialize/stop
and provider logic through IO ports, then read SQLite and ordered calls. Request construction and
capability/usage presentation are directly observed. No cloud resources are provisioned; upstream
AWS, Kubernetes and Substrate deployment correctness is not asserted by these local fixtures.

## Sensitivity evidence

Eight restored production mutations caused named failures: composite key, live-name uniqueness,
memory lower bound, unknown-destroy read-back, stopped-worker restart, absent usage presentation,
private DNS filtering and secret NUL refusal. See native/mutations.json. The no-op audit rejects
all 216 authored scenarios; six automatic acceptance/optional-return witnesses pass the inert
target and do not independently prove behavior.

The new CheckDrain scenario also found and reproduced a real defect: proxy connection tasks
survived the drain deadline. The proxy now owns them in a JoinSet and aborts/joins remaining
connections before returning.

## Delivery

The public repository is https://github.com/beyond10x/mantle and documentation is published at
https://beyond10x.github.io/mantle/. The operator approved the site preview and publication.
Common Gates enrollment, bot-only authority, required main checks and secret scanning are active.
The complete follow-up is recorded by story:close-specification-boundaries.
