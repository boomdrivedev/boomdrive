# Design

## Context

See proposal.md for why. This change brings the tree to core.41 and core.6 so the public repository can start from it (records 0012, 0014, 0025). It changes no behaviour of the runner.

What stands, read on 2026-11-07:

- **Which repositories are private.** By GitHub's visibility that day: agentplot/boomdrive-cloud (once agentplot/flywheel-cloud), afterthought/swancloud, afterthought/blueprints, agentplot/flywheel-next, agentplot/strategy, the operator's state repositories, and the repositories of the operator's other accounts and of their employer. Public: agentplot/flume, sluice, crew (whose old address under a personal account redirects there), weir, timber, boom, blueprints, boomdrive-web and flywheel. This repository becomes public, so its own records, rules and specs may be quoted in it.
- **The accounts.** `agentplot` is the family's organization. Every other owner the tests and docs give an example repository is a personal GitHub account.
- **Where the private names are.** crew's tests and the unit tests in its sources (`crates/boomdrive-crew`), the interface's tests (`crates/boomdrive-control/tests`), the runner's tests and the unit tests in its sources (`crates/boomdrive`), the README's examples, five capabilities' scenarios, the fixture READMEs, records 0002, 0006, 0009, 0015, 0016, 0017, 0022, 0023 and 0025, all of `design/`, and the archived changes. The hosted implementation's crate already uses neutral names: partitions `home` and `work`, the machine `host-a`, the repository `example/app`, places under `/home/runner/`. The nix tests already use the user `alice`.
- **The fixtures.** `crates/boomdrive/tests/fixtures/flume-0.4.1/` holds nineteen answers read on a scratch host, whose every path begins with a scratch directory that holds the operator's home path in its folder's name, and one, `status-live-agent.json`, read on the operator's own Flume under the operator's configuration. `crates/boomdrive/tests/fixtures/sluice-0.61.1/` holds three answers read on a scratch host, `branch-*.json`, which hold no path, and four, `status-*.json`, read on the operator's own host. The tests that read them are `crates/boomdrive/tests/facts.rs`, `flume.rs`, `after.rs`, `pass.rs`, `sluice.rs`, `entries.rs`, `branches.rs` and `fakes/mod.rs`, `crates/boomdrive-crew/tests/common/mod.rs`, and `crates/boomdrive-hosted/tests/cut.rs` and `plane.rs`. Most read a path from an answer rather than writing it out; `facts.rs` writes the live agent's pane and socket, and `sluice.rs` and `entries.rs` write the host entries' values.
- **The research transcripts.** `design/research/the-constitution-interview.md` restates about twenty of the cloud's rules and its research rows close to word for word, with the cloud's commit and file paths, and its roadmap says of the public opening: "These notes stay here and are not carried over." `design/research/the-flakes-configuration-holds-streams-and-repositories.md` is the talk about the operator's own host, its streams, owners, accounts and machines, many of them inside the user's quoted words. Records 0003 to 0014 cite the first; records 0009 and 0023 and the flake's design cite the second.
- **The protocol.** Version 1 under `crates/boomdrive-hosted/protocol/v1/` has been spoken by a release (tags `v0.1.*`), so core.42 forbids changing it, and it holds nothing private.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches. It relies on core.41 and core.6, which say what the tree may hold; core.42, which keeps the protocol's version 1 as it is; core.45, which keeps older versions' answers as the runner's unit fixtures; and core.7, which keeps the fixtures excluded from treefmt.

## Goals / Non-Goals

**Goals:**

- No host, partition, state repository, person's path or account of anyone's in the tree's code, tests, fixtures, specs or docs.
- No text of a private repository copied into the tree; each cited by its name and an id at most.
- Every fixture read on a scratch host, its paths neutral, and its README saying how.
- The specs reconciled through this change's deltas, and every scenario still proven by the test that proved it.

**Non-Goals:**

