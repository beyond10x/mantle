unit:                   story:retained-workspace-lifecycle — 8df716f6c53b56f62ba0ad5ebdfa75c97f859ddd
verdict:                green
cases:                  executed 64→67 package tests; native CLI 273→372; red 0 final
origin:                 n/a
wrote-outside-worktree: $HOME/.cache/mantle-reliability/retained-workspaces{,-target,-tmp}; shared /dev/shm/mantle-reliability-compiler-cache
needs-coordinator:      no — separate adversary and integrated gate remain coordinator-owned

## 1. Unit and acceptance

Implement retained stop, explicit `destroy --yes`, fresh-process restart using immutable persisted launch settings, expected-session-ID guards, migration-preserved destructive intent, fenced concurrent attempts and honest interrupted admission/retirement recovery.

Source is committed on `unit/mantle-retained-workspaces`; both author and committer are `b10x-bot[bot] <316511680+b10x-bot[bot]@users.noreply.github.com>`. Exact base is `e5624f2a907fb59d91c9e54a6680b64c2eee852a`. The working tree was clean after commit.

Runtime changes use versioned auxiliary SQLite lifecycle metadata and short IMMEDIATE transactions. The Session record includes a read generation, so the same visible state after an intervening restart/stop cannot authorize a stale caller. Initialization owns materialization before admission. Initial and restarted agent IDs are persisted before readiness IO. Recovery validates Operation id/kind/resource and then exec/workspace identity; unknown or missing observations do not trigger reconstructed resubmission. Terminal proof and retirement identity are durable before retirement, whose absent boolean is checked. Confirmed running/terminal pending admissions can be reconciled for stop/destroy after failed readiness.

Stop retains edited files/private home and reserves the name. Restart uses the stored argv/environment/cwd/resource contract and a frozen version-1 execution policy; it does not rematerialize sources. Destroy alone deletes workspace data. Legacy STOPPING migrates once to destructive intent; legacy STOPPED stays destroyed. The current prototype refuses unresolved initial materialization rather than fabricating safe retention.

All inferred paths were inspected. Added `app/lifecycle.rs`, `adapters/state/lifecycle.rs`, `tests/retained_workspaces.rs`, and `tests/support/lifecycle_ssh.rs` after coordinator scope acknowledgment. `spec/components.yaml` and `spec/conformance-baseline.json` were likewise added to typed scope before edits. No separate `generated/session-model` was needed; existing owned worker-model provenance changed with the ESS source.

## 2. Observed diff

Command: `git --no-pager diff e5624f2a907fb59d91c9e54a6680b64c2eee852a..HEAD --stat`

