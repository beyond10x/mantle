---
format: aep.planning-md/3
id: task:release-0-1-4
kind: task
status: active
title: Publish the integrated Codex release records and updated docs as 0.1.4
relations:
- delivers: epic:codex-parity
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T07:24:06Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-03T07:24:06Z", actor: "human:timo", revision: 3}
---
## Outcome
Cut the new source patch release explicitly requested by the operator after integration. All earlier source and evidence PRs are merged atb17e513. Runtime behavior remains that of0.1.3; this release includes the final release/acceptance records, operator-confirmed working attachment and refreshed public release documentation.

## Scope
Workspace and lockfile versions, README, website release/quickstart references and a short qualification update. No worker mutation, new runtime feature, capture-policy change or authentication claim.

## Acceptance
Validate planning, formatting, specification and documentation; require exact-candidate CI including full native conformance and static worker builds. Merge via bot with candidate-tree verification, publish immutable annotated0.1.4 tag and official GitHub source release, verify tag checks and downloadable source archive. Publish docs source with the release; report website publication only when live provenance is observed. Broader authenticated lifecycle qualification remains separate and incomplete.
