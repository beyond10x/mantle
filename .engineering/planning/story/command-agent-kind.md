---
format: aep.planning-md/3
id: story:command-agent-kind
kind: story
status: active
title: Start a session whose entry point is a declared argv
relations:
- decomposes: epic:substrate-0-16
scope:
- confidence: cited
  path: crates/mantle-worker
- confidence: cited
  path: crates/mantle/src/app/session.rs
- confidence: cited
  path: crates/mantle/src/domain
- confidence: cited
  path: examples/shell.yaml
- confidence: cited
  path: spec/domains
revision: 8
transitions:
- {from: "draft", to: "proposed", at: "2026-10-29T12:59:18Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-29T12:59:18Z", actor: "human:timo", revision: 3}
---
# Story: Start a session whose entry point is a declared argv

## Why

`agent.kind` accepts only `claude-code` and `codex` (`crates/mantle/src/domain/session.rs`,
`resolve_identity`), and the agent argv is fixed to `/opt/mantle/bin/claude` or
`/opt/mantle/bin/codex` (`crates/mantle/src/app/session.rs`). The launcher already takes any argv
(`mantle-launch serve … -- <argv>`), and Substrate runs any argv without a shell. The restriction is
Mantle's own. The operator wants a shell or any bootstrap command as a session's process
(2026-10-29).

## What changes

- The ESS specification first (`spec/domains/session.yaml`, `manifest.yaml`): a third
  `AgentKind` variant `Command`, a manifest `agent.argv` list, and the rules below as refusals.
- A manifest with `agent.kind: command` names `agent.argv`, a non-empty list whose first element
  is an absolute path. `agent.argv` with any other kind is refused, and `command` without it is
  refused, each by name.
- A `command` session receives no agent credential: no secret slot, no `CLAUDE_CODE_OAUTH_TOKEN`,
  no Codex home. `agent.auth` with `command` is refused.
- The session runs under the same launcher, terminal transport, attach/detach, stop, restart and
  destroy as the two agent kinds, with the same network policy and resource bounds.
- Restart reruns the persisted argv. A persisted row of an older version keeps its kind.
- `mantle status --json` reports the kind as `command` and the argv; README and site document it.

## Acceptance

- `examples/shell.yaml` with `agent.kind: command`, `agent.argv: ["/bin/bash", "-l"]` starts,
  attaches to an interactive bash on the worker's PTY, detaches with `Ctrl-] d` and reattaches to
  the same process.
- The refusals above each have a test that names the refusal.
- The ESS specification validates and the conformance suite passes with the new scenarios; the
  existing Claude and Codex cases are unchanged.
- No secret reaches a `command` session: a test asserts the exec request carries no secret slot
  and no credential environment variable.

## Not in scope

tmux inside the session (`story:tmux-in-a-session`).
