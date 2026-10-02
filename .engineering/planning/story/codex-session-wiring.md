---
format: aep.planning-md/3
id: story:codex-session-wiring
kind: story
status: active
title: Wire Codex sessions with private home and volatile runtime diagnostics
relations:
- decomposes: story:codex-interactive-start
- depends_on: story:launcher-volatile-replay
scope:
- confidence: cited
  path: AGENTS.md
- confidence: cited
  path: README.md
- confidence: cited
  path: crates/mantle-launch/src/cli.rs
- confidence: cited
  path: crates/mantle-launch/src/serve.rs
- confidence: cited
  path: crates/mantle-launch/src/session.rs
- confidence: cited
  path: crates/mantle-launch/tests/conformance.rs
- confidence: cited
  path: crates/mantle-launch/tests/launch.rs
- confidence: cited
  path: crates/mantle-launch/tests/support/probe.rs
- confidence: cited
  path: crates/mantle-worker/src/lib.rs
- confidence: cited
  path: crates/mantle/examples/codex_qualification.rs
- confidence: cited
  path: crates/mantle/examples/tests/codex_qualification_adversary.rs
- confidence: cited
  path: crates/mantle/src/adapters/conformance.rs
- confidence: cited
  path: crates/mantle/src/adapters/orchestration.rs
- confidence: cited
  path: crates/mantle/src/adapters/state.rs
- confidence: cited
  path: crates/mantle/src/adapters/substrate.rs
- confidence: cited
  path: crates/mantle/src/app/session.rs
- confidence: cited
  path: crates/mantle/src/domain/manifest.rs
- confidence: cited
  path: crates/mantle/src/domain/session.rs
- confidence: inferred
  path: crates/mantle/tests/codex_preflight.rs
- confidence: cited
  path: docs/evidence/codex-compatibility.md
- confidence: inferred
  path: examples/codex.yaml
- confidence: cited
  path: generated/worker-model/Cargo.toml
- confidence: cited
  path: generated/worker-model/source.schema.json
- confidence: cited
  path: generated/worker-model/types-report.json
- confidence: cited
  path: generated/worker-model/types.rs
- confidence: cited
  path: spec/README.md
- confidence: cited
  path: spec/components.yaml
- confidence: cited
  path: spec/conformance-baseline.json
- confidence: cited
  path: spec/domains/launch.yaml
- confidence: cited
  path: spec/domains/manifest.yaml
- confidence: cited
  path: spec/domains/orchestration.yaml
- confidence: cited
  path: spec/domains/session.yaml
- confidence: cited
  path: spec/ess-inputs.yaml
- confidence: inferred
  path: spec/scenarios/cli/codex-private-runtime.yaml
- confidence: inferred
  path: spec/scenarios/cli/codex-start.yaml
- confidence: cited
  path: spec/scenarios/cli/live-name-failedagentstart.yaml
- confidence: cited
  path: spec/scenarios/cli/live-name-failedmaterialization.yaml
- confidence: cited
  path: spec/scenarios/cli/live-name-materializing.yaml
- confidence: cited
  path: spec/scenarios/cli/live-name-running.yaml
- confidence: cited
  path: spec/scenarios/cli/live-name-starting.yaml
- confidence: cited
  path: spec/scenarios/cli/live-name-stopped.yaml
- confidence: cited
  path: spec/scenarios/cli/live-name-stopping.yaml
- confidence: cited
  path: spec/scenarios/cli/manifest-capabilities-are-subset-not-policy.yaml
- confidence: cited
  path: spec/scenarios/cli/manifest-defaults.yaml
- confidence: cited
  path: spec/scenarios/cli/manifest-lower-resource-bounds.yaml
- confidence: cited
  path: spec/scenarios/cli/manifest-nested-cwd.yaml
- confidence: cited
  path: spec/scenarios/cli/manifest-original-bytes-hashed.yaml
- confidence: cited
  path: spec/scenarios/cli/manifest-retention-upper-bound.yaml
- confidence: cited
  path: spec/scenarios/cli/manifest-workspace-root-cwd.yaml
