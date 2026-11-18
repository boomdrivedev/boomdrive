# Design

## Context

See proposal.md for why. The decision is record 0048, accepted by the user on 2026-11-16, with core.19 as amended with it. Record 0052's amendment of 2026-11-18, and core.42's and core.10's details, are written with this change. No area page binds the paths this change touches, so `docs/architecture/core.md` binds every one. The change relies on these rules:

- core.1, core.19: the runner takes work from its one control plane, and from an observer nothing;
- core.2: only the hosted implementation names the protocol, and `planes.rs` alone names an implementation;
- core.3, core.28: the observer's connection is outgoing, to a loopback address, so nothing of it leaves the host;
- core.10: a report to an observer names no lease, as core.10's details now allow (record 0048);
- core.11, core.27: what goes to an observer is version 3's copy of each tool's answer, and no line names a value;
- core.17: the local layer sets only the keys the runner lists, and `[observer]` is none of them;
- core.21 to core.23: an observer's connection is kept, tried again and said as a partition's is;
- core.26: an observer's connection names only its partition's repositories and the part's own places;
- core.31, core.32: an observer's partition is enrolled with a key made on the host and kept in its credential store;
- core.42: version 3 is unreleased, so it gains the observing connection, and every form stays closed;
- core.49: no part's pass waits on an observer;
- core.54: no end is confirmed on an observer's answer.

It departs from no rule.

### What stands

- **This branch** is cut from `bolt/a-hosted-control-plane-can-hold-a-waiting-place` at `f0e481e1`, which holds `main` and the change `the-hosted-protocol-carries-a-hold`, archived.
  - Version 3 is under `crates/boomdrive-hosted/protocol/v3/`. It is version 2 whole but for a hold, its `$id`s under `boomdrivedev/boomdrive`.
  - No release speaks version 3: the newest tag is `v0.15.1`, which speaks version 2.
  - `tests/files.rs`'s `version_3_is_version_2_but_for_its_hold` holds version 3 to version 2 but for the hold.
  - That change's design names the observing connection as a follow-up that "a change that builds it on this bolt before the bolt lands adds … to version 3".
- **The hosted implementation** opens a part for each partition of `[control_plane.hosted]`.
  - Each part is a `Hosted` over one `Connection`, on one runtime its parts share.
  - `Connection` is handed the hello and the report and sends them from its runtime. It keeps the newest report and sends it again after every reconnect until it is answered.
  - A `Connection` takes `pulled`, `reported`, `message_refused` and `terminal_open`, which it closes at once, and wakes the runner at each `pulled`.
  - `Partition::upward` cuts each launch's status and entry to version 3's copies, saying once while one fails.
- **A partition's key** is kept under `KeyName`: `<label>@<host>` in the keychain, `boomdrive.<label>.<host>` as a systemd credential. A label is lowercase letters, digits and `-`, so it holds no `.`.
- **The runner** keeps each part's share of its report, and a control pass builds the part's hello, pulls, answers its wants, carries its statuses, saves and sends its report, and confirms the ends taken (`carry.rs`). `run.rs` opens the control plane's parts at start, and carries each on a thread of its own in `keep`.
- **The configuration** names `[control_plane]`, read at every pass and taken at start, and `[forge]`. Its file is refused for a key it does not name. The local layer sets each drive's `repositories` and the keys a control plane's implementation lists.
- **boomdrive-cloud's record 0069**, accepted by the user on 2026-11-16, has a local stage take "the observing hello and reports of the host's own runner". It seeds "the enrollment the host's runner takes for its observer", and offers an observed host nothing. Its pin waits on the version that carries an observing hello.

## Goals / Non-Goals

**Goals:**

- A host's runner reports each observed part's work to a local observer: its hello, saying it observes, and the places it follows, with no lease.
- Nothing an observer sends reaches the host's work, and an observer's reach holds up nothing of the control plane the runner takes work from.
- The observing connection is in version 3, closed and checked as every message is.

**Non-Goals:**

- **The cloud's side.** boomdrive-cloud takes version 3 by a change of its own.
- **An observer off the host.** Record 0048, point 2.
- **More than one observer.** One `[observer]` per runner (Decision 3).
- **Acts, commands, inboxes and previews to an observer.** Decision 2.
- **A declared module option for `[observer]`.** The module's `settings` takes it through its freeform type (Decision 3).

