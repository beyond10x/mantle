# Interactive-start preparation

Read-only preparation while worker wave3 runs. This is not authentication acceptance,
not a production host allowlist, and not a revised implementation grant. Revalidate
against the completed worker merge before the next wave selects source ownership.

## Observed seams

Source: independent `interactive_scope_refresh` story-scoper against
`7f187c9f7d14cf8962c45171718af695f7b69f5f`, following the full AEP procedure.
The agent changed nothing and did not inspect the moving implementation tree.
These are coordinator extracts from its report, not an immutable adversary verdict.

- `crates/mantle/src/app/session.rs` owns selection, start, requested facts, failure
  persistence and status. Manifest/resolved domain structs currently assume Claude.
- `crates/mantle/src/adapters/state.rs` schema initialization only creates missing
  tables. Real agent/auth columns need an actual SQLite migration, insert/decode
  changes and legacy-row tests, together with both ESS views and complete adapter rows.
- `crates/mantle-launch/src/serve.rs:399` adds every PTY read to scrollback;
  `:494` replays it; `:153` persists last-output. Omitting the Claude descriptor
  alone cannot protect a Codex login transcript. Initial-only suppression cannot
  establish safety after expired/revoked credentials or a later login flow.
- Launcher ownership includes cli.rs, serve.rs, session.rs and existing
  `crates/mantle-launch/tests/launch.rs`, rather than a speculative new CLI test target.
  Authentication-path safety must validate parent components too.
- `app/session.rs:340` reads early launcher stderr; failures can persist formatted
  errors. Non-secret bounded error handling must precede DB/status, not only replay.
- Network policy lives in `crates/mantle-egress/src/allow.rs` and existing real
  CONNECT refusal tests. Current CQ report leaves auth/refresh/HTTPS/WSS untested.
- Adjacent ESS manifest/launch/egress domains, components.yaml, ess-inputs.yaml and
  actual conformance.rs need coordinated changes with Session. No invented fields
  only in the adapter. New scenarios likely belong in spec/scenarios/codex-start.yaml.
- Concrete proposed generated crate: generated/session-model with Cargo.toml,
  types.rs, source.schema.json and types-report.json. The scoper initially included
  .ess-output/state.json; subsequent actual worker generation showed it is operational
  location/inode metadata and must remain ignored, not committed. Generator roots and
  cross-crate consumers remain unproven; probe before reserving final scope.
- Remove stale worker-provisioning/config/daemon reservations only after inspecting
  the worker merge. Add actual selected readiness consumption, not a second installer.

## Pinned Codex source facts

