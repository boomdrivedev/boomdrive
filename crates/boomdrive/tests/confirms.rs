//! A control plane's confirm of a reader's ends, carried out as Flume's
//! confirm of exactly the keys it names, as the recipient it names, each
//! key held to the launch's place, and refused as the runner or of no key
//! (core.56; the work-into-flume spec's "A confirm is Flume's confirm of
//! the keys it names, as the recipient it names", "Each key a confirm
//! names is the delivery of an end of its launch's place" and "A confirm
//! as the runner, or of no key, is refused"; design Decisions 2 to 4).

mod fakes;

use boomdrive::flume::{Act, Reply};
use boomdrive_control::fake::Fake;
use boomdrive_control::{ActSort, Id, Outcome, Want};
use fakes::{act, offer, place_of, Bench};

/// A delivery of launch `a`'s place.
const K1: &str = "change--a/0a1b2c3d/stage/code/1/delivered";
/// Another delivery of launch `a`'s place.
const K2: &str = "change--a/0a1b2c3d/submission/1/landed";
/// A delivery of launch `b`'s place.
const K3: &str = "change--b/4e5f6a7b/stage/code/1/delivered";

/// A confirm of `keys` as `recipient`.
fn confirm(recipient: &str, keys: &[&str]) -> ActSort {
    ActSort::Confirm {
        recipient: recipient.to_string(),
        deliveries: keys.iter().map(|key| key.to_string()).collect(),
    }
}

/// Flume's confirm of `keys` as `recipient`.
fn confirmed(recipient: &str, keys: &[&str]) -> Act {
    Act::Confirm {
        recipient: recipient.to_string(),
        keys: keys.iter().map(|key| key.to_string()).collect(),
    }
}

/// A bench whose first pass launched `a`, its place's ends those of
/// `deliveries`, and the control plane that offered it.
fn launched(deliveries: &[&str]) -> (Bench, Fake) {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    bench
        .flume
        .set(&place_of("a"), "ends", fakes::ends(deliveries));
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

/// The work-into-flume spec's "A confirm".
#[test]
fn a_confirm_names_only_the_keys_it_was_given() {
    let (bench, mut plane) = launched(&[K1, K2]);
    let p = place_of("a");
    let lines = pull(
        &bench,
        &mut plane,
        "a/confirm",
        confirm("scratch/planner", &[K1]),
    );
    assert_eq!(
        bench.flume.acts(),
        [(p.clone(), confirmed("scratch/planner", &[K1]))]
    );
    let acted = &bench.report().acts[&Id::new("a/confirm")];
    assert_eq!(acted.outcome, Outcome::Done);
    assert_eq!(acted.words, None);
    assert!(
        lines.contains(&format!(
            "a/confirm: confirm as scratch/planner of a at {p}: done"
        )),
        "{lines:?}"
    );
}

/// The work-into-flume spec's "A confirm Flume refuses".
#[test]
fn flumes_refusal_is_the_confirms_answer() {
    let (bench, mut plane) = launched(&[K1, K2]);
    let p = place_of("a");
    let words = format!(
        "the delivery key {K1} was not confirmed as scratch/planner: it is not owed to scratch/planner"
    );
    bench
        .flume
        .act_answers(&p, Ok(Reply::Refused(words.clone())));
    pull(
        &bench,
        &mut plane,
        "a/confirm",
        confirm("scratch/planner", &[K1]),
    );
    assert_eq!(
        bench.flume.acts(),
        [(p, confirmed("scratch/planner", &[K1]))]
    );
    let acted = &bench.report().acts[&Id::new("a/confirm")];
    assert_eq!(acted.outcome, Outcome::Refused);
    assert_eq!(acted.words, Some(words));
}
