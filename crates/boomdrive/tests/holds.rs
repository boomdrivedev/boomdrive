//! A control plane's hold of the move its launch's place waits on it for,
//! carried out as Flume's hold of that waiting move with exactly the words
//! and the bound it names, its answer Flume's (core.59; the work-into-flume
//! spec's "A hold is Flume's hold of the move its launch's place waits on";
//! design Decisions 2 and 3).

mod fakes;

use boomdrive::flume::{Act, Reply};
use boomdrive_control::fake::Fake;
use boomdrive_control::{ActSort, Id, Outcome, Want};
use fakes::{act, offer, place_of, Bench};

/// Neutral words saying who makes the move and how.
const WORDS: &str = "the user sends it on the landings card, row 3";

/// Eight hours, in whole seconds.
const BOUND: u64 = 28800;

/// A hold under `words` for `bound_interval` seconds.
fn hold(words: &str, bound_interval: u64) -> ActSort {
    ActSort::Hold {
        words: words.to_string(),
        bound_interval,
    }
}

/// A bench whose first pass launched `a`, and the control plane that
/// offered it.
fn launched() -> (Bench, Fake) {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    (bench, plane)
}

/// The control plane's next pull: launch `a` again, and `what` of it as
/// act `id`.
fn pull(bench: &Bench, plane: &mut Fake, id: &str, what: ActSort) -> Vec<String> {
    plane.set_wants(vec![Want::Offer(offer("a", "a")), act(id, "a", what)]);
    bench.flume.take_calls();
    let (_, lines) = bench.pass(plane);
    lines
}

/// The work-into-flume spec's "A hold".
#[test]
fn a_hold_is_flumes_hold_of_the_waiting_move() {
    let (bench, mut plane) = launched();
    let p = place_of("a");
    let lines = pull(&bench, &mut plane, "a/hold", hold(WORDS, BOUND));
    assert_eq!(
        bench.flume.acts(),
        [(
            p.clone(),
            Act::Hold {
                words: WORDS.to_string(),
                bound_interval: BOUND,
            }
        )]
    );
    let acted = &bench.report().acts[&Id::new("a/hold")];
    assert_eq!(acted.outcome, Outcome::Done);
    assert_eq!(acted.words, None);
    assert!(
        lines.contains(&format!("a/hold: hold for 28800s of a at {p}: done")),
        "{lines:?}"
    );
    // The words stay out of the lines, as a retry's do (core.11).
    assert!(lines.iter().all(|line| !line.contains(WORDS)), "{lines:?}");
}

/// The work-into-flume spec's "A hold Flume refuses".
#[test]
fn flumes_refusal_is_the_holds_answer() {
    let (bench, mut plane) = launched();
    let p = place_of("a");
    let words = format!("the place {p} waits on no move of its caller's");
    bench
        .flume
        .act_answers(&p, Ok(Reply::Refused(words.clone())));
    pull(&bench, &mut plane, "a/hold", hold(WORDS, BOUND));
    assert_eq!(
        bench.flume.acts(),
        [(
            p,
            Act::Hold {
                words: WORDS.to_string(),
                bound_interval: BOUND,
            }
        )]
    );
    let acted = &bench.report().acts[&Id::new("a/hold")];
    assert_eq!(acted.outcome, Outcome::Refused);
    assert_eq!(acted.words, Some(words));
}
