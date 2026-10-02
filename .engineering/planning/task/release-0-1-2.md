---
format: aep.planning-md/3
id: task:release-0-1-2
kind: task
status: implemented
title: Release 0.1.2 with verified worker build documentation
relations:
- delivers: story:worker-rust-linker
- informed_by: epic:codex-parity
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T20:10:57Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-02T20:10:57Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-02T20:21:51Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":2}}}
---
## Outcome
Publish source release 0.1.2 from current main with the confined Rust/native compiler correction and updated README, public site and qualification evidence. User explicitly authorized this release with updated docs.

## Scope
Cargo.toml, Cargo.lock, README.md, website/index.html, docs/evidence/codex-compatibility.md, release evidence and this AEP task. No runtime changes beyond the already implemented worker-rust-linker story; no Substrate work or authentication claim.

## Acceptance
Repository gate and static musl worker build pass. Exact candidate required GitHub checks pass before main promotion. Tag 0.1.2 resolves to the reviewed bot commit; published bot-authored GitHub Release has valid source archives. Public documentation source and generated provenance build together; live publication is reported only if verified. Authenticated device login, model/tool turns, token refresh and full lifecycle parity remain unverified.

## Authorization
Latest operator request: release this version with updated docs. Existing approval covers all implementation waves. This task packages accepted and implemented source without expanding its behavioral scope.
