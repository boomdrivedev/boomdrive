# Tasks

Build the groups in order. Each group ends green on the moon tasks it names. A task is true once the moon task it names passes, run as it names it.

- Run each check as `devenv shell -- moon run <project>:<task>`, or through sluice while iterating, as CLAUDE.md says.
- The tests evaluate the modules against no store, as `nix/check.sh` does, with neutral names (core.41).
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.

## 1. The agent waits for the Nix store

- [x] 1.1 The nix-darwin module declares the agent with `command`, so its program is `/bin/sh -c '/bin/wait4path /nix/store && exec <package>/bin/boomdrive run --config <file>'`, and keeps every other key of the agent (design Decisions 1 and 2). True when `devenv shell -- moon run nix:test` passes with `nix/tests/darwin.nix` proving the service-modules spec's "The agent" and "The agent waits for the Nix store".
- [x] 1.2 The tests read the agent's command and file by meaning, and prove that a new file or package changes the agent's program and plist in that alone, and that `tools` leaves the plist as it was (design Decisions 3 and 5). True when `devenv shell -- moon run nix:test` passes with `nix/tests/darwin.nix` proving the spec's "A new file on nix-darwin", "A new package on nix-darwin" and "Nothing else changed on nix-darwin".
- [x] 1.3 core.46 and record 0020 say the modules start the runner only once the host's Nix store is mounted, and the README says the agent waits for the store. True when `devenv shell -- moon run root:format root:adrs` passes.

## 2. The change's checks

- [x] 2.1 Run the checks of every moon project this change touches. True when `devenv shell -- moon run nix:format nix:lint nix:typecheck nix:test root:format root:adrs root:specs root:changes` passes.
- [x] 2.2 The spec delta archives into specs that pass openspec's strict check. True when, after `openspec archive the-darwin-agent-waits-for-the-store --yes`, `devenv shell -- moon run root:specs root:changes` passes on the tree.
