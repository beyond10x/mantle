---
format: aep.planning-md/3
id: coordination-blocker:codex-terminal-capture
kind: coordination-blocker
status: open
title: Resolve terminal capture before Codex device login
refs:
- provider: github
  reference: beyond10x/substrate#112
relations:
- blocks: story:codex-interactive-start
revision: 4
---
## Observation

Read-only story-scoper tracing on 2026-10-02 found that Mantle's normal attachment uses the
pinned Substrate PTY session. Substrate retains each forwarded output byte and persists terminal
observations into SQLite stdout/stderr BLOBs. Clearing Mantle's own replay ring therefore cannot
prevent a displayed device-login code from being recorded downstream. Output limit zero is
refused; later retirement deletes the row only after a durable terminal record exists.

## Sources and proof level

Pinned Substrate revision05695970b069f79e6678f2f02cbd78bbe5fa2a56:
crates/substrate-host/src/process.rs:2184,2297,2744-2789 (capture and forwarding),
:1833 (zero-bound refusal); crates/substrate-daemon/src/app/sessions.rs:1844,1893
(terminal persistence); app/operations.rs:1125 (StoredExec bytes);
crates/substrate-store/src/execs.rs:733 (BLOB upsert), schema.rs:38,136 (WAL/schema),
sessions.rs:297,383 (durable prerequisite then retirement deletion).
Mantle app/session.rs:357 selects this PTY route. The coordinator inspected the capture,
zero-bound validation and SQLite write/delete code after the scoper's report. This is a source
trace, proof level3; no login/code or live persistence probe was performed.

## What is blocked

Do not dispatch the current interactive-start design as if launcher-only transcript suppression
satisfies CS-05. Do not weaken the accepted privacy boundary to exempt Substrate storage, turn a
zero output bound into an unsupported no-capture claim, or treat later row deletion as no recording.
The credential-independent worker story and its review/live acceptance can continue.

## Clearance evidence and next owner

Upstream owner: Substrate, via https://github.com/beyond10x/substrate/issues/112. The operator requested a GitHub issue instead of taking over Substrate implementation. No Substrate code or planning-store edits belong to this Mantle wave.

Clear only after a supported exact Substrate/SDK revision supplies policy-controlled non-recording streaming, accurate lifecycle/exit/resource/audit metadata and independent finite stream/buffer/backpressure bounds, with no terminal output in SQLite/WAL or diagnostics across success/failure/recovery paths. Mantle must request and verify the effective mode and refuse unsupported/denied requests before login; no silent fallback to captured output. Filing or closing the issue, probe-only success or later row deletion is insufficient evidence.

Mantle's launcher and Codex diagnostic sinks are separate mandatory obligations. Independent launcher work can proceed while this blocker remains open. Real login remains blocked until supported upstream behavior and downstream synthetic sink proof are available. design:codex-terminal-confidentiality records the inspected seams, ownership and proposed proof.
