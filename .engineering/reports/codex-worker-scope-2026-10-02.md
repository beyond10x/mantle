# Worker wave reconciliation — read-only scoper, 2026-10-02

Read the installed story-scoper procedure at `skills/implementing/references/story-scoper.md` (the supplied `skills/planning/references/story-scoper.md` does not exist). Read worker/start/qualification/epic bodies and current production source. Incoming contract is pinned to `2e6b1162a78855bfc7a28fc899e8300fcc229deb` (published source; compared relevant files with `87bed8f`). No build, store mutation, implementation edit or live-worker action was performed. This private report is the only write.

## Reconcile ESS before the next wave

1. Take the incoming accurate Session/Worker/SourceResolution contract. Session's production names are `worker`, optional `workspace`, optional `agent_exec`, `requested_json`, `created_at`, optional `failure`; retain its seven-state lifecycle, local write command outcomes and two views. SourceResolution is a value struct with explicitly unmapped compound SQLite identity. Worker remains the stored worker record, not an observed ready machine.
2. Keep `AgentKind` and `AuthenticationMethod` enum vocabulary with an explicit proposal comment; remove the provisional optional fields from the served Session entity for this merge. Amend the epic/specification paragraph and CS-01 to say the **future DB migration** supplies Claude defaults to old records. Add real persisted fields, constructor inputs, command assignments and both view projections together in the interactive-start implementation. No placeholder values in the adapter. Incoming adapter's `snapshot_complete_subject` checks every specified field exists in the row (`conformance.rs:256–282` at incoming commit), so retaining schema-only optional fields is already inconsistent with its honest boundary.
3. The worker story's contract should express pure readiness/provisioning decisions and observed result structures without changing WorkerRecord into liveness state. Current ESS explicitly marks orchestration unmapped. Expand that frontier only as actual Rust seams become testable. A new worker domain is unnecessary for this small unit: names under `mantle.session` can describe worker readiness commands/results beside Worker. Proposed stateless EvaluateWorkerReadiness/SelectProvisioningAction commands follow the existing `CheckDefaultDestination` accepts/refuses shape; actual names and input types need ESS authoring before implementation. Do not claim a model-generated success proves remote download/systemd behavior.
4. Existing incoming `ess_generated_local_conformance` synthesizes into git-ignored `.engineering/drafts/mantle-conformance.json`, executes every generated scenario and pins count57 plus two named synthesis refusals. Extend this adapter and update the exact reviewed obligation/refusal set when adding AW. Do not create a generic `generated/` tree or duplicate runner. Existing workspace cargo-test gate already runs this test; Taskfile needs no worker-specific edit if this integration is used. Author `spec/scenarios/agent-worker.yaml` and explicitly include it in `spec/ess-inputs.yaml`. If ESS cannot express the OS effects, name the actual Rust regression/live evidence separately and retain the specification's unmapped boundary.

## Recommended next-wave behavior

