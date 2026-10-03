---
format: aep.planning-md/3
id: task:release-0-2-0
kind: task
status: active
title: Publish Mantle 0.2.0 reliability workflows and prebuilt bundles
relations:
- delivers: epic:reliability-and-usability
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T19:35:59Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-03T19:35:59Z", actor: "human:timo", revision: 3}
---
## Outcome
Merge the approved reliability PR and publish Mantle0.2.0, explicitly requested by the operator. PR8 was merged through b10x-bot as d8e3f3ec111d2e222bff6a28426d5e7f177dc5e4; its tree equals reviewed8865390f218739e349b6ea8f5eb0184e363063a1. The minor version communicates the compatibility change from destructive stop to retained workspaces plus the five completed workflow improvements.

## Scope
Workspace/lockfile version, release documentation and third-party notices if regeneration changes them. Existing ESS/runtime behavior is unchanged from the reviewed PR. Publish an annotated bot tag, exact-source GNU/musl bundles, manifest and checksums through the existing bot release tool. No worker/session change, credential operation or authenticated qualification claim. Documentation publication follows the normal asynchronous producer.

## Acceptance
Validate the version/docs changes and planning store, require exact tagged-source CI with all499 native conformance scenarios, static worker builds and verified Linux candidate artifacts. Verify the remote tag, official GitHub Release and every required asset digest. Record observed delivery evidence; report docs as pending until live provenance is observed. This single release task does not need a decomposition panel.
