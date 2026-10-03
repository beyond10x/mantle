---
format: aep.planning-md/3
id: task:release-0-1-4
kind: task
status: implemented
title: Publish the integrated Codex release records and updated docs as 0.1.4
relations:
- delivers: epic:codex-parity
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T07:24:06Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-03T07:24:06Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-03T07:33:16Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"test_result":2}}}
---
## Outcome
Cut the new source patch release explicitly requested by the operator after integration. All earlier source and evidence PRs are merged atb17e513. Runtime behavior remains that of0.1.3; this release includes the final release/acceptance records, operator-confirmed working attachment and refreshed public release documentation.

## Scope
Workspace and lockfile versions, README, website release/quickstart references and a short qualification update. No worker mutation, new runtime feature, capture-policy change or authentication claim.

## Acceptance
Validate planning, formatting, specification and documentation; require exact-candidate CI including full native conformance and static worker builds. Merge via bot with candidate-tree verification, publish immutable annotated0.1.4 tag and official GitHub source release, verify tag checks and downloadable source archive. Publish docs source with the release; report website publication only when live provenance is observed. Broader authenticated lifecycle qualification remains separate and incomplete.

## Verified publication

Published https://github.com/beyond10x/mantle/releases/tag/0.1.4 at2026-10-03T07:32:09Z as an official non-draft, non-prerelease source release. Annotated bot tag55e303425658f959860f812cccdefff6da67489c resolves to exact candidate054d582211ca2a87f3eb4f8f396f1e657fd5cf63. PR6 merged through app/b10x-bot as8f6efd57e554eecb562efce701b34a4ba88afe30; both trees equal39288a6780b60c36b4c47578583178f19b9482d2. Exact candidate Gate37106352718, documentation37106352708 and common37106353065 passed; tagged common37106649210 passed. Downloaded GitHub source archive SHA2563db7abebfec6df7876ecd015770b649c1e5e105cf16d08a335650241fd7e9f8f; inspected root Cargo.toml0.1.4 and updated README. No prebuilt assets are promised. Public site provenance at /mantle/.well-known/b10x-site.json reports8f6efd57e554eecb562efce701b34a4ba88afe30, matching the released documentation tree. Runtime source, deployment and ESS specification are unchanged from0.1.3. No worker restart or new authenticated parity claim was made.
