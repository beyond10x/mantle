---
format: aep.planning-md/3
id: task:release-0-3-1
kind: task
status: implemented
title: Release Mantle 0.3.1
relations:
- decomposes: epic:substrate-0-16
- supersedes: task:release-0-3-0
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-30T00:44:39Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-30T00:44:39Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-30T01:52:29Z", actor: "human:timo", revision: 5}
---
# Task: Release Mantle 0.3.1

Publish the behaviour tagged as 0.3.0, which could not be uploaded, as 0.3.1 (D244 on `decision-blocker:release-0-3-0-gates-personal-paths`, option B).

## Steps

- `mantle-release build` remaps the cargo home, the source export and the build target with `--remap-path-prefix`, so no release binary names a directory of the build host.
- Version 0.3.1, CHANGELOG, README and site pages.
- Before tagging, `b10x-gates scan-text` on both archives of a candidate built from the release source finds no `personal-paths` match.
- Bot tag `0.3.1`, the tag's CI candidate, `mantle-release publish`, then `verify-release.sh mantle 0.3.1`.

Tag `0.3.0` stays as it is.

## Published

Published 2026-10-30: https://github.com/beyond10x/mantle/releases/tag/0.3.1. Tag `0.3.1` (annotated, b10x-bot[bot]) names `2775135cc9f4da0cf58902e1db493ebe9d13befa`, the merge of pull request 11. The files are the tag's CI candidate from run 40985494178 (glibc 2.39). Before upload, `b10x-gates scan-text` found nothing in both archives, `manifest.json` and `SHA256SUMS`. `mantle-release publish` ran with `TMPDIR=/proc/self/cwd/target/pubstage` (D249) and reported "published and verified 0.3.1". `verify-release.sh mantle 0.3.1` exit 0, with and without the announcement file.
