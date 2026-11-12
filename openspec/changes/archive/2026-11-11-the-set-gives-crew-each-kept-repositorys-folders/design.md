# Design

## Context

crew's teams file names each repository its host keeps, by its address, with exactly two absolute paths (crew record 0061): its `dir`, the folder crew names as the kit's and cuts worktrees in, and its `checkout`, the working tree it names as the kit's main checkout. crew's module at the tag the lock names, v0.50.2, declares them as `services.crew.settings.repositories`: `null` by default, else an attribute set of entries, each a submodule of exactly `dir` and `checkout`, each `types.strMatching "/.*"`. Where it is `null`, crew's file names none. crew's `repositories_of` there reads them.

On a runner's host both folders are the runner's. The clone is the host's one clone of the repository (core.14), and the set states its folder as `services.boomdrive.cloneDirs`, held to the runner's `clone::container` by `nix/tests/clone-dirs.json` (record 0036 as amended on 2026-11-11). The view is a worktree of that clone at `<repository_root>/<owner>/<name>`, which the runner keeps for every address its file names, another machine's drives' among them (`crates/boomdrive/src/pass.rs`, `keep_configured`). `crates/boomdrive/src/config.rs` gives it:

```rust
/// A repository's view, `<repository_root>/<owner>/<name>` (design 7).
pub fn view_path(config: &Config, address: &Address) -> PathBuf {
    config.repository_root.join(address.owner()).join(address.name())
}
```

`repository_root` is `~/Code` under the home where the file names none. Where it names one, `root_path` takes it beginning with `~/`, joined under the home, or absolute, as written, and refuses any other. The home is the job's `HOME`: the primary user's on nix-darwin, which the agent's `EnvironmentVariables` sets, and `services.boomdrive.user`'s on NixOS, which the user's manager gives. The local layer sets no `repository_root` (core.17), so the managed layer's is the runner's.

No option of the set's gives crew these folders. A host's draft of 2026-11-09 built crew's map in its own Nix from its accounts and the runner's layout, under the folder of before the rename. Record 0036's point 1 has the set give the map, and core.57 rules out a host building it.

## Goals / Non-Goals

**Goals:**

- With the runner and crew's module on, crew's kept repositories are each address the runner keeps, its `dir` the runner's clone and its `checkout` the runner's view (record 0036, point 1; core.57; principle 14).
- No host writes a kept repository's folders, and a host's value that names another fails evaluation.
- The module's view rule cannot drift from the runner's with no check failing, as `cloneDirs` cannot drift from `clone::container`.

**Non-Goals:**

- A read-only option stating each view, beside `cloneDirs` (Decision 3).
- crew's other settings, its partitions and host among them, which stay the host's (record 0036, point 4).
- Any change to the runner's code, its file's keys, its data folder, its layout or its views.
- A repository only the runner's local layer names, which `cloneDirs` does not hold (record 0036's second amendment of 2026-11-11).

## Decisions

### 1. Each platform module gives crew's map, with the runner and crew's module on

`nix/modules/darwin.nix` and `nix/modules/nixos.nix` each give, under their `lib.mkIf cfg.enable`:

```nix
services.crew.settings.repositories = lib.mkIf (config.services.crew.enable && cfg.cloneDirs != { }) (keptOf {
  inherit (cfg) cloneDirs;
  inherit home;
  root = cfg.settings.repository_root;
});
```

`keptOf` is `nix/modules/kept.nix` (Decision 2). `home` is the module's own: the primary user's on nix-darwin, `services.boomdrive.user`'s on NixOS, the same home each gives `dataDir`'s rule and the runner's job.

The Boomdrive module sets crew's module option, and crew's module writes crew's file from it (core.18, core.46), as `drives.nix` gives Flume's module each drive's stream. crew's module is imported with the runner's (the spec's "The modules bring each tool's own module"), so `services.crew` is always declared.

Not taken:

- **One shared module imported by both platforms,** as `clones.nix` is. The view reads the job's home, which each platform reads from its own user, and a module's imports cannot read `config`. Each platform calls one function file with its own home, as each calls `data-dir.nix`.
- **Giving the map with the runner not enabled.** No runner then keeps a clone or a view, so each entry would name a folder nothing makes, and a host that runs crew without the runner keeps its own repositories.
- **Giving the map with crew's module off.** crew's module then writes no file, and nothing reads the value.

