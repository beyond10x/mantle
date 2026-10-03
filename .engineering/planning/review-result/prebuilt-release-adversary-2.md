---
format: aep.planning-md/3
id: review-result:prebuilt-release-adversary-2
kind: review-result
status: active
title: Prebuilt release adversary, pass 2
relations:
- reviews: story:prebuilt-release-artifacts
revision: 1
---
unit: story:prebuilt-release-artifacts pass2 at 107f9c8bbdc9062cc24e54ac69e753d0f9e7fb68 plus one uncommitted adversary test
verdict: nothing found
cases: executed 14→15 release tests, red 0 final; initial parallel suite had3 infrastructure failures
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: 11 retained files/directories listed below
needs-coordinator: retain the uncommitted tests-only addition if wanted; quota-constrained parallel runner needs capacity before full gate

```text
 crates/mantle-release/tests/release.rs | 60 ++++++++++++++++++++++++++++++++++
 1 file changed, 60 insertions(+)
```

1. Read the final correction diff, README, added class tests and correction handoff before execution. The original pass1 regressions and assertions were unchanged. Added `adversary_exact_export_preserves_shared_blobs_and_per_path_modes` in `crates/mantle-release/tests/release.rs`: a real committed tree has the same Rust blob at three nested paths with spaces, a tab and Unicode, plus distinct executable modes. The production exporter must preserve every path, bytes, mode and exact directory inventory. This exercises raw tree parsing and deduplicated blob reads without trusting Git archive output as its own oracle.

The case was written before anything ran. First isolated command: `cargo test --locked -p mantle-release --test release adversary_exact_export_preserves_shared_blobs_and_per_path_modes -- --exact`, exit0. No red output. Verbatim output below substitutes only the operator home prefix with `$HOME`.

```text
   Compiling mantle-release v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-prebuilt-release/crates/mantle-release)
    Finished `test` profile [unoptimized] target(s) in 1.09s
     Running tests/release.rs (/dev/shm/mantle-prebuilt-release-target/debug/deps/release-1fd8b9d8598628d0)

running 1 test
test adversary_exact_export_preserves_shared_blobs_and_per_path_modes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.04s

```

2. Targeted retained regressions: `cargo test --locked -p mantle-release --test release adversary_`, exit0; both original cases and the new case passed. No original assertion was weakened. Output:

```text
    Finished `test` profile [unoptimized] target(s) in 0.10s
     Running tests/release.rs (/dev/shm/mantle-prebuilt-release-target/debug/deps/release-1fd8b9d8598628d0)

running 3 tests
test adversary_exact_export_preserves_shared_blobs_and_per_path_modes ... ok
test adversary_untracked_git_attributes_cannot_change_exact_source_export ... ok
test adversary_modified_generation_inventory_is_refused_without_activation ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 8.52s

```

3. Subsequent package command `cargo test --locked -p mantle-artifact -p mantle-release` exited101:12 passed and3 failed at valid-fixture verification/initial installation. The reported underlying error was `Disk quota exceeded (os error122)`. No case had reached the old defect assertion. Both tmpfs mounts carry usrquota; global free space was still root10GiB, tmpfs6.6GiB, so global free-space checks did not rule out per-user limits. The quota utility is absent; no quota setting or cache was changed. This is retained as a failed run, not counted as green or a product finding. Verbatim output:

```text
    Finished `test` profile [unoptimized] target(s) in 0.12s
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

running 15 tests
test source_identity_refuses_dirty_or_wrong_revision_even_same_version ... ok
test adversary_exact_export_preserves_shared_blobs_and_per_path_modes ... ok
test adversary_untracked_git_attributes_cannot_change_exact_source_export ... ok
test actual_git_archive_exports_exact_source_and_still_refuses_source_symlinks ... ok
test exact_source_export_checks_attributes_inventory_modes_and_replacement_objects ... ok
test strict_manifest_and_checksum_refusals_are_not_cli_parse_errors ... ok
test generation_reuse_checks_all_entry_locations_and_types ... FAILED
test adversary_modified_generation_inventory_is_refused_without_activation ... FAILED
test valid_packages_are_verified_through_cli ... FAILED
test missing_linked_fifo_and_oversized_inputs_and_unsupported_target_refuse ... ok
test concurrent_installers_and_interrupted_activation_preserve_a_complete_generation ... ok
test install_is_managed_and_atomic_and_refuses_unmanaged_collision ... ok
test deterministic_packaging_and_interrupted_process_keep_previous_activation ... ok
test unsafe_archive_members_and_nonstatic_workers_are_refused ... ok
test bot_publication_refuses_remote_tag_mismatch_existing_and_unknown_release_and_reports_upload_failure ... ok

failures:

---- generation_reuse_checks_all_entry_locations_and_types stdout ----

thread 'generation_reuse_checks_all_entry_locations_and_types' (3389087) panicked at crates/mantle-release/tests/release.rs:926:9:
assertion failed: fixture.cli("install", &["--prefix", prefix_arg]).status.success()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- adversary_modified_generation_inventory_is_refused_without_activation stdout ----

thread 'adversary_modified_generation_inventory_is_refused_without_activation' (3389077) panicked at crates/mantle-release/tests/release.rs:748:5:
assertion failed: fixture.cli("install", &["--prefix", prefix_arg]).status.success()

---- valid_packages_are_verified_through_cli stdout ----

thread 'valid_packages_are_verified_through_cli' (3389094) panicked at crates/mantle-release/tests/release.rs:128:5:
Error: Disk quota exceeded (os error 122)



failures:
    adversary_modified_generation_inventory_is_refused_without_activation
    generation_reuse_checks_all_entry_locations_and_types
    valid_packages_are_verified_through_cli

test result: FAILED. 12 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 17.36s

error: test failed, to rerun pass `-p mantle-release --test release`
```

