## ADDED Requirements

### Requirement: An act asks one thing of its launch's place

An act SHALL ask one of these of its launch's place:

- a retry, with its stage and its words where it names them, and whether it names the submission;
- a land, which asks for the landing of its launch's work, and holds nothing but its launch;
- a drop;
- a priority, with its number;
- a done.

A land SHALL name no stage, no words and no other word of Flume's, and SHALL be written as the word `land` (core.53).

#### Scenario: An act of each sort

- **WHEN** an act of each sort, a land among them, is written by `boomdrive-control`'s types and read back
- **THEN** each reads back equal, and the land's `what` is written as `"land"`, holding nothing more
