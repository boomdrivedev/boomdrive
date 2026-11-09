# Design

## Context

See proposal.md for why. This change makes the runner one more recipient of each place it launches whose launch names recipients, so Flume holds the place's end for the runner past its tidy, and has the runner collect it once its report holds it. The decision is record 0030, with core.53 beside it and the details of core.1, core.13 and core.15 amended.

What stands:

- **This repository**, at `c6eb79b`.
  - `flume::Cli` (`crates/boomdrive/src/flume.rs`) puts `--json` first, unsets `FLUME_RECIPIENT`, and reads each answer by Flume's exit: `Reply::Given` as printed on exit 0, `Reply::Refused` with Flume's reason on exit 1 or its standard error otherwise, and `Unrun` where it could not run. `launch_args` passes each `--recipient` the launch gives and ends `--mark boomdrive:<id>`. The `Flume` trait has `launch`, `act`, `status` and `status_of`. Its implementations are `Cli`, `tests/fakes/mod.rs`'s `FakeFlume` and `tests/once.rs`'s `Watching`.
  - `Carrier::pass` (`crates/boomdrive/src/carry.rs`) reads Flume's listing, pulls, answers the wants in two rounds and saves the report after each. Where a want ran a command, it reads the listing again (record 0027). Then it runs `carry_statuses`, `carry_entries` and `send`, which saves and then reports. `carry_statuses` carries, for each launch the pull gives whose clone stands, `flume --json status --repository <container> <place>`'s whole answer, asked again only where the place's entry in the listing differs from the one carried. `facts::entry` takes the place standing at a location, else its latest tidied record, `<location>@<at>`. A refused status leaves the last carried standing, not listed.
  - `Work::standing` reads a launch landed from the listing's entry for its place, else from the status carried, through `facts::over`, which reads `over` and `tidied`'s `over` alike. So an After, the keeper's landing and the drain already read a tidied record landed.
  - `report::ended` (`crates/boomdrive/src/report.rs`) says a place has ended once its carried status is not listed, or reads `tidied` or `gone`. The keeping pass removes a drained repository's clone once no place there has not ended (record 0021).
  - `Reach` (`carry.rs`) says the control plane's reach once each way, which is the model for saying a Flume older than the least.
  - `hello::hello` reads the first line of each tool's `--version` at every pass into `Hello::tool_versions`. `git::LEAST` and `git::starts_with` are the model for a tool's floor. The runner names no floor for Flume today.
  - The tests' fakes: `FakeFlume` keeps its places' entries, scripts answers, removes a place on a drop or a done, and holds a tidied record by `tidy_pending`. `FakeTools` prints `<tool> 1.0.0`. The binary's stub `flume` prints nothing for `--version`, which the hello reads as missing. `tests/family.rs` runs the set's Flume on a scratch host.
  - The set's Flume is `v0.17.1` (`flake.nix`, `flake.lock`, `nix/packages/flume-git-crates.nix`), built by this flake's recipe, `nix/packages/flume.nix`.
  - The hosted implementation's copy of a place's status (`crates/boomdrive-hosted/src/copy.rs`) is cut by version 1's `place_status` and checked against it. Its `ends` name a stage's, a submission's and a stall's end. A copy that fails the check goes up null, and is said once.
- **Flume `v0.20.0`** (`9ab0e2c`), which carries the bolt `a-tidied-places-end-stays-readable` (`72107a2`) and Flume record 0060:
  - A launch's recipients are those it names, else `FLUME_RECIPIENT`, else the launching session, else the answerer behind its stream (`launch.rs`'s `recipients_of`; 7). Naming any recipient drops the defaults.
  - `flume launch --recipient <name>`: a recipient is a session by its name or a role's address. A name no pane holds is sent no notice. Its ends, a drop's or a done's among them, wait for it, the place's record kept once it is tidied, until it collects them by `flume inbox` or `flume wait` as that name.
  - A drop or a done is an end, `{"of":"over","over":…}`, that no notice carries and no role is woken for, pending for the place's recipients that are no role's address (Flume record 0060).
  - One settle of a place reads its submission landed, makes its work over, tidies it and delivers its ends. It keeps a tidied place's document as `<location>@<at>` while an end of it is pending for a recipient (62). Its status gives the phase `{"tidied":{"over":…,"at":…,"pending_for":[…]}}`.
  - `flume inbox [<place>] --recipient <name>` collects every end pending for `<name>`, of the one place named, a tidied document's `<location>@<at>` naming that one alone, else of every place on the host (`wait.rs`'s `inbox`). An inbox naming a place it does not hold is refused, `no place <place> stands on this host`. Once no end of a tidied document stands pending, the document goes.
  - `flume launch` takes no `--caller`.
  - Flume publishes a flake of its own since `v0.18.0` (Flume record 0055), with its package and `services.flume`. Its git crates at `v0.20.0` are weir's `weir-capture` and `weir-kinds`, at `daacda39`.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches.

