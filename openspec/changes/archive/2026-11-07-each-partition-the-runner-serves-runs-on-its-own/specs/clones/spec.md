## ADDED Requirements

### Requirement: A clone's keep and a launch into it take turns on the clone's lock

The runner SHALL hold one lock for each repository it keeps, its address compared without case. The keeper SHALL hold it alone while it keeps or lets go of the repository, as SHALL a landing's keep. A launch into the clone SHALL hold it shared with other launches, from finding the clone made to the end of `flume launch`. No other command or read SHALL take it, and none SHALL be held across a pass (record 0026; core.14).

#### Scenario: A launch waits for its own repository's fetch alone

- **WHEN** the keeper holds `agentplot/flume`'s lock for its fetch, and a control pass offers one launch into `agentplot/flume` and one into `agentplot/sluice`
- **THEN** the launch into `agentplot/sluice` is made at once, and the one into `agentplot/flume` once the fetch is done

#### Scenario: Two parts launch into one clone at once

- **WHEN** `home`'s pass is launching into `agentplot/flume`, and `work`'s pass offers a launch into `agentplot/flume`
- **THEN** `work`'s launch is made without waiting for `home`'s to end

### Requirement: A launch finds its clone again under the lock

Holding a clone's lock, a control pass SHALL find the clone again before it launches into it, and SHALL leave an offer whose clone is no longer there unanswered, as one whose clone is not yet made (record 0021; core.14).

#### Scenario: A clone let go before the launch

- **WHEN** a control pass finds `agentplot/flume`'s clone made, and the keeper removes it, as dropped, before the pass takes its lock
- **THEN** the offer into `agentplot/flume` is left unanswered, and no `flume launch` is run for it

## MODIFIED Requirements

### Requirement: A repository is fetched when the runner is told of its landing

When the running runner is told that a kept repository has had a landing, it SHALL fetch that repository, settle its HEAD and keep its view at once, under its lock, without waiting for the next pass and without fetching any other repository. Told during a keeping pass, it SHALL keep the landing before the pass's next repository. A landing told of a repository the configuration does not name SHALL clone and fetch nothing, and SHALL be named.

#### Scenario: Told between passes

- **WHEN** the runner keeps `agentplot/flume` and `agentplot/sluice`, fetches every hour, and is told of a landing on `agentplot/flume` a minute after a pass, origin's `main` having moved
- **THEN** within seconds `agentplot/flume` is fetched and its view moved to the new `origin/main`, and `agentplot/sluice` is not fetched

#### Scenario: Told during a keeping pass

- **WHEN** a keeping pass keeps `agentplot/flume`, then `agentplot/sluice`, then `agentplot/timber`, and the runner is told of a landing on `agentplot/timber` while the pass keeps `agentplot/flume`
- **THEN** `agentplot/timber`'s landing is kept after `agentplot/flume` and before `agentplot/sluice`

#### Scenario: Told of a repository it does not keep

- **WHEN** the runner is told of a landing on `someone/else`, which its configuration does not name
- **THEN** nothing is cloned or fetched, and the notice is named as for a repository the configuration does not name
