---
format: aep.planning-md/3
id: review-result:reliability-wave7-adversary-pass1
kind: review-result
status: active
title: Acceptance review finds cleanup erasing a qualification failure
relations:
- reviews: story:repeatable-agent-acceptance
revision: 1
---
unit: story:repeatable-agent-acceptance at 2b8b21a0aeb1f85900ba12c2ed481f891452a7a5, adversary pass1
verdict: CONFIRMED
cases: executed 6→8 acceptance runner cases, red 1
origin: introduced 0 / pre-existing 0 / undecided 1
wrote-outside-worktree: six path roots, detailed in part 6
needs-coordinator: route the preserved failing case for correction and record this report

```text
 crates/mantle-acceptance/tests/runner.rs      | 112 ++++++++++++++++++++++++++
 crates/mantle-acceptance/tests/support/cli.rs |   3 +
 2 files changed, 115 insertions(+)
```

1. Only the two test paths above changed, with all prior assertions retained. The working tree remains uncommitted on `unit/mantle-agent-acceptance`. The review covers candidate `2b8b21a0aeb1f85900ba12c2ed481f891452a7a5` against base `7930ab106f3a2605ab8054552d68241c29e8566e`. Read the whole candidate diff, callers, story revision36, shared invariants, both unit briefs, and runtime/documentation handoffs before adding cases. No runtime, generated, specification, documentation or AEP file was edited. No live product operation or authentication was performed.

2. Two cases were written before any test execution. The first asserts the newly documented failure-preservation contract (README:297–299) using ordinary run/resume/cleanup/report CLI calls. A controlled installed-CLI fixture refuses only the final destroy command. The real acceptance runner first records nine passed cases and a failed destroy. A later successful cleanup must remove the owned resource while retaining the measured qualification failure and failure exit. The checkpoint is never hand-edited. This case is RED.

The second interrupts the runner during its machine PTY phase through the existing bounded process owner. Cleanup against a reused-name refusal must preserve session state; later exact-owned cleanup must destroy the owned resource while leaving unexecuted cases incomplete. Every recorded destroy invocation must carry the exact expected session ID. This case is GREEN.

Cargo environment for all commands below:

```console
export PATH="$HOME/.cache/aap-ess-upgrade/ess-0.50/bin:$PATH"
export TMPDIR="$HOME/.cache/mantle-reliability/agent-acceptance-tmp"
export CARGO_TARGET_DIR="$HOME/.cache/mantle-reliability/agent-acceptance-target"
export CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 RUST_TEST_THREADS=1
export RUSTC_WRAPPER=/usr/bin/sccache SCCACHE_SERVER_PORT=43179
export SCCACHE_DIR=/dev/shm/mantle-reliability-compiler-cache SCCACHE_CACHE_SIZE=1G SCCACHE_IDLE_TIMEOUT=0
```

Raw logs are retained unchanged in the assigned private scratch. In the portable copies below, only the personal home prefix is replaced with `$HOME`; no outcome, assertion or diagnostic is removed. The first compile emitted an unused-mut warning in the new second case; that redundant binding qualifier was removed before its isolated run. No assertion changed.

First isolated command, exit101:

```console
cargo test --locked -p mantle-acceptance --test runner adversary_cleanup_preserves_failed_final_qualification_case -- --exact
```

