---
format: aep.planning-md/3
id: story:tmux-in-a-session
kind: story
status: draft
title: tmux runs inside a session
relations:
- decomposes: epic:substrate-0-16
- depends_on: story:command-agent-kind
revision: 1
---
# Story: tmux runs inside a session

## Why

tmux's server listens on a Unix-domain socket and its client connects to it. Substrate's confinement
refuses `socket(AF_UNIX, …)` with `EACCES` (Substrate ADR 0015, ADR 0039 point 2), so tmux, screen
and every tool serving a Unix socket fail in a session. The operator chose option A on 2026-10-29:
Substrate admits Unix-domain sockets whose bind and connect stay inside the workspace, recorded by
a new Substrate ADR before code.

## What changes

- Mantle pins the Substrate release that implements that ADR and requires its capability fact
  before it relies on it; without the fact, nothing changes and the README says tmux is unavailable.
- The worker toolchain profile ships `tmux`.
- `TMUX_TMPDIR` points under `/workspace/.mantle/` so the socket lives inside the workspace.

## Acceptance

- In a `command` session running `/bin/bash`, `tmux new-session -d` and `tmux attach` work, and the
  tmux server survives a Mantle detach and reattach.
- A worker whose Substrate does not publish the fact reports tmux unavailable in `mantle doctor`
  rather than failing at run time.

## Blocked on

The Substrate ADR on workspace-scoped Unix-domain sockets, implemented and released.
