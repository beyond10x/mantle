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

Worker provisioning installs and verifies the pinned Codex binary. Existing workers need current
Mantle worker binaries and the gateway allowlist, including `auth.openai.com:443` and
`chatgpt.com:443`; arrange upgrades when no sessions are active. Codex does not need a Claude
OAuth token or an API key. A manifest without an agent kind continues to select Claude Code.

Codex uses the same Substrate terminal transport and capture behavior as Claude. This is **not
an end-to-end non-recording mode**. Its private auth and conversation files live under
`/workspace/.mantle/home/.codex`; launcher replay is volatile and runtime SQLite/text diagnostics
use launcher-verified tmpfs. `mantle stop codex-example` destroys the workspace, including those
auth files: preserve wanted project work remotely first. Detach retains it for the running session.

Local launcher, confinement and integration checks are recorded in the
[qualification evidence](docs/evidence/codex-compatibility.md). Real device login, authenticated
model/tool turns and token refresh have not yet been verified end to end.

## Release 0.1.2

This source release fixes Rust and native dependency builds inside confinement by selecting
`gcc` as the Rust linker and C compiler, and `g++` as the C++ compiler. Both agents and
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

Read the printed remote `ExecExit`: the current CLI can return success even when the remote
command fails. Release archives contain source; build the laptop CLI and static worker binaries
from the same tag. Upgrade worker binaries only when no sessions are active.

## Requirements on the laptop

- Linux, Rust 1.97 (`rust-toolchain.toml`), [go-task](https://taskfile.dev)
- AWS CLI v2 with an SSO profile that may create EC2, IAM and CloudWatch resources, and
  `session-manager-plugin`
- `ssh`
- for Claude Code, an OAuth token from `claude setup-token`; for Codex, ChatGPT device login

Copy [examples/config.toml](examples/config.toml) to `~/.config/mantle/config.toml` and fill it in.

Agent and contributor instructions are in [AGENTS.md](AGENTS.md).

## Executable specification

[spec/README.md](spec/README.md) describes the implemented contract, its source mappings and
coverage gaps. `task spec` validates it; `task conformance` synthesizes and executes the local
session-store and default-allowlist scenarios against production Rust code. `task check` includes
that test through the workspace suite. Contributors need ESS 0.50.x and AEP on `PATH`.