```text
   Compiling mantle-worker v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-agent-acceptance/crates/mantle-worker)
   Compiling mantle-acceptance v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-agent-acceptance/crates/mantle-acceptance)
warning: variable does not need to be mutable
   --> crates/mantle-acceptance/tests/runner.rs:414:9
    |
414 |     let mut child =
    |         ----^^^^^
    |         |
    |         help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: `mantle-acceptance` (test "runner") generated 1 warning (run `cargo fix --test "runner" -p mantle-acceptance` to apply 1 suggestion)
    Finished `test` profile [unoptimized] target(s) in 1.85s
     Running tests/runner.rs ($HOME/.cache/mantle-reliability/agent-acceptance-target/debug/deps/runner-70ad98faf127346c)

running 1 test
test adversary_cleanup_preserves_failed_final_qualification_case ... FAILED

failures:

---- adversary_cleanup_preserves_failed_final_qualification_case stdout ----

thread 'adversary_cleanup_preserves_failed_final_qualification_case' (530585) panicked at crates/mantle-acceptance/tests/runner.rs:390:5:
assertion `left == right` failed: successful cleanup must not erase a measured qualification failure: {"agent":"codex","executable":"$HOME/.cache/mantle-reliability/agent-acceptance-tmp/.tmp4jNv1R/fixture","executable_sha256":"5dce98317576df95eaff8522b7a1bb08592ba2cebf7bff5f2cb1fd8996026535","executable_version":"mantle 0.1.4","format":"mantle-acceptance/v1","phase":"cleanup","profile":"fixture","results":[{"case":"create","exec_id":"exec1","observed_at":"2026-10-03T17:21:58.410715729+00:00","origin":"machine","phase":"manual","status":"passed"},{"case":"login","exec_id":"exec1","observed_at":"2026-10-03T17:21:59.365830595+00:00","origin":"operator","phase":"manual","status":"passed"},{"case":"model-tool","exec_id":"exec1","observed_at":"2026-10-03T17:21:59.365838188+00:00","origin":"operator","phase":"manual","status":"passed"},{"case":"visual","exec_id":"exec1","observed_at":"2026-10-03T17:21:59.365839422+00:00","origin":"operator","phase":"manual","status":"passed"},{"case":"status-exec","exec_id":"exec1","observed_at":"2026-10-03T17:22:01.275177768+00:00","origin":"machine","phase":"status-exec","status":"passed"},{"case":"detach-reconnect","exec_id":"exec1","observed_at":"2026-10-03T17:22:06.559770784+00:00","origin":"machine","phase":"detach-reconnect","status":"passed"},{"case":"resize-interrupt","exec_id":"exec1","observed_at":"2026-10-03T17:22:09.615186234+00:00","origin":"machine","phase":"resize-interrupt","status":"passed"},{"case":"transport-loss","exec_id":"exec1","observed_at":"2026-10-03T17:22:14.962475285+00:00","origin":"machine","phase":"transport-loss","status":"passed"},{"case":"retain-restart","exec_id":"exec2","observed_at":"2026-10-03T17:22:17.941749405+00:00","origin":"machine","phase":"retain-restart","status":"passed"},{"case":"destroy","exec_id":"exec2","observed_at":"2026-10-03T17:22:19.900733678+00:00","origin":"machine","phase":"cleanup","status":"passed"}],"run_dir":"$HOME/.cache/mantle-reliability/agent-acceptance-tmp/.tmp7NEXV1/mantle-acceptance-Nn09wN","run_id":"01m41cjvf9ef230dr2ydd3df7w","selection":{"config_path":"$HOME/.cache/mantle-reliability/agent-acceptance-tmp/.tmp7NEXV1/config.toml","config_sha256":"ec356eea78fb4cbccc9885fa5c4159f62ee2b8d547bd5ee38e7e240d637b1ff5","profile":"fixture","state_dir":"$HOME/.cache/mantle-reliability/agent-acceptance-tmp/.tmp7NEXV1/state"},"session":{"agent":"codex","authentication":"chatgpt-device","exec_id":"exec2","generation":0,"id":"owned","name":"acceptance-01m41cjvf9ef230dr2ydd3df7w","observed":{"exec_state":"running","exit_code":null,"exit_signal":null,"refused":false,"workspace_state":"ready"},"recorded_state":"RUNNING","source_commits":["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],"workspace_id":"ws"},"source_commit":null}
  left: (Some(0), Some(0), Some("passed"))
 right: (Some(1), Some(1), Some("failed"))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary_cleanup_preserves_failed_final_qualification_case

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 7 filtered out; finished in 22.46s

error: test failed, to rerun pass `-p mantle-acceptance --test runner`
```

Second isolated command, exit0:

```console
cargo test --locked -p mantle-acceptance --test runner adversary_interrupted_machine_phase_cleanup_keeps_incomplete_and_exact_ownership -- --exact
```

```text
   Compiling mantle-acceptance v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-agent-acceptance/crates/mantle-acceptance)
    Finished `test` profile [unoptimized] target(s) in 0.28s
     Running tests/runner.rs ($HOME/.cache/mantle-reliability/agent-acceptance-target/debug/deps/runner-70ad98faf127346c)

running 1 test
test adversary_interrupted_machine_phase_cleanup_keeps_incomplete_and_exact_ownership ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 6.04s

```

3. Only after both new cases had executed alone, the acceptance package suite ran. Baseline6 comes from the implementor handoff, not a pre-attack suite execution. The package ran8 runner cases:7passed/1failed, exit101. Empty library and binary unit-test lanes also ran; no claim is made that this adversary reran the full143-package aggregate or CLI381 conformance. Both six existing cases and the new interrupted-cleanup case passed.

```console
cargo test --locked -p mantle-acceptance
```

