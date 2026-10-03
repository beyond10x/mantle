# Mantle

[Read the documentation](https://beyond10x.github.io/mantle/) · [Report an issue](https://github.com/beyond10x/mantle/issues)

Mantle runs a whole development session (Claude Code or Codex, its workspace and toolchain) on a remote
Linux worker, confined by [Substrate](https://github.com/beyond10x/substrate). The laptop renders
the terminal; builds use the worker's CPU.

```console
mantle worker up                 # one EC2 worker, reachable only through AWS SSM
mantle start examples/substrate.yaml
mantle attach substrate-work     # Ctrl-] d detaches; the agent keeps running
mantle status substrate-work
mantle stop substrate-work
mantle worker down
```

Status: first vertical slice, under construction. The design is
[docs/design/00-mantle-on-substrate.md](docs/design/00-mantle-on-substrate.md); its correction block
lists what the slice does differently from the design and why.

## Start Codex

[The Codex manifest](examples/codex.yaml) selects pinned Codex 0.153.4 with `agent.kind: codex`
and `agent.auth: chatgpt-device`. After configuring your worker, run:

```console
mantle worker up
mantle start examples/codex.yaml
# Complete ChatGPT device login in the agent terminal when prompted.
# Ctrl-] d detaches without stopping the agent.
mantle attach codex-example
mantle status codex-example
```

Worker provisioning installs and verifies the pinned Codex binary. Mantle 0.1.4 requires its
matching worker binaries and Substrate 0.7.10, plus the gateway allowlist including
`auth.openai.com:443` and `chatgpt.com:443`. Follow the existing-worker guidance below before
upgrading a running worker. Codex does not need a Claude
OAuth token or an API key. A manifest without an agent kind continues to select Claude Code.

Codex uses the same Substrate terminal transport and capture behavior as Claude. This is **not
an end-to-end non-recording mode**. Its private auth and conversation files live under
`/workspace/.mantle/home/.codex`; launcher replay is volatile and runtime SQLite/text diagnostics
use launcher-verified tmpfs. `mantle stop codex-example` destroys the workspace, including those
auth files: preserve wanted project work remotely first. Detach retains it for the running session.

Local launcher, confinement and integration checks are recorded in the
[qualification evidence](docs/evidence/codex-compatibility.md). Real device login, authenticated
model/tool turns and token refresh have not yet been verified end to end.

## Version 0.1.4

This patch release integrates the final release and acceptance records and updates the public
documentation. Runtime behavior is unchanged from 0.1.3. The operator confirmed that attachment
worked with the corrected CLI and worker profile; full authenticated lifecycle qualification
remains incomplete.

Build the laptop CLI and static worker binaries from the same
[0.1.4 source tag](https://github.com/beyond10x/mantle/releases/tag/0.1.4). This version pairs
Mantle's attachment-readiness handshake with Substrate 0.7.10's bounded output wait. Replay
starts after the client is ready; a temporarily full output queue waits for its consumer for
up to one second per frame. Queue and output limits remain in force, and a stalled consumer
can still end the attachment with `session.output-backpressure`.

An isolated, confined synthetic test delivered the full 256 KiB replay twice through real PTYs,
preserved keyboard input and terminal settings, and detached/reconnected to the same agent.
An isolated KubeVirt worker with the signed Substrate 0.7.10 runtime also passed two real
Codex 0.153.4 attachments through 100×50 PTYs: each ran for 12 seconds and detached with exit 0;
the second reused the same agent. Existing workers and sessions were preserved. These checks
establish transport behavior; they do not establish visual screen correctness, device login or
authenticated model/tool use. See the [qualification update](docs/evidence/codex-compatibility.md).

### Existing workers

Upgrade the laptop CLI, worker launcher and Substrate runtime together. Check `mantle list` and
`mantle status NAME`, preserve wanted project changes and workspace data, and arrange maintenance
before replacing a running daemon. Restarting Substrate can interrupt its executions.
`mantle worker up` starts services but does not restart an already running older daemon; it also
defers changed shared binaries while services are active. Rerunning it alone does not establish
that an existing worker has the compatible runtime. Verify the installed and running versions
after maintenance, or provision a separate compatible worker for new sessions.

Do not use `mantle stop NAME` as an upgrade step for a workspace you need to retain: it destroys
that workspace. Detach preserves the session only for the lifetime of its running agent and lease.

### Build behavior retained from 0.1.2

Mantle selects `gcc` as the Rust linker and C compiler, and `g++` as the C++ compiler. Both agents and
supplementary `mantle exec` commands receive these defaults on new execution requests. The
worker's filesystem and network boundaries are unchanged.

A real KubeVirt worker completed a confined `cargo check --locked -p b10x-substrate-host`,
including native dependencies. Two live Codex workspaces passed marker-isolation checks;
OpenAI TLS access through the gateway worked, while unlisted destinations and direct egress
were denied. These supplementary-command checks do not establish authenticated Codex tool use.
See the [current acceptance audit](.engineering/reports/codex-acceptance-audit-2026-10-02/audit.md).

Standalone `mantle exec` commands need an explicit proxy setting for networked Cargo builds:

```console
mantle exec SESSION -- cargo --config 'http.proxy="http://127.0.0.1:3128"' check --locked --manifest-path /workspace/PROJECT/Cargo.toml
```

The CLI preserves remote exit codes (0–255), maps the supported INT/TERM/KILL signals to
130/143/137, and
returns failure for refused, missing, contradictory or indeterminate results. Command stdout
and stderr retain their bytes after a valid Substrate response has decoded; the existing
status diagnostic follows remote stderr. Malformed responses fail without recovering raw output.
This corrects the false-success behavior in release 0.1.4.
Repository branches, lightweight and annotated tags, and full commit IDs are supported;
materialization records the actual checked-out commit. Release archives contain source;
no prebuilt Mantle binaries are provided.

## Requirements on the laptop

- Linux, Rust 1.97 (`rust-toolchain.toml`), [go-task](https://taskfile.dev)
- AWS CLI v2 with an SSO profile that may create EC2, IAM and CloudWatch resources, and
  `session-manager-plugin`
- `ssh`
- for Claude Code, an OAuth token from `claude setup-token`; for Codex, ChatGPT device login

Copy [examples/config.toml](examples/config.toml) to `~/.config/mantle/config.toml` and fill it in.

Named profiles are available in **development source, not release 0.1.4**. After building that
source, register separate absolute configuration and private state paths:

```console
mantle profile add personal --config "$HOME/.config/mantle/personal.toml" --state-dir "$HOME/.local/state/mantle-personal"
mantle profile list
mantle profile show personal
mantle --profile personal list
MANTLE_PROFILE=personal mantle worker status
```

Registration stores path references only; it does not copy credentials, read provider configuration,
create the selected state directory, or contact a worker. Names contain lowercase letters, digits,
underscores and hyphens, start with a letter or digit, and are at most 63 characters. Existing names
cannot be overwritten. The private registry is `~/.config/mantle/profiles/`.

`--profile` overrides `MANTLE_PROFILE`. A named selection refuses either `MANTLE_CONFIG` or
`MANTLE_STATE_DIR`; unset those legacy overrides first. Without a named selection, the legacy
variables and default paths still work. Each selected state directory holds its own SQLite database,
SSH key and known hosts. Point profiles at different state directories to keep them isolated.
Attachments retain separate, short private temporary socket directories, including with long or
Unicode state paths. Profile registry entries and selected paths refuse symlinks and unsafe file
types; configuration and state directory ancestors must be owned by the operator or root and not
writable by others (root-owned sticky temporary directories are allowed).
Spaces, `#`, colons and Unicode are supported in state paths. Quotes, backslashes, percent tokens
and `${...}` expressions are refused because OpenSSH interprets them rather than using literal paths.

Agent and contributor instructions are in [AGENTS.md](AGENTS.md).

Worker diagnostics are also **development/unreleased, not part of 0.1.4**:

```console
mantle --profile personal doctor
mantle --profile personal doctor --json --timeout-secs 5
```

Doctor checks the selected configuration, existing worker record and placement, provider, strict
SSH access, Substrate service/socket, installed Mantle binaries, live Substrate version/wire
compatibility, and required common confinement facts. Each check reports `healthy`, `failed` or
`skipped`; an early failure suppresses dependent probes and exits nonzero. JSON remains structured
even when selection or configuration fails. Human and JSON reports use fixed advice and omit raw
provider, SSH and configuration contents. A healthy worker does not establish agent authentication.

Doctor needs an existing SSH key and pinned known host. It never creates keys, accepts new hosts,
invokes an agent credential command, initializes or migrates a database, modifies application
records, or restarts services. SQLite opens read-only/query-only and observes committed WAL data;
ordinary SQLite locking/shared-memory coordination is permitted. Normal provider authentication
may run its existing plugins. External probes default to a 10-second deadline each (configurable
from 1 to 120 seconds); timed-out process groups are retired, including tunnel descendants.
Configuration input is capped at 1 MiB, database/sidecar files at 64 MiB, and SQLite busy/query
handling at 250 milliseconds. Refused or unavailable observations never become a healthy result.

## Executable specification

[spec/README.md](spec/README.md) describes the implemented contract, its source mappings and
coverage gaps. `task spec` validates it; `task conformance` synthesizes and executes the local
session-store and default-allowlist scenarios against production Rust code. `task check` includes
that test through the workspace suite. Contributors need ESS 0.50.x and AEP on `PATH`.

## Verified prebuilt bundles (development / unreleased)

The new `mantle-release` tooling is **not included in release 0.1.4**. This section describes
candidate artifacts produced from development source; this integration does not publish a new
release. CI builds and retains candidates without publication credentials.

Each candidate contains `manifest.json`, `SHA256SUMS`, a GNU Linux x86-64 archive with `mantle`
and `mantle-release`, and a static musl x86-64 archive with `mantle-egress`, `mantle-launch` and
`mantle-worker`. Archives contain the Apache license, complete third-party notices and the matching
Rust runtime notice; workers also contain the musl 1.2.5 notice. The manifest records the exact Git
commit, Substrate revision/version, Rust toolchain, targets, GNU libc build environment and every
payload's digest, size and mode. GNU compatibility is limited to the recorded build/test runtime;
we do not claim older libc, macOS or ARM support. Deterministic archive metadata does not promise
bit-identical compilation across different toolchains or hosts.

For a published bundle, obtain the manifest from the trusted release for the intended tag and
check its source commit. Download both named archives and `SHA256SUMS` into one directory. Hashes
provide integrity against that trusted manifest; an adjacent checksum file is not independent
publisher authentication. With `jq`, `sha256sum` and `tar` installed, bootstrap the bundled Rust
verifier without compiling it:

```sh
jq -r '.artifacts[] | "\(.sha256)  \(.name)"' manifest.json | sha256sum --check
mkdir bootstrap
# Replace VERSION with the manifest version. Extract only the checksum-verified installer.
tar --extract --file mantle-VERSION-x86_64-unknown-linux-gnu.tar \
  --directory bootstrap --no-same-owner bin/mantle-release
./bootstrap/bin/mantle-release verify --manifest "$PWD/manifest.json"
./bootstrap/bin/mantle-release install --manifest "$PWD/manifest.json" \
  --prefix "$HOME/.local/mantle"
export PATH="$HOME/.local/mantle/bin:$PATH"
```

The GNU installer refuses unmanaged destination collisions, links in the installation prefix,
unsupported targets, and modified existing generations, including extra files or directories. It stages verified files under the prefix,
serializes installers with a five-second bounded lock, and activates the whole generation with
one symlink rename. It preserves previous generations and leaves configuration and credentials
outside installation. Interruptions before activation retain the prior installation. The worker
bundle is **not** activated by this command: existing workers require the explicit offline worker
upgrade flow, and the real worker `bin` directory and agent installations must be preserved.

Maintainers build only from a clean exact checkout, using a new work directory and output path:

```sh
cargo run --locked -p mantle-release -- notices --check
cargo run --locked -p mantle-release -- build --source "$PWD" \
  --revision "$(git rev-parse HEAD)" --work "$HOME/.cache/mantle-release-build-UNIQUE" \
  --output "$HOME/.cache/mantle-release-bundle-UNIQUE"
```

Install `cargo-about 0.9.1`, the Rust musl target and a musl C toolchain first. Regenerate the committed
notice with `mantle-release notices` when the locked dependency graph changes. Build uses an isolated
Git export checked against the raw committed tree and blob bytes, and a fresh target. Export
attributes that rewrite or omit committed files are refused; Git replacement objects are ignored.
Build verifies both supported artifacts with actual version execution and
retains source and compiler logs in the supplied work directory. Pre-existing same-version binaries
cannot be supplied or relabeled as the chosen commit.

Publication is an explicit operator command after required repository checks and bot tag creation:

```sh
mantle-release publish --manifest /path/to/bundle/manifest.json --source "$PWD" \
  --tag VERSION --policy /path/to/private/gates-policy.json
```

It uses the installed `b10x-gates gh` bot wrapper, checks local and fully dereferenced remote tags
against the source, refuses an existing release, uploads only the verified assets, and verifies
GitHub's resulting asset digests. Publication failures may leave a partial release for operator
inspection; the command never clobbers it. No personal `gh` writes or publishing keys in CI.
