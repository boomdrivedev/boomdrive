# Design

## Context

The runner reads one TOML file, `--config <file>` or `$XDG_CONFIG_HOME/boomdrive/config.toml` (the configuration spec). It reads it again at every pass for its repositories, and at the start for `[control_plane]`, whose implementation's table it hands whole to the implementation (core.2).

Record 0009's amendment of 2026-11-06, accepted, gives the runner Flume's two layers (flume record 0032; design/principles.md 10). Its points:

- The managed layer, `config.toml`, is written by Nix, a fleet or a person.
- The local layer, `config.local.toml` beside it, is written by the operator or a control plane, and read over it.
- The local layer sets only the keys the tool lists, "for the runner, its repositories and its control plane's partitions", and a local file that sets another key is not taken.
- The managed layer may pin any key the local one could set.
- "Repositories add": the runner keeps every repository either layer names, so a control plane that drops a drive drops only the repositories it wrote.

Flume builds the same rule (`crates/flume/src/host/config.rs`, `HostConfig::layers_in`). A local layer is read key by key over the managed one, each listed key a unit taken whole. A local file that sets any other key, is not TOML, or holds a value the reader refuses, is not taken at all, and the managed layer runs alone. A key `[host] pin` holds is passed over and named.

On a Nix host the service runs `boomdrive run --config <store path>` (the service-modules spec). The activation links `~/.config/boomdrive/config.toml` to that store path, so commands run by hand read what the service reads.

The rules this design relies on:

- **core.17**: the local layer sets only the keys the runner lists for it, its repositories add to the managed layer's, and the managed layer may pin.
- **core.18**: a control plane writes a tool's local layer, never its managed one; no module writes another tool's file.
- **core.2**: no code outside an implementation names its table's keys; `planes.rs` is the one list naming the implementations.
- **core.4** and **core.11**: no refusal quotes a file's text or a value.
- **core.5**: the runner keeps a clone or a view only of a repository its configuration names.
- **core.23**: what the runner says of its reach is said once each way, the model for the local layer's line.
- **core.12**: keys and types name what they hold.
- **core.36**: the configuration a person or Nix writes is no file the runner keeps, and a host with no local file reads as before.
- **core.40**: every line starts with its time.
- **core.41**: neutral names in tests.
- **core.46**: the modules write the runner's file from `settings`, freeform at its top.

This design departs from no rule. Two rules are read as the configuration spec already reads them, and core.17's details say so (Decision 11):

- **core.5**: a repository only a local layer not taken names is neither fetched nor let go. A layer the runner cannot take still names what it named, as a file that cannot be read does.
- **core.43**: "set at every pass" holds for a pass that read its whole configuration. A pass whose file cannot be read sets no workspace today, and one whose local layer is not taken sets none either.

## Goals / Non-Goals

**Goals**

- The runner reads its local layer over its managed one, by Flume's rule and words, so a hosted control plane can add the repositories and partitions of the drives a host serves without touching Nix's file.
- A control plane's bad write never lets go of a repository and never stops the managed layer's.

**Non-Goals**

- **Writing the local layer.** No command of the runner writes either file. The hosted control plane's setup, which writes it, is the cloud's, and `boomdrive setup` is not built.
- **A `status` or `check` command** giving each value with its layer, as Flume's status does. The runner has none yet. Its lines say what it took.
- **Accounts and drives.** The sibling change `a-hosts-configuration-names-its-accounts-and-their-drives` moves the repositories under `accounts`. See Risks.
- **Reading `[control_plane]` at every pass.** It stays read at the start, both layers with it (the configuration spec).

## Decisions

### 1. The local layer is `config.local.toml` in the runner's own configuration directory

The local layer is `$XDG_CONFIG_HOME/boomdrive/config.local.toml`, else `~/.config/boomdrive/config.local.toml`. That is always the directory of the default managed file, whatever `--config` names. `config::local_path(&Env)` gives it, beside `config::config_path`. A file that does not exist there is no local layer.

Not taken:

- **Beside the file `--config` names**, as Flume's `local_config_file` puts it beside its one file. A Nix host's service names a store path, and nothing can be written beside it. The control plane would have no place to write.
- **A `--local-config` option.** The service and every command run by hand would have to pass it alike. The directory the activation links `config.toml` in is already the one both read.

### 2. The local layer sets `repositories` and each implementation's listed keys

The keys the local layer may set are `repositories`, and `control_plane.<word>.<key>` for each `<key>` the implementation `<word>` lists. Each implementation lists `partitions`:

- `boomdrive_crew::config::LOCAL_KEYS`;
- `boomdrive_hosted::table::LOCAL_KEYS`.

`planes::Entry` gains two fields:

- `local_keys`, the implementation's list;
- `read`, its own reading of its table: crew's `config::read` and the hosted `Table::read`. Each opens nothing and asks nothing of the host.

`config.rs` takes both from `planes::PLANES` and names no implementation's key (core.2).

Each listed key is one unit, its value taken whole as Flume takes each unit of its `LOCAL_KEYS`.

Not taken:

- **`partitions` for the hosted control plane alone.** The record says "its control plane's partitions". A host under crew may be configured by hand (core.17), and its operator's local layer may name a partition as a hosted control plane's would.
- **The runner listing the keys itself.** That would name the implementations' keys outside them (core.2).

### 3. Repositories add, and partitions replace

The configuration's repositories are:

1. the managed layer's, in their order;
2. then each of the local layer's that the managed layer does not name, in its order.

A repository both name is kept once, with the managed layer's spelling. Neither layer is refused for it, since a control plane may add a repository Nix already keeps. Within one file, two entries that are one repository are refused as today. A local file holding two is not taken (Decision 5).

A local layer's `control_plane.<word>.partitions` replaces the managed layer's whole, as every unit of Flume's does. Only repositories add, in the record's words.

### 4. The reading, step by step

`config::read_layers` takes the managed file, the local file and the home, and `config::parse_layers` their texts. `config::read` and `config::parse` stay the reading of one file, the managed layer alone, which the modules' file tests read with. The layered reading goes in this order:

1. **The managed layer** is read as one file is read today, with `pin` among its keys (Decision 6). Any fault refuses the configuration, naming the managed file, as today.
2. **No local file**: the configuration is the managed layer's, its local state `NoFile`.
3. **Not TOML**: the local file is not taken, with the fault.
4. **An unlisted key** at any depth: the local file is not taken, naming each such key by its dotted path, the shortest that holds no key the local layer may set. So `repository_root`, `token`, `control_plane.type`, `control_plane.hosted.url` and `streams`. The value is never named.
5. **A pinned key** the local layer sets is passed over, named with its pin. Every other unit goes into the managed layer's table: `repositories` added as Decision 3 has it, a unit under `control_plane` set in place.
6. **The merged table** is read by the same reader as the managed layer. For each implementation table the local layer set a key in, it is also read by that implementation's `read`. A fault of either leaves the local file not taken, with the fault named against the local file. Otherwise the merged reading is the configuration, its local state `Taken` with the keys passed over.

`Config` gains `local: LocalLayer`. Its three states are named as `flume_seams::LocalLayer` names them, with the runner's own types:

- `LocalLayer::NoFile`;
- `LocalLayer::Taken { passed_over: Vec<PinnedKey> }`;
- `LocalLayer::NotTaken { keys: Vec<String>, fault: Option<String> }`.

`PinnedKey { key, pin }` names a key passed over and the pin holding it.

A fault in the merged reading that the managed layer alone does not have is the local layer's, by step 1. A fault the managed layer has refuses the whole configuration before the local file is read.

### 5. A local layer not taken lets go of nothing and sets no workspace

Where `config.local` is `NotTaken`, the configuration is the managed layer alone, as Flume's is. The keeping pass:

- keeps the managed layer's repositories: clones, fetches and moves their views;
- lets go of no repository, `pass::pass` skipping its let-go loop;
- sets no workspace, `wanted_workspaces` giving none.

The local layer it could not take may name any clone the pass would let go of, and any folder a workspace would lose. That holds whether the file is half written by a control plane, mistyped by a person, or set with a key it may not. The configuration spec already rules that a file that cannot be read never counts as one that names no repository (crew record 0052 part 2). This applies that rule to the layer.

A control pass and a landing read the managed layer alone meanwhile. A launch in a repository only the local layer named is refused as one the host does not keep (core.9), and the hello names the managed layer's repositories (core.26). Neither removes anything.

A local file that does not exist is no local layer. A repository only it named is let go as any dropped repository is, with the let-go rules as they stand. Removing the file is how a person or a control plane takes back all it wrote.

Not taken:

- **The managed layer alone, letting go as it says**, which Flume's rule would give read literally. For Flume a local fault loses an hour's tuning. For the runner it would remove the clones and views of every drive a control plane added, at one bad write.
- **Refusing the whole configuration**, as a broken managed file is. That would stop the managed layer's repositories too, against Flume's reason for the rule: "A control plane's bad write never stops the host's work".

### 6. `pin` is a key of the managed layer's top

The managed layer's `pin` is a list of strings. Each is a dotted key in TOML's spelling:

- a key the local layer may set: `repositories`, or `control_plane.<word>.partitions` for a word `planes::PLANES` carries;
- or a table holding one: `control_plane.<word>` or `control_plane`, which pins each such key under it.

