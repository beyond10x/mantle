---
format: aep.planning-md/3
id: story:named-profiles
kind: story
status: draft
title: Select configuration and private state as one named profile
relations:
- decomposes: epic:reliability-and-usability
- depends_on: story:command-correctness
scope:
- confidence: inferred
  path: README.md
- confidence: inferred
  path: crates/mantle/src/adapters/orchestration.rs
- confidence: cited
  path: crates/mantle/src/adapters/ssh.rs
- confidence: cited
  path: crates/mantle/src/app/worker.rs
- confidence: cited
  path: crates/mantle/src/config.rs
- confidence: cited
  path: crates/mantle/src/main.rs
- confidence: inferred
  path: examples
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
revision: 18
---
## Contract and design
New values Profile and Selection have their typed home in spec/domains/operator.yaml. Inferred design: mantle profile add NAME --config ABS --state-dir ABS stores only these absolute path references under the owner's configuration directory, with strict names and private permissions. mantle profile list/show inspect those references without loading provider config or executing credential commands. No overwrite on duplicate name, no credential copy. Global --profile NAME (or MANTLE_PROFILE, explicit flag wins) selects both paths together. When a named profile is selected, legacy MANTLE_CONFIG/MANTLE_STATE_DIR overrides are refused to prevent mixing workers and state. Without profile selection, legacy environment/default behavior remains unchanged. No automatic active-profile migration is required.

## Acceptance
Named native ESS scenarios profile-add, profile-duplicate, profile-name-refusal, profile-path-refusal, profile-list-show, profile-selection, profile-env-conflict, profile-legacy-defaults, profile-state-isolation and profile-symlink-refusal. Real subprocess tests use two profiles and verify database, SSH key, known-host and socket selection all follow the selected state directory, with no process-global environment mutation or re-exec workaround. Profiles cannot traverse paths or follow unsafe descriptor links. Show/list print no credential file contents; fail closed on malformed data. Existing commands retain semantics. CLI help, examples and public docs show one reliable profile command sequence.

## Scope
Cited: config.rs selection and state_dir, main.rs CLI/dispatch, adapters/ssh.rs implicit state/key selection, app/worker.rs ssh_for/public_key. Inferred: profiles module and tests, operator domain executable commands/scenarios, conformance adapter wiring and generated provenance, README/site/examples. Depends on command-correctness because main and ESS wiring overlap. Follow source-safe path handling; private config files remain outside source.
