## MODIFIED Requirements

### Requirement: An act asks one thing of its launch's place

An act SHALL ask one of these of its launch's place:

- a retry, with its stage and its words where it names them, and whether it names the submission;
- a land, which asks for the landing of its launch's work, and holds nothing but its launch;
- a confirm, which asks ends of the place taken as delivered to one recipient;
- a drop;
- a priority, with its number;
- a done.

A land SHALL name no stage, no words and no other word of Flume's, and SHALL be written as the word `land` (core.53).

#### Scenario: An act of each sort

- **WHEN** an act of each sort, a land and a confirm among them, is written by `boomdrive-control`'s types and read back
- **THEN** each reads back equal, and the land's `what` is written as `"land"`, holding nothing more

## ADDED Requirements

### Requirement: A confirm holds the one recipient it confirms as and each delivery key

A confirm SHALL ask that each end of its launch's place it names by its delivery's key be taken as delivered to the one recipient it names. It SHALL hold its `recipient` and its `deliveries`, each a delivery's key as Flume gave it, in the control plane's order, and nothing else, and SHALL be written as `{"confirm": {"recipient": …, "deliveries": […]}}` (core.56).

#### Scenario: A confirm's form

- **WHEN** a confirm of two keys as `scratch/planner` is written by `boomdrive-control`'s types and read back
- **THEN** it reads back equal, and its `what` is written as `{"confirm": {"recipient": "scratch/planner", "deliveries": ["change--a/0a1b2c3d/stage/code/1/delivered", "change--a/0a1b2c3d/submission/1/landed"]}}`, its keys in the order it was given them
