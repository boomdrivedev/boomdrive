# Tasks

## 1. Serving the site in a place

- [x] 1.1 Add the `site` process and the `site-serve` and `site-stop` scripts to `devenv.nix` as the design's "Serving the site in a place" says; prove `devenv shell -- site-stop` exits zero in this worktree with nothing served, and `devenv shell -- site-serve` outside `sluice exec` fails with a message naming `SLUICE_PORT_WEB`
- [x] 1.2 Add `[checkout] ports = ["web"]` and `[tests] release = "site-stop"` to `.config/sluice.toml`, with a comment saying what each is for; prove `sluice exec <bolt> -- site-serve` prints an address on the worktree's port and exits, and that `curl -sf` on that address answers after the command has ended (where `<bolt>` is this worktree's base branch)
- [x] 1.3 Prove release: `sluice release` in this worktree stops the server and the address no longer answers; then in a fresh detached checkout of this branch, `devenv shell -- site-stop` exits zero with no output that fails, as a candidate's release would
- [x] 1.4 Prove two places at once: serve from this worktree and from a second worktree of this branch under `sluice exec`, check the two addresses use different ports and both answer, then release both

## 2. The kind and its prompts

- [x] 2.1 Write `.config/flume.toml` as the design's "The declaration" gives it, with its comment; prove it parses (`python3 -c 'import tomllib;tomllib.load(open(".config/flume.toml","rb"))'`) and that each stage names one agent, the change stage one deliverable `commits`, the preview stage `conversation = true`, `wait = true` and `on_fail = "change"`, and `[profiles] merge_queue = "sluice"` is set
- [x] 2.2 Write `docs/prompts/content-change.md` as the design's "The prompts" says; prove every `{{…}}` part it uses is one of C.2's parts (a grep for `{{` lists only those), that it names no flywheel-web page, and that it asks for no OpenSpec change and never asks to submit, merge or push
- [x] 2.3 Write `docs/prompts/content-preview.md` as the design says; prove the same grep for its parts, that it names no flywheel-web page, and that its serve command is the one task 1.2 proved
- [x] 2.4 Prove each stage's `prompt` path, resolved from `.config/`, names a file that exists (`test -f .config/../docs/prompts/content-change.md` and the same for preview)

## 3. The content brief

- [x] 3.1 Write `docs/content-brief.md` with the five parts the design's "The content brief" lists, drawn from the archived diagram designs and the SVGs as they are; prove each diagram convention it states holds in at least one of `capture-at-the-source.svg` and `never-prove-twice.svg` (viewBox width, palette blocks, loop length, reduced-motion rule, `<desc>` equal to the principle's `alt`)
- [x] 3.2 Prove every repository path the brief names exists: extract each backticked path and check it with `test -e`, reporting none missing
- [ ] 3.3 Prove the brief points to `docs/method-outline.md` for each Diagram line and the tools table rather than copying them: no Diagram line's text from the outline appears in the brief

## 4. The repository says so

- [ ] 4.1 Add the content-work paragraph to `CLAUDE.md` as the design's "CLAUDE.md" gives it; prove it names `docs/content-brief.md` and `.config/flume.toml` and that `CLAUDE.md` holds no instruction to propose an OpenSpec change for a page or diagram

## 5. The generic diagram spec

- [ ] 5.1 Check the drawn diagrams against the generic requirements in this change's `method-diagrams` delta: for each of the six SVGs, its `viewBox` width is 900, it uses only the ten family colour tokens, it carries a reduced-motion rule, and its principle's `alt` equals its `<desc>`; report any that does not as a finding for a later content task, and change no diagram here
- [ ] 5.2 Run `openspec validate the-content-kind-and-its-brief --strict` and prove it reports the change valid

## 6. The gate

- [ ] 6.1 Run `sluice exec <bolt>` on the committed work and prove the gate's tasks pass
- [ ] 6.2 Run `site-check` in the devenv shell and prove it passes, with Vale reporting no error or warning