Official current [authentication documentation](https://learn.chatgpt.com/docs/auth)
describes device-code sign-in for remote terminals and file-backed credentials under
CODEX_HOME. Account/workspace settings must enable device login. Documentation is
guidance; the pinned candidate and live evidence determine Mantle behavior.

Inspected upstream source is tag `rust-v0.153.4`:

- [device_code_auth.rs](https://github.com/openai/codex/blob/rust-v0.153.4/codex-rs/login/src/device_code_auth.rs):
  request obtains a device code; the CLI prints it directly; completion polls and
  exchanges credentials before persistence. The flow has a fifteen-minute poll limit.
  No real code was requested in this investigation.
- [cli/src/login.rs](https://github.com/openai/codex/blob/rust-v0.153.4/codex-rs/cli/src/login.rs):
  explicit device login clears previous auth before starting and exits0 only after
  success. Bare headless login has a browser fallback; explicit --device-auth is the
  relevant path under confinement. Direct login also initializes a private diagnostic
  file. Its sinks require examination rather than assuming terminal suppression covers it.
- The same source's login-status path loads cached auth and identifies its method;
  it does not by itself prove a live model turn or an accepted refresh. Do not turn
  file existence or a cache-status result into authenticated-readiness evidence.

Local pinned binary `codex login --help` confirms --device-auth and login status.
Only help was invoked; host credentials were not read or copied. Private downloaded
source excerpts live in wave3 scratch-int, alongside the source filenames and version.

## Unresolved design and acceptance

The launcher has no authoritative authentication-phase signal today. A deterministic
login subprocess boundary is a candidate design inference, not yet selected or proven;
it must also handle renewed login, detach/reattach and preserve useful ordinary replay.
Auth-cache existence alone cannot close that boundary or its races. Blanket removal
of Codex replay is not silently equivalent to the accepted experience.

Full CS acceptance still needs an operator-completed private login, real model/repository
turn, refresh/revocation behavior, exact observed destinations, retained confinement,
credential canaries, AGENTS.md observation and correct cleanup. Nothing here discharges
those obligations. Four planning critics must judge material refinements before dispatch.

## Downstream terminal capture blocks CS-05

The independent scoper traced pinned Substrate revision `0569597`; the coordinator
then inspected the capture, output-limit, persistence and retirement functions.
Paths below are relative to that Substrate source checkout. This is source evidence,
not a runtime canary result; no device code was requested.

1. Mantle `app/session.rs:357` opens `pty_session` for `mantle-launch attach`.
   Mantle `app/terminal.rs:111` writes the received bytes to local stdout.
2. `crates/substrate-host/src/process.rs:2184` captures merged PTY output.
   `drain_capped` retains bytes at `:2774` before forwarding those same bytes to
   the live channel at `:2789`. At `:2297` they become `observation.stdout`.
3. `crates/substrate-daemon/src/app/sessions.rs:1844` persists the terminal
   observation, including attachment loss at `:1893`. `app/operations.rs:1125`
   clones the output into `StoredExec`.
4. `crates/substrate-store/src/execs.rs:733` upserts stdout/stderr BLOBs;
   `schema.rs:136` declares the columns and `:38` enables SQLite WAL.
5. Retirement requires an already durable terminal session (`sessions.rs:297`)
   and subsequently deletes the exec row (`:383`). It cannot establish that
   output was never persisted or securely erased.

Mantle requests a 1 MiB bound, which limits capture rather than disabling it.
Zero is refused in host `process.rs:1833`. Inspected event and WebSocket tracing
paths carry metadata; that does not negate the separate stdout BLOB.

`coordination-blocker:codex-terminal-capture` records the clearance requirement.
Inference: protecting only launcher scrollback cannot satisfy CS-05 on this
transport. A Mantle-only encrypted attachment is being scoped as a candidate,
not selected: ephemeral laptop key material, public recipient metadata, framed
ciphertext through Substrate and decryption only at the operator terminal.
It still needs reviewed framing, limits, input/reattach behavior, raw TTY behavior,
key lifetime, failure handling and an independent canary proving every sink.
It does not solve the launcher's separate authentication-phase/replay boundary.
No Substrate change, crypto implementation or new credential exception is authorized
by this preparation record.

## Login diagnostic sinks need an explicit bound

Additional pinned upstream source inspected read-only:
[login/server.rs](https://github.com/openai/codex/blob/rust-v0.153.4/codex-rs/login/src/server.rs).
`exchange_code_for_tokens` at line809 logs sanitized URL fields, but its non-success
branch at line863 logs parsed backend `error_code` and `error_message` and returns a
display error. `parse_token_endpoint_error` at line1011 preserves a non-JSON body
for display, as its comment explicitly states. Direct device-login CLI at line360
prints the returned error to stderr; its logging initializer at line55 creates
`codex-login.log` with mode0600 and a default info filter.

Observed: success-path token fields are not directly included in these inspected
logging statements. Inference: the complete arbitrary-error-body path cannot be
treated as safe for generic Mantle errors, launcher last-output or diagnostic
evidence. A failed-login canary must cover it, and the selected design must govern
Codex's own diagnostic sink as well as the terminal. This source inspection neither
proves a real credential leak nor grants an additional persistent-secret exception.

## Candidate attachment feasibility, not design selection

Follow-up independent scoper inspection found the supported SDK entry point
`Workspace::pipe_session` at SDK `lib.rs:674`; `pty_session` at `:695` changes
that builder's mode. The inspected builder, execution policy and closed wire start
input expose no no-capture option. Pinned Substrate ADR0007:22,38 explicitly assigns
machine protocols to raw pipes. Existing outer PTY termios starts in default mode
and becomes raw only in launcher attach; input can therefore echo before the switch.

An encrypted raw-pipe candidate has these concrete implications:

- Launcher `attach.rs:67,78` and laptop `terminal.rs:111` are the output boundaries.
  Encrypt before every attach stdout write; keep private decryption material only
  in the laptop process. Do not silently fall back to plaintext on any error.
- Native SDK resize refuses pipes (`lib.rs:1445`). Frame initial and changed window
  sizes and translate them to existing launcher `ctl.rs` control messages; retain
  inner PTY keyboard semantics and local detach behavior.
- Substrate's 1,048,576-byte output bound counts ciphertext too. Unlike PTY drain,
  pipe drain receives no output-bound flag, so it can stop forwarding while the
  process remains apparently live (`host/process.rs:2190,2215,2229,2744`). The
  truncation frame arrives only with terminal observation (`app/sessions.rs:1811`).
  A complete protocol must prevent partial-frame budget exhaustion and define
  truthful attachment continuation without losing terminal output or ending the agent.
- Reattach currently replays plaintext scrollback (`serve.rs:490`). A new local key
  can encrypt that replay afresh; it cannot remove login data already put in the ring.
- Inspected stdin transport decodes/forwards input and counts bytes, with no durable
  stdin payload sink found. This is a bounded source trace, not runtime proof.

Primary crypto references examined by the scoper: [age Rust0.12.1 API](https://docs.rs/age/0.12.1/age/)
and [age format](https://age-encryption.org/v1). A never-finished age file is not an
established interactive solution: the format uses64KiB chunks and requires a final
authenticated chunk. Independently finalized bounded messages are a candidate only;
measure latency/overhead and define sequence, ordering, fragmentation, EOF, corruption,
wrong-key and allocation limits. Successful decryption alone proves neither ordering
nor transcript completeness. Ciphertext retention must be explicit in a reviewed
CS-05 decision; it is not a claim that Substrate records nothing.

Additional cited source scope if selected: launcher attach/CLI/Cargo manifest,
laptop terminal/session/Cargo manifest, Cargo.lock, launch domain and existing tests.
A shared transport module/model remains inferred; no broad directory scope was reserved.
Worker-merge revalidation and four planning critics still precede implementation.

## Pinned TUI reauthentication paths

A final bounded scoper pass narrowed the automatic re-login concern:

- `tui/src/chatwidget/slash_dispatch.rs:410` emits Logout; successful handling in
  `tui/src/app/event_dispatch.rs:708` logs out and exits with ShutdownFirst. Failure
  remains in the TUI with an error.
- `login/src/auth/manager.rs:1818` recovers by reload, token refresh, external refresh
  and done. Expired/revoked-token messages at line191 instruct logout/sign-in; the
  inspected managed recovery does not request a device code.
- `tui/src/app/app_server_events.rs:209` updates account/status on AccountUpdated;
  the inspected handler does not open onboarding.
- `tui/src/lib.rs:1123,1159,1722` reads startup account state and runs onboarding
  before the main App. Device login begins in `onboarding/auth.rs:974`; completion
  notifications are handled at line983.
- `tui/src/app_server_session.rs:677` reads account with refresh_token=false. Cached
  account recognition is not a live service-authentication test.

These paths are from official tag rust-v0.153.4, including
[auth manager](https://raw.githubusercontent.com/openai/codex/rust-v0.153.4/codex-rs/login/src/auth/manager.rs)
and [TUI onboarding](https://raw.githubusercontent.com/openai/codex/rust-v0.153.4/codex-rs/tui/src/onboarding/auth.rs).
This is not an exhaustive proof covering nested user commands/plugins or every error
path. A separate login child gives a real completion boundary only for that child;
the ensuing TUI still independently decides whether to open startup onboarding.
Never infer an authoritative replay boundary from auth.json presence.

The candidate choices and required review are now tracked in draft
`design:codex-terminal-confidentiality`. The blocker remains open and CS-05 unchanged.
