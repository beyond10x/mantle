---
format: aep.planning-md/3
id: review-result:reliability-wave5-adversary-pass1
kind: review-result
status: active
title: Worker upgrade sequential installer preservation review
relations:
- reviews: story:controlled-worker-upgrades
revision: 1
---
unit: story:controlled-worker-upgrades at ac534be9bfe3ca4a274fe5d53b06ca3822a18ba5
verdict: CONFIRMED
cases: executed 11→12 transaction cases, red 1
origin: introduced 0 / pre-existing 0 / undecided 1
wrote-outside-worktree: 8 retained scratch files and 4 build/temp/cache/registry paths, listed below
needs-coordinator: route the retained failing regression to the implementor; origin needs coordinator classification

```text
$ git --no-pager diff --stat
 crates/mantle-worker/tests/offline_upgrade.rs | 57 +++++++++++++++++++++++++++
 1 file changed, 57 insertions(+)
```

Tests-only bound held. The only source change is the uncommitted Rust test in
`crates/mantle-worker/tests/offline_upgrade.rs`. Existing assertions and implementation are
unchanged. Source HEAD remains the candidate above. Base is
`c63d5a6ceb4c2817f01586a00cf75390c69a69ce`. No commits, AEP edits, live worker/service operations,
provider calls or credential inspection occurred.

1. Case written before execution

Added `adversary_completed_upgrade_retry_preserves_later_codex_installation` at
`crates/mantle-worker/tests/offline_upgrade.rs:401`. It starts with helpers and Claude but no
Codex stable link, successfully applies the bundle, models a supported Codex installation
serialized under the real HostLock, and applies the same bundle again. The assertion requires
the Codex relative stable link to remain present, and also checks its target, helper digest and
agent-current selection. It is RED now; the first assertion after the retry fails at line 438.

All commands ran in
`$HOME/.local/state/worktree/trees/b10x/mantle/mantle-worker-upgrades`, with:

```text
PATH=$HOME/.cache/aap-ess-upgrade/ess-0.50/bin:$PATH
TMPDIR=/dev/shm/mantle-worker-upgrades-tmp
CARGO_TARGET_DIR=$HOME/.cache/mantle-reliability/worker-upgrades-target
CARGO_BUILD_JOBS=2
CARGO_PROFILE_DEV_DEBUG=0
RUST_TEST_THREADS=1
RUSTC_WRAPPER=/usr/bin/sccache
SCCACHE_SERVER_PORT=43179
SCCACHE_DIR=/dev/shm/mantle-reliability-compiler-cache
SCCACHE_CACHE_SIZE=1G
SCCACHE_IDLE_TIMEOUT=0
```

The first isolated command was:

```text
cargo test --locked -p mantle-worker --test offline_upgrade adversary_completed_upgrade_retry_preserves_later_codex_installation -- --exact
exit: 101
   Compiling mantle-worker v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-worker-upgrades/crates/mantle-worker)
    Finished `test` profile [unoptimized] target(s) in 0.54s
     Running tests/offline_upgrade.rs ($HOME/.cache/mantle-reliability/worker-upgrades-target/debug/deps/offline_upgrade-f0814be18930a83f)

running 1 test
test adversary_completed_upgrade_retry_preserves_later_codex_installation ... FAILED

failures:

---- adversary_completed_upgrade_retry_preserves_later_codex_installation stdout ----

thread 'adversary_completed_upgrade_retry_preserves_later_codex_installation' (548042) panicked at crates/mantle-worker/tests/offline_upgrade.rs:417:5:
retry of a completed upgrade removed the subsequently installed Codex link: Ok(Assessment { applicable: true, boot_id: "11111111-1111-1111-1111-111111111111", observed_at: "1791035188", units: [UnitObservation { name: "substrate.service", state: "inactive", mask: "effective", job: "none", pids: "none", cgroup: "empty" }, UnitObservation { name: "mantle-egress.service", state: "inactive", mask: "effective", job: "none", pids: "none", cgroup: "empty" }], jobs: "none", layout: "supported", compatibility: "matching", outcome: "applied-restart-required", diagnostic: "Verified bundle activated. Services remain masked and inactive; administrator restart is required.", candidate_source: "1111111111111111111111111111111111111111", candidate_manifest: "16dba5855d5bc70979d7499c22e9a34f5522f25eae91fcca68393b2fd0ac7d5b", installed_versions: [] })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary_completed_upgrade_retry_preserves_later_codex_installation

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 9.98s

error: test failed, to rerun pass `-p mantle-worker --test offline_upgrade`
```

