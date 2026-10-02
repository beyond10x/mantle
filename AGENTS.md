# Mantle — agent instructions

Mantle places whole development sessions (agent, workspace, toolchain) on a remote Linux worker
confined by Substrate. Design: `docs/design/00-mantle-on-substrate.md`.

## Rules

- **Rust only** for anything that runs. Command lines use clap derive. No shell or Python programs
  in the repository; cloud-init and systemd files under `deploy/` are declarative configuration.
- **Mantle depends on Substrate, never the reverse.** Use `b10x-substrate-sdk` pinned to an exact
  Substrate revision (`crates/mantle/Cargo.toml`). Do not import Substrate implementation crates.
- **No cloud-account identifiers in the repository.** Account ids, profile names, VPC or subnet ids
  and organization names live in the operator's `~/.config/mantle/config.toml`.
  `examples/config.toml` shows the shape with placeholder values.
- **Never report what Substrate did not observe.** Requested and applied resources are printed
  separately; an `unknown` exec is shown as unknown.
- No secret value in argv, logs, state or error messages. The Claude credential reaches a sandbox
  only through the Substrate secret slot `claude`.

## Commands

| What | Command |
|---|---|
| gate | `task check` |
| laptop CLI | `task build` |
| worker binaries (static musl) | `task build-worker` |
| specification | `ess specify validate --path spec` |
| plan | `aep plan artifact list`, `aep plan artifact validate` |

Builds use `CARGO_TARGET_DIR=$HOME/.cache/b10x-target/mantle` (set by the Taskfile).

## Planning

The AEP store in `.engineering/` is the plan. Work starts from a story there; a new noun gets its
ESS entity in `spec/domains/` before a story is written around it. Draft bodies go in the
git-ignored `.engineering/drafts/`.

## Layout

| Path | Contents |
|---|---|
| `crates/mantle` | laptop CLI: `domain/`, `app/`, `adapters/{aws,ssh,substrate,state}` |
| `crates/mantle-egress` | worker-side CONNECT proxy with a fixed allowlist |
| `crates/mantle-launch` | in-sandbox launcher: `serve` holds the agent on a pseudo-terminal, `attach` relays a terminal to it over FIFOs (the sandbox refuses AF_UNIX sockets) |
| `deploy/` | cloud-init, systemd units and the AppArmor profile embedded into the CLI |
| `spec/` | ESS specification |