- confidence: cited
  path: spec/scenarios/cli/manifest-zero-cpu-clamped.yaml
- confidence: cited
  path: spec/scenarios/cli/source-composite-insert-isolation.yaml
- confidence: cited
  path: spec/scenarios/cli/source-rows-survive-stop.yaml
- confidence: cited
  path: spec/scenarios/launch/argument-byte-preservation.yaml
- confidence: cited
  path: spec/scenarios/launch/argument-defaults.yaml
- confidence: inferred
  path: spec/scenarios/launch/private-path-initialization.yaml
- confidence: cited
  path: spec/scenarios/launch/replay-policy-arguments.yaml
revision: 47
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T14:42:30Z", actor: "human:timo", revision: 46}
- {from: "proposed", to: "active", at: "2026-10-02T14:42:30Z", actor: "human:timo", revision: 47}
---
## Context and parent boundary

This is the independently testable code portion of story:codex-interactive-start. The user directed this session to finish Mantle Codex integration while another session implements Substrate issue112. Current production code rejects Codex manifests, drops agent identity, stores no agent/auth columns, reads and installs a Claude token unconditionally, and hard-codes Claude start/attach request construction. The complete parent still owns first real authenticated conversation and CS02–CS08 live acceptance. This substory cannot close that parent or the epic.

Existing typed homes already declare the nouns: mantle.session.AgentKind and AuthenticationMethod in spec/domains/session.yaml, Session, mantle.manifest.Agent/Resolved and mantle.orchestration request commands. Extend those actual fields/commands/views and validate ESS before runtime edits; no new conceptual model or generic agent framework.

## Acceptance

A Codex manifest previously rejected now resolves and retains Codex/ChatgptDevice identity through actual SQLite and real start/attach request builders, while legacy records and Claude credential routing are preserved. Fixed Codex requests select a private credential/conversation home and verified tmpfs runtime diagnostics, with real launcher path checks and synthetic pinned-program evidence. Production refuses requests it cannot bind to supported non-recording Substrate admission before credential, session-insert or workspace side effects. CW01–CW09 below demonstrate this local integration; activation and authenticated end-to-end acceptance remain with the parent.

## Named conformance scenarios

- CW01 codex-manifest-selection: real parser accepts exactly claude-code/codex, maps fixed ClaudeOauth/ChatgptDevice identities, preserves original-byte digest and all existing manifest validation, and rejects unknown kinds/incompatible explicitly supplied auth input. No arbitrary argv/env or API-key fallback.
- CW02 codex-session-migration: real legacy SQLite schema with live/stopped rows and source records migrates transactionally/idempotently to explicit Claude identity; reopen preserves all old values and indexes. New Codex rows survive reopen with their identity; bad enum/pair values fail closed. Both single-session/live-session views and native complete-subject snapshots agree with real rows.
- CW03 codex-launch-request: real request builders select pinned Codex executable, volatile launcher flag, session home and no Claude slot/fd/env; the legacy Claude request remains equivalent and secret values never enter request JSON. A request's semantic non-recording requirement cannot disappear during SDK conversion.
- CW04 codex-preflight-refusal: the production selection/admission seam is invoked before token reads/install, session insertion or workspace creation. With the currently pinned SDK unable to fulfill non-recording capture, Codex gets a fixed unsupported-capability refusal and zero such side effects. Native fake ports measure calls to production logic; a public CLI process test with no Claude config confirms it does not fail by reading Claude credentials. No user-controlled bypass or invented capability fact.
- CW05 codex-attach-selection: reattach uses persisted identity, never re-reads a changed manifest and never routes Codex to a recorded legacy attachment; unsupported capture refuses without new exec/conversation. Agent/auth status is explicit and does not claim authentication from Running/cache existence.
- CW06 claude-selection-regression: all existing manifest/session/orchestration assertions remain, legacy credential installation still uses only its slot, and Codex selection never reads or installs Claude credentials. Preserve each prior scenario name and explain exact schema-driven assertion additions.

## Implementation boundaries

