---
format: aep.planning-md/3
id: credential-blocker:operator-agent-login
kind: credential-blocker
status: open
title: Operator-held Claude token and Codex device login for live acceptance
relations:
- blocks: story:session-lifecycle
- blocks: story:codex-interactive-start
revision: 1
---
## Observation

The live acceptance of the Claude and Codex sessions needs credentials only the operator holds: the
Claude OAuth token delivered through the Substrate secret slot `claude`, and a ChatGPT device login
completed by the operator in the attached Codex terminal. No agent session may read, enter or
substitute either. The last recorded state (story:codex-interactive-start, "Post-release live
acceptance access", 2026-10-03) is that authentication confirmation was requested and no answer was
recorded.

## What it withholds

Same-conversation reattach and an authenticated model/tool turn for story:session-lifecycle, and the
authenticated CS-02, CS-03, CS-05, CS-06, CS-07 and CS-08 evidence of story:codex-interactive-start.

## What clears this

With a READY worker (environment-blocker:live-worker), the operator runs `mantle-acceptance` in its opt-in real
mode, completes the Claude and Codex logins and model turns in their own terminal, and records the
operator attestations. A run report with those attestations is the clearing evidence.
