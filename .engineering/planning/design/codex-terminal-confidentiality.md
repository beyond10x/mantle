---
format: aep.planning-md/3
id: design:codex-terminal-confidentiality
kind: design
status: draft
title: Codex terminal confidentiality over retained Substrate streams
relations:
- designs: story:codex-interactive-start
- informed_by: coordination-blocker:codex-terminal-capture
revision: 4
---
## Problem and evidence

Pinned Substrate captures attached PTY/raw-pipe stdout and persists it in SQLite, including WAL, before Mantle retires the terminal session. See coordination-blocker:codex-terminal-capture and .engineering/reports/codex-interactive-preparation-2026-10-02.md for exact source paths. Launcher serve independently keeps a memory scrollback ring and writes last-output at process completion. Deleting stored output afterward does not prove non-persistence; a zero capture bound is refused. No supported no-capture session option was found.

The operator's objective remains a real confined Codex TUI with subscription login and Claude-equivalent attach behavior. This draft does not clear the blocker, approve a new credential exception, change CS-05, or authorize implementation. It records concrete candidate choices for the required design review.

## Candidate transport

Use supported Substrate raw-pipe sessions for a bounded application protocol between the laptop relay and mantle-launch attach. The agent still runs in the existing inner PTY under unchanged confinement. The laptop generates a new ephemeral X25519 decryption identity per attachment and passes only its public recipient to attach. Attach encrypts every terminal-output record before stdout; the laptop decrypts only to the operator terminal. No private identity reaches argv, environment, worker, state, logs or evidence. Ciphertext can remain in Substrate storage; do not claim that Substrate records nothing.

Evaluate a maintained implementation of the age format with independently finalized bounded records, rather than custom cryptography or a never-finalized stream. Records need an authenticated stream identifier, ordering/sequence and record kind inside the encrypted payload; clear framing must enforce limits before allocation. Fail closed on unknown version, malformed length, truncation, wrong key, authentication error or unexpected order. Never print undeciphered remote output or fall back to plaintext. Remote stderr/errors must use fixed non-secret diagnostics.

Frame keyboard bytes, initial window and subsequent resize separately over raw stdin; preserve inner PTY Ctrl-key behavior, local Ctrl-] d detachment and single-client locking. Do not use native pipe resize, which the SDK refuses. The inspected stdin path has no durable payload sink, but acceptance must test that boundary too.

The public 1MiB output bound counts ciphertext. Before implementing, choose and prove a bounded attachment-continuation policy which cannot silently freeze output, cut a frame, lose accepted terminal bytes, end the agent or change user detach semantics. Raw-pipe forwarding can stop at the capture bound while its process lives, so ignoring Truncated is unacceptable. This is an unresolved part of the candidate, not a claim that encryption alone completes the design.

## Launcher recording choices requiring review

The existing story proposes a protected login phase followed by ordinary replay. A separate codex login --device-auth child gives a real completion boundary, but the subsequent TUI can still choose startup onboarding independently. Cache existence and a previous exit0 cannot mechanically rule that out. Inspected main-TUI logout exits; managed expired/revoked recovery refreshes or returns error, without requesting a device code. This narrows the problem but is not a race-free phase signal.

Two choices remain explicit:

1. Preserve the currently proposed phase guarantee by finding a supported authoritative startup/authentication boundary and proving trailing-output drain and renewed-login handling. No such complete signal has yet been established.
2. Consider a revised persistence contract: Codex scrollback stays only in bounded launcher memory for the live session; never persist its last-output; encrypt all attachment output before Substrate; keep ordinary reattach replay available, including the current login screen. This avoids guessing the TUI phase, but changes the current prose that prohibits login data in generic scrollback. It would protect persistent plaintext, not claim that live terminal bytes never exist in memory. Four critics must judge whether this meets the intended privacy/experience boundary before the story is changed. This alternative is not silently adopted here.

Both choices must govern Codex's own diagnostic files. Pinned CLI login initializes codex-login.log and token-exchange errors can contain backend text. Only the explicit private credential cache is currently a planned persistent credential exception. Fixed generated config, safe home paths, restrictive modes/umask, configuration precedence and diagnostic canaries are required; a file-mode check alone does not make a transcript non-secret.

## Proof before dispatch

- Reconcile exact source scope after the worker merge and define any new typed protocol values in ESS before writing implementation stories around them.
- Resolve the two open choices: authoritative phase versus an explicit reviewed persistence-contract revision, and bounded stream continuation with preserved terminal behavior.
- Run the four AEP planning critics over the concrete design, changed story text, typed scope and acceptance cases. Keep every immutable verdict and outcome; at most two rounds for that revised set.
- Red-capable canaries must cover Substrate stdout/stderr storage and WAL, launcher memory/persistent policy, private Codex logs/errors, local DB/status/manifests and evidence. No real login code is used for these tests.
- Test malformed/reordered/truncated frames, wrong keys, slow readers, short-prompt latency, binary/escape bytes, Ctrl keys, initial/changed windows, same-size reattach and output-budget continuation.
- Keep real operator-completed private login, model/tool turn, refresh/revocation, exact observed destinations and both-agent lifecycle acceptance as required live evidence. Offline protocol tests do not discharge those cases.

## Scope and confidence

