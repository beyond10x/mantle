---
format: aep.planning-md/3
id: review-result:codex-qualification-adversary-2
kind: review-result
status: active
title: Codex qualification adversary final pass 2
relations:
- reviews: story:codex-confinement-qualification
revision: 1
---
unit: story:codex-confinement-qualification — corrected uncommitted mantle-wave2-qualification on ff500e4f8008aa96b83acb66ad231e851240182f
verdict: nothing found
cases: executed 90→91, red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: 5 retained paths plus automatically removed private test fixtures
needs-coordinator: none

## 1. Diff and ownership

`git --no-pager diff --stat`:

```text
 crates/mantle/Cargo.toml | 4 ++++
 1 file changed, 4 insertions(+)
```

That inherited Cargo registration belongs to the coordinator; the implementation, document and test files remain untracked. I edited only `crates/mantle/examples/tests/codex_qualification_adversary.rs`, appending 54 lines to its previous 49 lines without altering the existing assertion. Its complete no-index diff against /dev/null is:

```text
 .../tests/codex_qualification_adversary.rs         | 103 +++++++++++++++++++++
 1 file changed, 103 insertions(+)
```

No implementation, planning, Cargo, documentation, Git lifecycle or existing test changes were made by this adversary. The full corrected example and report were read before this attack.

## 2. Focused attack written before any execution

Added `adversary::binary_symlink_is_validated_against_its_current_opened_object` in the existing test file. First it hashes a regular file selected through a symlink and compares against the literal SHA256 of `abc`. It then replaces the target with a FIFO and re-executes only this case in a child. The child must return the exact object-type refusal, and the parent enforces a two-second deadline with kill/reap and scratch cleanup. This covers a caller selecting a binary via a symlink whose target is replaced between invocations; no concurrent race is claimed. It directly exercises the same digest path reached by `local --codex`.

The first execution was green: the corrected opened-descriptor guard refused the replacement FIFO promptly. No red output exists for this second-pass attack, and no finding is inferred from that absence.

All Cargo invocations used `CARGO_TARGET_DIR=$HOME/.cache/mantle-wave2/target-qualification RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 TMPDIR=$HOME/.cache/mantle-wave2/scratch-qualification/tmp`. Compiler and runner logs below have personal home-directory prefixes normalized to `$HOME`; private original logs remain unchanged.

Command: `cargo test -p mantle --example codex_qualification adversary::binary_symlink_is_validated_against_its_current_opened_object -- --exact --nocapture`, exit 0:

```text
   Compiling mantle v0.1.0 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-wave2-qualification/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 1.01s
     Running unittests examples/codex_qualification.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/examples/codex_qualification-ba921f870707f2bc)

running 1 test

running 1 test
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.01s

```

## 3. Suite after the attack

Before count 90 comes from correction-1.md. Command: `cargo test -p mantle -p mantle-launch --no-fail-fast`, exit 0. Executed top-level cases: 19+10+44+9+3+2+4 = 91. Re-executed children are not counted twice. All passed, including the unchanged first-pass FIFO regression. `cargo fmt --all --check` additionally exited0 without output.

