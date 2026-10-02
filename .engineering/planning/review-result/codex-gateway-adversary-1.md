---
format: aep.planning-md/3
id: review-result:codex-gateway-adversary-1
kind: review-result
status: active
title: Codex gateway adversary pass 1
relations:
- reviews: story:codex-gateway-destinations
revision: 1
---
unit: story:codex-gateway-destinations; mantle-wave7-codex-gateway at base 8046b4c2f6f72050bf049b1932e39442000b8d06; staged patch SHA256 92b2e94ba41ddae77bec4c64401fc08f5c6b308111188ca975c22b76c966a119
verdict: nothing found
cases: executed 34→36 Rust; 69→69 native; red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: 4 roots, expanded below
needs-coordinator: none

 crates/mantle-egress/src/conformance.rs | 77 +++++++++++++++++++++++++++++++++
 1 file changed, 77 insertions(+)

Only 77 added lines in the existing test-only module (proxy.rs:373–375); no old case, production code, adapter behavior, staged snapshot, specification or AEP artifact changed. Complete staged diff, story acceptance, authored scenarios and production callers were read first. Baseline count 34 comes from the implementor handoff, not a pre-attack run.

Two cases were written before any execution; each ran alone before the suite, passed on its first execution, and remains green. No fixture corrections or failed attempts occurred.

- `crates/mantle-egress/src/conformance.rs:261`: both new destinations refuse private/empty/failed/timed-out DNS with status 502 and no dialing or forwarded bytes. The same fixture admits mixed DNS only through the public socket and echoes the literal payload. Ten exchanges.
- `crates/mantle-egress/src/conformance.rs:289`: conflicting Host headers cannot admit wrong-port, suffix, double-dot, percent-encoded or userinfo authorities, or api.openai.com. Twelve refusals have no DNS/dial/forwarding. Two positive controls combine uppercase, terminal dot and zero-padded port normalization with a hostile Host header and 68 opaque bytes containing a fake CONNECT request. Exact fully qualified names, public dial address, captured and returned bytes are asserted.

Reachability: default CLI main.rs:47–52 constructs Allowlist::default_list and invokes serve; proxy.rs:39,227,311 reach the same handler and dial logic exercised by the unchanged exchange fixture at conformance.rs:47. Only DNS/upstream IO is substituted with local sockets. No external request, authenticated Codex session, deployment or per-agent isolation is claimed.

The package suite selected 27 library, 6 adversary and 3 refusal Rust tests (36 total); its native runner reported 69 passed, 0 failed, 0 unsupported. Package clippy, workspace formatting and diff whitespace checks passed. No full-workspace build or unrelated test gate was run. Findings: nothing found. No origin classification was needed.

Attacked and could not break: exact CONNECT authority ownership despite conflicting headers; both new hosts' private/DNS refusal paths; public-address pinning with mixed DNS; unchanged opaque binary relay after normalization. Retained native defense cases also passed. These observations do not establish live authentication endpoint sufficiency.

All commands used the assigned tree. Cargo environment: CARGO_TARGET_DIR=/dev/shm/mantle-wave7-target-gateway, CARGO_BUILD_JOBS=2, CARGO_PROFILE_DEV_DEBUG=0, CARGO_PROFILE_TEST_DEBUG=0, CARGO_INCREMENTAL=0, TMPDIR=~/.cache/mantle-wave7/scratch-gateway-adversary-1/tmp, MANTLE_TEST_SCRATCH=~/.cache/mantle-wave7/scratch-gateway-adversary-1/fixtures; machine-configured /usr/bin/sccache retained.

Outside-write inventory (four roots):
- `~/.cache/mantle-wave7/scratch-gateway-adversary-1/`: report.md, case-dns.log, case-authority.log, package.log, clippy.log, fmt.log, diff-check.log, diff-stat.log, test-additions.patch, lease-end.log, process-check.log; private tmp/ and fixtures/.
- `/dev/shm/mantle-wave7-target-gateway/`: assigned compiler/test artifacts, retained.
- `~/.cache/sccache/`: configured shared compiler cache.
- `~/.local/state/worktree/`: CLI-owned session lease metadata for codex-mantle-wave7-gateway-adversary-1; own lease ended successfully. No other lease was changed.

