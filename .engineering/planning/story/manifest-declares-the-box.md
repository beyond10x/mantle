---
format: aep.planning-md/3
id: story:manifest-declares-the-box
kind: story
status: draft
title: The manifest declares the box
summary: 'Glovebox requirement 1: read-only folders, outbound hosts, an inbound port and quotas in the manifest, each mapped onto a Substrate mechanism or refused by name.'
tags:
- glovebox
relations:
- decomposes: epic:substrate-0-16
- depends_on: story:substrate-0-16-0-pin
revision: 1
---
# Story: the manifest declares the box

Part of Glovebox (atlas `initiative:glovebox`, `docs/design/glovebox.md`), requirement 1.

## Why

- Glovebox requirement 1: "One command names the process to run, the folders it may read
  (read-only), the hosts it may reach outbound, the inbound port it may serve (port 80 in the
  example), and its quotas (CPU, memory, IO, network)" (`atlas/docs/design/glovebox.md:19-21`).
  The layer table gives Mantle "the one declaration (its manifest)" and Substrate the mechanisms
  (`atlas/docs/design/glovebox.md:39-40`).
- Today's manifest covers part of it. `resources` has `cpu`, `memory`, `pids` and `storage`
  (`crates/mantle/src/domain/manifest.rs:89-93`); `workspace` has repositories and a root
  (`crates/mantle/src/domain/manifest.rs:63-76`); `network` has `default`, `profile` and
  `capabilities` (`crates/mantle/src/domain/manifest.rs:102-108`), and the capabilities are a fixed
  allowlist a manifest may narrow but not widen (`crates/mantle/src/domain/manifest.rs:123-125`).
  There is no read-only folder, no outbound host beyond the fixed list, no inbound port and no IO
  or network quota.
- Substrate serves some of the mechanisms: at most four operator-declared read-only roots
  (`substrate/adr/0010-declared-host-roots-are-mounted-read-only.md`), named egress apertures
  (`substrate/adr/0013-egress-apertures-are-declared-by-the-operator.md`) and an aperture byte
  ceiling (`substrate/docs/design/12-aperture-byte-ceiling.md`). It does not serve an inbound
  endpoint or block-IO and bandwidth limits; Substrate's draft stories
  `story:inbound-endpoint-for-a-confined-exec` and `story:block-io-and-bandwidth-limits` cover
  them. Mantle pins Substrate at `65304edf` (`crates/mantle/Cargo.toml:26`), which predates several
  of these facts.

## What changes

1. The ESS specification first (`spec/domains/manifest.yaml`): `box.readOnly` (host folders by
   operator-configured name, each with a mount point), `box.egress` (outbound hosts by
   operator-configured aperture name), `box.inbound` (one port), and `box.quotas` adding `io` and
   `network` beside the existing `resources`.
2. Each field maps onto one Substrate mechanism: read-only folders onto declared read-only roots,
   outbound hosts onto egress apertures, the inbound port onto Substrate's endpoint, IO and network
   onto its limits. Mantle reads the worker daemon's capability facts and maps a field only when
   the fact is present.
3. A field the worker's Substrate cannot serve is refused by name before the session starts, for
   example `manifest.box.inbound-unserved`, never dropped or approximated (Mantle `AGENTS.md`
   § Rules: "Never report what Substrate did not observe", `AGENTS.md:22-23`).
4. Names, not paths or hosts, appear in a manifest; the operator's configuration binds names to
   host paths and destinations, as Substrate requires for roots and apertures.

## Acceptance

- The specification validates, and conformance passes with a scenario for each field served and
  each field refused.
- A manifest naming two read-only folders and one outbound host starts on a worker whose Substrate
  publishes the matching facts; the applied record lists both roots and the aperture; a write to a
  read-only folder fails inside the box.
- A manifest naming an inbound port, or an IO or network quota, on a worker whose Substrate lacks
  the fact is refused by name before any exec, and `mantle status` shows nothing started.
- Existing manifests without `box` behave exactly as before.

## Blocked on / Depends on

- Read-only folders and outbound hosts: Substrate facts that exist in releases after Mantle's pin;
  needs `story:substrate-0-16-0-pin`.
- Inbound port: Substrate `story:inbound-endpoint-for-a-confined-exec`. IO and network quotas:
  Substrate `story:block-io-and-bandwidth-limits`. Until each ships, its field is a named refusal.
