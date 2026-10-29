---
format: aep.planning-md/3
id: story:public-docs-sweep
kind: story
status: active
title: Public documentation ready to share
relations:
- decomposes: epic:substrate-0-16
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: README.md
- confidence: cited
  path: crates/mantle-docs
- confidence: cited
  path: docs/guides
- confidence: cited
  path: website
revision: 8
transitions:
- {from: "draft", to: "proposed", at: "2026-10-29T12:59:18Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-29T12:59:18Z", actor: "human:timo", revision: 3}
---
# Story: Public documentation ready to share

## Why

The operator wants to share Mantle soon (2026-10-29). README.md is 488 lines and interleaves the
current 0.2.0 behaviour with historical 0.1.x release notes, compatibility warnings and maintainer
procedures. A first reader has to work out what is current and what is history.

## What changes

- README.md: what Mantle is, how to install a verified release, a five-minute first session, then
  links. Historical release notes move to CHANGELOG or a release-history page; maintainer
  procedures (release build, publication, offline worker upgrade) move to their own pages.
- The public site (`website/`, built by `crates/mantle-docs`): the same structure, with an
  architecture page that shows the laptop, Mantle, Substrate and worker layers and what each owns.
- Every claim of verified behaviour cites its evidence in the repository; anything not verified end
  to end is labelled as such, as today.
- Links are checked; the site builds with `task site-build`.

## Acceptance

- A reader who has never seen Mantle can find, from the README's first screen, what it is, what
  it needs and the first command.
- Nothing current is removed: every fact the old README states is either on a page or recorded
  as deleted for being false, with the reason in the PR.
- `task check` and `task site-build` pass.
