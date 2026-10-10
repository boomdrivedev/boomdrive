---
order: 1
title: "One shell"
summary: "Every command, an agent's, a person's or CI's, runs inside one shell the repository declares, so the tools and their versions are the same everywhere."
line: "Every command runs in the one shell the repository declares."
status: draft
introducedIn: "0.1"
lastReviewed: 2026-10-06
diagram:
  src: /diagrams/one-shell.svg
  alt: "Four places where work runs: an agent in its worktree, a laptop, CI and a release. Each sends the same command, site-check, into the one shell the repository declares. The devenv shell resolves only the tools it declares, and every place gets back the same verdict from bun 1.4.2, node 24 and vale 3.24.0 alike. Partway through, the agent tries npx vale@latest around the shell, and it is refused because it is not declared."
---

## How we do it today

devenv declares every tool; hooks, CI and agents enter the same shell.

## Where it's going

The shell is part of a repository's manifest, so any platform that runs the repository knows how to enter it: devenv here, a container image or mise elsewhere.
