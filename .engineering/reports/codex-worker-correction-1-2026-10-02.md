unit: story:agent-ready-worker — correction 1
verdict: green (offline correction gates; coordinator rebase, independent review and live acceptance pending)
cases: executed 55→61, red 0 in final suite; two introduced review findings and one additional red ownership regression corrected
origin: n/a
wrote-outside-worktree: ~/.cache/mantle-wave3/scratch-worker/ files enumerated below; /dev/shm/mantle-wave3-target-worker/ build and temporary test artifacts; own worktree lease metadata
needs-coordinator: no source patches; coordinator owns latest-main rebase, second independent review and live AW acceptance

## 1. Corrections and bounded classes

The immutable first attack is adversary-1.md, recorded by the coordinator as review-result:codex-worker-adversary-1. Both introduced findings are corrected. All seven adversary tests and their assertions remain. No AEP, live system, index, commit or rebase action was performed in this correction.

Cancellation fix: one initialized signal coordinator owns every bounded transport's Child handle and registration identity. SIGINT, SIGTERM and SIGHUP request cleanup of registered process groups before preserving the original signal's default termination. Mantle initializes this before starting Tokio threads; the helper initializes after clap parsing; library callers have the same lazy initialization. Idle signals keep default behavior. Spawn and registration serialize against cancellation; failed spawn restores idle handling. Completion, error and unwinding request retirement through a guard. A retirement request is recorded before attempting the bounded registry lock, so a stalled owner cannot lose cleanup ownership. Direct children retain their PID until owned-group kill and reaping. A registration-specific Arc identity prevents a stale guard from targeting another registration after PID reuse.

Cancellation class enumeration: production executable version checks and bounded SSH delivery share run_bounded; normal exit, deadline, output limit, I/O/setup errors, unwinding, registration/spawn failure, concurrent active calls, interruption during active calls, idle interruption, owned descendants, lock contention and stale registration identity were inspected. Six added tests exercise all three signals in isolated child processes with concurrent calls and idle calls; exact signal exit status; direct-child reaping; unrelated process survival; same-group descendants; bounded lock wait with deferred cleanup; and stale-guard identity. Existing deadline/output/input-pump tests remain. Initialization-failure fallback and every possible I/O error or signal/spawn interleaving are inspected paths, not claims of exhaustive dynamic coverage.

Directory-race fix: the four shared directories (installation root, agents, agents/codex, bin) attempt creation, then revalidate an AlreadyExists winner using the same ancestor/owner/symlink/mode checks. The winner is never chmodded. Newly created directories retain required publication permissions through an owned non-symlink directory descriptor. Unique staging/activation directories remain temporary private allocations; the existing installer lock serializes generation/current publication.

Directory class enumeration: existing safe directory, simultaneous safe creators, unsafe writable directory, symlink and regular-file winners are covered. The original two-install test verifies one download and coherent Installed/AlreadyCurrent results. The new eight-creator case plus unsafe winners verify no permission repair or weakening. Foreign-owner rejection remains the existing validation path; no privileged foreign-UID mutation test is claimed.

## 2. Source changes

The following is the unstaged diff relative to the coordinator's staged first implementation. It includes the retained seven adversary tests as well as this correction; it is not a diff against the original repository base.

```text
 Cargo.lock                       |   1 +
 crates/mantle-worker/Cargo.toml  |   1 +
 crates/mantle-worker/src/lib.rs  | 848 ++++++++++++++++++++++++++++++++++++++-
 crates/mantle-worker/src/main.rs |   1 +
 crates/mantle/src/main.rs        |  11 +-
 5 files changed, 844 insertions(+), 18 deletions(-)
```

signal-hook 0.3.18 was already in the dependency cache/lock; the manifest and lock add the worker's dependency edge. No new dependency download was required. All permanent executable source remains Rust.

## 3. Red evidence and preservation

The review's scheduling-dependent first-install race failed both in its first selected suite and alone. Its later full suite passed that case but failed the SIGINT case: 55 top-level tests, 54 passed and 1 failed. That does not erase the separately reproduced race. The immutable standalone race output is retained verbatim here:

