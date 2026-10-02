---
format: aep.planning-md/3
id: review-result:codex-worker-adversary-1
kind: review-result
status: active
title: 'Worker adversary attack 1: cancellation and concurrent initialization'
relations:
- reviews: story:agent-ready-worker
revision: 1
---
unit: story:agent-ready-worker; staged working tree on 7f187c9f7d14cf8962c45171718af695f7b69f5f, implementation patch SHA256 5950cc2b18602be4765305e4f4e52ef4301221ad859abed596c7264155517d83
verdict: CONFIRMED
cases: executed 48→55, red 2
origin: introduced 2 / pre-existing 0 / undecided 0
wrote-outside-worktree: 11 paths (10 retained files and one disposable build directory)
needs-coordinator: route cancellation blocker and concurrent-initialization warning; retain both cases; live AW acceptance remains separate

## 1. Test-only diff

`git --no-pager diff --stat`:

```text
 crates/mantle-worker/src/lib.rs | 191 ++++++++++++++++++++++++++++++++++++++++
 1 file changed, 191 insertions(+)
```

The activated brief explicitly authorizes test-only additions inside this mixed Rust source file. The only hunk is `@@ -559,6 +559,197 @@`, wholly inside the existing `#[cfg(test)] mod tests` beginning at line558. Production bytes and staged implementation are unchanged. The staged patch hash still equals the pre-attack snapshot above. `adversary-1-tests.patch` preserves the complete unstaged diff. No test was deleted, skipped, weakened or staged. No AEP command, commit, live worker action, credential read or production edit occurred.

## 2. Added cases before the package suite

Before-count48 comes from the implementor's handoff (26 Mantle +10 qualification +12 helper), not a pre-attack suite run. Seven new helper tests produce55 top-level executions. Two distinct cases were observed red; the final package suite observed one red because the directory-creation race is scheduling-dependent. Nested child harnesses are not counted twice.

All added cases are in `crates/mantle-worker/src/lib.rs:563–751`:

- `adversary_concurrent_installation_fetches_once_and_publishes_one_generation`: two first installers begin together and must serialize into one Installed and one AlreadyCurrent with one fetch and coherent current facts. Red in its initial selected attack run and its later run alone; green in the final suite. The red is a real EEXIST race, not a deterministic assertion that every interleaving fails.
- `adversary_live_lock_refuses_without_fetching_or_changing_current`: holding the actual installer lock yields its bounded explicit contention refusal and preserves current facts. Green.
- `adversary_fifo_and_symlink_inputs_refuse_without_waiting_for_writers`: real FIFOs and symlinks at delivery/manifest boundaries refuse in under2seconds without a writer. Green.
- `adversary_unpublished_verified_generation_recovers_after_interrupted_activation`: an already verified generation with its current pointer absent is recovered and publishes coherent facts on retry. Green.
- `adversary_unmanaged_executable_is_preserved_on_activation_refusal`: an operator-owned executable is preserved byte-for-byte; current remains absent. Green.
- `adversary_refuses_binary_mismatch_after_valid_archive_before_execution`: an archive with a valid compressed digest but wrong expected executable digest refuses before execution/activation. Green.
- `adversary_operator_interrupt_reaps_bounded_child`: a Rust child harness runs the real bounded transport with sleep as its non-network child; SIGINT goes to the foreground driver group, as a terminal does. Its owned child remains non-zombie/running after the caller exits. Red alone and in the suite. The case kills the exact fixture child group before asserting; no live worker process is involved.

The first attempt to compile the six initial cases had a Rust scoped-closure lifetime typo (E0597), corrected only in the new test code. Its log is retained as `adversary-1-cases-compile-error.log`; that compiler failure is not a defect finding or a test execution. The first actual execution selected only those six new cases, before any package suite:

`cargo test --locked -p mantle-worker adversary_ -- --nocapture`, exit101:

