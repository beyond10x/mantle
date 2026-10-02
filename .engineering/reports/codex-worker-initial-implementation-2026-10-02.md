unit:                   story:agent-ready-worker — Provision an agent-ready worker without Claude credentials
verdict:                green (offline implementation gate; live acceptance remains coordinator-owned)
cases:                  executed 30→48, red 4
origin:                 n/a
wrote-outside-worktree: ~/.cache/mantle-wave3/scratch-worker/; /dev/shm/mantle-wave3-target-worker/; Cargo-managed registry cache described below; own worktree lease metadata
needs-coordinator:      no source patches; independent review and real AW acceptance still required

## 1. Unit and acceptance

Implement AW-01–AW-05: a fresh no-Claude worker reaches common confinement/toolchain readiness with a verified pinned Codex installation, reruns preserve active Claude services, legacy Claude remains selectable, and providers share prebuilt bootstrap. Offline green does not assert fresh KubeVirt readiness, preservation of the real active Claude exec, authentication, model access or live EC2 behavior. The coordinator owns those live observations.

The helper performs one bounded HTTPS/archive/binary/version verification transaction, publishes immutable executable plus generated facts through a single atomic current pointer, retains prior generations and refuses unsupported architecture before fetch. Inspection checks executable bytes against the coherent manifest. Staging and lock are private; published root-owned directories are traversable for Substrate's read-only toolchain projection. No daemon/gateway restart belongs to the installer.

Common configuration no longer requires a Claude table. Selected Claude still checks its real credential, executable and declared slot before creating a workspace. Fresh cloud-init renders either slot-free or credential-dependent service configuration; daemon enable/start occurs after common binaries, gateway and optional token installation. Existing units remain intact. A missing slot requires explicit maintenance. Changed active shared binaries are individually deferred, their observed identities remain reported, and independent helper/Codex installation continues.

MANTLE_CONFIG and MANTLE_STATE_DIR select absolute isolated paths without mutating HOME or global environment. The coordinator approved this scoped addition for actual safe CLI acceptance. The helper, CLI and process transport use Rust; commands use clap derive. Bounded process transport pumps nonblocking stdin/stdout/stderr together, retains PID ownership via waitid(WNOWAIT), and cleans its exact process group before reaping.

### Inferred scope confirmation

| Placement | Inspection and result |
| --- | --- |
| crates/mantle-worker/{Cargo.toml,src/lib.rs,src/main.rs} | New crate absent at base; confirmed as one library/CLI installer and bounded transport, no duplicate cloud-init installer. |
| generated/worker-model/{Cargo.toml,types.rs,source.schema.json,types-report.json} | New generated value-type crate; ESS generated all four, parent workspace excludes standalone crate, Rust path dependency imports it, drift verifier compares exact bytes. |
| generated/worker-model/.ess-output/state.json | Initial scope assumption was wrong: ESS operational metadata contains output path/inode/random anchor. Coordinator rescope removed it from source; untouched local metadata is ignored via assigned .gitignore change. |
| spec/scenarios/agent-worker.yaml | New authored sequence explicitly selected from ess-inputs.yaml by conformance synthesis. |
| spec/README.md | Existing coverage document confirmed; updated actual decision coverage and live boundaries. |
| existing conformance.rs and owned Rust modules | Existing test ownership confirmed; no extra test path added. |
| Taskfile.yml | Existing static build task required mantle-worker addition, as final story refinement anticipated. |

ESS command contracts precede their implementations. The actual adapter calls production decisions over explicit observations; it does not evaluate ESS guards or synthesize remote facts. All 65 local scenarios execute (57 prior, seven generated worker decisions and one authored sequence). The same two prior ESS-SYNTH-013 refusals remain visible. There are no fictitious WorkerRecord fields.

## 2. Observed change

Tracked diff stat (new source files listed immediately after):

