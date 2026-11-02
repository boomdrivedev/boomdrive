---
number: 9
title: A host is set up through each tool's own commands, and its repositories follow its Flywheels
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# A host is set up through each tool's own commands, and its repositories follow its Flywheels

Status: accepted by the user on 2026-11-01, in the constitution interview.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01, its topics on control planes and on several hosts and partitions; record 0002, its first point; agentplot/flywheel-cloud's records 0003 and 0007 and its rule core.15, cited by id; flume's change `a-host-declares-where-its-kinds-come-from` and its change on layers of an agent's settings; rules core.1 and core.5.

## Context and problem statement

Setup is one of the runner's own commands (record 0008). Still open: how setup works, what it reaches, and where a host's repositories come from.

core.5 and record 0002 say the runner's configuration names the repositories it keeps. Their details ruled out a repository being added because a control plane registered it. On the operator's own hosts, nix writes the configuration of the runner and of Flume.

The user framed setup: "you would be adding the start of the host via the New Host Wizard through the cloud setup. You'd get one quick install command to run and then, in theory, the host would get configured either through a wizard in the TUI or through setup from the cloud that gets pushed down." And of repositories: "I imagine a host gets its repository list inferred from the flywheels it's servicing, not from some kind of initial setup. I think we could probably have a similar concept for kinds."

## Considered options

For how setup works: by each tool's own commands, or by the runner writing a tool's files.

For what it covers: the runner's configuration, Flume's streams, Flume's kind sources, and installing the tools.

For a host whose configuration nix keeps: setup refuses what nix keeps; swancloud stops keeping it; or swancloud runs setup itself.

For core.5: a host keeps the repositories of the Flywheels it serves plus any of its own level, or the control plane's alone.

## Decision outcome

- **Through each tool's own commands**, the user's choice. Setup configures a host only through each tool's command line, the runner's own included, and never writes a tool's files. Flume's streams and kind sources are set through Flume's commands (core.18).
- **The wizard.** A host is added by one install command carrying its enrolment token. It is then configured by a wizard in the runner's terminal interface on the host, or by setup the control plane sends down, and both run the same tool commands. Setup covers all four: the runner's configuration, Flume's streams, Flume's kind sources, and installing the tools.
- **A managed level.** In the user's words, "If the tools support multiple levels of config, and the Flywheel Cloud manages one level while we can set defaults at a different level with nix, then any configs managed by the Flywheel Cloud would stop being managed by nix." The runner's configuration, and each tool's, has a level the control plane manages, apart from the host's own defaults. What the managed level holds wins.
- **Configured by hand, or managed.** In the user's words, "think about it as open source tools that could be configured by hand or a simplified cloud management interface that handles the configuration for you." The runner is configured by its own configuration alone, which a person can write by hand, and under the cloud it does nothing that a configuration written by hand could not ask of it (core.17).
- **A host's repositories follow its Flywheels.** Under the hosted control plane, a host keeps the repositories of the Flywheels it serves. Setup writes them into the managed level, and a repository leaves it when no Flywheel the host serves holds it, then goes by the let-go rules as they stand. Under crew, the configuration names them by hand. There is no separate mapping of repositories to hosts.
- **Kinds follow the Flywheel.** Kinds are assigned to a Flywheel, and setup pushes its kind sources to every host that serves it.

core.5 stands: the configuration names every repository kept, its managed level among them. Its details are amended in this record's commit. What they rule out narrows to a clone made, or a repository added, because work named it. What they allow gains the managed level written by setup from the Flywheels the host serves. Record 0002's first point reads the same way.

### Consequences

- Flume is asked for commands that declare and remove a stream and a kind source, and for a managed level of its own. sluice is asked for the same.
- On a nix host, nix writes the host's level of defaults and stops writing what the control plane manages.
- In what order a host takes up a Flywheel's repositories, and whether Boom's repositories are among them, is still open.
