---
number: 54
title: A task's inputs are the files the commit holds, and root:format leaves out every path git ignores
status: accepted
date: 2026-11-19
decision-makers:
- boomdrive-design, by its ruling of 2026-11-19
- the code stage of fix/main--format-hashes-only-tracked-files
---

# A task's inputs are the files the commit holds, and root:format leaves out every path git ignores

Status: accepted. Point 1 is boomdrive-design's ruling of 2026-11-19 on the signal `a-key-check-refuses-files-git-ignores`, carried to this repository by the fix's brief. Points 2 to 4 are the code stage's own choices in the fix `main--format-hashes-only-tracked-files`, within the rules, each cheap to change. None is the user's alone.

Deciders: boomdrive-design, whose ruling the brief quotes: "A check's inputs are the files the commit holds: no task's inputs take in a file git ignores." The code stage of `fix/main--format-hashes-only-tracked-files`, 2026-11-19, for points 2 to 4, from the brief: "Make moon hash only tracked files for the format task: by moon's own setting for that if the pinned moon has one, read from moon's documentation, or by inputs that leave every ignored path out. treefmt reads only what git tracks, so the check itself is unchanged."

Sources: boomdrivedev/sluice's `check_key` in `crates/sluice/src/profiles/moon.rs` at `75fd576`, which refuses a key whose manifest names an input the tree lacks, `<task>: <path> is not in the tree`; moonrepo/moon at `v2.6.1`, `crates/task-hasher/src/task_hasher.rs`, `aggregate_inputs`, and `crates/cache/src/cache_engine.rs`, `hash_files`; moon's documentation of `hasher.walkStrategy` and of a task's glob inputs; git's `gitignore(5)`; commit `d0d19a94`, which first negated `.gitignore`'s entries among root:format's inputs; record 0022; rule core.7.

## Context and problem statement

sluice signs a run's key for a task only where the key is moon's for the combination's tree: every input its hash manifest names is a file the tree holds, with the content the manifest gives. A task whose inputs take in a file git ignores names a file no commit holds, and its key is refused. boom's submission #1 was refused on `.devenv/bootstrap/bootstrapLib.nix` and timber's #1 on `.claude/settings.json`, each a file the devenv shell writes and git ignores, each taken in by their root:format's `/**/*`.

boomdrive's root:format has negated each entry of `.gitignore` among its inputs since `d0d19a94`, so its landings pass. Two gaps were left. An entry git matches at any depth, `.devenv/`, `.devenv.flake.nix`, `devenv.local.nix` and `.pre-commit-config.yaml`, was negated at the root alone, so a nested one was hashed. And `.claude/settings.local.json`, which Claude Code writes in a checkout for a person's own settings, was neither ignored nor left out.

moon 2.6.1 has no setting that limits a task's inputs to what git tracks. For a project at the workspace's root, `aggregate_inputs` walks the file system for the task's globs whatever `hasher.walkStrategy` says, and the native file hashing, on by default since 2.5, hashes each file it is given. The files `git status` lists as changed, untracked ones among them, are added for every project. For a project below the root, moon resolves the project's own globs from git's listing, `git ls-files --cached --modified --others --exclude-standard`, so `site:test`'s `dist/**/*` hashes no file git ignores. A glob from the workspace's root, `/…`, is walked over the file system in any project; root:format's `/**/*` is the one that spans the whole tree, and the folders the others name hold no path `.gitignore` names.

## Considered options

1. **`/**/*`, with each entry of `.gitignore` negated where git matches it** (chosen).
2. `hasher.walkStrategy: vcs` in `.moon/workspace.yml`.
3. Inputs that name the tree's top-level folders and files, each as a glob.
4. A file group of every tracked file, written from `git ls-files` and held to it by a check.
5. root:format uncached, `cache: false`.

## Decision outcome

1. **No task's inputs take in a file git ignores, so a task's key names only files the commit holds (boomdrive-design's).** core.7 says so.
2. **root:format's inputs are `/**/*`, with each entry of `.gitignore` negated where git matches it (option 1).** An entry with no slash but a last one is negated at any depth, `!/**/<entry>`, and one with a slash before its last is negated at the root, `!/<entry>`. A folder's entry is negated with `/**/*` after it. An entry added to `.gitignore` is added among the negations, and each file says so. With a file planted at each ignored path, at the root and nested, moon hashed the 1,275 files git tracks, and with main's inputs 1,280.
   - Not taken: option 2. moon reads the strategy only for a project below the workspace's root, and `d0d19a94` measured no change for root:format.
   - Not taken: option 3. A folder or a file added at the top of the tree, and named in no glob, would change nothing of the key, so a replay could pass a file treefmt was never run on.
   - Not taken: option 4. Every file added or removed would change `moon.yml`, and the check would be one more task for what the negations already do.
   - Not taken: option 5. treefmt would run over the whole tree at every pass, where a key that matches now replays it.
3. **`.claude/settings.local.json` is ignored and left out of root:format's inputs.** Claude Code writes it in a checkout, a place among them, for one person's own settings, which no commit holds. Until it is ignored, treefmt reads it too, as it reads every file git does not ignore.
4. **The check itself is unchanged.** treefmt reads only what git does not ignore, so leaving the same paths out of moon's hash changes nothing it formats.

### Consequences

- Good: a key for root:format names only files the commit holds, wherever a checkout's shell, tools or person wrote files git ignores under the tree's `.gitignore`.
- Bad: the negations are kept to `.gitignore` by hand, and a missed one is found when sluice refuses a key, naming the file.
- Bad: a file ignored only by a host's own excludes, `core.excludesFile` or `info/exclude`, is outside the tree's reach and still hashed where a checkout the gate runs in holds it, and refused the same way. So is an untracked file git does not ignore.
- core.7's rule and details say so, in the commit that writes this record.
