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

Generator `.ess-output/` operational state is ignored, not a proposed tracked artifact. This
corrects the initial placement list after design revision 3 and the worker generation evidence.

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

## Proposed ESS amendment after worker merge (text only)

Refreshed against `247ed1715ac24bba0c4188354d5aac3c02d6c26a`, design revision 3. Read the
ESS specifying procedure plus syntax/later-formats references. Baseline read-only checks:
`ess specify validate --path spec` exited 0: `mantle v1 — 7 file(s), 218 scenario(s), valid`;
`ess specify compile --path spec --format json` exited 0 with output discarded. The proposal
below has NOT been applied or validated; the coordinator must validate/type-generate it before
implementation. No generated/runtime model is hand-transcribed here.

### Proposed reviewed constants and byte semantics

These numerical choices are proposals so tests have falsifiable boundaries, not established
library measurements: protocol version 1; terminal payload at most 4096 bytes per output record;
clear length prefix exactly four unsigned big-endian bytes; encrypted body at most 32768 bytes;
one complete finalized age file per body; plaintext envelope at most 24576 bytes; fixed single
X25519 recipient; output cap 1048576 bytes; reserve 4096 bytes INCLUDING framing for the encrypted
End record. A minimum accepted attachment budget of 36868 bytes allows one maximum wire record
(32772 bytes) plus the reserved End record. The production path always requests the fixed 1MiB
cap; the smaller budget is a bounded fixture/configuration seam, not a user override of Substrate.

Use fresh stream identifier per attachment, 32 lowercase hexadecimal characters, supplied as a
non-secret expected value to both sides. Sequences start at zero, are contiguous and checked for
overflow. Encrypted records contain version, stream identifier, sequence, kind, payload and end
reason. End has empty payload and exactly one reason. Output has nonempty payload and no reason.
Decode/verify an entire record before releasing its payload; clear length never authorizes an
unbounded allocation. Unknown versions/kinds/extra envelope fields are refused. The actual ESS
generated serialization shape (especially Bytes) must determine exact wire encoding; do not
invent a second handwritten model or assume the generator serializes Bytes as a particular JSON
shape. The proposed maxima must be proved against that generated shape plus the pinned age crate.

Input is also length-delimited and bounded, with version/sequence/kind, keyboard Bytes or Window.
The first record must be InitialWindow; later Resize records use the same 1..1000 cell bounds.
Keyboard records preserve all byte values. Operator detach is handled locally before framing,
as today. Do not translate Ctrl-C/Ctrl-D into a process kill or raw-pipe half-close. Unexpected
input EOF terminates only attachment; no promise of flushing keyboard data after operator detach.

### `spec/domains/launch.yaml` additions

Append the following entries to the EXISTING types/commands lists, not duplicate top-level keys.
They model values/native boundary observations only; no stored entity or fabricated lifecycle.