### 2. Each entry reads `cloneDirs` and the view rule: `nix/modules/kept.nix`

```nix
{ lib }:
let
  viewDirOf = import ./view-dir.nix { inherit lib; };
in
{ cloneDirs, home, root }:
lib.mapAttrs (address: dir: {
  inherit dir;
  checkout = viewDirOf { inherit home root address; };
}) cloneDirs
```

The entries are `cloneDirs`'s names, every address of every drive of every account, another machine's drives' among them (record 0028). Each `dir` is read from `cloneDirs` and never joined under `dataDir` again (core.57). Each `checkout` is the view by Decision 3. The file's header comment cites record 0036's point 1 and crew record 0061, and names `cloneDirs` and `view-dir.nix` as the rules it reads.

Not taken: an entry's `dir` joined from `dataDir`, which writes the clone's layout a second time (record 0036's second amendment).

### 3. One file holds the view rule: `nix/modules/view-dir.nix`

```nix
{ lib }:
{ home, root, address }:
let
  join = base: name:
    if lib.hasPrefix "/" name then name
    else if lib.hasSuffix "/" base then "${base}${name}"
    else "${base}/${name}";
  homeOf = if home == null then throw "..." else toString home;
  base =
    if root == null then join homeOf "Code"
    else if lib.hasPrefix "~/" root then join homeOf (lib.removePrefix "~/" root)
    else if lib.hasPrefix "/" root then root
    else throw "...";
in
join base address
```

Each join is Rust's `PathBuf::push`: a name that is absolute replaces the base, and a `/` is added only where the base does not already end in one, as `data-dir.nix` joins. The address's one `/` sits between its owner and its name, which hold no `/` (`options.nix`), so one join of the address is the runner's two. The view is then the runner's text exactly, which crew reads as text.

Where the root is neither absolute nor begins with `~/`, such as `src` or `$HOME/Code`, the rule throws: `services.boomdrive.settings.repository_root is "<root>": the runner takes a repository root that is an absolute path or begins with ~/, and refuses this one`. The runner refuses that file at its start, so crew's map fails evaluation with it. Where the home is needed and none is given, it throws as `data-dir.nix` does.

The file's header comment names `crates/boomdrive/src/config.rs`'s `view_path` and `root_path` as the rule it follows, and `nix/tests/view-dirs.json` as the table that holds the two together.

Not taken:

- **A read-only option stating each view,** `services.boomdrive.viewDirs`, as `cloneDirs` states each clone. crew's map is the one reader of a view the set has today, and record 0036's second amendment left the views to this change for crew's `checkout` alone. A later change can declare the option over the same rule.
- **An assertion on `repository_root` in `options.nix`,** refusing a root the runner refuses for every host. That is a check of the runner's file, beside this change. Here the rule refuses it only where crew's map reads it.

### 4. Each entry at the module's own priority: a host writes none

The entries are given at the module's own priority, neither `lib.mkDefault` nor `lib.mkForce`. crew's `dir` and `checkout` are each a string, which nixpkgs merges only where every definition is equal. So:

- A host that gives a kept address another `dir` or `checkout` fails evaluation of crew's map. nixpkgs names both definitions and their files.
- A host's entry equal to the module's merges, so a host that still writes the runner's own folders evaluates. It writes them in vain, and drops them.
- A host's own entry for an address the runner does not keep stands beside the module's entries, since crew's map merges by address. That repository is the host's own, which the set does not know (core.57).
- A host that sets `repositories = null` beside the module's map fails evaluation, as nixpkgs fails a `null` beside a value.

The brief's check of the set is the gate's: the two tables of Decision 5 and `nix/tests/clone-dirs.json` hold each folder the module gives to the runner's own rule.

Not taken:

- **`lib.mkDefault`,** as `drives.nix` gives Flume's streams. A host's other folder would then win, silently, which is the very value core.57 takes from a host.
- **An assertion over crew's final map,** refusing any entry for a kept address that differs from the module's. It would fail a host's `lib.mkForce` too. A host that forces a value overrides the set on purpose, and nixpkgs already fails every other value.
- **Refusing a host's entry for an address the runner does not keep.** crew keeps the host's own repositories too, whatever the runner keeps.

### 5. One table holds the module's view rule to the runner's

`nix/tests/view-dirs.json` is a list of cases, each with a neutral home, as `config.rs`'s own tests use (core.41). `repository_root` is `null` where the file names none, and `folder` is `null` where the runner refuses the root:

| `home` | `repository_root` | `address` | `folder` |
|---|---|---|---|
| `/h` | `null` | `agentplot/flume` | `/h/Code/agentplot/flume` |
| `/h/` | `null` | `example/alpha` | `/h/Code/example/alpha` |
| `/h` | `~/src` | `agentplot/flume` | `/h/src/agentplot/flume` |
| `/h` | `~/` | `Example-1/a.b_c` | `/h/Example-1/a.b_c` |
| `/h` | `~//srv/code` | `example/alpha` | `/srv/code/example/alpha` |
| `/h` | `/srv/code` | `example/alpha` | `/srv/code/example/alpha` |
| `/h` | `/srv/code/` | `example/alpha` | `/srv/code/example/alpha` |
| `/h` | `src` | `agentplot/flume` | `null` |
| `/h` | `$HOME/Code` | `agentplot/flume` | `null` |

- `nix/tests/kept.nix`'s case "The runner's own view rule" holds `view-dir.nix` to each row: the row's folder where it names one, and a failing evaluation where it names none.
- `crates/boomdrive/tests/module_files.rs`'s `the_modules_views_are_the_runners` reads the same file with `serde_json`. For each row it parses `accounts = {}`, with `repository_root` written as a TOML string where the row's is not `null`, by `config::parse` with the row's home. Where the row's `folder` is `null`, it asserts the parse is refused. Else it asserts `config::view_path` of the row's address, as text, is the row's `folder`. Each message names the row. The file's module documentation adds that it holds the views the modules give crew to the runner's own `view_path`.
- `crates/boomdrive/moon.yml`'s `test` task takes `/nix/tests/view-dirs.json` as an input, beside `/nix/tests/clone-dirs.json`.

The runner's test compares text, not `Path`s, since `Path` equality passes a doubled or trailing `/` that crew's text would hold.

Not taken: one table with `clone-dirs.json`. The rows hold different values, and each test reads only its own.

### 6. Nothing is given where nothing is kept

With no address in `cloneDirs`, the module gives crew's map nothing, so crew's file names no `repositories` where the host names none, as before. With the runner off or crew's module off, it gives nothing either (Decision 1). The spec's "The tools' activation is their own" still holds: its system turns crew on with no repository kept.

Not taken: an empty map, `{ }`, which would write an empty `repositories` into crew's file on every runner's host that keeps nothing.

### 7. The README names it

In **Installing with nix**, a paragraph after `cloneDirs`'s says that, with the runner and crew's module on, the module gives `services.crew.settings.repositories` one entry for each address of `cloneDirs`. Its `dir` is the clone's folder, `cloneDirs`'s, and its `checkout` is the runner's view, `<repository_root>/<owner>/<name>`. It says a host writes none: an entry the host gives a kept repository with another folder fails evaluation. And it says a repository of the host's own that the runner does not keep stays the host's to name. No nix block is added.

`nix/tests/kept.nix`'s case "The README's kept repositories" holds **Installing with nix** to naming `services.crew.settings.repositories`.

### 8. The record and the rule

Record 0036 is amended a third time on 2026-11-11, with Decisions 1, 3, 4 and 6 as its points, each with the path not taken. core.57's details gain the cases they rule out and leave. Both are in this change's planning commit.

## Rules relied on

core.5 (the runner keeps a clone and a view of each address its file names); core.12 (no new option, and no key of another tool's named here); core.14 (the runner's clone is the host's one clone, and crew may work in its folder); core.17 (the local layer sets no `repository_root`); core.18 and core.46 (the Boomdrive module sets crew's module option, and crew's module writes crew's file); core.41 (the table's neutral names); core.57 (the set gives crew's kept repositories, each `dir` the runner's clone and each `checkout` the runner's view, a host writing none); core.7 (the table is JSON, which jsonfmt owns). No rule is departed from.

## Risks / Trade-offs

- **A view the runner stops keeping.** core.43 has the hosts turn on each drive's VS Code workspace and retire their views. While the runner keeps a view at each address, crew's `checkout` names it. A change that stops keeping views changes the `checkout` with it.
- **A repository only the local layer names.** On a host whose hosted control plane adds a repository in the runner's local layer, crew's map holds no entry for it, as `cloneDirs` holds none (record 0036's second amendment).
- **A host's `lib.mkForce`.** A host that forces another folder evaluates (Decision 4).
- **The rule written twice.** The runner's view rule is in Rust and the set's in Nix. The table of Decision 5 fails both tests' checks where either moves without the other.

## Migration Plan

None in this repository. A host that writes its own `services.crew.settings.repositories` for a kept address drops the entry once it takes a set that carries this change. Where its entry names another folder, the folder of before the rename among them (record 0035), its evaluation fails until it does.

## Task notes

- **1.1** `nix/tests/kept.nix` is `t: { ... }`, cases named as the spec's scenarios, each a list of what fails by `t.expect`, as `clone-dirs.nix` is, and `nix/tests/default.nix`'s `test` takes `// import ./kept.nix t`. The rule's case imports `../modules/view-dir.nix` with `{ inherit lib; }` and reads `./view-dirs.json` with `builtins.fromJSON (builtins.readFile ./view-dirs.json)`. A row's rule is read with `builtins.tryEval`: a row with a `folder` holds where the read succeeds with that text, and a row with none where it fails. A failing row is named by its `home`, `repository_root` and `address`, and what the rule gave.
- **1.2** The row's text is `accounts = {}\n`, then `repository_root = <root>\n` where the row's root is not `null`, the root written as `toml::Value::String(root.to_owned())` displays it. `config::parse(&text, Path::new("view-dirs.toml"), Path::new(home))` reads it, and `config::Address::parse` the row's address. The view is compared as `view_path(&config, &address).to_str()` against `Some(folder)`.
- **2.1** crew is turned on as `nix/tests/tools.nix`'s `crewOn` is: `services.crew = { enable = true; host = "host-a"; settings = settings.crewModule; }`, `settings` being `import ./settings.nix`. The drive `home` and the drive `garden` are `clone-dirs.nix`'s. crew's map is read with `builtins.tryEval (builtins.deepSeq evaluated.config.services.crew.settings.repositories true)`, a failing read told apart from a `null` value. On nix-darwin, `t.darwinSystem` of crew and the drive `home` gives, compared whole, `agentplot/flume` with the `dir` `/Users/alice/.local/share/boomdrive/repositories/agentplot/flume` and the `checkout` `/Users/alice/Code/agentplot/flume`, and `agentplot/sluice` likewise. On NixOS `t.nixosSystem` gives the same under `/home/alice`. With the drive `garden`, `example/alpha`'s `dir` is the system's own `cloneDirs."example/alpha"` and its `checkout` `<home>/Code/example/alpha`. With `services.boomdrive.settings.repository_root` `~/src`, `agentplot/flume`'s `checkout` is `<home>/src/agentplot/flume`, and with `/srv/code`, `/srv/code/agentplot/flume`, on each platform.
- **2.2** Each of these gives crew's map `null`, on each platform: the drive `home` with crew's module off; crew's module and the drive `home` with the runner not enabled, `t.darwinOf [ t.outputs.darwinModules.default <crew> <home> ]` on nix-darwin and `t.nixosOf [ t.outputs.nixosModules.default <crew> <home> ]` with `services.boomdrive.user = "alice"` on NixOS; and crew's module with no repository kept.
- **2.3** A host's `services.crew.settings.repositories."agentplot/flume".dir = "/srv/flume";` beside crew's module and the drive `home` fails to read, and again with `.checkout` in place of `.dir`, on each platform. A host's `services.crew.settings.repositories."example/own" = { dir = "/srv/own"; checkout = "/srv/own/main"; };` beside them gives crew's map `example/own` as the host gave it, beside `agentplot/flume` and `agentplot/sluice` as the module gives them.
