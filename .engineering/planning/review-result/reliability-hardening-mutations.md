---
format: aep.planning-md/3
id: review-result:reliability-hardening-mutations
kind: review-result
status: active
title: Five native implementation mutations and checksum coverage correction
relations:
- reviews: task:reliability-hardening-mutations
revision: 1
---
unit: task:reliability-hardening-mutations; source ab4d3f9bc81d3b3e1a3c9557f08fa4543bf62d60
verdict: green
cases: ESS 263→263; native 26→27 (exec 1, profiles 6, doctor 4, release 15→16); red: four ESS assertion failures, two ESS errors, seven native failures across planted-defect runs
origin: introduced 1 within current unreleased packaging unit, pre-existing 0 before that unit, undecided 0
wrote-outside-worktree: $HOME/.cache/mantle-reliability/hardening-mutations; assigned target and /dev/shm temporary root (cleanup below)
needs-coordinator: yes

Five independent implementation rules were audited using ESS hardening technique 1. Four were detected by the original suite; deleting archive SHA256 verification survived. Strengthening one existing ESS fixture and adding one native integration test made that survivor fail. All production mutations are restored. No shipped implementation defect is claimed, and no publication, commit, AEP write, live-session operation or credential access was performed.

The exact source is the assigned ab4d3f9 baseline, equivalent in runtime behavior to green integration 8195f85. Pinned ESS 0.50 compiled canonical IR before selection. IR SHA256 is 98c0dc6b990be11014b12754a9acda159638e7d8eeca2f152aaa1ec47d580053. Selected commands are ExecOutcome, SelectProfile, ProfileStateIsolation, DiagnoseWorker and VerifyRelease. Full identifiers and typed mutation sites are in evidence.json.

The baseline passed 263 CLI ESS scenarios, 11 exec/profile/doctor integration tests and 15 release integration tests. This worker did not run or claim a fresh 381-case complete integration gate. The component report's 24 outside scenarios are other components, not suppressed failures.

Manual implementation mutation is the supported fit here: crates/mantle-conformance/src/lib.rs::run synthesizes its own authored component suite into a fixed draft path; it has no input option for an emitted mutant suite or report. The selected production rules also live behind otherwise response boundaries in IR, not directly modeled guard or transition expressions. No interpreted target or new broad harness was used.

Mutation observations:
1. m1-exec-nonterminal (known-kill control): remove the state!=Exited part of the terminal observation guard. ESS mantle.orchestration/authored/exec-indeterminate failed (262 passed, 1 failed); actual_cli_preserves_remote_exit_status_and_output failed because unknown/expired/cancelled/running/accepted observations returned 0. Both commands exited 101. Restore: 263 ESS and 1 native passed, both exit 0.
2. m2-profile-override: change legacy-config AND legacy-state absence to OR. ESS mantle.operator/authored/profile-env-conflict failed (262 passed, 1 failed), as did profile_selection_precedence_conflicts_and_database_isolation (5 native passed, 1 failed). Both exits 101. Restore: 263 ESS and 6 native passed, exits 0.
3. m3-profile-isolation: resolve every named profile's state to the first profile's path. ESS ProfileStateIsolation/outcome/returned and authored/profile-state-isolation reported error, caused by duplicate session identity in the shared SQLite database (261 passed, 0 failed, 2 errors). Native profile_selection_precedence_conflicts_and_database_isolation, profile_selection_refuses_symlinked_configuration_state_and_registry_ancestors and selected_profiles_isolate_ssh_identity_known_hosts_and_private_sockets failed (3 passed, 3 failed). Both exits 101. Detection is supported by the actual native failures; ESS errors are reported as errors, not assertion failures or an unexplained harness kill. Restore: 263 ESS and 6 native passed, exits 0.
4. m4-doctor-initializes-state: insert prepare_state into read-only diagnosis after configuration selection. ESS mantle.operator/authored/doctor-state-missing failed (262 passed, 1 failed), and doctor_bounds_local_state_failures_and_suppresses_external_probes failed because the missing state directory was created (3 native passed, 1 failed). Both exits 101. Restore: 263 ESS and 4 native passed, exits 0.
5. m5-release-archive-digest: remove only archive SHA256 equality, retaining size and payload checks. Original suite: 263 ESS and 15 native release tests passed, both exit 0: a real survivor. The existing checksum fixture wrote b"tampered", also changing archive size. A missing digest check therefore remained invisible.

Finding MUT-001 is an insufficient corruption fixture / missing scenario, introduced in current unreleased packaging commit 617c89bcfcf4caffa769ecdc9a10ec426d8c8cb5 and present at the hardening baseline. The release crates are absent in that commit's parent; the truncating fixture first appears in that commit. This is not a claim that version 0.1.4 shipped the gap.

