# tools-hub Specification

## Purpose
Keeps the Tools page true to the Method: it lists every tool the Method names, says how the Method joins them, and each tool's card points at a principle that names that tool.

## Requirements

### Requirement: The Tools page lists timber

The Tools page's "Each stands alone" list SHALL include timber, after sluice, in the voice of the other entries: what it does on its own, in present tense, with its name in bold.

#### Scenario: Reading the list

- **WHEN** a reader reads "Each stands alone" on the Tools page
- **THEN** after sluice's entry they read that timber carries what the merge queue lands into a store you own and names each piece by its content, keeps channels and the environments that follow them, and serves consumers in their ecosystem's own form

#### Scenario: The house style

- **WHEN** Vale reads the Tools page's words
- **THEN** it reports no error or warning on timber's entry, and timber is a lowercase term of the glossary, as sluice is

### Requirement: The Method joins timber to the others

The Tools page's "The Method joins them" SHALL say, in one sentence after the work lands through sluice, that timber carries the landing to a channel and to every environment that follows it.

#### Scenario: Reading how the tools join

- **WHEN** a reader reads "The Method joins them" on the Tools page
- **THEN** the sentence that ends with the work landing through sluice is followed by one saying timber carries the landing to a channel and to every environment that follows it

### Requirement: A card's principle names its tool

Each tool's card on the Tools page that names a principle under "In the Method" SHALL name one whose "In the tools" strip names that tool, and the build SHALL fail when one does not.

#### Scenario: The cards as built

- **WHEN** the Tools page is built
- **THEN** headwater, eddy and millpond name Capture at the source, flume names One agent, one worktree, sounding names Measure the fleet, sluice names Order at landing, never at start, and timber names Never prove twice

#### Scenario: A card that disagrees

- **WHEN** a card names a principle whose strip does not name that card's tool
- **THEN** the build fails, naming the tool and the principle

### Requirement: timber has a card at Deliver

The Tools page SHALL show a card for timber after sluice's, in the form of every other tool's card: its track with its own place lit, its status, its one line, the principle it carries under "In the Method", and links to its site and design book, each read from `@agentplot/site-theme`'s family.

#### Scenario: Reading timber's card

- **WHEN** a reader reaches the family on the Tools page
- **THEN** after sluice's card they see timber's, its track lit at Deliver, its status "designed, not built" with an outlined dot, its line "Build once, sign everything, promote with proof, in your own account.", "In the Method" naming Never prove twice and linking that principle's place on the Method page, and links to timber's site and design book

#### Scenario: The page around the cards counts timber

- **WHEN** a reader opens the Tools page
- **THEN** the headline reads "Seven tools, one loop.", the legend lists timber under "Designed, not built", timber's bold name in "Each stands alone" carries its dot in its accent, and the family's diagram above the cards plays seven steps, timber's among them

#### Scenario: Light and dark

- **WHEN** a reader views timber's card with the light setting, then the dark one
- **THEN** its rule, its lit stop, its status dot and its name's dot take timber's accent in that theme's tone, as the other cards take theirs

### Requirement: Every track runs to Deliver

Each card's track on the Tools page, Boomdrive's included, SHALL show the loop's places in the order it runs: Notice, Inbox, Launch, Work, Land, Deliver. A tool's card SHALL light its own place, and Boomdrive's SHALL light every stop. No two stops' names SHALL overlap; a card too narrow to name every stop SHALL keep every stop and name only its own place.

#### Scenario: The stops in order

- **WHEN** a reader reads any card's track
- **THEN** it has six stops, Notice, Inbox, Launch, Work, Land and Deliver, and sluice's lights Land while timber's lights Deliver

#### Scenario: Phone width

- **WHEN** a reader opens the Tools page at 375 pixels wide
- **THEN** the cards stack, no stop's name overlaps another's, each tool's card names at least its own place, Boomdrive's track shows all six stops, and the page has no sideways scroll

#### Scenario: Scripts off and reduced motion

- **WHEN** a reader opens the Tools page with scripts off, or asking for reduced motion
- **THEN** every card, timber's included, shows its track, status, line, principle and links in full, and nothing on a card is reachable only by hovering or clicking
