---
format: aep.planning-md/3
id: story:bot-history-normalization
kind: story
status: draft
title: Normalize all published Mantle commit identities to the bot
relations:
- informed_by: story:public-repository-and-site
revision: 2
---
## Outcome

The operator requested on 2026-10-02 that all Mantle commits use the bot identity, correcting seven historical commits whose author and committer use a personal account. The public repository advertises only main and no tags. The three post-adoption commits already have the exact bot identity.

## Acceptance

Every commit reachable from published main has the exact b10x-bot[bot] author and committer, each rewritten commit preserves its tree, message, timestamps and parent topology, the corresponding Gates baseline is enrolled locally and in CI, and fresh checks plus site provenance validate the resulting history.

## Current blocker

The existing adoption baseline is 11b69db7b86076299a9dd92cbbc2c0196148c007, one of the personal commits. Rewriting it changes the baseline identifier, so the local policy and repository B10X_GATES_POLICY secret must be coordinated. The bot App request to GET /repos/beyond10x/mantle/actions/secrets/public-key returned: `b10x-gates: GitHub operation failed (403 Forbidden); response withheld`. No personal credential fallback is authorized. Do not force-push a history that would sever the enrolled baseline.

## Safe execution after access is restored

Retain recovery proof and an old-to-new commit map. Rewrite only the required identity metadata and descendant parent references, verify every mapped tree is identical, and preserve merge topology. Coordinate the identity-equivalent baseline replacement and regenerate Gates receipts. Publish through the bot with an exact remote-head lease. Reconcile the active Codex worktrees with their owners rather than rewriting their refs or dirty work. Verify bot identities and the deployed source revision afterwards.
