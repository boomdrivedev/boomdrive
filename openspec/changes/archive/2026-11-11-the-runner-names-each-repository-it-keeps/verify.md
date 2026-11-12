# Verify: the-runner-names-each-repository-it-keeps

Verified on `change/the-runner-names-each-repository-it-keeps` at
`b5b03d5c`, cut from `57b1d1e9`. The code stage archived the change by
task 7.1, so the change and this report are in
`openspec/changes/archive/2026-11-11-the-runner-names-each-repository-it-keeps/`.

## The whole checks

`devenv shell -- sluice exec main` ran once, on the branch as it stood.
It exited 0, with 21 tasks passed and 941 tests passed, none failed.
sluice ran the checks that `b5b03d5` affects against `main` (`57b1d1e`),
and recorded each result under its key:

- `boomdrive`, `boomdrive-control`, `boomdrive-crew` and
  `boomdrive-hosted`: format, lint, typecheck and test, each passed;
- `nix`: typecheck and test, passed;
- `root`: format, specs and changes, passed.

moon also warned that it could not archive `boomdrive:test`'s outputs
into its own cache. That is moon's cache. The task passed.

## Tasks

Every task in `tasks.md` is ticked. The gate's run passed every check
the tasks name:

| task | check | outcome |
|---|---|---|
| 1.1 | `boomdrive-control` `json`: `a_hello`, `a_report_from_before_reads` | ok |
| 1.2 | the workspace's type check (`*:typecheck`) | ok |
| 2.1 | `boomdrive` lib: `carry::tests::each_drive_names_its_repositories_whose_clones_are_made`, `…a_repository_is_kept_as_the_configuration_and_the_clones_have_it` | ok |
| 2.2 | `boomdrive` `offers`: `repositories_kept`, `each_drives_repositories_with_their_folders`, `a_repository_draining_in_the_hello` | ok |
| 3.1 | `boomdrive-hosted` `plane`: `each_hello_its_own`, `no_drive_or_folder_goes_up`; no file under `crates/boomdrive-hosted/src` or its protocol changed | ok |
| 4.1 | `boomdrive-crew` `write`: `the_first_report`, `a_drives_repositories_and_their_folders`, `nothing_changed`, `only_the_runners_branch`, `each_file_reads_as_crew_reads_it` | ok |
| 4.2 | `boomdrive-crew` `write`: `a_repository_the_drive_comes_to_keep`, `a_repository_the_drive_no_longer_names` | ok |
| 4.3 | `boomdrive-crew` `claims`, the whole file, including `a_claim_names_the_drives_repositories` and `a_launch_in_a_repository_another_drive_keeps` | ok |
| 4.4 | `boomdrive-crew` `write`: `a_label_no_drive_has` | ok |
| 5.1 | `boomdrive` `control`: `the_runners_branch_names_each_repository_of_its_drive`, `one_control_pass` | ok |
| 6.1 | `boomdrive` `readme`: `the_readme_names_the_repositories_each_drive_keeps` | ok |
| 7.1 | `root:specs`, after the archive | ok |

## Specs

Each requirement and scenario of the change's deltas is met, as read in
the code and proved by the test named.

**work-into-flume: "Every pull's hello names each drive's repositories
with their folders."** `Configured::new` keeps each drive of the
configuration with its addresses in the file's order. `named` is still
the flattening of the drives that `Config::repositories` was.
`Clones::drives` gives each drive with each address whose clone is
found, at its container, and a drive with none made is given with none.
`Carrier::pass` sets `hello.drives` beside `draining`. "Each drive's
repositories with their folders" is proved by `carry.rs`'s unit test,
which covers `idle` with none and `example/kit` in no drive, and by
`offers.rs`'s pass over `FakeClones`, which leaves out a clone not yet
made and one dropped.

**control-plane-interface: "A hello names each drive's repositories, and
reads with none where it names none."** `Hello::drives` is
`#[serde(default)]`, and `KeptRepository { address, container_dir }` is
exported. "A hello naming its drives" is proved by `a_hello`'s round
trip, with `home` naming `example/crew` and `work` naming none. "A hello
from before its drives" is proved by `a_report_from_before_reads`.

**crew-control-plane:**

- "repositories.rec names exactly the repositories the partition's drive
  keeps". `write::REPOSITORIES` matches crew's `plugin/lib/runner.py` at
  `323c77fb` byte for byte, read from crew's own clone on this host.
  `repositories_text` writes one `Repository` and `Folder` per kept
  repository, in the hello's order. `kept_of` and the report take the
  drive named by the label alone.
  - "A drive's repositories and their folders" is proved by
    `a_drives_repositories_and_their_folders`: the exact text, no
    `example/app`, and `recfix` takes it.
  - "A launch in a repository another drive keeps" is proved by
    `a_launch_in_a_repository_another_drive_keeps`: offer `home/20` in
    `example/app`, and the file names `example/crew` alone.
- "repositories.rec is written at every write of the runner's branch".
  The branch has three writes, at `lib.rs` 620, 683 and 1029: a claim, a
  withdrawal and a report. Each writes `repositories` from the drives of
  its hello, or of the latest pull. `what` names
  `repositories <address>…` for each record added, changed or removed.
  - "A repository the drive comes to keep" is proved by its test of the
    same name.
  - "A repository the drive no longer names" is proved by its test of
    the same name. Each asserts the tip's subject
    `repositories example/kit` and the file's text.
  - "A claim names the drive's repositories" is proved by
    `a_claim_names_the_drives_repositories`.
- "A partition whose label names no drive names no repository".
  - Where the hello has no drive by the label, the pull says the line,
    with `none` where the hello names no drive. Crew's `say_lines` says
    it once while it stands. The file is the descriptor alone, and the
    partition's launches go on.
  - "A label no drive has" is proved by `a_label_no_drive_has`, which
    asserts the line word for word.

**hosted-control-plane: "No hello names a drive or a clone's folder."**
The hosted implementation builds version 2's hello field by field. Its
source and protocol are unchanged. "No drive or folder goes up" is
proved by `no_drive_or_folder_goes_up`.

## Rules

- core.2: no source outside `crates/boomdrive-crew` names
  `repositories.rec`, crew or a partition.
- core.42: no protocol file changed.
- core.36: no field of `report.json` changed.
- core.17: no configuration key was added.
- core.41: the tests use neutral names and `/data/repositories/…` paths.

## Notes

- The proposal's Touches lists `crew/tests/drops.rs`, `letgo.rs`,
  `read.rs`, `roles.rs` and `hosted/tests/connection.rs`. They are
  unchanged: their hellos come from the shared helper `hello`, which now
  names the drive `home`. Nothing outside Touches changed.
- The design names one follow-up, which this change leaves as it stands:
  a drive's kit in a repository another drive keeps (Risk 1).

## Outcome

The whole checks pass, and the work meets every requirement, scenario
and task. The change stays archived with this report.