```text
 .gitignore                                |   1 +
 Cargo.lock                                | 113 +++++++++++++++-
 Cargo.toml                                |   3 +-
 Taskfile.yml                              |   2 +-
 crates/mantle/Cargo.toml                  |   1 +
 crates/mantle/src/adapters/conformance.rs |  74 ++++++++++-
 crates/mantle/src/adapters/ssh.rs         |  20 +++
 crates/mantle/src/adapters/substrate.rs   |  64 ++++++++-
 crates/mantle/src/app/session.rs          |  11 +-
 crates/mantle/src/app/worker.rs           | 207 ++++++++++++++++++++++++++----
 crates/mantle/src/config.rs               |  54 +++++++-
 crates/mantle/src/main.rs                 |   6 +-
 deploy/cloud-init.yaml                    |   4 +-
 deploy/substrate.service                  |   4 +-
 examples/config.toml                      |   4 +
 spec/README.md                            |  30 ++++-
 spec/components.yaml                      |   3 +
 spec/domains/session.yaml                 |  51 +++++++-
 spec/ess-inputs.yaml                      |   3 +-
 19 files changed, 606 insertions(+), 49 deletions(-)
crates/mantle-worker/Cargo.toml
crates/mantle-worker/src/lib.rs
crates/mantle-worker/src/main.rs
generated/worker-model/Cargo.toml
generated/worker-model/source.schema.json
generated/worker-model/types-report.json
generated/worker-model/types.rs
spec/scenarios/agent-worker.yaml
```

## 3. Red verifiers

The no-Claude test was added before changing Config and failed on the actual required `claude` field. Its original source is red-config-test.rs; final source retains the same assertion, formatted by rustfmt. Baseline source tests first ran green (20 mantle + 10 qualification example). A new installer test first failed against the unimplemented transaction (new lane; no package existed at base), then grew into real bounded filesystem/process fixture coverage. Later implementation review exposed two more real regressions: optional service-line removal broke ExecStart continuation, and a private-file link-count restriction incorrectly rejected Cargo's hardlinked delivery artifacts. Both gained red tests before correction.

Commands (all test commands used the assigned target/TMPDIR, RUSTC_WRAPPER empty, jobs2, dev/test debug0, incremental0):

- `cargo test --locked -p mantle common_worker_configuration_needs_no_claude_credentials`: exit101.
- `cargo test -p mantle-worker`: exit101, initial one-test installer stub.
- `cargo test --locked -p mantle both_profiles_keep_aperture_and_ca_in_the_daemon_command`: exit101.
- `cargo test --locked -p mantle-worker delivery_accepts_cargo_hardlinked_artifacts`: exit101.

Verbatim outputs, with personal path prefixes normalized to `~`:

```text
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 4.25s
     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle-00ae0b73f848d709)

running 1 test
test config::tests::common_worker_configuration_needs_no_claude_credentials ... FAILED

failures:

---- config::tests::common_worker_configuration_needs_no_claude_credentials stdout ----

thread 'config::tests::common_worker_configuration_needs_no_claude_credentials' (3280550) panicked at crates/mantle/src/config.rs:195:217:
credential-independent configuration: Error { message: "missing field `claude`", input: Some("provider = 'kubevirt'\nubuntu_serial = '20260926'\n[kubevirt]\ncontext = 'test'\nnamespace = 'mantle'\ncpu = 4\nmemory_gib = 8\nroot_disk_gib = 20\ndata_disk_gib = 20\n"), keys: [], span: Some(0..0) }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    config::tests::common_worker_configuration_needs_no_claude_credentials

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 20 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p mantle --bin mantle`
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 9 packages to latest Rust 1.97 compatible versions
      Adding mantle-worker-model v0.0.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/generated/worker-model)
      Adding nix v0.30.1 (available: v0.31.3)
      Adding ureq v3.4.2
      Adding ureq-proto v0.6.4
      Adding utf8-zero v0.8.1
      Adding webpki-roots v1.0.9
      Adding zstd v0.13.3 (available: v0.14.0)
      Adding zstd-safe v7.3.0
      Adding zstd-sys v2.1.0+zstd.1.5.7
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
 Downloading crates ...
  Downloaded zstd-safe v7.3.0
  Downloaded zstd-sys v2.1.0+zstd.1.5.7
   Compiling zstd-sys v2.1.0+zstd.1.5.7
   Compiling syn v3.0.6
   Compiling serde_core v1.0.229
   Compiling typenum v1.20.1
   Compiling generic-array v0.14.7
   Compiling httparse v1.10.1
   Compiling rustls v0.23.45
   Compiling rustix v1.1.5
   Compiling zstd-safe v7.3.0
   Compiling getrandom v0.4.3
   Compiling once_cell v1.21.4
   Compiling serde_json v1.0.151
   Compiling http v1.5.0
   Compiling nix v0.30.1
   Compiling rustls-webpki v0.103.15
   Compiling serde_derive v1.0.229
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.7
   Compiling subtle v2.6.1
   Compiling base64 v0.23.1
   Compiling ureq-proto v0.6.4
   Compiling clap_derive v4.6.7
   Compiling digest v0.10.7
   Compiling serde v1.0.229
   Compiling webpki-roots v1.0.9
   Compiling utf8-zero v0.8.1
   Compiling sha2 v0.10.9
   Compiling mantle-worker-model v0.0.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/generated/worker-model)
   Compiling ureq v3.4.2
   Compiling tempfile v3.27.0
   Compiling clap v4.6.7
   Compiling zstd v0.13.3
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
warning: unused import: `Path`
 --> crates/mantle-worker/src/lib.rs:2:17
  |
