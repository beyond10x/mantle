---
format: aep.planning-md/3
id: review-result:reliability-hardening-design
kind: review-result
status: active
title: ESS hardening design review of reliability units one through four
relations:
- reviews: task:reliability-hardening-design
revision: 1
---
# ESS hardening design review — reliability units 1–4

unit: task:reliability-hardening-design at ab4d3f9bc81d3b3e1a3c9557f08fa4543bf62d60
verdict: red (substantive specification gaps; no runtime defect established)
cases: one planted mapping defect detected, restored control passes; no runtime suite executed
origin: all real findings present in this review baseline; introduction attribution not assessed
wrote-outside-worktree: $HOME/.cache/mantle-reliability/hardening-design/**; empty /dev/shm/mantle-hardening-design-tmp
needs-coordinator: yes

## Baseline and method

Managed tree: $HOME/.local/state/worktree/trees/b10x/mantle/mantle-hardening-design.
Branch: verify/mantle-hardening-design. HEAD: ab4d3f9bc81d3b3e1a3c9557f08fa4543bf62d60.
Source was never edited. The coordinator supplies the prior full green 381-case gate at 8195f85;
this review does not present that inherited result as a newly executed test.

Read the complete accepted command-correctness, named-profiles, worker-doctor and
prebuilt-release-artifacts stories; matching README/website sections; spec/README mapping;
operator types/commands and all associated authored scenarios; relevant orchestration request,
ExecOutcome and SourceResolution declarations/scenarios. Read selected adapter/fixture code only
to establish what a scenario observation actually means and avoid calling package coverage absent.
This is technique 8 (design review), not an implementation adversary run or mutation score.

Validation first: pinned $HOME/.cache/aap-ess-upgrade/ess-0.50/bin/ess specify validate --path spec,
exit 0, "mantle v1 — 8 file(s), 268 scenario(s), valid". See baseline-validation.log.

## Negative control and restoration

Control ID HD-C01: delete only the source_commit String field from ReleaseManifest in the scratch
spec-copy/domains/operator.yaml; no changes to the genuine specification.
Design: .engineering/planning/story/prebuilt-release-artifacts.md:59 promises a
"strict manifest bound to source revision and Substrate pin"; :82 requires exact source identity.
Original declaration: spec/domains/operator.yaml:48, "{name: source_commit, type: String}".
Mutated declaration at scratch spec-copy/domains/operator.yaml:43–53 jumps from version to
substrate_revision and cannot represent the manifest's source commit.
Manual design review verdict: missing, named failure HD-C01. This is the planted failure.

Commands and observed statuses:
- cp -a spec $HOME/.cache/mantle-reliability/hardening-design/spec-copy: 0.
- sed -i '/      - {name: source_commit, type: String}/d' .../spec-copy/domains/operator.yaml: 0.
- pinned ess specify validate --path .../spec-copy: 0 (planted-validation.log).
- rg -n '\{name: source_commit, type: String\}' .../spec-copy/domains/operator.yaml: 1,
  planted-rule-location.log. This locator supports the manual finding; it is not an automated
  semantic review or a conformance failure.
- diff -u spec/domains/operator.yaml .../spec-copy/domains/operator.yaml: 1 (planted.diff).
- Restore original operator.yaml with cp: 0.
- pinned ess specify validate --path .../spec-copy: 0 (restored-validation.log).
- The same rg locator: 0, line 48 (restored-rule-location.log).
- diff -qr spec .../spec-copy: 0, no differences (restored-copy-comparison.log).

The restored HD-C01 mapping is green. Both valid specifications pass ESS validation: this control
demonstrates why syntactic validation is not a substitute for design review. It does not prove
the genuine baseline is gap-free; the following real findings remain.

## Findings

All citations below are relative to the exact baseline repository, except explicitly named scratch
copies. Each row cites both the accepted design and its existing specification boundary.
"Missing" refers to a missing ESS obligation/observation, not automatically missing implementation.
Package-only checks can protect runtime behavior while the executable specification remains weaker.

| ID | Classification | Design citation and quote | Specification citation and quote | Assessment |
|---|---|---|---|---|
| HD-01 | missing | command-correctness story:47, "Output streams retain exact bytes"; README.md:107–108, "stdout and stderr retain their bytes" | spec/domains/orchestration.yaml:61, stdout/stderr have "type":"String"; spec/scenarios/cli/orchestration-exec-output.yaml:16–17 uses "out\\n"/"err\\n" | The ESS boundary cannot express non-UTF-8 output. Its adapter converts output through String::from_utf8. The real CLI test already uses 0xff/0xfe (tests/command_correctness.rs:98–99), so this is a spec-domain gap, not a demonstrated implementation defect. |
| HD-02 | missing | named-profiles story:61, "database, SSH key, known-host and socket selection"; :68, "short random owner-private temporary directories" and long/colon/Unicode support | spec/domains/operator.yaml:116–121, ProfileStateIsolation returns "first_names", "second_names", "paths_distinct"; spec/scenarios/cli/profile-state-isolation.yaml:11–18 | Declared isolation observes only database names/paths. It cannot assert selected SSH identity, known-host target, distinct private short sockets or supported unusual paths. Adapter orchestration.rs:143–148 confirms database-only observation. Actual subprocess coverage exists at tests/profiles.rs:341; it is not represented by the ESS obligation. |
| HD-03 | missing | worker-doctor story:63, "timeout kill/wait"; :76, "Bound local file reads as well as external processes"; README.md:171–174 names timeout/file/busy bounds | spec/domains/operator.yaml:122–130 exposes only "failure", healthy, failed_stage, called, state_unchanged; doctor-timeout.yaml:10–18 supplies "timeout" and expects provider failure | The declared timeout outcome cannot assert elapsed bounds, child/tunnel retirement, FIFO/oversized-file refusal or socket cleanup. Adapter orchestration.rs:163–168 turns timeout into an immediate controlled refusal. This scenario proves failure routing, not the bounded-cleanup promise. |
| HD-04 | missing | worker-doctor story:63, "No raw remote output or private config/credential values in reports. Human and JSON reports agree"; :76 requires structured selection failures; :60 requires "local nonzero" | spec/domains/operator.yaml:22–27 declares DiagnosticReport, but DiagnoseWorker at :122–130 returns no report, rendered output, selection, exit status or safe-detail observation; doctor-config-invalid.yaml:13–18 checks only stage/calls/state | The model has a report-shaped value but no executable obligation for its privacy, optional selection, human/JSON agreement or process exit. The adapter always supplies Ok(selection) at orchestration.rs:249–260. Actual CLI tests exist; this finding is the weaker ESS seam. |
| HD-05 | missing | worker-doctor story:70, "installed worker launcher/worker/egress versions separately" and "matching disk version with stale live daemon"; :72, "current committed WAL state must be observed" | spec/domains/operator.yaml:124 permits a failure String only; doctor-version-mismatch.yaml:10–20 has "version" and generic compatibility failure; doctor-readonly.yaml:10–20 has "none" and "state_unchanged":true | There is no independent disk/live/wire identity or WAL-observed value in the declaration. Adapter orchestration.rs:198–202 supplies a Machine directly, and readonly observation compares schema/change counters. The ESS seam cannot pin stale-live-with-matching-disk, actual handshake compatibility or WAL freshness. tests/doctor.rs:186 supplies broader package coverage; not called a runtime defect. |
| HD-06 | missing | prebuilt-release-artifacts story:82, "Do not accept arbitrary supplied binaries and label them with the current Git HEAD merely because --version matches"; README.md:234–239 requires raw exact Git export, fresh build and no relabel | spec/domains/operator.yaml:72–79 VerifyRelease exposes "alteration" and "source_matches"; release-manifest-identities.yaml:1 says "A malformed source identity" with alteration "source" | A syntactically invalid source string is the only identity refusal in the ESS scenario. Adapter orchestration.rs:845,879–881 uses a fixed valid string and changes it to "invalid"; it never builds/export-checks the claimed source. Exact-source/wrong-valid-source and attribute/replacement-object defenses are package-tested but absent from the specification boundary. |
| HD-07 | missing | prebuilt-release-artifacts story:59, "atomic replacement" and "interrupted installation"; README.md:218–220, bounded lock, whole generation and prior installation preservation | spec/domains/operator.yaml:80–86 InstallRelease accepts only "collision" and returns accepted/previous_preserved; release-install-atomic.yaml:1 uses collision=true and accepted=false | Collision refusal is specified; successful activation, concurrency, interruption and previous managed-generation preservation are not. The production package tests include those cases (tests/release.rs:163,473,522), but the ESS interface cannot state their phase/identity observations. An always-refuse installer is compatible with this authored case. |
| HD-08 | missing | prebuilt-release-artifacts story:66, exact tag/source/version checks through "b10x-gates gh" and "No clobber/overwrite"; README.md:248–251 adds remote tag/asset verification and honest partial failure | spec/domains/operator.yaml:71–86 has only VerifyRelease and InstallRelease for release behavior; their sole outcome is "returned" | No ESS command/scenario describes publication source authority, remote-tag identity, existing-release refusal, verified asset set/digests, or partial failure. The tooling and package publication test exist (tests/release.rs:358); this is an unrepresented accepted boundary, not a request to publish or add an online dependency to conformance. |
| HD-09 | missing | prebuilt-release-artifacts story:74, "bounded expansion, duplicate/link refusal, exact version and static ELF checks"; :86, "refuse unrecognized build-runtime provenance" and retain complete notices | spec/domains/operator.yaml:72–79 has no bound/version/runtime/entry-type observations; release-package-layout.yaml:1 expects payload_count=12; release-path-safety.yaml:1 uses alteration "path" | Existing cases pin inventory count, malformed path/target/source and checksum tampering, not all accepted archive/runtime obligations. The fixture fills notices with "fixture notice" and alters manifest payload path (orchestration.rs:868,886). Complete matching license/runtime contents, archive links/duplicates/expansion and runtime version checks are not inferable from twelve payloads. Broader package tests and build qualification are acknowledged. |
| HD-10 | stale mapping | accepted named-profiles story:58 and worker-doctor story:60 assign their types to operator.yaml; release story:56 assigns ReleaseManifest/Artifact there | spec/README.md:3 says "Five ESS domains"; its boundary table :11–17 has no operator row, while spec/system.yaml:11 declares mantle.operator and operator.yaml:4,16,43 declares the accepted types | The mapping cannot lead a reader to the four green units' operator boundaries and still cites an older SDK/evidence inventory at spec/README.md:26,73–74,104. Already assigned to final documentation reconciliation; retain as known mapping debt, not a new feature. |
| HD-11 | unclear | command-correctness story:47 puts "invalid codes" and "Output streams retain exact bytes" in the same unconditional acceptance paragraph; README.md:107–108 repeats byte preservation | spec/scenarios/cli/orchestration-exec-indeterminate.yaml:192,203–204 passes code_json "256" with nonempty input streams but expects stdout "" and stderr_preserved false | Clarify whether byte preservation applies only after a valid SDK envelope decodes. The current spec deliberately drops streams for an undecodable code. Do not infer a new raw-response salvage requirement; owner clarification is needed. |

Story citation prefixes in the table are .engineering/planning/story/<story>.md.
Implementation/test citation prefixes are crates/mantle/src/ or crates/mantle/ unless the row
explicitly refers to release tests, which are crates/mantle-release/tests/release.rs.

Classification totals: missing 9; contradicts 0; stale mapping 1; spec-only 0; unclear 1.
Engineering disposition: nine substantive spec gaps, one known mapping/documentation gap,
one contract ambiguity. Implementation defects established: 0. Technique false positives retained: 0.
The planted HD-C01 is separate and excluded from real finding counts.

## Design-to-spec inventory

The inventory groups related promises; a finding above enumerates the uncovered portion.
It does not claim that one example proves a universal property.

- Command correctness: normal 0/1/42/255 mapping -> ExecOutcome and exec-exit-zero/nonzero;
  supported INT/TERM/KILL -> exec-signal; invalid/missing/conflicting/Unknown/Expired/Cancelled ->
  exec-indeterminate. Transport failure and actual OS exit use package CLI tests; not independent
  ESS observations. Exact binary streams -> HD-01/HD-11. Branch/lightweight/annotated/exact commit,
  persisted observed identity and missing-ref-no-agent -> SourceResolution and all five source cases.
  Secret-free supplemental argv/aperture/environment -> Request and orchestration-exec-request.
  The design rejected a clone rewrite; no missing rewrite or new Git credential feature is inferred.
- Profiles: Profile/Selection absolute path references -> operator value types and profile-add;
  duplicate no-overwrite -> profile-duplicate; traversal/relative/symlink refusal -> name/path/symlink
  cases; add/list/show local/no state creation -> registry cases; flag precedence -> profile-selection;
  named legacy conflict -> profile-env-conflict; legacy defaults -> profile-legacy-defaults;
  database separation -> ProfileStateIsolation. Transport selection and short path behavior -> HD-02.
  Malformed registry/descriptor/name boundaries and concurrent no-clobber have package tests;
  the ESS fixture's existing/symlink/relative switches specify only a subset. No active-profile
  migration was promised.
- Doctor: optional Selection and report shapes -> DiagnosticReport; ordered calls and failure
  suppression -> healthy and all stage failure cases; facts absence -> doctor-facts-missing;
  coarse state/schema preservation -> doctor-readonly. Process/local IO bounds -> HD-03;
  safe reports/CLI entry/exit -> HD-04; independent compatibility/WAL observations -> HD-05.
  Provider credentials are permitted only through existing provider authentication, distinct
  from forbidden agent token commands; no ban on all auth plugins is invented.
- Release: source/pin/runtime/payload typed identities -> ReleaseManifest/Artifact/Payload;
  two inventory targets and strict integrity examples -> six release scenarios;
  exact build provenance -> HD-06; whole-generation activation -> HD-07;
  bot publication/source/immutability -> HD-08; bounded archive/runtime/notices -> HD-09.
  Development artifact availability is distinct from an actual release. Rust/clap, dependency
  acyclicity, fixed CI action pins, generated-source ownership and external private policies are
  source/workflow constraints; reviewed as design context, not fabricated domain transitions.

## Reverse declaration pass and scope limits

Reviewed every field/command in operator.yaml used by the four selected units and relevant
ExecOutcome, SourceResolution and supplementary Request declarations. Profile/Selection fields
map to named-profiles:58; optional report selection maps to doctor:76; diagnostic fields to doctor:60;
release payload/source/runtime fields to release:59,82,86,90. No unjustified spec-only declaration
was found in these selected boundaries. Fixture controls such as alteration/failure are
observation harness inputs, not purported public CLI options.

UpgradeAssessment and AcceptanceResult are intentionally excluded as unit5/unit7 work in progress.
Retained-session lifecycle, unit5 offline observation/transaction, unit7 acceptance, the general
egress/launcher contracts, old architecture proposals, live cloud/provider/authentication behavior,
publication credential availability and all release/deployment actions are not reviewed here.
No scheduling, snapshots, fleet, peer-bus, shared-cache or encrypted transport requirement is added.

Techniques 1–7 were not run by this reviewer; no mutation score, reference-model exploration,
guard completeness, runtime determinism, caller replay, metamorphic result or spec-diff result
is claimed. No build, runtime test, external service call, credential inspection or source/planning
write occurred. Selection of a shared IO adapter does not itself prove the omitted guarantees.

## Evidence and handoff

All retained files are beneath $HOME/.cache/mantle-reliability/hardening-design:
report.md, evidence.json, baseline-validation.log, planted-validation.log,
planted-rule-location.log, planted.diff, restored-validation.log, restored-rule-location.log,
restored-copy-comparison.log, spec-reference-inventory.txt, source-state.log, files.txt,
and spec-copy/** (a restored byte-identical copy of the tracked spec directory).
/dev/shm/mantle-hardening-design-tmp was created empty; no compiler cache/target was allocated.
Lease codex-hardening-design is released at handoff; release status is recorded in evidence.json.
The managed tree is left clean at its supplied commit for coordinator disposition, not finished
or removed by this reviewer. No commits or publication are produced.

Route missing rules through ESS specifying/conformance; route HD-10 to existing final documentation
work; resolve HD-11 wording with the design owner. Do not describe this baseline review as clean.