```text
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
    Finished `test` profile [unoptimized] target(s) in 0.62s
     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-e379049379d396fa)

running 6 tests

thread '<unnamed>' (4096757) panicked at crates/mantle-worker/src/lib.rs:575:20:
called `Result::unwrap()` on an `Err` value: File exists (os error 17)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test tests::adversary_refuses_binary_mismatch_after_valid_archive_before_execution ... ok
test tests::adversary_fifo_and_symlink_inputs_refuse_without_waiting_for_writers ... ok
test tests::adversary_unmanaged_executable_is_preserved_on_activation_refusal ... ok
test tests::adversary_unpublished_verified_generation_recovers_after_interrupted_activation ... ok

thread 'tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation' (4096744) panicked at crates/mantle-worker/src/lib.rs:579:51:
called `Result::unwrap()` on an `Err` value: Any { .. }
test tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation ... FAILED
test tests::adversary_live_lock_refuses_without_fetching_or_changing_current ... ok

failures:

failures:
    tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation

test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 12 filtered out; finished in 5.03s

error: test failed, to rerun pass `-p mantle-worker --lib`
```

After writing the cancellation case, it ran alone before the suite. `cargo test --locked -p mantle-worker adversary_operator_interrupt_reaps_bounded_child -- --nocapture`, exit101:

```text
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
    Finished `test` profile [unoptimized] target(s) in 0.64s
     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-e379049379d396fa)

running 1 test

thread 'tests::adversary_operator_interrupt_reaps_bounded_child' (4110232) panicked at crates/mantle-worker/src/lib.rs:695:9:
operator SIGINT left the owned bounded child running after its parent exited
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test tests::adversary_operator_interrupt_reaps_bounded_child ... FAILED

failures:

failures:
    tests::adversary_operator_interrupt_reaps_bounded_child

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.11s

error: test failed, to rerun pass `-p mantle-worker --lib`
```

The concurrent case also ran alone before the suite. `cargo test --locked -p mantle-worker adversary_concurrent_installation_fetches_once_and_publishes_one_generation -- --nocapture`, exit101:

```text
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
    Finished `test` profile [unoptimized] target(s) in 0.60s
     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-e379049379d396fa)

running 1 test

thread '<unnamed>' (4115397) panicked at crates/mantle-worker/src/lib.rs:577:22:
called `Result::unwrap()` on an `Err` value: File exists (os error 17)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation' (4115395) panicked at crates/mantle-worker/src/lib.rs:581:27:
called `Result::unwrap()` on an `Err` value: Any { .. }
test tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation ... FAILED

failures:

failures:
    tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.09s

error: test failed, to rerun pass `-p mantle-worker --lib`
```

## 3. Package gate after all cases existed

Environment: `RUSTC_WRAPPER=`, `CARGO_TARGET_DIR=/dev/shm/mantle-wave3-target-worker`, `CARGO_BUILD_JOBS=2`, `CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0`, `CARGO_INCREMENTAL=0`, `TMPDIR=/dev/shm/mantle-wave3-target-worker/tmp`. Root had29GiB available and tmpfs28GiB before compilation, above the assigned floors.

`cargo test --locked -p mantle -p mantle-worker --all-targets`, exit101, actual executed55 =26+10+19;54passed,1failed,0ignored. The helper CLI's zero-test harness was not reached after the failing helper library lane. Local ESS scenarios65passed,0failed,0skipped at model digest `e4ed1d26c2e578d0af56d2aa8813a50292167069d9a6ba58b0bac6e649148d8a`; remote effects are excluded.

