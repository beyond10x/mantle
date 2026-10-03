---
format: aep.planning-md/3
id: story:repeatable-agent-acceptance
kind: story
status: proposed
title: Run and record a bounded real-agent lifecycle acceptance suite
relations:
- decomposes: epic:reliability-and-usability
- depends_on: story:retained-workspace-lifecycle
scope:
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Cargo.toml
- confidence: inferred
  path: README.md
- confidence: inferred
  path: crates/mantle-acceptance
- confidence: inferred
  path: crates/mantle-conformance
- confidence: cited
  path: crates/mantle/src/adapters/orchestration.rs
- confidence: cited
  path: crates/mantle/src/adapters/state.rs
- confidence: cited
  path: crates/mantle/src/app/session.rs
- confidence: cited
  path: crates/mantle/src/main.rs
- confidence: cited
  path: crates/mantle/tests/codex_preflight.rs
- confidence: inferred
  path: docs/evidence
- confidence: inferred
  path: generated/worker-model
- confidence: inferred
  path: spec/README.md
- confidence: inferred
  path: spec/components.yaml
- confidence: inferred
  path: spec/conformance-baseline.json
- confidence: cited
  path: spec/domains/operator.yaml
- confidence: inferred
  path: spec/ess-inputs.yaml
- confidence: inferred
  path: spec/scenarios/cli
- confidence: inferred
  path: website/index.html
revision: 28
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T08:57:35Z", actor: "human:timo", revision: 19}
---
## Outcome
Provide a Rust/clap acceptance runner plus operator procedure for disposable Claude and Codex sessions through the installed Mantle CLI, not duplicated orchestration. AcceptanceResult is typed in spec/domains/operator.yaml. Bound duration/output/disk; keep only non-secret metadata, exact version/source identities and explicit case outcomes.

## Acceptance
Named scenarios acceptance-case-inventory, acceptance-real-command-path, acceptance-auth-required, acceptance-no-raw-transcript, acceptance-failure-exit, acceptance-resume-evidence, acceptance-owned-cleanup and acceptance-no-fabricated-pass. Cases cover create/login handoff, repository/model tool turn, detach/reconnect, resize/interrupt, transport loss, status/exec, retained stop/restart and explicit destroy for both agents. User completes real login/model steps in their terminal; runner records operator assertions distinctly from machine observations and never reads credentials/transcripts or treats cache presence as authentication. Missing manual action is operator-required/not-run and aggregate incomplete, never green. Run-id and names isolate disposable sessions; cleanup only exact resources it created, never user sessions. Controlled CLI fixtures prove progression, timeouts, interruptions and failure counts; an opt-in real mode uses the actual installed CLI. Existing full native conformance gate remains mandatory. Documentation gives a short repeatable procedure and clearly separates full live qualification from runner implementation.

## Scope

Inferred: Rust acceptance crate/CLI, Cargo workspace/lock, ESS operator commands/scenarios, fixtures, docs and CI checks. Uses selected named profile; no new auth/agent protocol, no automated credential entry or API billing fallback. Does not claim missing old-epic live cases passed.

Inferred: spec/conformance-baseline.json is reconciled with the final actual suite, retaining previous required scenario identities (including explicit reviewed command renames), adding the new acceptance obligations and raising component floors to measured counts. This accompanies the final native evidence refresh; no required scenario is silently dropped.

## Metadata-only CLI observations

Read-only assessment found current human status fetches and prints terminal stderr (app/session.rs), so the runner must never capture or parse that path. Add versioned metadata-only JSON status and list output, including lookup of a recorded terminal session by exact session id. Status reports recorded versus observed states, session/workspace/agent-exec identity, exit/refusal/connectivity status, source commits and observation time; it must never request terminal output pages or serialize raw diagnostic payloads. Unavailable observations remain explicit and cannot produce a successful acceptance case. Authentication method is not an authenticated-state observation.