```text
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
    Finished `test` profile [unoptimized] target(s) in 0.60s
     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-e379049379d396fa)

running 1 test

thread '<unnamed>' (4115397) panicked at crates/mantle-worker/src/lib.rs:577:22:
called `Result::unwrap()` on an `Err` value: File exists (os error 17)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation' (4115395) panicked at crates/mantle-worker/src/lib.rs:581:27:
called `Result::unwrap()` on an `Err` value: Any { .. }
test tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation ... FAILED

failures:

failures:
    tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.09s

error: test failed, to rerun pass `-p mantle-worker --lib`
```

The original SIGINT case was reproduced before the correction with cargo test --locked -p mantle-worker adversary_operator_interrupt_reaps_bounded_child -- --nocapture, exit 101:

```text
    Finished `test` profile [unoptimized] target(s) in 0.25s
     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-e379049379d396fa)

running 1 test

thread 'tests::adversary_operator_interrupt_reaps_bounded_child' (55529) panicked at crates/mantle-worker/src/lib.rs:747:9:
operator SIGINT left the owned bounded child running after its parent exited
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test tests::adversary_operator_interrupt_reaps_bounded_child ... FAILED

failures:

failures:
    tests::adversary_operator_interrupt_reaps_bounded_child

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.11s

error: test failed, to rerun pass `-p mantle-worker --lib`
```

Class enumeration then exposed a stale-guard/PID-reuse hole in the initial correction. Its new test failed before the registration-identity fix: cargo test --locked -p mantle-worker stale_transport_guard_cannot_retire_a_reused_pid_registration, exit 101.

```text
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
    Finished `test` profile [unoptimized] target(s) in 0.75s
     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-52fb9b1b91688ca5)

running 1 test
test tests::stale_transport_guard_cannot_retire_a_reused_pid_registration ... FAILED

failures:

---- tests::stale_transport_guard_cannot_retire_a_reused_pid_registration stdout ----

thread 'tests::stale_transport_guard_cannot_retire_a_reused_pid_registration' (200131) panicked at crates/mantle-worker/src/lib.rs:833:9:
stale cleanup targeted a different registration that reused its PID
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::stale_transport_guard_cannot_retire_a_reused_pid_registration

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 24 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p mantle-worker --lib`
```

All three cases are green in the final full suite below. correction-1-helper.log was an E0716 compile failure in a newly added scoped-thread fixture, and correction-1-clippy.log was a collapsible_if lint in new fixture cleanup. Both were corrected without weakening assertions; neither is counted as a red behavioral test.

## 4. Actual final gates and artifacts

All Cargo commands used CARGO_TARGET_DIR=/dev/shm/mantle-wave3-target-worker, TMPDIR=/dev/shm/mantle-wave3-target-worker/tmp, RUSTC_WRAPPER empty, CARGO_BUILD_JOBS=2, CARGO_PROFILE_DEV_DEBUG=0, CARGO_PROFILE_TEST_DEBUG=0 and CARGO_INCREMENTAL=0. Commands ran serially. Before final builds root had 45 GiB available and tmpfs 28 GiB, above the assigned 2/10 GiB floors.

cargo test --locked -p mantle -p mantle-worker --all-targets: exit 0. Top-level counts are mantle 26→26, qualification 10→10, worker helper 19→25, total 55→61. Nested qualification subprocess summaries are not additional cases. Final result: 61 passed, 0 failed, 0 ignored. Six new helper tests supplement all seven retained adversary tests.

```text
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 4.34s
     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle-a303c75065fe9809)