No matching owned cargo/test/proxy process remained at handoff. Source remains unstaged for coordinator inspection/integration; no commits or staging performed. Aggregate token/cost metrics unavailable.

Raw output follows, mechanically appended from private logs in execution order. Personal-home path normalization to ~ was applied throughout this report; the private logs retain exact operational paths. Exit statuses are captured by the invoking shell.

Command: `cargo test --locked -p mantle-egress --lib proxy::conformance::adversary_codex_dns_failures_never_dial_and_public_control_relays -- --exact --nocapture`
Log: `~/.cache/mantle-wave7/scratch-gateway-adversary-1/case-dns.log`

```text
   Compiling mantle-egress v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave7-codex-gateway/crates/mantle-egress)
    Finished `test` profile [unoptimized] target(s) in 0.95s
     Running unittests src/lib.rs (/dev/shm/mantle-wave7-target-gateway/debug/deps/mantle_egress-d85058ffd944d7d9)

running 1 test
2026-10-02T17:02:35.316Z dest=auth.openai.com:443 outcome=refused:non-public-address up=0 down=0 ms=0
2026-10-02T17:02:35.316Z shutting down: no longer accepting connections
2026-10-02T17:02:35.316Z dest=auth.openai.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T17:02:35.317Z shutting down: no longer accepting connections
2026-10-02T17:02:35.317Z dest=auth.openai.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T17:02:35.317Z shutting down: no longer accepting connections
2026-10-02T17:02:35.378Z dest=auth.openai.com:443 outcome=refused:dns-failed up=0 down=0 ms=61
2026-10-02T17:02:35.378Z shutting down: no longer accepting connections
2026-10-02T17:02:35.379Z dest=auth.openai.com:443 outcome=allowed end=eof up=11 down=11 ms=0
2026-10-02T17:02:35.379Z shutting down: no longer accepting connections
2026-10-02T17:02:35.379Z dest=chatgpt.com:443 outcome=refused:non-public-address up=0 down=0 ms=0
2026-10-02T17:02:35.379Z shutting down: no longer accepting connections
2026-10-02T17:02:35.379Z dest=chatgpt.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T17:02:35.380Z shutting down: no longer accepting connections
2026-10-02T17:02:35.380Z dest=chatgpt.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T17:02:35.380Z shutting down: no longer accepting connections
2026-10-02T17:02:35.441Z dest=chatgpt.com:443 outcome=refused:dns-failed up=0 down=0 ms=61
2026-10-02T17:02:35.441Z shutting down: no longer accepting connections
2026-10-02T17:02:35.442Z dest=chatgpt.com:443 outcome=allowed end=eof up=11 down=11 ms=0
2026-10-02T17:02:35.442Z shutting down: no longer accepting connections
test proxy::conformance::adversary_codex_dns_failures_never_dial_and_public_control_relays ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.13s


EXIT_STATUS=0

```

Command: `cargo test --locked -p mantle-egress --lib proxy::conformance::adversary_codex_connect_authority_owns_policy_and_opaque_payload -- --exact --nocapture`
Log: `~/.cache/mantle-wave7/scratch-gateway-adversary-1/case-authority.log`