- **A check that the tree holds nothing private.** Any such check would have to hold the private names it looks for, which core.41 rules out, and the brief adds no check, test or task. core.41 stays a rule a reviewer holds each diff to.
- **The opening itself.** The fresh history, the rename to `agentplot/boomdrive-archive`, and the site's move (record 0025) come after this lands.
- **crew's own scrub** of its README and specs (record 0014's consequences), which is crew's.
- **The protocol's version 1**, unchanged (core.42).
- **The rules' text.** core.13 to core.41 adopt wording the cloud's research proposed for this repository. Each is this repository's own rule, made by an accepted record, and stays as it is. Not taken: rewording rules the user accepted, which would change records' decisions to scrub text that was written for this repository in the first place.
- **The deciders' names** in the records' `decision-makers`. core.41 names hosts, partitions, state repositories, people's paths and accounts, and a person's name is none of them.
- **The word customer.** core.6 rules out a customer named, and the records' generic "the customer picks the repositories" names none; nor is a marketplace a price, a tier or a customer.
- **The cache's secret names.** The workflow `cache` and the flake's archived change name the 1Password vault, item and service account the cache's token is read through. core.4 allows the name a credential is read from, core.47 the service account, and the workflow cannot run without them.

## Decisions

### 1. One vocabulary of neutral names, the hosted crate's

Every private value takes the neutral name for its role, the same in code, tests, specs and docs:

| role | neutral name |
|---|---|
| the partition of crew's example table, and every id, branch, stream, intent, recipient, place and run built from its label | `home` (`home/12`, `home/main`, `home/runner/host-a`, `home/planner`, `home-main`, `crew-planner--home`) |
| the second partition, where a test serves two | `work` |
| the host crew's tests and the runner's tests run as, the first in name order | `host-a` |
| the second host of crew's claim tests | `host-b` |
| the state repository of `home`, and of `work` | `example/home-state`, `example/work-state` |
| a state repository several partitions share, where a doc names one | `example/crew-state` |
| a repository a personal account owns: crew's old address, a blueprints repository, a repository named to be missing | `example/<its name>`, such as `example/crew`, `example/blueprints`, `example/home-blueprints`, `example/nowhere` |
| that owner, where a workspace or a folder is named by it | `example` (`/h/Code/example/example.code-workspace`) |
| a Flume pool in the runner's fakes, and its accounts | `pool-a`, `pool-a-1` |
| a label refused for its space | `ho me` |
| an agent or a pane named after a partition's role | the role, such as "the design agent" |

They are the names the hosted crate's tests use, so the tree reads one vocabulary. Where a test needs a name the rest of the answer cannot hold, as the hello test that holds the pool's name appears nowhere, the name is one no other value contains (`pool-a`, not `home`). The table names roles, not the values it replaces: this design is in the tree too.

Alternatives:

- **Fresh names per crate.** Not taken: two vocabularies for one thing.
- **Names that read as real machines or teams.** Not taken: a neutral name should read as nobody's.

### 2. A scratch host's answers keep every byte but the scratch directory's prefix

Each of Flume 0.4.1's nineteen answers read on a scratch host is kept as Flume printed it, but for the prefix of every path, from `/private/tmp/` through `scratchpad/host/`, which becomes `/private/tmp/scratch/host/`. The prefix is replaced in every form an answer gives it: in a path, in a command Flume printed, and in a state folder's name, where Flume joins the path's parts with `%`. Nothing else is touched: no field, no time, no digest, no newline. A digest Flume made of a path keeps its old value; it names nothing.

This is what record 0014 asks, "captured on a scratch host with its paths made neutral". It departs from the archived change `the-runner-carries-crews-work-into-flume`, whose Decision 21 kept each answer byte for byte, and from record 0016's "What keeps its text", which said the same through the rename; record 0016 is amended to say so (Decision 6). The README of `flume-0.4.1/` says which bytes changed and how.

Alternatives:

- **Read all nineteen again on a scratch host rooted at a neutral directory.** It would keep each answer byte for byte. Not taken: the scenario behind them, a dozen places in as many states under one Flume, would have to be played again, and every time, head and id the tests read would change, for paths only.

### 3. The answers read on the operator's own host are read again on a scratch host

`flume-0.4.1/status-live-agent.json` and `sluice-0.61.1/status-landed.json`, `status-failed.json`, `status-withdrawn.json` and `status-missing.json` are read again, at the same versions, on a scratch host:

- **The tools** are built from their tags, Flume `v0.4.1` (`78cf178`) and sluice `v0.61.1` (`074b0f5`), outside the tree, and run by their paths; nothing is installed.
- **The scratch host** is rooted at `/private/tmp/scratch/host/`, as Decision 2's prefix is: `HOME`, `XDG_CONFIG_HOME`, `XDG_STATE_HOME` and `XDG_DATA_HOME` under its `home/`, laid out as the scratch captures lay them out, a `PATH` and a git configuration of its own, and a forge of bare repositories beside it. No place, configuration, session or account of the user's is read or touched, and the root is removed once the answers are kept.
- **Flume's live agent.** Flume's configuration names herdr as its terminal, its server's session under the scratch `HOME`, a pool of one account `stand-in-1`, no bound on the host and no stream but `default`. A place is launched with no mark, its agent Flume's stand-in built from the same tag, in a pane of that server. Flume 0.4.1 types the prompt into a herdr pane, while its stand-in acts only on a prompt on its command line, so when `flume --json status <place>` is read the stand-in's session has ended with no prompt, and the place shows the pane it ran in. The answer, Flume 0.4.1's own and unrewritten, shows what the tests read: one place, live, no mark, its `clues.shown_at.agent` a herdr pane at a socket under the scratch root, and a host part unbounded with the one stream `default`.
- **sluice's entries.** sluice's peer is named `host-a`: sluice 0.61.1 names the host that ran a check by the machine's hostname, so its entries are read in a Linux container with no network whose hostname is `host-a`, rooted as the scratch host is. Its command runner keys a check by the commit it ran on, and a combination is a commit sluice forms with fixed words, author and time, so a worktree checked out at that combination gives a session's run on the queue's key. Its submissions show what each entry showed: one landed on `main` with its `landed_as`; one failed whose one failed row holds its task's `log` and a `flaky` reading whose `other` names a worktree; one withdrawn by its resubmission, whose failed row's `flaky` reading's `other` names a log; and `sluice status <n> --json` for a number no submission has, exiting 1.

The tests take the new answers' values where they write them out (`facts.rs`, `sluice.rs`, `entries.rs`), each test proving what it proved. Each README says how each answer was read, naming no machine.

Alternatives:

- **The same answers with their values rewritten.** Not taken: core.41 rules out a fixture captured anywhere but a scratch host, and a rewritten answer is no tool's.
- **The answers read again at the versions the flake pins now.** Not taken: the tests are written to 0.4.1's and 0.61.1's shapes, and core.45 keeps an older version's answers as the runner's unit fixtures.
- **The live agent dropped.** Not taken: it is the one answer that shows a pane, which `facts::pane` reads.
- **A stand-in from a later Flume tag, one that takes a typed prompt.** Not taken: the fixture would be no single version's answer.

### 4. A private repository is cited, and its text is not copied

Where a record, a doc or an archived change quotes a private repository, or restates its record, rule, ledger, research row, roadmap or design close to word for word, the passage becomes a citation, with the point said in this repository's own words where the passage needs it. A citation names the repository and what it cites: a record's, rule's, row's or item's id, or a file's path. What the cited text holds beyond that, its wording, its commit, its layout, its counts, its internal names and its own implementation, goes. A summary in this repository's own words of what a cited rule decides, citing it by id, stays, as the records' "herdr.13 in afterthought/swancloud says the views timer moves a host's default branch after its fetch" does. The user's own words stay, as core.41 allows, except a private name inside them, which is cut with `[...]`. A line that says its file copies no private text is true once the file is scrubbed.

