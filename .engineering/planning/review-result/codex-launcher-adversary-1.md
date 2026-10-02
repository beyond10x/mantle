---
format: aep.planning-md/3
id: review-result:codex-launcher-adversary-1
kind: review-result
status: active
title: Launcher volatile replay independent attack, pass 1
relations:
- reviews: story:launcher-volatile-replay
revision: 1
---
unit: story:launcher-volatile-replay; working tree mantle-wave4-launcher on 3f05cf06a7c12df69d6864b6849c325246c946bc; staged patch SHA256 5d5ad8d72d83e0c190993dc6903e16de9b201252b3113034d30629ccdb55ae07
verdict: nothing found
cases: executed 65→68, red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: 9 paths (7 retained files and 2 assigned directories)
needs-coordinator: record this pass; complete integration gate and story disposition remain coordinator-owned

```text
$ git --no-pager diff --stat
 crates/mantle-launch/tests/conformance.rs | 112 ++++++++++++++++++++++++++++++
 1 file changed, 112 insertions(+)
```

The diff above is the adversary's unstaged test-only addition. The implementor's 18-path snapshot remains staged, with the SHA256 in the header unchanged after the attack. No production edits, staging, commits, AEP writes, or live integrations were performed. Personal home prefixes are normalized to `~` in this report and embedded output; private raw logs retain exact operational paths.

Added cases, written before execution, all green on their first run:

- `crates/mantle-launch/tests/conformance.rs`: `adversary_volatile_preflight_preserves_fifo_and_socket_without_readiness` invokes the real volatile CLI against a FIFO without a writer and a local Unix socket at `last-output`. It asserts exit 1 within two seconds, no dispatch or readiness, no remaining server lock, fixed refusal diagnostics, and unchanged entry device/inode/mode.
- Same file: `adversary_persistent_tail_refuses_volatile_reuse_then_clean_retry_runs` first runs a real persistent child and verifies its canary reaches the durable tail. A volatile retry refuses before dispatch and preserves that exact tail without diagnostic disclosure. Explicit test-owned removal of the old transcript permits a successful volatile retry, which emits live output and exits 0 without a durable tail. This positive control makes the absence assertions observable.
- Same file: `adversary_sigint_while_detached_never_persists_replay` attaches to a real volatile child, observes its output, detaches, and sends SIGINT to the exact owned launcher PID. The fixture exits 0 through the launcher's existing child-termination path; neither durable replay nor diagnostic canary remains, and the server lock is removed.

The first execution selected only these three additions, before the package suite. There was no red output or failed compilation.

```text
$ cargo test --locked -p mantle-launch --test conformance adversary_ -- --nocapture
   Compiling mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave4-launcher/crates/mantle-launch)
    Finished `test` profile [unoptimized] target(s) in 1.40s
     Running tests/conformance.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/conformance-6f9433fcff2e5c73)

running 3 tests
test adversary_volatile_preflight_preserves_fifo_and_socket_without_readiness ... ok
test adversary_sigint_while_detached_never_persists_replay ... ok
test adversary_persistent_tail_refuses_volatile_reuse_then_clean_retry_runs ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.42s

exit status: 0
```

All Cargo commands used `RUSTC_WRAPPER=`, `CARGO_TARGET_DIR=/dev/shm/mantle-wave4-target-launcher`, `CARGO_BUILD_JOBS=2`, `CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0`, `CARGO_INCREMENTAL=0`, `TMPDIR=/dev/shm/mantle-wave4-target-launcher/tmp`, and `MANTLE_TEST_SCRATCH=~/.cache/mantle-wave4/scratch-adversary/cases` (the actual environment used the expanded absolute home path).

The before count, 65, comes from the implementor's handed-off report. The after count is 68 top-level package tests: 45 library, 9 earlier adversary, 3 second adversary, 2 compatibility, 5 conformance, and 4 launcher. The compatibility test's nested one-test subprocess is not counted twice.

