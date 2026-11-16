# crew's control plane

## MODIFIED Requirements

### Requirement: Each request for this host is a want under the lease number 1

crew's implementation SHALL give each request for this host, in `Request` order, as a want with the id `<label>/<Request>` and the lease number 1, with no expiry. A request naming a `Host` SHALL be for the host it names, and for no other. A launch naming no `Host`, and an act on one, SHALL be for this host as the requirements below give them. A request whose work has left crew's plan, a dropped launch and a moot priority SHALL be given only as the requirements below give them.

#### Scenario: Only this host's requests

- **WHEN** `requests.rec` holds launches naming `Host: host-a` and launches naming `Host: host-b`, and the host is `host-a`
- **THEN** the pull gives `host-a`'s alone, in `Request` order, each under lease 1

## ADDED Requirements

### Requirement: A priority the runner does not hold is given only while it is its launch's newest

A `priority` request that crew's implementation reads, its `Of` and its `Priority` each a whole number, and that neither the runner's report nor this host's branch answers, SHALL be given only while no later request that reads as a priority names the same `Of`, whatever `Host` either names. A later priority makes each earlier one moot, since each sets its launch's priority whole. A priority the runner holds SHALL be given as before (record 0002 as amended on 2026-11-16).

#### Scenario: Moot priorities the runner does not hold

- **WHEN** the runner's report holds launches 12 and 13 launched, naming this host, and requests 14, 15 and 17 set `Of: 12`'s priority to 3, 5 and 7 while request 16 sets `Of: 13`'s to 2, each naming this host, and the runner holds none of them
- **THEN** the pull gives `home/12`, `home/13`, `home/16` setting 2 and `home/17` setting 7, and nothing for 14 or 15

#### Scenario: A priority the runner holds

- **WHEN** the runner's report holds launch 12 launched, requests 14, 15 and 16 each set `Of: 12`'s priority, naming this host, and the report answers 14 `done`
- **THEN** the pull gives `home/12`, `home/14` and `home/16`, and nothing for 15

#### Scenario: Moot priorities naming no host

- **WHEN** the runner's report holds launch 20, which names no `Host`, launched, and requests 21 and 22 set `Of: 20`'s priority to 3 and 5, naming no `Host`
- **THEN** the pull gives `home/20` and `home/22` setting 5, and nothing for 21

### Requirement: A priority that cannot be read makes no earlier one moot

A request whose `Act` is `priority` but whose `Of` or `Priority` is no whole number SHALL make no earlier priority moot, and SHALL itself be given or left out as a request crew's implementation cannot read (record 0002 as amended on 2026-11-16).

#### Scenario: A later priority that cannot be read

- **WHEN** the runner's report holds launch 12 launched, request 14 sets `Of: 12`'s priority to 3, and request 15 names `Of: 12` with `Priority: high`, each naming this host
- **THEN** the pull gives `home/12`, `home/14` setting 3, and `home/15` as one the runner refuses, its words naming `high`
