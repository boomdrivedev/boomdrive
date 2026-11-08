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
Sources: the constitution interview of 2026-11-01 (agentplot/boomdrive-archive's `design/research/the-constitution-interview.md`), its topics on control planes and on several hosts and partitions; record 0002, its first point; agentplot/boomdrive-cloud's records 0003 and 0007 and its rule core.15, cited by id; flume's change `a-host-declares-where-its-kinds-come-from` and its change on layers of an agent's settings; rules core.1 and core.5.

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

## Amendment, 2026-11-06: two layers in every tool, in Flume's words

Status: accepted by the user on 2026-11-06, in the design agent's pane: "Other than that nitpick I approve them all" (the nitpick being flume record 0055's).

Deciders: the user, in the research talk `research--the-flakes-configuration-holds-streams-and-repositories` on 2026-11-06 (agentplot/boomdrive-archive's `design/research/the-flakes-configuration-holds-streams-and-repositories.md`), asking "once we have the cloud control plane, can a repository get added from the cloud and written into the other config file?", noting "we have a mismatch in terminology [...] in terms of what we call a managed config", and choosing the words, "Flume's: managed + local", what a local layer may set, "Flume's rule for all", and repositories under the cloud, "Add to Nix's". The design agent for writing it here.
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
