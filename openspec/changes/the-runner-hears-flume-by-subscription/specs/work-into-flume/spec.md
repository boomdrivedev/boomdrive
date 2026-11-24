## MODIFIED Requirements

### Requirement: Every pull carries Flume's host part as Flume printed it

The hello's capacity SHALL be Flume's host part, `running`, `bound`, `room` and `streams`, each as Flume printed it in the pass's listing, Flume's status as flume host gives it, read before its pull. It SHALL leave out Flume's `pools`, `addresses`, `bases` and `terminal_kept`.

#### Scenario: Streams as Flume printed them

- **WHEN** the stub `flume` prints a listing whose host part holds two streams and a pool named `pool-a`
- **THEN** the hello's capacity holds the two streams' text as the stub printed it, and the word `pool-a` appears nowhere in the hello

### Requirement: Room is read from Flume's status and counted down within a pass

The runner SHALL read room from the pass's listing, Flume's status as flume host gives it, read before its pull. The host SHALL have room while its `room` is `null` or above zero. A stream SHALL have room while the host has room, its own `room` is `null` or above zero, and its `queue` is empty. Each launch a part's pass makes SHALL count one against its stream's room and the host's, in that pass alone: the runner SHALL keep no count across parts (record 0026; core.9).

#### Scenario: Room for one

- **WHEN** two offers name stream `work`, whose `room` is 1 and whose `queue` is empty
- **THEN** the first is launched, and the second is left unanswered

#### Scenario: A stream with a queue

- **WHEN** an offer names stream `work`, whose `room` is 2 and whose `queue` holds a place
- **THEN** it is left unanswered, and no `flume` command is run for it

#### Scenario: A host with no room

- **WHEN** an offer names stream `work`, whose `room` is `null`, and the host's `room` is 0
- **THEN** it is left unanswered

#### Scenario: Unbounded

- **WHEN** an offer names stream `default`, whose `room` and the host's are both `null`
- **THEN** it is launched

#### Scenario: Each part counts its own room

- **WHEN** the runner carries `home` and `work`, each offering one launch naming no stream, and every listing Flume gives says the host's `room` is 1
- **THEN** each part launches its own offer, each pass counting only its own launch

### Requirement: The statuses are carried against Flume's listing as it stands once the wants are answered

The statuses' listing SHALL be Flume's listing as it stands once the pull's wants are answered. Where answering them ran a `flume launch` or an act of Flume's, it SHALL be Flume's status as flume host gives it then. Where it ran neither, it SHALL be the pass's listing (record 0027). A command runs no tool, so it SHALL NOT make the listing be read again.

#### Scenario: A place tidied while the pass answers its wants

- **WHEN** a carried place `P` stands live in the pass's listing, as carried, and while the pass launches another offer Flume tidies `P` landed, keeping its record `P@<at>`
- **THEN** Flume's listing is read again after the launch, and that pass carries Flume's answer for `P` as its status, naming `P@<at>`, tidied and landed

#### Scenario: A quiet pass reads the listing once

- **WHEN** a pass's wants run no `flume` command
- **THEN** it reads Flume's status once, before its pull

#### Scenario: A pass that answers a command reads the listing once

- **WHEN** the runner holds launch `a` at a place Flume stands, and a pull gives the offer `a` and a branch command, and no other want
- **THEN** the command is answered `refused`, naming sluice, and the pass reads Flume's status once, before its pull
