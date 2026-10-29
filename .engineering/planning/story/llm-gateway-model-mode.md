---
format: aep.planning-md/3
id: story:llm-gateway-model-mode
kind: story
status: draft
title: A manifest mode points the agent at llm-gateway
summary: 'Glovebox wave 6 (SM-D200): a manifest chooses per session to reach the model through llm-gateway with a per-run token instead of the upstream credential.'
tags:
- glovebox
relations:
- decomposes: epic:substrate-0-16
revision: 1
---
# Story: a manifest mode points the agent at llm-gateway

Part of Glovebox (atlas `initiative:glovebox`, `docs/design/glovebox.md`), wave 6.

## Why

- Glovebox rule SM-D200: "An optional second path: `llm-gateway` holds the model credential and
  the box holds only a short-lived per-run token. The manifest chooses per session"
  (`atlas/docs/design/glovebox.md:56-57`). Wave 6 assigns Mantle "a Mantle manifest mode that
  points the agent at the gateway" (`atlas/docs/design/glovebox.md:95`).
- Today the Claude credential reaches the sandbox only through the Substrate secret slot `claude`
  (`AGENTS.md:24-25`), set as `CLAUDE_CODE_OAUTH_TOKEN` (`crates/mantle/src/app/session.rs:1025`,
  `:1046`). Rule SM-D199 keeps that path (`atlas/docs/design/glovebox.md:53-55`).
- The manifest's `agent` has `kind`, `cwd` and `auth` and nothing that names a model endpoint
  (`crates/mantle/src/domain/manifest.rs:81-85`). Egress is a fixed gateway allowlist of
  `model.anthropic`, `git.github` and `rust.crates` (`crates/mantle/src/domain/manifest.rs:123-125`),
  so an llm-gateway host is not reachable today.

## What changes

1. The ESS specification first (`spec/domains/manifest.yaml`): an `agent.model` choice of
   `direct` (today's behaviour, the default) or `gateway`, with the gateway's name from operator
   configuration, never a URL in the manifest.
2. With `gateway`, Mantle obtains a per-run token from llm-gateway's issuer for the session,
   delivers it through a secret slot, and sets the agent's model endpoint to the gateway. No
   upstream model credential and no `CLAUDE_CODE_OAUTH_TOKEN` reach the sandbox. Which environment
   variables Claude Code and Codex read for a custom endpoint is inferred, not verified here.
3. The worker's egress admits the configured gateway destination for that session only, and not
   `model.anthropic`.
4. Restart obtains a fresh token; the session record names the mode, never the token.

## Acceptance

- The specification validates, and conformance passes with scenarios for both modes.
- A `gateway` session's exec request carries the per-run token slot and no upstream model
  credential; a test asserts it.
- A `gateway` session completes one Claude Code turn through a test gateway; the gateway's log
  names the run.
- `gateway` with no gateway configured, or with an unsupported agent kind, is refused by name.

## Blocked on / Depends on

- Depends on llm-gateway `story:per-run-client-tokens`.
- Open, from Atlas: whether llm-gateway relays a Claude Code conversation on the Anthropic wire
  across turns without translation is unverified (`atlas/docs/design/glovebox.md:102-104`).
