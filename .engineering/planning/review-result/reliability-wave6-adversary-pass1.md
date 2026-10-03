---
format: aep.planning-md/3
id: review-result:reliability-wave6-adversary-pass1
kind: review-result
status: active
title: Retained lifecycle admission recovery and concurrent CLI attack
relations:
- reviews: story:retained-workspace-lifecycle
revision: 1
---
unit: story:retained-workspace-lifecycle at 8df716f6c53b56f62ba0ad5ebdfa75c97f859ddd
verdict: nothing found
cases: executed 67→69 Mantle package cases; native CLI 372→372; red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: 8 retained scratch files, assigned target/tmp/cache/lease state and existing transient runtime paths listed below
needs-coordinator: retain two uncommitted cases and their test-only response barrier, then record this pass

```text
$ git --no-pager diff --stat
 crates/mantle/tests/retained_workspaces.rs   | 239 +++++++++++++++++++++++++++
 crates/mantle/tests/support/lifecycle_ssh.rs |  11 ++
 2 files changed, 250 insertions(+)
```

This is formal adversary pass 1 against candidate `8df716f6c53b56f62ba0ad5ebdfa75c97f859ddd`,
base `e5624f2a907fb59d91c9e54a6680b64c2eee852a`. Both changed paths are tests. The additions
append two actual CLI cases and shared synthetic setup, plus an optional bounded response barrier
in the existing test-owned OpenSSH double. Existing case assertions are unchanged. No runtime,
specification, generated source, documentation, AEP or manifest file was edited. HEAD remains
the candidate above; no commit or external publication occurred.

1. Cases written before execution

I read the persisted adversary/implementation/invariant briefs, accepted story, implementation
report/status, changed lifecycle/state/CLI/caller paths and their tests/contracts before selecting
the attacks. The baseline 67 package cases and 372 native CLI cases come from the implementor's
handoff; no initial suite was run to establish them.

Both new cases invoke the candidate `CARGO_BIN_EXE_mantle`, real SQLite stores, separate CLI
processes, the real pinned SDK HTTP parsing and real temporary workspace files. They replace the
OpenSSH endpoint with the existing Rust test double, not product behavior.

- `crates/mantle/tests/retained_workspaces.rs:299`,
  `adversary_cli_recovers_recorded_restart_without_second_admission`: start a retained session
  using its persisted version-1 context, fail a later readiness observation after the new exec
  identity is persisted, and invoke restart in a fresh CLI process. The test requires RESTARTING
  plus the admitted ID after failure, the original operation ID in the first POST, operation
  lookup on retry, only GET requests throughout that retry, then RUNNING with the same exec.
  It also checks user/private-home bytes remain unchanged.
- `crates/mantle/tests/retained_workspaces.rs:358`,
  `adversary_delayed_stop_cannot_signal_after_another_stop_and_restart`: hold one CLI stop's
  exec-observation response, complete another stop, then restart a new agent. Release the older
  response and require the stale caller to fail with the lifecycle fence, make no subsequent
  remote request, and leave the new RUNNING exec and workspace bytes unchanged. This exercises
  an actual delayed process response after another caller changes the durable generation.

The response barrier is enabled only by fixture files in the tests' private temporary directory;
it has a 15-second deadline. CLI processes have the existing 20-second bounded process owner, and
the controlled SSH helper retains its own 30-second lifetime bound. The explicit concurrent case
uses scoped threads; unrelated cases run serially.

Both cases passed on their FIRST isolated execution. There is no failing case or red finding.

All Cargo commands ran in
`$HOME/.local/state/worktree/trees/b10x/mantle/mantle-retained-workspaces`, using:

```text
PATH=$HOME/.cache/aap-ess-upgrade/ess-0.50/bin:$PATH
TMPDIR=$HOME/.cache/mantle-reliability/retained-workspaces-tmp
CARGO_TARGET_DIR=$HOME/.cache/mantle-reliability/retained-workspaces-target
CARGO_BUILD_JOBS=2
CARGO_PROFILE_DEV_DEBUG=0
RUST_TEST_THREADS=1
RUSTC_WRAPPER=/usr/bin/sccache
SCCACHE_SERVER_PORT=43179
SCCACHE_DIR=/dev/shm/mantle-reliability-compiler-cache
SCCACHE_CACHE_SIZE=1G
SCCACHE_IDLE_TIMEOUT=0
```

The existing task sccache server was not restarted; its stable server TMPDIR remains the
coordinator-owned integration temporary root. The baseline CLI override was never set. The package
run explicitly used `env -u MANTLE_LIFECYCLE_BASE_CLI`.

