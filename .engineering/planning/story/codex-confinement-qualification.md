---
format: aep.planning-md/3
id: story:codex-confinement-qualification
kind: story
status: implemented
title: Qualify the pinned Codex TUI under Substrate confinement
relations:
- decomposes: epic:codex-parity
scope:
- confidence: cited
  path: crates/mantle-launch/tests/codex_compatibility.rs
- confidence: cited
  path: crates/mantle/Cargo.toml
- confidence: cited
  path: crates/mantle/examples/codex_qualification.rs
- confidence: cited
  path: crates/mantle/examples/tests/codex_qualification_adversary.rs
- confidence: cited
  path: docs/evidence/codex-compatibility.md
revision: 17
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T08:54:04Z", actor: "human:timo", revision: 6}
- {from: "proposed", to: "active", at: "2026-10-02T08:54:04Z", actor: "human:timo", revision: 7}
- {from: "active", to: "implemented", at: "2026-10-02T09:58:09Z", actor: "human:timo", revision: 17, decided_on: {"recorded":{"test_result":1,"review_outcome":1,"verification":1}}}
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

Confirmed by implementation and adversary pass 1 on 2026-10-02. The initial three-path placement
was inferred; the following entries replace that proposal with observed files.

- **Harness:** `crates/mantle/examples/codex_qualification.rs` — cited; Rust/clap local/confined/control probe, pinned digest/version checks, private scratch, bounded unauthenticated TUI and explicit sandbox controls.
- **Gate registration:** `crates/mantle/Cargo.toml` — cited; coordinator added example `test = true` so the normal workspace test gate executes its regressions. No new dependency.
- **Launcher controls:** `crates/mantle-launch/tests/codex_compatibility.rs` — cited; two local controls model AF_UNIX denial and FIFO relay. They do not claim full Substrate policy equivalence.
- **Adversarial regression:** `crates/mantle/examples/tests/codex_qualification_adversary.rs` — cited; FIFO binary input must refuse without blocking; coordinator registers it as a test-only example module.
- **Evidence:** `docs/evidence/codex-compatibility.md` — cited; exact official binary identity, default nested-sandbox failure, candidate profile observations, CQ outcomes and unobserved obligations.
- **Confidence:** high for file ownership; live authentication, model turns, approvals and visual usability are still explicitly unqualified.
- **Boundary:** no production implementation, confinement-policy, daemon, gateway or auth changes. Coordinator owns AEP, wave and raw evidence files separately.
- **Scope correction:** original three new paths became five when normal-gate registration and the independent regression were required. These additions are recorded in typed scope; no dependency or Taskfile change was needed.

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

## Confirmed scope and first review

The implementor confirmed the three originally inferred paths by creating the Rust example,
launcher controls and evidence document. All three are now cited implementation surfaces. The
coordinator registered the example tests in Cargo.toml and the adversary added
`crates/mantle/examples/tests/codex_qualification_adversary.rs` with a test-only module declaration.
This corrects the earlier medium-confidence scope: these five paths now exist. Production source
and confinement remain unchanged. Complete implementor observations are retained with wave evidence.

Pass 1 found one introduced reachable CLI edge: an absolute FIFO supplied as `local --codex`
blocks before the regular-file check. The named regression compiled and failed after two seconds;
review-result:codex-qualification-adversary-1 records the full report before correction routing.
This does not invalidate the official regular-file live observations. The implementor is correcting
the open/read bound while preserving the regression assertion.

The coordinator removed the two staged probe binaries, their empty worker directory, and the
host-only sentinel. Worker cleanup command exited 0; both services remained active and the original
Claude exec remained running with null exit. The exact SSH tunnel and its virtctl child were
observed absent after termination. Source: wave private scratch `worker-cleanup.log` and
`claude-after-cleanup.json`; durable closing evidence will retain these observations.

## Reconciliation with implemented ESS contracts

Integration commit `4bb052c` reconciles incoming published source `2e6b116` with the Codex plan.
The actual Session entity, constructor and views keep their implemented SQLite shape; proposed
AgentKind and AuthenticationMethod remain enum vocabulary only. The earlier provisional optional
fields are superseded. `story:codex-interactive-start` adds real agent/auth fields, DB migration,
constructor inputs, command assignments, both views and implementation adapter snapshots together.
Legacy Claude values are resolved by that migration, not invented by today's conformance adapter.

Qualification's completed harness/report may contain exact refusals and not-run cases. Missing
operator authentication blocks live acceptance and completion of interactive start, not construction
of the login plumbing needed to resolve it. A demonstrated confinement violation or mandatory
unavailable socket remains a design blocker. Final epic acceptance still requires the full named
CQ/AW/CS/DP behavior, including actual authentication, model tools, approval prompts and lifecycle
observations. The current direct outer-profile controls do not discharge those obligations.

Source: read-only story-scoper report against `2e6b116`; incoming conformance.rs complete-subject
snapshot checks; qualification live JSON and docs/evidence/codex-compatibility.md. This sequencing
clarification is the coordinator's inference and does not weaken final acceptance.

## Precommit privacy correction provenance

Before its first commit, the coordinator corrected one explanatory sentence in
review-result:codex-qualification-adversary-1: the sentence had spelled out a personal home path
while explaining normalized logs. It now says "personal home-directory prefixes". No finding,
assertion, test output, verdict, frontmatter, revision or transition changed. The original agent
report SHA256 is `559cf6cb93b9c8cb4fc8a8fa39dab7524ce9d1032fb8f44fcba8d7543cb9fe06` and remains in
private wave scratch; the original uncommitted AEP file is retained beside it.

This was a disclosed, narrowly scoped exception to the skill's immutable-body edit procedure,
using the standing authorization for reversible local corrections; it is not represented as an
AEP CLI mutation or a new operator response. The initial extra permission question was withdrawn
as unnecessary. The complete findings block is unchanged and remains comparable across passes.