Reuse verified release manifest/install provenance where available; an unavailable source identity is unknown, never inferred from package version. Extend the existing AcceptanceResult ESS value with run/session/workspace identity, provenance and machine-versus-operator evidence origin before runtime implementation. The runner starts detached with a unique owned name, discards human startup output, records the resulting identity and uses the metadata path for subsequent checks.

Login/model turns run through an exact printed attach command in the operator's own terminal, outside runner pipes. Resume takes explicit operator attestations for login, model/tool action and visual terminal behavior; label these distinctly from machine observations. Missing attestations remain incomplete. Existing codex_qualification example supplies useful bounded scratch/digest patterns but its direct SDK path does not qualify installed Mantle CLI behavior. No raw terminal payload is retained.

## Final integration documentation coherence

This last source unit also reconciles the public documentation with the completed roadmap. Update spec/README.md to the actual domain inventory, SDK pin, supported agents, lifecycle semantics and executed conformance counts; its current text still mixes old Substrate 0.7.8, five domains and historical 304/348-scenario evidence. README and website must clearly distinguish historical release 0.1.4 behavior from new unreleased behavior, especially stop/destroy, source-only historical installation, managed upgrades and the new operator commands. Preserve historical live-qualification limits rather than relabeling them as current successful acceptance.

The coordinator refreshes .engineering/reports/native from the final exact complete run, including suites, reports, runs, results and the no-op audit, and identifies older evidence as historical. Update public metrics from that same run. Validate the site/source manifest and source docs, without claiming the public website is deployed: this task ends at one integration PR, with no main merge or release. Ordinary publication follows later source delivery.

## Executable provenance and documentation closure

Read-only inspection of release installation (crates/mantle-release/src/lib.rs:205) and the manifest definition (crates/mantle-artifact/src/lib.rs:36) confirms that installed generations contain archive payloads but no retained manifest.json; mantle --version has no source commit. The accepted provenance contract therefore requires an explicitly supplied verified manifest matched to the chosen executable digest, or unknown source. Neither the generation's digest-shaped directory name nor a version string supplies the missing commit. This is an implementation seam of the existing requirement, not a new online lookup or release service.

The final implementation brief retains the additional concrete documentation corrections from read-only review: provider prerequisites, source versus candidate acquisition, historical version framing, current navigation and upgrade anchor, agent-specific credential diagram, evidence labels and complete conformance boundaries. The site builder validates local navigation; future-main source links do not prove publication. No live publication or authenticated qualification is claimed by these edits.

## Structured ownership and bounded metadata preparation

# Unit7 implementation preparation — outside completed hardening findings

Read-only inspection of integration HEAD 12aae8aaeed2aed23648e75a7b92116fdc193958, the unit7 brief and story. No source/planning edits, lease, builds, tests or live calls. Unit6 has not landed; this document does not assume its eventual API. Citations below are relative to this exact source unless explicitly SDK.

The brief already covers output-page avoidance, terminal-id lookup, guarded cleanup, unknown provenance and distinct manual attestations. The following are concrete integration pitfalls underneath those requirements, not additional product scope.

1. **JSON dispatch must precede ordinary writable initialization.** main.rs:197–203 loads config, creates/prepares state and calls Store::open before routing list/status; state.rs:118–149 initializes/migrates SQLite. Merely adding a JSON branch to app/session.rs would still write local state and fail before producing structured metadata on bad selection/config. Doctor already has an early dispatch at main.rs:147–165. Store::open_readonly exists at state.rs:83 and sees committed WAL; its progress deadline is fixed 250ms after opening (104–106), so take the local metadata snapshot before waiting on remote IO. Exact terminal lookup currently exists only as private session_by_id (295); public live_session/live_sessions deliberately exclude STOPPED (290,308). Apply the forthcoming unit6 schema rather than copying these old predicates.

