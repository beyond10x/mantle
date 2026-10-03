---
format: aep.planning-md/3
id: story:controlled-worker-upgrades
kind: story
status: draft
title: Verify and apply offline worker upgrades with rollback
relations:
- decomposes: epic:reliability-and-usability
- depends_on: story:prebuilt-release-artifacts
scope:
- confidence: inferred
  path: README.md
- confidence: inferred
  path: crates/mantle-release
- confidence: inferred
  path: crates/mantle-worker
- confidence: inferred
  path: crates/mantle/src/adapters/ssh.rs
- confidence: inferred
  path: crates/mantle/src/app/worker.rs
- confidence: inferred
  path: crates/mantle/src/main.rs
- confidence: inferred
  path: generated/worker-model
- confidence: inferred
  path: spec/components.yaml
- confidence: inferred
  path: spec/domains/operator.yaml
- confidence: inferred
  path: spec/ess-inputs.yaml
- confidence: inferred
  path: spec/scenarios/cli
- confidence: inferred
  path: website/index.html
revision: 15
---
## Outcome
Provide mantle worker upgrade --check and explicit --apply consuming a verified matching release bundle. This is an OFFLINE Mantle binary upgrade, not an online daemon drain or a Substrate version migration. The pinned SDK has no global inventory/quiesce endpoint. Keeping that responsibility upstream avoids a freeze/encrypted/SQL workaround. UpgradeAssessment and ReleaseManifest are typed in spec/domains/operator.yaml; existing Worker owns placement.

## Acceptance
Named scenarios upgrade-current, upgrade-plan, upgrade-active-refusal, upgrade-unknown-refusal, upgrade-incompatible, upgrade-tampered, upgrade-apply, upgrade-rollback and upgrade-postcheck.
Check is bounded and read-only. It reports observed versions, artifact compatibility and maintenance prerequisites separately. Apply requires both substrate.service and mantle-egress.service already inactive and persistently or runtime masked, no queued service jobs, and their cgroup trees absent or conclusively empty. Unknown observations, active/pending jobs, nonempty cgroups, unmasked units, or unsupported unit/cgroup layout refuse before changing installed bytes. This observes worker-wide processes even if absent from Mantle records. Root-level external concurrent unmask/start is outside the exclusive operator maintenance contract and explicitly documented; acquire a host-wide upgrade lock and recheck prerequisites immediately before commit. Never stop, mask, unmask or restart services automatically; never signal sessions. Only administrators establish the maintenance window after deliberately retiring their sessions. A mask prevents normal systemd admission throughout the replacement transaction, including already connected client concerns because the daemon is absent.

Verify release manifest, targets, version and checksums before applying. Require the same pinned Substrate version; daemon changes need upstream maintenance support and are refused, not called applied. Stage and verify all Mantle binaries on the same filesystem; preserve exact previous bytes and modes, atomically switch the complete selected bundle, then verify installed versions/digests. Failure before commit leaves installed state unchanged; failure after commit restores the known previous bundle or reports exact partial state without claiming rollback. Interrupted transactions are inspectable and recoverable under the same lock. Preserve workspace volumes, configuration, secrets and ownership. Return applied-restart-required because services remain masked/inactive; installed version is not a claim about a live daemon.

Tests use local filesystem and process/service-port fixtures: every refusal has zero installation mutation calls, an untracked cgroup process refuses, a queued start refuses, a changed prerequisite refuses, concurrent installers cannot race, tampered artifacts refuse, and every staging/commit/postcheck/rollback boundary has failure injection. At least one eligible offline installation must succeed, so this is not an always-refuse implementation. No user's live worker is upgraded or put into maintenance during this task.

## Design decision after review round 2
The two blocker findings correctly rejected online idle-check-then-stop. The requirement is revised to explicit pre-established offline maintenance. This trades convenience for a supported, reviewable preservation boundary. No Substrate internals, database schema, signal-freeze protocol or new API is presumed. Online coordinated draining remains an upstream capability, not an unimplemented acceptance criterion disguised as complete.

## Scope
Cited: crates/mantle/src/app/worker.rs, crates/mantle/src/adapters/ssh.rs, crates/mantle-worker/src/lib.rs, crates/mantle-worker/src/main.rs, deploy/substrate.service and deploy/mantle-egress.service. Inferred: CLI dispatch, release verifier, offline installer transaction and failure fixtures, operator ESS scenarios/generated provenance, README and public website. Depends on packaging and doctor.