```text
    Finished `test` profile [unoptimized] target(s) in 0.12s
     Running unittests src/main.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/mantle-d8374a956fbc3727)

running 19 tests
test app::terminal::tests::ctrl_bracket_d_detaches_and_forwards_what_came_before ... ok
test app::terminal::tests::escape_followed_by_anything_else_passes_both_bytes ... ok
test app::terminal::tests::the_sequence_is_recognised_across_reads ... ok
test domain::manifest::tests::defaults_apply ... ok
test domain::manifest::tests::cwd_must_be_inside_a_mount ... ok
test domain::session::tests::running_is_never_reached_without_starting ... ok
test domain::manifest::tests::sizes_and_durations_parse ... ok
test domain::session::tests::names_round_trip ... ok
test domain::session::tests::stopped_is_final_and_failures_only_stop ... ok
test app::worker::tests::the_template_renders_every_placeholder ... ok
test domain::manifest::tests::retain_for_is_bounded_by_the_exec_limit ... ok
test domain::session::tests::the_happy_path_is_legal ... ok
test domain::manifest::tests::unserved_fields_are_refused_by_name ... ok
test domain::manifest::tests::the_example_manifest_resolves ... ok
test domain::manifest::tests::cwd_with_dot_dot_or_empty_components_is_refused ... ok
test domain::manifest::tests::mounts_and_refs_cannot_escape ... ok
test adapters::state::tests::sources_round_trip ... ok
test adapters::state::tests::a_live_name_is_unique_until_stopped ... ok
test adapters::state::tests::an_illegal_move_is_refused_and_not_written ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests examples/codex_qualification.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/examples/codex_qualification-ba921f870707f2bc)

running 10 tests
test tests::digest_format_is_strict ... ok
test tests::digest_refuses_other_non_regular_descriptors ... ok
test tests::digest_bounds_actual_input_even_when_it_exceeds_the_size_snapshot ... ok
test tests::fixed_observation_never_exports_terminal_payload ... ok
test tests::unsafe_scratch_parents_are_refused_without_creating_children ... ok
test tests::scratch_budget_refuses_large_files_and_does_not_follow_links ... ok

running 1 test

running 1 test
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

test tests::terminal_observation_is_bounded_and_handles_split_queries ... ok
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok
test tests::cq01_rejects_wrong_digest_before_executing_a_binary ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s

     Running unittests src/main.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/mantle_launch-71c5942c6fb19817)

running 44 tests
test cli::tests::env_names_are_validated ... ok
test cli::tests::proxy_urls_carry_no_userinfo_or_path ... ok
test cli::tests::paths_must_be_absolute ... ok
test cli::tests::full_command_line_parses ... ok
test ctl::tests::line_round_trips ... ok
test ctl::tests::lines_split_across_reads_are_joined ... ok
test ctl::tests::non_utf8_is_ignored ... ok
test cli::tests::attach_parses_and_hides_no_tty ... ok
test ctl::tests::over_long_lines_are_dropped_whole ... ok
test ctl::tests::parses_columns_then_rows ... ok
test cli::tests::secret_proxy_and_scrollback_are_optional ... ok
test ctl::tests::refuses_malformed_and_out_of_bounds_lines ... ok
test ring::tests::a_push_larger_than_capacity_keeps_its_tail ... ok
test ring::tests::clear_empties ... ok
test cli::tests::removed_tmux_flags_are_refused ... ok
test cli::tests::dir_and_cwd_are_required ... ok
test ring::tests::drops_the_oldest_bytes_on_overflow ... ok
test ring::tests::front_and_consume_drain_in_order_across_wraparound ... ok
test cli::tests::program_is_required_after_separator ... ok
test ring::tests::keeps_everything_within_capacity ... ok
test ring::tests::push_reports_how_many_bytes_it_dropped ... ok
test cli::tests::secret_fd_and_env_require_each_other ... ok
test ring::tests::push_ring_copies_oldest_first_and_respects_the_target_capacity ... ok
test secret::tests::unopened_descriptor_is_reported ... ok
test cli::tests::scrollback_is_bounded ... ok
test secret::tests::errors_do_not_echo_the_value ... ok
test serve::tests::a_nudge_holds_a_real_size_change_before_restoring ... ok
test serve::tests::a_new_terminal_does_not_inherit_an_owed_repaint ... ok
test serve::tests::a_nudge_under_way_is_not_restarted_and_can_be_cancelled ... ok
test secret::tests::empty_value_is_refused ... ok
test serve::tests::a_one_row_window_is_nudged_up ... ok
test serve::tests::dropped_output_is_repainted_once_the_terminal_has_caught_up ... ok
test serve::tests::nothing_is_repainted_while_no_output_was_dropped ... ok
test secret::tests::trailing_newline_is_trimmed ... ok
test serve::tests::poll_does_not_sleep_past_a_pending_restore ... ok
test cli::tests::standard_descriptors_are_refused ... ok
test secret::tests::oversized_value_is_refused ... ok
test session::tests::a_held_lock_is_refused_and_freed_on_drop ... ok
test session::tests::stale_fifos_are_removed_and_other_files_left_alone ... ok
test session::tests::a_lock_on_an_unlinked_file_does_not_count ... ok
test session::tests::a_symlinked_lock_is_refused_and_its_target_left_alone ... ok
test session::tests::fifos_are_private ... ok
test serve::tests::private_output_refuses_links_and_resets_the_mode ... ok
test session::tests::open_fifo_refuses_a_symlink_and_a_file_that_is_not_a_pipe ... ok

test result: ok. 44 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/adversary.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/adversary-a497c909199e9f99)

running 9 tests
test a_symlinked_session_dir_is_not_followed ... ok
test session_dir_is_owner_only_even_when_it_already_exists ... ok
test the_secret_descriptor_does_not_reach_the_agent ... ok
test exit_code_is_the_agents ... ok
test last_output_is_owner_only_even_when_the_file_already_exists ... ok
test last_output_does_not_follow_a_planted_symlink ... ok
test sigterm_to_serve_ends_the_agents_whole_process_group ... ok
test a_flooded_ctl_pipe_does_not_stall_the_agent ... ok
test a_client_that_never_reads_does_not_block_the_agent ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.21s

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

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

     Running tests/launch.rs ($HOME/.cache/mantle-wave2/target-qualification/debug/deps/launch-a016fea39c531b39)

running 4 tests
test attach_without_a_server_is_refused ... ok
test secret_stays_out_of_stderr_when_the_agent_cannot_start ... ok
test second_server_is_refused_and_sigterm_stops_the_first ... ok
test attach_relays_replays_scrollback_and_survives_a_detach ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.64s

```

