---
order: 8
title: "Secrets by declaration"
summary: "A repository declares the secrets it needs, and an agent gets them only through the shell, resolved from a password manager for the task at hand, never pasted, stored or printed."
line: "An agent gets a secret only through the shell, for the task at hand."
tools:
  - name: sluice
    href: https://agentplot.github.io/sluice/reference/configuration/#executor-what-the-tests-may-touch
status: draft
introducedIn: "0.1"
lastReviewed: 2026-10-06
diagram:
  src: /diagrams/secrets-by-declaration.svg
  alt: "The repository's declaration, secretspec.toml, lists the secrets it needs by name and holds no value. An agent's command, deploy preview, runs inside the shell the repository declares. For that command alone, a key appears, resolved from the password manager, such as 1Password, and it vanishes when the command ends. Nothing reaches a file, the wider environment or the transcript. When the agent pastes a token into a file, it is refused. Here secretspec names each secret and resolves it from the password manager, while sluice gives the run nothing outside its place."
---

## How we do it today

secretspec declares what each repository needs.

## Where it's going

Secrets resolved from a password manager such as 1Password at run time, scoped to the task, with each access attributable and revocable. Operations work, where agents connect to running services, depends on it.
