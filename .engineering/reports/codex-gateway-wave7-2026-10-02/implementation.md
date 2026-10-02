unit: story:codex-gateway-destinations — Fixed Codex gateway destinations
verdict: green
cases: executed 34→34 Rust (inherited workspace baseline), 66→69 native; red 2 Rust / 2 native
origin: n/a
wrote-outside-worktree: ~/.cache/mantle-wave7/scratch-gateway; /dev/shm/mantle-wave7-target-gateway; worktree lease metadata
needs-coordinator: no

The Rust package count is unchanged because the existing exact-default assertion was strengthened and the existing native bridge now executes three additional authored scenarios. Native execution increased 66→69. This is not a claim of three new Rust functions. The baseline Rust counts below come from the inherited wave6 full-workspace runner, not an identical package-only baseline invocation; the actual test-first command executed 25 Rust tests both before and after the runtime edit, with its two failures removed. The coordinator owns the independent same-command baseline/treatment claim and full gate.

1. Unit and acceptance

GW01 adds exactly auth.openai.com:443 and chatgpt.com:443 while preserving the eight original entries; GW02 exercises the real handler, normalization, fully qualified DNS, pinned public dialing and exact opaque binary bytes; GW03 refuses wrong ports, suffixes, subdomains, lookalikes and unadmitted API/CDN names before DNS/dial; GW04 retains all existing address/parser/resource/shutdown defenses.

Scope confirmation (12 assigned paths; 11 changed):

| Classification | Path | Confirmation |
| --- | --- | --- |
| cited | crates/mantle-egress/src/allow.rs | Existing default list and assertion; production edit adds only two entries. |
| cited | spec/domains/egress.yaml | Existing typed CheckDefaultDestination allowed guard receives the same two values. |
| inferred | spec/scenarios/egress/codex-default-destinations.yaml | Confirmed existing CheckDefaultDestination seam; two new accepted destinations. |
| inferred | spec/scenarios/egress/codex-connect-destinations.yaml | Confirmed existing real Exchange fixture; four public/mixed-DNS and normalized-authority exchanges, exact binary bytes. |
| inferred | spec/scenarios/egress/codex-destination-refusals.yaml | Confirmed same real Exchange seam; twelve denied authorities observe empty DNS/dial and forwarding. |
| cited | spec/ess-inputs.yaml | Explicitly admits all three authored files, preserving every prior admission. |
| cited | spec/conformance-baseline.json | Egress floor 66→69 and three required scenario names; all prior names retained. |
| cited | spec/README.md | Documents policy and actual 336/105/231 inventory; component counts 218/69/49. |
| cited | generated/worker-model/Cargo.toml | Included in deterministic generation; byte-identical, therefore unchanged. |
| cited | generated/worker-model/types.rs | Regenerated through pinned ESS0.50; only actual projection/provenance diff retained. |
| cited | generated/worker-model/source.schema.json | Regenerated through pinned ESS0.50. |
| cited | generated/worker-model/types-report.json | Regenerated through pinned ESS0.50. |

No inferred path was wrong. Existing crates/mantle-egress/src/conformance.rs was read and remains unchanged: its real TCP handler fixture already records all required observations. The mechanism measurement is the actual red run after authoring contracts/assertions but before adding runtime entries. Changing only those two runtime values made the same cases green. Generated ownership was adopted against a reference generated from the exact opening HEAD spec, then the four existing roots were regenerated. No generated output was hand edited.

2. Actual diff

Tracked diff stat follows; git omits three untracked authored YAML files from this statistic. The status output explicitly includes them.
 crates/mantle-egress/src/allow.rs         | 24 ++++++++++++++++++++++--
 generated/worker-model/source.schema.json |  2 +-
 generated/worker-model/types-report.json  |  4 ++--
 generated/worker-model/types.rs           |  4 ++--
 spec/README.md                            | 16 +++++++++++++---
 spec/conformance-baseline.json            |  7 +++++--
 spec/domains/egress.yaml                  |  2 +-
 spec/ess-inputs.yaml                      |  3 +++
 8 files changed, 49 insertions(+), 13 deletions(-)
 M crates/mantle-egress/src/allow.rs
 M generated/worker-model/source.schema.json
 M generated/worker-model/types-report.json
 M generated/worker-model/types.rs
 M spec/README.md
 M spec/conformance-baseline.json
 M spec/domains/egress.yaml
 M spec/ess-inputs.yaml
?? spec/scenarios/egress/codex-connect-destinations.yaml
?? spec/scenarios/egress/codex-default-destinations.yaml
?? spec/scenarios/egress/codex-destination-refusals.yaml

3. Actual red command and complete output

All cargo commands use the assigned TMPDIR/MANTLE_TEST_SCRATCH under scratch, CARGO_TARGET_DIR=/dev/shm/mantle-wave7-target-gateway, CARGO_BUILD_JOBS=2, CARGO_INCREMENTAL=0, and CARGO_PROFILE_DEV_DEBUG=CARGO_PROFILE_TEST_DEBUG=0. The brief explicitly authorizes the isolated target and private scratch instead of the generic procedure defaults. Configured sccache remained enabled.

Command: cargo test --locked -p mantle-egress --lib -- --nocapture
Exit: 101. Rust: 23 passed, 2 failed, 25 executed. Native: 67 passed, 2 failed, 69 executed. The exact-list assertion observes 8 versus 10; both new acceptance/CONNECT scenarios observe real denied outcomes. Complete output is appended below mechanically, abbreviating only the personal home prefix as ~ for portability. Raw logs remain unchanged beside the original private report; this retained copy also uses portable wording in this sentence.