```text
README.md                                          |  45 ++-
 crates/mantle/src/adapters/conformance.rs          |  11 +
 crates/mantle/src/adapters/orchestration.rs        | 427 ++++++++++++++++++++-
 crates/mantle/src/adapters/state.rs                |  35 +-
 crates/mantle/src/adapters/state/lifecycle.rs      | 223 +++++++++++
 crates/mantle/src/app/lifecycle.rs                 | 399 +++++++++++++++++++
 crates/mantle/src/app/session.rs                   | 251 ++++++++----
 crates/mantle/src/domain/session.rs                |  26 +-
 crates/mantle/src/main.rs                          |  64 ++-
 crates/mantle/tests/retained_workspaces.rs         | 171 +++++++++
 crates/mantle/tests/support/lifecycle_ssh.rs       |  60 +++
 generated/worker-model/source.schema.json          |   4 +-
 generated/worker-model/types-report.json           |   6 +-
 generated/worker-model/types.rs                    |   4 +-
 spec/components.yaml                               |   7 +-
 spec/conformance-baseline.json                     | 166 +++++++-
 spec/domains/orchestration.yaml                    |   2 +-
 spec/domains/session.yaml                          |  87 ++++-
 spec/ess-inputs.yaml                               |  30 ++
 .../cli/destroy-explicit-confirmation.yaml         |  19 +
 spec/scenarios/cli/destroy-readback.yaml           |  19 +
 spec/scenarios/cli/destroy-repeat.yaml             |  19 +
 .../cli/orchestration-stop-absent-agent.yaml       |   2 +-
 .../cli/orchestration-stop-absent-workspace.yaml   |   2 +-
 .../cli/orchestration-stop-live-agent.yaml         |   2 +-
 .../cli/orchestration-stop-no-resources.yaml       |   2 +-
 .../cli/orchestration-stop-refused-destroy.yaml    |   4 +-
 .../cli/orchestration-stop-refused-get-exec.yaml   |   4 +-
 .../orchestration-stop-refused-get-workspace.yaml  |   4 +-
 .../cli/orchestration-stop-refused-retire.yaml     |   4 +-
 .../cli/orchestration-stop-refused-signal.yaml     |   4 +-
 .../cli/orchestration-stop-refused-wait.yaml       |   4 +-
 spec/scenarios/cli/orchestration-stop-resumes.yaml |   2 +-
 .../cli/orchestration-stop-terminal-agent.yaml     |   2 +-
 ...hestration-stop-transport-unknown-readback.yaml |   2 +-
 .../orchestration-stop-unknown-readback-error.yaml |   4 +-
 .../cli/orchestration-stop-unknown-readback.yaml   |   2 +-
 .../orchestration-stop-unknown-still-present.yaml  |   4 +-
 .../orchestration-stop-upstream-destroy-error.yaml |   4 +-
 .../orchestration-stop-upstream-exec-error.yaml    |   4 +-
 ...rchestration-stop-upstream-workspace-error.yaml |   4 +-
 spec/scenarios/cli/retain-attach-refusal.yaml      |  19 +
 spec/scenarios/cli/retain-binding-refusal.yaml     |  19 +
 spec/scenarios/cli/retain-concurrent-restart.yaml  |  19 +
 .../cli/retain-destroy-versus-restart.yaml         |  19 +
 spec/scenarios/cli/retain-expected-session-id.yaml |  19 +
 .../cli/retain-failed-admission-cleanup.yaml       |  19 +
 .../cli/retain-fenced-completion-same-state.yaml   |  19 +
 spec/scenarios/cli/retain-files-auth.yaml          |  19 +
 .../cli/retain-initial-materialization.yaml        |  19 +
 spec/scenarios/cli/retain-interrupted-stop.yaml    |  19 +
 spec/scenarios/cli/retain-legacy-stopped.yaml      |  19 +
 spec/scenarios/cli/retain-migration-reopen.yaml    |  19 +
 .../cli/retain-missing-terminal-proof.yaml         |  19 +
 .../cli/retain-other-session-survives.yaml         |  19 +
 spec/scenarios/cli/retain-repeat-stop.yaml         |  19 +
 .../retain-restart-accepted-before-persist.yaml    |  19 +
 .../cli/retain-restart-before-dispatch.yaml        |  19 +
 spec/scenarios/cli/retain-restart-failure.yaml     |  19 +
 spec/scenarios/cli/retain-restart-readiness.yaml   |  19 +
 spec/scenarios/cli/retain-restart.yaml             |  19 +
 spec/scenarios/cli/retain-retirement-crash.yaml    |  19 +
 .../cli/retain-retirement-not-absent.yaml          |  19 +
 spec/scenarios/cli/retain-stale-completion.yaml    |  19 +
 spec/scenarios/cli/retain-stop-versus-restart.yaml |  19 +
 spec/scenarios/cli/retain-stop.yaml                |  19 +
 .../scenarios/cli/retain-unknown-no-duplicate.yaml |  19 +
 spec/scenarios/cli/retain-unsupported-policy.yaml  |  19 +
 website/index.html                                 |  13 +-
 69 files changed, 2490 insertions(+), 171 deletions(-)
```

Existing destructive scenario purposes and identifiers were retained under explicit orchestration.Destroy; expected pending state is now DESTROYING. The required generated Stop outcome identity was explicitly renamed to Destroy. Comparing the resulting native CLI report with all existing CLI baseline obligations found **zero missing existing obligations**. The CLI baseline now requires the 372 observed cases, including new lifecycle cases. No checks were skipped or deleted.

