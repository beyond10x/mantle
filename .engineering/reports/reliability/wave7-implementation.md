unit:                   story:repeatable-agent-acceptance — Installed CLI acceptance
verdict:                green
cases:                  executed 133→143 package tests; native CLI372→381, red0
origin:                 n/a
wrote-outside-worktree: assigned scratch/target/TMPDIR, shared compiler cache, managed lease metadata (listed below)
needs-coordinator:      no

Source commit: `3ce4d28037a4c6d3120b5594f463cd8924bf43bc`, both identities `b10x-bot[bot]`. Clean source tree. All final required checks exit0 (direct ledger `status.json`). Documentation commit `211332ba6f6b4f6d80d3881f7ce238cad21196e1` belongs to the delegate/root and is not merged by this implementor.

1. `story:repeatable-agent-acceptance`: installed-CLI runner, bounded metadata/creation ownership, distinct manual evidence, owned lifecycle cleanup, and GNU delivery. Runtime interfaces are frozen; README, website and spec/README remain the separate delegate's exclusive scope.

2. Exact source diff is retained in `diff-stat.txt`. Implemented paths matched the final narrowed scope; initial inferred scopes crates/mantle-conformance, docs/evidence and crates/mantle/tests/codex_preflight.rs were not changed and the coordinator removed those unused scopes while preserving their history; coordinator additionally scoped the existing `mantle-worker/src/lib.rs` ownership seam for supplied PTY descriptors, completion observation and SIGWINCH. The public CLI uses clap derive. Controlled Rust protocol doubles are test fixtures only. No AEP store edits.

3. Red evidence: before implementation, the standalone Rust test invoked the retained baseline executable and printed `test result: FAILED. 0 passed; 1 failed`, exit101 (`red-metadata.log`). The unchanged final test was also paired against baseline/current CLI (`claim-base.log`, exit101; `claim-treatment.log`, exit0). The full baseline executable is coordinator-owned at `$HOME/.cache/mantle-reliability/agent-acceptance/baseline/mantle`, SHA256 f787b7db40a264a870c9284d40a72d8342971a0a102fff3e6f7c1a9fb1e6307b. It was not modified.

Reproduce the paired claim using the same installed test executable:

```console
MANTLE_METADATA_BASE_CLI="$HOME/.cache/mantle-reliability/agent-acceptance/baseline/mantle" "$HOME/.cache/mantle-reliability/agent-acceptance-target/debug/deps/session_metadata-b543ba0061fc8741" json_selection_failure_is_structured_before_initialization --exact
env -u MANTLE_METADATA_BASE_CLI "$HOME/.cache/mantle-reliability/agent-acceptance-target/debug/deps/session_metadata-b543ba0061fc8741" json_selection_failure_is_structured_before_initialization --exact
```

For a rebuild, the same case is `cargo test --locked -p mantle --test session_metadata json_selection_failure_is_structured_before_initialization -- --exact`, with/without the baseline override.

The runner's first complete controlled pass was 3 passed/1 failed (`runner-first.log`, exit101): unresolved creation performed a read-only list call before refusing cleanup. Ownership now refuses before any CLI call; the exact assertion remains. The metadata HTTP fixture initially lacked the pinned SDK's required ledger facts; that setup failure and corrected real wire run are retained separately (`metadata-2.log`, `metadata-3.log`). Initial compilation errors are retained, not represented as semantic reds. `packages-2.log` passed runtime/CLI/release lanes then reported generated-model drift after final ESS DTO additions; ESS regenerated the owned outputs before the final gate. No checks or assertions were weakened.

4. Commands and raw complete outputs live in the assigned scratch root. Each completed check has a direct `.exit` file. Environment for Cargo: pinned ESS0.50 PATH; target/TMPDIR below; jobs2, dev debug0, tests serial; task sccache43179, bounded1G, shared cache below. The exact final commands are:

```console
cargo test --locked -p mantle -p mantle-acceptance -p mantle-worker -p mantle-release -p mantle-artifact
cargo clippy --locked -p mantle -p mantle-acceptance -p mantle-worker -p mantle-release -p mantle-artifact --all-targets -- -D warnings
cargo fmt --check
cargo test --locked -p mantle-worker generated_model_has_no_drift
cargo run --locked -p mantle-release -- notices --source . --check
rustfmt --edition 2024 --check crates/mantle/tests/support/metadata_ssh.rs crates/mantle-acceptance/tests/support/cli.rs
ess specify validate --path spec
```

