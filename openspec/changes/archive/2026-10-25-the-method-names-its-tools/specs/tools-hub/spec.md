## Purpose

Keeps the Tools page true to the Method: it lists every tool the Method names, says how the Method joins them, and each tool's card points at a principle that names that tool.

## ADDED Requirements

### Requirement: The Tools page lists tailrace

The Tools page's "Each stands alone" list SHALL include tailrace, after sluice, in the voice of the other entries: what it does on its own, in present tense, with its name in bold.

#### Scenario: Reading the list

- **WHEN** a reader reads "Each stands alone" on the Tools page
- **THEN** after sluice's entry they read that tailrace carries what the merge queue lands into a store you own and names each piece by its content, keeps channels and the environments that follow them, and serves consumers in their ecosystem's own form

#### Scenario: The house style

- **WHEN** Vale reads the Tools page's words
- **THEN** it reports no error or warning on tailrace's entry, and tailrace is a lowercase term of the glossary, as sluice is

### Requirement: The Method joins tailrace to the others

The Tools page's "The Method joins them" SHALL say, in one sentence after the work lands through sluice, that tailrace carries the landing to a channel and to every environment that follows it.

#### Scenario: Reading how the tools join

- **WHEN** a reader reads "The Method joins them" on the Tools page
- **THEN** the sentence that ends with the work landing through sluice is followed by one saying tailrace carries the landing to a channel and to every environment that follows it

### Requirement: A card's principle names its tool

Each tool's card on the Tools page that names a principle under "In the Method" SHALL name one whose "In the tools" strip names that tool, and the build SHALL fail when one does not.

#### Scenario: The cards as built

- **WHEN** the Tools page is built
- **THEN** headwater, eddy and millpond name Capture at the source, flume names One agent, one worktree, sounding names Measure the fleet, and sluice names Order at landing, never at start

#### Scenario: A card that disagrees

- **WHEN** a card names a principle whose strip does not name that card's tool
- **THEN** the build fails, naming the tool and the principle