Cited seams: launcher attach.rs/cli.rs/serve.rs/ctl.rs and its existing tests; laptop app/terminal.rs/app/session.rs and SDK adapter; relevant Cargo manifests/lock; ESS launch/session contracts and real conformance adapter. New shared protocol placement remains inferred and must not become a broad directory reservation. No Substrate source modification, public app-server, auth broker or new API-billing mode is part of this candidate.

Evidence is read-only source inspection plus upstream API/format documentation. Crypto framing, phase handling, latency and complete runtime sink exclusion are unproven. Status remains draft.

## Coordinator proposal for review

The coordinator proposes choice2 for the next concrete review: bounded volatile replay plus no persisted Codex last-output, and encrypted attachment output over supported raw pipes. This is an explicit proposed revision of the earlier phase-suppression mechanism, not a claim that a reliable authentication-phase signal has been found. The persistence boundary would be stated plainly: login screens may remain in the live server's bounded memory and be shown to its authorized attached terminal, but must not reach any persistent plaintext transcript, diagnostic, state or evidence. Normal reattach replay remains useful. Device codes and credential values remain absent from argv and non-secret state; the private Codex credential cache remains the sole persistent credential exception. Reviewers must judge this against the epic's intended privacy and terminal experience before CS-05 prose changes or the blocker clears.

For the fixed output budget, propose a complete encrypted end-of-attachment control record reserved within the1MiB capture allowance. Attach stops accepting additional FIFO reads before its next complete encrypted record plus reserved ending would exceed that allowance. It finishes all accepted records, emits the authenticated budget-end reason and exits, releasing the single-client lock. The laptop restores terminal mode and clearly says the terminal budget ended while the agent remains running, with the ordinary attach command. Never wait for Substrate's late Truncated frame or call a partial frame successful. Reattachment uses the existing bounded scrollback/redraw behavior; no unbounded transcript, automatic reconnection loop or exactly-once terminal history is promised. This keeps the existing finite attachment boundary explicit. Review must decide whether required parity cases demand transparent continuation; if so, this proposal remains insufficient rather than silently expanding its claim.

The proposed tradeoff reduces usable plaintext per attachment because ciphertext framing consumes the same fixed budget. Measure and report actual overhead and short-prompt latency. Ordinary frame limits, input/resize semantics, wrong-key/order/truncation refusal, current-session replay and slow-reader recovery need concrete ESS/test obligations before implementation.

## Native ESS sweep reconciliation

Read-only inspection against rebased integration 6a5aa99 is retained in .engineering/reports/codex-interactive-native-scope-2026-10-02.md. It supplies exact proposed paths and native scenario seams; this is preparation, not approval or blocker clearance.

The sweep's launcher library can host an inferred shared terminal_protocol module without a new runtime crate. The native Request adapter currently observes serialized RunRequest fields but cannot prove PTY versus pipe selection; production transport selection and its actual output bound must become observable before a conformance case can claim that property. Retain all legacy launcher cases, especially Emit, Signals and SlowReader assertions that rely on persistent last-output; add volatile-only siblings with bounded live collectors and explicit file-absence checks. Existing resize fixtures write directly to ctl, so a secure-attach case must deliver its frame through the actual attach process.

Source inspection also identifies blocking writes in both current attach and laptop output. Encryption alone does not establish responsiveness under a slow reader: bound complete-record queues and partial-write offsets, preserve control/signal handling, and never drop ciphertext bytes to make space. The fixture collectors themselves need finite byte/wait bounds for added flood cases. Treat a stale preexisting last-output path explicitly in the volatile contract rather than making an absence claim about a reused path.

The report's generated terminal-model operational state file is not a proposed committed artifact. As established by worker generation, only portable generated source/schema/report files may be tracked; ignore machine/inode state. Final exact generated placement remains inferred until the ESS probe establishes it. Typed protocol values, final story prose and the four-critic review remain pending after worker integration; no new protocol implementation is authorized by this section.

## Concrete contract preparation after worker integration

## Concrete contract preparation after worker integration

Read-only proposal against worker integration247ed1715ac24bba0c4188354d5aac3c02d6c26a is appended to .engineering/reports/codex-interactive-native-scope-2026-10-02.md. It names ESS value types, three real native commands, exact request mode/output-bound observations and the ten previously scoped authored scenario files. The current applied specification validated and compiled; the proposed YAML is text only and has not been applied or validated. This is preparation for the next review, not blocker clearance or implementation approval.

Before dispatch, settle the generated wire representation and numeric/Bytes bounds, measured age record overhead/end reserve, bounded partial-write/teardown behavior, prevention of accidental plaintext attachment selection for volatile Codex sessions, and truthful AgentStreamClosed versus observed remote exit. Proposed stale last-output behavior is refusal before child launch, preserving the existing path/target; legacy persistent sessions remain unchanged. Runtime wire types must be generated from reviewed ESS, not manually duplicated.

The concrete proposed cases preserve all current legacy scenarios and require actual codec/process/PTY/FIFO observations, malformed/refused record cases, full-byte input/resize/control delivery, no persisted last-output, encrypted canary exclusion, explicit budget ending and manual reattach. Native tests still cannot discharge actual Substrate SQLite/WAL or real Codex diagnostic sink tests. Choice2 and the finite manual-reattach tradeoff remain pending the four-critic planning review.
