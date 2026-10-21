---
format: aep.planning-md/3
id: spending-blocker:aws-worker-spend
kind: spending-blocker
status: open
title: AWS spending approval for one EC2 worker
relations:
- blocks: story:worker-provisioning
revision: 1
---
## Observation

`mantle worker up` with `provider = "aws"` creates an EC2 instance, an instance profile, a security
group and a retained data volume in the operator's account. story:kubevirt-provider records "AWS
spend is paused pending cost approval". The 2026-10-21 wave left story:worker-provisioning out for
that reason; no agent session may start paid cloud resources.

## What clears this

The operator approves AWS spending for one worker. The clearing evidence is that approval, followed
by a `mantle worker up` that ends READY.
