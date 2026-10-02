---
format: aep.planning-md/3
id: review-result:codex-qualification-adversary-1
kind: review-result
status: active
title: Codex qualification adversary pass 1
relations:
- reviews: story:codex-confinement-qualification
revision: 1
---
unit: story:codex-confinement-qualification — uncommitted mantle-wave2-qualification on ff500e4f8008aa96b83acb66ad231e851240182f
verdict: NEEDS-CHANGE
cases: executed 87→88, red 1
origin: introduced 1 / pre-existing 0 / undecided 0
wrote-outside-worktree: 5 retained paths plus automatically removed private test fixtures
needs-coordinator: record pass, route FIFO-open refusal fix, retain test-module registration and rerun corrected case

## 1. Diff and ownership

`git --no-pager diff --stat` (the supplied implementation files remain untracked):

```text
 crates/mantle/Cargo.toml | 4 ++++
 1 file changed, 4 insertions(+)
```

That Cargo change predates this pass and belongs to the coordinator. The implementor's new example, launcher controls and evidence document are also pre-existing working-tree content. I changed none of them. The coordinator registered my test module in the example after my test was written. My sole added worktree file is a Rust test file; its no-index diff against /dev/null is:

```text
 .../tests/codex_qualification_adversary.rs         | 49 ++++++++++++++++++++++
 1 file changed, 49 insertions(+)
```

This report covers the supplied uncommitted implementation plus the coordinator's test-only module registration. No implementation, planning, Cargo, documentation, Git lifecycle, branch or existing test edits were made by this adversary. No charter violation was observed in my changes.

## 2. First attack, before any suite run

Added `crates/mantle/examples/tests/codex_qualification_adversary.rs`. Its case `adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer` feeds a FIFO to the real `verify_digest` function in a separate test child. The parent enforces two seconds, kills/reaps the stuck child and removes its own private scratch before asserting. There are no credentials or model calls. The case is red now because opening the FIFO blocks before the regular-file guard.

For all Cargo test commands: `CARGO_TARGET_DIR=$HOME/.cache/mantle-wave2/target-qualification RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 TMPDIR=$HOME/.cache/mantle-wave2/scratch-qualification/tmp`.

Command: `cargo test -p mantle --example codex_qualification adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer -- --exact --nocapture`. Exit 101. This was the first execution after the attack was written; it compiled and failed the named behavioral assertion. Compiler and runner output below normalizes personal home-directory prefixes to `$HOME/`; raw originals remain in assigned private scratch.

```text
   Compiling mantle v0.1.0 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-wave2-qualification/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 0.57s
     Running unittests examples/codex_qualification.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/examples/codex_qualification-ba921f870707f2bc)

running 1 test

running 1 test

thread 'adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer' (2961758) panicked at crates/mantle/examples/tests/codex_qualification_adversary.rs:45:5:
non-regular binary path blocked beyond the deadline instead of being refused
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... FAILED

failures:

failures:
    adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 6 filtered out; finished in 2.01s

error: test failed, to rerun pass `-p mantle --example codex_qualification`
```

## 3. Full package suite after the red case

Before count 87 is taken from the implementor report, not a pre-attack suite run. Command: `cargo test -p mantle -p mantle-launch --no-fail-fast`. Exit 101. The option continues past the red example so every package lane runs. Executed top-level counts: 19+7+44+9+3+2+4 = 88; 87 passed, one failed. Re-executed nested children are not counted as extra top-level cases. Existing 87 cases remain green. `cargo fmt --all --check` additionally exited 0 without output.

