# Codex qualification under Substrate — 2026-10-02

## Private runtime preparation — CW01–CW09

The local `runtime-sinks` probe reverified pinned Codex 0.153.4 against SHA256
`56ef98ab4032d317ab26e9b5e5a175650717351edb16ed9cde0cb6d1734d62da`, then ran the real
launcher with a fresh synthetic home, cleared environment, volatile replay and tmpfs directories.
No operator credential/configuration was read; no login or model turn was requested. This local
observation does not change the confined qualification or authenticated limitations below.

The fixed `--strict-config`, ChatGPT/file authentication, `on-request` approval and explicit
`danger-full-access` outer-confinement candidate reached the welcome/login screen. A deliberately
conflicting ordinary user config selected a nonexistent provider, API auth, keyring, persistent
diagnostic paths, enabled history/analytics/feedback/updates and local endpoint overrides. Fixed
CLI settings kept the selected provider and endpoints at their ChatGPT values and the real TUI
remained alive across initial attach, reattach, resize and a bounded slow-reader pause. This is
ordinary config precedence evidence, not authenticated managed-policy evidence.

Observed real Codex sinks were `state_5.sqlite`, `logs_2.sqlite`, `goals_1.sqlite`,
`memories_1.sqlite`, `queue_1.sqlite`, their WAL/SHM files, and `codex-tui.log`, under the
launcher-verified tmpfs runtime root. The conflicting persistent sink paths and launcher
`last-output` were absent. An explicit synthetic SQLite fixture placed a canary in both its live
database and WAL; the bounded scanner found both and caught a deliberately planted persistent
diagnostic file before its removal. That fixture is independent of Codex authentication and does
not establish the behavior of an unexercised refresh or failure path.

Reproduce with the Rust `codex_qualification runtime-sinks` example, passing `--codex`, the
`--sha256` above, `--launcher`, an existing private `--scratch-parent`, an existing private tmpfs
`--runtime-parent`, and `--seconds 6`. The helper removes only its own disposable children and
reports observations without raw terminal bytes. Production requests use equivalent fixed
configuration with `/workspace/.mantle/home/.codex` for intended auth/conversation files and
`/tmp/mantle-codex/{sqlite,log}` for runtime diagnostics.

Mantle accepts and persists Codex/ChatgptDevice selection but still refuses start/attach before
credential or provider side effects. Supported Substrate non-recording capture, actual device
login, token refresh, authenticated tool approvals and lifecycle parity remain unverified and
belong to the parent integration story. No confidential end-to-end session is claimed here.

## Earlier confined qualification

Codex 0.153.4 reaches its unauthenticated interactive screen through the existing
Mantle PTY/FIFO launcher under Substrate 0.7.8 without relaxing confinement.
The default nested `workspace-write` tool sandbox fails its shell and Cargo controls
with a namespace error. This is a runtime-profile blocker, not an observed mandatory
Unix-listening-socket dependency. An explicitly selected outer-confinement-only candidate
runs both direct controls successfully while retaining all three negative confinement
observations. Authentication, a model turn, refresh and full terminal
usability are **not qualified** by these probes.

This report belongs to `story:codex-confinement-qualification`. It records the bounded
qualification deliverable; it does not claim the Codex parity epic or production support
is complete. Existing Claude sessions and the worker daemon were not restarted.

## Reproducible binary identity

