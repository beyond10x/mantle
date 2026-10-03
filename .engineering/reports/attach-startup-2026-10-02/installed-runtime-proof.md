# Installed Codex attachment — 2026-10-03

The original continuously drained 100×50 PTY failed with
`session.output-backpressure` on the older worker. See [diagnosis.md](diagnosis.md).
The corrected attachment path now passes against an installed, published runtime.

## Qualified components

- Mantle 0.1.3 implementation: `fac3b79` (client readiness and released runtime pins).
- Codex: 0.153.4, the verified worker installation.
- Substrate: [0.7.10](https://github.com/beyond10x/substrate/releases/tag/0.7.10),
  source `65304edf6ebdf4a95f9c2c6138b0c20ea47d157e`.
- Daemon image: `sha256:90469e101c828c7e88fbf1e98c63ec82b9b010e022cbe8432bfc0619c38d1511`.
- Normal daemon layer: `sha256:4812b85baaf0bdc80f08673e21f5daf1e141eaf0646b20f040b1965de99bf281`.
- Installed daemon binary: `sha256:cdfed912f75676fc2c99370c36699ae88d097b47f92fcb29f5d70fa2379aa707`.

The release recovery workflow succeeded, and the coordinator independently verified all three
published artifact signatures against its exact protected-main workflow identity and GitHub
Actions issuer. A separate reviewer verified the manifest, source-labelled configuration,
normal-daemon layer and extracted binary bindings. The installed binary reported 0.7.10 and
its systemd service was active.

## Live results

A separate KubeVirt worker preserved all original worker sessions and workspaces. A real Codex
process ran under the normal Mantle confinement and recorded terminal transport. A bounded Rust
probe attached the actual CLI through a 100×50 PTY, continuously drained output, answered cursor
queries and sent the normal detach keys after 12 seconds. It retained byte counts and exit status,
not terminal contents or authentication codes.

| Observation | First attach | Reconnect to the same agent |
| --- | ---: | ---: |
| CLI exit status | 0 | 0 |
| Terminal bytes drained | 329,157 | 346,630 |
| Cursor queries answered | 3 | 2 |
| Detach sent | yes | yes |
| CLI stderr bytes | 0 | 0 |

Substrate continued to observe the same agent execution as `Running` after reconnect and detach.
Neither attachment reported `session.output-backpressure`. The same worker advertised enforced
memory and process limits and no-egress confinement. Storage quota and CPU-core enforcement were
not claimed.

The complete local integration gate passed: formatting, Clippy, workspace tests, all 336 native
ESS scenarios, specification validation, AEP validation and site generation. The laptop CLI and
all three static musl worker binaries built successfully.

## Limits and retained evidence

This proves installed Codex attachment and reconnect through a real PTY. It does not establish
visual rendering in Ghostty or Terminator, device authentication, model-issued tool turns,
token refresh or complete live lifecycle parity. No authentication file was read or copied.
The earlier [synthetic replay proof](combined-proof.log), including its cleanup API
`operation.outcome-unknown` result and subsequent absence checks, remains separate evidence.

While the release was retrying a GitHub OIDC timeout, a provisional local source binary could
not start on the new worker because it required a host-specific libgit2 library. The baseline
was restored before installing the published image's binary; no session or successful transport
result is attributed to that provisional attempt.
