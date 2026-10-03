unit: story:controlled-worker-upgrades — a7ac0ca5c46a72a91e834d18b5cb01962563b800
verdict: green
cases: worker 42→46; transaction 11→14; Mantle 64→64; native CLI 273→273; 4 reproduced red cases
origin: n/a (coordinator classified the recorded finding as introduced)
wrote-outside-worktree: assigned scratch, target, TMPDIR, task compiler cache and owned lease metadata
needs-coordinator: no

## 1. Recorded finding and correction

Read the unchanged `adversary/review.md` and retained +57-line test. The coordinator recorded it
as `review-result:reliability-wave5-adversary-pass1` before dispatching this correction.
No review text, original test assertion or AEP artifact was changed. This is implementation,
not an independent adversary approval.
The retained adversary function extracted from its original `case.patch` and the final test file
compares byte-for-byte (`cmp`, exit 0, `correction-assertions-unchanged.log`); both SHA-256 values
are `36eadfe6a0b0ee9360c8cea3b14e9773250e03d8f330394ad66cd77c9b05bdbb`.

The fix: completed journals are now observed without running transactional postcheck/rollback.
An existing agent entry must remain exact; the supported later first Codex installation may add
its validated relative stable link. Unexpected changed helpers, Claude bytes or extra entries
refuse without exchanging directories or rewriting the terminal journal. New bundle activation
captures the current supported inventory, so it preserves the later agent entry.

The class: transaction recovery owns inventory only until terminal completion. A historical
journal is not authority to roll back a later serialized writer; conversely an unfinished durable
transaction still owns its inventory after its process releases the OS lock.

Enumeration covered:

- Completed `applied`, `rolled-back` and `not-applied` journals: recovery is read-only, including
  when a postcheck-failure hook is supplied. Same selected bundle, new candidate and actual retained
  adversary sequence preserve the supported Codex link and Claude bytes.
- Local and remote check: completed inventory comparison admits only the validated later Codex
  link addition; recorded entries remain exact. The remote path still hashes actual helper files.
- Refusal paths: changed helper bytes, changed Claude bytes and an unsupported extra entry retain
  today's complete inventory and the original terminal journal without historical rollback.
- Pending `prepared`, `exchanged`, `recovery-required` journals: the actual Codex Installer now
  refuses under HostLock before fetching or creating agent paths, until recovery completes.
  A real pre-fix Installer call reached its fetch callback, demonstrating this adjacent ownership
  gap. The fixture uses structurally valid journal states; it is not a second live incident claim.
- Existing incomplete-transaction rollback/interruption and host-lock tests remain in the suite.

## 2. Source shape and identity

Five files: worker upgrade runtime, Installer guard, retained/new worker tests/shared observation
fixture, and three README lines explaining durable ownership. No dependency, ESS, generated-model
or website HTML changes. Final diff/identity/status retained separately below.
`correction-diff-stat.txt`: `5 files changed, 322 insertions(+), 3 deletions(-)`.
Correction committed at `a7ac0ca5c46a72a91e834d18b5cb01962563b800`; author and committer both
`b10x-bot[bot]`. `correction-identity.txt` preserves the actual output; clean
`correction-source-status.txt` is empty. `correction-source.patch` is the exact committed binary
diff from reviewed candidate `ac534be9bfe3ca4a274fe5d53b06ca3822a18ba5`.

## 3. Red runs

All commands use the unchanged assigned environment from `implementation-report.md`.

```console
cargo test --locked -p mantle-worker --test offline_upgrade adversary_completed_upgrade_retry_preserves_later_codex_installation -- --exact
```

`correction-red.log`, direct exit 101:

```text
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 9.14s
```

The failure is the retained assertion that retry removed the subsequently installed Codex link.

```console
cargo test --locked -p mantle-worker --test offline_upgrade terminal_journal
```

`correction-class-red.log`, direct exit 101:

```text
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 12 filtered out; finished in 13.83s
```

