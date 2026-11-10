# Design

## Context

See proposal.md for why. This change brings what `main` has added since the last scrub to core.41, so the public repository can start from the tree (records 0012, 0014 and 0025). It changes no behaviour of the runner.

What stands, read on 2026-11-09 at `f4d98463`, every tracked file:

- **Code, tests and fixtures hold nothing private.** Every crate's sources and tests use the first scrub's neutral names. Each fixture directory, `flume-0.4.1`, `flume-0.17.1`, `sluice-0.61.1` and `sluice-0.74.3`, says in its README that its answers were read on a scratch host, and every path in them is under a scratch root (`/private/tmp/scratch/host/`) or a temporary directory. The protocol's examples use `/home/runner`, and the nix tests the user `alice`. `crates/boomdrive/src/view.rs` cites afterthought/swancloud's own test, by name, as the first scrub left it.
- **The specs hold nothing private.** Their purposes cite boomdrive-cloud's records and rules, its ledger items and afterthought/swancloud's `names.3`, `herdr.13` and `herdr.14` by id. Their scenarios take the first scrub's names.
- **The README, `design/`, `nix/` and the root's files hold nothing private.** `design/the-flakes-configuration/design.md` names afterthought/swancloud with what the modules ask of it, in one line.
- **Where the private text is.** Only in what `main` landed after the scrub of 2026-11-08:
  - an agent named after the drive it serves: the design agent of one drive in records 0009's and 0025's amendments, records 0030, 0031, 0032, 0034, 0035 and 0036, core.54's details, and the archived changes `2026-11-08-the-runner-brings-up-a-preview-its-control-plane-asks-for` and `2026-11-08-the-runner-carries-each-places-end`; another drive's design agent, the ops agent and the operator agent in record 0034;
  - a host of the user's, by its name, in records 0032, 0034, 0035 and 0036 and core.57's details;
  - a planner's words naming a drive, quoted in record 0029 and the archived proposal of `2026-11-07-each-pass-ends-with-a-settle`;
  - afterthought/swancloud's own commit, words its agents relayed about its own writers, and its own packages and actions, in record 0034; its own settings in record 0036; and a line of a host's configuration that it keeps, quoted in record 0035.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches. It relies on core.41, which says what the tree may hold, and on core.6, which no text here breaks. It changes the details of core.54 and core.57 only where they name an agent after its drive or a host of the user's, and no rule's text. It departs from no rule.

