# Design

## Context

See proposal.md for why. This change builds record 0035: the runner reads only its own names, and each host moves its data by hand before it runs the release. core.36 as amended and core.10's details already say it.

What stands, at `cee700c`:

- `crates/boomdrive/src/lib.rs` holds `BEFORE_RENAME`, the runner's name before record 0016. Three readings use it, and nothing else does.
- `config::data_dir` (`src/config.rs`) gives `<xdg data>/boomdrive`, unless that path is not there and `<xdg data>/flywheel` is a directory, when it gives that one. Its unit tests are `the_file_and_the_data_directory`, of the paths, and `a_data_directory_from_before_the_rename_is_kept`, of the fallback.
- `clone::find` (`src/clone.rs`) runs `carry_over` on every container before it reads `recorded`. `carry_over` moves `refs/flywheel/view` to `view::COMMIT` and renames the git config section `flywheel` to `SECTION`, `boomdrive`. `SECTION` has no other use. `recorded` reads `ADDRESS`, `boomdrive.address`, alone. A container `recorded` does not name is a folder the runner did not make: `a_folder_already_there` (`tests/clones.rs`) proves it is left and named with `<address>: left: <container> is not a clone the runner made, so none is made`.
- `carry::marks` (`src/carry.rs`) gives `[mark(id), "flywheel:<id>"]`, and the offer path takes the first that `facts::marked` finds in the listing. It says `<id>: launched at <place>, as its mark <mark> shows`.
- Flume 0.4.1's fixtures (`tests/fixtures/flume-0.4.1/`) were read with each place marked `flywheel:scratch/<n>`, as their README says. `tests/acts.rs`'s `place`, `tests/facts.rs`, `tests/once.rs`'s `a_launch_found_by_its_mark`, `tests/statuses.rs`'s `stalled` and `tests/acts.rs`'s `a_land` read those marks. The acts tests find each launch through `marks`.
- `tests/views.rs` has `a_clone_made_before_the_rename`, of the carry-over, and `a_new_root_while_sluice_keeps_the_default_branch`, whose clone is set up as one made before the rename though its point is a guarded `main` under a new repository root.
- README's section "A host from before the rename" says the three readings.
- The specs: clones' "A data directory from before the rename is kept", "A clone's records from before the rename are carried over" and the scenario "A new repository root while sluice keeps the branch"; work-into-flume's "A launch marked before the rename is found by its old mark" and "The runner acts only on its own places".
- git-cliff makes each release's notes from the subjects of the Conventional Commits since the newest tag (`cliff.toml`; core.48). A subject marked breaking, `<type>(<scope>)!:`, goes under "Breaking changes", linked to its commit. The newest tag is `v0.7.3`.
- `nix/`, the other crates and the specs outside these say nothing of the old name.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches.

## Goals / Non-Goals

**Goals:**

- No path, ref, git configuration key, mark, constant, test, fixture mark, spec or README section of the runner or the set reads or says the old name.
- The runner's data directory, its clones' records and its launches' marks are each read only under `boomdrive`, and a test proves each "only".
- The release that drops the readings names in its notes the move each host makes by hand (core.36).

**Non-Goals:**

