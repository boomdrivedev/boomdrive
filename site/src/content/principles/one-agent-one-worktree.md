---
order: 2
title: "One agent, one worktree"
summary: "Each agent works in its own worktree with its own ports, emulator and dev servers, so nothing it does can disturb another."
status: draft
introducedIn: "0.1"
lastReviewed: 2026-10-06
---

## How we do it today

Worktrunk, hashed ports, an emulator per worktree, Portless names, servers tethered to the worktree.

## Where it's going

Worktree environments declared once and stood up from the declaration.
