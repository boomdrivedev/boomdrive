# Spec Delta

## Purpose

Each principle on the Method page is read beside an animated diagram of its
idea. This capability says what each principle's diagram shows and how it
behaves in the reader's theme, with scripts off and under reduced motion.

## ADDED Requirements

### Requirement: Never prove twice has its diagram

The Method page SHALL show a diagram beside "Never prove twice" in place of its
numeral, drawn from the SVG the principle names, as the site design asks of
every principle.

#### Scenario: The diagram takes the numeral's place

- **WHEN** a reader opens the Method page and reaches "Never prove twice"
- **THEN** the principle's place shows the diagram beside its write-up, and the faint numeral placeholder is gone

### Requirement: tailrace's cache in the middle, four places around it

The diagram SHALL show tailrace's cache in its middle and four places that
run the same checks: sluice's merge queue, a worktree, a second worktree and
CI. Each place SHALL show the same three checks, and the cache SHALL show
what it holds as one entry for each check under the key of the inputs it
read.

#### Scenario: The four places run the same checks

- **WHEN** a reader looks at the four places
- **THEN** each is labelled as sluice's merge queue, a worktree, a second worktree or CI, and each shows the same three checks by the same names

#### Scenario: The cache is named as tailrace's

- **WHEN** a reader looks at the middle of the diagram
- **THEN** the cache is titled as tailrace's and says it is filled by landings

### Requirement: The first run takes its time and the landing fills the cache

The diagram SHALL show sluice's merge queue testing a candidate first, its
checks running slowly and passing, each run entering sluice's record, and
then the candidate landing and the landing's verdicts filling the cache with
one entry for each check. Nothing SHALL enter the cache before the landing.

#### Scenario: The queue runs and lands

- **WHEN** the diagram's cycle begins
- **THEN** the queue's three checks fill slowly, each is marked run and passed, a row for each run appears in sluice's record, and only once the candidate is marked landed do the cache's three entries fill from it

### Requirement: The next three replay in a flash

The diagram SHALL show the worktree, the second worktree and CI, in turn,
replaying each check from the cache in a flash, each check marked replayed
rather than run, with nothing travelling into the cache or the record from
them.

#### Scenario: A place replays

- **WHEN** the worktree's, the second worktree's or CI's turn plays
- **THEN** an entry travels from the cache to that place, its three checks are marked replayed at once, none of them fills slowly, and no row appears in sluice's record

### Requirement: One changed input runs only its check again

The diagram SHALL show a next candidate in sluice's merge queue that changes
one input, named, which only one check reads. That check's key SHALL no
longer match the cache, so that check alone runs again, slowly, while the
other two replay in a flash.

#### Scenario: Only the affected check runs

- **WHEN** the next candidate's turn plays
- **THEN** the changed input is named, the one check that reads it shows a new key and fills slowly as a run, and the other two checks are marked replayed at once

### Requirement: sluice's record keeps the fail the cache never remembers

The diagram SHALL show the check run again failing, the candidate not
landing, and that fail, with its key, entering sluice's record beside the
cache, while the cache takes no entry for it and says a fail never lands
there.

#### Scenario: The fail is recorded, not cached

- **WHEN** the run again ends in a fail
- **THEN** the check is marked failed in the fail colour, the candidate is marked not landed, a row with its key and the fail appears in sluice's record beneath the earlier passes, and the cache shows no entry for that key, with a label saying a fail never lands there

### Requirement: Numbered captions tell the story

The diagram SHALL carry four numbered captions that say, in order, that the
queue's run takes its time and its landing fills the cache, that the next
three replay in a flash, that one changed input runs only its check again,
and that sluice's record keeps the fail the cache never remembers.

#### Scenario: The captions light in turn

- **WHEN** the diagram plays
- **THEN** each caption is lit while its part of the story plays and dimmed otherwise, in the order one to four

### Requirement: The diagram names its tools and links their pages

The diagram SHALL carry a line naming the tools it shows: moon declaring what
each check reads, tailrace keeping the cache, and sluice testing, landing and
recording. It SHALL link tailrace's page (`https://agentplot.github.io/tailrace/`)
and sluice's page (`https://agentplot.github.io/sluice/`), each a link a
reader can follow from the Method page with the pointer or the keyboard.

#### Scenario: A reader follows a tool's link

- **WHEN** a reader on the Method page clicks, or tabs to and opens, the diagram's link to tailrace's page or to sluice's page
- **THEN** that tool's site opens, and the link showed a visible focus outline while it had the keyboard's focus

#### Scenario: The tools line stays put

- **WHEN** the diagram plays, or is held still under reduced motion
- **THEN** the tools line and its two links are shown throughout and never move

### Requirement: The family's diagram colours, in the reader's theme

The diagram SHALL take every colour from the family's diagram colours (`--bg`,
`--panel`, `--fg`, `--muted`, `--rule`, `--accent`, `--soft`, `--ok`, `--bad`,
`--ink`), carrying both a dark and a light palette, so that on the Method page
it follows the reader's light or dark setting.

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
moment that shows the whole idea: the first candidate landed, sluice's record
holding its passes, the cache's entries from the landing, the worktree's, the
second worktree's and CI's checks replayed, the one check run again and
failed on the next candidate, its fail in sluice's record and not in the
cache, all four captions lit, and the tools line with its links.

#### Scenario: Reduced motion is on

- **WHEN** the reader's setting asks for reduced motion
- **THEN** nothing in the diagram moves, and the still shows each part of the idea named above

### Requirement: Alt text says what the diagram shows

The principle SHALL name its diagram with alt text that says what the diagram
shows: tailrace's cache in the middle and the four places, the queue's first
run and its landing filling the cache, the replays, the one check run again
on a changed input, and its fail kept by sluice's record and not by the
cache. The diagram's own description SHALL carry the same words, and the alt
text SHALL pass the site's house style.

#### Scenario: A screen reader reaches the diagram

- **WHEN** a screen reader reaches the diagram on the Method page
- **THEN** it reads the principle's alt text as the diagram's label, naming tailrace's cache, the four places, the landing filling the cache, the three replays, the one check run again, and its fail kept by sluice's record and not by the cache

#### Scenario: The house style reads the alt text

- **WHEN** `site-check` runs
- **THEN** Vale reports no error or warning on the alt text
