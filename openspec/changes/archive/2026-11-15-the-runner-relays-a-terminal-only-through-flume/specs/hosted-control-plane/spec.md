## MODIFIED Requirements

### Requirement: A terminal the control plane opens is closed at once

Every `terminal_open` SHALL be answered with `terminal_closed`, its words saying the runner has no end-to-end exchange for a terminal yet and relays none in the clear, and no key made. Messages naming a terminal never opened SHALL be dropped, and a pull SHALL give no terminal (core.30; record 0004 as amended on 2026-11-15).

#### Scenario: A terminal opened

- **WHEN** the fake control plane sends `terminal_open` for a launch the runner holds
- **THEN** the runner answers `terminal_closed` for it, with words naming its end-to-end exchange and not naming Flume, and the pull gives no terminal