Each entry is checked as the managed layer is read. A `pin` that is no list of strings, or an entry naming anything else, such as `repository_root`, `control_plane.type` or a word not carried, is the managed layer's fault, naming the entry.

The local layer may not set `pin` (Decision 4, step 4).

The runner has no `[host]` table, so `pin` stands at the top of the file. Flume's word, and its spelling of a key, are kept, as principles.md 10 asks of every tool. Not taken: a `[local]` table or `pinned_keys`, a second word for one thing across the family (core.12).

### 7. The local layer's line is said once each way

`Runner` keeps the local layer's state as last said. A keeping pass compares the state it read with it, and says one line where they differ. The first pass compares with `NoFile`, so a host with no local file says nothing. Each line names the local file:

- `boomdrive: <local>: taken`, with `; <key> passed over, pinned by <pin>` for each key passed over;
- `boomdrive: <local>: not taken, the managed layer read alone and nothing let go: <why>`, where `<why>` names each key it may not set, or the fault;
- `boomdrive: <local>: gone, the managed layer read alone`, where a file that was there is not.

`run --once`'s one pass says it as a running runner's first pass does. A control pass and a landing say nothing of it, since the keeping pass does.

The start line keeps its form, its count both layers' repositories.

Not taken: a line at every pass, as a broken managed file's. A pinned key passed over is a steady state a host may keep for good, and the managed layer still runs while the local one is not taken. core.23 says the runner's reach once each way for the same reason.

### 8. Every reading reads both layers

These read both layers, through `config::read_layers` with the local path:

- `Runner::start`'s reading, and with it `[control_plane]`'s table, taken at the start as today;
- every keeping pass;
- every control pass;
- every landing's `keep_one`;
- `boomdrive enroll`.

So a partition the local layer names is enrolled as one the managed layer names. A change to the local layer's `partitions` takes effect when the runner starts again, as one to the managed layer's does.

### 9. The modules leave the local layer

The modules write the managed layer and never the local one (core.18). The activation links `config.toml` alone, and a test now holds it to leaving `config.local.toml` beside it unchanged. A host pins through `services.boomdrive.settings.pin`, which `settings`' freeform top already renders. No option is added.

### 10. README

README's configuration section says:

- the two files and where each is;
- what the local layer may set, and that its repositories add;
- `pin`;
- what a local layer at fault does.

It shows a local layer in a second ```toml block, after the configuration's. `tests/readme.rs` reads that block over the first, and asserts both layers' repositories kept.

### 11. The record and the rules

The choices of Decisions 1 to 7 are the writing agent's, made within record 0009's amendment of 2026-11-06. Record 0009 gains an amendment of 2026-11-07 holding them, with the paths not taken.

core.17's details gain:

- **Rules out**: a repository let go, or a workspace set, while the local layer stands not taken.
- **Not ruled out**:
  - the local layer at the runner's own configuration directory whatever `--config` names;
  - `pin` at the managed layer's top;
  - a local layer's `partitions` replacing the managed layer's;
  - while the local layer is not taken, a repository only it names left as it stands, neither fetched nor let go (core.5), and every workspace left as it is (core.43).

core.5's details still say "the configuration's managed level written by setup", and core.46's "a control plane writes the managed level". Both are record 0009's first words, which its amendment renames the local layer. They read "local layer" with this change's commit. Record 0028's "the managed layer writes the drives" is the same first word, and is left to the change that writes the drives into the local layer (Risks).

## Risks / Trade-offs

- **The sibling change `a-hosts-configuration-names-its-accounts-and-their-drives`** moves the repositories under `accounts.<account>.drives.<drive>.repositories`. Once both land, the local layer's repositories key is each drive's `repositories`. A drive the local layer names that the managed layer does not is declared by it. A drive both name keeps the repositories of both, a repository still in one drive on the host. Whichever change lands second carries that, as the sibling's design says. Record 0028's "the managed layer writes the drives" then reads "the local layer".
- **A control plane writing in place** leaves a half-written file for a pass. That pass does not take it, lets go of nothing, and says so once. The next says it taken. A control plane writes the file whole and renames it into place to avoid the two lines.
- **A local layer not taken keeps a control plane's repositories unserved** until it is mended: no launch is taken in them and the hello does not name them. That is the control plane's own write to mend, and its line names why.
- **Removing the local file lets go of what it named.** A person who means to keep a control plane's repositories without it moves them into the managed layer first.

## Migration Plan

Nothing to migrate. A host with no `config.local.toml` reads as today. A hosted control plane's setup writes the file when it is built.

Rollback: a runner before this change reads no local file, so its repositories are let go there. A host rolled back keeps them by moving them into its managed layer.
