unit:                   story:codex-confinement-qualification — correction round 1
verdict:                green
cases:                  executed 88→90, red 1→0
origin:                 n/a
wrote-outside-worktree: $HOME/.cache/mantle-wave2/scratch-qualification/correction-1* and assigned target/tmp directories
needs-coordinator:      no source patch; coordinator owns independent pass 2 and final integration

## 1. Finding, fix and class

Retained adversary case and registration without edits. Reproduced the exact FIFO-open hang red before changing implementation (2.01 seconds, exit 101).

Fix: OpenOptions read plus O_NONBLOCK, then fstat the descriptor and require a regular file. No path-only precheck introduces a replacement gap. Preserve initial 512 MiB metadata refusal, but additionally cap the actual reader to 512 MiB + one detection byte and refuse an overflowing stream. A growing regular file cannot continue hashing indefinitely beyond the byte limit.

Class rule: caller-supplied binary inputs must be validated on the opened object without waiting for a producer, and advertised byte bounds must cover actual reads, not only a mutable metadata snapshot.

Enumeration:
- FIFO: independently authored deadline/reap case now green; unchanged.
- Directory and character device: new real descriptor controls refuse `/` and `/dev/zero`.
- Symlinks: opening follows the selected object, then validates that same descriptor; a replacement path does not change the already opened descriptor. No path-only type claim is used.
- Socket paths/other unopenable objects: open errors remain refusals; no regular-file acceptance inferred from pathname.
- Already oversized regular file: original metadata limit remains enforced.
- File growing after metadata: new reader boundary case uses a 32 KiB cursor and an 8-byte test limit; it refuses after exactly nine bytes, with no large allocation. Exact-limit input still hashes correctly.
- Official immutable worker binary: digest unchanged; this input-validation correction does not reinterpret existing live evidence.

Scope: changed only original example and evidence document. The adversary file and coordinator Cargo registration remain present and unchanged. No production, dependencies, planning, specification, deployment, Taskfile, branch, staging or commits.

## 2. Actual diff shape

Files remain untracked as before; no staging was authorized. Current no-index stats:
```text
 .../mantle/examples/codex_qualification.rs         | 782 +++++++++++++++++++++
 1 file changed, 782 insertions(+)
 /dev/null => docs/evidence/codex-compatibility.md | 208 ++++++++++++++++++++++
 1 file changed, 208 insertions(+)
```

## 3. Exact red reproduced before fix

All logs below have personal home-directory prefixes normalized to `$HOME`; originals remain private. Every Cargo invocation used `TMPDIR=$HOME/.cache/mantle-wave2/scratch-qualification/tmp CARGO_TARGET_DIR=$HOME/.cache/mantle-wave2/target-qualification RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0`.

Command: `cargo test -p mantle --example codex_qualification adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer -- --exact --nocapture`, exit 101.
```text
    Finished `test` profile [unoptimized] target(s) in 0.30s
     Running unittests examples/codex_qualification.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/examples/codex_qualification-ba921f870707f2bc)

running 1 test

running 1 test

thread 'adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer' (2990444) panicked at crates/mantle/examples/tests/codex_qualification_adversary.rs:45:5:
non-regular binary path blocked beyond the deadline instead of being refused
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... FAILED

failures:

failures:
    adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 6 filtered out; finished in 2.01s

error: test failed, to rerun pass `-p mantle --example codex_qualification`
```

## 4. Green correction and gates

Exact same filtered command, exit 0; outer case executed one before/after. Child summary is not an extra top-level case.
```text
   Compiling mantle v0.1.0 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-wave2-qualification/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 0.59s
     Running unittests examples/codex_qualification.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/examples/codex_qualification-ba921f870707f2bc)

running 1 test

running 1 test
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.01s

```

