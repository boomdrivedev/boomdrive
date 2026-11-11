# Design

## Context

The runner keeps one clone of each repository its configuration names, in its data folder (core.5, core.14, core.35). `crates/boomdrive/src/clone.rs` gives the clone's folder:

```rust
/// The container of `address`'s clone, `<data>/repositories/<owner>/<name>`:
/// the directory `flume launch --repository` names.
pub fn container(data: &Path, address: &Address) -> PathBuf {
    repositories(data)
        .join(address.owner())
        .join(address.name())
}
```

The owner and the name are as the configuration writes them. The folder holds the bare repository, `<name>.git`, and a `.git` file naming it (`gitdir: ./<name>.git`), so git run in the folder finds the repository.

The flake's modules name the data folder as `services.boomdrive.dataDir`, read-only, by the runner's own rule (record 0036 as amended on 2026-11-11; `nix/modules/data-dir.nix`). They write the runner's `accounts` from `services.boomdrive.accounts`: every account's drives, and every drive's repositories by owner (record 0028; `nix/modules/accounts.nix`). A drive of another machine still has its repositories kept, to read.

No option holds a clone's folder. A reader joins `repositories/<owner>/<name>` under `dataDir` itself. afterthought/swancloud joins it in its doctor's manifest, its sluice drivers' jobs and crew's teams file. The change `the-set-gives-crew-each-kept-repositorys-folders` would join it again for crew's `dir` (record 0036, point 1). afterthought/swancloud's record 0059, decision 4, has the fleet read the runner's layout from the flake and never compute it, and its amendment of 2026-11-11 waits on this change.

## Goals / Non-Goals

**Goals:**

- The set states each kept repository's clone folder, by the runner's own rule, as an option every reader of it takes (principle 14; core.57).
- No host sets a clone's folder or joins one beside the set.
- The module's rule cannot drift from the runner's with no check failing.

**Non-Goals:**

- A view's folder. A view is under the host's own `repository_root`. The change `the-set-gives-crew-each-kept-repositorys-folders` gives each view to crew as its `checkout` (record 0036, point 1).
- crew's kept repositories. That change reads `cloneDirs` for each entry's `dir`.
- Any change to the runner's code, its file's keys, its data folder or its layout.
- A repository only the runner's local layer names (Decision 3).

## Decisions

### 1. A shared module declares `services.boomdrive.cloneDirs`, read-only

`nix/modules/clones.nix` is a module that `darwin.nix` and `nixos.nix` each import beside `./drives.nix`. It declares:

```nix
options.services.boomdrive.cloneDirs = lib.mkOption {
  type = lib.types.attrsOf lib.types.path;
  readOnly = true;
  default = lib.listToAttrs (map (address: lib.nameValuePair address (cloneDirOf {
    inherit (cfg) dataDir;
    inherit address;
  })) addresses);
  defaultText = lib.literalMD "...";
  description = "...";
};
```

`addresses` is every drive's addresses, `drivesOf cfg.accounts` from `accounts.nix`, concatenated. Its `defaultText` says each address of `services.boomdrive.accounts`, every drive's of every account, with its folder `<services.boomdrive.dataDir>/repositories/<owner>/<name>`. Its `description` says what the value is: the runner's clone of each repository the set names, the folder `flume launch --repository` names, by the runner's own rule. It says the option is read-only, read wherever a host names a clone's folder and set nowhere. And it says git run in the folder finds the bare repository by the folder's own `.git` file (Decision 4).

The option is declared whether or not the runner is enabled, and its value is lazy, as `dataDir`'s is. Its names are read from `services.boomdrive.accounts` alone, so reading the set's addresses forces no folder. Each value reads `dataDir`, so it fails where `dataDir` fails, on a nix-darwin host whose primary user has no home.

A host that sets the option, or one entry of it, at any priority, fails evaluation. nixpkgs refuses a read-only option with two definitions, and the declaration's default is one.

The name is `cloneDirs`. core.12 has a path's name end in `_dir`, and the modules name options in nix's camel case, as `dataDir` and `logFile`. It is plural since it holds one folder for each address.

Not taken:

- **Each reader joining the folder under `dataDir`.** That writes the runner's layout again in each reader, crew's entry among them, and none fails when the runner's moves.
- **A declaration in each platform module,** as `dataDir`'s. The value reads only `dataDir` and the accounts, which both platforms give alike, so one declaration serves both.
- **A declaration in `options.nix`.** `nix/tests/file.nix` evaluates `options.nix` alone, with no `dataDir`. A declaration there would have a default that cannot evaluate in that system.
- **The name `clones`,** which names no path (core.12), or `repositories`, which the runner's file and crew's teams file already use for something else.
- **A submodule for each address,** such as `{ dir; }`. It adds a level that holds one value.

### 2. One file holds the rule: `nix/modules/clone-dir.nix`

```nix
{ dataDir, address }: "${toString dataDir}/repositories/${address}"
```

It gives the runner's clone folder from `dataDir` and an address `<owner>/<name>`, each as written. The address's one `/` sits between its owner and its name. So the folder is `repositories`, then the owner, then the name, under `dataDir`, as `container` joins them. `dataDir` is `data-dir.nix`'s, which ends in `boomdrive` and never in `/`. So one `/` before `repositories` is the runner's join.

`clones.nix` imports it as `cloneDirOf`. No other file says the rule. Its header comment names `crates/boomdrive/src/clone.rs`'s `container` as the rule it follows, and `nix/tests/clone-dirs.json` as the table that holds them together.

### 3. The addresses are the set's, every drive's of every account

`cloneDirs` holds each address `services.boomdrive.accounts` names, under every account and every drive. That includes a drive whose `machine` is another host's, since the runner keeps its repositories to read (record 0028; core.5). Each address is a key once. An address two drives name fails evaluation with the runner enabled (`options.nix`, record 0028). Its folder depends on the address alone, so either drive gives the same folder.

It holds none the runner's local layer alone names. A hosted control plane writes that layer on the host, adding to the managed layer's repositories (core.17; record 0009 as amended 2026-11-06). The set does not read it at evaluation.

Not taken: reading the local layer at evaluation. It is no file of the set's, and a pure evaluation cannot read the host's own files.

### 4. The folder, not the bare repository in it

Each value is the clone's folder: the one `flume launch --repository` names, and crew's `dir` (record 0036, point 1). A reader that runs git, such as a doctor's check of the clone, runs it in the folder. Git reads the folder's `.git` file and finds `<name>.git`.

Not taken: a second value for the bare repository, `<folder>/<name>.git`. It states Flume's bare-container layout a second time, and git already finds the bare repository from the folder.

### 5. One table holds the module's rule to the runner's

`nix/tests/clone-dirs.json` is a list of cases, each with a neutral data folder, as `clone.rs`'s and `config.rs`'s own tests use (core.41):

| `dataDir` | `address` | `folder` |
|---|---|---|
| `/d/boomdrive` | `agentplot/flume` | `/d/boomdrive/repositories/agentplot/flume` |
| `/x/data/boomdrive` | `example/alpha` | `/x/data/boomdrive/repositories/example/alpha` |
| `/d/boomdrive` | `Example-1/a.b_c` | `/d/boomdrive/repositories/Example-1/a.b_c` |

The third row has an owner with a capital and a hyphen, and a name with a `.` and a `_`, each kept as written.

- `nix/tests/clone-dirs.nix`'s case "The runner's own clone rule" holds `clone-dir.nix` to each row.
- `crates/boomdrive/tests/module_files.rs`'s `the_modules_clone_folders_are_the_runners` reads the same file with `serde_json`. For each row it parses the address by `config::Address::parse` and asserts `clone::container` of the row's `dataDir` and that address is the row's folder. Its message names the row. The file's module documentation adds that it holds the clone folders the modules name to the runner's.
- `crates/boomdrive/moon.yml`'s `test` task takes `/nix/tests/clone-dirs.json` as an input, beside `/nix/tests/data-dirs.json`.

The table sits beside `data-dirs.json` under `nix/tests`. jsonfmt formats it as the tree's own JSON (`treefmt.nix`), and both tests read it as values.

Not taken: one table for both rules. The data folder's rows and the clone folder's rows hold different values, and each test reads only its own.