The named cases were added before runtime changes. One observed `rolled-back` instead of `current`
when inspecting a completed journal; the other observed silent restoration instead of refusal.

```console
cargo test --locked -p mantle-worker --lib upgrade::tests::pending_helper_transaction_excludes_codex_writer_before_fetch_or_agent_paths -- --exact
```

`correction-writer-red.log`, direct exit 101:

```text
unfinished helper transaction must exclude fetching
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 30 filtered out; finished in 0.00s
```

## 4. Green runs

Retained adversary case after the correction, same command as above:
`correction-case.log`, direct exit 0:

```text
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 7.62s
```

Worker package `cargo test --locked -p mantle-worker`, `correction-worker.log`, direct exit 0:

```text
test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.49s
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 169.22s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Worker: executed 42 → 46, exit 0. Transaction lane: original 11 → reviewer 12 (11 green/1 red)
→ corrected 14 green. Library 30 → 31; actual worker CLI 1 → 1. No case was removed or weakened.
`cargo fmt --check`: exit 0, empty `correction-fmt.log`.

Laptop package `cargo test --locked -p mantle`, `correction-mantle.log`, direct exit 0:

```text
test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 57.91s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.45s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.69s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.04s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 14.37s
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.33s
```

Mantle: executed 64 → 64, exit 0. No laptop cases added; unchanged existing actual SSH transport
and CLI tests run against the corrected worker library. Nested child summaries are not counted
twice. Native CLI: executed 273 → 273, exit 0, zero failed/skipped/unsupported/error; copied raw
`correction-cli-native.json`. No new ESS command or acceptance scenario was introduced by this
correction; existing nine upgrade cases and generated case still exercise the same contract.

```console
cargo clippy --locked -p mantle-worker -p mantle --all-targets -- -D warnings
cargo check --locked -p mantle-worker --target x86_64-unknown-linux-musl
```

Both direct exits 0. `correction-clippy.log`: Finished dev profile in 3.73s;
`correction-musl.log`: Finished dev profile in 0.86s. No warning suppression or target fallback.
Generated-model drift passed inside the worker package. Notices, dependency graph and ESS source
did not change, so their prior successful generation/validation is retained without a fabricated
regeneration or unnecessary package rerun.

Raw files are not rewritten;
`correction-status.json` will record direct observed exit statuses.

## 5. Deliberate boundaries

No live worker, service, provider or credential inspection. The authorized bot commit route did
call GitHub: its first attempt failed with `GitHub request failed; sensitive response withheld`,
exit 1; the source HEAD remained unchanged. The identical-route retry has its own retained log.
No personal client, alternative writer, push, PR, release or other external mutation was used.
The identical authorized bot-route retry succeeded, exit 0, and created the correction commit;
`correction-commit.log` and `correction-commit-retry.log` preserve both outcomes.
Unit 6 was not started. Root owns
the second independent adversary pass, integration gate and publication. The original implementation
report and adversary pass 1 report remain immutable evidence of the prior candidate.

## 6. External paths and handoff

All new evidence is under `$HOME/.cache/mantle-reliability/worker-upgrades/correction-*` and
enumerated by `correction-files.txt`. Build target remains
`$HOME/.cache/mantle-reliability/worker-upgrades-target`; TMPDIR remains
`/dev/shm/mantle-worker-upgrades-tmp`; shared task cache remains
`/dev/shm/mantle-reliability-compiler-cache`. Worktree CLI maintains only our own
`codex-worker-upgrades` lease in its registry. The unit tree is unchanged:
`$HOME/.local/state/worktree/trees/b10x/mantle/mantle-worker-upgrades`.

No output or managed tree is deleted. Observed retained sizes: target 3.1 GiB, entire unit scratch
1.8 MiB, TMPDIR 57 MiB. All command processes ended; own lease released with exit 0, recorded in
`correction-lease-end.log`. Documentation rebuilt against the correction's exact commit,
`correction-docs-build.log`, exit 0. Root owns second review and eventual cleanup.
