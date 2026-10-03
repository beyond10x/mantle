---
format: aep.planning-md/3
id: review-result:reliability-wave5-adversary-pass2
kind: review-result
status: active
title: Final worker upgrade interruption and preservation attack
relations:
- reviews: story:controlled-worker-upgrades
revision: 1
---
unit: story:controlled-worker-upgrades at a7ac0ca5c46a72a91e834d18b5cb01962563b800
verdict: nothing found
cases: executed 46→48 worker cases, red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: 6 retained scratch files and 4 build/temp/cache/registry paths, listed below
needs-coordinator: retain the two uncommitted test additions and record this final pass

```text
$ git --no-pager diff --stat
 crates/mantle-worker/tests/offline_upgrade.rs | 127 ++++++++++++++++++++++++++
 1 file changed, 127 insertions(+)
```

The only worktree change is 127 appended test lines. No existing case or assertion was changed,
including the retained first-pass regression. No production, generated, planning or documentation
file was edited. Source HEAD remains the candidate above on `unit/mantle-worker-upgrades`.

This is the second and final formal adversary attack. I read the correction diff from
`ac534be9bfe3ca4a274fe5d53b06ca3822a18ba5`, its final report/status, the original implementation
handoff and first review, and the persisted adversary/invariant/story contracts. No test ran before
the new cases were written. The baseline is the implementor's reported 46 worker cases:
31 library, 14 transaction, and 1 CLI case.

1. New cases, written before first execution

Both cases live in `crates/mantle-worker/tests/offline_upgrade.rs`:

- `adversary_later_codex_survives_next_bundle_marker_crash_and_recovery`, line 595:
  after completing the first bundle, model a coherent first Codex installation under the real
  HostLock, select a distinct source manifest, and run the actual transaction in a controlled
  child that exits 77 immediately after writing its marker. Both local and remote checks must
  suppress current while the journal is pending; apply must recover the selected source without
  losing any installed bytes, Codex stable link or current generation; another apply must report
  current. The synthetic agent generation models the real installer's filesystem effect and does
  not claim to download or authenticate Codex.
- `adversary_invalid_later_codex_entry_refuses_without_historical_restore`, line 675:
  after an upgrade that did not contain Codex, insert either a wrong-target symlink or a regular
  file at the newly permitted Codex name. Local check must not report current, remote check must
  report unknown/inapplicable, apply must refuse, and current inventory, journal and marker must
  remain byte-identical. These intentionally unsupported filesystem states probe refusal behavior;
  they are not alleged outputs of the supported Codex installer.

Both cases were GREEN on their first isolated execution. There is no red output or failing finding
to manufacture.

All commands ran in
`$HOME/.local/state/worktree/trees/b10x/mantle/mantle-worker-upgrades`, using:

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

First isolated case, direct exit 0:

```text
cargo test --locked -p mantle-worker --test offline_upgrade adversary_later_codex_survives_next_bundle_marker_crash_and_recovery -- --exact
   Compiling mantle-worker v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-worker-upgrades/crates/mantle-worker)
    Finished `test` profile [unoptimized] target(s) in 0.54s
     Running tests/offline_upgrade.rs ($HOME/.cache/mantle-reliability/worker-upgrades-target/debug/deps/offline_upgrade-f0814be18930a83f)

running 1 test
test adversary_later_codex_survives_next_bundle_marker_crash_and_recovery ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 21.62s

```

Second isolated case, direct exit 0:

```text
cargo test --locked -p mantle-worker --test offline_upgrade adversary_invalid_later_codex_entry_refuses_without_historical_restore -- --exact
    Finished `test` profile [unoptimized] target(s) in 0.11s
     Running tests/offline_upgrade.rs ($HOME/.cache/mantle-reliability/worker-upgrades-target/debug/deps/offline_upgrade-f0814be18930a83f)

running 1 test
test adversary_invalid_later_codex_entry_refuses_without_historical_restore ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 21.66s

```

Output blocks below and above preserve captured output except replacing the personal absolute home
prefix with `$HOME`. Raw original logs are retained privately at the listed paths.

2. Relevant suite, only after both isolated executions

