# Proposal

## Why

On 2026-11-26, a person enrolled a host in a hosted control plane. The host ran boomdrive 0.30.1 and Flume 0.36.0. The control plane offered `flume/fix@1`, and Flume refused the launch. Flume gave these reasons:

- the launch names no `--recipient`.
- `$FLUME_RECIPIENT` is unset.
- the terminal names no session that the launch runs in.
- no source of the host declares a role behind the stream `boomdrive`.

The offer names no recipient. The runner names itself only beside the recipients that a launch names (record 0030). So the launch named no recipient. Flume then looked for its default: the session that the command runs in, then the answerer behind the place's stream. Under crew, the default is a role of crew's, which runs on the host. A hosted control plane has no session and no answerer on the host. Its members reach the host only through the runner's connection. So Flume refuses each launch of a hosted control plane, and Flume makes no place for it.

## What Changes

- **A control plane says whether the runner alone reads it.** The interface gets `ControlPlane::read_through_the_runner`. It is true where the readers of a control plane's places reach the host only through the runner. The hosted implementation gives true. Crew's implementation gives false.
- **The runner names itself alone where no one else reads.** A launch that names no recipient, of a control plane that the runner alone reads, names the runner alone: `--recipient boomdrive`. Flume then takes the launch, and keeps each end of the place for the runner. The runner carries each end and each question to the control plane in its report, as it does now.
- **Other launches do not change.** A launch that names recipients names the runner after them. A launch of crew's that names no recipient names none, so Flume's default stays.
- **The rule and the record.** Record 0030 gets an amendment of 2026-11-26. core.54 and the details of core.15 say the new case.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `work-into-flume`: a launch of a control plane that the runner alone reads names the runner alone.
- `control-plane-interface`: a control plane says whether the runner alone reads it.

## Impact

- `crates/boomdrive-control`: `ControlPlane::read_through_the_runner`, and the fake's `reading_through_the_runner`.
- `crates/boomdrive-hosted`: `Hosted` gives true. `crates/boomdrive-crew`: `Crew` gives false.
- `crates/boomdrive`: `flume::recipients` and `LaunchArgs::read_through_the_runner`, set from the part's control plane in `carry.rs`.
- No protocol version changes. No setting changes. Hosts take the release through their own configuration.
