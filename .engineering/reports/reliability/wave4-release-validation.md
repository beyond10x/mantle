unit: story:prebuilt-release-artifacts — adversary pass 1 correction at 107f9c8bbdc9062cc24e54ac69e753d0f9e7fb68
verdict: green
cases: release package executed 10→14; red 2; native CLI count unchanged from prior 263, not rerun in correction
origin: n/a
wrote-outside-worktree: assigned scratch, target, temporary directory, compiler cache and worktree lease metadata; exact paths below
needs-coordinator: no

## 1. Unit and correction

Correct exact source binding and immutable-generation reuse for the accepted prebuilt-release story. Both unchanged adversary tests reproduced red before implementation. Root classified both findings as introduced. The immutable first report remains at `$HOME/.cache/mantle-reliability/prebuilt-release/adversary/review.md`; this separate handoff supersedes only the affected implementation evidence.

The source exporter now gets the full raw committed Git tree and unique blob bytes with bounded Git plumbing, explicitly disabling replacement objects. Every archive path, type, executable mode and byte sequence must agree; duplicate, unexpected and omitted entries refuse. This preserves bounded Git provenance parsing and source-link refusal while closing the attribute class: tracked or untracked export-subst and export-ignore cannot silently change a source-bound build. A verified file is written from its committed blob bytes. Git blob/commit replacement tests prove the original exact object is used.

Reuse enumerates the full existing generation, including required structural directories, before accepting its files. Extra root/bin entries, directories, links, missing payloads and changed entry types refuse; verified bytes and modes remain checked by the existing verifier. Unknown directories are refused before descent, bounding enumeration by the expected inventory. This concerns the destination generation being reused; prior retained generations are not executed or rewritten.

The fix is a complete inventory comparison in each location, not a list of special filenames. Two added class tests cover five source cases (local export-ignore, tracked export-subst, nested executable mode, replaced blob, replaced commit) and six generation cases (root file, root directory, nested directory, extra symlink, payload symlink, missing payload). The two original adversary tests and all ten prior release tests remain unchanged in assertions. No new dependency, ESS behavior, generated source, manifest schema, publication authority or sibling story changed. README explains exact Git verification and extra-inventory refusal.

Commit 107f9c8bbdc9062cc24e54ac69e753d0f9e7fb68 has both author and committer b10x-bot[bot], verified after the Gates bot commit. No push, tag, release, PR, live worker, provider or agent credential operation occurred.

## 2. Actual diffstat

 README.md                              |   6 +-
 crates/mantle-release/src/build.rs     | 161 ++++++++++++++++++-
 crates/mantle-release/src/lib.rs       |  46 ++++++
 crates/mantle-release/tests/release.rs | 273 +++++++++++++++++++++++++++++++++
 4 files changed, 481 insertions(+), 5 deletions(-)

## 3. Reproduced red

Command: `cargo test --locked -p mantle-release --test release adversary_`, exit 101. Verbatim output follows (only the local home path is made portable):

```text
    Finished `test` profile [unoptimized] target(s) in 0.24s
     Running tests/release.rs (/dev/shm/mantle-prebuilt-release-target/debug/deps/release-1fd8b9d8598628d0)

running 2 tests
test adversary_untracked_git_attributes_cannot_change_exact_source_export ... FAILED
test adversary_modified_generation_inventory_is_refused_without_activation ... FAILED

failures:

---- adversary_untracked_git_attributes_cannot_change_exact_source_export stdout ----

thread 'adversary_untracked_git_attributes_cannot_change_exact_source_export' (2874870) panicked at crates/mantle-release/tests/release.rs:735:9:
assertion `left == right` failed: accepted exact-source export changed committed Rust bytes using untracked Git attributes
  left: [102, 110, 32, 109, 97, 105, 110, 40, 41, 32, 123, 32, 112, 114, 105, 110, 116, 108, 110, 33, 40, 34, 49, 99, 55, 101, 102, 54, 98, 48, 56, 53, 53, 102, 56, 57, 102, 57, 99, 49, 100, 101, 49, 49, 101, 102, 98, 57, 57, 100, 49, 52, 51, 101, 51, 55, 53, 49, 51, 56, 101, 57, 34, 41, 59, 32, 125, 10]
 right: [102, 110, 32, 109, 97, 105, 110, 40, 41, 32, 123, 32, 112, 114, 105, 110, 116, 108, 110, 33, 40, 34, 36, 70, 111, 114, 109, 97, 116, 58, 37, 72, 36, 34, 41, 59, 32, 125, 10]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- adversary_modified_generation_inventory_is_refused_without_activation stdout ----

thread 'adversary_modified_generation_inventory_is_refused_without_activation' (2874869) panicked at crates/mantle-release/tests/release.rs:772:5:
installer accepted an existing generation with an extra unmanaged payload


failures:
    adversary_modified_generation_inventory_is_refused_without_activation
    adversary_untracked_git_attributes_cannot_change_exact_source_export

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 10 filtered out; finished in 3.46s

error: test failed, to rerun pass `-p mantle-release --test release`
```

## 4. Green lanes

