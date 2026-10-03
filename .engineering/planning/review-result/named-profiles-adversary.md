---
format: aep.planning-md/3
id: review-result:named-profiles-adversary
kind: review-result
status: active
title: Named profiles adversary review
relations:
- reviews: story:named-profiles
revision: 1
---
unit: story:named-profiles at 93dc4cab8e594f6bae1c61d7574987d4d57dcba1 plus uncommitted adversary test
verdict: nothing found
cases: executed 54→55 top-level Rust tests, red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: 9 retained files/directories, plus existing tests' self-cleaned temporary fixtures
needs-coordinator: decide whether to retain the uncommitted tests-only addition; no implementation finding

```text
 crates/mantle/tests/profiles.rs | 78 +++++++++++++++++++++++++++++++++++++++++
 1 file changed, 78 insertions(+)
```

1. Added `crates/mantle/tests/profiles.rs:433`, `adversary_profile_boundaries_and_corrupt_registry_fail_closed`. It exercises the actual CLI: 63-character names accepted, 64-character/uppercase/non-ASCII/invalid-prefix names refused, parent traversal and control bytes refused, list/show remain local with unreadable-as-TOML provider content, public profile descriptors and nonprivate registry refused, mismatched names/unknown fields refuse without partial output or secret text. Restoring valid contents restores exact list/show output. The selected state directory remains absent. The case was written before any execution and passed on its first isolated run; there is no red output or confirmed defect.

Command: `cargo test -p mantle --test profiles --locked adversary_profile_boundaries_and_corrupt_registry_fail_closed -- --exact`
Exit: 0. Output below is verbatim except the operator home prefix is replaced with `$HOME`.

```text
   Compiling mantle v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-named-profiles/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 0.66s
     Running tests/profiles.rs (/dev/shm/mantle-named-profiles-target/debug/deps/profiles-25d6821c2f5257ee)

running 1 test
test adversary_profile_boundaries_and_corrupt_registry_fail_closed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.18s

```

2. Only after the isolated case, ran `cargo test -p mantle --locked`, exit 0. The implementor supplied the 54-test baseline and 243 native CLI scenario count. This run executed 55 top-level tests (34+4+1+6+10), including the native ESS conformance driver; it did not independently recount the driver's internal scenarios. Output is verbatim except `$HOME` substitution.

Environment for both commands: `PATH=$HOME/.cache/aap-ess-upgrade/ess-0.50/bin:$PATH`, `TMPDIR=/dev/shm/mantle-named-profiles-tmp`, `CARGO_TARGET_DIR=/dev/shm/mantle-named-profiles-target`, `CARGO_BUILD_JOBS=2`, `CARGO_PROFILE_DEV_DEBUG=0`, `RUSTC_WRAPPER=/usr/bin/sccache`, `SCCACHE_SERVER_PORT=43179`, `SCCACHE_DIR=/dev/shm/mantle-reliability-compiler-cache`, `SCCACHE_CACHE_SIZE=1G`.

