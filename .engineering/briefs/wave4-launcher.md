# Wave 4 launcher unit

Follow aep:implementor procedure at ~/.codex/plugins/cache/b10x/aep/0.19.1/skills/implementing/references/implementor.md. Unit story:launcher-volatile-replay is active; read its complete body and exact scope. The Boolean model and two existing normalized argument expectations already appear in opening3f05cf06a7c12df69d6864b6849c325246c946bc; production code is unchanged and those expectations should now fail until implementation.

Work only in managed mantle-wave4-launcher: ~/.local/state/worktree/trees/b10x/mantle/mantle-wave4-launcher, branch impl/launcher-volatile-replay. Own lease codex-mantle-wave4-launcher-implementor. Scratch ~/.cache/mantle-wave4/scratch-launcher; build /dev/shm/mantle-wave4-target-launcher. Source/evidence always distinct. Acquire/heartbeat/release your own lease. No AEP writes, git staging/commits, external integrations, credentials, VM operations or other worktrees. Coordinator owns commits and store. All permanent runnable code is Rust, command lines clap derive; read AGENTS.md.

Implement LP01–LP06 exactly, with the smallest launcher-local --volatile-replay Boolean change. No transport/encryption/policy marker/framework. Keep default persistence. Stale entry refusal under the server lock before FIFOs/child dispatch must preserve entries/targets and cleanup lock properly. Native fixtures expose real observations and use bounded collectors/watchdogs; after forced server death clean the isolated fixture child explicitly. Require explicit observed no-canary diagnostics and no last-output, not a blanket private boolean. This does not prove Substrate/Codex privacy.

Test first: draft and validate named native ESS commands/scenarios before runtime changes; record a real red test against baseline, then green after smallest fix. You may add sources only within scoped paths; report exact extra paths if necessary. Existing generated worker-model artifacts must be regenerated with ESS0.50.0, never edited. Retain all prior218 authored/313 complete scenario obligations; only the two planned normalized objects gain false. If scenario count metadata needs updates, preserve every old name/assertion.

Build environment: CARGO_TARGET_DIR=/dev/shm/mantle-wave4-target-launcher CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0; installed sccache configured globally. Set no global HOME. Root reserve2GiB, tmpfs floor10GiB. No full-workspace gate: run fmt, package clippy/test for mantle-launch and relevant generated provenance test only. Full integration gate belongs to coordinator. Capture commands, each exit and raw logs under scratch. Retain baseline/treatment exact identical-input behavior evidence that passes only on unit. Do not describe --help parsing alone as proof of volatile runtime behavior.

Return report to scratch/report.md, normalize personal home prefixes in report; raw logs stay private. First six lines:
unit: launcher-volatile-replay
verdict: green | red | blocked
cases: executed <before>→<after>, red <n>
origin: n/a
wrote-outside-worktree: <paths> | none
needs-coordinator: yes | no

Include actual red/green commands/exits, scope confirmation table and deviations, generated drift results, storage usage, child/lease cleanup and anything not proven. Keep no tests ignored or failed intentionally. Do not claim story complete or full Codex support.
