---
format: aep.planning-md/3
id: epic:reliability-and-usability
kind: epic
status: active
title: Deliver all five Mantle reliability and usability improvements
relations:
- informed_by: epic:codex-parity
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T08:32:53Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"review_outcome":2}}}
- {from: "proposed", to: "active", at: "2026-10-03T08:32:53Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"review_outcome":2}}}
---
## Outcome

Implement ALL FIVE roadmap priorities, explicitly selected by the operator after the initial1–2 recommendation: truthful command/reference behavior; named profiles and diagnostics; installable prebuilt releases and controlled upgrades; retained workspaces with separate destruction and honest restart; a repeatable real-agent acceptance suite. New typed local values are in spec/domains/operator.yaml, validated before decomposition. Existing Session and Worker entities in spec/domains/session.yaml own lifecycle and placement; retention extends them instead of inventing a parallel workspace registry. Design choices are proposals reviewed under the operator's standing approval.

## Acceptance and delivery

Seven sequential reviewed units cover all five roadmap items. Each delivers named native ESS scenarios and meaningful process or filesystem regressions. All commits integrate into integration/mantle-reliability and exactly one final pull request to main. No intermediate PR, main merge or new release. The operator approved all waves in advance, confirmed all five priorities and unlimited budget. Primary checkout, live sessions and credentials remain untouched.

## Scope boundary

The operator answered All five roadmap items. The initial1–2 selection is superseded. Deliver all seven implementation units to the single integration branch and exactly one final PR. This task builds release packaging and upgrade capabilities; it does not publish a new tagged product release or alter the user's running workers. The real-agent suite must distinguish passed/failed/operator-required/not-run, never substitute mock passes for real authentication. Its repeatable implementation and controlled tests are deliverables; it does not retroactively claim completion of the older epic's unobserved authenticated matrix. No credential copy, new cloud spend, capture-policy change or confinement relaxation.
