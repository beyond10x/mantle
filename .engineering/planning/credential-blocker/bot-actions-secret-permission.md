---
format: aep.planning-md/3
id: credential-blocker:bot-actions-secret-permission
kind: credential-blocker
status: open
title: Bot cannot refresh the Mantle CI policy secret for a history rewrite
relations:
- blocks: story:bot-history-normalization
withholds: test_result
revision: 1
---
## Observation

The authorized bot route returned `b10x-gates: GitHub operation failed (403 Forbidden); response withheld` for GET /repos/beyond10x/mantle/actions/secrets/public-key on 2026-10-02. No alternate identity was used.

## What clears this

The GitHub App installation owner grants the bot the repository Secrets permission required to read the public key and update B10X_GATES_POLICY, or the secret owner performs the coordinated refresh after the rewritten adoption-baseline value is prepared. Local policy alone does not refresh CI's secret. A subsequent successful authorized access/refresh is the clearing evidence.

## Impact

Bot-only historical identity normalization is pending. The approved website can be published as a normal bot-authored descendant of the existing adoption baseline.