### 6. The README names the option, and `dataDir` sends a clone's reader to it

In **Installing with nix**, `dataDir`'s paragraph says the option is read wherever the host names the folder, and that a clone's folder is `cloneDirs`'s. A paragraph after it says the module states each kept repository's clone folder as `services.boomdrive.cloneDirs`. That is one entry for each address of the accounts, every drive's of every account, keyed by the address, `<dataDir>/repositories/<owner>/<name>`. The paragraph says the option is read-only: a host reads it wherever it names a clone's folder, and sets it nowhere. It says the option holds only the repositories `services.boomdrive.accounts` names. It gives a reader's line as an example, in a nix block:

```nix
config.services.boomdrive.cloneDirs."agentplot/flume"  # ~/.local/share/boomdrive/repositories/agentplot/flume
```

`dataDir`'s description in `darwin.nix` and `nixos.nix` says the same: read wherever a host names the folder, a clone's folder being `cloneDirs`'s. Each header comment names `cloneDirs` beside `dataDir`.

`nix/tests/clone-dirs.nix`'s case "The README's clone folders" holds **Installing with nix** to naming `services.boomdrive.cloneDirs`. "The README's data folder" still holds.

### 7. The record and the rule

Record 0036 is amended a second time on 2026-11-11, with each kept repository's clone folder, and core.57's text and details with it, in this change's planning commit. core.57 then has whatever names the data folder read `dataDir`, and whatever names a clone's folder read `cloneDirs`. Its details rule out a clone's folder joined under `dataDir` by a host or a module of the set.

## Risks / Trade-offs

- **A repository only the local layer names.** On a host whose hosted control plane adds a repository in the local layer, the runner keeps a clone that `cloneDirs` does not name. No host of the set's has one today, and the README and the option's description say which repositories the option holds.
- **The rule written twice.** The runner's rule is in Rust and the set's in Nix. The table of Decision 5 fails both tests' checks where either moves without the other.
- **The option is read before the runner is enabled.** It is lazy, so a configuration that never reads it pays nothing. One that reads a value with no home fails, as one reading `dataDir` does.

## Migration Plan

None in this repository. A host takes the option once it takes a set that carries it, and drops its own join of a clone's folder.

## Task notes

- **1.1** `nix/tests/clone-dirs.nix` is `t: { ... }`, cases named as the spec's scenarios, each a list of what fails by `t.expect`, as `data-dir.nix` is. `nix/tests/default.nix`'s `test` takes `// import ./clone-dirs.nix t`. The rule's case imports `../modules/clone-dir.nix` and reads `./clone-dirs.json` with `builtins.fromJSON (builtins.readFile ./clone-dirs.json)`. It names a failing row by its `dataDir` and `address` and what the rule gave.
- **2.1** A system's option value is read as `data-dir.nix` reads `dataDir`: `builtins.tryEval (builtins.deepSeq evaluated.config.services.boomdrive.cloneDirs true)`, null where it fails. The drive `home` is `{ services.boomdrive.accounts.own.drives.home.repositories.agentplot = [ "flume" "sluice" ]; }`. On nix-darwin, `t.darwinSystem` of it gives both folders under `/Users/alice/.local/share/boomdrive/repositories`, and on NixOS `t.nixosSystem` under `/home/alice/.local/share/boomdrive/repositories`, each compared whole. The drive `garden` of `drives.nix`, on `host-b` with `services.crew.host = "host-a"`, gives `example/alpha` its folder under the system's own `dataDir`. A job given `XDG_DATA_HOME` `/srv/data` as `data-dir.nix`'s cases give it, with the drive `home`, gives `agentplot/flume` at `/srv/data/boomdrive/repositories/agentplot/flume`. `t.darwinSystem { }` and `t.nixosSystem { }` give `{ }`. With the runner not enabled, `t.darwinOf [ t.outputs.darwinModules.default <the drive home> ]` gives `agentplot/flume` under `/Users/alice`. A host's `services.boomdrive.cloneDirs."agentplot/flume" = "/srv/flume";` beside the drive `home` fails to evaluate, on each platform.
