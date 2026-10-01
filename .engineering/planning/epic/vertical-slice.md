---
format: aep.planning-md/3
id: epic:vertical-slice
kind: epic
status: draft
title: Mantle vertical slice on one EC2 worker
summary: Remote confined Claude Code session on one On-Demand EC2 worker
relations:
- informed_by: executable-system-specification:mantle-session
revision: 1
---
# Epic: Mantle vertical slice on one EC2 worker

## Outcome

One developer runs `mantle start <manifest>` on a Linux laptop and gets a Claude Code prompt that
runs inside a Substrate-confined workspace on one On-Demand EC2 worker. Builds started by the agent
consume the worker's CPU. The developer detaches, reattaches to the same agent, and stops the
session; the worker can be stopped and started without losing the workspace volume.

## Sources

- Design: `docs/design/00-mantle-on-substrate.md` (§ 41 VM-first acceptance, § 42 AWS phase 1,
  § 62–63 first experiments), including the correction block for Substrate `0.7.8`.
- Domain: `spec/domains/session.yaml` (`Session`, `Worker`, `SourceResolution`), validated by
  `ess specify validate --path spec`.
- Substrate pin: tag `0.7.8`, commit `6169ef75392a98b6a503f9f5933b784cc14505ef`.

## Decisions taken for the slice

| Topic | Decision | Later |
|---|---|---|
| Detach/reattach | agent in a `tmux` server held by a long-lived exec; `attach` is a PTY session running a `tmux` client | reattachable PTY sessions in Substrate |
| Transport | SSH carried by AWS SSM, forwarding the daemon's Unix socket | TLS 1.3 + hosted Identity (design § 12) |
| Runtime | host packages (Substrate binds host `/usr`) plus `/opt/mantle` read-only root | Runtime Image resource |
| Egress | one gateway on worker loopback, one aperture, one fixed allowlist | per-session policy |
| Credential | Claude Code OAuth token through a Substrate secret slot | credential broker (design § 16) |
| Sources | initializer exec clones public repositories through the gateway | multi-source materialization |

Cloud account, profile, region and network identifiers live in the operator's
`~/.config/mantle/config.toml`, never in this repository.

## Out of scope

Peer bus, Git credential broker, `--from .` handoff, multi-worker placement, Spot, snapshots,
compiler cache, TLS/Identity transport, publication of this repository.
