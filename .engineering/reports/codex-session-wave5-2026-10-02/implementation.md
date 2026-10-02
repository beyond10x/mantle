unit:                   story:codex-session-wiring — Codex session selection and private runtime
verdict:                green
cases:                  executed 135→137 Rust; native 258→265; red 2
origin:                 n/a
wrote-outside-worktree: ~/.cache/mantle-wave5/scratch-codex-session; /dev/shm/mantle-wave5-target-codex-session; /dev/shm/mantle-wave5-runtime-codex-session; managed worktree lease metadata
needs-coordinator:      no

1. Unit and acceptance

Implemented CW01–CW09 in the assigned worktree and branch impl/codex-session-wiring, opening79e1ee6bc7a9c0f311664d31057087096d354fdf. Codex manifests resolve fixed generated identities, actual legacy SQLite migrates/reopens without changing old payloads, and real start/attach request builders preserve the explicit non-recording requirement. Production refuses that requirement before credentials/provider/session insertion because the pinned SDK cannot bind it. Generic launcher path primitives enforce private owner/mode, component traversal, metadata-only file checks and observed tmpfs before readiness/child dispatch. Fixed strict Codex configuration and isolated pinned-binary runtime evidence complete this preparatory unit.

Inferred scope paths were verified: the new example resolves through the actual parser; the new public CLI fixture reaches the actual admission error without credentials and also exercises stored-identity attachment after changing the manifest; both new CLI authored scenarios execute on the existing native adapter. The new private-path scenario exercises actual launcher subprocesses. No new architecture, guessed SDK API, AEP write, stage, commit, VM or authenticated integration was introduced.

The binary Rust bridge count remains29 because its existing conformance test runs the expanded native inventory: CLI213→218. Launcher Rust conformance harness5→6 and native45→47. Worker28→28 includes generated provenance drift. The Rust package baselines below are inherited runner output from .engineering/reports/codex-launcher-gate-2026-10-02/tests-private-tmp.log (workspace --locked), not a claim to have rerun the opening revision with identical package arguments. The new test-first red lanes have direct same-command red/green evidence.

2. Actual diff shape (tracked files; new files additionally listed in status.txt)
 AGENTS.md                                          |   4 +
 README.md                                          |   8 +
 crates/mantle-launch/src/cli.rs                    |  12 ++
 crates/mantle-launch/src/serve.rs                  |   9 +
 crates/mantle-launch/src/session.rs                | 128 ++++++++++++
 crates/mantle-launch/tests/conformance.rs          | 144 ++++++++++++++
 crates/mantle-launch/tests/support/probe.rs        |   5 +
 crates/mantle-worker/src/lib.rs                    |   5 +
 crates/mantle/examples/codex_qualification.rs      | 221 +++++++++++++++++++--
 .../tests/codex_qualification_adversary.rs         |  15 +-
 crates/mantle/src/adapters/conformance.rs          | 118 ++++++++++-
 crates/mantle/src/adapters/orchestration.rs        |  44 +++-
 crates/mantle/src/adapters/state.rs                |  66 +++++-
 crates/mantle/src/adapters/substrate.rs            |  11 +
 crates/mantle/src/app/session.rs                   | 171 ++++++++++++++--
 crates/mantle/src/domain/manifest.rs               |  15 +-
 crates/mantle/src/domain/session.rs                |  33 +++
 docs/evidence/codex-compatibility.md               |  38 ++++
 generated/worker-model/source.schema.json          |  24 ++-
 generated/worker-model/types-report.json           |  44 +++-
 generated/worker-model/types.rs                    |  22 +-
 spec/README.md                                     |  23 ++-
 spec/components.yaml                               |   4 +-
 spec/conformance-baseline.json                     |  17 +-
 spec/domains/launch.yaml                           |  14 ++
 spec/domains/manifest.yaml                         |   3 +
 spec/domains/orchestration.yaml                    |  17 ++
 spec/domains/session.yaml                          |  32 ++-
 spec/ess-inputs.yaml                               |   3 +
 spec/scenarios/cli/live-name-failedagentstart.yaml |   8 +-
 .../cli/live-name-failedmaterialization.yaml       |   8 +-
 spec/scenarios/cli/live-name-materializing.yaml    |   8 +-
 spec/scenarios/cli/live-name-running.yaml          |   8 +-
 spec/scenarios/cli/live-name-starting.yaml         |   8 +-
 spec/scenarios/cli/live-name-stopped.yaml          |   8 +-
 spec/scenarios/cli/live-name-stopping.yaml         |   8 +-
 ...anifest-capabilities-are-subset-not-policy.yaml |   4 +-
 spec/scenarios/cli/manifest-defaults.yaml          |   4 +-
 .../cli/manifest-lower-resource-bounds.yaml        |   4 +-
 spec/scenarios/cli/manifest-nested-cwd.yaml        |   4 +-
 .../cli/manifest-original-bytes-hashed.yaml        |   4 +-
 .../cli/manifest-retention-upper-bound.yaml        |   4 +-
 .../scenarios/cli/manifest-workspace-root-cwd.yaml |   4 +-
 spec/scenarios/cli/manifest-zero-cpu-clamped.yaml  |   4 +-
 .../cli/source-composite-insert-isolation.yaml     |   8 +-
 spec/scenarios/cli/source-rows-survive-stop.yaml   |   4 +-
 .../launch/argument-byte-preservation.yaml         |   5 +-
 spec/scenarios/launch/argument-defaults.yaml       |   5 +-
 spec/scenarios/launch/replay-policy-arguments.yaml |   3 +
 49 files changed, 1256 insertions(+), 107 deletions(-)

3. Red runs, before production edits

All Cargo commands used the assigned private TMPDIR/MANTLE_TEST_SCRATCH, assigned CARGO_TARGET_DIR, jobs2, incremental0 and dev/test debug0 unless explicitly stated otherwise.

