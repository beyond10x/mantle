---
format: aep.planning-md/3
id: story:attach-backpressure-cancellation
kind: story
status: implemented
title: Keep attached terminals cancellable under backpressure
relations:
- decomposes: story:dual-agent-session-parity
- depends_on: story:codex-session-wiring
scope:
- confidence: cited
  path: crates/mantle-launch/src/attach.rs
- confidence: cited
  path: crates/mantle-launch/src/sys.rs
- confidence: cited
  path: crates/mantle-launch/tests/conformance.rs
- confidence: cited
  path: crates/mantle-launch/tests/support/probe.rs
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
  path: spec/ess-inputs.yaml
- confidence: cited
  path: spec/scenarios/launch/attach-backpressure-cancellation.yaml
revision: 22
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T16:02:44Z", actor: "human:timo", revision: 18, decided_on: {"recorded":{"test_result":1}}, executor: "agent:codex-mantle"}
- {from: "proposed", to: "active", at: "2026-10-02T16:02:44Z", actor: "human:timo", revision: 19, decided_on: {"recorded":{"test_result":1}}, executor: "agent:codex-mantle"}
- {from: "active", to: "implemented", at: "2026-10-02T16:38:47Z", actor: "human:timo", revision: 22, decided_on: {"recorded":{"test_result":2,"review_outcome":1,"verification":1,"ess_conformance_coverage_v1":1}}}
---
## Context and measured failure

This is the independently testable attachment portion of DP02/DP03 under story:dual-agent-session-parity. It does not depend on Substrate capture or real authentication. The existing launcher uses blocking write_all in attach.rs:84–90, while termination is checked outside the relay and signal handlers use SA_RESTART. A source-scoper identified the gap; the coordinator then reproduced it against the unchanged wave4 launcher at79e1ee6.

A private synthetic real-process probe starts serve with bounded1024-byte replay and a synthetic yes child, attaches with an open input pipe, and compares drained versus unread output. Both clients are sent SIGTERM after600ms. Drained output exits within1500ms; unread output does not. The stalled client is killed/reaped, then the server is terminated/reaped. All owned processes were verified gone. This is an observed baseline failure, not yet a cause diagnosis or a fix. Initial fixture setup incorrectly waited for a stderr readiness line; that setup failure was corrected to observe the actual ctl FIFO readiness boundary and retained separately.

Baseline output and source are retained in ~/.cache/mantle-wave5/scratch-integration/debug-scaffolding/attach-cancellation. Command ./probe <that-private-root> exits1:

```text
drained=true termination_within_1500ms=true
drained=true client_and_server_reaped=true
drained=false termination_within_1500ms=false
drained=false client_and_server_reaped=true
```

## Acceptance

An attached client under output or agent-input backpressure responds to handled termination within a finite documented bound, restores inherited terminal/descriptor state, releases its client lock, and permits reattachment to the same surviving server/agent. Normal relay preserves byte ordering with bounded buffers. Named AB scenarios exercise actual launcher processes; no fake-agent result claims Claude/Codex conversation parity.

## Named conformance scenarios

- AB01 stalled-terminal-output: real attach with unread output receives SIGTERM and SIGHUP in separate cases; exits within a bounded deadline while the server/agent remain alive and a replacement attach acquires the lock. Include a drained-output control and finite fixture output.
- AB02 stalled-agent-input: real attachment with a full agent-input path remains cancellable under the same handled signals; no unbounded memory queue or busy polling, and later attach remains usable.
- AB03 terminal-restoration: a real PTY fixture observes original termios restored after handled cancellation while output is stalled; inherited file-status flags are restored too. Check child/server survival and client lock release through actual subsequent attachment.
- AB04 normal-relay: bidirectional data across short writes retains exact order and bytes, resize remains serviceable, existing duplicate-attach, same-size redraw, slow-reader and volatile-replay assertions stay green.

## Diagnosis and implementation boundary

Ranked falsifiable hypotheses, before additional probing: (1) blocking write_all prevents the event loop from observing the recorded signal; make only the stalled destination drain after the signal and predict the client then exits; (2) continuous agent output starves the loop even without blocking writes; replace the infinite trigger with a finite bounded producer and predict a different outcome if starvation is the cause; (3) signal handling was not installed when the signal arrived; observe established FIFO transfer/readiness before signaling and predict the failure disappears if startup ordering is the cause. The existing drained control already weakens hypotheses2/3 but does not by itself prove the implementation cause.

