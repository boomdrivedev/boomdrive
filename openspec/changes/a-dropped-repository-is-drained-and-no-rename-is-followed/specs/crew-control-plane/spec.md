# crew's Control Plane

## MODIFIED Requirements

### Requirement: A launch naming no host is given only where this runner keeps its repository

A launch naming no `Host` that no runner answers SHALL be for this host only where its `Repo` is a repository the runner keeps: an address the hello names among its repositories, or a name alone that exactly one of them has as its name. A repository the hello names as drained SHALL not count. Where the runner keeps no such repository, crew's implementation SHALL give nothing for the launch and write nothing of it, so that another host's runner may take it. Where two addresses the hello names share the name, it SHALL give nothing and name both in its log.

#### Scenario: A kit named by its name alone

- **WHEN** a launch naming no `Host` says `Repo: crew`, and the hello names `afterthought/crew` and `agentplot/flume`
- **THEN** the launch is for this host, its repository `afterthought/crew`

#### Scenario: A repository not kept

- **WHEN** a launch naming no `Host` says `Repo: agentplot/sluice`, and the hello does not name it
- **THEN** the pull gives nothing for it, and the runner's branch holds no answer to it

#### Scenario: A repository draining

- **WHEN** a launch naming no `Host` says `Repo: example/kit`, no runner answers it, and the hello names `example/kit` only as drained
- **THEN** the pull gives nothing for it, and the runner's branch holds no answer to it

## ADDED Requirements

### Requirement: A launch the runner answered is given while the runner keeps or drains its repository

A launch naming no `Host` that the runner's report or this host's branch answers, a claim apart, SHALL be given at every pull while its `Repo` names a repository the hello names: one among its repositories, else one among those it drains, each read as an unanswered launch's `Repo` is read. Its repository SHALL be that address, and it SHALL be neither claimed nor named in the log.

#### Scenario: A launch answered in a repository draining

- **WHEN** `madswan/runner/deneb` answers launch 20 `launched`, launch 20 names no `Host` and says `Repo: kit`, and the hello names `example/other` among its repositories and `example/kit` as drained
- **THEN** the pull gives the offer `madswan/20`, its repository `example/kit`, writes nothing, and says nothing

### Requirement: No old name of a repository is matched to a new one

Where neither the hello's repositories nor those it drains name the `Repo` of a launch the runner answered, crew's implementation SHALL give nothing for the launch, write nothing of it and say nothing. It SHALL match no name a `Repo` once had to another.

#### Scenario: A launch answered before its repository was renamed

- **WHEN** the runner's report answers launch 20 `launched`, launch 20 names no `Host` and says `Repo: example/old-kit`, and the hello names `example/new-kit` alone
- **THEN** the pull gives nothing for launch 20, writes nothing, and says nothing

### Requirement: A launch naming this host takes its repository from those kept, then those drained

A launch naming this host SHALL take as its repository the address its `Repo` names among the hello's repositories, else among those it drains, else its `Repo` as written.

#### Scenario: A launch naming this host in a repository draining

- **WHEN** request 12 is a launch naming `Host: deneb` and saying `Repo: kit`, and the hello names `example/kit` as drained
- **THEN** the pull gives the offer `madswan/12`, its repository `example/kit`

## REMOVED Requirements

### Requirement: A launch the runner answered is given whatever the hello names

**Reason**: Record 0021, as revised and accepted by the user on 2026-11-07, rules out handling a repository renamed in flight: a launch is given only for a repository the runner keeps, drained ones among them, and no old name is matched to a new one.

**Migration**: A launch the runner answered is given while the runner keeps or drains its repository, as "A launch the runner answered is given while the runner keeps or drains its repository" and "No old name of a repository is matched to a new one" have it. A launch whose `Repo` names a repository by an old name is given to no runner, and its work is launched again under the repository's name once it is added back.
