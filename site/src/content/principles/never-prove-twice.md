---
order: 5
title: "Never prove twice"
summary: "The task runner's cache, keyed by what each task reads and shared across every worktree, agent and CI run, means a check that passed on these inputs is not run again."
status: draft
introducedIn: "0.1"
lastReviewed: 2026-10-06
---

## How we do it today

moon's cache, with archives in the git common directory so worktrees of one repository share them. Inputs are declared per project, and a check catches a test that reads outside them.

## Where it's going

A remote cache shared by every machine. Agents read it; only CI, running in the declared shell, writes to it.
