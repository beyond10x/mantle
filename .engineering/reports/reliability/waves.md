## Wave execution record

AEP implementing skill 0.19.1; interactive session with explicit standing approval (approval-record:reliability-standing-waves). Seven serial units: command-correctness, named-profiles, worker-doctor, prebuilt-release-artifacts, controlled-worker-upgrades, retained-workspace-lifecycle, repeatable-agent-acceptance. Each is reassessed after its predecessor lands. Review round 2 rejected the proposed online idle-check/stop upgrade. The revised story requires pre-established offline maintenance: masked and inactive services, no queued jobs and empty cgroups, with rollback and an explicit restart-required result. This resolves the planning assumption; implementation and adversarial tests must still establish that the protocol holds.

The complete initial computed waves/collisions/unassessed output is retained in waves-all.json beside this report, including unrelated backlog items that are not selected. No unrelated story is approved by this selection.

Host could allocate only one new reusable agent. Four planning lenses ran sequentially on that agent; this is not independent panel review. Implementation runs on that worker, and coordinator performs a separate adversarial pass. No claim of native named agent-role dispatch.

Measured integration build: cargo build --locked -p mantle, exit 0, 4m10.425s, target 2.1GiB. Root free space 18GiB, tmpfs 15GiB at preflight; floors 10GiB and 3GiB. N=1 also follows shared Cargo, CLI, ESS and documentation paths. Compiler cache is sccache, task-owned server port 43179, limit 1GiB. Unlimited model budget was explicitly granted; per-agent token counters are unavailable.

Integration managed id mantle-reliability-integration; branch integration/mantle-reliability; lease codex-reliability-root. Checkout $HOME/.local/state/worktree/trees/b10x/mantle/mantle-reliability-integration; build $HOME/.cache/mantle-reliability/integration-target; scratch $HOME/.cache/mantle-reliability/integration.

Unit 1 planned id mantle-command-correctness; branch unit/mantle-command-correctness; lease codex-command-correctness. Checkout $HOME/.local/state/worktree/trees/b10x/mantle/mantle-command-correctness; build /dev/shm/mantle-command-correctness-target; scratch $HOME/.cache/mantle-reliability/command-correctness. Later triples are recorded before dispatch. Every target is unique; external targets follow this repository's external-cache convention.

Storage correction during unit 1: unrelated concurrent builds reduced root availability below 1GiB. The completed, unused integration preflight target (2.1GiB) was removed after its process finished and logs were retained. With implementor compiler calls paused, the task-owned cache server at 43179 was stopped and its cache moved to /dev/shm/mantle-reliability-compiler-cache, then restarted with the same 1GiB bound. Other cache servers and tasks were untouched. Unit 1 temporary build/test files use /dev/shm/mantle-command-correctness-tmp. Future integration builds use /dev/shm/mantle-reliability-integration-target with temporary files in /dev/shm/mantle-reliability-integration-tmp. Root now stores only source and small retained logs; its revised floor is 1GiB, while the tmpfs floor remains 3GiB. Available after relocation: root 4.2GiB, tmpfs 12GiB. Check both before every large command and remove only this task's completed disposable outputs.

Unit 1 source commit ebab85f28be1fcd76465a7befc26233a7dcb49e1 passed package tests (46→48 top-level cases), clippy, formatting, specification and generated-model drift checks. Its actual CLI test covers 20 wire observations and a dropped connection. Real Git fixtures disproved the suspected tag defect; production checkout was left unchanged, and forcing the wrong branch made the verifier fail. The separate coordinator review found no issue; source merged at 8022fec. The complete task check exited 0 on that integration source plus planning-only clarifications: workspace formatting, clippy/tests, complete conformance inventory, ESS/AEP validation and site build all ran. Detailed local red/green evidence remains under the recorded scratch root; no user terminal or credential data was collected. Retained complete/CLI reports and exact CLI suite are beside this report. The story is implemented.

Integration runner output, verbatim:

```text
Complete ESS inventory: 348 passed; 0 failed, skipped, unsupported, outside or refused
mantle v1 — 8 file(s), 241 scenario(s), valid
Mantle documentation built at website/build
```

The remaining accepted proposals were recomputed with the store's waves verb. Complete waves, collisions and unassessed output is retained verbatim in waves-remaining-after-command.json. It contains six sequential units, no unassessed items and no dependency cycle. All five roadmap priorities remain selected.

Unit 2 planned id mantle-named-profiles; branch unit/mantle-named-profiles; lease codex-named-profiles. Checkout $HOME/.local/state/worktree/trees/b10x/mantle/mantle-named-profiles; build /dev/shm/mantle-named-profiles-target; scratch $HOME/.cache/mantle-reliability/named-profiles; temporary files /dev/shm/mantle-named-profiles-tmp. It is not dispatched until the command-correctness integration gate passes. Source threading, registry and tests are scoped; the prior short random socket path behavior must remain intact.

Unit 1 publication/cleanup: integration/mantle-reliability was bot-published at 284c2572c2cb6d2e90b603ebcb77f14546541575 with a signed common-Gates receipt after the full repository gate. The unit's commit is contained in that advertised integration history. Its own and review leases were ended, reproducible target/temp output was removed after reading its handoff, and the remaining ignored conformance/ESS operational files were archived by worktree at $HOME/.local/state/worktree/archives/mantle/mantle-command-correctness. Exact-id GC dry-run returned eligible on that archive; exact-id apply removed the tree. Logs and report remain in the recorded scratch root. No PR was created. Unit 2 was dispatched from 284c257 after the green integration gate and common checks.

During unit 2, a bot fetch of origin/main still observed df32e9cc4876776aa46fee8c91c5d351d53f6650, already contained in integration history. Previously completed agents accepted bounded follow-up tasks for read-only release packaging and restart reconciliation assessment. This recovers useful review capacity without changing the serial implementation ordering or retrospectively claiming the earlier planning panel was independent.

Unit 2 implementation committed at 93dc4cab8e594f6bae1c61d7574987d4d57dcba1. It demonstrated three missing-command failures before implementation and a real OpenSSH parser regression before correcting quoted known-host paths. Package tests increased from 48 to 54. A separate adversary pass on the reused attach_readiness_impl agent found no defect and added a boundary case; its runner executed 55 top-level tests, exit 0. That tests-only addition was retained at a114872. The review is recorded verbatim as review-result:named-profiles-adversary, with public-safe home prefixes, without claiming an independent verifier.

