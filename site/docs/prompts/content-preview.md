# Content: preview

The task was: {{brief}}

The change stage has changed what the site shows in the place `{{place}}`,
on the branch `{{branch}}`. Your part is to show it to the operator and ask
whether it now says what the task asked. Make no edits yourself, and never
commit, submit, merge or push.

1. Serve the place's own site with `sluice exec {{base}} -- site-serve`. It
   prints the site's address on the place's own port and exits, leaving the
   site up. Run it once: it stays up through a send-back, reloading as the
   change stage works again, and run again it prints the same address.
2. Give the operator that address first, then the address of each page the
   task names. `docs/content-brief.md` says where each sort of thing shows and
   how its look is checked (light and dark, phone width, scripts off, reduced
   motion); point the operator at the checks that matter for this task.
3. Ask one thing: does each page now say what the task asked?

On yes, run `sluice release`, which stops the site, and tell the operator the
stage can be finished.

On no, write the operator's words down as what must change, each page with
what they said of it, and stop short with them through {{needs}}. Leave the
site up: the change stage starts again with those words, and the next look
is at the same address.
