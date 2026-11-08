## MODIFIED Requirements

### Requirement: An inbox's records are carried as references

Where weir takes an inbox's files, the runner SHALL carry the tip's commit, each record file under `inbox/` by its path with its text as the tip holds it, and each item's `brief.md` as `sha256:<hex>` of its bytes. It SHALL read nothing a capture's pointer names.

#### Scenario: An inbox carried

- **WHEN** an inbox is named on `inbox` in `example/crew`, and the forge's `inbox` branch holds a capture, a signal and an item with a brief written in weir's layout
- **THEN** the carried inbox holds the tip's commit, the capture's and signal's files with their text as the tip holds it, the item's record, and the brief as its digest
