---
format: aep.planning-md/3
id: story:worker-provisioning
kind: story
status: active
title: Provision one EC2 worker reachable only through SSM
relations:
- decomposes: epic:vertical-slice
scope:
- confidence: cited
  path: crates/mantle/src/adapters/aws.rs
- confidence: cited
  path: crates/mantle/src/adapters/ssh.rs
- confidence: cited
  path: crates/mantle/src/app/worker.rs
- confidence: cited
  path: deploy
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-01T22:38:39Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-01T22:38:39Z", actor: "human:timo", revision: 3}
---
# Story: Provision one EC2 worker reachable only through SSM

## Acceptance

`mantle worker up` against an account with no Mantle resources creates one tagged instance, its
instance profile, a security group with zero ingress rules and a retained data volume, and ends in
`READY` only after `GET /v1/machine` over the forwarded socket reports exec, PTY, cgroup limits,
the `egress` aperture and the `claude` secret slot. A missing fact ends in a named refusal instead.
`mantle worker down` stops the instance; a second `up` starts it again with the same data volume.

## Spike (before the session story starts)

On the worker, with a throwaway SDK program outside this repository:

1. bubblewrap and delegated cgroup facts are present;
2. a `tmux` server in one exec and a `tmux` client in a PTY session share `/workspace/.mantle/tmux.sock`;
3. Claude Code starts in a sandbox that has no `/etc/passwd`;
4. `cargo check` runs with a read-only `RUSTUP_HOME` under `/opt/mantle`.

Any failure is recorded here with the refusal text and stops the session story.

## Scope

- `crates/mantle/src/adapters/aws.rs`, `crates/mantle/src/adapters/ssh.rs`, `crates/mantle/src/app/worker.rs`
- `deploy/cloud-init.yaml`, `deploy/substrate.service`, `deploy/mantle-egress.service`, `deploy/bwrap.apparmor`
