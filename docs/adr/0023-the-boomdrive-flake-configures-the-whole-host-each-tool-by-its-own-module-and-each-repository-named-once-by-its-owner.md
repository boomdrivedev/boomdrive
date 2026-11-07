---
number: 23
title: The Boomdrive flake configures the whole host, each tool by its own module, and each repository named once by its owner
status: accepted
date: 2026-11-06
decision-makers:
- Chuck Swanberg
- agentplot-design
---

# The Boomdrive flake configures the whole host, each tool by its own module, and each repository named once by its owner

Status: accepted by the user on 2026-11-06, in agentplot-design's pane: "Other than that nitpick I approve them all" (the nitpick being flume record 0055's).

Amends record 0020 in part, and record 0012 where it keeps herdr the host's own, each marked so where it stands.

Deciders: the user, in the research talk `research--the-flakes-configuration-holds-streams-and-repositories` on 2026-11-06 (`design/research/the-flakes-configuration-holds-streams-and-repositories.md`, merged with this record), who asked for it, "it just has one flat list of repos, but we should be able to configure the streams and the repos as well", and settled it: "I thought we were going to create one flake for people to use from Boom Drive so they didn't have to set up two flakes. [...] I agree that you should set up a flake for Flume as well so Flume could be used independently but I want to make sure that it's a one-stop shop with the Boom Drive flake"; "I want to make sure the flake also packages and configures crew and sluice"; a repository's stream, "One block by owner"; Flume's module, "Flume's flake, re-exported"; its accounts, "Yes, all of Flume's file"; herdr, "In the set, overridable"; the way in, "An app to click" and "A shell"; the terminal app, "Install Ghostty too"; and crew's partitions, "Yes, derive it". agentplot-design for writing it into this repository's records and rules.

Sources: `design/the-flakes-configuration/design.md` and `deneb.nix` (the talk's design and its sketch of deneb), merged with this record; records 0009, 0012, 0013 and 0020; rules core.5, core.13, core.17, core.18, core.29, core.44 and core.46; flume record 0055 (a repository named by its address; Flume's own flake and module; Flume reading usage itself), proposed with this one; Flume's statement 143, 186 and C.7; deneb's `~/.config/flume/config.toml` (four streams, each repository's folder mapped to its stream) as afterthought/swancloud writes it.

## Context and problem statement

The flake's modules write the runner's file alone (record 0020): one flat list of repositories, and nothing of Flume's streams, sluice's host settings or crew's partitions. A host with four streams, as deneb has, writes each repository in the runner's list and again, by its folder, in Flume's file, each by a different writer. herdr and the terminal app are the host's own, so a fresh Mac has no way to see its agents after the install.

## Considered options

1. **The flake configures the whole host.** One flake brings the runner, Flume, sluice and crew, each configured by its own module, which the Boomdrive flake re-exports from the tool's own flake; each repository is written once, under its owner, with the owner's stream; herdr and Ghostty come in the set, with a Boomdrive app and a shell to open the agents.
2. The runner's flat list, beside a block of Flume's own naming each repository's stream. Each repository written twice.
3. The Boomdrive flake owning every tool's module. A tool used without Boomdrive has no module.

## Decision outcome

Option 1.

- **One flake.** A host adds the Boomdrive flake alone. Its modules bring the runner, Flume, sluice and crew, each packaged and configured. Flume, sluice and crew each publish a flake with their own module, so each can be used without Boomdrive; the Boomdrive flake takes each as a locked input and re-exports its module, and carries a tool's module itself only until that tool's flake gives one, as it carries Flume's package today.
- **Each tool's module writes its own file.** `services.flume.settings` is Flume's file, `services.sluice.settings` sluice's host settings, `services.crew.settings` crew's and `services.boomdrive.settings` the runner's, each key named as its file names it. No module writes another tool's file; the Boomdrive module sets the other tools' module options.
- **Each repository named once, by its owner.** A host writes each owner once, with its repositories' names and, where its work runs on this host, its stream: `services.boomdrive.repositories.<owner> = { stream = "<stream>"; names = [ ... ]; }`. The module writes each address into the runner's `repositories` and gives each address its owner's stream in Flume's module. An owner with no stream gives Flume no rule. The person writes no folder. (Amended in part by record 0028, accepted by the user on 2026-11-07: each repository is named under its account's drive and its owner, and the drive gives the stream.)
- **herdr and Ghostty are in the set.** The flake installs herdr at the version Flume's terminal profile was checked against, with Flume's and sluice's herdr plugins, and Ghostty, which the module names Flume's `terminal` and `terminal_app` where the host names neither. Each is overridable: a host replaces either through the modules' options, or drops it for its own.
- **One way in.** The module installs a Boomdrive app, in `/Applications/Nix Apps` on macOS and as a desktop entry on Linux, which runs `flume open`, and the flake gives a shell, its `PATH` the set's tools, that starts herdr on entering. Each runs at the person's asking: the runner still starts no multiplexer (core.13) and names none (core.29).
- **crew's partitions are written once, in crew's module.** `services.crew.settings.partitions` lists every partition with its `machine`, and `services.crew.host` names this host to crew. Where the control plane is crew and no `[control_plane.crew]` is written, the Boomdrive module writes it: the host from `services.crew.host`, and each partition whose `machine` is this host. A table written out wins. The runner's file still names its partitions (core.17), and neither the runner nor crew changes.
- **A laptop** writes `services.boomdrive.repositories.<owner>.names` and nothing else: no stream, no pool, every place in Flume's default stream under the login's own program.

Not taken: the runner reading crew's `teams.json`, which core.17 rules out and which ties the runner to crew's file versions; the runner reporting each clone's folder to Flume, which Flume naming a repository by its address (flume record 0055) makes needless.

### Consequences

- Rules core.18 and core.46 say a tool's file is written by its own module, and the Boomdrive module sets the others' options; core.44's details take herdr and Ghostty into the set; core.5's details say the owner block's stream is no key of the runner's file. 
- The configuration and service-modules specs change through an OpenSpec change.
- The queued units of the bolt `clones-are-found-from-boomdrive`, which give Flume each clone's folder, are no longer needed once Flume names a repository by its address.
- Flume publishes a flake with its `services.flume` module (flume record 0055), sluice a module for its host settings, and afterthought/crew a flake with its package and a module for its host file.
- afterthought/swancloud, once the modules land, passes deneb's facts as option values and stops writing Flume's, sluice's and crew's files.
