unit: story:controlled-worker-upgrades — ac534be9bfe3ca4a274fe5d53b06ca3822a18ba5
verdict: green
cases: worker 29→42; Mantle 61→64; release 15→15; CLI native 263→273; decisive reds retained
origin: n/a
wrote-outside-worktree: assigned scratch, unique target, assigned TMPDIR and task-owned sccache; inventory below
needs-coordinator: no

## 1. Unit and boundaries

Implements explicit offline helper upgrade: read-only old-worker assessment and verified,
locked complete-directory activation with crash inspection and honest rollback outcomes.
The coordinator owns AEP, publication, independent adversary review and the integrated gate.
No live worker, provider, session, credentials, systemd service or GitHub operation was contacted.

Managed tree `$HOME/.local/state/worktree/trees/b10x/mantle/mantle-worker-upgrades`, branch
`unit/mantle-worker-upgrades`, exact base `c63d5a6ceb4c2817f01586a00cf75390c69a69ce`.
Lease `codex-worker-upgrades` is released at handoff. Assigned external build paths override
the implementor skill's default in-tree target as explicitly required by the repository/brief.

Scope verification: existing worker/SSH/runtime/rendering paths were read. CLI app modules are
declared in `main.rs`; the initially inferred `app.rs` does not exist and was removed by the
coordinator. Added `app/worker_upgrade.rs`, worker maintenance/upgrade modules, shared fixture,
CLI protocol double and native adapter are now typed coordinator scopes. No AEP edits.

Source inspection corrected the proposed platform seam: nix 0.30.1 `src/fcntl.rs:475` gates
`renameat2` on GNU. The implementation uses safe `rustix::fs::renameat_with(EXCHANGE)` instead,
with no sequential fallback. A musl target check is recorded below.

Coordinator source review identified that systemd's masked FragmentPath is its effective link
source, not `/dev/null` (systemd v255 basic/unit-file.c:304–310 and core/load-fragment.c:5672).
The retained isolated regression failed before fixing the comparator. All mask-source paths
now require the actual root-owned link, exact load/file states, reload completion and supported
unit search layout. `show --all` and `list-jobs --full` make empty/complete facts explicit.

The old-worker CLI fixture replaces only OpenSSH command IO with a Rust protocol executable;
production CLI parsing, configuration/readonly SQLite, SSH argv, metadata parsing and maintenance
decisions run normally. Apply transports real verified fixture archives and hands its worker
boundary to the production transaction using actual filesystem identities, static ELF version
processes, host locking and atomic exchange. It does not run a real remote worker.

Check reports current only for an exact selected manifest and unchanged installed inventory.
An unfinished journal suppresses current even after a marker write. Recovery inspects actual
directory orientation; recovering another requested source returns non-success rather than
claiming the requested candidate was applied. Prior directories and journals are retained.

## 2. Source shape

`source-diff-stat.txt` contains the complete actual staged diff summary:
`34 files changed, 2913 insertions(+), 108 deletions(-)`.
Bot commit and both identities are verified in `commit.log` and `identity.txt`.
The source tree is clean. No unit push, PR or release was performed.

## 3. Meaningful red evidence

All commands use the environment listed below. Raw logs are retained verbatim, including setup
failures, and are not rewritten into a successful history.

- `red-cli.log`: actual worker CLI before implementation refused `upgrade-check`; 0 passed,
  1 failed, exit 101.
- `red-transaction.log`: explicit transaction scaffold refused an eligible local fixture;
  1 passed, 1 failed, exit 101. This scaffold is not falsely attributed to the base commit.
- `red-mask-source.log`: real effective FragmentPath fixture failed the old `/dev/null`
  comparator; 0 passed, 1 failed, 7 filtered, exit 101.
- `fixture-setup-failure.log`: initial laptop fixture compilation lacked tar/nix dependencies.
  This is a setup failure, NOT the decisive baseline red.
