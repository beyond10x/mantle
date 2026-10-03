---
format: aep.planning-md/3
id: story:controlled-worker-upgrades
kind: story
status: proposed
title: Verify and apply offline worker upgrades with rollback
relations:
- decomposes: epic:reliability-and-usability
- depends_on: story:prebuilt-release-artifacts
scope:
- confidence: inferred
  path: README.md
- confidence: inferred
  path: crates/mantle-artifact
- confidence: inferred
  path: crates/mantle-release
- confidence: inferred
  path: crates/mantle-worker
- confidence: cited
  path: crates/mantle-worker/src/lib.rs
- confidence: cited
  path: crates/mantle-worker/src/main.rs
- confidence: cited
  path: crates/mantle/src/adapters/ssh.rs
- confidence: cited
  path: crates/mantle/src/app/worker.rs
- confidence: inferred
  path: crates/mantle/src/main.rs
- confidence: cited
  path: deploy/mantle-egress.service
- confidence: cited
  path: deploy/substrate.service
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
revision: 26
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T08:57:35Z", actor: "human:timo", revision: 22, decided_on: {"recorded":{"review_outcome":2}}}
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

## Shared verifier dependency layout

Read-only implementation preparation identified a concrete dependency cycle if the shared verifier remains in mantle-release: release commands need mantle-worker's existing bounded subprocess ownership, while worker upgrades need the verifier. Put strict manifest/archive/ELF verification in a small leaf crate crates/mantle-artifact, with no dependency on the application or worker. mantle-release depends on mantle-artifact plus mantle-worker; the later upgrade makes mantle-worker depend on mantle-artifact. Reuse the existing process owner instead of copying it or extracting a second orchestration library. This models existing ReleaseManifest/ReleaseArtifact values and does not introduce a new product noun.

## Existing layout and first adoption

Read-only preparation found that /opt/mantle/bin must remain a real directory: mantle-worker Installer::inspect/trusted_ancestors requires it and requires bin/codex to point to ../agents/codex/current/codex. The smallest atomic legacy-compatible transaction stages a complete sibling bin directory on the same filesystem, preserves Claude bytes/mode/ownership and Codex's exact relative symlink, adds the three verified helpers, and exchanges the directories with Linux renameat2(RENAME_EXCHANGE). The existing nix fs feature exposes that primitive. Refuse unsupported layouts or exchange semantics without falling back to sequential replacement. Retain the exchanged old directory for exact rollback. Journal and sync both directory identities/inventories before exchange; reconcile an exchange-before-journal-update crash from actual identities/digests, not a phase flag alone. Do not modify agents/codex/current.

The existing app/worker.rs install_binaries path replaces helpers independently using shared .new names and starts egress. Managed bundle installations must refuse legacy binary reconciliation or route through the supported transaction; ordinary worker up must not silently invalidate the selected bundle. Serialize directory exchange and Codex installation through a shared host-wide lock with consistent lock ordering; its current agent-specific install.lock alone does not cover this race.

An older installed mantle-worker has no upgrade command. First apply may upload and execute an explicitly verified temporary helper outside installed paths, with bounded transport and owned cleanup. Read-only check must not upload a helper or provision anything; use bounded existing read-only observations. The supplied Substrate unit delegates its execution descendants under /sys/fs/cgroup/system.slice/substrate.service, so inspect the full validated subtree rather than only MainPID. Keep the offline-only and no-live-operation boundaries already stated.
