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
