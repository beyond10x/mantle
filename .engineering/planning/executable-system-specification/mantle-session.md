---
format: aep.planning-md/3
id: executable-system-specification:mantle-session
kind: executable-system-specification
status: validated
title: Mantle implemented contracts
summary: Four ESS domains, real local conformance and explicit external-boundary gaps
relations:
- specifies: epic:vertical-slice
model_digest: b6169ddbca6571a54a7d3534c5eeb06f0ad33984ec82672d4ae265cde251dfc3
revision: 6
transitions:
- {from: "draft", to: "validated", at: "2026-10-02T08:42:41Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"test_result":1}}}
---
# Mantle executable contract

## Sources and scope

`spec/ess-inputs.yaml` selects the system, components and four domains. The source/coverage ledger
is `spec/README.md`. The base implementation is commit `11b69db7b86076299a9dd92cbbc2c0196148c007`.
`domain/session.rs:10` defines seven stored states; `adapters/state.rs:30` defines nullable record
fields; `domain/manifest.rs:23` defines accepted manifest shapes; `mantle-egress/src/allow.rs:6`
defines eight default destinations. All abbreviated paths are under `crates/mantle/src/`.
The design document describes later capabilities and is not treated as shipped behavior.

## Verification

ESS 0.50.0: `ess specify validate --path spec` outputs `mantle v1 — 6 file(s), valid`;
`ess specify compile --path spec --format json` succeeds.
The real local adapter executes all 57 generated session-store/allowlist scenarios: 57 passed,
0 failed, 0 skipped. Its complete per-scenario report is `.engineering/reports/mantle-ess-local.json`.
`task check` exits 0, including format, clippy, workspace tests, ESS validation and AEP validation.

Synthesis retains two `ESS-SYNTH-013` refusals: `mantle.launch.ServeArgs` and
`mantle.manifest.Resolved` have no observable view for their value invariants. They are not included
in the 57 scenarios. This is partial local coverage, not whole-system conformance; do not move the
artifact to conforming. The report is intentionally not imported as an ESS conformance report.
The exploratory built-in interpreted target returned 7 passed and 46 unsupported on the earlier
53-scenario draft; it was not used as implementation evidence.

Removing the production transition guard failed 32 scenarios, including
`mantle.session.Session/state/FailedAgentStart/refuses/mantle.session.AgentStartFailed`.
Removing `api.anthropic.com:443` from the production default allowlist failed
`mantle.egress.CheckDefaultDestination/outcome/allowed`. Both mutations were restored before the
passing full gate. No service behavior was changed by the final patch.

## Explicit gaps

- `source-identity`: composite `(session_id,mount)` needs an ESS representation and source scenarios.
- `live-name-uniqueness`: partial cross-record uniqueness needs generated scenarios.
- `manifest-resolution`: parser/default/hash/path rules need an observable target.
- `egress-transport`: raw CONNECT, DNS/address safety, timing, forwarding and shutdown need a transport target.
- `launcher-os`: PTY/FIFO, locks, secret lifetime and byte-preserving arguments need an OS target.
- `orchestration`: worker/provider/Substrate operations and retryable cleanup need adapter scenarios.

Each marker is beside the affected YAML declaration, with sources and the closing work detailed
in `spec/README.md`. Existing unit/integration/adversary tests remain independent evidence.

## Session and handoff

Interactive operator session, 2026-10-02; store aep.project/5. Installed ESS remains 0.50.0; the
advertised 0.51.0 global upgrade is outside this change. No production resources were provisioned.
No parent epic was decomposed in this task; one implementation story (`story:ess-contract`) records
the specification work, so no decomposition critic panel was dispatched. There are no bypass records.
The repository has no configured Git remote. Keep the managed tree and its archive for operator
review; publication requires a repository remote and the mandated bot delivery route.
