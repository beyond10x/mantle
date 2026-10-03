---
format: aep.planning-md/3
id: story:worker-doctor
kind: story
status: draft
title: Diagnose worker readiness without changing it
relations:
- decomposes: epic:reliability-and-usability
- depends_on: story:named-profiles
scope:
- confidence: inferred
  path: README.md
- confidence: cited
  path: crates/mantle/src/adapters/ssh.rs
- confidence: cited
  path: crates/mantle/src/adapters/state.rs
- confidence: cited
  path: crates/mantle/src/adapters/substrate.rs
- confidence: inferred
  path: crates/mantle/src/app/doctor.rs
- confidence: cited
  path: crates/mantle/src/app/worker.rs
- confidence: cited
  path: crates/mantle/src/main.rs
- confidence: inferred
  path: generated/worker-model
- confidence: inferred
  path: spec/components.yaml
- confidence: inferred
  path: spec/domains/operator.yaml
- confidence: inferred
  path: spec/ess-inputs.yaml
- confidence: inferred
  path: spec/scenarios/cli
- confidence: inferred
  path: website/index.html
revision: 20
---
## Contract and design
DiagnosticCheck and DiagnosticReport are typed in spec/domains/operator.yaml. Add mantle [--profile NAME] doctor [--json] with bounded read-only checks in order: selected configuration, existing local state/worker placement, provider reachability, strict SSH, Substrate service/socket, installed/runtime compatibility and required common confinement facts. Report healthy/failed/skipped per stage, actionable fixed advice, and local nonzero for any required failed/unknown stage. Stop dependent probes after a failure. A provider's Running/Ready cannot establish guest or Substrate health. Authentication is never claimed from readiness.

## Acceptance
Named ESS scenarios doctor-healthy, doctor-config-invalid, doctor-state-missing, doctor-placement-mismatch, doctor-provider-unreachable, doctor-ssh-unreachable, doctor-service-unavailable, doctor-version-mismatch, doctor-facts-missing, doctor-timeout and doctor-readonly. Use controlled process/probe fixtures at the real production orchestration boundary, including timeout kill/wait and subsequent-stage suppression. No key generation, state/schema migration, chmod, known-host acceptance, credential command, daemon restart or session mutation. Existing known hosts are checked strictly. No raw remote output or private config/credential values in reports. Human and JSON reports agree. Resource bounds and confinement remain unchanged. Live unreachable-worker check may demonstrate the negative path but is not required for deterministic correctness or authenticated parity.

## Scope
Cited: worker.rs status/report_machine/ssh_for, adapters/{ssh,state,substrate,kubevirt,aws}.rs and main/config dispatch. Inferred: doctor module with probe port, read-only state/SSH constructors, tests/operator scenarios, generated provenance and README/site. Profiles are implemented first so doctor resolves and reports the same context as commands. Coordinator serializes all shared source/spec files.

## Compatibility oracle and authentication boundary

Fixing acceptance review: require the compiled SDK's admitted wire contract (currently0.17) and its pinned runtime version SUBSTRATE_VERSION (currently0.7.10). Compare the live discovery Machine.driver_version and SDK handshake to those expectations; inspect installed worker launcher/worker/egress versions separately against the CLI version. An installed --version or bootstrap record cannot establish the daemon currently serving requests. A missing/ambiguous observation is failed/unknown, never healthy. Test matching disk version with stale live daemon explicitly.

No agent credential command means never call Config::claude_token, execute its token_command, read Codex credentials or emit provider credential values. Normal configured provider authentication may be used for read-only provider observations, including existing auth plugins; its bounded failure must be reported without dumping plugin stderr. No privilege or credential-policy expansion is implied. The checker itself writes no profile/state/key/known-host data; temporary socket/process resources are bounded, owned and cleaned up.