```text
    Finished `test` profile [unoptimized] target(s) in 0.08s
     Running unittests src/lib.rs (/dev/shm/mantle-wave7-target-gateway/debug/deps/mantle_egress-d85058ffd944d7d9)

running 1 test
2026-10-02T17:03:09.551Z dest=auth.openai.com.attacker.test:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:09.551Z shutting down: no longer accepting connections
2026-10-02T17:03:09.551Z dest=auth.openai.com:444 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:09.552Z shutting down: no longer accepting connections
2026-10-02T17:03:09.552Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:09.552Z shutting down: no longer accepting connections
2026-10-02T17:03:09.552Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:09.552Z shutting down: no longer accepting connections
2026-10-02T17:03:09.552Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:09.552Z shutting down: no longer accepting connections
2026-10-02T17:03:09.553Z dest=api.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:09.553Z shutting down: no longer accepting connections
2026-10-02T17:03:09.553Z dest=auth.openai.com:443 outcome=allowed end=eof up=68 down=68 ms=0
2026-10-02T17:03:09.553Z shutting down: no longer accepting connections
2026-10-02T17:03:09.553Z dest=chatgpt.com.attacker.test:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:09.553Z shutting down: no longer accepting connections
2026-10-02T17:03:09.554Z dest=chatgpt.com:444 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:09.554Z shutting down: no longer accepting connections
2026-10-02T17:03:09.554Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:09.554Z shutting down: no longer accepting connections
2026-10-02T17:03:09.554Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:09.554Z shutting down: no longer accepting connections
2026-10-02T17:03:09.555Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:09.555Z shutting down: no longer accepting connections
2026-10-02T17:03:09.555Z dest=api.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:09.555Z shutting down: no longer accepting connections
2026-10-02T17:03:09.555Z dest=chatgpt.com:443 outcome=allowed end=eof up=68 down=68 ms=0
2026-10-02T17:03:09.555Z shutting down: no longer accepting connections
test proxy::conformance::adversary_codex_connect_authority_owns_policy_and_opaque_payload ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s


EXIT_STATUS=0

```

Command: `cargo test --locked -p mantle-egress -- --nocapture`
Log: `~/.cache/mantle-wave7/scratch-gateway-adversary-1/package.log`

