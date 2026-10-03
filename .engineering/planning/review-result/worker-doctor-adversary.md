---
format: aep.planning-md/3
id: review-result:worker-doctor-adversary
kind: review-result
status: active
title: Worker doctor adversary review
relations:
- reviews: story:worker-doctor
revision: 1
---
unit: story:worker-doctor at 3ed0f50723890a484a4ea5c8c064f5dab394faae plus uncommitted adversary test
verdict: nothing found
cases: executed 89→90 top-level Rust tests (Mantle60→61, worker29→29), red 0; CLI255 passed
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: 9 retained files/directories plus self-cleaned test fixtures
needs-coordinator: decide whether to retain the uncommitted tests-only addition; no implementation finding

```text
 crates/mantle/tests/doctor.rs | 119 ++++++++++++++++++++++++++++++++++++++++++
 1 file changed, 119 insertions(+)
```

1. Added `crates/mantle/tests/doctor.rs:340`, `adversary_doctor_interruption_retires_tunnel_and_removes_owned_socket_directory`. The actual doctor CLI uses controlled Rust provider/SSH programs and reaches a live stalled-discovery tunnel with a descendant. The test sends SIGTERM to doctor, requires process termination within three seconds, observes that the descendant no longer runs and requires the private socket directory to have been removed. The timeout configured on doctor is ten seconds, so the assertion exercises interruption rather than ordinary expiry. No real provider, authentication cache or worker is involved. If the assertion detects a leaked socket directory, it cleans only the observed allocation after recording that postcondition; the first execution found no leak.

The case was written before any execution. Its first isolated run passed; there is no red output or confirmed defect. Command: `cargo test -p mantle --test doctor --locked adversary_doctor_interruption_retires_tunnel_and_removes_owned_socket_directory -- --exact`, exit 0. Output below is verbatim except `$HOME` replaces the operator home prefix.

```text
   Compiling mantle v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-worker-doctor/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 1.43s
     Running tests/doctor.rs (/dev/shm/mantle-worker-doctor-target/debug/deps/doctor-5542f1a70344160f)

running 1 test
test adversary_doctor_interruption_retires_tunnel_and_removes_owned_socket_directory ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.20s

```

2. After that isolated case, ran `cargo test -p mantle -p mantle-worker --locked -- --nocapture`, exit 0. The baseline Mantle60/worker29/CLI255 counts come from the implementor. This execution ran Mantle61 (36+4+1+4+6+10) and worker29 top-level Rust tests. Nested child-test reexecutions are not counted twice. Native CLI runner reported255 passed,0 failed,0 unsupported.

Environment for both Cargo commands: `PATH=$HOME/.cache/aap-ess-upgrade/ess-0.50/bin:$PATH`, `TMPDIR=/dev/shm/mantle-worker-doctor-tmp`, `CARGO_TARGET_DIR=/dev/shm/mantle-worker-doctor-target`, `CARGO_BUILD_JOBS=2`, `CARGO_PROFILE_DEV_DEBUG=0`, `RUSTC_WRAPPER=/usr/bin/sccache`, `SCCACHE_SERVER_PORT=43179`, `SCCACHE_DIR=/dev/shm/mantle-reliability-compiler-cache`, `SCCACHE_CACHE_SIZE=1G`.

The complete suite output follows, verbatim except `$HOME` substitution:

```text
    Finished `test` profile [unoptimized] target(s) in 0.19s
     Running unittests src/main.rs (/dev/shm/mantle-worker-doctor-target/debug/deps/mantle-4bd5c60d9435f927)

running 36 tests
test app::terminal::tests::ctrl_bracket_d_detaches_and_forwards_what_came_before ... ok
test app::terminal::tests::escape_followed_by_anything_else_passes_both_bytes ... ok
test app::terminal::tests::readiness_marker_is_consumed_across_every_split ... ok
test app::terminal::tests::the_sequence_is_recognised_across_reads ... ok
test config::tests::explicit_path_selection_is_absolute_and_preserves_defaults ... ok
test app::worker::tests::legacy_claude_bootstrap_retains_required_slot ... ok
test app::worker::tests::the_template_renders_every_placeholder ... ok
test config::tests::common_worker_configuration_needs_no_claude_credentials ... ok
test adapters::ssh::tests::tunnel_socket_binds_with_live_length_state_prefix ... ok
test config::tests::legacy_claude_table_remains_parseable ... ok
test domain::manifest::tests::sizes_and_durations_parse ... ok
test adapters::substrate::worker_tests::common_readiness_and_selected_claude_are_distinct_observations ... ok
test domain::manifest::tests::mounts_and_refs_cannot_escape ... ok
test domain::session::tests::names_round_trip ... ok
test domain::session::tests::running_is_never_reached_without_starting ... ok
test domain::session::tests::stopped_is_final_and_failures_only_stop ... ok
test domain::session::tests::the_happy_path_is_legal ... ok
test domain::manifest::tests::unserved_fields_are_refused_by_name ... ok
test domain::manifest::tests::cwd_must_be_inside_a_mount ... ok
test domain::manifest::tests::retain_for_is_bounded_by_the_exec_limit ... ok
test domain::manifest::tests::cwd_with_dot_dot_or_empty_components_is_refused ... ok
test domain::manifest::tests::defaults_apply ... ok
test adapters::ssh::tests::known_hosts_option_preserves_one_literal_path_in_openssh_parser ... ok
test app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command ... ok
test domain::manifest::tests::the_example_manifest_resolves ... ok
test adapters::ssh::tests::socket_allocations_are_private_unique_and_state_path_independent ... ok
test adapters::state::tests::sources_round_trip ... ok
Workspace      workspace-1
test adapters::state::tests::an_illegal_move_is_refused_and_not_written ... ok
test adapters::state::tests::a_live_name_is_unique_until_stopped ... ok
test app::session::codex_transport_tests::codex_start_and_attach_construct_real_sdk_builders ... ok
test app::session::adversary_activation_wire::adversary_codex_dispatches_confined_secretless_command_and_pty ... ok
Source         /workspace/r main@7b9b9459dc448b124a52d20e88df83782bda320e
Agent          exec-1 (running, ends after 8h at the latest)
Attach with    mantle attach fixture
Workspace      workspace-1
Source         /workspace/r lightweight@e278f2c462d5965e37f3bdb8fac77bce7b89a2cb
Agent          exec-1 (running, ends after 8h at the latest)
Attach with    mantle attach fixture
Workspace      workspace-1
Source         /workspace/r annotated@e278f2c462d5965e37f3bdb8fac77bce7b89a2cb
Agent          exec-1 (running, ends after 8h at the latest)
Attach with    mantle attach fixture
test adapters::ssh::tests::tunnel_socket_directory_lives_until_child_cleanup_and_is_removed_on_errors ... ok
Workspace      workspace-1
Source         /workspace/r e278f2c462d5965e37f3bdb8fac77bce7b89a2cb@e278f2c462d5965e37f3bdb8fac77bce7b89a2cb
Agent          exec-1 (running, ends after 8h at the latest)
Attach with    mantle attach fixture
Workspace      workspace-1
test adapters::state::orchestration::real_git_resolves_branches_tags_and_commits ... ok
outside: mantle.egress.CheckBudgets/outcome/returned (OtherComponent)
outside: mantle.egress.CheckDefaultDestination/outcome/allowed (OtherComponent)
outside: mantle.egress.CheckDefaultDestination/outcome/denied (OtherComponent)
outside: mantle.egress.CheckDrain/outcome/returned (OtherComponent)
outside: mantle.egress.ClassifyAddress/outcome/returned (OtherComponent)
outside: mantle.egress.Exchange/outcome/returned (OtherComponent)
outside: mantle.launch.AttachBackpressure/outcome/returned (OtherComponent)
outside: mantle.launch.Emit/outcome/returned (OtherComponent)
outside: mantle.launch.FileSafety/outcome/returned (OtherComponent)
outside: mantle.launch.Lifecycle/outcome/returned (OtherComponent)
outside: mantle.launch.ParseArgs/outcome/returned (OtherComponent)
outside: mantle.launch.ParseWindow/outcome/returned (OtherComponent)
outside: mantle.launch.PersistentReplayCompatibility/outcome/returned (OtherComponent)
outside: mantle.launch.PrivatePaths/outcome/returned (OtherComponent)
outside: mantle.launch.ReadSecret/outcome/returned (OtherComponent)
outside: mantle.launch.ReplayRing/outcome/returned (OtherComponent)
outside: mantle.launch.Signals/outcome/returned (OtherComponent)
outside: mantle.launch.SlowReader/outcome/returned (OtherComponent)
outside: mantle.launch.ValidateScrollback/outcome/accepted (OtherComponent)
outside: mantle.launch.ValidateScrollback/outcome/refused (OtherComponent)
outside: mantle.launch.VolatileReplayBounds/outcome/returned (OtherComponent)
outside: mantle.launch.VolatileReplayExitPaths/outcome/returned (OtherComponent)
outside: mantle.launch.VolatileReplayLifecycle/outcome/returned (OtherComponent)
outside: mantle.launch.VolatileReplayPreflight/outcome/returned (OtherComponent)
255 selected scenario(s), 168 authored source(s), 0 refusal occurrence(s)

test adapters::ssh::tests::proxy_commands_preserve_literals_through_real_openssh_and_shell_parsing ... ok
test adapters::state::tests::readonly_diagnostics_observe_wal_and_refuse_writes_without_migrating_legacy_schema ... ok
Workspace      workspace-1
Source         /workspace/substrate main@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Agent          exec-1 (running, ends after 8h at the latest)
Workspace      workspace-1
Workspace      workspace-1
Source         /workspace/r main@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Agent          exec-1 (running, ends after 8h at the latest)
Workspace      destroyed
Session        fixture stopped
worker      instance-1 (running)
no worker
worker      instance-1 stopped; data volume volume-1 retained
worker      instance-1 stopped; data volume volume-1 retained
worker      instance-1 stopped; data volume volume-1 retained
instance profile mantle-worker created; waiting for it to propagate
launched    instance-1 (image-1, group-1)
launched    instance-1 (image-1, group-1)
launched    instance-1 (image-1, group-1)
launched    instance-1 (image-1, group-1)
worker      instance-1 (pending)
worker      instance-1 (running)
worker      instance-1 (stopped)
starting    instance-1
worker      instance-1 (stopping)
starting    instance-1
Workspace      workspace-1
Source         /workspace/substrate main@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Agent          exec-1 (running, ends after 8h at the latest)
worker      vm/mantle-default stopped; root and data volumes retained
no worker
launched    vm/mantle-default from https://example.com/fixture.img
worker      vm/mantle-default (Stopped)
worker      vm/mantle-default (Stopped)
starting    vm/mantle-default
launched    vm/mantle-default from https://example.com/fixture.img
Workspace      workspace-1
Source         /workspace/r annotated@fc6c787aa7d4b004d72a87a26a1320b105c0866f
Agent          exec-1 (running, ends after 8h at the latest)
Attach with    mantle attach fixture
Workspace      workspace-1
Source         /workspace/r main@c6eafc37103fbacd389f9932443dbff96c370f7e
Agent          exec-1 (running, ends after 8h at the latest)
Attach with    mantle attach fixture
Workspace      workspace-1
Source         /workspace/r lightweight@fc6c787aa7d4b004d72a87a26a1320b105c0866f
Agent          exec-1 (running, ends after 8h at the latest)
Attach with    mantle attach fixture
Workspace      workspace-1
Workspace      workspace-1
Source         /workspace/r fc6c787aa7d4b004d72a87a26a1320b105c0866f@fc6c787aa7d4b004d72a87a26a1320b105c0866f
Agent          exec-1 (running, ends after 8h at the latest)
Attach with    mantle attach fixture
Workspace      workspace-1
Source         /workspace/r main@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Agent          exec-1 (running, ends after 8h at the latest)
Workspace      workspace-1
Source         /workspace/r main@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Agent          exec-1 (running, ends after 8h at the latest)
Attach with    mantle attach s1
Workspace      workspace-1
Workspace      workspace-1
Source         /workspace/r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Agent          exec-1 (running, ends after 8h at the latest)
Attach with    mantle attach s1
Workspace      workspace-1
Source         /workspace/r main@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Workspace      workspace-1
Source         /workspace/r main@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Agent          exec-1 (running, ends after 8h at the latest)
Workspace      workspace-1
Workspace      workspace-1
Workspace      workspace-1
Agent          exec-1 already retired
Workspace      destroyed
Session        fixture stopped
Workspace      workspace-1 already destroyed
Session        fixture stopped
Workspace      destroyed
Session        fixture stopped
Session        fixture stopped
Workspace      destroyed
Session        fixture stopped
Workspace      destroyed
Session        fixture stopped
Workspace      destroyed (confirmed by read-back)
Session        fixture stopped
Workspace      destroyed (confirmed by read-back)
Session        fixture stopped
mantle-cli: 255 passed, 0 failed, 0 unsupported
test adapters::state::conformance::ess_generated_local_conformance ... ok

test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.08s

     Running tests/codex_preflight.rs (/dev/shm/mantle-worker-doctor-target/debug/deps/codex_preflight-66b2423f46693291)

running 4 tests
test codex_reaches_worker_boundary_without_claude_credentials_or_session_insertion ... ok
test adversary_codex_never_invokes_configured_claude_credential_command ... ok
test adversary_partial_identity_schema_refusal_rolls_back_initialization ... ok
test adversary_legacy_migration_preserves_indexes_and_stopped_sources_then_refuses_corruption ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

     Running tests/command_correctness.rs (/dev/shm/mantle-worker-doctor-target/debug/deps/command_correctness-ac9f71c0d07cdd15)

running 1 test
test actual_cli_preserves_remote_exit_status_and_output ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.54s

     Running tests/doctor.rs (/dev/shm/mantle-worker-doctor-target/debug/deps/doctor-5542f1a70344160f)

running 4 tests
test doctor_reports_selection_and_configuration_failures_as_json_without_mutating_state ... ok
test doctor_bounds_local_state_failures_and_suppresses_external_probes ... ok
test adversary_doctor_interruption_retires_tunnel_and_removes_owned_socket_directory ... ok
test doctor_observes_real_cli_wire_stages_and_preserves_current_wal_state ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.54s

     Running tests/profiles.rs (/dev/shm/mantle-worker-doctor-target/debug/deps/profiles-2f9551666ce4ba21)

running 6 tests
test profile_registry_is_local_private_and_concurrent_additions_do_not_clobber ... ok
test profile_selection_refuses_symlinked_configuration_state_and_registry_ancestors ... ok
test profile_selection_precedence_conflicts_and_database_isolation ... ok
test adversary_profile_boundaries_and_corrupt_registry_fail_closed ... ok
test profile_registry_rejects_unsafe_names_paths_and_descriptor_types ... ok
test selected_profiles_isolate_ssh_identity_known_hosts_and_private_sockets ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.41s

     Running unittests examples/codex_qualification.rs (/dev/shm/mantle-worker-doctor-target/debug/examples/codex_qualification-e9c9bbc247fe502d)

running 10 tests
test tests::digest_format_is_strict ... ok
test tests::digest_refuses_other_non_regular_descriptors ... ok
test tests::digest_bounds_actual_input_even_when_it_exceeds_the_size_snapshot ... ok
test tests::fixed_observation_never_exports_terminal_payload ... ok
test tests::unsafe_scratch_parents_are_refused_without_creating_children ... ok
test tests::scratch_budget_refuses_large_files_and_does_not_follow_links ... ok

running 1 test
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s


running 1 test
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok
test tests::terminal_observation_is_bounded_and_handles_split_queries ... ok
test tests::cq01_rejects_wrong_digest_before_executing_a_binary ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.32s

     Running unittests src/lib.rs (/dev/shm/mantle-worker-doctor-target/debug/deps/mantle_worker-408d5f96a174c825)

running 29 tests
test tests::byte_limit_detects_growth_beyond_declared_size ... ok
test tests::delivery_accepts_cargo_hardlinked_artifacts ... ok
test tests::concurrent_directory_winners_are_revalidated_without_permission_changes ... ok
test tests::stale_transport_guard_cannot_retire_a_reused_pid_registration ... ok
test tests::corrupt_archive_never_activates ... ok
test tests::adversary_fifo_and_symlink_inputs_refuse_without_waiting_for_writers ... ok
test tests::adversary_refuses_binary_mismatch_after_valid_archive_before_execution ... ok
test tests::symlinked_root_is_refused ... ok
test tests::installation_is_verified_atomic_and_idempotent ... ok
test tests::adversary_second_successful_child_exit_cleans_its_remaining_group ... ok
test tests::idle_transport_coordinator_does_not_swallow_termination ... ok
test tests::adversary_second_failed_spawn_preserves_following_transport_and_idle_state ... ok
test tests::adversary_unmanaged_executable_is_preserved_on_activation_refusal ... ok
test tests::termination_cleanup_includes_owned_descendants ... ok
test tests::unsupported_architecture_never_fetches_or_creates_paths ... ok
test tests::tampered_installed_bytes_are_refused ... ok
test tests::generated_model_has_no_drift ... ok
test tests::failed_update_preserves_previous_binary_and_facts ... ok
test tests::adversary_unpublished_verified_generation_recovers_after_interrupted_activation ... ok
test tests::version_mismatch_never_activates ... ok
test tests::termination_coordinator_cleans_concurrent_groups_and_preserves_signal_status ... ok
test tests::bounded_child_pumps_input_and_output_together ... ok
test tests::adversary_second_update_observers_only_see_complete_verified_generations ... ok
test tests::adversary_operator_interrupt_reaps_bounded_child ... ok
test tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation ... ok
test tests::bounded_child_refuses_output_flood_and_timeout ... ok
test tests::bounded_live_process_deadline_and_drop_retire_descendants_without_caller_progress ... ok
test tests::retirement_survives_a_bounded_registry_wait_timeout ... ok
test tests::adversary_live_lock_refuses_without_fetching_or_changing_current ... ok

test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.04s

     Running unittests src/main.rs (/dev/shm/mantle-worker-doctor-target/debug/deps/mantle_worker-18dc302baa80b475)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mantle_worker

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

3. No judgement findings. This report does not grant approval or claim independent-verifier status.

4. Read the complete source diff, accepted story, invariant/implementation/review briefs, executable fixtures and changed-function callers. Attacked SIGTERM during live discovery with real subprocesses and observed bounded descendant/socket cleanup. Existing regression suite remained green for structured early errors, readonly WAL observation, nonregular/oversized state, strict SSH flags, provider quoting through real OpenSSH, version/wire/facts mismatch and timeout cleanup. No live worker or agent qualification was attempted.

5. Exact retained external paths, expressed with portable `$HOME` anchors:
- `$HOME/.cache/mantle-reliability/worker-doctor/adversary/case.log`
- `$HOME/.cache/mantle-reliability/worker-doctor/adversary/suite.log`
- `$HOME/.cache/mantle-reliability/worker-doctor/adversary/review.md`
- `/dev/shm/mantle-worker-doctor-target`
- `/dev/shm/mantle-worker-doctor-tmp`
- `/dev/shm/mantle-reliability-compiler-cache`
- `$HOME/.local/state/worktree/registry.sqlite3`
- `$HOME/.local/state/worktree/registry.sqlite3-wal`
- `$HOME/.local/state/worktree/registry.sqlite3-shm`

Temporary fixtures under the assigned TMPDIR were cleaned by the tests. Existing SSH tests and the doctor transport create and remove short private `/tmp/mantle-*` allocations; their random names are not retained. The native suite also rewrote its expected ignored report artifacts under the worktree's `.engineering/drafts/`. No implementation, planning, generated-source or existing-test assertions were changed. No commits or publication. Only the119-line added test remains uncommitted. `git diff --check` passed. Own lease `codex-worker-doctor-adversary` ended; source tree, build target and logs remain for the coordinator. Final observed free space: root19GiB, tmpfs9.8GiB.

```findings
[]
```