First isolated execution, direct exit 0:

```text
cargo test --locked -p mantle --test retained_workspaces adversary_cli_recovers_recorded_restart_without_second_admission -- --exact
   Compiling mantle v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-retained-workspaces/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 0.85s
     Running tests/retained_workspaces.rs ($HOME/.cache/mantle-reliability/retained-workspaces-target/debug/deps/retained_workspaces-35167533a9ff5c6c)

running 1 test
test adversary_cli_recovers_recorded_restart_without_second_admission ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 6.56s

```

Second isolated execution, direct exit 0:

```text
cargo test --locked -p mantle --test retained_workspaces adversary_delayed_stop_cannot_signal_after_another_stop_and_restart -- --exact
    Finished `test` profile [unoptimized] target(s) in 0.21s
     Running tests/retained_workspaces.rs ($HOME/.cache/mantle-reliability/retained-workspaces-target/debug/deps/retained_workspaces-35167533a9ff5c6c)

running 1 test
test adversary_delayed_stop_cannot_signal_after_another_stop_and_restart ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 3.57s

```

All output blocks preserve captured output except substituting the personal absolute home prefix
with `$HOME`. Raw original logs remain in the private scratch paths below.

2. Suite after the isolated cases

```text
env -u MANTLE_LIFECYCLE_BASE_CLI [environment above] cargo test --locked -p mantle
exit: 0
    Finished `test` profile [unoptimized] target(s) in 0.19s
     Running unittests src/main.rs ($HOME/.cache/mantle-reliability/retained-workspaces-target/debug/deps/mantle-bd9b1d4f99eee58a)

running 38 tests
test adapters::ssh::tests::known_hosts_option_preserves_one_literal_path_in_openssh_parser ... ok
test adapters::ssh::tests::proxy_commands_preserve_literals_through_real_openssh_and_shell_parsing ... ok
test adapters::ssh::tests::socket_allocations_are_private_unique_and_state_path_independent ... ok
test adapters::ssh::tests::tunnel_socket_binds_with_live_length_state_prefix ... ok
test adapters::ssh::tests::tunnel_socket_directory_lives_until_child_cleanup_and_is_removed_on_errors ... ok
test adapters::state::conformance::ess_generated_local_conformance ... ok
test adapters::state::orchestration::real_git_resolves_branches_tags_and_commits ... ok
test adapters::state::orchestration::retained_lifecycle_reconciles_durable_intents_and_preserves_files ... ok
test adapters::state::orchestration::stop_retains_workspace_and_reserves_name ... ok
test adapters::state::tests::a_live_name_is_unique_until_stopped ... ok
test adapters::state::tests::an_illegal_move_is_refused_and_not_written ... ok
test adapters::state::tests::readonly_diagnostics_observe_wal_and_refuse_writes_without_migrating_legacy_schema ... ok
test adapters::state::tests::sources_round_trip ... ok
test adapters::substrate::worker_tests::common_readiness_and_selected_claude_are_distinct_observations ... ok
test app::session::adversary_activation_wire::adversary_codex_dispatches_confined_secretless_command_and_pty ... ok
test app::session::codex_transport_tests::codex_start_and_attach_construct_real_sdk_builders ... ok
test app::terminal::tests::ctrl_bracket_d_detaches_and_forwards_what_came_before ... ok
test app::terminal::tests::escape_followed_by_anything_else_passes_both_bytes ... ok
test app::terminal::tests::readiness_marker_is_consumed_across_every_split ... ok
test app::terminal::tests::the_sequence_is_recognised_across_reads ... ok
test app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command ... ok
test app::worker::tests::legacy_claude_bootstrap_retains_required_slot ... ok
test app::worker::tests::the_template_renders_every_placeholder ... ok
test config::tests::common_worker_configuration_needs_no_claude_credentials ... ok
test config::tests::explicit_path_selection_is_absolute_and_preserves_defaults ... ok
test config::tests::legacy_claude_table_remains_parseable ... ok
test domain::manifest::tests::cwd_must_be_inside_a_mount ... ok
test domain::manifest::tests::cwd_with_dot_dot_or_empty_components_is_refused ... ok
test domain::manifest::tests::defaults_apply ... ok
test domain::manifest::tests::mounts_and_refs_cannot_escape ... ok
test domain::manifest::tests::retain_for_is_bounded_by_the_exec_limit ... ok
test domain::manifest::tests::sizes_and_durations_parse ... ok
test domain::manifest::tests::the_example_manifest_resolves ... ok
test domain::manifest::tests::unserved_fields_are_refused_by_name ... ok
test domain::session::tests::names_round_trip ... ok
test domain::session::tests::running_is_never_reached_without_starting ... ok
test domain::session::tests::stopped_is_final_and_failures_only_stop ... ok
test domain::session::tests::the_happy_path_is_legal ... ok

test result: ok. 38 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 72.25s

     Running tests/codex_preflight.rs ($HOME/.cache/mantle-reliability/retained-workspaces-target/debug/deps/codex_preflight-89205aaa10f8d31a)

running 4 tests
test adversary_codex_never_invokes_configured_claude_credential_command ... ok
test adversary_legacy_migration_preserves_indexes_and_stopped_sources_then_refuses_corruption ... ok
test adversary_partial_identity_schema_refusal_rolls_back_initialization ... ok
test codex_reaches_worker_boundary_without_claude_credentials_or_session_insertion ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.35s

     Running tests/command_correctness.rs ($HOME/.cache/mantle-reliability/retained-workspaces-target/debug/deps/command_correctness-923b3c3467c2be64)

running 1 test
test actual_cli_preserves_remote_exit_status_and_output ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.65s

     Running tests/doctor.rs ($HOME/.cache/mantle-reliability/retained-workspaces-target/debug/deps/doctor-ea3c79540536258d)

running 4 tests
test adversary_doctor_interruption_retires_tunnel_and_removes_owned_socket_directory ... ok
test doctor_bounds_local_state_failures_and_suppresses_external_probes ... ok
test doctor_observes_real_cli_wire_stages_and_preserves_current_wal_state ... ok
test doctor_reports_selection_and_configuration_failures_as_json_without_mutating_state ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.09s

     Running tests/profiles.rs ($HOME/.cache/mantle-reliability/retained-workspaces-target/debug/deps/profiles-4ed62c0afc1b1662)

running 6 tests
test adversary_profile_boundaries_and_corrupt_registry_fail_closed ... ok
test profile_registry_is_local_private_and_concurrent_additions_do_not_clobber ... ok
test profile_registry_rejects_unsafe_names_paths_and_descriptor_types ... ok
test profile_selection_precedence_conflicts_and_database_isolation ... ok
test profile_selection_refuses_symlinked_configuration_state_and_registry_ancestors ... ok
test selected_profiles_isolate_ssh_identity_known_hosts_and_private_sockets ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.36s

     Running tests/retained_workspaces.rs ($HOME/.cache/mantle-reliability/retained-workspaces-target/debug/deps/retained_workspaces-35167533a9ff5c6c)

running 3 tests
test actual_cli_stop_retains_files_and_destroy_requires_explicit_confirmation ... ok
test adversary_cli_recovers_recorded_restart_without_second_admission ... ok
test adversary_delayed_stop_cannot_signal_after_another_stop_and_restart ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.46s

     Running tests/worker_upgrade.rs ($HOME/.cache/mantle-reliability/retained-workspaces-target/debug/deps/worker_upgrade-b16eed5734786258)

running 3 tests
test actual_apply_transports_verified_bundle_and_cleans_owned_stage ... ok
test actual_old_worker_check_observes_offline_facts_without_upload_or_state_writes ... ok
test upgrade_worker_transaction_child ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 15.52s

     Running unittests examples/codex_qualification.rs ($HOME/.cache/mantle-reliability/retained-workspaces-target/debug/examples/codex_qualification-47889d71f401503f)

running 10 tests
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... 
running 1 test
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

ok
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... 
running 1 test
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

ok
test tests::cq01_rejects_wrong_digest_before_executing_a_binary ... ok
test tests::digest_bounds_actual_input_even_when_it_exceeds_the_size_snapshot ... ok
test tests::digest_format_is_strict ... ok
test tests::digest_refuses_other_non_regular_descriptors ... ok
test tests::fixed_observation_never_exports_terminal_payload ... ok
test tests::scratch_budget_refuses_large_files_and_does_not_follow_links ... ok
test tests::terminal_observation_is_bounded_and_handles_split_queries ... ok
test tests::unsafe_scratch_parents_are_refused_without_creating_children ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.37s

```

