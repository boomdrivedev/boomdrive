# Flywheel is renamed Boomdrive

- Status: proposed
- Date: 2026-11-04
- Supersedes: [0001](0001-the-site-is-flywheel.md), whose reasons against
  MadSwan and a personal name still hold

## Context and problem statement

The user renamed Flywheel across every repository at once: "the ecosystem,
the org, the branding, the cloud and the opensource runner tool are all
boomdrive. There is also a "boom" open source tool that does the claim
reconciliation." The site was named Flywheel (record 0001), and its
repository, its pages, its glossary, the fleet's records and its rules all
carried the name, with Flywheel Next for the engine to come and tailrace
for the tool that delivers. What does each become here?

## Decision drivers

- The user's words above, and their rulings with them: a customer's unit of
  work, a Flywheel, is a drive, plural drives; Crew's flywheel, a
  partition's loop, follows the unit's word; tailrace is timber, at
  agentplot/timber; and the spelling is enroll, enrolls, enrolled,
  enrolling and enrollment.
- Boomdrive is one word with a capital B in prose, and `boomdrive` in code,
  commands, packages, paths and repository names.
- The glossary is the one vocabulary (`CLAUDE.md`), so a word the site uses
  changes there first.
- The family's members, names, lines and sites come from
  `@agentplot/site-theme`'s family, never a list of the site's own.
- A record's or a rule's names change, never its decision.

## Considered options

The rename leaves one name to this site: the four-ring figure Home opens
its benefits on, which agentplot/flywheel's site called the flywheel.

- The drive figure
- The flywheel, kept as the picture's name
- Boomdrive's figure

## Decision outcome

The names, and where each lands here:

| Was | Is | Where |
|---|---|---|
| Flywheel, the site, the brand, the ecosystem and the family of tools | Boomdrive | The site's title and name, Home, the Tools hub, the principles, the Method outline, the site design, the diagrams' notes |
| Flywheel Next, the engine the site names as coming | Boomdrive | The principles' tool strips and where each is going, the Uses page's `becomes` for Crew, the measurement roadmap's third stage |
| agentplot/flywheel-web | agentplot/boomdrive-web | `package.json`, the base path `/boomdrive-web/` GitHub Pages serves the site under, the masthead's repository link, `site-serve`'s address, devenv's task, the bare layout |
| Crew's flywheel, one loop we operate | drive, plural drives | The glossary, the fleet context's `drives`, a series' `drive` split, the Fleet's pages, the measurements' definitions |
| tailrace | timber, at `agentplot.github.io/timber/` | The glossary, the tool strips, the Tools hub's words, the diagrams and the command they show, `timber promote` |
| `styles/Flywheel/`, Vale's style and vocabulary | `styles/Boomdrive/` | `.vale.ini`, `scripts/vocab.ts`, `moon.yml` |

Chosen option for the figure: "The drive figure", in
`src/components/Drive.astro`, because what it draws is one drive's loop:
the operator's review, intent, bolts and construction turning together.
Keeping "the flywheel" would leave the old name on Home's first picture,
and "Boomdrive's figure" names the brand rather than what the picture
shows. The choice is this record's, not the user's, and is cheap to
change.

What keeps its name:

- agentplot/flywheel, the original, is archived; the drive figure's source
  names it so.
- agentplot/flywheel-next, the original design, which the rename does not
  rename: the clauses the Method outline cites are its statement's, and it
  stays among the repositories the measurements plan to read.
- Record 0001 and the Method's changelog keep their text, as the decisions
  and revisions they recorded; so do the archived OpenSpec changes.
- The measurements keep their definition versions: what each counts is
  unchanged, and only the word for what a series is split by is new.
- boom, and anything third-party that says flywheel; the site names
  neither.
- What the site takes from `@agentplot/site-theme`'s family, at the tag
  it pins, keeps the theme's names until the site takes the tag of the
  theme that renames them: the ids it passes the theme (`flywheel`,
  `tailrace`), and tailrace in Never prove twice's strip, which the Tools
  hub's build holds to the card the theme draws. The bar, the footer, the
  family's diagram and the Tools hub's cards say Flywheel and tailrace
  until then. The site lands without waiting for that tag, because GitHub
  Pages already serves it at `/boomdrive-web/` and its links break until
  the base path changes. Translating the theme's names in the hub would
  let a card and its principle's strip disagree, which the hub's check is
  there to refuse.

### Consequences

- Good: the site, its address and its pages say the name every repository
  of the family now says, and the site's links work under the address
  GitHub Pages serves it at since the repository was renamed.
- Bad: GitHub Pages does not forward `/flywheel-web/` or `/tailrace/` to
  the new addresses, so a link to either from elsewhere breaks until it is
  changed.
- Bad: "drive" is a common word, so the glossary's term reads as plain
  English where a reader does not know it.
