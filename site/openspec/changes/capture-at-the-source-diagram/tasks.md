# Tasks

## 1. The drawing

- [x] 1.1 `public/diagrams/capture-at-the-source.svg` exists, 900 by 476, with its `<title>`, its `<desc>` holding the alt text in design.md, one `<style>` whose palette copies `intents-and-bolts.svg`'s dark and light values, and every still part of the layout in design.md (header, column labels, four sources with their event dots, four adapters, connectors, the inbox's four row slots, the reader session, the ghosted hand-written ticket form, four captions); opened alone in a browser it shows every part legibly in light and dark, and its stylesheet names no colour outside the ten family diagram colours (design.md, The layout; Colour).
- [x] 1.2 The loop runs as design.md's table says, in CSS keyframes on one 16-second loop: four events pulse and their captures fill four keyed rows, the log's second capture merges into its row with a "sent twice · one capture" tag and no fifth row, the meeting's row is picked and the reader session shows its three signals, then the ticket form is struck, with the captions lit in turn; checked by watching two loops of the file opened alone, in both themes (design.md, The loop).
- [x] 1.3 The reduced-motion rule pauses every animation at 90% of the loop with all captions shown; with reduced motion emulated in a browser, the file opened alone shows the four sources and their event dots, four adapters, four rows with the sent-twice tag, the reader session with three signals, the struck ticket form and all four captions, and nothing moves (design.md, Reduced motion).

## 2. On the Method

- [x] 2.1 `src/content/principles/capture-at-the-source.md` names the diagram (`diagram.src: /diagrams/capture-at-the-source.svg`) with the alt text from design.md as `diagram.alt`, word for word the SVG's `<desc>`, and nothing else in the file changes; `devenv shell -- vale src/content/principles/capture-at-the-source.md` reports no error or warning, and the alt text piped to `vale --ext=.md` reports none either.
- [x] 2.2 `devenv shell -- moon run ':#check-fast'` passes, and the built `dist/method/index.html` holds an inline `<svg class="dg-capture-at-the-source" role="img" aria-label="…">` whose label is the alt text, with keyframes and ids renamed under `dg-capture-at-the-source` and no `:root` or `prefers-color-scheme` block left in its style.
- [x] 2.3 With the built site served, the Method shows the diagram beside "Capture at the source" in place of its numeral on a wide screen and straight after its claim at 390px width with no horizontal scroll; it follows the page in light and in dark; with scripts disabled it is visible and plays its loop; with reduced motion emulated it holds the full still frame.
- [x] 2.4 `site-check` passes in the devenv shell.
