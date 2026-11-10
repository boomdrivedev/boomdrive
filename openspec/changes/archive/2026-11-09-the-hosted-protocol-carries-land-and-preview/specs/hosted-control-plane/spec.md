## MODIFIED Requirements

### Requirement: What goes up is version 2's copy, checked before it goes

Each place's status, sluice entry, preview's answer and listing, and host part SHALL go up cut to version 2's copy by walking its schema, and checked against it. A copy that fails outside a place's ends SHALL go up `null`, or the host part `missing`, with one line naming the launch or the preview and the schema's path, never a value. Every message SHALL be checked against `from-runner.schema.json` before it is sent (core.11, core.27, core.42; record 0037 as amended on 2026-11-09).

#### Scenario: Flume's places cut

- **WHEN** each place of Flume 0.21.0's and Flume 0.4.1's fixtures is carried in a report
- **THEN** its copy passes `place-status.schema.json` of version 2 and holds no `last_words`, `screen`, `printed`, task `text`, `readiness` or `repository`; each end of Flume 0.21.0's goes up in a form version 2 names, with its `key` and `delivery`; and each end of Flume 0.4.1's, which Flume printed with no `key`, is left out

#### Scenario: A value Flume adds

- **WHEN** a place's status names a stage's state Flume 0.21.0 does not have, in its `stages`
- **THEN** that launch's status goes up `null`, the report passes and is sent, and one line names the launch and the path, and not the value

#### Scenario: sluice's previews cut

- **WHEN** sluice 0.74.3's answer of a preview brought up and each row of its listing are cut as a preview's copy, and its answers of a release as a release's copy
- **THEN** each copy passes its definition of version 2, keeps the `state` or `released` sluice printed, and no preview's copy holds `checkout`, `names`, `ports`, `exit` or `output`

#### Scenario: A preview's copy that fails

- **WHEN** a preview's status, as the runner carries it, holds a `made` that is a number
- **THEN** that preview's `status` goes up `null` and its `copy` as before, the report passes and is sent, and one line names the preview and the path, and not the value, once while it fails

## ADDED Requirements

### Requirement: What the runner answered of each preview goes up

A part's report SHALL carry each of its previews under its id without the partition's prefix, as the runner holds it, but its text, which goes up as its `copy`, cut as a release's copy where the preview is done and as a preview's where it is not, and its status, which goes up as `{copy, at, listed}`. No preview SHALL change which statuses the report's answer names taken (core.27, core.54, core.55; record 0037 as amended on 2026-11-09).

#### Scenario: Previews go up as their copies

- **WHEN** `home`'s part's report holds a preview brought up, with sluice's answer and its listing's row as the runner carries them, its done, answered with sluice's release, and a preview refused with sluice's words
- **THEN** the report passes and is sent; each preview goes up under its id without `home/`; the first's `copy` and `status.copy` hold sluice's `name`, `branch`, `revision`, `state`, `made` and `addresses`; the done's `copy` holds `preview` and `released`; and the refused one's `copy` and `status` are `null`

### Requirement: A control plane's land is given to the runner as any act

A `pulled` holding an act whose `what` is `land` SHALL pass version 2's check and be given to the runner as an act, its id and its launch under the partition's prefix. The hosted implementation SHALL read nothing of a land, the runner carrying it out as it carries crew's (core.2, core.26, core.53; record 0037 as amended on 2026-11-09).

#### Scenario: A land carried

- **WHEN** `home`'s control plane offers launch `l-1` and asks act `land-1` on it, a land
- **THEN** `home`'s part's pull gives act `home/land-1` on `home/l-1`, a land, and the runner sent no `message_refused`

### Requirement: A control plane's preview is given to the runner under the partition's prefix

A `pulled` holding a preview SHALL pass version 2's check and be given to the runner, its id under the partition's prefix and every other field as given, in the control plane's order. A preview of a repository the partition is not served SHALL be withheld, one line naming it while it is offered. The hosted implementation SHALL read nothing else of a preview (core.2, core.26, core.55; record 0037 as amended on 2026-11-09).

#### Scenario: A preview carried

- **WHEN** `home`'s control plane asks a preview `preview-1` of `main` in `example/app` at a revision, then `preview-2`, its done
- **THEN** `home`'s part's pull gives the previews `home/preview-1` and `home/preview-2`, in that order, each field as given, and the runner sent no `message_refused`

#### Scenario: A preview of a revision that is not a commit

- **WHEN** `home`'s control plane asks a preview of `example/app` at the revision `R1`
- **THEN** `home`'s part's pull gives it as given, the runner sent no `message_refused`, and it is sluice's to answer

#### Scenario: A preview of a repository not served

- **WHEN** `home`'s control plane asks, at two pulls, a preview of `example/service`, which `home` is not served
- **THEN** neither pull gives it, and one line names the preview, `home` and `example/service`

### Requirement: A report handed back holds its previews

A report the control plane hands back in a `pulled` SHALL give the runner each preview it holds, under the partition's prefix, with its lease, repository, revision, done, outcome, words and time, its copy's JSON as its text where it has one, and no status, so a done after the runner lost its report file puts away the preview by the name sluice gave it (core.10, core.55; record 0037 as amended on 2026-11-09).

#### Scenario: A report taken back holds its previews

- **WHEN** `home`'s control plane holds a report whose preview was brought up, and hands it back in its `pulled`
- **THEN** the part's pull gives a report holding that preview under `home/`, with its lease, repository, revision, done, outcome, words and time as held, its text a JSON object whose `name` is its copy's, and no status