2 | use std::path::{Path, PathBuf};
  |                 ^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `mantle-worker` (lib) generated 1 warning (run `cargo fix --lib -p mantle-worker` to apply 1 suggestion)
warning: `mantle-worker` (lib test) generated 1 warning (1 duplicate)
    Finished `test` profile [unoptimized] target(s) in 16.32s
     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-e379049379d396fa)

running 1 test
test tests::installation_is_verified_atomic_and_idempotent ... FAILED

failures:

---- tests::installation_is_verified_atomic_and_idempotent stdout ----

thread 'tests::installation_is_verified_atomic_and_idempotent' (3288510) panicked at crates/mantle-worker/src/lib.rs:21:40:
called `Result::unwrap()` on an `Err` value: not implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::installation_is_verified_atomic_and_idempotent

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p mantle-worker --lib`
   Compiling mantle-worker-model v0.0.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/generated/worker-model)
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 7.26s
     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle-ab69b5a819f4e749)

running 1 test
test app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command ... FAILED

failures:

---- app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command stdout ----

thread 'app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command' (3651057) panicked at crates/mantle/src/app/worker.rs:642:13:
daemon lost aperture after optional slot line
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 25 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p mantle --bin mantle`
   Compiling mantle-worker-model v0.0.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/generated/worker-model)
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
    Finished `test` profile [unoptimized] target(s) in 1.25s
     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-e379049379d396fa)

running 1 test
test tests::delivery_accepts_cargo_hardlinked_artifacts ... FAILED

failures:

---- tests::delivery_accepts_cargo_hardlinked_artifacts stdout ----

thread 'tests::delivery_accepts_cargo_hardlinked_artifacts' (3825594) panicked at crates/mantle-worker/src/lib.rs:720:52:
called `Result::unwrap()` on an `Err` value: installation input must be a regular, singly-linked file
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::delivery_accepts_cargo_hardlinked_artifacts

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p mantle-worker --lib`
```

The first helper suite also exposed two invalid new fixture assumptions: GNU true --version prints text, and ESS operational output state is not byte-deterministic across directories. The fixture now uses actual version output; coordinator-approved scope correction excludes operational state while preserving all four generated artifact byte comparisons. Original failing output remains helper-first.log. packages-first.log is a compile failure (E0425 during an edit after a dependency compiled), not test-red evidence; packages-second and later runs corrected it without weakening cases.

## 4. Green gate and produced artifacts

Final per-lane counts read from actual runner summaries:

- Mantle binary tests: executed20→26, exit0.
- Qualification example: executed10→10, exit0; this wave adds no qualification cases. Two nested child-run summaries are not counted again.
- New mantle-worker library: no package/lane on base; first stub run executed1 failed, final run executed12 passed, exit0.
- New mantle-worker CLI harness: executed0, exit0 (no tests claimed there).
- Real local ESS scenarios:57→65, failed0, skipped0, exact prior refusal set retained.

Final commands and exits:

- `cargo test --locked -p mantle -p mantle-worker --all-targets`:0 (packages-final2.log).
- `cargo clippy --locked -p mantle -p mantle-worker --all-targets -- -D warnings`:0 (clippy-final2.log).
- `cargo fmt -p mantle -p mantle-worker --check`:0 (fmt-final2.log).
- `ess specify validate --path spec`:0 (ess-final.log).
- `systemd-analyze verify <no-Claude rendered unit>`:0 (service-verify.log; only absent local daemon executable substituted with /usr/bin/true, parser exercised, no service started).
- `cargo build --locked --release --target x86_64-unknown-linux-musl -p mantle-worker -p mantle-egress -p mantle-launch`:0 (build-static-final.log).
- `cargo build --locked -p mantle`:0 (build-cli-final.log).
- Static mantle-worker `--help`:0 (helper-help.log, no privileged operation).
- `git diff --check`:0.

All three worker artifacts are x86-64 static PIE ELF, stripped. No Substrate compilation occurred. Final local conformance model digest: e4ed1d26c2e578d0af56d2aa8813a50292167069d9a6ba58b0bac6e649148d8a.

