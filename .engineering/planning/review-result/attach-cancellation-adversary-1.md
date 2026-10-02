---
format: aep.planning-md/3
id: review-result:attach-cancellation-adversary-1
kind: review-result
status: active
title: Attachment backpressure cancellation adversary pass 1
relations:
- reviews: story:attach-backpressure-cancellation
revision: 1
---
unit: story:attach-backpressure-cancellation; staged working tree mantle-wave6-attach-cancellation on a0f8dd8fc7f886af839f63cfc477470a7c3a7b15; staged patch SHA256 6d411bbf964de4dd7b8c6cb7b830dbfca5605dfa5b6b2c564623576508062095
verdict: nothing found
cases: executed 72→75, red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: 15 paths (10 retained files and 5 assigned directories)
needs-coordinator: record pass 1; full integration gate and parent parity acceptance remain coordinator-owned

```text
$ git --no-pager diff --stat
 crates/mantle-launch/tests/conformance.rs | 129 ++++++++++++++++++++++++++++++
 1 file changed, 129 insertions(+)
```

The diff above is the adversary's unstaged test-only addition. The implementor's complete13-path snapshot remains staged and its patch hash is unchanged. No implementation, specification or old assertion was edited. No staging, commits, AEP writes, live worker/Substrate operation, credentials or authentication were performed. Personal home prefixes are normalized to `~` in this report and embedded output; private raw logs retain exact operational paths.

All three cases were written before the first execution, then each was run individually before the package suite. All passed on their first run, with no fixture correction or compilation failure:

- `crates/mantle-launch/tests/conformance.rs`: `adversary_stalled_aliased_terminal_services_resize_then_sigint` establishes an actual aliased-PTY client, emits a finite2MiB burst without draining its terminal, changes the inherited PTY to137 columns/43 rows, and signals SIGWINCH. It observes that exact window through the synthetic agent's PTY metadata within1500ms, while the client remains live, then sends handled SIGINT and asserts bounded successful exit, exact flags/termios restoration, the same surviving server/agent and a usable replacement client. No terminal contents are read through `/proc`; only the owned fixture's PTY window is queried. Green.
- Same file: `adversary_closed_stdout_restores_pty_and_flags_and_releases_client_lock` establishes a real PTY client, closes its output reader, and asks the live synthetic agent to emit. The EPIPE path must exit successfully within1500ms and restore exact inherited flags/termios without a cancellation signal; the same server/agent survives and a replacement client requests a fresh response. This exercises a reachable terminal-reader departure path. Green.
- Same file: `adversary_nonblocking_setup_failure_restores_earlier_aliased_descriptors` calls the production descriptor guard with two aliases of a writable pipe followed by an O_PATH metadata descriptor. The later F_SETFL must fail with EBADF and restore both earlier aliases exactly; a positive control then successfully sets nonblocking mode through the same guard and restores it on drop. This is a guard failure-path boundary fixture, not a claim that normal Mantle requests supply O_PATH standard descriptors. Green.

Individual test-first commands and complete output:

```text
$ cargo test --locked -p mantle-launch --test conformance adversary_stalled_aliased_terminal_services_resize_then_sigint -- --exact --nocapture
   Compiling mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave6-attach-cancellation/crates/mantle-launch)
    Finished `test` profile [unoptimized] target(s) in 0.80s
     Running tests/conformance.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/conformance-6f9433fcff2e5c73)

running 1 test
test adversary_stalled_aliased_terminal_services_resize_then_sigint ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.45s

exit status: 0
$ cargo test --locked -p mantle-launch --test conformance adversary_closed_stdout_restores_pty_and_flags_and_releases_client_lock -- --exact --nocapture
    Finished `test` profile [unoptimized] target(s) in 0.08s
     Running tests/conformance.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/conformance-6f9433fcff2e5c73)

running 1 test
test adversary_closed_stdout_restores_pty_and_flags_and_releases_client_lock ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.25s

exit status: 0
$ cargo test --locked -p mantle-launch --test conformance adversary_nonblocking_setup_failure_restores_earlier_aliased_descriptors -- --exact --nocapture
    Finished `test` profile [unoptimized] target(s) in 0.10s
     Running tests/conformance.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/conformance-6f9433fcff2e5c73)

running 1 test
test adversary_nonblocking_setup_failure_restores_earlier_aliased_descriptors ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

exit status: 0
```

