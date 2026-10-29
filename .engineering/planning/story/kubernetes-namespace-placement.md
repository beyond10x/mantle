---
format: aep.planning-md/3
id: story:kubernetes-namespace-placement
kind: story
status: draft
title: Place sessions through Substrate's Kubernetes namespace driver
relations:
- decomposes: epic:substrate-0-16
revision: 1
---
# Story: Place sessions through Substrate's Kubernetes namespace driver

## Why

Substrate 0.16.0 adds `--driver kubernetes`: a workspace is a PersistentVolumeClaim with a keeper
Pod, and each exec is one Pod in a claimed namespace (Substrate design 34, ADR 0047). In 0.16.0
every exec start on that driver is refused `exec.sandbox-unavailable`, because the probe proves
neither the four Linux confinement facts nor `confinement.no-egress`. Exec on a cluster is
unverified until Substrate's clean-room conformance release.

## What changes, once exec is served

- A placement class whose worker is a Substrate daemon running the namespace driver.
- The session's long-lived launcher becomes one exec Pod; attach goes through the same PTY session
  route.

## Acceptance

Written when the blocker clears, against the facts that Substrate release publishes.

## Blocked on

Substrate's exec on a cluster verified (`story:remote-clean-room-conformance` in
`beyond10x/substrate`) and released.
