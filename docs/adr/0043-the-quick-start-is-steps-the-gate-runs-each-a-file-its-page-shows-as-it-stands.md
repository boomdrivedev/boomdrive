---
number: 43
title: The quick start is steps the gate runs, each a file its page shows as it stands
status: accepted
date: 2026-11-11
decision-makers:
- the construct stage of change/boomdrives-quick-start-takes-a-stranger-to-a-working-drive
---

# The quick start is steps the gate runs, each a file its page shows as it stands

Status: accepted. The direction is the change's brief. How it is carried out is the construct stage's own choice, within the rules, each part cheap to change, and none turns on a word of the user's.

Deciders: the construct stage of `change/boomdrives-quick-start-takes-a-stranger-to-a-working-drive`, from its brief: "A person with a Mac or a NixOS machine and nothing of the family installed follows Boomdrive's quick start on its site from nothing to a drive whose agents build and land a change, each step of it one a test runs".

Sources: record 0025 and its amendments of 2026-11-09, under which the site publishes at the opening; record 0038, site.5 and site.7; record 0009 and its amendments of 2026-11-09 and 2026-11-11, whose consequence asks Flume for a command that sets its host keys; records 0020 and 0023, the flake and its modules; agentplot/blueprints `design/principles.md` 3, 10, 11 and 14; agentplot/blueprints `design/research/2026-11-08-the-boomdrive-context-pack.md`, its "Plugins, sites and the record of a release"; sluice's site at `85773167`, its quick start and its guide `github-app`, under which sluice lands with the host's own git credentials where no `[forge]` is named; Flume's configuration reference at `e2973d57`, its `[host]` `multiplexer` and `terminal_app`, each `printed` where none is given; the setup spec; `crates/boomdrive/tests/readme.rs` and `nix/tests/cache.nix`, which hold README's examples to what the runner and the flake read; rules core.18, core.41, core.45, site.5 and site.7; the OpenSpec change `boomdrives-quick-start-takes-a-stranger-to-a-working-drive`, whose design gives each point below.

## Context and problem statement

Boomdrive's site has no quick start. A stranger's way from nothing to a working drive is spread over README's sections, which name commands a test reads in part. The brief asks for one page, a Mac's way and NixOS's, each step of it one a test runs. Three things meet it.

- **The gate's reach.** The gate runs with no network, and its nix evaluates and never builds (core.8, record 0020). A quick start's steps reach just what the gate cannot: Nix's profile and its switch, GitHub's sign-in and its repositories, Claude Code's agents and Pending You's sign-in.
- **A Mac's Flume.** On a Mac Nix does not manage, setup writes none of Flume's files (core.18). Flume with no file of its own runs the printed command as its multiplexer, so the runner's launches print their commands and no agent builds. Flume has no command that writes its host keys yet (record 0009, its amendment of 2026-11-11).
- **The site's words.** A change to a page's words is content work, and a new page type, its schema and its checks are an OpenSpec change (site.5).

## Considered options

For what a step's test is:

1. **Each step's text is files, which the page shows as they stand and one named test runs as they stand.** A step that reaches what the gate cannot runs there with stand-ins for that alone, and its text is held to what the family's own tool names for it.
2. The steps are words on the page, which README's tests and the specs vouch for.
3. One test outside the gate, a machine booted on CI with the network, runs the whole quick start from nothing.

For a Mac's Flume:

1. **The reader writes Flume's host keys by hand, `[host]` with `multiplexer = "herdr"` and `terminal_app = "terminal"`, a step of its own.** It stands until Flume has the command record 0009 asks for and setup runs it.
2. The Mac's way goes through nix-darwin, whose module writes Flume's file.
3. The quick start waits for Flume's command.
4. Setup writes Flume's file.

For where the reader starts:

1. **The reader brings an Apple silicon Mac with Nix and its flakes on, or a NixOS machine whose configuration is a flake, a GitHub account, and Claude Code.** Each is named and linked, and none is a step.
2. Installing Nix and Claude Code are steps.

## Decision outcome

**Option 1 for a step's test.** The words a reader copies are the text a test runs, so the page can't drift from what runs. A step's runnable text is a file under `site/src/start/`, in its own language: a shell line, a TOML file, a Nix file. The page shows each file as it stands. The step names one test, in the runner's crate or in the flake's checks, that reads the file by its path and runs it: through `sh`, as the program that reads it, or by nix's evaluation, as the flake's own checks evaluate a module. The family's tools in those runs are the gate's: the runner built from the tree, and the set's Flume and sluice by their paths (core.45). What the gate cannot reach is stood in for, and that alone: gh, Claude Code, Pending You, herdr's server, `launchctl` and nix's profile and switch. Where running a step's line would only run a stand-in, the test holds the line to what the family's own tool names for it: the sign-ins setup names, the state repository setup makes, and `start`, the command setup's answer gives. The last step's test runs a change from the drive's control plane to its landing, through the runner, the set's Flume and the set's sluice, with a stand-in agent. Not taken: option 2, under which a step changes with nothing to fail. Not taken: option 3, a machine booted with the network on each push. It would prove what the modules' evaluated tests and the runner's tests prove part by part, and the gate could not run it. It may come later, once Timber's measurements weigh it (principle 15).

**Option 1 for a Mac's Flume.** Principle 10 lets a person write a tool's file by hand, and the host keys are the host's own: how it opens its agents (principle 14). The drive's stream is not needed: a launch that names no stream runs in the stream Flume gives its repository, `default` where none is named. Terminal.app is on every Mac, while the profile's Ghostty is in no folder `open` reads. Not taken: option 2, which asks a stranger to adopt nix-darwin for one file. Not taken: option 3, which leaves strangers with no quick start for as long as Flume has no command. Not taken: option 4, which core.18 rules out.

**Option 1 for where the reader starts.** Nix's and Claude Code's installers are their own, they need the network, and a test of them would test someone else's installer. The page names each with its link, and the steps start where they end. Intel Macs are left out, since nixpkgs builds nothing for one and the flake gives it nothing.

**The steps' words are content.** This change writes each step's first words, since the page type is new and its files are what the tests run. A later rewording is the content kind's (site.5), and the gate holds each reworded file to its step's test.

### Consequences

- Good: what the page shows is what the gate ran, so a stranger copies only text a test passed.
- Good: a stranger on a Mac reaches agents that build in herdr now, without nix-darwin.
- Bad: a step that reaches GitHub, Claude Code or Nix's switch is proved only up to that edge.
- Bad: a Mac's reader writes one file by hand that a module writes on a host Nix manages. Principle 11 retires it: the change that has setup run Flume's command removes that step, its file and its test.
- The site gains site.8, and its design a section, "The quick start".
