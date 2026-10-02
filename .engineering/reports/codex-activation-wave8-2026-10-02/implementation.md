unit:                   story:codex-existing-transport — Activate Codex with existing Substrate transport
verdict:                green
cases:                  executed 43→44, red 4 (two native-bin failures, two public CLI failures)
origin:                 n/a
wrote-outside-worktree: ~/.cache/mantle-wave8/scratch-activation; /dev/shm/mantle-wave8-target-activation
needs-coordinator:      no

1. Acceptance: operator-authorized Codex start/attach use existing Claude Substrate transport; keep fixed confinement, private home/diagnostics and Claude credential separation.

Scope confirmed by reading the five refusal call sites, request builders, public CLI tests, native Codex adapter/scenarios, and generated provenance drift test. Both SDK builders are exercised against a bounded synthetic Unix HTTP machine/workspace fixture; no real worker, credentials, or login. Existing request transport behavior is retained, not a fabricated no-recording SDK binding.

2. Actual diff before independent adversary additions follows.
 AGENTS.md                                     |   6 +-
 crates/mantle/src/adapters/orchestration.rs   |  37 ++--
 crates/mantle/src/adapters/substrate.rs       |  10 -
 crates/mantle/src/app/session.rs              | 275 ++++++++++++++++++++++++--
 crates/mantle/tests/codex_preflight.rs        |  10 +-
 generated/worker-model/source.schema.json     |   2 +-
 generated/worker-model/types-report.json      |   4 +-
 generated/worker-model/types.rs               |   4 +-
 spec/README.md                                |   5 +-
 spec/domains/orchestration.yaml               |  15 +-
 spec/scenarios/cli/codex-private-runtime.yaml |   5 +-
 spec/scenarios/cli/codex-start.yaml           |  21 +-
 12 files changed, 321 insertions(+), 73 deletions(-)

