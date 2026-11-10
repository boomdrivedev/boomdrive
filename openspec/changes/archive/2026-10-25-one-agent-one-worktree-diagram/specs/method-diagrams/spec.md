# Spec Delta

## Purpose

Each principle on the Method page is read beside an animated diagram of its
idea. This capability says what each principle's diagram shows and how it
behaves in the reader's theme, with scripts off and under reduced motion.

## ADDED Requirements

### Requirement: One agent, one worktree has its diagram

The Method page SHALL show a diagram beside "One agent, one worktree" in place
of its numeral, drawn from the SVG the principle names, as the site design
asks of every principle.

#### Scenario: The diagram takes the numeral's place

- **WHEN** a reader opens the Method page and reaches "One agent, one worktree"
- **THEN** the principle's place shows the diagram beside its write-up, and the faint numeral placeholder is gone

### Requirement: Three worktrees, each a place sluice equips

The diagram SHALL show three worktrees side by side, each a place sluice
equips: each holds its own agent and a block of ports sluice gave it, with its
emulator and its dev server each running on one of those ports, and no two
worktrees share a port, an emulator or a server. It SHALL NOT show sluice
starting the emulator or the dev server, since sluice starts no service.

#### Scenario: Each worktree's environment is its own

- **WHEN** a reader looks at the three worktrees
- **THEN** each is marked as a sluice place and shows an agent, the ports sluice gave it, and an emulator and a dev server on those ports, and the ports differ from one worktree to the next

#### Scenario: The ports are sluice's blocks

- **WHEN** a reader reads the port numbers
- **THEN** each worktree's ports are one consecutive block from 20000 up, as sluice gives a place, and the emulator's and dev server's ports are among its own block

### Requirement: The agents work at their own pace

The diagram SHALL show the three agents working at different paces, so that
each finishes at a different moment of the animation.

#### Scenario: The agents finish one after another

- **WHEN** the diagram plays through one cycle
- **THEN** the three agents' work completes at three different moments, and none waits on another to start or finish

### Requirement: Each submits its head and none merges

The diagram SHALL show each agent, once finished, submitting its head to
sluice's queue on the line, and sluice performing the merge and the landing.
It SHALL show that no worktree merges its own work.

#### Scenario: A finished worktree submits

- **WHEN** an agent finishes its work
- **THEN** its head travels from its worktree to the line, and the line, labelled as sluice's queue, shows it landed

#### Scenario: No worktree merges

- **WHEN** a reader looks at any worktree at any moment
- **THEN** that worktree's merge is shown struck out, and nothing moves from a worktree onto the line except a submitted head

### Requirement: Reaching into another's server is refused

The diagram SHALL show, partway through its cycle, one agent reaching across
into another worktree's dev server and being refused at that worktree's edge,
with the refusal said in words.

#### Scenario: The reach is stopped

- **WHEN** the refusal moment plays
- **THEN** a reach from one agent toward a neighbouring worktree's dev server stops at that worktree's edge, the edge is marked in the refusal colour, and a label says the reach was refused

### Requirement: Numbered captions tell the story

The diagram SHALL carry four numbered captions under it that say, in order,
that each agent has its own worktree and environment, that each works at its
own pace, that another's server is out of reach, and that each submits its
head while sluice merges and lands it.

#### Scenario: The captions light in turn

- **WHEN** the diagram plays
- **THEN** each caption is lit while its part of the story plays and dimmed otherwise, in the order one to four

### Requirement: The diagram names its tools and links sluice's page

The diagram SHALL name the tools it shows, Worktrunk making each worktree and
sluice equipping it and landing its head, and SHALL link sluice's page,
`https://agentplot.github.io/sluice/`, from within the diagram. That link
SHALL be the diagram's only reference outside itself.

#### Scenario: A reader follows the link

- **WHEN** a reader on the Method page clicks or tabs to the link in the diagram and follows it
- **THEN** sluice's page opens

#### Scenario: The tools line is always shown

- **WHEN** the diagram is at any moment of its cycle, or held still under reduced motion
- **THEN** the line naming Worktrunk and sluice, and the link, are shown

### Requirement: The family's diagram colours, in the reader's theme

The diagram SHALL take every colour from the family's diagram colours
(`--bg`, `--panel`, `--fg`, `--muted`, `--rule`, `--accent`, `--soft`, `--ok`,
`--bad`, `--ink`), carrying both a dark and a light palette, so that on the
Method page it follows the reader's light or dark setting.

#### Scenario: The diagram follows the page's theme

- **WHEN** the reader's setting is light, and then dark
- **THEN** the diagram on the Method page is drawn in the page's light colours, and then in its dark colours, with no colour fixed outside the palette

#### Scenario: The file stands alone

- **WHEN** the SVG file is opened on its own in a browser
- **THEN** it draws in its own palette, light or dark by the browser's setting

### Requirement: The diagram moves with scripts off

The diagram's motion SHALL come from its own CSS, needing no script, so it
plays wherever the page is read with scripts off.

#### Scenario: Scripts are off

- **WHEN** a reader opens the Method page with scripts turned off
- **THEN** the diagram is shown and animates as it does with scripts on

### Requirement: The diagram is still under reduced motion

Under the reader's reduced-motion setting the diagram SHALL hold one still
moment that shows the whole idea: at least one head on the line, at least one
agent still working, the refused reach, every worktree's struck merge, and all
four captions lit.

#### Scenario: Reduced motion is on

- **WHEN** the reader's setting asks for reduced motion
- **THEN** nothing in the diagram moves, and the still shows each part of the idea named above

### Requirement: Alt text says what the diagram shows

The principle SHALL name its diagram with alt text that says what the diagram
shows: the three worktrees as places sluice equips and what each holds, the
agents' own pace, the heads submitted to sluice's queue that merges and lands
them, the refused reach, and the tools the diagram names.
The diagram's own description SHALL carry the same words, and the alt text
SHALL pass the site's house style.

#### Scenario: A screen reader reaches the diagram

- **WHEN** a screen reader reaches the diagram on the Method page
- **THEN** it reads the principle's alt text as the diagram's label, naming the three worktrees as places sluice equips, what each holds, the agents' own pace, the heads submitted to sluice's queue, sluice merging and landing them, the refused reach, Worktrunk and sluice

#### Scenario: The house style reads the alt text

- **WHEN** `site-check` runs
- **THEN** Vale reports no error or warning on the alt text
