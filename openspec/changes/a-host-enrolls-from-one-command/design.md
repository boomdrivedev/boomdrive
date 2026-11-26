# Design

## Context

`boomdrive enroll <label>` enrolls a partition that the configuration already names. `[control_plane.hosted]` gives its `url`, and a partition of the label gives its repositories. The token comes on standard input. The command writes no configuration (the hosted-control-plane spec). So a new host needs a file written by hand, a label and a pasted token before it can enroll.

The user ruled on 2026-11-25 that the control plane's page gives one whole command, as the first-run mockup draws it. The command holds the token, the control plane's address and the repositories to keep. The runner writes its own settings from them.

This design relies on core.2, core.4, core.5, core.13, core.17, core.18, core.31, core.32, core.36 and core.39.

## Decisions

### Decision 1: The command's form

```console
echo <token> | boomdrive enroll --control-plane <url> --drive <drive> --repository <owner/name> [--repository <owner/name>…] [--config <file>] [--json]
```

The page writes every part of this line, so the person fills in nothing. `--control-plane` is the hosted table's `url`, under which the runner enrolls at `runner/enroll`. `--drive` is the drive's name on the page. It takes the configuration's form for a name: 1 to 32 lower-case letters, digits and `-`, starting with a letter. `--repository` names each repository that the drive keeps, one or more. Each of the three flags is required. A flag that is missing, given twice where it is single, or out of form is a usage error, exit 2.

The token comes on standard input, never as a flag. core.31 rules out a token from an argument or the environment, because another user on the host reads the arguments of a process. `echo` is a builtin of `sh`, `bash`, `zsh` and `fish`. So the token is in the arguments of no process. The token is in the shell's history, as the user accepted. Where a token holds a character that a shell reads, the page quotes it.

The command takes no positional label. The old form, `boomdrive enroll <label>`, is gone with no alias. core.36 names files and not commands, and the release notes name the move.

### Decision 2: The drive names the account, the drive and the partition

The runner names the account, the drive and the hosted partition after `--drive`. The label of the partition also names its key in the credential store, `<drive>@<host of the control plane>` (core.32). The page knows the drive, so no person chooses a label.

One enrollment is one partition, with a key of its own (core.19). A second drive of the same account on the same host takes a second token and is a second partition. A drive's name is given once on the host (record 0028). So enroll refuses a drive that another account of the host holds, crew's among them, before the token goes.

### Decision 3: Where the runner writes, and what it writes

enroll reads both layers as `run` reads them (record 0009 as amended 2026-11-06). The managed layer is at `--config` or at the default path. Then enroll takes one of three ways.

1. **The file names the enrollment already.** The hosted `url` is `--control-plane`. A partition of the drive's name names each repository. The drive, under a hosted account, keeps each repository. enroll writes nothing and enrolls. A host whose Nix module writes these keys takes this way, and so does any host after its first enrollment.
2. **The file is missing, or is the runner's own, on macOS.** The runner's own file has setup's first line, `# Written by boomdrive setup; a change goes through boomdrive setup.`. enroll sets the account with `control_plane = "hosted"`, its drive with the repositories, the hosted `url`, and the partition with the repositories. It keeps every other account, drive and table as they stand, by setup's own writer, `runner_file` (record 0054). The first line stays setup's, so setup and enroll both read the file as the runner's own. enroll refuses a file whose hosted table names another `url`, since a host serves one plane of each implementation.
3. **Any other file.** A link, as the activation of Nix makes it, or a file that a person wrote, is not the runner's to write (core.17, core.18). enroll changes nothing and sends nothing. It exits 2, naming the file and the keys to add. For a link, it names the options `services.boomdrive.accounts.<drive>` and `services.boomdrive.settings.control_plane.hosted`. For a person's file, it names the keys as TOML. Its words end with this: the same command enrolls the host after the change, and the token is not spent.

On Linux, enroll refuses way 2 as it refuses way 3, naming `services.boomdrive`. Setup's agent is for macOS alone. A file with no runner to read it enrolls a host that never dials in.

The person on a host that Nix manages, such as a Mac whose `~/.config/boomdrive/config.toml` links into the store, sees the refusal. They see the Nix lines to add and the words "Switch, then run the same command again. The token is not spent.". After the switch, the same command takes way 1.

### Decision 4: Every check runs before the token goes

Before it reads the token, enroll checks the flags, reads both layers and chooses its way. It checks the file that it would write by the runner's own reading, `config::parse`, and by the hosted implementation's reading of its table. It finds the credential store as before (core.32). When any of them refuses, enroll exits 2 with no request. So the token stays good for its hour.

Then enroll reads the token and sends the one `POST runner/enroll`. It keeps the key only once `enrolled` answers, as before. enroll writes the file only after `enrolled`, so a refused token leaves the host as it was. When enroll cannot write the file after `enrolled`, it refuses as the host's to fix and names the host that the control plane enrolled. A new token enrolls the partition again.

### Decision 5: The runner's agent runs what enroll wrote

Where enroll wrote the file on macOS, it writes setup's agent, `io.github.agentplot.boomdrive`, by setup's own `agent` and `load_agent`. So the runner starts with the file. The runner reads `[control_plane]` when it starts (the configuration spec). So where the agent is loaded with its plist unchanged and enroll changed the file, enroll runs `launchctl kickstart -k gui/<uid>/io.github.agentplot.boomdrive`. The service manager carries out the restart (core.13). Where enroll wrote no file, it touches no agent. On way 1 a running runner already holds the partition, and its connection reads the key at its next try (the hosted-control-plane spec).

### Decision 6: The answer gains the file and the agent

The answer keeps its form and gains fields (core.39). It is `{"enrolled":{"partition","host","config":{"file","outcome"},"service":{"file","outcome"}}}`. `config.outcome` is `written` or `unchanged`. `service` is there only where enroll wrote the file. Its outcome is `loaded`, `reloaded`, `restarted` or `running`. Without `--json`, enroll says one line for each step. No output holds the token (core.4).

### Decision 7: The words that name the enrollment name the whole command

`Table::enroll_command(label)` gives `boomdrive enroll --control-plane <url> --drive <label>`, with a `--repository` for each repository of the partition. The words add "with its token on standard input". The refusals of a key and of an ask for a forge token name that command.

## Out of this change

- **The install.** The mockup's `curl … /install | sh -s -- …` also installs the runner. A release builds no binary (core.48). The set reaches a host from the family's cache, through the flake (core.37, core.47). So an install script installs Nix where it is missing. It trusts the family's cache at the person's asking, and adds the set at its release tag to the person's profile. It then runs the command of this change. This repository keeps the script, and its site serves it at a fixed address. So the page names that address and serves no script of its own. The script gives the token to `boomdrive enroll` on standard input, never in the arguments of `sh` (core.31). The install is a change of its own, with its own design.
- **The page.** The control plane's change follows this one and cites it.
