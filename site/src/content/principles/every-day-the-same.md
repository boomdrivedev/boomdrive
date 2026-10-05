---
order: 1
title: "Every day the same"
summary: "A check, a gate and an answer about the repository are identical whether the work happens in an agent's worktree, on a laptop, in CI or in a release."
status: draft
introducedIn: "0.1"
lastReviewed: 2026-10-05
---

## How we do it today

devenv declares the tools, moon runs every check, and the same hooks run locally and in CI.

## Where it's going

The agent's check and the server's check share one key and one record.
