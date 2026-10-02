unit: launcher-volatile-replay
verdict: green
cases: executed 59→65 top-level Rust tests, red 1 baseline / 9 native observations / 1 decisive runtime regression
origin: n/a
wrote-outside-worktree: ~/.cache/mantle-wave4/scratch-launcher; /dev/shm/mantle-wave4-target-launcher; existing test scratch roots and sccache
needs-coordinator: yes — local bot commit, independent review and complete integration gate

## 1. Unit and outcome

story:launcher-volatile-replay — keep launcher replay in memory without writing last-output.
The Boolean --volatile-replay preserves existing live FIFO/PTTY behavior and bounded rings, refuses any existing last-output entry before FIFO readiness/dispatch, and skips the sole final persistence write. Omission preserves the legacy behavior. LP01–LP06 have native real-process observations; none claims Substrate or Codex privacy.

Source is the managed mantle-wave4-launcher tree, branch impl/launcher-volatile-replay, based on opening 3f05cf06a7c12df69d6864b6849c325246c946bc. No staging, commit or AEP writes were performed. The coordinator explicitly assigned a separate target under /dev/shm, overriding the generic implementor target/ guidance for this unit; it was not shared.

## 2. Observed diff

The tracked diff below excludes six new scenario files shown separately by git status. All six inferred paths proved correct. Two missing admission/index paths, spec/components.yaml and spec/ess-inputs.yaml, were identified before use and added to AEP/authorized by the coordinator. Their exact patch is also retained as scope-additions.patch; it is already applied and is not pending.

 crates/mantle-launch/src/cli.rs             |   4 +
 crates/mantle-launch/src/serve.rs           |  44 ++-
 crates/mantle-launch/tests/conformance.rs   | 504 +++++++++++++++++++++++++++-
 crates/mantle-launch/tests/support/probe.rs |  58 ++++
 generated/worker-model/source.schema.json   |   2 +-
 generated/worker-model/types-report.json    |   4 +-
 generated/worker-model/types.rs             |   4 +-
 spec/README.md                              |  23 +-
 spec/components.yaml                        |   2 +-
 spec/conformance-baseline.json              |  15 +-
 spec/domains/launch.yaml                    |  52 +++
 spec/ess-inputs.yaml                        |   6 +
 12 files changed, 699 insertions(+), 19 deletions(-)

| Scope | Result |
| --- | --- |
| cli.rs / serve.rs | Boolean selection, preflight under lock, final write guard; ENOTDIR inspection regression |
| tests/conformance.rs / tests/support/probe.rs | Five observation commands, bounded collectors/watchdogs, coordinated Rust/clap probe, explicit adopted-child cleanup |
| launch domain / six new LP scenario files | Validated before runtime change; all six execute via native adapter |
| Existing argument-defaults / argument-byte-preservation | Opening commit already added false; this unit does not further alter them |
| components / ess-inputs / conformance-baseline | Preserve old admissions and required names, add all new obligations |
| spec/README.md | Local-only guarantee and inventory324 =100generated+224authored |
| generated/worker-model four files | ESS0.50 regeneration; Cargo.toml unchanged, three provenance-bearing outputs updated |

## 3. Red evidence

The opening baseline package command exited101: its new normalized false expectations failed because production/adapter lacked the field. It executed58 passing top-level Rust tests plus1 failed conformance test; Cargo stopped before the final4 launch tests. This is why the full-suite executed count59→65 must not be mistaken for6 newly added Rust tests: this unit adds2; the other4 are the previously unrun tail. The existing subprocess compatibility test's nested1-test summary is not double counted.

After drafting the native contracts/fixtures, unchanged production selected45 launcher scenarios (29authored), with9 failing observations: red-tests.log, exit101. The draft originally omitted explicit component/input admission and had a partial ServeArgs expectation; both were corrected before implementation. Draft validation then printed 7files,224scenarios,valid and synthesis printed45selected,29authored,0refusals.

