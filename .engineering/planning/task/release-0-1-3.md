---
format: aep.planning-md/3
id: task:release-0-1-3
kind: task
status: active
title: Ship the compatible Codex attachment runtime and updated docs
relations:
- delivers: story:attach-startup-readiness
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T22:52:53Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-02T22:52:53Z", actor: "human:timo", revision: 3}
---
## Outcome
Release Mantle0.1.3 with terminal readiness and compatible verified Substrate0.7.10 SDK/runtime pins. Continue the user's original request to ship usable Codex integration and updated docs, reinforced by the request to keep working after the prior release stop.

## Scope
Root owns Cargo workspace version/lock, release and artifact evidence. Runtime implementor owns crates/mantle/Cargo.toml, Cargo.lock dependency resolution and crates/mantle/src/app/worker.rs. Documentation unit owns README.md, website/index.html and docs/evidence/codex-compatibility.md, with no source-code overlap. Root serializes shared lock/version changes after runtime unit returns.

## Acceptance
Full Mantle gate, native336-scenario ESS inventory, static worker builds, exact tag required CI and artifacts. Public docs/source updated together; publication provenance only claimed after observed. Describe bounded queue fix and coordinated worker upgrade accurately. Do not claim authenticated model/tool turns, no-recording terminal mode or installed acceptance from synthetic transport tests. Verify installed attach separately; preserve existing sessions/workspaces during acceptance.
