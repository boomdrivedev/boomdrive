---
order: 3
title: "Capture at the source"
summary: "Work begins as captures taken where things happen, a meeting, a chat, a log, a pull request, by adapters that run unattended, so development is fed by what people and systems say rather than by tickets someone remembered to write."
status: draft
introducedIn: "0.1"
lastReviewed: 2026-10-12
related: [intents-and-bolts, two-loops-one-gate]
---

A capture is one source event as recorded: a meeting, a day of a chat channel, a log run, a forwarded message. It holds where it came from, when, who captured it, and a pointer to the raw material, which stays outside version control. Capturing the same event twice yields one capture ([Flywheel Next statement](https://github.com/agentplot/blueprints/blob/main/design/flywheel-next/requirements.md), clause 111).

Capture costs no more than a sentence: a forwarded message, a word on the phone, a file dropped in a folder, a key on the page (clause 112). Nobody is asked to classify what they wrote.

An adapter is what makes capture automatic. It enumerates a source's events and writes one capture per event, unattended. Reading a capture into signals is judgment, so a session does it and nothing does it unattended (clauses 115 and 215). The adapters cover a chat forward, a meeting transcript, a log or monitor webhook, a pull-request conversation, an issue tracker and a folder drop.

Curation then moves every signal. It attaches the signal to an open intent, challenges a standing claim, joins a proposed intent, marks it answered, routes it to a chore, or drops it with a reason. An unmoved signal is never discarded (clauses 116 and 118).

## How we do it today

Findings are recorded by hand with Crew's signal command, and the first corpus of captures was written by hand from meeting transcripts.

## Where it's going

Flywheel Next's adapters, each running on whichever host declares its source, writing captures from any machine (clause 217g). A meeting ends and its decisions are already waiting as signals beside the intents they bear on.
