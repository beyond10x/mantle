---
format: aep.planning-md/3
id: review-result:worker-linker-adversary-1
kind: review-result
status: active
title: Native Rust dependency build exposes incomplete compiler selection
relations:
- reviews: story:worker-rust-linker
revision: 1
---
unit: story:worker-rust-linker at9b83856
verdict: NEEDS-CHANGE
cases: native host-crate cargo check executed1, red1; no additional local suite claimed
origin: introduced0 / pre-existing0 / undecided1
wrote-outside-worktree: ~/.cache/mantle-wave10/native-build.log and timing/exit siblings
needs-coordinator: local role fallback, not independent-agent review

Reviewer diff: empty. No implementation or test files changed during this pass.

The actual b10x-substrate-host cargo check, after the wire-only crate passed, failed with remote exit101. Native cc-rs dependency libz-sys invokes cc; CC is absent and/usr/bin/cc remains a dangling/etc/alternatives symlink in confinement. Its exact stderr says: error occurred in cc-rs: failed to find tool "cc": No such file or directory (os error2). The deployed same-policy Codex workspace and ordinary Rust repository command reach this path, so it is feasible. CLI process exit0 is not the remote exit; evidence uses the printed remote ExecExit101. Raw output retained in private wave10 scratch and copied to durable live evidence by coordinator.

crates/mantle/src/adapters/substrate.rs:133 — NEEDS-CHANGE, undecided — selecting only Cargo's Rust linker leaves native build-script compiler lookup broken. Select the already-mounted gcc/g++ for native build helpers too; keep the existing confinement. The relevant base environment is the same in the source baseline, but this native-host command was not separately replayed on that baseline, so origin is undecided.

```findings
- file: crates/mantle/src/adapters/substrate.rs
  line: 133
  category: acceptance
  severity: blocker
  verdict: NEEDS-CHANGE
  origin: undecided
  message: Rust linker selection alone leaves native cc-rs build dependencies unable to find cc inside the confined root.
```
