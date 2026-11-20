# Design

## Context

Two changes of 2026-11-18 met at the merge of `main` at `f954b77` into the bolt `every-machine-takes-boomdrives-tools`:

- **`the-set-carries-rex-matched-to-flume`**, on `main`, added `services.boomdrive.rex.package` to `nix/modules/apps.nix`'s list of packages under `lib.mkIf cfg.enable`, so Rex came with the runner alone.
- **`boomdrives-tools-install-on-a-machine-without-the-runner`**, on the bolt, replaced that list with one `installing <tool>` for each tool, each reading `services.boomdrive.<tool>.enable`, and wrote core.63.

The merged `apps.nix` has no `cfg.enable` block left to hold Rex. Rules relied on:

- **core.63:** the set installs a tool only where the host turns it on, each by a switch of its own, which the runner's turns on by default.
- **core.46:** the Boomdrive module's own options install the tools with no module of their own, each by a switch of its own.
- **core.13:** the modules start no tool's process.

## Goals / Non-Goals

**Goals:**

- Rex is turned on as worktrunk is, by a switch of its own, on by default with the runner.
- A host with the runner gets the system it got from `main`.

**Non-Goals:**

- Naming Rex in Flume's settings, or starting Rex's server.
- A herdr plugin for Rex, which ships none.

## Decisions

### Decision 1: Rex takes `installing "rex"` and `switch "Rex"`

`services.boomdrive.rex` gains `enable = switch "Rex"`, and `config` gains `(installing "rex")` beside herdr's. That is the helper every other app of `apps.nix` uses, so Rex reads as they do.

Not taken: `(lib.mkIf cfg.enable { environment.systemPackages = … cfg.rex.package; })`, Rex with the runner alone, which core.63 rules out.

### Decision 2: Rex's cases follow worktrunk's

`nix/tests/apps.nix` gains *Rex without the runner*, as *worktrunk without the runner* is, and `rex` joins `everyTool` and the switches *Every tool turned on without the runner* turns on. *No Rex without the runner* stays: a host that leaves the runner off and turns nothing on gets no Rex.

## Risks / Trade-offs

- **None for a host with the runner.** Rex's switch is on wherever the runner's is, so its profile is as before.