3. Red evidence.
Initial command was named baseline.log when started; while dependencies compiled (before Mantle compiled), revised tests were applied. It therefore IS a red run, NOT a fresh baseline. Coordinator authorized reuse of exact preceding-wave 185 Rust/336 native baseline. Prior package's own summary had 29 bin +4 integration +10 example =43; new package has30+4+10=44; nested subprocess summaries are excluded.
Command: CARGO_TARGET_DIR=/dev/shm/mantle-wave8-target-activation CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 cargo test -p mantle --locked
Exit101. Output verbatim follows.
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
   Compiling shlex v2.0.1
   Compiling find-msvc-tools v0.1.14
   Compiling errno v0.3.14
   Compiling signal-hook-registry v1.4.8
   Compiling parking_lot_core v0.9.12
   Compiling pin-project-lite v0.2.17
   Compiling jobserver v0.1.35
   Compiling scopeguard v1.2.0
   Compiling lock_api v0.4.14
   Compiling cc v1.5.1
   Compiling mio v1.2.3
   Compiling parking_lot v0.12.5
   Compiling tokio-macros v2.7.2
   Compiling socket2 v0.6.5
   Compiling futures-core v0.3.34
   Compiling tokio v1.53.1
   Compiling once_cell v1.21.4
   Compiling zeroize v1.9.0
   Compiling http v1.5.0
   Compiling futures-sink v0.3.34
   Compiling autocfg v1.5.1
   Compiling fnv v1.0.7
   Compiling typenum v1.20.1
   Compiling slab v0.4.12
   Compiling pkg-config v0.3.34
   Compiling ryu v1.0.23
   Compiling num-traits v0.2.19
   Compiling tracing-core v0.1.36
   Compiling tracing-attributes v0.1.31
   Compiling futures-macro v0.3.34
   Compiling tokio-util v0.7.19
   Compiling futures-task v0.3.34
   Compiling futures-util v0.3.34
   Compiling tracing v0.1.44
   Compiling http-body v1.1.0
   Compiling http v0.2.12
   Compiling ring v0.17.14
   Compiling hashbrown v0.17.1
   Compiling equivalent v1.0.2
   Compiling indexmap v2.14.2
   Compiling http-body v0.4.6
   Compiling num-integer v0.1.47
   Compiling hybrid-array v0.4.15
   Compiling http-body-util v0.1.5
   Compiling powerfmt v0.2.0
   Compiling time-core v0.1.9
   Compiling cmov v0.5.4
   Compiling vsimd v0.8.0
   Compiling cpufeatures v0.3.1
   Compiling either v1.18.0
   Compiling num-conv v0.2.2
   Compiling outref v0.5.2
   Compiling deranged v0.5.8
   Compiling base64-simd v0.8.0
   Compiling bytes-utils v0.1.4
   Compiling ctutils v0.4.2
   Compiling crypto-common v0.2.2
   Compiling time v0.3.55
   Compiling block-buffer v0.12.1
   Compiling cmake v0.1.58
   Compiling getrandom v0.2.17
   Compiling fs_extra v1.3.0
   Compiling dunce v1.0.5
   Compiling httparse v1.10.1
   Compiling untrusted v0.9.0
   Compiling pin-utils v0.1.1
   Compiling const-oid v0.10.2
   Compiling aws-smithy-types v1.8.1
   Compiling digest v0.11.3
   Compiling aws-lc-sys v0.45.0
   Compiling aws-smithy-async v1.3.0
   Compiling aws-smithy-runtime-api-macros v1.1.0
   Compiling serde_core v1.0.229
   Compiling aws-smithy-runtime-api v1.19.0
   Compiling rustls-pki-types v1.15.1
   Compiling aws-lc-rs v1.18.1
   Compiling percent-encoding v2.3.2
   Compiling subtle v2.6.1
   Compiling synstructure v0.14.0
   Compiling try-lock v0.2.5
   Compiling rustls v0.23.45
   Compiling serde v1.0.229
   Compiling base64 v0.23.1
   Compiling semver v1.0.28
   Compiling rustc_version v0.4.1
   Compiling want v0.3.1
   Compiling sha2 v0.11.0
   Compiling futures-channel v0.3.34
   Compiling serde_derive v1.0.229
   Compiling tower-service v0.3.3
   Compiling httpdate v1.0.3
   Compiling zerofrom-derive v0.1.8
   Compiling atomic-waker v1.1.2
   Compiling h2 v0.4.19
   Compiling zerofrom v0.1.8
   Compiling yoke-derive v0.8.4
   Compiling aws-smithy-schema v0.2.1
   Compiling stable_deref_trait v1.2.1
   Compiling zmij v1.0.23
   Compiling rustls v0.21.12
   Compiling yoke v0.8.3
   Compiling hyper v1.11.1
   Compiling rustls-webpki v0.101.7
   Compiling sct v0.7.1
   Compiling memchr v2.8.3
   Compiling ipnet v2.12.2
   Compiling serde_json v1.0.151
   Compiling hyper-util v0.1.21
   Compiling form_urlencoded v1.2.2
   Compiling h2 v0.3.27
   Compiling zerovec-derive v0.11.6
   Compiling socket2 v0.5.10
   Compiling fastrand v2.5.0
   Compiling openssl-probe v0.2.1
   Compiling bitflags v2.13.2
   Compiling thiserror v2.0.21
   Compiling rustls-native-certs v0.8.4
   Compiling hyper v0.14.32
   Compiling zerovec v0.11.8
   Compiling tokio-rustls v0.24.1
   Compiling thiserror-impl v2.0.21
   Compiling displaydoc v0.2.7
   Compiling hex v0.4.3
   Compiling tower-layer v0.3.3
   Compiling tower v0.5.3
   Compiling hyper-rustls v0.24.2
   Compiling aws-types v1.6.0
   Compiling aws-smithy-http v0.64.1
   Compiling aws-credential-types v1.3.0
   Compiling version_check v0.9.5
   Compiling rand_core v0.10.1
   Compiling getrandom v0.4.3
   Compiling zerocopy v0.8.59
   Compiling rustversion v1.0.23
   Compiling generic-array v0.14.7
   Compiling aws-smithy-observability v0.3.0
   Compiling hmac v0.13.0
   Compiling zerocopy-derive v0.8.59
   Compiling aws-sigv4 v1.6.0
   Compiling tinystr v0.8.4
   Compiling writeable v0.6.4
   Compiling cfg_aliases v0.2.2
   Compiling litemap v0.8.3
   Compiling uuid v1.26.1
   Compiling icu_locale_core v2.3.0
   Compiling arc-swap v1.9.2
   Compiling zerotrie v0.2.5
   Compiling potential_utf v0.1.6
   Compiling aws-smithy-json v0.63.1
   Compiling serde_derive_internals v0.29.1
   Compiling regex-lite v0.1.9
   Compiling utf8parse v0.2.2
   Compiling icu_normalizer_data v2.3.0
   Compiling icu_properties_data v2.3.0
   Compiling schemars v0.8.22
   Compiling unsafe-libyaml v0.2.11
   Compiling utf8_iter v1.0.4
   Compiling icu_collections v2.3.0
   Compiling serde_yaml v0.9.34+deprecated
   Compiling anstyle-parse v1.0.0
   Compiling schemars_derive v0.8.22
   Compiling icu_provider v2.3.1
   Compiling chacha20 v0.10.2
   Compiling zstd-sys v2.1.0+zstd.1.5.7
   Compiling dyn-clone v1.0.20
   Compiling is_terminal_polyfill v1.70.2
   Compiling getrandom v0.3.4
   Compiling anstyle v1.0.14
   Compiling anstyle-query v1.1.5
   Compiling xmlparser v0.13.6
   Compiling rustix v1.1.5
   Compiling colorchoice v1.0.5
   Compiling anstream v1.0.0
   Compiling aws-smithy-xml v0.62.1
   Compiling rand v0.10.3
   Compiling crypto-common v0.1.7
   Compiling block-buffer v0.10.4
   Compiling libm v0.2.16
   Compiling anyhow v1.0.104
   Compiling crc32fast v1.5.2
   Compiling linux-raw-sys v0.12.1
   Compiling strsim v0.11.1
   Compiling clap_lex v1.1.1
   Compiling urlencoding v2.1.3
   Compiling signal-hook v0.3.18
   Compiling heck v0.5.0
   Compiling clap_derive v4.6.7
   Compiling aws-smithy-query v0.62.1
   Compiling clap_builder v4.6.7
   Compiling digest v0.10.7
   Compiling icu_properties v2.3.0
   Compiling icu_normalizer v2.3.0
   Compiling ess-primitives v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling curve25519-dalek v5.0.0
   Compiling bigdecimal v0.4.11
   Compiling vcpkg v0.2.15
   Compiling simd-adler32 v0.3.10
   Compiling pulldown-cmark v0.13.4
   Compiling zstd-safe v7.3.0
   Compiling adler2 v2.0.1
   Compiling unicode-segmentation v1.13.3
   Compiling cpufeatures v0.2.17
   Compiling convert_case v0.10.0
   Compiling miniz_oxide v0.9.1
   Compiling libsqlite3-sys v0.35.0
   Compiling rand_core v0.9.5
   Compiling ess-domain v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling idna_adapter v1.2.2
   Compiling rustls-webpki v0.103.15
   Compiling clap v4.6.7
   Compiling nix v0.31.3
   Compiling nix v0.30.1
   Compiling half v2.7.1
   Compiling ppv-lite86 v0.2.21
   Compiling sha1 v0.11.0
   Compiling num-bigint v0.4.8
   Compiling curve25519-dalek-derive v0.1.1
   Compiling pulldown-cmark-escape v0.11.0
   Compiling unicase v2.9.0
   Compiling tokio-rustls v0.26.6
   Compiling signature v3.0.0
   Compiling data-encoding v2.11.1
   Compiling hyper-rustls v0.27.10
   Compiling aws-smithy-http-client v1.5.0
   Compiling iana-time-zone v0.1.65
   Compiling foldhash v0.1.5
   Compiling hashbrown v0.15.5
   Compiling chrono v0.4.45
   Compiling tungstenite v0.30.0
   Compiling ed25519 v3.0.0
   Compiling aws-smithy-runtime v1.16.0
   Compiling rand_chacha v0.9.0
   Compiling minicbor v2.3.0
   Compiling aws-runtime v1.10.0
   Compiling flate2 v1.1.10
   Compiling idna v1.1.0
   Compiling ess-compiler v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling derive_more-impl v2.1.1
   Compiling tempfile v3.27.0
   Compiling ulid v3.0.0
   Compiling ureq-proto v0.6.4
   Compiling webpki-roots v1.0.9
   Compiling winnow v1.0.4
   Compiling litrs v1.0.0
   Compiling utf8-zero v0.8.1
   Compiling ureq v3.4.2
   Compiling document-features v0.2.12
   Compiling toml_parser v1.1.3+spec-1.1.0
   Compiling b10x-substrate-wire v0.7.8 (https://github.com/beyond10x/substrate?rev=05695970b069f79e6678f2f02cbd78bbe5fa2a56#05695970)
   Compiling derive_more v2.1.1
   Compiling ess-gen v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling url v2.5.8
   Compiling aws-smithy-compression v0.2.0
   Compiling aws-sdk-sso v1.114.0
   Compiling aws-sdk-sts v1.119.0
   Compiling aws-sdk-ssooidc v1.116.0
   Compiling zstd v0.13.3
   Compiling aws-smithy-cbor v0.62.2
   Compiling rand v0.9.5
   Compiling ed25519-dalek v3.0.0
   Compiling tokio-tungstenite v0.30.0
   Compiling hashlink v0.10.0
   Compiling signal-hook-mio v0.2.5
   Compiling sha1 v0.10.7
   Compiling sha2 v0.10.9
   Compiling mantle-worker-model v0.0.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave8-codex-activation/generated/worker-model)
   Compiling serde_urlencoded v0.7.1
   Compiling rustls-pemfile v2.2.0
   Compiling toml_datetime v0.7.5+spec-1.1.0
   Compiling serde_spanned v1.1.1
   Compiling fallible-streaming-iterator v0.1.9
   Compiling toml_writer v1.1.2+spec-1.1.0
   Compiling winnow v0.7.15
   Compiling fallible-iterator v0.3.0
   Compiling base64 v0.22.1
   Compiling rusqlite v0.37.0
   Compiling toml v0.9.12+spec-1.1.0
   Compiling b10x-substrate-sdk v0.7.8 (https://github.com/beyond10x/substrate?rev=05695970b069f79e6678f2f02cbd78bbe5fa2a56#05695970)
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave8-codex-activation/crates/mantle-worker)
   Compiling aws-config v1.12.0
   Compiling crossterm v0.29.0
   Compiling ulid v1.2.1
   Compiling aws-sdk-cloudwatch v1.134.0
   Compiling ess-conformance v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling aws-sdk-iam v1.128.0
   Compiling aws-sdk-ec2 v1.267.0
   Compiling mantle-conformance v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave8-codex-activation/crates/mantle-conformance)
   Compiling mantle-egress v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave8-codex-activation/crates/mantle-egress)
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave8-codex-activation/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 5m 31s
     Running unittests src/main.rs (/dev/shm/mantle-wave8-target-activation/debug/deps/mantle-277e3ad59dda8f7b)