Final logs: `packages-final.log`, `clippy-final.log`, `fmt.log`, `drift.log`, `notices-check.log`, `fixture-fmt.log`, `spec-final.log`. `notices.log` records actual regeneration. `model.log` records exact-base reference generation, ownership adoption with owner model-types, and final ESS generation using the four established session roots. Model bytes were not hand-edited.

Baseline per-lane summaries are in the coordinator's `$HOME/.cache/mantle-reliability/integration/wave6-gate.log` (exit0). Mantle main38, preflight4, command1, doctor4, profiles6, retained3, worker-upgrade3 and qualification10 remain unchanged; metadata is new4. Acceptance is a new crate/lane6 (absent on base, not an invented baseline test run). Release16 and worker31+16+1 remain unchanged; artifact/lib/doc empty lanes remain0. These unchanged lanes do not hide the new tests: the actual metadata/runner executables print4 and6 respectively. Package aggregate133→143. Native CLI372→381, complete inventory221 authored+160 generated, zero skipped; global specification authored307→315. The other components' full reconciler run belongs to root and was not rerun or claimed here. `spec/conformance-baseline.json` retains every prior obligation and adds the9 new command/authored identities with the measured CLI381 floor.

5. No live worker/session/provider or agent-auth operation, authenticated qualification, push, PR, tag, release or site deployment was performed. Bot source commit is the authorized GitHub operation, distinct from product operations. Reports contain fixed evidence fields only; startup/PTY output is transient, bounded and discarded. Manual login/model/visual assertions remain labelled operator evidence bound to the original exec. The subsequent restart verifies a fresh exec and retained synthetic marker; it does not claim conversation restoration or infer authentication. Unknown source provenance is allowed explicitly; supplied provenance must verify sibling archives and selected executable bytes. Interrupted creation without a receipt remains unresolved; interrupted machine phases retain incomplete evidence and allow explicit owned cleanup, not inferred recovery.

6. Outside paths written/retained (root owns cleanup):

- `$HOME/.cache/mantle-reliability/agent-acceptance/`: all logs/status/diff/report files, standalone red-test executable, exact-base `base/spec` and `base-model` references, conformance baseline staging copy. The nested `baseline/mantle` is root's unchanged input.
- `$HOME/.cache/mantle-reliability/agent-acceptance-target/`: isolated Cargo target, retained for reviewer.
- `$HOME/.cache/mantle-reliability/agent-acceptance-tmp/`: compiler/bot/test scratch, including controlled Rust fixture processes. Test-owned temporary dirs clean up normally; no managed-tree/target deletion was performed.
- `/dev/shm/mantle-reliability-compiler-cache/`: existing shared task compiler cache, bounded1G, port43179, server stable TMPDIR `$HOME/.cache/mantle-reliability/integration-tmp`; server was not restarted.
- Managed worktree lease records for `codex-agent-acceptance` were acquired/renewed and are ended at handoff. Existing SSH code allocates short private `/tmp/mantle-*` sockets; their guards remove them.

External root inventory and sizes are retained in `outside-inventory.txt`; direct check outcomes in `status.json`. No files belonging to the documentation delegate were edited. No source/target process remains at handoff; root owns review, integration, full gate and final publication.

Verbatim paired baseline red:

```text

running 1 test
test json_selection_failure_is_structured_before_initialization ... FAILED

failures:

---- json_selection_failure_is_structured_before_initialization stdout ----

thread 'json_selection_failure_is_structured_before_initialization' (3824047) panicked at crates/mantle/tests/session_metadata.rs:16:5:
missing bounded JSON response: 
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    json_selection_failure_is_structured_before_initialization

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.03s


```

Verbatim paired treatment:

```text

running 1 test
test json_selection_failure_is_structured_before_initialization ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.01s


```

Verbatim final runner summaries (complete output in packages-final.log):

```text
test result: ok. 38 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 84.38s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.46s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.37s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.01s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.41s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.81s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.38s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 19.65s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.40s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 82.43s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 89.25s
test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 14.11s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 238.03s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
