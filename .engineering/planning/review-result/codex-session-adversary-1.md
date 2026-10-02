---
format: aep.planning-md/3
id: review-result:codex-session-adversary-1
kind: review-result
status: active
title: Codex session wiring independent attack pass 1
relations:
- reviews: story:codex-session-wiring
revision: 1
---
unit: story:codex-session-wiring; staged working tree mantle-wave5-codex-session on 79e1ee6bc7a9c0f311664d31057087096d354fdf; staged patch SHA256 c2d479d70b94f290f19f58308a746b8c07428f55a660de92d6f406c6de97ae2a
verdict: nothing found
cases: executed 137→142, red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: 17 paths (12 retained files and 5 assigned directories)
needs-coordinator: record pass 1; transport-policy clarification and full integration gate remain coordinator-owned

```text
$ git --no-pager diff --stat
 crates/mantle-launch/tests/conformance.rs | 129 +++++++++++++++++++
 crates/mantle/tests/codex_preflight.rs    | 198 ++++++++++++++++++++++++++++++
 2 files changed, 327 insertions(+)
```

This is the adversary's unstaged test-only diff. The 54-file implementor snapshot remains staged and unchanged. No production edits, staging, commits, planning-store writes, login, model calls, external operations or live worker actions were performed. Personal home prefixes are normalized to `~` in this report and embedded output; private raw logs retain exact operational paths.

All five cases were written before the first test execution, and each was run individually before the package suite. The before count is the implementor's handed-off 137 top-level Rust tests. The final package suite executes 142: Mantle 43 (29 unit, 4 public-CLI, 10 qualification-example), launcher 71 (45 unit, 9 earlier adversary, 3 second adversary, 2 compatibility, 8 conformance, 4 launcher), and worker 28. Nested subprocess summaries are not counted twice.

Added cases:

- `crates/mantle/tests/codex_preflight.rs`: `adversary_legacy_migration_preserves_indexes_and_stopped_sources_then_refuses_corruption` opens a literal legacy SQLite schema through the public `mantle list` command three times. It asserts explicit Claude identities, preserved stopped-session payload/source, both the live-name and operator index, and fixed refusal when an incompatible Codex/Claude-auth pair is later placed in a stopped record. The stopped record cannot be silently omitted from initialization validation. Green.
- Same file: `adversary_partial_identity_schema_refusal_rolls_back_initialization` invokes the same real initialization path with only one identity column present. Repeated refusal must leave every sqlite_master definition unchanged, including absence of initialization's new workers table, and preserve both records. This exercises transactional rollback without inventing a successful partial migration. Green.
- Same file: `adversary_codex_never_invokes_configured_claude_credential_command` supplies a synthetic Claude credential command whose observable effect is a test-owned marker. Codex refuses the current capture requirement without invoking it; switching the fixture manifest to Claude does invoke it and then refuses its empty credential. Both leave zero session rows. This is a positive control for credential-source selection. Green.
- `crates/mantle-launch/tests/conformance.rs`: `adversary_private_file_special_entries_refuse_without_opening_or_dispatch` drives the actual launcher with a FIFO without a writer, local Unix socket, dangling parent symlink, and regular file with setuid mode at the checked file path. Every case refuses within two seconds, before child dispatch or readiness, and preserves device/inode/mode without diagnostic canary disclosure. Green.
- Same file: `adversary_private_paths_accept_tmpfs_and_preserve_auth_metadata` drives two real successful launches with private directories, an observed tmpfs directory and an absent optional file parent. It checks positive dispatch, mode0700, unchanged auth device/inode/mode/access and modification timestamps, unchanged synthetic bytes, no disclosure, and no creation of the absent optional parent. Green.

Individual first-run output, before the package suite:

```text
$ cargo test --locked -p mantle --test codex_preflight adversary_legacy_migration_preserves_indexes_and_stopped_sources_then_refuses_corruption -- --exact --nocapture
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 2.04s
     Running tests/codex_preflight.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/codex_preflight-f14a9f5dcb0fcf21)

running 1 test
test adversary_legacy_migration_preserves_indexes_and_stopped_sources_then_refuses_corruption ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.18s

exit status: 0
$ cargo test --locked -p mantle --test codex_preflight adversary_partial_identity_schema_refusal_rolls_back_initialization -- --exact --nocapture
    Finished `test` profile [unoptimized] target(s) in 0.26s
     Running tests/codex_preflight.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/codex_preflight-f14a9f5dcb0fcf21)

running 1 test
test adversary_partial_identity_schema_refusal_rolls_back_initialization ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.21s

exit status: 0
$ cargo test --locked -p mantle --test codex_preflight adversary_codex_never_invokes_configured_claude_credential_command -- --exact --nocapture
    Finished `test` profile [unoptimized] target(s) in 0.26s
     Running tests/codex_preflight.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/codex_preflight-f14a9f5dcb0fcf21)

running 1 test
test adversary_codex_never_invokes_configured_claude_credential_command ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.06s

exit status: 0
$ cargo test --locked -p mantle-launch --test conformance adversary_private_file_special_entries_refuse_without_opening_or_dispatch -- --exact --nocapture
   Compiling mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle-launch)
    Finished `test` profile [unoptimized] target(s) in 2.22s
     Running tests/conformance.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/conformance-6f9433fcff2e5c73)

running 1 test
test adversary_private_file_special_entries_refuse_without_opening_or_dispatch ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.05s

exit status: 0
```

One fixture correction is disclosed rather than counted as a product defect. The positive metadata case initially used a non-UTF8 private-directory CLI argument and was named `adversary_private_paths_accept_non_utf8_tmpfs_and_preserve_auth_metadata`. It received clap exit2 before reaching the intended filesystem check. The pre-existing `absolute_path(&str)` parser accepts UTF-8 paths; the existing byte-preservation scenario covers child argv, and the actual production private paths are fixed ASCII. This extra fixture assumption was therefore removed: the case now uses `private-home` and retains every filesystem, metadata, absence and positive-dispatch assertion. No production code changed. Its original raw failure is retained below and in `case-private-positive.log`; it is not a confirmed regression or a counted red product case.

```text
$ cargo test --locked -p mantle-launch --test conformance adversary_private_paths_accept_non_utf8_tmpfs_and_preserve_auth_metadata -- --exact --nocapture
    Finished `test` profile [unoptimized] target(s) in 0.19s
     Running tests/conformance.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/conformance-6f9433fcff2e5c73)

running 1 test

thread 'adversary_private_paths_accept_non_utf8_tmpfs_and_preserve_auth_metadata' (1185729) panicked at crates/mantle-launch/tests/conformance.rs:1317:9:
assertion `left == right` failed
  left: Some(2)
 right: Some(0)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test adversary_private_paths_accept_non_utf8_tmpfs_and_preserve_auth_metadata ... FAILED

failures:

failures:
    adversary_private_paths_accept_non_utf8_tmpfs_and_preserve_auth_metadata

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p mantle-launch --test conformance`
exit status: 101 (fixture assumption; no product finding)
$ cargo test --locked -p mantle-launch --test conformance adversary_private_paths_accept_tmpfs_and_preserve_auth_metadata -- --exact --nocapture
   Compiling mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle-launch)
    Finished `test` profile [unoptimized] target(s) in 1.98s
     Running tests/conformance.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/conformance-6f9433fcff2e5c73)

running 1 test
test adversary_private_paths_accept_tmpfs_and_preserve_auth_metadata ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.02s

exit status: 0
```

Cargo used the configured `/usr/bin/sccache`, `CARGO_TARGET_DIR=/dev/shm/mantle-wave5-target-codex-session`, `CARGO_BUILD_JOBS=2`, `CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0`, `CARGO_INCREMENTAL=0`, `TMPDIR=~/.cache/mantle-wave5/scratch-session-adversary-1/tmp`, `MANTLE_TEST_SCRATCH=~/.cache/mantle-wave5/scratch-session-adversary-1/cases`, and `MANTLE_ADVERSARY_RUNTIME=/dev/shm/mantle-wave5-runtime-adversary-1`. Home paths were expanded in the real environment. The new tmpfs test can allocate its own private temporary directory under `/dev/shm` when the task-specific override is absent; no process-global HOME or environment mutation was added.

Package suite, run only after all five individual cases above:

```text
$ cargo test --locked -p mantle -p mantle-launch -p mantle-worker --all-targets -- --nocapture
   Compiling mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle-launch)
    Finished `test` profile [unoptimized] target(s) in 1.63s
     Running unittests src/main.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/mantle-277e3ad59dda8f7b)

