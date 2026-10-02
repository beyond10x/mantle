---
format: aep.planning-md/3
id: review-result:codex-activation-adversary-1
kind: review-result
status: active
title: Codex existing transport independent request review
relations:
- reviews: story:codex-existing-transport
revision: 1
---
unit: Codex capture gate removal; source commit 050c1485b165f066678cac1dace4a00522d17949, plus docs 91722e2
verdict: nothing found
cases: executed 44→1 (different selections; see scope), red 0 product defects
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: 6 scratch files and the assigned build target
needs-coordinator: full workspace gate and source publication; live authentication remains unverified

1. Change boundary

My contribution is exclusively the appended `#[cfg(test)] mod adversary_activation_wire` in `crates/mantle/src/app/session.rs`. No production implementation, existing test, planning artifact or documentation was changed by this adversary. The shared tree's total diff also contains the implementor's pre-existing source/spec/generated edits; it is not a claim that I authored those paths. The added module's diff stat is:

```text
crates/mantle/src/app/session.rs | 162 lines, test-only module
```

The implementor reported 44 top-level Rust cases before this contribution. Per the coordinator's explicit bounded-review instruction, I ran only the one added case, not the package suite again; 44→1 is intentionally not a comparable suite-growth claim. The coordinator owns the complete suite after merge.

2. Added case and first execution

`app::session::adversary_activation_wire::adversary_codex_dispatches_confined_secretless_command_and_pty` exercises the actual pinned SDK against a bounded local Unix HTTP listener. Both GET setup requests and both actual POST mutation requests must arrive. The assertion inspects the wire bodies for workspace confinement, the read-only toolchain root, nonzero output/memory/process/lease limits, empty inherited environment and absent credential slots. It checks the Codex executable, private home and diagnostic recording settings, and the isolated PTY request with bounded input/frame/queue resources.

The fixture deliberately returns an invalid result after capturing each POST. The expected SDK protocol errors are not a production execution success claim; positive assertions on both recorded request bodies establish dispatch.

The first run failed because my new test guessed `sandbox.required` instead of the pinned wire's serde-renamed `sandbox.require`. Reading `substrate-wire::ConfinementRequest` and `ExecStartInput` also showed that empty secret slots are omitted, not encoded as an empty list. I corrected only those two assertions in the new test. This was a fixture error, not a product finding, and is not counted as a confirmed red case.

First-run assertion output (verbatim excerpt):

```text
assertion `left == right` failed
  left: Null
 right: true
```

The complete untouched first-run output and exit 101 remain in `~/.cache/mantle-wave8/scratch-adversary/wire-test.log` and `.exit`.

3. Focused corrected check

Command:

```text
CARGO_TARGET_DIR=/dev/shm/mantle-wave8-target-activation CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 cargo test -p mantle --bin mantle --locked app::session::adversary_activation_wire::adversary_codex_dispatches_confined_secretless_command_and_pty -- --exact --nocapture
```

Output follows; personal home prefixes are abbreviated to `~` in this report only. Raw logs are unchanged.
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave8-codex-activation/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 10.30s
     Running unittests src/main.rs (/dev/shm/mantle-wave8-target-activation/debug/deps/mantle-277e3ad59dda8f7b)

running 1 test
test app::session::adversary_activation_wire::adversary_codex_dispatches_confined_secretless_command_and_pty ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 30 filtered out; finished in 0.01s


Exit: 0. `cargo fmt --check` and `git diff --check` also exited 0 after the focused case. The full suite is deliberately delegated to the coordinator rather than repeated here.

4. Judgement findings

None.

5. Attacked scope and limits

- All five capture refusal call sites, the refusal function and the request flag are removed; searching production Rust and CLI tests found no residual identifier or refusal message.
- Actual Codex command and PTY requests reach the pinned SDK's HTTP mutation boundary with confinement and resource bounds preserved.
- The existing CLI positive-control test still requires that selecting Claude actually invokes its configured credential command, while Codex reaches the ordinary missing-worker prerequisite without invoking it; the implementor's package run covers this existing case.
- Read docs commit 91722e2 across README, design, qualification evidence, example and website. It explicitly discloses the existing Substrate capture behavior, workspace auth lifetime and unverified authentication/model/refresh acceptance.
- This review does not verify a live Substrate daemon, applied confinement, device login, model/tool turns, token refresh or complete terminal lifecycle. The requests retain the same SDK confinement fields; actual application is a separate observation.

6. Outside-worktree paths and handoff

Portable paths below use `~` for the operator's home:

- `~/.cache/mantle-wave8/scratch-adversary/wire-test.rs` — original throwaway test draft; final committed module is authoritative.
- `~/.cache/mantle-wave8/scratch-adversary/wire-test.log` — untouched first run, fixture assertion error.
- `~/.cache/mantle-wave8/scratch-adversary/wire-test.exit` — 101.
- `~/.cache/mantle-wave8/scratch-adversary/wire-test-corrected.log` — untouched green run.
- `~/.cache/mantle-wave8/scratch-adversary/wire-test-corrected.exit` — 0.
- `~/.cache/mantle-wave8/scratch-adversary/report.md` — this report.
- `/dev/shm/mantle-wave8-target-activation` — assigned compiler target, used only after implementor explicitly yielded it; yielded back after the focused test. Coordinator owns cleanup.

The adversary session lease is released before returning. No cleanup, commit, AEP write or publication performed by this role.

```findings
[]
```