running 26 tests
test app::terminal::tests::ctrl_bracket_d_detaches_and_forwards_what_came_before ... ok
test app::terminal::tests::escape_followed_by_anything_else_passes_both_bytes ... ok
test app::terminal::tests::the_sequence_is_recognised_across_reads ... ok
test adapters::substrate::worker_tests::common_readiness_and_selected_claude_are_distinct_observations ... ok
test config::tests::explicit_path_selection_is_absolute_and_preserves_defaults ... ok
test config::tests::common_worker_configuration_needs_no_claude_credentials ... ok
test config::tests::legacy_claude_table_remains_parseable ... ok
test domain::manifest::tests::defaults_apply ... ok
test app::worker::tests::legacy_claude_bootstrap_retains_required_slot ... ok
test domain::manifest::tests::cwd_must_be_inside_a_mount ... ok
test domain::manifest::tests::sizes_and_durations_parse ... ok
test app::worker::tests::the_template_renders_every_placeholder ... ok
test domain::session::tests::names_round_trip ... ok
test domain::session::tests::running_is_never_reached_without_starting ... ok
test domain::session::tests::stopped_is_final_and_failures_only_stop ... ok
test domain::session::tests::the_happy_path_is_legal ... ok
test domain::manifest::tests::retain_for_is_bounded_by_the_exec_limit ... ok
test domain::manifest::tests::unserved_fields_are_refused_by_name ... ok
test domain::manifest::tests::the_example_manifest_resolves ... ok
test domain::manifest::tests::cwd_with_dot_dot_or_empty_components_is_refused ... ok
test domain::manifest::tests::mounts_and_refs_cannot_escape ... ok
test adapters::state::tests::an_illegal_move_is_refused_and_not_written ... ok
test adapters::state::tests::sources_round_trip ... ok
test adapters::state::tests::a_live_name_is_unique_until_stopped ... ok
test app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command ... ok
test adapters::state::conformance::ess_generated_local_conformance ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

     Running unittests examples/codex_qualification.rs (/dev/shm/mantle-wave3-target-worker/debug/examples/codex_qualification-597f4304b95b6e31)

running 10 tests
test tests::digest_format_is_strict ... ok
test tests::digest_bounds_actual_input_even_when_it_exceeds_the_size_snapshot ... ok
test tests::digest_refuses_other_non_regular_descriptors ... ok
test tests::fixed_observation_never_exports_terminal_payload ... ok
test tests::unsafe_scratch_parents_are_refused_without_creating_children ... ok
test tests::scratch_budget_refuses_large_files_and_does_not_follow_links ... ok

running 1 test

running 1 test
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok

test result: test adversary::binary_symlink_is_validated_against_its_current_opened_object ... okok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out
; finished in 0.00s


test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

test tests::terminal_observation_is_bounded_and_handles_split_queries ... ok
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok
test tests::cq01_rejects_wrong_digest_before_executing_a_binary ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.31s

     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-7690d03aee1459ac)

running 25 tests
test tests::byte_limit_detects_growth_beyond_declared_size ... ok
test tests::delivery_accepts_cargo_hardlinked_artifacts ... ok
test tests::stale_transport_guard_cannot_retire_a_reused_pid_registration ... ok
test tests::concurrent_directory_winners_are_revalidated_without_permission_changes ... ok
test tests::symlinked_root_is_refused ... ok
test tests::unsupported_architecture_never_fetches_or_creates_paths ... ok
test tests::adversary_refuses_binary_mismatch_after_valid_archive_before_execution ... ok
test tests::corrupt_archive_never_activates ... ok
test tests::termination_cleanup_includes_owned_descendants ... ok
test tests::generated_model_has_no_drift ... ok
test tests::adversary_unmanaged_executable_is_preserved_on_activation_refusal ... ok
test tests::adversary_fifo_and_symlink_inputs_refuse_without_waiting_for_writers ... ok
test tests::installation_is_verified_atomic_and_idempotent ... ok
test tests::version_mismatch_never_activates ... ok
test tests::tampered_installed_bytes_are_refused ... ok
test tests::idle_transport_coordinator_does_not_swallow_termination ... ok
test tests::failed_update_preserves_previous_binary_and_facts ... ok
test tests::termination_coordinator_cleans_concurrent_groups_and_preserves_signal_status ... ok
test tests::adversary_unpublished_verified_generation_recovers_after_interrupted_activation ... ok
test tests::bounded_child_pumps_input_and_output_together ... ok
test tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation ... ok
test tests::bounded_child_refuses_output_flood_and_timeout ... ok
test tests::adversary_operator_interrupt_reaps_bounded_child ... ok
test tests::retirement_survives_a_bounded_registry_wait_timeout ... ok
test tests::adversary_live_lock_refuses_without_fetching_or_changing_current ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.02s

     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-18bf116e4ea3a749)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

cargo fmt -p mantle -p mantle-worker --check: exit 0; correction-1-fmt-final.log is empty.
git diff --check: exit 0, empty output.
cargo clippy --locked -p mantle -p mantle-worker --all-targets -- -D warnings: exit 0.