## Decisions

### 1. Version 3 carries an observing hello and an observed report

Version 3 gains these, and nothing else of version 3 changes:

- **`hello.schema.json`'s `$defs` gains `observing_hello`:**

  ```json
  {
    "description": "…",
    "type": "object",
    "additionalProperties": false,
    "required": ["observing", "interface_version", "runner_version", "tool_versions", "repositories"],
    "properties": {
      "observing": { "description": "…", "const": true },
      "interface_version": "<hello's>",
      "runner_version": "<hello's>",
      "tool_versions": "<hello's>",
      "repositories": "<hello's>"
    }
  }
  ```

  Each of its four other properties is the hello's, written out whole, its descriptions its own. It names no `capacity`, so it offers no room, and no `leases`, since an observer gives none (record 0048, points 1 and 4).
- **`report.schema.json`'s `$defs` gains `observed_report` and `observed_launch`:**

  ```json
  "observed_report": {
    "description": "…",
    "type": "object",
    "additionalProperties": false,
    "required": ["observing", "launches"],
    "properties": {
      "observing": { "description": "…", "const": true },
      "launches": {
        "type": "object",
        "propertyNames": { "$ref": "shared.schema.json#/$defs/id" },
        "additionalProperties": { "$ref": "#/$defs/observed_launch" }
      }
    }
  },
  "observed_launch": {
    "description": "…",
    "type": "object",
    "additionalProperties": false,
    "required": ["answer", "status", "entry"],
    "properties": { "answer": "<launched's>", "status": "<launched's>", "entry": "<launched's>" }
  }
  ```

  `observed_launch` is `launched` without its `lease`.
- **`from-runner.schema.json`'s `hello` and `report` kinds** each take either form:

  ```json
  "hello": { "oneOf": [
    { "$ref": "hello.schema.json#/$defs/hello" },
    { "$ref": "hello.schema.json#/$defs/observing_hello" }
  ] }
  ```

  and the same for `report`, with `report` and `observed_report`.
- **Descriptions.** `observing` describes that the connection observes, and that a control plane offers nothing on it. `shared.schema.json`'s `message_refused` describes either end's answer to a message that failed its check, and the runner's to one an observing connection takes nothing from.

Each form is closed. `observing` is required in the observing forms and named in neither the hello nor the report, so no message passes as two forms, and a `oneOf` of two never finds both.

Version 3's examples gain three:

- `from-runner/hello-observing.json`: `hello.json`'s hello with `observing` true, and its `capacity` and `leases` left out;
- `from-runner/report-observed.json`: `report-a-live-agent-with-its-pane.json`'s launches, each without its lease, under `observing` true;
- `to-runner/pulled-answering-an-observing-hello.json`: a `pulled` of no want and a `null` report, id `plane:14`.

Alternatives:

- **A version 4 for the observing connection.** Not taken. Version 3 is unreleased (core.42), and the cloud would pin a version nothing has spoken beside another. Record 0052 is amended instead.
- **New kinds, `observing_hello` and `observed_report`.** Not taken. Record 0048, point 4: "the hello says the connection observes". A hello and a report keep their kinds, and the envelope's kinds stay version 2's.
- **The hello with `observing: false` on every working connection.** Not taken. Every working hello would change, and the test of version 3 against version 2 would set aside every hello.
- **A report's `lease` made `null`-able.** Not taken. A working report could then name no lease, which only the connection's state could refuse, and a schema checks no state.

### 2. An observed report holds the part's launches alone

The observed report holds each launch of the part's share, by its id as the runner holds it, with its answer, its status and its entry as version 3's copies. Ids are neither prefixed nor stripped. The runner reads no id (the control-plane-interface spec), and an observer reads into none. So under crew's control plane an observer of `home` is sent `home/1`, and under a hosted one `home/l-1`.

Each status and entry is cut as `Partition::upward` cuts it: `null` where its copy fails, and an end of a form version 3 does not name by its delivery alone. Each is said once while it lasts (core.11, core.27). The cut of one launch moves out of `Partition::upward` into a function both use, so the two never differ.

Alternatives:

- **The whole share, acts, commands, inboxes and previews, each with its lease left out.** Record 0048, point 3, says "its reports are the reports that partition's control plane gets". The brief narrows it to "its reports of the places it follows", and the cloud's local stage shows "that host and its places" (its record 0069). Not taken: each of the four would need a lease-less copy of its definition, and each holds answers to wants the observer never gave. This is cheap to change until a release speaks version 3, and a later version may add them.

### 3. The managed layer names one `[observer]`, read as a hosted table at a loopback origin

```toml
[observer]
url = "http://localhost:8787"

[[observer.partitions]]
label = "home"
repositories = ["example/app"]
```

- **Its keys are the hosted table's**: `url`, and `partitions`, each a `label` and its `repositories`. They are read by the hosted implementation's own reading, `Table::read_observer`. It refuses as `Table::read` refuses, naming `[observer]`, and refuses a `url` that is not `http://` or `https://` to a loopback address: `localhost`, `127.0.0.0/8` or `::1` (record 0048, point 2). A credential's key is refused, saying the key is made by `boomdrive enroll --observer`.
- **A partition's `label` names the part whose work it observes**, a part of the runner's own control plane. Under crew, a part is a drive, so its label is the drive's.
- **Its `repositories` are those its observing hello names**, of those the host keeps, as a hosted partition's are (core.26). Under crew a drive's repositories may be of several owners. So nothing but the partition's own names what an observer of one drive is told.
- **`config.rs` holds `[observer]` as written**, `Config::observer`, and has `planes::read_observer` read it at every read of the file, as `[forge]` is read. So a broken `[observer]` makes the file unreadable: the runner refuses to start, and a pass carries nothing.
- **It is refused where the file names no `[control_plane]`**, since it observes the work of that control plane's parts.
- **The local layer may not set it** (core.17). An observer is the host's own, and a control plane writes the local layer.
- **The runner takes it when it starts**, as it takes `[control_plane]`.
- **The flake's modules** write it through `services.boomdrive.settings.observer`, which the settings' freeform type takes as the file names it (core.46). No module option is declared for it.

Alternatives:

- **`[[observers]]`, several.** Not taken. Record 0048 and core.19 name "an observer". Two local stages side by side are each enrolled again when pointed at, at no cost to the host.
- **An observer naming a `partition` alone, with no `repositories`.** Not taken. Its hello would name every repository the host keeps, other accounts' among them, against core.26.
- **`observers` inside `[control_plane]`.** Not taken. `[control_plane]` holds `type`, `poll_interval` and its implementation's table, all of the one control plane the runner takes work from (core.19).

### 4. An observer's partition is enrolled as a hosted partition is, under a key of its own

`boomdrive enroll --observer <label>` reads `[observer]` with the local layer over the managed one, as `enroll` does. It reaches the hosted implementation's enrollment through `planes::enroll_observer` alone (core.2), whatever `[control_plane]`'s `type`.

- It enrolls exactly as a hosted partition is enrolled: the token from standard input, a key made in memory, one `POST runner/enroll` under the observer's `url`, the token its bearer, its body an `enroll` of version 3 (records 0006 and 0018).
- The key is kept once `enrolled` answers, under `KeyName` with the label `observer.<label>`: `observer.<label>@<host>` in the keychain, and `boomdrive.observer.<label>.<host>` as a systemd credential. A label holds no `.`, so no hosted partition's key can share the name, even where a test's control plane and an observer are both on loopback.
- It answers `observer <label>: enrolled as host <host>`, or with `--json` the partition's answer, `{"enrolled":{"partition":"<label>","host":"<host>"}}`.
- It is refused with exit 2 where the file names no `[observer]`, or no partition `<label>` in it, before the token is asked for.
- `hosted::table::Table` gains its role, a control plane's or an observer's. The role chooses the key's label, the words of a refusal (`[observer]` or `[control_plane.hosted]`), and the command a refused key names: `boomdrive enroll --observer <label>`, or `boomdrive enroll <label>`.

Alternatives:

- **The observer under the partition's own key.** Not taken. Under crew there is no hosted partition and no key. Under a hosted control plane, one key in two control planes would let either be refused for the other's enrollment.
- **The port in the key's name.** Not taken. One observer per runner (Decision 3) needs no port to tell two apart.

### 5. The interface names an `Observer`, which gives nothing back

`boomdrive-control` gains:

```rust
/// A part's observer (record 0048; core.19).
pub trait Observer {
    /// Takes a control pass's hello and the part's share of its report,
    /// returning at once and waiting on nothing.
    fn observe(&mut self, hello: &Hello, report: &Report);
}

/// An observer, with the label of the part it observes.
pub struct Observed {
    pub label: String,
    pub observer: Box<dyn Observer + Send>,
}
```

Nothing returns from an observer, and it is handed no wake. So nothing it is sent can reach a pull, a want, an answer or a timer of the runner's. Refusing work is then the observer's connection's alone (Decision 6). `boomdrive-control::fake` gains `Observing`, a fake observer that records each hello and report it is handed, shared with the test that holds it.

`planes.rs` gains `observers(table, file, poll_interval, say_for)`, which opens the hosted implementation's observers. It also gains `read_observer` and `enroll_observer`, beside the hosted entry, the one list that names an implementation (core.2).

Alternatives:

- **An observer as a `ControlPlane` whose wants the runner drops.** Not taken. It would give wants to the runner's code and rely on the runner dropping each. `Observer` gives none.

### 6. The hosted observer: one connection per partition, observing

`crates/boomdrive-hosted/src/observer.rs` holds `open`, which reads `[observer]` and gives an `Observed` for each partition, in its order. Each holds an `Observing` over one `Connection` dialed in the observing role. They run on one runtime of their own, apart from the hosted parts', which the last of them stops once dropped. A host with no credential store says so once for each partition and opens nonetheless, as `Hosted` does.

`Observing::observe(hello, share)`:

- **The hello.** It hands its connection `v3::ObservingHello`: the hello's interface and runner versions and tool versions, and the repositories its partition names of those the hello names kept. The connection sends it first at every connection and again when it changes. It is checked against `observing_hello` before it is handed.
- **The report.** It hands its connection `v3::ObservedReport` of the share's launches, cut as Decision 2 says, checked against `observed_report`. The connection sends the newest at once on a live connection, and after the hello of the next one, replacing any it was not answered for.
- **Answers.** It takes the connection's answers and drops them, waiting on none, so none piles up. A `reported` ends the resending of its report. Its let-go and refused ids are read as nothing, and no status is taken, so no end is confirmed on it (core.54).

`Connection` in the observing role:

- its upgrade and its proof are a partition's: `boomdrive.v3`, the key proved at every attempt (core.31);
- the hello and the report it is handed are checked against the observing forms;
- a `pulled` holding no want and a `null` report is the observer's answer: it is reached, and the backoff's cap goes back to its least;
- a `pulled` holding a want or a report is also the observer's answer. It is answered with `message_refused`, its words `this connection observes, and the runner takes no work from it`. One line says `<label>: the observer sent pulled <id>, which an observer gives none of, so it is refused`, and nothing of it is kept;
- each `terminal_open`, `terminal_input`, `terminal_resume` and `terminal_close` is answered and said the same way. No `terminal_closed` is sent and no key is made;
- its wake is none, so no message of an observer's wakes the runner;
- its lines name the observer where a partition's name the control plane: `<label>: the observer is unreachable: <why>`, `<label>: the observer is reached`, and a key refused naming `boomdrive enroll --observer <label>`.

Every other message is taken as a partition's connection takes it: a failing one refused, an id unread closing the connection, an enrollment's answer refused.

Alternatives:

- **Closing the connection on any work from an observer.** Not taken. It would be tried again on the backoff, and would refuse the same `pulled` again at every connection. A refusal per message is said once, and the connection keeps carrying reports.
- **A `terminal_closed` for a `terminal_open`, as a partition does.** Not taken. That answers a terminal as one the runner considered; an observer's is refused as work (record 0048, point 1).

### 7. The runner hands each pass to its part's observer

- **`Runner::start`** opens the observers beside the parts, through `planes::observers`, where `[observer]` is named and the control plane opened. Under `run --once` none is handed a hello. A `Connection` is first attempted once a hello is handed it, so no connection is made and no key read.
- **`keep`** gives each part the observer whose label is the part's, and says once, at its start, each observer whose label no part has: `boomdrive: observer <label>: no part of the control plane is labelled <label>, so it observes nothing`.
- **`Carrier::observed_pass`** takes the part's observer, where it has one, and `Carrier::pass` is it with none, so the tests that build a `Carrier` outside this change's paths keep their form. Once the pass has sent its report and confirmed what it confirms, it hands the observer its hello and the part's share as it stands, whatever its control plane answered. A pass whose pull could not be read hands nothing, since it carried nothing. A pass that could not save its share (core.25) hands the share as it stands.
- **`Runner::observe_with`**, beside `carry_for`, puts a test's observers in place of those `[observer]` opens.
- **Observers that could not be opened**, their runtime not made, are said once at `keep`'s start, `boomdrive: observer: <why>: nothing is observed`, and the parts are carried as before.

