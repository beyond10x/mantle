---
format: aep.planning-md/3
id: review-result:prebuilt-release-adversary-1
kind: review-result
status: active
title: Prebuilt release adversary, pass 1
relations:
- reviews: story:prebuilt-release-artifacts
revision: 1
---
unit: story:prebuilt-release-artifacts at 6f4303dfda5031e111380d1b28441e07dc7e4144 plus two uncommitted adversary tests
verdict: CONFIRMED
cases: executed 10→12 release tests, red 2
origin: introduced 0 / pre-existing 0 / undecided 2
wrote-outside-worktree: 10 retained files/directories listed below
needs-coordinator: route source-export blocker and modified-generation warning to implementor; source unchanged

```text
 crates/mantle-release/tests/release.rs | 92 ++++++++++++++++++++++++++++++++++
 1 file changed, 92 insertions(+)
```

1. Cases were written before any execution. Existing cases were not changed or weakened. The baseline10 release tests is from the implementor's final handoff; its Mantle61/worker29/CLI263 counts are not claimed as rerun here.

First new case: `crates/mantle-release/tests/release.rs:687`, `adversary_untracked_git_attributes_cannot_change_exact_source_export`. A real temporary Git repository commits Rust source containing a Git substitution marker. Its untracked `.git/info/attributes` enables export-subst. The checkout remains clean and source_identity accepts its exact HEAD. The production export_source function succeeds but emits different Rust bytes. The case permits safe refusal or an exact export; current behavior is neither. The build CLI calls that function before compiling the exported source.

Command: `cargo test --locked -p mantle-release --test release adversary_untracked_git_attributes_cannot_change_exact_source_export -- --exact`
Exit101. First isolated output, verbatim except portable `$HOME` substitution:

```text
   Compiling mantle-release v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-prebuilt-release/crates/mantle-release)
    Finished `test` profile [unoptimized] target(s) in 2.37s
     Running tests/release.rs (/dev/shm/mantle-prebuilt-release-target/debug/deps/release-1fd8b9d8598628d0)

running 1 test
test adversary_untracked_git_attributes_cannot_change_exact_source_export ... FAILED

failures:

---- adversary_untracked_git_attributes_cannot_change_exact_source_export stdout ----

thread 'adversary_untracked_git_attributes_cannot_change_exact_source_export' (2771372) panicked at crates/mantle-release/tests/release.rs:735:9:
assertion `left == right` failed: accepted exact-source export changed committed Rust bytes using untracked Git attributes
  left: [102, 110, 32, 109, 97, 105, 110, 40, 41, 32, 123, 32, 112, 114, 105, 110, 116, 108, 110, 33, 40, 34, 102, 97, 99, 52, 50, 102, 50, 54, 97, 54, 55, 53, 51, 49, 98, 52, 102, 49, 50, 54, 52, 50, 51, 51, 51, 100, 101, 101, 50, 54, 101, 56, 51, 97, 48, 55, 55, 99, 102, 99, 34, 41, 59, 32, 125, 10]
 right: [102, 110, 32, 109, 97, 105, 110, 40, 41, 32, 123, 32, 112, 114, 105, 110, 116, 108, 110, 33, 40, 34, 36, 70, 111, 114, 109, 97, 116, 58, 37, 72, 36, 34, 41, 59, 32, 125, 10]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary_untracked_git_attributes_cannot_change_exact_source_export

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.04s

error: test failed, to rerun pass `-p mantle-release --test release`
```

Second new case: `crates/mantle-release/tests/release.rs:743`, `adversary_modified_generation_inventory_is_refused_without_activation`. Install a valid fixture through the actual CLI, add an unmanaged file inside the active generation, then run the same install again. Existing verified bytes and activation are preserved, but the CLI exits0 and accepts the modified inventory. This is narrower than code replacement: the added file is not one of the exposed CLI symlinks, so severity is warning.

Command: `cargo test --locked -p mantle-release --test release adversary_modified_generation_inventory_is_refused_without_activation -- --exact`
Exit101. First isolated output, verbatim except portable `$HOME` substitution:

```text
    Finished `test` profile [unoptimized] target(s) in 0.24s
     Running tests/release.rs (/dev/shm/mantle-prebuilt-release-target/debug/deps/release-1fd8b9d8598628d0)

running 1 test
test adversary_modified_generation_inventory_is_refused_without_activation ... FAILED

failures:

---- adversary_modified_generation_inventory_is_refused_without_activation stdout ----

thread 'adversary_modified_generation_inventory_is_refused_without_activation' (2775684) panicked at crates/mantle-release/tests/release.rs:772:5:
installer accepted an existing generation with an extra unmanaged payload
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary_modified_generation_inventory_is_refused_without_activation

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 3.50s

error: test failed, to rerun pass `-p mantle-release --test release`
```

2. Only after both isolated cases, ran `cargo test --locked -p mantle-artifact -p mantle-release`, exit101. All10 original release cases passed; both added cases failed. Cargo stopped at this failed integration target; subsequent doc-test execution is not claimed. No full repository gate or production artifact rebuild was attempted.

Environment for all three commands: `PATH=$HOME/.cache/aap-ess-upgrade/ess-0.50/bin:$PATH`, `TMPDIR=/dev/shm/mantle-prebuilt-release-tmp`, `CARGO_TARGET_DIR=/dev/shm/mantle-prebuilt-release-target`, `CARGO_BUILD_JOBS=2`, `CARGO_PROFILE_DEV_DEBUG=0`, `RUSTC_WRAPPER=/usr/bin/sccache`, `SCCACHE_SERVER_PORT=43179`, `SCCACHE_DIR=/dev/shm/mantle-reliability-compiler-cache`, `SCCACHE_CACHE_SIZE=1G`.

Full suite output, verbatim except portable `$HOME` substitution:

```text
    Finished `test` profile [unoptimized] target(s) in 0.19s
     Running unittests src/lib.rs (/dev/shm/mantle-prebuilt-release-target/debug/deps/mantle_artifact-c8f34bd4e86578bd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/dev/shm/mantle-prebuilt-release-target/debug/deps/mantle_release-58c0e9cbe9387d86)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/dev/shm/mantle-prebuilt-release-target/debug/deps/mantle_release-5176d4dd5c88dc9c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/release.rs (/dev/shm/mantle-prebuilt-release-target/debug/deps/release-1fd8b9d8598628d0)

running 12 tests
test source_identity_refuses_dirty_or_wrong_revision_even_same_version ... ok
test adversary_untracked_git_attributes_cannot_change_exact_source_export ... FAILED
test actual_git_archive_exports_exact_source_and_still_refuses_source_symlinks ... ok
test strict_manifest_and_checksum_refusals_are_not_cli_parse_errors ... ok
test valid_packages_are_verified_through_cli ... ok
test missing_linked_fifo_and_oversized_inputs_and_unsupported_target_refuse ... ok
test install_is_managed_and_atomic_and_refuses_unmanaged_collision ... ok
test adversary_modified_generation_inventory_is_refused_without_activation ... FAILED
test concurrent_installers_and_interrupted_activation_preserve_a_complete_generation ... ok
test deterministic_packaging_and_interrupted_process_keep_previous_activation ... ok
test unsafe_archive_members_and_nonstatic_workers_are_refused ... ok
test bot_publication_refuses_remote_tag_mismatch_existing_and_unknown_release_and_reports_upload_failure ... ok

failures:

---- adversary_untracked_git_attributes_cannot_change_exact_source_export stdout ----

thread 'adversary_untracked_git_attributes_cannot_change_exact_source_export' (2783350) panicked at crates/mantle-release/tests/release.rs:735:9:
assertion `left == right` failed: accepted exact-source export changed committed Rust bytes using untracked Git attributes
  left: [102, 110, 32, 109, 97, 105, 110, 40, 41, 32, 123, 32, 112, 114, 105, 110, 116, 108, 110, 33, 40, 34, 54, 57, 102, 56, 48, 99, 50, 97, 54, 52, 49, 99, 49, 97, 53, 53, 54, 97, 102, 99, 99, 51, 101, 50, 98, 98, 99, 48, 56, 53, 48, 50, 52, 53, 53, 100, 97, 50, 57, 48, 34, 41, 59, 32, 125, 10]
 right: [102, 110, 32, 109, 97, 105, 110, 40, 41, 32, 123, 32, 112, 114, 105, 110, 116, 108, 110, 33, 40, 34, 36, 70, 111, 114, 109, 97, 116, 58, 37, 72, 36, 34, 41, 59, 32, 125, 10]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- adversary_modified_generation_inventory_is_refused_without_activation stdout ----

thread 'adversary_modified_generation_inventory_is_refused_without_activation' (2783349) panicked at crates/mantle-release/tests/release.rs:772:5:
installer accepted an existing generation with an extra unmanaged payload


failures:
    adversary_modified_generation_inventory_is_refused_without_activation
    adversary_untracked_git_attributes_cannot_change_exact_source_export

test result: FAILED. 10 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 18.34s

error: test failed, to rerun pass `-p mantle-release --test release`
```

