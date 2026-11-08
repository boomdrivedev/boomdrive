# Design

## Context

See proposal.md for why. This change has every launch name the runner its caller, so Flume holds each place's end for the runner past its tidy, and has the runner collect it once its report holds it. The decision is record 0030, in this change's commit, with core.53 beside it and the details of core.1, core.13 and core.15 amended.

What stands:

- **This repository**, at `c6eb79b`.
  - `flume::Cli` (`crates/boomdrive/src/flume.rs`) puts `--json` first, unsets `FLUME_RECIPIENT`, and reads each answer by Flume's exit: `Reply::Given` as printed on exit 0, `Reply::Refused` with Flume's reason on exit 1 or its standard error otherwise, and `Unrun` where it could not run. `launch_args` ends `--mark boomdrive:<id>`. The `Flume` trait has `launch`, `act`, `status` and `status_of`. Its implementations are `Cli`, `tests/fakes/mod.rs`'s `FakeFlume` and `tests/once.rs`'s `Watching`.
  - `Carrier::pass` (`crates/boomdrive/src/carry.rs`) reads Flume's listing, pulls, answers the wants in two rounds and saves the report after each. Where a want ran a command, it reads the listing again (record 0027). Then it runs `carry_statuses`, `carry_entries` and `send`, which saves and then reports. `carry_statuses` carries, for each launch the pull gives whose clone stands, `flume --json status --repository <container> <place>`'s whole answer, asked again only where the place's entry in the listing differs from the one carried. `facts::entry` takes the place standing at a location, else its latest tidied record, `<location>@<at>`. A refused status leaves the last carried standing, not listed.
  - `Work::standing` reads a launch landed from the listing's entry for its place, else from the status carried, through `facts::over`, which reads `over` and `tidied`'s `over` alike. So an After, the keeper's landing and the drain already read a tidied record landed.
  - `report::ended` (`crates/boomdrive/src/report.rs`) says a place has ended once its carried status is not listed, or reads `tidied` or `gone`. The keeping pass removes a drained repository's clone once no place there has not ended (record 0021).
  - `Reach` (`carry.rs`) says the control plane's reach once each way, which is the model for saying a Flume older than the least.
  - `hello::hello` reads the first line of each tool's `--version` at every pass into `Hello::tool_versions`. `git::LEAST` and `git::starts_with` are the model for a tool's floor. The runner names no floor for Flume today.
  - The tests' fakes: `FakeFlume` keeps its places' entries, scripts answers, removes a place on a drop or a done, and holds a tidied record by `tidy_pending`. `FakeTools` prints `<tool> 1.0.0`. The binary's stub `flume` prints nothing for `--version`, which the hello reads as missing. `tests/family.rs` runs the set's Flume on a scratch host.
  - The set's Flume is `v0.17.1` (`flake.nix`, `flake.lock`, `nix/packages/flume-git-crates.nix`).