- Make `[claude]` optional for common config parsing. Calling `claude_token()` with absent config still fails clearly when Claude is selected. Parsing an old Claude configuration remains unchanged. Avoid consulting the token command merely for worker status or a credential-free worker.
- Split common readiness from selected-Claude readiness. Preserve all existing common facts (argv-only, no direct egress, process/memory cgroups and kill, PTY, egress aperture). Move only the slot requirement to a Claude-specific check called before creating a workspace. A missing executable must likewise be detected before creating a session. Installed version and successful common readiness never mean authenticated Codex.
- Fresh credential-free unit omits BOTH `ConditionFileNotEmpty` and `--secret-slot claude=...`. The pinned daemon validates every configured slot at start, so removing only the systemd condition fails. Render the unit's optional Claude block from provisioning intent, not from a dummy token file.
- Fresh legacy-config mode can retain the conditional slot unit: cloud-init leaves it stopped until the real token is securely installed by `worker up`, then `systemctl start` activates it. Fresh no-Claude mode starts without the slot. This preserves first provisioning for legacy users while making no-Claude readiness real.
- Existing active daemon: preserve its installed unit and advertised slots; never overwrite the unit using a newly absent Claude config. Existing slot token rotation remains atomic and does not restart the daemon. If a previously credential-free daemon lacks the slot, selected-Claude setup must report the explicit maintenance prerequisite. It may stage a real token/private config, but must not claim slot availability until a new machine observation proves it. Never auto-restart an active daemon merely to add the slot. This rule avoids pretending local SQLite live-session records enumerate every remote exec.
- Fix existing `install_binaries` unconditionally restarting `mantle-egress.service` (`worker.rs:269–274`). An unchanged active gateway needs no restart. A changed gateway/launcher binary must not become an incoherent active-runtime/status combination: prefer deferring changed shared binaries while the service is active with an actionable prerequisite. Adding Codex alone must leave gateway/substrate process identities and the live Claude exec unchanged. Atomic rename preserves existing process mapping but gateway restart would close its established CONNECT streams.
- Install exact Codex0.153.4 official binary, verify compressed artifact digest and decompressed binary digest before activation, check worker architecture and exact `--version` with clean environment, and update capability facts only after successful atomic publication. Use distinct private temporary names and bounded transfer/time limits. Failed/truncated/wrong-version update leaves previous binary and facts intact. Re-running up on an existing VM must execute this path, since cloud-init is not rerun. Shared EC2/KubeVirt render paths must name the same pin. No Substrate compilation; correct stale progress text in worker.rs:192,226 that currently says it builds from source.
- Official identity supplied by coordinator: compressed SHA256 `c485e889611b73ff5c3cc11fb5cea7551ef504465ad8675163766b9b1a9ec84a`, binary SHA256 `56ef98ab4032d317ab26e9b5e5a175650717351edb16ed9cde0cb6d1734d62da`. Treat this as installed candidate, not support certification.
- Keep implementation in Rust, clap derive for any new CLI; declarative cloud-init/systemd edits are permitted. Do not add a shell/Python installer. The current app drives SSH shell commands; if implementation needs a new lasting remote program rather than bounded existing command primitives, rescope a Rust worker helper before creating it instead of quietly committing an embedded shell program.

## Dependencies and acceptance circularity

The qualification story's explicit deliverable is an exhaustive report with pass/refusal/not-run, and its CQ-06 specifically permits not-run. Its `implemented` state therefore can mean the harness/report obligation is complete while runtime support remains unqualified. Interactive-start can depend on this completed investigation plus the worker story. Preserve a separately named live-acceptance prerequisite for CQ-04/CQ-06 and the chosen profile/approval behavior.

The current prose “Missing user authentication ... is a recorded blocker on codex-interactive-start” becomes circular if interpreted as forbidding construction of the login plumbing needed to perform authentication. Clarify through AEP: missing operator auth blocks **live acceptance/completion**, not implementation of safe auth/terminal plumbing. A mandatory socket refusal or proven confinement violation remains a design blocker. Production routing/default activation is gated on observed compatibility; the diagnostic outer-only profile currently proves only direct shell/Cargo controls, not authenticated TUI tool use or approval prompts.

No need to split or weaken the final epic: it still requires all CQ/AW/CS/DP live obligations passing. It is legitimate to implement CS plumbing and retain its active status pending operator device login. Endpoint discovery can use a disposable exact-host gateway before final production allowlist admission; do not widen production policy speculatively.

## Scope

