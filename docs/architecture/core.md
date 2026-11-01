# flywheel-runner rules

The rules that bind wherever a change is made in this repository,
written so a reviewer can point at a diff and say which rule it breaks.
The runner is the thin part on each host: it keeps the clones and views
of the repositories its configuration names, and turns the control
plane's work into `flume launch` calls, carrying their results back.
Every decision about what work there is, where it runs and how it is
carried through is the control plane's or flume's. There are no area
pages yet; a rule that binds only one part of the tree moves to an area
page when that part exists.

## Rules

### What the runner decides

- **[core.1]** The runner decides nothing the control plane or flume decides: it starts work only as a `flume launch` or a tool command the control plane asked for, and reports each tool's results as the tool gave them.
- **[core.5]** The runner keeps a clone or a view only of a repository its configuration names, and keeps none of one it does not; the configuration names the repositories and where their views go, and no stream or kind.

### The control plane

- **[core.2]** The control plane is reached through exactly one interface, and an implementation of it, crew's state repository or a hosted control plane, implements that interface and nothing of the runner's own; no code outside an implementation names crew, its records or the hosted control plane's protocol.
- **[core.3]** No connection comes in to a host: the runner opens no listening socket and every connection it makes is outgoing.

### Work under a lease

- **[core.9]** The runner takes the control plane's work only under a lease it was offered, and takes a launch only while it keeps the launch's repository and the Flume stream the launch runs in has room on its host, as Flume's status gives it; it names a stream only where the launch names one.
- **[core.10]** Every report about leased work names the lease's number, and the runner carries out each act and command once, across restarts; once the control plane refuses a report for a lease the runner no longer holds, the runner neither acts on that work nor reports it again, and removes nothing of it.

### What goes back

- **[core.11]** What the runner carries back of a tool is the tool's own answer, each field as the tool gave it, or a reference to it: no repository content, task log, raw material or terminal byte is carried in a report, written to the runner's data or its log, or kept.

### Secrets and records

- **[core.4]** No token, key or credential is in the repository: none is committed, none is a default in code, configuration or tests, and the runner reads each from the host when it runs.
- **[core.6]** Nothing commercial, no pricing, tier or customer, appears in this repository.

### Names

- **[core.12]** Every configuration key and every type names what it holds to its reader: a duration's key ends in `_interval`, a path's in `_root`, `_dir` or `_file`, and no key or type takes a word another tool uses for something else.

### The gate

- **[core.7]** Every file the tree tracks is owned by a formatter the gate runs, or is excluded by name with its reason; every crate is a moon project of its own, listed in `.moon/workspace.yml` with its language declared, whose check-fast tasks format, lint, type-check and test it, one task per check.
- **[core.8]** Every tool a gate task runs is the version the tree pins, on every host that runs the gate.

## Details

