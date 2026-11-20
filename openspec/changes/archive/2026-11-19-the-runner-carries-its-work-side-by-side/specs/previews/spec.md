## ADDED Requirements

### Requirement: A preview's command starts once every launch is under way

A preview's command, `sluice preview` or `sluice release`, SHALL start on a thread of its own once every launch its pull offers is under way, and SHALL wait on no other want's command; a done SHALL wait for a bring-up of its revision in flight. A preview answered from the report, under a lapsed lease, not kept or with no clone SHALL run no command (core.50, core.55).

#### Scenario: A preview runs while a launch runs

- **WHEN** a pull gives the preview `home/30`, then the offer `a`, whose launch is readied only once sluice has been asked for the preview
- **THEN** `home/30` is brought up while `a`'s launch runs, and `a` is launched

## REMOVED Requirements

### Requirement: A preview's command runs after every launch

**Reason**: A preview's command no longer waits for a launch to return, only for it to be under way, and none waits on a launch held for its turn, since none is held (core.50, core.55). "A preview's command starts once every launch is under way" holds what stands of it.

**Migration**: None.

### Requirement: Once the next pass is due, a pass runs no preview's command but its first

**Reason**: No preview is left to the next pass because the next is due: each preview's command runs on a thread of its own, waiting on no other (record 0002 as amended on 2026-11-19; core.50, core.55).

**Migration**: None.