The official [0.153.4 release](https://github.com/openai/codex/releases/tag/rust-v0.153.4)
was published 2026-09-04T23:25:48Z. The coordinator downloaded the official
`codex-x86_64-unknown-linux-musl.zst` asset and checked its digest against the release's
asset metadata:

| Item | Observed value |
|---|---|
| Compressed SHA256 | `c485e889611b73ff5c3cc11fb5cea7551ef504465ad8675163766b9b1a9ec84a` |
| Executable SHA256 | `56ef98ab4032d317ab26e9b5e5a175650717351edb16ed9cde0cb6d1734d62da` |
| Executable bytes | 258659424 |
| Executable version | `codex-cli 0.153.4` |
| Worker | Linux x86_64, Ubuntu 24.04, glibc 2.39, Substrate 0.7.8 |
| Worker kernel | `6.8.0-146-generic` |
| Existing launcher SHA256 | `4a66681c6a3f539f0f9d053d4dda1af770ddda704820b4246df8fac4d83931c2` |
| Existing daemon SHA256 | `024ee80d1e5046f09641df2bc739c4d6d35cc8e73c94639e2447a1cc9bfe73ec` |
| Live-tested helper SHA256 (before adversarial correction) | `c52cc6d1805ffc619b67e378945054c4d3700fd22b02aa3e27d60edf2aaed7a3` |
| Corrected helper SHA256 (local gates; not redeployed) | `1ffbe3d87627bbdeb6eb4a4d930321e74540995b958a1fef9212b47f5d7a8463` |
| Mantle implementation baseline | `11b69db7b86076299a9dd92cbbc2c0196148c007` |

The helper checks the executable digest before **any execution**, then checks the version
in a fresh private HOME. An all-zero SHA256 refused with exit 1 and
`binary SHA256 mismatch; binary was not executed`. A test with verification initially
stubbed to succeed failed first; it passed after implementing the verifier.

Independent review subsequently found that opening a FIFO binary path blocked before
the descriptor's regular-file check. The corrected verifier opens nonblocking, validates
the opened descriptor, and caps the actual stream at 512 MiB plus one overflow-detection
byte so file growth cannot evade the metadata size check. The retained FIFO case was
observed red and then green; additional controls cover directories, character devices,
exact-limit input and oversized streaming input without a large allocation. This input
validation correction was built and tested locally; the live observations below belong
to the earlier helper digest and were not rerun or relabelled as the corrected binary.

Only the official standalone executable was staged, under the probe-owned read-only root
`/opt/mantle-codex-qualification-wave2`. The installed local distribution also has
auxiliary files such as `code-mode-host`, `codex-path` and `codex-resources`; those were
not projected into the worker. The welcome-screen result does not prove those resources
are unnecessary for every authenticated tool.

## Probe and boundaries

`crates/mantle/examples/codex_qualification.rs` is a Rust/clap executable. Its `confined`
mode connects through a caller-owned Substrate tunnel, creates a new labelled workspace,
projects the staged helper and Codex read-only plus the existing `/opt/mantle` toolchain,
and requests **no aperture and no secret slot**. The helper's `local` mode runs where
invoked and never independently labels that environment confined.

The current bounded policy is 150 seconds wall time, 90 seconds cumulative CPU, 1 GiB
memory, 128 processes and 64 KiB SDK output. Earlier observations used 90 seconds and
60 seconds CPU. The TUI uses a 12-second observation schedule, bounded 256 KiB per phase,
8 KiB of in-memory text for fixed marker classification, and an 8 KiB launcher replay
buffer. A 64 MiB/4096-entry scratch budget is sampled during the TUI and between tool
controls; this is **not a filesystem quota** and can overshoot between samples. Each
benign tool control has a 20-second deadline and an 8 KiB bound per output stream.

Every run creates private 0700 scratch directories, clears the inherited environment,
uses the private HOME's `.codex`, and never loads laptop credentials or sends a login
selection, device code or model prompt. Only terminal cursor-position replies are sent.
Raw terminal bytes are not emitted in the report. The launcher's `last-output` remains
inside private scratch until the helper removes it unread. The driver destroys its own
workspace and records that result. It emits the owned workspace ID before dispatch so
an interrupted operator can recover that exact resource; it never destroys a supplied
existing workspace.

The documented default invocation is retained:
`--sandbox workspace-write --ask-for-approval on-request`, with update checks disabled
and the credential store set to file. The unauthenticated screen uses an intentionally
closed loopback proxy (`http://127.0.0.1:9`), which is **not** a candidate working CONNECT
policy. Substrate independently applied `network: none`.

Build and invoke the probe after staging its executable and the verified Codex binary in
a private worker root and opening a probe-owned tunnel:

```console
cargo build -p mantle --example codex_qualification
$CARGO_TARGET_DIR/debug/examples/codex_qualification confined \
  --socket "$PROBE_SOCKET" \
  --worker-root /opt/mantle-codex-qualification-wave2 \
  --sha256 56ef98ab4032d317ab26e9b5e5a175650717351edb16ed9cde0cb6d1734d62da
```

Use a separate target directory and the repository's bounded build settings. For the
explicit additional outer-confinement profile experiment, add
`--outer-profile-control`. It retains the default failure results, requests
`sandbox_mode="danger-full-access"` and `approval_policy="on-request"` only for separate
benign direct sandbox controls, and checks the same denied socket/network/host-path
observations before doing so. It does not use
`--dangerously-bypass-approvals-and-sandbox`, change Substrate, or demonstrate an actual
interactive approval prompt. The interactive TUI in this probe keeps `workspace-write`.

The worker host sentinel `/opt/mantle-codex-qualification-host-only.txt` was separately
created and observed readable (mode 0644, 29 bytes) outside all projected roots. The probe
only attempts to open it, never reads its content. The coordinator owns its teardown.

## Actual observations

The first confined run ended at `2026-10-02T09:08:37.486579519Z`, exec
`ex_01M3XXXZZ055XETGEAJSFFFCM4`, workspace `ws_01M3XXXZYJ1YN6SVFXZF2WA0C4`.
The second, including default nested tool controls, ended at
`2026-10-02T09:14:38.209041638Z`, exec `ex_01M3XY90H6QMVQ295KPF95NPA7`, workspace
`ws_01M3XY90F6H5YMKTQEQS5A0M5T`. Both execs exited 0, reported
`workspace-rw-system-ro`, `profile: workspace`, `network: none`, and their workspaces
were destroyed. Helper success means the observations were collected, not that all
qualification cases passed. Resource usage was not requested or observed.

The third run, including the explicitly selected candidate profile, ended at
`2026-10-02T09:20:38.301365894Z`, exec `ex_01M3XYKYJ6KDKVGZA4M7PWSGTB`, workspace
`ws_01M3XYKYHR8XZAC7KJBR3D0D59`. It retained the same applied outer filesystem/network
profile, exited 0 and destroyed its workspace. Default tool controls still failed;
candidate shell and Cargo controls each exited 0 and the Cargo library artifact existed.
Negative controls launched through that candidate returned the same socket errno 13,
direct-network errno 101 and host-sentinel errno 2. These observations support selecting
an outer-confinement profile for subsequent authenticated qualification. They do not
prove the independent `on-request` setting produces a visible interactive prompt.

| Control | Confined observation | Local unconfined comparison |
|---|---|---|
| `UnixListener::bind` in private scratch | refused, errno 13 (`EACCES`) | permitted |
| Direct TCP to `1.1.1.1:443` | errno 101 (`ENETUNREACH`) | permitted |
| Open known host-only sentinel | errno 2 (`ENOENT`) | absent locally; not isolation evidence |
| Initial real Codex screen | welcome/login markers, cursor query, launcher alive | same markers |
| Same-size FIFO reattach | welcome/login markers, cursor query, same launcher alive | same markers |
| Resize from 100×30 to 80×24 | emitted 2201 bytes and login marker in second run | emitted login marker |
| 500 ms reader pause | emitted 1584 bytes; launcher alive | emitted bytes; launcher alive |
| Default shell via `codex sandbox` | exit 1; namespace-error marker; 105 stderr bytes | not run as acceptance |
| Default Cargo via `codex sandbox` | exit 1; namespace-error marker; 105 stderr bytes; artifact absent | not run as acceptance |
| Explicit outer-only shell/Cargo controls | both exit 0; Cargo library artifact exists | not run outside confinement |
| Negative controls under outer-only child | Unix socket 13; direct TCP 101; host-path open 2 | not run outside confinement |

The shell control is `/bin/sh -n /dev/null`. Cargo builds a freshly generated, dependency-free
Rust library with `build --offline --quiet -j 1`. Both are direct calls through the pinned
Codex sandbox CLI, **not model-issued tool calls**. This release's documented CLI is
`codex sandbox [COMMAND]...`; it has no `linux` subcommand. The exact fixed classification
is exit 1 plus a namespace-error marker, without a timeout, missing-file marker or output
truncation. The probe does not retain arbitrary tool stderr.

Pinned [TUI source](https://github.com/openai/codex/blob/rust-v0.153.4/codex-rs/tui/src/lib.rs#L861)
provides an embedded app-server fallback when no explicit remote or reachable daemon is
selected. This agrees with the live screen observation; it does not prove later authenticated
features never require IPC. Substrate's pinned seccomp implementation permits stream
socketpairs for process-local IPC while refusing Unix listening sockets and repurposable
datagram socketpairs. The local integration tests model the `socket(AF_UNIX)` boundary
only, and are explicitly separate from these live worker observations.

## CQ acceptance matrix

| Case | Result and exact remaining obligation |
|---|---|
| CQ-01 | **Pass** for official standalone binary identity, local/worker version and digest match, and incorrect-digest refusal before execution. |
| CQ-02 | **Observed partial pass**: real isolated Codex welcome/login screen through existing launcher under unchanged confinement; forbidden Unix-listening socket refused. Authenticated conversation screen remains unobserved. |
| CQ-03 | **Profile qualification partial**: both default nested controls exit 1 with namespace error. Explicit outer-only shell/Cargo controls exit 0 with a real library artifact; outer host/socket/network refusals remain. Model-issued shell/build and interactive approval behavior remain not run without authentication. |
| CQ-04 | **Not run** for device authentication, model turn, refresh, HTTPS, WSS/fallback and unlisted-host CONNECT refusal. No disposable working gateway was supplied and no operator device login occurred. Direct-egress refusal was observed separately. |
| CQ-05 | **Partial evidence only**: real TUI output after same-size reattach and resize; process survives a bounded slow reader. Marker/byte counts do not establish usable rendered screen, authenticated conversation preservation, or forced ring overflow recovery. |
| CQ-06 | **Not run**: no credentials were acquired, expired, revoked or refreshed. No renewal claim is made. |

Candidate service hosts remain `auth.openai.com:443` and `chatgpt.com:443`; neither was
admitted or traffic-qualified by this no-egress experiment. `api.openai.com:443` remains
conditional, not an approved requirement. No wildcard or production gateway change was
made. The next authenticated qualification must separately observe HTTPS and WSS or its
fallback, test an unlisted hostname, exercise refresh/revocation, and keep login output out
of replay/diagnostic artifacts.

## Evidence and tests

The coordinator retains the final structured live observation at
`.engineering/reports/codex-qualification-2026-10-02.json` and official release metadata
at `.engineering/reports/codex-release-0.153.4.json` on the integration branch.

Private raw **non-credential** runner logs and structured observations are retained under
`$HOME/.cache/mantle-wave2/scratch-qualification/` and `scratch-int/`. The relevant producer
files are `before.log`, `red.log`, `control-debug.log`, `after.log`, `example-green.log`,
`clippy.log`, `fmt.log`, `wrong-digest.log`, `local.json`, `confined-probe-1.json`,
`confined-probe-2.json`, `confined-probe-3.json` and `codex-release-metadata.json`. No raw TUI/login transcript is
retained. Personal filesystem prefixes are deliberately represented by `$HOME` here.

The new launcher lane is a local negative control, not a substitute for the real CLI run.
Its first fixture incorrectly denied stream socketpairs as well as socket creation, which
prevented Rust's child-spawn IPC. The coordinator verified the pinned Substrate policy and
approved correcting that new fixture; the original failure is retained in
`control-debug.log`. Existing production assertions and confinement were not weakened.

Before adversarial review, `cargo test -p mantle -p mantle-launch` executed **79 → 87**
top-level cases with exit 0 after the coordinator registered the example with `test=true`.
The added cases were six example controls and two launcher controls. Review added the
failing FIFO case: 88 executed, 87 passed. After the correction and two additional input
boundary cases, `cargo test -p mantle -p mantle-launch --no-fail-fast` executed **90**
top-level cases with exit 0, including nine example controls. The FIFO and socket controls
each re-execute a child and print nested one-test summaries; these are not counted twice.
Package clippy with `--all-targets -- -D warnings`, formatting and diff
whitespace checks also passed. These checks validate the probe and launcher controls;
they do not fill the explicitly unobserved authenticated CQ requirements.
