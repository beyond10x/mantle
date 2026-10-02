# Mantle

[Read the documentation](https://beyond10x.github.io/mantle/) · [Report an issue](https://github.com/beyond10x/mantle/issues)

Mantle runs a whole development session (Claude Code, its workspace and toolchain) on a remote
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

[The Codex manifest](examples/codex.yaml) now selects pinned Codex 0.153.4 with ChatGPT device
authentication and retains that identity in session records. Start and attach currently refuse
with a non-recording capture capability error before credentials or remote work are touched:
the pinned Substrate SDK cannot yet provide that mode. Private home, volatile launcher replay
and tmpfs diagnostic configuration are implemented; this is preparation for supported Codex
sessions, not an authenticated compatibility claim. See the
[qualification evidence](docs/evidence/codex-compatibility.md).

## Requirements on the laptop

- Linux, Rust 1.97 (`rust-toolchain.toml`), [go-task](https://taskfile.dev)
- AWS CLI v2 with an SSO profile that may create EC2, IAM and CloudWatch resources, and
  `session-manager-plugin`
- `ssh`
- a Claude Code OAuth token from `claude setup-token`

Copy [examples/config.toml](examples/config.toml) to `~/.config/mantle/config.toml` and fill it in.

Agent and contributor instructions are in [AGENTS.md](AGENTS.md).

## Executable specification

[spec/README.md](spec/README.md) describes the implemented contract, its source mappings and
coverage gaps. `task spec` validates it; `task conformance` synthesizes and executes the local
session-store and default-allowlist scenarios against production Rust code. `task check` includes
that test through the workspace suite. Contributors need ESS 0.50.x and AEP on `PATH`.