```text
    Finished `test` profile [unoptimized] target(s) in 0.11s
     Running unittests src/lib.rs (/dev/shm/mantle-wave7-target-gateway/debug/deps/mantle_egress-d85058ffd944d7d9)

running 27 tests
test addr::tests::public_addresses_are_permitted ... ok
test addr::tests::refused_ipv4_ranges ... ok
test addr::tests::refused_ipv6_ranges ... ok
test allow::tests::allow_entry_rejects_ip_literal ... ok
test allow::tests::ip_literals_are_refused ... ok
test allow::tests::malformed_targets ... ok
test allow::tests::default_list_has_ten_exact_hosts_on_443 ... ok
test allow::tests::match_is_case_insensitive ... ok
test allow::tests::port_must_match_exactly ... ok
test allow::tests::match_is_exact_not_suffix ... ok
test allow::tests::resolver_name_is_fully_qualified ... ok
test allow::tests::trailing_dot_is_stripped ... ok
test head::tests::connect_without_headers ... ok
test head::tests::malformed_heads ... ok
test head::tests::read_head_closed_early ... ok
test head::tests::read_head_keeps_bytes_past_the_head ... ok
test head::tests::wrong_method ... ok
test head::tests::valid_connect ... ok
test log::tests::formats_known_instants ... ok
test head::tests::read_head_terminator_split_across_reads ... ok
test proxy::tests::response_shape ... ok
test head::tests::read_head_exactly_at_cap ... ok
test head::tests::read_head_oversize ... ok
2026-10-02T17:03:32.178Z dest=auth.openai.com.attacker.test:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.178Z dest=auth.openai.com:443 outcome=refused:non-public-address up=0 down=0 ms=0
2026-10-02T17:03:32.178Z shutting down: no longer accepting connections
2026-10-02T17:03:32.178Z shutting down: no longer accepting connections
2026-10-02T17:03:32.178Z dest=auth.openai.com:444 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.179Z dest=auth.openai.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T17:03:32.179Z shutting down: no longer accepting connections
2026-10-02T17:03:32.179Z shutting down: no longer accepting connections
2026-10-02T17:03:32.179Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:32.179Z dest=auth.openai.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T17:03:32.179Z shutting down: no longer accepting connections
2026-10-02T17:03:32.179Z shutting down: no longer accepting connections
2026-10-02T17:03:32.179Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:32.179Z shutting down: no longer accepting connections
2026-10-02T17:03:32.180Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:32.180Z shutting down: no longer accepting connections
2026-10-02T17:03:32.180Z dest=api.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.180Z shutting down: no longer accepting connections
2026-10-02T17:03:32.181Z dest=auth.openai.com:443 outcome=allowed end=eof up=68 down=68 ms=0
2026-10-02T17:03:32.181Z shutting down: no longer accepting connections
2026-10-02T17:03:32.181Z dest=chatgpt.com.attacker.test:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.181Z shutting down: no longer accepting connections
2026-10-02T17:03:32.181Z dest=chatgpt.com:444 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.181Z shutting down: no longer accepting connections
2026-10-02T17:03:32.182Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:32.182Z shutting down: no longer accepting connections
2026-10-02T17:03:32.182Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:32.182Z shutting down: no longer accepting connections
2026-10-02T17:03:32.182Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:32.183Z shutting down: no longer accepting connections
2026-10-02T17:03:32.183Z dest=api.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.183Z shutting down: no longer accepting connections
2026-10-02T17:03:32.183Z dest=chatgpt.com:443 outcome=allowed end=eof up=68 down=68 ms=0
2026-10-02T17:03:32.184Z shutting down: no longer accepting connections
test proxy::conformance::adversary_codex_connect_authority_owns_policy_and_opaque_payload ... ok
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

2026-10-02T17:03:32.240Z dest=auth.openai.com:443 outcome=refused:dns-failed up=0 down=0 ms=61
2026-10-02T17:03:32.240Z shutting down: no longer accepting connections
2026-10-02T17:03:32.241Z dest=auth.openai.com:443 outcome=allowed end=eof up=11 down=11 ms=0
2026-10-02T17:03:32.241Z shutting down: no longer accepting connections
2026-10-02T17:03:32.242Z dest=chatgpt.com:443 outcome=refused:non-public-address up=0 down=0 ms=0
2026-10-02T17:03:32.242Z shutting down: no longer accepting connections
2026-10-02T17:03:32.242Z dest=chatgpt.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T17:03:32.242Z shutting down: no longer accepting connections
2026-10-02T17:03:32.242Z dest=chatgpt.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T17:03:32.242Z shutting down: no longer accepting connections
2026-10-02T17:03:32.304Z dest=chatgpt.com:443 outcome=refused:dns-failed up=0 down=0 ms=61
2026-10-02T17:03:32.304Z shutting down: no longer accepting connections
2026-10-02T17:03:32.305Z dest=chatgpt.com:443 outcome=allowed end=eof up=11 down=11 ms=0
2026-10-02T17:03:32.305Z shutting down: no longer accepting connections
test proxy::conformance::adversary_codex_dns_failures_never_dial_and_public_control_relays ... ok
2026-10-02T17:03:32.328Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=0
2026-10-02T17:03:32.344Z shutting down: no longer accepting connections
2026-10-02T17:03:32.344Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.344Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.344Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.344Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.344Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.344Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.344Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.344Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.344Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.344Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=15
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.345Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.345Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=1
2026-10-02T17:03:32.407Z drain timeout reached; closing open tunnels
2026-10-02T17:03:32.408Z shutting down: no longer accepting connections
2026-10-02T17:03:32.471Z drain timeout reached; closing open tunnels
2026-10-02T17:03:32.472Z dest=- outcome=refused:client-closed up=0 down=0 ms=0
2026-10-02T17:03:32.472Z shutting down: no longer accepting connections
2026-10-02T17:03:32.479Z dest=github.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T17:03:32.479Z shutting down: no longer accepting connections
2026-10-02T17:03:32.479Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:32.479Z shutting down: no longer accepting connections
2026-10-02T17:03:32.480Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:32.480Z shutting down: no longer accepting connections
2026-10-02T17:03:32.483Z dest=- outcome=refused:client-closed up=0 down=0 ms=0
2026-10-02T17:03:32.483Z shutting down: no longer accepting connections
2026-10-02T17:03:32.484Z dest=auth.openai.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T17:03:32.484Z shutting down: no longer accepting connections
2026-10-02T17:03:32.485Z dest=chatgpt.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T17:03:32.485Z shutting down: no longer accepting connections
2026-10-02T17:03:32.486Z dest=auth.openai.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T17:03:32.486Z shutting down: no longer accepting connections
2026-10-02T17:03:32.486Z dest=chatgpt.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T17:03:32.486Z shutting down: no longer accepting connections
2026-10-02T17:03:32.487Z dest=auth.openai.com:80 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.487Z shutting down: no longer accepting connections
2026-10-02T17:03:32.491Z dest=auth.openai.com:8443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.491Z shutting down: no longer accepting connections
2026-10-02T17:03:32.491Z dest=chatgpt.com:80 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.491Z shutting down: no longer accepting connections
2026-10-02T17:03:32.492Z dest=chatgpt.com:8443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.492Z shutting down: no longer accepting connections
2026-10-02T17:03:32.492Z dest=auth.openai.com.attacker.test:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.492Z shutting down: no longer accepting connections
2026-10-02T17:03:32.493Z dest=chatgpt.com.attacker.test:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.493Z shutting down: no longer accepting connections
2026-10-02T17:03:32.496Z dest=evil.auth.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.497Z shutting down: no longer accepting connections
2026-10-02T17:03:32.497Z dest=evil.chatgpt.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.497Z shutting down: no longer accepting connections
2026-10-02T17:03:32.498Z dest=auth-openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.498Z shutting down: no longer accepting connections
2026-10-02T17:03:32.498Z dest=chatgpt-com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.498Z shutting down: no longer accepting connections
2026-10-02T17:03:32.503Z dest=api.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.503Z shutting down: no longer accepting connections
2026-10-02T17:03:32.503Z dest=cdn.openai.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.503Z shutting down: no longer accepting connections
2026-10-02T17:03:32.504Z dest=github.com:443 outcome=refused:connect-failed up=0 down=0 ms=0
2026-10-02T17:03:32.504Z shutting down: no longer accepting connections
2026-10-02T17:03:32.505Z dest=github.com:443 outcome=allowed end=eof up=5 down=5 ms=0
2026-10-02T17:03:32.505Z shutting down: no longer accepting connections
2026-10-02T17:03:32.566Z dest=github.com:443 outcome=refused:connect-timeout up=0 down=0 ms=61
2026-10-02T17:03:32.566Z shutting down: no longer accepting connections
2026-10-02T17:03:32.568Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=1
2026-10-02T17:03:32.574Z shutting down: no longer accepting connections
2026-10-02T17:03:32.574Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.574Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.574Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.574Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.574Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=4
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=5
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=4
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=4
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.575Z dest=- outcome=refused:client-closed up=0 down=0 ms=1
2026-10-02T17:03:32.575Z dest=github.com:443 outcome=allowed end=eof up=0 down=0 ms=1
2026-10-02T17:03:32.635Z drain timeout reached; closing open tunnels
2026-10-02T17:03:32.697Z shutting down: no longer accepting connections
2026-10-02T17:03:32.798Z drain timeout reached; closing open tunnels
2026-10-02T17:03:32.861Z dest=github.com:443 outcome=allowed end=idle up=0 down=0 ms=61
2026-10-02T17:03:32.861Z shutting down: no longer accepting connections
2026-10-02T17:03:32.862Z dest=169.254.169.254 outcome=refused:ip-literal up=0 down=0 ms=0
2026-10-02T17:03:32.862Z shutting down: no longer accepting connections
2026-10-02T17:03:32.863Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:32.863Z shutting down: no longer accepting connections
2026-10-02T17:03:32.864Z dest=github.com:443 outcome=allowed end=eof up=4 down=4 ms=0
2026-10-02T17:03:32.864Z shutting down: no longer accepting connections
2026-10-02T17:03:32.864Z dest=- outcome=refused:method up=0 down=0 ms=0
2026-10-02T17:03:32.864Z shutting down: no longer accepting connections
2026-10-02T17:03:32.868Z dest=- outcome=refused:head-too-large up=0 down=0 ms=3
2026-10-02T17:03:32.868Z shutting down: no longer accepting connections
2026-10-02T17:03:32.869Z dest=github.com:80 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.869Z shutting down: no longer accepting connections
2026-10-02T17:03:32.870Z dest=github.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T17:03:32.870Z shutting down: no longer accepting connections
2026-10-02T17:03:32.870Z dest=github.com:443 outcome=refused:dns-failed up=0 down=0 ms=0
2026-10-02T17:03:32.870Z shutting down: no longer accepting connections
2026-10-02T17:03:32.872Z dest=github.com:443 outcome=refused:non-public-address up=0 down=0 ms=0
2026-10-02T17:03:32.872Z shutting down: no longer accepting connections
2026-10-02T17:03:32.934Z dest=github.com:443 outcome=refused:dns-failed up=0 down=0 ms=61
2026-10-02T17:03:32.934Z shutting down: no longer accepting connections
2026-10-02T17:03:32.935Z shutting down: no longer accepting connections
2026-10-02T17:03:32.996Z drain timeout reached; closing open tunnels
2026-10-02T17:03:32.997Z dest=github.com.attacker.test:443 outcome=refused:not-allowed up=0 down=0 ms=0
2026-10-02T17:03:32.997Z shutting down: no longer accepting connections
2026-10-02T17:03:32.997Z dest=- outcome=refused:malformed up=0 down=0 ms=0
2026-10-02T17:03:32.997Z shutting down: no longer accepting connections
mantle-egress: 69 passed, 0 failed, 0 unsupported
test proxy::conformance::ess_egress_conformance ... ok

test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.83s

     Running unittests src/main.rs (/dev/shm/mantle-wave7-target-gateway/debug/deps/mantle_egress-d9ae47290ba24e26)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/adversary.rs (/dev/shm/mantle-wave7-target-gateway/debug/deps/adversary-2c7a92e6a4db14b8)

running 6 tests
test ipv4_benchmarking_and_documentation_ranges_are_refused ... ok
test ipv4_ietf_protocol_block_is_refused ... ok
test ipv6_non_global_unicast_space_is_refused ... ok
test ipv6_local_use_nat64_prefix_embedding_metadata_is_refused ... ok
test ipv6_siit_translated_form_embedding_metadata_is_refused ... ok
test silent_sockets_do_not_starve_a_well_formed_request ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

     Running tests/refusal.rs (/dev/shm/mantle-wave7-target-gateway/debug/deps/refusal-0356a4942c04b79d)

running 3 tests
2026-10-02T17:03:33.211Z dest=- outcome=refused:method up=0 down=0 ms=0
2026-10-02T17:03:33.211Z dest=example.com:443 outcome=refused:not-allowed up=0 down=0 ms=0
test refuses_non_connect ... ok
2026-10-02T17:03:33.211Z dest=github.com:80 outcome=refused:not-allowed up=0 down=0 ms=0
test refuses_unlisted_destination_with_403 ... ok
2026-10-02T17:03:33.212Z dest=localhost:36497 outcome=refused:non-public-address up=0 down=0 ms=0
test allowed_name_resolving_to_loopback_is_not_dialled ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

   Doc-tests mantle_egress

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


EXIT_STATUS=0

```