Alternatives:

- **The observer handed the report before its control plane's send.** Not taken. The observer would show answers its control plane may refuse, and the send would follow the observer's handing.
- **`Carrier::pass` itself taking the observer.** Not taken. `tests/fakes/mod.rs` and `tests/pass.rs` call it and build a `Carrier`, and neither is among this change's paths.

### 8. Nothing waits on an observer

`observe` returns at once. The connection's runtime sends what it is handed, and no call of the observer's waits on a socket or an answer. Version 3's validators an observer's handing reads are built when the observers open (`check::prepare_observing`), so no handing builds one. So an observer unreachable, slow or deaf holds up no pass, launch or report (record 0048, point 5; core.49). Its connection is tried again on the partition's backoff, its cap stopping at `[control_plane]`'s `poll_interval` (core.22). Dropping the observers at the runner's stop closes each socket, waiting at most a second, as the hosted parts' drop does.

### 9. A test holds version 3 to version 2 but for the hold and the observing connection

`tests/files.rs`'s `version_3_is_version_2_but_for_its_hold` becomes `version_3_is_version_2_but_for_its_hold_and_observing`. Beside what it sets aside now, it reads each file of version 3 with these set aside:

- `hello.schema.json`'s `observing_hello`, and `report.schema.json`'s `observed_report` and `observed_launch`;
- `from-runner.schema.json`'s `hello` and `report` kinds, each read as the `$ref` of its first branch, the working form.

It then asserts:

- `observing_hello`, descriptions aside, is `hello` with `capacity` and `leases` taken out of `required` and `properties`, and `observing` put first in each, `{"const": true}`;
- `observed_launch`, descriptions aside, is `launched` with `lease` taken out of both;
- `observed_report`, descriptions aside, is Decision 1's;
- each kind's second branch is the `$ref` of its observing form.

Version 3's examples are version 2's names, `to-runner/pulled-with-a-hold.json`, and Decision 1's three, exactly. A failure names the file and the first differing JSON pointer, never a value (core.11).

### 10. What the runner says of it

- **README's hosted section** gains a paragraph on an observer:
  - its table, with its example, `crates/boomdrive/tests/examples/observer.toml`, a file of its own that `tests/examples.rs` reads (record 0049);
  - its enrollment, `boomdrive enroll --observer <label>`;
  - that the runner sends it each observed part's hello and places with no lease, takes nothing from it and waits on nothing of it;
  - its lines.
- **The protocol's README** describes the observing hello and the observed report in its messages table and a paragraph. It drops an observing connection from what version 3 has no message for. *Version 2* says version 3 is version 2 but for its subprotocol, contexts, `$id`s, the hold and the observing connection.

## Who owns what

| piece | owner | cited by |
|---|---|---|
| version 3's observing forms, the observer's connection and its reading of `[observer]` | the hosted implementation, `boomdrive-hosted` (core.2, core.42; records 0048 and 0052) | boomdrive-cloud, which copies them |
| `Observer` and `Observed` | the interface, `boomdrive-control` (core.2) | the runner, the hosted implementation |
| which part an observer observes and when it is handed a pass | the runner (core.19) | the hosted observer |
| offering an observing connection nothing, and showing an observed host | boomdrive-cloud's local stage (its record 0069) | the runner, which refuses what it offers |
| naming an observer on a host and enrolling it | the host's person, through its managed layer | the runner |

## Named follow-ups

- **boomdrivedev/boomdrive-cloud: take version 3.** Its local stage takes the observing hello and the observed report, answers an observing hello with a `pulled` of no want and a `null` report, and offers nothing on that connection (its record 0069). It seeds an enrollment for `observer.<label>`'s key. Until it does, an observer named on a host is said unreachable once, naming `boomdrive.v3`, and tried again on its backoff.
- **A host observed.** The host's configuration names `[observer]` and the host enrolls it with the local stage's token. That is the host's own, at its person's asking, and no change of this repository's.

