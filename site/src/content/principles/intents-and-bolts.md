---
order: 4
title: "Intents and bolts"
summary: "Work is an intent broken into bolts made of units, one reviewable change each. A unit merges into its bolt as it finishes, and the bolt lands on the shared line as one delivery."
line: "Work is an intent, landed in bolts of small units."
status: draft
introducedIn: "0.1"
lastReviewed: 2026-10-12
diagram:
  src: /diagrams/intents-and-bolts.svg
  alt: "An intent, one plain sentence of what someone wants, breaks into two bolts, each one delivery. Each bolt breaks into three units, one reviewable change each. As each unit finishes, it merges into its bolt's line, a short branch off the shared line, and the shared line does not move. Once all three units are in, the bolt lands on the shared line as one step; then the second bolt does the same. Below, a long-lived feature branch of 3,000 lines, which the principle rules out, keeps taking commits but never lands, and is struck through."
---

## How we do it today

Crew's bolts and units, as OpenSpec changes.

## Where it's going

Flywheel Next tracks intents from capture to landed.