Implement the regression at the real launcher seam before changes. Prefer bounded nonblocking relay and poll readiness, with careful restoration of any changed inherited descriptor flags. Do not alter global signal semantics or the FIFO wire protocol solely to satisfy a fixture. Reuse existing launch contract/observation conventions; extend ESS and named scenarios before runtime implementation, validate the specification, regenerate existing provenance through pinned ESS0.50. No new product entity or generic session abstraction is introduced.

This story owns launcher cancellation, not the laptop async terminal loop, new Substrate features, authentication, cloud workers or live TUI rendering. Broader DP acceptance stays with the parent. Preserve the user's current sessions. All permanent runnable code is Rust. The shared launcher/spec/model surfaces serialize this unit after the active Codex session wiring wave. One child extraction under parity does not need the multi-child decomposition panel; independent scoping and an implementation adversary still apply.

## Scope

Final implementation confirms13 changed paths within the14 reserved at selection; generated/worker-model/Cargo.toml was checked by the real drift test but remains byte-identical, so it is removed from actual typed edit scope. The formerly inferred spec/scenarios/launch/attach-backpressure-cancellation.yaml now exists and is cited. Runtime edits are only attach.rs and sys.rs; test fixtures remain conformance.rs and support/probe.rs. First adversary added129 lines only to the already-scoped conformance.rs and reports75 launcher Rust tests/49native green. No server, protocol, signal-policy, Substrate or worker runtime change was required. Sources: implementation.md in .engineering/reports/attach-cancellation-wave6-2026-10-02 and the exact staged/unstaged unit diff at first review handoff.

- cited: crates/mantle-launch/src/attach.rs
- cited: crates/mantle-launch/src/sys.rs
- cited: crates/mantle-launch/tests/conformance.rs
- cited: crates/mantle-launch/tests/support/probe.rs
- cited: spec/domains/launch.yaml
- cited: spec/components.yaml
- cited: spec/ess-inputs.yaml
- cited: spec/scenarios/launch/attach-backpressure-cancellation.yaml
- cited: spec/conformance-baseline.json
- cited: spec/README.md
- cited: generated/worker-model/types.rs
- cited: generated/worker-model/source.schema.json
- cited: generated/worker-model/types-report.json

## One-variable diagnostic result

After the stalled-output client missed its1500ms SIGTERM deadline, the coordinator changed only the unread stdout pipe to an actively drained pipe. The same client then exited within1500ms, with no second signal, while the producer remained running. This supports hypothesis1: the pending signal is observed only after the blocked write makes progress. Continuous-output starvation and an uninstalled signal handler do not explain the immediate response to draining alone. Both clients and servers were reaped; source/binaries remain explicitly debug scaffolding for the next unit to turn into a regression.

```text
drained=true termination_within_1500ms=true
drained=true client_and_server_reaped=true
drained=false termination_within_1500ms=false
stalled_client_exits_after_only_draining_output=true
drained=false client_and_server_reaped=true
```

Raw drain-probe.log/drain-probe.exit and revised probe.rs are beside the retained probe-baseline.rs/probe-baseline and baseline.log. Exit1 remains intentional: the original required bounded-cancellation assertion is still red. No launcher source was changed and no agent/worker/network authentication was used.

## Scope confirmation at wave6 selection

Read-only story-scoper rechecked local main229f877 after Codex wiring merged. All14 exact scope entries remain sufficient:13 existing paths cited and only the new scenario path inferred. Current native fixtures include LimitedOutput at conformance.rs:489, ReplayFixture at539, Process cleanup at112, ready at389 and finite Replay/Flood probe modes. Reuse these rather than introducing another support file.

The unchanged worker generated-model drift gate at mantle-worker/src/lib.rs:1772–1816 compiles the whole spec and compares all four portable model artifacts; keep all four in scope and regenerate only through pinned ESS0.50. Expected runtime implementation remains attach.rs plus sys.rs, with bounded directional buffers, POLLOUT only while bytes are pending, and RAII restoration of exact inherited F_GETFL flags. Current evidence does not require serve.rs or global signal-policy changes. The ESS/native paths provide the new AB acceptance before runtime edits. Source-scoper confidence remains high; actual cancellation success is still to be implemented and tested.