```text
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 4.78s
     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle-ab69b5a819f4e749)

running 26 tests
test app::terminal::tests::ctrl_bracket_d_detaches_and_forwards_what_came_before ... ok
test app::terminal::tests::escape_followed_by_anything_else_passes_both_bytes ... ok
test app::terminal::tests::the_sequence_is_recognised_across_reads ... ok
test adapters::substrate::worker_tests::common_readiness_and_selected_claude_are_distinct_observations ... ok
test config::tests::common_worker_configuration_needs_no_claude_credentials ... ok
test config::tests::explicit_path_selection_is_absolute_and_preserves_defaults ... ok
test domain::manifest::tests::sizes_and_durations_parse ... ok
test config::tests::legacy_claude_table_remains_parseable ... ok
test domain::session::tests::names_round_trip ... ok
test domain::manifest::tests::defaults_apply ... ok
test app::worker::tests::legacy_claude_bootstrap_retains_required_slot ... ok
test domain::session::tests::running_is_never_reached_without_starting ... ok
test domain::session::tests::stopped_is_final_and_failures_only_stop ... ok
test domain::session::tests::the_happy_path_is_legal ... ok
test domain::manifest::tests::cwd_must_be_inside_a_mount ... ok
test app::worker::tests::the_template_renders_every_placeholder ... ok
test domain::manifest::tests::retain_for_is_bounded_by_the_exec_limit ... ok
test domain::manifest::tests::the_example_manifest_resolves ... ok
test domain::manifest::tests::unserved_fields_are_refused_by_name ... ok
test domain::manifest::tests::mounts_and_refs_cannot_escape ... ok
test domain::manifest::tests::cwd_with_dot_dot_or_empty_components_is_refused ... ok
test adapters::state::tests::an_illegal_move_is_refused_and_not_written ... ok
test adapters::state::tests::sources_round_trip ... ok
test adapters::state::tests::a_live_name_is_unique_until_stopped ... ok
test app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command ... ok
test adapters::state::conformance::ess_generated_local_conformance ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

     Running unittests examples/codex_qualification.rs (/dev/shm/mantle-wave3-target-worker/debug/examples/codex_qualification-acd4056f7e1b59e7)

running 10 tests
test tests::digest_format_is_strict ... ok
test tests::digest_bounds_actual_input_even_when_it_exceeds_the_size_snapshot ... ok
test tests::digest_refuses_other_non_regular_descriptors ... ok
test tests::fixed_observation_never_exports_terminal_payload ... ok
test tests::unsafe_scratch_parents_are_refused_without_creating_children ... ok
test tests::scratch_budget_refuses_large_files_and_does_not_follow_links ... ok

running 1 test

running 1 test
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... test adversary::binary_symlink_is_validated_against_its_current_opened_object ... okok


test result: 
oktest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s; finished in 0.00s



test tests::terminal_observation_is_bounded_and_handles_split_queries ... ok
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok
test tests::cq01_rejects_wrong_digest_before_executing_a_binary ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.33s

     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-c4999a0e43038d4b)

running 19 tests
test tests::byte_limit_detects_growth_beyond_declared_size ... ok
test tests::delivery_accepts_cargo_hardlinked_artifacts ... ok
test tests::corrupt_archive_never_activates ... ok
test tests::symlinked_root_is_refused ... ok
test tests::unsupported_architecture_never_fetches_or_creates_paths ... ok
test tests::adversary_refuses_binary_mismatch_after_valid_archive_before_execution ... ok
test tests::adversary_fifo_and_symlink_inputs_refuse_without_waiting_for_writers ... ok
test tests::version_mismatch_never_activates ... ok
test tests::tampered_installed_bytes_are_refused ... ok
test tests::adversary_unmanaged_executable_is_preserved_on_activation_refusal ... ok
test tests::installation_is_verified_atomic_and_idempotent ... ok
test tests::generated_model_has_no_drift ... ok
test tests::failed_update_preserves_previous_binary_and_facts ... ok
test tests::adversary_unpublished_verified_generation_recovers_after_interrupted_activation ... ok
test tests::bounded_child_pumps_input_and_output_together ... ok
test tests::bounded_child_refuses_output_flood_and_timeout ... ok
test tests::adversary_operator_interrupt_reaps_bounded_child ... FAILED
test tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation ... ok
test tests::adversary_live_lock_refuses_without_fetching_or_changing_current ... ok

failures:

---- tests::adversary_operator_interrupt_reaps_bounded_child stdout ----

thread 'tests::adversary_operator_interrupt_reaps_bounded_child' (4118171) panicked at crates/mantle-worker/src/lib.rs:747:9:
operator SIGINT left the owned bounded child running after its parent exited
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::adversary_operator_interrupt_reaps_bounded_child

test result: FAILED. 18 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.03s

error: test failed, to rerun pass `-p mantle-worker --lib`
```

`cargo clippy --locked -p mantle -p mantle-worker --all-targets -- -D warnings`, exit0:

```text
    Checking mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
    Checking mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `dev` profile [unoptimized] target(s) in 2.75s
```

`cargo fmt -p mantle -p mantle-worker --check`, exit0, no stdout/stderr. `git diff --check`, exit0, no stdout/stderr.

`ess specify validate --path spec`, exit0:

```text
mantle v1 — 6 file(s), 1 scenario(s), valid
```