Same full-package command as adversary: `cargo test -p mantle -p mantle-launch --no-fail-fast`, exit 0. Executed 88 → 90; before from adversary runner summaries 19+7+44+9+3+2+4, after 19+9+44+9+3+2+4. Example lane executed 7 → 9 with one failing case corrected; all other lanes unchanged at 19/44/9/3/2/4 and green. Both FIFO and seccomp control children print nested one-case summaries, excluded from totals.
```text
    Finished `test` profile [unoptimized] target(s) in 0.15s
     Running unittests src/main.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/mantle-d8374a956fbc3727)

running 19 tests
test app::terminal::tests::the_sequence_is_recognised_across_reads ... ok
test app::terminal::tests::escape_followed_by_anything_else_passes_both_bytes ... ok
test app::terminal::tests::ctrl_bracket_d_detaches_and_forwards_what_came_before ... ok
test domain::manifest::tests::sizes_and_durations_parse ... ok
test domain::session::tests::names_round_trip ... ok
test domain::session::tests::running_is_never_reached_without_starting ... ok
test domain::session::tests::stopped_is_final_and_failures_only_stop ... ok
test domain::session::tests::the_happy_path_is_legal ... ok
test domain::manifest::tests::defaults_apply ... ok
test app::worker::tests::the_template_renders_every_placeholder ... ok
test domain::manifest::tests::cwd_must_be_inside_a_mount ... ok
test domain::manifest::tests::retain_for_is_bounded_by_the_exec_limit ... ok
test domain::manifest::tests::unserved_fields_are_refused_by_name ... ok
test domain::manifest::tests::the_example_manifest_resolves ... ok
test domain::manifest::tests::mounts_and_refs_cannot_escape ... ok
test domain::manifest::tests::cwd_with_dot_dot_or_empty_components_is_refused ... ok
test adapters::state::tests::sources_round_trip ... ok
test adapters::state::tests::an_illegal_move_is_refused_and_not_written ... ok
test adapters::state::tests::a_live_name_is_unique_until_stopped ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests examples/codex_qualification.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/examples/codex_qualification-ba921f870707f2bc)

running 9 tests
test tests::digest_refuses_other_non_regular_descriptors ... ok
test tests::digest_bounds_actual_input_even_when_it_exceeds_the_size_snapshot ... ok
test tests::digest_format_is_strict ... ok
test tests::unsafe_scratch_parents_are_refused_without_creating_children ... ok
test tests::fixed_observation_never_exports_terminal_payload ... ok
test tests::scratch_budget_refuses_large_files_and_does_not_follow_links ... ok

running 1 test
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

test tests::terminal_observation_is_bounded_and_handles_split_queries ... ok
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok
test tests::cq01_rejects_wrong_digest_before_executing_a_binary ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.30s

     Running unittests src/main.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/mantle_launch-71c5942c6fb19817)

running 44 tests
test ctl::tests::parses_columns_then_rows ... ok
test ctl::tests::refuses_malformed_and_out_of_bounds_lines ... ok
test ring::tests::clear_empties ... ok
test ctl::tests::non_utf8_is_ignored ... ok
test ring::tests::a_push_larger_than_capacity_keeps_its_tail ... ok
test ctl::tests::line_round_trips ... ok
test ctl::tests::lines_split_across_reads_are_joined ... ok
test cli::tests::env_names_are_validated ... ok
test cli::tests::proxy_urls_carry_no_userinfo_or_path ... ok
test ctl::tests::over_long_lines_are_dropped_whole ... ok
test ring::tests::drops_the_oldest_bytes_on_overflow ... ok
test ring::tests::front_and_consume_drain_in_order_across_wraparound ... ok
test ring::tests::push_reports_how_many_bytes_it_dropped ... ok
test ring::tests::push_ring_copies_oldest_first_and_respects_the_target_capacity ... ok
test ring::tests::keeps_everything_within_capacity ... ok
test secret::tests::unopened_descriptor_is_reported ... ok
test serve::tests::a_nudge_holds_a_real_size_change_before_restoring ... ok
test serve::tests::a_new_terminal_does_not_inherit_an_owed_repaint ... ok
test serve::tests::a_nudge_under_way_is_not_restarted_and_can_be_cancelled ... ok
test secret::tests::empty_value_is_refused ... ok
test serve::tests::a_one_row_window_is_nudged_up ... ok
test serve::tests::nothing_is_repainted_while_no_output_was_dropped ... ok
test serve::tests::poll_does_not_sleep_past_a_pending_restore ... ok
test secret::tests::errors_do_not_echo_the_value ... ok
test serve::tests::dropped_output_is_repainted_once_the_terminal_has_caught_up ... ok
test secret::tests::trailing_newline_is_trimmed ... ok
test cli::tests::paths_must_be_absolute ... ok
test cli::tests::secret_proxy_and_scrollback_are_optional ... ok
test secret::tests::oversized_value_is_refused ... ok
test cli::tests::full_command_line_parses ... ok
test cli::tests::removed_tmux_flags_are_refused ... ok
test cli::tests::attach_parses_and_hides_no_tty ... ok
test cli::tests::secret_fd_and_env_require_each_other ... ok
test cli::tests::program_is_required_after_separator ... ok
test cli::tests::dir_and_cwd_are_required ... ok
test cli::tests::standard_descriptors_are_refused ... ok
test cli::tests::scrollback_is_bounded ... ok
test session::tests::a_held_lock_is_refused_and_freed_on_drop ... ok
test session::tests::fifos_are_private ... ok
test session::tests::open_fifo_refuses_a_symlink_and_a_file_that_is_not_a_pipe ... ok
test session::tests::stale_fifos_are_removed_and_other_files_left_alone ... ok
test session::tests::a_lock_on_an_unlinked_file_does_not_count ... ok
test session::tests::a_symlinked_lock_is_refused_and_its_target_left_alone ... ok
test serve::tests::private_output_refuses_links_and_resets_the_mode ... ok

test result: ok. 44 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/adversary.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/adversary-a497c909199e9f99)

running 9 tests
test a_symlinked_session_dir_is_not_followed ... ok
test last_output_does_not_follow_a_planted_symlink ... ok
test last_output_is_owner_only_even_when_the_file_already_exists ... ok
test session_dir_is_owner_only_even_when_it_already_exists ... ok
test the_secret_descriptor_does_not_reach_the_agent ... ok
test exit_code_is_the_agents ... ok
test sigterm_to_serve_ends_the_agents_whole_process_group ... ok
test a_flooded_ctl_pipe_does_not_stall_the_agent ... ok
test a_client_that_never_reads_does_not_block_the_agent ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.09s

     Running tests/adversary_2.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/adversary_2-8b45462a8a80a4b6)

running 3 tests
test a_size_aware_agent_repaints_on_a_real_resize ... ok
test attaching_at_an_unchanged_size_makes_the_agent_repaint ... ok
test a_terminal_that_lost_output_gets_a_repaint_once_it_catches_up ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.70s

     Running tests/codex_compatibility.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/codex_compatibility-5e3e4edf41d6ed6a)

running 2 tests

running 1 test
test local_socket_control_rejects_af_unix ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

test local_socket_control_rejects_af_unix ... ok
mantle-launch: received signal 15; stopping the agent
mantle-launch: agent exited (signal: 1 (SIGHUP))
test fifo_launcher_relays_without_unix_sockets_or_secret_slot ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s

     Running tests/launch.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/launch-a016fea39c531b39)

running 4 tests
test attach_without_a_server_is_refused ... ok
test secret_stays_out_of_stderr_when_the_agent_cannot_start ... ok
test second_server_is_refused_and_sigterm_stops_the_first ... ok
test attach_relays_replays_scrollback_and_survives_a_detach ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.63s

```