4. Green commands, counts and outputs

- cargo test --locked -p mantle-egress -- --nocapture: exit 0. Lib executed 25→25; binary 0→0; adversary 6→6; refusal 3→3; doctests 0→0. Baseline is inherited wave6 workspace output, retained in inherited-egress-baseline.log; current full package output is tests.log. Total 34→34. The library lane includes the same test-first cases.
- Native component suite: executed 66→69, exit 0; current report 69 passed, 0 failed/error/skipped/unsupported. Two new defaults, four real CONNECT exchanges and twelve pre-DNS refusals run across three authored scenarios. Unchanged component-external inventory reported during synthesis is not execution failure.
- cargo clippy --locked -p mantle-egress --all-targets -- -D warnings: exit 0.
- cargo fmt --check: exit 0; no output.
- ess specify validate --path spec: exit 0; 7 files, 231 authored scenarios valid.
- ess verify conform synthesize --path spec --scenarios spec --suite-format 5 --out <scratch>/complete-suite.json: exit 0; 336 selected, 231 authored, 0 refusals. This is inventory synthesis only; this worker did not execute all components.
- cargo test --locked -p mantle-worker generated_model_has_no_drift -- --nocapture: exit 0; executed 1, 27 filtered; binary executed 0. Inherited full-worker suite includes the same drift test, but no equivalent isolated-filter baseline was run by this unit.
- git diff --check: exit 0, no output.

The first specification validation caught duplicate timeline instants in new authored steps (ESS-AUTHOR-023); the timestamps were made sequential, then validation passed before the runtime policy edit. No assertion was weakened or skipped.

5. Deliberate boundaries

No fixture extension was needed. No change to protocol parsing, DNS/address classification, relay, resource bounds, auth, launcher capture, Substrate, credentials or worker deployment. Gateway policy remains worker-shared. Real controlled local TCP exchanges establish handler behavior, not live OpenAI authentication or end-to-end Codex privacy. No AEP mutation, staging or commit. Coordinator owns independent attack, same-command startup comparison, full workspace gate and story delivery.

6. Outside paths and handoff

Assigned scratch: ~/.cache/mantle-wave7/scratch-gateway (all retained files enumerated in outside-paths.txt, including raw red/green reports, generated ownership/reference, declarative scenario construction intermediates and this report).
Assigned task-exclusive compiler output: /dev/shm/mantle-wave7-target-gateway. It is retained for coordinator cleanup; no build directory or managed tree was removed.
Worktree CLI lease metadata for codex-mantle-wave7-gateway-implementor was acquired/renewed and released; session-end output is lease-ended.log. Ordinary configured Cargo/sccache caches may have been populated by these commands; none were manually edited or removed.
Tree: ~/.local/state/worktree/trees/b10x/mantle/mantle-wave7-codex-gateway, branch impl/codex-gateway-destinations, opening8046b4c2f6f72050bf049b1932e39442000b8d06. No commit. Next owner: coordinator, for adversary and integration. All assigned source edits are complete and lease ended.

Appendix: complete mechanically appended command outputs (home prefix normalized only)

### red.log