The unit merged at efb4752f42720eeb76757f0660f0bae0cf7d343f. Full task check exited 0 on that exact source: formatting, workspace clippy/tests, complete conformance, ESS/AEP validation and documentation build. Raw log/exit remain at $HOME/.cache/mantle-reliability/integration/wave2-gate.{log,exit}; stable CLI report/suite and complete report are retained beside this record. The CLI inventory increased 230 to 243; complete inventory increased 348 to 361. Runner output:

```text
Complete ESS inventory: 361 passed; 0 failed, skipped, unsupported, outside or refused
mantle v1 — 8 file(s), 251 scenario(s), valid
Mantle documentation built at website/build
```

Named profiles is implemented. Complete recomputed remaining waves/collisions/unassessed output is in waves-remaining-after-profiles.json; five serial units remain. Worker doctor is active under the existing standing approval. Its read-only contract permits SQLite's ordinary locking/SHM coordination while prohibiting database creation, migrations and application writes; it must read committed WAL records. This replaces an overstrict preparatory suggestion that would have required a custom snapshot/VFS protocol.

Unit 3 planned id mantle-worker-doctor; branch unit/mantle-worker-doctor; lease codex-worker-doctor. Checkout $HOME/.local/state/worktree/trees/b10x/mantle/mantle-worker-doctor; build /dev/shm/mantle-worker-doctor-target; scratch $HOME/.cache/mantle-reliability/worker-doctor; temporary files /dev/shm/mantle-worker-doctor-tmp. Dispatch follows the green unit 2 integration gate. Provider processes use bounded existing ownership; the live SSH guard extends that same ownership seam. The existing source trees, targets and live workers are never shared for concurrent mutation.

Unit 2 publication and cleanup: 4d5af8c00e38378080912a4d22e76d3159c5d5e0 was bot-published to integration/mantle-reliability after signed common checks passed over 93 commits. Both unit source commits are contained in that advertised history. All unit leases ended; coordinator read implementation/review handoffs, retained logs, and removed only the completed disposable target/temp directories. Remaining ignored ESS/conformance files were archived with zero local-only commits at $HOME/.local/state/worktree/archives/mantle/mantle-named-profiles. Exact-id GC dry-run reported eligible by archive, and exact-id apply removed the tree. Unit 3 was then dispatched from the published 4d5af8c base; no PR or release was created.

Unit 3 implementation committed at 3ed0f50723890a484a4ea5c8c064f5dab394faae, with bot author and committer verified. Two actual CLI cases first failed on the missing doctor command. Final package counts are Mantle 55 to 60 and worker 28 to 29; native CLI conformance is 243 to 255. Clippy, formatting, explicit generated drift, specification and documentation checks passed. The initial baseline run overlapped draft specification authoring; it is retained as a failed preliminary run, and the restored exact baseline was rerun before runtime implementation. The handoff documents that correction rather than claiming the preliminary run passed.

The coordinator read the complete implementation handoff at the recorded unit scratch root. All unit processes ended and the implementor lease was released before attach_readiness_impl began a separate adversary pass with lease codex-worker-doctor-adversary and scratch $HOME/.cache/mantle-reliability/worker-doctor/adversary. Implementation and review charters are retained in .engineering/briefs/reliability-*.md; the implementation charter explicitly records that it was persisted after the original inline dispatch. Review and full integration gate remain pending at this checkpoint.

Unit 3 review and integration completed: review-result:worker-doctor-adversary retains the separate review verbatim, with no findings. Its additional interruption regression passed first in isolation and then with Mantle 61 / worker 29 tests and CLI 255 scenarios. The tests-only addition was bot-committed as f31e095faa417361dd313a7884ff8f82e44db9ea. The unit merged at 1a4c91e8fb876cba6dd5588d349eb7c8130e4127; the full task check on that commit exited 0, including formatting, workspace clippy/tests, complete conformance, specification/planning validation and site build. Raw log and exit code remain under integration/wave3-gate in the assigned scratch root. Exact CLI report/suite and complete report are retained beside this record and imported into the story before its implemented move.

```text
Complete ESS inventory: 373 passed; 0 failed, skipped, unsupported, outside or refused
mantle v1 — 8 file(s), 262 scenario(s), valid
Mantle documentation built at website/build
```

The complete recomputed waves/collisions/unassessed output is retained in waves-remaining-after-doctor.json. Four serial units remain, with no unassessed story or cycle. Prebuilt release artifacts is active under the standing approval. A read-only preparation identified the need for a small mantle-artifact leaf crate so release commands can reuse worker process ownership and future worker upgrades can reuse verification without a dependency cycle; both stories' scopes and the release brief now record it.

Unit 4 planned id mantle-prebuilt-release; branch unit/mantle-prebuilt-release; lease codex-prebuilt-release. Checkout $HOME/.local/state/worktree/trees/b10x/mantle/mantle-prebuilt-release; build /dev/shm/mantle-prebuilt-release-target; scratch $HOME/.cache/mantle-reliability/prebuilt-release; temporary files /dev/shm/mantle-prebuilt-release-tmp. Dispatch follows bot publication of this green checkpoint and retirement of the completed unit 3 tree. The persisted unit brief covers source-bound builds, bounded shared verification, atomic installation, notices, credentialless CI artifacts and local bot-authorized publication tooling. No actual release is authorized by this task.

Unit 3 publication and cleanup completed: checkpoint 70033de298405e7a28eaad9eaa9b0bb5ea0d8c44 was bot-published to integration/mantle-reliability after common checks passed over 105 commits, with a signed receipt and GitHub check run 111184824332. Its source and adversary commits are in the advertised history. The publication command did not create a local remote-tracking ref, so the coordinator fetched that exact branch through the bot route before verifying ancestry. All unit leases and processes ended; both complete handoffs were read, logs retained, and only the completed unit target/tmp outputs removed. Managed archive recorded zero local-only commits and retained ignored ESS/conformance artifacts. Exact-id dry-run reported eligible by archive, and exact-id apply removed mantle-worker-doctor. Archive: $HOME/.local/state/worktree/archives/mantle/mantle-worker-doctor.

Unit 4 was created through worktree from the published 70033de base and dispatched to correctness_scope using the persisted brief. It reuses the bounded worker process owner through a small independent artifact-verification crate. At dispatch, root had roughly 42 GiB free and tmpfs roughly 13 GiB after completed doctor outputs were removed. No PR, tag or release has been created.

