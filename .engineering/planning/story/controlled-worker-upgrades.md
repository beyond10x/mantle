---
format: aep.planning-md/3
id: story:controlled-worker-upgrades
kind: story
status: draft
title: Plan and apply compatible worker upgrades with active-session protection
relations:
- decomposes: epic:reliability-and-usability
- depends_on: story:prebuilt-release-artifacts
scope:
- confidence: inferred
  path: README.md
- confidence: inferred
  path: crates/mantle-release
- confidence: inferred
  path: crates/mantle-worker
- confidence: inferred
  path: crates/mantle/src/adapters/ssh.rs
- confidence: inferred
  path: crates/mantle/src/app/worker.rs
- confidence: inferred
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
revision: 13
---
## Outcome
Provide mantle worker upgrade --check and an explicit apply path consuming verified matching release artifacts. UpgradeAssessment and ReleaseManifest are typed in spec/domains/operator.yaml. Existing Worker is the placement entity; current app/worker.rs:315 already refuses some shared-binary changes while active services run, but worker up alone does not complete a coordinated daemon upgrade.

## Acceptance
Named scenarios upgrade-current, upgrade-plan, upgrade-active-refusal, upgrade-unknown-refusal, upgrade-incompatible, upgrade-tampered, upgrade-apply, upgrade-rollback and upgrade-postcheck. Read installed/running versions and Substrate live executions; refuse any active or unknown execution before mutation, including executions absent from Mantle's local records. No force bypass. Verify artifacts first; explicit apply performs bounded staging, service transition, atomic replacement and postchecks; failed application restores known previous binaries/service state or reports exact partial state without claiming rollback. Preserve workspace volumes, config, secrets and permissions. Never silently stop/delete a session. Local process/provider fixtures test refusal with zero mutation calls and failure injection at each transaction stage. Live user workers are not upgraded during this task.

## Scope
Inferred: worker app/CLI, SSH bounded helpers, worker installer/release verifier, operator ESS and tests; current worker.rs/service deployment provides cited baseline. Depends on packaging and doctor. No new Substrate feature is presumed.