```text
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.26
   Compiling serde_core v1.0.229
   Compiling syn v3.0.6
   Compiling zmij v1.0.23
   Compiling serde v1.0.229
   Compiling syn v2.0.119
   Compiling serde_derive v1.0.229
   Compiling libc v0.2.189
   Compiling memchr v2.8.3
   Compiling itoa v1.0.18
   Compiling serde_json v1.0.151
   Compiling typenum v1.20.1
   Compiling serde_derive_internals v0.29.1
   Compiling hybrid-array v0.4.15
   Compiling schemars v0.8.22
   Compiling equivalent v1.0.2
   Compiling hashbrown v0.17.1
   Compiling thiserror v2.0.21
   Compiling cfg-if v1.0.5
   Compiling indexmap v2.14.2
   Compiling schemars_derive v0.8.22
   Compiling thiserror-impl v2.0.21
   Compiling unsafe-libyaml v0.2.11
   Compiling dyn-clone v1.0.20
   Compiling ryu v1.0.23
   Compiling serde_yaml v0.9.34+deprecated
   Compiling crypto-common v0.2.2
   Compiling block-buffer v0.12.1
   Compiling const-oid v0.10.2
   Compiling ess-primitives v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling digest v0.11.3
   Compiling pulldown-cmark v0.13.4
   Compiling utf8parse v0.2.2
   Compiling cpufeatures v0.3.1
   Compiling sha2 v0.11.0
   Compiling ess-domain v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling anstyle-parse v1.0.0
   Compiling anstyle v1.0.14
   Compiling is_terminal_polyfill v1.70.2
   Compiling unicase v2.9.0
   Compiling pulldown-cmark-escape v0.11.0
   Compiling colorchoice v1.0.5
   Compiling anstyle-query v1.1.5
   Compiling parking_lot_core v0.9.12
   Compiling bitflags v2.13.2
   Compiling anstream v1.0.0
   Compiling heck v0.5.0
   Compiling smallvec v1.16.2
   Compiling strsim v0.11.1
   Compiling clap_lex v1.1.1
   Compiling anyhow v1.0.104
   Compiling scopeguard v1.2.0
   Compiling lock_api v0.4.14
   Compiling clap_builder v4.6.7
   Compiling clap_derive v4.6.7
   Compiling errno v0.3.14
   Compiling ess-compiler v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling signal-hook-registry v1.4.8
   Compiling clap v4.6.7
   Compiling parking_lot v0.12.5
   Compiling socket2 v0.6.5
   Compiling mio v1.2.3
   Compiling tokio-macros v2.7.2
   Compiling base64 v0.22.1
   Compiling pin-project-lite v0.2.17
   Compiling bytes v1.12.1
   Compiling ess-gen v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling tokio v1.53.1
   Compiling ess-conformance v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling mantle-conformance v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave7-codex-gateway/crates/mantle-conformance)
   Compiling mantle-egress v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave7-codex-gateway/crates/mantle-egress)
    Finished `test` profile [unoptimized] target(s) in 33.86s
     Running unittests src/lib.rs (/dev/shm/mantle-wave7-target-gateway/debug/deps/mantle_egress-d85058ffd944d7d9)

running 25 tests

thread 'allow::tests::default_list_has_ten_exact_hosts_on_443' (1964475) panicked at crates/mantle-egress/src/allow.rs:168:9:
assertion `left == right` failed
  left: 8
 right: 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test addr::tests::public_addresses_are_permitted ... ok
test addr::tests::refused_ipv4_ranges ... ok
test addr::tests::refused_ipv6_ranges ... ok
test allow::tests::allow_entry_rejects_ip_literal ... ok
test allow::tests::ip_literals_are_refused ... ok
test allow::tests::malformed_targets ... ok
test allow::tests::default_list_has_ten_exact_hosts_on_443 ... FAILED
test allow::tests::match_is_case_insensitive ... ok
test allow::tests::match_is_exact_not_suffix ... ok
test allow::tests::resolver_name_is_fully_qualified ... ok
test allow::tests::port_must_match_exactly ... ok
test allow::tests::trailing_dot_is_stripped ... ok
test head::tests::connect_without_headers ... ok
test head::tests::malformed_heads ... ok
test head::tests::valid_connect ... ok
test head::tests::read_head_keeps_bytes_past_the_head ... ok
test head::tests::read_head_closed_early ... ok
test head::tests::read_head_terminator_split_across_reads ... ok
test head::tests::wrong_method ... ok
test log::tests::formats_known_instants ... ok
test proxy::tests::response_shape ... ok
test head::tests::read_head_oversize ... ok
test head::tests::read_head_exactly_at_cap ... ok
test head::tests::read_head_slow_times_out ... ok
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
69 selected scenario(s), 63 authored source(s), 0 refusal occurrence(s)

2026-10-02T16:47:45.606Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=0
2026-10-02T16:47:45.615Z shutting down: no longer accepting connections
2026-10-02T16:47:45.615Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.615Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.615Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=9
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=2
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=2
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=2
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=2
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.616Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.617Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.617Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.617Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=1
2026-10-02T16:47:45.676Z drain timeout reached; closing open tunnels
2026-10-02T16:47:45.678Z shutting down: no longer accepting connections
2026-10-02T16:47:45.739Z drain timeout reached; closing open tunnels
2026-10-02T16:47:45.739Z dest=- outcome=refused:client-closed up=0 down=0 ms=0
2026-10-02T16:47:45.739Z shutting down: no longer accepting connections
2026-10-02T16:47:45.742Z dest=github.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T16:47:45.742Z shutting down: no longer accepting connections
2026-10-02T16:47:45.742Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T16:47:45.742Z shutting down: no longer accepting connections
2026-10-02T16:47:45.742Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T16:47:45.742Z shutting down: no longer accepting connections
2026-10-02T16:47:45.743Z dest=- outcome=refused:client-closed up=0 down=0 ms=0
2026-10-02T16:47:45.743Z shutting down: no longer accepting connections
2026-10-02T16:47:45.743Z dest=auth.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.743Z shutting down: no longer accepting connections
2026-10-02T16:47:45.744Z dest=chatgpt.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.744Z shutting down: no longer accepting connections
2026-10-02T16:47:45.744Z dest=auth.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.744Z shutting down: no longer accepting connections
2026-10-02T16:47:45.744Z dest=chatgpt.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.744Z shutting down: no longer accepting connections
2026-10-02T16:47:45.745Z dest=auth.openai.com:80 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.745Z shutting down: no longer accepting connections
2026-10-02T16:47:45.745Z dest=auth.openai.com:8443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.745Z shutting down: no longer accepting connections
2026-10-02T16:47:45.746Z dest=chatgpt.com:80 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.746Z shutting down: no longer accepting connections
2026-10-02T16:47:45.746Z dest=chatgpt.com:8443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.746Z shutting down: no longer accepting connections
2026-10-02T16:47:45.746Z dest=auth.openai.com.attacker.test:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.746Z shutting down: no longer accepting connections
2026-10-02T16:47:45.747Z dest=chatgpt.com.attacker.test:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.747Z shutting down: no longer accepting connections
2026-10-02T16:47:45.747Z dest=evil.auth.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.747Z shutting down: no longer accepting connections
2026-10-02T16:47:45.747Z dest=evil.chatgpt.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.748Z shutting down: no longer accepting connections
2026-10-02T16:47:45.748Z dest=auth-openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.748Z shutting down: no longer accepting connections
2026-10-02T16:47:45.748Z dest=chatgpt-com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.748Z shutting down: no longer accepting connections
2026-10-02T16:47:45.749Z dest=api.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.749Z shutting down: no longer accepting connections
2026-10-02T16:47:45.749Z dest=cdn.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:45.749Z shutting down: no longer accepting connections
2026-10-02T16:47:45.749Z dest=github.com:443 outcome=refused:connect-failed up=0 down=0 ms=0
2026-10-02T16:47:45.749Z shutting down: no longer accepting connections
2026-10-02T16:47:45.750Z dest=github.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T16:47:45.750Z shutting down: no longer accepting connections
2026-10-02T16:47:45.812Z dest=github.com:443 outcome=refused:connect-timeout up=0 down=0 ms=61
2026-10-02T16:47:45.812Z shutting down: no longer accepting connections
2026-10-02T16:47:45.813Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=0
2026-10-02T16:47:45.815Z shutting down: no longer accepting connections
2026-10-02T16:47:45.815Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.815Z dest=- outcome=refused:client-closed up=0 down=0 ms=2
2026-10-02T16:47:45.815Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.815Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.815Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.815Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.815Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:47:45.816Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=1
2026-10-02T16:47:45.876Z drain timeout reached; closing open tunnels
2026-10-02T16:47:45.938Z shutting down: no longer accepting connections
2026-10-02T16:47:46.039Z drain timeout reached; closing open tunnels
2026-10-02T16:47:46.101Z dest=github.com:443 outcome=allowed end=idle up=0 down=0 ms=61
2026-10-02T16:47:46.101Z shutting down: no longer accepting connections
2026-10-02T16:47:46.102Z dest=169.254.169.254 outcome=refused:ip-literal up=0 down=0 ms=0
2026-10-02T16:47:46.102Z shutting down: no longer accepting connections
2026-10-02T16:47:46.102Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T16:47:46.102Z shutting down: no longer accepting connections
2026-10-02T16:47:46.103Z dest=github.com:443 outcome=allowed end=eof up=4 down=4 ms=0
2026-10-02T16:47:46.103Z shutting down: no longer accepting connections
2026-10-02T16:47:46.103Z dest=- outcome=refused:method up=0 down=0 ms=0
2026-10-02T16:47:46.103Z shutting down: no longer accepting connections
2026-10-02T16:47:46.104Z dest=- outcome=refused:head-too-large up=0 down=0 ms=0
2026-10-02T16:47:46.104Z shutting down: no longer accepting connections
2026-10-02T16:47:46.104Z dest=github.com:80 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:46.104Z shutting down: no longer accepting connections
2026-10-02T16:47:46.105Z dest=github.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T16:47:46.105Z shutting down: no longer accepting connections
2026-10-02T16:47:46.105Z dest=github.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T16:47:46.105Z shutting down: no longer accepting connections
2026-10-02T16:47:46.105Z dest=github.com:443 outcome=refused:non-public-address up=0 down=0 ms=0
2026-10-02T16:47:46.105Z shutting down: no longer accepting connections
2026-10-02T16:47:46.167Z dest=github.com:443 outcome=refused:dns-failed up=0 down=0 ms=61
2026-10-02T16:47:46.167Z shutting down: no longer accepting connections
2026-10-02T16:47:46.168Z shutting down: no longer accepting connections
2026-10-02T16:47:46.229Z drain timeout reached; closing open tunnels
2026-10-02T16:47:46.230Z dest=github.com.attacker.test:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:47:46.230Z shutting down: no longer accepting connections
2026-10-02T16:47:46.230Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T16:47:46.230Z shutting down: no longer accepting connections
Error: ESS failures: [
    ScenarioResult {
        scenario: Authored {
            domain: DomainRef(
                QualifiedName(mantle.egress),
            ),
            name: AuthoredName(
                "codex-connect-destinations",
            ),
        },
        purpose: "GW02: real CONNECT normalizes both fixed hosts, pins public dialing and preserves binary bytes",
        status: Failed,
        checks: [
            CheckResult {
                code: Outcome,
                about: "outcome mantle.egress.Exchange/returned",
                status: Passed,
                diagnostic: None,
            },
            CheckResult {
                code: Payload,
                about: "direct response mantle.egress.Exchange",
                status: Failed,
                diagnostic: Some(
                    Diagnostic {
                        code: Payload,
                        scenario: Authored {
                            domain: DomainRef(
                                QualifiedName(mantle.egress),
                            ),
                            name: AuthoredName(
                                "codex-connect-destinations",
                            ),
                        },
                        source: [
                            Command {
                                name: CommandRef(
                                    QualifiedName(mantle.egress.Exchange),
                                ),
                            },
                        ],
                        input: None,
                        expected: [
                            "actual typed return satisfies its complete schema and authored literals",
                        ],
                        observed: [
                            "response field body differs from its declared literal",
                        ],
                    },
                ),
            },
            CheckResult {
                code: Outcome,
                about: "outcome mantle.egress.Exchange/returned",
                status: Passed,
                diagnostic: None,
            },
            CheckResult {
                code: Payload,
                about: "direct response mantle.egress.Exchange",
                status: Failed,
                diagnostic: Some(
                    Diagnostic {
                        code: Payload,
                        scenario: Authored {
                            domain: DomainRef(
                                QualifiedName(mantle.egress),
                            ),
                            name: AuthoredName(
                                "codex-connect-destinations",
                            ),
                        },
                        source: [
                            Command {
                                name: CommandRef(
                                    QualifiedName(mantle.egress.Exchange),
                                ),
                            },
                        ],
                        input: None,
                        expected: [
                            "actual typed return satisfies its complete schema and authored literals",
                        ],
                        observed: [
                            "response field body differs from its declared literal",
                        ],
                    },
                ),
            },
            CheckResult {
                code: Outcome,
                about: "outcome mantle.egress.Exchange/returned",
                status: Passed,
                diagnostic: None,
            },
            CheckResult {
                code: Payload,
                about: "direct response mantle.egress.Exchange",
                status: Failed,
                diagnostic: Some(
                    Diagnostic {
                        code: Payload,
                        scenario: Authored {
                            domain: DomainRef(
                                QualifiedName(mantle.egress),
                            ),
                            name: AuthoredName(
                                "codex-connect-destinations",
                            ),
                        },
                        source: [
                            Command {
                                name: CommandRef(
                                    QualifiedName(mantle.egress.Exchange),
                                ),
                            },
                        ],
                        input: None,
                        expected: [
                            "actual typed return satisfies its complete schema and authored literals",
                        ],
                        observed: [
                            "response field body differs from its declared literal",
                        ],
                    },
                ),
            },
            CheckResult {
                code: Outcome,
                about: "outcome mantle.egress.Exchange/returned",
                status: Passed,
                diagnostic: None,
            },
            CheckResult {
                code: Payload,
                about: "direct response mantle.egress.Exchange",
                status: Failed,
                diagnostic: Some(
                    Diagnostic {
                        code: Payload,
                        scenario: Authored {
                            domain: DomainRef(
                                QualifiedName(mantle.egress),
                            ),
                            name: AuthoredName(
                                "codex-connect-destinations",
                            ),
                        },
                        source: [
                            Command {
                                name: CommandRef(
                                    QualifiedName(mantle.egress.Exchange),
                                ),
                            },
                        ],
                        input: None,
                        expected: [
                            "actual typed return satisfies its complete schema and authored literals",
                        ],
                        observed: [
                            "response field body differs from its declared literal",
                        ],
                    },
                ),
            },
        ],
        duration_ms: 1,
    },
    ScenarioResult {
        scenario: Authored {
            domain: DomainRef(
                QualifiedName(mantle.egress),
            ),
            name: AuthoredName(
                "codex-default-destinations",
            ),
        },
        purpose: "GW01: exactly the two selected subscription destinations use the production default policy",
        status: Failed,
        checks: [
            CheckResult {
                code: Outcome,
                about: "outcome mantle.egress.CheckDefaultDestination/allowed",
                status: Failed,
                diagnostic: Some(
                    Diagnostic {
                        code: Outcome,
                        scenario: Authored {
                            domain: DomainRef(
                                QualifiedName(mantle.egress),
                            ),
                            name: AuthoredName(
                                "codex-default-destinations",
                            ),
                        },
                        source: [
                            Outcome {
                                name: OutcomeRef {
                                    command: CommandRef(
                                        QualifiedName(mantle.egress.CheckDefaultDestination),
                                    ),
                                    outcome: OutcomeName(allowed),
                                },
                            },
                        ],
                        input: Some(
                            "mantle.egress.CheckDefaultDestination(host = \"auth.openai.com\", port = 443.0)",
                        ),
                        expected: [
                            "outcome = allowed",
                        ],
                        observed: [
                            "outcome = denied",
                        ],
                    },
                ),
            },
            CheckResult {
                code: Payload,
                about: "direct response mantle.egress.CheckDefaultDestination",
                status: Failed,
                diagnostic: Some(
                    Diagnostic {
                        code: Payload,
                        scenario: Authored {
                            domain: DomainRef(
                                QualifiedName(mantle.egress),
                            ),
                            name: AuthoredName(
                                "codex-default-destinations",
                            ),
                        },
                        source: [
                            Command {
                                name: CommandRef(
                                    QualifiedName(mantle.egress.CheckDefaultDestination),
                                ),
                            },
                        ],
                        input: None,
                        expected: [
                            "actual typed return satisfies its complete schema and authored literals",
                        ],
                        observed: [
                            "direct response names a different command or outcome",
                        ],
                    },
                ),
            },
            CheckResult {
                code: Outcome,
                about: "outcome mantle.egress.CheckDefaultDestination/allowed",
                status: Failed,
                diagnostic: Some(
                    Diagnostic {
                        code: Outcome,
                        scenario: Authored {
                            domain: DomainRef(
                                QualifiedName(mantle.egress),
                            ),
                            name: AuthoredName(
                                "codex-default-destinations",
                            ),
                        },
                        source: [
                            Outcome {
                                name: OutcomeRef {
                                    command: CommandRef(
                                        QualifiedName(mantle.egress.CheckDefaultDestination),
                                    ),
                                    outcome: OutcomeName(allowed),
                                },
                            },
                        ],
                        input: Some(
                            "mantle.egress.CheckDefaultDestination(host = \"chatgpt.com\", port = 443.0)",
                        ),
                        expected: [
                            "outcome = allowed",
                        ],
                        observed: [
                            "outcome = denied",
                        ],
                    },
                ),
            },
            CheckResult {
                code: Payload,
                about: "direct response mantle.egress.CheckDefaultDestination",
                status: Failed,
                diagnostic: Some(
                    Diagnostic {
                        code: Payload,
                        scenario: Authored {
                            domain: DomainRef(
                                QualifiedName(mantle.egress),
                            ),
                            name: AuthoredName(
                                "codex-default-destinations",
                            ),
                        },
                        source: [
                            Command {
                                name: CommandRef(
                                    QualifiedName(mantle.egress.CheckDefaultDestination),
                                ),
                            },
                        ],
                        input: None,
                        expected: [
                            "actual typed return satisfies its complete schema and authored literals",
                        ],
                        observed: [
                            "direct response names a different command or outcome",
                        ],
                    },
                ),
            },
        ],
        duration_ms: 1,
    },
]
test proxy::conformance::ess_egress_conformance ... FAILED

failures:

failures:
    allow::tests::default_list_has_ten_exact_hosts_on_443
    proxy::conformance::ess_egress_conformance

test result: FAILED. 23 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.79s

error: test failed, to rerun pass `-p mantle-egress --lib`

```