Output blocks preserve captured output except replacing the personal absolute home prefix with
`$HOME`. Raw originals are retained in the listed private logs.

2. Suite after the initial case

The before-count 11 comes from the implementor's handoff; no baseline suite was run before writing
the case. The full package run executed 30 library cases and 12 transaction cases. It stopped at
the failing integration target, before the separate CLI target and doc tests, so this report does
not claim the implementor's aggregate 42 became 43 executed cases.

```text
cargo test --locked -p mantle-worker
exit: 101
    Finished `test` profile [unoptimized] target(s) in 0.09s
     Running unittests src/lib.rs ($HOME/.cache/mantle-reliability/worker-upgrades-target/debug/deps/mantle_worker-dc055e33108a811f)

running 30 tests
test tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation ... ok
test tests::adversary_fifo_and_symlink_inputs_refuse_without_waiting_for_writers ... ok
test tests::adversary_live_lock_refuses_without_fetching_or_changing_current ... ok
test tests::adversary_operator_interrupt_reaps_bounded_child ... ok
test tests::adversary_refuses_binary_mismatch_after_valid_archive_before_execution ... ok
test tests::adversary_second_failed_spawn_preserves_following_transport_and_idle_state ... ok
test tests::adversary_second_successful_child_exit_cleans_its_remaining_group ... ok
test tests::adversary_second_update_observers_only_see_complete_verified_generations ... ok
test tests::adversary_unmanaged_executable_is_preserved_on_activation_refusal ... ok
test tests::adversary_unpublished_verified_generation_recovers_after_interrupted_activation ... ok
test tests::bounded_child_pumps_input_and_output_together ... ok
test tests::bounded_child_refuses_output_flood_and_timeout ... ok
test tests::bounded_live_process_deadline_and_drop_retire_descendants_without_caller_progress ... ok
test tests::byte_limit_detects_growth_beyond_declared_size ... ok
test tests::codex_installation_respects_host_upgrade_lock_before_fetching ... ok
test tests::concurrent_directory_winners_are_revalidated_without_permission_changes ... ok
test tests::corrupt_archive_never_activates ... ok
test tests::delivery_accepts_cargo_hardlinked_artifacts ... ok
test tests::failed_update_preserves_previous_binary_and_facts ... ok
test tests::generated_model_has_no_drift ... ok
test tests::idle_transport_coordinator_does_not_swallow_termination ... ok
test tests::installation_is_verified_atomic_and_idempotent ... ok
test tests::retirement_survives_a_bounded_registry_wait_timeout ... ok
test tests::stale_transport_guard_cannot_retire_a_reused_pid_registration ... ok
test tests::symlinked_root_is_refused ... ok
test tests::tampered_installed_bytes_are_refused ... ok
test tests::termination_cleanup_includes_owned_descendants ... ok
test tests::termination_coordinator_cleans_concurrent_groups_and_preserves_signal_status ... ok
test tests::unsupported_architecture_never_fetches_or_creates_paths ... ok
test tests::version_mismatch_never_activates ... ok

test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.43s

     Running unittests src/main.rs ($HOME/.cache/mantle-reliability/worker-upgrades-target/debug/deps/mantle_worker-dbb1340fad8796bd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/offline_upgrade.rs ($HOME/.cache/mantle-reliability/worker-upgrades-target/debug/deps/offline_upgrade-f0814be18930a83f)

running 12 tests
test adversary_completed_upgrade_retry_preserves_later_codex_installation ... FAILED
test changed_prerequisites_and_unsupported_or_tampered_inputs_leave_prior_bytes ... ok
test concurrent_first_adoption_rechecks_marker_only_under_host_lock ... ok
test crash_recovery_uses_actual_directory_identity_before_and_after_exchange ... ok
test deployed_units_missing_originals_pending_jobs_and_descendant_processes_refuse ... ok
test eligible_rendered_maintenance_applies_complete_bundle_and_preserves_agents ... ok
test fresh_provisioning_uses_private_delivery_and_defers_active_existing_helpers ... ok
test interrupted_upgrade_child ... ok
test offline_refusals_do_not_mutate_installed_inventory ... ok
test separate_process_installer_cannot_exchange_while_host_lock_is_held ... ok
test staging_exchange_postcheck_and_rollback_failures_report_observed_state ... ok
test systemd_mask_fragment_is_its_effective_source_path ... ok

failures:

---- adversary_completed_upgrade_retry_preserves_later_codex_installation stdout ----

thread 'adversary_completed_upgrade_retry_preserves_later_codex_installation' (572800) panicked at crates/mantle-worker/tests/offline_upgrade.rs:417:5:
retry of a completed upgrade removed the subsequently installed Codex link: Ok(Assessment { applicable: true, boot_id: "11111111-1111-1111-1111-111111111111", observed_at: "1791035257", units: [UnitObservation { name: "substrate.service", state: "inactive", mask: "effective", job: "none", pids: "none", cgroup: "empty" }, UnitObservation { name: "mantle-egress.service", state: "inactive", mask: "effective", job: "none", pids: "none", cgroup: "empty" }], jobs: "none", layout: "supported", compatibility: "matching", outcome: "applied-restart-required", diagnostic: "Verified bundle activated. Services remain masked and inactive; administrator restart is required.", candidate_source: "1111111111111111111111111111111111111111", candidate_manifest: "6742a535525795c440a2985d46f4099fd1650012175a24e9d0b6aaf259a8c8ee", installed_versions: [] })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary_completed_upgrade_retry_preserves_later_codex_installation

test result: FAILED. 11 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 139.61s

error: test failed, to rerun pass `-p mantle-worker --test offline_upgrade`
```

