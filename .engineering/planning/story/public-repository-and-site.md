---
format: aep.planning-md/3
id: story:public-repository-and-site
kind: story
status: active
title: Publish Mantle with common Gates and a standalone documentation site
relations:
- decomposes: epic:vertical-slice
- depends_on: story:ess-contract
scope:
- confidence: cited
  path: .github/
- confidence: cited
  path: AGENTS.md
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Cargo.toml
- confidence: cited
  path: README.md
- confidence: cited
  path: Taskfile.yml
- confidence: cited
  path: crates/mantle-docs/
- confidence: cited
  path: website/
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T08:52:30Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-02T08:52:30Z", actor: "human:timo", revision: 4}
---
## Outcome

Create the public beyond10x/mantle repository, enroll its exact identity and adoption baseline in
common Gates, install coordinated hooks and remote branch authority, and publish the implemented
source including the ESS specification. Build a public documentation site at
https://beyond10x.github.io/mantle/ using the same immutable Website project-site publication
boundary as ESS, without integrating a /docs/mantle collection into the organization portal.

## Acceptance

The public repository exposes the exact bot-authored source with green repository/common checks,
and the live /mantle/ page and provenance identify that source commit and serve its usable docs.

## Authorization and sources

The operator explicitly requested public creation, Gates onboarding and end-to-end website
publication on 2026-10-02. The existing spec is in story:ess-contract. Reference contracts:
`gates/docs/adoption.md`, `ess/.github/workflows/pages.yml`,
`ess/.github/workflows/b10x-docs-site.yml`, and `website/.github/workflows/project-site.yml`.
Use protected local policy and bot credentials only through the established tooling.

## Progress

Public repository created through the bot App: https://github.com/beyond10x/mantle (id 1401410271).
App-only branch authority is active (ruleset 24355562). Common Gates local hooks are installed;
protected policy enrollment is prepared in its managed tree. The historical baseline audit has
two personal-path findings in existing immutable session review records; no exceptions were added.

The documentation source, Rust builder, repository CI, shared-Gates caller and immutable Website
project-site caller are ready locally. `task check` passed with exit 0, including 57 local ESS
scenarios and site generation. Desktop and mobile previews rendered successfully.

The operator asked to see the page before publication. Publication is paused at that review:
no source commits or website have been pushed. Local preview: http://127.0.0.1:4178/mantle/.
Next owner: operator reviews the page; agent resumes publication after that feedback.

Remote common CI setup is separately blocked: the bot App GET of the repository Actions secrets
public-key endpoint returned HTTP 403. Installing B10X_GATES_POLICY requires the App's Actions
secrets permission; do not use a personal account or another client to bypass the refusal.

## Review

The operator reviewed the desktop/mobile site preview and explicitly approved it: “looks good,
approved”. Publication of the approved page and the source resumes under the original request.
