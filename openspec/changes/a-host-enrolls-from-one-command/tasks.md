# Tasks

## 1. The command's form

- [ ] 1.1 `boomdrive enroll` takes `--control-plane`, `--drive`, one or more `--repository`, `--config` and `--json`. It takes no positional label and no `--token`. A flag that is missing or out of form is a usage error before enroll reads the token (Decisions 1 and 4). True when `devenv shell -- cargo test -p boomdrive enroll::` passes, proving "A flag missing", "A drive name out of form" and "A token flag".
- [ ] 1.2 enroll names the account, the drive and the partition after `--drive`. It refuses a drive that another account holds before it reads the token (Decision 2). True when `devenv shell -- cargo test -p boomdrive enroll::` passes, proving "The names" and "A drive another account holds".

## 2. Where enroll writes

- [ ] 2.1 enroll chooses its way from both layers, before it reads the token (Decisions 3 and 4). It leaves a file that names the enrollment as it is. On macOS, it writes a missing file, or one with setup's first line, after `enrolled`, by setup's `runner_file`. It refuses any other file, and any file off macOS, naming the keys to add. True when `devenv shell -- cargo test -p boomdrive enroll::` passes, proving "A host Nix set up", "A host with no file", "Beside crew's drives", "Another hosted control plane" and "The token refused".
- [ ] 2.2 The refusals name the keys to add. True when `devenv shell -- cargo test -p boomdrive enroll::` passes, proving "The file is Nix's", "A person's own file", "On Linux with no file" and "A partition enrolled from the local layer".
- [ ] 2.3 Where enroll wrote the file, it writes and loads setup's agent. Where the agent is loaded with its plist unchanged, enroll runs `launchctl kickstart -k` (Decision 5). True when `devenv shell -- cargo test -p boomdrive enroll::` passes, proving "No agent loaded" and "The agent running".
- [ ] 2.4 The answer gains `config` and `service` (Decision 6). True when `devenv shell -- cargo test -p boomdrive enroll::` passes, proving "The answer".

## 3. The hosted implementation

- [ ] 3.1 `Table::enroll_command` gives the whole command, and the refusals of a key and of an ask for a forge token name it (Decision 7). True when `sluice exec main -- moon run boomdrive-hosted:test` passes, proving "A key refused" and "No key of the partition".
- [ ] 3.2 The enrollment's own scenarios hold with the new form. True when `sluice exec main -- moon run boomdrive-hosted:test` passes, proving "A partition enrolled", "A lapsed token", "The answer as JSON", "The token never shown" and "Nothing listening".

## 4. The words

- [ ] 4.1 The section of README on enrollment gives the new form and the three ways. `boomdrive enroll --help` names each flag. True when `sluice exec main -- moon run root:format` passes.
