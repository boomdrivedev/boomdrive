---
title: Tools
description: The Flywheel family, tools that each stand alone and the Method that joins them into one loop, from what is noticed to what lands.
---

Each tool does one job and is useful on its own. Flywheel is the Method that joins them into one loop, from what is noticed to what lands, and the engine that runs it.

## Each stands alone

- **headwater** reads what a repository declares and reports every release newer than its pin, with the notes and advisories. Its Claude Code and Herdr plugins keep unread releases in a sidebar, and one key starts an agent on the bump in a worktree.
- **eddy** lists and fetches recorded meetings across meeting services. Its Claude Code plugin pulls a transcript into a session, and its Herdr plugin shows the day's meetings.
- **millpond** keeps an inbox in a git repository of your own. A session curates it through millpond's tools, you accept each batch, and millpond sends an item to one worktree.
- **flume** launches one agent in one worktree for an item, runs its stages, and tidies the place once the work lands. `/launch` starts one from any session, and each stage ends when its deliverable exists, never when the pane goes quiet.
- **sounding** works one question in one session, in a worktree of its own, and ends on a conclusion: the answer, its evidence and the work it proposes. `/sound` opens one from any session, and you accept the conclusion, send it back for another round, or drop it.
- **sluice** tests a change as merged and lands it. It never needs to know where the change came from.

## The Method joins them

The tools meet at contracts, never inside each other. headwater and eddy print captures in one format, and millpond records each event once, whichever host ran the adapter. millpond sends an item to flume, which launches its work in one worktree, and the work lands through sluice. An item that asks a question goes to sounding instead, and its accepted conclusion comes back to millpond as new items.

Today the engine that plans across items is Crew, our team runner, which calls millpond and flume as it calls sluice. Flywheel's engine is designed to stand on the same parts as millpond and read the same inboxes.

So a tool can be taken up alone and joined later. Going from millpond on its own to Crew, and later to Flywheel, changes the host and the curator. The adapters and the inbox stay put: nothing is captured twice, and nothing is imported when Flywheel takes over.
