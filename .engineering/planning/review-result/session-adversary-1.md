---
format: aep.planning-md/3
id: review-result:session-adversary-1
kind: review-result
status: active
title: Adversary review of mantle-launch serve|attach, wave 1 pass 1
relations:
- reviews: story:session-lifecycle
revision: 1
---
unit: session-lifecycle (worktree mantle-wave1-session, HEAD e440552 plus the untracked crates/mantle-launch/tests/adversary.rs)
verdict: red
cases: executed 37→46, red 6
origin: introduced 7, pre-existing 0, undecided 0
wrote-outside-worktree: /home/timo/.cache/mantle-wave1/target-session, /home/timo/.cache/mantle-wave1/scratch-session/mantle-launch-adv-* (12 dirs)
needs-coordinator: no

# Adversary review of mantle-launch serve|attach, wave 1 pass 1

Reviewer: `aep:adversary`, 2026-10-02. Command:
`CARGO_TARGET_DIR=…/target-session MANTLE_TEST_SCRATCH=…/scratch-session cargo test -p mantle-launch --no-fail-fast` → exit 101
(unit 33/0, launch.rs 4/0, adversary.rs 3 passed / 6 failed).

| Test | Result | Red output |
|---|---|---|
| `sigterm_to_serve_ends_the_agents_whole_process_group` | red | `serve exited (exit status: 129) while pid 2514017 of the agent's process group still runs` |
| `last_output_does_not_follow_a_planted_symlink` | red | `left: "agent-marker-5e1" right: "untouched"` |
| `last_output_is_owner_only_even_when_the_file_already_exists` | red | `last-output left at mode 644` |
| `session_dir_is_owner_only_even_when_it_already_exists` | red | `session directory left at mode 755` |
| `a_symlinked_session_dir_is_not_followed` | red | `followed the symlinked --dir and wrote ["last-output"] into ".../elsewhere"` |
| `a_flooded_ctl_pipe_does_not_stall_the_agent` | red | `the agent made no progress for 8s while ctl was flooded` |
| `a_client_that_never_reads_does_not_block_the_agent` | green | |
| `exit_code_is_the_agents` | green | |
| `the_secret_descriptor_does_not_reach_the_agent` | green | |

Reach: every red case needs a process in the same workspace (uid 900) planting a path or flooding a
pipe, or is covered in production by the worker's cgroup kill (`exec.cgroup_kill`). No privilege
boundary is crossed.

```findings
- file: crates/mantle-launch/src/serve.rs
  line: 227
  category: acceptance
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: serve returns once the agent's main process is reaped and never kills the rest of its process group; only the worker's cgroup kill ends the survivors
- file: crates/mantle-launch/src/serve.rs
  line: 490
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: last-output is opened without O_NOFOLLOW and keeps a pre-existing file's mode, so a planted symlink or a 0644 file is written through
- file: crates/mantle-launch/src/serve.rs
  line: 52
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: an existing --dir keeps its mode and a symlink at --dir is followed
- file: crates/mantle-launch/src/serve.rs
  line: 362
  category: concurrency
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: read_ctl reads until the pipe is empty, so a continuous ctl writer starves the pty and blocks the agent
- file: crates/mantle-launch/src/serve.rs
  line: 315
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: overflow of the pending ring drops bytes mid-stream with no redraw afterwards, so a slow client is left with a garbled screen
- file: crates/mantle/src/domain/manifest.rs
  line: 240
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: agent.cwd containment is a string prefix check, so /workspace/r/../.. is accepted
```
