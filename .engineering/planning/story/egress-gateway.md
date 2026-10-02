---
format: aep.planning-md/3
id: story:egress-gateway
kind: story
status: active
title: Egress gateway with a fixed allowlist
relations:
- decomposes: epic:vertical-slice
scope:
- confidence: cited
  path: crates/mantle-egress
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-01T22:38:39Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-01T22:38:40Z", actor: "human:timo", revision: 3}
---
# Story: Egress gateway with a fixed allowlist

## Acceptance

`mantle-egress` listens on worker loopback `127.0.0.1:3128`, answers `CONNECT host:443` for the
seven allowlisted hosts (`api.anthropic.com`, `github.com`, `codeload.github.com`,
`objects.githubusercontent.com`, `index.crates.io`, `static.crates.io`, `crates.io`) and refuses any
other host or port with `403` and a reason line naming the destination. It resolves DNS itself and
logs one line per connection with destination, bytes each way and outcome, and no header or
credential. Inside a session, `cargo fetch` succeeds and `curl https://example.com` is refused.
Unit tests cover the allowlist and request parsing.

## Scope

- `crates/mantle-egress/`