## 3. Red evidence

The production stop regression was authored and run before runtime changes:

```console
cargo test --locked -p mantle stop_retains_workspace_and_reserves_name
# exit 101
```

The retained baseline was then driven through the same actual CLI/OpenSSH/SDK HTTP fixture. It actually sent DELETE /v1/workspaces/ws, and the fixture removed its real filesystem workspace:

```console
MANTLE_LIFECYCLE_BASE_CLI="$HOME/.cache/mantle-reliability/retained-workspaces/baseline/mantle" \
  cargo test --locked -p mantle --test retained_workspaces
# exit 101
```

Baseline SHA256: `6385e997a17bc333c87956819a3fd876c238508fdb180dd5c2e7e1de9d205e86`. This is the coordinator-retained source-equivalent baseline from green source19c817f; it was not changed.

Verbatim decisive output excerpts:

```text
Running unittests src/main.rs ($HOME/.cache/mantle-reliability/retained-workspaces-target/debug/deps/mantle-bd9b1d4f99eee58a)

running 1 test
test adapters::state::orchestration::stop_retains_workspace_and_reserves_name ... FAILED

failures:

---- adapters::state::orchestration::stop_retains_workspace_and_reserves_name stdout ----
Workspace      destroyed
Session        fixture stopped

thread 'adapters::state::orchestration::stop_retains_workspace_and_reserves_name' (3140361) panicked at crates/mantle/src/adapters/orchestration.rs:371:5:
stop must preserve workspace contents
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adapters::state::orchestration::stop_retains_workspace_and_reserves_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 36 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p mantle --bin mantle`
running 1 test
test actual_cli_stop_retains_files_and_destroy_requires_explicit_confirmation ... FAILED

failures:

---- actual_cli_stop_retains_files_and_destroy_requires_explicit_confirmation stdout ----

thread 'actual_cli_stop_retains_files_and_destroy_requires_explicit_confirmation' (3496873) panicked at crates/mantle/tests/retained_workspaces.rs:40:5:
stop deleted the retained user workspace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    actual_cli_stop_retains_files_and_destroy_requires_explicit_confirmation

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.26s

