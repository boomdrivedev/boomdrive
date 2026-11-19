---
number: 53
title: A runner serves each account's own control plane, and each sees all the work in its repositories
status: accepted
date: 2026-11-19
decision-makers:
- Chuck Swanberg
- boomdrive-design
---

# A runner serves each account's own control plane, and each sees all the work in its repositories

Status: accepted by the user on 2026-11-19, in the design agent's pane.

deneb's runner connects to Boomdrive Cloud the normal way, beside crew, and the cloud shows all the work in your repositories, whoever started it. Each account on a host says where its work comes from, crew or a cloud. The runner keeps one ordinary connection for each, with the same messages any host sends. Each one sees every piece of work in its own repositories. It can act only on the work it started itself. Nothing here is a mode for one setup, so nothing comes out later.

Deciders: the user, for point 1, which loosens their rule that a runner serves one control plane (core.19; record 0008); boomdrive-design for points 2 to 4, each cheap to change.

Sources: records 0006, 0008, 0013, 0014, 0018, 0026, 0028, 0048 (withdrawn) and 0052; rules core.10, core.19, core.26, core.27 and core.42; boomdrivedev/boomdrive-cloud record 0069 and its roadmap's wave 1; signal `2026-11-19-boomdrive-planner-976fec28/01` on afterthought/crew-state's `boomdrive/main`.

## Context and problem statement

The user wants to see all of a host's work in Boomdrive Cloud. They want it carried by the runner's ordinary connection to the cloud, over the same protocol, with nothing built for it alone that must be removed later.

Today a runner serves the one control plane its configuration names, and every account it serves is that plane's (core.19; record 0008). deneb's runner serves crew. A hosted plane is told only of the places it leased (core.10). Record 0048 added a watch-only connection to fill the gap, a mode no customer would run, and it is withdrawn. Record 0008 keeps crew's implementation until the operator's own accounts run on the cloud. The cloud cannot yet carry their work, so deneb cannot move to it whole.

## Considered options

1. **Each account names its own control plane, and each plane sees all the work in its repositories** (chosen). A runner serves crew for some accounts and a cloud for others at once, each over its own ordinary connection. Each is told of every place in its drives' repositories.
2. **Move deneb's runner whole to the cloud now.** The cloud starts no work and gives no leases yet (its roadmap's wave 1), so deneb's work would stop.
3. **A second runner on deneb for the cloud.** One plane per runner holds, but the host keeps two sets of clones and fetches each repository twice against GitHub's allowance. Both runners settle every place. A host runs one runner.
4. **Keep record 0048's watch-only connection.** That is a mode for a local preview alone, which comes out once deneb moves to the cloud.

## Decision outcome

1. **Each account names its control plane** (the user's). The runner serves each account it holds through the control plane that account names, crew's or a hosted one, several at once, each over its own connection as record 0008 describes. Each is a control plane in full. It may offer work in its account's repositories, and the runner takes it, within its stream's room, as from any plane. A repository may sit in drives of two accounts, so one clone serves both (core.14). Once crew's implementation leaves (record 0008), none of this comes out. A host may still serve two clouds, such as production and a copy run on a computer.
2. **Each plane sees all the work in its repositories** (boomdrive-design's). The runner reports to a control plane every place Flume runs in the repositories that plane's drives name, whoever started it. A place the plane leased goes with its lease's number. Any other goes with no lease, marked as started elsewhere. A plane acts only on what it leased (core.10). Nothing of another account's repositories crosses a connection (core.26), and what leaves the host stays what records 0013 and 0014 allow.
3. **The protocol says it** (boomdrive-design's). From the protocol's next version, a report on any connection may name no lease for a place the plane did not start. The observing hello of version 3 goes, which no release may edit (core.42).
4. **The cloud shows it as the drive's work** (boomdrive-design's). Boomdrive Cloud shows a place started elsewhere among the drive's work, saying where it came from. It shows no button for an act it cannot carry, since it holds no lease. Nothing marks the host itself as different.

## Consequences

- Good: in the cloud the user sees all the work in their repositories on every host they enrolled, through the connection every host uses. The move off crew becomes one account at a time.
- Good: the same holds for any person who starts work by hand on an enrolled host. The cloud shows it as theirs.
- Bad: a runner holds one connection per plane, and a repository in two accounts is reported to both.
- Neutral: core.19 is amended with this record, and record 0008's "one control plane per runner" with it. The runner's configuration gives each account its plane, which the construct of the change that builds it designs.
- The queued units `the-runner-reports-only-to-its-control-plane` (boomdrive) and `the-cloud-shows-only-hosts-that-take-its-work` (boomdrive-cloud) are replaced by this record's units once it is accepted.