The retained fix changes only test code. The existing named ESS release-checksum-refusal fixture now flips one TAR padding byte after a short notice: same total archive length, unchanged member metadata, unchanged payload bytes and hashes. The new same_length_archive_padding_corruption_is_refused integration test checks the CLI refusal for equivalent corruption, while strict_manifest_and_checksum_refusals_are_not_cli_parse_errors retains the original truncation case. Against the still-planted digest defect, the ESS named scenario failed and the new native test observed CLI exit 0 instead of 1; both test commands exited 101. Restoring the exact digest implementation passed 263 ESS scenarios and the complete 16-test release suite, exits 0. Initial survivors 1; remaining observed survivors 0 after this focused test improvement.

Retained files:
- crates/mantle/src/adapters/orchestration.rs: checksum conformance fixture only; this module is cfg(test) through adapters/state.rs.
- crates/mantle-release/tests/release.rs: one 28-line native integration test.
The portable patch is tests-only.patch (38 additions, 1 deletion). No specification, generated model, dependency or production behavior changed. restored-production.diff is empty and restored-production.exit is 0.

Reproduction and evidence:
All commands run from the assigned managed tree. commands.json and evidence.json provide the explicit build environment, pinned ESS path, selected commands and mutation substitutions. Each m*.patch contains the exact planted defect. Each red/green prefix retains raw .log and direct .exit values; each ESS invocation retains its raw .run.json. M5 original survivor and strengthened red/green prefixes remain separate. run-summary.json provides the measured ESS counts without relabeling errors. The first mistaken --lib baseline attempt is preserved as preflight-wrong-target.* (exit 101; Mantle is a binary target); it is excluded from mutation evidence. The successful baseline uses --bin mantle.

Core commands:
cargo test --locked -p mantle --bin mantle ess_generated_local_conformance -- --nocapture
cargo test --locked -p mantle --test command_correctness --test profiles --test doctor
cargo test --locked -p mantle-release --test release
cargo test --locked -p mantle-release --test release same_length_archive_padding_corruption_is_refused
cargo fmt --check
cargo clippy --locked -p mantle -p mantle-release --tests -- -D warnings

Build environment: jobs 2, development/test debug 0, serial Rust fixtures, assigned target $HOME/.cache/mantle-reliability/hardening-mutations-target, TMPDIR=/dev/shm/mantle-hardening-mutations-tmp, shared sccache 43179 with 1G cap/idle 0. The existing cache server was neither stopped nor restarted. Space was inspected throughout and never approached the 1 GiB root/3 GiB tmpfs floors.

Only technique 1 ran. Techniques 2 random sequences, 3 real-caller replay, 4 determinism, 5 metamorphic relations, 6 exhaustive guard analysis, 7 spec-diff gate and 8 design review were not run. This bounded audit measures five implementation rules, not exhaustive coverage or a system-wide mutation score. Unit 5 was excluded.

Final formatting and relevant Clippy both exited 0. Clippy initially rejected the new test fixture\'s modulo spelling; is_multiple_of fixed that style issue and the final 263-scenario ESS rerun also exited 0. Both lint attempts are retained.

All task commands ended. The target (3,044,045,408 bytes before removal) and temporary root (150,862,815 bytes) were removed; the shared sccache server/cache was left untouched. Scratch evidence remains about 3.9 MB, and ignored generated .engineering/drafts remains 1,617,655 bytes inside the managed tree. Root and tmpfs free space were approximately 17 GiB and 13 GiB after cleanup.

Lease codex-hardening-mutations is released; worktree inspection reports zero live leases. Root owns review, bot commit, integration and eventual worktree retirement. The managed worktree remains at $HOME/.local/state/worktree/trees/b10x/mantle/mantle-hardening-mutations on verify/mantle-hardening-mutations, HEAD ab4d3f9, with exactly the two retained test files changed. It was not finished or archived because root must review and retain the uncommitted patch. Never delete the managed tree manually.

Historical finding (measured against baseline ab4d3f9; source location is the original fixture):

| File:line | Category | Severity | Verdict | Origin | Finding |
| --- | --- | --- | --- | --- | --- |
| crates/mantle/src/adapters/orchestration.rs:883 | mutant | warning | CONFIRMED | introduced | MUT-001: At hardening baseline ab4d3f9, deleting whole-archive digest verification survived all 263 CLI ESS scenarios and 15 release tests because the checksum fixtures also changed archive size; this coverage gap was introduced within the current unreleased packaging unit. |

Resolution, separately: the retained test patch closes this measured gap. The strengthened existing ESS checksum scenario and new native padding-corruption test both failed with the digest guard removed (exit 101), then passed with production code restored. Final ESS has 263 passes, release has 16 passes, formatting and Clippy exit 0. The historical finding stays CONFIRMED; it is not an open production defect.

```findings
- file: crates/mantle/src/adapters/orchestration.rs
  line: 883
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "MUT-001: At hardening baseline ab4d3f9, deleting whole-archive digest verification survived all 263 CLI ESS scenarios and 15 release tests because the checksum fixtures also changed archive size; this coverage gap was introduced within the current unreleased packaging unit."
```
