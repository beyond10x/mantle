# Mantle public-site clarity preview

The local preview adds static YAML highlighting, a short Substrate explanation and a nested security-boundary diagram. It preserves the published site's dark green theme, manifest text and five-domain/304-scenario ESS section. The initial delivery was a local preview for visual review. The operator subsequently approved the page and requested publication on 2026-10-02.

Base: `a7b26668a20dce232c5e068989b6818bdefa4171`, verified against remote main and the live project-site manifest before implementation. Work item: `story:public-site-clarity`. Branch: `docs/mantle-site-clarity`. Managed tree: `mantle-site-clarity`.

## Acceptance evidence

| Scenario | Observation |
| --- | --- |
| YAMLTextPreserved | Browser code text equals the original published manifest byte for byte at 1440, 768 and 390 CSS pixels. Caption is outside the code. Token contrast ratios: keys 13.30:1, strings 12.54:1, numbers/booleans 12.48:1, punctuation 8.97:1. |
| ArchitectureMatchesImplementation | Sources below verify laptop/host/sandbox separation, socket forwarding, toolchain and workspace mounts, credential delivery and gateway placement. Existing limits remain visible. |
| ResponsiveAccessible | Desktop, tablet and mobile document widths equal viewport widths, with no page overflow. The existing command table and YAML block scroll locally. Skip link moves focus to main; Architecture link works by keyboard; the YAML block accepts keyboard focus. A 200% CSS zoom probe remains within the viewport. Desktop/tablet/mobile diagrams inspected visually. |
| StaticSiteValid | Full `task check` exited 0. Final HTML/CSS refinements passed `cargo test --locked -p mantle-docs` (2 tests), `task site-build` and `git diff --check`. Built HTML and CSS match authored bytes. Browser found zero scripts, duplicate IDs, broken internal anchors or page errors. |

The full gate reported:

```text
Complete ESS inventory: 304 passed; 0 failed, skipped, unsupported, outside or refused
mantle v1 — 7 file(s), 216 scenario(s), valid
```

Two small accessibility repairs accompany the diagram: grid children now shrink so preformatted blocks do not widen the mobile page, and the existing skip link has a programmatically focusable main target.

## Architecture sources

- `crates/mantle/src/adapters/ssh.rs`, `Ssh::tunnel`: private local socket forwarding to the worker's Substrate Unix socket.
- `crates/mantle/src/app/worker.rs`, `ssh_for` and `install_token`: AWS/KubeVirt provider transport and protected worker credential installation.
- `crates/mantle/src/adapters/substrate.rs`: named egress aperture, Claude secret slot, toolchain path and observed capabilities.
- `crates/mantle/src/app/session.rs`, `RunRequest`, `agent_request`, `exec_request`: read-only toolchain projection, agent secret descriptor, and ordinary exec without injected credentials.
- `crates/mantle-launch/src/serve.rs`, `spawn_agent`: credential enters the agent environment; child tools may inherit it.
- `deploy/substrate.service` and `deploy/mantle-egress.service`: separate host daemon and loopback gateway, outside session confinement.
- `crates/mantle-egress/src/allow.rs` and `addr.rs`: eight exact destination pairs and public-address enforcement.
- `spec/README.md`: writable workspace, resource enforcement qualifications, fixed allowlist and current conformance coverage.

## Review handoff

Preview: http://127.0.0.1:4180/mantle/ (loopback only; serves the authored files, with no publication manifest).

The managed worktree retains the reviewed source and evidence. Logs, browser observations and desktop/mobile screenshots are retained under `.engineering/drafts/public-site-clarity/`, ignored by Git. The temporary preview server is intentionally left running on port 4180. At the preview handoff no source delivery or cloud deployment had occurred. Publication is now authorized through Mantle’s existing Gates and project-site workflow; live provenance must be verified before reporting completion. The separate historical-identity request is tracked in `story:bot-history-normalization` with its bot Secrets-permission blocker.
