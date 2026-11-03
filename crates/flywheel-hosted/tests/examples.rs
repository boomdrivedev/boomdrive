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

/// The runner-protocol spec's "A terminal's messages": an example of each
/// terminal kind, and of a frame with and without a redraw, passes its
/// direction's envelope (design Decision 10).
#[test]
fn a_terminals_messages() {
    let from = from_runner();
    for name in [
        "terminal_opened.json",
        "terminal_output.json",
        "terminal_output-bytes-as-they-come.json",
        "terminal_output-a-redraw-after-a-resume.json",
        "terminal_closed.json",
    ] {
        passes(&from, &example("from-runner", name), name);
    }
    let to = to_runner();
    for name in [
        "terminal_open.json",
        "terminal_input.json",
        "terminal_resume.json",
        "terminal_close.json",
    ] {
        passes(&to, &example("to-runner", name), name);
    }
    let redraws: Vec<bool> = [
        "terminal_output-bytes-as-they-come.json",
        "terminal_output-a-redraw-after-a-resume.json",
    ]
    .iter()
    .map(|name| example("from-runner", name)["body"]["terminal_output"]["redraw"] == true)
    .collect();
    assert_eq!(redraws, [false, true], "a frame without and with a redraw");
}