running 29 tests
test app::terminal::tests::ctrl_bracket_d_detaches_and_forwards_what_came_before ... ok
test app::terminal::tests::the_sequence_is_recognised_across_reads ... ok
test app::terminal::tests::escape_followed_by_anything_else_passes_both_bytes ... ok
test config::tests::explicit_path_selection_is_absolute_and_preserves_defaults ... ok
test domain::manifest::tests::sizes_and_durations_parse ... ok
test app::worker::tests::legacy_claude_bootstrap_retains_required_slot ... ok
test domain::session::tests::names_round_trip ... ok
test domain::session::tests::running_is_never_reached_without_starting ... ok
test domain::session::tests::stopped_is_final_and_failures_only_stop ... ok
test domain::session::tests::the_happy_path_is_legal ... ok
test config::tests::common_worker_configuration_needs_no_claude_credentials ... ok
test adapters::substrate::worker_tests::common_readiness_and_selected_claude_are_distinct_observations ... ok
test config::tests::legacy_claude_table_remains_parseable ... ok
test domain::manifest::tests::defaults_apply ... ok
test domain::manifest::tests::cwd_must_be_inside_a_mount ... ok
test domain::manifest::tests::mounts_and_refs_cannot_escape ... ok
test domain::manifest::tests::the_example_manifest_resolves ... ok
test domain::manifest::tests::retain_for_is_bounded_by_the_exec_limit ... ok
test domain::manifest::tests::unserved_fields_are_refused_by_name ... ok
test app::worker::tests::the_template_renders_every_placeholder ... ok
test domain::manifest::tests::cwd_with_dot_dot_or_empty_components_is_refused ... ok
test app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command ... ok
test adapters::state::tests::sources_round_trip ... ok
test adapters::ssh::tests::tunnel_socket_binds_with_live_length_state_prefix ... ok
test adapters::state::tests::a_live_name_is_unique_until_stopped ... ok
test adapters::state::tests::an_illegal_move_is_refused_and_not_written ... ok
test adapters::ssh::tests::socket_allocations_are_private_unique_and_state_path_independent ... ok
test adapters::ssh::tests::tunnel_socket_directory_lives_until_child_cleanup_and_is_removed_on_errors ... ok
outside: mantle.egress.CheckBudgets/outcome/returned (OtherComponent)
outside: mantle.egress.CheckDefaultDestination/outcome/allowed (OtherComponent)
outside: mantle.egress.CheckDefaultDestination/outcome/denied (OtherComponent)
outside: mantle.egress.CheckDrain/outcome/returned (OtherComponent)
outside: mantle.egress.ClassifyAddress/outcome/returned (OtherComponent)
outside: mantle.egress.Exchange/outcome/returned (OtherComponent)
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
218 selected scenario(s), 137 authored source(s), 0 refusal occurrence(s)

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
worker      vm/mantle-default stopped; root and data volumes retained
no worker
launched    vm/mantle-default from https://example.com/fixture.img
worker      vm/mantle-default (Stopped)
worker      vm/mantle-default (Stopped)
starting    vm/mantle-default
launched    vm/mantle-default from https://example.com/fixture.img
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
mantle-cli: 218 passed, 0 failed, 0 unsupported
test adapters::state::conformance::ess_generated_local_conformance ... ok

test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.41s

     Running tests/codex_preflight.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/codex_preflight-f14a9f5dcb0fcf21)

running 4 tests
test adversary_codex_never_invokes_configured_claude_credential_command ... ok
test codex_refuses_capture_before_claude_credentials_or_session_insertion ... ok
test adversary_partial_identity_schema_refusal_rolls_back_initialization ... ok
test adversary_legacy_migration_preserves_indexes_and_stopped_sources_then_refuses_corruption ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.39s

     Running unittests examples/codex_qualification.rs (/dev/shm/mantle-wave5-target-codex-session/debug/examples/codex_qualification-2c3ae41e1f0be255)

running 10 tests
test tests::digest_format_is_strict ... ok
test tests::digest_refuses_other_non_regular_descriptors ... ok
test tests::fixed_observation_never_exports_terminal_payload ... ok
test tests::scratch_budget_refuses_large_files_and_does_not_follow_links ... ok
test tests::unsafe_scratch_parents_are_refused_without_creating_children ... ok
test tests::digest_bounds_actual_input_even_when_it_exceeds_the_size_snapshot ... ok

running 1 test

running 1 test
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered outtest adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s; finished in 0.00s