A state repository is no citation: its name, its branches and its signal ids leave a record's sources, which keep the user's words, their time and where they were said in the family's own terms ("in a main level").

### 5. The research transcripts stay in the archive

`design/research/the-constitution-interview.md` and `design/research/the-flakes-configuration-holds-streams-and-repositories.md` leave the tree. The repository kept as the archive holds them, so a record that cites one cites it there, as agentplot/boomdrive-archive's `design/research/<file>`, a private repository named with the path as its id. The records already quote the user's words each decision rests on.

The interview's own roadmap planned this ("These notes stay here and are not carried over"), and the flake talk is about the operator's own host throughout.

Alternatives:

- **Scrub both in place.** Not taken: the interview's restatements of the cloud's rules are its findings, and cut to citations it would no longer be the dated record of what was asked and said; the flake talk would lose most of its words to `[...]`.

### 6. The records are amended where they stand

Each record is amended surgically, its decision unchanged:

- Quoted or restated private text becomes a citation (Decision 4): records 0002, 0006, 0015 and 0017.
- Hosts, partitions, state repositories and accounts take Decision 1's names or words: records 0017, 0022 and 0023.
- An agent named after a partition is named by its role, in the text and in `decision-makers`: records 0009, 0017, 0022, 0023 and 0025.
- A cited research transcript is cited in the archive (Decision 5): records 0003 to 0014, 0009's amendment and 0023.
- Record 0016's "What keeps its text" says that those files kept their text through the rename, and that record 0014's scrub makes their private names and paths neutral and moves the interview to the archive.

No rule changes: core.41 and core.6 already say what the tree may hold.

### 7. The flake's design is scrubbed, and its sketch of one host becomes a neutral one

`design/the-flakes-configuration/design.md` keeps its design, with neutral owners and streams and no host named. The Nix sketch beside it, named for the operator's host, becomes `host.nix`: the same options in the same shape, with the family's public repositories and `example`'s, streams `family` and `work`, a pool `pool-a` of `pool-a-1` and `pool-a-2`, crew's partitions `home` and `work` on the machine `host-a` with `example/crew-state`, and none of the operator's paths, values, instructions or employer. What cites the sketch follows its new name.

### 8. The archived changes are scrubbed in place

The archived changes are the history of the specs, and they are in the tree. Their proposals, designs, tasks, verify reports and spec deltas take Decision 1's names and Decision 4's citations in place, each otherwise as it was. Most of what they copy is in the designs' and proposals' accounts of the cloud's records, rules, roadmap and domain model and of afterthought/swancloud's rules and layout, and of flywheel-next's engine; each becomes a citation by id with the point the design needed. A cited research transcript is cited in the archive (Decision 5). They are not re-archived; `root:changes` does not read them.

### 9. The specs take the neutral names through this change's deltas

Each requirement whose scenarios name a private value is modified with Decision 1's names, and nothing else of it changes: seventeen of `crew-control-plane`'s, eight of `work-into-flume`'s, and one each of `views`, `branch-commands` and `tool-records`. Each scenario is proven by the test that proved it, renamed with it. No requirement is added (Non-Goals). The specs' purposes cite afterthought/swancloud's rules by name and id, which core.41 allows, and stay.

## Risks / Trade-offs

- **The live agent on a scratch host needs herdr to run under the scratch `HOME`.** Flume 0.4.1's herdr profile was checked against the herdr of its day. Where today's herdr cannot serve it, the herdr of that day is built from its tag beside the tools, outside the tree.
- **sluice 0.61.1's flaky reading needs a task that passes and fails on the same inputs.** A task reading a file outside its inputs, changed between runs, gives one, as a flaky test does.
- **A missed name.** Nothing in the gate finds one (Non-Goals). Each task names the files it covers, and the verify stage reads the tree.
- **The records lose their links to the interview.** A public reader sees a citation of a private repository where a file stood. The decisions and the user's words are in the records themselves.
