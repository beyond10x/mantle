---
format: aep.planning-md/3
id: task:release-0-3-0
kind: task
status: draft
title: Release Mantle 0.3.0
relations:
- decomposes: epic:substrate-0-16
- depends_on: story:substrate-0-16-0-pin
- depends_on: story:command-agent-kind
- depends_on: story:public-docs-sweep
revision: 1
---
# Task: Release Mantle 0.3.0

Cut 0.3.0 from `wave/mantle-next-1` once merged to `main`, with the implemented stories of
`epic:substrate-0-16`: at least `story:substrate-0-16-0-pin`, `story:command-agent-kind` and
`story:public-docs-sweep`.

## Steps

The repository's release flow (README § Verified prebuilt bundles): bot tag, CI candidate,
`mantle-release publish`, verified asset digests, CHANGELOG and live site provenance checked.