Unit 4 storage adjustment: concurrent unrelated builds reduced tmpfs to 6.7 GiB while root retained 28 GiB. The root's completed integration target was idle after the successful unit 3 gate; its logs and evidence had been retained. The coordinator removed only that reproducible target to recover roughly 3.3 GiB. Its assigned path is unchanged and the next integration gate will rebuild it. The unit implementor may place the isolated production-source build under its assigned scratch on root disk, with a unique directory and exact size/path in the handoff, rather than placing another complete build on tmpfs. Root and tmpfs floors remain 1 GiB and 3 GiB.

Unit 4 source checkpoint is 617c89bcfcf4caffa769ecdc9a10ec426d8c8cb5, with bot author and committer verified. Package tests (Mantle 61, worker 29, release 9), CLI conformance (263), clippy, formatting, notices, specification, documentation and explicit model drift passed. A source-bound production build is now running from its clean Git export: work `$HOME/.cache/mantle-reliability/prebuilt-release/production-617c89bcfcf4`, output `$HOME/.cache/mantle-reliability/prebuilt-release/bundle-617c89bcfcf4`, top log `production-build.log` in that scratch root. It builds GNU CLI and musl workers before artifact/static/version/notice validation; success is not yet claimed. Starting availability was 23 GiB root and 7.1 GiB tmpfs.

The shared compiler server encountered `Disk quota exceeded (os error 122)` while writing its temporary dependency file on `/tmp`, despite free filesystem capacity. Explicit model drift succeeded with the wrapper disabled; the production build also disables it. A coordinator bot commit returned `b10x-gates: Quota exceeded (os error 122)` and `b10x-gates: bot Git operation refused`; the same bot operation succeeded after assigning its temporary files to the task-owned integration tmp directory. No check or bot route was bypassed. The unit adversary charter is retained before dispatch as `.engineering/briefs/reliability-prebuilt-release-adversary.md`; review waits for production build completion and handoff.

The first production build refused Git's global PAX provenance header as an unsupported link before compilation; no bundle was produced. An actual Git-export regression reproduced that failure, then passed after admitting the bounded expected provenance header while retaining source-link refusal. Source correction 6f4303dfda5031e111380d1b28441e07dc7e4144 passed release tests (10), clippy and formatting. Its new isolated production work is `$HOME/.cache/mantle-reliability/prebuilt-release/production-6f4303dfda50`, output `bundle-6f4303dfda50` beside it, and log `production-build-6f4303.log`. Both attempts remain recorded; no source-bound success is yet claimed.

With all wrapper-using unit builds complete, the coordinator restarted only the task-owned compiler-cache server at port 43179, preserving its 1 GiB cache and assigning server TMPDIR to `/dev/shm/mantle-reliability-integration-tmp`. Other servers were untouched. A bot fetch again observed main at df32e9cc4876776aa46fee8c91c5d351d53f6650, contained in integration. The future adversary was given a read-only pre-read of candidate 6f4303d while its separate production build runs; no edits, leases or test execution are authorized until the final handoff, and this preparation is not a review verdict.

Unit 4 implementor handed back clean source 6f4303dfda5031e111380d1b28441e07dc7e4144 after the real production build exited 0. GNU compilation took 7m07s and musl 45.62s; all 12 payloads and five executable versions passed verification. The manifest records Rust 1.97.1, glibc 2.44, musl 1.2.5 and exact pinned Substrate 0.7.10. Bootstrap of the bundled installer, installation into scratch-only `install-6f4303dfda50`, both installed version probes and repeat-install idempotence passed. These private candidates carry the existing workspace version; they were not published as release 0.1.4 and do not establish older-host compatibility.

The coordinator read the complete portable `prebuilt-release/implementation.md` and `evidence-files.txt` handoff. Affected Rust lanes are 90 to 100 (Mantle 61, worker 29, release 10), native CLI 255 to 263. The initial three CLI failures and later actual Git-export regression remain retained. All implementor processes ended and its lease was released before formal review. The separate attach_readiness_impl adversary now owns the unit target under `codex-prebuilt-release-adversary`, with scratch `prebuilt-release/adversary`, following the persisted charter. Target is 3.0 GiB, tmp 111 MiB, production work 1.8 GiB and bundle 35 MiB; all retained for review. The integrated full gate and hosted CI remain pending.

Unit 4 adversary pass 1 returned two failing cases (release 10 to 12: 10 passed, 2 failed, exit 101), each first run in isolation. Its complete portable report was read and recorded unchanged as review-result:prebuilt-release-adversary-1 before routing. Local Git export-subst attributes changed committed Rust bytes in an accepted exact-source export (blocker); reinstall accepted an extra unmanaged generation payload (warning, without claiming execution of that payload). The reviewer left origins undecided. The coordinator classified both as introduced from the base tree's absence of mantle-release and the complete new implementations in this unit; the story's Adversary routing section records that source-only inference. Both retained tests return to the same implementor after the adversary ended its lease. No merge occurs while the blocker stands. A corrected clean-source production build and a second review will follow; no third attack is authorized by the procedure. AEP validation: 95 artifacts, valid.

Unit 4 correction committed as 107f9c8bbdc9062cc24e54ac69e753d0f9e7fb68. The original adversary cases were reproduced red before changes, then passed unchanged with two additional class tests (14 release tests total). The source exporter compares complete member inventory, types, modes and bytes against raw committed tree/blob objects with replacement refs disabled. Existing generation reuse validates exact inventory/types before digest checks. Clippy, formatting, notices and documentation passed. Both findings have fixed review outcomes recorded, while final review and integration remain pending. A fresh production build runs under `prebuilt-release/production-107f9c8bbdc9`, with output `bundle-107f9c8bbdc9` and log `production-build-107f9c8.log`; prior successful candidate evidence remains retained.

Read-only preparation for unit 5 found a concrete deployment prerequisite: the current regular `/etc/systemd/system` unit files prevent ordinary masking and supersede runtime masks. The upgrade story and both charters now require feasible operator preservation of exact originals before effective masking, plus direct cgroup subtree observations even when ControlGroup is empty. Primary systemd/kernel citations and exact known/unknown observation requirements are recorded in the story. This work queried no live services and supplies no conformance claim; its fixtures must still prove the implementation. The unit remains proposed until release packaging passes its integration gate.

