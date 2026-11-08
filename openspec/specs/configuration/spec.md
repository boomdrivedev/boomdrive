# configuration Specification

## Purpose
The runner's configuration on a host is one TOML file. It names the repositories the host keeps by their forge address, where their views go and how often they are fetched. This spec covers where the file is read from, what it may say, and its defaults and refusals. It also requires that a file which cannot be read never counts as one that names no repository (core.4, core.5; crew record 0052 part 2).

## Requirements

### Requirement: The configuration is a managed file and a local one over it

The runner SHALL read its managed layer from the file `--config <file>` names, else `$XDG_CONFIG_HOME/boomdrive/config.toml`, else `~/.config/boomdrive/config.toml` where `XDG_CONFIG_HOME` is unset or empty, and its local layer over it from `config.local.toml` in the default file's directory whatever `--config` names, if there is one (record 0009 as amended 2026-11-06). A managed file that is missing or not valid TOML SHALL be refused, naming the file, the line and what is wrong, never its text.

#### Scenario: The default file

- **WHEN** `boomdrive run --once` runs with no `--config`, `XDG_CONFIG_HOME` unset and `HOME` set to `/h`
- **THEN** it reads `/h/.config/boomdrive/config.toml` as its managed layer and `/h/.config/boomdrive/config.local.toml` as its local layer

#### Scenario: The file named

- **WHEN** `boomdrive run --once --config /etc/boomdrive.toml` runs with `XDG_CONFIG_HOME` set to `/x`
- **THEN** it reads `/etc/boomdrive.toml` as its managed layer and `/x/boomdrive/config.local.toml` as its local layer, and no other file

#### Scenario: No file

- **WHEN** `boomdrive run` starts and the managed file does not exist
- **THEN** it exits 2 naming the path it looked for, and makes, fetches, moves and removes nothing

#### Scenario: No local file

- **WHEN** the managed layer keeps `agentplot/flume` in the account `own`'s drive `home`, and there is no `config.local.toml`
- **THEN** the runner keeps `agentplot/flume` alone and says nothing of a local layer

### Requirement: The configuration names each repository by its forge address

Each entry of a drive's `repositories` SHALL be a GitHub address `owner/name`. The owner SHALL be letters, digits and inner hyphens. The name SHALL be letters, digits, `-`, `_` and `.`, but not `.`, not `..`, and not ending in `.git`. A repository SHALL belong to one drive: two entries that are equal ignoring case, in one drive or in two, SHALL be refused, naming both and their drives (record 0028).

#### Scenario: An address with no owner

- **WHEN** the drive `home` says `repositories = ["flume"]`
- **THEN** it is refused, naming the entry `flume` and the form `owner/name`

#### Scenario: One repository twice

- **WHEN** the drive `home` says `repositories = ["agentplot/flume", "AgentPlot/flume"]`
- **THEN** it is refused, naming both entries

#### Scenario: One repository in two drives

- **WHEN** the drive `home` names `agentplot/flume` and the drive `work` names `AgentPlot/flume`
- **THEN** it is refused, naming both entries and the drives `home` and `work`

#### Scenario: No list

- **WHEN** the configuration sets `repository_root` and has no `accounts`
- **THEN** it is refused, naming `accounts` as required

#### Scenario: Keeping none

- **WHEN** the configuration says `accounts = {}`
- **THEN** it is read as a host that keeps no repository

### Requirement: The repository root is a setting with a default

The configuration SHALL set the directory each kept repository's view is made under with `repository_root`. The value SHALL be an absolute path or one that begins with `~/`, where `~` is the host user's home. Where `repository_root` is absent, the repository root SHALL be `~/Code`. Any other value, a relative path or one naming a variable among them, SHALL be refused, naming it.

#### Scenario: No views setting

- **WHEN** the configuration names `agentplot/flume` and no `repository_root`, and `HOME` is `/h`
- **THEN** the repository's view is `/h/Code/agentplot/flume`

#### Scenario: A home-relative root