```text
cargo test --locked -p mantle-worker
exit: 0
    Finished `test` profile [unoptimized] target(s) in 0.20s
     Running unittests src/lib.rs ($HOME/.cache/mantle-reliability/worker-upgrades-target/debug/deps/mantle_worker-dc055e33108a811f)

running 31 tests
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
test upgrade::tests::pending_helper_transaction_excludes_codex_writer_before_fetch_or_agent_paths ... ok

test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.56s

     Running unittests src/main.rs ($HOME/.cache/mantle-reliability/worker-upgrades-target/debug/deps/mantle_worker-dbb1340fad8796bd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/offline_upgrade.rs ($HOME/.cache/mantle-reliability/worker-upgrades-target/debug/deps/offline_upgrade-f0814be18930a83f)

running 16 tests
test adversary_completed_upgrade_retry_preserves_later_codex_installation ... ok
test adversary_invalid_later_codex_entry_refuses_without_historical_restore ... ok
test adversary_later_codex_survives_next_bundle_marker_crash_and_recovery ... ok
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
test terminal_journal_refusal_never_restores_stale_inventory ... ok
test terminal_journals_do_not_replay_rollback_after_a_supported_later_writer ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 248.56s

     Running tests/upgrade_cli.rs ($HOME/.cache/mantle-reliability/worker-upgrades-target/debug/deps/upgrade_cli-43e6b8918dfba113)

running 1 test
test offline_upgrade_commands_are_available_without_agent_initialization ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

   Doc-tests mantle_worker

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

Executed 31 library + 16 transaction + 1 CLI cases = 48; no skipped or failed cases.
The zero-case main/doc targets are not counted. Controlled interruption children are not
double-counted as additional top-level cases.

The original first-pass regression now passes unchanged. The correction's completed-journal
preservation/refusal tests and pending-transaction Installer exclusion test also pass in this run.
Neither the Mantle package nor native CLI conformance was rerun here; their 64 and 273 green counts
remain attributed to the implementor's correction handoff, not this adversary.

Formatting was checked with
`rustfmt --check --edition 2024 crates/mantle-worker/tests/offline_upgrade.rs`; exit 0, no output.
`git diff --check` also passed.

3. Findings

Nothing found. The first-pass finding is not carried as a live finding: the identical retained
assertion passed, and this pass added successful interruption and refusal probes around the
corrected behavior. The first review remains unchanged at
`$HOME/.cache/mantle-reliability/worker-upgrades/adversary/review.md`; its origin classification
and correction outcome are coordinator-owned records.

4. What was attacked without breaking

- A later supported Codex installation, followed by a new bundle whose child exits after marker
  persistence: pending ownership remains visible and recovery preserves the agent.
- Wrong-target symlink and regular-file additions at the allowed Codex inventory name:
  unsupported entries refuse without historical restoration or transaction-record changes.
- Existing complete/rolled-back/not-applied journal cases and pending Installer exclusion:
  rerun in the actual worker suite and green.
- Existing maintenance, queued-job, descendant-cgroup, locking, rollback and interruption cases:
  rerun in the worker suite and green.

These are local filesystem/process/service-observation fixtures. This pass makes no claim about
actual remote workers, systemd execution, SSH deployment, authentication, or a downloaded Codex
binary. No live service, session, provider or credential operation occurred. This is an agent
review with test-runner results, not approval or an independent-verifier claim.

5. External paths and handoff

Every external path written by this pass is:

- `$HOME/.cache/mantle-reliability/worker-upgrades/adversary/pass2/crash-case.log`
- `$HOME/.cache/mantle-reliability/worker-upgrades/adversary/pass2/invalid-entry-case.log`
- `$HOME/.cache/mantle-reliability/worker-upgrades/adversary/pass2/worker-suite.log`
- `$HOME/.cache/mantle-reliability/worker-upgrades/adversary/pass2/cases.patch`
- `$HOME/.cache/mantle-reliability/worker-upgrades/adversary/pass2/lease-end.log`
- `$HOME/.cache/mantle-reliability/worker-upgrades/adversary/pass2/review.md` (this report)
- `$HOME/.cache/mantle-reliability/worker-upgrades-target` (assigned compiler target, 3.1 GiB observed)
- `/dev/shm/mantle-worker-upgrades-tmp` (assigned temporary root, 57 MiB observed)
- `/dev/shm/mantle-reliability-compiler-cache` (assigned shared bounded compiler cache, 1.1 GiB observed)
- `$HOME/.local/state/worktree/registry.sqlite3` (CLI-owned lease and heartbeat writes)

The new pass2 scratch directory metadata was created under the assigned adversary root.
No prior review/log, target or managed tree was removed or overwritten. Final global free space was
22 GiB on root and 9.6 GiB on tmpfs. No allocation or quota error occurred.

All commands exited before handoff. Only my own lease, `codex-worker-upgrades-adversary`, was ended:

```text
session-end $HOME/.local/state/worktree/trees/b10x/mantle/mantle-worker-upgrades
```

The managed tree remains at
`$HOME/.local/state/worktree/trees/b10x/mantle/mantle-worker-upgrades`, with only the two appended
tests uncommitted. The coordinator owns committing the tests, integration and eventual cleanup.
No commits, pushes, planning-store writes or third attack were performed.

```findings
[]
```