`site/` is not in this tree. The change `boomdrive-keeps-its-site`, on its own bolt, brings the site with boomdrive-web's history, which is already public, and amends core.41 itself, so the site's measurement and field-note pages keep naming our own drives and machines as ours, as published (its rule site.7). The opening carries `site/` across by its own history (record 0025's amendment).

## Goals / Non-Goals

**Goals:**

- No agent named after a drive, and no host of the user's named, in the tree's docs.
- No private repository's commit, words, packages, actions or settings in the tree's docs; each cited by its name and an id or a file's path at most.
- Every record keeps its decision and its sources, and the user's own words.

**Non-Goals:**

- **A check that the tree holds nothing private.** It would have to hold the names it looks for, which core.41 rules out, and the brief adds no check, test or task.
- **A spec delta.** No requirement or scenario names anything private, so the change declares `skip_specs` and adds no requirement.
- **`site/`**, its pages and its amendment of core.41, which are the change `boomdrive-keeps-its-site`'s (record 0025's amendment).
- **The opening itself**: the fresh history, the rename to `agentplot/boomdrive-archive` and turning Pages on. The commit messages of this history stay in the archive.
- **The deciders' names** in the records' `decision-makers`. core.41 names hosts, drives, state repositories, people's paths and accounts, and a person's name is none of them (the first scrub's Non-Goals).

## Decisions

### 1. The first scrub's vocabulary, with the agents' roles and the user's hosts

Every private value takes the neutral words for its role, the same in every file:

| role | neutral words |
|---|---|
| the design agent of the drive whose work this repository is, named after that drive | the design agent; `the design agent` in `decision-makers`, as records 0026 to 0028 list it |
| the design agent of another drive of the user's | another drive's design agent |
| the ops agent and the operator agent, named after their drive | the ops agent; the operator agent, as record 0026 says it |
| a host of the user's, named | a host of the user's; a host's draft configuration; on each of the user's hosts |
| a drive named inside words quoted that are not the user's | cut with `[...]`, or its role in brackets where the sentence needs it, as `[the drive's]` |

The table names roles, not the values it replaces: this design is in the tree too. Where the words for a role read awkwardly in a sentence, the sentence is reworded around the role and keeps its sense.

Alternatives:

- **The hosts' neutral test names, `host-a` and `host-b`, in a record's prose.** Not taken: they stand for a host a test runs as; a record's prose speaks of a host of the user's as one of theirs.

### 2. A private repository is cited, never described

A record may name a private repository and cite its record, rule or file by id or path (core.41). What that repository holds goes from the tree's text: its commits, the words its agents wrote or relayed about its own files, its own packages, wrappers, actions, health checks and settings, and a line of any file it keeps. A consequence that a record asks of a private repository stays as one line of this repository's own words naming it, as records 0003, 0012, 0023 and 0028 do, and says what it does with this repository's options, not how it does its own work.

For record 0034 that means: the questions it answers are cited as another drive's design agent's, relayed by the operator agent, about the Boomdrive flake's bolt at `14caa11` as afterthought/swancloud takes it, citing that repository's record 0059 with no commit; what the first question listed is said in this record's own words, as the facts no option covers that each point below decides; the two questions about this repository's module and pins are quoted as asked, since they hold nothing of that repository's; its sources cite afterthought/swancloud's `lib/herdr-plugins.nix` and `modules/herdr-host.nix` by path; point 1 names no action of that repository's; and its consequences say what afterthought/swancloud and a host of the user's set through the set's options, naming none of that repository's own packages, wrappers, actions or checks. For record 0036: a host's own log name and PATH go, not that repository's.

Alternatives:

- **Keep the relayed words with the names cut.** Not taken: what they list is that repository's own writers, its implementation, which the first scrub's Decision 4 rules out.

### 3. A host's configuration line is described

Record 0035's deciders quote the line of a host's draft configuration the user was reading. That configuration is kept in a private repository, so the record says what the line did, that it put each repository's folder under the old data directory `~/.local/share/flywheel`, and quotes the user's words about it as they stand. `~/.local/share/flywheel` is the runner's old default, no person's path.

### 4. A drive's name inside quoted words is cut

Record 0029 and the archived proposal of `2026-11-07-each-pass-ends-with-a-settle` quote a planner's signal of 2026-11-06 that names a drive. The quote keeps its words with the drive's name cut (Decision 1), so it still says the user wanted Boomdrive's work first.

### 5. What stays

Each of these names no host, drive, state repository, person's path or account, and copies no private repository's text:

- **A brief's, a planner's or the design agent's words** quoted as a record's or a change's source, but a private name inside them (Decisions 1 and 4). A brief is the statement of this repository's own work, as the records since 0026 quote it. Record 0002's settled points are the brief's own words: none of them is boomdrive-cloud's text, though each rests on a record or ledger item of the cloud's that it cites by id.
- **A request's, a launch's or a submission's number, and the times a host ran it**, as records 0002, 0026 and 0027 and the archived changes of 2026-11-07 give them: ids of a state repository named nowhere, and a host's run told in this repository's own words.
- **What boomdrive-cloud does with this repository**, one line in this repository's words citing its record by id, such as that it pins the protocol by revision or lands a bolt by version 1's retry.
- **afterthought/swancloud's rules and test cited by id or name** in the views and clones specs and `crates/boomdrive/src/view.rs`, as the first scrub left them.
- **This repository's own commit ids** in records and archived changes, which the archive keeps.

Alternatives:

- **Every quoted brief or agent's words out, as text of a state repository.** Not taken: the records rest on them, the earlier scrubs kept them, and they hold nothing of anyone's once a private name inside them is cut.
- **Every number and time of a host's run out.** Not taken: they name nothing, and the records' account of what happened is the ground for their decisions.

### 6. The records are amended where they stand

Each record is edited in place, its decision, its sources and the user's words unchanged but for the names and text Decisions 1 to 4 replace, and gains no amendment section: the first scrub did the same to records 0009, 0017, 0022, 0023 and 0025. A record's `decision-makers` follows its text. No record's status changes.

### 7. The archived changes are scrubbed in place

The archived changes are the history of the specs and are in the tree. The three that name an agent after its drive or quote a drive's name take Decisions 1 and 4 in place, each otherwise as it was. They are not archived again; `root:changes` does not read them.

## Risks / Trade-offs

- **A missed name.** Nothing in the gate finds one (Non-Goals). Each task names the files it covers, and the verify stage reads the tree.
- **`main` lands more before the opening.** What it adds after this change's base takes the same scrub before the public repository starts from the tree, as `what-main-adds-takes-the-neutral-names` did on 2026-11-08.
- **The site lands with names of our own fleet.** That is the site change's amendment of core.41 (site.7), deliberately: its measurement and field-note pages are published as they are.
