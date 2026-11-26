# Design

## Context

Flume gives each end of a place to the recipients that its launch names. Where a launch names none, Flume uses its default. The default is `FLUME_RECIPIENT`, then the session of the command, then the answerer behind the place's stream (Flume's 7 and 193). Where none of these is there, Flume refuses the launch (Flume's 4). The runner runs Flume with `FLUME_RECIPIENT` unset, and its service runs in no session.

Record 0030 makes the runner one more recipient of a launch that names recipients. It passes a launch that names none with none, so that Flume's default stays. Under crew, the default is the answerer behind the stream, a role of crew's. A role's run names no recipient for this reason.

A hosted control plane's offer names no recipient. No session or role on the host is the cloud's to name. The cloud's members read a place's ends and questions only from the runner's report.

## Decision 1: the control plane says whether the runner alone reads it

`ControlPlane` gets `read_through_the_runner(&self) -> bool`. It says whether the readers of the control plane's places reach the host only through the runner. `Hosted` gives true. `Crew` gives false. The method has no default, so each implementation says it.

The runner reads this fact and decides the recipients itself. No code outside an implementation names an implementation (core.2). The implementation does not hold the runner's own name.

## Decision 2: the runner alone, where no one else reads

`flume::recipients(given, read_through_the_runner)` gives:

- `given`, then `boomdrive`, where `given` names a recipient and not `boomdrive`.
- `given` alone, where it names `boomdrive`.
- `boomdrive` alone, where `given` is empty and the runner alone reads the control plane.
- nothing, where `given` is empty and other readers are on the host.

`LaunchArgs` holds `read_through_the_runner`, which `carry.rs` sets from the part's control plane for each offer.

With `boomdrive` as the recipient, Flume takes the launch. It owes each end of the place to the runner, and keeps the place's record after its tidy until the runner confirms each end. The runner confirms an end only after the control plane takes a report that holds it (record 0030 as amended on 2026-11-09). A place's question is in its status, which the runner carries in its report as it carries every status.

## Options not taken

- **The runner alone on every launch that names none.** A role's run of crew's would lose Flume's default, and the answerer would not get its ends.
- **The cloud names `boomdrive` in its offer.** The name is the runner's own. The cloud would name a recipient that only the runner reads.
- **The hosted implementation adds the name.** The implementation would hold the runner's own name (core.2).
- **A recipient in the host's settings.** A host could leave it out, and Flume would refuse again.

## Risks

- A host can declare an answerer for the stream. That answerer then gets a hosted place's stalls (Flume's 8 and 193) and no other end of it.
