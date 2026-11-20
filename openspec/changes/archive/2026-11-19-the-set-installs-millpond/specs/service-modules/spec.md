## ADDED Requirements

### Requirement: The modules install millpond with the runner, replaceable or dropped

With `services.boomdrive.enable`, the modules SHALL install `services.boomdrive.millpond.package`, the set's millpond by default, in the system profile. A host's own package SHALL replace it, and `null` SHALL install none, write no millpond configuration and name no inbox to the runner (record 0033).

#### Scenario: The set's millpond

- **WHEN** a system turns on `services.boomdrive` and names no millpond
- **THEN** the set's millpond is in its system packages, on nix-darwin and on NixOS

#### Scenario: No millpond

- **WHEN** a system sets `services.boomdrive.millpond.package = null` and names inboxes
- **THEN** no millpond is installed, no configuration of millpond's is rendered, and the runner's file names no `inboxes`

### Requirement: The modules write millpond's host configuration and the runner's inboxes from one place

With the runner and millpond, and `services.boomdrive.millpond.settings.inbox` naming any inbox, the modules SHALL render millpond's host configuration from `services.boomdrive.millpond.settings` at evaluation, each key as millpond names it, link it at each activation at `~/.config/millpond/config.toml`, and give the runner's file `inboxes`, the inboxes' names in order, a host's own `inboxes` winning. With no inbox, they SHALL render and link nothing of millpond's (record 0033).

#### Scenario: Two inboxes

- **WHEN** a system names the inboxes `home`, of `boomdrivedev/strategy`, and `work`, of `/Users/alice/Code/work` on the branch `signals`, with the host `deneb` and a secrets folder
- **THEN** millpond's configuration renders the host, the secrets folder and both inboxes, `home` on the branch `inbox`; the activation links it at millpond's own path; and the runner's file says `inboxes = ["home", "work"]`, on nix-darwin and on NixOS

#### Scenario: No inbox

- **WHEN** a system turns on the runner and names no inbox
- **THEN** no configuration of millpond's is rendered or linked, and the runner's file names no `inboxes`

### Requirement: An inbox named twice or secrets in the store fail evaluation

An inbox `services.boomdrive.millpond.settings.inbox` names more than once, or a `secrets` folder in the Nix store, SHALL fail evaluation, naming it (record 0033; core.4).

#### Scenario: An inbox named twice

- **WHEN** a system names the inbox `home` twice
- **THEN** its evaluation fails, saying an inbox is named more than once