cargo test --locked -p mantle --test codex_preflight
exit101; executed1; unknown auth field refused before new selected-agent behavior. Full output follows:
   Compiling proc-macro2 v1.0.107
   Compiling unicode-ident v1.0.26
   Compiling quote v1.0.47
   Compiling cfg-if v1.0.5
   Compiling libc v0.2.189
   Compiling syn v3.0.6
   Compiling syn v2.0.119
   Compiling itoa v1.0.18
   Compiling bytes v1.12.1
   Compiling log v0.4.34
   Compiling smallvec v1.16.2
   Compiling find-msvc-tools v0.1.14
   Compiling shlex v2.0.1
   Compiling errno v0.3.14
   Compiling signal-hook-registry v1.4.8
   Compiling parking_lot_core v0.9.12
   Compiling jobserver v0.1.35
   Compiling pin-project-lite v0.2.17
   Compiling scopeguard v1.2.0
   Compiling lock_api v0.4.14
   Compiling cc v1.5.1
   Compiling mio v1.2.3
   Compiling parking_lot v0.12.5
   Compiling tokio-macros v2.7.2
   Compiling socket2 v0.6.5
   Compiling futures-core v0.3.34
   Compiling once_cell v1.21.4
   Compiling tokio v1.53.1
   Compiling zeroize v1.9.0
   Compiling http v1.5.0
   Compiling futures-sink v0.3.34
   Compiling autocfg v1.5.1
   Compiling typenum v1.20.1
   Compiling fnv v1.0.7
   Compiling slab v0.4.12
   Compiling pkg-config v0.3.34
   Compiling ryu v1.0.23
   Compiling num-traits v0.2.19
   Compiling tracing-core v0.1.36
   Compiling tracing-attributes v0.1.31
   Compiling futures-macro v0.3.34
   Compiling futures-task v0.3.34
   Compiling futures-util v0.3.34
   Compiling tokio-util v0.7.19
   Compiling tracing v0.1.44
   Compiling http-body v1.1.0
   Compiling http v0.2.12
   Compiling ring v0.17.14
   Compiling equivalent v1.0.2
   Compiling hashbrown v0.17.1
   Compiling http-body v0.4.6
   Compiling indexmap v2.14.2
   Compiling num-integer v0.1.47
   Compiling hybrid-array v0.4.15
   Compiling http-body-util v0.1.5
   Compiling outref v0.5.2
   Compiling vsimd v0.8.0
   Compiling either v1.18.0
   Compiling deranged v0.5.8
   Compiling time-core v0.1.9
   Compiling powerfmt v0.2.0
   Compiling cmov v0.5.4
   Compiling cpufeatures v0.3.1
   Compiling num-conv v0.2.2
   Compiling ctutils v0.4.2
   Compiling time v0.3.55
   Compiling bytes-utils v0.1.4
   Compiling base64-simd v0.8.0
   Compiling block-buffer v0.12.1
   Compiling crypto-common v0.2.2
   Compiling cmake v0.1.58
   Compiling getrandom v0.2.17
   Compiling fs_extra v1.3.0
   Compiling const-oid v0.10.2
   Compiling dunce v1.0.5
   Compiling httparse v1.10.1
   Compiling untrusted v0.9.0
   Compiling pin-utils v0.1.1
   Compiling aws-smithy-types v1.8.1
   Compiling aws-lc-sys v0.45.0
   Compiling digest v0.11.3
   Compiling aws-smithy-async v1.3.0
   Compiling aws-smithy-runtime-api-macros v1.1.0
   Compiling serde_core v1.0.229
   Compiling aws-smithy-runtime-api v1.19.0
   Compiling rustls-pki-types v1.15.1
   Compiling aws-lc-rs v1.18.1
   Compiling percent-encoding v2.3.2
   Compiling subtle v2.6.1
   Compiling synstructure v0.14.0
   Compiling base64 v0.23.1
   Compiling semver v1.0.28
   Compiling serde v1.0.229
   Compiling rustls v0.23.45
   Compiling try-lock v0.2.5
   Compiling want v0.3.1
   Compiling rustc_version v0.4.1
   Compiling sha2 v0.11.0
   Compiling futures-channel v0.3.34
   Compiling serde_derive v1.0.229
   Compiling httpdate v1.0.3
   Compiling tower-service v0.3.3
   Compiling zerofrom-derive v0.1.8
   Compiling atomic-waker v1.1.2
   Compiling h2 v0.4.19
   Compiling zerofrom v0.1.8
   Compiling yoke-derive v0.8.4
   Compiling aws-smithy-schema v0.2.1
   Compiling rustls v0.21.12
   Compiling zmij v1.0.23
   Compiling stable_deref_trait v1.2.1
   Compiling yoke v0.8.3
   Compiling hyper v1.11.1
   Compiling rustls-webpki v0.101.7
   Compiling sct v0.7.1
   Compiling serde_json v1.0.151
   Compiling memchr v2.8.3
   Compiling ipnet v2.12.2
   Compiling hyper-util v0.1.21
   Compiling form_urlencoded v1.2.2
   Compiling h2 v0.3.27
   Compiling zerovec-derive v0.11.6
   Compiling socket2 v0.5.10
   Compiling thiserror v2.0.21
   Compiling openssl-probe v0.2.1
   Compiling bitflags v2.13.2
   Compiling fastrand v2.5.0
   Compiling rustls-native-certs v0.8.4
   Compiling hyper v0.14.32
   Compiling zerovec v0.11.8
   Compiling tokio-rustls v0.24.1
   Compiling displaydoc v0.2.7
   Compiling thiserror-impl v2.0.21
   Compiling hex v0.4.3
   Compiling tower-layer v0.3.3
   Compiling tower v0.5.3
   Compiling hyper-rustls v0.24.2
   Compiling aws-types v1.6.0
   Compiling aws-smithy-http v0.64.1
   Compiling aws-credential-types v1.3.0
   Compiling rand_core v0.10.1
   Compiling version_check v0.9.5
   Compiling rustversion v1.0.23
   Compiling getrandom v0.4.3
   Compiling zerocopy v0.8.59
   Compiling generic-array v0.14.7
   Compiling aws-smithy-observability v0.3.0
   Compiling hmac v0.13.0
   Compiling zerocopy-derive v0.8.59
   Compiling aws-sigv4 v1.6.0
   Compiling tinystr v0.8.4
   Compiling uuid v1.26.1
   Compiling litemap v0.8.3
   Compiling cfg_aliases v0.2.2
   Compiling writeable v0.6.4
   Compiling icu_locale_core v2.3.0
   Compiling arc-swap v1.9.2
   Compiling zerotrie v0.2.5
   Compiling potential_utf v0.1.6
   Compiling aws-smithy-json v0.63.1
   Compiling serde_derive_internals v0.29.1
   Compiling icu_properties_data v2.3.0
   Compiling unsafe-libyaml v0.2.11
   Compiling icu_normalizer_data v2.3.0
   Compiling regex-lite v0.1.9
   Compiling utf8parse v0.2.2
   Compiling utf8_iter v1.0.4
   Compiling schemars v0.8.22
   Compiling icu_collections v2.3.0
   Compiling anstyle-parse v1.0.0
   Compiling serde_yaml v0.9.34+deprecated
   Compiling schemars_derive v0.8.22
   Compiling icu_provider v2.3.1
   Compiling chacha20 v0.10.2
   Compiling zstd-sys v2.1.0+zstd.1.5.7
   Compiling colorchoice v1.0.5
   Compiling getrandom v0.3.4
   Compiling anstyle-query v1.1.5
   Compiling anstyle v1.0.14
   Compiling rustix v1.1.5
   Compiling is_terminal_polyfill v1.70.2
   Compiling dyn-clone v1.0.20
   Compiling xmlparser v0.13.6
   Compiling aws-smithy-xml v0.62.1
   Compiling anstream v1.0.0
   Compiling rand v0.10.3
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.7
   Compiling linux-raw-sys v0.12.1
   Compiling strsim v0.11.1
   Compiling crc32fast v1.5.2
   Compiling urlencoding v2.1.3
   Compiling libm v0.2.16
   Compiling clap_lex v1.1.1
   Compiling signal-hook v0.3.18
   Compiling anyhow v1.0.104
   Compiling heck v0.5.0
   Compiling clap_derive v4.6.7
   Compiling rustls-webpki v0.103.15
   Compiling clap_builder v4.6.7
   Compiling aws-smithy-query v0.62.1
   Compiling tokio-rustls v0.26.6
   Compiling hyper-rustls v0.27.10
   Compiling aws-smithy-http-client v1.5.0
   Compiling digest v0.10.7
   Compiling icu_normalizer v2.3.0
   Compiling icu_properties v2.3.0
   Compiling ess-primitives v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling curve25519-dalek v5.0.0
   Compiling bigdecimal v0.4.11
   Compiling unicode-segmentation v1.13.3
   Compiling adler2 v2.0.1
   Compiling cpufeatures v0.2.17
   Compiling pulldown-cmark v0.13.4
   Compiling simd-adler32 v0.3.10
   Compiling zstd-safe v7.3.0
   Compiling vcpkg v0.2.15
   Compiling libsqlite3-sys v0.35.0
   Compiling aws-smithy-runtime v1.16.0
   Compiling miniz_oxide v0.9.1
   Compiling convert_case v0.10.0
   Compiling ess-domain v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling aws-runtime v1.10.0
   Compiling rand_core v0.9.5
   Compiling idna_adapter v1.2.2
   Compiling clap v4.6.7
   Compiling nix v0.31.3
   Compiling nix v0.30.1
   Compiling half v2.7.1
   Compiling ppv-lite86 v0.2.21
   Compiling sha1 v0.11.0
   Compiling num-bigint v0.4.8
   Compiling curve25519-dalek-derive v0.1.1
   Compiling signature v3.0.0
   Compiling foldhash v0.1.5
   Compiling pulldown-cmark-escape v0.11.0
   Compiling iana-time-zone v0.1.65
   Compiling unicase v2.9.0
   Compiling data-encoding v2.11.1
   Compiling tungstenite v0.30.0
   Compiling chrono v0.4.45
   Compiling ess-compiler v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling hashbrown v0.15.5
   Compiling ed25519 v3.0.0
   Compiling rand_chacha v0.9.0
   Compiling minicbor v2.3.0
   Compiling flate2 v1.1.10
   Compiling idna v1.1.0
   Compiling derive_more-impl v2.1.1
   Compiling tempfile v3.27.0
   Compiling ulid v3.0.0
   Compiling ureq-proto v0.6.4
   Compiling webpki-roots v1.0.9
   Compiling litrs v1.0.0
   Compiling winnow v1.0.4
   Compiling utf8-zero v0.8.1
   Compiling ureq v3.4.2
   Compiling toml_parser v1.1.3+spec-1.1.0
   Compiling document-features v0.2.12
   Compiling b10x-substrate-wire v0.7.8 (https://github.com/beyond10x/substrate?rev=05695970b069f79e6678f2f02cbd78bbe5fa2a56#05695970)
   Compiling derive_more v2.1.1
   Compiling url v2.5.8
   Compiling aws-smithy-compression v0.2.0
   Compiling aws-smithy-cbor v0.62.2
   Compiling zstd v0.13.3
   Compiling rand v0.9.5
   Compiling ed25519-dalek v3.0.0
   Compiling ess-gen v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling hashlink v0.10.0
   Compiling tokio-tungstenite v0.30.0
   Compiling signal-hook-mio v0.2.5
   Compiling aws-sdk-sts v1.119.0
   Compiling aws-sdk-sso v1.114.0
   Compiling aws-sdk-ssooidc v1.116.0
   Compiling sha2 v0.10.9
   Compiling sha1 v0.10.7
   Compiling mantle-worker-model v0.0.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/generated/worker-model)
   Compiling serde_urlencoded v0.7.1
   Compiling rustls-pemfile v2.2.0
   Compiling serde_spanned v1.1.1
   Compiling toml_datetime v0.7.5+spec-1.1.0
   Compiling fallible-iterator v0.3.0
   Compiling toml_writer v1.1.2+spec-1.1.0
   Compiling fallible-streaming-iterator v0.1.9
   Compiling base64 v0.22.1
   Compiling winnow v0.7.15
   Compiling rusqlite v0.37.0
   Compiling b10x-substrate-sdk v0.7.8 (https://github.com/beyond10x/substrate?rev=05695970b069f79e6678f2f02cbd78bbe5fa2a56#05695970)
   Compiling toml v0.9.12+spec-1.1.0
   Compiling aws-config v1.12.0
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle-worker)
   Compiling ess-conformance v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling crossterm v0.29.0
   Compiling ulid v1.2.1
   Compiling aws-sdk-cloudwatch v1.134.0
   Compiling aws-sdk-iam v1.128.0
   Compiling aws-sdk-ec2 v1.267.0
   Compiling mantle-conformance v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle-conformance)
   Compiling mantle-egress v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle-egress)
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 4m 24s
     Running tests/codex_preflight.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/codex_preflight-f14a9f5dcb0fcf21)