- **The move on each host.** It is ops' chore on each host, with the runner stopped and no place live, as record 0035 says. It is no code and no task here.
- **A guard against a host that has not moved.** Record 0035 rules it out (its option 3; core.36's details).
- **The records, the rules and the archive.** `docs/adr/`, `docs/architecture/core.md` and `openspec/changes/archive/` record what was, and keep the old name where they quote it. Record 0035 and core.36 are already written, so this change writes no record and no rule.
- **A gate check for the old name.** Nothing is added to the gate. core.36 is a rule a reviewer holds every diff to, as core.41 is (the change `what-main-adds-takes-the-neutral-names`).

## Decisions

### 1. The data directory is Boomdrive's alone

`config::data_dir` gives `xdg(data_home, home, ".local/share").join("boomdrive")` and reads nothing on disk. Its doc says that, citing record 0035. The unit test `a_data_directory_from_before_the_rename_is_kept` becomes `the_data_directory_is_the_runners_own`: under a temporary `XDG_DATA_HOME` that holds a folder `other` with a `repositories/` and a `control/report.json`, the data directory is `<under>/boomdrive`, both before and after that folder is made.

### 2. A clone is the runner's only by its own records

`clone::carry_over`, its call in `find`, and `SECTION` go. `find` reads each container's `recorded`, as it did after the carry-over, and its doc no longer speaks of the rename. A bare repository that records its address under another section is a folder the runner did not make, which core.14 and the clones spec already have left as it is and named. The new test `a_clone_recording_its_address_under_another_name` (`tests/clones.rs`) proves it. A bare `flume.git` in `agentplot/flume`'s container records `other.address` as `agentplot/flume`. A pass keeping `agentplot/flume` says `a_folder_already_there`'s line for its container and makes no view. The bare repository's config still holds `other.address` and no `boomdrive.address`.

`a_clone_made_before_the_rename` goes. `a_new_root_while_sluice_keeps_the_default_branch` drops the setup that renamed its clone's records. Its clone is the runner's own, its `main` guarded and behind origin's, and every assertion it makes stays.

### 3. A launch's place is found only by `boomdrive:<id>`

`carry::marks` goes. The offer path asks `facts::marked(listing, &mark(id))` alone, and its line stays `<id>: launched at <place>, as its mark boomdrive:<id> shows`. `BEFORE_RENAME` goes from `lib.rs`, since nothing then reads it. The new test `a_mark_the_runner_does_not_give` (`tests/once.rs`) proves the "only". Flume 0.4.1's listing has the mark `boomdrive:scratch/4` replaced by `other:scratch/4`, and an offer `scratch/4` with no answer is pulled. The offer is not answered `launched` at `fix--tabs`'s place, and no line says the mark `other:scratch/4`.

### 4. The fixtures carry the runner's mark

Record 0035: "A fixture keeps a tool's own words as that tool printed them, except a mark the runner gave the tool, which becomes the runner's own mark." In every fixture of `tests/fixtures/flume-0.4.1/`, each `"mark":"flywheel:scratch/<n>"` becomes `"mark":"boomdrive:scratch/<n>"`, byte for byte otherwise. The README's commands give `--mark boomdrive:scratch/<n>`. Its sentence before the table says each place carries the runner's mark, `boomdrive:scratch/<n>`, the one word of Flume's answers that record 0035 rewrote in these fixtures. The tests that read the marks read the runner's: `acts.rs`'s `place` and `a_land`, `facts.rs`'s marks, `once.rs`'s `a_launch_found_by_its_mark` and `statuses.rs`'s `stalled`.

`a_launch_found_by_its_mark` keeps what it proves over Flume's own listing: `scratch/4` is found at `fix--tabs`, and `scratch/1` at the place `fix--parser`'s tidied record `fix--parser@<at>` stood, and nothing is launched. Its doc no longer speaks of the rename. It now proves work-into-flume's "A launch found by its mark" over Flume's own listing, as `a_launch_whose_answer_was_lost` proves it over the fake's.

### 5. The release's notes name the move

core.36 has a release that drops a field the release before read name in its notes the move each host makes by hand. core.48 makes the notes git-cliff's alone, from each commit's subject. So each task that drops a reading, Tasks 1.1, 1.2 and 1.4, is committed on its own, with a subject marked breaking that names its part of the move. The body of each commit carries record 0035's chore whole. The notes link each subject to its commit. The subjects are in tasks.md's notes.

Not taken: notes written by hand, or a step of the release workflow's, which core.48 rules out. A README section on the move, which record 0035 removes and which is not the release's notes. One breaking commit for all three readings, which would make one task of three checks.

### 6. No test says the old name

Each test that proved a reading goes, or proves its inverse with the neutral name `other`. No test sets up a folder, a git configuration section, a ref or a mark under the old name (record 0035; core.41's neutral names as the model).

## Rules relied on

- **core.36**, as amended by record 0035: the readings are dropped in this release, with the move named in its notes (Decision 5). Nothing reads the old name to refuse to start (Non-Goals).
- **core.10**: a launch is found by its mark `boomdrive:<id>` alone across restarts (Decision 3). Its details, under record 0035, find no place by an old mark.
- **core.14**: a folder the runner did not make, a clone recording another section among them, is never moved or rewritten (Decision 2).
- **core.35**: the data directory is the runner's own (Decision 1).
- **core.48**: the notes are git-cliff's, from Conventional Commits (Decision 5).
- **core.41**: tests and fixtures use neutral names (Decisions 2, 3 and 6).

No rule is departed from.

## Risks / Trade-offs

- **A host that runs the release before it moves** starts with an empty data directory and no report. It clones every repository again and loses its record of the acts it carried out (record 0035, Bad). The notes' breaking entries name the move, and the chore's order, with the runner stopped, prevents it.
- **A place still marked with the old mark** is no longer found by it. The chore runs with no place live, so none is left to find (record 0035).
- **A fixture's mark rewritten** makes the fixtures differ from what Flume printed in one word. The fixtures' README names that word, and the tests read every other byte as Flume printed it.