- **WHEN** the configuration says `repository_root = "~/src"` and `HOME` is `/h`
- **THEN** the repository root is `/h/src`

#### Scenario: A relative root

- **WHEN** the configuration says `repository_root = "Code"` or `repository_root = "$HOME/Code"`
- **THEN** it is refused, naming the value and saying a repository root is absolute or begins with `~/`

### Requirement: How often repositories are fetched is a setting with a default

The configuration SHALL set how often every kept repository is fetched with `fetch_interval`: a whole number of seconds, minutes or hours, written `<n>s`, `<n>m` or `<n>h`, more than zero. Where it is absent, the interval SHALL be two minutes. Any other value SHALL be refused, naming it.

#### Scenario: No period

- **WHEN** the configuration has no `fetch_interval`
- **THEN** the runner fetches every two minutes

#### Scenario: A period of nothing

- **WHEN** the configuration says `fetch_interval = "0m"` or `fetch_interval = "soon"`
- **THEN** it is refused, naming the value and the form `<n>s`, `<n>m` or `<n>h`

### Requirement: The configuration holds nothing the runner does not read

The configuration SHALL be refused, naming the key by its path, when it holds any key this spec does not name, at its top, in an account or in a drive. In particular, no key SHALL hold a token, key or credential: git's own credential helper on the host answers for every fetch (core.4).

#### Scenario: A misspelt key

- **WHEN** the configuration says `repositories_root = "~/Code"`
- **THEN** it is refused, naming `repositories_root` as a key the runner does not read

#### Scenario: A token

- **WHEN** the configuration says `token = "..."`
- **THEN** it is refused, naming `token` as a key the runner does not read, and the value appears in no output

#### Scenario: A key in a drive

- **WHEN** the account `own`'s drive `home` says `machine = "elsewhere"`
- **THEN** it is refused, naming `accounts.own.drives.home.machine` as a key the runner does not read

#### Scenario: A key in an account

- **WHEN** the account `own` says `state = "example/state"`
- **THEN** it is refused, naming `accounts.own.state` as a key the runner does not read

### Requirement: The configuration is read again at every pass

The running runner SHALL read its configuration again at the start of every pass. A repository added SHALL be kept from that pass, and one dropped SHALL be let go from that pass. A pass that cannot read the configuration SHALL name the file and what is wrong with it, and SHALL make, fetch, move and remove nothing. The configuration SHALL never be taken to name no repository because it could not be read.

#### Scenario: A repository added while running

- **WHEN** `agentplot/sluice` is added to the configuration of a running runner
- **THEN** the next pass clones it and makes its view

#### Scenario: A file broken while running

- **WHEN** the configuration of a running runner that keeps `agentplot/flume` is made invalid
- **THEN** the next pass names the file's error and removes nothing, `agentplot/flume`'s clone and view among them; and once the file is valid again, the pass after keeps as it says

### Requirement: A refused stream, kind or key of before says where it is held

No key, at the top, in an account or in a drive, SHALL name a stream, a kind, a kinds file or a kind's source: these are each host's Flume configuration (core.5). A refusal of `stream` or `streams` SHALL say streams are, and one of `kind`, `kinds` or `kinds_file` that kinds are. A refusal of `views`, `fetch_every` or a top-level `repositories` SHALL name the key that holds it now: `repository_root`, `fetch_interval` or `accounts` (core.12; record 0028).

#### Scenario: A stream named

- **WHEN** the configuration says `stream = "work"`, or holds a `[streams]` table
- **THEN** it is refused, naming the key as one the runner does not read and saying streams are Flume's host configuration

#### Scenario: A stream in a drive

- **WHEN** the account `own`'s drive `home` says `stream = "home"`
- **THEN** it is refused, naming `accounts.own.drives.home.stream` as a key the runner does not read and saying streams are Flume's host configuration

#### Scenario: A kinds file named

- **WHEN** the configuration holds `kinds = { crew = "/k/kinds.toml" }`
- **THEN** it is refused, naming `kinds` as a key the runner does not read and saying kinds are Flume's host configuration

#### Scenario: A key of before

