---
format: aep.planning-md/3
id: story:helm-chart-worker
kind: story
status: draft
title: Evaluate Substrate's Helm chart as a worker provider
relations:
- decomposes: epic:substrate-0-16
- depends_on: story:substrate-0-16-0-pin
revision: 1
---
# Story: Evaluate Substrate's Helm chart as a worker provider

## Why

Substrate 0.15.0 ships a Helm chart for a node-bound daemon (Substrate ADR 0046,
`oci://ghcr.io/beyond10x/charts/b10x-substrate`): one daemon per node, TLS and Identity admission,
a default-deny NetworkPolicy. Mantle's KubeVirt provider runs a whole VM per worker to host that
daemon. If the chart's daemon proves the confinement facts a session needs on a real node, Mantle
could place sessions on a cluster without a VM.

## Questions to answer, with evidence

1. Which exec facts does the chart's daemon publish on a kind node and on a real node? Read from
   `GET /v1/facts`, not from the chart's values.
2. Mantle reaches the daemon over SSH-forwarded Unix sockets today. What does TLS plus Identity
   admission need from Mantle (token source, trust roots, DNS identity)?
3. Where do `mantle-egress` and the toolchain at `/opt/mantle` live when there is no VM?

## Acceptance

A design note in `docs/design/` answers the three questions with recorded observations and ends
in a recommendation: adopt, adopt later on a named condition, or keep the VM provider.

## Blocked on

`story:substrate-0-16-0-pin`.
