## MODIFIED Requirements

### Requirement: A pull gives each partition's wants under its own ids

Each partition SHALL be a part of its own. A part's pull SHALL give the wants of its connection once it has taken a `pulled`, waiting up to 10 seconds for that connection alone, each want's id as `<label>/<id>`. A part not read SHALL be unread, its words not repeating its label, while every other part pulls on its own. A `pulled` that comes SHALL wake the part whose connection took it, and no other (record 0026; core.26).

#### Scenario: Ids under their partition

- **WHEN** `home`'s control plane offers launch `l-1` after `l-0`, and `work`'s offers launch `l-1`
- **THEN** `home`'s part gives `home/l-1`, after `home/l-0`, and `work`'s part gives `work/l-1`

#### Scenario: One partition unreachable

- **WHEN** `work`'s control plane cannot be reached and `home`'s offers a launch
- **THEN** `home`'s part gives its offer without waiting on `work`'s connection, and `work`'s part is unread

#### Scenario: Woken

- **WHEN** `home`'s control plane sends a `pulled` between passes
- **THEN** the wake the opener was handed for `home` is called, and the one handed for `work` is not

### Requirement: A report goes to each partition, answered by its id

A part's report SHALL send its connection the part's share as `report` under a new message id, and wait up to 10 seconds for its `reported`, waiting on no other part's connection. It SHALL be unsent where its connection is not live or does not answer. After a reconnect, a part SHALL send its last `report` no `reported` answered again under the same id, after its hello, unless a newer one was sent (core.21; boomdrive-cloud core.38).

#### Scenario: A lease refused

- **WHEN** `home`'s control plane answers a report refusing `l-1`
- **THEN** `home`'s part's report answer refuses `home/l-1` with the fake's words

#### Scenario: A report resent

- **WHEN** `home`'s connection drops after a `report` and before its `reported`, then connects again
- **THEN** after the `hello`, the same `report` is sent again under the same message id

#### Scenario: A report unanswered by another partition

- **WHEN** `work`'s control plane takes a `report` and never answers it, and `home`'s answers at once
- **THEN** `home`'s part's report is answered at once, and `work`'s part's report is unsent after 10 seconds