running 30 tests
test app::terminal::tests::ctrl_bracket_d_detaches_and_forwards_what_came_before ... ok
test app::terminal::tests::escape_followed_by_anything_else_passes_both_bytes ... ok
test adapters::substrate::worker_tests::common_readiness_and_selected_claude_are_distinct_observations ... ok
test app::terminal::tests::the_sequence_is_recognised_across_reads ... ok
test config::tests::explicit_path_selection_is_absolute_and_preserves_defaults ... ok
test config::tests::legacy_claude_table_remains_parseable ... ok
test app::worker::tests::legacy_claude_bootstrap_retains_required_slot ... ok
test app::worker::tests::the_template_renders_every_placeholder ... ok
test domain::manifest::tests::sizes_and_durations_parse ... ok
test domain::manifest::tests::cwd_must_be_inside_a_mount ... ok
test config::tests::common_worker_configuration_needs_no_claude_credentials ... ok
test domain::manifest::tests::defaults_apply ... ok
test domain::session::tests::names_round_trip ... ok
test domain::manifest::tests::retain_for_is_bounded_by_the_exec_limit ... ok
test domain::manifest::tests::unserved_fields_are_refused_by_name ... ok
test domain::session::tests::running_is_never_reached_without_starting ... ok
test domain::session::tests::stopped_is_final_and_failures_only_stop ... ok
test domain::manifest::tests::cwd_with_dot_dot_or_empty_components_is_refused ... ok
test domain::session::tests::the_happy_path_is_legal ... ok
test domain::manifest::tests::mounts_and_refs_cannot_escape ... ok
test domain::manifest::tests::the_example_manifest_resolves ... ok
test app::session::codex_transport_tests::codex_start_and_attach_construct_real_sdk_builders ... FAILED
test adapters::state::tests::an_illegal_move_is_refused_and_not_written ... ok
test app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command ... ok
test adapters::ssh::tests::tunnel_socket_binds_with_live_length_state_prefix ... ok
test adapters::state::tests::sources_round_trip ... ok
test adapters::state::tests::a_live_name_is_unique_until_stopped ... ok
test adapters::ssh::tests::socket_allocations_are_private_unique_and_state_path_independent ... ok
test adapters::ssh::tests::tunnel_socket_directory_lives_until_child_cleanup_and_is_removed_on_errors ... ok
test adapters::state::conformance::ess_generated_local_conformance ... FAILED