- **WHEN** the configuration says `views = "~/Code"` and `fetch_every = "2m"`
- **THEN** it is refused, naming `views` as held by `repository_root` and `fetch_every` as held by `fetch_interval`

#### Scenario: A flat list of before

- **WHEN** the configuration says `repositories = ["agentplot/flume"]` at its top
- **THEN** it is refused, naming `repositories` as held by `accounts`, each repository named under its drive

### Requirement: The configuration may name the control plane

The configuration MAY hold a `[control_plane]` table. In it, `type`, the word of the implementation to open, SHALL be required, and the table named by `type` SHALL be handed whole to that implementation, which reads its own keys. Where the configuration holds no `[control_plane]`, the runner SHALL keep its repositories and make no control pass.

#### Scenario: crew named

- **WHEN** the configuration holds `[control_plane]` with `type = "crew"` and a `[control_plane.crew]` table
- **THEN** the runner opens crew's implementation with that table

#### Scenario: No type

- **WHEN** the configuration holds `[control_plane]` with `poll_interval = "30s"` and no `type`
- **THEN** it is refused, naming `type` as required

#### Scenario: No control plane

- **WHEN** the configuration holds no `[control_plane]`
- **THEN** `boomdrive run --once` keeps the repositories and runs no `flume` command

### Requirement: How often the control plane is polled is a setting with a default

`[control_plane]`'s `poll_interval` SHALL be how often the runner makes a control pass, a whole number of seconds, minutes or hours written `<n>s`, `<n>m` or `<n>h`, more than zero. Where it is absent, the interval SHALL be one minute. Any other value SHALL be refused, naming it.

#### Scenario: No poll interval

- **WHEN** `[control_plane]` has no `poll_interval`
- **THEN** the runner makes a control pass every minute

#### Scenario: A poll interval of nothing

- **WHEN** `[control_plane]` says `poll_interval = "0s"`
- **THEN** it is refused, naming the value and the form `<n>s`, `<n>m` or `<n>h`

### Requirement: The control-plane table is read when the runner starts

The `[control_plane]` table SHALL be read when the runner starts, and a change to it SHALL take effect when the runner starts again.

#### Scenario: A poll interval changed while running

- **WHEN** `poll_interval` is changed from `1m` to `5m` in the file of a running runner
- **THEN** its control passes keep a one-minute interval until the runner starts again

### Requirement: Whether each drive has a VS Code workspace is a setting

The configuration MAY set `code_workspaces`: true where the runner keeps a VS Code workspace for each drive of its configuration (views spec), false where it keeps none. Where it is absent, it SHALL be false. Any value but `true` or `false` SHALL be refused, naming the key and the value. It SHALL be read again at every pass, as `accounts` is.

#### Scenario: No workspaces setting

- **WHEN** the configuration keeps `agentplot/flume` and has no `code_workspaces`
- **THEN** it is read with `code_workspaces` false

#### Scenario: Workspaces asked for in words

- **WHEN** the configuration says `code_workspaces = "yes"`
- **THEN** it is refused, naming `code_workspaces` and saying it is `true` or `false`

#### Scenario: Workspaces asked for while running

- **WHEN** `code_workspaces = true` is added to the configuration of a running runner that keeps `agentplot/flume` in the drive `home`
- **THEN** the next pass writes `/h/Code/home.code-workspace`, listing `agentplot/flume`

### Requirement: The local layer sets the repositories and a control plane's partitions alone

The local layer MAY set each drive's `repositories`, `accounts.<account>.drives.<drive>.repositories`, and in a control plane's table, `[control_plane.<type>]`, the keys its implementation lists for it: `partitions`, in crew's table and in the hosted one's. Each SHALL be one key, its value read whole as the managed layer's is read. Every command of the runner SHALL read both layers (record 0009 as amended 2026-11-06; record 0028 as amended 2026-11-07; core.17; flume record 0032).

#### Scenario: The local layer's repositories

