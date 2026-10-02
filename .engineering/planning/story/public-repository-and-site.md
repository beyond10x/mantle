---
format: aep.planning-md/3
id: story:public-repository-and-site
kind: story
status: implemented
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
revision: 10
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T08:52:30Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-02T08:52:30Z", actor: "human:timo", revision: 4}
- {from: "active", to: "implemented", at: "2026-10-02T09:27:31Z", actor: "human:timo", revision: 9, decided_on: {"recorded":{"test_result":1,"approval":1}}}
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

Public repository https://github.com/beyond10x/mantle is enrolled in common Gates, with coordinated
hooks, published private policy enrollment, App-only branch/tag authority, exact bot identity
rules, required main checks and enabled secret scanning/push protection. No policy exceptions were
added. The approved website and initial specification were published, and source commit
2e6b1162a78855bfc7a28fc899e8300fcc229deb passed repository/common checks and live documentation
verification. The operator authorized gh only for installing the one policy secret; subsequent
writes use the bot App route.

Story close-specification-boundaries delivers the requested follow-up: complete native ESS
coverage and updated public coverage facts. The website remains a standalone /mantle/ project
site, using the same immutable publication boundary as ESS.

## Review

The operator reviewed the desktop/mobile site preview and explicitly approved it: “looks good,
approved”. Publication of the approved page and the source resumes under the original request.

## Verification

Repository Gate run 36988877900 passed at commit 2e6b1162a78855bfc7a28fc899e8300fcc229deb.
Shared Gates run 36988878648 and documentation validation 36988877958 passed at that same commit.
Documentation deployment 36988921559 passed. The live /mantle/ index is byte-identical to the
approved website/index.html, and /.well-known/b10x-docs.json under /mantle/ names the exact commit.
Required main checks are now Gate and common / Security and privacy, both bound to GitHub Actions.
