## MODIFIED Requirements

### Requirement: An act asks one thing of its launch's place

An act SHALL ask one of these of its launch's place:

- a retry, with its stage and its words where it names them, and whether it names the submission;
- a land, which asks for the landing of its launch's work, and holds nothing but its launch;
- a confirm, which asks ends of the place taken as delivered to one recipient;
- a hold, which asks that the move the place waits on its caller for be held under its words for its bound;
- a drop;
- a priority, with its number;
- a done.

A land SHALL name no stage, no words and no other word of Flume's, and SHALL be written as the word `land` (core.53).

#### Scenario: An act of each sort

- **WHEN** an act of each sort, a land, a confirm and a hold among them, is written by `boomdrive-control`'s types and read back
- **THEN** each reads back equal, and the land's `what` is written as `"land"`, holding nothing more

## ADDED Requirements

### Requirement: A hold holds its words and its bound

A hold SHALL ask that the move its launch's place waits on its caller for be held under its words until its bound passes. It SHALL hold its `words`, as the control plane gave them, and its `bound_interval`, how long the hold holds in whole seconds, and nothing else, and SHALL be written as `{"hold": {"words": …, "bound_interval": …}}` (core.59; Flume's 130).

#### Scenario: A hold's form

- **WHEN** a hold under the words `the user sends it on the landings card, row 3` for 28800 seconds is written by `boomdrive-control`'s types and read back
- **THEN** it reads back equal, and its `what` is written as `{"hold": {"words": "the user sends it on the landings card, row 3", "bound_interval": 28800}}`