- **WHEN** the managed layer keeps `agentplot/flume` in the account `own`'s drive `home`, and the local layer names `agentplot/sluice` in that drive
- **THEN** the runner keeps `agentplot/flume`, then `agentplot/sluice`, both in the drive `home`

#### Scenario: A partition enrolled from the local layer

- **WHEN** `boomdrive enroll work` runs where only the local layer's hosted `partitions` name `work`, and `url` names a loopback port with nothing on it
- **THEN** it is refused as the control plane unreachable, exiting 1, and not as a label the table does not name

### Requirement: The local layer's repositories add to the managed layer's, and its partitions replace them

A drive both layers name under one account SHALL keep the managed layer's repositories, then each the local layer names that the managed layer does not, a repository both name kept once and refused in neither. A drive or an account only the local layer names SHALL be declared by it, after the managed layer's. A control plane's `partitions` the local layer sets SHALL replace the managed layer's whole (record 0009 as amended 2026-11-06; record 0028 as amended 2026-11-07).

#### Scenario: A repository both layers name

- **WHEN** the managed layer keeps `agentplot/flume` in the drive `home`, and the local layer names `AgentPlot/flume` and `agentplot/sluice` in the drive `home`
- **THEN** the runner keeps `agentplot/flume` once and `agentplot/sluice`, and refuses neither layer

#### Scenario: A drive only the local layer names

- **WHEN** the managed layer keeps `agentplot/flume` in the account `own`'s drive `home`, and the local layer names `afterthought/crew` in the account `own`'s drive `work` and `agentplot/sluice` in the account `client`'s drive `garden`
- **THEN** the runner keeps the drives `home`, `work` and `garden`, in that order, each with its repository

#### Scenario: A repository the local layer drops

- **WHEN** a running runner keeps `agentplot/flume` from its managed layer and `agentplot/sluice` from its local layer, and the local layer is rewritten to name no repository in the drive
- **THEN** the next pass lets go of `agentplot/sluice` as a repository dropped, and keeps `agentplot/flume`

#### Scenario: The local layer's partitions

- **WHEN** the managed layer names `[control_plane]` with `type = "hosted"` and `[control_plane.hosted]` with a `url` and the partition `home`, and the local layer names the hosted partition `work` alone
- **THEN** the table handed to the hosted implementation holds the managed layer's `url` and the partition `work` alone

### Requirement: A local layer that sets any other key is not taken

A local file that sets any key the runner does not list for it SHALL not be taken, and the configuration SHALL be the managed layer alone: one of the runner's own, such as `repository_root`, `pin` or a flat `repositories`; any of an account or a drive but `drives` and `repositories`; or any of a control plane's tables it does not list, such as `type` or `url`. The runner SHALL name each by its dotted path, the shortest holding no key the local layer may set, never its value (core.4).

#### Scenario: A root in the local layer

- **WHEN** the managed layer keeps `agentplot/flume` in the drive `home`, and the local layer says `repository_root = "~/src"` and names `agentplot/sluice` in the drive `home`
- **THEN** the local layer is not taken: the repository root is the managed layer's, `agentplot/sluice` is not kept, and the runner names `repository_root` as a key the local layer does not set

#### Scenario: A key in a local drive

- **WHEN** the local layer's account `own` says `state_repository = "example/state"`, and its drive `home` names `agentplot/sluice` and says `stream = "home"`
- **THEN** the local layer is not taken, and the runner names `accounts.own.drives.home.stream` and `accounts.own.state_repository`

#### Scenario: A flat list in the local layer

- **WHEN** the local layer says `repositories = ["agentplot/sluice"]` at its top
- **THEN** the local layer is not taken, and the runner names `repositories`

#### Scenario: A control plane named in the local layer

- **WHEN** the local layer holds `[control_plane]` with `type = "hosted"`
- **THEN** the local layer is not taken, and the runner names `control_plane.type`

#### Scenario: A token in the local layer

- **WHEN** the local layer says `token = "..."`
- **THEN** the local layer is not taken, the runner names `token`, and the value appears in no output

### Requirement: A local layer the runner cannot read is not taken

