# crew's Control Plane

## MODIFIED Requirements

### Requirement: A launch naming no host is given only where this runner keeps its repository

A launch naming no `Host` SHALL be for this host only where its `Repo` is a repository the runner keeps: an address the hello names, or a name alone that exactly one address the hello names has as its name. Where the runner keeps no such repository, crew's implementation SHALL give nothing for a launch it has not answered and write nothing of it, so that another host's runner may take it. Where two addresses the hello names share the name, it SHALL give nothing and name both in its log.

#### Scenario: A kit named by its name alone

- **WHEN** a launch naming no `Host` says `Repo: crew`, and the hello names `afterthought/crew` and `agentplot/flume`
- **THEN** the launch is for this host, its repository `afterthought/crew`

#### Scenario: A repository not kept

- **WHEN** a launch naming no `Host` says `Repo: agentplot/sluice`, and the hello does not name it
- **THEN** the pull gives nothing for it, and the runner's branch holds no answer to it

## ADDED Requirements

### Requirement: A launch the runner answered is given whatever the hello names

A launch naming no `Host` that the runner's report or this host's branch answers, a claim apart, SHALL be given at every pull whatever the hello names, so that its place is followed to its end. Its repository SHALL be the address its `Repo` names among those the hello names where there is one, else its `Repo` as written. It SHALL be neither claimed nor named in the log.

#### Scenario: A launch answered before its repository left the hello

- **WHEN** `madswan/runner/deneb` answers launch 20 `launched`, launch 20 names no `Host` and says `Repo: old-kit`, and the hello names `example/new-kit` alone
- **THEN** the pull gives the offer `madswan/20`, its repository `old-kit`, writes nothing, and says nothing

#### Scenario: An answered launch whose name two kept addresses share

- **WHEN** the runner's report answers launch 20 `launched`, launch 20 names no `Host` and says `Repo: crew`, and the hello names `afterthought/crew` and `agentplot/crew`
- **THEN** the pull gives the offer `madswan/20`, its repository `crew`
