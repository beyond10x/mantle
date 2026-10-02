---
format: aep.planning-md/3
id: story:codex-gateway-destinations
kind: story
status: active
title: Admit fixed Codex subscription destinations through the existing gateway
relations:
- decomposes: story:codex-interactive-start
- depends_on: story:attach-backpressure-cancellation
- depends_on: story:codex-session-wiring
scope:
- confidence: cited
  path: crates/mantle-egress/src/allow.rs
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
  path: spec/conformance-baseline.json
- confidence: cited
  path: spec/domains/egress.yaml
- confidence: cited
  path: spec/ess-inputs.yaml
- confidence: inferred
  path: spec/scenarios/egress/codex-connect-destinations.yaml
- confidence: inferred
  path: spec/scenarios/egress/codex-default-destinations.yaml
- confidence: inferred
  path: spec/scenarios/egress/codex-destination-refusals.yaml
revision: 18
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T16:25:14Z", actor: "human:timo", revision: 16, decided_on: {"recorded":{"review_outcome":1}}}
- {from: "proposed", to: "active", at: "2026-10-02T16:44:05Z", actor: "human:timo", revision: 18, decided_on: {"recorded":{"review_outcome":1}}}
---
## Context

The existing worker-local gateway and aperture are already wired, but its compiled default allowlist lacks the two fixed destinations selected by pinned Codex0.153.4. This local policy unit makes that route ready for Codex; it neither changes Substrate nor claims real authentication/network acceptance. Actual active-worker deployment remains with the parent because worker::install_binaries deliberately defers upgrading a running shared gateway.

Source-scoper at79e1ee6/229f877 cites crates/mantle-egress/src/allow.rs DEFAULT_ALLOW and Allowlist::permits, existing CheckDefaultDestination in spec/domains/egress.yaml, and the real Exchange fixture in crates/mantle-egress/src/conformance.rs:42–142. Pinned Codex login/src/server.rs:59 and login/src/device_code_auth.rs:166 use auth.openai.com for device login/polling/token exchange/refresh; model-provider-info/src/lib.rs:290 selects chatgpt.com/backend-api/codex for subscription inference, with ChatGPT backend services on the same host. These are static source-established requirements, not a complete observed live traffic inventory. Browser-side authorization dependencies are outside worker policy.

## Acceptance

GW01–GW04 pass through the production default policy and real CONNECT handler, demonstrating admission of exactly auth.openai.com:443 and chatgpt.com:443 alongside the unchanged eight existing destinations while preserving exact-host, address, DNS, opaque-byte-relay and resource defenses.

## Named conformance scenarios

- GW01 fixed-default-destinations: the real default policy accepts each new exact normalized host on443, retains all eight existing entries, and introduces no wildcard, alternative port, API-key endpoint or caller-controlled expansion. Update the existing exact-count assertion intentionally rather than dropping it.
- GW02 real-connect-destinations: each new host exercises the existing real Exchange handler with controlled DNS/dial boundaries; observe the expected fully qualified resolver name, pinned public dialing, successful CONNECT and unchanged opaque binary payload bytes. No reference-only adapter result substitutes for actual handler execution.
- GW03 pre-dial-refusals: wrong ports, lookalike/suffix authorities and unselected api.openai.com refuse before resolver/dial activity. Preserve existing authority normalization; do not invent different host parsing rules for Codex.
- GW04 retained-defenses: all existing private/reserved/mixed-DNS, IP-literal, suffix/port, rebinding-resistant public dialing, parser/connection/output limits, idle timeout and shutdown/drain cases remain green. The gateway remains shared across the worker; no per-agent destination-isolation claim is added.

## Boundaries and sequencing

