# Proposal

## Why

The control plane's page copies `boomdrive enroll <label>`, and a person must then paste a token and find a label. The configuration must already name the control plane, the label and the repositories. So a new host cannot enroll from the page alone. On 2026-11-25 the user said this of that step:

> the copy button give me "boomdrive enroll <label>" i don't know how to know to put boomdrive (as a dumb user)

The user then answered a card that asked if the page gives one whole command, with nothing to fill in. The user chose one whole command, as the first-run mockup draws it. The command holds the token, the control plane's address and the repositories to keep. The runner writes its own settings from them. The token is in the shell's history. It works once, for one hour, and is dead after the enrollment.

## What Changes

- **One whole command.** `boomdrive enroll --control-plane <url> --drive <drive> --repository <owner/name>…` takes every part of an enrollment as a flag, and the token on standard input. The control plane's page writes the command whole, as `echo <token> | boomdrive enroll …`. So the person fills in nothing. **BREAKING**: `boomdrive enroll <label>` is gone. A host enrolls with the new form, which the page gives.
- **The drive names the partition.** The drive's name is the partition's label and the account's name. No person chooses a label.
- **The runner writes its own file where nobody else wrote it.** The runner's file can be missing, or be the one that setup or enroll wrote. There, on macOS, enroll writes the account, the drive, its repositories and the hosted table into it. It then loads or restarts the runner's launchd agent. Where Nix or a person wrote the file, enroll changes nothing and names the lines to add.
- **Every check runs before the token goes.** enroll refuses a host that it cannot set up before the request. So the token stays good for its hour.
- **The words that name the enrollment** name the whole command, with its control plane, drive and repositories.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `hosted-control-plane`: the form of the enroll command, the file it writes, the refusals before the token goes, and the words that name the command.
- `configuration`: the local layer's scenario that enrolls a partition takes the new form.

## Impact

- `crates/boomdrive`: `main.rs`, `enroll.rs`, `planes.rs`, and `setup.rs`, whose file writer and agent enroll shares.
- `crates/boomdrive-hosted`: `table.rs` (`enroll_command`), `enrollment.rs`, `forge_token.rs`, `connection.rs`, and their tests.
- `README.md`, the section on enrollment.
- The control plane's page changes its machine step to copy the whole command. That change follows this one and cites it.