```text
    Checking mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
    Checking mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `dev` profile [unoptimized] target(s) in 2.98s
```

ess specify validate --path spec: exit 0. The ESS conformance test in the package gate still executes the same 65 scenarios, zero failures/skips; model digest remains e4ed1d26c2e578d0af56d2aa8813a50292167069d9a6ba58b0bac6e649148d8a. The original two ESS-SYNTH-013 refusals remain; no generated contract was changed by this correction.

```text
mantle v1 — 6 file(s), 1 scenario(s), valid
```

cargo build --locked --release --target x86_64-unknown-linux-musl -p mantle-worker -p mantle-egress -p mantle-launch: exit 0.

```text
   Compiling signal-hook v0.3.18
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
    Finished `release` profile [optimized] target(s) in 2.42s
```

cargo build --locked -p mantle: exit 0.

```text
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `dev` profile [unoptimized] target(s) in 3.79s
```

Both rebuilt mantle --help and static mantle-worker --help exited 0. Fresh SHA-256 identities, superseding the first implementation's worker and CLI hashes:

```text
797c4d4f5d762dadf2895e977c50f297c02fd0293125f2254826b5d6e0790760  /dev/shm/mantle-wave3-target-worker/x86_64-unknown-linux-musl/release/mantle-worker
ac8f3fcbd9ea7b14422aecb14c351b998950530e37529b9fe3a84e579228aad3  /dev/shm/mantle-wave3-target-worker/x86_64-unknown-linux-musl/release/mantle-egress
593823a5198b7f7180edc32392948ff703367460dbbc20c2e2c60d29cf9fbc45  /dev/shm/mantle-wave3-target-worker/x86_64-unknown-linux-musl/release/mantle-launch
f5266ac4de82789ab5335e0dc8387f69825a862b1f5ddc4dd7d962f242df47f2  /dev/shm/mantle-wave3-target-worker/debug/mantle
```

## 5. Remaining boundaries and handoff

This is Linux evidence. SIGKILL/SIGSTOP, non-unwinding aborts, kernel-uninterruptible children and foreign/custom conflicting signal handlers are not claimed as cleaned. Registry/termination cleanup waits are bounded to two seconds even when an owner cannot progress; the coordinator retains Child ownership for deferred cleanup while the process remains alive. Descendants in the owned process group are killed; only direct Child handles are reaped here, and orphaned descendant zombies belong to init. No claim covers descendants that deliberately escape their owned group.

Local SSH cancellation does not claim remote process cancellation, session teardown or cleanup of previously running Claude/Codex workloads. Real AW acceptance, authentication and model access remain coordinator-owned. No live worker, VM, service, token, namespace or session was touched.

The user's latest-main rebase is coordinator-owned. Source remains uncommitted; this correction did not stage anything or rebase. Package, lint and both build processes have exited. Final process inspection found no process using the assigned target and no owned sleep fixture. An unrelated GitLab logrotate sleep and an unrelated EKR cargo test were observed and left untouched. Own lease was released with worktree hook session-end --session codex-wave3-worker-implementor --path <assigned-tree>, exit 0.

## 6. Outside-worktree writes

Under ~/.cache/mantle-wave3/scratch-worker/, this correction wrote:
- correction-1-red-cancel.log
- correction-1-first.log
- correction-1-helper.log
- correction-1-helper2.log
- correction-1-packages.log
- correction-1-packages-final.log
- correction-1-packages-final2.log
- correction-1-red-pid-reuse.log
- correction-1-clippy.log
- correction-1-clippy-final.log
- correction-1-fmt.log
- correction-1-fmt-final.log
- correction-1-ess.log
- correction-1-ess-final.log
- correction-1-build-static.log
- correction-1-build-cli.log
- correction-1-hashes.txt
- correction-1-cli-help.log
- correction-1-worker-help.log
- correction-1-report.md (this file)

Build artifacts and temporary fixture directories were created under /dev/shm/mantle-wave3-target-worker/ using its TMPDIR; fixture cleanup is owned by the tests. Own managed-worktree lease metadata was acquired/heartbeated and released at handoff. Existing briefs, immutable adversary evidence, first report and generated local ESS operational state were preserved. No AEP, staging, commits, live mutations or other repository writes were made. Report output normalizes the personal home prefix; private raw logs remain available.