3. Fixture precision correction, then isolated case and suite again

The initial case left the shared fixture's preexisting `agents/codex/current` behind while
removing `bin/codex`. Before final reporting, the new case alone was strengthened to remove the
entire Codex subtree initially, then construct a coherent synthetic content-addressed executable,
manifest and current symlink under HostLock before adding the stable link. No preservation
assertion changed. This models Installer::install_with's published filesystem effect; the test
does not claim to invoke the actual download/install path, and the synthetic executable is a local
fixture ELF.

The corrected case was run alone first (`strict-case.log`) and failed at the same preservation
assertion. A repeated isolated execution captured the following direct process status:

```text
cargo test --locked -p mantle-worker --test offline_upgrade adversary_completed_upgrade_retry_preserves_later_codex_installation -- --exact
exit: 101
    Finished `test` profile [unoptimized] target(s) in 0.13s
     Running tests/offline_upgrade.rs ($HOME/.cache/mantle-reliability/worker-upgrades-target/debug/deps/offline_upgrade-f0814be18930a83f)

running 1 test
test adversary_completed_upgrade_retry_preserves_later_codex_installation ... FAILED

failures:

---- adversary_completed_upgrade_retry_preserves_later_codex_installation stdout ----

thread 'adversary_completed_upgrade_retry_preserves_later_codex_installation' (752368) panicked at crates/mantle-worker/tests/offline_upgrade.rs:438:5:
retry of a completed upgrade removed the subsequently installed Codex link: Ok(Assessment { applicable: true, boot_id: "11111111-1111-1111-1111-111111111111", observed_at: "1791035633", units: [UnitObservation { name: "substrate.service", state: "inactive", mask: "effective", job: "none", pids: "none", cgroup: "empty" }, UnitObservation { name: "mantle-egress.service", state: "inactive", mask: "effective", job: "none", pids: "none", cgroup: "empty" }], jobs: "none", layout: "supported", compatibility: "matching", outcome: "applied-restart-required", diagnostic: "Verified bundle activated. Services remain masked and inactive; administrator restart is required.", candidate_source: "1111111111111111111111111111111111111111", candidate_manifest: "0951ca15b48deb8f69c05f4774b983272daed751c3343010f3a52e3b13f6019c", installed_versions: [] })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary_completed_upgrade_retry_preserves_later_codex_installation

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 8.60s

error: test failed, to rerun pass `-p mantle-worker --test offline_upgrade`
```