running 1 test
test codex_refuses_capture_before_claude_credentials_or_session_insertion ... FAILED

failures:

---- codex_refuses_capture_before_claude_credentials_or_session_insertion stdout ----

thread 'codex_refuses_capture_before_claude_credentials_or_session_insertion' (481381) panicked at crates/mantle/tests/codex_preflight.rs:26:5:
Error: parsing the manifest

Caused by:
    agent: unknown field `auth`, expected `kind` or `cwd` at line 15 column 3

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    codex_refuses_capture_before_claude_credentials_or_session_insertion

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s

error: test failed, to rerun pass `-p mantle --test codex_preflight`

cargo test --locked -p mantle-launch --test conformance private_path_initialization_preserves_auth_and_refuses_unsafe_entries
exit101; executed1; old launcher did not initialize private paths. Full output follows:
   Compiling mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle-launch)
    Finished `test` profile [unoptimized] target(s) in 0.73s
     Running tests/conformance.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/conformance-6f9433fcff2e5c73)

running 1 test
test private_path_initialization_preserves_auth_and_refuses_unsafe_entries ... FAILED

failures:

---- private_path_initialization_preserves_auth_and_refuses_unsafe_entries stdout ----

thread 'private_path_initialization_preserves_auth_and_refuses_unsafe_entries' (513508) panicked at crates/mantle-launch/tests/conformance.rs:1158:5:
assertion `left == right` failed
  left: Bool(false)
 right: true
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    private_path_initialization_preserves_auth_and_refuses_unsafe_entries

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.12s

