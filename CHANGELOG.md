# Changelog

All notable changes to Mantle. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Evidence and compatibility notes for each release are on the
[release history](https://beyond10x.github.io/mantle/history/) page; notes and assets are on
[GitHub releases](https://github.com/beyond10x/mantle/releases).

## [Unreleased]

### Added

- `agent.kind: command` with `agent.argv`: the session's process is the declared argv, run
  directly with no shell; the first element must be an absolute path, and the argv may have at
  most 243 elements, each non-empty, at most 4096 bytes and without NUL. A command session
  receives no agent credential and refuses `agent.auth`. `mantle list` shows it as agent
  `command`, auth method `none`; `mantle status` prints its `argv`, and `list --json` and
  `status --json` carry it as `argv`. Example: `examples/shell.yaml`. Not run on a live worker.

### Changed

- Public documentation split into a short README and site pages for architecture, configuration,
  operations, releases, security and release history.
- The executable specification is pinned to ESS 0.76.0 (`requires: ess 0.76.0`, format `ess/22`),
  and CI installs that release after checking its SHA-256. The conformance runner uses ESS's 0.76.0
  Rust crates. The session name's live uniqueness is now a declared `InsertSession` refusal rather
  than an `external:` outcome, so synthesis arranges the colliding row itself. The complete native
  inventory is unchanged at 508 scenarios, all passing.
- Substrate 0.16.1 (was 0.7.10): `b10x-substrate-sdk` is pinned to revision
  `f2fd606b88a0b778970e8babe9a96eb600cd57b2`, the worker helpers expect daemon 0.16.1 and release
  manifests record that version and revision. Tag 0.16.0 was never published; 0.16.1 is the first
  release of its scope. `mantle worker up` installs the daemon from the release's signed
  `substrate-daemon-0.16.1-x86_64-unknown-linux-gnu.tar.gz` instead of a layer of the daemon image:
  cloud-init accepts only the pinned digest of `SHA256SUMS`, the document the release signs, and an
  archive whose pinned digest that document lists. `bootstrap.json` records `substrate_release`,
  `substrate_sums` and `substrate_archive` in place of `substrate_image` and `substrate_layer`.
  Signature verification is the release's; the worker does not run `cosign`. `mantle worker
  upgrade` and `mantle doctor` refuse a worker still on 0.7.10, as they refuse any other version.
  Existing workers keep 0.7.10 until they are replaced. Not run on a live worker.
- Wire behaviour Mantle sees on Substrate 0.16.1:
  - The daemon and the SDK advertise contract bundle `substrate-wire/0.25.0` (was `0.17.0`);
    `doctor`'s wire-compatibility check follows the SDK's.
  - A workspace observation carries `origin` and `root_name`; Mantle ignores both.
  - A workspace storage limit that is not a multiple of 1024 bytes is refused
    `request.schema-invalid` (422), where it was `storage.quota-limit` (429). A manifest size given
    in plain bytes can meet it; one in KiB, MiB, GiB or TiB cannot.
  - One subject holds at most 16 session attachments; the 17th is refused
    `session.attachment-capacity` (429).
  - A `session.not-attachable` refusal names the failed condition in its message; its code is
    unchanged.
  - A PTY session whose child exited before the daemon recorded the start reads `exited` with
    attachment `consumed`, not `ready` and `available`.
  - A just-started session or exec is no longer expired by the lease sweeper, and two concurrent
    renewals of one lease no longer refuse the slower one `lease.expired`.
  - Resource usage may omit `io_read_bytes` and `io_write_bytes`; Mantle does not show them.
  - Inside the sandbox, `socket(2)` admits only IPv4 and IPv6 TCP or UDP and route netlink, and
    `socket(AF_UNIX)` stays refused; `socketpair(2)` admits only Unix stream and seqpacket pairs.
    Anything else fails with `EACCES`.
  - Substrate states that its state store upgrade is one-way: 0.16.1 stamps the store with a schema
    version that earlier daemons refuse. Back up `/var/lib/substrate/state` before a worker's daemon
    moves to 0.16.1 if it may need to go back.

## [0.2.0] - 2026-10-03

### Changed

- `mantle stop NAME` keeps the workspace and the private agent home. Delete them with
  `mantle destroy NAME --yes`. Historical 0.1.4 `STOPPING` records keep their destroy intent.
- `mantle exec` returns the remote exit code and output; missing or indeterminate results fail.

### Added

- `mantle restart`, which starts a fresh agent in a retained workspace without resuming a
  conversation.
- Named profiles, `mantle doctor`, JSON metadata for `list` and `status`, and creation receipts.
- Verified Linux x86-64 release bundles with the `mantle-release` installer and the
  `mantle-acceptance` runner; offline worker helper upgrades.

## [0.1.4] - 2026-10-03

### Changed

- Release and acceptance records and documentation only; runtime unchanged from 0.1.3.

## [0.1.3] - 2026-10-03

### Fixed

- Codex attachment startup and sustained output: replay waits for client readiness; adopts
  Substrate 0.7.10 with its bounded output-queue wait.

## [0.1.2] - 2026-10-02

### Fixed

- Rust and native dependency compilation inside the sandbox: `gcc` and `g++` are selected as
  linker and compilers.

## [0.1.1] - 2026-10-02

### Fixed

- The static musl worker build.

## [0.1.0] - 2026-10-02

### Added

- Codex 0.153.4 sessions with ChatGPT device login beside Claude Code.

[Unreleased]: https://github.com/beyond10x/mantle/compare/0.2.0...HEAD
[0.2.0]: https://github.com/beyond10x/mantle/releases/tag/0.2.0
[0.1.4]: https://github.com/beyond10x/mantle/releases/tag/0.1.4
[0.1.3]: https://github.com/beyond10x/mantle/releases/tag/0.1.3
[0.1.2]: https://github.com/beyond10x/mantle/releases/tag/0.1.2
[0.1.1]: https://github.com/beyond10x/mantle/releases/tag/0.1.1
[0.1.0]: https://github.com/beyond10x/mantle/releases/tag/0.1.0
