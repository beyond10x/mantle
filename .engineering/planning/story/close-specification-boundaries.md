---
format: aep.planning-md/3
id: story:close-specification-boundaries
kind: story
status: implemented
title: Resolve every Mantle specification boundary with executable evidence
relations:
- decomposes: epic:vertical-slice
- depends_on: story:ess-contract
scope:
- confidence: cited
  path: .engineering/reports/
- confidence: cited
  path: .github/workflows/ci.yml
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Cargo.toml
- confidence: cited
  path: Taskfile.yml
- confidence: cited
  path: crates/mantle-conformance/
- confidence: cited
  path: crates/mantle-egress/
- confidence: cited
  path: crates/mantle-launch/
- confidence: cited
  path: crates/mantle/
- confidence: cited
  path: spec/
- confidence: cited
  path: website/index.html
revision: 8
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T09:18:02Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-02T09:18:02Z", actor: "human:timo", revision: 4}
- {from: "active", to: "implemented", at: "2026-10-02T10:40:55Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"test_result":1}}, executor: "agent:codex-mantle"}
---
## Outcome

Resolve all six original Mantle specification boundaries with executable observations, preserve
actual implementation semantics, and publish the resulting contracts, adapters, evidence and docs.

## Acceptance

Source composite identity and retention, partial live-name uniqueness, manifest resolution,
CONNECT transport safety/lifetime, launcher OS behavior and provider/session orchestration each
have named native ESS scenarios. The complete inventory has no unresolved mapping, synthesis
refusal, omitted scenario, skip or unsupported observation. task check enforces this inventory.

## Scope

Five domains under spec/domains, 216 authored scenarios, the shared Rust ESS runner adapter and
aggregate gate, native adapters in all three product crates, private IO seams around provider
operations, CI evidence retention, recorded reports and the public website coverage section.
All executable additions are Rust. No cloud resources, real credentials, source identifiers or
ownership cascades are invented. External IO fixtures do not attest to live deployments.

## Verification

The full task check passed. Three consecutive restored native runs reported the same 304 passes:
CLI 204, egress 66, launcher 34; 88 generated and 216 authored. Zero failed, skipped, unsupported,
outside or refused. The official suite/report/run files are in .engineering/reports/native/.
The complete external aggregation is admitted by ESS and imported into the specification's AEP
evidence at model digest 8336f2747da565d5095d1469d79302729d8f9583382a012cdef92b09840e9ece.
The specification artifact is conforming. All 216 authored scenarios fail a no-op target.

Eight restored implementation mutations each failed named scenarios; native/mutations.json records
the changes and failures. A real shutdown regression was found and fixed: the proxy now owns and
joins its connection tasks, aborting remaining tunnels when its drain deadline expires.
The original adversary suites remain green.

## Delivery

Publish this candidate through common Gates and the App-only branch authority. Main requires
Gate and common / Security and privacy. The standalone website deployment follows main and must
be checked against live provenance before reporting publication complete. The operator already
approved the site and public publication; this follow-up updates its coverage facts.