```text
    Finished `test` profile [unoptimized] target(s) in 0.15s
     Running unittests src/main.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/mantle-d8374a956fbc3727)

running 19 tests
test app::terminal::tests::ctrl_bracket_d_detaches_and_forwards_what_came_before ... ok
test app::terminal::tests::escape_followed_by_anything_else_passes_both_bytes ... ok
test app::terminal::tests::the_sequence_is_recognised_across_reads ... ok
test domain::manifest::tests::sizes_and_durations_parse ... ok
test domain::manifest::tests::defaults_apply ... ok
test domain::session::tests::names_round_trip ... ok
test domain::manifest::tests::cwd_must_be_inside_a_mount ... ok
test domain::session::tests::running_is_never_reached_without_starting ... ok
test domain::session::tests::stopped_is_final_and_failures_only_stop ... ok
test domain::session::tests::the_happy_path_is_legal ... ok
test app::worker::tests::the_template_renders_every_placeholder ... ok
test domain::manifest::tests::retain_for_is_bounded_by_the_exec_limit ... ok
test domain::manifest::tests::unserved_fields_are_refused_by_name ... ok
test domain::manifest::tests::the_example_manifest_resolves ... ok
test domain::manifest::tests::mounts_and_refs_cannot_escape ... ok
test domain::manifest::tests::cwd_with_dot_dot_or_empty_components_is_refused ... ok
test adapters::state::tests::an_illegal_move_is_refused_and_not_written ... ok
test adapters::state::tests::sources_round_trip ... ok
test adapters::state::tests::a_live_name_is_unique_until_stopped ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests examples/codex_qualification.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/examples/codex_qualification-ba921f870707f2bc)

running 7 tests
test tests::digest_format_is_strict ... ok
test tests::fixed_observation_never_exports_terminal_payload ... ok
test tests::unsafe_scratch_parents_are_refused_without_creating_children ... ok
test tests::scratch_budget_refuses_large_files_and_does_not_follow_links ... ok

running 1 test
test tests::terminal_observation_is_bounded_and_handles_split_queries ... ok
test tests::cq01_rejects_wrong_digest_before_executing_a_binary ... ok
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... FAILED

failures:

---- adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer stdout ----

thread 'adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer' (2964251) panicked at crates/mantle/examples/tests/codex_qualification_adversary.rs:45:5:
non-regular binary path blocked beyond the deadline instead of being refused
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer

test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.00s

error: test failed, to rerun pass `-p mantle --example codex_qualification`
     Running unittests src/main.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/mantle_launch-71c5942c6fb19817)

running 44 tests
test cli::tests::env_names_are_validated ... ok
test cli::tests::proxy_urls_carry_no_userinfo_or_path ... ok
test ctl::tests::line_round_trips ... ok
test cli::tests::paths_must_be_absolute ... ok
test ctl::tests::non_utf8_is_ignored ... ok
test ctl::tests::lines_split_across_reads_are_joined ... ok
test ctl::tests::parses_columns_then_rows ... ok
test ctl::tests::over_long_lines_are_dropped_whole ... ok
test cli::tests::full_command_line_parses ... ok
test ctl::tests::refuses_malformed_and_out_of_bounds_lines ... ok
test ring::tests::a_push_larger_than_capacity_keeps_its_tail ... ok
test ring::tests::clear_empties ... ok
test cli::tests::secret_proxy_and_scrollback_are_optional ... ok
test ring::tests::drops_the_oldest_bytes_on_overflow ... ok
test ring::tests::front_and_consume_drain_in_order_across_wraparound ... ok
test cli::tests::attach_parses_and_hides_no_tty ... ok
test cli::tests::removed_tmux_flags_are_refused ... ok
test ring::tests::push_reports_how_many_bytes_it_dropped ... ok
test ring::tests::push_ring_copies_oldest_first_and_respects_the_target_capacity ... ok
test ring::tests::keeps_everything_within_capacity ... ok
test cli::tests::secret_fd_and_env_require_each_other ... ok
test secret::tests::unopened_descriptor_is_reported ... ok
test serve::tests::a_new_terminal_does_not_inherit_an_owed_repaint ... ok
test cli::tests::program_is_required_after_separator ... ok
test cli::tests::dir_and_cwd_are_required ... ok
test serve::tests::a_nudge_holds_a_real_size_change_before_restoring ... ok
test cli::tests::scrollback_is_bounded ... ok
test serve::tests::dropped_output_is_repainted_once_the_terminal_has_caught_up ... ok
test cli::tests::standard_descriptors_are_refused ... ok
test serve::tests::nothing_is_repainted_while_no_output_was_dropped ... ok
test secret::tests::empty_value_is_refused ... ok
test serve::tests::a_one_row_window_is_nudged_up ... ok
test serve::tests::poll_does_not_sleep_past_a_pending_restore ... ok
test serve::tests::a_nudge_under_way_is_not_restarted_and_can_be_cancelled ... ok
test session::tests::a_held_lock_is_refused_and_freed_on_drop ... ok
test session::tests::a_lock_on_an_unlinked_file_does_not_count ... ok
test serve::tests::private_output_refuses_links_and_resets_the_mode ... ok
test secret::tests::errors_do_not_echo_the_value ... ok
test secret::tests::trailing_newline_is_trimmed ... ok
test session::tests::a_symlinked_lock_is_refused_and_its_target_left_alone ... ok
test session::tests::fifos_are_private ... ok
test secret::tests::oversized_value_is_refused ... ok
test session::tests::stale_fifos_are_removed_and_other_files_left_alone ... ok
test session::tests::open_fifo_refuses_a_symlink_and_a_file_that_is_not_a_pipe ... ok

test result: ok. 44 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/adversary.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/adversary-a497c909199e9f99)

running 9 tests
test a_symlinked_session_dir_is_not_followed ... ok
test last_output_does_not_follow_a_planted_symlink ... ok
test last_output_is_owner_only_even_when_the_file_already_exists ... ok
test exit_code_is_the_agents ... ok
test session_dir_is_owner_only_even_when_it_already_exists ... ok
test the_secret_descriptor_does_not_reach_the_agent ... ok
test sigterm_to_serve_ends_the_agents_whole_process_group ... ok
test a_flooded_ctl_pipe_does_not_stall_the_agent ... ok
test a_client_that_never_reads_does_not_block_the_agent ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.21s

     Running tests/adversary_2.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/adversary_2-8b45462a8a80a4b6)

running 3 tests
test a_size_aware_agent_repaints_on_a_real_resize ... ok
test attaching_at_an_unchanged_size_makes_the_agent_repaint ... ok
test a_terminal_that_lost_output_gets_a_repaint_once_it_catches_up ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.56s

     Running tests/codex_compatibility.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/codex_compatibility-5e3e4edf41d6ed6a)

running 2 tests

running 1 test
test local_socket_control_rejects_af_unix ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

test local_socket_control_rejects_af_unix ... ok
mantle-launch: received signal 15; stopping the agent
mantle-launch: agent exited (signal: 1 (SIGHUP))
test fifo_launcher_relays_without_unix_sockets_or_secret_slot ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

     Running tests/launch.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/launch-a016fea39c531b39)

running 4 tests
test attach_without_a_server_is_refused ... ok
test secret_stays_out_of_stderr_when_the_agent_cannot_start ... ok
test second_server_is_refused_and_sigterm_stops_the_first ... ok
test attach_relays_replays_scrollback_and_survives_a_detach ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.64s

error: 1 target failed:
    `-p mantle --example codex_qualification`
```

