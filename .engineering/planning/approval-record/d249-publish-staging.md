---
format: aep.planning-md/3
id: approval-record:d249-publish-staging
kind: approval-record
status: approved
title: 'D249: publish 0.3.1 with staging under the checkout; fix publish staging next wave'
relations:
- decides: story:publish-stages-inside-checkout
- decides: task:release-0-3-1
revision: 2
transitions:
- {from: "draft", to: "approved", at: "2026-10-30T01:52:19Z", actor: "human:timo", revision: 2}
---
Decided by b10x-chef-orchestrator under the operator's grant on 2026-10-30, as D249: publish 0.3.1 with the release files staged under the checkout (option a), and file the code fix as `story:publish-stages-inside-checkout` for the next mantle wave (option b).

Reason, as given: Gates refused the home path in the argument, not the contents of the files (`gates/src/main.rs:209-238`). On GitHub the asset name is the file's basename, so a staging path outside the home directory supplies exactly the input the refusal names. Gates still checks every argument, and the files are the CI build that passed.

## D244's premise was wrong

D244 (`decision-blocker:release-0-3-0-gates-personal-paths`) assumed that Gates refused the 0.3.0 upload because the binaries contain runner-home (`home/runner`) strings. That was wrong. Gates never reads the release files' contents. It checks positional arguments as strings, and argument 11 was the first file's absolute staging path under the operator's home. The `scan-text` runs that reproduced the refusal tested a check the upload does not perform. The 0.3.1 build path remapping is harmless but did not fix the refusal: the 0.3.1 upload was refused the same way until `TMPDIR` was set to `/proc/self/cwd/target/pubstage`. Evidence: `b10x-gates gh … -- release view 0.3.1 <a path under the home directory>` is refused at that argument, and the same command with `/proc/self/cwd/target/x.tar` passes the check.

The 0.3.1 CHANGELOG entry and the 0.3.1 entry on the history page repeat the wrong cause ("the organization's release checks refused to upload them"). Correct both on `main` in the next wave. The CHANGELOG at tag 0.3.1 stays as released.

Result: Mantle 0.3.1 published from tag `0.3.1` (`2775135cc9f4da0cf58902e1db493ebe9d13befa`), the tag's CI candidate (run 40985494178, glibc 2.39). `verify-release.sh mantle 0.3.1` exit 0.