Correction package: `cargo test --locked -p mantle-artifact -p mantle-release`, executed release 10→14, exit 0. Artifact/unit/doc entry points contain zero tests. The baseline ten is the previous implementation's measured package result; this correction did not rerun other unchanged package lanes. Verbatim output:

```text
   Compiling mantle-release v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-prebuilt-release/crates/mantle-release)
    Finished `test` profile [unoptimized] target(s) in 1.09s
     Running unittests src/lib.rs (/dev/shm/mantle-prebuilt-release-target/debug/deps/mantle_artifact-c8f34bd4e86578bd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (/dev/shm/mantle-prebuilt-release-target/debug/deps/mantle_release-58c0e9cbe9387d86)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/dev/shm/mantle-prebuilt-release-target/debug/deps/mantle_release-5176d4dd5c88dc9c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/release.rs (/dev/shm/mantle-prebuilt-release-target/debug/deps/release-1fd8b9d8598628d0)

running 14 tests
test adversary_untracked_git_attributes_cannot_change_exact_source_export ... ok
test source_identity_refuses_dirty_or_wrong_revision_even_same_version ... ok
test actual_git_archive_exports_exact_source_and_still_refuses_source_symlinks ... ok
test exact_source_export_checks_attributes_inventory_modes_and_replacement_objects ... ok
test strict_manifest_and_checksum_refusals_are_not_cli_parse_errors ... ok
test valid_packages_are_verified_through_cli ... ok
test missing_linked_fifo_and_oversized_inputs_and_unsupported_target_refuse ... ok
test concurrent_installers_and_interrupted_activation_preserve_a_complete_generation ... ok
test install_is_managed_and_atomic_and_refuses_unmanaged_collision ... ok
test adversary_modified_generation_inventory_is_refused_without_activation ... ok
test deterministic_packaging_and_interrupted_process_keep_previous_activation ... ok
test unsafe_archive_members_and_nonstatic_workers_are_refused ... ok
test bot_publication_refuses_remote_tag_mismatch_existing_and_unknown_release_and_reports_upload_failure ... ok
test generation_reuse_checks_all_entry_locations_and_types ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.99s

   Doc-tests mantle_artifact

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mantle_release

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

`cargo clippy --locked -p mantle-artifact -p mantle-release --all-targets -- -D warnings`, exit 0:

```text
    Checking mantle-release v0.1.4 ($HOME/.local/state/worktree/trees/b10x/mantle/mantle-prebuilt-release/crates/mantle-release)
    Finished `dev` profile [unoptimized] target(s) in 0.97s
```

`cargo fmt --check`, exit 0, no output (`correction-fmt.log`). `git diff --check` also exit 0.

`cargo run --locked -p mantle-release -- notices --check`, exit 0:

```text
    Finished `dev` profile [unoptimized] target(s) in 0.13s
     Running `/dev/shm/mantle-prebuilt-release-target/debug/mantle-release notices --check`
```

`cargo run --locked -p mantle-docs -- check`, exit 0:

```text
    Finished `dev` profile [unoptimized] target(s) in 0.17s
     Running `/dev/shm/mantle-prebuilt-release-target/debug/mantle-docs check`
Mantle documentation: anchors, routes and document valid
```

Unit check environment: PATH begins `$HOME/.cache/aap-ess-upgrade/ess-0.50/bin`; CARGO_TARGET_DIR=/dev/shm/mantle-prebuilt-release-target, TMPDIR=/dev/shm/mantle-prebuilt-release-tmp, CARGO_BUILD_JOBS=2, CARGO_PROFILE_DEV_DEBUG=0, RUSTC_WRAPPER=/usr/bin/sccache, SCCACHE_SERVER_PORT=43179, SCCACHE_DIR=/dev/shm/mantle-reliability-compiler-cache, SCCACHE_CACHE_SIZE=1G. The separate production build unsets RUSTC_WRAPPER and the release CLI assigns its fresh target under the unique work directory.

### Fresh production qualification

The real release CLI built from clean source commit 107f9c8bbdc9062cc24e54ac69e753d0f9e7fb68 into the new absolute work/output paths below. Exact command (with RUSTC_WRAPPER unset and the declared TMPDIR/jobs settings):

```sh
/dev/shm/mantle-prebuilt-release-target/debug/mantle-release build \
  --source "$HOME/.local/state/worktree/trees/b10x/mantle/mantle-prebuilt-release" \
  --revision 107f9c8bbdc9062cc24e54ac69e753d0f9e7fb68 \
  --work "$HOME/.cache/mantle-reliability/prebuilt-release/production-107f9c8bbdc9" \
  --output "$HOME/.cache/mantle-reliability/prebuilt-release/bundle-107f9c8bbdc9"