Only then the corrected transaction suite ran:

```text
cargo test --locked -p mantle-worker --test offline_upgrade
exit: 101
    Finished `test` profile [unoptimized] target(s) in 0.09s
     Running tests/offline_upgrade.rs ($HOME/.cache/mantle-reliability/worker-upgrades-target/debug/deps/offline_upgrade-f0814be18930a83f)

running 12 tests
test adversary_completed_upgrade_retry_preserves_later_codex_installation ... FAILED
test changed_prerequisites_and_unsupported_or_tampered_inputs_leave_prior_bytes ... ok
test concurrent_first_adoption_rechecks_marker_only_under_host_lock ... ok
test crash_recovery_uses_actual_directory_identity_before_and_after_exchange ... ok
test deployed_units_missing_originals_pending_jobs_and_descendant_processes_refuse ... ok
test eligible_rendered_maintenance_applies_complete_bundle_and_preserves_agents ... ok
test fresh_provisioning_uses_private_delivery_and_defers_active_existing_helpers ... ok
test interrupted_upgrade_child ... ok
test offline_refusals_do_not_mutate_installed_inventory ... ok
test separate_process_installer_cannot_exchange_while_host_lock_is_held ... ok
test staging_exchange_postcheck_and_rollback_failures_report_observed_state ... ok
test systemd_mask_fragment_is_its_effective_source_path ... ok

failures:

---- adversary_completed_upgrade_retry_preserves_later_codex_installation stdout ----

thread 'adversary_completed_upgrade_retry_preserves_later_codex_installation' (758394) panicked at crates/mantle-worker/tests/offline_upgrade.rs:438:5:
retry of a completed upgrade removed the subsequently installed Codex link: Ok(Assessment { applicable: true, boot_id: "11111111-1111-1111-1111-111111111111", observed_at: "1791035668", units: [UnitObservation { name: "substrate.service", state: "inactive", mask: "effective", job: "none", pids: "none", cgroup: "empty" }, UnitObservation { name: "mantle-egress.service", state: "inactive", mask: "effective", job: "none", pids: "none", cgroup: "empty" }], jobs: "none", layout: "supported", compatibility: "matching", outcome: "applied-restart-required", diagnostic: "Verified bundle activated. Services remain masked and inactive; administrator restart is required.", candidate_source: "1111111111111111111111111111111111111111", candidate_manifest: "9266c456813bd668fad5ec31022f6228355b53c274a3f41d1fb29063509d3403", installed_versions: [] })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary_completed_upgrade_retry_preserves_later_codex_installation

test result: FAILED. 11 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 122.89s

error: test failed, to rerun pass `-p mantle-worker --test offline_upgrade`
```

The original 11 transaction cases remain green; the new case is the sole failure. No infrastructure
or quota failure occurred. `git diff --check` passed after the final test edit.

4. Finding

| File:line | Category / severity | Verdict / origin | Finding |
|---|---|---|---|
| crates/mantle-worker/src/upgrade.rs:510 | acceptance / blocker | CONFIRMED / undecided | Retrying a completed helper upgrade after the first supported Codex installation removes bin/codex while reporting applied-restart-required. |

What was measured: the real `upgrade::apply` transaction ran twice against a real temporary
filesystem and coherent synthetic bundle, with a serialized Codex installation filesystem effect
between them. At `crates/mantle-worker/tests/offline_upgrade.rs:438`, the stable symlink was absent;
the retry returned `Ok(Assessment { outcome: "applied-restart-required", ... })`. Both the isolated
corrected execution and its suite exited 101. This is removal of the stable entry, not merely a
dangling link assertion.

What reaches it: workers without Codex are supported by `inventory`, which requires the three
helpers but makes Codex optional. `mantle-worker install-codex` dispatches to
`Installer::production()?.install()`; `Installer::install_with` acquires the same host lock and
creates a missing `bin/codex` at `crates/mantle-worker/src/lib.rs:296–303`. It does not update the
completed upgrade journal's saved directory inventory. A later supported `upgrade-apply` (also
reached through laptop `worker upgrade --apply`) calls `upgrade::apply`. No concurrent
administrator action, service start, unsupported link or manual journal change is needed.

