# Proposal

## Why

On the Method, "Capture at the source" still has only its faint numeral
where its diagram belongs. The site gives every principle a diagram, and
this one's idea was agreed with the operator on 2026-10-22
(`docs/method-outline.md`, 3. Capture at the source: Diagram;
`docs/site-design.md`, Every principle gets a diagram). Drawing it lets a
reader see the principle at a glance: work starts from what sources say,
not from a ticket someone remembered to write.

## What Changes

- "Capture at the source" gets its diagram, drawn beside its write-up on
  the Method in place of the numeral, and straight after the claim on a
  phone.
- The diagram shows four sources, a meeting, a chat, a log and a pull
  request, each with an adapter beside it.
- Events pulse at the sources, and each adapter's captures flow into one
  inbox, where each is listed by its key.
- The log's alert is sent twice. The second copy carries the same key and
  merges into the first, so the inbox keeps one capture, marked as sent
  twice.
- A reader session reads the meeting's capture into three signals: an ask,
  a constraint and a question.
- Below, a hand-written ticket form, which the principle rules out, is
  struck through.
- Four numbered captions under the drawing light in turn with the story.
- The diagram is drawn in the family's diagram colours and follows the
  reader's light or dark setting, as the other diagrams do.
- It moves with scripts off, since its motion is the drawing's own. Under
  reduced motion it holds one still frame in which every part is shown.
- Its alt text says what it shows, in sentences the house style accepts.

## Capabilities

### New Capabilities

- `method-diagrams`: each principle's diagram on the Method: what it
  shows, how it is coloured, how it moves, and what it says to a reader
  who cannot see it.

### Modified Capabilities

None. The project has no specs yet.

## Impact

- One new standalone SVG under `public/diagrams/`, in the same manner as
  the three diagrams already drawn.
- The principle's front matter names the diagram and its alt text. Its
  write-up, version and changelog are unchanged.
- No code changes: the Method page already draws any diagram a principle
  names, and the inlining in `src/lib/diagram.ts` already scopes, renames
  and recolours it.
- No new dependency.

## Touches

```
public/diagrams/capture-at-the-source.svg
src/content/principles/capture-at-the-source.md
```
