# Design

## Context

See proposal.md for why. The decision is record 0037 as amended on 2026-11-09, with the details of core.27, core.42, core.53 and core.55, written with this change.

What stands:

- **This branch** was cut from `bolt/the-hosted-protocol-carries-every-end` at `e348c83`, which holds version 2 as the change `the-hosted-protocol-carries-every-end-of-a-place` wrote it. No tag holds `protocol/v2/`: the newest, `v0.7.3`, holds `protocol/v1/` alone. So no release has spoken version 2, and it may still change (core.42).
- **The interface** (`crates/boomdrive-control/src/want.rs`) names five wants, `Offer`, `Act`, `Command`, `Inbox` and `Preview`, and six acts, `Retry`, `Land`, `Confirm`, `Drop`, `Priority` and `Done`. serde writes `ActSort::Land` as `"land"`, and `Want::Preview` as `{"preview":{"id","lease","repository","branch","revision","done"}}`. The report's `previews` (`report.rs`) holds, by id, a `Previewed`: `lease`, `repository`, `revision`, `done`, `outcome` (`held`, `done` or `refused`), `text`, `words`, `status` (a `Carried`) and `at`. The interface writes `previews` only where it holds one.
- **What the runner carries of a preview** (`crates/boomdrive/src/sluice.rs`): of a preview's document, sluice's answer or a row of its listing, a JSON object of `name`, `branch`, `revision`, `state`, `made` and `addresses`, each where sluice printed it; of a release's, `preview` and `released`. A preview's `text` is the first, a done's the second, and a preview's `status.text` the first, from sluice's listing. Its `checkout`, `names`, `ports`, `exit`, `output` and `release_failed` are never carried (core.11, core.55).
- **crew's implementation** gives every want and act the interface names, a land and a preview among them (core.53, core.55).
- **Version 2** names every want but a preview and every act but a land, and its report has no `previews`. `tests/interface.rs` holds that: `a_land_is_no_act_of_version_2` and `a_preview_is_no_want_of_version_2`.
- **The hosted implementation** (`src/plane.rs`) already names a preview's repository in `own_wants`, withholding one the partition is not served, prefixes a preview's id in `prefixed_want`, keeps each part's previews in `part`, `answered` and `merge`, and gives back none in `taken_back`. `upward` writes a report's four other parts and no `previews`.
- **The copy's cut** (`src/copy.rs`) walks a definition of version 2's: a closed object keeps the fields it names, a map keeps each key with its value cut, and a value is kept as the tool printed it. `copy::submission_entry` cuts and checks sluice's entry.
- **sluice 0.74.3's answers of a preview** are the binary's fixtures, `crates/boomdrive/tests/fixtures/sluice-0.74.3/`: `preview-starting.json`, `previews.json` (one preview `up`, one `failed` with its `output`), `release.json`, `release-nothing.json` and `preview-no-table.json`.

## Goals / Non-Goals

**Goals:**

- A hosted control plane asks a land and a preview with its done, as crew's does, in version 2's first release.
- A hosted control plane reads what the runner answered of each preview, sluice's answer and listing among it, as their copies.
- Every want and act the interface names passes version 2, and a test fails when one is added that it does not.
- Version 1 stays as releases spoke it.

**Non-Goals:**

- **The runner's carrying of a land or a preview.** It is the same for every implementation (core.2, core.53, core.55), and does not change.
- **The cloud's side.** boomdrive-cloud takes version 2 by a change of its own.
- **A version 3.** Version 2 is unreleased, so it takes them itself.
- **The report file.** No field of `report.json` changes (core.36).

## Decisions

### 1. Version 2 carries a land

`wants.schema.json`'s `act_sort` takes `"land"` beside `"drop"` and `"done"`, as the interface writes `ActSort::Land`. A land holds nothing but its act's launch and lease (core.53). The retry naming the submission stays, since version 2 is version 1 whole but for what record 0037 lists, and crew's implementation still gives one.

Not taken: a version 3 for it. Version 2 is unreleased (Context), and a third version would have the runner and the cloud hold three directories for one release's worth of difference.

### 2. Version 2 carries a preview

`wants.schema.json`'s `want` takes a fifth form, `{"preview": {"$ref": "#/$defs/preview"}}`. `preview` is closed, each field required, as the interface always writes them:

- `id`: `shared.schema.json#/$defs/id`;
- `lease`: `shared.schema.json#/$defs/lease`;
- `repository`: `shared.schema.json#/$defs/repository`;
- `branch`: a string;
- `revision`: a string;
- `done`: a boolean.

