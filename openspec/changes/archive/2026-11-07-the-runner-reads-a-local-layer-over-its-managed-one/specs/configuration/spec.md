# Configuration

## RENAMED Requirements

- FROM: `### Requirement: The configuration is one file the runner reads`
- TO: `### Requirement: The configuration is a managed file and a local one over it`

## MODIFIED Requirements

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

- **WHEN** the managed layer says `repositories = ["agentplot/flume"]` and there is no `config.local.toml`
- **THEN** the runner keeps `agentplot/flume` alone and says nothing of a local layer

## ADDED Requirements

### Requirement: The local layer sets the repositories and a control plane's partitions alone

The local layer MAY set `repositories`, and in a control plane's table, `[control_plane.<type>]`, the keys its implementation lists for it: `partitions`, in crew's table and in the hosted one's. Each SHALL be one key, its value read whole as the managed layer's is read. Every command of the runner SHALL read both layers (record 0009 as amended 2026-11-06; core.17; flume record 0032).

#### Scenario: The local layer's repositories

- **WHEN** the managed layer says `repositories = ["agentplot/flume"]` and the local layer says `repositories = ["agentplot/sluice"]`
- **THEN** the runner keeps `agentplot/flume`, then `agentplot/sluice`

#### Scenario: A partition enrolled from the local layer

- **WHEN** `boomdrive enroll work` runs where only the local layer's hosted `partitions` name `work`, and `url` names a loopback port with nothing on it
- **THEN** it is refused as the control plane unreachable, exiting 1, and not as a label the table does not name

### Requirement: The local layer's repositories add to the managed layer's, and its partitions replace them

The runner SHALL keep the managed layer's repositories, in their order, then each the local layer names that the managed layer does not, in its order. A repository both layers name SHALL be kept once and refused in neither. A control plane's `partitions` the local layer sets SHALL replace the managed layer's whole (record 0009 as amended 2026-11-06, "Repositories add").

#### Scenario: A repository both layers name

- **WHEN** the managed layer says `repositories = ["agentplot/flume"]` and the local layer says `repositories = ["AgentPlot/flume", "agentplot/sluice"]`
- **THEN** the runner keeps `agentplot/flume` once and `agentplot/sluice`, and refuses neither layer

#### Scenario: A repository the local layer drops

- **WHEN** a running runner keeps `agentplot/flume` from its managed layer and `agentplot/sluice` from its local layer, and the local layer is rewritten to say `repositories = []`
- **THEN** the next pass lets go of `agentplot/sluice` as a repository dropped, and keeps `agentplot/flume`

#### Scenario: The local layer's partitions

- **WHEN** the managed layer names `[control_plane]` with `type = "hosted"` and `[control_plane.hosted]` with a `url` and the partition `home`, and the local layer names the hosted partition `work` alone
- **THEN** the table handed to the hosted implementation holds the managed layer's `url` and the partition `work` alone

### Requirement: A local layer that sets any other key is not taken

A local file that sets any key the runner does not list for it SHALL not be taken, and the configuration SHALL be the managed layer alone. Such a key is one of the runner's own, such as `repository_root` or `pin`; one of `[control_plane]`, such as `type`; one of an implementation's table it does not list, such as `url`; or one the runner does not read. The runner SHALL name each by its dotted path, the shortest holding no key the local layer may set, and never its value (core.4).

#### Scenario: A root in the local layer

- **WHEN** the managed layer says `repositories = ["agentplot/flume"]`, and the local layer says `repository_root = "~/src"` and `repositories = ["agentplot/sluice"]`
- **THEN** the local layer is not taken: the repository root is the managed layer's, `agentplot/sluice` is not kept, and the runner names `repository_root` as a key the local layer does not set

#### Scenario: A control plane named in the local layer

- **WHEN** the local layer holds `[control_plane]` with `type = "hosted"`
- **THEN** the local layer is not taken, and the runner names `control_plane.type`

#### Scenario: A token in the local layer

- **WHEN** the local layer says `token = "..."`
- **THEN** the local layer is not taken, the runner names `token`, and the value appears in no output

### Requirement: A local layer the runner cannot read is not taken

A local file SHALL not be taken where it is not valid TOML, where the runner's reading refuses a value it sets, such as a `repositories` entry out of form or two that are one repository, or where the implementation's reading of its own table refuses one. The configuration SHALL then be the managed layer alone. The runner SHALL name the fault as it names a managed file's: the file, the line where it has one and what is wrong, never the file's text.

