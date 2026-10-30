---
format: aep.planning-md/3
id: story:publish-stages-inside-checkout
kind: story
status: proposed
title: Publish stages release files inside the checkout and passes relative paths
relations:
- decomposes: epic:reliability-and-usability
revision: 2
transitions:
- {from: "draft", to: "proposed", at: "2026-10-30T01:52:18Z", actor: "human:timo", revision: 2}
---
# Story: Publish stages release files inside the checkout and passes relative paths

`mantle-release publish` copies the release files into `tempfile::tempdir()` and passes their absolute paths to `b10x-gates gh … release create` (`crates/mantle-release/src/publish.rs:79-121`). Gates checks every positional argument as text (`gates/src/main.rs:209-238`). When the temporary directory is under the operator's home, the first file argument breaks the `personal-paths` rule and the upload is refused. That is what refused 0.3.0 twice. The 0.3.1 upload only went through because `TMPDIR` was set to `/proc/self/cwd/target/pubstage` (D249).

## Acceptance

- `publish` stages the files in a new private directory inside the source checkout (for example under `target/`), and passes the file paths to `gh` relative to the checkout. `gh` resolves relative paths against `--repo`, so no file argument names the operator's home. The notes file may stay absolute, because Gates reads that file's content instead of checking its path.
- A test shows that the `release create` arguments contain no absolute path to a file, whatever `TMPDIR` is set to.
- The release procedure on the site and in the README no longer needs a `TMPDIR` override.

## Scope

`crates/mantle-release/src/publish.rs`, its tests, `website/pages/releases.html` (maintainer release procedure).
