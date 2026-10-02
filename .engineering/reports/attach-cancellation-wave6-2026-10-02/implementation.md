unit:                   story:attach-backpressure-cancellation — Keep attached terminals cancellable
verdict:                green
cases:                  executed 71→72 launcher Rust; 47→49 native; red 1
origin:                 n/a
wrote-outside-worktree: ~/.cache/mantle-wave6/scratch-attach; /dev/shm/mantle-wave6-target-attach; /dev/shm/mantle-wave6-runtime-attach; managed lease metadata
needs-coordinator:      no

1. Unit and acceptance

Implemented AB01–AB04 in the assigned managed tree mantle-wave6-attach-cancellation, branch impl/attach-backpressure-cancellation, opening a0f8dd8fc7f886af839f63cfc477470a7c3a7b15. Source changes stay within the 14 approved paths (13 changed, generated Cargo.toml remains identical). The new scenario path was verified against the existing ESS launcher boundary and admitted explicitly. No AEP/staging/commit/integration/worker/SDK changes were made.

The relay now keeps one fixed 16KiB buffer per direction, reads only with room, polls POLLOUT only with pending data, retains partial-write suffixes and checks signals/resize between bounded operations. A descriptor guard snapshots every inherited/shared open-file status before changing any, then restores exact saved flags on every return. Existing raw-mode restoration remains in place. Server protocol, signal policy and agent lifecycle remain unchanged.

The fixture uses a finite 2MiB output producer, independently paused/resumed input, a 1500ms handled-cancellation deadline, blocking/nonblocking PTYs and aliased stdin/stdout. It observes exact terminal/flag restoration, acquires a new client lock and obtains a fresh response from the same server/agent. AB04 transfers a 24KiB burst and 128KiB bidirectional payload through 4096-byte pipes with bounded outstanding data, comparing exact bytes. Prior resize/redraw/duplicate-client/replay/private-path assertions all remain green.

Inherited baseline runner output is .engineering/reports/codex-session-wave5-2026-10-02/gate/tests.log: launcher 45 library + 9 adversary + 3 adversary_2 + 2 compatibility + 8 conformance harness + 4 launch =71. It came from the coordinator's workspace gate, not a same-command baseline rerun by this worker. Current package has 9 conformance harness tests, total 72; the native launcher suite rises 47→49. The existing bridge test executes the increased native inventory. The selected generated-drift lane runs 1 test; no new worker runtime test was added.

2. Actual tracked diff shape (untracked scenario also appears in status.txt)
 crates/mantle-launch/src/attach.rs          |  89 +++++-
 crates/mantle-launch/src/sys.rs             |  34 +++
 crates/mantle-launch/tests/conformance.rs   | 434 ++++++++++++++++++++++++++++
 crates/mantle-launch/tests/support/probe.rs |  92 ++++++
 generated/worker-model/source.schema.json   |   2 +-
 generated/worker-model/types-report.json    |   4 +-
 generated/worker-model/types.rs             |   4 +-
 spec/README.md                              |  14 +
 spec/components.yaml                        |   2 +-
 spec/conformance-baseline.json              |   6 +-
 spec/domains/launch.yaml                    |  15 +
 spec/ess-inputs.yaml                        |   1 +
 12 files changed, 677 insertions(+), 20 deletions(-)

3. Red evidence

The inherited one-variable drain-only probe supports blocking write_all as the cause: only making unread stdout drain lets the pending termination complete, with no second signal. Its retained original logs were read, not changed.

Before production edits, red-observed.log reproduced both stalled-output signals as cancelled=false and the drained positive control as true. That initial aggregate then exposed a fixture-only marker-length typo. A subsequent run reached all PTY cases but hit an AB04 fixture flush omission. Both fixture issues were fixed. The final exact fixture was then run with only attach.rs restored to its original HEAD implementation; a shell exit trap restored the treatment source afterward. That red run completes the fixture and fails the intended cancellation assertion, including both paused-input failures. No runtime mutant is left in the worktree.

Command (assigned private environment):
cargo test --locked -p mantle-launch --test conformance attach_cancellation_under_finite_output_and_input_backpressure -- --nocapture