- `red-old-cli.log`: after fixing the fixture, the actual retained source-equivalent baseline
  CLI refused `worker upgrade`; 0 passed, 1 failed, exit 101.
- `red-pending-marker.log`: process interruption after marker publication made check falsely
  claim current despite unfinished journal; 0 passed, 1 failed, 10 filtered, exit 101.

The retained baseline CLI is
`$HOME/.cache/mantle-reliability/prebuilt-release/install-107f9c8bbdc9/bin/mantle`, SHA-256
`178a5ff40c6991d3253aab11bae36223e0427e5f00242384151276e0d7d5ec1f`.
Root independently verified empty source diff from 107f9c8 to this unit's base for Cargo files,
Mantle, mantle-worker and generated worker model. Its production build/install provenance is
retained in the preceding unit's scratch; the installation was not modified.

Reproduce the same-input causal CLI case from this tree using the environment below:

```console
MANTLE_UPGRADE_BASE_CLI="$HOME/.cache/mantle-reliability/prebuilt-release/install-107f9c8bbdc9/bin/mantle" cargo test --locked -p mantle --test worker_upgrade actual_old_worker_check_observes_offline_facts_without_upload_or_state_writes -- --exact
cargo test --locked -p mantle --test worker_upgrade actual_old_worker_check_observes_offline_facts_without_upload_or_state_writes -- --exact
```

The test builds its Rust SSH double and scratch SQLite/config/unit/cgroup tree automatically.
It proves eligible offline observations and nonzero populated-cgroup refusal, exact retained
local DB/known-hosts/bin bytes, and no helper upload or state initialization.

## 4. Green evidence

The complete unedited command outputs are the `.log` files below. `status-ledger.json` records
every exit observed directly through the execution tool, with session identifiers where provided.
Intermediate green logs remain separate:
`transaction-first.log`, `transaction-second.log`, `transaction-third.log`, `app-check.log`,
`green-cli.log`, `transport.log`, `green-pending-marker.log`, `model-generate.log`,
`spec-final.log`. `transaction-boundaries.log` preserves the earlier not-found classification
failure before correction. Final package execution includes these cases plus existing regressions.

Final command:

```console
cargo test --locked -p mantle-worker -p mantle -p mantle-artifact -p mantle-release
```

`packages.log`, exit 0. Top-level summaries, verbatim in execution order (nested child-test
outputs are present in the raw file and are not counted again):

```text
test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 57.37s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.45s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.69s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.04s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 14.33s
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 69.93s
test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.43s
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 107.86s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- Worker package: executed 29 → 42, exit 0. Base worker was run locally, raw
  `baseline-worker.log`; current library 30 + filesystem transaction 11 + actual CLI 1.
- Mantle package: executed 61 → 64, exit 0. Base 61 is the coordinator's preceding integrated
  gate record, not an additional baseline run claimed here. Actual transport lane adds 3 cases.
- Release package: executed 15 → 15, exit 0. No new release tests were added; all existing
  source-export, inventory, installer and publication regressions ran. Artifact unit/doc lanes
  remain 0 → 0; its production verifier is exercised by release and upgrade process fixtures.
- CLI native ESS: executed 263 → 273, exit 0, zero failed/error/skipped/unsupported.
  `cli-native-report.json` is copied directly from `.engineering/drafts/mantle-cli-report.json`.
  The increase is nine authored upgrade scenarios and one generated UpgradeWorker example.
  The full native reconciler remains the coordinator's integrated gate, not claimed here.
- Authored specification: 268 → 277, exit 0; `spec-final.log`:
  `mantle v1 — 8 file(s), 277 scenario(s), valid`.
- Explicit drift: `cargo test --locked -p mantle-worker tests::generated_model_has_no_drift -- --exact`,
  exit 0; `model-drift.log`: `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 29 filtered out; finished in 0.02s`.
- `cargo clippy --locked -p mantle-worker -p mantle -p mantle-artifact -p mantle-release --all-targets -- -D warnings`,
  exit 0; `clippy.log`: `Finished` in 27.61s, no warnings.
- `cargo fmt --check`, exit 0, `fmt.log` empty; standalone Rust SSH fixture rustfmt check also
  exits 0 with empty `fixture-fmt.log`.
- `cargo check --locked -p mantle-worker --target x86_64-unknown-linux-musl`, exit 0,
  `musl-check.log`: `Finished` in 21.31s. This is a musl target compile check, not a newly
  published production bundle. Local transaction tests execute real musl fixture ELFs.
- `cargo run --locked -p mantle-release -- notices --source .`, then the same with `--check`,
  both exit 0, `notices-generate.log` and `notices-check.log`. Dependency edges changed but the
  complete locked workspace notice inventory is byte-identical, so no notices diff is fabricated.
- `cargo run --locked -p mantle-docs -- check`, exit 0: `Mantle documentation: anchors, routes and document valid`.
- `cargo run --locked -p mantle-docs -- build --out website/build --commit ac534be9bfe3ca4a274fe5d53b06ca3822a18ba5`,
  exit 0, `docs-build.log`. The output manifest is tied to this actual source commit.

Final same-input claim runs use the exact commands in section 3 and final committed test files:

```text
claim-base.log, exit 101:
error: unrecognized subcommand 'upgrade'
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2 filtered out; finished in 1.74s