test tests::terminal_observation_is_bounded_and_handles_split_queries ... ok
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok
test tests::cq01_rejects_wrong_digest_before_executing_a_binary ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.32s

     Running unittests src/lib.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/mantle_launch-b94d68bc2f8cbdb8)

running 45 tests
test cli::tests::env_names_are_validated ... ok
test cli::tests::proxy_urls_carry_no_userinfo_or_path ... ok
test ctl::tests::lines_split_across_reads_are_joined ... ok
test ctl::tests::parses_columns_then_rows ... ok
test ctl::tests::over_long_lines_are_dropped_whole ... ok
test ctl::tests::line_round_trips ... ok
test ctl::tests::non_utf8_is_ignored ... ok
test ctl::tests::refuses_malformed_and_out_of_bounds_lines ... ok
test ring::tests::clear_empties ... ok
test ring::tests::front_and_consume_drain_in_order_across_wraparound ... ok
test ring::tests::drops_the_oldest_bytes_on_overflow ... ok
test ring::tests::a_push_larger_than_capacity_keeps_its_tail ... ok
test ring::tests::push_reports_how_many_bytes_it_dropped ... ok
test ring::tests::keeps_everything_within_capacity ... ok
test ring::tests::push_ring_copies_oldest_first_and_respects_the_target_capacity ... ok
test secret::tests::unopened_descriptor_is_reported ... ok
test serve::tests::a_nudge_holds_a_real_size_change_before_restoring ... ok
test serve::tests::a_new_terminal_does_not_inherit_an_owed_repaint ... ok
test serve::tests::a_nudge_under_way_is_not_restarted_and_can_be_cancelled ... ok
test serve::tests::a_one_row_window_is_nudged_up ... ok
test serve::tests::dropped_output_is_repainted_once_the_terminal_has_caught_up ... ok
test serve::tests::nothing_is_repainted_while_no_output_was_dropped ... ok
test serve::tests::poll_does_not_sleep_past_a_pending_restore ... ok
test secret::tests::errors_do_not_echo_the_value ... ok
test secret::tests::empty_value_is_refused ... ok
test secret::tests::trailing_newline_is_trimmed ... ok
test secret::tests::oversized_value_is_refused ... ok
test serve::tests::volatile_preflight_refuses_metadata_errors_other_than_absence ... ok
test session::tests::a_held_lock_is_refused_and_freed_on_drop ... ok
test session::tests::a_lock_on_an_unlinked_file_does_not_count ... ok
test session::tests::fifos_are_private ... ok
test session::tests::a_symlinked_lock_is_refused_and_its_target_left_alone ... ok
test session::tests::stale_fifos_are_removed_and_other_files_left_alone ... ok
test session::tests::open_fifo_refuses_a_symlink_and_a_file_that_is_not_a_pipe ... ok
test serve::tests::private_output_refuses_links_and_resets_the_mode ... ok
test cli::tests::secret_proxy_and_scrollback_are_optional ... ok
test cli::tests::attach_parses_and_hides_no_tty ... ok
test cli::tests::removed_tmux_flags_are_refused ... ok
test cli::tests::secret_fd_and_env_require_each_other ... ok
test cli::tests::dir_and_cwd_are_required ... ok
test cli::tests::standard_descriptors_are_refused ... ok
test cli::tests::full_command_line_parses ... ok
test cli::tests::program_is_required_after_separator ... ok
test cli::tests::scrollback_is_bounded ... ok
test cli::tests::paths_must_be_absolute ... ok

test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running unittests src/main.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/mantle_launch-fac975cfcc56cc2e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests tests/support/probe.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/mantle_launch_probe-5a3189013b5332d3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/adversary.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/adversary-3fdd24c12f8c926d)

running 9 tests
test the_secret_descriptor_does_not_reach_the_agent ... ok
test a_symlinked_session_dir_is_not_followed ... ok
test session_dir_is_owner_only_even_when_it_already_exists ... ok
test exit_code_is_the_agents ... ok
test last_output_is_owner_only_even_when_the_file_already_exists ... ok
test last_output_does_not_follow_a_planted_symlink ... ok
test sigterm_to_serve_ends_the_agents_whole_process_group ... ok
test a_flooded_ctl_pipe_does_not_stall_the_agent ... ok
test a_client_that_never_reads_does_not_block_the_agent ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.13s

     Running tests/adversary_2.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/adversary_2-fd9198f6cf4c14e4)

