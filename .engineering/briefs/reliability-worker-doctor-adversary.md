# Unit 3 adversary

Read [shared invariants](reliability-invariants.md), the implementation brief and accepted
`story:worker-doctor`. Follow the installed AEP adversary procedure. This is a separate review
pass on the completed unit; the coordinator supplies its exact source commit and report path.

Use the assigned `mantle-worker-doctor` managed tree and existing unit target/tmp directories.
Acquire your own `codex-worker-doctor-adversary` lease, renew it, and end only that lease on return.
Scratch is `$HOME/.cache/mantle-reliability/worker-doctor/adversary`. No concurrent builds in that
target; the implementor must have handed it back first. Rust-only test additions are permitted;
no implementation, planning, generated-code edits, commits or publication.

Try concrete counterexamples at the actual CLI/transport seams. Prioritize early JSON failure
with no state creation; committed WAL visibility without migration/application writes; bounded
nonregular/oversized inputs; safe diagnostics without leaking provider/config values; strict
known-host handling; provider placement and ProxyCommand argument boundaries; installed versus
live compatibility; bounded subprocess/tunnel cleanup on failure and interruption. SQLite's
normal SHM/locking coordination is allowed; do not impose a byte-identical database-file rule.
No real provider, credential, SSH-host or live worker calls.

Begin with the implementor's exact executed counts. If adding a test, run it first, then the
relevant package suite. Do not weaken existing assertions or broaden scope for hypothetical
issues. Attribute findings as introduced, pre-existing or undecided with concrete reproduction.
Return the prescribed six-line header and required machine-readable findings block. Retain
verbatim commands and outputs in a portable report using `$HOME` anchors; the coordinator
records that report unchanged in AEP. Name every outside-worktree path and remaining lease.