Each is told to the orchestrator.

## Risks / Trade-offs

- **Version 3 grows on an unlanded bolt.** → A release follows every push to `main` (record 0024), so version 3 freezes when the bolt lands. A field the observer wants next is then a version 4.
- **A runner holds a second socket for each observed part.** → It is to loopback, on a runtime of its own, and no pass waits on it (Decision 8).
- **A loopback origin can be any process on the host.** → An observer gets only what a hosted partition's control plane gets of that part, cut to version 3's copies (core.27), and nothing leaves the host. Its enrollment needs a token a person gave it.
- **The observer's report is the part's launches alone.** → The cloud's local stage shows places (its record 0069). The rest is a later version's, cheaply, if asked (Decision 2).

## Migration Plan

Nothing on a host moves. A host that names no `[observer]` runs as before, and no field of the report file changes, so the release's notes name no move by hand (core.36). A host names `[observer]` and enrolls it to be observed.

## Open Questions

None.

## Task notes

- **1.1**
  - The four schema files change as Decision 1 gives them. jsonfmt keeps their layout.
  - The three examples are written as Decision 1 lists them.
  - `tests/interface.rs` gains `an_observing_hello_and_an_observed_report_are_of_version_3`. It checks each of the three examples against its direction's envelope of version 3, each passing. Against version 3's `from-runner.schema.json` it checks each of these, each refused:
    - `hello-observing.json` with `capacity` added, as `hello.json` holds it;
    - `hello-observing.json` with `leases: {}` added;
    - `hello.json` with `observing: true` added;
    - `report-observed.json` with a launch's `lease` added, as `report-a-live-agent-with-its-pane.json` holds it;
    - `report-observed.json` with `acts: {}` added;
    - `report.json` with `observing: true` added.

    It checks `hello-observing.json` and `report-observed.json`, each with `protocol_version` 2, against version 2's `from-runner.schema.json`, and each with 1 against version 1's, each refused.
- **1.2**
  - `version_3_is_version_2_but_for_its_hold` is renamed and extended as Decision 9 gives it.
  - The kinds' branches are read by their position in `$defs.body.oneOf`, each found by its `required`.
  - `observing_hello`'s check builds the expected value from version 3's own `hello`, so the four shared properties are held to the hello's.
- **1.3** `tests/refused.rs`'s `a_field_the_version_does_not_name` adds `reason` to `hello-observing.json`'s hello and to `report-observed.json`'s launch, beside what it adds now, each refused.
- **1.4**
  - `src/protocol/v3.rs` gains `ObservingHello`, `ObservedReport` and `ObservedLaunch`, `ObservingHello` writing `observing` from a field the type fixes as `true`.
  - `FromRunner::Hello` and `FromRunner::Report` each take an untagged enum of the working form and the observing one, the observing one first.
  - `check.rs` gains `observing_hello()` and `observed_report()`.
  - `tests/examples.rs`'s `every_example_read_and_written_back` reads the three new examples as it reads the others.
- **2.1**
  - `Table` gains a `Role`, and `Table::read_observer(table, file)` reads `[observer]`. The loopback test is `address`'s own, required here for `https://` as well.
  - `key_name` gives the label `observer.<label>` for an observer.
  - Unit tests in `table.rs`: `an_observer_read`, `an_observer_off_the_host` (`https://control.example`, refused naming `[observer]` and `url`), `an_observer_with_a_token` (refused saying `boomdrive enroll --observer`, the value in no output), and `an_observers_key_name` (`observer.home@localhost`, `boomdrive.observer.home.localhost`).
- **2.2**
  - `lib.rs` gains `enroll_observer(table, file, label, token)`, which is `enroll` with `Table::read_observer`.
  - `tests/enrollment.rs` gains `an_observer_enrolled`: the fake control plane as the observer, the key in a `Memory` store under `observer.home`, the request to its `runner/enroll`, its `enroll` passing version 3.
  - `tests/key.rs` gains `an_observers_key_kept_by_systemd_creds`, `kept_by_systemd_creds` for an observer's table, its stub run with `--name=boomdrive.observer.home.<host>`.
