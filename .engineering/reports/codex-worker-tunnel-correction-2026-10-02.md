unit: story:agent-ready-worker — live socket-path correction
verdict: green for offline correction; coordinator owns fresh-worker readiness retry
cases: Rust164→167 passed,0failed/ignored; native ESS313 unchanged,0gaps; retained long-path regression red before fix
origin: n/a
wrote-outside-worktree: assigned target/TMPDIR, owned /tmp fixture/socket directories, scratch-worker/tunnel-* evidence below, own lease metadata
needs-coordinator: commit reviewed correction and reuse existing fresh VM; source and CLI frozen, no third independent attack requested

## Fix and evidence

The fresh worker already installed Codex, but local SSH forwarding could not use the derived socket path. The production allocation was state/run/<UUID>-<PID>.sock; the observed state prefix alone was61bytes. I extracted the exact existing allocation without changing its algorithm, then exercised it in an isolated child with MANTLE_STATE_DIR set only for that subprocess. UnixListener::bind on the actual allocated path failed with the Unix address-length limit. The retained test therefore witnesses the real allocation/bind boundary, without depending on a VM, SSH authentication or a fabricated length check.

Red command: cargo test --locked -p mantle tunnel_socket_binds_with_live_length_state_prefix -- --nocapture, exit101. Exact red source is tunnel-red-source.rs.

```text
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 12.84s
     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle-277e3ad59dda8f7b)

running 1 test

thread 'adapters::ssh::tests::tunnel_socket_binds_with_live_length_state_prefix' (1718302) panicked at crates/mantle/src/adapters/ssh.rs:283:9:

running 1 test
test adapters::ssh::tests::tunnel_socket_binds_with_live_length_state_prefix ... FAILED

failures:

failures:
    adapters::ssh::tests::tunnel_socket_binds_with_live_length_state_prefix

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s



thread 'adapters::ssh::tests::tunnel_socket_binds_with_live_length_state_prefix' (1718309) panicked at crates/mantle/src/adapters/ssh.rs:273:18:
production tunnel socket must bind at the observed live state-prefix length: Error { kind: InvalidInput, message: "path must be shorter than SUN_LEN" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test adapters::ssh::tests::tunnel_socket_binds_with_live_length_state_prefix ... FAILED

failures:

failures:
    adapters::ssh::tests::tunnel_socket_binds_with_live_length_state_prefix

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p mantle --bin mantle`
```

Socket allocation now creates a unique tempfile directory under explicit short /tmp, requests0700 atomically at creation, and uses its single-character socket filename. Persistent state, key and known-host paths do not participate in the forwarding pathname. Ambient TMPDIR, non-ASCII state names, colons and worker identifiers cannot lengthen or change that address. Tunnel retains the directory's RAII owner; its existing child kill/wait runs before fields drop and remove the socket directory. Spawn failure drops the allocation immediately; early exit/timeout/error drops Tunnel through the same ownership path.

The first expanded permission test caught an incorrect assumption about tempfile's default: actual mode0755 (493) failed the required0700 (448) assertion in tunnel-green.log, exit101. The correction uses Builder.permissions before creation, never a permissive-create/chmod interval. No assertion was relaxed.

The private tunnel_with spawn seam preserves Command::spawn in production and substitutes controlled child processes only in tests. No shell/Python program, new CLI surface, credential relocation or ESS contract change was introduced.

## Class coverage

Three new tests preserve all164 prior tests and every313 named ESS scenario:

- Original61-byte live-prefix case binds the actual production allocation.
- Long ASCII, non-ASCII and colon-containing state paths each run in isolated subprocesses with a deliberately missing/long ambient TMPDIR. Eight simultaneous allocations for the same worker bind distinct real Unix sockets. Directory mode0700 and creator ownership are observed. Existing identity/known-host marker files and command key paths remain unchanged. Every allocated directory is removed by RAII.
- A controlled live child proves the directory remains available while Tunnel exists, and that drop reaps the direct child and removes the directory. Real missing-executable spawn failure and immediate nonzero child exit both return errors and remove their allocations.

The90-second startup-timeout branch was inspected as the same Tunnel-drop path, not exercised by waiting90seconds. Existing child kill/wait semantics were preserved; this change does not make new promises about unkillable kernel tasks or cleanup after SIGKILL. Allocation failure returns contextual error before spawning; it was inspected, not simulated by changing global /tmp permissions.

Targeted treatment command: cargo test --locked -p mantle adapters::ssh::tests -- --nocapture, exit0:

```text
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 22.64s
     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle-277e3ad59dda8f7b)

