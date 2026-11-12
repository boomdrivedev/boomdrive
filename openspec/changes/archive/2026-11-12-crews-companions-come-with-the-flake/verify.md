# Verify: crews-companions-come-with-the-flake

The verify stage's report on the branch `change/crews-companions-come-with-the-flake` at `59f6602`, 2026-11-12. The work holds: the whole checks pass, every task is ticked, and every requirement and scenario of the four delta specs is met and held by a test the gate ran.

## The whole checks

`devenv shell -- sluice exec main`, run once on the branch as it stood, at `59f6602`:

- **Outcome: passed, exit 0.** sluice recorded 34 results under their keys for `59f6602`. moon ran 36 tasks, none from its cache.
- **The runner's crates:** `boomdrive`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`, each `format`, `lint`, `typecheck` and `test`.
- **The flake:** `nix:format`, `nix:lint`, `nix:typecheck` and `nix:test`.
- **The root:** `root:format`, `root:toolchain`, `root:adrs`, `root:specs`, `root:changes`, `root:commits`, `root:versions`, `root:releases` and `root:workflow`.
- **The site:** `site:install`, `site:check`, `site:prose`, `site:vocab`, `site:build`, `site:test` and `site:workflow`.

moon warned once that it could not write `boomdrive:test`'s outputs into its own cache folder. That is moon's cache, which the gate does not read, and the task passed.

## The tasks

All 16 tasks of `tasks.md` are ticked, 1.1 to 5.2, and none is open. Each names a check, and each check is among those the gate ran and passed: `nix:typecheck` for 1.1 to 1.3 and 2.1, `nix:test` for 1.4, 2.2, 3.1 to 3.5, 4.1 and 4.2, `root:format` for 4.3, `nix:format nix:typecheck nix:test root:format` for 5.1, and `root:specs` after the archive for 5.2.

Task 5.2's check is the archive itself, so the code stage archived the change before this stage ran, at `59f6602`. This report is therefore written into the archived change's folder, and the archive stands as the code stage made it.

## The specs

Every scenario of the four deltas is named by a case in the flake's tests, `nix/tests/`, which `nix:test` and `nix:typecheck` ran. Every requirement and scenario of the deltas is in `openspec/specs/` as archived, and `root:specs` passes on it.

| Capability | Requirements | Scenarios | Held by |
|---|---|---|---|
| `family-packages` | 3 modified, 7 added | 27 | `typecheck.nix`, `packages.nix`, `companions.nix` |
| `service-modules` | 3 modified, 9 added | 27 | `apps.nix`, `tools.nix`, `darwin.nix`, `nixos.nix`, `cache.nix`, `sources.nix`, `activate.sh` |
| `binary-cache` | 2 modified | 6 | `workflow.nix`, `cache.nix` |
| `way-in` | 1 modified | 4 | `way-in.nix`, `way-in.sh`, `typecheck.nix` |

The code was also read against each requirement:

- **The packages.** `pendingyou` is the npm package's `dist`, `mod` and `package.json`, wrapped on the flake's `nodejs_24`. `plannotator` and `plannotator-tui` are llm-agents.nix's, unchanged. `pendingyou-herdr-plugin` is a whole copy of its input. `plannotator-herdr-plugin`'s `bin` holds its runtime, built from `rust/` with that folder's `Cargo.lock`, and the set's plannotator-tui, each beside its version file.
- **The marketplace.** `boomdrive-claude-plugins` is named `boomdrive` and lists `pendingyou` and `plannotator` at `./plugins/<name>`.
  - Pending You's plugin holds its nine hooks in `init`'s order, each running the launcher by `$HOME`, with SessionEnd's untimed. It also holds the skill, and no `.mcp.json` or wake mod.
  - Plannotator's plugin is its source's `apps/hook`, copied with `-L`.
  - The build fails on any link, or where Plannotator's plugin names another version.
- **The modules.**
  - `companions.nix` installs the three packages and `pendingyou-signin` only with `services.boomdrive` and `services.crew` both on. Each is replaceable or `null`, and `null` Pending You installs no sign-in.
  - `apps.nix`'s plugins default gains the two companions' plugins after Flume's and sluice's, where crew is on.
  - `darwin.nix` and `nixos.nix` run `activate.sh pendingyou` as the runner's user, after the link and before the plugins' step, where crew is on and Pending You is not `null`.
  - No step, job or unit runs `claude` or names a Claude Code home.
- **The folder's step.** `activate.sh pendingyou` follows design Decision 8:
  - it writes the wake mod's `wake.js`, `register.js` and `hooks.json` first;
  - it writes each file by a temporary file renamed over its path, only where bytes or mode differ, and replaces a link;
  - it makes a missing folder 0700 and touches nothing else;
  - it says one line per file written, or not written, and exits 0.
- **The shell** holds the ten tools, the companions after portless and worktrunk.
- **The words.** The README's Installing with nix names the companions, their herdr plugins, the three options and their `null`, `~/.config/pendingyou` and what stays the command line's, `pendingyou-signin`, and `boomdrive-claude-plugins` as installed into no Claude Code home yet. CLAUDE.md and `openspec/config.yaml` name the companions. Record 0044 is written, record 0023 points to it, and core.md cites it.
- **What no task changes.** No crate changed (`git diff main...HEAD -- crates` is empty), and no tool of the gate or tracked file type was added.

## One note, not a shortfall

The scenario "A crew session's stop check" of `service-modules` says that for every call but a crew session's `stopcheck` the launcher `exec`s `pendingyou`. Design Decision 7 also gives the launcher Pending You's own `init` shim's early exit: for `permission-done`, it exits 0 at once where `permission-cards/` holds no card, and runs no Pending You. The code follows the design, and the test holds the launcher's tail, the stop check then the `exec`. The scenario's words leave that one case out. A later change to `service-modules` may name it in the scenario. Nothing in the work needs to change.