failures:

---- app::session::codex_transport_tests::codex_start_and_attach_construct_real_sdk_builders stdout ----

thread 'app::session::codex_transport_tests::codex_start_and_attach_construct_real_sdk_builders' (2918582) panicked at crates/mantle/src/app/session.rs:1098:13:
Codex start builder refused: Some(FAILED_CAPABILITY: Codex requires supported non-recording terminal capture; the pinned Substrate SDK cannot provide it)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- adapters::state::conformance::ess_generated_local_conformance stdout ----
outside: mantle.egress.CheckBudgets/outcome/returned (OtherComponent)
outside: mantle.egress.CheckDefaultDestination/outcome/allowed (OtherComponent)
outside: mantle.egress.CheckDefaultDestination/outcome/denied (OtherComponent)
outside: mantle.egress.CheckDrain/outcome/returned (OtherComponent)
outside: mantle.egress.ClassifyAddress/outcome/returned (OtherComponent)
outside: mantle.egress.Exchange/outcome/returned (OtherComponent)
outside: mantle.launch.AttachBackpressure/outcome/returned (OtherComponent)
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
Error: ESS failures: [
    ScenarioResult {
        scenario: Authored {
            domain: DomainRef(
                QualifiedName(mantle.orchestration),
            ),
            name: AuthoredName(
                "codex-start",
            ),
        },
        purpose: "Codex identity survives migration and starts and attaches through the existing transport.",
        status: Failed,
        checks: [
            CheckResult {
                code: Outcome,
                about: "outcome mantle.session.IdentityMigration/returned",
                status: Passed,
                diagnostic: None,
            },
            CheckResult {
                code: Payload,
                about: "direct response mantle.session.IdentityMigration",
                status: Passed,
                diagnostic: None,
            },
            CheckResult {
                code: Outcome,
                about: "outcome mantle.orchestration.CodexPreflight/returned",
                status: Passed,
                diagnostic: None,
            },
            CheckResult {
                code: Payload,
                about: "direct response mantle.orchestration.CodexPreflight",
                status: Failed,
                diagnostic: Some(
                    Diagnostic {
                        code: Payload,
                        scenario: Authored {
                            domain: DomainRef(
                                QualifiedName(mantle.orchestration),
                            ),
                            name: AuthoredName(
                                "codex-start",
                            ),
                        },
                        source: [
                            Command {
                                name: CommandRef(
                                    QualifiedName(mantle.orchestration.CodexPreflight),
                                ),
                            },
                        ],
                        input: None,
                        expected: [
                            "actual typed return satisfies its complete schema and authored literals",
                        ],
                        observed: [
                            "response field agent_exec differs from its declared literal",
                        ],
                    },
                ),
            },
            CheckResult {
                code: Outcome,
                about: "outcome mantle.manifest.Parse/returned",
                status: Passed,
                diagnostic: None,
            },
            CheckResult {
                code: Payload,
                about: "direct response mantle.manifest.Parse",
                status: Passed,
                diagnostic: None,
            },
            CheckResult {
                code: Outcome,
                about: "outcome mantle.manifest.Parse/returned",
                status: Passed,
                diagnostic: None,
            },
            CheckResult {
                code: Payload,
                about: "direct response mantle.manifest.Parse",
                status: Passed,
                diagnostic: None,
            },
            CheckResult {
                code: Outcome,
                about: "outcome mantle.manifest.Parse/returned",
                status: Passed,
                diagnostic: None,
            },
            CheckResult {
                code: Payload,
                about: "direct response mantle.manifest.Parse",
                status: Passed,
                diagnostic: None,
            },
        ],
        duration_ms: 6,
    },
]


