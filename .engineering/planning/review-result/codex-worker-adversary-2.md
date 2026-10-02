---
format: aep.planning-md/3
id: review-result:codex-worker-adversary-2
kind: review-result
status: active
title: Agent-ready worker adversary, final attack 2
relations:
- reviews: story:agent-ready-worker
revision: 1
---
unit: story:agent-ready-worker; e54bd4aeb8a939e10c4448fada1f84d02019b46b atop b3a9c456b9e2098ff1c237f6a3cf2ca3aa480dae; full unit patch SHA256 a65e860a96e55802a9c59dd526fd7911792da632b6bee6c22ac8882f08e9c9e6
verdict: nothing found
cases: executed 161→164, red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: 11 paths (10 retained files and one disposable build directory)
needs-coordinator: record this final pass and run findings ledger; fresh/existing live AW acceptance remains separate

## 1. Test-only diff

`git --no-pager diff --stat`:

```text
 crates/mantle-worker/src/lib.rs | 145 ++++++++++++++++++++++++++++++++++++++++
 1 file changed, 145 insertions(+)
```

The sole hunk is `@@ -818,0 +819,145 @@`, inside the existing `#[cfg(test)] mod tests`. The activated brief explicitly permits additions in this mixed Rust file's test region. Production/index bytes remain unchanged, and the complete index diff against the named integration base still hashes to the activated digest. All first-pass cases and correction assertions remain. No production edit, staging, commit, AEP operation, live action or credential read occurred.

## 2. New attack cases before any suite run

Before-count161 is the implementor's reconciled handoff, not a preemptive suite run. Three added helper cases raise the complete workspace total to164; nested isolated fixture invocations are not counted twice. Each case remains in the assigned helper's test region:

- `adversary_second_successful_child_exit_cleans_its_remaining_group`: a Rust child fixture exits successfully with a real sleep descendant still in its group and with closed inherited output pipes. The actual bounded transport must report success and stop that descendant. Green.
- `adversary_second_failed_spawn_preserves_following_transport_and_idle_state`: the real transport refuses a nonexistent executable, then successfully returns literal bytes from a later child; its isolated registry has no children, no interruption and restored idle signal behavior. Green.
- `adversary_second_update_observers_only_see_complete_verified_generations`: install one real fixture executable, concurrently inspect during installation of a different verified executable, and require every observation to contain the complete old or new facts. The final stable executable equals the new bytes, the final manifest is new, and the previous immutable generation remains. Green.

The first new-case run exposed an invalid fixture assumption: `/usr/bin/false --version` exits nonzero. Its assertion failed before the replacement transaction under attack began. I changed only that newly authored fixture executable to `/usr/bin/printf`, retaining all success, generation, observation and byte assertions. The original output is retained in `adversary-2-invalid-fixture.log`; it is not classified as a product defect or a resolved first-pass finding. No existing test was changed.

Initial fixture failure, `cargo test --locked -p mantle-worker adversary_second_ -- --nocapture`, exit101:

```text
   Compiling mantle-worker-model v0.0.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/generated/worker-model)
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
    Finished `test` profile [unoptimized] target(s) in 4.54s
     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-52fb9b1b91688ca5)

running 3 tests
test tests::adversary_second_successful_child_exit_cleans_its_remaining_group ... ok
test tests::adversary_second_failed_spawn_preserves_following_transport_and_idle_state ... ok

thread 'tests::adversary_second_update_observers_only_see_complete_verified_generations' (1298664) panicked at crates/mantle-worker/src/lib.rs:980:9:
isolated tests::adversary_second_update_observers_only_see_complete_verified_generations exited exit status: 101

running 1 test
test tests::adversary_second_update_observers_only_see_complete_verified_generations ... FAILED

failures:

failures:
    tests::adversary_second_update_observers_only_see_complete_verified_generations

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 27 filtered out; finished in 0.04s



thread 'tests::adversary_second_update_observers_only_see_complete_verified_generations' (1298668) panicked at crates/mantle-worker/src/lib.rs:914:9:
assertion failed: version.status.success()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test tests::adversary_second_update_observers_only_see_complete_verified_generations ... FAILED

failures:

failures:
    tests::adversary_second_update_observers_only_see_complete_verified_generations

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 25 filtered out; finished in 0.05s

error: test failed, to rerun pass `-p mantle-worker --lib`
```