```markdown
## Scope

Derived 2026-10-02 by `story-scoper` against incoming ESS commit `2e6b116`; every entry distinguishes inspected source from proposed placement — cited.

- **Primary surface:** `crates/mantle/src/app/worker.rs` — cited; `up`, `install_binaries`, `install_token`, `report_identity`, `report_machine`, `render_user_data` and their tests own this behavior.
- **Configuration:** `crates/mantle/src/config.rs`, `examples/config.toml` — cited; make common provisioning independent of the required Claude table while preserving selected-token errors and legacy parsing.
- **Common and selected readiness:** `crates/mantle/src/adapters/substrate.rs`, `crates/mantle/src/app/session.rs` — cited; split shared `missing_facts` and preserve Claude slot validation before workspace creation.
- **Bootstrap and daemon:** `deploy/cloud-init.yaml`, `deploy/substrate.service` — cited; render credential-free versus selected-Claude startup, verified pinned candidate installation and observed bootstrap identity.
- **ESS contract:** `spec/domains/session.yaml`, `spec/components.yaml`, `spec/ess-inputs.yaml` — cited; incoming files own worker vocabulary, command ownership and explicit authored scenario inputs.
- **Authored AW scenarios:** `spec/scenarios/agent-worker.yaml` — inferred; new named obligations for readiness/provisioning decisions, with OS/live cases represented honestly as external evidence.
- **Actual adapter:** `crates/mantle/src/adapters/conformance.rs` — cited; extend the incoming adapter's dispatch, actual Rust observations, exact count/refusal accounting and scenario coverage.
- **Contract coverage documentation:** `spec/README.md` — inferred; incoming mapped/unmapped coverage frontier must explain the added decision seam and remaining live provisioning effects.
- **Tests and generated artifacts:** tests remain in the owned Rust modules and existing conformance adapter; synthesis remains in the existing ignored drafts directory — inferred; remove provisional `crates/mantle/tests/agent_worker.rs` and `generated/` reservations.
- **Gate:** existing workspace cargo-test already executes the conformance adapter; no Taskfile change required for this implementation design — cited.
- **Confidence:** high — cited; all production ownership and the incoming runner are inspected; only the authored AW scenario/documentation placement is proposed.
- **Would collide with:** worker/config/session readiness, bootstrap/service configuration, and session-domain/conformance input/adapter changes — cited.
- **Safety fact:** `worker.rs:269–274` restarts a shared gateway on every rerun, while `session.rs:78` shares the credential-dependent readiness function with worker readiness; both must be changed together before AW-01/AW-03 can pass — cited; proof level2, unproven by this read-only pass.
- **Safety fact:** incoming Worker is a local stored record and its specification explicitly excludes provider/daemon liveness; new observed readiness must not be inferred from that row — cited; proof level2, unproven by this pass.
```

```sh
aep plan artifact scope story:agent-ready-worker --add crates/mantle/src/app/worker.rs
aep plan artifact scope story:agent-ready-worker --add crates/mantle/src/config.rs
aep plan artifact scope story:agent-ready-worker --add examples/config.toml
aep plan artifact scope story:agent-ready-worker --add crates/mantle/src/adapters/substrate.rs
aep plan artifact scope story:agent-ready-worker --add crates/mantle/src/app/session.rs
aep plan artifact scope story:agent-ready-worker --add deploy/cloud-init.yaml
aep plan artifact scope story:agent-ready-worker --add deploy/substrate.service
aep plan artifact scope story:agent-ready-worker --add spec/domains/session.yaml
aep plan artifact scope story:agent-ready-worker --add spec/components.yaml
aep plan artifact scope story:agent-ready-worker --add spec/ess-inputs.yaml
aep plan artifact scope story:agent-ready-worker --add spec/scenarios/agent-worker.yaml --inferred
aep plan artifact scope story:agent-ready-worker --add crates/mantle/src/adapters/conformance.rs
aep plan artifact scope story:agent-ready-worker --add spec/README.md --inferred
```

The coordinator should remove the stale reservations noted above using the installed CLI's supported removal form; commands above are not run by this scoper.

## Not established

- No live no-Claude fresh worker exists in the evidence inspected; AW-01 needs a new isolated provisioned worker rather than stopping the existing Claude worker.
- No proof that full Codex package resources beyond the standalone binary are unnecessary after authentication; qualification currently tests the standalone candidate.
- No model-issued tool command or approval prompt has been observed under the explicit outer-only profile.
- No device authentication, renewal, HTTPS/WSS/fallback endpoint set or credential expiry has been observed.
- Exact ESS authored-scenario syntax for OS-side effects was not established; never substitute a reference result or broad skipped scenario.
- If avoiding permanent shell programs requires a new Rust helper/CLI, its files and Cargo/Taskfile/main registration are outside the narrow scope above and must be added before dispatch; do not smuggle this architectural decision into implementation.

# Follow-up: smallest complete Rust installer boundary

## Choice

Use one new, small `mantle-worker` Rust library + clap binary, shipped in `--binaries` alongside the two existing worker binaries. The helper owns the complete Codex installation transaction on the worker. Do not put Codex installation in cloud-init: both fresh and existing workers call the same helper after common bootstrap and before common readiness reports a Codex capability. A second `inspect` command returns bounded typed installed facts without network access. No generic worker-agent framework or user-configurable download URL is needed.

