---
number: 36
title: The set gives crew each kept repository's folders, and a host writes only its own
status: accepted
date: 2026-11-09
decision-makers:
- Chuck Swanberg
- agentplot-design
---

# The set gives crew each kept repository's folders, and a host writes only its own

Status: accepted on 2026-11-09. The direction is the user's, in agentplot-design's pane that day. Each point's form is agentplot-design's own choice, cheap to change.

Deciders: the user, on 2026-11-09, reading deneb's draft Boomdrive configuration in agentplot-design's pane: "why is this in swancloud configs? the idea is that the are part of the recipe from crew/boomdrive"; "Jeezum, we need to externalize the things that can be and make sense to be externalized"; and "It seems like there are still lots of things you need to fix." agentplot-design, for the rest.

Sources: agentplot/blueprints `design/principles.md` 14, written from those words; records 0023, 0028, 0032 and 0034; rule core.46; the flake bolt `the-boomdrive-flake-configures-the-whole-host` at 295e3bb, its `nix/modules/darwin.nix`, `drives.nix`, `options.nix` and `tools.nix`; the README's account of a pass (the clone at `repositories/<owner>/<name>/<name>.git` in the data directory, the view at `<repository_root>/<owner>/<name>`); crew v0.54.0's README, where the teams file may name each kept repository of its host with its `dir` and its `checkout`; deneb's draft configuration of 2026-11-09, which evaluates against the bolt at 14caa11.

## Context and problem statement

deneb's draft configuration still writes values the set knows. The largest is crew's `repositories`, which maps each address the runner keeps to the runner's clone folder and the runner's view. The draft builds that map in the host's own Nix from the accounts and the runner's layout. The runner knows both. The draft also sets the runner's log file and keeps its own builds of crew and herdr. And it marks facts the bolt has no option for, which the host would keep writing beside the set.

Principle 14 has the set derive or default every value the recipe knows. A host writes only what is its own, and only through a tool's own settings.

## Decision outcome

1. **The set gives crew each kept repository's folders.** Where crew's module is on, the Boomdrive module gives crew's `repositories` one entry for each address the host's runner keeps. That is every repository of every drive of every account, since the runner keeps another machine's drives' repositories too (record 0028). Each entry's `dir` is the runner's clone folder, `<data directory>/repositories/<owner>/<name>`, and its `checkout` is the runner's view, `<repository_root>/<owner>/<name>`. Both follow the runner's layout, which its README states, and use the same `repository_root` the runner's file is given. A check of the set fails wherever the module's folder for an address differs from the runner's own. A host writes none of the entries. Not taken: crew reading each folder from the runner's `repositories.rec` on its own host. That would be a pass late on a new host, and it is crew's change to weigh.
2. **The runner's job is the set's.** The job's log file defaults to `~/Library/Logs/boomdrive.log`, its PATH is the profiles' (the bolt's design Decision 11), and its label is nix-darwin's `org.nixos.boomdrive` (record 0034, point 6). A host sets none of them, so swancloud's own log name and PATH go.
3. **Each tool's build is the set's.** The set pins each tool at its latest release that runs with the others. A host keeps no build of its own for a tool the set pins, crew and herdr among them on deneb. Record 0034's point 7 still lets a host keep one, and none should need to.
4. **What stays the host's is set through a tool's own settings.** These are facts only the host knows. They are its accounts and drives: each drive's name, machine, stream room and repositories. They are the runner's own settings: `repository_root`, its intervals and its control plane. They are sluice's GitHub App, signer, runs and caches; Flume's pools and the accounts in them; and nix's own trust of the family's cache (core.47). Each is set through the option that names it, never by a file, wrapper or script beside the set.

The rest of the draft's gaps already have their records: each account's crew state (record 0032), Flume's sources and crew's source (record 0034, point 2), and herdr's own settings and plugins (record 0034, point 1). Other changes take sluice's telemetry and restarts, Flume's usage readings, its kinds and its labels' reserves, and crew's per-role choices.

## Consequences

- Good: deneb's configuration names its accounts, drives and settings, and builds no map of the runner's folders.
- Neutral: the map follows the runner's data directory, so it moves with record 0035, from `flywheel` to `boomdrive`.
- This record adds core.57. The change `the-set-gives-crew-each-kept-repositorys-folders` builds point 1. Points 2 and 3 need no code: the bolt already defaults them, and a host drops its own values.
