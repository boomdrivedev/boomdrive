## MODIFIED Requirements

### Requirement: A tool's link lands where the tool does it

Each tool on a strip that has a public product page SHALL link to the page, and the section of it, that does what the principle says; where the product page has no such section, it SHALL link to the section of the tool's design book that does. A tool with neither a public product page nor a public design book SHALL be named without a link.

#### Scenario: A product page section

- **WHEN** a reader follows sluice on "One shell"
- **THEN** it opens sluice's commands reference at `sluice exec`, `https://boomdrivedev.github.io/sluice/reference/commands/#sluice-exec`

#### Scenario: A tool with no public page

- **WHEN** a reader looks at "Intents and bolts"
- **THEN** Crew and Boomdrive are named in its strip as plain words, with no link

#### Scenario: Every link answers

- **WHEN** each link on every strip is requested
- **THEN** each page answers, and each link that names a section names a heading present on that page