The before count72 comes from the implementor's handed-off report, not a pre-attack suite run. The final launcher count75 comprises45 library,9 earlier adversary,3 second adversary,2 compatibility,12 conformance and4 launcher tests. Nested compatibility subprocess output is not counted twice. The separate generated-model drift check below executes1 worker test and is not added to the launcher count.

All Cargo commands used the configured compiler wrapper and `CARGO_TARGET_DIR=/dev/shm/mantle-wave6-target-attach`, `CARGO_BUILD_JOBS=2`, `CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0`, `CARGO_INCREMENTAL=0`, `TMPDIR=~/.cache/mantle-wave6/scratch-attach-adversary-1/tmp`, `MANTLE_TEST_SCRATCH=~/.cache/mantle-wave6/scratch-attach-adversary-1/cases`, and `MANTLE_ADVERSARY_RUNTIME=/dev/shm/mantle-wave6-runtime-adversary-1`. Home prefixes were expanded in the actual environment.

Package suite, executed after the individual cases:

```text
$ cargo test --locked -p mantle-launch -- --nocapture
    Finished `test` profile [unoptimized] target(s) in 0.10s
     Running unittests src/lib.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/mantle_launch-4b0355da7ea5620a)

running 45 tests
test cli::tests::env_names_are_validated ... ok
test cli::tests::proxy_urls_carry_no_userinfo_or_path ... ok
test cli::tests::paths_must_be_absolute ... ok
test cli::tests::full_command_line_parses ... ok
test ctl::tests::line_round_trips ... ok
test ctl::tests::lines_split_across_reads_are_joined ... ok
test ctl::tests::non_utf8_is_ignored ... ok
test ctl::tests::over_long_lines_are_dropped_whole ... ok
test cli::tests::attach_parses_and_hides_no_tty ... ok
test ctl::tests::parses_columns_then_rows ... ok
test ctl::tests::refuses_malformed_and_out_of_bounds_lines ... ok
test cli::tests::secret_proxy_and_scrollback_are_optional ... ok
test ring::tests::a_push_larger_than_capacity_keeps_its_tail ... ok
test ring::tests::clear_empties ... ok
test ring::tests::drops_the_oldest_bytes_on_overflow ... ok
test ring::tests::front_and_consume_drain_in_order_across_wraparound ... ok
test ring::tests::keeps_everything_within_capacity ... ok
test cli::tests::dir_and_cwd_are_required ... ok
test ring::tests::push_reports_how_many_bytes_it_dropped ... ok
test ring::tests::push_ring_copies_oldest_first_and_respects_the_target_capacity ... ok
test cli::tests::program_is_required_after_separator ... ok
test cli::tests::removed_tmux_flags_are_refused ... ok
test secret::tests::unopened_descriptor_is_reported ... ok
test serve::tests::a_nudge_holds_a_real_size_change_before_restoring ... ok
test serve::tests::a_nudge_under_way_is_not_restarted_and_can_be_cancelled ... ok
test secret::tests::errors_do_not_echo_the_value ... ok
test cli::tests::secret_fd_and_env_require_each_other ... ok
test serve::tests::a_one_row_window_is_nudged_up ... ok
test serve::tests::a_new_terminal_does_not_inherit_an_owed_repaint ... ok
test serve::tests::nothing_is_repainted_while_no_output_was_dropped ... ok
test serve::tests::poll_does_not_sleep_past_a_pending_restore ... ok
test cli::tests::standard_descriptors_are_refused ... ok
test secret::tests::empty_value_is_refused ... ok
test cli::tests::scrollback_is_bounded ... ok
test serve::tests::dropped_output_is_repainted_once_the_terminal_has_caught_up ... ok
test secret::tests::trailing_newline_is_trimmed ... ok
test serve::tests::volatile_preflight_refuses_metadata_errors_other_than_absence ... ok
test session::tests::a_symlinked_lock_is_refused_and_its_target_left_alone ... ok
test session::tests::fifos_are_private ... ok
test session::tests::a_lock_on_an_unlinked_file_does_not_count ... ok
test session::tests::a_held_lock_is_refused_and_freed_on_drop ... ok
test serve::tests::private_output_refuses_links_and_resets_the_mode ... ok
test secret::tests::oversized_value_is_refused ... ok
test session::tests::open_fifo_refuses_a_symlink_and_a_file_that_is_not_a_pipe ... ok
test session::tests::stale_fifos_are_removed_and_other_files_left_alone ... ok

test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/mantle_launch-b608369d969ef244)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/adversary.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/adversary-be071dee0f9b4017)

running 9 tests
test a_symlinked_session_dir_is_not_followed ... ok
test last_output_does_not_follow_a_planted_symlink ... ok
test the_secret_descriptor_does_not_reach_the_agent ... ok
test last_output_is_owner_only_even_when_the_file_already_exists ... ok
test exit_code_is_the_agents ... ok
test session_dir_is_owner_only_even_when_it_already_exists ... ok
test sigterm_to_serve_ends_the_agents_whole_process_group ... ok
test a_flooded_ctl_pipe_does_not_stall_the_agent ... ok
test a_client_that_never_reads_does_not_block_the_agent ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.21s

     Running tests/adversary_2.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/adversary_2-a15724909d6d3c42)

running 3 tests
test a_size_aware_agent_repaints_on_a_real_resize ... ok
test attaching_at_an_unchanged_size_makes_the_agent_repaint ... ok
test a_terminal_that_lost_output_gets_a_repaint_once_it_catches_up ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.70s

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

running 12 tests
test adversary_nonblocking_setup_failure_restores_earlier_aliased_descriptors ... ok
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

test private_path_initialization_preserves_auth_and_refuses_unsafe_entries ... ok
test adversary_private_paths_accept_tmpfs_and_preserve_auth_metadata ... ok
test adversary_sigint_while_detached_never_persists_replay ... ok
AB output mode=pipe signal=15 drained=false cancelled=true
test adversary_closed_stdout_restores_pty_and_flags_and_releases_client_lock ... ok
test adversary_persistent_tail_refuses_volatile_reuse_then_clean_retry_runs ... ok
AB output mode=pipe signal=15 drained=false cancelled=true
test adversary_stalled_aliased_terminal_services_resize_then_sigint ... ok
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

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 18.16s

     Running tests/launch.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/launch-82e925d20d0d1cc0)

running 4 tests
test attach_without_a_server_is_refused ... ok
test secret_stays_out_of_stderr_when_the_agent_cannot_start ... ok
test second_server_is_refused_and_sigterm_stops_the_first ... ok
test attach_relays_replays_scrollback_and_survives_a_detach ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.64s

   Doc-tests mantle_launch

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

exit status: 0
```

