---
format: aep.planning-md/3
id: epic:substrate-0-16
kind: epic
status: draft
title: 'Mantle on Substrate 0.16: any entry point, tmux, cluster placement, shareable docs'
revision: 1
---
# Epic: Mantle on Substrate 0.16

## Why

Mantle 0.2.0 pins Substrate 0.7.10 (`crates/mantle/Cargo.toml`, `b10x-substrate-sdk` at
`65304edf`). Substrate has since released 0.8.0 to 0.15.0 and is cutting 0.16.0 with the
Kubernetes namespace driver (Substrate design 34, ADR 0047). Three operator asks from 2026-10-29
ride on that move:

1. A session's entry point is not limited to Claude Code or Codex: a shell or any bootstrap
   command can be the session's process.
2. tmux runs inside a session, once Substrate admits Unix-domain sockets scoped to the workspace
   (Substrate ADR, option A of 2026-10-29).
3. Mantle's public documentation is ready to be shared.

## Stories

| story | depends on |
|---|---|
| `story:substrate-0-16-0-pin` | Substrate 0.16.0 tagged and published |
| `story:command-agent-kind` | nothing; starts now |
| `story:tmux-in-a-session` | the Substrate workspace Unix-socket ADR implemented and released |
| `story:helm-chart-worker` | `story:substrate-0-16-0-pin` |
| `story:kubernetes-namespace-placement` | Substrate's exec on a cluster verified (its `story:remote-clean-room-conformance`) |
| `story:public-docs-sweep` | nothing; starts now |
| `task:release-0-3-0` | the stories above that are implemented when it is cut |

## Done when

Mantle runs on a released Substrate 0.16.x worker, a manifest can name any argv as the session's
entry point, and the public site and README describe the current release first, with every claim
traceable to evidence in the repository.
