## ADDED Requirements

### Requirement: A launch Flume refuses goes up refused

Where Flume refuses a launch that a hosted control plane offers, the part's next report SHALL hold that launch under its own id. Its answer SHALL have the outcome `refused`, Flume's words as Flume gave them, and no place (core.1, core.11, core.27).

#### Scenario: A launch Flume refuses goes up refused

- **WHEN** `home`'s control plane offers the launch `l-1`, and the stub `flume` refuses it with its words
- **THEN** the report that the pass sends holds `l-1` with the outcome `refused`, Flume's words and no place, and the report passes version 5
