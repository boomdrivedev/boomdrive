# Verify: timber-comes-with-boomdrives-flake

The work holds. The gate passes, every task is ticked, and each scenario of the four deltas has a case that proves it.

The code stage archived the change under task 5.1, which needs the archive before `root:specs` can prove it. This report sits beside the archive, on `change/timber-comes-with-boomdrives-flake` at `3811b4e`.

## The whole checks

`sluice exec main` ran once on `3811b4e`. It ran the checks the commit affects against `main` (`4b2da33`) and exited 0. All 14 passed, and sluice recorded their results under their keys for `3811b4e`:

- `nix`: `format`, `lint`, `typecheck` and `test`;
- `root`: `format`, `adrs`, `changes`, `specs` and `releases`;
- `site`: `check`, `build`, `prose`, `vocab` and `test`;
- `boomdrive`: `test`.

`root:specs` passed 26 items, 0 failed. moon also warned that it could not write its output archives under `.moon/cache/outputs`. Those warnings come from the sandbox, and no check failed because of them.

## Tasks

All ten tasks are ticked, 1.1 to 5.1, and none is open.

- 1.1 and 1.2 rely on `nix:typecheck` and `nix:test`.
- 2.1 to 4.1 rely on `nix:test`.
- 4.2 relies on `root:format`.
- 5.1 relies on `root:specs`.

Each of those checks passed in the run above. The diff stays inside proposal.md's Touches. It adds no crate, gate tool or tracked file type, and the runner's code is unchanged. Records 0023 and 0044 and the details of core.13, core.44 and core.46 were written in the proposing commit, `b13f925b`, as the tasks say.

## The deltas, scenario by scenario

### family-packages

| Scenario | Proven by |
|---|---|
| Every system's packages | `nix/tests/typecheck.nix`'s set: the twenty packages and `default`, with `timber` and `timber-herdr-plugin` |
| No tool of the host's own | `typecheck.nix`'s `hosts`, unchanged |
| timber named by no release tag | `nix/tests/packages.nix`'s case of that name; `tags.nix` lists timber among the tools held to a release tag |
| A tag moved without the lock, and the other tools' "named by no release tag" | unchanged cases of `packages.nix` |
| timber's plugin moves with timber | `packages.nix`'s `pluginOf "timber"` |
| timber through its own flake | `packages.nix`'s case of that name: the flake's `default` by `drvPath`, its tag's version, program `timber` |
| herdr is Flume's | `packages.nix`'s `floorsOf`, which now reads timber's plugin's `min_herdr_version` |
| The marketplace's three plugins | `nix/tests/companions.nix`'s case of that name, with `timber` at `./plugins/timber` and the set's timber's version |
| No link into the store | the marketplace's build fails on any link, its `cp -RL` of timber's plugin among what it copies |
| timber's plugins say the set's timber | `nix/tests/timber.nix`, reading both manifests against `packages.timber.version` |
| A timber plugin other than the set's timber | `timber.nix`'s case, `0.5.3` against `0.6.0`, with the plugin named |
| timber's Claude Code plugin from its command's source | `timber.nix`'s case: `passthru.sources.timber` is `${timber.src}/plugins/claude-code`, copied whole by `cp -RL` |

The marketplace's build also fails where timber's `plugin.json` says another version than the set's timber.

### service-modules

| Scenario | Proven by |
|---|---|
| The README's words | `nix/tests/cache.nix`, which now names `services.boomdrive.timber.package` |
| The README's timber | `cache.nix`'s case of that name: `github:boomdrivedev/boomdrive#timber`, `timber-herdr-plugin`, `timber serve` |
| The plugins linked on nix-darwin and on NixOS | `nix/tests/apps.nix`'s `stepOf` and `nix/tests/nixos.nix`'s `keeps`, with `timber-herdr-plugin` after sluice's |
| The companions' plugins with crew | `apps.nix`, in the order Flume, sluice, timber, Pending You, Annotate |
| No companions without crew | `apps.nix`, with Flume's, sluice's and timber's plugins alone |
| timber with the runner, A host's own timber, No timber of the set's, No timber without the runner | `apps.nix`'s four cases, each on nix-darwin and on NixOS |
| No serving process of the modules' | `apps.nix`'s `noServe`: no launchd job, systemd unit or activation step names the set's timber's path |

`nix/modules/apps.nix` declares `timber.package` with `installed`, adds it to the system packages, and adds `timber-herdr-plugin` to the plugins' default with its `defaultText`.

### binary-cache

| Scenario | Proven by |
|---|---|
| The runner is pushed with its tools | `nix/tests/workflow.nix`, which lists `timber` and `timber-herdr-plugin` |
| A host's own nixpkgs | `cache.nix`'s `ownNixpkgs`, with `timber` among the host's overlaid names and `timber-herdr-plugin` in the linked order |

`.github/workflows/cache.yml` is unchanged, as the proposal says.

### way-in

| Scenario | Proven by |
|---|---|
| Every system's shell | `nix/tests/way-in.nix`'s eleven tools, with `timber`; `nix/shell.nix` holds `set.timber` |

## The words

- The README's **Installing with nix** names each thing task 4.1 lists:
  - timber in the set, its herdr plugin and its Claude Code plugin;
  - `#timber` on the profile line;
  - `timber-herdr-plugin` among the plugins a host links by hand;
  - the option and its `null`;
  - `timber serve` as timber's own;
  - the marketplace, installed into no Claude Code home yet;
  - timber in the shell.
- CLAUDE.md and `openspec/config.yaml` name timber in the set, its plugin in the marketplace, and timber among what the modules install with the runner.

## Archive

The change is at `openspec/changes/archive/2026-11-13-timber-comes-with-boomdrives-flake`, and no change of that name is open. `openspec/specs/` holds the four deltas, and `root:specs` passes them under the strict check.
