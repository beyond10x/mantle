---
format: aep.planning-md/3
id: task:reliability-hardening-mutations
kind: task
status: implemented
title: Verify green reliability rules with planted implementation defects
relations:
- delivers: epic:reliability-and-usability
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T13:50:43Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-03T13:50:43Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-03T14:24:57Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":1,"review_outcome":1,"verification":1}}}
---
## Outcome
Run a bounded ESS hardening mutation audit against already-green Mantle reliability behavior, as the operator explicitly requested. Compile canonical IR and choose at least four independently observable rules spanning truthful exec outcomes, profile isolation/override refusal, read-only doctor and release verification. Use the native implementation and real retained scenarios; no interpreted model substituted for Mantle.

## Acceptance
First establish a green baseline, then plant one known-kill defect to demonstrate the harness fails at a named scenario. Run selected implementation-rule mutations one at a time, preserving exact patches, commands, exit statuses and named failures; fully restore after each and rerun green. Report survivors honestly, distinguish spec gaps/implementation defects/technique false positives, and list techniques not run. Prefer native mutation CLI if its custom-runner seam fits; if it does not, use the skill's manual implementation-mutation procedure and state that choice. Do not build a new broad harness just to use every technique.

## Scope
Isolated managed worktree from source equivalent to the last green381-case integration gate, unique build and temporary paths, canonical IR for rule selection. No mutation of worker-upgrade worktree, root integration source, live sessions, credentials or providers. Root owns AEP and integration. Any retained executable test/harness code is Rust with clap derive for a command line. Never commit planted defects. Report small evidence to root for the single final PR.
