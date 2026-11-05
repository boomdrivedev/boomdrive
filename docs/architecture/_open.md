# Code that stands against a rule

What the tree holds today against a rule of `core.md`, each with the
change that brings it to the rule. A rule still stands while its line is
here; this page is deleted once it is empty.

- **core.29.** `crates/flywheel/src/terminal.rs` relays a terminal by
  running herdr's command line, and `crates/flywheel/src/hello.rs` runs
  `herdr --version`; the terminal-relay spec requires both. Record 0004
  takes the relay out until Flume relays one. The change: the relay
  removed, a terminal closed with words saying the host's Flume relays
  none.
- **core.40.** Every line the running runner writes, in `run.rs`,
  `pass.rs` and `carry.rs`, has no time. The change: each line starts
  with its time.
- **core.35.** A brief left by a launch killed while Flume read it
  stays after the restart (`crates/flywheel/src/flume.rs`). The change:
  a start removes the briefs no launch holds.
- **core.38.** Only git's floor is named, as a start's check; no floor
  is named for Flume, sluice or the multiplexer. The change: each
  release names its floors and the versions it was checked against.
- **core.39.** No command takes `--json`. The change: the local
  commands, each with its JSON form.
- **core.41.** Tests and fixtures name the operator's hosts and
  partitions and hold home paths; the specs, the archived changes,
  record 0002 and README name them too or quote a private repository.
  The change: the scrub, before the public repository's fresh history.
