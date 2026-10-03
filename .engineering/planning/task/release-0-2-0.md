---
format: aep.planning-md/3
id: task:release-0-2-0
kind: task
status: implemented
title: Publish Mantle 0.2.0 reliability workflows and prebuilt bundles
relations:
- delivers: epic:reliability-and-usability
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T19:35:59Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-03T19:35:59Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-03T20:13:40Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"test_result":2}}}
---
## Outcome
Merge the approved reliability PR and publish Mantle0.2.0, explicitly requested by the operator. PR8 was merged through b10x-bot as d8e3f3ec111d2e222bff6a28426d5e7f177dc5e4; its tree equals reviewed8865390f218739e349b6ea8f5eb0184e363063a1. The minor version communicates the compatibility change from destructive stop to retained workspaces plus the five completed workflow improvements.

## Scope
Workspace/lockfile version, release documentation and third-party notices if regeneration changes them. Existing ESS/runtime behavior is unchanged from the reviewed PR. Publish an annotated bot tag, exact-source GNU/musl bundles, manifest and checksums through the existing bot release tool. No worker/session change, credential operation or authenticated qualification claim. Documentation publication follows the normal asynchronous producer.

## Acceptance
Validate the version/docs changes and planning store, require exact tagged-source CI with all499 native conformance scenarios, static worker builds and verified Linux candidate artifacts. Verify the remote tag, official GitHub Release and every required asset digest. Record observed delivery evidence; report docs as pending until live provenance is observed. This single release task does not need a decomposition panel.

## Verified publication

Published https://github.com/beyond10x/mantle/releases/tag/0.2.0 at2026-10-03T20:11:47Z as an official non-draft, non-prerelease release by b10x-bot[bot]. Annotated bot tag054f5aeef0d7691d39fb1f16c358c3a69d08a15f resolves to54710558ebe096a8e5faf98bbb430ff7f9eaa17e, also published to main after its required Gate and common checks passed. PR8 was already merged through the App asd8e3f3ec111d2e222bff6a28426d5e7f177dc5e4 with the reviewed candidate tree.

Tagged CI37148818459 passed the full gate, all499 native scenarios, static worker builds and exact-source Linux candidate verification. Common37148818801 passed. Both candidate archives were downloaded and verified locally before publication. The release tool verified the exact remote tag and all four uploaded asset digests after publication, exit0. The bot release API confirms both archive assets, manifest.json and SHA256SUMS, all uploaded by the bot. Exact asset metadata is retained in .engineering/reports/release-0.2.0.json.

The initial main push was refused because required checks did not yet exist; after the tagged run supplied those results, the same signed publication succeeded. Initial publisher staging under the personal home was refused by the personal-paths rule; API404 proved no release was created. A private system-temporary staging directory supplied an admissible file path to the same publisher, which completed and removed its staging directory. No policy or gate was bypassed.

Documentation validation37149375752 and site publication37149432841 passed. The live /mantle/.well-known/b10x-site.json identifies54710558ebe096a8e5faf98bbb430ff7f9eaa17e; downloaded HTML is byte-identical to that source. Authenticated qualification remains separate. No worker or user session was changed. Private command receipts, release manifest/assets and downloaded CI reports remain under $HOME/.cache/mantle-release-0.2.0. This closing change records observed delivery only and does not change the released source or tag.