error: test failed, to rerun pass `-p mantle --test retained_workspaces`
```

Full raw logs are `red-retain.log` and `red-baseline-process-2.log`. The first process attempt, `red-baseline-process.log`, failed because the fixture omitted the wire-required observed_at timestamp; it is preserved as a setup failure, **not** causal red evidence. The corrected second attempt reached actual workspace deletion. `red-cli.log` independently records old CLI rejection of the absent destroy verb (exit2).

The first broader native run also exposed the stale retained no-op path; it was corrected to claim/fence repeated stops. Later native coverage exercises same-state generation changes and two concurrent SQLite claimers. Those tests were added during implementation, not represented as independently test-first per-case measurements.

## 4. Green verification

All following commands ran in the assigned tree. Shared environment:

```console
export PATH="$HOME/.cache/aap-ess-upgrade/ess-0.50/bin:$PATH"
export CARGO_TARGET_DIR="$HOME/.cache/mantle-reliability/retained-workspaces-target"
export TMPDIR="$HOME/.cache/mantle-reliability/retained-workspaces-tmp"
export CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 RUST_TEST_THREADS=1
export RUSTC_WRAPPER=/usr/bin/sccache SCCACHE_SERVER_PORT=43179
export SCCACHE_DIR=/dev/shm/mantle-reliability-compiler-cache SCCACHE_CACHE_SIZE=1G
```

The final package command was `cargo test --locked -p mantle`, exit0. Full raw output: `package-final.log`. Verbatim runner output (compiler/process-path lines omitted; raw log remains exact):

```text
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
test result: ok. 38 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 76.60s
running 4 tests
test adversary_codex_never_invokes_configured_claude_credential_command ... ok
test adversary_legacy_migration_preserves_indexes_and_stopped_sources_then_refuses_corruption ... ok
test adversary_partial_identity_schema_refusal_rolls_back_initialization ... ok
test codex_reaches_worker_boundary_without_claude_credentials_or_session_insertion ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.56s
running 1 test
test actual_cli_preserves_remote_exit_status_and_output ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.54s
running 4 tests
test adversary_doctor_interruption_retires_tunnel_and_removes_owned_socket_directory ... ok
test doctor_bounds_local_state_failures_and_suppresses_external_probes ... ok
test doctor_observes_real_cli_wire_stages_and_preserves_current_wal_state ... ok
test doctor_reports_selection_and_configuration_failures_as_json_without_mutating_state ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.00s
running 6 tests
test adversary_profile_boundaries_and_corrupt_registry_fail_closed ... ok
test profile_registry_is_local_private_and_concurrent_additions_do_not_clobber ... ok
test profile_registry_rejects_unsafe_names_paths_and_descriptor_types ... ok
test profile_selection_precedence_conflicts_and_database_isolation ... ok
test profile_selection_refuses_symlinked_configuration_state_and_registry_ancestors ... ok
test selected_profiles_isolate_ssh_identity_known_hosts_and_private_sockets ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.52s
running 1 test
test actual_cli_stop_retains_files_and_destroy_requires_explicit_confirmation ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.54s
running 3 tests
test actual_apply_transports_verified_bundle_and_cleans_owned_stage ... ok
test actual_old_worker_check_observes_offline_facts_without_upload_or_state_writes ... ok
test upgrade_worker_transaction_child ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 15.60s
running 10 tests
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... 
running 1 test
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... 
running 1 test
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s
test tests::cq01_rejects_wrong_digest_before_executing_a_binary ... ok
test tests::digest_bounds_actual_input_even_when_it_exceeds_the_size_snapshot ... ok
test tests::digest_format_is_strict ... ok
test tests::digest_refuses_other_non_regular_descriptors ... ok
test tests::fixed_observation_never_exports_terminal_payload ... ok
test tests::scratch_budget_refuses_large_files_and_does_not_follow_links ... ok
test tests::terminal_observation_is_bounded_and_handles_split_queries ... ok
test tests::unsafe_scratch_parents_are_refused_without_creating_children ... ok
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.35s
```

Counts come from the runner summaries. Baseline source19c817f is the coordinator's `$HOME/.cache/mantle-reliability/integration/wave5-gate-retry.log`, with its directly recorded exit0 file `wave5-gate-retry.exit`. The original quota-failed wave5-gate log was not used as the baseline.

- Mantle unit lane: executed36→38, exit0.
- codex_preflight integration: executed4→4, exit0.
- command_correctness integration: executed1→1, exit0.
- doctor integration: executed4→4, exit0.
- profiles integration: executed6→6, exit0.
- retained_workspaces actual CLI: new lane, baseline fixture executed1 red→1 green, exit0; package base did not contain this test.
- worker_upgrade integration: executed3→3, exit0.
- codex_qualification example: executed10→10, exit0. Its two child invocations each print1 pass and are not double-counted in package total.
- Package aggregate from these summaries: executed64→67, exit0.
- Native CLI ESS report: executed273→372, exit0; 372 passed, zero failed/error/skipped/unsupported. Thirty new authored lifecycle scenarios execute production orchestration with real SQLite/filesystem boundaries, not response literals alone.
- Authored specification validation: 277→307 scenarios, exit0: `mantle v1 — 8 file(s), 307 scenario(s), valid`.
- Generated worker model drift filter: executed1→1, exit0. Other worker binaries/integration files printed zero selected cases under this explicit filter; no claim is made that this filter ran the full worker suite. The coordinator's integrated gate runs the complete workspace.

Other direct checks and logs:

```console
cargo clippy --locked -p mantle --all-targets -- -D warnings
# exit0, clippy-final.log
cargo fmt --check
# exit0
rustfmt --edition 2024 --check crates/mantle/tests/support/lifecycle_ssh.rs
# exit0
ess specify validate --path spec
# exit0, spec-final.log
cargo test --locked -p mantle-worker generated_model_has_no_drift
# exit0, drift-final.log and drift-final.exit
cargo run --locked -p mantle-docs -- check
# exit0, docs-final.log and docs-final.exit
cargo run --locked -p mantle-release -- notices --source . --check
# exit0, notices-final.log and notices-final.exit
cargo run --locked -p mantle-docs -- build --out website/build --commit 8df716f6c53b56f62ba0ad5ebdfa75c97f859ddd
# exit0, docs-build.log; site-provenance.json binds this exact commit
git diff --cached --check
# exit0 before commit
```

The first Clippy attempt found one clone_on_copy in a fixture; its log/exit101 remain in `clippy-first.log`/status ledger. No lint suppression was used for that defect. Existing store-command boundaries retained solely for ESS have documented non-test dead-code allowances; runtime completion paths use the fenced lifecycle writer.

Owned generation followed the prior-unit procedure: export exact-base spec into scratch/base, generate the same four worker roots to scratch/base-model, adopt with `--owner model-types`, regenerate normally, then run explicit drift. The generation/adoption commands all exited0; raw logs are listed below. No generated source was hand-edited. No dependency or lockfile changes occurred; notices --check passed unchanged.

`status.json` records each measured red/green command and direct exit, including intermediate failures. Native `mantle-cli-{suite,run,report}.json` are copied into scratch so cleanup cannot erase evidence. Final complete-repository conformance reconciliation and AEP state changes belong to the coordinator.

## 5. Deliberate limits and remaining ownership

No worker/VM/provider/session/auth calls were made against real resources. Synthetic private-home bytes were created only in isolated fixtures. The actual CLI fixture substitutes the OpenSSH executable and still exercises real SDK HTTP parsing. No production fixture flags, environment mutation/re-exec selection, shell/Python committed programs, new Substrate capability, real publication or release was added.

The bot commit used the authorized App route and therefore made its normal GitHub authentication/API calls; “no live operations” here refers to product workers/auth, not those authorized delivery calls. No push/PR/tag/release was performed. This source builds documentation and exact-commit provenance; it does not establish public website publication.

Missing/unqualified operation observations, unsupported launch context, changed recorded worker binding and interrupted unproven initial materialization remain actionable nonzero/incomplete outcomes. Restart is a fresh process, not same-conversation resume. No worker-loss recovery or authenticated model qualification is claimed. Legacy rows without launch context cannot invent a restart from current configuration.

The independent adversary and root full gate remain pending. No additional implementation scope is requested.

## 6. Outside paths and handoff

All persistent unit output is under these exact roots:

- `$HOME/.cache/mantle-reliability/retained-workspaces-target` — 3.0GiB measured, assigned compiler/test target; retained for reviewer.
- `$HOME/.cache/mantle-reliability/retained-workspaces-tmp` — 35MiB measured, assigned temporary files; retained.
- `$HOME/.cache/mantle-reliability/retained-workspaces` — 79MiB before copied reports, including the coordinator-owned baseline executable (unchanged).
- `$HOME/.cache/mantle-reliability/retained-workspaces/base/spec` — exact-base Git archive export.
- `$HOME/.cache/mantle-reliability/retained-workspaces/base-model` — exact-base ESS generated reference/ownership metadata.
- `/dev/shm/mantle-reliability-compiler-cache` — shared task-owned bounded sccache (1.1GiB measured including overhead). Server port43179 was not restarted; stable server TMPDIR remains `$HOME/.cache/mantle-reliability/integration-tmp`.

The scratch log/evidence inventory at report creation was:

```text
$HOME/.cache/mantle-reliability/retained-workspaces/base-model/.ess-output/state.json
$HOME/.cache/mantle-reliability/retained-workspaces/base-model/source.schema.json
$HOME/.cache/mantle-reliability/retained-workspaces/base-model/types-report.json
$HOME/.cache/mantle-reliability/retained-workspaces/base/spec/README.md
$HOME/.cache/mantle-reliability/retained-workspaces/base/spec/conformance-baseline.json
$HOME/.cache/mantle-reliability/retained-workspaces/bot-commit.log
$HOME/.cache/mantle-reliability/retained-workspaces/clippy-final.log
$HOME/.cache/mantle-reliability/retained-workspaces/clippy-first.log
$HOME/.cache/mantle-reliability/retained-workspaces/docs-build.log
$HOME/.cache/mantle-reliability/retained-workspaces/docs-final.exit
$HOME/.cache/mantle-reliability/retained-workspaces/docs-final.log
$HOME/.cache/mantle-reliability/retained-workspaces/drift-final.exit
$HOME/.cache/mantle-reliability/retained-workspaces/drift-final.log
$HOME/.cache/mantle-reliability/retained-workspaces/external-files.txt
$HOME/.cache/mantle-reliability/retained-workspaces/generate-baseline.log
$HOME/.cache/mantle-reliability/retained-workspaces/green-process.log
$HOME/.cache/mantle-reliability/retained-workspaces/mantle-cli-report.json
$HOME/.cache/mantle-reliability/retained-workspaces/mantle-cli-run.json
$HOME/.cache/mantle-reliability/retained-workspaces/mantle-cli-suite.json
$HOME/.cache/mantle-reliability/retained-workspaces/model-adopt.log
$HOME/.cache/mantle-reliability/retained-workspaces/model-generate-final.log
$HOME/.cache/mantle-reliability/retained-workspaces/model-generate.log
$HOME/.cache/mantle-reliability/retained-workspaces/notices-final.exit
$HOME/.cache/mantle-reliability/retained-workspaces/notices-final.log
$HOME/.cache/mantle-reliability/retained-workspaces/package-final.log
$HOME/.cache/mantle-reliability/retained-workspaces/package-first.log
$HOME/.cache/mantle-reliability/retained-workspaces/package-second.log
$HOME/.cache/mantle-reliability/retained-workspaces/red-baseline-process-2.log
$HOME/.cache/mantle-reliability/retained-workspaces/red-baseline-process.log
$HOME/.cache/mantle-reliability/retained-workspaces/red-cli.log
$HOME/.cache/mantle-reliability/retained-workspaces/red-retain.log
$HOME/.cache/mantle-reliability/retained-workspaces/retained-expanded.log
$HOME/.cache/mantle-reliability/retained-workspaces/retained-first.log
$HOME/.cache/mantle-reliability/retained-workspaces/retained-second.log
$HOME/.cache/mantle-reliability/retained-workspaces/site-provenance.json
$HOME/.cache/mantle-reliability/retained-workspaces/source-stat.txt
$HOME/.cache/mantle-reliability/retained-workspaces/spec-defined-green.log
$HOME/.cache/mantle-reliability/retained-workspaces/spec-defined.log
$HOME/.cache/mantle-reliability/retained-workspaces/spec-final.log
$HOME/.cache/mantle-reliability/retained-workspaces/spec-initial.log
$HOME/.cache/mantle-reliability/retained-workspaces/status.json
```

Additional handoff outputs are this `implementation-report.md`, copied `mantle-cli-report.json`, `mantle-cli-run.json`, `mantle-cli-suite.json`, and `site-provenance.json`. `external-files.txt` enumerates scratch/temp files recursively, including exact base reference files. Temporary controlled SQLite/workspace fixtures use the assigned TMPDIR and are ordinarily dropped by their Rust owners. Existing SSH code allocates its short private socket directories under the system temporary directory and removes them on Tunnel drop, as established by the earlier profile unit.

Generated ignored files inside the managed tree: ESS operational ownership metadata, `.engineering/drafts/mantle-cli-*.json`, and `website/build`. No AEP store files were edited.

All build/test commands ended before handoff. Own lease `codex-retained-workspaces` will be ended immediately after this report is written; `lease-end.log` records that command. No target/temp/worktree cleanup was performed. Coordinator owns final review, integration, publication and cleanup.

