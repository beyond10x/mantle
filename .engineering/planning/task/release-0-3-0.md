---
format: aep.planning-md/3
id: task:release-0-3-0
kind: task
status: archived
title: Release Mantle 0.3.0
relations:
- decomposes: epic:substrate-0-16
- depends_on: story:substrate-0-16-0-pin
- depends_on: story:command-agent-kind
- depends_on: story:public-docs-sweep
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-29T22:54:43Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-29T22:54:43Z", actor: "human:timo", revision: 3}
- {from: "active", to: "archived", at: "2026-10-30T00:44:46Z", actor: "human:timo", revision: 5}
---
# Task: Release Mantle 0.3.0

Cut 0.3.0 from `wave/mantle-next-1` once merged to `main`, with the implemented stories of
`epic:substrate-0-16`: at least `story:substrate-0-16-0-pin`, `story:command-agent-kind` and
`story:public-docs-sweep`.

## Steps

The repository's release flow (README § Verified prebuilt bundles): bot tag, CI candidate,
`mantle-release publish`, verified asset digests, CHANGELOG and live site provenance checked.

## Superseded

Superseded by `task:release-0-3-1` (D244, 2026-10-30). Tag `0.3.0` names `1811ebe2a03e0186c74d46f4d5cd39d388af4aab`; its tag CI run 40977750914 passed. The upload was refused by the Gates `personal-paths` rule (`decision-blocker:release-0-3-0-gates-personal-paths`), so 0.3.0 has no GitHub release. The tag stays as it is.
