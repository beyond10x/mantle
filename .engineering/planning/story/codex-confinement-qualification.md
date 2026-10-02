---
format: aep.planning-md/3
id: story:codex-confinement-qualification
kind: story
status: active
title: Qualify the pinned Codex TUI under Substrate confinement
relations:
- decomposes: epic:codex-parity
scope:
- confidence: inferred
  path: crates/mantle-launch/tests/codex_compatibility.rs
- confidence: cited
  path: crates/mantle/Cargo.toml
- confidence: inferred
  path: crates/mantle/examples/codex_qualification.rs
- confidence: cited
  path: crates/mantle/examples/tests/codex_qualification_adversary.rs
- confidence: inferred
  path: docs/evidence/codex-compatibility.md
revision: 11
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T08:54:04Z", actor: "human:timo", revision: 6}
- {from: "proposed", to: "active", at: "2026-10-02T08:54:04Z", actor: "human:timo", revision: 7}
---
## Context

The local CLI is 0.153.4, while the confined worker only has Claude. Mantle's original tmux design failed on Substrate's AF_UNIX denial; assuming another TUI works repeats that mistake. The existing generic PTY relay is at `crates/mantle-launch/src/serve.rs`. Follow the decisions and source links in `epic:codex-parity`.

## Acceptance

Starting from an unqualified pinned Codex binary, the qualification report gives a reproducible pass or an exact blocking refusal for every named scenario below under the unchanged worker confinement.

## Named conformance scenarios

- CQ-01: versioned Linux binary and digest match before the probe; an incorrect digest is rejected.
- CQ-02: isolated Codex home and the current pinned TUI reach their interactive screen through mantle-launch without a required Unix socket; a forbidden socket probe is refused.
- CQ-03: a shell and Cargo child execute inside that workspace with the chosen documented Codex approval/sandbox profile; a host-path read outside it fails.
- CQ-04: device authentication, a real model turn and refresh travel through the candidate exact-host CONNECT policy; direct egress and an unlisted hostname fail. Measure HTTPS and WSS/fallback separately. Use a disposable, task-owned gateway/configuration rather than widening production policy for the probe.
- CQ-05: same-size reattach, real resize and a slow reader yield a usable Codex screen; use a real TUI plus bounded terminal probes, not a fake process alone.
- CQ-06: expired/revoked credentials produce an actionable login requirement without leaking values. If renewal cannot be observed in the bounded run, report not-run, never pass.

## Deliverable and stop rule

Add a Rust probe/harness (clap derive if a CLI) and a dated qualification report with binary identity, confinement facts, dependency/syscall failures, terminal observations and host list. Probe results are not production support. Missing user authentication or a denied mandatory socket is a recorded blocker on codex-interactive-start. A refusal must not be bypassed by changing Substrate policy. Choose a supported no-socket execution mode only if documented for the pinned release and demonstrated; otherwise propose the required upstream work separately.

Use an isolated temporary workspace; never replace the running substrate-work session or rebuild its VM. Maintain disk bounds and teardown only probe-owned resources. Device-code entry belongs to the operator's terminal; redact or omit credential-bearing PTY captures. No real secrets in fixtures.

## Scope

Derived 2026-10-02 by `story-scoper`; classifications distinguish inspected evidence from proposed implementation placement.

- **Primary surface:** bounded qualification tooling and sanitized evidence; production behavior remains outside this story — cited, from its deliverable and stop rule.
- **Harness:** new `crates/mantle/examples/codex_qualification.rs` — inferred, a standalone Rust CLI using clap derive and this crate's existing Substrate SDK, digest, terminal and serialization dependencies; enforce deadlines, output/disk bounds, private scratch ownership and cleanup restricted to resources created by this probe.
- **Tests:** new `crates/mantle-launch/tests/codex_compatibility.rs` — inferred, the proposed location for bounded launcher/probe controls and refusal checks; local controls must not claim live Substrate or authenticated Codex coverage.
- **Documents:** new `docs/evidence/codex-compatibility.md` — inferred, dated binary identity, confinement observations, exact hosts and separate CQ-01–CQ-06 outcomes, recording blockers and not-run cases without credentials or login transcripts.
- **Boundary:** production source, deployment configuration, specification and task-runner changes are excluded; use a disposable gateway configuration and preserve the running worker session — cited, from the story's explicit restrictions.
- **Confidence:** medium — the story names the test/report destinations, but both remain proposed and absent; the CLI example is a proposed placement supported by existing dependencies — inferred.
- **Would collide with:** changes to the three exact new paths above; dependency or shared-production edits would require rescoping before implementation — inferred.
- **Safety fact:** the inspected generic launcher already accepts arbitrary agent argv and optional proxy configuration, while the inspected gateway accepts an independent listener and replacement exact-host allowlist; this supports attempting qualification without production edits, but does not prove Codex or disposable-worker compatibility — cited, step 2, unproven.

## Runtime profile qualification

Live probe on 2026-10-02T09:14:38Z: pinned Codex's documented `sandbox` command with explicit
`sandbox_mode="workspace-write"` refused both a shell syntax check and a dependency-free Cargo
build inside Substrate. Both exited 1 with the harness's namespace-error marker; no Cargo artifact
was created. This is a nested namespace incompatibility observation, not a required Unix socket
failure. The TUI still reached its unauthenticated welcome/login screen.

Under epic decision 6, the coordinator selected an additional, separately labelled profile probe:
`sandbox_mode="danger-full-access"` and `approval_policy="on-request"`, relying on unchanged outer
Substrate confinement. Preserve the default-profile failures, and compare benign shell/Cargo and
host/socket/network negative controls in the same outer workspace. No combined bypass flag and
no Substrate relaxation. This is a qualification candidate, not yet a production default or proof
that authenticated approval prompts work.

Sources: `confined-probe-2.json` in the wave's private integration scratch; official
`openai/codex` tag `rust-v0.153.4`, `codex-rs/cli/src/debug_sandbox.rs:302-319` explicitly handles
Disabled/External enforcement by spawning directly; `codex-rs/protocol/src/config_types.rs:104-114`
declares the legacy mode; pinned CLI help independently exposes the `on-request` approval policy.

## Scope correction during implementation

The coordinator added cited `crates/mantle/Cargo.toml` to this unit's typed scope: declaring the
new example with `test = true` makes the repository's existing `cargo test --workspace --locked`
gate execute the probe's regression tests. Without it Cargo builds examples but does not run their
unit tests by default. No dependency or Taskfile change is required. The other ESS session's
Cargo change is now in committed source `87bed8f`; reconcile its dev-dependency hunk on integration.
The three original new probe/test/report paths remain the implementor's assignment.