- **core.1** Rules out: the runner choosing which work runs, on which host, in what order or under which kind; a `flume launch` or a tool command with no ask of the control plane behind it; a result rewritten, summarised or judged before it is carried back; the runner deciding a stage, a submission or a landing that flume decides, or a branch's making that sluice decides. Not ruled out: refusing work for a repository its configuration does not name, and saying so to the control plane (core.5); leaving work untaken while its stream has no room (core.9); a launch or an act that a control plane's implementation gives because it read the control plane's own rule from the control plane's own records, as crew's implementation reads when a role is wanted (core.2). Source: record 0001; record 0002; the user, 2026-10-31 and 2026-11-01; flywheel-cloud record 0017.
- **core.5** Rules out: a clone made because work named a repository the configuration does not; a dropped repository's clone or view fetched, moved or made again, or left once neither holds work the forge lacks; a stream, a kind or a Flywheel named in the configuration, in the runner's own keys or a control plane's table; a kinds file, a kind's source or a lock named there or passed to Flume by the runner, since where a host's kinds come from is its Flume configuration's; a repository added to the configuration because a control plane registered it or work named it. Not ruled out: a dropped repository's clone and view left untouched and named while either holds work the forge lacks (crew record 0052 part 2); the repositories kept reported to the control plane as the host's capacity, which registers none of them to a Flywheel (flywheel-cloud record 0007); a launch's kind passed to Flume as its control plane gave it, by its name or its identity `<source>/<name>@<version>` (flume record 0022). Source: the user, 2026-10-31, in this rule and in the brief of the change the-runner-keeps-its-repositories; record 0002; the user, 2026-11-01, reviewing the change the-runner-carries-crews-work-into-flume ("Kinds are flume's concern, not the runner's or a control plane's"); flywheel-cloud records 0003 and 0007, and its ledger item 3.b.
- **core.2** Rules out: a call to crew's state repository or to the hosted control plane's socket from outside its implementation; an implementation calling another implementation; a function of the runner's own with a branch for one implementation. Not ruled out: the implementations' shared tests run against the interface. Source: the user, 2026-10-31.
- **core.3** Rules out: a listening socket, an HTTP server or a webhook endpoint on a host; a control plane that reaches a host by connecting to it. Not ruled out: an outgoing WebSocket the runner opens and keeps; a fetch or a push the runner makes to a repository. Source: the user, 2026-10-31.
- **core.9** Rules out: a launch made while the host's room or its stream's, as Flume's status gives it, is spent, or while the stream's queue holds a place; a stream the runner chooses, or a default stream of its own; work taken that no offer gave. Not ruled out: a launch that names no stream, which runs in the stream Flume's host configuration gives it; a command, which runs in no stream, taken whatever the room. Source: record 0002; flywheel-cloud records 0003 and 0009; the user, 2026-11-01.
- **core.10** Rules out: a report with no lease number; an act or a command carried out twice because it was given twice or the runner restarted; a place dropped, tidied or removed because its lease was lost. Not ruled out: a lost lease's place left as Flume holds it and named in the runner's log; the lease number 1 for every launch, where the control plane never moves one, as crew's implementation gives it. Source: record 0002; flywheel-cloud record 0009, core.38 and core.39; departing from flywheel-next's `definitions/engine/host.yaml` and its requirement 55, whose host removed stray places.
- **core.11** Rules out: sluice's task logs or their locations, or its `refs/sluice/logs` and `refs/sluice/heads`, carried; a capture's raw material, or a file its pointer names, read or carried; a terminal's screen or input written to a file, a log or a report; a file of a repository in a report. Not ruled out: commit ids, branch names, paths and digests; Flume's status of a place as Flume printed it; a field of a tool's answer left out because it holds a log or content the cloud may not hold; a screen passed through while its terminal is open. Source: record 0002; flywheel-cloud core.4, core.18 and core.28, and record 0016; its ledger items c-flume.a, c-sluice.a, c-inbox.a and N1.b.
- **core.4** Rules out: a token, key or credential in a tracked file, in a test's fixture or in a commit message; a default credential in code; a credential printed in output or logs. Not ruled out: the name of the variable or file a credential is read from. Source: the user, 2026-10-31.
- **core.6** Rules out: pricing, a tier or a customer named in code, docs or commits. Source: record 0001.
- **core.12** Rules out: a duration's key with no `_interval`, such as `fetch_every` or `every`; a path's key with no `_root`, `_dir` or `_file`, such as `views`; a key such as `state` that names less than what it holds, a repository; a key or type named view, kind, record, stream, ask or watch for something that is not that tool's thing, such as `kind` for a control plane's implementation, `Record` for what the runner reports, or `Asks` for what a launch gives Flume. Not ruled out: a word naming the other tool's own thing as that tool means it, as a launch's `kind` holds the kind Flume runs and its `stream` the stream Flume runs it in; a field of another tool's format spelled as that tool spells it, crew's `Host`, `Kinds` or `Answer` among them. Source: the user, 2026-11-01, reviewing the change the-runner-carries-crews-work-into-flume ("Every key names what it is to the reader: a duration ends in _interval, a path in _root, _dir or _file, and no word another tool uses for something else (view, kind, record, stream)").
- **core.7** Rules out: a file type treefmt neither formats nor excludes; an exclude with no comment saying why; a crate with no project of its own in `.moon/workspace.yml`; one task bundling several checks. Source: record 0001; flume's core.21, the pattern this repository copies.
- **core.8** Rules out: a tool a task finds outside its pin, in rustup's default toolchain or a runner's own install; a pinned toolchain without a component a task runs; two files pinning one tool with no check that fails when they differ. Source: flume's core.22.