The native launcher report records49 passed,0 failed/error/skipped/unsupported/refused, with31 authored and18 generated scenarios. Its87 outside obligations belong to other components; this is not a whole-workspace zero-outside claim. Spec digest: `2dc5342bfece562e4b6ac7402c8f029d37cb77225cef07625c9af65b7718ef37`.

```text
$ cargo clippy --locked -p mantle-launch --all-targets -- -D warnings
    Checking mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave6-attach-cancellation/crates/mantle-launch)
    Finished `dev` profile [unoptimized] target(s) in 0.51s
exit status: 0
$ cargo test --locked -p mantle-worker generated_model_has_no_drift -- --nocapture
    Finished `test` profile [unoptimized] target(s) in 0.13s
     Running unittests src/lib.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/mantle_worker-52fb9b1b91688ca5)

running 1 test
test tests::generated_model_has_no_drift ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 27 filtered out; finished in 0.14s

     Running unittests src/main.rs (/dev/shm/mantle-wave6-target-attach/debug/deps/mantle_worker-cf6823bc9926dda3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

exit status: 0
$ cargo fmt --check
(no output)
exit status: 0
$ ess specify validate --path spec
mantle v1 — 7 file(s), 228 scenario(s), valid
exit status: 0
$ git diff --check
(no output)
exit status: 0
```

