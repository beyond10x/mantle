---
format: aep.planning-md/3
id: review-result:reliability-acceptance-r2
kind: review-result
status: active
title: Reliability plan acceptance review round 2
relations:
- reviews: story:controlled-worker-upgrades
revision: 1
---
needs-revision
story:controlled-worker-upgrades — The acceptance requires observing executions absent from Mantle records but names no complete inventory oracle supported by the pinned Substrate SDK, so upgrade-active-refusal cannot establish worker-wide absence — .engineering/planning/story/controlled-worker-upgrades.md:41
Read: all seven stories and expanded epic through `aep plan artifact show`, operator types, worker deployment, and pinned SDK/daemon routes; the SDK exposes get_exec(id), while daemon routes expose POST /v1/execs but no GET inventory.
Could not establish: a supported complete inventory mechanism; a permanently refusing implementation would not satisfy upgrade-apply.
```findings
- file: .engineering/planning/story/controlled-worker-upgrades.md
  line: 41
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: The acceptance requires observing executions absent from Mantle records but names no complete inventory oracle supported by the pinned Substrate SDK, so upgrade-active-refusal cannot establish worker-wide absence
```