Red lane: executed1→1, exit101→0. Raw red-exact.log follows with only the home prefix rendered portably; raw files are retained verbatim beside this report:
   Compiling mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave6-attach-cancellation/crates/mantle-launch)
    Finished `test` profile [unoptimized] target(s) in 2.17s
     Running tests/conformance.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/conformance-6f9433fcff2e5c73)

running 1 test
AB output mode=pipe signal=15 drained=false cancelled=false
AB output mode=pipe signal=1 drained=false cancelled=false
AB output mode=pipe signal=15 drained=true cancelled=true
AB paused input signal=15 sent=151552 cancelled=false
AB paused input signal=1 sent=151552 cancelled=false
AB output mode=pty-blocking signal=15 drained=false cancelled=false
AB output mode=pty-nonblocking signal=1 drained=false cancelled=false
AB output mode=pty-aliased signal=15 drained=false cancelled=false

thread 'attach_cancellation_under_finite_output_and_input_backpressure' (1627828) panicked at crates/mantle-launch/tests/conformance.rs:1777:5:
assertion `left == right` failed
  left: Array [Bool(false), Bool(false), Bool(true)]
 right: Array [Bool(true), Bool(true), Bool(true)]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test attach_cancellation_under_finite_output_and_input_backpressure ... FAILED

failures:

failures:
    attach_cancellation_under_finite_output_and_input_backpressure

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 8 filtered out; finished in 12.99s

error: test failed, to rerun pass `-p mantle-launch --test conformance`

4. Green gates

Environment for Cargo: TMPDIR and MANTLE_TEST_SCRATCH=~/.cache/mantle-wave6/scratch-attach/tmp; MANTLE_ADVERSARY_RUNTIME=/dev/shm/mantle-wave6-runtime-attach; CARGO_TARGET_DIR=/dev/shm/mantle-wave6-target-attach; CARGO_BUILD_JOBS=2; CARGO_INCREMENTAL=0; CARGO_PROFILE_DEV_DEBUG=0; CARGO_PROFILE_TEST_DEBUG=0. Configured sccache remains in use.

cargo test --locked -p mantle-launch -- --nocapture
exit0. Executed71→72 against the inherited suite baseline; native47→49. No failures, ignored cases, skips or unsupported outcomes. New native inventory entries are AttachBackpressure/outcome/returned and authored/attach-backpressure-cancellation. Baseline inventory retains all prior names and raises the launcher floor to49. Whole-repository333 is the updated inventory size, not a claim that this worker ran other packages' native suites.

Complete output, home prefix only normalized:
   Compiling mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave6-attach-cancellation/crates/mantle-launch)
    Finished `test` profile [unoptimized] target(s) in 1.32s
     Running unittests src/lib.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/mantle_launch-4b0355da7ea5620a)

running 45 tests
test cli::tests::env_names_are_validated ... ok
test cli::tests::proxy_urls_carry_no_userinfo_or_path ... ok
test cli::tests::paths_must_be_absolute ... ok
test ctl::tests::line_round_trips ... ok
test ctl::tests::lines_split_across_reads_are_joined ... ok
test cli::tests::full_command_line_parses ... ok
test ctl::tests::non_utf8_is_ignored ... ok
test ctl::tests::over_long_lines_are_dropped_whole ... ok
test ctl::tests::parses_columns_then_rows ... ok
test cli::tests::attach_parses_and_hides_no_tty ... ok
test ctl::tests::refuses_malformed_and_out_of_bounds_lines ... ok
test ring::tests::a_push_larger_than_capacity_keeps_its_tail ... ok
test ring::tests::clear_empties ... ok
test ring::tests::drops_the_oldest_bytes_on_overflow ... ok
test cli::tests::secret_proxy_and_scrollback_are_optional ... ok
test ring::tests::front_and_consume_drain_in_order_across_wraparound ... ok
test cli::tests::dir_and_cwd_are_required ... ok
test ring::tests::push_reports_how_many_bytes_it_dropped ... ok
test cli::tests::removed_tmux_flags_are_refused ... ok
test ring::tests::push_ring_copies_oldest_first_and_respects_the_target_capacity ... ok
test ring::tests::keeps_everything_within_capacity ... ok
test cli::tests::program_is_required_after_separator ... ok
test cli::tests::secret_fd_and_env_require_each_other ... ok
test secret::tests::unopened_descriptor_is_reported ... ok
test serve::tests::a_new_terminal_does_not_inherit_an_owed_repaint ... ok
test serve::tests::a_nudge_holds_a_real_size_change_before_restoring ... ok
test serve::tests::a_nudge_under_way_is_not_restarted_and_can_be_cancelled ... ok
test secret::tests::errors_do_not_echo_the_value ... ok
test serve::tests::a_one_row_window_is_nudged_up ... ok
test serve::tests::nothing_is_repainted_while_no_output_was_dropped ... ok
test serve::tests::poll_does_not_sleep_past_a_pending_restore ... ok
test serve::tests::dropped_output_is_repainted_once_the_terminal_has_caught_up ... ok
test secret::tests::empty_value_is_refused ... ok
test cli::tests::standard_descriptors_are_refused ... ok
test serve::tests::volatile_preflight_refuses_metadata_errors_other_than_absence ... ok
test cli::tests::scrollback_is_bounded ... ok
test secret::tests::trailing_newline_is_trimmed ... ok
test session::tests::a_lock_on_an_unlinked_file_does_not_count ... ok
test secret::tests::oversized_value_is_refused ... ok
test session::tests::a_symlinked_lock_is_refused_and_its_target_left_alone ... ok
test session::tests::open_fifo_refuses_a_symlink_and_a_file_that_is_not_a_pipe ... ok
test session::tests::a_held_lock_is_refused_and_freed_on_drop ... ok
test session::tests::stale_fifos_are_removed_and_other_files_left_alone ... ok
test session::tests::fifos_are_private ... ok
test serve::tests::private_output_refuses_links_and_resets_the_mode ... ok

