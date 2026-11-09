## REMOVED Requirements

### Requirement: A bolt's branch is made by sluice in the clone

**Reason**: A bolt is a launch of its caller's kind with no stages. Flume asks sluice to make its branch at the launch (Flume's 202, 205), so the runner makes no branch (core.52; record 0002 as amended on 2026-11-07).

**Migration**: A control plane offers a bolt as a launch of a kind that lands with no stages, its base the branch the bolt lands on (the work-into-flume spec's "A bolt is a launch"). A branch command is refused, naming sluice (the commands spec's "A command is refused, naming its tool").

### Requirement: A branch command carries the commit origin's branch stands at

**Reason**: No command makes a branch, so none carries the commit a branch stands at. Flume's status of the bolt's place gives its branch and head, as Flume gives them (Flume's 202).

**Migration**: A control plane reads a bolt's branch from Flume's status of its place, which the runner carries as Flume gave it. A branch command an earlier release carried out keeps its origin commit in the report (the commands spec's "A command is answered once").

### Requirement: A branch command is taken only where its repository is kept

**Reason**: A branch command is refused, naming sluice, whatever its repository, so its repository is not read. A bolt's launch is taken only where its repository is kept, as every launch is.

**Migration**: The work-into-flume spec's "An offer is taken only for a repository the runner keeps" holds a bolt's launch to the repositories the runner keeps.

### Requirement: A command is carried out once

**Reason**: Moved to the commands spec, which holds what the runner does with every command.

**Migration**: The commands spec's "A command is answered once".

### Requirement: A command this binary does not carry is refused

**Reason**: Moved to the commands spec, where a branch command is one such command.

**Migration**: The commands spec's "A command is refused, naming its tool".