## 4. Findings and reachability

| File:line | Verdict / origin / severity | What was measured | What reaches it |
|---|---|---|---|
| crates/mantle-worker/src/lib.rs:451 | CONFIRMED / introduced / blocker | Terminal SIGINT leaves the bounded transport child running after its caller exits. The new test asserts no running child at line747 and exits101 alone and in the suite. | `mantle worker up` uses `Ssh::bounded` / `checked_bounded` for upload/install, which calls this function. The CLI installs no SIGINT handler for this workflow. `.process_group(0)` removes SSH and its proxy descendants from the foreground CLI group, while cleanup runs only after the synchronous loop returns. An operator interrupt therefore exits the only process enforcing the deadline, leaving the child group alive. The test uses sleep instead of a real SSH connection to exercise the exact local process boundary safely. |
| crates/mantle-worker/src/lib.rs:355 | CONFIRMED / introduced / warning | Concurrent first installers can fail with EEXIST before reaching their exclusive installer lock. The new concurrency case exits101 both initially and alone, but passes in the final suite because scheduling changes which caller observes the directory first. | Two root helper `install-codex` invocations on a fresh worker, including overlapping worker-up calls, both execute `install_with` directory setup before lock acquisition. Both can observe a missing directory and race `create_dir`. The fixture uses the same actual filesystem/lock path and only substitutes the pinned download. No corruption or false capability was observed; retry can recover, so this is a reliability warning. |

Both affected functions are newly introduced in the staged diff; the base has no mantle-worker crate. Neither origin relies on guessing that an older implementation reproduces. Suggested correction boundaries: cancellation must retain or explicitly clean the owned child group when the caller is interrupted, and first-directory creation must tolerate a raced AlreadyExists only after revalidating the actual safe directory. These are suggestions, not repairs applied by this pass.

No additional judgement finding.

## 5. Boundaries attacked without another break

- Real FIFO/symlink delivery and installed-manifest inputs refused promptly.
- A held filesystem lock produced the bounded refusal and preserved verified current facts.
- An interrupted activation with a verified unpublished generation recovered on retry.
- Existing unmanaged executable bytes were preserved on refusal.
- Correct archive digest with incorrect executable digest refused before execution/activation.
- Existing corruption/version/idempotence/partial-fetch/unsupported-architecture cases all executed and passed.
- Generated portable model drift check passed;65 actual local decision/store/allowlist scenarios executed without skips, without claiming remote provisioning.
- Read the entire staged change, revision30 acceptance, rendered service tests, selected-Claude call sites and bounded-SSH callers; no live provisioning or authentication claim follows from this pass.

## 6. Outside-worktree writes and handoff

Retained files, with personal home prefixes normalized consistently:

- `~/.cache/mantle-wave3/scratch-worker/adversary-1-cases-compile-error.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-1-cases.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-1-cancel.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-1-concurrent-alone.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-1-suite.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-1-clippy.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-1-fmt.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-1-ess.log`
- `~/.cache/mantle-wave3/scratch-worker/adversary-1-tests.patch`
- `~/.cache/mantle-wave3/scratch-worker/adversary-1.md`
- `/dev/shm/mantle-wave3-target-worker/` and its private `tmp/` descendants: assigned compiler/test output, no deletion.

Managed-worktree CLI also recorded this pass's own lease acquisition, heartbeat and release under session `codex-wave3-worker-adversary1`; no other lease or lifecycle was changed. Conformance regenerated its existing ignored suite/report under the assigned worktree. Final process inspection found no running fixture sleep or cancellation driver; every observed child group was killed before its assertion. The source tree remains `mantle-wave3-worker`, branch `impl/agent-ready-worker`, with staged implementor source unchanged and191 unstaged test lines. Coordinator owns fixes, durable recording, live AW acceptance, commits and cleanup.

## 7. Machine-readable findings

```findings
- file: crates/mantle-worker/src/lib.rs
  line: 451
  category: boundary
  severity: blocker
  verdict: CONFIRMED
  origin: introduced
  message: Terminal SIGINT leaves the bounded transport child running after its caller exits.
- file: crates/mantle-worker/src/lib.rs
  line: 355
  category: concurrency
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: Concurrent first installers can fail with EEXIST before reaching their exclusive installer lock.
```
