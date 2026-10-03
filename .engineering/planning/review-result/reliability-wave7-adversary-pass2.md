---
format: aep.planning-md/3
id: review-result:reliability-wave7-adversary-pass2
kind: review-result
status: active
title: Agent acceptance final adversary pass
relations:
- reviews: story:repeatable-agent-acceptance
revision: 1
---
unit: story:repeatable-agent-acceptance at f4bf80579e8daba5ab9a93d281d1ed5a8acab6a2, adversary pass2
verdict: nothing found
cases: executed 8→9 acceptance runner cases, red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: six path roots, detailed in part 6
needs-coordinator: record final review and retain the tests-only addition

```text
 crates/mantle-acceptance/tests/runner.rs | 52 ++++++++++++++++++++++++++++++++
 1 file changed, 52 insertions(+)
```

1. This second and final bounded attack covers correction `f4bf80579e8daba5ab9a93d281d1ed5a8acab6a2`. Read the correction diff/report/status, previous review and retained tests before adding the new case. The original two adversary cases and fixture remain unchanged from committed `a4f4ee33bf6b2fb577b66e2bfd8933e42eff3d62`. Only the new test above was added, without editing runtime, specification, generated files, docs or AEP. It remains uncommitted. No live worker/provider/session/auth operation occurred.

2. Added `adversary_cleanup_retries_preserve_original_failure_evidence_and_remove_resources` at `crates/mantle-acceptance/tests/runner.rs:459` before any test execution. It drives actual runner CLI processes using the existing controlled installed-CLI fixture and Claude agent selection. After the final qualification destroy fails, it captures the checkpoint's complete evidence array. A failed cleanup retry and a later successful cleanup must each preserve every original evidence field, including the failed case's phase, exec, origin and timestamp, and the earlier operator attestations. The test separately checks that successful cleanup actually reaches STOPPED and that offline report retains the failure. No checkpoint contents are hand-edited. The isolated case passed, exit0.

Cargo environment for the following commands:

```console
export PATH="$HOME/.cache/aap-ess-upgrade/ess-0.50/bin:$PATH"
export TMPDIR="$HOME/.cache/mantle-reliability/agent-acceptance-tmp"
export CARGO_TARGET_DIR="$HOME/.cache/mantle-reliability/agent-acceptance-target"
export CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 RUST_TEST_THREADS=1
export RUSTC_WRAPPER=/usr/bin/sccache SCCACHE_SERVER_PORT=43179
export SCCACHE_DIR=/dev/shm/mantle-reliability-compiler-cache SCCACHE_CACHE_SIZE=1G SCCACHE_IDLE_TIMEOUT=0
```

Raw output is retained unchanged in private scratch. Portable output below replaces only the personal home prefix with `$HOME`.

```console
cargo test --locked -p mantle-acceptance --test runner adversary_cleanup_retries_preserve_original_failure_evidence_and_remove_resources -- --exact
```

```text
   Compiling mantle-acceptance v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-agent-acceptance/crates/mantle-acceptance)
    Finished `test` profile [unoptimized] target(s) in 0.54s
     Running tests/runner.rs ($HOME/.cache/mantle-reliability/agent-acceptance-target/debug/deps/runner-70ad98faf127346c)

running 1 test
test adversary_cleanup_retries_preserve_original_failure_evidence_and_remove_resources ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 22.73s

```

3. The relevant package suite ran only after the new case passed alone. The baseline8 is the implementor correction handoff count, not a pre-attack suite execution. The actual runner lane executes9 cases, all passed; library/main/doc lanes remain0. Direct package exit0. No broader package or conformance totals are inferred from this bounded rerun.

```console
cargo test --locked -p mantle-acceptance
```

```text
    Finished `test` profile [unoptimized] target(s) in 0.11s
     Running unittests src/lib.rs ($HOME/.cache/mantle-reliability/agent-acceptance-target/debug/deps/mantle_acceptance-95131a4bc350eb70)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs ($HOME/.cache/mantle-reliability/agent-acceptance-target/debug/deps/mantle_acceptance-ad97a0714945412c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/runner.rs ($HOME/.cache/mantle-reliability/agent-acceptance-target/debug/deps/runner-70ad98faf127346c)

running 9 tests
test adversary_cleanup_preserves_failed_final_qualification_case ... ok
test adversary_cleanup_retries_preserve_original_failure_evidence_and_remove_resources ... ok
test adversary_interrupted_machine_phase_cleanup_keeps_incomplete_and_exact_ownership ... ok
test both_agents_use_actual_cli_pty_and_same_inventory_without_raw_reports ... ok
test lost_receipt_never_authorizes_cleanup_and_name_reuse_refuses ... ok
test output_flood_fails_and_manual_abort_does_not_fabricate_completion ... ok
test pty_timeout_retires_descendants_and_interruption_retains_unfinished_evidence ... ok
test stale_attestation_selection_binary_and_public_checkpoint_refuse ... ok
test unverified_release_and_replaced_selected_executable_refuse_before_mutation ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 129.67s

   Doc-tests mantle_acceptance

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

4. No findings in this second bounded attack. The prior runner.rs:458 finding is not carried: its original assertion now passes, and the new case also preserves the entire original evidence record through failed and successful cleanup retries. The original pass1 report remains unchanged at `$HOME/.cache/mantle-reliability/agent-acceptance/adversary/pass1/review.md`; the coordinator owns recording its correction outcome and source-based introduced attribution.

5. Could not break failure-evidence immutability while still performing successful resource cleanup; the new case executes the public runner CLI with the existing controlled fixture. Supported incomplete/manual-abort cleanup, interrupted ownership refusal and both-agent normal progression remain covered by the unchanged passing cases. This is synthetic runner testing, not live agent qualification or an independent-verifier claim.

6. Outside paths retained or used for writes:

- `$HOME/.cache/mantle-reliability/agent-acceptance/adversary/pass2/`: this report, raw isolated/package/check logs, direct `.exit` files, status ledger, tests patch and lease-end log.
- `$HOME/.cache/mantle-reliability/agent-acceptance-target/`: assigned Cargo outputs.
- `$HOME/.cache/mantle-reliability/agent-acceptance-tmp/`: compiler and controlled CLI test temporary files. Normal test guards retire their own temporary directories; no independent cleanup was performed.
- `/dev/shm/mantle-reliability-compiler-cache/`: existing bounded1GiB compiler cache on port43179; no server restart or cache deletion.
- `$HOME/.cache/mantle-reliability/integration-tmp/`: existing compiler server's stable temporary root; unchanged.
- `$HOME/.local/state/worktree/registry.sqlite3`: CLI-managed lease metadata for `codex-agent-acceptance-adversary-pass2`; only this lease is ended on handoff.

The source tree and target remain for the coordinator. The new test is uncommitted; no other source changed. All commands end and the own lease is released before return.

Supplementary direct checks: `cargo clippy --locked -p mantle-acceptance --all-targets -- -D warnings` exit0; `cargo fmt --check` exit0; `git diff --check` exit0. Raw outputs and direct statuses are retained beside this report.

```findings
[]
```