error: test failed, to rerun pass `-p mantle-launch --test conformance`

4. Green gates and counts

cargo test --locked -p mantle -p mantle-launch -p mantle-worker --all-targets -- --nocapture
exit0. Mantle executed39→40; launcher68→69; worker28→28. Native CLI213→218 and launcher45→47, zero failed/skipped/unsupported. Generated worker-model byte-for-byte drift check passed inside worker28. Public red lane1→1, exit0; private-path red lane1→1, exit0. The final command also includes the zero-test probe target; nested subprocess test output is not double-counted.

Full final runner output:
   Compiling mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle-launch)
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle)
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle-worker)
    Finished `test` profile [unoptimized] target(s) in 6.39s
     Running unittests src/main.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/mantle-277e3ad59dda8f7b)

running 29 tests
test app::terminal::tests::ctrl_bracket_d_detaches_and_forwards_what_came_before ... ok
test app::terminal::tests::escape_followed_by_anything_else_passes_both_bytes ... ok
test app::terminal::tests::the_sequence_is_recognised_across_reads ... ok
test adapters::substrate::worker_tests::common_readiness_and_selected_claude_are_distinct_observations ... ok
test config::tests::explicit_path_selection_is_absolute_and_preserves_defaults ... ok
test config::tests::common_worker_configuration_needs_no_claude_credentials ... ok
test config::tests::legacy_claude_table_remains_parseable ... ok
test domain::manifest::tests::sizes_and_durations_parse ... ok
test domain::session::tests::names_round_trip ... ok
test domain::manifest::tests::defaults_apply ... ok
test app::worker::tests::legacy_claude_bootstrap_retains_required_slot ... ok
test domain::session::tests::running_is_never_reached_without_starting ... ok
test domain::session::tests::stopped_is_final_and_failures_only_stop ... ok
test domain::manifest::tests::cwd_must_be_inside_a_mount ... ok
test app::worker::tests::the_template_renders_every_placeholder ... ok
test domain::manifest::tests::retain_for_is_bounded_by_the_exec_limit ... ok
test domain::manifest::tests::the_example_manifest_resolves ... ok
test domain::manifest::tests::unserved_fields_are_refused_by_name ... ok
test domain::manifest::tests::mounts_and_refs_cannot_escape ... ok
test adapters::state::tests::an_illegal_move_is_refused_and_not_written ... ok
test domain::session::tests::the_happy_path_is_legal ... ok
test adapters::state::tests::sources_round_trip ... ok
test domain::manifest::tests::cwd_with_dot_dot_or_empty_components_is_refused ... ok
test adapters::state::tests::a_live_name_is_unique_until_stopped ... ok
test app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command ... ok
test adapters::ssh::tests::tunnel_socket_binds_with_live_length_state_prefix ... ok
test adapters::ssh::tests::socket_allocations_are_private_unique_and_state_path_independent ... ok
test adapters::ssh::tests::tunnel_socket_directory_lives_until_child_cleanup_and_is_removed_on_errors ... ok
outside: mantle.egress.CheckBudgets/outcome/returned (OtherComponent)
outside: mantle.egress.CheckDefaultDestination/outcome/allowed (OtherComponent)
outside: mantle.egress.CheckDefaultDestination/outcome/denied (OtherComponent)
outside: mantle.egress.CheckDrain/outcome/returned (OtherComponent)
outside: mantle.egress.ClassifyAddress/outcome/returned (OtherComponent)
outside: mantle.egress.Exchange/outcome/returned (OtherComponent)
outside: mantle.launch.Emit/outcome/returned (OtherComponent)
outside: mantle.launch.FileSafety/outcome/returned (OtherComponent)
outside: mantle.launch.Lifecycle/outcome/returned (OtherComponent)
outside: mantle.launch.ParseArgs/outcome/returned (OtherComponent)
outside: mantle.launch.ParseWindow/outcome/returned (OtherComponent)
outside: mantle.launch.PersistentReplayCompatibility/outcome/returned (OtherComponent)
outside: mantle.launch.PrivatePaths/outcome/returned (OtherComponent)
outside: mantle.launch.ReadSecret/outcome/returned (OtherComponent)
outside: mantle.launch.ReplayRing/outcome/returned (OtherComponent)
outside: mantle.launch.Signals/outcome/returned (OtherComponent)
outside: mantle.launch.SlowReader/outcome/returned (OtherComponent)
outside: mantle.launch.ValidateScrollback/outcome/accepted (OtherComponent)
outside: mantle.launch.ValidateScrollback/outcome/refused (OtherComponent)
outside: mantle.launch.VolatileReplayBounds/outcome/returned (OtherComponent)
outside: mantle.launch.VolatileReplayExitPaths/outcome/returned (OtherComponent)
outside: mantle.launch.VolatileReplayLifecycle/outcome/returned (OtherComponent)
outside: mantle.launch.VolatileReplayPreflight/outcome/returned (OtherComponent)
218 selected scenario(s), 137 authored source(s), 0 refusal occurrence(s)

