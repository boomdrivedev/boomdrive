## MODIFIED Requirements

### Requirement: The configuration chooses the implementation

The runner SHALL open each implementation an account of the configuration names, from the list in `planes.rs`, in the list's order, handing each the table of its name, the `poll_interval`, and, for each part it opens, a say and a wake of that part's own, which it calls to wake that part when its control plane pushes the part's work. This binary SHALL carry `crew` and `hosted`. Any other word SHALL be refused, naming the words this binary carries (record 0054).

#### Scenario: The hosted control plane named

- **WHEN** the account `own` says `control_plane = "hosted"`, and the configuration holds a `[control_plane.hosted]` table naming a partition
- **THEN** the runner opens the hosted implementation with that table, and names no word as one this binary does not carry

#### Scenario: A word not carried

- **WHEN** the account `own` says `control_plane = "elsewhere"`
- **THEN** the configuration is refused, naming `elsewhere` as a control plane this binary does not carry and naming `crew` and `hosted`

## ADDED Requirements

### Requirement: A part's share names each place it did not launch, with no lease

`Report` SHALL hold `unleased`: each place by its path as Flume printed it, with the address of its repository and its status as Flume printed it, carried. It SHALL never be written to the report file, and SHALL read as none where a report lacks it. A part SHALL let go of nothing, refuse nothing and confirm no end of what `unleased` holds, and no want SHALL name it (record 0054; core.10, core.35, core.54).

#### Scenario: A report written with unleased places

- **WHEN** a report holding one launch and one unleased place is written by `boomdrive-control`'s types and read back
- **THEN** it reads back with the launch and no unleased place

#### Scenario: The fake holds what it is handed

- **WHEN** the fake control plane is reported a share holding the unleased place `/p/places/note--by-hand`
- **THEN** the report it holds names that place, and its answer lets go of nothing

## REMOVED Requirements

### Requirement: An observer is handed a part's passes and gives nothing back

**Reason**: Record 0048 is superseded by record 0054; every plane is a control plane in full, told of every place in its repositories.
**Migration**: None; no runner reports to an observer.