Extend the existing ESS allowed-host guard and authored scenarios before runtime default changes. This changes values in existing typed policy/commands; no new product noun, proxy abstraction or configurable credential scheme is introduced. All permanent runnable code is Rust. Source-only scope; no live gateway restart, VM modification, Substrate feature, browser automation, login, real credential or model call. Existing configured-worker policy is distinguished from live gateway health. Do not add speculative CDN/wildcard hosts to obtain a green result.

This follows story:attach-backpressure-cancellation because shared ESS inputs/inventory/full-spec generated provenance serialize the source units; it also consumes implemented story:codex-session-wiring. The parent consumes both local wiring units and remains responsible for usable startup, transport policy, controlled deployment, real traffic observations and end-to-end CS acceptance. Selecting fixed source-established destinations makes that future observation possible without claiming it already happened. The open transport clarification does not change whether these fixed provider destinations are required.

## Scope

Confidence high for cited production seams; the three authored scenario files remain inferred until implementation. Existing conformance.rs already supports these real handler scenarios and is not reserved for edits absent demonstrated need. Full-spec generated-model drift obliges regeneration, with only actual generated changes retained.

- cited: crates/mantle-egress/src/allow.rs
- cited: spec/domains/egress.yaml
- inferred: spec/scenarios/egress/codex-default-destinations.yaml
- inferred: spec/scenarios/egress/codex-connect-destinations.yaml
- inferred: spec/scenarios/egress/codex-destination-refusals.yaml
- cited: spec/ess-inputs.yaml
- cited: spec/conformance-baseline.json
- cited: spec/README.md
- cited: generated/worker-model/Cargo.toml
- cited: generated/worker-model/types.rs
- cited: generated/worker-model/source.schema.json
- cited: generated/worker-model/types-report.json

## Sources

Pinned official source: https://github.com/openai/codex/blob/rust-v0.153.4/codex-rs/login/src/server.rs#L59 ; https://github.com/openai/codex/blob/rust-v0.153.4/codex-rs/login/src/device_code_auth.rs#L166 ; https://github.com/openai/codex/blob/rust-v0.153.4/codex-rs/model-provider-info/src/lib.rs#L290 . Local provisioning references: worker.rs:78,291–363; deploy/mantle-egress.service:8; deploy/substrate.service:23. No source-scoper build or external traffic was run.

## Decomposition review

Round1 immutable reviews codex-gateway-{design,acceptance,scope,parallel}-r1 recorded three approvals and two acceptance findings; the two statements were consolidated around their existing named observable matrices through CLI body changes, and one fixed outcome was recorded per finding. No implementation or implemented state was reopened. Round2 immutable reviews with -r2 all return approve with empty findings. The parallel critic reissued its original report with an explicitly abbreviated personal home path for portable admitted evidence. The three-worker concurrency ceiling alongside active implementation required two dispatch batches; reviewers were independent and saw no other findings. Native aep critic role types/Sonnet unavailable: inherited generic role adapters read their full procedures. Live transport/authentication obligations remain parent-owned and unestablished. Validation after the batch:61 artifacts, valid.

## Scope confirmation at wave7 selection

Read-only aep:story-scoper againstde01750ff88db09c7caf33812a31e05c64766840 confirms all12existing entries sufficient:9cited paths and3inferred authored scenario paths. DEFAULT_ALLOW/count assertion atallow.rs:5,166 and CheckDefaultDestination guard ategress.yaml:71 are the policy seams. Existing conformance.rs:47–115 uses the real handler/default list and records controlled resolver names, public dial addresses and opaque bytes; no fixture extension is indicated. Worker lib.rs:1772–1816 compares all4whole-spec generated outputs, retaining only actual changes. No scope correction or new path was required. Confidence high, execution and live behavior unproven by this read-only pass.

Existing in-scope documentation correction: spec/README.md:71–72 still reports324 and213/66/45, whereas current baseline floors218/66/49 total333. Update its headline inventory to the actual fresh gateway results, not only append another paragraph. Source: gateway_parallel_critic returned scoper report2026-10-02; no source/test/build/AEP write by that agent.