## 4. Findings

Nothing found in this second pass. This is not an approval or completed Codex support claim.

## 5. Attacks that did not break the subject

- The unchanged first-pass FIFO case now completes; a symlink to a replaced FIFO also refuses promptly while a regular symlink target hashes the expected literal value.
- Actual-reader overflow and exact-limit controls remained green; the implementation caps actual bytes read after the descriptor metadata snapshot.
- The report distinguishes the original live-tested helper digest from the corrected helper that has only local gate evidence; it does not relabel old observations as a new live run.
- The report continues to mark authentication, model-issued tools, visible approval behavior, refresh, CONNECT traffic and visual usability as unqualified. No new acceptance claim was inferred from the passing local suite.

## 6. Outside-tree writes and handoff

Retained paths:
- `$HOME/.cache/mantle-wave2/scratch-qualification/adversary-2-case.log`
- `$HOME/.cache/mantle-wave2/scratch-qualification/adversary-2-suite.log`
- `$HOME/.cache/mantle-wave2/scratch-qualification/adversary-2.md`
- `$HOME/.cache/mantle-wave2/target-qualification/` (existing assigned build tree updated by warm Cargo builds)
- `$HOME/.cache/mantle-wave2/scratch-qualification/tmp/` (existing assigned private test parent)

The new case automatically removed two private `codex-probe-<ULID>` fixtures below that TMPDIR, containing its selected symlink and replacement FIFO. Exact deleted leaf names were not retained. Existing tests removed their own usual scratch beneath the same TMPDIR and `$HOME/.cache/claude-tmp/`; standard reusable sccache/rustc caches persist. No worker mutation, authentication, raw terminal capture, unrelated cleanup or new target directory occurred. Host free space remained 11 GiB, above the 10 GiB floor.

Lease `qualification-adversary2-20261002` ended. Worktree ID `mantle-wave2-qualification`, path `$HOME/.local/state/worktree/trees/b10x/mantle/mantle-wave2-qualification`, branch `impl/codex-confinement-qualification`, base ff500e4f8008aa96b83acb66ad231e851240182f remains handed to the coordinator. This completes the second and final adversarial pass. Next actions are coordinator-owned recording and integration under the approved wave.

```findings
[]
```
