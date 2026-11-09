# control-plane-interface Specification

## Purpose
The one interface through which the runner reaches its control plane, by a pull and a report. It covers what passes over it: the host's hello, launches offered under leases, acts on a launch, commands, inboxes to carry, terminals, and the runner's report. It also covers how the configuration chooses the implementation, and the cases every implementation passes. crew's implementation is built now. A hosted control plane's is named and left unbuilt, and the interface carries the cloud's runner contract so that it can take crew's place. Nothing that passes over it names a kinds file or a kind's source: a launch names its kind, and where the host's kinds come from is its Flume configuration (core.2, core.5, core.12; record 0002; boomdrive-cloud records 0005, 0009, 0017 and 0022).

## Requirements

### Requirement: The control plane is reached through one interface

The runner SHALL reach its control plane only through `boomdrive-control`'s `ControlPlane` interface, served as parts: a pull, which takes the runner's hello and the part's share of its report, a report, which takes that share, and `owns`. No code outside an implementation SHALL name crew, its records or a hosted control plane's protocol. The one list of implementations, `crates/boomdrive/src/planes.rs`, SHALL name each by its word and its opener alone (core.2).

#### Scenario: Only crew's crate names crew

- **WHEN** the sources of `crates/boomdrive` and `crates/boomdrive-control` are read
- **THEN** no file names `boomdrive_crew` but `crates/boomdrive/src/planes.rs`, and `boomdrive-control` depends on no implementation

### Requirement: A pull gives what the control plane wants, in its order

A pull SHALL give the control plane's wants of the host in the control plane's order, each an offer of a launch under a lease, an act on a launch, a command, an inbox to carry or a preview. With them it SHALL give the terminals to open, and the runner's report as the control plane last held it, or none. A pull that cannot be read SHALL give `Unread` with words saying why, and the runner SHALL then act on nothing that pass.

#### Scenario: Wants in order

- **WHEN** the fake control plane holds an offer of launch `a`, then a priority of `a`, then a command `c`
- **THEN** a pull gives the offer, the act and the command in that order

#### Scenario: A pull that cannot be read

- **WHEN** the fake control plane refuses a pull with the words `no network`
- **THEN** the control pass names `no network`, runs no `flume` command but the hello's `flume --json status` and `flume --version`, and reports nothing

### Requirement: Work carries the control plane's ids and leases

Every want SHALL carry an id the control plane gave, which the runner SHALL never read into. An offer, a command and a preview SHALL carry a lease: its number, and its expiry or no expiry where the control plane never moves the work. An act SHALL name its launch and that launch's lease number. The report's leases SHALL name each launch's, command's and preview's id with its lease number.

#### Scenario: An offer's lease kept with its answer

- **WHEN** the fake control plane offers launch `a` under lease number 3, and the runner launches it
- **THEN** the runner's report holds `a` under lease 3, and its next report names 3 with `a`

#### Scenario: A preview's lease among the report's leases

- **WHEN** a report holds launch `a` under lease 3 and preview `p` under lease 5
- **THEN** its leases are `a` with 3 and `p` with 5

### Requirement: A launch names what Flume is to be given

An offer's launch SHALL hold its repository, kind and name. It MAY hold its base, brief, stream, priority, intent, recipients, held stages with their words, a held submission, the launches it comes after and its agent settings. Its kind SHALL be the kind as the control plane gave it, a name or an identity `<source>/<name>@<version>`, and the launch SHALL hold no kinds file and no kind's source apart from it (core.5).

#### Scenario: An offer with every field

- **WHEN** the fake control plane holds an offer giving every field of a launch, its kind `crew/crew-planner`
- **THEN** a pull gives each field as the fake holds it, its kind `crew/crew-planner`, and the runner's report holding the offer round-trips through JSON unchanged

### Requirement: The runner's report is reported whole

A report SHALL take the runner's whole report: each launch with its lease number, the repository it was answered in where the runner recorded one, its answer and carried statuses, and each act, command, inbox and preview, by id. A report file or a message that names no repository for a launch SHALL read as recording none, and one that names no previews SHALL read as holding none. A second report of the same report SHALL change nothing the control plane holds.

