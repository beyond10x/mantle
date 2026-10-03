# Unit 4 adversary

Read [shared invariants](reliability-invariants.md), the prebuilt release implementation brief,
accepted `story:prebuilt-release-artifacts`, and the installed AEP adversary procedure. The
coordinator supplies the final source commit and implementation report after every build ends.

Use the assigned `mantle-prebuilt-release` managed tree and its existing target/tmp directories.
Acquire, renew and end only your own `codex-prebuilt-release-adversary` lease. Scratch is
`$HOME/.cache/mantle-reliability/prebuilt-release/adversary`. Rust-only tests are permitted;
no implementation, planning, generated-code edits, commits, publication or release. Never build
concurrently with the implementor in the assigned target.

Start with the new ESS contract and published installation instructions, then attack the actual
release CLI against them. Prioritize exact source/tag identity, strict manifest and archive
inventory, malformed ELF and static-worker classification, bounded nonregular inputs,
unmanaged destination collisions, activation interruption/concurrency, and preservation of the
previous installed generation. Check that publication observes the remote dereferenced tag
commit and refuses immutable assets or upload failure. Use controlled local command fixtures;
no actual GitHub writes, provider calls, worker calls, credentials or live sessions.

Use the implementor's reported executed counts as the baseline. Write a concrete case before
running it, run that case alone first, and only then run the relevant package suite. Preserve all
existing assertions. Report reachable counterexamples with measured output, file/line and
introduced/pre-existing/undecided attribution. Hypotheses that do not reproduce are not findings.

Return the prescribed six-line header, tests-only diff, verbatim commands/output and matching
machine-readable findings block. Use portable `$HOME` paths in the full retained report and name
every outside-worktree path and remaining lease. The coordinator records the report unchanged
in AEP before routing any findings. The review has at most two attacks under the wave procedure.
