unit: story:repeatable-agent-acceptance — bounded final documentation delegate
verdict: green
cases: documentation tests executed 2→2 between stable draft and final text, red 0; no tests added
origin: n/a
wrote-outside-worktree: $HOME/.cache/mantle-reliability/final-documentation/**; final-documentation-target/**; final-documentation-tmp/**; assigned shared compiler cache
needs-coordinator: yes — combine documentation and runtime branches; reconcile aggregate metrics and final native evidence

Commit: 211332ba6f6b4f6d80d3881f7ce238cad21196e1, branch unit/mantle-final-documentation.
Base: 7930ab106f3a2605ab8054552d68241c29e8566e. Author and committer verified as b10x-bot[bot].
Only README.md, website/index.html and spec/README.md changed. No runtime code, specification declarations or planning files changed.

## Change

```text
 README.md          | 169 ++++++++++++++++++++++++++++++++++++++++++++---------
 spec/README.md     |  94 +++++++++++++++++++++++------
 website/index.html |  30 +++++-----
 3 files changed, 231 insertions(+), 62 deletions(-)
```

Reconciled development versus historical 0.1.4 behavior, provider and source-build prerequisites,
candidate acquisition/three-binary GNU installation, profile/doctor navigation, offline-upgrade
anchors, retained lifecycle semantics, metadata/creation receipts, acceptance procedure and
qualification limits. The website's worker-upgrades anchor now points to current offline guidance;
offline-upgrades remains an alias and the historical notice has its own anchor. Architecture
credential delivery is explicitly Claude-specific; Codex's existing private-home/device-login path
is described separately. The specification guide now maps all six domains and eleven lifecycle
states, the current SDK pin, exact-owned acceptance and the outstanding hardening gaps. HD-10 is
reconciled; HD-11 retains the decoded-envelope boundary. No same-conversation-resume claim.

Public runner interface and report/cleanup exit semantics were confirmed by the runtime implementor.
Unknown source is explicit when the manifest option is absent; a supplied invalid/mismatching bundle
refuses. Operator attestations remain separate from machine observations. The exact printed attach
command uses the private CLI copy and resolved profile paths. Cleanup cannot adopt a name after a
lost creation receipt.

## Validation

These are documentation checks I ran, not runtime or live-qualification claims:

- `cargo run --locked -p mantle-docs -- check`: exit 0 (initial documentation patch).
- `cargo test --locked -p mantle-docs`: stable draft 2 passed; final text 2 passed, exit 0.
- `cargo clippy --locked -p mantle-docs --all-targets -- -D warnings`: exit 0, including final text.
- `cargo fmt --check -p mantle-docs`: exit 0.
- `git diff --check`: exit 0.
- Final local Markdown file-link existence checks: 11/11, each direct exit 0.
- `cargo run --locked -p mantle-docs -- build --out $HOME/.cache/mantle-reliability/final-documentation/site --commit 211332ba6f6b4f6d80d3881f7ce238cad21196e1`: exit 0 on the committed clean tree.
- Both built HTML/CSS compare byte-for-byte to source (cmp exit 0); site provenance commit/baseUrl check exit 0.

Final test output:

```text
running 2 tests
test tests::authored_links_resolve ... ok
test tests::broken_anchors_and_wrong_asset_bases_are_refused ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

No new failing-first runtime case was introduced for these reversible prose edits. The existing
documentation suite includes broken-anchor/wrong-asset negative checks; no runtime hardening
technique was added. Full repository gate remains with the coordinator.

## Source inputs and remaining coordination

Read the accepted brief/story/invariants, current session/lifecycle/state code and ESS declarations,
profile/doctor behavior, package builder/verifier/installer and Rust site builder. Read the sibling
runtime implementation's mantle-acceptance main/runner, CLI flags and app/metadata code. The source
implementor confirmed the frozen interface and runner6, metadata4, release16 and worker31 passing
cases before this commit; those are attributed inputs, not tests executed by this delegate. Remaining
offline-upgrade regression and final runtime checks continued independently.

Metrics intentionally state only measured CLI381 (221 authored + 160 generated) and validated315
authored scenarios, as directed by the coordinator. Do not report inferred499 as a measured
aggregate. After the final gate, update spec/README.md's Gate and evidence paragraph and the
website contract-panel metric/label, and refresh .engineering/reports/native from that exact run.
No new release, publication, deployed website or full authenticated live qualification is claimed.
New main-branch source links become available after integration merges.

## Retained paths and handoff

Tree: $HOME/.local/state/worktree/trees/b10x/mantle/mantle-final-documentation, clean at the commit above.
Lease codex-final-documentation released through worktree session-end. No push, cleanup or tree retirement.

Outside paths retained:

- $HOME/.cache/mantle-reliability/final-documentation/report.md (this report).
- Same scratch directory: commit.log; docs-check-initial.log; docs-tests-stable.log;
  docs-tests-final.log; docs-clippy.log; docs-clippy-final.log; docs-fmt.log;
  diff-check-initial.log; markdown-links-initial.json; markdown-links-final.json; site-build.log.
- Same scratch directory: site/index.html, site/styles.css, site/.nojekyll,
  site/.well-known/b10x-site.json. Build output is local evidence, not publication.
- $HOME/.cache/mantle-reliability/final-documentation-target/** (67 MiB compiler output).
- $HOME/.cache/mantle-reliability/final-documentation-tmp/** (empty directory, 4 KiB allocation).
- Shared assigned sccache at /dev/shm/mantle-reliability-compiler-cache, server port43179;
  used without restarting or deleting it. Jobs2, dev debug0, RUST_TEST_THREADS1.

Coordinator owns integration, aggregate-count reconciliation, adversarial review and eventual
artifact/tree cleanup. All retained logs and the small generated site remain available.
