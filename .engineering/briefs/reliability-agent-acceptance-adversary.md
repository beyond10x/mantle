# Unit 7 adversary

Read shared invariants, acceptance runner implementation brief, accepted story and AEP adversary
procedure. The coordinator provides exact source and handoff. Use the assigned
`mantle-agent-acceptance` tree/target/tmp after handoff; lease
`codex-agent-acceptance-adversary`; scratch
`$HOME/.cache/mantle-reliability/agent-acceptance/adversary`. Rust tests only. No implementation,
planning or generated edits, commits or publication. End only your own lease on return.

Start with the declared scenario inventory and report schema, then run the actual runner/CLI
against them using controlled local Rust CLI/PTY fixtures. Prioritize missing or contradictory
observations, failed commands, bounded timeouts, interrupted checkpoint/resume and malformed
metadata. Missing login/model/visual attestations must produce incomplete qualification; an
auth method, executable version or synthetic fixture must never turn into authenticated evidence.

Check that metadata paths never fetch terminal output or retain raw diagnostics, credentials,
conversation text or human status output. Machine observations and operator attestations remain
distinct. Reports must retain exact session identity and known/unknown source provenance.
Attack stale names and interrupted cleanup: only the runner's exact owned session IDs may be
stopped/restarted/destroyed, with every destructive call guarded before mutation.

No real device login, model traffic, workers, credentials or user sessions. Public documentation
must distinguish development behavior from release 0.1.4 and historical qualification from new
local fixture evidence. Source documentation validation is not website publication.

Take baseline executed counts from the implementor. Write and run each new case alone before
its package suite; preserve existing assertions. Return the prescribed header, tests-only diff,
portable full report with verbatim commands/output, attributed reachable findings, every external
path and matching findings block. Root records it unchanged in AEP. At most two attacks apply.