The completed-journal branch at line 510 calls `recover_locked` despite the journal's terminal
status. The saved next-directory inventory lacks the legitimate newly added Codex link, so the
postcheck at lines 309–317 enters rollback. Apply then archives the rolled-back journal and stages
a fresh helper bundle from the restored older bin inventory, which again lacks Codex. The final
successful assessment therefore conceals loss of the supported agent entry.

The accepted first-adoption contract explicitly preserves Codex's relative link and serializes
Codex installation with helper exchange. A shared lock prevents overlap, but does not make this
sequential workflow safe. The agent generation remains outside bin; the measured loss is the
stable executable entry, not deletion of downloaded agent data.

Suggested correction boundary: distinguish terminal completed transactions from unfinished
recovery and preserve valid agent installation changes made by supported writers after completion.
A refusal must also leave that newer supported inventory unchanged. Do not weaken the retained
preservation regression.

Origin remains undecided here because no executable base comparison was performed. Read-only diff
shows upgrade.rs was introduced in the candidate, but this pass does not claim a base execution.
The coordinator can classify origin using its governing procedure.

5. Other attacked boundaries

The existing 11 transaction cases were rerun and remained green: effective maintenance masks,
queued/unknown observations, descendant cgroups, changed prerequisites, tampered inputs, full
exchange/rollback, crash recovery, host-lock exclusion and stale provisioning under that lock.
The existing 30 library cases also passed during the full package run. Those are existing tests,
not new independent claims of exhaustive coverage.

This attack adds one sequential-writer preservation case. It does not certify real systemd
maintenance, remote SSH behavior or an actual downloaded Codex build. No second distinct finding
was inferred from source inspection alone.

6. Retained outputs, ownership and handoff

All external paths written by this pass are:

- `$HOME/.cache/mantle-reliability/worker-upgrades/adversary/case.log`
- `$HOME/.cache/mantle-reliability/worker-upgrades/adversary/suite.log`
- `$HOME/.cache/mantle-reliability/worker-upgrades/adversary/strict-case.log`
- `$HOME/.cache/mantle-reliability/worker-upgrades/adversary/strict-case-confirmed.log`
- `$HOME/.cache/mantle-reliability/worker-upgrades/adversary/strict-suite.log`
- `$HOME/.cache/mantle-reliability/worker-upgrades/adversary/case.patch`
- `$HOME/.cache/mantle-reliability/worker-upgrades/adversary/lease-end.log`
- `$HOME/.cache/mantle-reliability/worker-upgrades/adversary/review.md` (this report)
- `$HOME/.cache/mantle-reliability/worker-upgrades-target` (assigned existing compiler target, 3.1 GiB)
- `/dev/shm/mantle-worker-upgrades-tmp` (assigned test/build temporary root, 65 MiB observed)
- `/dev/shm/mantle-reliability-compiler-cache` (assigned shared bounded sccache store, 1018 MiB observed)
- `$HOME/.local/state/worktree/registry.sqlite3` (worktree CLI-owned lease/heartbeat state only)

Scratch directory metadata was also created/updated at the assigned adversary root. No build
directory, managed tree or prior output was deleted. Root free space was 11 GiB and tmpfs free
space 13 GiB at the final storage observation; neither global nor user-quota allocation failed.

All test commands have exited. Own lease `codex-worker-upgrades-adversary` ended:

```text
session-end $HOME/.local/state/worktree/trees/b10x/mantle/mantle-worker-upgrades
```

The tree remains on `unit/mantle-worker-upgrades`, with only the uncommitted test addition.
The coordinator owns handoff to the implementor and eventual managed cleanup. This is adversary
pass 1; it is not approval, an independent-verifier claim or publication.

```findings
- file: crates/mantle-worker/src/upgrade.rs
  line: 510
  category: acceptance
  severity: blocker
  verdict: CONFIRMED
  origin: undecided
  message: Retrying a completed helper upgrade after the first supported Codex installation removes bin/codex while reporting applied-restart-required.
```