### tests.log

```text
   Compiling mantle-egress v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave7-codex-gateway/crates/mantle-egress)
    Finished `test` profile [unoptimized] target(s) in 1.36s
     Running unittests src/lib.rs (/dev/shm/mantle-wave7-target-gateway/debug/deps/mantle_egress-d85058ffd944d7d9)

running 25 tests
test addr::tests::refused_ipv4_ranges ... ok
test addr::tests::public_addresses_are_permitted ... ok
test addr::tests::refused_ipv6_ranges ... ok
test allow::tests::allow_entry_rejects_ip_literal ... ok
test allow::tests::ip_literals_are_refused ... ok
test allow::tests::malformed_targets ... ok
test allow::tests::default_list_has_ten_exact_hosts_on_443 ... ok
test allow::tests::match_is_case_insensitive ... ok
test allow::tests::match_is_exact_not_suffix ... ok
test allow::tests::port_must_match_exactly ... ok
test allow::tests::resolver_name_is_fully_qualified ... ok
test allow::tests::trailing_dot_is_stripped ... ok
test head::tests::connect_without_headers ... ok
test head::tests::malformed_heads ... ok
test head::tests::valid_connect ... ok
test head::tests::read_head_keeps_bytes_past_the_head ... ok
test head::tests::read_head_closed_early ... ok
test proxy::tests::response_shape ... ok
test log::tests::formats_known_instants ... ok
test head::tests::read_head_terminator_split_across_reads ... ok
test head::tests::wrong_method ... ok
test head::tests::read_head_oversize ... ok
test head::tests::read_head_exactly_at_cap ... ok
test head::tests::read_head_slow_times_out ... ok
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
69 selected scenario(s), 63 authored source(s), 0 refusal occurrence(s)

2026-10-02T16:48:48.247Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=0
2026-10-02T16:48:48.264Z shutting down: no longer accepting connections
2026-10-02T16:48:48.264Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.264Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.264Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.264Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.264Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.264Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.264Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.264Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.264Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.264Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.264Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=16
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.265Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.265Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=1
2026-10-02T16:48:48.325Z drain timeout reached; closing open tunnels
2026-10-02T16:48:48.326Z shutting down: no longer accepting connections
2026-10-02T16:48:48.387Z drain timeout reached; closing open tunnels
2026-10-02T16:48:48.388Z dest=- outcome=refused:client-closed up=0 down=0 ms=0
2026-10-02T16:48:48.388Z shutting down: no longer accepting connections
2026-10-02T16:48:48.390Z dest=github.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T16:48:48.390Z shutting down: no longer accepting connections
2026-10-02T16:48:48.391Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T16:48:48.391Z shutting down: no longer accepting connections
2026-10-02T16:48:48.391Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T16:48:48.391Z shutting down: no longer accepting connections
2026-10-02T16:48:48.392Z dest=- outcome=refused:client-closed up=0 down=0 ms=0
2026-10-02T16:48:48.392Z shutting down: no longer accepting connections
2026-10-02T16:48:48.392Z dest=auth.openai.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T16:48:48.392Z shutting down: no longer accepting connections
2026-10-02T16:48:48.393Z dest=chatgpt.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T16:48:48.393Z shutting down: no longer accepting connections
2026-10-02T16:48:48.393Z dest=auth.openai.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T16:48:48.393Z shutting down: no longer accepting connections
2026-10-02T16:48:48.394Z dest=chatgpt.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T16:48:48.394Z shutting down: no longer accepting connections
2026-10-02T16:48:48.394Z dest=auth.openai.com:80 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:48.394Z shutting down: no longer accepting connections
2026-10-02T16:48:48.394Z dest=auth.openai.com:8443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:48.395Z shutting down: no longer accepting connections
2026-10-02T16:48:48.395Z dest=chatgpt.com:80 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:48.395Z shutting down: no longer accepting connections
2026-10-02T16:48:48.395Z dest=chatgpt.com:8443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:48.395Z shutting down: no longer accepting connections
2026-10-02T16:48:48.396Z dest=auth.openai.com.attacker.test:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:48.396Z shutting down: no longer accepting connections
2026-10-02T16:48:48.396Z dest=chatgpt.com.attacker.test:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:48.396Z shutting down: no longer accepting connections
2026-10-02T16:48:48.396Z dest=evil.auth.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:48.396Z shutting down: no longer accepting connections
2026-10-02T16:48:48.397Z dest=evil.chatgpt.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:48.397Z shutting down: no longer accepting connections
2026-10-02T16:48:48.397Z dest=auth-openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:48.397Z shutting down: no longer accepting connections
2026-10-02T16:48:48.398Z dest=chatgpt-com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:48.398Z shutting down: no longer accepting connections
2026-10-02T16:48:48.398Z dest=api.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:48.398Z shutting down: no longer accepting connections
2026-10-02T16:48:48.398Z dest=cdn.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:48.398Z shutting down: no longer accepting connections
2026-10-02T16:48:48.399Z dest=github.com:443 outcome=refused:connect-failed up=0 down=0 ms=0
2026-10-02T16:48:48.399Z shutting down: no longer accepting connections
2026-10-02T16:48:48.399Z dest=github.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T16:48:48.399Z shutting down: no longer accepting connections
2026-10-02T16:48:48.461Z dest=github.com:443 outcome=refused:connect-timeout up=0 down=0 ms=61
2026-10-02T16:48:48.461Z shutting down: no longer accepting connections
2026-10-02T16:48:48.462Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=0
2026-10-02T16:48:48.464Z shutting down: no longer accepting connections
2026-10-02T16:48:48.464Z dest=- outcome=refused:client-closed up=0 down=0 ms=2
2026-10-02T16:48:48.464Z dest=- outcome=refused:client-closed up=0 down=0 ms=2
2026-10-02T16:48:48.464Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T16:48:48.465Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=1
2026-10-02T16:48:48.525Z drain timeout reached; closing open tunnels
2026-10-02T16:48:48.587Z shutting down: no longer accepting connections
2026-10-02T16:48:48.688Z drain timeout reached; closing open tunnels
2026-10-02T16:48:48.750Z dest=github.com:443 outcome=allowed end=idle up=0 down=0 ms=61
2026-10-02T16:48:48.750Z shutting down: no longer accepting connections
2026-10-02T16:48:48.751Z dest=169.254.169.254 outcome=refused:ip-literal up=0 down=0 ms=0
2026-10-02T16:48:48.751Z shutting down: no longer accepting connections
2026-10-02T16:48:48.751Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T16:48:48.751Z shutting down: no longer accepting connections
2026-10-02T16:48:48.752Z dest=github.com:443 outcome=allowed end=eof up=4 down=4 ms=0
2026-10-02T16:48:48.752Z shutting down: no longer accepting connections
2026-10-02T16:48:48.752Z dest=- outcome=refused:method up=0 down=0 ms=0
2026-10-02T16:48:48.752Z shutting down: no longer accepting connections
2026-10-02T16:48:48.753Z dest=- outcome=refused:head-too-large up=0 down=0 ms=0
2026-10-02T16:48:48.753Z shutting down: no longer accepting connections
2026-10-02T16:48:48.753Z dest=github.com:80 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:48.753Z shutting down: no longer accepting connections
2026-10-02T16:48:48.754Z dest=github.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T16:48:48.754Z shutting down: no longer accepting connections
2026-10-02T16:48:48.754Z dest=github.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T16:48:48.754Z shutting down: no longer accepting connections
2026-10-02T16:48:48.754Z dest=github.com:443 outcome=refused:non-public-address up=0 down=0 ms=0
2026-10-02T16:48:48.754Z shutting down: no longer accepting connections
2026-10-02T16:48:48.816Z dest=github.com:443 outcome=refused:dns-failed up=0 down=0 ms=61
2026-10-02T16:48:48.816Z shutting down: no longer accepting connections
2026-10-02T16:48:48.817Z shutting down: no longer accepting connections
2026-10-02T16:48:48.878Z drain timeout reached; closing open tunnels
2026-10-02T16:48:48.879Z dest=github.com.attacker.test:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:48.879Z shutting down: no longer accepting connections
2026-10-02T16:48:48.879Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T16:48:48.879Z shutting down: no longer accepting connections
mantle-egress: 69 passed, 0 failed, 0 unsupported
test proxy::conformance::ess_egress_conformance ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.81s

     Running unittests src/main.rs (/dev/shm/mantle-wave7-target-gateway/debug/deps/mantle_egress-d9ae47290ba24e26)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/adversary.rs (/dev/shm/mantle-wave7-target-gateway/debug/deps/adversary-2c7a92e6a4db14b8)

running 6 tests
test ipv4_benchmarking_and_documentation_ranges_are_refused ... ok
test ipv4_ietf_protocol_block_is_refused ... ok
test ipv6_local_use_nat64_prefix_embedding_metadata_is_refused ... ok
test ipv6_siit_translated_form_embedding_metadata_is_refused ... ok
test ipv6_non_global_unicast_space_is_refused ... ok
test silent_sockets_do_not_starve_a_well_formed_request ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

     Running tests/refusal.rs (/dev/shm/mantle-wave7-target-gateway/debug/deps/refusal-0356a4942c04b79d)

running 3 tests
2026-10-02T16:48:49.092Z dest=- outcome=refused:method up=0 down=0 ms=0
2026-10-02T16:48:49.092Z dest=example.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T16:48:49.092Z dest=github.com:80 outcome=refused:not-allowed up=0 down=0 ms=0
test refuses_non_connect ... ok
test refuses_unlisted_destination_with_403 ... ok
2026-10-02T16:48:49.092Z dest=localhost:43821 outcome=refused:non-public-address up=0 down=0 ms=0
test allowed_name_resolving_to_loopback_is_not_dialled ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

   Doc-tests mantle_egress

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


```

