# Interactive terminal scope after the native ESS sweep

Read-only story-scoper preparation, 2026-10-02. Inspected integration HEAD
`6a5aa99c14247f430ba2471392de35a44b6923e0`, incorporating main
`a7b26668a20dce232c5e068989b6818bdefa4171`; read the full
`design:codex-terminal-confidentiality` revision 2 and interactive story revision 4.
No implementation, builds, live requests, authentication or planning-store changes.
Only this private scratch report was written. The moving worker tree was not inspected.

## Finding

The sweep supplies useful real executable boundaries, but none currently proves the proposed
encrypted attachment. The design remains draft and its proposed volatile replay contract still
differs from the interactive story's phase-suppression prose. This report scopes the coordinator's
choice-2 proposal for review; it does not approve it or clear the capture blocker.

The narrowest plausible placement is a new public `terminal_protocol` module in the existing
`mantle-launch` library, shared by the worker attach implementation and laptop relay. The sweep
has already split launcher library/binary (`crates/mantle-launch/src/lib.rs:1`). The laptop can
depend on that existing crate; no new runtime crate is necessary for this placement. This is an
inference, not a selected architecture. The library must expose an implementation seam, not
couple laptop code to worker CLI parsing or process-global signal handling.

## Concrete source scope

Each listed path is exact; cited means an existing inspected owner, inferred means proposed
placement. This is the transport/recording slice of the eventual interactive story, not its
whole manifest/migration/auth/network scope.

Existing cited production and adapter paths:

- `crates/mantle-launch/src/cli.rs`: explicit attachment transport/public recipient and replay
  persistence policy; preserve default legacy argument meaning.
- `crates/mantle-launch/src/attach.rs`: bounded framed input/resize and encrypted output,
  complete-record budget ending, same single-client lock and inner FIFO boundary.
- `crates/mantle-launch/src/serve.rs`: selected volatile-only replay and no Codex last-output;
  keep the real PTY and existing bounded rings/redraw. Do not infer login phase.
- `crates/mantle-launch/src/lib.rs`: export the shared protocol module.
- `crates/mantle-launch/Cargo.toml`: reviewed crypto/model dependency bindings.
- `crates/mantle/src/app/terminal.rs`: incremental decoder, local private identity lifetime,
  explicit input/resize records, fixed errors and a truthful budget-ended result.
- `crates/mantle/src/app/session.rs`: agent-selected recording and attachment request, supported
  raw-pipe builder, actual output bound, public-recipient plumbing and exit presentation.
- `crates/mantle/src/adapters/orchestration.rs`: observe the real request's transport and bounds;
  its `Request` arm currently serializes `RunRequest` at lines 269-294.
- `crates/mantle/src/adapters/conformance.rs`: keep the CLI native boundary and its minimum
  accounting aligned if additional commands are registered there.
- `crates/mantle/Cargo.toml`: path dependency on the shared launcher library/model as needed.
- `Cargo.toml`: exclude a standalone generated crate, consistent with the worker model approach.
- `Cargo.lock`: actual reviewed dependency resolution.
- `crates/mantle-launch/tests/conformance.rs`: native launcher dispatch and actual process probes.
- `crates/mantle-launch/tests/support/probe.rs`: extend the existing Rust/clap fixture with finite
  canary/interactive observations, never a script or fixture-calculated pass verdict.
- `spec/domains/launch.yaml`: typed arguments/protocol/persistence decisions and native commands.
- `spec/domains/orchestration.yaml`: actual attachment request transport, bounds and public facts.
- `spec/components.yaml`: register added native commands on the existing components.
- `spec/ess-inputs.yaml`: enumerate exact new authored scenario files.
- `spec/conformance-baseline.json`: add required scenario IDs and raise achieved floors; retain
  every existing requirement and zero-skip ceiling.
- `spec/README.md`: describe the revised persisted/volatile boundary and native/live proof split.

New inferred implementation path:

- `crates/mantle-launch/src/terminal_protocol.rs`: shared bounded codec/encryption/decryption and
  budget accounting, with meaningful inline red-capable tests.

Inferred exact generated model placement, only after the types/generator probe is reviewed:

- `generated/terminal-model/Cargo.toml`
- `generated/terminal-model/types.rs`
- `generated/terminal-model/source.schema.json`
- `generated/terminal-model/types-report.json`
- `generated/terminal-model/.ess-output/state.json`

Do not reserve broad `generated/`, duplicate the worker model or hand-copy generated types.
If a suitable reviewed shared generated model exists after worker merge, rescope these five
paths to that concrete model before edits rather than keeping both speculative placements.

