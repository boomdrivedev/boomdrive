## MODIFIED Requirements

### Requirement: Each of the site's checks is one task

The site's check-fast tasks SHALL be one check each: `check`, Astro's type check; `build`, the site built into `site/dist`; `test`, the site's tests; and `workflow`, actionlint over `.github/workflows/pages.yml`. Its `install` and `vocab` tasks SHALL be in no layer, run as the tasks and the shell's scripts that need them ask. No task of the site's SHALL read a Markdown file of the site's but its type check and its build, Astro's own reading of its content (core.7; record 0049).

#### Scenario: The site's type check

- **WHEN** `moon run site:check` runs
- **THEN** Astro's check reports no error in the site's pages, components or content, every content reference resolving

#### Scenario: The site's build

- **WHEN** `moon run site:build` runs
- **THEN** the site is built into `site/dist`, its content's schemas and references holding

## REMOVED Requirements

### Requirement: The site's words are held to its glossary

**Reason**: No task of the gate reads a Markdown file (blueprints principle 16; record 0049). `site:prose` ran Vale over the site's content, so it goes.

**Migration**: Vale stays the writers' own check of their words: the shell's `prose` writes the vocabulary from the glossary with `site:vocab` and runs Vale over `site/src/content` under `site/.vale.ini`, and `site-check` runs it after the site's tasks. site.4 keeps the glossary the one vocabulary, and a Vale warning on words a change wrote is fixed by its writer.