The decisive runtime probe accepts the CLI switch but leaves persistence unchanged. This isolates the actual sink behavior from argument rejection. Command (same test/input before and after):

CARGO_TARGET_DIR=/dev/shm/mantle-wave4-target-launcher CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 cargo test --locked -p mantle-launch --test conformance volatile_runtime_does_not_persist_output_on_exit -- --exact --nocapture

   Compiling mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave4-launcher/crates/mantle-launch)
    Finished `test` profile [unoptimized] target(s) in 1.44s
     Running tests/conformance.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/conformance-6f9433fcff2e5c73)

running 1 test

thread 'volatile_runtime_does_not_persist_output_on_exit' (3870039) panicked at crates/mantle-launch/tests/conformance.rs:889:5:
assertion `left == right` failed
  left: Array [Bool(true), Bool(true), Bool(true), Bool(false), Bool(false)]
 right: Array [Bool(false), Bool(false), Bool(false), Bool(false), Bool(false)]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test volatile_runtime_does_not_persist_output_on_exit ... FAILED

failures:

failures:
    volatile_runtime_does_not_persist_output_on_exit

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.66s

error: test failed, to rerun pass `-p mantle-launch --test conformance`

Exit101. Clean, nonzero and SIGTERM exits demonstrably wrote last-output. The minimal conditional write and preflight corrected those observations. The first treated native run exposed one fixture assumption: closing the killed server's PTY can make the child exit before cleanup SIGKILL. The fixture now still kills and explicitly waitpid-reaps the exact adopted child, but reports its actual status instead of asserting the unrelated terminal signal. The required reaped=true assertion remains; killed_child_exit adds the actual observation.

## 4. Green evidence and checks

Final package command uses the same build environment plus MANTLE_TEST_SCRATCH=~/.cache/mantle-wave4/scratch-launcher/cases:

cargo test --locked -p mantle-launch

   Compiling mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave4-launcher/crates/mantle-launch)
    Finished `test` profile [unoptimized] target(s) in 1.47s
     Running unittests src/lib.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/mantle_launch-4b0355da7ea5620a)

running 45 tests
test cli::tests::env_names_are_validated ... ok
test cli::tests::proxy_urls_carry_no_userinfo_or_path ... ok
test cli::tests::paths_must_be_absolute ... ok
test ctl::tests::line_round_trips ... ok
test cli::tests::full_command_line_parses ... ok
test cli::tests::attach_parses_and_hides_no_tty ... ok
test ctl::tests::lines_split_across_reads_are_joined ... ok
test ctl::tests::non_utf8_is_ignored ... ok
test ctl::tests::over_long_lines_are_dropped_whole ... ok
test cli::tests::secret_proxy_and_scrollback_are_optional ... ok
test ctl::tests::parses_columns_then_rows ... ok
test cli::tests::dir_and_cwd_are_required ... ok
test ctl::tests::refuses_malformed_and_out_of_bounds_lines ... ok
test ring::tests::a_push_larger_than_capacity_keeps_its_tail ... ok
test ring::tests::clear_empties ... ok
test cli::tests::removed_tmux_flags_are_refused ... ok
test ring::tests::drops_the_oldest_bytes_on_overflow ... ok
test ring::tests::front_and_consume_drain_in_order_across_wraparound ... ok
test ring::tests::keeps_everything_within_capacity ... ok
test ring::tests::push_reports_how_many_bytes_it_dropped ... ok
test ring::tests::push_ring_copies_oldest_first_and_respects_the_target_capacity ... ok
test cli::tests::program_is_required_after_separator ... ok
test secret::tests::unopened_descriptor_is_reported ... ok
test secret::tests::errors_do_not_echo_the_value ... ok
test cli::tests::secret_fd_and_env_require_each_other ... ok
test serve::tests::a_nudge_holds_a_real_size_change_before_restoring ... ok
test serve::tests::a_nudge_under_way_is_not_restarted_and_can_be_cancelled ... ok
test serve::tests::a_new_terminal_does_not_inherit_an_owed_repaint ... ok
test serve::tests::poll_does_not_sleep_past_a_pending_restore ... ok
test secret::tests::empty_value_is_refused ... ok
test cli::tests::standard_descriptors_are_refused ... ok
test serve::tests::dropped_output_is_repainted_once_the_terminal_has_caught_up ... ok
test serve::tests::a_one_row_window_is_nudged_up ... ok
test secret::tests::trailing_newline_is_trimmed ... ok
test serve::tests::nothing_is_repainted_while_no_output_was_dropped ... ok
test cli::tests::scrollback_is_bounded ... ok
test session::tests::a_held_lock_is_refused_and_freed_on_drop ... ok
test serve::tests::volatile_preflight_refuses_metadata_errors_other_than_absence ... ok
test session::tests::a_lock_on_an_unlinked_file_does_not_count ... ok
test secret::tests::oversized_value_is_refused ... ok
test session::tests::a_symlinked_lock_is_refused_and_its_target_left_alone ... ok
test serve::tests::private_output_refuses_links_and_resets_the_mode ... ok
test session::tests::open_fifo_refuses_a_symlink_and_a_file_that_is_not_a_pipe ... ok
test session::tests::fifos_are_private ... ok
test session::tests::stale_fifos_are_removed_and_other_files_left_alone ... ok

