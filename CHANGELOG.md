# Changelog

All notable changes to Mantle. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Evidence and compatibility notes for each release are on the
[release history](https://beyond10x.github.io/mantle/history/) page; notes and assets are on
[GitHub releases](https://github.com/beyond10x/mantle/releases).

## [Unreleased]

### Changed

- Public documentation split into a short README and site pages for architecture, configuration,
  operations, releases, security and release history.

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