Workspace      workspace-1
Source         /workspace/r main@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Agent          exec-1 (running, ends after 8h at the latest)
Workspace      destroyed
Session        fixture stopped
worker      instance-1 (running)
no worker
worker      instance-1 stopped; data volume volume-1 retained
worker      instance-1 stopped; data volume volume-1 retained
worker      instance-1 stopped; data volume volume-1 retained
instance profile mantle-worker created; waiting for it to propagate
launched    instance-1 (image-1, group-1)
launched    instance-1 (image-1, group-1)
launched    instance-1 (image-1, group-1)
launched    instance-1 (image-1, group-1)
worker      instance-1 (pending)
worker      instance-1 (running)
worker      instance-1 (stopped)
starting    instance-1
worker      instance-1 (stopping)
starting    instance-1
worker      vm/mantle-default stopped; root and data volumes retained
no worker
launched    vm/mantle-default from https://example.com/fixture.img
worker      vm/mantle-default (Stopped)
worker      vm/mantle-default (Stopped)
starting    vm/mantle-default
launched    vm/mantle-default from https://example.com/fixture.img
Workspace      workspace-1
Source         /workspace/r main@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Agent          exec-1 (running, ends after 8h at the latest)
Workspace      workspace-1
Source         /workspace/r main@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Agent          exec-1 (running, ends after 8h at the latest)
Attach with    mantle attach s1
Workspace      workspace-1
Workspace      workspace-1
Source         /workspace/r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Agent          exec-1 (running, ends after 8h at the latest)
Attach with    mantle attach s1
Workspace      workspace-1
Source         /workspace/r main@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Workspace      workspace-1
Source         /workspace/r main@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Agent          exec-1 (running, ends after 8h at the latest)
Workspace      workspace-1
Workspace      workspace-1
Workspace      workspace-1
Agent          exec-1 already retired
Workspace      destroyed
Session        fixture stopped
Workspace      workspace-1 already destroyed
Session        fixture stopped
Workspace      destroyed
Session        fixture stopped
Session        fixture stopped
Workspace      destroyed
Session        fixture stopped
Workspace      destroyed
Session        fixture stopped
Workspace      destroyed (confirmed by read-back)
Session        fixture stopped
Workspace      destroyed (confirmed by read-back)
Session        fixture stopped
mantle-cli: 218 passed, 0 failed, 0 unsupported
test adapters::state::conformance::ess_generated_local_conformance ... ok

test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.46s

     Running tests/codex_preflight.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/codex_preflight-f14a9f5dcb0fcf21)

running 1 test
test codex_refuses_capture_before_claude_credentials_or_session_insertion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

     Running unittests examples/codex_qualification.rs (/dev/shm/mantle-wave5-target-codex-session/debug/examples/codex_qualification-2c3ae41e1f0be255)

running 10 tests
test tests::digest_bounds_actual_input_even_when_it_exceeds_the_size_snapshot ... ok
test tests::digest_refuses_other_non_regular_descriptors ... ok
test tests::digest_format_is_strict ... ok
test tests::fixed_observation_never_exports_terminal_payload ... ok
test tests::unsafe_scratch_parents_are_refused_without_creating_children ... ok
test tests::scratch_budget_refuses_large_files_and_does_not_follow_links ... ok

running 1 test

running 1 test
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

test tests::terminal_observation_is_bounded_and_handles_split_queries ... ok
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok
test tests::cq01_rejects_wrong_digest_before_executing_a_binary ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.31s

     Running unittests src/lib.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/mantle_launch-b94d68bc2f8cbdb8)

running 45 tests
test cli::tests::env_names_are_validated ... ok
test cli::tests::proxy_urls_carry_no_userinfo_or_path ... ok
test cli::tests::paths_must_be_absolute ... ok
test ctl::tests::line_round_trips ... ok
test cli::tests::full_command_line_parses ... ok
test ctl::tests::lines_split_across_reads_are_joined ... ok
test ctl::tests::non_utf8_is_ignored ... ok
test ctl::tests::over_long_lines_are_dropped_whole ... ok
test ctl::tests::parses_columns_then_rows ... ok
test ctl::tests::refuses_malformed_and_out_of_bounds_lines ... ok
test cli::tests::attach_parses_and_hides_no_tty ... ok
test ring::tests::a_push_larger_than_capacity_keeps_its_tail ... ok
test ring::tests::clear_empties ... ok
test cli::tests::secret_proxy_and_scrollback_are_optional ... ok
test ring::tests::drops_the_oldest_bytes_on_overflow ... ok
test ring::tests::front_and_consume_drain_in_order_across_wraparound ... ok
test ring::tests::keeps_everything_within_capacity ... ok
test cli::tests::program_is_required_after_separator ... ok
test ring::tests::push_reports_how_many_bytes_it_dropped ... ok
test ring::tests::push_ring_copies_oldest_first_and_respects_the_target_capacity ... ok
test cli::tests::dir_and_cwd_are_required ... ok
test cli::tests::removed_tmux_flags_are_refused ... ok
test secret::tests::unopened_descriptor_is_reported ... ok
test secret::tests::errors_do_not_echo_the_value ... ok
test serve::tests::a_new_terminal_does_not_inherit_an_owed_repaint ... ok
test serve::tests::a_nudge_holds_a_real_size_change_before_restoring ... ok
test serve::tests::a_nudge_under_way_is_not_restarted_and_can_be_cancelled ... ok
test secret::tests::empty_value_is_refused ... ok
test serve::tests::dropped_output_is_repainted_once_the_terminal_has_caught_up ... ok
test cli::tests::secret_fd_and_env_require_each_other ... ok
test serve::tests::a_one_row_window_is_nudged_up ... ok
test serve::tests::nothing_is_repainted_while_no_output_was_dropped ... ok
test serve::tests::poll_does_not_sleep_past_a_pending_restore ... ok
test secret::tests::trailing_newline_is_trimmed ... ok
test cli::tests::standard_descriptors_are_refused ... ok
test secret::tests::oversized_value_is_refused ... ok
test cli::tests::scrollback_is_bounded ... ok
test session::tests::a_symlinked_lock_is_refused_and_its_target_left_alone ... ok
test session::tests::a_held_lock_is_refused_and_freed_on_drop ... ok
test session::tests::a_lock_on_an_unlinked_file_does_not_count ... ok
test serve::tests::volatile_preflight_refuses_metadata_errors_other_than_absence ... ok
test session::tests::fifos_are_private ... ok
test session::tests::stale_fifos_are_removed_and_other_files_left_alone ... ok
test serve::tests::private_output_refuses_links_and_resets_the_mode ... ok
test session::tests::open_fifo_refuses_a_symlink_and_a_file_that_is_not_a_pipe ... ok

test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/mantle_launch-fac975cfcc56cc2e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests tests/support/probe.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/mantle_launch_probe-5a3189013b5332d3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/adversary.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/adversary-3fdd24c12f8c926d)