Package count is 38 unit + 4 Codex preflight + 1 command correctness + 4 doctor + 6 profiles +
3 retained-workspace CLI + 3 worker upgrade + 10 qualification-example cases = 69.
The qualification example prints two one-case child summaries; these are not counted twice.

The native report copied from this package run records:

```json
{
  "conformance_status": "passed",
  "execution_status": "passed",
  "counts": {
    "error": 0,
    "failed": 0,
    "passed": 372,
    "skipped": 0,
    "total": 372,
    "unsupported": 0
  }
}
```

The three CLI cases consist of the unchanged original plus both additions. Native lifecycle,
migration/reopen, initial-materialization ownership, expected-identity guards and completion-fence
cases were rerun through the existing suite; those are existing cases rather than additional cases
authored by this pass.

Focused lint check, direct exit 0:

```text
cargo clippy --locked -p mantle --test retained_workspaces -- -D warnings
    Checking mantle v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-retained-workspaces/crates/mantle)
    Finished `dev` profile [unoptimized] target(s) in 0.68s
```

`rustfmt --check --edition 2024 crates/mantle/tests/retained_workspaces.rs crates/mantle/tests/support/lifecycle_ssh.rs`
and `git diff --check` produced no errors/output. No lint suppression, implementation adjustment,
test weakening or retry was needed.

