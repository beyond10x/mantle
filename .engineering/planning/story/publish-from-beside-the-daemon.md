---
format: aep.planning-md/3
id: story:publish-from-beside-the-daemon
kind: story
status: draft
title: Publish a session's commits from beside the daemon
summary: 'Glovebox wave 2 (SM-D201): mantle publish pushes a session''s commits from the worker host, outside the sandbox, with a credential the box never sees; first step toward the design 16 broker.'
tags:
- glovebox
relations:
- decomposes: epic:substrate-0-16
revision: 1
---
# Story: publish a session's commits from beside the daemon

Part of Glovebox (atlas `initiative:glovebox`, `docs/design/glovebox.md`), wave 2.

## Why

- Glovebox rule SM-D201: "Every other credential stays outside the box. Git pushes happen beside
  the daemon" (`atlas/docs/design/glovebox.md:58-59`). Wave 2 assigns Mantle "a publish step that
  pushes a session's commits from the worker, outside the box" (`atlas/docs/design/glovebox.md:91`);
  the gap table records "Push from inside | no route" (`atlas/docs/design/glovebox.md:80`).
- Mantle design §16 wants Git-native output without "a long-lived GitHub token in the session" and
  names a Git credential broker as "the target architecture"
  (`docs/design/00-mantle-on-substrate.md:862-907`); its CLI sketch lists `mantle publish
  <session>` (`docs/design/00-mantle-on-substrate.md:2010`). Nothing implements it: `publish` in
  `crates/mantle/src/` occurs only for profiles (`crates/mantle/src/profile.rs:183`).
- The only credential Mantle delivers into a sandbox is the agent's, through the Substrate secret
  slot `claude` (`AGENTS.md:24-25`).
- This story is the first step toward the §16 broker: the push happens outside the sandbox, and no
  Git credential enters it at all.

## What changes

1. The ESS specification first (`spec/domains/session.yaml` or a new domain): the publish
   operation, its inputs (session, repository name from the manifest, target branch) and its
   refusals.
2. `mantle publish <session> [--repository <name>] [--branch <name>]` runs on the worker host,
   outside the sandbox: it reads the session's commits from the workspace's Git directory on the
   host side, checks they descend from the materialized commit, and pushes them to the target
   branch with a credential held on the worker host or on the laptop, never in the workspace, the
   sandbox's environment or a secret slot.
3. A push never force-updates a branch, and never pushes to the repository's default branch unless
   the manifest names it.
4. The session's record names each publication: repository, branch, pushed commit, time.

## Acceptance

- The specification validates, and conformance passes with scenarios for publish and its refusals.
- After a session commits in its workspace, `mantle publish` pushes exactly those commits to the
  named branch of a test remote; the remote's branch head equals the session's commit.
- A test asserts that the exec request of the session carries no Git credential: no secret slot
  other than the agent's, no credential environment variable, no credential file in the workspace.
- Publishing a session whose commits do not descend from the materialized commit, to a branch
  whose update would not be a fast-forward, or with no commits is refused by name.

## Blocked on / Depends on

- Depends on nothing outside Mantle. takt's `story:run-the-next-story-in-a-mantle-session` uses it.
- Later: the §16 broker (`docs/design/00-mantle-on-substrate.md:879-907`) for pushes requested
  from inside the session; out of scope here.
