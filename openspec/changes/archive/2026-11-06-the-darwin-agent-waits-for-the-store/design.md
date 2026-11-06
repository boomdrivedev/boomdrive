# Design

## Context

See proposal.md for why. This change puts nix-darwin's wait for the Nix store in front of the module's agent, and keeps the rest of the agent as it is.

What stands:

- **This repository**, at `2937b09`. `nix/modules/darwin.nix` sets `launchd.user.agents.boomdrive.serviceConfig.ProgramArguments` to `<package>/bin/boomdrive run --config <file>`, with `RunAtLoad`, `KeepAlive`, `ThrottleInterval` 30, `ProcessType` `Interactive`, `AbandonProcessGroup`, `HOME` and `PATH`, and the log (the archived change `boomdrive-publishes-a-flake-of-the-family-and-the-runners-service`, its Decision 10). `nix/tests/darwin.nix` reads the file as `ProgramArguments`' fourth item.
- **nix-darwin**, at the lock's revision, `modules/launchd/default.nix`. A job's `command` is a string. Where it is set, nix-darwin sets `serviceConfig.ProgramArguments` to `/bin/sh`, `-c` and `/bin/wait4path /nix/store && exec ${command}`, splicing the string in as it is. `script` is written to a store file, which becomes the `command`. `modules/system/launchd.nix` copies a user agent's plist into the user's `~/Library/LaunchAgents` and reloads the agent where the new plist differs from the one there, and only then.
- **NixOS**, at the lock's nixpkgs, `nixos/lib/utils.nix`. `/nix/store` is among `pathsNeededForBoot`, which the first stage mounts before systemd, itself a store path, starts.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches. It relies on core.46 (the modules run the runner as the user's service reading its file), core.13 (the service manager keeps the runner's process) and core.47 (a module's packages are the flake's own).

## Goals / Non-Goals

**Goals:**

- An agent loaded at boot, before the store's volume is mounted, waits for it, then runs the runner.
- A stop's signal reaches the runner, and a `flume launch` in flight runs to its end.
- nix-darwin reloads the agent when the package or the file changes, and leaves it running when neither does.

**Non-Goals:**

- **A wait for anything but the store.** The runner's file, its package and the tools on `PATH` are all under `/nix/store`, or under profiles that point into it.
- **The NixOS unit.** It cannot start before the store is mounted (Decision 4).

## Decisions

### 1. The agent is declared with nix-darwin's `command`

```nix
launchd.user.agents.boomdrive = {
  command = lib.escapeShellArgs [ "${cfg.package}/bin/boomdrive" "run" "--config" "${file}" ];
  serviceConfig = { RunAtLoad = true; KeepAlive = true; … };
};
```

launchd then runs `/bin/sh -c '/bin/wait4path /nix/store && exec <package>/bin/boomdrive run --config <file>'`. `command` is spliced into the shell's line as it is, so the module quotes each word with `lib.escapeShellArgs`. A store path and the words `run` and `--config` need no quoting, so the line reads as the command does.

This is the form nix-darwin gives every job declared by `command` or `script`, its own services among them, so the module's agent waits as they do. The tests read the program launchd gets, not the module's string, so a lock that changes nix-darwin's wrapper fails the gate where it no longer waits for `/nix/store` and then `exec`s.

Alternatives:

- **The same shell written into `serviceConfig.ProgramArguments` by the module.** It gives the same plist byte for byte, and the same signals. Not taken: it copies nix-darwin's wrapper into the module, where nix-darwin already owns how a job waits for the store, and the copy would not follow a change nix-darwin makes to it.
- **`script`.** Not taken: it adds a store file of its own and a second shell between launchd and the runner, which would have to `exec` the runner in turn, and the job's program names the script, not the package and the file.

### 2. A stop's signal reaches the runner alone

`exec` replaces the shell with the runner in the same process, so the runner is the job's process, the one launchd signals. A stop, by `launchctl bootout` or `unload`, at logout, or at nix-darwin's reload, sends SIGTERM to that process. With `AbandonProcessGroup`, launchd sends nothing to the rest of its process group, so a `flume launch` the runner started runs to its end. This was checked on a Mac: a program started by `/bin/sh -c '/bin/wait4path /nix/store && exec …'` held the shell's process id, took the TERM sent to it, and a child it started that ignored TERM ran on.

A stop while the shell still waits for the store ends the shell. `wait4path`, its child, is abandoned with the group and exits once the store is mounted. Nothing of the runner's has started.

Without the `exec`, the shell would stay the job's process and the runner its child, so a stop would end the shell and leave the runner running, abandoned with the group. The test that the agent waits also holds that its program `exec`s its command.

`ThrottleInterval` is unchanged: once the store is mounted, `wait4path` returns at once, so a runner that exits is started again no sooner than before.

### 3. A changed package or file reloads the agent, and nothing else does

nix-darwin reloads a user agent where its plist differs from the one installed. The plist's program holds the package's path and the file's path in the shell's line, so a new package or file changes the plist, and the agent is reloaded. Nothing else in the plist holds a store path: `HOME`, `PATH` and the log are plain paths. So a change to the system that leaves the package, the file, the home and the log as they were, such as `tools`, leaves the plist byte for byte the same, and the agent running.

The tests prove each: a new `fetch_interval` changes the program in the file alone, a new `package` changes it in the package alone, each changes the plist, and `tools = [ ]` leaves the plist as it was.

### 4. The NixOS unit needs nothing

NixOS mounts `/nix/store` in its first stage, as one of `pathsNeededForBoot`, before it starts systemd, which runs from the store. A lingering user's manager, and the unit it starts, come after that. The unit is unchanged.

### 5. The tests read what the agent runs

`nix/tests/darwin.nix` reads the agent's program as launchd gets it: the shell, its wait and `exec`, then the command's words. The command is the set's `boomdrive` followed by `run`, `--config` and the file, the word after `--config`. A program not shaped so gives no file, and every case that reads one fails, rather than evaluation stopping.

## Risks / Trade-offs

- **nix-darwin's wrapper is nix-darwin's.** Where a new lock changes it, the agent's program changes with it, and the plist does too, so every host reloads its runner once at that switch. The gate's tests fail where the new wrapper no longer waits for `/nix/store` and then `exec`s.
- **One reload at the switch that lands this change.** The plist's program changes from the runner's path to the shell's line, so nix-darwin reloads the agent once. The runner stops at a TERM and starts again, as at any new package.
