# Reliability wave invariants

The operator approved all five roadmap priorities and all implementation waves. Deliver all
work on `integration/mantle-reliability` with one PR at the end. No merge to main or release.
Read the repository AGENTS.md and the assigned AEP story before implementation. The coordinator
is the only AEP writer. Unit workers change source, tests, specifications and documentation only.

All committed executable code is Rust. Command lines use clap derive. Never commit shell or
Python programs. Keep credentials, personal paths and cloud identifiers out of source and logs.
Do not inspect real auth caches or operate existing live sessions, workers or VMs.

Use the assigned managed worktree, branch, scratch root and build directory. Acquire and renew
your own worktree lease; release only that lease when handing back. Never delete managed trees.
All commits use the bot through `b10x-gates bot` and the external policy. Never bypass a refusal.
No unit pushes, tags, PRs or releases; the coordinator publishes integrated commits.

Read AEP's `implementing/references/implementor.md` for implementation or `adversary.md` for
review. The host uses reusable general agent roles executing those procedures, not native named
plugin agents. Implementation establishes a meaningful red case first, then the smallest change.
Update ESS before the runtime behavior. An adversary may add tests but never edit implementation.
No planning changes by either role. Reports use portable `$HOME` paths, not personal absolute paths.

Use the pinned ESS 0.50 binary. Generated model output is owned: adopt a fresh exact-base reference
before regeneration, never hand-edit generated code. Run formatting without `--all`, package tests,
affected conformance, relevant clippy, explicit generated-model drift and documentation checks.
The full `task check` runs once on the integrated source after review. Record every exit code
directly; do not mask a gate with a successful pipeline tail. Test counts mean cases executed.

Builds use task-owned sccache server port 43179, directory
`/dev/shm/mantle-reliability-compiler-cache`, size 1G, wrapper `/usr/bin/sccache`, jobs 2, development
debug information disabled. Each worktree has a distinct target and temporary directory. Keep at
least 1 GiB free on root and 3 GiB on tmpfs. Do not stop other compiler-cache servers. Unset
`TASK_X_ENV_PRECEDENCE` when running Task so the assigned target remains in effect.

Keep detailed red/green outputs and the final handoff in assigned scratch. Return this header:

```text
unit: story identifier and source commit
verdict: green | red | blocked
cases: executed before→after, red count
origin: introduced count, pre-existing count, undecided count (implementation: n/a)
wrote-outside-worktree: exact retained paths | none
needs-coordinator: yes | no
```

Name source changes, tests and their own output, limitations, all external files, build sizes,
leases and cleanup status. An adversary closes with the required machine-readable findings block.
Report observed behavior honestly: process readiness is not authenticated model qualification.
Documentation describes development/unreleased behavior until an actual source release occurs.

After release tooling lands, dependency or workspace-member changes also regenerate notices with
`cargo run --locked -p mantle-release -- notices --source .`, then verify the same command with
`--check`. Keep the locked graph, complete notices and source-bound release workflow coherent.

Use RUST_TEST_THREADS=1 for independent fixtures; explicit concurrency tests keep their threads.
Set SCCACHE_IDLE_TIMEOUT=0 and keep the task server's temporary directory at the integration
path, never a unit path that will be retired. Global filesystem free space does not establish
per-user quota availability; preserve quota failures and rerun only after correcting allocation.
