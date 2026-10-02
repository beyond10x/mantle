---
format: aep.planning-md/3
id: design:codex-terminal-confidentiality
kind: design
status: draft
title: Assess Substrate-owned terminal capture policy before Mantle integration
refs:
- provider: github
  reference: beyond10x/substrate#112
relations:
- designs: story:codex-interactive-start
- informed_by: coordination-blocker:codex-terminal-capture
- designs: story:codex-private-terminal
revision: 11
---
## Current decision — operator correction, 2026-10-02

Pause the Mantle encrypted terminal proposal. Assess a supported Substrate capture mode first. The previous no-Substrate-changes exclusion is withdrawn for this assessment. Standing wave approval does not make the previous implementation direction appropriate; no encryption implementation wave is selected.

The requirement is a confined interactive session whose terminal contents are streamed without durable recording, with lifecycle, exit status, resource observations and audit metadata retained. Substrate owns capture and persistence. Mantle should select an admitted mode through the public SDK. It must separately prevent its launcher and Codex diagnostics from persisting terminal credentials. The existing privacy requirement and Claude-equivalent terminal experience remain in force.

## Verified source assessment

Source reviewed at Substrate 05695970b069f79e6678f2f02cbd78bbe5fa2a56, the current Mantle dependency; this is local source evidence, not a claim about a newly fetched remote revision or a live no-capture capability.

- crates/substrate-wire/src/lib.rs:1911 PipeSessionStartInput supports pipe/PTY mode, input/frame/queue limits and window, but no capture selection. PipeSessionLimits at 2147 and the durable PipeSession resource are the public observation seams.
- crates/substrate-host/src/process.rs:2744 drain_capped both accumulates output and forwards the retained bytes. Its remaining allowance derives from stored.len(). Removing the accumulator without independent accounting would remove the output bound. run_child at 2162 uses this drain for both PTY and pipes.
- crates/substrate-daemon/src/app/sessions.rs:52 PipeSessionPolicy already has attachment capacity, message/write-buffer limits, a five-second send timeout and a one-hour attachment lifetime. These are reusable resource protections, not evidence that a no-capture mode exists.
- app/sessions.rs:1844 persists terminal observations. app/service.rs:589 also persists completed executions; lease cleanup at 637 has another completion path. A change confined to the attachment handler would miss durable sinks.
- crates/substrate-store/src/execs.rs:733 upserts stdout/stderr BLOBs. Capture policy must reach the driver observation and durable resource, not merely redact one HTTP response or delete the row afterward.
- crates/b10x-substrate-sdk/src/model.rs:682 exposes the current output bound; zero is refused. A zero limit is not a supported privacy mode.
- Mantle crates/mantle-launch/src/serve.rs:152 writes last-output after the agent ends. That remains a separate sink even with Substrate capture disabled.

## Candidate contract to evaluate before implementation

Substrate owns an explicit capture-policy request, policy admission, verified capability and observed effective mode. Names and wire representation are not yet selected. Existing requests retain their current recording behavior. A requested non-recording mode must fail with a named refusal if unsupported or disallowed; it must never silently record. Admission must happen before child dispatch and record non-content policy/audit facts durably.

Separate captured bytes, streamed-output accounting, per-frame and queue bounds, attachment capacity, backpressure deadlines and process resource limits. Define whether stream accounting is per execution or attachment, what counts when writes are partial or readers disappear, and what happens when each bound is reached. Preserve finite limits without introducing encryption overhead or an arbitrary smaller Mantle ciphertext budget. Do not claim unlimited streaming or automatic continuation.

Retain lifecycle, observed exit status, resource usage, lease/cancellation state and non-content audit metadata. The observation must distinguish deliberately unrecorded output from empty output and accidental loss. Output-query, reconnect, restart and diagnostic behavior need explicit semantics. No replay from Substrate storage should be implied for an unrecorded stream. Mantle's bounded live replay can preserve its session experience independently.

Substrate invariant 8 requires a design document or ADR before capability code. Its current published contract bundle is immutable; the API/SDK change needs a successor bundle with compatibility checks, capability negotiation and named refusals. Remote main checked at 6af1b91889edf5fa5455c03e68829b56e6b6cc56 has since adopted AEP v5 and an initial ESS operation-ledger specification under spec/. The runtime seams above are unchanged in that comparison. Upstream authors must extend their current ESS contract rather than using the older local checkout as the planning baseline. Do not invent a second contract authority or hand-edit generated models.

## Required proof

Use synthetic terminal canaries only. For both stdout/stderr and PTY output, prove terminal delivery while SQLite, live WAL, operation/audit payloads, daemon/driver diagnostics and crash/recovery artifacts contain no terminal payload. Inspect before checkpoint or retirement can erase evidence. Include success, child failure, cancellation, lease expiry, stream limit, slow/disconnected client, persistence error and daemon restart. Retained metadata must remain accurate on those paths. A recorded-mode positive control must detect the same canary to prove the sink scanner can fail.

Independently test queue high-water marks, streamed-byte accounting, stalled-reader deadlines, capacity recovery, whole-tree cancellation and unchanged confinement. Preserve the existing capture-default contract and old clients. Run real Substrate adapter/driver/store conformance rather than checking only serialized request fields.

Mantle acceptance then proves SDK selection/refusal, private launcher replay without last-output, Codex diagnostic suppression including repeated login/failure, and real interactive login/model/tools plus the original two-agent lifecycle matrix. Substrate completion alone cannot clear those Mantle obligations.

## Ownership and next action

Substrate capability implementation is tracked upstream at https://github.com/beyond10x/substrate/issues/112, filed as b10x-bot[bot] at the operator's explicit request. This Mantle effort does not take over Substrate implementation or modify its planning store. Substrate owns its capability, admission policy, SDK and conformance evidence; the issue names the required behavior and inspected source seams.

Mantle owns downstream SDK consumption and verifies the effective non-recording mode before any real login. Continue independent Mantle work while this dependency is unavailable. The next bounded candidate is an explicit launcher volatile replay policy: keep bounded live replay, prevent last-output persistence, preserve legacy defaults, and verify native failure paths. This is only one local sink; it cannot clear the upstream blocker or authorize real login by itself.

Upstream issue closure alone is insufficient. Clearance requires an exact supported Substrate/SDK revision, policy/capability behavior, sink and resource-bound evidence, and downstream synthetic canary acceptance. Public source writes are limited here to the explicitly requested issue; no push, release or deployment is authorized by this handoff.

## Superseded preparation

story:codex-private-terminal records the paused encryption candidate and is archived from scheduling. Its generated model and unapplied eleven-scenario proposal are retained outside the repository at ~/.cache/mantle-interactive-probe/paused-encryption-20261002 and ~/.cache/mantle-interactive-probe/private-terminal-spec.patch. The size probe report remains historical evidence only; it neither justifies the architecture nor proves terminal privacy. Draft encryption values have been removed from the active Mantle specification. No encryption runtime code was implemented.
