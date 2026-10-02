---
format: aep.planning-md/3
id: story:slice-evidence
kind: story
status: draft
title: Record the vertical-slice evidence
relations:
- decomposes: epic:vertical-slice
- depends_on: story:session-lifecycle
scope:
- confidence: cited
  path: STATUS.md
revision: 3
---
# Story: Record the vertical-slice evidence

## Acceptance

`STATUS.md` holds one dated row per slice check (design § 41 items 1–9, 11–13, 15, plus teardown),
each naming the command it was read from and its observed result, including the checks that
failed or were not run. Laptop and worker CPU during one `cargo check` of the Substrate repository
are recorded as numbers.

## Scope

Derived 2026-10-02 by `story-scoper`; confidence labels distinguish read evidence from inference.

- **Primary surface:** `STATUS.md` — cited, explicitly named by the story's acceptance and existing scope.
- **Files:** `STATUS.md` — cited; absent from the inspected checkout, so implementation creates it.
- **Symbols:** none — cited, acceptance requires recorded observations rather than code changes.
- **Documents:** `STATUS.md` only — cited; dated command/result rows for design § 41 items 1–9, 11–13 and 15, plus teardown, and numeric laptop/worker CPU observations during one Substrate `cargo check`.
- **Also likely:** no additional change surface — inferred; running and measuring the checks need not change repository code.
- **Confidence:** high — cited, the entire acceptance is an explicitly named evidence document.
- **Would collide with:** any unit creating or editing `STATUS.md` — inferred from the single document write surface.
- **Safety fact:** the acceptance explicitly requires failed and unrun checks to remain visible; the document must preserve observed results rather than imply successful execution — cited, proof level 1 (stated in acceptance), unproven.
