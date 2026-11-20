## ADDED Requirements
### Requirement: Each want is carried on a thread of its own

A part SHALL start each want's tool command, a `flume launch`, Flume's command for an act on a place, with Flume's status of the place where the listing does not hold it, and sluice's preview or release, on a thread of its own as soon as the pull gives it, and SHALL wait on none of them before it takes the pull's next want. Each launch taken SHALL run at once (record 0002 as amended on 2026-11-19; core.50).

#### Scenario: A slow launch holds no other launch

- **WHEN** a pull offers `a`, whose launch is readied only once `b`'s has been asked of Flume, then `b`
- **THEN** both are launched in the one pass, `b`'s launch asked while `a`'s runs

#### Scenario: A slow launch holds no land

- **WHEN** the runner holds `x` at a place Flume stands, and a pull offers `x` and `a`, whose launch is readied only once `x`'s land has been asked, then gives a land of `x`
- **THEN** `flume retry --submission` is run on `x`'s place while `a`'s launch runs, and the land is answered `done`

### Requirement: A launch in flight holds its room

A pull's launches SHALL take the stream's room in the control plane's order, and a launch in flight whose place Flume's listing does not hold yet SHALL hold its room at each later pass until its answer is taken (core.9, core.50).

#### Scenario: A slow launch keeps its room

- **WHEN** stream `work` has room for one, a part's pass starts `a`'s slow launch into it, and its next pass, whose listing does not hold `a`'s place yet, is offered `b` into `work`
- **THEN** `b` is not launched while `a`'s launch runs

### Requirement: An act waits only until its launch is made

An act on a launch being made SHALL wait, neither run nor answered, until that launch's answer is taken. Acts on one launch SHALL run one at a time in the control plane's order, each once every act before it on that launch is answered; acts on different launches SHALL run side by side. A drop of a launch not yet made SHALL be answered `done`, and the launch never made (core.50).

#### Scenario: An act on a launch being made

- **WHEN** a pull gives the offer `a`, whose launch is slow, then a priority of `a`, and the part makes two passes while the launch runs
- **THEN** no `flume priority` is run and the act is not answered at either; once `a` is made, `flume priority` is run on its place and its answer goes up before the next pass

### Requirement: An offer waits for a launch being made that it comes after or works from

An offer whose After names a launch being made, or whose base is the branch `<kind>/<name>` of a place being made, SHALL wait, neither launched nor answered, until that launch's answer is taken, so its answer reads where that launch stands and its base stands (core.50; Flume's 202).

#### Scenario: A unit waits for its bolt's launch

- **WHEN** a pull offers the bolt `home/bolt/3` of kind `bolt` named `x`, then its unit `home/14` on `bolt/x`
- **THEN** the unit's `flume launch` runs only once the bolt's has returned, and both are answered `launched`

### Requirement: Each answer goes up once it is made

Each command's answer SHALL come back to its part over a channel as soon as the command returns, and SHALL wake the part, which SHALL take it, save its share, start each act or preview of the last pull that waited on it, and send its report where it changed, between its passes, waiting on no other command and on no pass's end (core.50).

#### Scenario: An answer goes up while another launch runs

- **WHEN** a part's pass starts `a`'s launch, which is slow, and `b`'s
- **THEN** the pass ends without waiting on either; a report holding `b` launched goes up as soon as `b`'s answer comes back, while `a`'s launch runs, and one holding `a` launched once it is made, each before the part's next pass

### Requirement: `run --once`'s pass waits for every command it started

`boomdrive run --once`'s one pass SHALL wait for every command it started, take their answers and start what waited on them, before it carries the statuses and sends (core.50).

#### Scenario: A launch and an act on it in one pass

- **WHEN** `run --once`'s pull gives a priority of `a`, then the offer `a`
- **THEN** `a` is launched, then `flume priority` is run on its place, and the one report the pass sends holds both

### Requirement: A want in flight is not taken again

A want whose command is in flight SHALL NOT be taken again from a later pull, and its answer SHALL be reported once (core.10).

#### Scenario: A later pass while a launch runs

- **WHEN** a part's pass starts `a`'s slow launch, and its next pass's pull offers `a` again
- **THEN** no second `flume launch` is run for `a`, and the pass says nothing of it