```

Exit 0. GNU finished in 7m 06s, static musl in 36.26s. Notices, all 12 archive payloads, three static worker ELF checks and actual version probes for all five executables passed. Provenance remains Rust 1.97.1, glibc 2.44, musl 1.2.5 and the exact Substrate pin. The GNU runtime qualification remains limited to this recorded host.

Verbatim build output:

```text
built and verified 0.1.4 from 107f9c8bbdc9062cc24e54ac69e753d0f9e7fb68 (glibc 2.44); source and compiler logs retained at $HOME/.cache/mantle-reliability/prebuilt-release/production-107f9c8bbdc9
```

Ran `sha256sum --check SHA256SUMS` in the new bundle, extracted only its verified `bin/mantle-release` with tar into the new bootstrap directory, and ran that executable's `verify` and `install` commands against the new manifest. Ran both installed --version commands, then the installed mantle-release's install command again. Every command exited 0. `cmp` of active-generation paths before and after the second install exited 0. Exact fresh installation prefix is `$HOME/.cache/mantle-reliability/prebuilt-release/install-107f9c8bbdc9`; no user PATH or existing installation changed.

Verbatim qualification outputs in execution order:

```text
mantle-0.1.4-x86_64-unknown-linux-gnu.tar: OK
mantle-0.1.4-x86_64-unknown-linux-musl.tar: OK
manifest.json: OK
verified 0.1.4 107f9c8bbdc9062cc24e54ac69e753d0f9e7fb68
installed $HOME/.cache/mantle-reliability/prebuilt-release/install-107f9c8bbdc9/.mantle/generations/19df84871f6f7cb45a4dc2e724971084b54e7a12929dde317b1190e5eb918b93
mantle 0.1.4
mantle-release 0.1.4
installed $HOME/.cache/mantle-reliability/prebuilt-release/install-107f9c8bbdc9/.mantle/generations/19df84871f6f7cb45a4dc2e724971084b54e7a12929dde317b1190e5eb918b93
generations/19df84871f6f7cb45a4dc2e724971084b54e7a12929dde317b1190e5eb918b93
```

GNU archive: 29,387,776 bytes, SHA256 `6e8fc4526b1fe9addb3fe3e05705aa8b04c0f86d7408ee1e59cd9afea67a2e77`.
Musl archive: 7,073,280 bytes, SHA256 `20bea8cb729822101a5e9c2ae9e584c872cc934467c4841197d23ee7ec8a4bb5`.
Manifest SHA256/generation: `19df84871f6f7cb45a4dc2e724971084b54e7a12929dde317b1190e5eb918b93`.
These candidates use the current workspace version 0.1.4 privately; they do not overwrite or publish the existing release.

## 5. Deliberate boundaries

No AEP mutation, publication, push, release/tag/PR creation, worker activity, authentication access, or shared/default cache-server operation occurred. No sibling source changed. No unchanged model/spec/native ESS test was rerun for this correction; those remain the prior handoff's evidence, with the full integrated gate owned by the coordinator. The next separate adversary pass remains required before integration.

The first adversary report and all its original logs were preserved. No original assertion was weakened. The coordinator read the previous handoff and chose to remove only the completed `production-6f4303dfda50/target` to recover capacity; its exact source/compiler logs, bundles, bootstrap and installation remain. This correction did not delete any worktree, target or evidence.

## 6. External paths and ownership handoff

New/updated files from this correction, all under `$HOME/.cache/mantle-reliability/prebuilt-release/`:

- `correction-1.md`
- `correction-red.log`
- `correction-package.log`
- `correction-clippy.log`
- `correction-fmt.log`
- `correction-notices.log`
- `correction-docs.log`
- `correction-commit.log`
- `production-build-107f9c8.log`
- `production-checksums-107f9c8.log`
- `production-bootstrap-verify-107f9c8.log`
- `production-install-first-107f9c8.log`
- `production-install-second-107f9c8.log`
- `installed-mantle-version-107f9c8.log`
- `installed-release-version-107f9c8.log`
- `installed-generation-first-107f9c8.txt`
- `installed-generation-second-107f9c8.txt`

Complete retained subtrees:

- `$HOME/.cache/mantle-reliability/prebuilt-release/production-107f9c8bbdc9/` — exact clean source export, fresh production target, fetch and both compiler logs.
- `$HOME/.cache/mantle-reliability/prebuilt-release/bundle-107f9c8bbdc9/` — manifest, checksums, two verified archives.
- `$HOME/.cache/mantle-reliability/prebuilt-release/bootstrap-107f9c8bbdc9/` — actual extracted installer.
- `$HOME/.cache/mantle-reliability/prebuilt-release/install-107f9c8bbdc9/` — verified private real installation with unchanged second-install generation.
- `/dev/shm/mantle-prebuilt-release-target/` — original unit target retained for second adversary.
- `/dev/shm/mantle-prebuilt-release-tmp/` — compiler/test process-lifetime fixtures and normal bounded probe temporary activity.
- `/dev/shm/mantle-reliability-compiler-cache/` — task-owned shared compiler cache, still bounded to configured 1 GiB; coordinator owns its server.
- `$HOME/.local/state/worktree/registry.sqlite3`, plus normal `-wal`/`-shm` coordination — lease management only.
- `$HOME/.cargo/` — normal Cargo dependency cache coordination during locked source builds; no downloaded source edit.

All own test/compiler/build/install commands have ended. Source tree remains clean at 107f9c8; codex-prebuilt-release lease is released on handoff. The coordinator owns second review, merge, full gate, publication and cleanup.
