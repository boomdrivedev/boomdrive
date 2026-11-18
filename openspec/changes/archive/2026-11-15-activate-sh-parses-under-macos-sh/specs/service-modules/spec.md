## ADDED Requirements

### Requirement: A Mac's /bin/sh reads every script the modules and packages give it

Every script under `nix/modules` and `nix/packages`, which a Mac runs by its `/bin/sh`, `activate.sh`, which the nix-darwin module's activation runs as `/bin/sh activate.sh`, and `launcher.sh`, the launcher's script, run by its `#!/bin/sh`, SHALL be one macOS's `/bin/sh`, bash 3.2.57 in POSIX mode, reads. The gate SHALL read each with that bash, as the tree pins it, on every host that runs the gate, and SHALL fail for a script it cannot read, naming the script and bash's words.

#### Scenario: Each script read by macOS's /bin/sh

- **WHEN** `nix:mac-sh` reads each script under `nix/modules` and `nix/packages` with bash 3.2.57 in POSIX mode and `-n`
- **THEN** each is read, none is run, and the task passes

#### Scenario: A script macOS's /bin/sh cannot read

- **WHEN** a script under `nix/modules` holds a `case` whose pattern's bare `)` stands inside `$( )`, as `activate.sh` did at `23e41810`
- **THEN** `nix:mac-sh` fails, naming the script and the line bash 3.2.57 names
