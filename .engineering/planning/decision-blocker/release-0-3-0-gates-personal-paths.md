---
format: aep.planning-md/3
id: decision-blocker:release-0-3-0-gates-personal-paths
kind: decision-blocker
status: cleared
title: 'Release 0.3.0 upload refused: Gates reads /home/ strings in the archives as personal paths'
relations:
- blocks: task:release-0-3-0
revision: 6
transitions:
- {from: "open", to: "cleared", at: "2026-10-30T00:53:03Z", actor: "human:timo", revision: 5}
---
# Release 0.3.0 cannot be uploaded: Gates refuses the release archives as personal paths

Tag `0.3.0` (annotated, b10x-bot[bot]) names `1811ebe2a03e0186c74d46f4d5cd39d388af4aab`, the merge of pull request 10. The tag's CI run 40977750914 passed the gate and built the verified Linux release candidate `mantle-linux-release-1811ebe2…`, with glibc 2.39 in its manifest. `sha256sum -c SHA256SUMS` passes on the downloaded candidate.

`mantle-release publish --tag 0.3.0` ran twice, and both times `b10x-gates gh … release create` refused it before any write: `argument 11 breaks 1 private rule(s): line 1 personal-paths`. No GitHub release exists for 0.3.0 (`gh release view 0.3.0` answers "release not found").

Cause, observed by grepping the candidate. Gates scans the content of each uploaded file, and the two archives contain `/home/` strings. The GNU archive has 1935 occurrences of `home/runner` under the root: the hosted runner's cargo registry paths that Rust embeds in panic locations. It also contains `home/workspace` under the root, which is Mantle's own in-sandbox path constant (for example `home/workspace/.mantle/cargo/…` under the root). The musl archive has 34 lines with such strings. Neither path is personal. Release 0.2.0 shipped the same kind of binaries on 2026-10-03, before this content scan refused them.

## Options

- A. Gates admits release-asset uploads whose `/home/` strings are only `home/runner` under the root and `home/workspace` under the root, through a policy allowance or a `personal-paths` scope change in Gates. Then publish the existing 0.3.0 candidate unchanged. This is not Mantle's change; it belongs to the Gates and policy owner.
- B. Mantle's build remaps the registry prefix (`--remap-path-prefix`) and stops spelling the in-sandbox home as a `/home/…` literal. Then release 0.3.1. The tag 0.3.0 stays unpublished.
- C. Publish through a path that skips the Gates scan. Not acceptable: a refused delivery tool is a stop.

Recommendation: A. The strings are not personal data, and B changes runtime paths only to satisfy a scanner.

Decider: operator

## Decision (D244)

Decided by b10x-chef-orchestrator under the operator's grant, 2026-10-30, D244: option B. Rebuild without the two strings and release 0.3.1.

Reason, as given: `gates/src/scan.rs:314-317` says allowances never reach `personal-paths`, on purpose, so A would change a Gates design rule just to publish one release. C stays refused.

Tag `0.3.0` stays as it is: not deleted, not moved, with no GitHub release.

Correction to the cause above. Mantle's in-sandbox paths are `/workspace/.mantle/home` and `/workspace/.mantle/cargo` (`crates/mantle/src/adapters/substrate.rs`, `crates/mantle-launch/src/cli.rs`, `crates/mantle/src/app/session.rs`). The source has no `home/workspace` literal. The `home/workspace` strings in the 0.3.0 binaries are adjacent read-only strings: `/workspace/.mantle/home` followed directly by `/workspace/.mantle/cargo`. No runtime path changes for 0.3.1.

Done for 0.3.1: `mantle-release build` passes `--remap-path-prefix` for `CARGO_HOME` (to `/cargo`), the source export (to `/mantle`) and the build target (to `/target`). A local candidate built from `12fe423` passes `b10x-gates scan-text` (exit 0 on both archives, the manifest and `SHA256SUMS`); its only `/home/` strings are inside `/workspace/.mantle/home…`, which the rule does not match. The tag's CI candidate is scanned the same way before publishing.

## Premise corrected (D249)

The premise of this blocker and of D244 was wrong: Gates refused the absolute staging path of the first release file, which was under the operator's home, and did not refuse the runner-home (`home/runner`) strings inside the binaries. See `approval-record:d249-publish-staging` (D249) for the evidence and the fix, `story:publish-stages-inside-checkout`.