This is smaller in correctness surface than a laptop-only installer: the latter needs several SSH transactions for remote staging, architecture/version checks, publication and fact writes, plus equivalent remote coordination/rollback logic to survive a lost connection. Sending a verified laptop buffer still does not make the remote activation/facts update atomic. The helper also avoids buffering the observed 258,659,424-byte uncompressed binary in the laptop CLI.

## One coherent publication

- Root-owned fixed tree `/opt/mantle/agents/codex/`; reject existing unsafe/symlink directory components before writes. Hold an exclusive installation lock (Rust standard File locking) for the transaction; concurrent invocations wait only a bounded time or report busy.
- A generation is an immutable digest-named directory containing `codex` and `manifest.json`. Its manifest contains version, architecture, compressed digest and binary digest, with a format version; no credential/config content. Private random/create-new staging directory is on this same filesystem.
- Stable `/opt/mantle/bin/codex` points through `../agents/codex/current/codex`; a stable agent-bootstrap-facts path can likewise point through `current/manifest.json`. Prepare missing stable links before capability publication, refuse conflicting existing unexpected files, and preserve legacy valid installations until successful conversion.
- Download/decompress/hash/version-check/fsync both generation files, then rename the generation directory into place. Atomically replace one `current` symlink (same-directory temporary symlink + rename). That single switch changes both executable and its facts. A separate binary rename followed by an independent `bootstrap.json` rewrite has a crash window and is not an atomic result.
- Failed transfer, digest/version mismatch, oversized input or preactivation failure leaves the prior `current` pointer unchanged. Remove only this invocation's private staging tree. Do not delete an old generation during installation; running processes can still use it. No service operation belongs to this helper.
- `inspect` resolves the current generation once while holding the same lock and checks its metadata/content identity before returning available capability. A second call finding the valid desired generation does no download and no pointer change. Do not regard manifest text alone as verification of an externally replaced binary.
- Existing base `bootstrap.json` remains common image/toolchain evidence. Record agent installation as a distinct coherent bootstrap-facts manifest, and have laptop status render both. Clarify AW-02 wording accordingly rather than duplicating mutable installed facts across two unrelated atomic files. Shared cloud-init rendering can include desired pin metadata if desired, but must distinguish intended from installed identity.

## Bounded implementation and dependencies

Minimal library dependencies: workspace `anyhow`, `serde`, `serde_json`; `sha2` for both streams; a synchronous HTTPS client (`ureq` with defaults disabled and rustls enabled); Rust `zstd` decoder. Binary adds workspace `clap`. A private staging helper may use the already-locked `tempfile` crate rather than custom randomness. `ureq` 3.4.2 source is locally available and explicitly exposes HTTPS-only redirects, redirect count, global request timeout and streamed body reads; implementation should use those controls. New library versions must be resolved and recorded normally in Cargo.lock; this scoper did not resolve or build them. Rust zstd's bundled C library is an upstream dependency, not a new committed shell/C program; verify static musl build in the worker build gate.

Use exact trusted official release URL and pinned digest from qualification, HTTPS-only redirects with a small count, 180s request total timeout, 128MiB compressed byte ceiling and 512MiB decompressed ceiling (the observed binary is about247MiB), checking one byte beyond each limit to distinguish exact bound from truncation. Bound zstd decoder window memory as well as output; hash/verify the compressed artifact before decompressing or executing it. Store archive and output on disk, not giant Vec buffers. Verify ELF/target or worker architecture before execution. Run only the authenticated candidate's `--version`, environment cleared, fixed HOME/PATH, stdin null, 5s deadline and e.g.4KiB total output bound. Unsupported architecture refuses before the request. All numeric bounds belong to typed Rust constants/policy, not shell loops.

Network, decompression and version failures must map to bounded typed errors without dumping body/terminal content. A corrupted upstream download is not a reason to use host Codex or a floating version. Install scope may use direct worker HTTPS because this trusted host operation is not a sandboxed agent invocation; do not claim it qualified gateway inference traffic.

Existing `Ssh::run` is unbounded, writes stdin synchronously and then accumulates stdout/stderr with wait_with_output. Do not use it unchanged to upload the new helper and then call an allegedly bounded installer. Add a narrowly used bounded SSH operation with a transfer/deadline budget, bounded concurrent readers and writer, and exact child cleanup on timeout. Tokio is already present. For guaranteed Unix process-group cleanup without violating unsafe-code denial, add `nix` with signal/process features; apply only to owned SSH children and their ProxyCommand descendants. Existing run behavior can remain for unrelated callers; worker installation must use the bounded variant. Upload helper via existing fixed primitive calls (`install`, `tee`, `chmod`, `mv`), coordinated from Rust, not an embedded shell installer. Keep helper-size limit (e.g.64MiB) and verify the uploaded helper digest before executing it. These few trusted bootstrap primitives solve only delivery of the Rust helper, not Codex installer logic.

