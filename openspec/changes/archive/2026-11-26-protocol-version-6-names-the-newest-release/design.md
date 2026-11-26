# Design

## Context

The person ruled on 2026-11-26 that the cloud leads the runner's protocol (Boom: `the-cloud-leads-the-runners-protocol`). The cloud keeps each older version while an enrolled runner speaks it, and it tells that runner that a newer release exists. Versions 2 to 5 have no message for this, and a version that a release spoke never changes (core.42). The runner's gate refuses a runner that offers a version the cloud's `main` does not speak (`boomdrive-hosted:cloud-protocol`). The design relies on these rules: core.1, the runner decides nothing. core.11, a tool's answer as it gave it. core.23, one line and no alarm. core.40, a line's form. core.42, the protocol.

## Decisions

### Decision 1: version 6 is version 5 and one kind

`protocol/v6/` is version 5 whole, with three changes: each `$id` is under `…/protocol/v6/`, each message names `protocol_version` 6, and `to-runner.schema.json` takes one more kind, `newest_release`. Its body is `{"version": "<major>.<minor>.<patch>"}`, closed. The version is three numbers with no leading zero, as git-cliff makes them and as the tag `v<version>` names them. The control plane sends it on the socket after its `pulled` answers the hello, where the hello's `runner_version` is older. It may send it again when a newer release is made. Each example of version 5 is an example of version 6, with `protocol_version` 6, and `to-runner/newest_release.json` is new. The test `files` holds version 6 to version 5 but for this.

A new kind is used, and not a field of `pulled`. `pulled` comes whenever the wants change, and a field there would be sent again each time, or be `null` most of the time. A kind of its own is sent once, when the control plane has something to say.

### Decision 2: the types are version 6's, and read version 5 too

Each message of version 5 is a message of version 6 but for its `protocol_version`. So the runner's types move to `protocol::v6`, and `ToRunner` gains `NewestRelease`. `protocol::v5` keeps its files and constants, as `v4` does. A message is checked against the version spoken before it is read. So under version 5 a `newest_release` fails the check and is answered with `message_refused`, and the types never see it.

### Decision 3: `spoken.json`'s mark chooses the version

`protocol::Version` holds a version's number, subprotocol, address, files and checks. `protocol::V5` and `protocol::V6` are the two that the code can speak. `protocol::spoken()` reads `spoken.json`, which the crate includes, and gives the newest version it marks spoken. Each check, each context that a key signs, each message's number and each upgrade's subprotocol take a version. Enrollment, a forge token's ask and the copies take `spoken()`. A connection takes the version of its `Dial`, which `Dial::new` sets to `spoken()`. A test that sets a `Dial`'s version to `V6` runs the code that a host runs once the mark moves. This is no mode for tests: it is the same field the host uses.

The test `spoken` holds the file to one version marked spoken, which the code can speak. Each test that named version 5 now names the version spoken. The whole suite passes with either mark. So the change that follows, when the cloud's `main` speaks version 6, only marks version 6 spoken.

### Decision 4: the runner says the newest release, and judges nothing

On `newest_release`, the connection says one line: `<label>: this runner is <runner_version>, and the newest release is <version>`. `<runner_version>` is the hello's, as the connection sent it. It says each release one time on each connection, and says it again on a new connection. It compares no version and acts on none (core.1). It raises no alarm (core.23). The runner has no `boomdrive status` command, so the log is the one place it says it. The person sees the runner as out of date on the host page, which is the cloud's.

## Risks

- The runner says the release that the control plane names. A wrong name from the control plane is said as it came. The schema holds it to three numbers, so it is never text of another form.
- Until the cloud's `main` speaks version 6, no runner says a newer release. That is the cloud's lead.
