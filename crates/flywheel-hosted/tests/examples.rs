//! Version 1's examples, each a whole message under
//! `examples/from-runner/` or `examples/to-runner/`, pass their
//! direction's envelope (design Decisions 2 and 14).

mod common;

use common::{example, examples, from_runner, passes, to_runner};

#[test]
fn every_example_passes_its_directions_schema() {
    let directions = [("from-runner", from_runner()), ("to-runner", to_runner())];
    for (direction, validator) in directions {
        let examples = examples(direction);
        assert!(!examples.is_empty(), "no example {direction}");
        for (name, value) in examples {
            passes(&validator, &value, &format!("{direction}/{name}"));
        }
    }
}

/// The runner-protocol spec's "A message refused": either end's
/// `message_refused` names a message by its id, with words.
#[test]
fn a_message_refused() {
    let from = example("from-runner", "message_refused.json");
    passes(&from_runner(), &from, "a runner's message_refused");
    let to = example("to-runner", "message_refused.json");
    passes(&to_runner(), &to, "a control plane's message_refused");
    for refusal in [&from, &to] {
        let body = &refusal["body"]["message_refused"];
        assert!(body["message"].is_string() && body["words"].is_string());
    }
}