3. Findings

Nothing found. No source-inspection suspicion is promoted to a finding, and no previous unit's
finding is carried into this lifecycle review.

4. Attacks and their limits

- A failed readiness response after actual wire admission persists the operation/exec and recovers
  without a second start. The controlled failure is a malformed observation, not a claim that a
  real daemon returned malformed JSON.
- A delayed older stop response cannot authorize work after a stop/restart generation change.
  A terminal observation for the winning stop models an agent that ended before that caller
  observed it; no different session or execution identity is substituted.
- Both cases preserve synthetic user edits and private-home markers, with a fresh CLI process
  reopening durable state at each operation.

The new launch context is a synthetic valid version-1 fixture with explicit argv/environment,
not a copied real authentication context. These tests exercise lifecycle/admission boundaries;
they do not execute Codex or establish authenticated behavior, workspace durability after worker
loss, real systemd state or provider availability. No live workers, sessions, services, credentials
or product integrations were touched. This report is an agent adversary result, not approval or
an independent-verifier claim.

5. External paths, lease and handoff

Eight retained files were written beneath the assigned scratch directory:

- `$HOME/.cache/mantle-reliability/retained-workspaces/adversary/pass1/recovery-case.log`
- `$HOME/.cache/mantle-reliability/retained-workspaces/adversary/pass1/concurrent-case.log`
- `$HOME/.cache/mantle-reliability/retained-workspaces/adversary/pass1/package.log`
- `$HOME/.cache/mantle-reliability/retained-workspaces/adversary/pass1/clippy.log`
- `$HOME/.cache/mantle-reliability/retained-workspaces/adversary/pass1/cases.patch`
- `$HOME/.cache/mantle-reliability/retained-workspaces/adversary/pass1/mantle-cli-report.json`
- `$HOME/.cache/mantle-reliability/retained-workspaces/adversary/pass1/lease-end.log`
- `$HOME/.cache/mantle-reliability/retained-workspaces/adversary/pass1/review.md` (this report)

Other written paths are:

- `$HOME/.cache/mantle-reliability/retained-workspaces-target`: assigned compiler/test target,
  3.0 GiB observed.
- `$HOME/.cache/mantle-reliability/retained-workspaces-tmp`: assigned temporary root for Rust
  fixture databases/files and compiler temporaries, 48 MiB observed.
- `/dev/shm/mantle-reliability-compiler-cache`: existing task-owned shared bounded sccache,
  1.1 GiB observed; no server restart or unrelated cache change.
- `$HOME/.local/state/worktree/registry.sqlite3`: CLI-owned own-lease/heartbeat updates.
- Existing production/test SSH socket helpers create transient randomized directories matching
  `/tmp/mantle-*`, `/tmp/mantle-bind-*` and `/tmp/mantle-path-test-*`; their Rust owners
  ordinarily remove them on drop, and the package includes its existing tunnel-cleanup tests.
  Exact randomized names were not captured. No reviewer scratch was placed under /tmp.

Inside the managed tree, the existing conformance runner refreshed its ignored operational outputs
`.engineering/drafts/mantle-cli-suite.json`, `mantle-cli-run.json`, and
`mantle-cli-report.json`. These are test outputs, not planning-store or generated source edits.
No managed tree, compiler target, previous evidence or unrelated output was deleted. The final
global storage observation was 15 GiB free on root and 31 GiB on tmpfs; no quota/allocation error
occurred.

All commands have exited. Only my lease `codex-retained-workspaces-adversary` was ended:

```text
session-end $HOME/.local/state/worktree/trees/b10x/mantle/mantle-retained-workspaces
```

The assigned tree remains on `unit/mantle-retained-workspaces`, with the two test paths uncommitted
and no other tracked change. The coordinator owns committing desired tests, integration and
eventual managed cleanup. No second attack, push, AEP write or release was performed.

```findings
[]
```

