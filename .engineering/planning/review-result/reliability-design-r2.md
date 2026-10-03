---
format: aep.planning-md/3
id: review-result:reliability-design-r2
kind: review-result
status: active
title: Reliability plan design review round 2
relations:
- reviews: story:controlled-worker-upgrades
revision: 1
---
needs-revision
story:controlled-worker-upgrades — The upgrade transaction specifies an idle check before mutation but no admission exclusion across the check and service transition, allowing a concurrent new execution to be interrupted despite the active-session preservation promise — .engineering/planning/story/controlled-worker-upgrades.md:41
Read: all seven stories, operator model, six implementation dependency edges, seven decomposition edges, parent informed_by edge, existing installation flow, SDK public methods, and daemon routes.
Could not establish: a supported admission barrier or equivalent host maintenance protocol that covers already connected clients; a local Mantle lock alone does not cover other Substrate consumers.
```findings
- file: .engineering/planning/story/controlled-worker-upgrades.md
  line: 41
  category: design
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: The upgrade transaction specifies an idle check before mutation but no admission exclusion across the check and service transition, allowing a concurrent new execution to be interrupted despite the active-session preservation promise
```

