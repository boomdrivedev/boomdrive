# Design

## Context

See proposal.md for why. The runner confirms each end Flume owes it only once its control plane has taken a report holding it. The decision is record 0030 as amended on 2026-11-09, with core.54's rule and details, and core.1's and core.13's details. All of these are written with this change.

What stands:

- **This repository**, at `0859e14`.
  - `flume::RECIPIENT` is `"boomdrive"`, named after the recipients of every launch that names any (record 0030). `flume::LEAST` is `(0, 20, 0)`. The `Flume` trait has `launch`, `act`, `status`, `status_of` and `collect(record)`. `Cli::collect` runs `flume --json inbox <record> --recipient boomdrive`. The trait's implementations are `Cli`, `tests/fakes/mod.rs`'s `FakeFlume` and `tests/once.rs`'s `Watching`.
  - `Carrier::pass` (`crates/boomdrive/src/carry.rs`) carries statuses, entries and previews, then saves. Where the save holds, it runs `Work::collect`, opens the pull's terminals, and runs `send`. `send` saves, reports, takes the answer through `reported` (which forgets each id let go or refused), saves again, and gives whether the control plane took the report.
  - `Work::collect` walks the pull's offers in order. For each launch answered `launched` at a place, whose clone is `Keeping::Ready` or `Keeping::Dropped`, whose carried status is listed and whose entry (`carried_entry`) reads `facts::pending_for(entry, RECIPIENT)`, it runs `collect(<the entry's place>)`. A refusal or an unrun call is said in one line.
  - `Work::below_least` holds why no offer is launched where Flume's `--version` is older than `LEAST`, said once through `Least`.
  - `report::ended` reads a place `tidied` or `gone` as ended only where `facts::pending_for(entry, RECIPIENT)` is false, so a drained clone waits for the runner (record 0021).
  - `boomdrive-control`'s `Reported` holds `let_go: Vec<Id>` and `refused: BTreeMap<Id, String>`, serialised, with a JSON round trip in `tests/json.rs`. `Fake::report` answers each id of the report the test let go or refused, and records the report.
  - crew's `Crew::report` writes one partition's runner branch by `write::write`, the files `write::texts` forms from the report less what `write::let_go` gives. `places.rec` holds a `Place` with the status of each request launched at a place with a status, and a `Run` with the status of each role's latest run. The answer is `Ok` once the branch is written, or holds the same files already.
  - The hosted implementation's `report` sends the part's share, cut by `upward`, which puts `null` for each status whose copy fails version 1 and keeps those ids in `failing`. It waits up to 10 seconds for the `reported` of the message it sent. `answered` turns a `reported` into `Reported`, and keeps the share sent as `held` where it answers the last report sent.
  - The set's Flume is `v0.20.0` (`9ab0e2c`), built by this flake's recipe, `nix/packages/flume.nix`, with Flume's git crates `weir-capture` and `weir-kinds` at `daacda39` in `nix/packages/flume-git-crates.nix`. README's override example names `v0.21.0`.
- **Flume `v0.21.0`** (`77d5b93`), which carries the bolt `flumes-ends-are-owed-until-confirmed` and Flume record 0061:
  - Each end of a place is a delivery under the key `<kind>--<name>/<token>/<end key>`, owed to each of its recipients until that recipient confirms it. Status gives each end of a place its `delivery`, its `pending_for` (each recipient that has not confirmed it), `expires_at`, `owed` and `confirmed`. A tidied phase keeps `{"tidied":{"over":…,"at":…,"pending_for":[…]}}`. A drop's and a done's end are keyed `over/dropped` and `over/done`.
  - `flume confirm <key>… --recipient <name>` confirms each key as the recipient. A key confirmed before writes nothing and says when it first was. A key no end is under, or one whose end is another recipient's, is refused on exit 1, and the keys after it are not confirmed. No hand-over is needed before it. `confirm` takes no `--repository`.
  - `flume inbox` and `flume wait` hand an end over and confirm it only with `--confirm`.
  - A tidied place's document stays while an end of it is pending for a recipient. The next settle after every end is confirmed removes it. An end pending past `[host] ends_expire`, a week where unset, is shown expired with its recipient.
  - Its Cargo.lock takes `weir-capture`, `weir-delivery`, `weir-kinds` and `weir-pack` from weir at `52b9552`. Flume's own `nix/packages/git-crates.nix` holds each at `sha256-29h7paqrZ2AONJ/Xs2KBTF+9qYJWJMoJBZDE+OIxqoc=`.

## Goals / Non-Goals

**Goals:**

- No end Flume owes the runner leaves Flume before the runner's control plane holds it, so losing the runner's report loses no end.
- Every end owed to the runner, a live place's and a tidied record's alike, is confirmed once taken, so Flume lets a tidied record go and none of the runner's ends shows expired while the control plane is reached.
- The runner decides only when it has carried an end. What counts as taken is each implementation's to say, through the interface (core.2).

**Non-Goals:**

- **Confirming as another recipient.** A role's end confirmed at its control plane's ask is the change `the-runner-carries-a-control-planes-confirm`, on the same bolt.
- **A version of the hosted protocol that carries a drop's or a done's end, or a hand-over by a session's start.** Version 1's copy of such a status goes up null, so its ends stay owed (Risks).
- **Letting go an end of a launch the control plane let go, or whose lease it refused.** It stays owed in Flume (core.10).
- **Taking Flume from its own flake and module.** That is record 0023's, a change of its own. The set keeps this flake's recipe, as the change that wrote record 0030 did (Rules departed from).

## Decisions

### 1. The answer to a report names the statuses taken

`boomdrive-control`'s `Reported` gains `statuses_taken: BTreeSet<Id>`: each launch whose carried status the control plane took with the report this answer answers. It is serde `default` and skipped where empty, so an answer's JSON of before reads as naming none and writes as before. It names what was taken, never what was not, so an implementation that names nothing has nothing confirmed.

No message of the hosted protocol carries it. Each implementation knows what it wrote or sent.

### 2. What each implementation names taken

- **The fake control plane** names each launch of the report that holds a status, but those the test leaves with `Fake::leave_status(id)` until `Fake::take_status(id)`. A refused report is unsent, as now.
- **crew's implementation** names, once the partition's runner branch is written or already holds the same files, each launch whose status `places.rec` holds. That is each request launched at a place with a status, and each role's latest run with a status, of the report less what `write::let_go` gives. `write::statuses_held(label, &kept)` gives those ids, and `write::texts` and the answer read the same rule. A partition not written names nothing.
- **The hosted implementation** keeps, beside the share it sends (`Partition::sent`), the ids whose status went up as its copy: each launch of the part with a status, less those `upward` puts in `failing` for its status. `answered`, for the answer to the last report sent, adds those ids, under the partition's prefix, to `statuses_taken`. An answer to an earlier report, read from `answers()`, names none, since the statuses it took may have changed since.

### 3. The runner confirms after the answer

`Carrier::pass` keeps its carrying and its save, and no longer collects. After the terminals open, it runs `send`. `send` gives the control plane's answer where it took the report, with whether the file holds the part's share after `reported`, and none where the report is unsent. Where it gave an answer, the save after it held, and `below_least` is none, the pass runs `Work::confirm(&answer)`.

`Work::confirm` walks the pull's offers in order. It confirms for each launch where all of these hold:

- the report, after `reported` forgot what was let go or refused, holds it, answered `launched` at a place;
- its repository's clone stands, `Keeping::Ready` or `Keeping::Dropped`, so its status is one this pass read from Flume;
- its carried status is listed;
- the answer's `statuses_taken` names it;
- `facts::owed(&entry, flume::RECIPIENT)` gives at least one key, the entry being `carried_entry(launched, place)`.

It runs `Flume::confirm(flume::RECIPIENT, &keys)`, the keys in the status's order. On `Reply::Given` it says nothing and keeps nothing of the answer (core.11). On `Reply::Refused(words)` it says `<id>: Flume refused to confirm the ends of <place> as boomdrive: <words>`. On `Unrun(why)` it says `<id>: the ends of <place> could not be confirmed: <why>`. `<place>` is the entry's own `place`, a location or `<location>@<at>`. Either is made again at a later pass whose report is taken, since the carried status still gives the ends pending.

Once confirmed, the place's entry in Flume's listing changes, so the next pass asks its status again (record 0027). It is carried with the ends pending no more, and nothing is confirmed again. A tidied record whose ends are all confirmed is removed by the settle that ends the pass (core.13). The next pass's status is then refused, and the carried status stands, not listed.

Confirming in the pass that sent the report, after its answer, holds no end across an unreachable control plane beyond what is needed. Waiting for the next pull to give the report back would say no more, since the hosted implementation gives back the share it last sent.

### 4. Which ends are the runner's

`facts::owed(entry, name) -> Vec<String>` gives, in the order of the entry's `ends`, the `delivery` of each end whose `pending_for` names `name`. An end with no `delivery` string is left out: an end with no recipient has no delivery, and Flume records one for an end written before at its next settle. Nothing else of the end is read.

Every end owed to the runner is confirmed, a live place's among them, each stage's, stall's and submission's, and a drop's or a done's. The tidied phase's `pending_for` stays what `report::ended` and the drain read (Decision 6).

### 5. Flume's command line

`Flume::collect` gives way to `Flume::confirm(&self, recipient: &str, keys: &[String]) -> Result<Reply, Unrun>`. `Cli::confirm` runs `flume --json confirm <key>… --recipient <recipient>`, naming no repository, with `FLUME_RECIPIENT` unset as for every call, and reads the answer by Flume's exit. It takes the recipient as Flume's command does. This change passes only `flume::RECIPIENT`. The runner runs no `flume inbox`.

### 6. The drain

`report::ended` is unchanged. A place whose carried record is pending for `boomdrive` has not ended, so a drained clone stays until the runner has confirmed the record's ends and carried the status that follows. Its documentation says confirmed where it said collected.

### 7. The least Flume and the set

`flume::LEAST` is `(0, 21, 0)`. `flume::runs_with`'s words name `0.21.0`, "the first whose ends a recipient confirms by key". Below it no offer is launched, as now, and nothing is confirmed. A host below it keeps its records until its Flume is upgraded. A missing version changes nothing: the launch and the confirmation are tried, and a failure is named.

The set moves to `v0.21.0`: `flake.nix`'s Flume `url`, the lock by `nix flake update flume` under the devenv shell, and `nix/packages/flume-git-crates.nix` holding the four git crates of Flume's Cargo.lock at `52b9552` with the hash Flume's own `git-crates.nix` holds. README's override example moves one tag on, to `v0.22.0`, as the move to `v0.20.0` moved it to `v0.21.0`.

### 8. The tests

All run on the crate's fakes and stubs, with neutral names and paths (core.41).

- `FakeFlume`:
  - Each place it launches, and each record it tidies, holds `ends`. Each end has a `key`, a `delivery` `<name>/t/<key>`, the name the place's last path segment, and a `pending_for`.
  - `tidy_held(place, at, pending_for)` gives the record its landing, `submission/1/landed`, pending for `pending_for`. A drop or a done of a place whose recipients name `boomdrive` gives it `over/dropped` or `over/done`, pending for `boomdrive`.
  - `end(place, key, pending_for)` adds an end to a standing place.
  - `confirm(recipient, keys)` is recorded as `Call::Confirm(recipient, keys)`. Each key takes `recipient` out of its end's `pending_for`, and a tidied record's phase `pending_for` is made again from its ends'. A record pending for nobody is removed, as the settle after would remove it. A key no end is under is refused, `the delivery key <key> was not confirmed as <recipient>: no delivery is under it`, and the keys after it are not confirmed.
  - `confirm_answers(key, reply)` scripts the answer to a confirmation whose first key is `key`, and `confirm_as_flume(key)` lifts it. `confirms()` gives each confirmation's keys.
  - `Call::Collect`, `collect`, `collect_answers`, `collect_as_flume` and `collects` go.
- `Watching` (`tests/once.rs`) passes `confirm` through. Its test `a_held_record_is_collected_once_its_report_holds_it` goes: the file holding an end no longer lets it go. `tests/statuses.rs` proves when it goes, by the fake control plane's answer.
- `the_sets_flume_takes_the_runner_as_a_recipient` (`tests/family.rs`) asks the set's Flume `flume --json confirm fix--none/t/stage/code/1/delivered --recipient boomdrive` on the scratch host, in place of the inbox. It must exit 1 with a JSON refusal, read by `flume::Cli` as `Reply::Refused` with Flume's reason, and not exit 2.

### Rules relied on

- **core.1**: an end is Flume's, carried as Flume gave it, and the confirmation decides nothing. Its details say so.
- **core.2**: what counts as taken is each implementation's, through `Reported`, and the runner has no branch for an implementation.
- **core.10**: nothing is confirmed for a launch whose lease was refused, or which was let go.
- **core.11** and **core.35**: nothing of the confirmation's answer is carried or kept.
- **core.25**: a pass whose share could not be saved after the answer confirms nothing.
- **core.27** and **core.42**: the hosted copy is unchanged, and a status that fails it is not named taken.
- **core.38**: the least Flume.
- **core.54**: this change's rule, amended with it.
- **core.13**: the confirmation is the end of carrying an end and no housekeeping of Flume's. Its details say so.

### Rules departed from

- **core.44**, for Flume, as the change that wrote record 0030 departed from it: the set keeps this flake's recipe for Flume at `v0.21.0`, whose own flake builds it. Taking Flume's package from its flake goes with taking its `services.flume` module, record 0023's, a change of its own.

## Risks / Trade-offs

- **An end stays owed while its control plane cannot be reached.** A tidied record stays on the host, a drained clone with it, and Flume shows the runner's ends expired past `ends_expire`. That is the brief's intent: owed, and shown, in place of lost.
- **Version 1 of the hosted protocol.** A status naming an end `{"of":"over",…}`, or a hand-over `by` `session_start`, fails version 1's copy and goes up null. Under a hosted control plane such a place's ends stay owed until a later version carries them, or a person confirms them as `boomdrive`. crew takes the status whole.
- **A launch let go before its end was taken** leaves the end owed, its record on the host until a person confirms it.
- **One more Flume command for each place with a new end in a pass.** Ends come a few at a time, and a pass with none runs none.
- **The change `the-runner-carries-a-control-planes-confirm`, on the same bolt,** asks Flume's same command for a role's address, and needs Flume `v0.21.0` too. `Flume::confirm` takes the recipient so both run it through one call. Whichever lands on the bolt second takes the first's `confirm`, `LEAST` and set as they stand.

## Migration Plan

None of the runner's files change. A runner of this change, on a host whose Flume is `v0.21.0`, confirms at its first pass whose report is taken each end owed to it in the statuses it carries. That includes the records an earlier runner's inbox handed over without confirming. Ends an earlier runner collected under Flume `v0.20.0` read as confirmed (Flume record 0061). Rolling back to a runner that collects by `flume inbox` leaves its records handed over and owed until a runner of this change confirms them, or a person does.

## Task notes

- **1.1** Move the set as `1a6b349` did: the tag, the lock under the devenv shell, each git crate's entry at the tag, and README's override example. `nix:test` holds each entry's revision to the tag's Cargo.lock.
- **2.1** `a_reports_answer` round-trips an answer naming `home/1` let go, `home/2` refused and `home/3`'s status taken. `a_reports_answer_from_before_reads` reads `{"let_go":[],"refused":{}}` as naming no status taken, and writes it back with no `statuses_taken`.
- **3.1** `write::statuses_held` is the one place that says which launches `places.rec` holds a status of. `texts` uses it.
- **3.2** `a_status_that_passes_goes_up_as_its_copy` asserts `statuses_taken` is `{home/l-1}`. `a_value_flume_adds` asserts both answers name it not taken.
- **4.1** `Flume::confirm` is added beside `collect`. Task 5.2 stops the pass collecting, and task 5.9 removes `collect` once no test names it.
- **5.2** `a_landing_held_for_the_runner` asserts the order of the fake Flume's calls: the status of `P` read, then one `Call::Confirm("boomdrive", [<the landing's delivery>])`, and no `inbox`. It asserts the fake control plane's last report holds the status confirmed.
- **5.3** `the_report_file_lost`: two passes carry `a` live, the second taken; `P` then lands and is tidied, held for `boomdrive`; a pass under `refuse_reports` confirms nothing; the report file is removed; `take_reports`; the next pass takes `a` from the pull, carries `P@<at>` and confirms its landing.