### Requirement: An answer for a lease let go while its command ran is not reported

Where the control plane lets go or refuses a want, or the launch an act acts on, while that want's command runs, its answer SHALL NOT be reported or kept, and one line SHALL say so, beginning with the want's id (core.10).

#### Scenario: A land whose launch's lease is refused while it runs

- **WHEN** the runner holds `x`, a land of `x` runs slowly, and the control plane refuses `x`'s lease at the part's next pass
- **THEN** once the land returns, a line reads `x/land: let go by the control plane while its command ran: its answer is not reported`, the land is not in the report, and nothing more is sent

### Requirement: A command in flight when the runner stops is left to finish

Stopped, a part SHALL begin no further pass, and SHALL wait for each command in flight to end, take its answers and send them, starting nothing more (core.10).

#### Scenario: A launch in flight at the stop

- **WHEN** a part's pass starts `a`'s slow launch, and the runner stops
- **THEN** the part waits for the launch, and a report holding `a` launched goes up before the part ends

## MODIFIED Requirements

### Requirement: No answer turns on the round a want is taken in

The launches a part makes SHALL be those the control plane's order makes. An act answered from the report SHALL be answered where the control plane's order puts it, and an act on a launch the same pull offers SHALL be carried out once that launch is made, in the same pass where the pass waits for its commands (core.50).

#### Scenario: An act ahead of its own launch

- **WHEN** a pull gives a priority of `a`, then the offer `a`, and `a` is launched that pass at place `P`
- **THEN** `flume priority P` is run in the same pass, after the launch, and the act is answered `done`

#### Scenario: A drop ahead of a held launch

- **WHEN** launch `b` is held on its After, and a pull gives a drop of `b`, then the offer `b`, at the pass at which `b`'s After reads landed
- **THEN** the drop is answered `done`, `b` is answered `done` with words naming the drop, and no `flume launch` is run for `b`

## REMOVED Requirements

### Requirement: A pull's launches go ahead of its acts on places

**Reason**: A want's command runs on a thread of its own as soon as the pull gives it, so no launch waits behind an act, nor an act behind a launch (record 0002 as amended on 2026-11-19; core.50). "Each want is carried on a thread of its own" holds what stands of it.

**Migration**: None: the control plane reads every answer as it did, sooner.

### Requirement: A launch waits on no act of the pass before its own

**Reason**: No act waits behind another act's command but one on the same launch, and none is left to the next pass because the next is due (record 0002 as amended on 2026-11-19; core.50).

**Migration**: None: the control plane reads every answer as it did, sooner.

### Requirement: A pass past due sends each answer as it makes it

**Reason**: Each answer goes up in the part's next send once it is made, whether or not the next pass is due. "Each answer goes up once it is made" holds it.

**Migration**: None: the control plane reads every answer as it did, sooner.

### Requirement: What a send of the pass lets go is taken no more that pass

**Reason**: A pass waits on no command, so nothing of a pull is taken after a send of the same pass. What the control plane lets go or refuses while a command runs is held by "An answer for a lease let go while its command ran is not reported".

**Migration**: None: the control plane reads every answer as it did, sooner.

### Requirement: A send that fails before a pass's end leaves its answers to the end

**Reason**: No pass sends before its end. A send between passes that fails leaves its answers to the next send, as a pass's send does (core.22).

**Migration**: None: the control plane reads every answer as it did, sooner.

### Requirement: Once a pass is due, it makes no launch after its first

**Reason**: The turn-taking of 2026-11-14 is retired: a pull's launches each run at once within the stream's room (record 0002 as amended on 2026-11-19; core.50).

**Migration**: None: the control plane reads every answer as it did, sooner.

### Requirement: A launch the pass took and left is held for its turn

**Reason**: No launch is held for a turn; a launch is answered `held` only on its After or for lack of room.

**Migration**: None: the control plane reads every answer as it did, sooner.

### Requirement: Once a pass is due, its launches and its acts take turns

**Reason**: Launches and acts run side by side, neither waiting on the other's command but an act on a launch being made.

**Migration**: None: the control plane reads every answer as it did, sooner.

### Requirement: The commands a pass past due sends before

**Reason**: No pass sends before a command; each answer goes up once made.

**Migration**: None: the control plane reads every answer as it did, sooner.