```yaml
# types additions
- name: mantle.launch.ReplayPersistence
  kind: enum
  variants: [PersistentLastOutput, VolatileOnly]
- name: mantle.launch.AttachmentTransport
  kind: enum
  variants: [Terminal, EncryptedPipes]
- name: mantle.launch.OutputRecordKind
  kind: enum
  variants: [Output, End]
- name: mantle.launch.AttachmentEndReason
  kind: enum
  variants: [BudgetEnded, AgentStreamClosed]
- name: mantle.launch.InputRecordKind
  kind: enum
  variants: [InitialWindow, Keyboard, Resize]
- name: mantle.launch.TerminalOutputRecord
  kind: struct
  fields:
    - {name: version, type: Integer}
    - {name: stream_id, type: String}
    - {name: sequence, type: Integer}
    - {name: kind, type: mantle.launch.OutputRecordKind}
    - {name: payload, type: Bytes}
    - {name: end_reason, type: 'Optional<mantle.launch.AttachmentEndReason>'}
- name: mantle.launch.TerminalInputRecord
  kind: struct
  fields:
    - {name: version, type: Integer}
    - {name: sequence, type: Integer}
    - {name: kind, type: mantle.launch.InputRecordKind}
    - {name: payload, type: Bytes}
    - {name: window, type: 'Optional<mantle.launch.Window>'}
- name: mantle.launch.TerminalRefusal
  kind: enum
  variants: [Version, Length, Key, Authentication, Stream, Sequence, Shape, Truncated, MissingEnd, TrailingRecord, Budget, Window, InputOrder, StaleOutput]
- name: mantle.launch.RecordFault
  kind: enum
  variants: [None, UnknownVersion, OversizeLength, ZeroLength, WrongKey, CorruptCiphertext, WrongStream, FirstSequenceOne, Duplicate, Reordered, PartialPrefix, PartialBody, MissingEnd, RecordAfterEnd, SequenceOverflow]
- name: mantle.launch.AttachmentProbe
  kind: enum
  variants: [Lifecycle, BudgetEnd, SlowReader, InputControls, Canary]
- name: mantle.launch.StaleOutputKind
  kind: enum
  variants: [Absent, RegularFile, Symlink]

# commands additions: adapter creates fresh private fixture identity internally, never returns it.
- name: mantle.launch.ValidateTerminalRecords
  input:
    - {name: payload, type: Bytes}
    - {name: fault, type: mantle.launch.RecordFault}
    - {name: fragment_bytes, type: Integer, example: 1}
  response:
    - {name: delivered, type: Bytes}
    - {name: end_reason, type: 'Optional<mantle.launch.AttachmentEndReason>'}
    - {name: refusal, type: 'Optional<mantle.launch.TerminalRefusal>'}
    - {name: plaintext_in_wire, type: Boolean}
    - {name: peak_buffer_bytes, type: Integer}
  outcomes: [{name: returned, returns: true}]
- name: mantle.launch.EncryptedAttachment
  input:
    - {name: probe, type: mantle.launch.AttachmentProbe}
  response:
    - {name: output_matches, type: Boolean}
    - {name: control_observations_match, type: Boolean}
    - {name: observed_windows, type: 'List<mantle.launch.Window>'}
    - {name: input_refusals, type: 'List<mantle.launch.TerminalRefusal>'}
    - {name: keyboard_bytes, type: Bytes}
    - {name: interrupt_observed, type: Boolean}
    - {name: terminal_eof_observed, type: Boolean}
    - {name: replayed, type: Boolean}
    - {name: agent_survived, type: Boolean}
    - {name: exclusive_attach, type: Boolean}
    - {name: last_output_exists, type: Boolean}
    - {name: plaintext_in_capture, type: Boolean}
    - {name: bytes_accepted, type: Integer}
    - {name: bytes_delivered, type: Integer}
    - {name: wire_bytes, type: Integer}
    - {name: budget_observed, type: Boolean}
    - {name: end_reason, type: 'Optional<mantle.launch.AttachmentEndReason>'}
    - {name: within_deadline, type: Boolean}
    - {name: bounded_buffers, type: Boolean}
  outcomes: [{name: returned, returns: true}]
- name: mantle.launch.ObserveReplayPersistence
  input:
    - {name: policy, type: mantle.launch.ReplayPersistence}
    - {name: stale_output, type: mantle.launch.StaleOutputKind}
  response:
    - {name: started, type: Boolean}
    - {name: replayed, type: Boolean}
    - {name: last_output_exists, type: Boolean}
    - {name: stale_target_unchanged, type: Boolean}
    - {name: refusal, type: 'Optional<mantle.launch.TerminalRefusal>'}
  outcomes: [{name: returned, returns: true}]
```

Register `mantle.launch.ObserveReplayPersistence` in components and every corresponding scenario;
the differently named `ReplayPersistence` is its value type. All three commands call actual production codec/launcher code.
Probe enums select fixed finite fixtures, never expected answers; every response field comes
from measured bytes/process/files. Invalid automatic synthesized examples must yield a typed
refusal/observation, not panic. Define fixture-safe scalar examples before synthesis.

