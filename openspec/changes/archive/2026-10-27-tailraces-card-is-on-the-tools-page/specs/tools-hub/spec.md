## ADDED Requirements

### Requirement: tailrace has a card at Deliver

The Tools page SHALL show a card for tailrace after sluice's, in the form of every other tool's card: its track with its own place lit, its status, its one line, the principle it carries under "In the Method", and links to its site and design book, each read from `@agentplot/site-theme`'s family.

#### Scenario: Reading tailrace's card

- **WHEN** a reader reaches the family on the Tools page
- **THEN** after sluice's card they see tailrace's, its track lit at Deliver, its status "designed, not built" with an outlined dot, its line "Build once, sign everything, promote with proof, in your own account.", "In the Method" naming Two loops, one gate and linking that principle's place on the Method page, and links to tailrace's site and design book

#### Scenario: The page around the cards counts tailrace

- **WHEN** a reader opens the Tools page
- **THEN** the headline reads "Seven tools, one loop.", the legend lists tailrace under "Designed, not built", tailrace's bold name in "Each stands alone" carries its dot in its accent, and the family's diagram above the cards plays seven steps, tailrace's among them

#### Scenario: Light and dark

- **WHEN** a reader views tailrace's card with the light setting, then the dark one
- **THEN** its rule, its lit stop, its status dot and its name's dot take tailrace's accent in that theme's tone, as the other cards take theirs

### Requirement: Every track runs to Deliver

Each card's track on the Tools page, Flywheel's included, SHALL show the loop's places in the order it runs: Notice, Inbox, Launch, Work, Land, Deliver. A tool's card SHALL light its own place, and Flywheel's SHALL light every stop. No two stops' names SHALL overlap; a card too narrow to name every stop SHALL keep every stop and name only its own place.

#### Scenario: The stops in order

- **WHEN** a reader reads any card's track
- **THEN** it has six stops, Notice, Inbox, Launch, Work, Land and Deliver, and sluice's lights Land while tailrace's lights Deliver

#### Scenario: Phone width

- **WHEN** a reader opens the Tools page at 375 pixels wide
- **THEN** the cards stack, no stop's name overlaps another's, each tool's card names at least its own place, Flywheel's track shows all six stops, and the page has no sideways scroll

#### Scenario: Scripts off and reduced motion

- **WHEN** a reader opens the Tools page with scripts off, or asking for reduced motion
- **THEN** every card, tailrace's included, shows its track, status, line, principle and links in full, and nothing on a card is reachable only by hovering or clicking

## MODIFIED Requirements

### Requirement: A card's principle names its tool

Each tool's card on the Tools page that names a principle under "In the Method" SHALL name one whose "In the tools" strip names that tool, and the build SHALL fail when one does not.

#### Scenario: The cards as built

- **WHEN** the Tools page is built
- **THEN** headwater, eddy and millpond name Capture at the source, flume names One agent, one worktree, sounding names Measure the fleet, sluice names Order at landing, never at start, and tailrace names Two loops, one gate

#### Scenario: A card that disagrees

- **WHEN** a card names a principle whose strip does not name that card's tool
- **THEN** the build fails, naming the tool and the principle