## Exact added/expanded source paths

New paths (inferred until implementation creates them):

- `crates/mantle-worker/Cargo.toml`: small library/binary dependencies.
- `crates/mantle-worker/src/lib.rs`: pin, manifest, install/inspect transaction, production decision functions, bounded transfer/decode/version helpers and inline regression tests.
- `crates/mantle-worker/src/main.rs`: clap install-codex/inspect-codex commands, fixed privileged production root, bounded JSON/error output. Test root injection belongs to library tests, not an arbitrary privileged path exposed by default.

Existing paths to add to the previously recommended AW scope (cited ownership):

- `Cargo.toml`: register new workspace member.
- `Cargo.lock`: resolved Rust HTTP/decompression and helper dependencies.
- `crates/mantle/Cargo.toml`: path dependency on helper library to reuse pin/fact types and real decision functions for conformance; optional safe process-group dependency for bounded SSH implementation. Avoid duplicating pin constants.
- `crates/mantle/src/adapters/ssh.rs`: bounded owned transfer/process operation.
- `crates/mantle/src/main.rs`: --binaries help now lists all three worker binaries.
- `Taskfile.yml`: add `-p mantle-worker` to static `build-worker`. Its existing workspace fmt/clippy/test already includes the helper.

Already scoped paths now have concrete additions: app/worker.rs reads/delivers the third binary, calls install then inspect on both fresh/existing paths, reports combined base+agent facts; conformance.rs calls real helper/readiness decisions; deploy/cloud-init.yaml does not download Codex; spec/session.yaml, components.yaml, inputs and AW scenarios cover the new decisions. No CI workflow change is required because current gate tests the whole workspace. Static helper build still needs explicit local validation; task check alone does not prove a musl artifact.

## ESS seams and evidence

Before code, define typed AgentInstallationFacts (version/architecture/digests), installation outcomes (already-current, installed, refused) and refusal reasons (unsupported architecture, digest mismatch, version mismatch, size/deadline exceeded, busy, unsafe destination). Keep an installation fact as an observed value, not a fictional database entity. Define command decisions only where inputs fully determine the real Rust behavior, and use the helper library as the implementation boundary.

Useful named AW cases: current valid generation avoids network; unsupported target refuses before transfer; bad compressed digest never executes/decompresses; decompressed digest mismatch never activates; version mismatch never activates; size/time-bound failures preserve current; duplicate invocation cannot race publication; compound binary/fact identity changes through one pointer; absent Claude configuration retains common readiness; active shared services produce preserve/defer action. Do not write an adapter that merely returns expected ESS outcomes without calling those real functions.

Filesystem/HTTP/process regression tests should exercise the actual helper library with task-owned temporary trees, controlled input readers/HTTP server and a benign Rust executable for wrong-version checks. No shell-script test fixture. Keep authenticated model interaction outside AW: final live evidence still must provision a fresh no-Claude worker and add Codex to an existing active Claude worker while proving existing exec/gateway/daemon identities and confinement unchanged.

## Scope command additions (not executed)

```sh
aep plan artifact scope story:agent-ready-worker --add crates/mantle-worker/Cargo.toml --inferred
aep plan artifact scope story:agent-ready-worker --add crates/mantle-worker/src/lib.rs --inferred
aep plan artifact scope story:agent-ready-worker --add crates/mantle-worker/src/main.rs --inferred
aep plan artifact scope story:agent-ready-worker --add Cargo.toml
aep plan artifact scope story:agent-ready-worker --add Cargo.lock
aep plan artifact scope story:agent-ready-worker --add crates/mantle/Cargo.toml
aep plan artifact scope story:agent-ready-worker --add crates/mantle/src/adapters/ssh.rs
aep plan artifact scope story:agent-ready-worker --add crates/mantle/src/main.rs
aep plan artifact scope story:agent-ready-worker --add Taskfile.yml
```