The unchanged source/tests were then run serially to reduce concurrent temporary allocation: `cargo test --locked -p mantle-artifact -p mantle-release -- --test-threads=1`. Baseline14 is attributed to the implementor's correction handoff, not a pre-attack suite run. Other unchanged package/native scenario counts were not rerun here. Output follows:

```text
    Finished `test` profile [unoptimized] target(s) in 0.14s
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

running 15 tests
test actual_git_archive_exports_exact_source_and_still_refuses_source_symlinks ... ok
test adversary_exact_export_preserves_shared_blobs_and_per_path_modes ... ok
test adversary_modified_generation_inventory_is_refused_without_activation ... ok
test adversary_untracked_git_attributes_cannot_change_exact_source_export ... ok
test bot_publication_refuses_remote_tag_mismatch_existing_and_unknown_release_and_reports_upload_failure ... ok
test concurrent_installers_and_interrupted_activation_preserve_a_complete_generation ... ok
test deterministic_packaging_and_interrupted_process_keep_previous_activation ... ok
test exact_source_export_checks_attributes_inventory_modes_and_replacement_objects ... ok
test generation_reuse_checks_all_entry_locations_and_types ... ok
test install_is_managed_and_atomic_and_refuses_unmanaged_collision ... ok
test missing_linked_fifo_and_oversized_inputs_and_unsupported_target_refuse ... ok
test source_identity_refuses_dirty_or_wrong_revision_even_same_version ... ok
test strict_manifest_and_checksum_refusals_are_not_cli_parse_errors ... ok
test unsafe_archive_members_and_nonstatic_workers_are_refused ... ok
test valid_packages_are_verified_through_cli ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 67.22s

   Doc-tests mantle_artifact

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mantle_release

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

Serial package exit0:15 passed,0 failed; zero-case artifact/library/binary/doc-test lanes passed. Full gate parallelism and host capacity remain the coordinator's concern, not an implementation change requested by this pass.

Environment for all Cargo calls: `PATH=$HOME/.cache/aap-ess-upgrade/ess-0.50/bin:$PATH`, `TMPDIR=/dev/shm/mantle-prebuilt-release-tmp`, `CARGO_TARGET_DIR=/dev/shm/mantle-prebuilt-release-target`, `CARGO_BUILD_JOBS=2`, `CARGO_PROFILE_DEV_DEBUG=0`, `RUSTC_WRAPPER=/usr/bin/sccache`, `SCCACHE_SERVER_PORT=43179`, `SCCACHE_DIR=/dev/shm/mantle-reliability-compiler-cache`, `SCCACHE_CACHE_SIZE=1G`.

4. No carried findings: the exact pass1 signatures `crates/mantle-release/src/build.rs:81 / CONFIRMED / undecided` and `crates/mantle-release/src/lib.rs:164 / CONFIRMED / undecided` no longer reproduce with their unchanged tests. The coordinator's later origin resolution to introduced is recorded in its store and is not rewritten here. No new or judgement-only finding. This is not approval or an independent-verifier claim.

5. Attacked shared blob/path/mode parsing and reran complete release cases. Corrected export rejects untracked/tracked attribute changes and missing members; correction tests also preserve original blob/commit identities despite replacement refs. Corrected generation inventory refuses extra/missing/symlink/type changes. Existing archive, concurrent install, interruption and controlled bot-publication cases passed serially. No real production rebuild, release, GitHub operation, provider/worker operation or authentication inspection was performed in this pass.

6. Exact retained external paths:
- `$HOME/.cache/mantle-reliability/prebuilt-release/adversary/pass2/case.log`
- `$HOME/.cache/mantle-reliability/prebuilt-release/adversary/pass2/regressions.log`
- `$HOME/.cache/mantle-reliability/prebuilt-release/adversary/pass2/suite.log`
- `$HOME/.cache/mantle-reliability/prebuilt-release/adversary/pass2/suite-serial.log`
- `$HOME/.cache/mantle-reliability/prebuilt-release/adversary/pass2/review.md`
- `/dev/shm/mantle-prebuilt-release-target`
- `/dev/shm/mantle-prebuilt-release-tmp` including process-lifetime fixture binaries
- `/dev/shm/mantle-reliability-compiler-cache`
- `$HOME/.local/state/worktree/registry.sqlite3`
- `$HOME/.local/state/worktree/registry.sqlite3-wal`
- `$HOME/.local/state/worktree/registry.sqlite3-shm`

Report1 and its logs remain unchanged. Only the60-line new test remains uncommitted. No implementation, planning, generated-source edits or commits. `git diff --check` passed. Own lease codex-prebuilt-release-adversary ended on handoff. Source tree, target, fixtures and all outputs remain for coordinator ownership; no cleanup of shared/user files was attempted.

```findings
[]
```
