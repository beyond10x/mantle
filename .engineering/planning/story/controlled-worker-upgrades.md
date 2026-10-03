---
format: aep.planning-md/3
id: story:controlled-worker-upgrades
kind: story
status: active
title: Verify and apply offline worker upgrades with rollback
relations:
- decomposes: epic:reliability-and-usability
- depends_on: story:prebuilt-release-artifacts
scope:
- confidence: cited
  path: Cargo.lock
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
  path: crates/mantle/Cargo.toml
- confidence: cited
  path: crates/mantle/src/adapters/orchestration.rs
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
revision: 33
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T08:57:35Z", actor: "human:timo", revision: 22, decided_on: {"recorded":{"review_outcome":2}}}
- {from: "proposed", to: "active", at: "2026-10-03T12:22:48Z", actor: "human:timo", revision: 29, decided_on: {"recorded":{"review_outcome":2}}}
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

## Effective maintenance on existing workers

Read-only preparation found that the supplied cloud-init writes regular root-owned 0644 unit files in /etc/systemd/system (deploy/cloud-init.yaml:51 and :56). Ordinary masking fails over such local files, and /etc takes precedence over /run, so runtime masking alone is not an effective maintenance barrier. See systemd v255 [mask documentation](https://github.com/systemd/systemd/blob/v255/man/systemctl.xml#L956) and [unit lookup order](https://github.com/systemd/systemd/blob/v255/man/systemd.unit.xml#L307). No live worker or service was queried; these are source/documentation observations, not conformance evidence.

Supported first adoption requires operator-prepared preserved originals at /var/lib/mantle/maintenance/units: a real root-owned 0700 directory and the two original, singly linked root:root 0644 unit files. Match egress against the supplied unit and Substrate against either exact supported rendered variant from crates/mantle/src/app/worker.rs::render_user_data_for (with or without Claude's path-only condition/slot). Reject different or missing originals, aliases, transient units, alternate slices and unsupported applicable drop-ins as layout unknown. Observe relevant unit-specific, mantle-.service.d and generic service.d locations in the supported manager search path; masked DropInPaths alone does not prove the original layout. Share the existing rendering/validation definition rather than maintaining subtly different copies.

The operator deliberately retires sessions, stops services, preserves/moves the original /etc unit files into that directory, installs effective masks and reloads systemd. Neither upgrade check nor apply performs this preparation. Leaving maintenance explicitly removes the masks, restores the preserved files, reloads and starts the chosen services. The documentation must explain these actual steps and preservation; do not advertise a mask command that cannot work on the deployed layout. Existing configuration and original units remain outside the helper-bundle transaction.

Observe each fixed unit separately with bounded, fixed-locale systemctl show properties: Id, Names, LoadState, ActiveState, SubState, UnitFileState, FragmentPath, NeedDaemonReload, Job, MainPID, ControlPID, ControlGroup and Transient. Require effective masked load/file states, inactive state, exact identity, no reload needed, zero PIDs and no job. A successful show exit does not prove existence; absent units can return success. In v255 no job is rendered as empty Job=, not Job=0; missing, duplicate or malformed properties are unknown. Also observe complete list-jobs for the two units, root cgroup2 mount identity and a stable boot ID across the assessment. Unsupported commands, permission errors, truncation or exceeded overall deadlines remain unknown. [systemctl show implementation](https://github.com/systemd/systemd/blob/v255/src/systemctl/systemctl-show.c#L1023).

Always inspect the fixed supported /sys/fs/cgroup/system.slice/substrate.service and mantle-egress.service paths even if ControlGroup is empty. systemd [unit_release_cgroup](https://github.com/systemd/systemd/blob/v255/src/core/cgroup.c#L2803) can forget the property without destroying the cgroup. An existing directory needs one valid populated=0 observation from bounded cgroup.events; that value covers the full descendant tree according to the [kernel cgroup-v2 contract](https://docs.kernel.org/admin-guide/cgroup-v2.html#un-populated-notification). Confirm actual ENOENT under a verified readable parent before calling a subtree absent; access/I/O failures are unknown. Older-worker read-only checks may use bounded parent enumeration and file observations through existing SSH commands, without uploading a new checker. Apply repeats the observations locally under the host lock immediately before exchange.

Update the existing ESS assessment before code to carry real known/unknown unit, layout, job, PID, cgroup and assessment/reason observations. A boot identity and observation time can bind one assessment; do not invent active_execs from these facts. Add regression fixtures for deployed regular /etc units, ineffective runtime masks, absent-show-success, empty Job, empty ControlGroup with populated descendants and missing originals. At least one fixture must use the actual supported rendered units and feasible maintenance layout and succeed. The exclusive operator-maintenance assumption still excludes external root changes; this preparation does not introduce automatic stop/drain or a Substrate API dependency.

## Prebuilt provisioning and first adoption

The completed package unit provides both verified archives, while existing fresh provisioning already accepts `mantle worker up --binaries DIR` (crates/mantle/src/main.rs:106 and app/worker.rs:281). The default DIR points at a source-build target, so the public prebuilt workflow must show the supported explicit path rather than implicitly requiring a Rust build.

Document verification of the trusted complete bundle, extraction of only the three known static worker payloads from its verified musl archive into a fresh operator-owned local staging directory, and `mantle worker up --binaries <stage>/bin` for fresh provisioning. No new downloader, package manager or worker activation command is required for this handoff. Existing workers use the explicit offline upgrade path with its maintenance checks. Keep filenames and bounded delivery verification compatible.

The implementation also needs to examine serialization of the current worker-up helper reconciliation with first adoption, not only the managed-marker happy path. A check made before acquiring the host lock must not authorize later overwrites after a bundle transaction commits. Preserve the accepted host-wide lock, managed-bundle refusal and exclusive operator-maintenance boundary; do not introduce a committed shell checker to work around missing old-helper commands.

## Implementation scope refinement

Implementor inspection at the published c63d5a6 base identified three additional cited seams: crates/mantle/src/adapters/orchestration.rs hosts the native ESS routing for the named upgrade cases; crates/mantle/Cargo.toml needs artifact verification as a production dependency; Cargo.lock records worker artifact/serde dependency edges. Typed scope now includes them. Shared unit rendering moves into the worker library and existing render_user_data_for reuses it. This is within the accepted offline upgrade behavior; implementation remains serial and does not overlap a second writing unit.
