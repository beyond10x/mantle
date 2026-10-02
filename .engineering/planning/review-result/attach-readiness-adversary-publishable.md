---
format: aep.planning-md/3
id: review-result:attach-readiness-adversary-publishable
kind: review-result
status: active
title: Independent attach readiness review with portable evidence references
relations:
- reviews: story:attach-startup-readiness
revision: 1
---
unit: Mantle readiness c58e601 plus adversary tests
verdict: nothing found locally; live acceptance remains unresolved
cases: executed 78→80, red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: assigned scratch logs only
lease: released; no builds running

```text
crates/mantle-launch/tests/conformance.rs | 93 +++++++++++++++++++++++++++++++
1 file changed, 93 insertions(+)
```

Added tests exercise fragmented ACK, final-byte rejection, replacement attachment, first command sharing the ACK write, resize before ACK, and signal restoration. Both passed on first execution.

Full launcher suite: 80 Rust tests passed. Explicit pinned ESS 0.50 rerun:

```text
mantle-launch: 49 passed, 0 failed, 0 unsupported
```

Logs:

- `.engineering/reports/attach-startup-2026-10-02/readiness-cases.log`
- `.engineering/reports/attach-startup-2026-10-02/launcher-suite.log`
- `.engineering/reports/attach-startup-2026-10-02/launcher-ess-pinned.log`

Root owns the test additions and available target directory. These results do not resolve the previously demonstrated production queue overflow.

```findings
[]
```
