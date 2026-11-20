## ADDED Requirements

### Requirement: Each pass ticks each inbox after its settle

After each settle, `boomdrive run` and `boomdrive run --once` SHALL run `millpond tick --inbox <name> --json` once for each inbox the configuration's `inboxes` names, in its order, each awaited on the settle's thread, and none where it names none. A pass that shares a settle under way SHALL share its ticks. millpond SHALL decide what a tick runs, and the runner SHALL read nothing of its answer but whether millpond gave one (record 0033; core.1, core.13).

#### Scenario: Each pass ticks each inbox

- **WHEN** the configuration names the inboxes `home` and `work`, and the runner makes three keeping passes with a stub `millpond`
- **THEN** each pass's settle is followed by `millpond tick --inbox home --json`, then `millpond tick --inbox work --json`, and an answered tick says nothing

#### Scenario: No inbox, no tick

- **WHEN** the configuration names no `inboxes`
- **THEN** the runner runs no `millpond` command

### Requirement: A tick is said only when it changes

Where an inbox's tick is refused, or gives no answer, the runner SHALL say so in one line only where that inbox's tick before was answered or it is its first, and SHALL say so again at its first tick answered after one that was not. No tick SHALL stop the runner or fail a pass, and none SHALL be carried to a control plane or kept (record 0033; core.11, core.23).

#### Scenario: A refused tick said once until millpond ticks again

- **WHEN** millpond refuses `home`'s ticks with the words ``this host keeps no inbox named `home` `` at two passes, then answers the third
- **THEN** the first says ``boomdrive: millpond refused the tick of the inbox home: this host keeps no inbox named `home` ``, the second says nothing, and the third says `boomdrive: millpond ticks the inbox home again`

#### Scenario: A tick with no answer

- **WHEN** `millpond` cannot be run for two ticks of `home` in a row
- **THEN** the first says `boomdrive: millpond gave no answer to the tick of the inbox home: ` and why, and the second says nothing