Keep existing Resolved, SessionRecord, RunRequest and StartPort/InitPort/StopPort architecture. Extract only the small production preflight decision needed to observe pre-mutation refusal. Codex output privacy admission is an internal semantic requirement with an explicit unavailable adapter until upstream supplies the actual API; never guess SDK methods, capability names or future availability. The parent replaces that binding with the supported SDK and proves full upstream sink behavior.

This unit includes fixed Codex request configuration and private workspace/tmpfs initialization as specified by CW07–CW09 below. Launcher filesystem primitives carry no Codex policy. It does not implement an unconditional login helper or widen network hosts. The outer-confinement candidate with on-request approvals is explicit for qualification; no dangerous-bypass flag, and actual model-issued approvals remain parent acceptance. Local synthetic tests do not establish managed-policy behavior after authentication.

An exact-host candidate list is not evidence of required traffic. Parent owns network observations, supported capture activation and full authenticated session/lifecycle acceptance. CLI examples must state the current admission limitation and must not claim end-to-end support from this preparatory milestone.

## Scope

Source-scoper report cites production seams in manifest.rs:81/128/173, state.rs:25/48/127/249/281, session.rs:74/119/374/418/805/863, orchestration adapter:274/305. Exact existing scope: crates/mantle/src/domain/manifest.rs; crates/mantle/src/domain/session.rs; crates/mantle/src/adapters/state.rs; crates/mantle/src/adapters/substrate.rs; crates/mantle/src/app/session.rs; crates/mantle/src/adapters/conformance.rs; crates/mantle/src/adapters/orchestration.rs; spec/domains/manifest.yaml; spec/domains/session.yaml; spec/domains/orchestration.yaml; spec/conformance-baseline.json; spec/README.md; README.md; generated/worker-model/Cargo.toml; generated/worker-model/types.rs; generated/worker-model/source.schema.json; generated/worker-model/types-report.json.

Inferred: examples/codex.yaml; spec/scenarios/cli/codex-start.yaml; crates/mantle/tests/codex_preflight.rs. Existing scenario response/input updates must be enumerated from real schema validation before implementation and added to typed scope. No config.rs, worker.rs, secret.rs, deployment files or new generated/session-model crate is reserved without evidence.

Run after launcher-volatile-replay because request construction consumes its flag and whole-spec generated provenance is shared. Parent consumes this substory's implementation rather than claiming it twice; parent alone closes full interactive acceptance. One substory decomposes the parent, so the four-way decomposition panel is skipped for this extraction under planning section7; the cited story-scoper and independent unit adversary still apply.

## Concrete existing scenario updates

Coordinator source scan of 59 existing Parse/InsertSession scenario files identifies 17 whose complete normalized Resolved objects or explicit constructor inputs require the new identity fields. Those exact files are now typed cited scope (eight successful manifest cases, seven live-name cases, two source-retention/isolation cases). Existing assertions and digest semantics stay intact. spec/components.yaml and spec/ess-inputs.yaml are also cited because commands and authored cases are explicitly admitted. Unknown-kind examples that use Codex as the old unsupported value must become explicit positive Codex cases plus retained unknown-kind refusal for an actually unsupported kind; do not erase refusal coverage to make the parser pass.

## Private runtime scope extension

Read-only aep:story-scoper found a supported existing per-exec tmpfs route, so the same local integration unit now includes private-home/configuration initialization. This replaces the earlier exclusion of those Mantle-owned changes. Parent still owns supported Substrate capture activation and real authenticated CS02–CS08 acceptance. No new conceptual model is introduced: extend existing ServeArgs with private_dirs, volatile_dirs and check_private_files (List<Bytes>, omitted means empty), and reuse the existing orchestration Request/EnvironmentEntry. Draft and validate these contracts before runtime edits.

CW07 private-runtime-initialization: real launcher component-by-component descriptor traversal creates private directories without following parent/final symlinks, refuses traversal and non-directories, enforces target owner/mode0700 without chmod of shared ancestors, and proves all checks happen before readiness/child dispatch. Volatile directories additionally require observed tmpfs via fstatfs; ordinary writable disk refuses. Existing private-file checks permit absence or an owned regular0600 singly-linked file, never read/replace/truncate contents, and refuse symlinks/hardlinks/nonregular/loose modes. Repeated initialization preserves synthetic auth bytes/inode. Existing CLI byte-preserving semantics remain.

