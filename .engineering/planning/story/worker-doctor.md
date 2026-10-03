---
format: aep.planning-md/3
id: story:worker-doctor
kind: story
status: active
title: Diagnose worker readiness without changing it
relations:
- decomposes: epic:reliability-and-usability
- depends_on: story:named-profiles
scope:
- confidence: cited
  path: README.md
- confidence: cited
  path: crates/mantle-worker/src/lib.rs
- confidence: cited
  path: crates/mantle/Cargo.toml
- confidence: cited
  path: crates/mantle/src/adapters/aws.rs
- confidence: cited
  path: crates/mantle/src/adapters/kubevirt.rs
- confidence: cited
  path: crates/mantle/src/adapters/orchestration.rs
- confidence: cited
  path: crates/mantle/src/adapters/ssh.rs
- confidence: cited
  path: crates/mantle/src/adapters/state.rs
- confidence: cited
  path: crates/mantle/src/app/doctor.rs
- confidence: cited
  path: crates/mantle/src/app/worker.rs
- confidence: cited
  path: crates/mantle/src/config.rs
- confidence: cited
  path: crates/mantle/src/main.rs
- confidence: cited
  path: crates/mantle/src/profile.rs
- confidence: cited
  path: crates/mantle/tests/doctor.rs
- confidence: cited
  path: crates/mantle/tests/support/doctor_fixture.rs
- confidence: cited
  path: generated/worker-model
- confidence: cited
  path: spec/components.yaml
- confidence: cited
  path: spec/domains/operator.yaml
- confidence: cited
  path: spec/ess-inputs.yaml
- confidence: cited
  path: spec/scenarios/cli
- confidence: cited
  path: website/index.html
revision: 32
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T08:57:34Z", actor: "human:timo", revision: 21, decided_on: {"recorded":{"review_outcome":1}}}
- {from: "proposed", to: "active", at: "2026-10-03T09:45:04Z", actor: "human:timo", revision: 30, decided_on: {"recorded":{"review_outcome":1}}}
---
## Contract and design
DiagnosticCheck and DiagnosticReport are typed in spec/domains/operator.yaml. Add mantle [--profile NAME] doctor [--json] with bounded read-only checks in order: selected configuration, existing local state/worker placement, provider reachability, strict SSH, Substrate service/socket, installed/runtime compatibility and required common confinement facts. Report healthy/failed/skipped per stage, actionable fixed advice, and local nonzero for any required failed/unknown stage. Stop dependent probes after a failure. A provider's Running/Ready cannot establish guest or Substrate health. Authentication is never claimed from readiness.

## Acceptance
Named ESS scenarios doctor-healthy, doctor-config-invalid, doctor-state-missing, doctor-placement-mismatch, doctor-provider-unreachable, doctor-ssh-unreachable, doctor-service-unavailable, doctor-version-mismatch, doctor-facts-missing, doctor-timeout and doctor-readonly. Use controlled process/probe fixtures at the real production orchestration boundary, including timeout kill/wait and subsequent-stage suppression. No key generation, state/schema migration, chmod, known-host acceptance, credential command, daemon restart or session mutation. Existing known hosts are checked strictly. No raw remote output or private config/credential values in reports. Human and JSON reports agree. Resource bounds and confinement remain unchanged. Live unreachable-worker check may demonstrate the negative path but is not required for deterministic correctness or authenticated parity.

## Scope
Cited: worker.rs status/report_machine/ssh_for, adapters/{ssh,state,substrate,kubevirt,aws}.rs and main/config dispatch. Inferred: doctor module with probe port, read-only state/SSH constructors, tests/operator scenarios, generated provenance and README/site. Profiles are implemented first so doctor resolves and reports the same context as commands. Coordinator serializes all shared source/spec files.

## Compatibility oracle and authentication boundary

Fixing acceptance review: require the compiled SDK's admitted wire contract (currently0.17) and its pinned runtime version SUBSTRATE_VERSION (currently0.7.10). Compare the live discovery Machine.driver_version and SDK handshake to those expectations; inspect installed worker launcher/worker/egress versions separately against the CLI version. An installed --version or bootstrap record cannot establish the daemon currently serving requests. A missing/ambiguous observation is failed/unknown, never healthy. Test matching disk version with stale live daemon explicitly.

No agent credential command means never call Config::claude_token, execute its token_command, read Codex credentials or emit provider credential values. Normal configured provider authentication may be used for read-only provider observations, including existing auth plugins; its bounded failure must be reported without dumping plugin stderr. No privilege or credential-policy expansion is implied. The checker never creates a state database, migrates its schema or modifies application records, profile references, keys or known-host contents. Use SQLite's supported read-only/query-only connection and bounded busy handling; ordinary SQLite locking/shared-memory coordination is permitted, and current committed WAL state must be observed. Do not invent a custom VFS or unsafe snapshot-copy protocol merely to suppress transient coordination. Tests compare schema and application records, include WAL data and prove zero application writes. Temporary socket/process resources are bounded, owned and cleaned up.

## Entry, failure and timeout boundaries

Handle doctor before the ordinary mutating Config/Store initialization path. For --json, selection/configuration failures must still produce a structured failed report (ordinary clap usage errors are outside this runtime-report contract). A report whose selection could not be resolved must model that as absent, not invent selected paths; adjust the proposed DiagnosticReport selection field accordingly before implementation. Bound local file reads as well as external processes, so a FIFO or oversized malformed configuration/registry/database entry cannot hang diagnosis. Raw TOML parser excerpts, provider stderr, SSH output and credential-command text are not diagnostics: emit stable safe stage details and actionable remedies. Ensure failed early stages suppress downstream calls. Reuse the existing bounded process-group cleanup for provider/SSH subprocesses; an async timeout around a blocking child alone does not establish bounded cleanup. Temporary private SSH sockets retain the existing short-path behavior.

## Observed implementation scope

Source commit 3ed0f50723890a484a4ea5c8c064f5dab394faae establishes the previously inferred doctor module, actual CLI fixture, bounded live-process extension, read-only state/SSH constructors, operator scenarios, generated provenance and README/site paths. Existing conformance routing required changes in adapters/orchestration.rs; adapters/conformance.rs, adapters/substrate.rs and Cargo.lock did not change and are removed from the write scope. The implementation handoff records package counts 55 to 60 and worker counts 28 to 29, with CLI conformance 243 to 255. These are package results pending separate adversary review and the full integrated gate, not yet story closure.