After correcting the fixture, the same selected new-case run, still before the complete suite, exited0:

```text
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
    Finished `test` profile [unoptimized] target(s) in 2.46s
     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-52fb9b1b91688ca5)

running 3 tests
test tests::adversary_second_successful_child_exit_cleans_its_remaining_group ... ok
test tests::adversary_second_failed_spawn_preserves_following_transport_and_idle_state ... ok
test tests::adversary_second_update_observers_only_see_complete_verified_generations ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 25 filtered out; finished in 0.09s

     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-cf6823bc9926dda3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

## 3. Complete workspace and native gates

All Cargo runs used the assigned `/dev/shm/mantle-wave3-target-worker`, private `TMPDIR=/dev/shm/mantle-wave3-target-worker/tmp`, empty `RUSTC_WRAPPER`, jobs2, dev/test debug0 and incremental0. Before compilation root had19GiB available and tmpfs24GiB, above the assigned2/10GiB floors. No build output was removed or shared with the coordinator's target.

`cargo test --workspace --locked`, exit0. Actual top-level lane counts are26+10+1+2+25+6+3+44+9+3+2+1+4+28 =164, all passed, zero failed/ignored. Native CLI/egress/launcher conformance tests ran as part of this complete workspace command; their actual reports feed the aggregate below. The helper lane25→28 contains all first-pass/correction cases plus the three additions. Zero-test CLI/doc-test lanes and nested fixture summaries add no cases.

```text
   Compiling mantle-worker-model v0.0.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/generated/worker-model)
   Compiling mantle-docs v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-docs)
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 24.97s
     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle-0e72e51ffb98da8f)

running 26 tests
test adapters::state::tests::an_illegal_move_is_refused_and_not_written ... ok
test domain::manifest::tests::cwd_must_be_inside_a_mount ... ok
test domain::manifest::tests::cwd_with_dot_dot_or_empty_components_is_refused ... ok
test app::terminal::tests::the_sequence_is_recognised_across_reads ... ok
test adapters::state::tests::a_live_name_is_unique_until_stopped ... ok
test domain::manifest::tests::retain_for_is_bounded_by_the_exec_limit ... ok
test app::worker::tests::the_template_renders_every_placeholder ... ok
test domain::manifest::tests::mounts_and_refs_cannot_escape ... ok
test adapters::substrate::worker_tests::common_readiness_and_selected_claude_are_distinct_observations ... ok
test config::tests::explicit_path_selection_is_absolute_and_preserves_defaults ... ok
test domain::manifest::tests::sizes_and_durations_parse ... ok
test adapters::state::tests::sources_round_trip ... ok
test app::terminal::tests::ctrl_bracket_d_detaches_and_forwards_what_came_before ... ok
test domain::session::tests::the_happy_path_is_legal ... ok
test domain::session::tests::stopped_is_final_and_failures_only_stop ... ok
test domain::session::tests::running_is_never_reached_without_starting ... ok
test domain::manifest::tests::defaults_apply ... ok
test domain::manifest::tests::unserved_fields_are_refused_by_name ... ok
test domain::session::tests::names_round_trip ... ok
test app::worker::tests::legacy_claude_bootstrap_retains_required_slot ... ok
test app::terminal::tests::escape_followed_by_anything_else_passes_both_bytes ... ok
test domain::manifest::tests::the_example_manifest_resolves ... ok
test config::tests::common_worker_configuration_needs_no_claude_credentials ... ok
test config::tests::legacy_claude_table_remains_parseable ... ok
test app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command ... ok
test adapters::state::conformance::ess_generated_local_conformance ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.08s

     Running unittests examples/codex_qualification.rs (/dev/shm/mantle-wave3-target-worker/debug/examples/codex_qualification-04ed43213458c938)

