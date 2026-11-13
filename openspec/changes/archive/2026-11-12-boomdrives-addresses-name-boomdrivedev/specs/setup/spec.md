## MODIFIED Requirements

### Requirement: Each program setup runs is found before it runs one

Before it writes anything or asks anything of a forge, setup SHALL refuse as a host to fix each of `crew`, `gh`, `herdr`, `hostname` and `launchctl` that is not on its `PATH`, naming where it comes from. It SHALL also refuse a host whose name, as `hostname -s` gives it, is not one crew takes, lower-case letters, digits and `-`, naming the command that sets one.

#### Scenario: crew missing

- **WHEN** setup runs with every answer, and no `crew` is on its `PATH`
- **THEN** it exits 2, naming `crew` and `github:boomdrivedev/boomdrive#crew`, and runs no `gh`, writes no file and loads no agent

#### Scenario: A host name crew refuses

- **WHEN** setup runs with every answer, and `hostname -s` says `Pats-MacBook-Air`
- **THEN** it exits 2, naming `Pats-MacBook-Air` and `sudo scutil --set HostName`, and writes nothing

### Requirement: A plugin missing or refused is named, and setup goes on

herdr's refusal of a link SHALL be named with herdr's words. Where no profile holds `share/herdr/plugins`, setup SHALL name the plugins as missing, with `github:boomdrivedev/boomdrive#herdr-plugins`. In either case setup SHALL go on.

#### Scenario: No plugin in the profile

- **WHEN** setup runs with every answer, and no profile holds `share/herdr/plugins`
- **THEN** no `herdr plugin link` is run, it exits 0, and its answer names the plugins as missing with `github:boomdrivedev/boomdrive#herdr-plugins`