A local file SHALL not be taken where it is not valid TOML, where the runner's reading refuses a value it sets, such as an address or a name out of form, one repository named twice or a drive under two accounts, or where the implementation's reading of its own table refuses one. The configuration SHALL then be the managed layer alone. The runner SHALL name the fault as it names a managed file's: the file, the line where it has one and what is wrong, never the file's text.

#### Scenario: A local file that is not TOML

- **WHEN** the local layer says `accounts.own.drives.home.repositories = [`
- **THEN** the local layer is not taken, and the runner names `config.local.toml`, its line and what is wrong

#### Scenario: A local address out of form

- **WHEN** the local layer names `sluice` in the drive `home`
- **THEN** the local layer is not taken, and the runner names the entry `sluice` and the form `owner/name`

#### Scenario: A partition the implementation refuses

- **WHEN** the managed layer names the hosted control plane with the partition `home`, and the local layer names a hosted partition labelled `Home`
- **THEN** the local layer is not taken, the runner names the label as the hosted implementation's reading refuses it, and the hosted implementation is handed the managed layer's table, naming `home`

### Requirement: While the local layer is not taken, nothing is let go

While a local file stands that is not taken, a keeping pass SHALL keep the managed layer's repositories, SHALL let go of no repository, and SHALL set no workspace and remove none, since the local layer it could not take may name them. A local file that does not exist is no local layer, and a repository only it named SHALL be let go as any dropped repository is (crew record 0052 part 2; this spec's "read again at every pass").

#### Scenario: A local file broken while running

- **WHEN** a running runner asked for workspaces keeps `agentplot/flume` in the drive `home` from its managed layer and `agentplot/sluice` in the drive `work` from its local layer, and the local layer is made invalid
- **THEN** the next passes fetch `agentplot/flume` and remove nothing, `agentplot/sluice`'s clone and view among them, and leave `home.code-workspace` and `work.code-workspace` as they are; and once the local layer is valid again, the pass after keeps both

#### Scenario: A local file removed

- **WHEN** a running runner keeps `agentplot/flume` from its managed layer and `agentplot/sluice` from its local layer, and `config.local.toml` is removed
- **THEN** the next pass lets go of `agentplot/sluice` as a repository dropped, and keeps `agentplot/flume`

### Requirement: The managed layer may pin a key the local layer could set

The managed layer MAY set `pin`, a list of keys in TOML's dotted spelling, each a key the local layer may set, such as `"control_plane.hosted.partitions"`, or a table holding one, such as `"accounts"` or `"accounts.own.drives.home"`, which pins each such key under it. A key the local layer sets that a pin holds SHALL be passed over and named with its pin, and the rest of the local layer SHALL be taken (flume record 0032, point 4; core.17).

#### Scenario: Repositories pinned

- **WHEN** the managed layer says `pin = ["accounts"]` and keeps `agentplot/flume` in the drive `home`, and the local layer names `agentplot/sluice` in the drive `home` and `afterthought/crew` in the drive `work`
- **THEN** the runner keeps `agentplot/flume` alone, and names `accounts.own.drives.home.repositories` and `accounts.own.drives.work.repositories` as passed over, pinned by `accounts`

#### Scenario: A table pinned

- **WHEN** the managed layer says `pin = ["control_plane"]`, keeps no repository in the drive `home` and names the hosted partition `home`, and the local layer names `agentplot/sluice` in the drive `home` and the hosted partition `work`
- **THEN** the runner keeps `agentplot/sluice`, the hosted table names the partition `home` alone, and the runner names `control_plane.hosted.partitions` as passed over, pinned by `control_plane`

### Requirement: A pin of nothing is a fault of the managed layer

A `pin` that is not a list of strings, or an entry of it that names nothing a local layer may set, SHALL be refused as a fault of the managed layer, naming the entry, as any other fault of the managed layer is refused (flume record 0032, point 4).

#### Scenario: A pin of nothing

- **WHEN** the managed layer says `pin = ["repository_root"]`
- **THEN** the configuration is refused, naming the `pin` entry `repository_root` as no key a local layer may set, and `boomdrive run` exits 2