## Goals / Non-Goals

**Goals:**

- No tidy loses the end of a place the runner launched under this change for a launch that names a recipient, whatever tidied it: a landing, a drop or a done is in the runner's report as Flume's record of the place before Flume lets that record go.
- A control plane that names a recipient on every launch, as crew does, reads every place's end from the runner's report alone (crew record 0056).
- The runner's After, the keeper's landing and the drain read a held record as they read any tidied record.

**Non-Goals:**

- **crew reading the end.** crew's implementation already writes the carried status, a held record among them, to `places.rec`. crew's unit `crew-reads-each-pieces-end-from-the-runners-report` reads it there and retires record 0062's stopgap.
- **A launch that names no recipient.** It is passed with none, so Flume's default is its recipient, and Flume holds nothing of it for the runner (Decision 1).
- **Places launched before this change.** The runner is no recipient of theirs, so Flume holds nothing of them for it, and they are followed as before.
- **A report field or a protocol message.** The end goes as the place's status, which both implementations carry already. The hosted copy of the tidied phase names `pending_for`. A drop's or a done's end, which version 1 does not name, is a version of the protocol of its own (Risks).
- **Letting go a record held for a launch the runner no longer holds.** It stays on the host as Flume holds it (core.10).
- **Taking Flume from its own flake and module.** That is record 0023's, a change of its own (Rules departed from).

## Decisions

### 1. The runner is one more recipient

`flume::RECIPIENT` is `"boomdrive"`: fixed, the same on every launch and every host. `flume::recipients(given)` gives the launch's recipients as given, then `RECIPIENT` where `given` names any and does not name it already. `Cli`'s `launch_args` passes `--recipient` for each of them, so every `flume launch` the runner runs holds the runner beside the recipients its launch names. `LaunchArgs::recipients` stays what the launch gives (core.15).

A launch that names no recipient gets none of the runner's. Naming the runner alone would drop Flume's default, the answerer behind the stream among them (7), and that role would hear none of the place's ends. agentplot-design's ground for the answer was that naming the runner beside the role "drops no default recipient", and this keeps it. Such a place is followed as before.

Not taken: a caller apart from the recipients, `--caller`, which Flume record 0060 folds into the recipients and Flume does not take.

### 2. The held record is the place's status

Nothing changes in `carry_statuses`. A held record is the place's latest tidied record, which `facts::entry` already takes, and which Flume's `status --repository <container> <location>` already answers. A control plane reads each end there: a landing, a drop and a done as the record's `over`, a failed or refused submission in the status while the place stands, and a refused launch as the launch's answer.

### 3. A held record is collected once the report file holds it

After `carry_entries`, the pass saves the report. Only where the file holds it, `Work::collect` runs, before `send`. For each launch the pull gives, in the pull's order, it collects where all of these hold:

- the launch is answered `launched` at a place;
- its repository's clone stands, `Keeping::Ready` or `Keeping::Dropped`;
- its carried status is listed;
- the place's entry in it reads the phase `tidied`, its `pending_for` naming `flume::RECIPIENT`.

It runs `Flume::collect(<the entry's place>)`, the record's id `<location>@<at>`, as `flume --json inbox <location>@<at> --recipient boomdrive`, naming no repository. On `Reply::Given` it says nothing and keeps nothing of the answer. On `Reply::Refused(words)` it says `<id>: Flume refused the collection of <record>: <words>`. On `Unrun(why)` it says `<id>: Flume's record <record> could not be collected: <why>`. Either is tried again at the next pass, since the carried status still says the same.

At the next pass the listing's entry for the place has changed, kept for others or gone, so its status is asked again and carried, or refused and left standing not listed. So the collection is made once, on Flume's own word.

Collecting before `send` keeps an unreachable control plane from holding records. The file holds the end and is sent at every later pass until one is taken, and resent whole after a reconnect (core.21). A pass whose save failed collects nothing (core.25). A launch whose report the control plane refused leaves the report in `Work::reported`, so no later pass collects it (core.10).

### 4. A drained place has not ended while its record is held

`report::ended` reads a place `tidied` or `gone` as ended only where its `pending_for` does not name `flume::RECIPIENT`. So a drained repository's clone stays until the place's record is collected, and the collection, which needs the clone to stand (Decision 3), is never stranded by the drain.

### 5. The least Flume

`flume::LEAST` is `(0, 20, 0)`, Flume `v0.20.0`'s version. `flume::runs_with(printed)` reads `flume <major>.<minor>.<patch>` from `--version`'s first line, a suffix after the patch ignored, as `git::version` does. It refuses one older than `LEAST`, or no version, with `flume --version says "<printed>": boomdrive launches with Flume 0.20.0 or later, the first that holds a place's end for the runner`, and for no version `flume --version says "<printed>", which names no version: boomdrive launches with Flume 0.20.0 or later, …`.