### clippy.log

```text
    Checking serde_core v1.0.229
    Checking cfg-if v1.0.5
    Checking libc v0.2.189
    Checking itoa v1.0.18
    Checking memchr v2.8.3
    Checking typenum v1.20.1
    Checking serde v1.0.229
    Checking zmij v1.0.23
    Checking hashbrown v0.17.1
    Checking hybrid-array v0.4.15
    Checking equivalent v1.0.2
    Checking serde_json v1.0.151
    Checking indexmap v2.14.2
    Checking utf8parse v0.2.2
    Checking ryu v1.0.23
    Checking unsafe-libyaml v0.2.11
    Checking dyn-clone v1.0.20
    Checking schemars v0.8.22
    Checking serde_yaml v0.9.34+deprecated
    Checking anstyle-parse v1.0.0
    Checking thiserror v2.0.21
    Checking block-buffer v0.12.1
    Checking crypto-common v0.2.2
    Checking colorchoice v1.0.5
    Checking const-oid v0.10.2
    Checking is_terminal_polyfill v1.70.2
    Checking anstyle-query v1.1.5
    Checking anstyle v1.0.14
    Checking digest v0.11.3
    Checking anstream v1.0.0
    Checking ess-primitives v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking cpufeatures v0.3.1
    Checking strsim v0.11.1
    Checking clap_lex v1.1.1
    Checking smallvec v1.16.2
    Checking scopeguard v1.2.0
    Checking parking_lot_core v0.9.12
    Checking lock_api v0.4.14
    Checking ess-domain v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking clap_builder v4.6.7
    Checking sha2 v0.11.0
    Checking errno v0.3.14
    Checking pulldown-cmark-escape v0.11.0
    Checking bitflags v2.13.2
    Checking unicase v2.9.0
    Checking pulldown-cmark v0.13.4
    Checking signal-hook-registry v1.4.8
    Checking clap v4.6.7
    Checking parking_lot v0.12.5
    Checking anyhow v1.0.104
    Checking socket2 v0.6.5
    Checking mio v1.2.3
    Checking bytes v1.12.1
    Checking ess-compiler v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking pin-project-lite v0.2.17
    Checking tokio v1.53.1
    Checking ess-gen v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking ess-conformance v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking base64 v0.22.1
    Checking mantle-egress v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave7-codex-gateway/crates/mantle-egress)
    Checking mantle-conformance v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave7-codex-gateway/crates/mantle-conformance)
    Finished `dev` profile [unoptimized] target(s) in 17.67s

```

