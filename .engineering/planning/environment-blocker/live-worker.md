---
format: aep.planning-md/3
id: environment-blocker:live-worker
kind: environment-blocker
status: open
title: No reachable Mantle worker for live session acceptance
relations:
- blocks: story:session-lifecycle
- blocks: story:egress-gateway
- blocks: story:codex-interactive-start
revision: 1
---
## Observation

On 2026-10-21 no Mantle worker was reachable from the operator laptop. `mantle worker status` with
the configured `provider = "kubevirt"` failed with `error: context "k3d-mantle" does not exist`;
`k3d cluster list` lists no `mantle` cluster. The EC2 provider needs AWS spending, which
story:kubevirt-provider records as paused pending cost approval. Both worker stories
(story:worker-provisioning, story:kubevirt-provider) were left out of the 2026-10-21 wave because
they need cloud accounts or spending.

## What it withholds

Every LIVE-ONLY acceptance clause of story:session-lifecycle (clone, attach, resize, detach,
reattach, stop/destroy against a real worker), story:egress-gateway (`cargo fetch` succeeds inside a
session; `curl https://example.com` refused was observed once on 2026-10-02 in
`.engineering/reports/codex-acceptance-audit-2026-10-02/unlisted-host.log`) and
story:codex-interactive-start (CS-02 to CS-08 live parts).

## What clears this

A READY worker: either a recreated local KubeVirt cluster (`task k3d-up`, then `mantle worker up`)
or an approved EC2 worker. The clearing evidence is a `mantle worker status` that reports READY.