The coordinator removed only the completed reproducible target subtree of production-6f4303dfda50 (1.8 GiB), after reading its handoff/inventory and confirming no process used it. Its exact source snapshot, compiler logs, verified bundle, bootstrap and isolated installation remain. Current production-107f9c8bbdc9 and the unit review target were untouched. The corrected production build subsequently printed built-and-verified for source 107f9c8; real installer qualification and second review handoff remain with the implementor.

The corrected real build and isolated installation qualification both completed with exit 0. The full correction report and exact private-candidate manifest are retained as wave4-release-validation.md and wave4-release-manifest.json. Second adversary pass is recorded unchanged as review-result:prebuilt-release-adversary-2. Its new shared-blob/path/mode test passed in isolation and both unchanged regressions passed; the final serial release/artifact package run passed all 15 cases. The initial parallel run failed during valid fixture setup with a tmpfs user-quota error despite globally free space; that failed run is preserved in the review. It is not counted as a product finding or a green run.

The complete AEP findings comparison is retained verbatim as wave4-findings.json: carried [], new [], resolved contains both pass-1 signatures. The trend is 2 findings to 0, with no carried finding. The tests-only addition was retained in bot commit 25bac0d. The coordinator ran the same real-bundle verification command against the pre-merge integration and reviewed unit: baseline exited 101 because mantle-release does not exist there, treatment exited 0 and verified source 107f9c8. This is recorded as VERIFIED new behavior, with both raw logs/exit files retained under integration/wave4-claim in scratch. No third review was run.

For the full integration gate, compiler output returns to `$HOME/.cache/mantle-reliability/integration-target` on root disk because both tmpfs mounts enforce user quotas that df does not measure. Tests use RUST_TEST_THREADS=1 to bound independent fixture allocation; tests that explicitly exercise concurrency retain their own concurrent execution. Temporary files remain under the assigned integration tmpfs path for volatile-runtime fixtures. Root free space was 17 GiB before this adjustment. The completed unit target and production compiler target can be removed after handoff, while all reports, exact source snapshots, bundles and install evidence remain; managed source cleanup still waits for publication.

Unit 4 merged at 8195f852596309d2f09c1742c6015b9894237f66. The first integration gate failed before tests because the task cache server had restarted with the retired unit temporary directory. Only server port 43179 was restarted with the stable integration temporary directory and SCCACHE_IDLE_TIMEOUT=0; the same-source retry exited 0. Raw logs and exits are retained as integration/wave4-gate and wave4-gate-retry. All workspace checks, specification, planning and documentation build passed. Exact producer output:

```text
Complete ESS inventory: 381 passed; 0 failed, skipped, unsupported, outside or refused
mantle v1 — 8 file(s), 268 scenario(s), valid
```

The exact CLI suite/report and complete report are retained as wave4-*.json, imported into the packaging story before its implemented move. Completed disposable directories removed after handoff and process checks: /dev/shm/mantle-prebuilt-release-target, /dev/shm/mantle-prebuilt-release-tmp, and the target subtree of $HOME/.cache/mantle-reliability/prebuilt-release/production-107f9c8bbdc9. Source snapshots, all failed/successful logs, verified bundles and isolated installation evidence remain retained.

The full recomputed waves, collisions and unassessed output is retained unchanged as waves-remaining-after-packaging.json: three serial units, no unassessed items or cycles. Unit 5 controlled-worker-upgrades is active under standing approval. It will start from the bot-published checkpoint of this record after completed unit 4 retirement. Managed id mantle-worker-upgrades, branch unit/mantle-worker-upgrades, checkout $HOME/.local/state/worktree/trees/b10x/mantle/mantle-worker-upgrades, implementor lease codex-worker-upgrades. Target is now $HOME/.cache/mantle-reliability/worker-upgrades-target; temporary directory /dev/shm/mantle-worker-upgrades-tmp; scratch $HOME/.cache/mantle-reliability/worker-upgrades. Root had 22 GiB free and tmpfs 9.6 GiB at selection. Assigned target moved to root disk to avoid tmpfs quota pressure, independent fixture tests run serially, and the cache server no longer expires during idle periods. The persisted implementation and adversary briefs govern this dispatch; fresh prebuilt provisioning and first-adoption locking are now explicit in the story.

Unit 4 publication and cleanup completed: c63d5a6ceb4c2817f01586a00cf75390c69a69ce was bot-published after common checks passed over 128 commits, with signed receipt and check run https://github.com/beyond10x/mantle/runs/111201914098. The exact advertised branch was fetched and retained unit commit 25bac0d verified in its history. Managed archive retained ignored ESS outputs and zero local-only commits. Exact-id dry-run reported eligible by archive, and exact-id apply removed mantle-prebuilt-release. Archive: $HOME/.local/state/worktree/archives/mantle/mantle-prebuilt-release.

Unit 5 was created through worktree from that exact published checkpoint and dispatched to correctness_scope using the persisted briefs. Read-only preparation for the later lifecycle unit runs independently on substrate_output_queue; attach_readiness_impl is pre-reading the upgrade review charter without touching implementation or starting a formal attack. Root remains the sole store writer.

During unit 5 implementation, read-only lifecycle preparation identified durable-retirement proof, initial-materialization ownership, fenced completion writes, migration versioning and launch-policy versioning as necessary details of unit 6's existing recovery contract. The story and both charters now retain those cited seams and fixture obligations. No live calls or implementation edits were made by preparation. Coordinator also added the existing fresh mantle-noop-audit.json to CI's retained evidence paths; the next full gate validates the integration containing that one-line artifact-retention correction.

Unit 5 implementation milestone: baseline worker suite29 passed, absent-command CLI regression failed as expected, and an explicitly recorded pre-implementation transaction scaffold failed its eligible fixture. The first implementation run passed both eligible actual-layout exchange and unchanged-inventory refusal tests. Crash/rollback/locking, laptop transport and independent review remain pending. The atomic primitive uses already-present rustix renameat_with(EXCHANGE), because nix0.30 exposes renameat2 only on GNU while workers target musl; semantics and no-fallback requirement are unchanged. Laptop transport is separated into inferred app/worker_upgrade.rs plus app.rs registration; both were added to typed scope. No live worker operations were performed.

At the implementor's request, a coordinator source check found the initial positive maintenance fixture used inaccurate masked FragmentPath metadata. systemd255 reports the mask source path, not /dev/null; exact primary-source citations and test-first routing are retained in the story. Correction and fixture rerun are with the implementor. Empty no-job output was confirmed. This was implementation-time consultation, not a formal adversarial pass, and used no live service observations.