```text
$ cargo test --locked -p mantle-launch
    Finished `test` profile [unoptimized] target(s) in 0.47s
     Running unittests src/lib.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/mantle_launch-4b0355da7ea5620a)

running 45 tests
test cli::tests::proxy_urls_carry_no_userinfo_or_path ... ok
test ring::tests::clear_empties ... ok
test ctl::tests::refuses_malformed_and_out_of_bounds_lines ... ok
test ctl::tests::lines_split_across_reads_are_joined ... ok
test ring::tests::front_and_consume_drain_in_order_across_wraparound ... ok
test ring::tests::keeps_everything_within_capacity ... ok
test ring::tests::a_push_larger_than_capacity_keeps_its_tail ... ok
test ring::tests::drops_the_oldest_bytes_on_overflow ... ok
test ring::tests::push_reports_how_many_bytes_it_dropped ... ok
test cli::tests::paths_must_be_absolute ... ok
test ring::tests::push_ring_copies_oldest_first_and_respects_the_target_capacity ... ok
test serve::tests::a_new_terminal_does_not_inherit_an_owed_repaint ... ok
test cli::tests::secret_proxy_and_scrollback_are_optional ... ok
test cli::tests::removed_tmux_flags_are_refused ... ok
test cli::tests::env_names_are_validated ... ok
test ctl::tests::non_utf8_is_ignored ... ok
test cli::tests::full_command_line_parses ... ok
test secret::tests::empty_value_is_refused ... ok
test cli::tests::standard_descriptors_are_refused ... ok
test cli::tests::dir_and_cwd_are_required ... ok
test serve::tests::nothing_is_repainted_while_no_output_was_dropped ... ok
test ctl::tests::line_round_trips ... ok
test secret::tests::unopened_descriptor_is_reported ... ok
test cli::tests::attach_parses_and_hides_no_tty ... ok
test cli::tests::program_is_required_after_separator ... ok
test ctl::tests::over_long_lines_are_dropped_whole ... ok
test serve::tests::a_nudge_under_way_is_not_restarted_and_can_be_cancelled ... ok
test ctl::tests::parses_columns_then_rows ... ok
test cli::tests::secret_fd_and_env_require_each_other ... ok
test serve::tests::a_one_row_window_is_nudged_up ... ok
test secret::tests::oversized_value_is_refused ... ok
test cli::tests::scrollback_is_bounded ... ok
test serve::tests::dropped_output_is_repainted_once_the_terminal_has_caught_up ... ok
test secret::tests::trailing_newline_is_trimmed ... ok
test serve::tests::volatile_preflight_refuses_metadata_errors_other_than_absence ... ok
test session::tests::a_symlinked_lock_is_refused_and_its_target_left_alone ... ok
test session::tests::fifos_are_private ... ok
test session::tests::open_fifo_refuses_a_symlink_and_a_file_that_is_not_a_pipe ... ok
test session::tests::a_held_lock_is_refused_and_freed_on_drop ... ok
test session::tests::a_lock_on_an_unlinked_file_does_not_count ... ok
test serve::tests::poll_does_not_sleep_past_a_pending_restore ... ok
test session::tests::stale_fifos_are_removed_and_other_files_left_alone ... ok
test serve::tests::private_output_refuses_links_and_resets_the_mode ... ok
test serve::tests::a_nudge_holds_a_real_size_change_before_restoring ... ok
test secret::tests::errors_do_not_echo_the_value ... ok

test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running unittests src/main.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/mantle_launch-b608369d969ef244)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/adversary.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/adversary-be071dee0f9b4017)

running 9 tests
test a_symlinked_session_dir_is_not_followed ... ok
test session_dir_is_owner_only_even_when_it_already_exists ... ok
test last_output_is_owner_only_even_when_the_file_already_exists ... ok
test exit_code_is_the_agents ... ok
test last_output_does_not_follow_a_planted_symlink ... ok
test the_secret_descriptor_does_not_reach_the_agent ... ok
test sigterm_to_serve_ends_the_agents_whole_process_group ... ok
test a_flooded_ctl_pipe_does_not_stall_the_agent ... ok
test a_client_that_never_reads_does_not_block_the_agent ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.13s

     Running tests/adversary_2.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/adversary_2-a15724909d6d3c42)

running 3 tests
test a_size_aware_agent_repaints_on_a_real_resize ... ok
test attaching_at_an_unchanged_size_makes_the_agent_repaint ... ok
test a_terminal_that_lost_output_gets_a_repaint_once_it_catches_up ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.57s

     Running tests/codex_compatibility.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/codex_compatibility-c3ed6525cdd0b861)

running 2 tests

running 1 test
test local_socket_control_rejects_af_unix ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

test local_socket_control_rejects_af_unix ... ok
mantle-launch: received signal 15; stopping the agent
mantle-launch: agent exited (signal: 1 (SIGHUP))
test fifo_launcher_relays_without_unix_sockets_or_secret_slot ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s

     Running tests/conformance.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/conformance-6f9433fcff2e5c73)

running 5 tests
test adversary_volatile_preflight_preserves_fifo_and_socket_without_readiness ... ok
test adversary_sigint_while_detached_never_persists_replay ... ok
test adversary_persistent_tail_refuses_volatile_reuse_then_clean_retry_runs ... ok
test volatile_runtime_does_not_persist_output_on_exit ... ok
test ess_launch_conformance ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 14.00s

     Running tests/launch.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/launch-82e925d20d0d1cc0)

running 4 tests
test attach_without_a_server_is_refused ... ok
test secret_stays_out_of_stderr_when_the_agent_cannot_start ... ok
test second_server_is_refused_and_sigterm_stops_the_first ... ok
test attach_relays_replays_scrollback_and_survives_a_detach ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.66s

   Doc-tests mantle_launch

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

exit status: 0
```

