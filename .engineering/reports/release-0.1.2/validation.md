# Mantle 0.1.2 release candidate

Base: 868d711fc8d8abb18484bec2696676a7644ea177, current remote main at preparation. Candidate adds only workspace version 0.1.2, six workspace lockfile versions, README/site/qualification updates and this release record. Compiler implementation and live evidence are already on the base.

Local checks: cargo fmt --check; locked workspace clippy with warnings denied; locked workspace Rust tests; native conformance aggregation; ESS validation; AEP validation; documentation build; and release musl builds for mantle-egress, mantle-launch and mantle-worker all passed. Native inventory: 336 passed, zero failed/skipped/unsupported/outside/refused. Rust suite: 187 top-level tests. ESS: 231 authored scenarios; generated scenarios bring execution to 336. Tool pins remain unchanged.

Documentation build initially refused a symbolic HEAD argument; rerunning with the full base revision passed. Publication rebuilds provenance against the exact committed candidate. The source review checked the six package version edits, documentation claims against the live acceptance audit, and absence of dependency updates. Review was a separate coordinator pass, not an independent subagent; prior agent capacity was exhausted.

Source release requires exact candidate Gate and common Security and privacy success, signed publication, bot tag and bot GitHub Release, and source archive verification. Completion evidence follows publication. Documentation publication is asynchronous and is not claimed by this preparation record.

Authenticated Codex device login, model/tool turns, token refresh and complete live lifecycle parity remain unverified; epic:codex-parity remains incomplete. This release does not depend on the separate Substrate capture feature.
