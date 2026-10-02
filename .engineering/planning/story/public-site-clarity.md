---
format: aep.planning-md/3
id: story:public-site-clarity
kind: story
status: implemented
title: Clarify Mantle YAML and security architecture in a local website preview
relations:
- informed_by: story:public-repository-and-site
- decomposes: epic:vertical-slice
scope:
- confidence: cited
  path: website/index.html
- confidence: cited
  path: website/styles.css
revision: 8
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T11:27:37Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-02T11:27:37Z", actor: "human:timo", revision: 4}
- {from: "active", to: "implemented", at: "2026-10-02T11:37:24Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"test_result":1}}}
---
## Outcome

Refine the current dark green Mantle page with static YAML syntax highlighting, a brief Substrate explanation and a responsive technical security-boundary diagram. The operator approved implementation of the plan on 2026-10-02 and selected preview for review, with no publication.

## Acceptance

The local preview passes YAMLTextPreserved, ArchitectureMatchesImplementation, ResponsiveAccessible and StaticSiteValid, with desktop/mobile screenshots and a retained managed worktree ready for operator review.

## Named acceptance scenarios

- YAMLTextPreserved: the highlighted manifest's code text is byte-for-byte identical to the published example; the filename/language caption is outside the code and token contrast is at least 4.5:1.
- ArchitectureMatchesImplementation: the diagram depicts the laptop, trusted worker host, nested confined session and admitted external services; labels show SSH socket forwarding, the secret-slot/launcher/agent path and the aperture/CONNECT gateway path. Mounts and limitations agree with the published source.
- ResponsiveAccessible: at 1440, 768 and 390 CSS pixels and 200% zoom, labels remain readable, the document has no horizontal overflow, code scrolls within its block, keyboard navigation reaches the diagram and reading order remains logical.
- StaticSiteValid: task check passes the existing Rust, ESS, AEP and static documentation checks; the site contains no scripts.

## Design and scope

Authored spans supply YAML token classes, with no runtime highlighter. Nested HTML/CSS boxes form the diagram with descriptive text, a legend, labeled paths and a stacked mobile layout. Explain Substrate as the per-host execution data plane and Mantle as the session/placement layer. Preserve fixed allowlist, CPU, storage, credential and trusted-administrator qualifications.

Cited surfaces: website/index.html and website/styles.css; the existing mantle-docs builder embeds these files unchanged. No new runtime entity, API, schema, ESS model or delivery workflow. Keep the published five-domain/304-scenario documentation. Baseline: a7b26668a20dce232c5e068989b6818bdefa4171, verified against remote main and live site provenance.

## Delivery

The operator reviewed the preview and explicitly requested on 2026-10-02: "okay, publish it like this". Publish the approved HTML/CSS through the existing Mantle common-Gates and project-site workflow, then verify the exact live source revision. The separately requested historical identity normalization is recorded in story:bot-history-normalization and blocked by credential-blocker:bot-actions-secret-permission; it does not block a normal bot-authored website update on admitted history.

## Progress

The operator approved the desktop/mobile preview and authorized publication on 2026-10-02. The full repository gate passed, including all 304 native ESS scenarios. Final documentation tests/build and browser checks passed at 1440, 768 and 390 CSS pixels plus 200% CSS zoom. YAML bytes are unchanged; minimum syntax-token contrast is 8.97:1. See `.engineering/reports/public-site-clarity.md` for source citations and results.

The approved HTML/CSS is being delivered from `docs/mantle-site-clarity` through the bot and existing common Gates. Verify repository and documentation workflows and the live site provenance against the resulting commit. Historical identity normalization is separate and remains pending the bot Secrets permission recorded by credential-blocker:bot-actions-secret-permission.
