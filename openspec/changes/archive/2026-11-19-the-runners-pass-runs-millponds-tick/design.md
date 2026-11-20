# Design

## Decision 1: The configuration names the inboxes

Record 0033 has the pass tick "each inbox the host's configuration names". The runner's file names them in `inboxes`, by millpond's own names for them, so the runner reads no file of millpond's (core.5) and the set's module, which writes both files, names each inbox once and gives the runner its names.

## Decision 2: The tick runs inside the watch, after the settle

The settle's watch already runs once at the end of each pass and is shared between the loop and the parts' threads, so the ticks run there, after the settle, under the same lock: no two ticks of one inbox run at once, and a pass that ends while a settle is under way shares its ticks too.

## Decision 3: millpond's answer is its exit

millpond prints its answer and exits 0, or says `millpond: <why>` on standard error and exits 1. A tick is answered where it exits 0, refused with its words, the prefix left off, where it exits otherwise, and unanswered where it cannot be run or a signal ends it. The runner reads nothing of the answer's JSON (core.1, core.11).