Native conformance ran through `ess_launch_conformance`. Its resulting `.engineering/drafts/mantle-launch-report.json` reports 45 passed, 0 failed, 0 error, 0 skipped, 0 unsupported, and 0 refused; 29 authored and 16 generated. The selection is component `mantle-launch`; 84 obligations are outside that component selection. This is not a whole-system zero-outside claim. Spec digest: `3a08777de0d97b8f4b6127d959d931d021cd3fbac3a529f3522372ec4169c4ea`.

```text
$ cargo fmt --check
(no output)
exit status: 0
$ cargo clippy --locked -p mantle-launch --all-targets -- -D warnings
    Checking mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave4-launcher/crates/mantle-launch)
    Finished `dev` profile [unoptimized] target(s) in 0.48s
exit status: 0
$ ess specify validate --path spec
mantle v1 — 7 file(s), 224 scenario(s), valid
exit status: 0
$ git diff --check
(no output)
exit status: 0
```

Judgement findings: nothing found.

Attacked boundaries and limits:

- Read the complete staged unit diff, source callers, changed contract/scenario files, native adaptation, and active story LP01–LP06 before writing cases.
- Exercised stale special entries through the actual CLI, confirming refusal precedes child dispatch and FIFO readiness; all entries remained intact.
- Exercised a reachable persistent-to-volatile reuse workflow and successful retry after explicit owner cleanup.
- Exercised detached SIGINT against an owned real launcher and child, retaining actual canary and cleanup assertions.
- Ran existing bounded detached replay and unread-client termination scenarios, plus normal, nonzero, SIGTERM, spawn-failure, and forced-death paths in the native package suite.
- Read the existing server-lock ordering, metadata-only refusal, bounded replay ring, and conditional final persistence. Same-UID malicious path races remain outside the story contract.
- No claim is made about Substrate's transport, real Codex credential flows, or the full workspace integration gate. Those were not attacked or executed here.

Outside-worktree paths written, using normalized home prefixes:

1. `~/.cache/mantle-wave4/scratch-adversary/cases.log` — selected first-run output.
2. `~/.cache/mantle-wave4/scratch-adversary/package.log` — complete package output.
3. `~/.cache/mantle-wave4/scratch-adversary/fmt.log` — empty successful formatter output.
4. `~/.cache/mantle-wave4/scratch-adversary/ess.log` — specification validation output.
5. `~/.cache/mantle-wave4/scratch-adversary/tests.patch` — retained test-only patch.
6. `~/.cache/mantle-wave4/scratch-adversary/clippy.log` — complete clippy output.
7. `~/.cache/mantle-wave4/scratch-adversary/report.md` — this report.
8. `~/.cache/mantle-wave4/scratch-adversary/cases/` — assigned test scratch and temporary fixture children.
9. `/dev/shm/mantle-wave4-target-launcher/` — assigned build output, including `tmp/`.

The worktree manager also maintained its lease metadata for session `codex-mantle-wave4-launcher-adversary`. No build directory or managed tree was removed. All invoked builds and test commands exited; the final owned-launcher/conformance process check found no fixture process. Lease release is reported separately in the handoff.

```findings
[]
```