running 9 tests
test a_symlinked_session_dir_is_not_followed ... ok
test last_output_does_not_follow_a_planted_symlink ... ok
test session_dir_is_owner_only_even_when_it_already_exists ... ok
test the_secret_descriptor_does_not_reach_the_agent ... ok
test last_output_is_owner_only_even_when_the_file_already_exists ... ok
test exit_code_is_the_agents ... ok
test sigterm_to_serve_ends_the_agents_whole_process_group ... ok
test a_flooded_ctl_pipe_does_not_stall_the_agent ... ok
test a_client_that_never_reads_does_not_block_the_agent ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.11s

     Running tests/adversary_2.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/adversary_2-fd9198f6cf4c14e4)

running 3 tests
test a_size_aware_agent_repaints_on_a_real_resize ... ok
test attaching_at_an_unchanged_size_makes_the_agent_repaint ... ok
test a_terminal_that_lost_output_gets_a_repaint_once_it_catches_up ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.60s

     Running tests/codex_compatibility.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/codex_compatibility-cd1886455c8675a2)

running 2 tests

running 1 test
test local_socket_control_rejects_af_unix ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

test local_socket_control_rejects_af_unix ... ok
mantle-launch: received signal 15; stopping the agent
mantle-launch: agent exited (signal: 1 (SIGHUP))
test fifo_launcher_relays_without_unix_sockets_or_secret_slot ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s

     Running tests/conformance.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/conformance-0adf67a1aa5f2849)

running 6 tests
test adversary_volatile_preflight_preserves_fifo_and_socket_without_readiness ... ok
outside: mantle.egress.CheckBudgets/outcome/returned (OtherComponent)
outside: mantle.egress.CheckDefaultDestination/outcome/allowed (OtherComponent)
outside: mantle.egress.CheckDefaultDestination/outcome/denied (OtherComponent)
outside: mantle.egress.CheckDrain/outcome/returned (OtherComponent)
outside: mantle.egress.ClassifyAddress/outcome/returned (OtherComponent)
outside: mantle.egress.Exchange/outcome/returned (OtherComponent)
outside: mantle.manifest.Parse/outcome/returned (OtherComponent)
outside: mantle.manifest.ValidateMemory/outcome/accepted (OtherComponent)
outside: mantle.manifest.ValidateMemory/outcome/refused (OtherComponent)
outside: mantle.manifest.ValidatePids/outcome/accepted (OtherComponent)
outside: mantle.manifest.ValidatePids/outcome/refused (OtherComponent)
outside: mantle.manifest.ValidateRetention/outcome/accepted (OtherComponent)
outside: mantle.manifest.ValidateRetention/outcome/refused (OtherComponent)
outside: mantle.orchestration.CodexPreflight/outcome/returned (OtherComponent)
outside: mantle.orchestration.Observe/outcome/returned (OtherComponent)
outside: mantle.orchestration.Request/outcome/returned (OtherComponent)
outside: mantle.orchestration.Start/outcome/returned (OtherComponent)
outside: mantle.orchestration.Stop/outcome/returned (OtherComponent)
outside: mantle.orchestration.Worker/outcome/returned (OtherComponent)
outside: mantle.orchestration.WorkerRecords/outcome/returned (OtherComponent)
outside: mantle.session.AgentStartFailed/outcome/missing (OtherComponent)
outside: mantle.session.AgentStartFailed/outcome/moved (OtherComponent)
outside: mantle.session.AgentStarted/outcome/missing (OtherComponent)
outside: mantle.session.AgentStarted/outcome/moved (OtherComponent)
outside: mantle.session.AssessAgentInstallation/outcome/already-current (OtherComponent)
outside: mantle.session.AssessAgentInstallation/outcome/installed (OtherComponent)
outside: mantle.session.AssessAgentInstallation/outcome/refused (OtherComponent)
outside: mantle.session.AssessSharedBinaryUpgrade/outcome/applicable (OtherComponent)
outside: mantle.session.AssessSharedBinaryUpgrade/outcome/deferred (OtherComponent)
outside: mantle.session.AssessWorkerReadiness/outcome/missing (OtherComponent)
outside: mantle.session.AssessWorkerReadiness/outcome/ready (OtherComponent)
outside: mantle.session.BeginStop/outcome/missing (OtherComponent)
outside: mantle.session.BeginStop/outcome/moved (OtherComponent)
outside: mantle.session.FinishStop/outcome/missing (OtherComponent)
outside: mantle.session.FinishStop/outcome/moved (OtherComponent)
outside: mantle.session.IdentityMigration/outcome/returned (OtherComponent)
outside: mantle.session.InsertSession/outcome/conflict (OtherComponent)
outside: mantle.session.InsertSession/outcome/invalid-identity (OtherComponent)
outside: mantle.session.InsertSession/outcome/recorded (OtherComponent)
outside: mantle.session.MaterializationFailed/outcome/missing (OtherComponent)
outside: mantle.session.MaterializationFailed/outcome/moved (OtherComponent)
outside: mantle.session.Materialized/outcome/missing (OtherComponent)
outside: mantle.session.Materialized/outcome/moved (OtherComponent)
outside: mantle.session.PutSource/outcome/returned (OtherComponent)
outside: mantle.session.ReadSources/outcome/returned (OtherComponent)
outside: mantle.session.Session/state/FailedAgentStart/refuses/mantle.session.AgentStartFailed (OtherComponent)
outside: mantle.session.Session/state/FailedAgentStart/refuses/mantle.session.AgentStarted (OtherComponent)
outside: mantle.session.Session/state/FailedAgentStart/refuses/mantle.session.FinishStop (OtherComponent)
outside: mantle.session.Session/state/FailedAgentStart/refuses/mantle.session.MaterializationFailed (OtherComponent)
outside: mantle.session.Session/state/FailedAgentStart/refuses/mantle.session.Materialized (OtherComponent)
outside: mantle.session.Session/state/FailedMaterialization/refuses/mantle.session.AgentStartFailed (OtherComponent)
outside: mantle.session.Session/state/FailedMaterialization/refuses/mantle.session.AgentStarted (OtherComponent)
outside: mantle.session.Session/state/FailedMaterialization/refuses/mantle.session.FinishStop (OtherComponent)
outside: mantle.session.Session/state/FailedMaterialization/refuses/mantle.session.MaterializationFailed (OtherComponent)
outside: mantle.session.Session/state/FailedMaterialization/refuses/mantle.session.Materialized (OtherComponent)
outside: mantle.session.Session/state/Materializing/refuses/mantle.session.AgentStartFailed (OtherComponent)
outside: mantle.session.Session/state/Materializing/refuses/mantle.session.AgentStarted (OtherComponent)
outside: mantle.session.Session/state/Materializing/refuses/mantle.session.FinishStop (OtherComponent)
outside: mantle.session.Session/state/Running/refuses/mantle.session.AgentStartFailed (OtherComponent)
outside: mantle.session.Session/state/Running/refuses/mantle.session.AgentStarted (OtherComponent)
outside: mantle.session.Session/state/Running/refuses/mantle.session.FinishStop (OtherComponent)
outside: mantle.session.Session/state/Running/refuses/mantle.session.MaterializationFailed (OtherComponent)
outside: mantle.session.Session/state/Running/refuses/mantle.session.Materialized (OtherComponent)
outside: mantle.session.Session/state/Starting/refuses/mantle.session.FinishStop (OtherComponent)
outside: mantle.session.Session/state/Starting/refuses/mantle.session.MaterializationFailed (OtherComponent)
outside: mantle.session.Session/state/Starting/refuses/mantle.session.Materialized (OtherComponent)
outside: mantle.session.Session/state/Stopped/refuses/mantle.session.AgentStartFailed (OtherComponent)
outside: mantle.session.Session/state/Stopped/refuses/mantle.session.AgentStarted (OtherComponent)
outside: mantle.session.Session/state/Stopped/refuses/mantle.session.BeginStop (OtherComponent)
outside: mantle.session.Session/state/Stopped/refuses/mantle.session.FinishStop (OtherComponent)
outside: mantle.session.Session/state/Stopped/refuses/mantle.session.MaterializationFailed (OtherComponent)
outside: mantle.session.Session/state/Stopped/refuses/mantle.session.Materialized (OtherComponent)
outside: mantle.session.Session/state/Stopping/refuses/mantle.session.AgentStartFailed (OtherComponent)
outside: mantle.session.Session/state/Stopping/refuses/mantle.session.AgentStarted (OtherComponent)
outside: mantle.session.Session/state/Stopping/refuses/mantle.session.BeginStop (OtherComponent)
outside: mantle.session.Session/state/Stopping/refuses/mantle.session.MaterializationFailed (OtherComponent)
outside: mantle.session.Session/state/Stopping/refuses/mantle.session.Materialized (OtherComponent)
outside: mantle.session.Session/transition/materialization_failed/by/mantle.session.MaterializationFailed/moved (OtherComponent)
outside: mantle.session.Session/transition/materialized/by/mantle.session.Materialized/moved (OtherComponent)
outside: mantle.session.Session/transition/start_failed/by/mantle.session.AgentStartFailed/moved (OtherComponent)
outside: mantle.session.Session/transition/started/by/mantle.session.AgentStarted/moved (OtherComponent)
outside: mantle.session.Session/transition/stopped/by/mantle.session.FinishStop/moved (OtherComponent)
outside: mantle.session.Session/transition/stopping/by/mantle.session.BeginStop/moved (OtherComponent)
outside: mantle.session.SetAgentExec/outcome/missing (OtherComponent)
outside: mantle.session.SetAgentExec/outcome/recorded (OtherComponent)
outside: mantle.session.SetWorkspace/outcome/missing (OtherComponent)
outside: mantle.session.SetWorkspace/outcome/recorded (OtherComponent)
47 selected scenario(s), 30 authored source(s), 0 refusal occurrence(s)

