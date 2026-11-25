## MODIFIED Requirements

### Requirement: A pull gives what the control plane wants, in its order

A pull SHALL give the control plane's wants of the host in the control plane's order, each an offer of a launch under a lease, an act on a launch, a command, an inbox to carry or a preview. With them it SHALL give the terminals to open, and the runner's report as the control plane last held it, or none. A pull that cannot be read SHALL give `Unread` with words saying why, and the runner SHALL then act on nothing that pass.

#### Scenario: Wants in order

- **WHEN** the fake control plane holds an offer of launch `a`, then a priority of `a`, then a command `c`
- **THEN** a pull gives the offer, the act and the command in that order

#### Scenario: A pull that cannot be read

- **WHEN** the fake control plane refuses a pull with the words `no network`
- **THEN** the control pass names `no network`, reads Flume's status for the hello, runs no `flume` command but `flume --version`, and reports nothing