Verbatim final gate outputs:

```text
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 8.23s
     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle-ab69b5a819f4e749)

running 26 tests
test app::terminal::tests::the_sequence_is_recognised_across_reads ... ok
test app::terminal::tests::ctrl_bracket_d_detaches_and_forwards_what_came_before ... ok
test config::tests::explicit_path_selection_is_absolute_and_preserves_defaults ... ok
test domain::manifest::tests::defaults_apply ... ok
test app::worker::tests::legacy_claude_bootstrap_retains_required_slot ... ok
test adapters::substrate::worker_tests::common_readiness_and_selected_claude_are_distinct_observations ... ok
test adapters::state::tests::sources_round_trip ... ok
test app::worker::tests::the_template_renders_every_placeholder ... ok
test domain::manifest::tests::cwd_with_dot_dot_or_empty_components_is_refused ... ok
test app::terminal::tests::escape_followed_by_anything_else_passes_both_bytes ... ok
test domain::manifest::tests::cwd_must_be_inside_a_mount ... ok
test config::tests::common_worker_configuration_needs_no_claude_credentials ... ok
test domain::manifest::tests::mounts_and_refs_cannot_escape ... ok
test adapters::state::tests::an_illegal_move_is_refused_and_not_written ... ok
test adapters::state::tests::a_live_name_is_unique_until_stopped ... ok
test domain::manifest::tests::retain_for_is_bounded_by_the_exec_limit ... ok
test config::tests::legacy_claude_table_remains_parseable ... ok
test domain::session::tests::stopped_is_final_and_failures_only_stop ... ok
test domain::session::tests::names_round_trip ... ok
test domain::session::tests::the_happy_path_is_legal ... ok
test domain::manifest::tests::sizes_and_durations_parse ... ok
test domain::manifest::tests::the_example_manifest_resolves ... ok
test app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command ... ok
test domain::session::tests::running_is_never_reached_without_starting ... ok
test domain::manifest::tests::unserved_fields_are_refused_by_name ... ok
test adapters::state::conformance::ess_generated_local_conformance ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s

     Running unittests examples/codex_qualification.rs (/dev/shm/mantle-wave3-target-worker/debug/examples/codex_qualification-acd4056f7e1b59e7)

running 10 tests
test tests::digest_bounds_actual_input_even_when_it_exceeds_the_size_snapshot ... ok
test tests::digest_refuses_other_non_regular_descriptors ... ok
test tests::fixed_observation_never_exports_terminal_payload ... ok
test tests::scratch_budget_refuses_large_files_and_does_not_follow_links ... ok
test tests::unsafe_scratch_parents_are_refused_without_creating_children ... ok

running 1 test

running 1 test
test tests::digest_format_is_strict ... ok
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.01s

test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok
test tests::terminal_observation_is_bounded_and_handles_split_queries ... ok
test tests::cq01_rejects_wrong_digest_before_executing_a_binary ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.45s

     Running unittests src/lib.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-c4999a0e43038d4b)

running 12 tests
test tests::byte_limit_detects_growth_beyond_declared_size ... ok
test tests::delivery_accepts_cargo_hardlinked_artifacts ... ok
test tests::corrupt_archive_never_activates ... ok
test tests::generated_model_has_no_drift ... ok
test tests::unsupported_architecture_never_fetches_or_creates_paths ... ok
test tests::symlinked_root_is_refused ... ok
test tests::installation_is_verified_atomic_and_idempotent ... ok
test tests::tampered_installed_bytes_are_refused ... ok
test tests::version_mismatch_never_activates ... ok
test tests::failed_update_preserves_previous_binary_and_facts ... ok
test tests::bounded_child_pumps_input_and_output_together ... ok
test tests::bounded_child_refuses_output_flood_and_timeout ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s

     Running unittests src/main.rs (/dev/shm/mantle-wave3-target-worker/debug/deps/mantle_worker-d9b2d0846547dc26)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Checking mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
    Checking mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `dev` profile [unoptimized] target(s) in 3.87s
mantle v1 — 6 file(s), 1 scenario(s), valid
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle-worker)
    Finished `release` profile [optimized] target(s) in 2.27s
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `dev` profile [unoptimized] target(s) in 5.68s
/dev/shm/mantle-wave3-target-worker/x86_64-unknown-linux-musl/release/mantle-worker: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), static-pie linked, BuildID[sha1]=56d03a03b8449eb997ed4df7cfcfdc3cc59626b7, stripped
/dev/shm/mantle-wave3-target-worker/x86_64-unknown-linux-musl/release/mantle-egress: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), static-pie linked, BuildID[sha1]=4f8fb8ea43be4299d0c5c7b085d60484d12a95c5, stripped
/dev/shm/mantle-wave3-target-worker/x86_64-unknown-linux-musl/release/mantle-launch: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), static-pie linked, BuildID[sha1]=c3d859bf0008c4fc331001e8379ae0bf920aca96, stripped
716f24ec0daaaa5b5e687aeb91fa56fcaca730046653a9e981bbdf4435fb2284  /dev/shm/mantle-wave3-target-worker/x86_64-unknown-linux-musl/release/mantle-worker
ac8f3fcbd9ea7b14422aecb14c351b998950530e37529b9fe3a84e579228aad3  /dev/shm/mantle-wave3-target-worker/x86_64-unknown-linux-musl/release/mantle-egress
593823a5198b7f7180edc32392948ff703367460dbbc20c2e2c60d29cf9fbc45  /dev/shm/mantle-wave3-target-worker/x86_64-unknown-linux-musl/release/mantle-launch
aaf78fc7b929279773440a5518a7b341371f6ba60dc94e99f4e0b1d216624816  /dev/shm/mantle-wave3-target-worker/debug/mantle
```

