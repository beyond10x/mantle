---
format: aep.planning-md/3
id: story:substrate-0-16-0-pin
kind: story
status: active
title: Move Mantle onto Substrate 0.16.0
relations:
- decomposes: epic:substrate-0-16
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-29T22:54:41Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-29T22:54:41Z", actor: "human:timo", revision: 3}
---
# Story: Move Mantle onto Substrate 0.16.0

## Why

Mantle pins `b10x-substrate-sdk` at Substrate revision `65304edf` (0.7.10). `mantle worker upgrade`
refuses a Substrate version or revision other than the bundle's (README § Offline worker upgrades),
so the laptop CLI, the worker bundle and the worker's daemon move together.

## What changes

- `crates/mantle/Cargo.toml`: the SDK pinned to the exact revision of Substrate tag `0.16.0`.
- Worker provisioning installs the Substrate 0.16.0 daemon from its signed release (the release's
  `substrate-daemon-0.16.0-x86_64-unknown-linux-gnu.tar.gz`, verified with `cosign verify-blob`)
  instead of a source build, if the release binaries satisfy the worker's glibc; otherwise the
  source build at the tagged revision.
- `mantle doctor`'s wire-compatibility check accepts the bundle the 0.16.0 daemon advertises.
- Every call site the SDK changes between 0.7.10 and 0.16.0 is adapted; each changed wire behaviour
  is listed in the PR.
- The release manifest records Substrate 0.16.0 and its revision.

## Acceptance

- `task check` passes on the new pin.
- On a KubeVirt worker (`task k3d-up`), `mantle doctor` is healthy against the 0.16.0 daemon, and a
  Claude session and a `command` session start, attach, detach and stop.
- `mantle worker upgrade --check` against a worker on 0.7.10 reports the Substrate mismatch and
  refuses, as today.

## Blocked on

Substrate 0.16.0 tagged and published (`beyond10x/substrate`, `task:release-0-16-0`).

## Retargeted to Substrate 0.16.1

Tag `0.16.0` never published; this story is retargeted to Substrate 0.16.1 (tag `0.16.1`, commit
`f2fd606b`, wire bundle `substrate-wire/0.25.0`). The pin, the daemon install from the signed release
archive and the documented wire changes are in the wave (`54cea70`, `6f4144e`).

The live KubeVirt acceptance (`mantle doctor` healthy, a Claude session and a `command` session
started, attached, detached and stopped on a 0.16.1 worker) is not yet run. The story stays active
until it is.