2. **Do not reuse the current status connection unchanged for bounded metadata.** app/session.rs:37–57 invokes Ssh::run before SDK discovery. ssh.rs:138–157 uses unbounded wait_with_output with both streams piped; an async timeout around that synchronous call cannot preempt it. Use the existing bounded administration/diagnostic transport seams (ssh.rs:173,104) with actual owned-process cleanup. SDK get_exec/get_workspace already return metadata only (pinned 65304ed SDK lib.rs:211–228). Keep recorded metadata on unavailable observations; project typed refusal/connectivity categories rather than serializing SdkError strings or raw observed refusal messages. The brief states these outcomes, but the existing connect helper is an unsafe implementation shortcut for them.

3. **First-create ownership has a gap before any expected-id guard can help.** Current start generates its session id internally only after connectivity/capability/credential setup and inserts it at app/session.rs:106–124; the only immediate id output is human text. A runner interrupted after insertion but before its structured lookup has not durably learned the created id. A prior name-absence query followed by later name lookup is not an atomic ownership receipt: start can lose a name collision, and STOPPED names are reusable (state.rs:73,194–218). Define the first-create identity handoff against the actual unit6 implementation. If the handoff remains unprovable, preserve an unresolved checkpoint and refuse automatic cleanup; do not adopt a discovered row merely because its name matches. Subsequent expected-id checks belong inside each CLI mutation before remote effects, not solely in the runner's preceding status call. Existing stop/attach/exec resolve a name once at session.rs:680,443,478; their eventual unit6 replacements must carry the exact selected record rather than re-resolve by name after a guard.

4. **An explicit profile name alone does not pin a resumed run's target.** Every CLI process resolves the registry again (profile.rs:274–290), and main.rs:242–263 reads HOME plus profile/config/state environment. Persist and compare the resolved selection metadata as well as the profile label before continuing a checkpoint. The printed operator attach command must make that same selection; inherited MANTLE_CONFIG/MANTLE_STATE_DIR cause a named selection to refuse (profile.rs:282–285). Bind submitted attestations to the checkpoint's run, agent, case/phase and owned session/workspace/exec generation, so a previous phase's manual assertion cannot silently complete a later restarted generation. AcceptanceResult currently has exec_before/after but no such provenance bindings (operator.yaml:61–70); the brief already authorizes extending it. No separate authentication protocol is needed.

5. **The existing provenance verifier needs more than a standalone manifest.** mantle_artifact::verify at lib.rs:209–219 reads every sibling archive and checks its contents; Manifest::parse alone only validates the declaration and cannot establish that the selected executable came from it. Known provenance should use the verified GNU bin/mantle bytes/digest and keep manifest source separate from binary version. A manifest without its required bundle must yield an explicit unavailable/unknown provenance result, not fall back to trusting its source_commit. Also, an installed bin/mantle path is an indirect symlink through .mantle/current, which install atomically replaces (mantle-release/src/lib.rs:251–270). A digest recorded before that switch need not identify a later invocation after resume. Revalidate/bind the actual selected executable identity at resume/phase transitions; never carry a formerly-known digest forward merely because the pathname/version is unchanged.

No additional SDK capability is required for these points. ExecObservation already exposes exec/workspace ids, observed_at, exit and refusal (SDK model.rs:234–247). Unit6's final ownership/terminal semantics remain the dependency for implementation; no new lifecycle protocol is proposed here.

Outside file added: $HOME/.cache/mantle-reliability/hardening-design/acceptance-preparation.md only. This preparation is separate from HD-01–11 and supplies no new conformance result or runtime defect claim.

## Creation ownership receipt

The read-only preparation identifies an unsafe shortcut in the original brief: a successful
detached start followed only by lookup-by-name cannot prove ownership across concurrent destruction
and name reuse. Use a bounded structured receipt tied to the successful creation, or another proven
immutable creation binding, and persist it before dependent mutation. A lost/interrupted receipt
stays unresolved and never authorizes automatic cleanup of a subsequently discovered name. This
refines the already accepted exact-owned-identity contract; it does not require live qualification.