## 5. Deliberate boundaries

No AEP writes, commits, staging, pushes, tags, live worker mutations, credential reads or service restarts. No source path beyond coordinator-authorized scope changed. Full workspace integration gate and independent adversary remain coordinator-owned. Authentication and lifecycle parity are later stories. Actual fresh no-Claude KubeVirt and rerun beside the active Claude exec remain necessary; local decision/transaction tests do not claim them. Live EC2 is not claimed.

Old installed Codex generations are retained. Corrupt installed generations refuse inspection/install rather than silently repairing or claiming readiness. A failure after the atomic activation fsync can require inspection to determine the current observed generation; preactivation failures preserve the prior pointer. Trusted root-owned filesystem ancestors are assumed not maliciously replaced by root during an operation.

## 6. Writes outside the worktree

All task-owned evidence is in `~/.cache/mantle-wave3/scratch-worker/`; exact file inventory follows in scratch-inventory.txt. It includes raw private logs, normalized report, original red test specimen, temporary specification drafts, parser fixture, hashes and source inventory. The coordinator's existing brief.md/story.txt are also listed for a complete directory inventory but were not modified by this agent.

All disposable build/test output is under `/dev/shm/mantle-wave3-target-worker/`, including its private tmp directory. No target was removed. Cargo's standard registry cache automatically downloaded zstd-safe7.3.0 and zstd-sys2.1.0+zstd.1.5.7 and updated registry index metadata; these are shared dependency-cache entries, not cleanup candidates. Their exact cache/source paths are in dependency-cache-paths.txt. Worktree CLI acquired/refreshed this agent's own `codex-wave3-worker-implementor` lease; the coordinator owns lifecycle cleanup.

The managed tree remains `mantle-wave3-worker`, branch `impl/agent-ready-worker`, base7f187c9f7d14cf8962c45171718af695f7b69f5f, with uncommitted reviewed source ready for the adversary. Final handoff releases only this agent's lease.

### Final help correction

A final CLI smoke check exposed a stale --binaries description naming only two artifacts. The owned main.rs help text now lists mantle-worker as the third required binary. This changes help text only. `cargo build --locked -p mantle`, `cargo fmt -p mantle -p mantle-worker --check`, and `mantle worker up --help` each exit0 afterward. The final CLI hash below supersedes the earlier CLI hash; worker hashes and behavioral gate evidence are unchanged.

```text
c45203e855b5871f3426575d1a2a9213ffc47b8b6b0021601a9191d419863686  /dev/shm/mantle-wave3-target-worker/debug/mantle
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave3-worker/crates/mantle)
    Finished `dev` profile [unoptimized] target(s) in 7.11s
Create or start the worker, install Mantle's binaries, and check Substrate's facts

Usage: mantle worker up [OPTIONS]

Options:
      --binaries <BINARIES>  Directory holding static `mantle-egress`, `mantle-launch`, and `mantle-worker` binaries [default: ~/.cache/b10x-target/mantle/x86_64-unknown-linux-musl/release]
      --no-idle-stop         Do not stop the worker after two idle hours
  -h, --help                 Print help
```
