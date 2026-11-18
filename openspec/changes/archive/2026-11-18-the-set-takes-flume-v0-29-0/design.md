# Design

## Context

Flume v0.29.0 is Flume's newest release. Its flake and lock are as at v0.26.0, so the set takes it through Flume's own flake as before, and its tree pins herdr 0.9.5, the set's herdr. What changes for the set is Flume's module: since v0.28.0 a label's table declares `owed` and `reserved` and no `reserve` or `keep_free`, a stream's and the host's `agents` are read from 1, and an account declares no `usage` (Flume record 0063 and record 0062). The set gives Flume's module a drive's stream's `agents`, `priority` and each label's `reserve`, and none of an account's keys.

## Decisions

### Decision 1: the drive's label option is `owed`

The drive's option `stream.labels.<label>.reserve` is renamed `owed`, with the same defaults, `operation` 1 and `planning` 2, and given Flume as the label's `owed`. Flume's record 0063 names Boomdrive's drive option `owed` with Flume's, and core.12 rules out a key taking a word another tool uses for something else: `reserve` beside the family's `reserved` reads as its opposite. A label's `reserved` stays Flume's own setting, as `keep_free` was, since record 0028 gives the drive no second option for each of Flume's keys (core.46).

Not taken: keeping `reserve` as the drive's word and giving Flume `owed`, for core.12's reason.

### Decision 2: the old word fails evaluation, naming `owed`

The label's submodule declares `reserve`, hidden, null by default, and `drives.nix` fails evaluation where any drive's label sets it, naming each such option and `owed`. The failure is outside the runner's `enable`, as the set's removed options fail whatever is enabled. A label that sets `reserve` is given Flume nothing, so Flume's own checks never read an `owed` it lacks.

Not taken: nix's own failure for an undeclared option, which names the option and no replacement; `lib.mkRemovedOptionModule`, which sets `assertions`, an option a submodule has not; an alias taking both words, which Flume's record keeps none of.

### Decision 3: a stream's agents are read from 1

The drive's `stream.agents` takes `types.ints.positive`, as Flume's option does at v0.29.0 (Flume's 143), so a drive of 0 agents fails at its own option, which names the drive.

Not taken: `types.ints.unsigned`, which passes 0 on for Flume's option to refuse, naming Flume's option.

## Risks and trade-offs

A host that sets a drive label's `reserve` fails evaluation until it writes `owed`. The failure names each option to change and the word to write.