running 10 tests
test tests::fixed_observation_never_exports_terminal_payload ... ok
test tests::digest_bounds_actual_input_even_when_it_exceeds_the_size_snapshot ... ok
test tests::digest_refuses_other_non_regular_descriptors ... ok
test tests::scratch_budget_refuses_large_files_and_does_not_follow_links ... ok
test tests::digest_format_is_strict ... ok
test tests::unsafe_scratch_parents_are_refused_without_creating_children ... ok

running 1 test

running 1 test
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok
test tests::terminal_observation_is_bounded_and_handles_split_queries ... ok
test tests::cq01_rejects_wrong_digest_before_executing_a_binary ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.76s

     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_conformance-3351e25d6a902300)

running 1 test
test tests::observations_preserve_full_integer_width ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_conformance-349561a0ba511922)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_docs-08817e77d99364c8)

running 2 tests
test tests::authored_links_resolve ... ok
test tests::broken_anchors_and_wrong_asset_bases_are_refused ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_egress-dee410d83fd8a7bc)

running 25 tests
test addr::tests::refused_ipv6_ranges ... ok
test addr::tests::public_addresses_are_permitted ... ok
test addr::tests::refused_ipv4_ranges ... ok
test allow::tests::malformed_targets ... ok
test head::tests::malformed_heads ... ok
test allow::tests::resolver_name_is_fully_qualified ... ok
test head::tests::valid_connect ... ok
test allow::tests::default_list_has_eight_hosts_on_443 ... ok
test allow::tests::match_is_exact_not_suffix ... ok
test allow::tests::match_is_case_insensitive ... ok
test allow::tests::trailing_dot_is_stripped ... ok
test proxy::tests::response_shape ... ok
test allow::tests::allow_entry_rejects_ip_literal ... ok
test head::tests::wrong_method ... ok
test log::tests::formats_known_instants ... ok
test allow::tests::ip_literals_are_refused ... ok
test head::tests::read_head_terminator_split_across_reads ... ok
test head::tests::read_head_oversize ... ok
test allow::tests::port_must_match_exactly ... ok
test head::tests::connect_without_headers ... ok
test head::tests::read_head_keeps_bytes_past_the_head ... ok
test head::tests::read_head_exactly_at_cap ... ok
test head::tests::read_head_closed_early ... ok
test head::tests::read_head_slow_times_out ... ok
2026-10-02T12:15:50.630Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=1
2026-10-02T12:15:50.646Z shutting down: no longer accepting connections
2026-10-02T12:15:50.646Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=14
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=14
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=14
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=14
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=14
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=14
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=14
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=14
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=14
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=14
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=14
2026-10-02T12:15:50.647Z dest=- outcome=refused:client-closed up=0 down=0 ms=14
2026-10-02T12:15:50.648Z dest=- outcome=refused:client-closed up=0 down=0 ms=14
2026-10-02T12:15:50.648Z dest=- outcome=refused:client-closed up=0 down=0 ms=14
2026-10-02T12:15:50.648Z dest=- outcome=refused:client-closed up=0 down=0 ms=3
2026-10-02T12:15:50.648Z dest=- outcome=refused:client-closed up=0 down=0 ms=3
2026-10-02T12:15:50.648Z dest=- outcome=refused:client-closed up=0 down=0 ms=2
2026-10-02T12:15:50.648Z dest=- outcome=refused:client-closed up=0 down=0 ms=2
2026-10-02T12:15:50.648Z dest=- outcome=refused:client-closed up=0 down=0 ms=2
2026-10-02T12:15:50.648Z dest=- outcome=refused:client-closed up=0 down=0 ms=2
2026-10-02T12:15:50.648Z dest=- outcome=refused:client-closed up=0 down=0 ms=2
2026-10-02T12:15:50.648Z dest=- outcome=refused:client-closed up=0 down=0 ms=2
2026-10-02T12:15:50.648Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=2
2026-10-02T12:15:50.708Z drain timeout reached; closing open tunnels
2026-10-02T12:15:50.711Z shutting down: no longer accepting connections
2026-10-02T12:15:50.772Z drain timeout reached; closing open tunnels
2026-10-02T12:15:50.777Z dest=- outcome=refused:client-closed up=0 down=0 ms=0
2026-10-02T12:15:50.777Z shutting down: no longer accepting connections
2026-10-02T12:15:50.782Z dest=github.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T12:15:50.782Z shutting down: no longer accepting connections
2026-10-02T12:15:50.783Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T12:15:50.783Z shutting down: no longer accepting connections
2026-10-02T12:15:50.784Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T12:15:50.784Z shutting down: no longer accepting connections
2026-10-02T12:15:50.785Z dest=- outcome=refused:client-closed up=0 down=0 ms=0
2026-10-02T12:15:50.785Z shutting down: no longer accepting connections
2026-10-02T12:15:50.786Z dest=github.com:443 outcome=refused:connect-failed up=0 down=0 ms=0
2026-10-02T12:15:50.786Z shutting down: no longer accepting connections
2026-10-02T12:15:50.787Z dest=github.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T12:15:50.787Z shutting down: no longer accepting connections
2026-10-02T12:15:50.849Z dest=github.com:443 outcome=refused:connect-timeout up=0 down=0 ms=61
2026-10-02T12:15:50.849Z shutting down: no longer accepting connections
2026-10-02T12:15:50.853Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=1
2026-10-02T12:15:50.861Z shutting down: no longer accepting connections
2026-10-02T12:15:50.861Z dest=- outcome=refused:client-closed up=0 down=0 ms=7
2026-10-02T12:15:50.861Z dest=- outcome=refused:client-closed up=0 down=0 ms=7
2026-10-02T12:15:50.861Z dest=- outcome=refused:client-closed up=0 down=0 ms=7
2026-10-02T12:15:50.861Z dest=- outcome=refused:client-closed up=0 down=0 ms=7
2026-10-02T12:15:50.862Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.863Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.863Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.863Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.863Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.864Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.864Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.864Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.864Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.864Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.865Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.865Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.865Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.865Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.865Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.865Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.866Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.866Z dest=- outcome=refused:client-closed up=0 down=0 ms=8
2026-10-02T12:15:50.866Z dest=- outcome=refused:client-closed up=0 down=0 ms=7
2026-10-02T12:15:50.866Z dest=- outcome=refused:client-closed up=0 down=0 ms=7
2026-10-02T12:15:50.866Z dest=- outcome=refused:client-closed up=0 down=0 ms=7
2026-10-02T12:15:50.866Z dest=- outcome=refused:client-closed up=0 down=0 ms=7
2026-10-02T12:15:50.866Z dest=- outcome=refused:client-closed up=0 down=0 ms=7
2026-10-02T12:15:50.866Z dest=- outcome=refused:client-closed up=0 down=0 ms=7
2026-10-02T12:15:50.867Z dest=- outcome=refused:client-closed up=0 down=0 ms=7
2026-10-02T12:15:50.867Z dest=- outcome=refused:client-closed up=0 down=0 ms=7
2026-10-02T12:15:50.867Z dest=- outcome=refused:client-closed up=0 down=0 ms=7
2026-10-02T12:15:50.867Z dest=- outcome=refused:client-closed up=0 down=0 ms=7
2026-10-02T12:15:50.868Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=8
2026-10-02T12:15:50.922Z drain timeout reached; closing open tunnels
2026-10-02T12:15:50.986Z shutting down: no longer accepting connections
2026-10-02T12:15:51.088Z drain timeout reached; closing open tunnels
2026-10-02T12:15:51.152Z dest=github.com:443 outcome=allowed end=idle up=0 down=0 ms=61
2026-10-02T12:15:51.152Z shutting down: no longer accepting connections
2026-10-02T12:15:51.155Z dest=169.254.169.254 outcome=refused:ip-literal up=0 down=0 ms=0
2026-10-02T12:15:51.155Z shutting down: no longer accepting connections
2026-10-02T12:15:51.159Z dest=- outcome=refused:malformed up=0 down=0 ms=1
2026-10-02T12:15:51.159Z shutting down: no longer accepting connections
2026-10-02T12:15:51.165Z dest=github.com:443 outcome=allowed end=eof up=4 down=4 ms=2
2026-10-02T12:15:51.165Z shutting down: no longer accepting connections
2026-10-02T12:15:51.168Z dest=- outcome=refused:method up=0 down=0 ms=0
2026-10-02T12:15:51.168Z shutting down: no longer accepting connections
2026-10-02T12:15:51.172Z dest=- outcome=refused:head-too-large up=0 down=0 ms=0
2026-10-02T12:15:51.172Z shutting down: no longer accepting connections
2026-10-02T12:15:51.175Z dest=github.com:80 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T12:15:51.175Z shutting down: no longer accepting connections
2026-10-02T12:15:51.177Z dest=github.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T12:15:51.179Z shutting down: no longer accepting connections
2026-10-02T12:15:51.182Z dest=github.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T12:15:51.182Z shutting down: no longer accepting connections
2026-10-02T12:15:51.184Z dest=github.com:443 outcome=refused:non-public-address up=0 down=0 ms=0
2026-10-02T12:15:51.185Z shutting down: no longer accepting connections
2026-10-02T12:15:51.249Z dest=github.com:443 outcome=refused:dns-failed up=0 down=0 ms=61
2026-10-02T12:15:51.249Z shutting down: no longer accepting connections
2026-10-02T12:15:51.251Z shutting down: no longer accepting connections
2026-10-02T12:15:51.313Z drain timeout reached; closing open tunnels
2026-10-02T12:15:51.316Z dest=github.com.attacker.test:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T12:15:51.317Z shutting down: no longer accepting connections
2026-10-02T12:15:51.318Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T12:15:51.318Z shutting down: no longer accepting connections
test proxy::conformance::ess_egress_conformance ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.06s

     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_egress-8b1a663a1538d966)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/adversary.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/adversary-9d3c4af2de31d47b)

