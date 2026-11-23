---
format: aep.planning-md/3
id: story:substrate-0-24-0-paired-pin
kind: story
status: draft
title: Move Mantle onto Substrate 0.24.0 as one paired step
summary: The SDK pin (0.16.1, wire 0.25.0) cannot use a 0.24.0 daemon (wire 0.27.0); pin, worker and doctor move together
revision: 1
---
## Outcome

Mantle runs on Substrate 0.24.0. The SDK pin, the Substrate version a worker installs and the
version `mantle doctor` expects move in one change, so no worker's daemon is moved alone.

## What was seen

- read: `crates/mantle/Cargo.toml:26` pins `b10x-substrate-sdk` at Substrate revision
  `f2fd606b88a0b778970e8babe9a96eb600cd57b2`, the commit of Substrate tag `0.16.1` (Mantle 0.3.1,
  `main` at `2775135`).
- read: at that revision Substrate advertises `substrate-wire/0.25.0`
  (`crates/substrate-wire/src/lib.rs:18` in `beyond10x/substrate`). Substrate 0.24.0 (commit
  `85da2e2be46b690993007990905d6d3934650774`) advertises `substrate-wire/0.27.0` on the same line.
- read: the pinned SDK refuses a response whose `x-b10x-contract` or
  `x-b10x-contract-bundle-sha256` header is not its own (`verify_contract_headers`,
  `crates/b10x-substrate-sdk/src/transport.rs:868` at the pinned revision). The name and the digest
  must both match, so the 0.3.1 CLI cannot use a 0.24.0 daemon.
- run by the review, not repeated here: the released 0.24.0 daemon answered with
  `substrate-wire/0.27.0`.
- read: Mantle already names a daemon that was moved alone. `crates/mantle/src/app/doctor.rs:209`
  fails step 7 of `mantle doctor` with "live runtime version mismatch" when the worker's daemon is
  not `SUBSTRATE_VERSION` (`0.16.1`, `crates/mantle/src/app/worker.rs:27` and
  `crates/mantle-worker/src/maintenance.rs:29`), and `crates/mantle-worker/src/upgrade.rs:425`
  marks a bundle of another Substrate version or revision incompatible.
- read: `README.md:109` says that no Mantle command, offline upgrades included, moved a 0.2.0
  worker (Substrate 0.7.10) to Substrate 0.16.1.
- read: no item in this store moves the pin past 0.16.1. `story:substrate-0-16-0-pin` is active and
  was retargeted to 0.16.1.

## What changes

Mantle's SDK pin, worker install, worker bundle manifest and doctor expectation name Substrate
0.24.0 together, and the wire behaviour that changed between `substrate-wire/0.25.0` and `0.27.0`
is adapted where Mantle calls it. How an existing 0.16.1 worker gets there is the owner's decision.

## Acceptance

On a worker provisioned by the new release, `mantle doctor` is healthy against a Substrate 0.24.0
daemon, and against a worker whose daemon is still 0.16.1 it reports the version mismatch and
starts no session.

## Where the requirement comes from

A review of the Substrate release 0.24.0 dated 2026-11-23, finding 9, which the operator handed to
the substrate session on 2026-11-23 with the order to address every finding. Filed by the substrate
session. Stakeholder: the operator.
