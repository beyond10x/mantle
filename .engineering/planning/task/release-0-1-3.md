---
format: aep.planning-md/3
id: task:release-0-1-3
kind: task
status: implemented
title: Ship the compatible Codex attachment runtime and updated docs
relations:
- delivers: story:attach-startup-readiness
revision: 10
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T22:52:53Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-02T22:52:53Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-03T00:11:33Z", actor: "human:timo", revision: 9, decided_on: {"recorded":{"test_result":2}}}
---
## Outcome
Release Mantle0.1.3 with terminal readiness and compatible verified Substrate0.7.10 SDK/runtime pins. Continue the user's original request to ship usable Codex integration and updated docs, reinforced by the request to keep working after the prior release stop.

## Scope

Root owns Cargo workspace version/lock, release, public installed-runtime proof and AEP evidence. Runtime implementor owns crates/mantle/Cargo.toml, Cargo.lock dependency resolution, crates/mantle/src/app/worker.rs and the learned crates/mantle/src/app/session.rs wire-default assertion/status-version correction. Documentation unit owns README.md, website/index.html and docs/evidence/codex-compatibility.md. Root serialized shared lock/version changes after runtime unit return. No capture-policy change or original worker restart.

## Acceptance
Full Mantle gate, native336-scenario ESS inventory, static worker builds, exact tag required CI and artifacts. Public docs/source updated together; publication provenance only claimed after observed. Describe bounded queue fix and coordinated worker upgrade accurately. Do not claim authenticated model/tool turns, no-recording terminal mode or installed acceptance from synthetic transport tests. Verify installed attach separately; preserve existing sessions/workspaces during acceptance.

## Review and build progress

Independent reviewer Kuhn found no SDK-adoption defect in the original0118 pin; final65304 pin adds only generated release notice labels. Root reviewed and merged the SDK update f1395f73 as1d4de26; workspace version0.1.3 and its six lock entries are93b2085. Root reviewed README/site/evidence documentation f3220f74 and merged it as c631bd5. Native CLI and all three static musl worker binaries built successfully. Complete integration gate remains pending the verified published Substrate0.7.10 worker manifest/layer; no installed or authenticated acceptance claim yet. Existing worker remains untouched. A separate namespace/profile has been prepared for actual PTY attach/detach/reconnect acceptance.

## Isolated runtime acceptance

The release encountered a GitHub Actions OIDC timeout and is retrying through Substrate's protected-main recovery. To overlap VM setup with that external wait, root may bootstrap the separate mantle-codex-013 worker using the existing released0.7.8 bootstrap, then install the fully gated exact65304 Substrate0.7.10 source build while that new worker has no user sessions. Its GLIBC requirements were observed to end at2.39, matching Ubuntu24.04. This is provisional source-build acceptance, never published-image qualification. Before final handoff, use the signed0.7.10 image binary and final Mantle0.1.3 pins and repeat attach acceptance. Original namespace mantle and every original workspace remain untouched. Only newly created disposable acceptance sessions may be stopped/recreated before handoff; no user authentication is captured or replayed.

## Provisional binary portability finding

The new isolated worker completed its official0.7.8 bootstrap with no sessions. The exact65304 local source binary reports0.7.10 but could not start there because libgit2.so.1.9 is absent; matching the GLIBC ceiling alone was insufficient portability evidence. The native host also links libllhttp.so.9.3. No Codex session was created. Root restores the published0.7.8 baseline on that empty new worker and waits for the Ubuntu-compatible release image; no ad-hoc library copying or production-source change is introduced. Original worker remained untouched. The first provisional session attempt returned worker-not-ready while the service transition was pending and is not a transport regression result.

## Release verification

Released https://github.com/beyond10x/mantle/releases/tag/0.1.3 at 2026-10-03T00:10:11Z. Annotated bot tag77c1f2ed5a34c3f2fbc0e33523ff4bfa80e5d591 resolves to exact candidate367c7beb440feaec842ba5f8fe141b0fcd0f0cf1. PR3 merged by app/b10x-bot as6058df06 with identical tree82d36448. Exact candidate CI37079969914 and tagged common gate37080872271 passed. GitHub source archive downloaded and version inspected; SHA256492af851c32a065d96b1f8cf0aa63370e542aa11e315ae57c8cc10098fce0d19. No prebuilt binaries promised. Live site provenance at /mantle/.well-known/b10x-site.json reports6058df06ac244d3f94c03903936eab30d7b23834, so updated documentation publication is verified. Published Substrate0.7.10 installed proof is in the committed report. Disposable verification session stopped successfully and workspace destruction was confirmed. The separate versioned CLI leaves the original0.1.2 CLI/worker usable for original sessions; old wave9 attachments still use the unfixed runtime.

## Operator session handoff

A new codex-read session uses exact released source367c7beb440feaec842ba5f8fe141b0fcd0f0cf1 on the isolated published0.7.10 worker. Initial materialization by tag0.1.3 failed because the existing clone path passes --no-tags with --branch; the failed disposable session was stopped and replaced using the exact tag commit. This is a checkout limitation, not evidence against terminal transport. The final12-second100x50PTY attach drained324846bytes, answered4cursorqueries, detached normally with exit0 and zero CLIstderr. Status observed the same agent Running afterward. The versioned CLI, separate configuration and separate state-directory command were handed to the operator; original sessions remain untouched. Actual interactive authentication is still for the operator to complete. Follow-up: support tag references in the existing repository materializer; exact commit references work now.
