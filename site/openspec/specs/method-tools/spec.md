# method-tools Specification

## Purpose
Lets a reader of any principle of the Method go straight to the tools that do what it says, from the principle's place on the Method page, and keeps the statements' clause numbers in the outline rather than on the site.

## Requirements

### Requirement: Each principle names its tools

Each principle's place on the Method page SHALL show a strip labelled "In the tools" naming, in order, the tools that do what the principle says, read from that principle's own record. The statement, which is not one of the principles, SHALL show none.

#### Scenario: A principle with tools

- **WHEN** a reader reaches "Never prove twice" on the Method page
- **THEN** its place shows "In the tools" followed by tailrace, then sluice

#### Scenario: The tools each principle names

- **WHEN** the Method page is built
- **THEN** the strips name: One shell, sluice; One agent, one worktree, sluice and flume; Capture at the source, millpond, headwater and eddy; Intents and bolts, Crew and Flywheel Next; Two loops, one gate, tailrace and sluice; Order at landing, never at start, sluice; Never prove twice, tailrace and sluice; Secrets by declaration, sluice; Measure the fleet, Crew and sounding

#### Scenario: The strip is the principle's own data

- **WHEN** a writer changes the tools a principle's record names
- **THEN** its strip changes with it on the next build, and no other list on the site has to change

#### Scenario: A malformed entry fails the build

- **WHEN** a principle's record gives a tool with no name, or a link that is not a full web address
- **THEN** the build fails and names the principle

### Requirement: A tool's link lands where the tool does it

Each tool on a strip that has a public product page SHALL link to the page, and the section of it, that does what the principle says; where the product page has no such section, it SHALL link to the section of the tool's design book that does. A tool with neither a public product page nor a public design book SHALL be named without a link.

#### Scenario: A product page section

- **WHEN** a reader follows sluice on "One shell"
- **THEN** it opens sluice's commands reference at `sluice exec`, `https://agentplot.github.io/sluice/reference/commands/#sluice-exec`

#### Scenario: A tool with no public page

- **WHEN** a reader looks at "Intents and bolts"
- **THEN** Crew and Flywheel Next are named in its strip as plain words, with no link

#### Scenario: Every link answers

- **WHEN** each link on every strip is requested
- **THEN** each page answers, and each link that names a section names a heading present on that page

### Requirement: The strip reads in every setting

The strip SHALL sit with the principle's write-up, outside the folded "Today, where it's going, and its record", so it reads without opening anything, and SHALL be legible in the reader's light and dark settings, at phone width, with scripts off and under reduced motion.

#### Scenario: Scripts off

- **WHEN** a reader opens the Method page with scripts off
- **THEN** every principle's strip is visible and every link in it works

#### Scenario: Phone width

- **WHEN** a reader opens the Method page at phone width
- **THEN** each strip follows its principle's write-up, within the screen's width and with no sideways scroll

#### Scenario: Light and dark

- **WHEN** a reader's setting is light, and then dark
- **THEN** the strip's label and links take the page's own colours in each, as the Tools page's "In the Method" does

#### Scenario: Reduced motion

- **WHEN** a reader has asked for reduced motion
- **THEN** the strip is shown at once with no transition

### Requirement: No clause number on the site

No page of the site SHALL show a clause number of any statement, in its text, its diagrams or its alt text. A principle's text says what the tool does and leaves the clause numbers to the outline.

#### Scenario: The built site

- **WHEN** the site is built
- **THEN** no page of it carries the word "clause" or "clauses" followed by a number

#### Scenario: A principle that cited clauses

- **WHEN** a reader opens "Order at landing, never at start"
- **THEN** its write-up says what the merge queue does with no "(clauses 10 to 12)" or any other clause citation

#### Scenario: A clause number written into content

- **WHEN** a writer adds "clause 4" to a page under `src/content/`
- **THEN** Vale reports a warning on it, which the house style requires fixed before the change lands

### Requirement: The outline records each principle's tools

The Method's outline SHALL hold a table giving, for each principle, the tools its strip names, the address each link lands on and what that section says, and the clauses behind it. The table and the principles' records SHALL name the same tools and the same addresses.

#### Scenario: Reading the outline

- **WHEN** a writer opens `docs/method-outline.md`
- **THEN** it has a table with one row per principle, each naming its tools, their links and the clauses behind them, and these agree with what the Method page shows
