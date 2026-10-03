---
format: aep.planning-md/3
id: review-result:reliability-acceptance-r1
kind: review-result
status: active
title: Reliability plan acceptance review round 1
relations:
- reviews: story:worker-doctor
revision: 1
---
needs-revision
story:worker-doctor — The acceptance does not name the compatibility oracle or distinguish the installed binary from the running daemon, so doctor-version-mismatch cannot establish which observation proves compatibility — .engineering/planning/story/worker-doctor.md:16
Read: three complete stories via `aep plan artifact show`, their cited production seams, and `spec/domains/operator.yaml`; inspected kinds and story lifecycle.
Could not establish: exact compatible runtime/wire requirements or authoritative running-version observation.
```findings
- file: .engineering/planning/story/worker-doctor.md
  line: 16
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: The acceptance does not name the compatibility oracle or distinguish the installed binary from the running daemon, so doctor-version-mismatch cannot establish which observation proves compatibility
```