Scope refinement correction: the implementor confirmed app modules are declared inline in already-scoped main.rs; app.rs does not exist. Its earlier inferred app.rs entry was removed through AEP; worker_upgrade.rs remains. The current transaction run measured six passes and one unknown-versus-refused classification failure for absent units, retained for correction together with the mask-source regression.

Unit 5 worker lane now passes eight transaction cases and one command-availability case, including child-process interruptions, failed rollback, first-adoption locking and the corrected mask-source regression. Laptop transport remains in progress. For forthcoming same-input command verification, the coordinator supplied the retained source-bound baseline `$HOME/.cache/mantle-reliability/prebuilt-release/install-107f9c8bbdc9/bin/mantle`, source107f9c8, SHA256178a5ff40c6991d3253aab11bae36223e0427e5f00242384151276e0d7d5ec1f. Its version probe exited0. Git diff107f9c8..c63d5a6 is empty for workspace manifests/lock, Mantle and worker crates and generated worker model, establishing CLI-source equivalence to the assigned base; it is not represented as a build from c63d5a6. Added inferred laptop CLI fixture paths to typed scope. Baseline installation remains untouched.

Unit 5 implementor handed back bot source ac534be9bfe3ca4a274fe5d53b06ca3822a18ba5, clean tracked/untracked state, all commands ended and own lease released. Both author and committer were verified as the bot. Root read the complete implementation report, direct-exit ledger and external inventory; portable report and ledger are retained as wave5-implementation.md and wave5-implementation-status.json. Worker29→42, Mantle61→64, release15 unchanged, native CLI263→273, authored specification268→277. Packages, clippy, format, musl compile, generated drift, specification, notices and documentation check/build all exited0. Baseline claim exits101 on the actual old CLI's unknown upgrade command; treatment exits0 through eligible and populated-cgroup refusal behavior. Root's independent premerge comparison remains pending.

The expanded tests caught and fixed a second interruption seam: marker publication before journal completion must still report recovery pending. Its isolated red and green remain retained. The implementation also distinguishes recovery of a previous candidate from successful application of a newly requested one. Source-bound website build uses ac534be9bfe3ca4a274fe5d53b06ca3822a18ba5; publication and authenticated live-agent qualification are not claimed. spec/README.md was added as cited scope from the actual diff.

Separate attach_readiness_impl adversary pass1 now owns the unit target under lease codex-worker-upgrades-adversary and scratch $HOME/.cache/mantle-reliability/worker-upgrades/adversary. It uses the persisted review charter and current accepted root story. Its earlier pre-read and the coordinator's requested systemd source consultation were not formal attacks. Target3.1GiB and temporary44MiB are retained for review; root12GiB and tmpfs14GiB were available at handoff. No unit integration, final gate or publication is claimed yet.

Read-only implementor preparation for unit6 found no contract contradiction or product decision. Its existing native SessionRecord constructors and lifecycle routing also require crates/mantle/src/adapters/conformance.rs; that cited path is now in typed scope. No unit6 implementation or unit5 changes occurred during this preparation.

Operator explicitly requested subagents for ess:hardening while unit5 review runs. Two bounded verification tasks are active: task:reliability-hardening-design (technique8 against the four already-green accepted contracts, with a deleted-rule negative control on a scratch copy) and task:reliability-hardening-mutations (technique1, at least four native implementation-rule mutations with named red and restored-green evidence). They do not mutate the red unit5 or claim final units6–7 hardened. Root will record findings and route relevant corrections before the one final PR. New features and broad new harnesses are excluded.

Assigned design triple: managed mantle-hardening-design, branch verify/mantle-hardening-design, checkout $HOME/.local/state/worktree/trees/b10x/mantle/mantle-hardening-design, scratch $HOME/.cache/mantle-reliability/hardening-design, temporary /dev/shm/mantle-hardening-design-tmp; no compiler target. Assigned mutation triple: managed mantle-hardening-mutations, branch verify/mantle-hardening-mutations, checkout $HOME/.local/state/worktree/trees/b10x/mantle/mantle-hardening-mutations, scratch $HOME/.cache/mantle-reliability/hardening-mutations, target $HOME/.cache/mantle-reliability/hardening-mutations-target, temporary /dev/shm/mantle-hardening-mutations-tmp. Both fork this committed checkpoint, whose runtime matches the green381-case gate. Only mutation task compiles; root12GiB/tmpfs14GiB available at selection. Persisted briefs own exact scope and controls. The unit5 implementor remains available for its correction; hardening uses separate agents/worktrees.

Both hardening worktrees were created through worktree at ab4d3f9bc81d3b3e1a3c9557f08fa4543bf62d60 after the opening commit's AEP and format checks. Exact git diff from green gate8195f85 to that base is empty for Cargo manifests/lock, crates, generated, deploy and spec. substrate_output_queue owns the design review; fresh reliability_mutation_hardening owns the mutation audit. Both received persisted briefs and explicit Rust-only/no-live-operation/sole-AEP-writer rules. They run alongside unit5 review, leaving its implementor available when review hands back.

Unit5 adversary pass1 is recorded unchanged as review-result:reliability-wave5-adversary-pass1.
The corrected isolated regression and transaction suite both exit101 (suite11 passed/1 failed).
A supported first Codex installation after a completed helper upgrade adds its stable link;
retrying the helper upgrade then removes that link while reporting applied-restart-required.
The fixture models the coherent installer filesystem effect under the real HostLock, not an
actual agent download. Root read the full report and tests-only diff, classified the finding
introduced from the entire upgrade module's absence in the base, and routed correction to the
same implementor after the reviewer released its lease. No base execution is claimed for origin.

The design hardener validated the pinned specification and demonstrated HD-C01 by removing
source_commit on a scratch copy: validation alone stayed green, manual mapping review detected
the omission, and restoring yielded the original exact tree. The full report is retained as
review-result:reliability-hardening-design with its small evidence JSON. Nine actual specification
gaps, one stale mapping and one ambiguity remain; no runtime defect was established by this review.
HD-10 is assigned to unit7's existing final mapping work. Root clarified HD-11 in the command
story and README: byte preservation follows successful SDK-envelope decoding, without a malformed
response salvage promise. A read-only remediation scope is requested for HD-01 through HD-09.
The mutation audit remains running and has measured its first known-kill control red then green.

A fresh bot fetch still observes main df32e9cc4876776aa46fee8c91c5d351d53f6650 as an ancestor of
integration. Future unit6/unit7 briefs now use unique root-filesystem compiler targets rather
than tmpfs targets; their temporary directories remain distinct. No live resources were queried.