test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/mantle_launch-b608369d969ef244)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/adversary.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/adversary-be071dee0f9b4017)

running 9 tests
test a_symlinked_session_dir_is_not_followed ... ok
test session_dir_is_owner_only_even_when_it_already_exists ... ok
test last_output_does_not_follow_a_planted_symlink ... ok
test the_secret_descriptor_does_not_reach_the_agent ... ok
test last_output_is_owner_only_even_when_the_file_already_exists ... ok
test exit_code_is_the_agents ... ok
test sigterm_to_serve_ends_the_agents_whole_process_group ... ok
test a_flooded_ctl_pipe_does_not_stall_the_agent ... ok
test a_client_that_never_reads_does_not_block_the_agent ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.21s

     Running tests/adversary_2.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/adversary_2-a15724909d6d3c42)

running 3 tests
test a_size_aware_agent_repaints_on_a_real_resize ... ok
test attaching_at_an_unchanged_size_makes_the_agent_repaint ... ok
test a_terminal_that_lost_output_gets_a_repaint_once_it_catches_up ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.60s

     Running tests/codex_compatibility.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/codex_compatibility-c3ed6525cdd0b861)

running 2 tests

running 1 test
test local_socket_control_rejects_af_unix ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

test local_socket_control_rejects_af_unix ... ok
mantle-launch: received signal 15; stopping the agent
mantle-launch: agent exited (signal: 1 (SIGHUP))
test fifo_launcher_relays_without_unix_sockets_or_secret_slot ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

     Running tests/conformance.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/conformance-6f9433fcff2e5c73)

running 9 tests
test adversary_private_paths_accept_tmpfs_and_preserve_auth_metadata ... ok
test adversary_volatile_preflight_preserves_fifo_and_socket_without_readiness ... ok
test adversary_private_file_special_entries_refuse_without_opening_or_dispatch ... ok
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
49 selected scenario(s), 31 authored source(s), 0 refusal occurrence(s)