```text
   Compiling mantle-acceptance v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-agent-acceptance/crates/mantle-acceptance)
    Finished `test` profile [unoptimized] target(s) in 0.41s
     Running unittests src/lib.rs ($HOME/.cache/mantle-reliability/agent-acceptance-target/debug/deps/mantle_acceptance-95131a4bc350eb70)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs ($HOME/.cache/mantle-reliability/agent-acceptance-target/debug/deps/mantle_acceptance-ad97a0714945412c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/runner.rs ($HOME/.cache/mantle-reliability/agent-acceptance-target/debug/deps/runner-70ad98faf127346c)

running 8 tests
test adversary_cleanup_preserves_failed_final_qualification_case ... FAILED
test adversary_interrupted_machine_phase_cleanup_keeps_incomplete_and_exact_ownership ... ok
test both_agents_use_actual_cli_pty_and_same_inventory_without_raw_reports ... ok
test lost_receipt_never_authorizes_cleanup_and_name_reuse_refuses ... ok
test output_flood_fails_and_manual_abort_does_not_fabricate_completion ... ok
test pty_timeout_retires_descendants_and_interruption_retains_unfinished_evidence ... ok
test stale_attestation_selection_binary_and_public_checkpoint_refuse ... ok
test unverified_release_and_replaced_selected_executable_refuse_before_mutation ... ok

failures:

---- adversary_cleanup_preserves_failed_final_qualification_case stdout ----

thread 'adversary_cleanup_preserves_failed_final_qualification_case' (543579) panicked at crates/mantle-acceptance/tests/runner.rs:390:5:
assertion `left == right` failed: successful cleanup must not erase a measured qualification failure: {"agent":"codex","executable":"$HOME/.cache/mantle-reliability/agent-acceptance-tmp/.tmpKCS3w3/fixture","executable_sha256":"5dce98317576df95eaff8522b7a1bb08592ba2cebf7bff5f2cb1fd8996026535","executable_version":"mantle 0.1.4","format":"mantle-acceptance/v1","phase":"cleanup","profile":"fixture","results":[{"case":"create","exec_id":"exec1","observed_at":"2026-10-03T17:23:35.378348773+00:00","origin":"machine","phase":"manual","status":"passed"},{"case":"login","exec_id":"exec1","observed_at":"2026-10-03T17:23:36.360414699+00:00","origin":"operator","phase":"manual","status":"passed"},{"case":"model-tool","exec_id":"exec1","observed_at":"2026-10-03T17:23:36.360421934+00:00","origin":"operator","phase":"manual","status":"passed"},{"case":"visual","exec_id":"exec1","observed_at":"2026-10-03T17:23:36.360423303+00:00","origin":"operator","phase":"manual","status":"passed"},{"case":"status-exec","exec_id":"exec1","observed_at":"2026-10-03T17:23:38.255085923+00:00","origin":"machine","phase":"status-exec","status":"passed"},{"case":"detach-reconnect","exec_id":"exec1","observed_at":"2026-10-03T17:23:43.532465229+00:00","origin":"machine","phase":"detach-reconnect","status":"passed"},{"case":"resize-interrupt","exec_id":"exec1","observed_at":"2026-10-03T17:23:46.595904780+00:00","origin":"machine","phase":"resize-interrupt","status":"passed"},{"case":"transport-loss","exec_id":"exec1","observed_at":"2026-10-03T17:23:51.893850171+00:00","origin":"machine","phase":"transport-loss","status":"passed"},{"case":"retain-restart","exec_id":"exec2","observed_at":"2026-10-03T17:23:54.788076850+00:00","origin":"machine","phase":"retain-restart","status":"passed"},{"case":"destroy","exec_id":"exec2","observed_at":"2026-10-03T17:23:56.664975215+00:00","origin":"machine","phase":"cleanup","status":"passed"}],"run_dir":"$HOME/.cache/mantle-reliability/agent-acceptance-tmp/.tmpRDP2kG/mantle-acceptance-YXnh8f","run_id":"01m41cnt64ry8x2znn7ye8vb42","selection":{"config_path":"$HOME/.cache/mantle-reliability/agent-acceptance-tmp/.tmpRDP2kG/config.toml","config_sha256":"ec356eea78fb4cbccc9885fa5c4159f62ee2b8d547bd5ee38e7e240d637b1ff5","profile":"fixture","state_dir":"$HOME/.cache/mantle-reliability/agent-acceptance-tmp/.tmpRDP2kG/state"},"session":{"agent":"codex","authentication":"chatgpt-device","exec_id":"exec2","generation":0,"id":"owned","name":"acceptance-01m41cnt64ry8x2znn7ye8vb42","observed":{"exec_state":"running","exit_code":null,"exit_signal":null,"refused":false,"workspace_state":"ready"},"recorded_state":"RUNNING","source_commits":["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],"workspace_id":"ws"},"source_commit":null}
  left: (Some(0), Some(0), Some("passed"))
 right: (Some(1), Some(1), Some("failed"))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary_cleanup_preserves_failed_final_qualification_case

test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 106.52s

error: test failed, to rerun pass `-p mantle-acceptance --test runner`
```

