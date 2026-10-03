# Mantle

[Read the documentation](https://beyond10x.github.io/mantle/) · [Report an issue](https://github.com/beyond10x/mantle/issues)

Mantle runs a whole development session (Claude Code or Codex, its workspace and toolchain) on a remote
Linux worker, confined by [Substrate](https://github.com/beyond10x/substrate). The laptop renders
the terminal; builds use the worker's CPU.

```console
mantle worker up                 # start the configured AWS or KubeVirt worker
mantle start examples/substrate.yaml
mantle attach substrate-work     # Ctrl-] d detaches; the agent keeps running
mantle status substrate-work
mantle stop substrate-work       # development: retain workspace; 0.1.4: destructive
mantle worker down
```

Development source is an early preview. The commands below distinguish unreleased behavior from
the historical 0.1.4 release. See [installation](#requirements-on-the-laptop),
[named profiles](#named-profiles-development--unreleased),
[worker diagnostics](#worker-diagnostics-development--unreleased),
[repeatable acceptance](#repeatable-agent-acceptance-development--unreleased) and
[offline upgrades](#offline-worker-upgrades-development--unreleased).

The design is
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
`auth.openai.com:443` and `chatgpt.com:443`. Follow the
[offline upgrade guidance](#offline-worker-upgrades-development--unreleased) for development workers before
upgrading a running worker. Codex does not need a Claude
OAuth token or an API key. A manifest without an agent kind continues to select Claude Code.

Codex uses the same Substrate terminal transport and capture behavior as Claude. This is **not
an end-to-end non-recording mode**. Its private auth and conversation files live under
`/workspace/.mantle/home/.codex`; launcher replay is volatile and runtime SQLite/text diagnostics
use launcher-verified tmpfs. In development source, `mantle stop codex-example` retains the workspace
and login files; `mantle destroy codex-example --yes` deletes them. **Historical release 0.1.4 has
destructive stop semantics:** its `stop` deletes the workspace and auth files. Preserve wanted
project work remotely before using that release's stop command.

## Retained workspaces (development / unreleased)

**Compatibility change:** development `mantle stop NAME` terminates and retires the agent while
retaining its workspace, edited files and private login home. Release 0.1.4 still destroys them.
Use a development CLI for the following commands; this integration does not cut a release.

```console
mantle stop my-project
mantle restart my-project
mantle destroy my-project --yes
```

Restart creates a fresh agent execution in the same workspace, using the persisted agent, cwd,
environment and resource policy. It does not clone repositories again or overwrite private homes.
It does not promise same-conversation resume. Attach never starts an agent. A missing workspace
is refused and is never recreated. Retained names remain reserved until explicit destruction.
Ordinary `mantle exec` continues to use the existing workspace without injecting agent credentials.

Attach, exec, stop, restart and destroy accept `--expected-session-id ID` for automation. The CLI checks that
immutable identity before remote mutation; use the recorded ID, not merely a reusable name.
Concurrent commands claim a durable generation and intent. Interrupted restart observes its
original admission operation instead of launching another process. Retry the same command to
reconcile it. Missing or unknown operation/process observations remain incomplete with a nonzero
result. A confirmed running or terminal admission can be reconciled by stop/destroy even when
readiness failed. An interrupted initial materialization stays incomplete when termination cannot
be proven; it is not reported as a safely retained workspace.

Historical `STOPPED` rows mean destroyed and cannot be restarted. Migration preserves historical
`STOPPING` as destructive intent, so retrying that cleanup may still delete the workspace. Legacy
rows without validated launch context cannot restart from today's configuration. Changed worker
bindings or unsupported persisted policy versions are refused. Retention does not recover a lost
worker or volume; retained cloud storage may still incur charges.

Local launcher, confinement and integration checks are recorded in the
[qualification evidence](docs/evidence/codex-compatibility.md). Real device login, authenticated
model/tool turns and token refresh have not yet been verified end to end.

## Command results (development / unreleased)

The development CLI preserves remote exit codes (0–255), maps supported INT/TERM/KILL signals to
130/143/137, and returns failure for refused, missing, contradictory or indeterminate results.
Command stdout and stderr retain their bytes after a valid Substrate response has decoded; the
existing status diagnostic follows remote stderr. Malformed responses fail without recovering raw
output. This corrects the false-success behavior in release 0.1.4.

Repository branches, lightweight and annotated tags, and full commit IDs are supported;
materialization records the actual checked-out commit.

## Historical release 0.1.4

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

For historical **release 0.1.4**, do not use `mantle stop NAME` as an upgrade step for a workspace
you need to retain: it destroys that workspace. Development retention semantics are described above.

### Build behavior retained from 0.1.2

Mantle selects `gcc` as the Rust linker and C compiler, and `g++` as the C++ compiler. Both agents and
supplementary `mantle exec` commands receive these defaults on new execution requests. The
worker's filesystem and network boundaries are unchanged.

A real KubeVirt worker completed a confined `cargo check --locked -p b10x-substrate-host`,
including native dependencies. Two live Codex workspaces passed marker-isolation checks;
OpenAI TLS access through the gateway worked, while unlisted destinations and direct egress
were denied. These supplementary-command checks do not establish authenticated Codex tool use.
See the [historical 2026-10-02 acceptance audit](.engineering/reports/codex-acceptance-audit-2026-10-02/audit.md).

Standalone `mantle exec` commands need an explicit proxy setting for networked Cargo builds:

```console
mantle exec SESSION -- cargo --config 'http.proxy="http://127.0.0.1:3128"' check --locked --manifest-path /workspace/PROJECT/Cargo.toml
```

The 0.1.4 release archives contain source, without prebuilt Mantle binaries. Development candidate
bundles and their verifier are described below; their availability does not announce another release.

## Requirements on the laptop

- Linux x86-64 and `ssh`; GNU candidate binaries require the libc environment recorded in their manifest
- for AWS: AWS CLI v2, an authenticated profile allowed to create the required EC2, IAM and
  CloudWatch resources, and `session-manager-plugin`
- for KubeVirt: `kubectl` and access to the configured cluster; the local `task k3d-up` test bed
  additionally needs Docker, k3d and nested virtualization
- for Claude Code, an OAuth token from `claude setup-token`; for Codex, ChatGPT device login

Source builds additionally need Rust 1.97 (`rust-toolchain.toml`), [go-task](https://taskfile.dev),
the `x86_64-unknown-linux-musl` Rust target and a musl C toolchain for worker binaries. Verified
[candidate bundles](#verified-prebuilt-bundles-development--unreleased) do not require a local
Rust compiler; fresh worker provisioning accepts their static helpers via `worker up --binaries`.

Copy [examples/config.toml](examples/config.toml) to `~/.config/mantle/config.toml` and fill it in.

## Named profiles (development / unreleased)

Named profiles are available in **development source and candidate bundles, not release 0.1.4**.
Register separate absolute configuration and private state paths:

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

## Worker diagnostics (development / unreleased)

Worker diagnostics are **development/unreleased, not part of 0.1.4**:

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

## Metadata for automation (development / unreleased)

Use the versioned metadata interface for scripts. Human status can print launcher diagnostics and
is not a transcript-free input format.

```console
mantle --profile personal list --json
mantle --profile personal status my-project --json --timeout-secs 10
mantle --profile personal status --session-id SESSION_ID --json
```

JSON list includes terminal records and reports local state without contacting a worker. JSON
status separates recorded identity/state from bounded Substrate observations, including unavailable
or refused observations. An exact session-id lookup also works for destroyed records. Neither JSON
command fetches terminal output pages. Authentication method is metadata, not proof of login.

For an ownership receipt, use `mantle start MANIFEST --detached --receipt PATH`. Choose a new path
in a private directory. The receipt is tied to that successful creation, including its immutable
session id and selected profile binding. Persist it before dependent operations; a name lookup
cannot replace a lost receipt. An interrupted start may have created resources even when no receipt
arrived. Treat that result as unresolved rather than automatically claiming or destroying a name.

## Repeatable agent acceptance (development / unreleased)

`mantle-acceptance` exercises the installed Mantle CLI with disposable Claude Code or Codex
sessions. It belongs to the GNU candidate bundle alongside `mantle` and `mantle-release`; it is
not part of historical release 0.1.4. Choose an already configured qualification worker and named
profile. The runner does not provision workers or adopt existing user sessions.

Start a fresh run with a new private checkpoint directory:

```console
mkdir -m 0700 "$HOME/.cache/mantle-acceptance-run"
mantle-acceptance run --real --mantle "$HOME/.local/mantle/bin/mantle" \
  --profile personal --agent codex --manifest examples/codex.yaml \
  --checkpoint "$HOME/.cache/mantle-acceptance-run/run.json"
```

For Claude Code, select `--agent claude-code` and a Claude manifest such as
`examples/substrate.yaml`, using a profile with its existing Claude credential source configured.
Add `--release-manifest /absolute/path/to/bundle/manifest.json` when the complete verified bundle
is available. The runner creates its own uniquely named disposable session.

Login, a model/tool turn and visual terminal checks happen in the operator's own terminal through
the exact attach command printed by the runner. They are explicit operator attestations, separate
from machine observations. The runner does not read authentication caches, store terminal transcripts
or infer authentication from a running process. Missing manual evidence leaves the run incomplete.

Use the printed attach command unchanged: it selects the run's private copy of the CLI and pins
the resolved configuration/state paths, rather than resolving a possibly changed profile again.
After completing the requested actions, submit only the attestations you actually observed, with
the current run and exec IDs from that handoff:

```console
mantle-acceptance resume --real \
  --checkpoint "$HOME/.cache/mantle-acceptance-run/run.json" \
  --expected-run-id RUN_ID --expected-exec-id EXEC_ID \
  --attest login --attest model-tool --attest visual
mantle-acceptance report --checkpoint "$HOME/.cache/mantle-acceptance-run/run.json"
```

Follow any subsequent handoff with its current IDs. Exit 2 means incomplete/operator action
required, exit 1 means failure, and exit 0 means all required cases are complete. `report` reads
the checkpoint without contacting a worker. `cleanup` also returns the aggregate qualification
status: successful cleanup after an aborted run can still return 2, and a prior failed case remains
failed. To explicitly remove only the owned disposable
workspace and its private login state:

```console
mantle-acceptance cleanup --real --checkpoint "$HOME/.cache/mantle-acceptance-run/run.json"
```

Checkpoints bind the run to its selected CLI, resolved profile and exact owned session identities.
Cleanup requires a proven creation identity; a matching name or name prefix is insufficient. A lost
or interrupted creation receipt remains unresolved and cannot authorize automatic cleanup. Resume
must revalidate those bindings before continuing. Retained-workspace checks establish that workspace
identity and a synthetic marker survive a fresh agent execution, not that a conversation resumes.

For known source provenance, supply the verified release manifest with both sibling archives and
match its CLI payload digest to the chosen executable. A version string or installed generation
directory name is not source proof. Without `--release-manifest`, source identity stays explicitly
unknown. A supplied manifest with missing/invalid archives or a mismatching executable is refused.
The runner's controlled CLI/PTY tests verify its behavior; they do not establish a fully authenticated
live Claude or Codex qualification. Historical qualification limits remain in the
[recorded evidence](docs/evidence/codex-compatibility.md).

## Executable specification

[spec/README.md](spec/README.md) describes the implemented contract, its source mappings and
coverage gaps. `task spec` validates all six domains. `task conformance` exercises the production
SQLite store, manifest parser, egress proxy, launcher PTYs, application orchestration and operator
workflows over controlled external IO. `task check` includes the complete native suite and repository
checks. Contributors need ESS 0.50.x and AEP 0.68.0 on `PATH`. Green local tests do not establish
authenticated Claude or Codex qualification; see the separate historical evidence above.

## Verified prebuilt bundles (development / unreleased)

The new `mantle-release` tooling is **not included in release 0.1.4**. This section describes
candidate artifacts produced from development source; this integration does not publish a new
release. CI builds and retains candidates without publication credentials.

Each candidate contains `manifest.json`, `SHA256SUMS`, a GNU Linux x86-64 archive with `mantle`,
`mantle-release` and `mantle-acceptance`, and a static musl x86-64 archive with `mantle-egress`, `mantle-launch` and
`mantle-worker`. Archives contain the Apache license, complete third-party notices and the matching
Rust runtime notice; workers also contain the musl 1.2.5 notice. The manifest records the exact Git
commit, Substrate revision/version, Rust toolchain, targets, GNU libc build environment and every
payload's digest, size and mode. GNU compatibility is limited to the recorded build/test runtime;
we do not claim older libc, macOS or ARM support. Deterministic archive metadata does not promise
bit-identical compilation across different toolchains or hosts.

Obtain a candidate from the trusted CI run for the intended source commit and check that commit
in its manifest. For a future published bundle, use the trusted release for the intended tag.
Keep `manifest.json`, both named archives and `SHA256SUMS` in one directory. Hashes
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

## Offline worker upgrades (development / unreleased)

Release 0.1.4 does not include `mantle worker upgrade`. Use a matching development CLI and a
verified development bundle. This upgrades the three Mantle helpers while preserving worker data,
configuration, Claude, Codex's generation link and the existing Substrate daemon. A different
Substrate version or revision is refused. The command does not drain sessions or stop, mask,
unmask, restart or signal services. `--check` also works with an older installed worker helper:
it uses bounded, read-only SSH observations and never uploads a checker or initializes state.

The administrator must establish an exclusive maintenance window. Save wanted work and deliberately
retire all executions first. Exclude other administrative changes and already-running older Mantle
clients throughout maintenance; those clients do not participate in the new installation lock.
Use `mantle worker ssh` and prepare the supported layout on the worker:

```console
sudo systemctl stop substrate.service mantle-egress.service
sudo install -d -m 0700 /var/lib/mantle/maintenance/units
sudo mv --no-clobber /etc/systemd/system/substrate.service /var/lib/mantle/maintenance/units/substrate.service
sudo mv --no-clobber /etc/systemd/system/mantle-egress.service /var/lib/mantle/maintenance/units/mantle-egress.service
sudo systemctl mask substrate.service mantle-egress.service
sudo systemctl daemon-reload
```

These are first-entry maintenance commands: inspect any existing saved originals before proceeding
and never overwrite them. The originals must retain the supported rendered unit bytes, root:root
ownership, mode 0644 and a single hard link. Ordinary masking cannot replace the regular unit files
that Mantle originally installs under `/etc/systemd/system`; a runtime mask alone does not override
those files. The checker refuses missing originals, unsupported overrides, ineffective masks,
queued jobs, nonzero service PIDs, populated delegated cgroups and unknown observations. An empty
`ControlGroup` property alone is insufficient: the fixed cgroup subtree is inspected as well.

On the laptop, verify the complete trusted bundle as described above, then run:

```console
mantle worker upgrade --check --manifest /absolute/path/to/bundle/manifest.json
mantle worker upgrade --apply --manifest /absolute/path/to/bundle/manifest.json
```

The check report separates observed installed versions, selected source/digests, Substrate compatibility
and maintenance prerequisites. Equal version strings alone never establish that a bundle is current.
Apply repeats the observations under `/opt/mantle/install.lock`, stages a complete sibling of the
real `/opt/mantle/bin` directory and uses one atomic directory exchange. Unsupported exchange
semantics refuse; there is no sequence of individual helper replacements. Codex installation and
current-client helper reconciliation use the same host lock before any agent-specific lock.
`worker up` refuses managed helper reconciliation; use the explicit upgrade flow for those workers.

Success is `applied-restart-required`. The services remain masked and inactive. The prior directory
and a synced journal remain under `/opt/mantle`; a retry of the same apply command inspects actual
directory identities and either completes the transaction or verifies restoration. If the report
says `recovery-required`, keep `/opt/mantle/upgrade-pending.json` and the `.upgrade-*` directories
for inspection; do not remove or rename them to force a retry. Rollback is reported only after the
exact prior directory is observed restored. A disconnected SSH transport can leave its private
`/var/tmp/mantle-delivery.*` stage for administrator inspection; it is not an installed generation.
An interrupted journal takes precedence over a new candidate: `recovered-other-bundle` names the
earlier recovered source and returns nonzero. Review that result and repeat apply for the intended
candidate. A written bundle marker alone does not establish `current` while recovery is pending.
Codex installation refuses an unfinished helper transaction until recovery completes. Completed
journals are checked without replaying rollback: a later supported Codex installation is preserved,
and unexpected inventory changes refuse without restoring an older directory over current files.

After a successful upgrade, deliberately leave maintenance on the worker:

```console
sudo systemctl unmask substrate.service mantle-egress.service
sudo systemctl unmask --runtime substrate.service mantle-egress.service
sudo mv --no-clobber /var/lib/mantle/maintenance/units/substrate.service /etc/systemd/system/substrate.service
sudo mv --no-clobber /var/lib/mantle/maintenance/units/mantle-egress.service /etc/systemd/system/mantle-egress.service
sudo systemctl daemon-reload
sudo systemctl start mantle-egress.service substrate.service
```

Check the restored unit files before starting, then run `mantle doctor` from the laptop. Installed
version checks do not establish a running daemon's identity or authenticated agent readiness.

For **fresh provisioning**, a local Rust build is unnecessary. Verify the complete trusted bundle,
then extract only the three static worker payloads into a new local directory:

```console
mkdir -m 0700 worker-stage
tar --extract --file mantle-VERSION-x86_64-unknown-linux-musl.tar --directory worker-stage --no-same-owner bin/mantle-egress bin/mantle-launch bin/mantle-worker
mantle worker up --binaries "$PWD/worker-stage/bin"
```

This is the existing provisioning flow, including its normal provider setup and selected agent
configuration. Existing workers that need binary replacement use offline upgrade instead. The
upgrade transport bounds each transferred file to 64 MiB and preserves its verified digest.