running 6 tests
test ipv4_benchmarking_and_documentation_ranges_are_refused ... ok
test ipv6_siit_translated_form_embedding_metadata_is_refused ... ok
test ipv4_ietf_protocol_block_is_refused ... ok
test ipv6_non_global_unicast_space_is_refused ... ok
test ipv6_local_use_nat64_prefix_embedding_metadata_is_refused ... ok
test silent_sockets_do_not_starve_a_well_formed_request ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

     Running tests/refusal.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/refusal-db63098d9bfa952a)

running 3 tests
2026-10-02T12:15:51.582Z dest=example.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T12:15:51.582Z dest=github.com:80 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T12:15:51.583Z dest=- outcome=refused:method up=0 down=0 ms=0
2026-10-02T12:15:51.583Z dest=localhost:43579 outcome=refused:non-public-address up=0 down=0 ms=2
test refuses_unlisted_destination_with_403 ... ok
test refuses_non_connect ... ok
test allowed_name_resolving_to_loopback_is_not_dialled ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s

     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_launch-b94d68bc2f8cbdb8)

running 44 tests
test cli::tests::env_names_are_validated ... ok
test cli::tests::full_command_line_parses ... ok
test cli::tests::proxy_urls_carry_no_userinfo_or_path ... ok
test ctl::tests::line_round_trips ... ok
test ctl::tests::lines_split_across_reads_are_joined ... ok
test cli::tests::secret_fd_and_env_require_each_other ... ok
test cli::tests::attach_parses_and_hides_no_tty ... ok
test cli::tests::dir_and_cwd_are_required ... ok
test cli::tests::program_is_required_after_separator ... ok
test cli::tests::removed_tmux_flags_are_refused ... ok
test cli::tests::scrollback_is_bounded ... ok
test cli::tests::standard_descriptors_are_refused ... ok
test cli::tests::paths_must_be_absolute ... ok
test cli::tests::secret_proxy_and_scrollback_are_optional ... ok
test ctl::tests::parses_columns_then_rows ... ok
test ctl::tests::over_long_lines_are_dropped_whole ... ok
test ctl::tests::non_utf8_is_ignored ... ok
test ring::tests::front_and_consume_drain_in_order_across_wraparound ... ok
test ring::tests::push_reports_how_many_bytes_it_dropped ... ok
test secret::tests::unopened_descriptor_is_reported ... ok
test serve::tests::a_one_row_window_is_nudged_up ... ok
test ctl::tests::refuses_malformed_and_out_of_bounds_lines ... ok
test serve::tests::dropped_output_is_repainted_once_the_terminal_has_caught_up ... ok
test ring::tests::push_ring_copies_oldest_first_and_respects_the_target_capacity ... ok
test serve::tests::a_new_terminal_does_not_inherit_an_owed_repaint ... ok
test serve::tests::nothing_is_repainted_while_no_output_was_dropped ... ok
test serve::tests::poll_does_not_sleep_past_a_pending_restore ... ok
test ring::tests::a_push_larger_than_capacity_keeps_its_tail ... ok
test ring::tests::clear_empties ... ok
test session::tests::open_fifo_refuses_a_symlink_and_a_file_that_is_not_a_pipe ... ok
test serve::tests::private_output_refuses_links_and_resets_the_mode ... ok
test session::tests::stale_fifos_are_removed_and_other_files_left_alone ... ok
test session::tests::a_lock_on_an_unlinked_file_does_not_count ... ok
test secret::tests::trailing_newline_is_trimmed ... ok
test ring::tests::drops_the_oldest_bytes_on_overflow ... ok
test session::tests::a_held_lock_is_refused_and_freed_on_drop ... ok
test serve::tests::a_nudge_holds_a_real_size_change_before_restoring ... ok
test secret::tests::oversized_value_is_refused ... ok
test serve::tests::a_nudge_under_way_is_not_restarted_and_can_be_cancelled ... ok
test secret::tests::errors_do_not_echo_the_value ... ok
test secret::tests::empty_value_is_refused ... ok
test ring::tests::keeps_everything_within_capacity ... ok
test session::tests::a_symlinked_lock_is_refused_and_its_target_left_alone ... ok
test session::tests::fifos_are_private ... ok