running 3 tests
test a_size_aware_agent_repaints_on_a_real_resize ... ok
test attaching_at_an_unchanged_size_makes_the_agent_repaint ... ok
test a_terminal_that_lost_output_gets_a_repaint_once_it_catches_up ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.61s

     Running tests/codex_compatibility.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/codex_compatibility-cd1886455c8675a2)

running 2 tests

running 1 test
test local_socket_control_rejects_af_unix ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

test local_socket_control_rejects_af_unix ... ok
mantle-launch: received signal 15; stopping the agent
mantle-launch: agent exited (signal: 1 (SIGHUP))
test fifo_launcher_relays_without_unix_sockets_or_secret_slot ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

     Running tests/conformance.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/conformance-0adf67a1aa5f2849)

running 8 tests
test adversary_volatile_preflight_preserves_fifo_and_socket_without_readiness ... ok
outside: mantle.egress.CheckBudgets/outcome/returned (OtherComponent)
outside: mantle.egress.CheckDefaultDestination/outcome/allowed (OtherComponent)
outside: mantle.egress.CheckDefaultDestination/outcome/denied (OtherComponent)
outside: mantle.egress.CheckDrain/outcome/returned (OtherComponent)
outside: mantle.egress.ClassifyAddress/outcome/returned (OtherComponent)
outside: mantle.egress.Exchange/outcome/returned (OtherComponent)
outside: mantle.manifest.Parse/outcome/returned (OtherComponent)
outside: mantle.manifest.ValidateMemory/outcome/accepted (OtherComponent)
outside: mantle.manifest.ValidateMemory/outcome/refused (OtherComponent)
outside: mantle.manifest.ValidatePids/outcome/accepted (OtherComponent)
outside: mantle.manifest.ValidatePids/outcome/refused (OtherComponent)
outside: mantle.manifest.ValidateRetention/outcome/accepted (OtherComponent)
outside: mantle.manifest.ValidateRetention/outcome/refused (OtherComponent)
outside: mantle.orchestration.CodexPreflight/outcome/returned (OtherComponent)
outside: mantle.orchestration.Observe/outcome/returned (OtherComponent)
outside: mantle.orchestration.Request/outcome/returned (OtherComponent)
outside: mantle.orchestration.Start/outcome/returned (OtherComponent)
outside: mantle.orchestration.Stop/outcome/returned (OtherComponent)
outside: mantle.orchestration.Worker/outcome/returned (OtherComponent)
outside: mantle.orchestration.WorkerRecords/outcome/returned (OtherComponent)
outside: mantle.session.AgentStartFailed/outcome/missing (OtherComponent)
outside: mantle.session.AgentStartFailed/outcome/moved (OtherComponent)
outside: mantle.session.AgentStarted/outcome/missing (OtherComponent)
outside: mantle.session.AgentStarted/outcome/moved (OtherComponent)
outside: mantle.session.AssessAgentInstallation/outcome/already-current (OtherComponent)
outside: mantle.session.AssessAgentInstallation/outcome/installed (OtherComponent)
outside: mantle.session.AssessAgentInstallation/outcome/refused (OtherComponent)
outside: mantle.session.AssessSharedBinaryUpgrade/outcome/applicable (OtherComponent)
outside: mantle.session.AssessSharedBinaryUpgrade/outcome/deferred (OtherComponent)
outside: mantle.session.AssessWorkerReadiness/outcome/missing (OtherComponent)
outside: mantle.session.AssessWorkerReadiness/outcome/ready (OtherComponent)
outside: mantle.session.BeginStop/outcome/missing (OtherComponent)
outside: mantle.session.BeginStop/outcome/moved (OtherComponent)
outside: mantle.session.FinishStop/outcome/missing (OtherComponent)
outside: mantle.session.FinishStop/outcome/moved (OtherComponent)
outside: mantle.session.IdentityMigration/outcome/returned (OtherComponent)
outside: mantle.session.InsertSession/outcome/conflict (OtherComponent)
outside: mantle.session.InsertSession/outcome/invalid-identity (OtherComponent)
outside: mantle.session.InsertSession/outcome/recorded (OtherComponent)
outside: mantle.session.MaterializationFailed/outcome/missing (OtherComponent)
outside: mantle.session.MaterializationFailed/outcome/moved (OtherComponent)
outside: mantle.session.Materialized/outcome/missing (OtherComponent)
outside: mantle.session.Materialized/outcome/moved (OtherComponent)
outside: mantle.session.PutSource/outcome/returned (OtherComponent)
outside: mantle.session.ReadSources/outcome/returned (OtherComponent)
outside: mantle.session.Session/state/FailedAgentStart/refuses/mantle.session.AgentStartFailed (OtherComponent)
outside: mantle.session.Session/state/FailedAgentStart/refuses/mantle.session.AgentStarted (OtherComponent)
outside: mantle.session.Session/state/FailedAgentStart/refuses/mantle.session.FinishStop (OtherComponent)
outside: mantle.session.Session/state/FailedAgentStart/refuses/mantle.session.MaterializationFailed (OtherComponent)
outside: mantle.session.Session/state/FailedAgentStart/refuses/mantle.session.Materialized (OtherComponent)
outside: mantle.session.Session/state/FailedMaterialization/refuses/mantle.session.AgentStartFailed (OtherComponent)
outside: mantle.session.Session/state/FailedMaterialization/refuses/mantle.session.AgentStarted (OtherComponent)
outside: mantle.session.Session/state/FailedMaterialization/refuses/mantle.session.FinishStop (OtherComponent)
outside: mantle.session.Session/state/FailedMaterialization/refuses/mantle.session.MaterializationFailed (OtherComponent)
outside: mantle.session.Session/state/FailedMaterialization/refuses/mantle.session.Materialized (OtherComponent)
outside: mantle.session.Session/state/Materializing/refuses/mantle.session.AgentStartFailed (OtherComponent)
outside: mantle.session.Session/state/Materializing/refuses/mantle.session.AgentStarted (OtherComponent)
outside: mantle.session.Session/state/Materializing/refuses/mantle.session.FinishStop (OtherComponent)
outside: mantle.session.Session/state/Running/refuses/mantle.session.AgentStartFailed (OtherComponent)
outside: mantle.session.Session/state/Running/refuses/mantle.session.AgentStarted (OtherComponent)
outside: mantle.session.Session/state/Running/refuses/mantle.session.FinishStop (OtherComponent)
outside: mantle.session.Session/state/Running/refuses/mantle.session.MaterializationFailed (OtherComponent)
outside: mantle.session.Session/state/Running/refuses/mantle.session.Materialized (OtherComponent)
outside: mantle.session.Session/state/Starting/refuses/mantle.session.FinishStop (OtherComponent)
outside: mantle.session.Session/state/Starting/refuses/mantle.session.MaterializationFailed (OtherComponent)
outside: mantle.session.Session/state/Starting/refuses/mantle.session.Materialized (OtherComponent)
outside: mantle.session.Session/state/Stopped/refuses/mantle.session.AgentStartFailed (OtherComponent)
outside: mantle.session.Session/state/Stopped/refuses/mantle.session.AgentStarted (OtherComponent)
outside: mantle.session.Session/state/Stopped/refuses/mantle.session.BeginStop (OtherComponent)
outside: mantle.session.Session/state/Stopped/refuses/mantle.session.FinishStop (OtherComponent)
outside: mantle.session.Session/state/Stopped/refuses/mantle.session.MaterializationFailed (OtherComponent)
outside: mantle.session.Session/state/Stopped/refuses/mantle.session.Materialized (OtherComponent)
outside: mantle.session.Session/state/Stopping/refuses/mantle.session.AgentStartFailed (OtherComponent)
outside: mantle.session.Session/state/Stopping/refuses/mantle.session.AgentStarted (OtherComponent)
outside: mantle.session.Session/state/Stopping/refuses/mantle.session.BeginStop (OtherComponent)
outside: mantle.session.Session/state/Stopping/refuses/mantle.session.MaterializationFailed (OtherComponent)
outside: mantle.session.Session/state/Stopping/refuses/mantle.session.Materialized (OtherComponent)
outside: mantle.session.Session/transition/materialization_failed/by/mantle.session.MaterializationFailed/moved (OtherComponent)
outside: mantle.session.Session/transition/materialized/by/mantle.session.Materialized/moved (OtherComponent)
outside: mantle.session.Session/transition/start_failed/by/mantle.session.AgentStartFailed/moved (OtherComponent)
outside: mantle.session.Session/transition/started/by/mantle.session.AgentStarted/moved (OtherComponent)
outside: mantle.session.Session/transition/stopped/by/mantle.session.FinishStop/moved (OtherComponent)
outside: mantle.session.Session/transition/stopping/by/mantle.session.BeginStop/moved (OtherComponent)
outside: mantle.session.SetAgentExec/outcome/missing (OtherComponent)
outside: mantle.session.SetAgentExec/outcome/recorded (OtherComponent)
outside: mantle.session.SetWorkspace/outcome/missing (OtherComponent)
outside: mantle.session.SetWorkspace/outcome/recorded (OtherComponent)
47 selected scenario(s), 30 authored source(s), 0 refusal occurrence(s)

