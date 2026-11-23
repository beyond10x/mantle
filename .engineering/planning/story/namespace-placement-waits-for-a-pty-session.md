---
format: aep.planning-md/3
id: story:namespace-placement-waits-for-a-pty-session
kind: story
status: draft
title: Namespace-driver placement waits for a PTY session, not for clean-room conformance
summary: Substrate 0.24.0's namespace driver admits exec but refuses a PTY session; the placement story names a blocker that shipped
relations:
- informed_by: story:kubernetes-namespace-placement
revision: 1
---
## Outcome

Mantle's plan for placement on Substrate's Kubernetes namespace driver names the requirement that
is still open: a PTY session served by that driver. It no longer waits for a Substrate release that
has shipped.

## What was seen

- read: `.engineering/planning/story/kubernetes-namespace-placement.md` says attach goes "through
  the same PTY session route" and is blocked on Substrate's exec on a cluster being verified by its
  clean-room conformance and released. `external-blocker:substrate-cluster-exec-unverified` is open
  with the title "Substrate refuses every exec on the Kubernetes namespace driver until clean-room
  conformance".
- read: Substrate's `CHANGELOG.md` at release 0.24.0 (commit
  `85da2e2be46b690993007990905d6d3934650774`): 0.17.0 (2026-10-31) shipped the shared clean-room
  conformance runner, and 0.18.0 (2026-10-31) made the namespace driver admit an exec start on a
  cluster whose probe observed the confinement.
- read: Substrate's `docs/design/34-kubernetes-namespace-driver.md` at the same commit, in its
  table of refusals: on the namespace driver a raw-pipe or PTY session is refused
  `session.unserved` or `session.pty-unserved` (501).
- read: `crates/mantle/src/adapters/substrate.rs:48` (`missing_facts`) reports `sessions.pty`
  missing when the worker does not publish that fact, so Mantle's readiness check refuses a
  namespace-driver worker today.
- not run: no cluster was started for this item. Whether a given cluster admits an exec depends on
  its deployment.

## What changes

`story:kubernetes-namespace-placement` and its blocker state the PTY session as what the placement
waits for, read at Substrate 0.24.0. Whether Mantle asks Substrate for a PTY session on that driver
or attaches another way is the owner's decision.

## Acceptance

`story:kubernetes-namespace-placement` and the blocker on it name `sessions.pty` on the namespace
driver as the open requirement, with the Substrate release it was read at, and neither names the
clean-room conformance release as outstanding.

## Where the requirement comes from

A review of the Substrate release 0.24.0 dated 2026-11-23, finding 13 (its note on Mantle), which
the operator handed to the substrate session on 2026-11-23 with the order to address every finding.
Filed by the substrate session. Stakeholder: the operator.