The design reviewer supplied an enumerable eleven-finding JSON companion and a source-cited
remediation proposal. The successor review-result:reliability-hardening-design-structured preserves
its original prose and appends that supplied array; the original immutable prose-only review is
archived, not rewritten. AEP still prints the historical prose-only warning for that archived record.
HD-11 has a fixed outcome. HD-01 through HD-09 remain open, each recorded as escalated to proposed
task:reliability-conformance-boundary-expansion; none is called fixed or a reproduced runtime defect.
This broader fixture/spec observation work is outside the bounded audit and five-item integration.
HD-10 remains assigned to unit7. The small checksum mutation gap is corrected in the current audit.

The mutation audit finished on five rules: four original detections, one whole-archive-digest
survivor, and zero remaining observed survivors after a focused fixture correction. The report
is preserved unchanged as review-result:reliability-hardening-mutations and its structured evidence
is beside this log. Original 263 native CLI scenarios and15 release tests missed digest removal;
the existing checksum scenario now changes TAR padding without changing length or payload bytes.
The added real CLI regression and strengthened scenario fail with the guard removed, then pass
with exact production restoration (263 ESS,16 release). Original truncation coverage stays.
Formatting and relevant Clippy exited0 after a recorded test-style correction.

Root reviewed the entire report/evidence and38-addition/1-deletion tests-only diff, committed it as
bot30d5709 and merged it at1aabc06. The focused real CLI regression also passed on that merge,
1 case/exit0, with raw root evidence integration/hardening-merged-case.*. Both bounded audit tasks
are implemented; their open specification findings remain explicitly proposed follow-up work.
The final full integration gate is still pending. The mutation agent removed only its reproducible
target/tmp before root's cleanup reminder; raw scratch evidence and ignored generated drafts remain.
No source tree or shared cache was removed. Source worktree retirement waits for publication.

Unit5 correction a7ac0ca is bot-authored and clean: worker46, Mantle64/native273, formatting,
Clippy, musl and exact-commit docs all exit0. The retained adversary function is byte-identical.
Completed journals are now read-only observations rather than historical rollback authority;
pending journals exclude the actual Installer before fetching, even after the OS lock is released.
The initial bot commit request failed; one identical authorized retry succeeded, with both logs
retained. Root recorded the correction and fixed outcome and handed the released tree to the same
adversary for pass2. No third attack is authorized. Source and target remain owned by that reviewer.

Additional read-only unit7 preparation found concrete implementation traps under its existing
contract: early JSON routing before writable state initialization, bounded synchronous transport,
provable first-created ownership, resolved selection/attestation binding and verified executable
provenance across resume. The full report is recorded in the story and wave7-preparation.md,
with brief references. It establishes no runtime defect or new live acceptance claim.

Unit5 final review is recorded unchanged as review-result:reliability-wave5-adversary-pass2.
The complete findings comparison reports carried0/new0/resolved1. Reviewer-added tests were
retained in bot5b922a5 after both isolated cases and all48 worker tests passed. Root then measured
the same actual CLI check against the verified source107f9c8 baseline and the reviewed candidate:
baseline101 (upgrade absent), treatment0 (observations/refusal, no uploads/local writes). Production
CLI source is equivalent from107f9c8 through the pre-merge baseline; later hardening changes only
tests. Both raw claim logs/statuses are retained. The unit merged cleanly at19c817f.

The full integration gate at19c817f first exited201: release tests reported disk quota122 on
tmpfs (three explicit diagnostics and one generic installation assertion in the same run).
No product source was changed. Root created a private main-filesystem integration-tmp, restarted
only task sccache43179 with that stable TMPDIR (bounded cache retained), and reran the same gate.
The retry exited0: 391 complete ESS cases, nativeCLI273, authored277, all workspace tests, Clippy,
formatting, AEP and exact-commit documentation passed. Both logs/exit files remain under
integration/wave5-gate*. Main filesystem reports ext4 without quota; tmpfs reports usrquota.
Future unit briefs use their own main-filesystem temporary directories. No other workload or quota
was changed. Unit5 is implemented, revision41; the recomputed next waves are retained lifecycle,
then acceptance, with no unassessed scope or cycles and existing overlap kept serial.

For the next independent comparison, root retained the actual CLI produced by the green gate at
19c817f in $HOME/.cache/mantle-reliability/retained-workspaces/baseline/mantle, SHA256
6385e997a17bc333c87956819a3fd876c238508fdb180dd5c2e7e1de9d205e86. It is a private test baseline,
not a release artifact. Native CLI report/suite and complete report were copied as wave5 evidence;
final public native evidence reconciliation remains with the final unit.

Checkpoint e5624f2a907fb59d91c9e54a6680b64c2eee852a passed common checks (152 scanned commits),
retained signed integration/wave5-receipt.json and published through Gates to the one integration
branch. Bot check run: https://github.com/beyond10x/mantle/runs/111225921090. The advertised ref was
fetched and all wanted unit5/hardening commits were confirmed ancestors. Root archived ignored
unit5 and mutation evidence, finished all three completed trees and reviewed exact-id GC dry-run;
GC removed mantle-worker-upgrades, mantle-hardening-mutations and mantle-hardening-design.
Design scratch reports remain retained outside its clean source tree. Archives are under
$HOME/.local/state/worktree/archives/mantle for the two archived trees.

After handoff and publication, root removed only the old unit5 compiler target (3.1GiB), its unused
temporary directory (65MiB), and the retired tmpfs integration temporary directory (47MiB).
The tool rejected a forced-remove spelling; the safer non-forced removal succeeded. Source trees
were removed only by managed GC. Raw scratch logs, baseline binaries, current integration target,
new main-filesystem temporary paths and shared compiler cache remain intact.

Unit6 dispatch: active story:retained-workspace-lifecycle revision30, managed
mantle-retained-workspaces, branch unit/mantle-retained-workspaces, exact published basee5624f2.
Checkout $HOME/.local/state/worktree/trees/b10x/mantle/mantle-retained-workspaces; scratch
$HOME/.cache/mantle-reliability/retained-workspaces; target
$HOME/.cache/mantle-reliability/retained-workspaces-target; temporary
$HOME/.cache/mantle-reliability/retained-workspaces-tmp. correctness_scope implements the complete
current brief under codex-retained-workspaces, with no AEP or live-resource operations. The root
retains orchestration/cleanup ownership; unit7 remains serially dependent on this unit's gate.

