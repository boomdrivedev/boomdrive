# Content: change

The task: {{brief}}

You are in the place `{{place}}`, on the branch `{{branch}}`, cut from
`{{base}}`. The task changes what this repository's site shows: a diagram, a
page's words, a figure or a tools strip, one of them or several.

1. Read `docs/content-brief.md` whole before you edit anything. It says what
   to read first, where the agreed ideas are, the conventions of each sort of
   thing the site shows with finished examples, where each lands, and what
   proves a change.
2. Read each page or diagram the task names, as it stands in this place.
3. Make the change the task asks for, to every page or diagram it names.
4. Run `site-check` in the devenv shell and fix what it reports, every Vale
   warning on words you wrote included, until it passes.
5. Commit by explicit paths with a Conventional Commit: `git add <paths>`,
   then `git commit -m "<subject>" -- <paths>`.
6. Run `sluice exec {{base}}` on the committed work; fix, commit and run it
   again until it passes.

{{#words}}
This place was sent back. What must change, in the operator's words or a
failed landing's:

{{words}}

Change what they name, then run the checks and commit as above.
{{/words}}

Where only the operator can settle something, such as a diagram whose idea
the brief's sources do not record as agreed, do not guess: stop short with
{{needs}}, its words naming what you need.

Write no OpenSpec change, spec or report: the commits and the checks are the
record. Never submit, merge or push; the place is submitted once the operator
has looked at it.
