---
format: aep.planning-md/3
id: coordination-blocker:codex-worker-ess-overlap
kind: coordination-blocker
status: open
title: Reconcile active Mantle ESS work before worker changes
relations:
- blocks: story:agent-ready-worker
revision: 3
---
## Observation

On 2026-10-02, `worktree inspect --repo ~/beyond10x/mantle --id wt-2a7946b5d60b` reported Active, one live lease, branch `spec/mantle-ess-contracts`, at base `11b69db`, with uncommitted ESS implementation work. `git status --short` showed changes to Taskfile.yml, spec/domains/session.yaml, spec/system.yaml, crates/mantle/src/adapters/state.rs, Cargo manifests and README, plus a new conformance adapter and explicit ESS inputs/components.

## What is blocked

Do not dispatch agent-ready-worker against an unreconciled specification/gate. Its scope overlaps Taskfile.yml and spec/domains/session.yaml; the incoming conformance adapter changes where its AW scenarios must attach. This also affects the later stories through their existing dependencies. Codex qualification only writes its three new probe/test/evidence paths and can be proposed independently.

## Clearance evidence

The owning ESS session completes or explicitly hands over its changes; the coordinator reads the actual resulting commit/diff, reconciles the Codex proposal's optional Session fields through an ordinary merge, validates ESS/AEP, and refreshes worker scope against the integrated adapter. Do not clear another session's lease or remove its worktree. A mere elapsed lease or this wave's approval does not settle this blocker.

## Next owner

Mantle wave coordinator after the active ESS work is handed over. No external message has been sent. This is coordination, not a finding that the ESS code is defective.

## Source now available

The owning ESS tree is now clean at bot-authored commit
`87bed8fccfd8612a713da4aed2cbfe9b2f28689d` (2026-10-02 observation). Its
`story:ess-contract` is implemented and records a passing 57-scenario local conformance report.
The same commit bundles separately authorised public repository/site work. This coordinator has
not published it or changed its owner's checkout. Reconciliation with our proposed Codex fields
is still required before clearing this blocker and scheduling worker work. Do not make public
publication a prerequisite for local implementation; integrate the exact source when appropriate.