test adversary_private_paths_accept_tmpfs_and_preserve_auth_metadata ... ok
test adversary_private_file_special_entries_refuse_without_opening_or_dispatch ... ok
test private_path_initialization_preserves_auth_and_refuses_unsafe_entries ... ok
test adversary_sigint_while_detached_never_persists_replay ... ok
test adversary_persistent_tail_refuses_volatile_reuse_then_clean_retry_runs ... ok
test volatile_runtime_does_not_persist_output_on_exit ... ok
mantle-launch: 47 passed, 0 failed, 0 unsupported
test ess_launch_conformance ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 14.70s

     Running tests/launch.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/launch-1bdf17dc3932f203)

running 4 tests
test attach_without_a_server_is_refused ... ok
test secret_stays_out_of_stderr_when_the_agent_cannot_start ... ok
test second_server_is_refused_and_sigterm_stops_the_first ... ok
test attach_relays_replays_scrollback_and_survives_a_detach ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.65s

     Running unittests src/lib.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/mantle_worker-0a69710eec6bc7a7)

running 28 tests
test tests::delivery_accepts_cargo_hardlinked_artifacts ... ok
test tests::byte_limit_detects_growth_beyond_declared_size ... ok
test tests::stale_transport_guard_cannot_retire_a_reused_pid_registration ... ok
test tests::concurrent_directory_winners_are_revalidated_without_permission_changes ... ok
test tests::symlinked_root_is_refused ... ok
test tests::adversary_second_failed_spawn_preserves_following_transport_and_idle_state ... ok
test tests::idle_transport_coordinator_does_not_swallow_termination ... ok
test tests::termination_cleanup_includes_owned_descendants ... ok
test tests::adversary_second_successful_child_exit_cleans_its_remaining_group ... ok
test tests::unsupported_architecture_never_fetches_or_creates_paths ... ok
test tests::corrupt_archive_never_activates ... ok
test tests::bounded_child_pumps_input_and_output_together ... ok
test tests::termination_coordinator_cleans_concurrent_groups_and_preserves_signal_status ... ok
test tests::adversary_refuses_binary_mismatch_after_valid_archive_before_execution ... ok
test tests::version_mismatch_never_activates ... ok
test tests::adversary_operator_interrupt_reaps_bounded_child ... ok
test tests::adversary_unmanaged_executable_is_preserved_on_activation_refusal ... ok
test tests::bounded_child_refuses_output_flood_and_timeout ... ok
test tests::tampered_installed_bytes_are_refused ... ok
test tests::installation_is_verified_atomic_and_idempotent ... ok
test tests::adversary_fifo_and_symlink_inputs_refuse_without_waiting_for_writers ... ok
test tests::failed_update_preserves_previous_binary_and_facts ... ok
test tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation ... ok
test tests::adversary_unpublished_verified_generation_recovers_after_interrupted_activation ... ok
test tests::adversary_second_update_observers_only_see_complete_verified_generations ... ok
test tests::generated_model_has_no_drift ... ok
test tests::retirement_survives_a_bounded_registry_wait_timeout ... ok
test tests::adversary_live_lock_refuses_without_fetching_or_changing_current ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.16s

     Running unittests src/main.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/mantle_worker-5f8b887456570a7c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