test adversary_sigint_while_detached_never_persists_replay ... ok
test private_path_initialization_preserves_auth_and_refuses_unsafe_entries ... ok
test adversary_persistent_tail_refuses_volatile_reuse_then_clean_retry_runs ... ok
test volatile_runtime_does_not_persist_output_on_exit ... ok
mantle-launch: 47 passed, 0 failed, 0 unsupported
test ess_launch_conformance ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 15.21s

     Running tests/launch.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/launch-1bdf17dc3932f203)

running 4 tests
test attach_without_a_server_is_refused ... ok
test secret_stays_out_of_stderr_when_the_agent_cannot_start ... ok
test second_server_is_refused_and_sigterm_stops_the_first ... ok
test attach_relays_replays_scrollback_and_survives_a_detach ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.64s

     Running unittests src/lib.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/mantle_worker-0a69710eec6bc7a7)

running 28 tests
test tests::byte_limit_detects_growth_beyond_declared_size ... ok
test tests::delivery_accepts_cargo_hardlinked_artifacts ... ok
test tests::stale_transport_guard_cannot_retire_a_reused_pid_registration ... ok
test tests::concurrent_directory_winners_are_revalidated_without_permission_changes ... ok
test tests::symlinked_root_is_refused ... ok
test tests::termination_cleanup_includes_owned_descendants ... ok
test tests::adversary_second_successful_child_exit_cleans_its_remaining_group ... ok
test tests::idle_transport_coordinator_does_not_swallow_termination ... ok
test tests::unsupported_architecture_never_fetches_or_creates_paths ... ok
test tests::adversary_second_failed_spawn_preserves_following_transport_and_idle_state ... ok
test tests::termination_coordinator_cleans_concurrent_groups_and_preserves_signal_status ... ok
test tests::bounded_child_pumps_input_and_output_together ... ok
test tests::corrupt_archive_never_activates ... ok
test tests::adversary_refuses_binary_mismatch_after_valid_archive_before_execution ... ok
test tests::version_mismatch_never_activates ... ok
test tests::adversary_operator_interrupt_reaps_bounded_child ... ok
test tests::bounded_child_refuses_output_flood_and_timeout ... ok
test tests::adversary_unmanaged_executable_is_preserved_on_activation_refusal ... ok
test tests::adversary_fifo_and_symlink_inputs_refuse_without_waiting_for_writers ... ok
test tests::installation_is_verified_atomic_and_idempotent ... ok
test tests::tampered_installed_bytes_are_refused ... ok
test tests::failed_update_preserves_previous_binary_and_facts ... ok
test tests::adversary_concurrent_installation_fetches_once_and_publishes_one_generation ... ok
test tests::adversary_unpublished_verified_generation_recovers_after_interrupted_activation ... ok
test tests::adversary_second_update_observers_only_see_complete_verified_generations ... ok
test tests::generated_model_has_no_drift ... ok
test tests::retirement_survives_a_bounded_registry_wait_timeout ... ok
test tests::adversary_live_lock_refuses_without_fetching_or_changing_current ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.20s

     Running unittests src/main.rs (/dev/shm/mantle-wave5-target-codex-session/debug/deps/mantle_worker-5f8b887456570a7c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


cargo clippy --locked -p mantle -p mantle-launch -p mantle-worker --all-targets -- -D warnings
exit0; complete output:
    Checking mantle-launch v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle-launch)
    Checking mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle)
    Finished `dev` profile [unoptimized] target(s) in 11.67s

cargo fmt --check
exit0 (empty stdout/stderr). Repository-default command preserves generated dependency formatting. An earlier --all invocation was an incorrect selection and its generated bytes were restored by ESS regeneration before passing drift.

