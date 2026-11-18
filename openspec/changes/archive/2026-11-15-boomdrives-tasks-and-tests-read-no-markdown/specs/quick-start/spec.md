## ADDED Requirements

### Requirement: A step is a record its writers keep whole

Each step SHALL be one Markdown record of the collection `start`: its title, its place on a Mac, on NixOS or both, each file it shows under `site/src/start/` with the path the reader writes it to where they write one, and its test, a project, `boomdrive` or `nix`, and a name. The build SHALL refuse a step with no place, no file or no test. Its places' order, and that its test is one its project holds, SHALL be its writers' to keep, since no task reads a step's record (site.8; record 0049).

#### Scenario: Every step is whole

- **WHEN** `site:check` and `site:build` run
- **THEN** each record under `site/src/content/start/` gives a place on at least one platform, at least one file and a test, and neither reports an error

### Requirement: Every file of the quick start's is on its page

Every file under `site/src/start/` but the folder's shellcheck settings SHALL be a file some step shows, on the built page as it stands (site.8).

#### Scenario: No file is left unshown

- **WHEN** `site:test` lists `site/src/start/` and reads the built page
- **THEN** every file there but `.shellcheckrc` is, escaped as HTML, the text of a block under a step

## MODIFIED Requirements

### Requirement: The page shows each step's files as they stand

The page SHALL show each file a step names in a block of its own, its text exactly as the file holds it, and SHALL name above the block the path the reader writes it to where the step gives one. No step's file SHALL be shown in any other form, and no block under a step SHALL hold text that is no file under `site/src/start/` as it stands.

#### Scenario: Each file as it stands

- **WHEN** `site:test` reads the built page's steps and each file under `site/src/start/`
- **THEN** the text of each block under a step is, escaped as HTML, the text of one of those files

### Requirement: The quick start names none of ours

No step's words or files, and nothing on the page, SHALL name a drive or a machine of the fleet's contexts. The reader's own names SHALL be the placeholders `your-github-name`, `your-repository`, `your-host` and `you`, the account `me` and the drive `home` (site.7).

#### Scenario: None of our drives or machines

- **WHEN** `site:test` reads every drive's and machine's name in `site/src/content/contexts/contexts.yaml`
- **THEN** none appears on the built quick start or in a file under `site/src/start/`

## REMOVED Requirements

### Requirement: A step is a record of its words, its files and its test

**Reason**: Its scenario "A platform's places are its order" had `site:test` read each step's record, a Markdown file, which no task does (blueprints principle 16; record 0049).

**Migration**: "A step is a record its writers keep whole" holds the record, its schema and the build's refusal, and leaves a platform's places in order to the step's writers.

### Requirement: Every step names a test its project holds, and every file is a step's

**Reason**: Its scenario "Each step's test is there" had `site:test` read each step's record for the test it names, and "No file is left unshown" read the records for the files they show, each a Markdown file, which no task does (blueprints principle 16; record 0049).

**Migration**: "A step is a record its writers keep whole" leaves the test a step names to its writers, and "Every file of the quick start's is on its page" has `site:test` find every file on the built page.