Command: `cargo clippy --locked -p mantle-egress --all-targets -- -D warnings`
Log: `~/.cache/mantle-wave7/scratch-gateway-adversary-1/clippy.log`

```text
    Checking mantle-egress v0.1.0 (~/.local/state/worktree/trees/b10x/mantle/mantle-wave7-codex-gateway/crates/mantle-egress)
    Finished `dev` profile [unoptimized] target(s) in 0.48s

EXIT_STATUS=0

```

Command: `cargo fmt --check`
Log: `~/.cache/mantle-wave7/scratch-gateway-adversary-1/fmt.log`

```text

EXIT_STATUS=0

```

Command: `git diff --check`
Log: `~/.cache/mantle-wave7/scratch-gateway-adversary-1/diff-check.log`

```text

EXIT_STATUS=0

```

Command: `worktree hook session-end --path <assigned-tree> --session codex-mantle-wave7-gateway-adversary-1`
Log: `~/.cache/mantle-wave7/scratch-gateway-adversary-1/lease-end.log`

```text
session-end ~/.local/state/worktree/trees/b10x/mantle/mantle-wave7-codex-gateway

EXIT_STATUS=0

```

Command: `ps -eo pid,ppid,args | rg owned-target-or-package-cargo`
Log: `~/.cache/mantle-wave7/scratch-gateway-adversary-1/process-check.log`

```text
MATCH_EXIT_STATUS=1 (1 means no matching owned process)

```

```findings
[]
```