```text
    Finished `test` profile [unoptimized] target(s) in 0.19s
     Running unittests src/main.rs (/dev/shm/mantle-named-profiles-target/debug/deps/mantle-7e1143531ce7acdb)

running 34 tests
test adapters::substrate::worker_tests::common_readiness_and_selected_claude_are_distinct_observations ... ok
test app::terminal::tests::ctrl_bracket_d_detaches_and_forwards_what_came_before ... ok
test app::terminal::tests::escape_followed_by_anything_else_passes_both_bytes ... ok
test app::terminal::tests::readiness_marker_is_consumed_across_every_split ... ok
test config::tests::common_worker_configuration_needs_no_claude_credentials ... ok
test config::tests::legacy_claude_table_remains_parseable ... ok
test config::tests::explicit_path_selection_is_absolute_and_preserves_defaults ... ok
test app::worker::tests::legacy_claude_bootstrap_retains_required_slot ... ok
test domain::manifest::tests::cwd_must_be_inside_a_mount ... ok
test domain::manifest::tests::cwd_with_dot_dot_or_empty_components_is_refused ... ok
test adapters::state::tests::sources_round_trip ... ok
test adapters::state::tests::a_live_name_is_unique_until_stopped ... ok
test app::worker::tests::the_template_renders_every_placeholder ... ok
test adapters::state::tests::an_illegal_move_is_refused_and_not_written ... ok
test domain::manifest::tests::defaults_apply ... ok
test domain::manifest::tests::sizes_and_durations_parse ... ok
test app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command ... ok
test app::terminal::tests::the_sequence_is_recognised_across_reads ... ok
test domain::manifest::tests::unserved_fields_are_refused_by_name ... ok
test domain::manifest::tests::mounts_and_refs_cannot_escape ... ok
test domain::session::tests::running_is_never_reached_without_starting ... ok
test domain::session::tests::stopped_is_final_and_failures_only_stop ... ok
test domain::manifest::tests::retain_for_is_bounded_by_the_exec_limit ... ok
test domain::session::tests::names_round_trip ... ok
test domain::manifest::tests::the_example_manifest_resolves ... ok
test domain::session::tests::the_happy_path_is_legal ... ok
test app::session::adversary_activation_wire::adversary_codex_dispatches_confined_secretless_command_and_pty ... ok
test app::session::codex_transport_tests::codex_start_and_attach_construct_real_sdk_builders ... ok
test adapters::ssh::tests::known_hosts_option_preserves_one_literal_path_in_openssh_parser ... ok
test adapters::ssh::tests::tunnel_socket_binds_with_live_length_state_prefix ... ok
test adapters::ssh::tests::socket_allocations_are_private_unique_and_state_path_independent ... ok
test adapters::ssh::tests::tunnel_socket_directory_lives_until_child_cleanup_and_is_removed_on_errors ... ok
test adapters::state::orchestration::real_git_resolves_branches_tags_and_commits ... ok
test adapters::state::conformance::ess_generated_local_conformance ... ok

test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.63s

     Running tests/codex_preflight.rs (/dev/shm/mantle-named-profiles-target/debug/deps/codex_preflight-89a2f68a317ae895)

running 4 tests
test codex_reaches_worker_boundary_without_claude_credentials_or_session_insertion ... ok
test adversary_codex_never_invokes_configured_claude_credential_command ... ok
test adversary_partial_identity_schema_refusal_rolls_back_initialization ... ok
test adversary_legacy_migration_preserves_indexes_and_stopped_sources_then_refuses_corruption ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

     Running tests/command_correctness.rs (/dev/shm/mantle-named-profiles-target/debug/deps/command_correctness-267b6b7ce303e249)

running 1 test
test actual_cli_preserves_remote_exit_status_and_output ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.51s

     Running tests/profiles.rs (/dev/shm/mantle-named-profiles-target/debug/deps/profiles-25d6821c2f5257ee)

running 6 tests
test profile_registry_is_local_private_and_concurrent_additions_do_not_clobber ... ok
test profile_selection_precedence_conflicts_and_database_isolation ... ok
test profile_selection_refuses_symlinked_configuration_state_and_registry_ancestors ... ok
test adversary_profile_boundaries_and_corrupt_registry_fail_closed ... ok
test profile_registry_rejects_unsafe_names_paths_and_descriptor_types ... ok
test selected_profiles_isolate_ssh_identity_known_hosts_and_private_sockets ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.41s

     Running unittests examples/codex_qualification.rs (/dev/shm/mantle-named-profiles-target/debug/examples/codex_qualification-7ab3e88cb70c4116)

running 10 tests
test tests::digest_format_is_strict ... ok
test tests::digest_bounds_actual_input_even_when_it_exceeds_the_size_snapshot ... ok
test tests::digest_refuses_other_non_regular_descriptors ... ok
test tests::fixed_observation_never_exports_terminal_payload ... ok
test tests::unsafe_scratch_parents_are_refused_without_creating_children ... ok
test tests::scratch_budget_refuses_large_files_and_does_not_follow_links ... ok

running 1 test

running 1 test
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... 
oktest result: 
ok
. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered outtest result: ok; finished in 0.00s. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out

; finished in 0.00s

test tests::terminal_observation_is_bounded_and_handles_split_queries ... ok
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok
test tests::cq01_rejects_wrong_digest_before_executing_a_binary ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.33s

```

3. No judgement findings. This is an adversary report, not approval or an independent-verifier claim.

4. Attacked registry boundaries, malformed descriptors, ownership/privacy guards and path traversal without finding a failure. Existing suite retained coverage of explicit/environment precedence, conflicting legacy overrides, SQLite isolation, symlink refusal, concurrent no-clobber registration and actual SSH argument/path isolation. Read all unit changes, acceptance and production call paths. No implementation, AEP, live-worker or integration writes.

5. Outside paths written, using portable `$HOME` anchors: 
- `$HOME/.cache/mantle-reliability/named-profiles/adversary/case.log`
- `$HOME/.cache/mantle-reliability/named-profiles/adversary/suite.log`
- `$HOME/.cache/mantle-reliability/named-profiles/adversary/review.md`
- `/dev/shm/mantle-named-profiles-target`
- `/dev/shm/mantle-named-profiles-tmp`
- `/dev/shm/mantle-reliability-compiler-cache`
- `$HOME/.local/state/worktree/registry.sqlite3`
- `$HOME/.local/state/worktree/registry.sqlite3-wal`
- `$HOME/.local/state/worktree/registry.sqlite3-shm`

Temporary Rust test fixtures were created under the declared TMPDIR and removed by their RAII guards. Existing SSH tests additionally allocate and remove their private `/tmp/mantle-*` directories; their random names are not retained. Own lease `codex-named-profiles-adversary` ended. Build target, source tree and logs remain for coordinator handoff. The 78-line test addition remains uncommitted; `git diff --check` passed.

```findings
[]
```