#### Scenario: A report sent twice

- **WHEN** the runner reports the same report twice to the fake control plane
- **THEN** the fake control plane's last report is the same after the second report as after the first

#### Scenario: A report from before

- **WHEN** a report's JSON names no repository for launch `a` and no previews, and a hello's JSON names no drained repository
- **THEN** each reads, `a` recording no repository, the report holding no preview and the hello naming none drained, and each round-trips through JSON unchanged, the report written with no `previews`

### Requirement: A report's answer names what is let go and what is refused

A report's answer SHALL name the ids the control plane carries no more, which the runner SHALL drop from its report, and the ids whose reports it refused because their leases are no longer the runner's, with its words. A report that cannot be sent SHALL give `Unsent`, and the runner SHALL keep its report to send at the next pass.

#### Scenario: A report not sent

- **WHEN** the fake control plane refuses a report
- **THEN** the runner's report file holds every answer of the pass, and the next pass reports them

#### Scenario: A launch let go

- **WHEN** the fake control plane answers a report naming launch `a` as carried no more
- **THEN** `a` leaves the runner's report, and later reports do not carry it

### Requirement: The configuration chooses the implementation

The runner SHALL open the implementation the configuration's `[control_plane]` `type` names, from the list in `planes.rs`, handing it the table of that name, the `poll_interval`, and, for each part it opens, a say and a wake of that part's own, which it calls to wake that part when its control plane pushes the part's work. This binary SHALL carry `crew` and `hosted`. Any other word SHALL be refused at start, naming the words this binary carries.

#### Scenario: The hosted control plane named

- **WHEN** the configuration says `[control_plane]` with `type = "hosted"` and a `[control_plane.hosted]` table naming a partition
- **THEN** the runner opens the hosted implementation with that table, and names no word as one this binary does not carry

#### Scenario: A word not carried

- **WHEN** the configuration says `[control_plane]` with `type = "elsewhere"`
- **THEN** the runner names `elsewhere` as a control plane this binary does not carry and names `crew` and `hosted`, runs no control pass, and keeps its repositories

### Requirement: Every implementation passes the shared cases

`boomdrive-control` SHALL hold the cases every implementation passes, each over a harness that makes a control plane holding given wants: the wants given in order; a reported answer and status coming back in the next pull's report; a second report changing nothing; a settled want still given, which the runner answers from its report; and each want's id owned by its giver. The fake control plane, crew's implementation and the hosted implementation SHALL each run them.

#### Scenario: The fake control plane passes

- **WHEN** `boomdrive-control`'s tests run the shared cases over the fake control plane
- **THEN** every case passes

#### Scenario: crew's implementation passes

- **WHEN** `boomdrive-crew`'s tests run the shared cases over a bare state repository holding crew's records
- **THEN** every case passes

#### Scenario: The hosted implementation passes

- **WHEN** `boomdrive-hosted`'s tests run the shared cases over a fake hosted control plane on the loopback address, holding the wants of one partition
- **THEN** every case passes

### Requirement: The protocol is held in the hosted implementation's crate alone

The runner protocol's files and types SHALL be held in `crates/boomdrive-hosted` alone, which SHALL depend on no crate of the tree's but `boomdrive-control`. Only the binary's crate SHALL depend on it, and of the binary's sources only `planes.rs` SHALL name it, as it names crew's (core.2, core.42).

#### Scenario: Only the hosted implementation's crate holds the protocol

- **WHEN** the sources of `crates/boomdrive` but `planes.rs`, and every file of `crates/boomdrive-control` and `crates/boomdrive-crew`, are read, and every file under `crates/` outside `crates/boomdrive-hosted` is listed
- **THEN** none names `boomdrive_hosted` or `boomdrive-hosted`, none's name ends `.schema.json`, `planes.rs` opens `boomdrive_hosted`, only `boomdrive` depends on `boomdrive-hosted`, and `boomdrive-hosted` depends on `boomdrive-control` alone of the tree's crates

### Requirement: An act asks one thing of its launch's place

