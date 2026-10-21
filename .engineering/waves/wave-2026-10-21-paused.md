# Wave 2026-10-21 (paused before any build ran)

Stories: story:session-lifecycle, story:codex-interactive-start, story:egress-gateway (local parts).
Paused on the orchestrator's order on 2026-10-21. No build, test or gate ran on this branch: the
build slot needs 20G free on `/` and the machine had 6.7G.

## Done on this branch (unverified by any build)

- ESS pin 0.50.0 -> 0.67.0: CI download and checksum (`.github/workflows/ci.yml`, SHA256 from the
  0.67.0 release `SHA256SUMS`), `spec/ess-inputs.yaml` `requires: ess 0.67`, ESS crates in
  `crates/mantle-conformance/Cargo.toml` at tag commit `4579d8049bb5b83749b46059dfc0982de7633485`,
  `Cargo.lock` re-resolved (adds `ess-http`, `ess-transport`, `ess-yaml`, `serde_norway`,
  `unsafe-libyaml-norway`), README/spec README/website version text.
- `generated/worker-model` regenerated with ess 0.67.0 (same four roots). Differences: version
  stamps, and serde_json no longer has `arbitrary_precision`.
- `ess specify validate --path spec` with ess 0.67.0: `mantle v1 — 8 file(s), 315 scenario(s), valid`.
- story:session-lifecycle acceptance amended to shipped semantics (stop retains, `destroy --yes`,
  `Ctrl-] d`, unknown exec).
- Blockers filed: environment-blocker:live-worker, credential-blocker:operator-agent-login,
  spending-blocker:aws-worker-spend.

## Open: offline test gaps found by a read-only audit of main `fc141b9`

| Story | Gap |
|---|---|
| egress-gateway | listen default `127.0.0.1:3128` and unit `ExecStart` untested |
| egress-gateway | no test of one log line per connection, of its `dest`/`up`/`down`/`outcome` values, or that a `Proxy-Authorization` canary stays out of it |
| egress-gateway | `tests/refusal.rs` checks only the status for `169.254.169.254`, not the body |
| session-lifecycle | no fixture returns an Unknown exec; status text, stop (neither RETAINED nor STOPPED) and attach refusal untested |
| session-lifecycle | `mantle status` Sources line, `list` after `destroy --yes`, Claude `--cwd /workspace/<mount>` untested |
| codex-interactive-start | CS-02: Codex `start --detached` hint untested |
| codex-interactive-start | CS-06: missing Codex binary refusal untested (only Claude, `adapters/substrate.rs:234`) |
| codex-interactive-start | CS-08: no probe with `.codex/config.toml`, hooks or `AGENTS.md` inside the repository checkout |

Designs prepared before the pause, not written: an injectable log sink for `serve_using` in
`crates/mantle-egress/src/proxy.rs` plus a pure line-render function in `src/log.rs` (production
format unchanged); an `orchestration.rs` fixture that returns and keeps an Unknown exec, with the
CLI checks in `crates/mantle/tests/retained_workspaces.rs`.

Noted, not in scope: `mantle exec` sets no Cargo proxy (only the git clone has one,
`crates/mantle/src/app/session.rs:21`).

## First step on resume

Run `task check` on this branch with ess 0.67.0 on `PATH` (gate for the pin move, including
`cargo run -p mantle-release -- notices --check` if the new lock entries reach a shipped binary),
then implement the gaps above: egress and session-lifecycle in parallel, codex after
session-lifecycle (both touch `crates/mantle`).
