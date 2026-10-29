---
format: aep.planning-md/3
id: external-blocker:substrate-0-16-0-release
kind: external-blocker
status: cleared
title: Substrate 0.16.0 is not yet tagged and published
relations:
- blocks: story:substrate-0-16-0-pin
revision: 3
transitions:
- {from: "open", to: "cleared", at: "2026-10-29T22:54:37Z", actor: "human:timo", revision: 3}
---
# Substrate 0.16.0 is not yet tagged and published

## Cleared

Substrate 0.16.1 (tag `0.16.1`, commit `f2fd606b88a0b778970e8babe9a96eb600cd57b2`) was published on
2026-10-29. Tag `0.16.0` never published, so 0.16.1 is the first release of its scope, and
`story:substrate-0-16-0-pin` is retargeted to it.
