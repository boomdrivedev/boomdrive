# method-diagrams Specification

## Purpose
Each principle on the Method is shown beside a diagram of its idea, agreed
with the operator before it is drawn, so a reader can see the principle at
a glance in either theme, with or without motion or scripts.

## Requirements

### Requirement: A drawn diagram takes its principle's numeral's place

Once a principle names its diagram, the Method page SHALL show that diagram
beside the principle's write-up in place of its faint outlined numeral, and
straight after the principle's claim at phone width.

#### Scenario: A wide screen shows the diagram beside the write-up

- **WHEN** a reader on a wide screen reaches a principle that names a diagram
- **THEN** the diagram stands beside the write-up and the principle's outlined numeral is not shown

#### Scenario: A phone shows the diagram after the claim

- **WHEN** a reader at phone width reaches a principle that names a diagram
- **THEN** the diagram comes straight after the principle's claim, with no horizontal scroll

#### Scenario: A principle not yet drawn

- **WHEN** a reader on a wide screen reaches a principle that names no diagram
- **THEN** its numeral in faint outline holds the diagram's place

### Requirement: A diagram draws its principle's agreed idea

Each principle's diagram SHALL show every part of the idea that
`docs/method-outline.md` records as agreed with the operator in that
principle's Diagram line, and nothing that changes it. A diagram SHALL be
drawn only once its idea is recorded there.

#### Scenario: The diagram matches the outline

- **WHEN** a reader compares a principle's diagram with its Diagram line in `docs/method-outline.md`
- **THEN** each part the line names is shown in the diagram

#### Scenario: No agreed idea, no diagram

- **WHEN** a principle's Diagram line records no agreed idea
- **THEN** the principle names no diagram, and its numeral holds the place

### Requirement: Every diagram takes the family's colours in the reader's theme

Every diagram SHALL take each colour from the family's diagram colours
(`--bg`, `--panel`, `--fg`, `--muted`, `--rule`, `--accent`, `--soft`,
`--ok`, `--bad`, `--ink`), so on the Method page it follows the reader's
light or dark setting, and opened on its own it SHALL draw in its own
light and dark palette.

#### Scenario: The diagram follows a light setting

- **WHEN** a reader with a light setting opens the Method page
- **THEN** every diagram is drawn in the page's light colours, with no colour from outside the family's diagram colours

#### Scenario: The diagram follows a dark setting

- **WHEN** a reader with a dark setting opens the Method page
- **THEN** every diagram is drawn in the page's dark colours

#### Scenario: The file stands alone

- **WHEN** a diagram's SVG file is opened on its own in a browser
- **THEN** it draws in its own palette, light or dark by the browser's setting

### Requirement: Every diagram moves with scripts off

Every diagram's motion SHALL be its own, needing no script, so it plays the
same whether or not the page's scripts run.

#### Scenario: Scripts are off

- **WHEN** a reader opens the Method page with scripts turned off and no reduced-motion setting
- **THEN** every drawn diagram is shown and plays as it does with scripts on

### Requirement: Every diagram is still and whole under reduced motion

Under the reader's reduced-motion setting every diagram SHALL hold one still
moment in which each part of its agreed idea is shown at once, with every
caption and every tools line it carries visible.

#### Scenario: Reduced motion is on

- **WHEN** a reader with a reduced-motion setting opens the Method page
- **THEN** nothing in any diagram moves, and each diagram's still shows every part its principle's Diagram line names

### Requirement: Every diagram's alt text says what it shows

Each principle SHALL name its diagram with alt text that says what the
diagram shows, every part of its agreed idea and any tool it names, in the
glossary's terms. The diagram's own description SHALL carry the same words,
and the alt text SHALL pass the site's house style.

#### Scenario: A screen reader reaches the diagram

- **WHEN** a screen reader reaches a diagram on the Method page
- **THEN** it reads the principle's alt text as the diagram's label, describing every part the diagram shows

#### Scenario: The house style reads the alt text

- **WHEN** `site-check` runs
- **THEN** Vale reports no error or warning on any principle's alt text

### Requirement: A diagram's numbered captions light in turn

Where a diagram carries numbered captions, each SHALL be lit while its part
of the story plays and dimmed otherwise, in order.

#### Scenario: The captions light in turn

- **WHEN** a diagram with numbered captions plays
- **THEN** each caption is lit while its part plays, from the first to the last

### Requirement: A diagram that names a tool links its page

Where a diagram names the tools it shows, it SHALL carry them on a line that
never moves, each tool with a page linked from inside the diagram to that
page, followable with the pointer or the keyboard and showing a visible
focus outline. Those links SHALL be the diagram's only references outside
itself.

#### Scenario: A reader follows a tool's link

- **WHEN** a reader on the Method page clicks, or tabs to and opens, a tool's link in a diagram
- **THEN** that tool's page opens, and the link showed a visible focus outline while it had the keyboard's focus

#### Scenario: The tools line stays put

- **WHEN** the diagram plays, or is held still under reduced motion
- **THEN** its tools line and links are shown throughout and never move
