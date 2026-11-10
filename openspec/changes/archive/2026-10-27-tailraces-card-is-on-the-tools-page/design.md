## Context

See proposal.md for why. The design rests on `docs/site-design.md`'s "The Tools hub" (the family is one list, and it is not ours; the hub reads in four parts, each card with its place in the loop lit and the principle it carries; its words are content) and the design note `agentplot/blueprints design/tailrace/models/the-method-names-its-tools.md`, "Each principle names the tools that realise it" (the principle table and "The Tools page gains tailrace").

As the code stands on main (`b8a4576`):

- `package.json` pins `@agentplot/site-theme` at `github:agentplot/site-theme#v0.1.1`, which `bun.lock` resolves to `90e7b0f`. That family has seven members and no tailrace; its `Place` is `umbrella | notice | inbox | launch | work | land`.
- site-theme v0.1.2 (annotated tag `ed248ed` on commit `ad9299e`, both on GitHub) adds tailrace to `src/family.ts`: line "Build once, sign everything, promote with proof, in your own account.", place `deliver`, status "designed, not built", site `agentplot.github.io/tailrace/`, book `agentplot.github.io/blueprints/tailrace/`, accent `#1d7368` / `#3cc4ad`. `Place` gains `deliver`. `Family.astro` and `src/diagram.ts` draw a seventh step, "tailrace publishes the landing", with tailrace's station under main and the step's head labelled DELIVER; the drawing's three copies grow taller. Nothing else the site imports changes: `FamilyBar.astro`, `FamilyFooter.astro`, `base.css` and the exports `member`, `family`, `Place`, `Status`, `MemberId` keep their shapes.
- `src/pages/tools.astro` keeps its own `places` list, five entries from Notice to Land, which every track draws; `placeOf()` falls back to the raw id for a place the list lacks. Its `serves` map chooses each card's principle, and `principle()` fails the build when that principle's `tools` does not name the card's tool. The cards run `--cols` across: one row up to five tools, `ceil(n / 2)` beyond, so seven tools give four columns. A `@container (max-width: 15rem)` rule hides every stop's name but the card's own.
- `src/content/pages/tools.md` already carries tailrace's entry in "Each stands alone" and the sentence "tailrace carries the landing to a channel and to every environment that follows it." in "The Method joins them" (the archived change `the-method-names-its-tools`).
- Two principles' strips name tailrace: `two-loops-one-gate.md` (tailrace first, linking `tailrace/how-it-works/#channels-moved-only-by-proof`) and `never-prove-twice.md` (tailrace first, linking `tailrace/#one-store-three-jobs`).
- The glossary's `tailrace` term has no `principle`, because tailrace had no card; each sibling family term's `principle` is the one its card names, except `sounding`, whose term kept `two-loops-one-gate` when its card moved.

## Goals / Non-Goals

**Goals:**

- The Tools page reads tailrace from the family like every other member, with no list of the site's own beyond the loop's place labels it already keeps.
- tailrace's card carries one principle, checked against that principle's strip by the rule already in place.

**Non-Goals:**

- Any change to the Tools page's words, a principle's record or strip, or `docs/`.
- tailrace's line on its card. It is the family's (`site-theme`), read as every card reads its member's; changing it is an edit in site-theme.
- The `sounding` term's `principle`, which disagrees with its card; that predates this change.

## Decisions

### Move the pin to v0.1.2

`package.json` names `github:agentplot/site-theme#v0.1.2`, and `bun install` rewrites `bun.lock`'s entry to resolve `ad9299e`, as v0.1.1 resolved its commit. The tag form stays: it is how the site pins the theme today, and a family site moves by tag. Rejected: pinning the commit hash, which no other part of the site does.

### Deliver joins the loop's places

`places` gains `{ id: "deliver", label: "Deliver" }` after Land, so every track has six stops and tailrace's lights its own. The label matches the diagram's DELIVER head. `places` stays the site's list: the theme exports `Place` as a type but no ordered list with labels, and the type now includes `deliver`, so `{ id: Place }` checks the new entry. Rejected: building the track from the family's members' places, which would drop a place no member holds and cannot order them.

### tailrace's card carries Two loops, one gate

`serves` gains `tailrace: "two-loops-one-gate"`, with a comment: the word that moves a channel is the second place Two loops, one gate puts the operator's word, and moving a channel is what tailrace does at Deliver. The design note names tailrace in two strips and does not choose a card's principle. Two loops, one gate is the one whose strip links what tailrace does at its own place in the loop (channels moved only by proof), and whose diagram ends on tailrace's channel; Never prove twice links tailrace's cache, a job beside its delivery. No card carries Two loops, one gate since sounding's moved to Measure the fleet, and no card carries Never prove twice either, so the choice leaves one principle without a card either way.

The `tailrace` term in `terms.yaml` gains `principle: two-loops-one-gate`, after `definition` and before `lowercase`, as its siblings order theirs. Rejected: leaving it without one, which made sense only while tailrace had no card.

### Six stops on a narrow card

Six names in the space five had may collide. The `@container` threshold that hides every name but the card's own is raised to the narrowest card width at which all six names still fit apart, measured in a browser with the Tools page at the widths its grid takes (four columns at full width, then fewer as it narrows, one at 375 pixels). If six names fit wherever five did, the threshold stays at 15rem. Flywheel's track, in its wider column, is checked at 375 pixels too, where its card is one column about 295 pixels inside: six names of up to seven mono capitals fit there with a few pixels between them. If they collide all the same, that track's names lose their letter-spacing below 48rem, where the umbrella card already changes its layout, and keep every name, since Flywheel's card lights every stop. Rejected: shortening the labels, which would make the track disagree with the diagram's heads.

### What follows from the bump without code

The headline's count, the cards' column count, the legend's lists, `named()`'s dot on tailrace's bold name, the bar and footer on every page, and the diagram's seven steps all read the family. They are checked, not written.

## Risks / Trade-offs

- [The diagram grows taller at every width (wide 436 to 456, square 798 to 934, tall 1024 to 1162)] → it is the theme's drawing and the hub's only figure; the task checks it in light and dark, at phone width and under reduced motion, where it stops at the loop's last step with every step lit.
- [Seven cards in four columns leave the second row one short] → the grid already handles a short row (`auto-fit` with a floor of 13rem); the cards keep their width.
- [The design agent prefers Never prove twice for tailrace's card] → it is one line in `serves` and one in `terms.yaml`, and the build's check holds either.
- [bun cannot reach GitHub during install] → the install is the same fetch v0.1.1 needed; the tag and its commit are both on GitHub.
