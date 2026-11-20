---
number: 55
title: The flake runs Boomdrive Cloud's local copy on a host that turns it on
status: accepted
date: 2026-11-19
decision-makers:
- Chuck Swanberg
- boomdrive-design
---

# The flake runs Boomdrive Cloud's local copy on a host that turns it on

Status: accepted on 2026-11-19. The user chose that deneb's Boomdrive Cloud is a local copy on deneb for now, on their Pending You card `req_8f4a596fa5af441d9dfe`. How the copy runs is boomdrive-design's, and each choice is cheap to change.

A host can run its own copy of Boomdrive Cloud, kept by the Boomdrive flake like every other tool it brings. The copy listens on the host alone. An account of the host's runner takes its work from it the way any account takes work from the hosted cloud. When the hosted cloud takes that account later, only the account's address and enrollment change.

Deciders: the user, for a local copy on deneb for now; boomdrive-design for points 1 to 6.

Sources: record 0054, under which each account names its own control plane; record 0053, a cloud host's forge tokens; records 0006, 0018 and 0023; rules core.4, core.13, core.18, core.33 and core.46; boomdrivedev/boomdrive-cloud records 0069 and 0073 and its rules core.21, core.56 and core.65.

## Context and problem statement

Under record 0054 the runner serves each account through the control plane that account names, so one account of deneb's can move to Boomdrive Cloud while crew keeps the rest. The hosted cloud does not yet serve the user's work, so the user chose a copy of the cloud on deneb itself for now. Something has to run that copy, keep it running across restarts and switches, and keep its state. Everything of Boomdrive's tools a host runs is set up by the flake, and a host's own configuration only installs it (core.13).

## Considered options

1. **The flake's module runs the cloud's local stage as a service** (chosen).
2. **A branch's sluice preview.** sluice keeps a preview only while a checkout's place holds it, and releases it with the place (boomdrive-cloud record 0069, point 3). Real work can't rest on a branch.
3. **A dev server started by hand.** Nothing keeps it up after a restart or a switch, and it is set up outside the flake.
4. **The hosted staging cloud.** It is not built to carry the user's work. Staging data and a person's work must not mix (cloud core.65).

## Decision outcome

1. **A service the flake keeps.** On a host that turns it on, the Boomdrive module runs the cloud's local stage as a service of the runner's user, started at login once the Nix store is mounted and kept running, as the runner is. The host's own configuration sets only the module's option.
2. **Built from the flake's pin.** The flake builds the copy from its own pin of boomdrivedev/boomdrive-cloud, the same tree the cloud deploys, and runs its Worker as the local stage runs it. A new release of the set brings a new copy, as for every tool.
3. **On loopback alone.** The copy listens on a loopback port the module names. Nothing of it is reached from off the host, and nothing a host's runner reports to it leaves the host.
4. **Its own state.** The copy keeps its objects, databases and buckets in a folder of its own on the host, kept across restarts and switches. It shares nothing with another stage or another copy (cloud core.65).
5. **The local stage's sign-in and seeds.** The copy is a local stage. Its sign-in is the stand-in adapter (cloud record 0069, point 2): the person on the host is its seeded member, owning the seeded Account. The Worker refuses the stand-in in any other stage.
6. **The one App, with the copy's own key, kept as a secret.** The copy reaches GitHub through the one Boomdrive App with a private key and client secret of its own (cloud record 0073). It reads them from the host's credential store, where a command run on the host puts them, as enrollment is run. No option, file or store path holds them (core.4, core.46). A branch's preview reads its secrets from 1Password while a person works. A service started at login has no one to unlock 1Password, so the standing copy reads the host's store instead.

The runner's account for this copy names the loopback address as its hosted control plane and is enrolled as any account is (records 0006 and 0018). Its forge tokens come from that plane (core.33 as amended with this record).

## Consequences

- Good: the user sees a host's real work in the cloud now, through the connection every host uses (record 0054), with no mode built for one setup.
- Good: moving the account to the hosted cloud later changes its address and enrollment, and nothing else.
- Bad: the host keeps one more long-running service, and the copy's state lives on that host alone.
- Neutral: core.13 and core.46 are amended in the commit that writes this record. core.33 is amended beside it, per account.
