---
format: aep.planning-md/3
id: task:release-0-3-1
kind: task
status: active
title: Release Mantle 0.3.1
relations:
- decomposes: epic:substrate-0-16
- supersedes: task:release-0-3-0
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-30T00:44:39Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-30T00:44:39Z", actor: "human:timo", revision: 3}
---
# Task: Release Mantle 0.3.1

Publish the behaviour tagged as 0.3.0, which could not be uploaded, as 0.3.1 (D244 on `decision-blocker:release-0-3-0-gates-personal-paths`, option B).

## Steps

- `mantle-release build` remaps the cargo home, the source export and the build target with `--remap-path-prefix`, so no release binary names a directory of the build host.
- Version 0.3.1, CHANGELOG, README and site pages.
- Before tagging, `b10x-gates scan-text` on both archives of a candidate built from the release source finds no `personal-paths` match.
- Bot tag `0.3.1`, the tag's CI candidate, `mantle-release publish`, then `verify-release.sh mantle 0.3.1`.

Tag `0.3.0` stays as it is.