CW08 codex-private-request: real agent_request selects fixed private dirs /workspace/.mantle, /workspace/.mantle/home and its .codex child, volatile dirs /tmp/mantle-codex/{sqlite,log}, and metadata-only checks for auth.json/config.toml/environments.toml. Fixed CODEX_HOME, RUST_LOG=off and CODEX_TUI_RECORD_SESSION=0 plus strict config argv select ChatGPT/file auth, disable updates/analytics/feedback/input-history, route sqlite_home/log_dir to tmpfs, and request on-request approvals. Keep the reviewed outer-confinement candidate explicit; never the bypass flag. No host env/config, API key, issuer override or recording override enters production requests. Claude requests retain existing semantics.

CW09 synthetic-codex-runtime-sinks: bounded Rust fixtures exercise real SQLite/WAL and launcher behavior, scan live sinks and catch an intentionally planted persistent diagnostic positive control. Digest-verified pinned Codex runs with a synthetic private home and fixed configuration, no real credentials or model request; observe actual SQLite/text-log placement and conflict handling for ordinary local settings. A controlled local issuer may exercise a synthetic login failure using Codex's supported test-only issuer flag, never in production. Record exactly which sink was exercised and retain any unexercised real refresh/managed-policy acceptance with the parent. Auth and intended conversation rollouts remain private workspace files; terminal/diagnostic persistence is forbidden. This is source-and-local-runtime preparation, not authenticated compatibility.

Cited additions to scope: crates/mantle-launch/src/{cli,serve,session}.rs; crates/mantle-launch/tests/{conformance,launch}.rs; crates/mantle-launch/tests/support/probe.rs; crates/mantle/examples/codex_qualification.rs; spec/domains/launch.yaml; spec/scenarios/launch/argument-defaults.yaml; spec/scenarios/launch/argument-byte-preservation.yaml; AGENTS.md; docs/evidence/codex-compatibility.md. Inferred additions: spec/scenarios/launch/private-path-initialization.yaml and spec/scenarios/cli/codex-private-runtime.yaml. Existing wiring scope already covers app/session.rs, orchestration contract, admissions and generated provenance. Any newly complete ServeArgs expectations added by wave4 must be enumerated after merge and updated without dropping prior assertions.

Implementation remains small filesystem primitives in the launcher, with all Codex choices in Mantle. No new generic configuration framework, helper login flow, policy marker or SDK invention. A malicious same-UID process racing initialization remains outside the existing private-directory threat model. Private tmpfs uses the existing process memory/zero-swap/lifetime bounds; no separate diagnostic quota is claimed. Source scoper confidence is high for existing seams, inferred for new scenario names; no source or live runtime changes were made during scoping.

## Generated identity type scope

Coordinator source inspection found that the existing generated/worker-model selects only AgentInstallationFacts and AgentInstallationOutcome. The drift-test roots are hard-coded in crates/mantle-worker/src/lib.rs:1771–1809, and that crate already exposes the generated installation types to Mantle. Add AgentKind and AuthenticationMethod to the same generated selection and narrow public reexports, and update that exact drift check. This adds cited scope crates/mantle-worker/src/lib.rs only for generated identity-type exposure and provenance validation; installer/runtime worker behavior is unchanged. Use generated identities with wire conversion helpers, rather than hand-copying enum definitions or creating another generated crate.

## Qualification fixture independence

Wave4 integration exposed an existing ambient-environment requirement in crates/mantle/examples/tests/codex_qualification_adversary.rs:12,60: two tests panic if TMPDIR is unset. There is no corresponding Taskfile/CI setup. Private TMPDIR unblocks the unchanged current gate, with initial red retained. As part of CW09 qualification fixtures, make those tests allocate their own private temporary parent when no task-specific parent is supplied, preserving all binary/FIFO/symlink assertions and owned cleanup. Never change process-global HOME or mutate shared environment during parallel tests. This adds cited scope for that exact existing test file; it does not change runtime behavior.