3. Findings against6f4303df plus the two tests:

| File:line | Verdict / origin / severity | Finding | What was measured | Reachable caller |
|---|---|---|---|---|
| crates/mantle-release/src/build.rs:81 | CONFIRMED / undecided / blocker | Untracked Git export attributes can change committed Rust bytes while the release builder accepts the checkout and labels the export with its exact source commit. | Test assertion at release.rs:735 failed after source_identity and export_source both succeeded on a clean real Git fixture; isolated and suite exits101. | `mantle-release build --source ... --revision HEAD` calls export_source on the operator checkout; Git's ordinary .git/info/attributes affects archive while remaining absent from status. No injected Git executable or impossible filesystem state is required. |
| crates/mantle-release/src/lib.rs:164 | CONFIRMED / undecided / warning | Reinstall accepts an existing generation containing an extra unmanaged payload despite the documented refusal of modified generations. | Test assertion at release.rs:772 failed: actual install CLI exited0 after a file was added under the active generation; original activation and extra file remained. | Rerunning the documented install command against a previously installed prefix after its generation directory was modified. The test does not establish execution of the extra file or corruption of the exposed binaries. |

Origins are conservatively undecided: the base was read, but no executable base comparison was performed. Both implicated implementations are additions within the unit's source diff; no pre-existing attribution is asserted. Suggested bounded corrections are to validate exported member inventory/content against the exact Git tree, and to validate exact installed generation inventory before accepting reuse. These are suggestions, not implementation edits. No separate judgement-only findings.

4. Existing counterexamples for manifest/checksum failures, malformed archive inventory, nonstatic workers, bounded special inputs, dirty/wrong source identity, concurrent installation, killed lock waiter and bot publication failures remained green. Publication tests use the existing local Rust double; no actual GitHub, provider, worker or authentication operation occurred. No approval or independent-verifier claim.

5. Retained external paths:
- `$HOME/.cache/mantle-reliability/prebuilt-release/adversary/source-case.log`
- `$HOME/.cache/mantle-reliability/prebuilt-release/adversary/generation-case.log`
- `$HOME/.cache/mantle-reliability/prebuilt-release/adversary/suite.log`
- `$HOME/.cache/mantle-reliability/prebuilt-release/adversary/review.md`
- `/dev/shm/mantle-prebuilt-release-target`
- `/dev/shm/mantle-prebuilt-release-tmp` including process-lifetime binary fixtures
- `/dev/shm/mantle-reliability-compiler-cache`
- `$HOME/.local/state/worktree/registry.sqlite3`
- `$HOME/.local/state/worktree/registry.sqlite3-wal`
- `$HOME/.local/state/worktree/registry.sqlite3-shm`

The92-line test addition remains uncommitted. No implementation, planning or generated-source edits, commits, publication or release. `git diff --check` passed. Own lease codex-prebuilt-release-adversary ended on handoff. Source tree, target, temporary fixtures and logs remain for the coordinator; no managed tree or shared cache was removed.

```findings
- file: crates/mantle-release/src/build.rs
  line: 81
  category: acceptance
  severity: blocker
  verdict: CONFIRMED
  origin: undecided
  message: Untracked Git export attributes can change committed Rust bytes while the release builder accepts the checkout and labels the export with its exact source commit.
- file: crates/mantle-release/src/lib.rs
  line: 164
  category: contract-drift
  severity: warning
  verdict: CONFIRMED
  origin: undecided
  message: Reinstall accepts an existing generation containing an extra unmanaged payload despite the documented refusal of modified generations.
```
