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

### Requirement: A cache in the middle, four places around it

The diagram SHALL show the cache in its middle and four places around it that
run the same checks: a worktree, a second worktree, CI and the merge queue.
Each place SHALL show the same three checks, and the cache SHALL show what it
holds as one pass for each check under the key of the inputs it read.

#### Scenario: The four places run the same checks

- **WHEN** a reader looks at the four places
- **THEN** each is labelled as a worktree, a second worktree, CI or the merge queue, and each lists the same three checks by the same names

#### Scenario: The cache holds passes by key

- **WHEN** the cache has been filled
- **THEN** it shows one row for each check, each with an inputs key and a pass

### Requirement: The first run takes its time and fills the cache

The diagram SHALL show the first worktree running each check slowly, and each
pass, as it finishes, travelling into the cache and filling that check's row.

#### Scenario: The first run fills the cache

- **WHEN** the first worktree's run plays
- **THEN** its three checks fill slowly, each is marked run and passed, and the cache's three rows fill from it, one pass for each check

### Requirement: The next three replay in a flash

The diagram SHALL show the second worktree, CI and the merge queue, in turn,
answering each check from the cache in a flash, each check marked replayed
rather than run, with nothing travelling into the cache from them.

#### Scenario: A place replays

- **WHEN** the second worktree's, CI's or the merge queue's turn plays
- **THEN** a pass travels from the cache to that place, its three checks are marked replayed at once, and none of them fills slowly

### Requirement: One changed input runs only its check again

The diagram SHALL show a next candidate in the merge queue that changes one
input, named, which only one check reads. That check's key SHALL no longer
match the cache, so that check alone runs again, slowly, while the other two
replay in a flash.

#### Scenario: Only the affected check runs

- **WHEN** the changed candidate's turn plays
- **THEN** the changed input is named, the one check that reads it shows a new key and fills slowly as a run, and the other two checks are marked replayed at once

### Requirement: The queue's record keeps the fail the cache never remembers

The diagram SHALL show the check run again failing, and that fail, with its
key, entering the queue's own record beside the cache, while the cache takes
no row for it and says it holds passes only.

#### Scenario: The fail is recorded, not cached

- **WHEN** the run again ends in a fail
- **THEN** the check is marked failed in the fail colour, a row with its key and the fail appears in the queue's record, and the cache shows no row for that key, with a label saying the cache keeps passes only

### Requirement: Numbered captions tell the story

The diagram SHALL carry four numbered captions that say, in order, that the
first run takes its time and fills the cache, that the next three replay in a
flash, that one changed input runs only its check again, and that the queue's
record keeps the fail the cache never remembers.

#### Scenario: The captions light in turn

- **WHEN** the diagram plays
- **THEN** each caption is lit while its part of the story plays and dimmed otherwise, in the order one to four

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
moment that shows the whole idea: the first worktree's checks run and passed,
the cache's three passes, the other places' checks replayed, the one check run
again and failed, its fail in the queue's record and not in the cache, and all
four captions lit.

#### Scenario: Reduced motion is on

- **WHEN** the reader's setting asks for reduced motion
- **THEN** nothing in the diagram moves, and the still shows each part of the idea named above

### Requirement: Alt text says what the diagram shows

The principle SHALL name its diagram with alt text that says what the diagram
shows: the cache in the middle and the four places, the first run filling the
cache, the replays, the one check run again on a changed input, and its fail
kept by the queue's record and not by the cache. The diagram's own description
SHALL carry the same words, and the alt text SHALL pass the site's house style.

#### Scenario: A screen reader reaches the diagram

- **WHEN** a screen reader reaches the diagram on the Method page
- **THEN** it reads the principle's alt text as the diagram's label, naming the cache, the four places, the first run filling the cache, the three replays, the one check run again, and its fail kept by the queue's record and not by the cache

#### Scenario: The house style reads the alt text

- **WHEN** `site-check` runs
- **THEN** Vale reports no error or warning on the alt text