At 2026-10-03T15:02Z, a fresh bot fetch still found remote main at
 df32e9cc4876776aa46fee8c91c5d351d53f6650, already an ancestor of the integration branch.
No rebase or source rewrite was needed. Root confirmed bot author and committer on the current
planning commit. Unit6 scope now includes separate app/state lifecycle modules, real CLI fixture
paths, and spec/components.yaml admission updates. The first production regression exits101 with
old Stop destroying the workspace; the unit's ESS draft refusal remains in its raw evidence.
The final PR draft is retained privately under .engineering/drafts/reliability-pr.md with explicit
placeholders for the last two units and final gate; it is not published or treated as completion.

Final-unit scheduling refinement: after the retained-lifecycle gate, the acceptance runtime unit
will receive a disjoint documentation delegate for README.md, website/index.html and spec/README.md.
This is existing story:repeatable-agent-acceptance scope, not another roadmap item or audit. Runtime
implementation excludes those paths. The documentation branch will join the acceptance unit before
its adversarial review and full integration gate. The persisted delegate brief defines the boundary.
Planned managed id mantle-final-documentation, branch unit/mantle-final-documentation, tree
$HOME/.local/state/worktree/trees/b10x/mantle/mantle-final-documentation, lease codex-final-documentation,
target $HOME/.cache/mantle-reliability/final-documentation-target, TMPDIR
$HOME/.cache/mantle-reliability/final-documentation-tmp, scratch
$HOME/.cache/mantle-reliability/final-documentation. No tree or delegate has been started yet.

Unit6 implementation handoff: bot8df716f6c53b56f62ba0ad5ebdfa75c97f859ddd, clean branch
unit/mantle-retained-workspaces. The full report and command ledger are retained in
wave6-implementation.md and wave6-implementation-status.json. Package64→67, nativeCLI273→372,
authored277→307; final package, Clippy, formatting, drift, notices and exact-commit docs exit0.
Thirty authored lifecycle cases exercise production SQLite/filesystem and orchestration seams.
The actual old-CLI filesystem fixture is red because Stop deletes the workspace; the candidate
passes. Its earlier malformed-wire setup failure is retained separately, not causal evidence.
Scope now cites observed paths and explicitly removes the unneeded inferred generated/session-model.
The existing generated/worker-model owns changed ESS provenance. No dependencies changed.

The implementor ended codex-retained-workspaces and all commands. At handoff root observed
19GiB free on root, 31GiB tmpfs; unit target3.0GiB, TMPDIR35MiB, scratch79MiB before report copies.
Adversary pass1 is assigned to attach_readiness_impl, owning the same tree/target under
codex-retained-workspaces-adversary; scratch ends retained-workspaces/adversary/pass1.
Root owns AEP, final baseline/treatment measurement and full integration gate. No source merge or
completion move has happened. Agent token/tool counters are unavailable; no estimates are invented.

Unit6 adversary pass1 is recorded unchanged as review-result:reliability-wave6-adversary-pass1:
findings[], package67→69, CLI372 unchanged, direct package/isolated-case/Clippy exits0. The two
new actual CLI cases cover persisted restart recovery with no second POST and a delayed stale stop
response after another stop/restart. Only test files changed; desired additions are retained in bot
fe0e72c after the reviewer ended its lease. No correction or second attack was needed.

Root acquired codex-retained-workspaces-integration and independently measured the same original
actual CLI/filesystem case against the retained baseline and reviewed candidate. The baseline
SHA256 is unchanged; git diff confirmed source19c817f and the pre-merge integration source agree
across crates, Cargo, generated and spec. Baseline exit101 asserts that Stop deleted workspace
files; candidate exit0 preserves them, checks explicit confirmation/identity refusals and destroys
only when requested. Both logs and direct exits are retained under integration/wave6-claim-base.*
and wave6-claim-treatment.*. Claim VERIFIED before integration; full gate still pending.

Unit6 full gate at deb88905fc6f008f5a2bcbffff8711b9026442f2 exited0. Complete ESS490,
CLI372 and authored307 pass; workspace checks, AEP and exact-source docs build pass.
Direct output/status retained in integration/wave6-gate.log/.exit. Portable reports copied beside
this record. Story moved implemented after the clean adversary and independently VERIFIED claim.
The historical archived prose-review warning remains visible; validation reports valid.

Unit7 replan is recorded in wave7-selection.json under standing approval, skill0.19.1.
The retained-lifecycle dependency is implemented. Runtime owner correctness_scope uses
mantle-agent-acceptance, branch unit/mantle-agent-acceptance, lease codex-agent-acceptance;
documentation owner substrate_output_queue uses mantle-final-documentation and its recorded
branch/lease. Both will start at this checkpoint's commit. Exact paths and storage are in their
persisted briefs. They own disjoint files within the same accepted unit; docs merge into the unit
before independent review. Existing GNU packaging will include the runner. No new roadmap item,
live qualification, main merge, tag, release or website deployment is authorized by this dispatch.

Checkpoint hook initially refused a personal absolute path in the new uncommitted gate evidence
reference. Root preserved that unpublished record privately as integration/wave6-private-path-evidence.json
and re-recorded the same observation through AEP with a portable $HOME reference. No finding was
waived, source assertion changed or hook bypassed. This corrects publication metadata only.

Checkpoint7930ab106f3a2605ab8054552d68241c29e8566e passed signed common checks165commits and
published to integration/mantle-reliability; receipt integration/wave6-receipt.json, bot check
https://github.com/beyond10x/mantle/runs/111242067169. Both final-unit trees were created at that
exact checkpoint and dispatched to their named owners. Runtime baseline CLI was copied from the
green root build to agent-acceptance/baseline/mantle, SHA256
f787b7db40a264a870c9284d40a72d8342971a0a102fff3e6f7c1a9fb1e6307b.

Root ended its final unit6 lease; all unit6 commands were finished. Ignored model/draft/site
outputs were archived under worktree/archives/mantle/mantle-retained-workspaces, and exact-id GC
removed the tree after finish and dry-run review. Removed only its reproducible target and TMPDIR;
raw scratch, original baseline and adversary reports remain. Root storage increased13→16GiB free.
The unchanged premerge VERIFIED claim, already recorded above, now also has typed AEP evidence.

