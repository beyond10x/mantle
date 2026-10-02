---
format: aep.planning-md/3
id: story:kubevirt-provider
kind: story
status: active
title: A KubeVirt worker provider equivalent to the EC2 worker
relations:
- decomposes: epic:vertical-slice
scope:
- confidence: cited
  path: Taskfile.yml
- confidence: cited
  path: crates/mantle/src/adapters/kubevirt.rs
- confidence: cited
  path: crates/mantle/src/adapters/ssh.rs
- confidence: cited
  path: crates/mantle/src/app/worker.rs
- confidence: cited
  path: crates/mantle/src/config.rs
- confidence: cited
  path: deploy/cloud-init.yaml
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-01T23:29:33Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-01T23:29:33Z", actor: "human:timo", revision: 3}
---
# Story: A KubeVirt worker provider equivalent to the EC2 worker

## Why

AWS spend is paused pending cost approval. The slice still has to be exercised end to end, so a
second provider places the worker as a KubeVirt virtual machine in a Kubernetes cluster — locally a
k3d cluster on the developer's machine (`task k3d-up`).

## Equivalence

The KubeVirt worker must be semantically equivalent to the EC2 worker:

| Layer | How both providers get the same thing |
|---|---|
| OS image | one Ubuntu 24.04 build serial (`ubuntu_serial` in the operator config): the AMI named with that serial, and the cloud image `release-<serial>` |
| Kernel | the shared cloud-init converges both to the newest `linux-generic` and reboots; any `-aws` kernel is removed from boot |
| Packages, units, binaries | one rendered cloud-init; identical `mantle-egress`/`mantle-launch` bytes; Substrate built from one commit with one toolchain |
| Transport | one SSH adapter; only the ProxyCommand differs (`aws ssm start-session` vs `virtctl port-forward --stdio`) |

Inherent differences, reported rather than hidden: hypervisor (Nitro vs KVM/QEMU), disk device
names, cloud-init datasource (Ec2 vs NoCloud), the SSM agent present only on EC2, instance size.

## Acceptance

With `provider = "kubevirt"`, `mantle worker up` creates one VirtualMachine with a root DataVolume
imported from the pinned cloud image, a retained data DataVolume and the rendered cloud-init, and
ends READY on the same Substrate facts as EC2. `mantle worker status` prints the OS image serial
and the running kernel, and both providers print the same values for the same serial.
`mantle worker down` halts the VM and keeps both volumes. The session story's acceptance then runs
unchanged against this worker.

## Scope

- `crates/mantle/src/adapters/kubevirt.rs`, `crates/mantle/src/app/worker.rs`, `crates/mantle/src/config.rs`, `crates/mantle/src/adapters/ssh.rs`
- `deploy/cloud-init.yaml` (kernel convergence)
- `Taskfile.yml` (`k3d-up`, `virtctl`, `k3d-down`)
