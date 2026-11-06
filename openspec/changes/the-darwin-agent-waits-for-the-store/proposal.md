# Proposal

## Why

The nix-darwin module's agent sets `launchd.user.agents.boomdrive.serviceConfig.ProgramArguments` to the runner's store path, with `RunAtLoad` and `KeepAlive`. nix-darwin puts `/bin/wait4path /nix/store` in front of a job's program only where the job is declared with `command` or `script`, and a hand-set `ProgramArguments` gets none.

At boot, launchd can start the agent before macOS has mounted the Nix store's volume. The exec then fails, and launchd may never try it again, so a host's runner can stay down after a reboot until someone loads it by hand. A host's configuration cannot wrap the job from outside, since the module owns it.

## What Changes

- **The agent waits for the store.** The module declares the agent with nix-darwin's `command`, so launchd runs `/bin/sh -c '/bin/wait4path /nix/store && exec <package>/bin/boomdrive run --config <file>'`. The shell's `exec` hands the job's process to the runner, so a stop's signal still reaches the runner alone, and `AbandonProcessGroup` still leaves a `flume launch` in flight.
- **Everything else of the agent stays.** It keeps `RunAtLoad`, `KeepAlive`, `ThrottleInterval` 30, `ProcessType` `Interactive`, `AbandonProcessGroup`, `HOME` and `PATH` alone, and the log. Its plist names the package and the file, as before, so nix-darwin reloads it when either changes and leaves it running when neither does.
- **The NixOS unit is unchanged.** NixOS mounts the store in its first stage, before systemd starts, so no unit can start before the store is there.
- **The tests read what the agent runs.** They read the command after the wait, and the file after `--config`, in place of positions in `ProgramArguments`. A new case fails where the agent's program does not wait for `/nix/store` and then `exec` its command. New cases show that a new package changes the agent's program and plist, and a change to `tools` leaves the plist as it was.
- **Rule core.46 and record 0020** are amended in this change's commit: the modules start the runner only once the host's Nix store is mounted.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: a new requirement, that on nix-darwin the agent runs behind `/bin/wait4path /nix/store`, handing its process to the runner; and a new package, like a new file, changes the agent, while a change to `tools` does not.

## Impact

- `nix/modules/darwin.nix`: the agent is declared with `command`.
- `nix/tests/darwin.nix`: the agent read by meaning; the wait, a new package and an unchanged plist proven.
- `docs/architecture/core.md` amends core.46. `docs/adr/0020-…` gains an amendment.
- `README.md`: the agent waits for the store.
- Nothing else changes: no crate, no tool, no tracked file type, no option of the modules, nothing of the NixOS module.

## Touches

```
nix/modules/darwin.nix
nix/tests/darwin.nix
docs/architecture/core.md
docs/adr/0020-boomdrive-publishes-a-flake-of-the-familys-host-tools-their-binaries-in-the-familys-cache-and-modules-for-the-runners-service.md
README.md
```