ess specify validate --path spec
exit0; output:
mantle v1 — 7 file(s), 227 scenario(s), valid

Generated through pinned ESS0.50 using AgentInstallationFacts, AgentInstallationOutcome, AgentKind and AuthenticationMethod roots. Initial ownership adopted from an exact opening-revision spec projection; generation was never hand-repaired. Existing scenario names and assertions were retained, adding required identity/ServeArgs fields to complete values. Guarded invalid-identity refusal uses ESS refusal precedence; a redundant accepting guard originally made external conflict synthesis fail ESS-SYNTH-003, preserved in first-mantle-package.log. Removing that redundant accepting guard retains the invalid-pair refusal and the old external conflict obligation; all218 native cases pass.

env -u TMPDIR ... cargo test --locked -p mantle --example codex_qualification adversary::
exit0; executed2, no skips/ignores, eight filtered non-adversary tests. These existing tests no longer require ambient TMPDIR; private parent lifetime and child watchdogs remain. Output:
    Blocking waiting for file lock on build directory
   Compiling mantle-worker-model v0.0.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/generated/worker-model)
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle-worker)
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave5-codex-session/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 1m 56s
     Running unittests examples/codex_qualification.rs (/dev/shm/mantle-wave5-target-codex-session/debug/examples/codex_qualification-2c3ae41e1f0be255)

running 2 tests

running 1 test

running 1 test
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... test adversary::binary_symlink_is_validated_against_its_current_opened_object ... okok


test result: 
oktest result: . 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered outok; finished in 0.00s. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out

; finished in 0.00s

test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.01s


Pinned synthetic probe (runtime-sinks-final.exit0): built codex_qualification and invoked runtime-sinks with the existing digest-verified0.153.4 binary, assigned launcher, assigned private scratch/tmpfs parents and --seconds6. No login/model/network acceptance was requested. Real Codex created five SQLite databases plus WAL/SHM and a zero-byte text log in verified tmpfs; fixed CLI config overrode conflicting ordinary user settings. The separate synthetic SQLite fixture's live database/WAL canary and persistent diagnostic positive control were both detected. Complete structured observation follows:
{
  "auth_refresh": "not-run",
  "authenticated_model_turn": "not-run",
  "codex_sqlite_files": [
    "goals_1.sqlite",
    "goals_1.sqlite-shm",
    "goals_1.sqlite-wal",
    "logs_2.sqlite",
    "logs_2.sqlite-shm",
    "logs_2.sqlite-wal",
    "memories_1.sqlite",
    "memories_1.sqlite-shm",
    "memories_1.sqlite-wal",
    "queue_1.sqlite",
    "queue_1.sqlite-shm",
    "queue_1.sqlite-wal",
    "state_5.sqlite",
    "state_5.sqlite-shm",
    "state_5.sqlite-wal"
  ],
  "conflicting_local_sink_paths_absent": true,
  "credentials": "isolated synthetic home; no login",
  "end_to_end_non_recording": "not-established; Substrate capture binding unavailable",
  "format": "mantle.codex-runtime-sinks/1",
  "managed_policy": "not-run",
  "sha256": "56ef98ab4032d317ab26e9b5e5a175650717351edb16ed9cde0cb6d1734d62da",
  "synthetic_fixture": {
    "persistent_canary_after_removal": false,
    "persistent_positive_control_detected": true,
    "provenance": "explicit synthetic SQLite fixture, not Codex refresh",
    "sqlite_and_live_wal_detected": true
  },
  "text_log_bytes": 0,
  "text_log_created_in_tmpfs": true,
  "tui": {
    "cleanup": "launcher reaped",
    "phases": [
      {
        "launcher_alive": true,
        "phase": "initial",
        "screen": {
          "bytes": 3496,
          "cursor_query": true,
          "login_marker": true,
          "permission_or_socket_error_marker": false,
          "welcome_marker": true
        }
      },
      {
        "launcher_alive": true,
        "phase": "same_size_reattach",
        "screen": {
          "bytes": 5946,
          "cursor_query": true,
          "login_marker": true,
          "permission_or_socket_error_marker": false,
          "welcome_marker": true
        }
      },
      {
        "launcher_alive": true,
        "phase": "resize",
        "screen": {
          "bytes": 1495,
          "cursor_query": true,
          "login_marker": true,
          "permission_or_socket_error_marker": false,
          "welcome_marker": false
        }
      },
      {
        "launcher_alive": true,
        "phase": "slow_reader",
        "screen": {
          "bytes": 900,
          "cursor_query": false,
          "login_marker": false,
          "permission_or_socket_error_marker": false,
          "welcome_marker": false
        }
      }
    ],
    "real_model_turn": "not-run: no authentication",
    "slow_reader": "500ms reader pause; no forced overflow claim",
    "usable_screen": "requires human verification"
  },
  "version": "codex-cli 0.153.4"
}

5. Deliberately outside this unit

Supported Substrate capture binding is unavailable in the pin; production explicitly refuses, and no flag can turn it into synthetic availability. The parent owns the real SDK update, upstream output-sink conformance, actual ChatGPT device login, refresh/failure sinks, authenticated managed-policy/approval behavior and complete lifecycle parity. The synthetic SQLite fixture is explicitly not Codex refresh. Intended auth/conversation rollout files remain private workspace files, so this does not claim zero storage of all session data. No Substrate source or network allowlist was changed. Full workspace/integration gate and independent adversary remain coordinator work; this report does not close the parent story or overall goal.

6. Outside paths and handoff

All retained task logs, read-only baseline model projection/adoption evidence, generated model reference, this report and temporary-test children are rooted at ~/.cache/mantle-wave5/scratch-codex-session (retained file inventory outside-paths.txt). Compiler output is exclusively the assigned /dev/shm/mantle-wave5-target-codex-session, with ordinary Cargo/rustc shared dependency caches inherited from the environment. Synthetic pinned-runtime children under /dev/shm/mantle-wave5-runtime-codex-session and persistent scratch were owned by Scratch and removed after each run. Native ESS runner artifacts are inside this worktree's ignored .engineering/drafts (CLI/launcher suite, run and report). Managed worktree session-start/heartbeat/session-end update manager-owned lease metadata only.

Lease codex-mantle-wave5-session-implementor is ended in lease-ended.log. Worktree remains dirty by design for coordinator inspection/integration; no commits/staging/publication were performed. Parent owns review, integration gate and eventual managed-worktree cleanup. Last resource observation root19GiB/tmpfs26GiB free.