Judgement findings: nothing found.

Attack coverage and limits:

- Read complete unit source/test/spec/generated diff, AB01–AB04 acceptance, new ESS response/scenario, implementor report and actual attach/connect/guard/raw-mode/poll/signal callers before writing cases.
- Exercised resize forwarding while terminal output remains unread, then handled SIGINT with aliased descriptors; exact agent window and subsequent fresh response provide positive observations beyond process exit.
- Exercised reader disappearance/EPIPE cleanup and exact state restoration without a signal; reattachment establishes actual client-lock release and surviving server usability.
- Exercised partial setup failure and successful setup through the actual nonblocking guard, including descriptor aliases.
- Package/native scenarios retain handled TERM/HUP under output and independently paused input, drained control, blocking/nonblocking/aliased PTYs, finite producer bounds,24KiB short-write burst,128KiB exact bidirectional relay, resize/redraw, duplicate-client, replay and private-path behavior.
- Read fixed16KiB-per-direction pending buffers and poll/write gating; no runtime/spec mutation or deliberately weakened assertion was used.
- Existing connection-establishment timeout behavior and authenticated Claude/Codex conversation continuity remain outside this unit's connected-relay contract. No broader integration or live acceptance was claimed.

Outside-worktree paths written, with portable home prefixes:

1. `~/.cache/mantle-wave6/scratch-attach-adversary-1/` — assigned scratch/report root.
2. `~/.cache/mantle-wave6/scratch-attach-adversary-1/tmp/` — assigned private temporary children.
3. `~/.cache/mantle-wave6/scratch-attach-adversary-1/cases/` — owned launcher fixture children.
4. `/dev/shm/mantle-wave6-runtime-adversary-1/` — assigned tmpfs fixture parent.
5. `/dev/shm/mantle-wave6-target-attach/` — sequentially assigned Cargo output.
6. `~/.cache/mantle-wave6/scratch-attach-adversary-1/case-resize.log`
7. `~/.cache/mantle-wave6/scratch-attach-adversary-1/case-closed-output.log`
8. `~/.cache/mantle-wave6/scratch-attach-adversary-1/case-setup-rollback.log`
9. `~/.cache/mantle-wave6/scratch-attach-adversary-1/package.log`
10. `~/.cache/mantle-wave6/scratch-attach-adversary-1/fmt.log`
11. `~/.cache/mantle-wave6/scratch-attach-adversary-1/ess.log`
12. `~/.cache/mantle-wave6/scratch-attach-adversary-1/clippy.log`
13. `~/.cache/mantle-wave6/scratch-attach-adversary-1/drift.log`
14. `~/.cache/mantle-wave6/scratch-attach-adversary-1/tests.patch`
15. `~/.cache/mantle-wave6/scratch-attach-adversary-1/report.md`

The compiler wrapper manages its existing shared cache; the worktree manager maintains lease metadata for `codex-mantle-wave6-attach-adversary-1`. Neither was manually edited or cleaned. Native suite/report artifacts remain inside the worktree's ignored `.engineering/drafts/`. Initial observed free space41GiB root and28GiB tmpfs exceeded assigned floors. All invoked test/build commands exited; the final owned launcher/conformance process check found no live fixture process. Lease release and immutable report hash are supplied at handoff. No managed worktree or build directory was removed. Aggregate token/tool/wall metrics are unavailable from this host.

```findings
[]
```