4. Findings against the exact candidate above:

| File:line | Category | Severity | Verdict | Origin | Message |
| --- | --- | --- | --- | --- | --- |
| crates/mantle-acceptance/src/runner.rs:458 | contract-drift | blocker | CONFIRMED | undecided | Successful cleanup overwrites a previously failed destroy qualification case, causing cleanup and offline report to return success instead of preserving the measured failure. |

**What was measured:** `adversary_cleanup_preserves_failed_final_qualification_case` asserts the documented preservation rule at `crates/mantle-acceptance/tests/runner.rs:390`. Both isolated and package runs fail with actual `(Some(0), Some(0), Some("passed"))`, expected `(Some(1), Some(1), Some("failed"))`. Before cleanup the unchanged checkpoint contains exactly9passed/1failed; the fixture confirms the owned session is subsequently destroyed successfully. `cleanup` unconditionally replaces the destroy row via `observe`; the aggregate evaluator then sees ten passes. The retained case exercises actual runner process/CLI/checkpoint/PTY code, not a manually fabricated all-passed checkpoint.

**What reaches it:** README:287–304 documents `resume` followed by explicit `cleanup`, and README:297–299 promises that a prior failed case remains failed. A transient nonzero exit from the final guarded installed-CLI destroy is a normal reachable failure (for example an unavailable worker transport or incomplete destruction). The later operator cleanup can succeed. Both commands route through public clap commands to this function, with the same exact-owned receipt and no altered metadata. The counterexample is specifically the already-measured final qualification failure being erased; it does not object to the successful cleanup of resources.

**Attribution:** no base executable of the new acceptance runner is available, so origin remains undecided in this report. Read-only `git ls-tree 7930ab1 crates/mantle-acceptance` returns no entry; the crate is added by the candidate, which the coordinator may use for its source attribution. No checkout was moved to the base.

Suggested correction: keep the original failed qualification result when later cleanup succeeds, while recording cleanup completion separately or otherwise preserving the failure in the aggregate. Retain this case and the existing manual-abort cleanup assertions. No correction was applied here.

5. Other bounded attacks: interrupted machine-phase cleanup preserved unexecuted cases and exact-owned mutation guards; the second new case passed. The six existing runner cases also passed, including both agent inventories, missing manual evidence, receipt loss, name reuse, stale attestations, changed binary/selection, private checkpoint refusal, output flood and bounded PTY interruption. Source review of metadata avoids terminal output requests and raw diagnostic serialization; no additional measured finding is claimed. Documentation distinguishes unreleased behavior, historical0.1.4 evidence and synthetic versus authenticated qualification; no publication or live authentication was attempted.

6. Outside paths retained or used for writes, all explicit:

- `$HOME/.cache/mantle-reliability/agent-acceptance/adversary/pass1/`: this full report, raw isolated/package/check logs, direct `.exit` files, tests patch, status and handoff logs.
- `$HOME/.cache/mantle-reliability/agent-acceptance-target/`: assigned Cargo build/test outputs; retained for the implementor.
- `$HOME/.cache/mantle-reliability/agent-acceptance-tmp/`: assigned compiler/test fixture temporary directories, copied fixture CLIs, synthetic state and PTYs. Normal test TempDir guards retire their own files; no separate cleanup was performed.
- `/dev/shm/mantle-reliability-compiler-cache/`: existing shared bounded1GiB sccache storage on port43179; no server restart or cache deletion.
- `$HOME/.cache/mantle-reliability/integration-tmp/`: the existing sccache server's stable temporary root; not changed to the unit TMPDIR.
- `$HOME/.local/state/worktree/registry.sqlite3`: managed CLI lease metadata for `codex-agent-acceptance-adversary`; only that lease is ended on handoff.

The tree, target and logs remain for the coordinator/implementor. The two test paths are uncommitted; no implementation changes, AEP writes, commits, pushes or live product operations occurred. No source process is left running on return.

Supplementary checks after the package run: `cargo clippy --locked -p mantle-acceptance --all-targets -- -D warnings` exit0; `cargo fmt --check` exit0; `git diff --check` exit0. Raw outputs and direct exits are retained. No additional test assertion was changed after execution (only rustfmt whitespace).

```findings
- file: crates/mantle-acceptance/src/runner.rs
  line: 458
  category: contract-drift
  severity: blocker
  verdict: CONFIRMED
  origin: undecided
  message: Successful cleanup overwrites a previously failed destroy qualification case, causing cleanup and offline report to return success instead of preserving the measured failure.
```