### fmt.log

```text

```

### spec.log

```text
mantle v1 — 7 file(s), 231 scenario(s), valid

```

### inventory.log

```text
336 selected scenario(s), 231 authored source(s), 0 refusal occurrence(s)

```

### generated-drift.log

```text
   Compiling find-msvc-tools v0.1.14
   Compiling jobserver v0.1.35
   Compiling shlex v2.0.1
   Compiling libc v0.2.189
   Compiling cc v1.5.1
   Compiling version_check v0.9.5
   Compiling generic-array v0.14.7
   Compiling ring v0.17.14
   Compiling pkg-config v0.3.34
   Compiling zstd-sys v2.1.0+zstd.1.5.7
   Compiling typenum v1.20.1
   Compiling zeroize v1.9.0
   Compiling rustls-pki-types v1.15.1
   Compiling getrandom v0.2.17
   Compiling untrusted v0.9.0
   Compiling httparse v1.10.1
   Compiling once_cell v1.21.4
   Compiling getrandom v0.4.3
   Compiling cfg_aliases v0.2.2
   Compiling serde_json v1.0.151
   Compiling bitflags v2.13.2
   Compiling rustix v1.1.5
   Compiling log v0.4.34
   Compiling rustls v0.23.45
   Compiling zstd-safe v7.3.0
   Compiling http v1.5.0
   Compiling nix v0.30.1
   Compiling rustls-webpki v0.103.15
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.7
   Compiling errno v0.3.14
   Compiling base64 v0.23.1
   Compiling subtle v2.6.1
   Compiling linux-raw-sys v0.12.1
   Compiling memchr v2.8.3
   Compiling signal-hook v0.3.18
   Compiling ureq-proto v0.6.4
   Compiling signal-hook-registry v1.4.8
   Compiling digest v0.10.7
   Compiling webpki-roots v1.0.9
   Compiling fastrand v2.5.0
   Compiling percent-encoding v2.3.2
   Compiling utf8-zero v0.8.1
   Compiling cpufeatures v0.2.17
   Compiling sha2 v0.10.9
   Compiling tempfile v3.27.0
   Compiling ureq v3.4.2
   Compiling zstd v0.13.3
   Compiling mantle-worker-model v0.0.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave7-codex-gateway/generated/worker-model)
   Compiling mantle-worker v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave7-codex-gateway/crates/mantle-worker)
    Finished `test` profile [unoptimized] target(s) in 12.81s
     Running unittests src/lib.rs (/dev/shm/mantle-wave7-target-gateway/debug/deps/mantle_worker-52fb9b1b91688ca5)

running 1 test
test tests::generated_model_has_no_drift ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 27 filtered out; finished in 0.14s

     Running unittests src/main.rs (/dev/shm/mantle-wave7-target-gateway/debug/deps/mantle_worker-cf6823bc9926dda3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


```

### adoption.log

```text
4 model type(s), written to ~/.cache/mantle-wave7/scratch-gateway/reference-model; runtime obligations in types-report.json

```

### generated.log

```text
4 model type(s), written to generated/worker-model; runtime obligations in types-report.json

```

### lease-ended.log

```text
session-end ~/.local/state/worktree/trees/b10x/mantle/mantle-wave7-codex-gateway

```
