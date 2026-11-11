# Design

## Context

The runner keeps its clones, its lock and its report in one data folder (core.35). `crates/boomdrive/src/config.rs` gives it:

```rust
/// The runner's data directory, `$XDG_DATA_HOME/boomdrive`, else
/// `~/.local/share/boomdrive` (design 3), and no other, whatever else is
/// beside it: nothing on disk is read to choose it (core.35, core.36;
/// record 0035).
pub fn data_dir(env: &Env) -> PathBuf {
    xdg(env.data_home.as_deref(), &env.home, ".local/share").join("boomdrive")
}
```

`xdg` takes `XDG_DATA_HOME` only where it is absolute, so an empty or relative one is taken as unset, as the XDG base directory specification says. `Env::read` reads `HOME` and `XDG_DATA_HOME` from the runner's own environment, which its job gives it.

The flake's modules run the runner as the user's service:

- On nix-darwin, `launchd.user.agents.boomdrive`, whose `serviceConfig.EnvironmentVariables` the module gives as `HOME`, the primary user's home, and `PATH`. The module declares `services.boomdrive.logFile` beside it, defaulting under that home. A host reads the job's label and program from nix-darwin's own option (record 0034, point 6), and its log from `logFile`.
- On NixOS, `systemd.user.services.boomdrive`, whose `environment` the module gives as `PATH`. The user's manager gives `HOME`, the home of `services.boomdrive.user`.

Neither module gives `XDG_DATA_HOME`, so on a host that adds none the folder is `~/.local/share/boomdrive`. No option of the set holds the folder, so a host that names it, or a clone in it, computes it in its own Nix. A host's example configuration of 2026-11-11 does so three times: in its doctor's manifest, in its sluice drivers' jobs, and in the repositories of crew's teams file. It also keeps the folder's name from before the rename, `flywheel`, for a host whose data has not moved (record 0035).

Record 0036, point 1, has the Boomdrive module give crew each kept repository's `dir`, `<data directory>/repositories/<owner>/<name>`. The change `the-set-gives-crew-each-kept-repositorys-folders` builds it, after this one.

## Goals / Non-Goals

**Goals:**

- The set gives the runner's data folder, by the runner's own rule, as an option every reader of it takes.
- No host sets the folder or computes it beside the set.
- The module's rule cannot drift from the runner's with no check failing.

**Non-Goals:**

- A clone's folder or a view's. A reader joins the runner's layout under the folder itself. The change `the-set-gives-crew-each-kept-repositorys-folders` does it for crew.
- Any change to the runner's code, its file's keys or where it keeps its data.
- Any change to the job's environment, or an `XDG_DATA_HOME` given by the module.
- The folder of before the rename. A host whose data is still there moves it by hand before it takes a set with this option (record 0035).

## Decisions

### 1. Each module declares `services.boomdrive.dataDir`, read-only

`darwin.nix` and `nixos.nix` each declare the option beside the job it is derived from, as `darwin.nix` declares `logFile`:

```nix
options.services.boomdrive.dataDir = lib.mkOption {
  type = lib.types.path;
  readOnly = true;
  default = dataDirOf { inherit home; dataHome = <the job's XDG_DATA_HOME, or null>; };
  defaultText = lib.literalMD "...";
  description = "...";
};
```

Its description says it is the runner's data folder by the runner's own rule; that it is read-only, read wherever a host names the folder or a clone in it; and that an `XDG_DATA_HOME` the job takes from outside its declaration is not seen (Decision 3). Its `defaultText` says the rule in words: `$XDG_DATA_HOME/boomdrive` where the runner's job gives an absolute `XDG_DATA_HOME`, else `~/.local/share/boomdrive` under the home of the primary user on nix-darwin, or of `services.boomdrive.user` on NixOS.

The option is declared whether or not the runner is enabled, and its value is lazy, as `logFile`'s is. Reading it on a nix-darwin host whose primary user has no home fails, as reading `logFile` does. With the runner enabled, the module's own assertion names the missing home.

A host that sets the option, at any priority, fails evaluation. nixpkgs' module system refuses a read-only option with two definitions, and the declaration's default is one: "The option `services.boomdrive.dataDir' is read-only, but it's set multiple times."

The name is `dataDir`. core.12 has a path's name end in `_dir`, and the modules name options in nix's camel case, as `logFile`.

Not taken:

- **An option a host may set.** The runner takes no other folder, so a value set would name a folder the runner never keeps, and every reader would follow it there.
- **A setting of the runner's for its folder,** a key or a flag the module would pass. Nothing needs one: the job's `XDG_DATA_HOME` already moves the folder. It would also be a new setting of the runner's, and core.36's move for every host that keeps one.
- **One declaration in `options.nix`, defined by each platform module.** The value needs the platform's home and job. Each module declares it with its default, as `logFile` is declared, so the option has one definition on each platform.
- **The folder of before the rename,** chosen by what is on disk. The runner reads no folder by its old name (core.36; record 0035), and neither does the set.

### 2. One file holds the rule: `nix/modules/data-dir.nix`

```nix
{ lib }:
{ home, dataHome }:
# The runner's data folder (crates/boomdrive/src/config.rs, data_dir)...
```

It gives the runner's folder from `home` and `dataHome`, the job's `XDG_DATA_HOME` or `null`:

- `dataHome` is taken where it is not `null` and its string starts with `/`. A path or a package is taken as its string. An empty or relative `dataHome` is taken as unset, as `xdg` takes one.
- Otherwise the base is `home` joined with `.local/share`.
- The folder is the base joined with `boomdrive`.
- Each join is the runner's: `/` then the name, the `/` added only where the base does not already end in one, as Rust's `PathBuf::push` adds it. So `/x/data/` gives `/x/data/boomdrive`.

`darwin.nix` and `nixos.nix` import it as `dataDirOf`. No other file says the rule.

### 3. The job's `XDG_DATA_HOME` is the one its declaration gives

- On nix-darwin, `config.launchd.user.agents.boomdrive.serviceConfig.EnvironmentVariables.XDG_DATA_HOME or null`, with `home` the primary user's, the job's `HOME`.
- On NixOS, `config.systemd.user.services.boomdrive.environment.XDG_DATA_HOME or null`, with `home` `config.users.users.<services.boomdrive.user>.home`, which the user's manager gives the unit as `HOME`. A value of `null` there is NixOS's unset.

Nix's `or` gives `null` both where the attribute is missing and where a value on the way is not a set. That covers a runner not enabled, with no job, and nix-darwin's `EnvironmentVariables` of `null`. A host gives the job its own `XDG_DATA_HOME` through those same options, which merge with the module's `HOME` and `PATH`, and the option follows it. Nothing of the job reads `dataDir`, so there is no cycle.

The service-modules requirement for the nix-darwin agent says the module gives `HOME` and `PATH` alone, beside any variable the host gives the job, so a host's `XDG_DATA_HOME` meets the spec.

Not taken:

- **An environment the job inherits from outside its declaration,** such as launchd's `launchctl setenv`, or a user manager's `environment.d`, PAM or `systemctl --user set-environment`. Evaluation cannot read it. A host that keeps the runner's data elsewhere names `XDG_DATA_HOME` in the job, and core.57's details say so.
- **The module giving the job an `XDG_DATA_HOME` of its own,** so the two could never differ. Every process the runner starts inherits it, Flume and its agents among them, so their own data would move too. On a host whose user manager gives another `XDG_DATA_HOME`, the runner's report would move with it (core.36).

### 4. One table holds the module's rule to the runner's

`nix/tests/data-dirs.json` is a list of cases, each with the neutral home of `config.rs`'s own tests (core.41):

| `HOME` | `XDG_DATA_HOME` | `folder` |
|---|---|---|
| `/h` | `null` | `/h/.local/share/boomdrive` |
| `/h` | `/x/data` | `/x/data/boomdrive` |
| `/h` | `/x/data/` | `/x/data/boomdrive` |
| `/h` | `""` | `/h/.local/share/boomdrive` |
| `/h` | `data` | `/h/.local/share/boomdrive` |

- `nix/tests/data-dir.nix`'s case "The runner's own rule" holds `data-dir.nix` to each row, `XDG_DATA_HOME` given as `dataHome`.
- `crates/boomdrive/tests/module_files.rs`'s `the_modules_data_folder_is_the_runners` reads the same file with `serde_json`. For each row it builds an `Env` by `Env::read` from a lookup that gives `HOME`, and `XDG_DATA_HOME` where the row's is not `null`, and asserts `config::data_dir` is the row's folder. Its message names the row. The file's module documentation adds that it holds the data folder the modules name to the runner's.
- `crates/boomdrive/moon.yml`'s `test` task takes `/nix/tests/data-dirs.json` as an input, beside `/nix/tests/files/**/*`.

