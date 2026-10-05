# flywheel-web

The site where we learn in public how to run a fleet of coding agents, from
five agents to five hundred: a handbook with a lab notebook attached. The
Method is kept current, field notes are dated and age honestly, and live fleet
numbers show what is happening now.

Working name; the brand and domain are not chosen.

## What is here

| Path | What |
|---|---|
| `docs/site-design.md` | The decisions, the site map, the content model and its types, how today's practice and what is coming are shown, and how measurements are collected |
| `docs/method-outline.md` | The Method's principles, each sketched as its page would read, and the reshaped list proposed for v0.1 |
| `docs/measurement-roadmap.md` | Candidate measurements, by the stage that makes them measurable |
| `docs/audience-plan.md` | How readers find the site: targets, the weekly field note, channels and the first 90 days |

## Running it

Every tool comes from devenv: `devenv shell`, or `devenv shell -- <cmd>`.

```sh
site-dev     # the site on localhost, drafts included
prose        # the glossary into Vale's vocabulary, then Vale over src/content
site-check   # the gate: astro check, prose, then the build
```

| Path | What |
|---|---|
| `src/content.config.ts` | The page types and their schemas |
| `src/content/` | Principles, notes, the glossary (`terms/`) and the stack (`components/`) |
| `src/pages/` | Home, the Method, Notes, Uses, About |
| `.vale.ini`, `styles/Flywheel/` | The house style |
| `scripts/vocab.ts` | The glossary into Vale's vocabulary |
