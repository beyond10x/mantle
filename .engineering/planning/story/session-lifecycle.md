---
format: aep.planning-md/3
id: story:session-lifecycle
kind: story
status: draft
title: Start, attach, detach and stop a confined Claude Code session
relations:
- decomposes: epic:vertical-slice
- depends_on: story:worker-provisioning
- depends_on: story:egress-gateway
revision: 1
---
# Story: Start, attach, detach and stop a confined Claude Code session

## Acceptance

With a READY worker, `mantle start examples/substrate.yaml` creates a workspace, clones the declared
repository at an exact commit that `mantle status` prints, and attaches the local terminal to Claude
Code running in `/workspace/<mount>`. Resizing the local terminal changes `tput cols` inside.
`Ctrl-b d` returns to the laptop while `mantle status` still reports the agent exec `running`;
`mantle attach` returns to the same Claude conversation. `mantle stop` ends the agent exec, destroys
the workspace and leaves `mantle list` without the session. An exec that Substrate reports `unknown`
is shown as unknown, never as stopped.

## Scope

- `crates/mantle/src/domain/{manifest,session}.rs`, `crates/mantle/src/app/{start,attach,stop,status}.rs`
- `crates/mantle/src/adapters/{substrate,state}.rs`, `crates/mantle-launch/`
- `examples/substrate.yaml`

## Depends on

The worker story's spike.