test result: ok. 44 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_launch-fac975cfcc56cc2e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/adversary.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/adversary-3fdd24c12f8c926d)

running 9 tests
test a_symlinked_session_dir_is_not_followed ... ok
test last_output_is_owner_only_even_when_the_file_already_exists ... ok
test session_dir_is_owner_only_even_when_it_already_exists ... ok
test the_secret_descriptor_does_not_reach_the_agent ... ok
test last_output_does_not_follow_a_planted_symlink ... ok
test exit_code_is_the_agents ... ok
test sigterm_to_serve_ends_the_agents_whole_process_group ... ok
test a_client_that_never_reads_does_not_block_the_agent ... ok
test a_flooded_ctl_pipe_does_not_stall_the_agent ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.39s

     Running tests/adversary_2.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/adversary_2-fd9198f6cf4c14e4)

running 3 tests
test a_size_aware_agent_repaints_on_a_real_resize ... ok
test attaching_at_an_unchanged_size_makes_the_agent_repaint ... ok
test a_terminal_that_lost_output_gets_a_repaint_once_it_catches_up ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.64s

     Running tests/codex_compatibility.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/codex_compatibility-cd1886455c8675a2)

running 2 tests

running 1 test
test local_socket_control_rejects_af_unix ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

test local_socket_control_rejects_af_unix ... ok
mantle-launch: received signal 15; stopping the agent
mantle-launch: agent exited (signal: 1 (SIGHUP))
test fifo_launcher_relays_without_unix_sockets_or_secret_slot ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s

     Running tests/conformance.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/conformance-0adf67a1aa5f2849)

