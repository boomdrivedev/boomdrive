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

- **[core.1]** The runner decides nothing the control plane or flume decides: it starts work only as a `flume launch` the control plane asked for, and reports flume's results as flume gave them.
- **[core.5]** The runner keeps a clone or a view only of a repository its configuration names, and keeps none of one it does not.

### The control plane

- **[core.2]** The control plane is reached through exactly one interface, and an implementation of it, crew's state repository or a hosted control plane, implements that interface and nothing of the runner's own; no code outside an implementation names crew, its records or the hosted control plane's protocol.
- **[core.3]** No connection comes in to a host: the runner opens no listening socket and every connection it makes is outgoing.

### Secrets and records

- **[core.4]** No token, key or credential is in the repository: none is committed, none is a default in code, configuration or tests, and the runner reads each from the host when it runs.
- **[core.6]** Nothing commercial, no pricing, tier or customer, appears in this repository.

### The gate

- **[core.7]** Every file the tree tracks is owned by a formatter the gate runs, or is excluded by name with its reason; every crate is a moon project of its own, listed in `.moon/workspace.yml` with its language declared, whose check-fast tasks format, lint, type-check and test it, one task per check.
- **[core.8]** Every tool a gate task runs is the version the tree pins, on every host that runs the gate.

## Details

- **core.1** Rules out: the runner choosing which work runs, on which host, in what order or under which kind; a `flume launch` with no request from the control plane behind it; a result rewritten, summarised or judged before it is carried back; the runner deciding a stage, a submission or a landing that flume decides. Not ruled out: refusing work for a repository its configuration does not name, and saying so to the control plane (core.5). Source: record 0001; the user, 2026-10-31.
- **core.5** Rules out: a clone made because work named a repository the configuration does not; a clone or view left after its repository leaves the configuration. Source: the user, 2026-10-31.
- **core.2** Rules out: a call to crew's state repository or to the hosted control plane's socket from outside its implementation; an implementation calling another implementation; a function of the runner's own with a branch for one implementation. Not ruled out: the implementations' shared tests run against the interface. Source: the user, 2026-10-31.
- **core.3** Rules out: a listening socket, an HTTP server or a webhook endpoint on a host; a control plane that reaches a host by connecting to it. Not ruled out: an outgoing WebSocket the runner opens and keeps; a fetch or a push the runner makes to a repository. Source: the user, 2026-10-31.
- **core.4** Rules out: a token, key or credential in a tracked file, in a test's fixture or in a commit message; a default credential in code; a credential printed in output or logs. Not ruled out: the name of the variable or file a credential is read from. Source: the user, 2026-10-31.
- **core.6** Rules out: pricing, a tier or a customer named in code, docs or commits. Source: record 0001.
- **core.7** Rules out: a file type treefmt neither formats nor excludes; an exclude with no comment saying why; a crate with no project of its own in `.moon/workspace.yml`; one task bundling several checks. Source: record 0001; flume's core.21, the pattern this repository copies.
- **core.8** Rules out: a tool a task finds outside its pin, in rustup's default toolchain or a runner's own install; a pinned toolchain without a component a task runs; two files pinning one tool with no check that fails when they differ. Source: flume's core.22.
