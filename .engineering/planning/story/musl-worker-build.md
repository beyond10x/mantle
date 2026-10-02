---
format: aep.planning-md/3
id: story:musl-worker-build
kind: story
status: implemented
title: Build the released worker binaries on musl
relations:
- decomposes: story:codex-interactive-start
scope:
- confidence: cited
  path: .github/workflows/ci.yml
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Cargo.toml
- confidence: cited
  path: crates/mantle-launch/src/session.rs
revision: 8
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T18:19:32Z", actor: "human:timo", revision: 6}
- {from: "proposed", to: "active", at: "2026-10-02T18:19:32Z", actor: "human:timo", revision: 7}
- {from: "active", to: "implemented", at: "2026-10-02T18:32:56Z", actor: "human:timo", revision: 8, decided_on: {"recorded":{"test_result":1,"ess_conformance_coverage_v1":1}}}
---
## Acceptance

The declared task build-worker command compiles all three static worker binaries for x86_64-unknown-linux-musl, while existing private-path conformance continues to accept only observed tmpfs and the repository CI executes that target build.

## Observation

The first live preparation after0.1.0 ran the exact three-package release target build and failed with E0308 at crates/mantle-launch/src/session.rs:230: musl statfs.f_type is u64 while libc::TMPFS_MAGIC is i64. The same field comparison compiles on the native GNU target, so the existing CI gate missed it. Red output retained in private wave9 scratch-integration/worker-build.log, exit101. All repo f_type/TMPFS_MAGIC occurrences were enumerated; this is the only one.

## Scope

Cited crates/mantle-launch/src/session.rs (portable comparison); .github/workflows/ci.yml (static target/toolchain build gate); Cargo.toml and Cargo.lock (corrective0.1.1 release version). No ESS semantics, confinement requirement or temporary diagnostic behavior changes. Existing PrivatePaths native scenarios are the regression contract; musl compilation itself is the red-capable additional platform check. Source release0.1.0 is already immutable; correct via0.1.1, never move its tag.
