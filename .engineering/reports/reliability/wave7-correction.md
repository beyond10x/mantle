unit:                   story:repeatable-agent-acceptance — pass1 correction
verdict:                green
cases:                  executed8→8, red1→0
origin:                 n/a
wrote-outside-worktree: assigned correction scratch, target/TMPDIR, shared cache and own managed lease metadata
needs-coordinator:      no

Bot source commit `f4bf80579e8daba5ab9a93d281d1ed5a8acab6a2`; both author and committer `b10x-bot[bot]`. Clean source tree. Final isolated/package/Clippy/fmt/diff-check and commit exits are0. No build/test/bot process remains; own correction lease ended at handoff.

1. Corrects the retained pass1 finding: successful resource cleanup must not erase a failed qualification case. The original review remains unchanged; root classified the new implementation as introduced.

2. `git diff --stat`:

```text
 crates/mantle-acceptance/src/runner.rs | 5 +++++
 1 file changed, 5 insertions(+)
```

3. Re-ran the original regression before editing and measured exit101. After the five-line correction the same test measured exit0. Commands and complete raw outputs are retained in `red.log`/`red.exit` and `isolated.log`/`isolated.exit`:

```console
cargo test --locked -p mantle-acceptance --test runner adversary_cleanup_preserves_failed_final_qualification_case -- --exact
```

4. The complete affected package and checks:

```console
cargo test --locked -p mantle-acceptance
cargo clippy --locked -p mantle-acceptance --all-targets -- -D warnings
cargo fmt --check
git diff --check
```

Raw outputs and direct exits: `package.log`, `clippy.log`, `fmt.log`, `diff-check.log` and corresponding `.exit` files; combined ledger `status.json`. Acceptance runner executed8→8; prior measured7passed/1failed becomes8passed/0failed. Empty library/main/doc lanes remain0. No test assertion, fixture, specification, documentation or interface changed. The old comparison uses the preserved adversary package run; this correction independently re-executed its isolated red before editing.

The class is any later observation replacing already-failed qualification evidence. The shared `observe` method now leaves a failed row unchanged, including its timestamp, phase, exec and evidence origin. The guard covers every inventory member: create, login, model-tool, visual, status-exec, detach-reconnect, resize-interrupt, transport-loss, retain-restart and destroy. Cleanup still calls guarded destroy and verifies STOPPED before computing the unchanged failure aggregate. Existing manual-abort/interrupted-cleanup assertions remain present and execute in the package suite. The formal test paths are byte-identical to `a4f4ee33bf6b2fb577b66e2bfd8933e42eff3d62`.

5. No broader runtime surface changed, so unrelated package/model/release suites were not repeated. Root owns the second review and integrated gate. No live worker/provider/session/auth operation, AEP edit, push or release occurred. The bot source commit is an authorized GitHub operation, distinct from product operations.

6. Outside writes remain only assigned roots: `$HOME/.cache/mantle-reliability/agent-acceptance/correction1/` (reports, logs, exits and inventory), `$HOME/.cache/mantle-reliability/agent-acceptance-target/`, `$HOME/.cache/mantle-reliability/agent-acceptance-tmp/`, existing shared `/dev/shm/mantle-reliability-compiler-cache/` on43179 with stable server TMPDIR `$HOME/.cache/mantle-reliability/integration-tmp/`, and managed lease records for `codex-agent-acceptance-correction`. No target/cache/server/tree cleanup or restart. All commands end and the own lease is released before handoff. Raw logs remain private; appended report excerpts preserve exact test-result lines and omit personal-path Running lines.

Verbatim result lines, red / isolated green / complete package:

```text
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 108.99s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 7 filtered out; finished in 22.04s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 23.02s

```