#### Scenario: A local file that is not TOML

- **WHEN** the local layer says `repositories = [`
- **THEN** the local layer is not taken, and the runner names `config.local.toml`, its line and what is wrong

#### Scenario: A local address out of form

- **WHEN** the local layer says `repositories = ["sluice"]`
- **THEN** the local layer is not taken, and the runner names the entry `sluice` and the form `owner/name`

#### Scenario: A partition the implementation refuses

- **WHEN** the managed layer names the hosted control plane with the partition `home`, and the local layer names a hosted partition labelled `Home`
- **THEN** the local layer is not taken, the runner names the label as the hosted implementation's reading refuses it, and the hosted implementation is handed the managed layer's table, naming `home`

### Requirement: While the local layer is not taken, nothing is let go

While a local file stands that is not taken, a keeping pass SHALL keep the managed layer's repositories, SHALL let go of no repository and SHALL set no workspace, since the local layer it could not take may name them. A local file that does not exist is no local layer, and a repository only it named SHALL be let go as any dropped repository is (crew record 0052 part 2; this spec's "read again at every pass").

#### Scenario: A local file broken while running

- **WHEN** a running runner keeps `agentplot/flume` from its managed layer and `agentplot/sluice` from its local layer, and the local layer is made invalid
- **THEN** the next passes fetch `agentplot/flume` and remove nothing, `agentplot/sluice`'s clone and view among them; and once the local layer is valid again, the pass after keeps both

#### Scenario: A local file removed

- **WHEN** a running runner keeps `agentplot/flume` from its managed layer and `agentplot/sluice` from its local layer, and `config.local.toml` is removed
- **THEN** the next pass lets go of `agentplot/sluice` as a repository dropped, and keeps `agentplot/flume`

### Requirement: The managed layer may pin a key the local layer could set

The managed layer MAY set `pin`, a list of keys in TOML's dotted spelling, each a key the local layer may set, such as `"repositories"` or `"control_plane.hosted.partitions"`, or a table holding one, such as `"control_plane"`, which pins each such key under it. A key the local layer sets that a pin holds SHALL be passed over and named with its pin, and the rest of the local layer SHALL be taken (flume record 0032, point 4; core.17).

#### Scenario: Repositories pinned

- **WHEN** the managed layer says `pin = ["repositories"]` and `repositories = ["agentplot/flume"]`, and the local layer says `repositories = ["agentplot/sluice"]`
- **THEN** the runner keeps `agentplot/flume` alone, and names `repositories` as passed over, pinned by `repositories`

#### Scenario: A table pinned

- **WHEN** the managed layer says `pin = ["control_plane"]` and names the hosted partition `home`, and the local layer says `repositories = ["agentplot/sluice"]` and names the hosted partition `work`
- **THEN** the runner keeps `agentplot/sluice`, the hosted table names the partition `home` alone, and the runner names `control_plane.hosted.partitions` as passed over, pinned by `control_plane`

### Requirement: A pin of nothing is a fault of the managed layer

A `pin` that is not a list of strings, or an entry of it that names nothing a local layer may set, SHALL be refused as a fault of the managed layer, naming the entry, as any other fault of the managed layer is refused (flume record 0032, point 4).

#### Scenario: A pin of nothing

- **WHEN** the managed layer says `pin = ["repository_root"]`
- **THEN** the configuration is refused, naming the `pin` entry `repository_root` as no key a local layer may set, and `boomdrive run` exits 2

### Requirement: How the local layer was read is said once each way

A keeping pass SHALL say one line naming the local file where its reading of the local layer differs from the last one said: taken, with each key passed over and its pin; not taken, with each key or the fault, the managed layer read alone and nothing let go; or gone, the managed layer read alone. Nothing SHALL be said of a local file absent at the start, nor by a pass that reads the local layer as the one before did (core.23 as the model; core.40).

#### Scenario: Taken at the start

- **WHEN** `boomdrive run --once` runs with a local layer saying `repositories = ["agentplot/sluice"]`
- **THEN** it says one line naming `config.local.toml` as taken

#### Scenario: A fault said once

- **WHEN** a running runner's local layer is not TOML for three passes and is then mended
- **THEN** the runner says one line naming the fault, and then one line saying the local layer is taken
