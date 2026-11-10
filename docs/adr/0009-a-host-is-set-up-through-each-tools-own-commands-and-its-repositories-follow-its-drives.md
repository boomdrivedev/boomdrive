---
number: 9
title: A host is set up through each tool's own commands, and its repositories follow its drives
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# A host is set up through each tool's own commands, and its repositories follow its drives

Status: accepted by the user on 2026-11-06, in the design agent's pane, with its amendment of that day: "Other than that nitpick I approve them all". Proposed on 2026-11-01, from the user's words in the constitution interview.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01 (agentplot/boomdrive's `design/research/the-constitution-interview.md`), its topics on control planes and on several hosts and partitions; record 0002, its first point; agentplot/boomdrive-cloud's records 0003 and 0007 and its rule core.15, cited by id; flume's change `a-host-declares-where-its-kinds-come-from` and its change on layers of an agent's settings; rules core.1 and core.5.

## Context and problem statement

Setup is one of the runner's own commands (record 0008). Still open: how setup works, what it reaches, and where a host's repositories come from.

core.5 and record 0002 say the runner's configuration names the repositories it keeps. Their details ruled out a repository being added because a control plane registered it. On the operator's own hosts, nix writes the configuration of the runner and of Flume.

The user framed setup: "you would be adding the start of the host via the New Host Wizard through the cloud setup. You'd get one quick install command to run and then, in theory, the host would get configured either through a wizard in the TUI or through setup from the cloud that gets pushed down." And of repositories: "I imagine a host gets its repository list inferred from the flywheels it's servicing, not from some kind of initial setup. I think we could probably have a similar concept for kinds."

## Considered options

For how setup works: by each tool's own commands, or by the runner writing a tool's files.

For what it covers: the runner's configuration, Flume's streams, Flume's kind sources, and installing the tools.

For a host whose configuration nix keeps: setup refuses what nix keeps; the host's nix configuration stops keeping it; or that configuration runs setup itself.

For core.5: a host keeps the repositories of the drives it serves plus any of its own level, or the control plane's alone.

## Decision outcome

- **Through each tool's own commands**, the user's choice. Setup configures a host only through each tool's command line, the runner's own included, and never writes a tool's files. Flume's streams and kind sources are set through Flume's commands (core.18).
- **The wizard.** A host is added by one install command carrying its enrollment token. It is then configured by a wizard in the runner's terminal interface on the host, or by setup the control plane sends down, and both run the same tool commands. Setup covers all four: the runner's configuration, Flume's streams, Flume's kind sources, and installing the tools: the host's, never a repository's pins (the amendment of 2026-11-05 below).
- **A managed level.** In the user's words, "If the tools support multiple levels of config, and the Flywheel Cloud manages one level while we can set defaults at a different level with nix, then any configs managed by the Flywheel Cloud would stop being managed by nix." The runner's configuration, and each tool's, has a level the control plane manages, apart from the host's own defaults. What the managed level holds wins.
- **Configured by hand, or managed.** In the user's words, "think about it as open source tools that could be configured by hand or a simplified cloud management interface that handles the configuration for you." The runner is configured by its own configuration alone, which a person can write by hand, and under the cloud it does nothing that a configuration written by hand could not ask of it (core.17).
- **A host's repositories follow its drives.** Under the hosted control plane, a host keeps the repositories of the drives it serves. Setup writes them into the managed level, and a repository leaves it when no drive the host serves holds it, then goes by the let-go rules as they stand. Under crew, the configuration names them by hand. There is no separate mapping of repositories to hosts.
- **Kinds follow the drive.** Kinds are assigned to a drive, and setup pushes its kind sources to every host that serves it.

core.5 stands: the configuration names every repository kept, its managed level among them. Its details are amended in this record's commit. What they rule out narrows to a clone made, or a repository added, because work named it. What they allow gains the managed level written by setup from the drives the host serves. Record 0002's first point reads the same way.

### Consequences

- Flume is asked for commands that declare and remove a stream and a kind source, and for a managed level of its own. sluice is asked for the same.
- On a nix host, nix writes the host's level of defaults and stops writing what the control plane manages.
- In what order a host takes up a drive's repositories, and whether Boom's repositories are among them, is still open.

## Amendment, 2026-11-07: how the runner reads its two layers

Status: the writing agent's, made on 2026-11-07 within the amendment of 2026-11-06 below, which the user accepted. Each choice is cheap to change and turns on no word of the user's.

Deciders: the agent writing the change `the-runner-reads-a-local-layer-over-its-managed-one`, from its brief of 2026-11-07: "The runner reads its local layer, config.local.toml, over its managed one, taking from it only the keys it lists for it, its repositories added to the managed layer's, and a key the managed layer pins kept."
Sources: the amendment of 2026-11-06 below; flume record 0032 and Flume's reading of its two layers, `HostConfig::layers_in`; the change's design, its Decisions 1 to 7; rules core.2, core.5, core.12, core.17, core.18, core.23 and core.43.

### Context and problem statement

The amendment of 2026-11-06 gives the runner a managed layer and a local one, in Flume's words, and says what the local layer may set. It leaves open where the runner finds its local file when `--config` names a store path, which keys of a control plane's table are the local layer's, how the two layers' partitions meet, where a pin is written, and what a pass does while a local file stands that it cannot take.

### Decision outcome

- **The local file is in the runner's own configuration directory.** It is `$XDG_CONFIG_HOME/boomdrive/config.local.toml`, else `~/.config/boomdrive/config.local.toml`, whatever `--config` names. Not taken: the file beside the one `--config` names, as Flume's is, since a Nix host's service names a store path and nothing can be written beside it; and a `--local-config` option, which the service and every command run by hand would have to pass alike.
- **Each implementation lists its own keys.** The local layer may set `repositories`, and in `[control_plane.<type>]` each key that implementation lists, `partitions` for crew's and for the hosted one's, each one key taken whole. The runner takes each list from the implementation, and names none of their keys (core.2). Not taken: `partitions` for the hosted control plane alone, since a host under crew is configured by hand (core.17) and its operator's local layer may name a partition as a control plane's would; and the runner listing the implementations' keys itself.
- **Repositories add, and partitions replace.** The repositories are the managed layer's, in their order, then each of the local layer's the managed layer does not name, in its order; a repository both name is kept once, with the managed layer's spelling, and neither layer is refused for it. A local layer's `partitions` replace the managed layer's whole, as every unit of Flume's does. Not taken: partitions added as repositories are, which the amendment of 2026-11-06 says of repositories alone.
- **The reading, in order.** The managed layer is read as one file is, and any fault of it refuses the configuration. A local file that is not TOML, that sets any key the local layer may not set, naming each by its dotted path and never its value, or whose units leave a table the runner's reading or the implementation's own refuses, is not taken, and the managed layer runs alone. Not taken: a local layer half taken, its good keys read and its bad ones dropped, which Flume does not do either.
- **A local layer not taken lets go of nothing.** While a local file stands that is not taken, a keeping pass keeps the managed layer's repositories, lets go of none and sets no workspace, since the layer it could not take may name them; a local file that does not exist is no layer, and what only it named is let go. Not taken: the managed layer alone letting go as it says, which would remove the clones and views of every drive a control plane added at one bad write; and the whole configuration refused, as a broken managed file is, which would stop the managed layer's repositories too.
- **`pin` stands at the managed layer's top.** It is a list of dotted keys in TOML's spelling, each a key the local layer may set or a table holding one; a pin of anything else is the managed layer's fault, and the local layer may not set `pin`. The runner has no `[host]` table, so Flume's word and spelling are kept at the top. Not taken: a `[local]` table or `pinned_keys`, a second word for one thing across the family (core.12).
- **Said once each way.** A keeping pass says one line naming the local file where its reading differs from the last one said: taken, with each key passed over and its pin; not taken, with each key or the fault; or gone. Not taken: a line at every pass, since a pinned key passed over is a steady state a host may keep, and core.23 says the runner's reach once each way for the same reason.

core.17's details are amended in this record's commit with what this amendment rules out and allows. core.5's and core.46's details read the local layer where they said the managed level of the decision above.

### Consequences

- A hosted control plane writes the repositories and partitions of the drives a host serves into `config.local.toml`, and Nix's `config.toml` stays as Nix wrote it (core.18).
- A control plane writes the file whole and renames it into place, so a pass never reads it half written.
- Removing the local file lets go of what only it named; a person who means to keep those repositories moves them into the managed layer first.

## Amendment, 2026-11-06: two layers in every tool, in Flume's words

Status: accepted by the user on 2026-11-06, in the design agent's pane: "Other than that nitpick I approve them all" (the nitpick being flume record 0055's).