Extend ServeArgs with `replay_persistence: mantle.launch.ReplayPersistence`, default
PersistentLastOutput. Extend AttachArgs with `transport`, optional `recipient`, optional
`stream_id`, and `output_budget_bytes`; validate required/forbidden combinations through clap.
No identity/private key field belongs to ESS argv/request/response fixtures. Do not claim
these optional fields permit plaintext fallback. Existing ParseArgs snapshots gain explicit
default fields; all preexisting acceptance remains.

### Request mode and exact existing-file changes

In `spec/domains/orchestration.yaml`, extend `Request` input with optional public recipient and
stream identifier; select agent from its already-supplied manifest. Extend response with
`channel: mantle.orchestration.RunChannel` (`Exec`, `Pty`, `Pipes`) and `output_bytes: Integer`.
These must be fields of the real request descriptor used by the SDK-building method, not facts
manufactured in the conformance serializer. `agent`/`exec` retain Exec; legacy attach retains Pty;
Codex attach is Pipes. The Codex agent request includes the explicit VolatileOnly launcher mode.
All old root/aperture/slot/FD/argv/env/lease/resource assertions remain.

`spec/components.yaml` registers the three new launch commands. `spec/ess-inputs.yaml` adds the
ten exact inferred scenario paths already listed above. `spec/conformance-baseline.json` keeps
all existing IDs/floors and adds the new synthesized/authored IDs once actually generated.
Do not lower counts, introduce skips or refresh recorded successes without real native runs.

### Exact named acceptance expectations in those ten files

Each file remains one authored scenario with explicit timeline steps. The slash names below
identify its separately asserted steps; do not collapse them into an unqualified aggregate pass.

| File stem | Named expectations |
| --- | --- |
| orchestration-codex-attach-request | `codex-pipes`: channel Pipes, output_bytes 1048576, no secret slot/FD/aperture, public fixture recipient/stream id only, argv selects encrypted attach. `legacy-pty`: old Claude request unchanged except explicit channel Pty/output cap. `private-absent`: no private identity input/output. |
| orchestration-codex-agent-request | `codex-volatile`: Codex executable, fixed private home, VolatileOnly flag, no Claude slot/FD/token. `legacy-claude`: existing exact Claude request remains. |
| encrypted-attach-arguments | `valid`: encrypted mode plus recipient/stream/cap accepted; `missing-recipient`, `bad-recipient`, `missing-stream`, `bad-stream`, `too-small-budget`, `over-cap`, `incompatible-no-tty`: refused before opening FIFOs; `legacy-defaults`: unchanged default mode. No diagnostic echoes raw supplied recipient garbage. |
| encrypted-record-validation | Payload `Q01FLUNBTkFSWS0wMQ==` (synthetic CME-CANARY-01). `complete-byte-fragmented`: delivered exact input, AgentStreamClosed, no refusal, plaintext_in_wire false. `unknown-version`: Version; `oversize/zero-length`: Length; `wrong-key`: Key; `bit-flip`: Authentication; `wrong-stream`: Stream; `first-sequence-one/duplicate/reordered/overflow`: Sequence; `partial-prefix/body`: Truncated; `missing-end`: MissingEnd; `record-after-end`: TrailingRecord. No payload from an invalid record is delivered; already-verified earlier records remain valid observations. |
| encrypted-input-controls | `initial-window`: real child observes 80x24; `resize`: framed 120x40 reaches child; `binary-input`: all byte values preserved through protocol (inner raw-mode fixture); `ctrl-c`: real child observes signal without ending server; `ctrl-d`: terminal EOF behavior preserved; `bad-window/input-before-initial/duplicate-initial`: Window/InputOrder refusals; `local-detach`: Ctrl-] d not forwarded. |
| encrypted-attachment-lifecycle | `attach-detach-reattach`: exact known output and bounded replay survive fresh identities; `same-size-redraw`: observable repaint; `single-client`: competing attach refused; server/agent survive detach; FIFO modes stay0600. After final agent exit, last_output_exists false; no decrypted capture persisted. |
| encrypted-budget-end | Fixture budget36868, infinite-beyond-budget FINITE watchdog-controlled source. Authenticated BudgetEnded record fits the admitted total; wire_bytes<=36868; bytes_delivered==bytes_accepted; no FIFO bytes accepted without room for their full encrypted record+ending; agent_survived true; exclusive lock released; later manual fresh-key attach replays current bounded screen. No automatic reconnect. Truncated/EOF alone is not BudgetEnded. |
| encrypted-slow-reader | Pause consumer500ms, then drain; bounded_buffers true; within_deadline true (5s completion/recovery watchdog); frame authentication/order retained; input/resize delivered when output can resume; no lost accepted record. A permanently blocked consumer yields bounded teardown, never a successful complete ending. Short prompt arrives within1s under an otherwise idle local fixture. |
| volatile-replay-no-last-output | `fresh`: started/replayed true, last_output_exists false during/after exit. `stale-regular/stale-symlink`: proposed refusal StaleOutput before child launch; stale_target_unchanged true; do not unlink or overwrite. `legacy`: existing persistent behavior and permissions unchanged. |
| encrypted-output-canary | Real fixture emits synthetic canary, laptop decoder observes it, raw captured stdout/stderr/launcher files exclude it; private fixture identity absent from argv/env/reports. Native response stores only equality/digest/count findings. Substrate DB/WAL and actual Codex diagnostic exclusion remain separately required live tests. |

