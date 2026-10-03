# Unit 7: repeatable acceptance through the installed CLI

Read [shared invariants](reliability-invariants.md) and
[the story](../planning/story/repeatable-agent-acceptance.md). Dispatch follows the green retained
lifecycle gate; the execution record supplies the exact source base and measured counts.

- Managed id and branch: `mantle-agent-acceptance`, `unit/mantle-agent-acceptance`.
- Checkout: `$HOME/.local/state/worktree/trees/b10x/mantle/mantle-agent-acceptance`.
- Implementor lease: `codex-agent-acceptance`.
- Target: `/dev/shm/mantle-agent-acceptance-target`.
- Temporary files: `/dev/shm/mantle-agent-acceptance-tmp`.
- Scratch: `$HOME/.cache/mantle-reliability/agent-acceptance`.

Deliver a Rust/clap runner for the installed Mantle CLI, not another direct SDK orchestrator.
Extend the existing ESS acceptance values with run/session/workspace identity, source provenance
and evidence origin before implementing them. Both Claude and Codex have the same scenario
inventory. Missing login/model/visual attestations remain operator-required or not-run, and the
aggregate remains incomplete. Authentication method or a running exec is never authentication.

Add versioned metadata-only `status --json` and `list --json`, including terminal records and
exact session-id lookup. Separate recorded state from observed state and expose explicit
connectivity/refusal outcomes. Never fetch terminal output pages for these commands. Do not
parse or capture the old human status path, which can print terminal stderr. Reuse verified
release provenance when available; unavailable source identity stays unknown rather than being
inferred from a version string. Keep these interfaces useful without the runner.

Persist the run's exact owned identities before dependent work. All lifecycle mutations and
cleanup pass expected-session-id guards; a name prefix is not ownership proof. Refuse reused
names and leave unrelated sessions alone. The runner may start detached, discard human startup
output, and obtain identities through structured metadata. Keep execution/output/time bounds and
owned temporary/process cleanup; no credential values, terminal transcripts or auth-cache scans.

Login and the model/tool action run in the operator's terminal through the exact selected-profile
attach command, outside runner pipes. Resume from a private checkpoint with explicit, bounded
attestations; label them separately from machine observations. Automate observable detach/reconnect,
resize/interrupt and transport-loss controls through owned subprocess/PTY seams where meaningful,
while visual correctness remains an operator observation. Verify retained workspace identity and
synthetic markers across stop/restart, a changed exec identity, and explicit owned destruction.

Use controlled CLI/PTY and real local filesystem fixtures for deterministic test coverage, plus
an opt-in real mode. Never use existing user sessions or VMs, provision a worker, read credentials,
or claim that this development run establishes full authenticated agent qualification. The story
delivers the repeatable tool and its verified behavior; real attestations are separate evidence.

Reconcile README, website and spec/README with the final source. Update the SDK pin, domain
inventory, supported agents, lifecycle and current conformance counts; distinguish historical
0.1.4 claims from unreleased behavior. Preserve historical qualification limits. The coordinator
refreshes the published native evidence ledger from the final exact run and creates the one PR.
No main merge, tag, source release or website deployment is part of this task.