exit status: 0
```

Native report results: CLI218 passed (137 authored, 81 generated), launcher47 passed (30 authored, 17 generated); both have zero failed/error/skipped/unsupported/refused. CLI reports23 outside and launcher87 outside their respective component selections. These are component runs, not a whole-system zero-outside claim. Both bind spec digest `45700f1d2dc4e3c23ba9a26824b43bef5e98d2623981472ab852f3b57951dcac`. Worker `generated_model_has_no_drift` passed within its28 tests.

```text
$ cargo clippy --locked -p mantle -p mantle-launch -p mantle-worker --all-targets -- -D warnings
    Checking mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle)
    Checking mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle-launch)
    Finished `dev` profile [unoptimized] target(s) in 0.84s
exit status: 0
$ cargo fmt --check
(no output)
exit status: 0
$ ess specify validate --path spec
mantle v1 — 7 file(s), 227 scenario(s), valid
exit status: 0
$ git diff --check
(no output)
exit status: 0
```

Judgement findings: nothing found.

Attack coverage and boundaries:

- Read the complete staged diff, current story CW01–CW09 and its private-runtime/generated-identity/provider/fixture extensions, implementor report, caller chains, new ESS contracts/scenarios, and generated wire identities before writing cases.
- Actual public CLI initialization preserved legacy records and schema objects across repeated opens, refused a corrupt stopped identity, and rolled back schema creation on incomplete identity-schema refusal.
- Actual production selection refused Codex before the observable configured Claude credential command; the corresponding Claude invocation served as the positive control.
- The existing public-CLI test reattached persisted Codex identity after manifest replacement and preserved stored exec/workspace fields; the final package run executed it.
- Actual launcher checks refused special files without blocking, preserved synthetic file metadata and bytes, accepted real tmpfs, and dispatched a real child on safe input. Existing cases additionally exercised hardlinks, ordinary-disk refusal, traversal, parent/final links and shared-ancestor modes.
- Read `RunRequest::command` and `RunRequest::pty`: both call capture admission before SDK builder use. The current fixed unavailable adapter remains unchanged. Production request native snapshots exercised strict configuration, the pinned ChatGPT provider/endpoints, absent Claude slot/fd, and the explicit non-recording requirement.
- The package qualification tests passed. The implementor's digest-verified pinned-program sink observation was read; this attack did not repeat a Codex invocation or claim authenticated refresh, managed-policy, model-turn or live upstream transport behavior.
- Same-UID malicious filesystem races remain outside the accepted threat model. The pending user clarification about transport policy was not guessed and does not alter this report's tested staged snapshot.

Outside-worktree paths written (portable home prefixes):

1. `~/.cache/mantle-wave5/scratch-session-adversary-1/` — assigned report/log root.
2. `~/.cache/mantle-wave5/scratch-session-adversary-1/tmp/` — private temporary children for package fixtures/compiler.
3. `~/.cache/mantle-wave5/scratch-session-adversary-1/cases/` — private launcher/worker fixture children.
4. `/dev/shm/mantle-wave5-runtime-adversary-1/` — private tmpfs test parent; owned temporary children removed by fixtures.
5. `/dev/shm/mantle-wave5-target-codex-session/` — sequentially assigned Cargo output.
6. `~/.cache/mantle-wave5/scratch-session-adversary-1/case-migration.log`
7. `~/.cache/mantle-wave5/scratch-session-adversary-1/case-rollback.log`
8. `~/.cache/mantle-wave5/scratch-session-adversary-1/case-credentials.log`
9. `~/.cache/mantle-wave5/scratch-session-adversary-1/case-private-refusals.log`
10. `~/.cache/mantle-wave5/scratch-session-adversary-1/case-private-positive.log`
11. `~/.cache/mantle-wave5/scratch-session-adversary-1/case-private-positive-corrected.log`
12. `~/.cache/mantle-wave5/scratch-session-adversary-1/package.log`
13. `~/.cache/mantle-wave5/scratch-session-adversary-1/fmt.log`
14. `~/.cache/mantle-wave5/scratch-session-adversary-1/ess.log`
15. `~/.cache/mantle-wave5/scratch-session-adversary-1/clippy.log`
16. `~/.cache/mantle-wave5/scratch-session-adversary-1/tests.patch`
17. `~/.cache/mantle-wave5/scratch-session-adversary-1/report.md`

The configured compiler wrapper also manages its shared cache; the worktree manager manages lease metadata for `codex-mantle-wave5-session-adversary-1`. Those existing services were not manually edited or cleaned. Native runner artifacts remain inside this worktree's ignored `.engineering/drafts/`. Initial free space was17GiB root and26GiB tmpfs, above assigned floors. All tests/builds exited and the final owned launcher/conformance/CLI-fixture process check found no live fixture process. Lease release and report hash are supplied separately at handoff. No managed worktree or build directory was deleted. Aggregate token/tool/wall metrics are unavailable from this host.

```findings
[]
```
