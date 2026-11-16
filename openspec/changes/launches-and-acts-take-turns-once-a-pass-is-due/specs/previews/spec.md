# Previews

## MODIFIED Requirements

### Requirement: A preview's command runs after every launch

A preview's command, `sluice preview` or `sluice release`, SHALL run in the pass's second round, once every offer the pull gives is answered, a launch held for its turn among them, in the control plane's order, where a done is matched to what the report then holds. A preview answered from the report, one under a lapsed lease, a bring-up in a repository not kept, and a done whose repository has no clone SHALL be answered in the first. A pass's preview commands SHALL NOT have Flume's listing read again (core.50, core.55; record 0027).

#### Scenario: A launch ahead of a preview

- **WHEN** a pull gives the preview `home/30`, then the offer `a`
- **THEN** `a`'s `flume launch` is run before `sluice preview`

#### Scenario: A preview past due waits on no launch held for its turn

- **WHEN** the next control pass is due from a pass's start, and its pull gives the preview `home/30`, then offers `a` and `b`
- **THEN** `a` is launched, `b` is answered `held` for its turn, and `home/30` is brought up in the same pass
