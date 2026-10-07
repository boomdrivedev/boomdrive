---
order: 6
title: "Secrets by declaration"
summary: "A repository declares the secrets it needs, and an agent gets them only through the shell, resolved from a password manager for the task at hand, never pasted, stored or printed."
status: draft
introducedIn: "0.1"
lastReviewed: 2026-10-06
---

## How we do it today

secretspec declares what each repository needs.

## Where it's going

Secrets resolved from a password manager such as 1Password at run time, scoped to the task, with each access attributable and revocable. Operations work, where agents connect to running services, depends on it.