An act SHALL ask one of these of its launch's place:

- a retry, with its stage and its words where it names them, and whether it names the submission;
- a land, which asks for the landing of its launch's work, and holds nothing but its launch;
- a drop;
- a priority, with its number;
- a done.

A land SHALL name no stage, no words and no other word of Flume's, and SHALL be written as the word `land` (core.53).

#### Scenario: An act of each sort

- **WHEN** an act of each sort, a land among them, is written by `boomdrive-control`'s types and read back
- **THEN** each reads back equal, and the land's `what` is written as `"land"`, holding nothing more

### Requirement: A preview names a repository, a branch and a revision, under a lease of its own

A preview SHALL hold its id, a lease, its repository's address, a branch, a revision and whether it is done. Not done, it asks the host to bring up the preview of the branch at the revision; done, it asks the host to put away the preview of that repository and revision it brought up. It SHALL name no word of sluice's, and SHALL be written as `{"preview": {"id", "lease", "repository", "branch", "revision", "done"}}` (core.55).

#### Scenario: A preview of each kind

- **WHEN** a preview not done and a preview done are written by `boomdrive-control`'s types and read back
- **THEN** each reads back equal, the first written as `{"preview": {…}}` holding `"done": false`

### Requirement: The report holds what the runner answered of each preview

The report's `previews` SHALL hold, by id, what the runner answered of each preview: its lease, the repository and revision it named, whether it was done, its outcome, `held`, `done` or `refused`, the tool's text and the words where it has them, the tool's listing of it where carried, and its time (core.55). A report holding no preview SHALL be written with no `previews`.

#### Scenario: A report holding a preview

- **WHEN** a report holding a preview answered `held`, with a carried status, is written by `boomdrive-control`'s types and read back
- **THEN** it reads back equal, the preview written under `"previews"`

### Requirement: An implementation serves its control plane as parts

An implementation SHALL serve its control plane as one or more parts, in its table's order, each a `Part` holding a label, given once among its parts, and a `ControlPlane` of its own. crew's implementation SHALL give a part for each partition of its table, a drive, and the hosted implementation a part for each partition of its table, an account's enrollment (record 0026; record 0028; core.49).

#### Scenario: crew's partitions as parts

- **WHEN** crew's table names the partitions `home` and `work`
- **THEN** crew's implementation opens two parts, `home` then `work`

#### Scenario: The hosted partitions as parts

- **WHEN** the hosted table names the partitions `home` and `work`
- **THEN** the hosted implementation opens two parts, `home` then `work`

### Requirement: A part says which ids are its own

`ControlPlane` SHALL say, by `owns`, whether an id is one it gives. Every want a part gives SHALL carry an id that part owns and no other part of its implementation owns. The runner SHALL keep each part's share of its report by `owns`, and SHALL read no id itself.

#### Scenario: crew's ids by partition

- **WHEN** crew's parts `home` and `work` each give a launch numbered 1
- **THEN** `home` owns `home/1` and not `work/1`, and `work` owns `work/1` and not `home/1`

#### Scenario: The hosted ids by partition

- **WHEN** the hosted parts `home` and `work` each give a launch `l-1`
- **THEN** `home` owns `home/l-1` and not `work/l-1`, and `work` owns `work/l-1` and not `home/l-1`

### Requirement: A report's answer names which statuses were taken

A report's answer SHALL name each launch whose carried status the control plane took with the report it answers, and no other, so the runner confirms an end only once its control plane has it (core.2, core.54; record 0030 as amended on 2026-11-09). An answer that names no statuses taken SHALL read as naming none.

#### Scenario: A status taken and one left

- **WHEN** the fake control plane is reported launches `a` and `b`, each with a carried status, and launch `c` with none, and the test leaves `b`'s status
- **THEN** its answer names `a`'s status taken, and neither `b`'s nor `c`'s

#### Scenario: An answer from before

- **WHEN** a report's answer is written by `boomdrive-control`'s types naming a status taken, and an answer's JSON names no statuses taken
- **THEN** the first round-trips unchanged; and the second reads as naming none, and is written back with no `statuses_taken`
