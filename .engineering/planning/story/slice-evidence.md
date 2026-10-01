---
format: aep.planning-md/3
id: story:slice-evidence
kind: story
status: draft
title: Record the vertical-slice evidence
relations:
- decomposes: epic:vertical-slice
- depends_on: story:session-lifecycle
revision: 1
---
# Story: Record the vertical-slice evidence

## Acceptance

`STATUS.md` holds one dated row per slice check (design § 41 items 1–9, 11–13, 15, plus teardown),
each naming the command it was read from and its observed result, including the checks that
failed or were not run. Laptop and worker CPU during one `cargo check` of the Substrate repository
are recorded as numbers.

## Scope

- `STATUS.md`
