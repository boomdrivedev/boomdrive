## Why

The Tools page already names tailrace in its words, but shows no card for it: the cards come from `@agentplot/site-theme`'s family, and the version the site pins (v0.1.1) does not list tailrace. site-theme v0.1.2 (tag `ed248ed` on commit `ad9299e`) adds tailrace to the family at a new place in the loop, Deliver, after sluice's Land. Moving the pin, and giving the card's track that stop and the card its principle, finishes what the design note asks of the Tools page (`agentplot/blueprints design/tailrace/models/the-method-names-its-tools.md`, "The Tools page gains tailrace").

## What Changes

- The site's pin on `@agentplot/site-theme` moves from v0.1.1 to v0.1.2, so the family it reads lists tailrace: its line, its status (designed, not built), its site, its design book and its teal accent.
- The Tools page shows tailrace's card after sluice's, in the same form as the others: its track lit at Deliver, its status, its one line, the principle it carries, and links to its site and design book.
- Every card's track, and Flywheel's, gains a sixth stop, Deliver, after Land, in the order the loop runs. A card too narrow to name every stop names only its own, as now.
- tailrace's card names **Two loops, one gate** under "In the Method": the principle whose "In the tools" strip names tailrace first, for the word that moves a channel, which is what tailrace does at Deliver. The glossary's `tailrace` term takes the same principle, as each sibling term carries its card's.
- The headline counts seven tools, the legend lists tailrace under "Designed, not built", tailrace's name in the page's prose takes its dot and accent, and the family's diagram above the cards draws its seven steps with tailrace's Deliver among them. All of these follow from the bumped family with no change to the page's words.
- The words need no change: "Each stands alone" already gives tailrace's entry in the others' voice, and "The Method joins them" already says the work lands through sluice and tailrace carries the landing to a channel and to every environment that follows it. The change keeps both as they read.

This carries out `docs/site-design.md`'s "The Tools hub" (a card per member of the family's one list, each with its place in the loop lit and the principle of the Method it carries, one whose strip names the tool or the build fails), and the design note's "Each principle names the tools that realise it", whose table puts tailrace in the strips of Two loops, one gate and Never prove twice.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tools-hub`: the Tools page gains tailrace's card at the Deliver stop, every track gains that stop, and the cards as built include tailrace naming Two loops, one gate.

## Impact

- `package.json` and `bun.lock`: the site-theme pin, v0.1.1 to v0.1.2, resolving to `ad9299e`. Every page's family bar and footer list tailrace too, since they read the same family; the Tools page is the only page that draws the family's diagram.
- `src/pages/tools.astro`: the loop's places gain Deliver, the card's principle map gains tailrace, and the narrow-card rule is retuned if six stops' names no longer fit where five did.
- `src/content/terms/terms.yaml`: the `tailrace` term gains `principle: two-loops-one-gate`. `scripts/vocab.ts` does not read `principle`, so the files it generates in `styles/` do not change.
- No principle, no page's words and no doc changes: the strips already name tailrace, and the Tools page's words already carry it.

## Touches

```
package.json
bun.lock
src/pages/tools.astro
src/content/terms/terms.yaml
```
