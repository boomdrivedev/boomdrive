## 1. The family with tailrace

- [x] 1.1 In `package.json`, move `@agentplot/site-theme` to `github:agentplot/site-theme#v0.1.2`, then run `devenv shell -- bun install` to rewrite `bun.lock`; prove it by `bun.lock`'s site-theme entry naming `#ad9299e`, by `grep -n "'deliver'" node_modules/@agentplot/site-theme/src/family.ts` finding tailrace's place, and by `devenv shell -- moon run :install` passing with the frozen lockfile

## 2. tailrace's card

- [x] 2.1 In `src/pages/tools.astro`, add `{ id: "deliver", label: "Deliver" }` to `places` after Land; prove it with `devenv shell -- moon run :check` reporting no error, and with the built Tools page (`devenv shell -- moon run :build`, then `dist/tools/index.html`) showing six stops in every card's track, Notice to Deliver, tailrace's `aria-current="step"` on Deliver
- [x] 2.2 In `src/pages/tools.astro`, map tailrace to `two-loops-one-gate` in `serves`, with the comment design.md gives its reason in; prove it with `devenv shell -- moon run :build` passing, the built page's tailrace card naming Two loops, one gate under "In the Method" and linking `/flywheel-web/method/#two-loops-one-gate`, every other card naming what it did before, and by mapping tailrace to `one-shell` for a moment, which makes the build fail naming tailrace and `one-shell`, then putting it back
- [x] 2.3 In `src/content/terms/terms.yaml`, give the `tailrace` term `principle: two-loops-one-gate` between `definition` and `lowercase`; prove it with `devenv shell -- moon run :vocab` leaving `git status styles/` clean and `devenv shell -- moon run :prose` reporting no error or warning
- [ ] 2.4 Look at the Tools page in a browser on `sluice exec main -- bun run dev`, at full width, at each width where the grid drops a column, and at 375 pixels: no two stops' names overlap on any card, Flywheel's included; where they do, raise the `@container` threshold, or adjust Flywheel's track, as design.md says, and look again. Prove it by the page in light and dark showing tailrace's card in its teal after sluice's, the headline "Seven tools, one loop.", tailrace under "Designed, not built" in the legend, tailrace's dot on its name in "Each stands alone" and the diagram's seven steps; at 375 pixels with no sideways scroll; with scripts off showing every card whole; and under reduced motion with the diagram stopped at its last step, every step lit
- [ ] 2.5 Prove the Tools page's words are unchanged and still carry tailrace: `git diff main -- src/content/pages/tools.md` reports nothing, and the built page's "The Method joins them" holds "tailrace carries the landing to a channel and to every environment that follows it."

## 3. Integration

- [ ] 3.1 Run `site-check` in the devenv shell and `devenv shell -- moon run ':#check-fast'`; both pass, with Vale reporting no error or warning
