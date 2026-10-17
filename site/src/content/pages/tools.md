---
title: Tools
description: The Flywheel family, four tools that each stand alone and the Method that joins them into one loop, from what is noticed to what lands.
---

Each tool does one job and is useful on its own. Flywheel is the Method that joins them into one loop, from what is noticed to what lands, and the engine that runs it.

## Each stands alone

- **headwater** reads what a repository declares and reports every release newer than its pin, with the notes and advisories. Its Claude Code and Herdr plugins keep unread releases in a sidebar, and one key starts an agent on the bump in a worktree.
- **eddy** lists and fetches recorded meetings across meeting services. Its Claude Code plugin pulls a transcript into a session, and its Herdr plugin shows the day's meetings.
- **millpond** keeps an inbox in a git repository of your own. A session curates it through millpond's tools, you accept each batch, and millpond sends an item to one worktree.
- **sluice** tests a change as merged and lands it. It never needs to know where the change came from.

## The Method joins them

The tools meet at contracts, never inside each other. headwater and eddy print captures in one format, and millpond records each event once, whichever host ran the adapter. An item becomes work in a worktree, and that work lands through sluice.

Today the engine that plans across items is Crew, our team runner, which calls millpond as it calls sluice. Flywheel's engine is designed to stand on the same parts as millpond and read the same inboxes.

So a tool can be taken up alone and joined later. Going from millpond on its own to Crew, and later to Flywheel, changes the host and the curator. The adapters and the inbox stay put: nothing is captured twice, and nothing is imported when Flywheel takes over.