test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/mantle_launch-b608369d969ef244)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/adversary.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/adversary-be071dee0f9b4017)

running 9 tests
test a_symlinked_session_dir_is_not_followed ... ok
test last_output_does_not_follow_a_planted_symlink ... ok
test last_output_is_owner_only_even_when_the_file_already_exists ... ok
test the_secret_descriptor_does_not_reach_the_agent ... ok
test exit_code_is_the_agents ... ok
test session_dir_is_owner_only_even_when_it_already_exists ... ok
test sigterm_to_serve_ends_the_agents_whole_process_group ... ok
test a_flooded_ctl_pipe_does_not_stall_the_agent ... ok
test a_client_that_never_reads_does_not_block_the_agent ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.21s

     Running tests/adversary_2.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/adversary_2-a15724909d6d3c42)

running 3 tests
test a_size_aware_agent_repaints_on_a_real_resize ... ok
test attaching_at_an_unchanged_size_makes_the_agent_repaint ... ok
test a_terminal_that_lost_output_gets_a_repaint_once_it_catches_up ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.70s

     Running tests/codex_compatibility.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/codex_compatibility-c3ed6525cdd0b861)

running 2 tests

running 1 test
test local_socket_control_rejects_af_unix ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

test local_socket_control_rejects_af_unix ... ok
mantle-launch: received signal 15; stopping the agent
mantle-launch: agent exited (signal: 1 (SIGHUP))
test fifo_launcher_relays_without_unix_sockets_or_secret_slot ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s

     Running tests/conformance.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/conformance-6f9433fcff2e5c73)

running 2 tests
test volatile_runtime_does_not_persist_output_on_exit ... ok
test ess_launch_conformance ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 14.27s

     Running tests/launch.rs (/dev/shm/mantle-wave4-target-launcher/debug/deps/launch-82e925d20d0d1cc0)

running 4 tests
test attach_without_a_server_is_refused ... ok
test secret_stays_out_of_stderr_when_the_agent_cannot_start ... ok
test second_server_is_refused_and_sigterm_stops_the_first ... ok
test attach_relays_replays_scrollback_and_survives_a_detach ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.64s

   Doc-tests mantle_launch

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


Exit0. Lane counts read from runner summaries:

- Package: executed59→65, exit0; opening stopped at its1failed native test, current65 all pass.
- Library unit tests: executed44→45, exit0.
- Native Rust test harness: executed1→2, exit0; native scenario report34→45,45passed/0failed/0unsupported/0skipped.
- Adversary9→9, adversary_2 3→3, compatibility2→2, unchanged existing launch integration lane final4pass.
- Exact runtime regression: executed1→1, redexit101→greenexit0; it was added before the persistence correction, so equal counts are expected and the observed file-presence values changed.
- Generated worker provenance: executed1, exit0. Initial mistyped --exact selection ran0 and is retained as provenance-zero-selected.log; it is not validation. Correct tests::generated_model_has_no_drift ran1andpassed.

Other checks:

- cargo clippy --locked -p mantle-launch --all-targets -- -D warnings: exit0.
- cargo fmt --check: exit0.
- ess specify validate --path spec: exit0, mantle v1 — 7file(s),224scenario(s),valid.
- Complete synthesis:324selected,224authored,0refusals; synthesis only, not a full-workspace execution claim.
- Previous required scenario names are all retained in the324-scenario suite: preserved-inventory.log true, exit0. No existing authored scenario changed in this unit.
- git diff --check: exit0.
- Generated model digest:3a08777de0d97b8f4b6127d959d931d021cd3fbac3a529f3522372ec4169c4ea.

ESS initially refused the generated destination as unowned. Generated an exact reference from b3372f0's spec in scratch, then used ess output adopt --ownership-root generated/worker-model --from <scratch>/baseline-model/reference --owner model-types. Initial owner spelling types:rust was refused; the named admitted family model-types succeeded. No ownership data/generated source was hand-edited. After adoption, ESS regeneration and the existing byte-for-byte provenance test succeeded. Complete synthesis initially refused the scenario parent directory because it does not recurse; the corrected --scenarios spec uses the explicit ess-inputs.yaml list.

## 5. Limits, cleanup and handoff

No Substrate SDK/code, capture-mode claims, real credentials, Codex login, transport, encryption, attachment policy marker, network integration or agent selection was changed. Complete-workspace execution/no-op audit/AEP/website gates belong to coordinator integration. No additional hardening was started after the required package checks passed.

New fixture collectors cap each observation at64KiB and5seconds. Preflight observes inotify creation/move events so transient ctl readiness cannot disappear unnoticed. Flood emits2MiB while detached, waits for a non-output progress marker, allows250ms for two server poll turns, then compares exactly1024 newest replay bytes before redraw; its second unread client exercises the14second termination budget. Synthetic output never becomes an actual credential. SIGKILL is not described as normal launcher cleanup.

After tests, no process command line matched this unit's target launcher or conformance binaries. Test tempdirs are removed by their owners; no managed tree/build directory was removed. Own lease is released immediately after this report, coordinator owns next action and cleanup. Observed storage: root17GiB free; tmpfs29GiB free; own target554MiB, scratch1.3MiB before final native evidence copy. Both reserve thresholds held.

## 6. Outside-worktree paths

- ~/.cache/mantle-wave4/scratch-launcher/ — report.md; baseline/red/treatment/final test logs and exits; exact runtime red/green logs; fmt/clippy/ESS logs; provenance logs (including zero-selected attempt); generation/adoption logs; scope-additions.patch; baseline-model/{spec,reference}; complete-suite.json; synthesis logs; baseline-inventory.json; preserved-inventory.log; mantle-launch-{suite,report,run}.json; cases/ temporary test directories.
- /dev/shm/mantle-wave4-target-launcher/ — assigned isolated build output.
- ~/.cache/mantle-conformance/ — pre-override existing conformance scratch tempdirs, removed by TempDir.
- ~/.cache/claude-tmp/ — pre-override existing launcher unit/integration scratch paths, owned test cleanup.
- Existing machine-configured sccache storage — compiler cache writes; not modified/configured/cleaned by this unit.

Raw logs retain local paths. This report normalizes home prefixes for repository evidence consumption. No secret values were used.