### Requirement: How the local layer was read is said once each way

A keeping pass SHALL say one line naming the local file where its reading of the local layer differs from the last one said: taken, with each key passed over and its pin; not taken, with each key or the fault, the managed layer read alone and nothing let go; or gone, the managed layer read alone. Nothing SHALL be said of a local file absent at the start, nor by a pass that reads the local layer as the one before did (core.23 as the model; core.40).

#### Scenario: Taken at the start

- **WHEN** `boomdrive run --once` runs with a local layer naming `agentplot/sluice` in the drive `home`
- **THEN** it says one line naming `config.local.toml` as taken

#### Scenario: A fault said once

- **WHEN** a running runner's local layer is not TOML for three passes and is then mended
- **THEN** the runner says one line naming the fault, and then one line saying the local layer is taken

### Requirement: The configuration names its accounts, each account's drives and each drive's repositories

The configuration SHALL name what the host keeps under `accounts`: a table of accounts, each holding `drives`, a table of drives, each holding `repositories`, the drive's repositories in the order kept. `accounts`, each account's `drives` and each drive's `repositories` SHALL be required, and each MAY be empty, so a host that keeps none says `accounts = {}`. The runner SHALL keep every drive's repositories, in the file's order (record 0028; core.5).

#### Scenario: One drive

- **WHEN** the configuration holds `[accounts.own.drives.home]` with `repositories = ["agentplot/flume"]`
- **THEN** it is read as keeping `agentplot/flume` in the drive `home` of the account `own`

#### Scenario: Drives of two accounts

- **WHEN** the account `own`'s drive `home` names `agentplot/flume` and `afterthought/crew`, and then the account `client`'s drive `garden` names `agentplot/sluice`
- **THEN** the runner keeps `agentplot/flume`, `afterthought/crew` and `agentplot/sluice`, in that order

#### Scenario: A drive with no list

- **WHEN** `[accounts.own.drives.home]` holds no `repositories`
- **THEN** it is refused, naming `accounts.own.drives.home.repositories` as required

#### Scenario: A drive keeping none

- **WHEN** the account `own`'s drive `home` says `repositories = []`
- **THEN** it is read with the drive `home` keeping no repository

### Requirement: An account's and a drive's name is of one form, and a drive's is given once on the host

An account's and a drive's name SHALL be 1 to 32 lower-case letters, digits and `-`, starting with a letter; any other SHALL be refused, naming it and the form. A drive's name is its stream's, its crew branch's and its workspace's, so it SHALL be given once on the host: a drive two accounts name SHALL be refused, naming the drive and both accounts (records 0017 and 0028).

#### Scenario: One drive in two accounts

- **WHEN** the accounts `own` and `client` each hold a drive `home`
- **THEN** it is refused, naming `home`, `own` and `client`

#### Scenario: A name out of form

- **WHEN** the configuration holds `[accounts.own.drives.Home]`, or `[accounts.my_own.drives.home]`
- **THEN** it is refused, naming `Home`, or `my_own`, and the form

### Requirement: A local layer naming a repository or a drive the managed layer holds elsewhere is not taken

A repository SHALL belong to one drive, and a drive's name SHALL be given once, across both layers: a local layer that names a repository the managed layer keeps in another drive, or a drive the managed layer holds under another account, SHALL not be taken, and the runner SHALL name both (record 0028 as amended 2026-11-07).

#### Scenario: A repository the managed layer keeps in another drive

- **WHEN** the managed layer keeps `agentplot/flume` in the drive `home`, and the local layer names `AgentPlot/Flume` in the drive `work`
- **THEN** the local layer is not taken, the runner names the local entry, `agentplot/flume` and the drives `work` and `home`, and `agentplot/flume` stays in the drive `home`

#### Scenario: A drive the managed layer holds under another account

- **WHEN** the managed layer holds the drive `home` under the account `own`, and the local layer names the drive `home` under the account `client`
- **THEN** the local layer is not taken, and the runner names the drive `home` and the accounts `own` and `client`