running 1 test
test ess_launch_conformance ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 9.93s

     Running tests/launch.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/launch-1bdf17dc3932f203)

running 4 tests
test attach_without_a_server_is_refused ... ok
test secret_stays_out_of_stderr_when_the_agent_cannot_start ... ok
test second_server_is_refused_and_sigterm_stops_the_first ... ok
test attach_relays_replays_scrollback_and_survives_a_detach ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.70s

     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-0a69710eec6bc7a7)

running 28 tests
test tests::byte_limit_detects_growth_beyond_declared_size ... ok
test tests::delivery_accepts_cargo_hardlinked_artifacts ... ok
test tests::concurrent_directory_winners_are_revalidated_without_permission_changes ... ok
test tests::stale_transport_guard_cannot_retire_a_reused_pid_registration ... ok
test tests::idle_transport_coordinator_does_not_swallow_termination ... ok
test tests::termination_cleanup_includes_owned_descendants ... ok
test tests::failed_update_preserves_previous_binary_and_facts ... ok
test tests::bounded_child_pumps_input_and_output_together ... ok
test tests::adversary_second_successful_child_exit_cleans_its_remaining_group ... ok
test tests::corrupt_archive_never_activates ... ok
test tests::adversary_refuses_binary_mismatch_after_valid_archive_before_execution ... ok
test tests::adversary_fifo_and_symlink_inputs_refuse_without_waiting_for_writers ... ok
test tests::tampered_installed_bytes_are_refused ... ok
test tests::adversary_unpublished_verified_generation_recovers_after_interrupted_activation ... ok
test tests::adversary_second_failed_spawn_preserves_following_transport_and_idle_state ... ok
test tests::adversary_operator_interrupt_reaps_bounded_child ... ok
test tests::unsupported_architecture_never_fetches_or_creates_paths ... ok
test tests::symlinked_root_is_refused ... ok
test tests::adversary_unmanaged_executable_is_preserved_on_activation_refusal ... ok
test tests::termination_coordinator_cleans_concurrent_groups_and_preserves_signal_status ... ok
test tests::version_mismatch_never_activates ... ok
test tests::installation_is_verified_atomic_and_idempotent ... ok
test tests::adversary_second_update_observers_only_see_complete_verified_generations ... ok
test tests::bounded_child_refuses_output_flood_and_timeout ... ok
test tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation ... ok
test tests::generated_model_has_no_drift ... ok
test tests::retirement_survives_a_bounded_registry_wait_timeout ... ok
test tests::adversary_live_lock_refuses_without_fetching_or_changing_current ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.08s

     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-5f8b887456570a7c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mantle_conformance

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mantle_egress

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mantle_launch

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mantle_worker

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