test adversary_sigint_while_detached_never_persists_replay ... ok
test private_path_initialization_preserves_auth_and_refuses_unsafe_entries ... ok
AB output mode=pipe signal=15 drained=false cancelled=true
AB output mode=pipe signal=15 drained=false cancelled=true
test adversary_persistent_tail_refuses_volatile_reuse_then_clean_retry_runs ... ok
test volatile_runtime_does_not_persist_output_on_exit ... ok
AB output mode=pipe signal=1 drained=false cancelled=true
AB output mode=pipe signal=1 drained=false cancelled=true
AB output mode=pipe signal=15 drained=true cancelled=true
AB output mode=pipe signal=15 drained=true cancelled=true
AB paused input signal=15 sent=151552 cancelled=true
AB paused input signal=15 sent=151552 cancelled=true
AB paused input signal=1 sent=151552 cancelled=true
AB paused input signal=1 sent=151552 cancelled=true
AB output mode=pty-blocking signal=15 drained=false cancelled=true
AB output mode=pty-blocking signal=15 drained=false cancelled=true
AB output mode=pty-nonblocking signal=1 drained=false cancelled=true
AB output mode=pty-nonblocking signal=1 drained=false cancelled=true
AB output mode=pty-aliased signal=15 drained=false cancelled=true
AB output mode=pty-aliased signal=15 drained=false cancelled=true
test attach_cancellation_under_finite_output_and_input_backpressure ... ok
AB output mode=pipe signal=15 drained=false cancelled=true
AB output mode=pipe signal=1 drained=false cancelled=true
AB output mode=pipe signal=15 drained=true cancelled=true
AB paused input signal=15 sent=151552 cancelled=true
AB paused input signal=1 sent=151552 cancelled=true
AB output mode=pty-blocking signal=15 drained=false cancelled=true
AB output mode=pty-nonblocking signal=1 drained=false cancelled=true
AB output mode=pty-aliased signal=15 drained=false cancelled=true
mantle-launch: 49 passed, 0 failed, 0 unsupported
test ess_launch_conformance ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 18.01s

     Running tests/launch.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/launch-82e925d20d0d1cc0)

running 4 tests
test attach_without_a_server_is_refused ... ok
test secret_stays_out_of_stderr_when_the_agent_cannot_start ... ok
test second_server_is_refused_and_sigterm_stops_the_first ... ok
test attach_relays_replays_scrollback_and_survives_a_detach ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.66s

   Doc-tests mantle_launch

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


cargo clippy --locked -p mantle-launch --all-targets -- -D warnings
exit0; complete output:
    Checking typenum v1.20.1
    Checking cfg-if v1.0.5
    Checking serde_core v1.0.229
    Checking itoa v1.0.18
    Checking memchr v2.8.3
    Checking zmij v1.0.23
    Checking hybrid-array v0.4.15
    Checking serde v1.0.229
    Checking equivalent v1.0.2
    Checking hashbrown v0.17.1
    Checking utf8parse v0.2.2
    Checking anstyle-parse v1.0.0
    Checking serde_json v1.0.151
    Checking indexmap v2.14.2
    Checking generic-array v0.14.7
    Checking dyn-clone v1.0.20
    Checking anstyle v1.0.14
    Checking unsafe-libyaml v0.2.11
    Checking colorchoice v1.0.5
    Checking is_terminal_polyfill v1.70.2
    Checking anstyle-query v1.1.5
    Checking ryu v1.0.23
    Checking anstream v1.0.0
    Checking schemars v0.8.22
    Checking serde_yaml v0.9.34+deprecated
    Checking thiserror v2.0.21
    Checking crypto-common v0.2.2
    Checking block-buffer v0.12.1
    Checking clap_lex v1.1.1
    Checking strsim v0.11.1
    Checking bitflags v2.13.2
    Checking const-oid v0.10.2
    Checking clap_builder v4.6.7
    Checking digest v0.11.3
    Checking ess-primitives v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking libc v0.2.189
    Checking block-buffer v0.10.4
    Checking crypto-common v0.1.7
    Checking cpufeatures v0.3.1
    Checking sha2 v0.11.0
    Checking digest v0.10.7
    Checking clap v4.6.7
    Checking ess-domain v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking anyhow v1.0.104
    Checking cpufeatures v0.2.17
    Checking unicase v2.9.0
    Checking pulldown-cmark-escape v0.11.0
    Checking pulldown-cmark v0.13.4
    Checking sha2 v0.10.9
    Checking linux-raw-sys v0.12.1
    Checking rustix v1.1.5
    Checking getrandom v0.4.3
    Checking mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave6-attach-cancellation/crates/mantle-launch)
    Checking fastrand v2.5.0
    Checking once_cell v1.21.4
    Checking base64 v0.22.1
    Checking ess-compiler v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking tempfile v3.27.0
    Checking ess-gen v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking ess-conformance v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking mantle-conformance v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave6-attach-cancellation/crates/mantle-conformance)
    Finished `dev` profile [unoptimized] target(s) in 19.80s