Dynamic counts/peaks cannot be guessed as exact constants. Native observations may additionally
return derived bound/equality booleans above, computed from actual counters/bytes; preserve raw
non-secret counts in results. Authored expectations compare concrete payloads and refusal codes,
and assert the defined bounds/equalities. If current ESS scenario syntax cannot express numeric
inequalities, assert a typed production boundary decision plus actual counts, never change the
native runner into a predicate evaluator. Full YAML scenario materialization is still required.

### Remaining mechanisms that must be settled before claiming a complete contract

1. `UNMAPPED[wire-representation]`: generator Bytes/union/numeric serialization and strict decoder
   behavior must be probed before the proposed maxima or exact wire v1 is binding. The contract
   text is falsifiable, but its generator/runtime mapping is unverified. Generate portable models
   with ESS; never hand-transcribe them. Ignore `.ess-output/` operational state.
2. `UNMAPPED[budget-proof]`: prove a maximum encrypted data/end size for the pinned age dependency
   and generated envelope. If 32768/4096 bounds fail, revise reviewed constants and tests BEFORE
   implementation dispatch. Do not encrypt/read arbitrary extra FIFO data and drop it afterward.
3. `UNMAPPED[slow-reader-end]`: choose explicit finite partial-write/teardown deadlines and observe
   their interaction with output backpressure; a complete ending cannot be guaranteed to a reader
   that never reads. Distinguish budget ending from transport failure; no success claim on timeout.
4. `UNMAPPED[volatile-mode-selection]`: public attach-by-name must recover the actual stored agent
   and always select encrypted mode for Codex. A direct legacy launcher attach on the same FIFO is
   outside that selected path; decide how server metadata/argument checks prevent accidental
   downgrade in supported commands. Do not promise same-uid malicious-process isolation.
5. `UNMAPPED[stream-end-observation]`: FIFO closure proves the agent stream closed, not the remote
   agent exit status. Keep reason AgentStreamClosed; status must use actual Substrate observation.
6. Design choice2/privacy prose and finite manual budget reattach remain pending critic judgment;
   real Codex diagnostics and Substrate durable sinks are not covered by a native codec test.

Minimal exact files remain those in this report; no new broad directory reservation. The intended
shared module is still `crates/mantle-launch/src/terminal_protocol.rs`; compiled generated model
location is the four portable `generated/terminal-model/` files above, subject to the ESS probe.
The existing Taskfile already runs all native component gates and inventory reconciliation.