Final-unit handoff: bot runtime3ce4d28037a4c6d3120b5594f463cd8924bf43bc, docs
211332ba6f6b4f6d80d3881f7ce238cad21196e1. Runtime report/status and docs report are retained as
wave7-implementation.md, wave7-implementation-status.json and wave7-documentation.md. Raw runtime
report remains private; its portable copy omits Running lines containing personal paths, retaining
exact result lines and explicitly correcting unused scopes. No test assertion or raw log changed.
Affected packages133→143, metadata4, runner6, release16, worker48; CLI372→381, authored307→315.
Final package, Clippy, fmt, generated drift, notices and fixture checks exit0. Docs tests2/2 and
exact-commit site build pass. Full aggregate run is still pending;499 is not yet claimed measured.

Runtime and docs owners ended their leases and all commands. Root acquired a short integration
lease, merged docs into the unit at2b8b21a0aeb1f85900ba12c2ed481f891452a7a5, independently reran
the unchanged metadata selection-failure test against the retained baseline and current CLI,
and released its lease. Baseline101 lacks bounded JSON; candidate0 emits versioned selection
refusal without stderr. Claim VERIFIED; direct logs/exits are integration/wave7-claim-{base,treatment}.

Adversary pass1 is assigned to attach_readiness_impl on the combined unit2b8b21a, base7930ab1,
lease codex-agent-acceptance-adversary, existing unit target/TMPDIR and scratch
agent-acceptance/adversary/pass1. Root owns the store and final integration gate. No source merge
into integration or implementation-complete move is claimed yet. Agent token/tool counters are
unavailable. The docs/source merge does not publish the website or qualify authenticated sessions.

Planning checkpoint7567deae47c0e4905258439b0647933a4f4bf6d2 was signed/published after168 scanned
commits; integration/wave7-planning-receipt.json and bot check
https://github.com/beyond10x/mantle/runs/111247513909 retain evidence. Fresh main fetch still found
df32e9cc4876776aa46fee8c91c5d351d53f6650 already an ancestor. Local branches for the six completed
units were deleted only after exact ancestry to the published integration ref was verified;
their commits and retained evidence remain recoverable there.

Unit7 adversary pass1 is recorded unchanged as review-result:reliability-wave7-adversary-pass1.
Runner6→8, sevenpassed/onefailed; isolated failure101, isolated interruption/ownership case0,
Clippy/fmt0. Source reporting erases a previously failed final destroy when explicit cleanup
later succeeds. README297–299 promises the failure remains; ordinary transient destruction
failure followed by documented cleanup reaches it without checkpoint edits. Root verified that
runner.rs is an added file in7930ab1..3ce4d28, so routes it as introduced by this unit; the
reviewer's immutable origin remains undecided. No broader failure is inferred.

Root retained the two tests-only paths in bot a4f4ee3 after reviewer lease release, then ended
its short integration lease. correctness_scope owns the same unit under codex-agent-acceptance-correction,
scratch agent-acceptance/correction1, for the minimal correction and unchanged regression.
No existing assertion or documented contract may be weakened. Source has not merged into integration;
full gate and pass2 remain pending. This correction is within existing unit7 scope.

Correction f4bf80579e8daba5ab9a93d281d1ed5a8acab6a2 changes only runner.rs by five lines.
The shared observation writer preserves already-failed qualification rows, including their original
phase, generation, timestamp and origin; cleanup still runs its guarded destroy/readback. Original
isolated regression reproduced101 then passes0; acceptance8/8, Clippy/fmt/diff0. Root read the
complete portable correction report/status and verified tests are byte-identical to a4f4ee3.
Reports are retained as wave7-correction.md and wave7-correction-status.json. Implementor ended
its lease and all commands. Final adversary pass2 now owns the same unit/target under
codex-agent-acceptance-adversary-pass2, scratch agent-acceptance/adversary/pass2. No third attack
is planned. Full integrated gate remains pending; no source completion claim is made yet.

Final adversary pass2 found nothing; runner8→9, all9passed, isolated/package/Clippy/fmt/diff0.
Its unchanged report is review-result:reliability-wave7-adversary-pass2. The CLI findings diff
records carried0/new0/resolved1; original failure and correction evidence remain immutable.
Tests-only commit0c5c495 retained the cleanup retry case. All worker leases ended before integration.

Reviewed source and documentation merged at dee44cf83d2dafc18d447b82674424893f1c916f.
Full task check exited0: complete499=315authored+184generated, CLI381, egress69, launcher49;
zero failed/skipped/unsupported/outside/refused in the complete inventory. All component reports
share digest40dc7bfa7a332091e37575b2bb7fe04934452aa6054539b46dae37a4ae141f5d.
All workspace tests, Clippy, formatting, ESS/AEP validation and site build passed. Direct raw
log/status remain at $HOME/.cache/mantle-reliability/integration/wave7-gate.{log,exit}; portable
counts are wave7-gate-status.json. Fresh coordinated evidence replaced reports/native together;
mutations.json remains explicitly historical. The six no-op outcome passes and zero authored
passes are retained in the new audit. Unit7 moved implemented on these observations.

Docs now report the measured complete499 inventory. HD10's source mapping/counts are corrected;
HD11's decoded-envelope boundary remains explicit. The nine wider observation gaps remain proposed
follow-up, not claimed fixed. No authenticated qualification, release or website publication ran.
Agent token/tool counters are unavailable. Final publication, one PR and managed cleanup follow.

Final delivery: signed source96b3a7a8c430eb274611dc7f3b9f5406393cbcce published with180 scanned
commits and bot check https://github.com/beyond10x/mantle/runs/111259193045. Exactly one final PR,
https://github.com/beyond10x/mantle/pull/8, was created by b10x-bot[bot]. Epic moved implemented
on all seven completed stories, the measured gate and the existing PR. CI remains observable on
that PR; no unobserved remote result is asserted here.

Managed cleanup verified: mantle-agent-acceptance archived its ignored run/generated-state records
under $HOME/.local/state/worktree/archives/mantle/mantle-agent-acceptance; both that unit and
mantle-final-documentation then finished and passed exact-id GC dry-run/apply. Both source paths
were removed, and their branches deleted only after ancestry to the advertised integration ref
was proved. Their own leases and processes had ended. Exact assigned reproducible target/TMPDIR
paths were removed; raw reports, regression logs and baseline executable remain in private scratch.
The integration tree remains owned by codex-reliability-root through final CI, after which its
ignored evidence will be archived and the published tree retired through the same managed path.
