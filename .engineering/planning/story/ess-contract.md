---
format: aep.planning-md/3
id: story:ess-contract
kind: story
status: implemented
title: Specify Mantle implemented contracts and execute local conformance
relations:
- decomposes: epic:vertical-slice
- informed_by: executable-system-specification:mantle-session
scope:
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: README.md
- confidence: cited
  path: Taskfile.yml
- confidence: cited
  path: crates/mantle/Cargo.toml
- confidence: cited
  path: crates/mantle/src/adapters/conformance.rs
- confidence: cited
  path: crates/mantle/src/adapters/state.rs
- confidence: cited
  path: spec/
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T08:28:52Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-02T08:28:52Z", actor: "human:timo", revision: 4}
- {from: "active", to: "implemented", at: "2026-10-02T08:43:11Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"test_result":1}}}
---
## Outcome

Expand `spec/` to describe Mantle's implemented records, state transitions and worker boundaries,
with source citations and explicit unmapped semantics. Preserve the distinction between local
recorded intent and live Substrate observations.

## Acceptance

`task check` validates the expanded specification and executes its locally supported named
conformance scenarios against production Rust code, with cloud and process boundaries and
any remaining synthesis refusals recorded in the specification's coverage documentation.

## Sources

`crates/mantle/src/domain/session.rs:10` declares seven states; `crates/mantle/src/adapters/state.rs:30`
declares nullable workspace and exec ids; `crates/mantle/src/app/session.rs:588` resumes stopping;
`spec/domains/session.yaml:1` is the existing seed. No cloud infrastructure is provisioned.