## 4. Findings and reachability

| File:line | Verdict / origin | What was measured | What reaches it |
|---|---|---|---|
| crates/mantle/examples/codex_qualification.rs:116 | NEEDS-CHANGE / introduced | A FIFO binary path blocks in File::open before its regular-file guard, so the bounded probe never reaches its execution deadlines. The new case fails at its line45 after2.01s, exit101. | `local --codex <absolute FIFO> --sha256 <64 hex digits> --launcher <absolute launcher> --scratch-parent <private parent>` calls `local`→`verify_digest`→`digest` before version/TUI deadlines. The CLI accepts that path and documents bounded operation. |

Finding message: A FIFO binary path blocks in File::open before its regular-file guard, so the bounded probe never reaches its execution deadlines.

Origin is introduced because the entire digest implementation and example are new relative to ff500e4f8008aa96b83acb66ad231e851240182f; `git ls-tree` confirms the path absent there. This is a reachable CLI input edge, not a claim that the already measured official regular binary was a FIFO. It does not invalidate the live digest/TUI observations. A nonblocking open followed by descriptor-based regular-file validation should refuse special files without waiting; a path-only precheck would leave a replacement race.

There are no additional judgement findings. Missing authenticated CQ observations remain exactly the existing documented limitations, not new successful acceptance evidence.

## 5. Attacks that did not break the subject

- Existing wrong-digest, malformed-digest and bounded terminal-output tests remained green.
- Existing literal marker-canary test retained its non-export assertion and remained green.
- Existing private scratch, link and size controls remained green.
- Existing local Unix-socket-denial/FIFO-relay controls remained green.
- Documentation explicitly distinguishes byte/marker observations from visual usability, direct sandbox controls from model-issued tools, and not-run authentication from success; no additional contradictory pass claim was established.

## 6. Outside-tree writes and handoff

Retained paths (personal prefix normalized to `$HOME` only):
- `$HOME/.cache/mantle-wave2/scratch-qualification/adversary-1-red.log`
- `$HOME/.cache/mantle-wave2/scratch-qualification/adversary-1-suite.log`
- `$HOME/.cache/mantle-wave2/scratch-qualification/adversary-1.md`
- `$HOME/.cache/mantle-wave2/target-qualification/` (existing coordinator-assigned build tree, updated by Cargo)
- `$HOME/.cache/mantle-wave2/scratch-qualification/tmp/` (existing assigned private test parent)

The new case created and automatically removed two `codex-probe-<ULID>` children beneath that private TMPDIR, each holding its own `candidate-codex` FIFO. Exact deleted ULIDs were not retained. Existing suite tests additionally create/remove their usual `mantle-launch-test-*` fixtures under `$HOME/.cache/claude-tmp/`; sccache/rustc maintain their standard reusable caches. No worker process, gateway, daemon, login or existing session was touched. Free disk remained14GiB above the10GiB floor.

Lease `qualification-adversary1-20261002` ended. Worktree remains ID `mantle-wave2-qualification`, path `$HOME/.local/state/worktree/trees/b10x/mantle/mantle-wave2-qualification`, branch `impl/codex-confinement-qualification`, base ff500e4f8008aa96b83acb66ad231e851240182f. Next owner coordinator: record pass, route correction to implementor, preserve the failing case and obtain corrected gate evidence. No commit or cleanup was performed.

```findings
- file: crates/mantle/examples/codex_qualification.rs
  line: 116
  category: boundary
  severity: blocker
  verdict: NEEDS-CHANGE
  origin: introduced
  message: A FIFO binary path blocks in File::open before its regular-file guard, so the bounded probe never reaches its execution deadlines.
```