claim-treatment.log, exit 0:
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 5.43s
```

## 5. Deliberate boundaries

This is helper replacement under an administrator-established exclusive offline maintenance
window. No automatic stop, drain, mask, start, restart, session signalling or Substrate migration.
Older already-running clients and other root administrators must be excluded by the operator;
the implementation does not claim retrospective lock cooperation or atomicity against root.
Unknown layouts refuse. A disconnected SSH session can leave its privately owned temporary
delivery directory; documentation distinguishes it from an installed generation.

All user instructions are development/unreleased: release 0.1.4 lacks these commands.
The source website is validated here; no website publication or actual release is claimed.
Fresh provisioning is documented with verified prebuilt static helpers and --binaries.
Live authenticated model turns remain the responsibility of the later acceptance unit.

## 6. External state and environment

Retained outputs:

- `$HOME/.cache/mantle-reliability/worker-upgrades/`: raw logs, exact-base spec export,
  fresh baseline generated reference, design notes, report, final inventory and diff evidence.
- `$HOME/.cache/mantle-reliability/worker-upgrades-target/`: unique cargo build/test target.
- `/dev/shm/mantle-worker-upgrades-tmp/`: assigned compiler/test temporary data.
- `/dev/shm/mantle-reliability-compiler-cache/`: shared task-owned bounded 1 GiB compiler cache.
- Managed worktree operational lease/ESS ownership metadata; normal cargo registry/cache metadata
  needed by the existing dependency graph. No other worktree, installation or daemon was mutated.

```console
export PATH="$HOME/.cache/aap-ess-upgrade/ess-0.50/bin:$PATH"
export CARGO_TARGET_DIR="$HOME/.cache/mantle-reliability/worker-upgrades-target"
export TMPDIR=/dev/shm/mantle-worker-upgrades-tmp
export CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 RUST_TEST_THREADS=1
export RUSTC_WRAPPER=/usr/bin/sccache SCCACHE_SERVER_PORT=43179
export SCCACHE_DIR=/dev/shm/mantle-reliability-compiler-cache SCCACHE_CACHE_SIZE=1G SCCACHE_IDLE_TIMEOUT=0
```

No target or managed tree is deleted on handoff. Coordinator owns cleanup after review.

Observed retained sizes before handoff: target 3.1 GiB; scratch approximately 1.6 MiB;
TMPDIR 44 MiB; shared bounded cache approximately 1.1 GiB including operational metadata.
`external-files.txt` enumerates scratch evidence and baseline-export files. The worktree also
retains ignored native drafts and `website/build` for review. All build/test commands have ended.