failures:
    adapters::state::conformance::ess_generated_local_conformance
    app::session::codex_transport_tests::codex_start_and_attach_construct_real_sdk_builders

test result: FAILED. 28 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.18s

error: test failed, to rerun pass `-p mantle --bin mantle`

Separate public CLI red: same environment cargo test -p mantle --locked --test codex_preflight; exit101. Exact output follows.
    Finished `test` profile [unoptimized] target(s) in 18.63s
     Running tests/codex_preflight.rs (/dev/shm/mantle-wave8-target-activation/debug/deps/codex_preflight-f14a9f5dcb0fcf21)

running 4 tests
test codex_reaches_worker_boundary_without_claude_credentials_or_session_insertion ... FAILED
test adversary_codex_never_invokes_configured_claude_credential_command ... FAILED
test adversary_partial_identity_schema_refusal_rolls_back_initialization ... ok
test adversary_legacy_migration_preserves_indexes_and_stopped_sources_then_refuses_corruption ... ok

failures:

---- codex_reaches_worker_boundary_without_claude_credentials_or_session_insertion stdout ----

thread 'codex_reaches_worker_boundary_without_claude_credentials_or_session_insertion' (2930724) panicked at crates/mantle/tests/codex_preflight.rs:36:5:
Error: FAILED_CAPABILITY: Codex requires supported non-recording terminal capture; the pinned Substrate SDK cannot provide it


---- adversary_codex_never_invokes_configured_claude_credential_command stdout ----

thread 'adversary_codex_never_invokes_configured_claude_credential_command' (2930721) panicked at crates/mantle/tests/codex_preflight.rs:260:5:
assertion failed: String::from_utf8(output.stderr).unwrap().contains("no worker recorded; run `mantle worker up` first")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary_codex_never_invokes_configured_claude_credential_command
    codex_reaches_worker_boundary_without_claude_credentials_or_session_insertion

test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

error: test failed, to rerun pass `-p mantle --test codex_preflight`

The first native red fixture also initially used a non-commit synthetic source token and guessed materialization argv. Before removing production guards, source inspection corrected these to a valid fixed40hex commit and actual mkdir/clone/rev-parse sequence. No green claim is based on that initial fixture. The SDK builder red independently names the exact capture refusal; public CLI red independently names expected missing-worker boundary versus capture refusal.

4. Green package command: CARGO_TARGET_DIR=/dev/shm/mantle-wave8-target-activation CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 cargo test -p mantle --locked; exit0. Exact output follows.
   Compiling mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave8-codex-activation/crates/mantle)
    Finished `test` profile [unoptimized] target(s) in 4.76s
     Running unittests src/main.rs (/dev/shm/mantle-wave8-target-activation/debug/deps/mantle-277e3ad59dda8f7b)

running 30 tests
test app::terminal::tests::ctrl_bracket_d_detaches_and_forwards_what_came_before ... ok
test app::terminal::tests::escape_followed_by_anything_else_passes_both_bytes ... ok
test app::terminal::tests::the_sequence_is_recognised_across_reads ... ok
test adapters::substrate::worker_tests::common_readiness_and_selected_claude_are_distinct_observations ... ok
test config::tests::common_worker_configuration_needs_no_claude_credentials ... ok
test config::tests::explicit_path_selection_is_absolute_and_preserves_defaults ... ok
test config::tests::legacy_claude_table_remains_parseable ... ok
test domain::manifest::tests::sizes_and_durations_parse ... ok
test app::worker::tests::legacy_claude_bootstrap_retains_required_slot ... ok
test domain::manifest::tests::defaults_apply ... ok
test domain::manifest::tests::cwd_must_be_inside_a_mount ... ok
test domain::session::tests::the_happy_path_is_legal ... ok
test domain::session::tests::stopped_is_final_and_failures_only_stop ... ok
test app::worker::tests::the_template_renders_every_placeholder ... ok
test domain::manifest::tests::retain_for_is_bounded_by_the_exec_limit ... ok
test adapters::ssh::tests::tunnel_socket_directory_lives_until_child_cleanup_and_is_removed_on_errors ... ok
test domain::manifest::tests::cwd_with_dot_dot_or_empty_components_is_refused ... ok
test domain::manifest::tests::unserved_fields_are_refused_by_name ... ok
test domain::session::tests::running_is_never_reached_without_starting ... ok
test domain::session::tests::names_round_trip ... ok
test domain::manifest::tests::mounts_and_refs_cannot_escape ... ok
test domain::manifest::tests::the_example_manifest_resolves ... ok
test adapters::state::tests::sources_round_trip ... ok
test adapters::state::tests::an_illegal_move_is_refused_and_not_written ... ok
test adapters::state::tests::a_live_name_is_unique_until_stopped ... ok
test app::worker::tests::both_profiles_keep_aperture_and_ca_in_the_daemon_command ... ok
test app::session::codex_transport_tests::codex_start_and_attach_construct_real_sdk_builders ... ok
test adapters::ssh::tests::tunnel_socket_binds_with_live_length_state_prefix ... ok
test adapters::ssh::tests::socket_allocations_are_private_unique_and_state_path_independent ... ok
test adapters::state::conformance::ess_generated_local_conformance ... ok

