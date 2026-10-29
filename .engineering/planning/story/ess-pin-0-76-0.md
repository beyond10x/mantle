---
format: aep.planning-md/3
id: story:ess-pin-0-76-0
kind: story
status: draft
title: Move Mantle's ESS pin to 0.76.0 in this wave
relations:
- decomposes: epic:substrate-0-16
revision: 1
---
## Outcome

Mantle's specification, generated model and conformance suite run on ESS 0.76.0, the newest release, and CI installs that release with its checksum.

## Why

The workspace rule (atlas ace662b1, workspace AGENTS.md § ESS version): a repository's ESS pin moves to the newest ESS release when a wave changes that repository anyway, in the same wave. This wave (`wave/mantle-next-1`) changes Mantle; the pin is 0.50 (`spec/ess-inputs.yaml`, `.github/workflows/ci.yml`).

Observed before the move: under ESS 0.76.0 the drift test `generated_model_has_no_drift` and the conformance suite fail (`$suite.provenance.scenario_initial_state: UnknownField`).

## Acceptance

- `spec/ess-inputs.yaml` requires `ess 0.76.0`; `ci.yml` downloads 0.76.0 and checks its SHA-256 taken from the release assets.
- Generated output regenerated with 0.76.0; the drift test passes.
- `ess specify validate --path spec` exit 0; the conformance inventory runs with 0 failed, skipped, unsupported, outside or refused, and the count before and after is reported.
- Every other `task check` step exit 0.

## Scope

`spec/`, `generated/`, `crates/mantle-conformance`, `crates/mantle/src/adapters/conformance.rs`, `.github/workflows/ci.yml`, `Taskfile.yml`, docs that cite the ESS version or scenario counts.
