# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it.

- Every case evaluates against no store, as `nix/check.sh` does, with core.41's neutral names.
- No task moves an input, or adds a crate, a tool or a tracked file type (core.7, core.8, core.45). The runner's code does not change. Nothing outside the paths proposal.md's Touches lists is changed but this change's own files.

## 1. The marketplace

- [x] 1.1 `boomdrive-claude-plugins` copies each tool's plugins whole, from the folders design Decisions 1 and 2 name, lists each at its tool's version, and fails its build where a manifest says another (design Decisions 3 and 4). `flake.nix` gives it Flume, sluice, crew, worktrunk and worktrunk's input. True when `sluice exec main -- moon run nix:test` passes, proving "The marketplace's plugins", "No link into the store", "Each tool's plugins from its tool's pin", "Each tool's plugins say its tool's version", "A plugin other than its tool" and "worktrunk's plugin, which names no version", and Plannotator's and timber's cases as before.
- [x] 1.2 The nix files this change changes keep to the nix project's format, lint and type check. True when `sluice exec main -- moon run nix:format nix:lint nix:typecheck` passes.

## 2. The words

- [x] 2.1 Record 0044's amendment of 2026-11-18 names crew's and worktrunk's plugins and how each is taken, and core.44's details name the marketplace's plugins of the set's tools. The README, `CLAUDE.md` and `openspec/config.yaml` name the marketplace's plugins. True when `sluice exec main -- moon run root:format` passes.

## 3. The specs

- [x] 3.1 The change validates and archives into specs that pass OpenSpec's strict check (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