test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.14s

     Running tests/codex_preflight.rs (/dev/shm/mantle-wave8-target-activation/debug/deps/codex_preflight-f14a9f5dcb0fcf21)

running 4 tests
test codex_reaches_worker_boundary_without_claude_credentials_or_session_insertion ... ok
test adversary_codex_never_invokes_configured_claude_credential_command ... ok
test adversary_partial_identity_schema_refusal_rolls_back_initialization ... ok
test adversary_legacy_migration_preserves_indexes_and_stopped_sources_then_refuses_corruption ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running unittests examples/codex_qualification.rs (/dev/shm/mantle-wave8-target-activation/debug/examples/codex_qualification-2c3ae41e1f0be255)

running 10 tests
test tests::digest_format_is_strict ... ok
test tests::digest_bounds_actual_input_even_when_it_exceeds_the_size_snapshot ... ok
test tests::digest_refuses_other_non_regular_descriptors ... ok
test tests::fixed_observation_never_exports_terminal_payload ... ok
test tests::unsafe_scratch_parents_are_refused_without_creating_children ... ok
test tests::scratch_budget_refuses_large_files_and_does_not_follow_links ... ok

running 1 test

running 1 test
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

test tests::terminal_observation_is_bounded_and_handles_split_queries ... ok
test adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer ... ok
test adversary::binary_symlink_is_validated_against_its_current_opened_object ... ok
test tests::cq01_rejects_wrong_digest_before_executing_a_binary ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s


Package: executed43→44, exit0 (bin29→30; publicCLI4→4; example10→10). PublicCLI count unchanged because obsolete policy assertions were replaced under explicit operator authorization, not new test functions. Native CLI executed218→218; scenario policy assertions replaced without removing/skipping cases. Exact report/suite/run retained under unit .engineering/drafts/mantle-cli-{report,suite,run}.json. Counts passed218 failed0 error0 skipped0 unsupported0; authored137 generated81, expected other-component outside24, refused0.

cargo fmt --check exit0 (fmt.log/fmt.exit). ess specify validate --path spec exit0: mantle v1 — 7 file(s), 231 scenario(s), valid.

Direct ESS generation into committed output refused: “error: unowned output destination: Cargo.toml; adopt exact generated reference bytes explicitly”. Preserved generate.log/exit1. Generated into fresh scratch/generated using repository drift-test command and copied exactly Cargo.toml/types.rs/source.schema.json/types-report.json; generate-fresh.log/exit0. Cargo.toml byte-unchanged; three generated provenance files changed.

5. Did not perform real authentication/device login, model/tool execution, worker setup/restart, Substrate change, gateway changes, docs agent files, or AEP changes; these are outside this source unit. Existing outer confinement and resource bounds remain unchanged. Substrate transport can record terminal output per explicit authorization.

6. External writes: all logs, statuses, sdk-test.txt/preflight.txt drafts, generated/ and this report under ~/.cache/mantle-wave8/scratch-activation. Exclusive reproducible build output /dev/shm/mantle-wave8-target-activation; coordinator owns cleanup. No commits/pushes made.