cargo test --locked -p mantle-worker generated_model_has_no_drift -- --nocapture
exit0, executed1,27 intentionally filtered worker tests. Existing same test passed in inherited worker28 baseline; no same-command baseline was rerun. Complete output:
   Compiling shlex v2.0.1
   Compiling jobserver v0.1.35
   Compiling find-msvc-tools v0.1.14
   Compiling libc v0.2.189
   Compiling cc v1.5.1
   Compiling ring v0.17.14
   Compiling pkg-config v0.3.34
   Compiling zstd-sys v2.1.0+zstd.1.5.7
   Compiling typenum v1.20.1
   Compiling zeroize v1.9.0
   Compiling rustls-pki-types v1.15.1
   Compiling generic-array v0.14.7
   Compiling getrandom v0.2.17
   Compiling untrusted v0.9.0
   Compiling httparse v1.10.1
   Compiling serde_json v1.0.151
   Compiling bytes v1.12.1
   Compiling rustls v0.23.45
   Compiling zstd-safe v7.3.0
   Compiling log v0.4.34
   Compiling cfg_aliases v0.2.2
   Compiling nix v0.30.1
   Compiling http v1.5.0
   Compiling rustls-webpki v0.103.15
   Compiling crypto-common v0.1.7
   Compiling block-buffer v0.10.4
   Compiling errno v0.3.14
   Compiling signal-hook v0.3.18
   Compiling subtle v2.6.1
   Compiling memchr v2.8.3
   Compiling base64 v0.23.1
   Compiling ureq-proto v0.6.4
   Compiling signal-hook-registry v1.4.8
   Compiling digest v0.10.7
   Compiling getrandom v0.4.3
   Compiling webpki-roots v1.0.9
   Compiling percent-encoding v2.3.2
   Compiling utf8-zero v0.8.1
   Compiling sha2 v0.10.9
   Compiling tempfile v3.27.0
   Compiling ureq v3.4.2
   Compiling zstd v0.13.3
   Compiling mantle-worker-model v0.0.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave6-attach-cancellation/generated/worker-model)
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave6-attach-cancellation/crates/mantle-worker)
    Finished `test` profile [unoptimized] target(s) in 13.39s
     Running unittests src/lib.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/mantle_worker-52fb9b1b91688ca5)

running 1 test
test tests::generated_model_has_no_drift ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 27 filtered out; finished in 0.11s

     Running unittests src/main.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/mantle_worker-cf6823bc9926dda3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


cargo fmt --check
exit0, empty output. git diff --check also exits0.

ess specify validate --path spec
exit0; output:
mantle v1 — 7 file(s), 228 scenario(s), valid

ESS0.50 generated the four existing worker-model roots after exact baseline projection/adoption. Generated source/report hashes changed; type definitions and Cargo.toml did not. The required drift test compares every portable artifact and passed.

5. Deliberately not done

No Substrate/SDK change, global signal-policy change, FIFO protocol change, worker change, real credentials/authentication or VM/network integration. No authenticated Claude/Codex conversation parity claim. Cancellation while a connected relay is backpressured is this unit's contract; existing connection-establishment timeout behavior remains. Full workspace gate, independent adversary and parent parity acceptance belong to the coordinator. No broader tests or unrelated changes were added after package gates passed.

6. Outside writes and handoff

Retained raw logs, exact source treatment used for the old-relay red check, baseline spec projection, ESS reference/adoption material, report and file inventory are under ~/.cache/mantle-wave6/scratch-attach; outside-paths.txt enumerates current retained files. All task test temporaries use its tmp child. Compiler output is exclusively /dev/shm/mantle-wave6-target-attach; ordinary pre-existing Cargo/rustc/sccache dependency caches are inherited. Existing tmpfs-dependent tests use /dev/shm/mantle-wave6-runtime-attach. Generated ESS native suite/report/run files are inside this worktree's ignored .engineering/drafts. Managed lease commands write manager-owned lease metadata only.

Own lease codex-mantle-wave6-attach-implementor is ended, recorded in lease-ended.log. The branch remains dirty for coordinator review; nothing was staged, committed or published. The coordinator owns review, integration and worktree/target cleanup. Last resource observation root42GiB/tmpfs28GiB free. No aggregate token/tool metrics are supplied by this host; individual command wall times remain in raw runner output.