`cargo clippy -p mantle -p mantle-launch --all-targets -- -D warnings`, exit 0:
```text
    Checking mantle v0.1.0 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-wave2-qualification/crates/mantle)
    Finished `dev` profile [unoptimized] target(s) in 0.60s
```

`cargo build -p mantle --example codex_qualification`, exit 0:
```text
   Compiling mantle v0.1.0 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-wave2-qualification/crates/mantle)
    Finished `dev` profile [unoptimized] target(s) in 1.09s
```

`cargo fmt --all --check`: exit 0, no output.
`git diff --check`: exit 0, no output.
Corrected executable SHA256: `1ffbe3d87627bbdeb6eb4a4d930321e74540995b958a1fef9212b47f5d7a8463`.

## 5. Limits and unchanged evidence

The live-tested old helper SHA remains `c52cc6d1805ffc619b67e378945054c4d3700fd22b02aa3e27d60edf2aaed7a3`; worker live probes were not rerun for this input-validation-only correction. The document explicitly labels both identities and separates pre-review87 cases from current90 cases. Authentication, model-issued commands, actual approval prompts, refresh, CONNECT and human terminal usability remain at their previously documented unverified status.

No worker access, network/model request or production change occurred in this correction. Host free space was11 GiB throughout warm verification, above10 GiB floor; no disposable output was removed.

## 6. Outside-tree paths and handoff

Retained additions (personal prefix normalized):
- $HOME/.cache/mantle-wave2/scratch-qualification/correction-1-red.log
- $HOME/.cache/mantle-wave2/scratch-qualification/correction-1-green.log
- $HOME/.cache/mantle-wave2/scratch-qualification/correction-1-suite.log
- $HOME/.cache/mantle-wave2/scratch-qualification/correction-1-clippy.log
- $HOME/.cache/mantle-wave2/scratch-qualification/correction-1-build.log
- $HOME/.cache/mantle-wave2/scratch-qualification/correction-1-fmt.log
- $HOME/.cache/mantle-wave2/scratch-qualification/correction-1.md
- $HOME/.cache/mantle-wave2/target-qualification/ (existing assigned target, updated)
- $HOME/.cache/mantle-wave2/scratch-qualification/tmp/ (generated private test children removed; exact deleted ULIDs not retained)

Existing launcher suites also create/remove their original private fixtures under $HOME/.cache/claude-tmp; standard sccache/rustc caches persist. Lease qualification-correction1-20261002 released at handoff. Next owner coordinator: adversarial pass2 and integration; no implementor commit, staging or tree cleanup.