- **Flume**, at main `81d57226` (after `v0.18.0`), and its statement:
  - One settle of a place takes its steps in a loop: it reads the submission landed, makes the work over and tidies the place, then delivers its ends (`settle.rs`'s `settle_place` and `step`, `submit.rs`). It keeps a tidied place's document as `<location>@<at>` only while an end of it is pending for a recipient (`ends.rs`'s `keep_for_ends`; 62).
  - A launch's recipients are those it names, else `FLUME_RECIPIENT`, else the launching session, else the answerer behind its stream (`launch.rs`'s `recipients_of`; 7). A recipient is a session or a role's address. An end is delivered as a notice into its pane, and stays pending where none is reached (`deliver_ends`; 43).
  - An end is a stage's, a submission's outcome (landed, failed, conflict, withdrawn or refused) or a stall. A drop and a done record no end.
  - `flume inbox --recipient <r>` collects every end pending for `r` of every place on the host (`wait.rs`'s `inbox`). `flume drop` of a tidied location removes its every record (`acts.rs`).
  - Status of a tidied record gives its phase as `{"tidied":{"over":…,"at":…,"pending_for":[…]}}`.

**What Flume lacks**, which the Flume unit ahead of this change builds:

1. **A launch's caller.** `flume launch --caller <name>` names who launched the place, kept with it apart from its recipients. It takes the place of no default recipient, gets no notice, wakes nothing, and no stall reaches it.
2. **A record held for the caller.** A place whose launch names a caller keeps its document after its tidy, as `<location>@<at>`, whatever ended its work: a landing, a drop or a done. It keeps it until that caller collects it. Its status names the caller in the tidied phase's `pending_for` while it does.
3. **One place collected for the caller.** `flume inbox --caller <name> <place>` collects, for the caller, the one record it names, by `<location>@<at>` or its location, naming no repository. Every end of the place pending for the caller is delivered to it, and the caller's hold is let go, so the document goes once no recipient's end is pending either. Another place's record is untouched. It answers as the inbox does, and refuses a place it does not hold.
4. **A release** carrying the three, whose version is the least Flume the runner names.

The spellings `--caller` and `inbox --caller <name> <place>` are this change's asks. Where Flume's release spells them otherwise, the build follows Flume's, and changes this change's specs and README to match.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches.

## Goals / Non-Goals

**Goals:**

- No tidy loses the end of a place the runner launched under this change, whatever tidied it: a landing, a drop or a done is in the runner's report as Flume's record of the place before Flume lets that record go.
- A control plane reads every place's end from the runner's report alone (crew record 0056).
- The runner's After, the keeper's landing and the drain read a held record as they read any tidied record.

**Non-Goals:**

- **Flume's half**, items 1 to 4 above: the Flume unit's.
- **crew reading the end.** crew's implementation already writes the carried status, a held record among them, to `places.rec`. crew's unit `crew-reads-each-pieces-end-from-the-runners-report` reads it there and retires record 0062's stopgap.
- **Places launched before this change.** They name no caller, so Flume holds nothing of them, and they are followed as before.
- **A report field or a protocol message.** The end goes as the place's status, which both implementations carry already. The hosted copy of the tidied phase names `pending_for`.
- **Letting go a record held for a launch the runner no longer holds.** It stays on the host as Flume holds it (core.10).

## Decisions

### 1. Every launch names the runner its caller

`launch_args` ends `--mark <mark> --caller boomdrive`, from `flume::CALLER`, `"boomdrive"`: fixed, the same on every launch and every host. It is no `LaunchArgs` field, since no launch gives it. `--recipient`s stay those the launch gives (core.15).

Not taken: `--recipient boomdrive`. Where a launch names no recipient, that would take the place of Flume's default, and Flume would send notices into any session of the name (record 0030).

### 2. The held record is the place's status

Nothing changes in `carry_statuses`. A held record is the place's latest tidied record, which `facts::entry` already takes, and which Flume's `status --repository <container> <location>` already answers. A control plane reads each end there: a landing, a drop and a done as the record's `over`, a failed or refused submission in the status while the place stands, and a refused launch as the launch's answer.

### 3. A held record is collected once the report file holds it

After `carry_entries`, the pass saves the report. Only where the file holds it, `Work::collect` runs, before `send`. For each launch the pull gives, in the pull's order, it collects where all of these hold:

- the launch is answered `launched` at a place;
- its repository's clone stands, `Keeping::Ready` or `Keeping::Dropped`;
- its carried status is listed;
- the place's entry in it reads the phase `tidied`, its `pending_for` naming `flume::CALLER`.

It runs `Flume::collect(<the entry's place>)`, the record's id `<location>@<at>`, as `flume --json inbox --caller boomdrive <location>@<at>`, naming no repository. On `Reply::Given` it says nothing and keeps nothing of the answer. On `Reply::Refused(words)` it says `<id>: Flume refused the collection of <record>: <words>`. On `Unrun(why)` it says `<id>: Flume's record <record> could not be collected: <why>`. Either is tried again at the next pass, since the carried status still says the same.

At the next pass the listing's entry for the place has changed, kept for others or gone, so its status is asked again and carried, or refused and left standing not listed. So the collection is made once, on Flume's own word.

Collecting before `send` keeps an unreachable control plane from holding records. The file holds the end and is sent at every later pass until one is taken, and resent whole after a reconnect (core.21). A pass whose save failed collects nothing (core.25). A launch whose report the control plane refused leaves the report in `Work::reported`, so no later pass collects it (core.10).

### 4. A drained place has not ended while its record is held

`report::ended` reads a place `tidied` or `gone` as ended only where its `pending_for` does not name `flume::CALLER`. So a drained repository's clone stays until the place's record is collected, and the collection, which needs the clone to stand (Decision 3), is never stranded by the drain.

### 5. The least Flume

`flume::LEAST`, the least Flume, is the version of the release that carries items 1 to 3. `flume::runs_with(printed)` reads `flume <major>.<minor>.<patch>` from `--version`'s first line, a suffix after the patch ignored, as `git::version` does. It refuses one older than `LEAST`, or no version, with `Flume is <printed>: boomdrive launches with Flume <least> or later, the first that holds a place's end for the runner`.

`Carrier::pass` takes `hello.tool_versions["flume"]`. Where it is `Printed` and `runs_with` refuses it, `Work::offer` makes no launch where it would launch, so a new offer stays unanswered and a held one stays held. The words are said through `Least`, a `Cell<Option<String>>` kept across passes beside `Reach` (`Carrier::least`). It says the line only where it differs from the one said last, and clears it once Flume runs with the least. A `Missing` version changes nothing: the launch is tried and goes unanswered as today, its failure named. Acts and statuses go on.

### 6. The set's Flume

The set moves to the release carrying items 1 to 3, its tag with the lock, Flume's git crates' hashes for that tag, and README's override example, as the chore at `4dd74ca` did. `tests/family.rs` gains `the_sets_flume_takes_the_runners_caller`. It runs the set's Flume on the scratch host as `flume --json launch --repository <a folder that is no repository> --kind change --name x --mark m --caller boomdrive`, and `flume --json inbox --caller boomdrive /no/such/place@1`. Each must exit 1 with a JSON refusal, read by `flume::Cli` as `Reply::Refused` with Flume's reason, and not exit 2. It also asserts `flume::runs_with` takes the set's `--version`.

### 7. The code

- `src/flume.rs`: `CALLER`; `--caller` in `launch_args`; `Flume::collect` and `Cli`'s, `["inbox", "--caller", CALLER, record]`; `LEAST` and `runs_with`, with unit tests; the module's doc saying the caller.
- `src/facts.rs`: `kept_for(entry, name) -> bool`, true where the entry's phase is `tidied` and its `pending_for` names `name`, with unit tests beside `over`'s.
- `src/carry.rs`: `Work::collect`, called after `carry_entries` where `save` holds; the least in `Work::offer` before `launch`; `Least` and `Carrier::least`.
- `src/run.rs`: the runner keeps one `Least` beside its `Reach`, and hands it to each `Carrier`.
- `src/report.rs`: `ended` by Decision 4, with a unit test.

### 8. The tests

All run on the crate's fakes and stubs, with neutral names and paths (core.41).

- `FakeFlume` gains:
  - `hold()`, after which a drop or a done tidies the place into a record `<place>@<n>` whose phase is `tidied` with `over` `dropped` or `done` and `pending_for` `["boomdrive"]`, in place of removing it;
  - `tidy_held(place, at, pending_for)`, landed at `c0ffee`, which `tidy_pending` becomes a case of;
  - `collect`, recorded as `Call::Collect(record)`, which takes `boomdrive` out of the record's `pending_for` and removes the record where nothing is left, and refuses a record it does not hold;
  - `collect_answers(record, reply)`.
- `Watching` in `tests/once.rs` gains `collect`, reading the report file as it does at an act.
- `FakeTools` gains `prints(tool, line)`.
- `Bench` (`tests/fakes/mod.rs`) and `tests/pass.rs` build their `Carrier` with a `Least` of their own.
- Each launch's argv asserted whole (`tests/flume.rs`, `tests/offers.rs`, `tests/bolts.rs`) ends `--mark boomdrive:<id> --caller boomdrive`.

### Rules relied on

- **core.1**: the end is Flume's record as Flume gave it, and the collection decides nothing. Its details name the collection.
- **core.10**: no collection for a launch whose lease was refused.
- **core.11** and **core.35**: nothing of the collection's answer is carried or kept.
- **core.15**: the caller is no recipient and no setting. Its details name it beside the mark.
- **core.25**: a pass whose save failed collects nothing.
- **core.27**: the hosted copy carries the held record's fields its schema names.
- **core.38**: the least Flume.
- **core.53**: this change's rule.
- **core.13**: the collection is the end of carrying a place, and no housekeeping of Flume's. Its details say so.

### Rules departed from

None. The work-into-flume spec's "Flume runs with no recipient of the runner's own" is amended in this change, as agentplot-design asked: the runner names itself the caller, which is no recipient.

## Risks / Trade-offs

- **The build waits on Flume.** Items 1 to 4 must be released first. Until then task 1.1 cannot pass. agentplot-planner places Flume's unit ahead.
- **Flume's spelling may differ.** Then the build follows Flume's, and changes `CALLER`'s use, `collect`'s argv, the specs and README to match (Context).
- **Leaked records.** A record held for a launch whose lease was refused, or one the control plane let go before the runner collected it, stays on the host until a person drops it. Both are rare: the runner collects in the pass that carries the record, before it reports.
- **A host below the least launches nothing.** It says so once, and the hello names its Flume. That is core.38's intent, and better than refusing every offer with Flume's usage error.

## Migration Plan

None of the runner's files change. A runner of this change, on a host whose Flume is at the least, names itself the caller from its first launch. Places launched before are followed as before. Rolling back leaves records held for `boomdrive` on the host until someone drops them, or until a runner of this change collects them.

## After deploy

Once this runs on every host crew serves, crew's unit `crew-reads-each-pieces-end-from-the-runners-report` retires record 0062's stopgap.

## Task notes

- **1.1** Move the set exactly as `4dd74ca` did: `flake.nix`'s Flume `url` tag, `nix flake lock --update-input flume` under the devenv shell, each of Flume's git crates' hashes at the new tag in `nix/packages/flume-git-crates.nix`, and README's `--override-input flume` example. Set `flume::LEAST` to the release's version in the same task.
- **2.1** `flume_runs_with_no_recipient_of_the_runners_own` keeps its name: it proves `FLUME_RECIPIENT` unset. `every_launch_names_the_runner_its_caller` in `tests/flume.rs` asserts the argv of a launch with recipients and of one without.
- **4.2** `Work::collect` reads the entry through `carried_entry`, and the record's id from the entry's `place`, never by joining the location and a time itself.
- **4.3** The scenario "A landing held for the runner" is proven by two tests. `statuses.rs`'s `a_landing_held_for_the_runner` covers the calls and the statuses carried. `once.rs`'s `a_held_record_is_collected_once_its_report_holds_it` checks, through `Watching`, that the file held the record when the inbox ran.
- **5.2** In `the_places_end_held_for_the_runner`, the record held is written to the report as `report_holding` does, `pending_for` `["boomdrive"]`, and then `["scratch/planner"]`.
- **6.1** `said_once_while_it_lasts` runs two passes with `FakeTools::prints("flume", "flume 0.18.0")`.