Final package lint: CARGO_TARGET_DIR=/dev/shm/mantle-wave8-target-activation CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 cargo clippy -p mantle --all-targets --locked -- -D warnings; exit0. Coordinator requested an interrupt for urgency, but the exact cargo process had already exited naturally before SIGINT (kill: No such process); exec handle completed0 and clippy.exit contains0. No build processes retained when target was handed exclusively to adversary. Exact lint output follows.
    Blocking waiting for file lock on package cache
    Checking cfg-if v1.0.5
    Checking libc v0.2.189
    Checking itoa v1.0.18
    Checking bytes v1.12.1
    Checking log v0.4.34
    Checking smallvec v1.16.2
    Checking pin-project-lite v0.2.17
    Checking scopeguard v1.2.0
    Checking lock_api v0.4.14
    Checking errno v0.3.14
    Checking parking_lot_core v0.9.12
    Checking signal-hook-registry v1.4.8
    Checking mio v1.2.3
    Checking parking_lot v0.12.5
    Checking socket2 v0.6.5
    Checking once_cell v1.21.4
    Checking futures-core v0.3.34
    Checking tokio v1.53.1
    Checking zeroize v1.9.0
    Checking http v1.5.0
    Checking futures-sink v0.3.34
    Checking slab v0.4.12
    Checking typenum v1.20.1
    Checking fnv v1.0.7
    Checking ryu v1.0.23
    Checking tracing-core v0.1.36
    Checking futures-task v0.3.34
    Checking futures-util v0.3.34
    Checking tokio-util v0.7.19
    Checking tracing v0.1.44
    Checking num-traits v0.2.19
    Checking http-body v1.1.0
    Checking http v0.2.12
    Checking hashbrown v0.17.1
    Checking equivalent v1.0.2
    Checking http-body v0.4.6
    Checking indexmap v2.14.2
    Checking num-integer v0.1.47
    Checking hybrid-array v0.4.15
    Checking http-body-util v0.1.5
    Checking vsimd v0.8.0
    Checking deranged v0.5.8
    Checking num-conv v0.2.2
    Checking powerfmt v0.2.0
    Checking outref v0.5.2
    Checking cpufeatures v0.3.1
    Checking time-core v0.1.9
    Checking either v1.18.0
    Checking cmov v0.5.4
    Checking bytes-utils v0.1.4
    Checking ctutils v0.4.2
    Checking time v0.3.55
    Checking base64-simd v0.8.0
    Checking crypto-common v0.2.2
    Checking block-buffer v0.12.1
    Checking getrandom v0.2.17
    Checking untrusted v0.9.0
    Checking pin-utils v0.1.1
    Checking const-oid v0.10.2
    Checking aws-smithy-types v1.8.1
    Checking digest v0.11.3
    Checking ring v0.17.14
    Checking aws-smithy-async v1.3.0
    Checking aws-smithy-runtime-api v1.19.0
    Checking httparse v1.10.1
    Checking serde_core v1.0.229
    Checking rustls-pki-types v1.15.1
    Checking percent-encoding v2.3.2
    Checking aws-lc-sys v0.45.0
    Checking subtle v2.6.1
    Checking aws-lc-rs v1.18.1
    Checking try-lock v0.2.5
    Checking base64 v0.23.1
    Checking rustls-webpki v0.103.15
    Checking want v0.3.1
    Checking sha2 v0.11.0
    Checking futures-channel v0.3.34
    Checking httpdate v1.0.3
    Checking tower-service v0.3.3
    Checking serde v1.0.229
    Checking rustls v0.23.45
    Checking atomic-waker v1.1.2
    Checking h2 v0.4.19
    Checking zerofrom v0.1.8
    Checking aws-smithy-schema v0.2.1
    Checking stable_deref_trait v1.2.1
    Checking yoke v0.8.3
    Checking hyper v1.11.1
    Checking sct v0.7.1
    Checking rustls-webpki v0.101.7
    Checking ipnet v2.12.2
    Checking memchr v2.8.3
    Checking hyper-util v0.1.21
    Checking rustls v0.21.12
    Checking zmij v1.0.23
    Checking tokio-rustls v0.26.6
    Checking form_urlencoded v1.2.2
    Checking h2 v0.3.27
    Checking socket2 v0.5.10
    Checking fastrand v2.5.0
    Checking openssl-probe v0.2.1
    Checking bitflags v2.13.2
    Checking rustls-native-certs v0.8.4
    Checking zerovec v0.11.8
    Checking tokio-rustls v0.24.1
    Checking serde_json v1.0.151
    Checking hyper v0.14.32
    Checking tower-layer v0.3.3
    Checking hex v0.4.3
    Checking tower v0.5.3
    Checking thiserror v2.0.21
    Checking hyper-rustls v0.27.10
    Checking aws-smithy-http v0.64.1
    Checking aws-credential-types v1.3.0
    Checking rand_core v0.10.1
    Checking aws-smithy-observability v0.3.0
    Checking hyper-rustls v0.24.2
    Checking hmac v0.13.0
    Checking aws-smithy-http-client v1.5.0
    Checking zerocopy v0.8.59
    Checking aws-smithy-runtime v1.16.0
    Checking aws-sigv4 v1.6.0
    Checking aws-types v1.6.0
    Checking getrandom v0.4.3
    Checking tinystr v0.8.4
    Checking uuid v1.26.1
    Checking writeable v0.6.4
    Checking litemap v0.8.3
    Checking aws-runtime v1.10.0
    Checking icu_locale_core v2.3.0
    Checking generic-array v0.14.7
    Checking arc-swap v1.9.2
    Checking zerotrie v0.2.5
    Checking potential_utf v0.1.6
    Checking aws-smithy-json v0.63.1
    Checking regex-lite v0.1.9
    Checking utf8_iter v1.0.4
    Checking unsafe-libyaml v0.2.11
    Checking utf8parse v0.2.2
    Checking anstyle-parse v1.0.0
    Checking icu_collections v2.3.0
    Checking serde_yaml v0.9.34+deprecated
    Checking icu_provider v2.3.1
    Checking chacha20 v0.10.2
    Checking xmlparser v0.13.6
    Checking is_terminal_polyfill v1.70.2
    Checking anstyle-query v1.1.5
    Checking colorchoice v1.0.5
    Checking dyn-clone v1.0.20
    Checking anstyle v1.0.14
    Checking schemars v0.8.22
    Checking anstream v1.0.0
    Checking aws-smithy-xml v0.62.1
    Checking rand v0.10.3
    Checking icu_normalizer_data v2.3.0
    Checking icu_properties_data v2.3.0
    Checking crypto-common v0.1.7
    Checking block-buffer v0.10.4
    Checking strsim v0.11.1
    Checking linux-raw-sys v0.12.1
    Checking urlencoding v2.1.3
    Checking clap_lex v1.1.1
    Checking rustix v1.1.5
    Checking clap_builder v4.6.7
    Checking aws-smithy-query v0.62.1
    Checking digest v0.10.7
    Checking icu_properties v2.3.0
    Checking icu_normalizer v2.3.0
    Checking ess-primitives v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking getrandom v0.3.4
    Checking simd-adler32 v0.3.10
    Checking adler2 v2.0.1
    Checking cpufeatures v0.2.17
    Checking miniz_oxide v0.9.1
    Checking rand_core v0.9.5
    Checking ess-domain v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking idna_adapter v1.2.2
    Checking signal-hook v0.3.18
    Checking clap v4.6.7
    Checking crc32fast v1.5.2
    Checking libm v0.2.16
    Checking anyhow v1.0.104
    Checking zstd-sys v2.1.0+zstd.1.5.7
    Checking ppv-lite86 v0.2.21
    Checking half v2.7.1
    Checking sha1 v0.11.0
    Checking num-bigint v0.4.8
    Checking iana-time-zone v0.1.65
    Checking signature v3.0.0
    Checking data-encoding v2.11.1
    Checking foldhash v0.1.5
    Checking unicase v2.9.0
    Checking pulldown-cmark-escape v0.11.0
    Checking pulldown-cmark v0.13.4
    Checking ess-compiler v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking hashbrown v0.15.5
    Checking tungstenite v0.30.0
    Checking ed25519 v3.0.0
    Checking chrono v0.4.45
    Checking curve25519-dalek v5.0.0
    Checking bigdecimal v0.4.11
    Checking minicbor v2.3.0
    Checking rand_chacha v0.9.0
    Checking zstd-safe v7.3.0
    Checking flate2 v1.1.10
    Checking idna v1.1.0
    Checking tempfile v3.27.0
    Checking ulid v3.0.0
    Checking ureq-proto v0.6.4
    Checking webpki-roots v1.0.9
    Checking winnow v1.0.4
    Checking utf8-zero v0.8.1
    Checking toml_parser v1.1.3+spec-1.1.0
    Checking ureq v3.4.2
    Checking b10x-substrate-wire v0.7.8 (https://github.com/beyond10x/substrate?rev=05695970b069f79e6678f2f02cbd78bbe5fa2a56#05695970)
    Checking aws-smithy-compression v0.2.0
    Checking derive_more v2.1.1
    Checking libsqlite3-sys v0.35.0
    Checking url v2.5.8
    Checking zstd v0.13.3
    Checking rand v0.9.5
    Checking nix v0.30.1
    Checking nix v0.31.3
    Checking aws-smithy-cbor v0.62.2
    Checking ed25519-dalek v3.0.0
    Checking tokio-tungstenite v0.30.0
    Checking ess-gen v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking hashlink v0.10.0
    Checking signal-hook-mio v0.2.5
    Checking sha2 v0.10.9
    Checking sha1 v0.10.7
    Checking aws-sdk-sts v1.119.0
    Checking aws-sdk-ssooidc v1.116.0
    Checking aws-sdk-sso v1.114.0
    Checking mantle-worker-model v0.0.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave8-codex-activation/generated/worker-model)
    Checking serde_urlencoded v0.7.1
    Checking serde_spanned v1.1.1
    Checking toml_datetime v0.7.5+spec-1.1.0
    Checking rustls-pemfile v2.2.0
    Checking toml_writer v1.1.2+spec-1.1.0
    Checking fallible-streaming-iterator v0.1.9
    Checking fallible-iterator v0.3.0
    Checking winnow v0.7.15
    Checking base64 v0.22.1
    Checking rusqlite v0.37.0
    Checking toml v0.9.12+spec-1.1.0
    Checking b10x-substrate-sdk v0.7.8 (https://github.com/beyond10x/substrate?rev=05695970b069f79e6678f2f02cbd78bbe5fa2a56#05695970)
    Checking aws-config v1.12.0
    Checking mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave8-codex-activation/crates/mantle-worker)
    Checking ess-conformance v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking crossterm v0.29.0
    Checking aws-sdk-cloudwatch v1.134.0
    Checking ulid v1.2.1
    Checking aws-sdk-iam v1.128.0
    Checking aws-sdk-ec2 v1.267.0
    Checking mantle-conformance v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave8-codex-activation/crates/mantle-conformance)
    Checking mantle-egress v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave8-codex-activation/crates/mantle-egress)
    Checking mantle v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave8-codex-activation/crates/mantle)
    Finished `dev` profile [unoptimized] target(s) in 2m 55s
