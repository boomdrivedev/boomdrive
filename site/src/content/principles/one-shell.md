---
order: 1
title: "One shell"
summary: "Every command, an agent's, a person's or CI's, runs inside one shell the repository declares, so the tools and their versions are the same everywhere."
status: draft
introducedIn: "0.1"
lastReviewed: 2026-10-06
---

## How we do it today

devenv declares every tool; hooks, CI and agents enter the same shell.

## Where it's going

The shell is part of a repository's manifest, so any platform that runs the repository knows how to enter it: devenv here, a container image or mise elsewhere.
