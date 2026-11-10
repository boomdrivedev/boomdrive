# Spec Delta

## Purpose

Each principle on the Method is shown beside a diagram of its idea, agreed
with the operator before it is drawn, so a reader can see the principle at
a glance in either theme, with or without motion or scripts.

## ADDED Requirements

### Requirement: Capture at the source shows its diagram on the Method
The Method SHALL show the "Capture at the source" diagram beside the
principle's write-up in place of its outlined numeral, and straight after
its claim at phone width.

#### Scenario: A wide screen shows the diagram beside the write-up
- **WHEN** a reader opens the Method on a wide screen and reaches "Capture at the source"
- **THEN** the diagram stands beside the write-up and the principle's outlined numeral is not shown

#### Scenario: A phone shows the diagram after the claim
- **WHEN** a reader opens the Method at phone width
- **THEN** the "Capture at the source" diagram comes straight after the principle's claim, with no horizontal scroll

### Requirement: The capture diagram shows four sources, each beside its adapter
The diagram SHALL show four sources, a meeting, a chat, a log and a pull
request, each with an adapter beside it. Events SHALL pulse at the
sources, and each adapter's captures SHALL flow into one inbox, each
capture listed there by its key.

#### Scenario: Every source has an adapter and reaches the one inbox
- **WHEN** a reader watches one loop of the diagram
- **THEN** each of the meeting, the chat, the log and the pull request shows an event, its adapter writes a capture, and all four captures arrive in the same inbox

### Requirement: The same event arriving twice becomes one capture
The diagram SHALL show one event, the log's alert, sent twice: the second
copy carries the same key as the first and merges into it, so the inbox
lists one capture for that event, marked as sent twice.

#### Scenario: A repeated event adds no row
- **WHEN** the log's alert arrives a second time
- **THEN** the inbox still lists four captures, and the log's capture is marked as sent twice

### Requirement: A reader session turns a capture into signals
The diagram SHALL show a reader session taking one capture from the inbox,
the meeting's, and reading it into signals, shown as an ask, a constraint
and a question.

#### Scenario: The meeting's capture becomes three signals
- **WHEN** every capture has arrived in the inbox
- **THEN** the meeting's capture is picked out, a reader session takes it, and three signals appear beside the session: an ask, a constraint and a question

### Requirement: The hand-written ticket form is struck through
The diagram SHALL show a hand-written ticket form, set apart from the flow,
struck through as the way work does not begin.

#### Scenario: The ticket form is struck
- **WHEN** the reader session has produced its signals
- **THEN** a line strikes through the hand-written ticket form, and the strike stays until the loop restarts

### Requirement: The capture diagram follows the page's theme
The diagram SHALL use only the family's diagram colours (`--bg`, `--panel`,
`--fg`, `--muted`, `--rule`, `--accent`, `--soft`, `--ok`, `--bad`,
`--ink`), so on the Method it follows the reader's light or dark setting.
Opened on its own, the file SHALL carry its own light and dark palette.

#### Scenario: The diagram follows a light setting
- **WHEN** a reader with a light setting opens the Method
- **THEN** the capture diagram is drawn in the page's light tokens, with no colour from outside the family's diagram colours

#### Scenario: The diagram follows a dark setting
- **WHEN** a reader with a dark setting opens the Method
- **THEN** the capture diagram is drawn in the page's dark tokens

### Requirement: The capture diagram moves with scripts off
The diagram's motion SHALL be its own, so it plays the same loop whether
or not the page's scripts run.

#### Scenario: Scripts are off
- **WHEN** a reader opens the Method with scripts disabled and no reduced-motion setting
- **THEN** the capture diagram is visible and plays its loop

### Requirement: The capture diagram is still under reduced motion
Under a reduced-motion setting the diagram SHALL hold one still frame in
which every part is shown: the four sources with their events, the four
adapters, the four captures in the inbox, the log's capture marked as sent
twice, the reader session with its three signals, the struck ticket form
and all four captions.

#### Scenario: Reduced motion shows the whole story at once
- **WHEN** a reader with a reduced-motion setting opens the Method
- **THEN** nothing in the capture diagram moves, and every part listed above is visible

### Requirement: The capture diagram's alt text says what it shows
The diagram SHALL carry alt text that names the four sources and their
adapters, the one inbox, the repeated event kept as one capture, the
reader session's signals and the struck ticket form, using the glossary's
terms, with no sentence that the house style flags.

#### Scenario: A screen reader hears the diagram
- **WHEN** a screen reader reaches the capture diagram on the Method
- **THEN** it reads the alt text, which describes every part the diagram shows

#### Scenario: The alt text passes the house style
- **WHEN** the site's checks run
- **THEN** Vale reports no error or warning on the principle's alt text
