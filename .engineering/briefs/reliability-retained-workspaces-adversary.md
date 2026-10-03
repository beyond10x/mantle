# Unit 6 adversary

Read shared invariants, the retained workspaces implementation brief, accepted story and AEP
adversary procedure. Final source and handoff come from the coordinator. Use the assigned
`mantle-retained-workspaces` tree/target/tmp only after handoff; lease
`codex-retained-workspaces-adversary`; scratch
`$HOME/.cache/mantle-reliability/retained-workspaces/adversary`. Rust tests only, no runtime,
planning, generated edits, commits or publication. End only your own lease.

Drive the new ESS lifecycle against production state/application seams. Prioritize retained
filesystem/auth markers across stop/restart, no reclone or auth overwrite, reserved names,
legacy destructive STOPPING/STOPPED interpretation and refusal of missing restart context.
Exercise uncertain admission and process readiness separately: operation terminal does not mean
execution terminal. Reopen durable state after each interruption; an accepted/unknown operation
must not cause a fresh start identifier on retry. Missing observations across a changed daemon
deployment are not proof an earlier request was never accepted.

Attack concurrent stop/restart/destroy and exact session-ID guards with controlled interleavings.
A stale intent must not signal a new execution or mutate the newer session occupying a name.
Attachment must never trigger restart. Use real local SQLite/filesystem fixtures and controlled
SDK IO only; no live worker, authentication files or user session calls.

Use reported baseline counts. Add a case before running it, capture the isolated result before
the relevant suite, and preserve all existing assertions. Return the required header, tests-only
diff, portable full commands/output, reachable and attributed findings, every external path and
the matching machine-readable findings block. The root records the report unchanged before
routing. At most two attacks apply.