Deciders: the user, in the research talk `research--the-flakes-configuration-holds-streams-and-repositories` on 2026-11-06 (agentplot/boomdrive's `design/research/the-flakes-configuration-holds-streams-and-repositories.md`), asking "once we have the cloud control plane, can a repository get added from the cloud and written into the other config file?", noting "we have a mismatch in terminology [...] in terms of what we call a managed config", and choosing the words, "Flume's: managed + local", what a local layer may set, "Flume's rule for all", and repositories under the cloud, "Add to Nix's". The design agent for writing it here.
Sources: `design/the-flakes-configuration/design.md`; flume record 0032 (a managed and a local layer); record 0023; rules core.5, core.17 and core.18.

### Decision outcome

- **Two layers, by one pair of words.** Each tool's configuration is its managed layer, `config.toml`, written by Nix, a fleet or a person, and its local layer, `config.local.toml` beside it, written by the operator or a control plane and read over it. The managed level above is the runner's local layer.
- **The local layer sets only what runs the work.** Each tool lists the keys its local layer may set; for the runner, its repositories and its control plane's partitions. Accounts, pools, credentials' paths and every other key are the managed layer's alone, and a local file that sets one is not taken. The managed layer may pin any key the local one could set, which the local layer then cannot change.
- **Repositories add.** The runner keeps every repository its managed layer names and every one its local layer names. A control plane that drops a drive drops only the repositories it wrote.
- **A control plane writes a tool's local file.** Setup under a hosted control plane writes the runner's and Flume's local layers, with the repositories of the drives the host serves and each one's stream by its address, and the files Nix wrote stay as Nix wrote them. This replaces setup through each tool's own commands above, and Flume is asked for no command that declares a stream.

Rules core.17 and core.18 are amended with it.

## Amendment, 2026-11-05: setup installs the host's tools, never a repository's pins

Status: accepted by the user on 2026-11-05, on their approval of flume's cross-tool design `a-places-readiness-is-one-design-across-the-tools`, whose decision 7 words this amendment.

Deciders: the user, 2026-11-05, who approved the design's words for this record. How they read beside record 0012 is the writing agent's.
Sources: agentplot/flume's change `a-places-readiness-is-one-design-across-the-tools`, its decisions 1, 6 and 7 and its finding 11; flume's record 0036; the statements' clauses on agentplot/blueprints main at 9f1f0fd, sluice's clause 147 (`sluice equip`) among them; record 0012; rules core.14 and core.37.

### Context and problem statement

The decision above has setup cover "installing the tools", and says neither whose tools nor which. On 2026-11-05 a Flume place launched on a fresh clone the runner had made failed its readiness, because nothing had installed what its repository's `.prototools` pins. flume's design gives each part of a place's readiness one owner across the tools: the repository declares the tools it needs, sluice equips a working copy with them (`sluice equip`, sluice clause 147), and flume asks sluice to equip each place at its launch (flume record 0036). The runner's clone and view are never worked in, and the runner readies nothing.

### Decision outcome

- **The host's tools, never a repository's pins.** In decision 7's words, this record's "installing the tools" says "the host's tools (the runner, flume, sluice and herdr), never a repository's pins, which flume's places are equipped with by sluice". Setup installs no tool a repository pins, in its `.prototools`, its toolchain or its runner's setup.
- **Record 0012 stands** on which of the host's tools are installed: the one install brings the runner, Flume and sluice, at the versions the release was checked against, and herdr, the host's multiplexer, is found on the host and never installed.

Nothing else of the decision above changes. core.37's details are amended in this record's commit to rule out a repository's pins brought by an install or by setup, and record 0012's lines on the install cite this amendment.

### Consequences

- The runner runs nothing in a clone or a view to make it ready (core.14), and its setup, when built, has no step that reads a repository's pins.
- A repository's pins are its own to install: in a place by sluice's equip, and for a person's or an agent's shell by the repository's own devenv task, as this repository's `boomdrive:proto:install` does.

## Amendment, 2026-11-09: a person's own drive, set up by one command on a host Nix does not manage

Status: the route and its timing are the user's, chosen on their card on 2026-11-09. The rest is the design agent's, each choice cheap to change and given with the path not taken.

Deciders: the user, on their card "Colleagues running their own drive with crew: may Nix manage their Macs, and how soon?", who chose "No: Nix installs the programs, a setup command does the rest" and "With Boomdrive going public". Before it, in the design agent's pane on 2026-11-08, of colleagues on their own Macs: "The idea is that they would be able to start talking to crew and create work items that the runner pulls and schedules, with decisions put on pending-you. That is the same way I'm working today."; "They wouldn't be taking my work and they'd be doing their own work. They'd have to set up their own crew state."; "How do we assist them in that setup process?"

Sources: the decision above and its amendment of 2026-11-06; records 0012, 0023 and 0028; agentplot/crew README, `crew main up` and `crew signal land`.

### Decision outcome

- **The programs from Nix, the rest by setup (the user's).** A person whose Mac Nix does not manage installs Boomdrive's programs with `nix profile add` and runs setup, which does the rest. It is ready when Boomdrive goes public. Not taken: a starter nix-darwin configuration for each person, which the user turned down.
- **One command, which an agent can run.** Setup is this record's wizard, run as one command. Each of its questions is also a flag, and it reports in JSON, so an agent in the person's own session can run it for them. Run again with the same answers, it changes nothing. Not taken: a wizard with no flags, which an agent cannot run.
- **What the person gives once.** Their account, their drive, the drive's repositories by owner, and the drive's crew state repository. Setup makes the state repository where none exists, under the person's account. Not taken: asking the person to make the state repository by hand first.
- **What setup leaves.**
  - The runner runs as the login's service, as the one install leaves it (record 0012).
  - Each tool's managed layer is written for the person by the same keys Nix writes on a host it manages, so a host reads the same whichever wrote it.
  - herdr's plugins are linked with herdr's own command.
  - crew's file names this drive and its state, so the person can start crew's agents and talk to them.
  - A file of the person's own already there is moved aside once and named, as the modules do. Not taken: a second set of keys for hosts set up without Nix.
- **Later changes are setup's too.** Adding or removing a repository of the drive is a setup command, never a hand edit asked of the person.
- **On a host Nix manages.** There, setup writes nothing Nix writes and says so.
- **Sign-ins stay the person's.** Claude Code, GitHub and Pending You are each signed in to by the person. Setup checks each and names any that is missing.

### Consequences

- crew is asked to make a new drive's state and name the drive in its own file at setup's asking, since crew alone knows its state's layout.
- A person's first drive needs no Flume setting of its own: its places run in Flume's default stream, as a laptop's do in record 0023.