The schema bounds neither the branch nor the revision. A preview whose revision sluice cannot bring up reaches the runner and is answered with sluice's refusal, as a confirm of no key reaches it as an act (record 0037). `hello.schema.json`'s `leases` says it holds each launch's, command's and preview's id, as the interface's `Report::leases` gives them, a description alone.

Not taken: `revision` as `shared.schema.json#/$defs/commit`. A short revision would fail the whole `pulled`, every want of it refused with one, where the runner can answer that preview alone.

### 3. A report holds what the runner answered of each preview

`report.schema.json`'s `report` requires a fifth part, `previews`, a map of ids to `previewed`. `previewed` is closed, each field required:

- `lease`: `shared.schema.json#/$defs/lease`;
- `repository`: `shared.schema.json#/$defs/repository`, as the preview named it;
- `revision`: a string, as the preview named it;
- `done`: a boolean, whether the preview asked that it be put away;
- `outcome`: one of `held`, `done` and `refused`;
- `copy`: the copy of sluice's answer, `preview.schema.json#/$defs/preview` for a preview brought up, `preview.schema.json#/$defs/release` for a done, or `null`;
- `words`: a string or `null`;
- `status`: `carried_preview`, or `null`;
- `at`: `shared.schema.json#/$defs/time`.

`carried_preview` is `{copy, at, listed}`, its `copy` `preview.schema.json#/$defs/preview`, as `carried_status` and `carried_entry` are.

The field the interface names `text` goes up as `copy`, since it holds a copy and no text (core.12), as a launch's status goes up as `{copy, …}`.

`src/protocol/v2.rs`'s `Report` gains `previews: BTreeMap<Id, Previewed>`, and `Previewed` is version 2's own type, its `copy` a `Option<Box<RawValue>>` and its `status` a `Option<Carried>`, reading no field it does not name.

Not taken:

- `previews` left out where empty, as the interface's file writes it. A report of version 2 is whole, every part present, so a control plane reads one form.
- The text carried whole as a string, as a command's `text` is. The cloud's check would hold nothing of it, and core.27 carries of a tool's answer only what the copy names.

### 4. The copies of sluice's answers of a preview

`preview.schema.json`, new, holds two definitions:

- `preview`: closed, `name` required; `name`, `branch`, `revision`, `state` and `made` strings, and `addresses` an object whose values are strings, each as sluice printed it.
- `release`: closed, `preview` required; `preview` a string and `released` a boolean.

The two never pass for one value, since each requires a field the other does not name, so `previewed.copy`'s `oneOf` holds. `state` is any string, so a state sluice adds still goes up. No definition names `checkout`, `names`, `ports`, `exit`, `output` or `release_failed` (core.11, core.27, core.55).

`src/protocol/v2.rs`'s `FILES` lists the file, so every check is built with it.

Not taken:

- The definitions in `report.schema.json`. Each tool's copy has a file of its own, as `place-status.schema.json` and `submission-entry.schema.json` do.
- `state` as an enum of sluice 0.74.3's states. A state sluice adds would fail the copy, and version 2 is fixed from its release.

### 5. What the runner answered of each preview goes up as its copies

`copy::preview` and `copy::release` cut a text by walking `preview.schema.json`'s `preview` and `release`, and check the cut, as `copy::submission_entry` does.

`Partition::upward` writes `previews` beside the four other parts: each preview of the part under its bare id, with its `lease`, `repository`, `revision`, `done`, `outcome`, `words` and `at` as the runner holds them; its `copy`, its text cut by `release` where it is done and by `preview` where it is not, or `null` where it has no text; and its `status`, `{copy, at, listed}`, its status's text cut by `preview`, or `null`.