No current need to change `Taskfile.yml`: workspace tests execute native component adapters and
the check task runs the complete inventory aggregator (lines 72-79). No current need to change
the generic `mantle-conformance` crate: `Boundary` already supports typed command responses,
Bytes, lists and integer observations. No current need to change `ctl.rs`, `ring.rs`, `sys.rs`
or `session.rs`: the required bounded windows, replay ring and FIFO/lock helpers exist. Add a
specific helper path only when implementation demonstrates the need. Existing legacy launcher
tests remain requirements; new native cases can exercise the candidate without rewriting them.

## Existing cases to retain and extend explicitly

1. `spec/scenarios/cli/orchestration-attach-request.yaml` currently proves exact argv/environment,
   resource limits and no secret/aperture. It does NOT observe PTY versus pipes: `RunRequest`
   has no transport field and its private `pty` method chooses the transport separately
   (`app/session.rs:773`). Extend the actual request contract with the mode/output bound used by
   production and retain this legacy Claude case; add a Codex sibling with a fixed public test
   recipient. The test must not manufacture mode only in the adapter.
2. `spec/scenarios/cli/orchestration-agent-request.yaml`: retain the exact Claude slot/FD route;
   add a Codex sibling proving volatile-only replay selection, no Claude slot and fixed Codex
   launch arguments. Existing `exec-request` remains unchanged unless the shared request shape
   adds explicit fields, in which case augment its expected values without dropping assertions.
3. `spec/scenarios/launch/argument-defaults.yaml`, `argument-byte-preservation.yaml`,
   `attach-no-tty.yaml`: their ParseArgs snapshots must explicitly include newly served argument
   fields/defaults. Preserve Bytes/non-UTF8 coverage and legacy no-tty semantics. Add valid and
   rejected encrypted-attach argument cases rather than converting `--no-tty` into a security mode.
4. `spec/scenarios/launch/pty-fifo-lifecycle.yaml`: preserve its real PTY/FIFO/locking/descriptor
   behavior. The existing native fixture writes resize directly to ctl at
   `tests/conformance.rs:268`, bypassing attachment resize. A new encrypted case must send the
   framed resize through the actual attach process and observe the inner child's window.
5. `spec/scenarios/launch/slow-reader-bounded-output.yaml`: current adapter reads last-output at
   `tests/conformance.rs:437` and asserts its 262144-byte tail. Preserve this legacy persistent
   case. Add a Codex case that verifies live bounded replay/recovery and file absence; do not
   replace its expected retained byte count with zero or call absence sufficient replay evidence.
6. `spec/scenarios/launch/process-group-signals.yaml` and `os-argument-bytes-and-exit.yaml` also
   observe legacy last-output (`tests/conformance.rs:398`, `:199`). Retain them. Candidate signal
   and exit observations must come from the actual decrypted attachment while connected, plus
   process/cleanup facts; never re-enable last-output merely to inspect the test.
7. Preserve `bounded-scrollback`, `oversize-scrollback-write`, `window-fragmentation`,
   `window-oversize-discard`, `filesystem-links-and-locks` and all secret refusal cases. They
   remain useful underlying controls, but do not prove encryption, framed control delivery or
   Codex transcript non-persistence.

Proposed exact new authored files (inferred), each may contain several timeline observations:

- `spec/scenarios/cli/orchestration-codex-attach-request.yaml`
- `spec/scenarios/cli/orchestration-codex-agent-request.yaml`
- `spec/scenarios/launch/encrypted-attach-arguments.yaml`
- `spec/scenarios/launch/encrypted-record-validation.yaml`
- `spec/scenarios/launch/encrypted-input-controls.yaml`
- `spec/scenarios/launch/encrypted-attachment-lifecycle.yaml`
- `spec/scenarios/launch/encrypted-budget-end.yaml`
- `spec/scenarios/launch/encrypted-slow-reader.yaml`
- `spec/scenarios/launch/volatile-replay-no-last-output.yaml`
- `spec/scenarios/launch/encrypted-output-canary.yaml`

Malformed lengths/version, wrong key, truncated ciphertext, duplicate/reordered records and
unexpected terminal end need individually named assertions, not one hardcoded `secure: true`.
If ESS inventory granularity requires separate scenarios per refusal, split into exact files
and update typed scope before authoring. All existing required IDs remain in the baseline.

## Feasible typed nouns and executable commands (inferred)

Use value types, not invented persisted attachment entities or auth states:

- `mantle.launch.ReplayPersistence`: enum `PersistentLastOutput`, `VolatileOnly`.
- `mantle.launch.AttachmentTransport`: enum `Terminal`, `EncryptedPipes`.
- `mantle.launch.AttachmentEndReason`: enum distinguishing agent/stream completion, operator
  detach and budget end as actually observable; unexpected EOF remains a refusal, not success.
- `mantle.launch.TerminalOutputRecord`: version, stream identifier, sequence, kind, byte payload
  and optional end reason. Secrets are never generator examples; synthetic Bytes only.
- `mantle.launch.TerminalInputRecord`: kind plus keyboard Bytes or bounded Window, with invalid
  combinations refused by the real decoder. Reuse existing `mantle.launch.Window`.
- `mantle.launch.AttachmentBudget`: admitted ciphertext bytes and reserved terminal-record bytes;
  sequence/length arithmetic must narrow ESS Integer safely into the implementation widths.

Extend ParseArgs and orchestration.Request. Candidate native commands can be named
`ValidateTerminalRecords` (real codec observations and specific refusal),
`EncryptedAttachment` (real launcher + probe, local real decoder, input/resize/detach/budget), and
`ReplayPersistence` (real serve lifecycle and files). Reuse one real production codec from both
endpoints and the native adapter; avoid a second protocol implementation in the test harness.
Entropy-dependent ciphertext is not a fixed ESS golden string: assert actual decoded bytes,
specific error/result, recorded-byte count, canary absence and key separation. Private key
material must not be returned as a command response or included in native run artifacts.

## Bounded-loop constraints from actual source

- Serve has a 64KiB to-agent bound, 16KiB read buffer, 16 reads per turn, 100ms probe interval,
  200ms redraw hold and 1s final flush (`serve.rs:29-40`). Its scrollback is bounded by the CLI
  (default 256KiB, maximum 64MiB), with a separate pending ring at least 64KiB. A volatile policy
  removes the final file write; it does not eliminate these deliberate memory copies.
- Ring overflow discards oldest bytes and requests repaint (`serve.rs:399`, `:387`); it cannot
  be used for ciphertext queues, where dropping bytes would destroy framing/authentication.
- Current attach uses blocking `write_all` for stdout and FIFO input (`attach.rs:78`). Wrapping
  that call with encryption alone can stall resize/control processing indefinitely under a slow
  reader. Candidate output needs a bounded complete-record queue and partial-write offsets;
  avoid dropping records, unlimited buffering or starving signals/input/control handling.
- Current laptop terminal spawns a 64-entry input channel with 4096-byte reads
  (`app/terminal.rs:76`); its synchronous stdout writes can block the async select loop. A new
  decoder must retain bounded partial ciphertext and complete plaintext records while keeping
  the promised input/resize/backpressure behavior explicit. Never release unauthenticated partial
  plaintext to the operator terminal.
- The proposed 1MiB capture boundary must count every actual stdout byte, including headers,
  tags, framing and end record. Before reading more FIFO data, reserve a proven maximum complete
  data record plus complete ending. If it cannot fit, stop reading, finish accepted records,
  emit the authenticated budget-end and exit attachment only. Resolve zero-fit/minimum budgets,
  short writes, connection loss and full end-record validation as named cases.
- Native fixture `collect` currently grows an unbounded Vec (`tests/conformance.rs:140`), and
  its probe line accumulator is also unbounded (`tests/support/probe.rs:100`). These are finite
  in existing cases; new flood/malformed/slow-reader tests must cap collectors, input accumulation
  and waits rather than treating the test harness as evidence of production boundedness.
- Existing default Signal/Emit/SlowReader adapters rely on persistent last-output. A new volatile
  test must collect bounded live observations before child completion and inspect file absence
  afterward. Cover a preexisting last-output path explicitly; do not silently reuse a stale file
  or follow a hostile link. That safety behavior needs a reviewed exact contract.

## Proof frontier and remaining decisions

Native tests can prove actual frame codecs, local process/FIFO/PTY behavior, ciphertext-only
attach stdout, live replay, absence of launcher last-output, control delivery and orderly budget
termination. They cannot by themselves prove Substrate SQLite/WAL exclusion or real Codex's own
diagnostic safety. Keep those as real sink canary/live obligations. Budget termination is an
explicit finite attachment, not transparent continuation or exactly-once terminal history.
Review must decide if its reduced plaintext allowance satisfies intended parity.

The four-critic design/story review and worker-merge revalidation remain required. Auth cache,
actual agent identity migration, Codex config precedence and observed gateway hosts stay in the
larger interactive story; this report neither removes them nor claims them implemented.