The table sits under `nix/tests`, not `nix/tests/files`, which holds the files the modules render byte for byte and which `each_rendered_file_is_one_read_here` lists. jsonfmt formats it as the tree's own JSON (`treefmt.nix`), and both tests read it as values.

Not taken: literal folders in each test alone, which could drift from each other with nothing failing.

### 5. The README names the option

**Installing with nix**, after the paragraph on the agent and the unit, says the runner keeps its clones and its report in its data folder, `services.boomdrive.dataDir`. That is `~/.local/share/boomdrive` under the runner's user's home, or `$XDG_DATA_HOME/boomdrive` where the job is given an absolute `XDG_DATA_HOME`: the agent's `EnvironmentVariables` on nix-darwin, the unit's `environment` on NixOS. It says the option is read-only, read wherever a host names the folder or a clone in it, and set nowhere. And it says an `XDG_DATA_HOME` the job takes from elsewhere is not seen, so a host gives the job its own.

`nix/tests/data-dir.nix`'s case "The README's data folder" holds **Installing with nix** to naming `services.boomdrive.dataDir` and saying `read-only`.

### 6. The record and the rule

Record 0036 is amended on 2026-11-11 with the runner's data folder, and core.57's text and details with it, in this change's planning commit. The amendment answers record 0034's point 6, which did not take read-only options for the job's label and program, since nix-darwin's own option holds them. No option holds the data folder: the runner derives it as it runs. So `dataDir` is the folder's one home in the set's evaluation, as `logFile` is the log's.

## Risks / Trade-offs

- **An `XDG_DATA_HOME` from outside the job.** A host whose user manager, or launchd's user domain, gives the runner an `XDG_DATA_HOME` the job's declaration does not has a `dataDir` that differs from the runner's folder, with no check failing. The option's description, the README and core.57 say to give it in the job. Neither module gives one, and no host of the set's sets one today.
- **The rule written twice.** The runner's rule is in Rust and the set's in Nix. The table of Decision 4 fails both tests' checks where either moves without the other.
- **The option is read before the runner is enabled.** It is lazy, so a configuration that never reads it pays nothing. One that reads it with no home fails, as one reading `logFile` does.

## Migration Plan

None in this repository. A host takes the option once it takes a set that gives it, and drops its own computation of the folder. A host whose data is still in the folder of before the rename moves it by hand first (record 0035).

## Task notes

- **1.1** `nix/tests/data-dir.nix` is `t: { ... }`, cases named as the spec's scenarios, each a list of what fails by `t.expect`, as `drives.nix` is. `nix/tests/default.nix`'s `test` takes `// import ./data-dir.nix t`. The rule's case imports `../modules/data-dir.nix { inherit (t) lib; }` and reads `./data-dirs.json` with `builtins.fromJSON (builtins.readFile ./data-dirs.json)`. It names a failing row by its `HOME` and `XDG_DATA_HOME` and what the rule gave.
- **2.1** On nix-darwin, `t.darwinSystem { }` gives `/Users/alice/.local/share/boomdrive`. A host module `{ launchd.user.agents.boomdrive.serviceConfig.EnvironmentVariables.XDG_DATA_HOME = "/srv/data"; }` gives `/srv/data/boomdrive`, and `"data"` and `""` each give `/Users/alice/.local/share/boomdrive`. The agent's `EnvironmentVariables` still hold `HOME` and `PATH` as "The agent" has them. A host's `services.boomdrive.dataDir = "/srv/boomdrive";` fails: `!(builtins.tryEval (builtins.seq evaluated.config.services.boomdrive.dataDir true)).success`, as `file.nix` reads a failure. A system with the runner not enabled, `t.darwinOf [ t.outputs.darwinModules.default ]`, still reads `/Users/alice/.local/share/boomdrive`.
- **2.2** On NixOS, `t.nixosSystem { }` gives `/home/alice/.local/share/boomdrive`, alice's home by NixOS's default for a normal user. A host module `{ systemd.user.services.boomdrive.environment.XDG_DATA_HOME = "/srv/data"; }` gives `/srv/data/boomdrive`, and `"data"` and `""` each give `/home/alice/.local/share/boomdrive`. Setting `services.boomdrive.dataDir` fails as on nix-darwin. A system of `t.nixosOf` with the flake's module enabled for `bob`, `services.boomdrive = { enable = true; user = "bob"; }`, gives `/home/bob/.local/share/boomdrive`.