A copy that fails goes up `null`, and is said once while it fails, as a launch's status or entry is: `<id>: its answer's copy fails version 2's check, so it goes up null: <words>`, or `its status's copy`. The words name the schema's path and never a value (core.11). It is kept in `failing` as `(id, "answer")` or `(id, "status")`. A preview's ids differ from every launch's, so the set holds both.

`statuses_up` reads launches alone, so no preview changes which statuses are taken (core.54).

Not taken: the report refused for a preview's failing copy. One answer of sluice's would hold every other part of the report back.

### 6. A land and a preview reach the runner as any want

A `pulled` holding them passes version 2's check, and `own_wants` gives them as it gives every want. A land is an act: its id and its launch take the partition's prefix. A preview takes the prefix on its id, and one naming a repository the partition is not served is withheld, and said once while it is offered, as an offer, a branch command or an inbox is (core.26). The hosted implementation reads nothing else of either, and the runner carries each out as it carries crew's (core.2, core.53, core.55).

The documentation of `own_wants`, `taken_back` and `upward`, and of `src/protocol/v2.rs`, says version 2 carries both.

### 7. A report handed back holds its previews

`taken_back` gives back each preview the control plane's report holds, under the partition's prefix, with its `lease`, `repository`, `revision`, `done`, `outcome`, `words` and `at`; its `copy` as its `text`, the copy's JSON, where it has one; and no `status`.

A done puts away, by `sluice release <name>`, the name sluice gave the preview, which the runner reads from its text (core.55; the previews spec's "A done puts away what the runner asked sluice for"). The copy holds that name. A runner that lost its report file, and takes the control plane's back, so puts away the preview sluice brought up for it. Its status is read again from sluice's listing at the pass's end, as the previews spec reads a preview with no status carried.

Not taken: no text, as a launch's status and entry are given back none. A done after a lost report file would then ask sluice to release the revision, not the name sluice gave it.

### 8. Every sort the interface names passes version 2

`tests/interface.rs` gains `every_sort_the_interface_names`: a value of every `Want`, every `ActSort` and every `CommandSort`, each named by a match with no `_` arm, and each checked against version 2's `want`. A sort added to the interface fails to build the match until the test gives it a value, and then fails its check until a version names it. That holds the brief's "everything crew's control plane can ask" as the interface grows.

`the_interfaces_values_pass` gains a land and a preview of each kind. `a_land_is_no_act_of_version_2` and `a_preview_is_no_want_of_version_2` are replaced by `a_land_is_an_act_of_version_2` and `a_preview_is_a_want_of_version_2`. Version 1's `a_land_is_no_act_of_version_1`, `a_preview_is_no_want_of_version_1` and `a_confirm_is_no_act_of_version_1` stay, checked against version 1.

### 9. Version 1 stays, and the words follow

Nothing under `protocol/v1/` changes, which `version_1_is_as_released` holds. The protocol's README names a land among a `pulled`'s acts and a preview among its wants, `previews` in a `report`, `preview.schema.json` among the files, and the copies of a preview and of a release; its section **Version 1** says version 1 has no land, no preview and no confirm, and no `previews`. The root README's hosted section says a control plane's land and preview reach the runner as crew's do. `boomdrive-control`'s documentation of `Want::Preview`, `ActSort::Land`, `Preview` and `Previewed` says version 2 carries them and version 1 does not.

### Rules relied on

- **core.42**: a version changes until a release speaks it; version 1, which releases spoke, never does; each end checks what it is sent.
- **core.27, core.11**: a preview's answer and listing go up as their copies, with no command's output.
- **core.12**: the field holding a copy is named `copy`.
- **core.2**: the protocol stays in the hosted crate, and the runner carries a land and a preview by its own code for every implementation.
- **core.26**: a preview naming a repository the partition is not served is withheld.
- **core.53, core.55**: what a land and a preview ask of the runner, unchanged.
- **core.54**: no preview changes which statuses are taken.
- **core.36**: no field of the report file changes.

### Rules departed from

None.

## Risks / Trade-offs

- **[Version 2 is fixed once the bolt lands.]** → This change must land on `bolt/the-hosted-protocol-carries-every-end` before that bolt lands on `main`. The orchestrator orders it.
- **[boomdrive-cloud must take three more shapes.]** → It pins version 1 and takes version 2 by a change of its own, which now holds a land, a preview and `previews`. The orchestrator asks it.
- **[A revision version 2 does not bound.]** → sluice answers a revision it cannot bring up, and the runner carries that refusal as the preview's (Decision 2).
- **[Every report carries `"previews": {}`.]** → A few bytes, for one form of report.
- **[sluice changes a field's form.]** → A field it adds is cut away. A field it prints in another form fails the copy, which goes up `null`, said once (Decision 5).

## Migration Plan

Nothing on a host moves. No release has spoken version 2, so no control plane holds it. No file the runner keeps changes, so the release's notes need name no move by hand (core.36).

## Open Questions

None.

## Task notes

- **1.1** `act_sort`'s first branch becomes `{"enum": ["drop", "done", "land"]}`, and its description names a land. `examples/to-runner/pulled-with-a-land.json` holds an offer `offer-1` and an act `land-1` on it, `"what": "land"`. `a_land_is_an_act_of_version_2` checks a want whose act is a land, written by the interface's types, against version 2's `want`, and the example against `to-runner.schema.json`.
- **1.2** `a_land_carried`, beside `a_confirm_carried`: the fake gives `home` an offer `l-1` and an act `land-1` on it, `"what": "land"`; the part's pull gives `Want::Act` `home/land-1` on `home/l-1`, lease 1, `ActSort::Land`, and no `message_refused` was sent.
- **2.1** `examples/to-runner/pulled-with-a-preview-and-its-done.json` holds a preview `preview-1` of `example/app`, `main` at a full revision, `done` false, then `preview-2`, the same marked done. `a_preview_is_a_want_of_version_2` checks a preview not done, one done, and one whose revision is `R1`, each written by the interface's types, against version 2's `want`, and the example against `to-runner.schema.json`.
- **2.2** `a_preview_carried`: the fake gives `home` `preview-1` and its done `preview-2`, of `example/app`; the pull gives `Want::Preview` `home/preview-1` and `home/preview-2`, each field as given, and no `message_refused`. `a_preview_of_a_revision_that_is_not_a_commit`: a preview of `example/app` at `R1` is given as given, and no `message_refused`. `a_preview_of_a_repository_not_served`, beside `another_partitions_repository`: a preview of `example/service`, which `home` is not served, is not given at two pulls, and one line names it, `home` and `example/service`.
- **3.1** `files.rs`'s checks read every file `FILES` lists, so the new file is checked by them alone. The file's `$id` is under `v2/`.
- **3.2** `copies.rs` gains `PREVIEW` (`name`, `branch`, `revision`, `state`, `made`, `addresses`) and `RELEASE` (`preview`, `released`), and the tests `the_previews_fields_are_the_list` and `the_releases_fields_are_the_list`, as `the_entrys_fields_are_the_list` is.
- **3.3** Each example holding a report gains `"previews": {}`, but `pulled-with-the-report-held.json`, whose report holds one preview brought up. Edit them with `jq` and then `devenv shell -- treefmt` the files. `previews_go_up_as_their_copies` carries, under `home`, a preview brought up whose text is what the runner carries of `preview-starting.json` and whose status is what it carries of `previews.json`'s `up` row, its done answered with what it carries of `release.json`, and a preview refused with sluice's words and no text: the report passes, and its `previews` hold each bare id, the first's `copy` and `status.copy` the six fields, the done's `copy` the two, the refused's `copy` and `status` `null`.
- **3.4** From-runner examples to add: `report-a-preview-brought-up.json`, `report-a-preview-held.json` (outcome `held`, `copy` `null`, its status sluice's row `waiting`), `report-a-preview-put-away.json` (the bring-up and its done) and `report-a-preview-sluice-refused.json` (outcome `refused`, the words of `preview-no-table.json`'s error). `sluices_previews_as_copies` checks each against `from-runner.schema.json`, and that each preview's `copy` is an object or `null`, as `flumes_answers_as_copies` does.
- **3.5** `a_previews_copy_that_fails`: a preview whose status's text holds `"made": 1793750400`, a number: its `status` goes up `null`, its `copy` as before, the report passes and is sent, and one line names the preview's id and the path, and holds no `1793750400`. A second report says nothing more.
- **3.6** `sluices_previews_cut`, beside `sluices_entries_cut`, reads `sluice-0.74.3`'s `preview-starting.json` and each row of `previews.json`, cut by `copy::preview`, and `release.json` and `release-nothing.json`, cut by `copy::release`: each passes its definition; no preview's copy holds `checkout`, `names`, `ports`, `exit` or `output`; and each copy's `state`, or `released`, is the fixture's. Three previews and two releases are cut.
- **3.7** `a_previews_output` alters `report-a-preview-brought-up.json`, adding to its preview's `copy`, then to its `status.copy`, each of `output`, `checkout`, `names`, `ports`, `exit` and `release_failed`, and checks each refused. `a_field_the_version_does_not_name` adds a field to a preview want, a `previewed`, a preview's copy and a release's copy. `a_previews_outcome`, beside `an_acts_outcome`, checks a `previewed` whose outcome is `launched` refused.
- **3.8** `a_report_taken_back_holds_its_previews`, beside `a_report_taken_back_holds_no_status`: the fake holds, for `home`, the report of `report-a-preview-brought-up.json`; the pull's report holds `home/<its id>` with the example's lease, repository, revision, done, outcome, words and at, its `text` a JSON object whose `name` is the copy's, and no `status`.
- **4.1** `every_sort_the_interface_names` builds its values in a function that matches on each `Want`, then each `ActSort` and `CommandSort`, with no `_` arm, naming each sort; the test checks the names it built are every sort's, and each value passes version 2's `want`.
- **5.1** `the_readme_describes_version_2` reads, beside what it reads now, `"land"` and `preview` in the messages table, `previews` in the `report` row, `preview.schema.json` in the files table, and, in the section **Version 1**, that it has no land and no preview. The README's sentence "and no land and no preview among a `pulled`'s wants" goes.
- **5.2** The hosted section says, after its confirm: "A control plane's land and preview, with its done, are given to the runner as crew's are, and what the runner answered of each preview goes up with sluice's answer and listing as their copies." `the_readme_names_version_2_of_the_protocol` reads "land and preview" and "as their copies" there.