`cargo run --locked -p mantle-conformance -- --root .`, exit0:

```text
    Finished `dev` profile [unoptimized] target(s) in 0.55s
     Running `/dev/shm/mantle-wave3-target-worker/debug/mantle-conformance --root .`
Complete ESS inventory: 313 passed; 0 failed, skipped, unsupported, outside or refused
```

Inspection of the actual aggregate report's counts, coverage and passed scenario names, compared with `spec/conformance-baseline.json` read from integration base b3a9c456, yields:

```json
{
  "counts": {
    "error": 0,
    "failed": 0,
    "passed": 313,
    "skipped": 0,
    "total": 313,
    "unsupported": 0
  },
  "coverage": {
    "authored": 218,
    "generated": 95,
    "outside": 0,
    "refused": 0
  },
  "spec_digest": "8a272638ba1e78a3a382335a397e223d44436d4b3423664c5c1c22ffe8f0f895",
  "missing_incoming_names": [],
  "incoming_required": 304,
  "current_passed": 313
}
```

The native no-op audit has an empty `authored_passes` array: all218 authored cases reject the no-op target. Its six generated acceptance/optional-return witnesses still pass and are not treated alone as proof of substantive behavior. The304 incoming required names all occur in the313 passing aggregate names; CLI213, egress66 and launcher34 retain complete component accounting. Generated model drift passes in the workspace suite. Remote installation/authentication remains outside these native scenarios.

`cargo clippy --workspace --all-targets --locked -- -D warnings`, exit0:

```text
    Checking mantle-conformance v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-conformance)
    Checking mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-launch)
    Checking mantle-worker-model v0.0.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/generated/worker-model)
    Checking mantle-egress v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-egress)
    Checking mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
    Checking mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Checking mantle-docs v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-docs)
    Finished `dev` profile [unoptimized] target(s) in 9.67s
```

`cargo fmt --check`, exit0, no stdout/stderr. `git diff --check`, exit0, no stdout/stderr.

`ess specify validate --path spec`, exit0:

```text
mantle v1 — 7 file(s), 218 scenario(s), valid
```

The coordinator owns AEP validation, site generation and live acceptance; no literal complete `task check` or real worker run is claimed here.

## 4. Findings and first-pass disposition

Nothing found.

The first-pass cancellation finding at the original lib.rs:451 is resolved by executed retained `adversary_operator_interrupt_reaps_bounded_child`, all three concurrent/idle signal cases, descendant cleanup, bounded retirement and stale-registration tests. The new successful-exit descendant and failed-spawn recovery cases also pass. Current production ownership registers each Child before releasing the spawn/registry boundary; signal cleanup and guards retire exact registration identities and retain PID ownership until reaping. Mantle initializes before its Tokio runtime; the helper initializes before installation. This does not claim cleanup after SIGKILL, kernel-uninterruptible processes or descendants that deliberately escape the owned group.

The first-pass first-directory race at original lib.rs:355 is resolved by the retained concurrent two-installer case and safe/unsafe directory-winner case in the actual suite. The code revalidates AlreadyExists winners without chmodding their objects. The explicit two-install case still runs both threads behind its barrier, checks one fetch, one Installed, one AlreadyCurrent and coherent published facts.

Carried findings0, new findings0, resolved first-pass findings2. This is the final full attack; these counts are observations for the coordinator's AEP ledger, not a replacement for its machine comparison.

## 5. Attacked boundaries and limits

- Successful direct-child exit with an otherwise surviving group descendant: stopped by actual cleanup.
- Spawn failure followed by another transport and idle-state restoration: correct literal output and empty clean ownership state observed.
- Real generation replacement with concurrent inspection: complete old/new facts only, new stable bytes, prior generation retained.
- All prior FIFO, symlink, lock, partial download, checksum, version, architecture, publication, signal, concurrency and identity assertions execute and pass.
- Native Boundary/Reply dispatch calls real worker decisions; common-only Observe and unchanged legacy selected-Claude cases execute within the complete inventory.
- Inspected fixture isolation preserves every original assertion, and each selected child uses the exact current test name. The explicit two-installer concurrency remains inside its isolated process; no production retry or weakened assertion was introduced for ETXTBSY.
- Inspected optional Claude configuration, selected readiness, shared-service deferral, immutable generation publication, bounded HTTPS/decompression/version flow and all changed callers; no further exercised defect was found. Live AW-01/AW-03 and later authentication are not proven by this pass.

## 6. Outside writes and handoff

Exact retained paths:

- `~/.cache/mantle-wave3/scratch-worker/adversary-2-invalid-fixture.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-2-cases.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-2-workspace.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-2-aggregate.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-2-clippy.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-2-fmt.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-2-ess.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-2-inventory.json`
- `~/.cache/mantle-wave3/scratch-worker/adversary-2-tests.patch`
- `~/.cache/mantle-wave3/scratch-worker/adversary-2.md`
- `/dev/shm/mantle-wave3-target-worker/` and its private temporary descendants: compiler/test output, no deletion.

Native commands regenerated existing ignored reports/suites beneath this worktree's `.engineering/drafts/`; no planning artifacts were written. The worktree CLI recorded acquisition/heartbeat/release of only `codex-wave3-worker-adversary2`. Final process inspection found no remaining compiler, test fixture or sleep child using this target. The tree remains `mantle-wave3-worker` on `impl/agent-ready-worker` at e54bd4a, with145 unstaged test-only lines. Coordinator owns recording, commits, live acceptance and cleanup.

## 7. Machine-readable findings

```findings
[]
```