running 3 tests
test adapters::ssh::tests::tunnel_socket_binds_with_live_length_state_prefix ... ok
test adapters::ssh::tests::socket_allocations_are_private_unique_and_state_path_independent ... ok
test adapters::ssh::tests::tunnel_socket_directory_lives_until_child_cleanup_and_is_removed_on_errors ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.13s

     Running unittests examples/codex_qualification.rs (/dev/shm/mantle-wave3-target-worker/debug/examples/codex_qualification-2c3ae41e1f0be255)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s
```

## Gates and rebuilt artifact

Commands used the assigned tmpfs CARGO_TARGET_DIR/TMPDIR, empty RUSTC_WRAPPER, jobs2, dev/test debug0 and incremental0. Builds ran serially. Capacity was21GiB root/18GiB tmpfs, above assigned floors.

| Command | Exit | Evidence |
| --- | --- | --- |
| cargo test --workspace --locked | 0 | tunnel-workspace.log;167 top-level tests, no failures/ignored, nested child summaries excluded |
| cargo run --locked -p mantle-conformance -- --root . | 0 | tunnel-aggregate.log;313 passed, no gaps |
| cargo clippy --workspace --all-targets --locked -- -D warnings | 0 | tunnel-clippy.log |
| cargo fmt --check | 0 | tunnel-fmt.log, empty |
| ess specify validate --path spec | 0 | tunnel-ess.log |
| git diff --check | 0 | empty output |
| cargo build --locked -p mantle | 0 | tunnel-build-cli.log |
| rebuilt mantle --help | 0 | tunnel-cli-help.log |

Workspace lane counts are Mantle29, qualification10, conformance1, docs2, egress25+6+3, launcher44+9+3+2+1+4 and worker28. All retained adversary/correction assertions remain. The coordinator reviewed this exact bounded correction and explicitly waived a repeat static build because the worker trio is unchanged.

```text
    Finished `dev` profile [unoptimized] target(s) in 0.28s
     Running `/dev/shm/mantle-wave3-target-worker/debug/mantle-conformance --root .`
Complete ESS inventory: 313 passed; 0 failed, skipped, unsupported, outside or refused
```

```text
    Blocking waiting for file lock on package cache
    Checking mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `dev` profile [unoptimized] target(s) in 8.66s
```

```text
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `dev` profile [unoptimized] target(s) in 13.18s
```

```text
0762c0584d9c1043d7f398f06ca4ee2712c514e54b8e7c219a89b7d667bb4390  /dev/shm/mantle-wave3-target-worker/debug/mantle
```

## Handoff

Changed source is only crates/mantle/src/adapters/ssh.rs, crates/mantle/Cargo.toml and Cargo.lock. tempfile was already locked/cached; this adds Mantle's production dependency edge. Source and rebuilt CLI are frozen and remain unstaged/uncommitted on79c175c58ab8c739f0ff359383f8370c6be481fd.

No live action, VM recreation, fixture shortening, AEP write, commit or independent review was performed. The coordinator may retry the same booted worker with the same configuration/state. No compiler or fixture using this unit's target remains. An unrelated /bin/sleep30 under sleep infinity was left untouched. Own codex-wave3-worker-implementor lease was released, exit0.

Evidence written under ~/.cache/mantle-wave3/scratch-worker/: tunnel-metadata.json (offline Cargo metadata/lock refresh); tunnel-red-source.rs; tunnel-red.log; tunnel-green.log (permission red); tunnel-green2.log; tunnel-workspace.log; tunnel-aggregate.log; tunnel-fmt.log; tunnel-ess.log; tunnel-clippy.log; tunnel-build-cli.log; tunnel-cli-hash.txt; tunnel-cli-help.log; tunnel-correction-report.md. Native runner artifacts were refreshed under the worktree's ignored .engineering/drafts. Owned temporary /tmp allocation/fixture directories were removed by their RAII owners; build/test files stayed in the assigned target/TMPDIR. Raw logs remain private; report home-path prefixes are normalized.

