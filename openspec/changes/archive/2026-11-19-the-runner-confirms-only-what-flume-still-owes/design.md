# Design

## Decision 1: Flume's listing is read again after the send, only where something is to be confirmed

The status a control plane took names which ends it holds; Flume's listing names which it still owes. A confirmation needs both, so the runner confirms their intersection. The listing the pass read before its pull, or again once the wants' commands ran, can be minutes old by the time the report is taken, so the runner reads `flume --json status` once more at the confirm. It reads it only where some launch has a key to confirm, so a pass with nothing owed runs no more commands than before.

Not taken: confirming and ignoring Flume's refusal. The refusal is said and the confirmation tried again at every later pass while the carried status is listed, which is the noise of 2026-11-19. Not taken: dropping a key once Flume refuses it. It would turn Flume's words into a state of the runner's (core.1).
