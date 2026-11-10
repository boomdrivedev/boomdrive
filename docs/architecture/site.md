---
paths:
  - site/**
  - .github/workflows/pages.yml
---

# Boomdrive's site rules

The rules that bind Boomdrive's site, `site/`, and the workflow that
publishes it, beside `core.md`, which binds wherever a change is made.
The site is the Method, its field notes and the fleet's measurements,
an Astro site served at `https://agentplot.github.io/boomdrive/`. Its
rules came with it from boomdrive-web (records 0025 and 0038).

## Rules

- **[site.1]** Nothing commercial is on the site: no pricing, revenue or product plan of ours, and a product appears only as a waitlist.
- **[site.2]** No client is named on the site, and no measurement or example comes from client work: a measurement reads our own repositories only, and a drive, machine or repository that carries client work is neither named nor counted.
- **[site.3]** A field note is never rewritten once it is published, and a later note supersedes it; the Method's pages are revised in place, each with a version and a changelog entry.
- **[site.4]** The glossary, `site/src/content/terms/terms.yaml`, is the site's one vocabulary: a term is added there, never under `site/styles/`, whose vocabulary `site/scripts/vocab.ts` writes from it, and Vale's error on the site's words fails its checks while its warning is fixed, never ignored.
- **[site.5]** A change to what the site shows, a diagram, a page's words, a figure or a tools strip, is content work: it runs as the repository's `content` kind, from `site/docs/content-brief.md` and a task line, and is not written up as an OpenSpec change.
- **[site.6]** The site is served under `/boomdrive/`, its repository's name, and every internal link goes through `url()` in `site/src/lib/url.ts`, never a bare root path.
- **[site.7]** Our own fleet's drives and machines are named only on the site's measurement and field-note pages, and in the content those pages are built from, as ours, a case study of our own fleet; the site's guides, reference, glossary and quick start, its design and its archived changes name none of ours, and no page presents a name of ours as the reader's own.

## Details

- **site.1** Rules out: a price, a tier, a revenue figure or a plan of ours for a product, on a page, in a diagram or in the site's design; a product shown as more than a waitlist. Not ruled out: a model vendor's list prices, cited and dated, that the site's measurements are costed with (core.6); a tool of the family named with its status, as `@agentplot/site-theme`'s family gives it. Source: boomdrive-web's `CLAUDE.md` and `openspec/config.yaml` at `4ed5fbe`; record 0025; core.6.
- **site.2** Rules out: a client's name; a measurement read from a repository, drive or machine that carries client work, or an example drawn from it; such a drive, machine or repository named or counted. Not ruled out: our own fleet's drives and machines named on the site's measurement and field-note pages, as ours (site.7). Source: boomdrive-web's `CLAUDE.md` and `openspec/config.yaml` at `4ed5fbe`; records 0025 and 0038; core.41.
- **site.3** Rules out: a published field note's words, figures or date changed; a principle's page changed with no new version and no changelog entry. Not ruled out: a note moved to a new path with its words unchanged, as the move into `site/` moves every note; a later note that supersedes one and says so; a typo's fix in a note not yet published. Source: boomdrive-web's `CLAUDE.md` at `4ed5fbe`; record 0025.
- **site.4** Rules out: a term added under `site/styles/`; a vocabulary or terms file edited by hand; a Vale warning on words a change wrote left in place; a Vale error passing the checks. Not ruled out: the files `site/scripts/vocab.ts` writes, tracked as it writes them. Source: boomdrive-web's `CLAUDE.md` and `openspec/config.yaml` at `4ed5fbe`; record 0025.
- **site.5** Rules out: an OpenSpec change written up for a change to what a page or a diagram shows; content work run as any kind but `content`. Not ruled out: an OpenSpec change to how the site is built, a new page type, its schema or its checks, under the site's rules in `openspec/config.yaml`; several pages or diagrams in one task line. Source: boomdrive-web's `CLAUDE.md` and its `content-kind` spec at `4ed5fbe`; record 0025.
- **site.6** Rules out: a link written as a bare root path, `/notes/`, in a page or a component; a built page whose link or asset leaves `/boomdrive/` for this site's own pages; the site served under boomdrive-web's name. Not ruled out: a link to another site of the family, or anywhere else, by its full address; boomdrive-web's address pointing readers to this site (record 0025); the family's entries `@agentplot/site-theme` draws, its Boomdrive member's `site` and its `hub`, as the theme gives them, `/boomdrive-web/` until the theme's release at the opening names the new address (record 0040). Source: boomdrive-web's `CLAUDE.md` and its record 0002 at `4ed5fbe`; record 0025; the design agent, 2026-11-09, answering the code stage of `boomdrive-keeps-its-site`.
- **site.7** Rules out: a drive or a machine of ours named in a guide, the reference, the glossary, the quick start, the site's design under `site/docs/` or an archived change; an example that gives a name of ours as the reader's drive, machine or host; a name of ours anywhere but `site/src/content/measurements/`, `site/src/content/contexts/`, `site/src/content/notes/` and the pages built from them. Not ruled out: those pages naming our drives and machines as ours, which boomdrive-web published before the move (core.41); the site's history as boomdrive-web published it, our names and the agents named after a drive in its messages (record 0025); MadSwan named as the company, which is no drive or machine; neutral names in the site's design and examples, as this repository's tests take them. Source: the user, 2026-10-26, that the site never presents a name of ours as the reader's own; the planner's answer to the construct stage of `boomdrive-keeps-its-site`, 2026-11-09; records 0025 and 0038; core.41.
