# crew-control-plane

## MODIFIED Requirements

### Requirement: The runner's branch holds crew's three files in crew's formats

crew's implementation SHALL write `runner.rec`, one `Runner` with `Host`, `Version` and `Since`, the first write's time, kept after. It SHALL write `answers.rec`, one `Answer` per request the report answers or a claim stands for. It SHALL write `places.rec`, one `Place` per launched request, then one `Run` per role, by `Address`. Each file SHALL open with crew's own descriptor, byte for byte as crew's `runner.py` holds it. Once the branch is written, or holds those files already, the answer to the report SHALL name taken each launch whose status `places.rec` holds, and no other (record 0030 as amended on 2026-11-09).

#### Scenario: The first report

- **WHEN** the runner first reports for `home` on host `host-a`
- **THEN** the state repository gains `home/runner/host-a`, holding `runner.rec` with `Host: host-a`, the runner's version and the time, and `home/main`'s tip is unchanged

#### Scenario: A place's status as Flume gave it

- **WHEN** the report carries a place's status as a fixture's text, beside a request answered `held` with no status
- **THEN** `places.rec`'s `Status` for its request is that text, byte for byte, crew's descriptors open each file byte for byte, and the report's answer names that launch's status taken and not the held request's