- **3.1**
  - `crates/boomdrive-control/src/lib.rs` gains Decision 5's `Observer` and `Observed`.
  - `src/fake.rs` gains `Observing`, holding what it is handed in an `Arc<Mutex<Vec<(Hello, Report)>>>` it gives a clone of.
  - Its unit test `an_observer_holds_what_it_is_handed` hands it two passes and reads them back in order.
- **4.1**
  - `Dial` gains the role, `Dial::observing` making one for an observer with a wake that does nothing.
  - `Connection::hand` takes any hello that serializes, written as JSON and checked against the role's form, so a partition's `v3::Hello` and an observer's `v3::ObservingHello` are each handed as they are. `Connection::report` checks against the role's form.
  - The run loop's `ToRunner` match takes Decision 6's refusals in the observing role. `reached` and `unreached` take their words from the role.
  - `tests/observer.rs`, new, holds the connection's tests, each over `common::fake`, the fake answering an observing hello with `pulled` from its `set_wants` and `set_report`:
    - `the_observing_hello_first`;
    - `an_offer_from_an_observer` (`message_refused`, its words, one line, no wake);
    - `a_report_from_an_observer` (a `pulled` holding a report, refused);
    - `a_terminal_from_an_observer` (`message_refused`, no `terminal_closed`);
    - `an_observers_outage` (said once each way, `the observer`).
- **4.2**
  - `observer.rs` holds `open`, `Observing` and its runtime. `plane.rs`'s cut of one launch moves into a function `observer.rs` calls, and its `Shared` runtime is `pub(crate)`.
  - `tests/observer.rs` gains:
    - `the_places_observed` (Decision 2's report, `home/1` as held, no lease, no act);
    - `the_observing_hello_names_the_partitions_repositories`;
    - `a_report_answered_and_let_go` (not sent again; the next still holds `home/1`);
    - `a_pulled_wakes_nothing`;
    - `an_observer_down_holds_up_nothing` (three handings, each within 100 ms, the fake down);
    - `back_again_the_newest_alone` (two handings while down, the second alone sent after the hello);
    - `the_hosted_observers` (`[observer]` naming `home` and `work` opens two, in order).
- **5.1**
  - `config.rs` gains `Config::observer`, read and refused as Decision 3 says, through `planes::read_observer`. The local layer's walk refuses `observer` as a key it does not set.
  - Its unit tests, in `config.rs`'s `tests`: `an_observer_beside_crew`, `an_observer_of_no_control_plane` and `an_observer_off_the_host`.
  - `tests/local_layer.rs` gains `an_observer_in_the_local_layer`.
- **5.2**
  - `main.rs`'s `Enroll` gains `--observer`.
  - `enroll.rs` reads `Config::observer`, refusing as Decision 4 says, and calls `planes::enroll_observer`. It answers as Decision 4 gives.
  - `tests/enroll.rs` gains:
    - `an_observer_enrolled_beside_crew`: `type = "crew"`, `[observer]` at a loopback port with nothing on it; refused as unreachable, exit 1, not as crew enrolling nothing;
    - `no_observer_named` (exit 2, naming `[observer]`).
- **6.1**
  - `planes.rs` gains `observers`, `read_observer` and `enroll_observer`.
  - `run.rs`'s `Control` holds the observers. `Runner::observe_with` puts a test's in place.
  - `keep` pairs each part with its observer by label, and says each unpaired one once at its start.
  - `Host::carry_on` and `carry` take the part's observer. `Carrier::observed_pass` hands it the pass, as Decision 7 says.
  - `tests/control.rs` gains these tests, beside its tests of parts, using its `Running` helper and `boomdrive_control::fake::Observing`:
    - `a_part_observed` (`home` and `work`, an observer of `home` alone);
    - `an_observer_of_no_part` (the line, and `home` carried);
    - `an_observer_unreachable`: a hosted observer of `home` over a `Memory` store, at a loopback port with nothing on it; `home`'s offered launch made and reported within seconds.
  - `tests/once.rs` gains `one_pass_observes_nothing`: `[observer]` names a loopback port where a listener records any connection, and `run --once` makes none.
- **7.1**
  - `crates/boomdrive/tests/examples/observer.toml` is README's example of `[observer]` beside a crew control plane.
  - `tests/examples.rs`'s new `the_observer_table_reads` reads it as the configuration, and finds `Config::observer` the table as written, and `planes::read_observer` taking it.