`Carrier::pass` takes `hello.tool_versions["flume"]`. Where it is `Printed` and `runs_with` refuses it, `Work::offer` makes no launch where it would launch, so a new offer stays unanswered and a held one stays held. The words are said through `Least`, a cell kept across passes beside `Reach` (`Carrier::least`), as `boomdrive: <words>`. It says the line only where it differs from the one said last, and clears it once Flume runs with the least. A `Missing` version changes nothing: the launch is tried and goes unanswered as today, its failure named. Acts and statuses go on.

### 6. The set's Flume

The set moves to `v0.20.0`, its tag with the lock, Flume's git crates' hashes at that tag (`weir-capture` and `weir-kinds` at `daacda39`, the hashes Flume's own `nix/packages/git-crates.nix` holds), and README's override example, as the chore at `4dd74ca` did. `tests/family.rs` gains `the_sets_flume_takes_the_runner_as_a_recipient`. It runs the set's Flume on the scratch host as `flume --json launch --repository <a folder that is no repository> --kind change --name x --recipient scratch/planner --recipient boomdrive --mark m`, and `flume --json inbox /no/such/place@1 --recipient boomdrive`. Each must exit 1 with a JSON refusal, read by `flume::Cli` as `Reply::Refused` with Flume's reason, and not exit 2. It also asserts `flume::runs_with` takes the set's `--version`.

### 7. The code

- `src/flume.rs`: `RECIPIENT` and `recipients`; the runner's `--recipient` in `launch_args`; `Flume::collect` and `Cli`'s, `["inbox", record, "--recipient", RECIPIENT]`; `LEAST` and `runs_with`, with unit tests; the module's doc saying the runner's recipient.
- `src/facts.rs`: `pending_for(entry, name) -> bool`, true where the entry's phase is `tidied` and its `pending_for` names `name`, with unit tests beside `over`'s.
- `src/carry.rs`: `Work::collect`, called after `carry_entries` where `save` holds; the least in `Work::offer` before `launch`; `Least` and `Carrier::least`.
- `src/run.rs`: the runner keeps one `Least` beside its `Reach`, and hands it to each `Carrier`.
- `src/report.rs`: `ended` by Decision 4, with a unit test.

### 8. The tests

All run on the crate's fakes and stubs, with neutral names and paths (core.41).

- `FakeFlume` gains:
  - the recipients of each place it launched, `flume::recipients` of the launch's, so a drop or a done of a place whose recipients name `boomdrive` tidies it into a record `<place>@<n>` whose phase is `tidied` with `over` `dropped` or `done` and `pending_for` `["boomdrive"]`, in place of removing it;
  - `tidy_held(place, at, pending_for)`, landed at `c0ffee`, which `tidy_pending` becomes a case of;
  - `collect`, recorded as `Call::Collect(record)`, which takes `boomdrive` out of the record's `pending_for` and removes the record where nothing is left, and refuses a record it does not hold;
  - `collect_answers(record, reply)`.
- `Watching` in `tests/once.rs` gains `collect`, reading the report file as it does at an act.
- `FakeTools` gains `prints(tool, line)`.
- `Bench` (`tests/fakes/mod.rs`) and `tests/pass.rs` build their `Carrier` with a `Least` of their own.
- Each launch's argv asserted whole (`tests/flume.rs`, `tests/offers.rs`) holds `--recipient boomdrive` after the launch's own recipients, and a launch naming none (`tests/flume.rs`, `tests/bolts.rs`) holds no `--recipient`.

### Rules relied on

- **core.1**: the end is Flume's record as Flume gave it, and the collection decides nothing. Its details name the collection.
- **core.10**: no collection for a launch whose lease was refused.
- **core.11** and **core.35**: nothing of the collection's answer is carried or kept.
- **core.15**: the runner's own name is the one recipient it adds, beside the launch's, none of which it leaves out. Its details say so.
- **core.25**: a pass whose save failed collects nothing.
- **core.27**: the hosted copy carries the held record's fields its schema names.
- **core.38**: the least Flume.
- **core.53**: this change's rule.
- **core.13**: the collection is the end of carrying a place, and no housekeeping of Flume's. Its details say so.

### Rules departed from

- **core.44**, for Flume: the set keeps this flake's recipe for Flume at `v0.20.0`, whose own flake builds it (Flume record 0055), where core.44 rules out "a recipe kept here for a tool whose own flake builds it". Taking Flume's package from its flake goes with taking its `services.flume` module, which record 0023 has the Boomdrive flake re-export from the tool's own flake. That is a change of its own, which moves the set's Flume, its tests and the modules together. Here the set's Flume moves only to the least the runner names, by its tag and its git crates' hashes as the recipe took every tag before.

The work-into-flume spec's "Flume runs with no recipient of the runner's own" is amended in this change, as the user's answer and agentplot-design ask.

## Risks / Trade-offs

- **A launch that names no recipient keeps the gap.** Flume's default is its recipient and Flume holds nothing of it for the runner. crew names a role on every launch. A control plane that wants every end in the runner's report names a recipient on every launch.
- **A drop's or a done's end under the hosted protocol.** Under Flume `0.20.0` a dropped or done place's status names an end `{"of":"over",…}`, which version 1's `end_record` does not name. The hosted copy of such a status fails version 1's check, goes up null, and is said once (core.42). crew takes the status whole. Carrying it to a hosted control plane is a new version of the protocol, a change of its own.
- **Every end is owed to the runner.** Each stage's and stall's end of its places stands pending for `boomdrive` until the runner collects the tidied record. A live place's ends stay pending, which changes no notice, wake or stall of Flume's.
- **A session named `boomdrive`.** Flume would send it the notices of every place the runner launches. No host names one.
- **Leaked records.** A record held for a launch whose lease was refused, or one the control plane let go before the runner collected it, stays on the host until a person drops it. Both are rare: the runner collects in the pass that carries the record, before it reports.
- **A host below the least launches nothing.** It says so once, and the hello names its Flume. That is core.38's intent, and better than every launch failing on Flume's older answers.

## Migration Plan

None of the runner's files change. A runner of this change, on a host whose Flume is at the least, names itself a recipient from its first launch that names one. Places launched before are followed as before. Rolling back leaves records held for `boomdrive` on the host until someone drops them, or until a runner of this change collects them.

## After deploy

Once this runs on every host crew serves, crew's unit `crew-reads-each-pieces-end-from-the-runners-report` retires record 0062's stopgap.

## Task notes

- **1.1** Record 0030 and core.53 say what the user's answer and agentplot-design's words decide, and Flume `v0.20.0` gives. The record is proposed until the user has seen it.
- **1.2** Move the set exactly as `4dd74ca` did: `flake.nix`'s Flume `url` tag, `nix flake lock --update-input flume` under the devenv shell, each of Flume's git crates' hashes at the new tag in `nix/packages/flume-git-crates.nix`, and README's `--override-input flume` example. Set `flume::LEAST` to the release's version in the same task.
- **2.1** `flume_runs_with_no_recipient_of_the_runners_own` keeps its name: it proves `FLUME_RECIPIENT` unset. `the_runner_is_one_more_recipient` in `tests/flume.rs` asserts the argv of a launch with recipients and of one without.
- **4.2** `Work::collect` reads the entry through `carried_entry`, and the record's id from the entry's `place`, never by joining the location and a time itself.
- **4.3** The scenario "A landing held for the runner" is proven by two tests. `statuses.rs`'s `a_landing_held_for_the_runner` covers the calls and the statuses carried. `once.rs`'s `a_held_record_is_collected_once_its_report_holds_it` checks, through `Watching`, that the file held the record when the inbox ran.
- **5.2** In `the_places_end_held_for_the_runner`, the record held is written to the report as `report_holding` does, `pending_for` `["boomdrive"]`, and then `["scratch/planner"]`.
- **6.1** `said_once_while_it_lasts` runs two passes with `FakeTools::prints("flume", "flume 0.18.0")`.
