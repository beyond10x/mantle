---
format: aep.planning-md/3
id: review-result:session-adversary-2
kind: review-result
status: active
title: Adversary review of the round-1 correction, wave 1 pass 2
relations:
- reviews: story:session-lifecycle
revision: 1
---
unit: session-lifecycle (worktree mantle-wave1-session, HEAD e440552 plus the uncommitted round-1 correction and the untracked tests/adversary.rs and tests/adversary_2.rs)
verdict: red
cases: executed 53→56, red 2
origin: introduced 1, pre-existing 0, undecided 0
wrote-outside-worktree: /home/timo/.cache/mantle-wave1/target-session (build), /home/timo/.cache/mantle-wave1/scratch-session/{adv2-probe,adv2-claude,adv2-ovf,adv2-probe-resize.js,mantle-launch-adv2-*}, all deleted
needs-coordinator: no

# Adversary review of the round-1 correction, wave 1 pass 2

Reviewer: `aep:adversary`, 2026-10-02. Command:
`CARGO_TARGET_DIR=…/target-session MANTLE_TEST_SCRATCH=…/scratch-session cargo test -p mantle-launch --no-fail-fast` → exit 101
(unit 40/0, adversary.rs 9/0, adversary_2.rs 1 passed / 2 failed, launch.rs 4/0).

| Test | Result | Red output |
|---|---|---|
| `a_size_aware_agent_repaints_on_a_real_resize` | green (control) | |
| `attaching_at_an_unchanged_size_makes_the_agent_repaint` | red | `serve's attach redraw did not make a size-aware agent repaint; terminal got "ready\r\n"` |
| `a_terminal_that_lost_output_gets_a_repaint_once_it_catches_up` | red | `no repaint after the terminal lost output and caught up; 394247 bytes received, tail "xxxx…xxx\r\nFLOODED\r\n"` |

What reaches it: every `mantle attach` from a terminal the size of the agent's window
(`serve.rs:442`) and every repaint owed after dropped output (`serve.rs:328-329`). Probe with real
Claude Code 2.1.287 under this `serve`: three attaches each received exactly 3566 bytes (the
scrollback replay, nothing after); after a real `120 40` resize an attach received 9970 bytes.
Bun 1.1.30 and Node receive the SIGWINCH and emit no `resize`. Already ineffective in `e440552`.

```findings
- file: crates/mantle-launch/src/serve.rs
  line: 458
  category: acceptance
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: redraw() sets rows-1 and restores the size back to back, so Claude Code (Bun) sees no size change and never repaints on reattach or after the correction's dropped-output Repaint
```
