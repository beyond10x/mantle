# Mantle

[Documentation](https://beyond10x.github.io/mantle/) · [Releases](https://github.com/beyond10x/mantle/releases) · [Issues](https://github.com/beyond10x/mantle/issues)

Mantle runs a whole development session (Claude Code, Codex or a command you declare, its Git
repositories and its toolchain) on a remote Linux worker on AWS or KubeVirt. Your laptop renders the terminal; the
agent runs in a sandbox confined by [Substrate](https://github.com/beyond10x/substrate), and
builds use the worker's CPU. Detach, and the agent keeps running.

```console
mantle worker up
mantle start mantle.yaml        # Ctrl-] d detaches; the agent keeps running
mantle attach my-project
mantle stop my-project          # agent ends, workspace kept
```

Current release: **0.2.0**, an early release. Its 499 conformance scenarios run with controlled
external IO. Device login, authenticated model and tool turns, token refresh and a 0.2.0 run on a
live worker are **not verified end to end**:
[what is verified](https://beyond10x.github.io/mantle/releases/#what-is-verified).

## Requirements

- A Linux x86-64 laptop with `ssh`. The 0.2.0 CLI is built against glibc 2.39; older glibc,
  macOS and ARM are not qualified.
- AWS worker: the AWS CLI, `session-manager-plugin`, and a profile that may create EC2, IAM and
  CloudWatch resources. KubeVirt worker: `kubectl` and `virtctl` with access to the cluster.
- Claude Code: an OAuth token from `claude setup-token`. Codex: a ChatGPT account for device
  login.
- To install: `curl`, `jq`, `sha256sum`, `tar`. No Rust compiler.

## Install a verified release

```sh
mkdir mantle-0.2.0 && cd mantle-0.2.0
for f in manifest.json SHA256SUMS \
    mantle-0.2.0-x86_64-unknown-linux-gnu.tar \
    mantle-0.2.0-x86_64-unknown-linux-musl.tar; do
  curl -fLO "https://github.com/beyond10x/mantle/releases/download/0.2.0/$f"
done
jq -r .source_commit manifest.json   # 54710558ebe096a8e5faf98bbb430ff7f9eaa17e
jq -r '.artifacts[] | "\(.sha256)  \(.name)"' manifest.json | sha256sum --check
mkdir bootstrap
tar --extract --file mantle-0.2.0-x86_64-unknown-linux-gnu.tar \
  --directory bootstrap --no-same-owner bin/mantle-release
./bootstrap/bin/mantle-release verify --manifest "$PWD/manifest.json"
./bootstrap/bin/mantle-release install --manifest "$PWD/manifest.json" \
  --prefix "$HOME/.local/mantle"
export PATH="$HOME/.local/mantle/bin:$PATH"
mkdir -m 0700 worker-stage
tar --extract --file mantle-0.2.0-x86_64-unknown-linux-musl.tar \
  --directory worker-stage --no-same-owner \
  bin/mantle-egress bin/mantle-launch bin/mantle-worker
```

What each step proves, and building from source:
[releases and verification](https://beyond10x.github.io/mantle/releases/).

## A first session

1. Copy [examples/config.toml](examples/config.toml) to `~/.config/mantle/config.toml`, choose
   `provider = "aws"` or `"kubevirt"` and replace every placeholder. For Claude Code, set
   `claude.token_command` to a command that prints your token.
2. Save a manifest as `mantle.yaml`. For Codex, use `kind: codex` and `auth: chatgpt-device`
   ([examples/codex.yaml](examples/codex.yaml)). To run a program of your own, such as a login
   shell, use `kind: command` with an `argv` whose first element is an absolute path, and no
   `auth` ([examples/shell.yaml](examples/shell.yaml)); it receives no agent credential. The
   command kind is not in 0.2.0.

   ```yaml
   apiVersion: mantle.beyond10x.dev/v1alpha1
   kind: Session
   metadata:
     name: my-project
   workspace:
     repositories:
       - name: mantle
         repository: https://github.com/beyond10x/mantle.git
         ref: main
         mount: mantle
   agent:
     kind: claude-code
     cwd: /workspace/mantle
   ```

3. Start, from the directory you installed in:

   ```console
   mantle worker up --binaries "$PWD/worker-stage/bin"
   mantle start mantle.yaml
   mantle attach my-project
   mantle stop my-project            # agent ends; workspace kept
   mantle destroy my-project --yes   # workspace deleted
   mantle worker down                # VM stopped; data volume kept
   ```

## Documentation

| Page | Contents |
|---|---|
| [Architecture](https://beyond10x.github.io/mantle/architecture/) | Laptop, Mantle, Substrate and worker, and what each owns |
| [Configuration](https://beyond10x.github.io/mantle/configuration/) | `config.toml` and every manifest field, with defaults and bounds |
| [Operations](https://beyond10x.github.io/mantle/operations/) | Commands, retained workspaces, profiles, doctor, automation, acceptance runs, offline worker upgrades |
| [Releases and verification](https://beyond10x.github.io/mantle/releases/) | Bundles, verification, source builds, the conformance suite, the maintainer release procedure |
| [Security model](https://beyond10x.github.io/mantle/security/) | Network allowlist, credentials, recording, what the sandbox does not cover |
| [Release history](https://beyond10x.github.io/mantle/history/) | 0.1.0 to 0.2.0, compatibility changes and recorded evidence; summary in [CHANGELOG.md](CHANGELOG.md) |

**Upgrading from 0.1.4:** `mantle stop` now keeps the workspace; use `mantle destroy NAME --yes`
to delete it. Existing workers need an
[offline upgrade](https://beyond10x.github.io/mantle/operations/#offline-upgrades).

## Repository

- Design: [docs/design/00-mantle-on-substrate.md](docs/design/00-mantle-on-substrate.md). Its
  correction block lists where the implementation differs.
- Qualification evidence: [docs/evidence/codex-compatibility.md](docs/evidence/codex-compatibility.md).
- Executable specification and coverage: [spec/README.md](spec/README.md).
- Contributor and agent instructions: [AGENTS.md](AGENTS.md). The gate is `task check`; it needs
  Rust 1.97, [go-task](https://taskfile.dev), ESS 0.77.0 and AEP 0.68.0.
- The site source is `website/`, built by `crates/mantle-docs` with `task site-build`.

Licensed under [Apache-2.0](LICENSE).
