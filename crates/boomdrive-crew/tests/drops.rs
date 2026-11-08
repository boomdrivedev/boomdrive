//! A launch a drop names, offered and claimed only where the runner
//! launched it, let go where the runner holds it unlaunched, and its drop
//! given to the runner that holds it whatever the plan holds (the change
//! a-dropped-launch-is-never-offered, design Decisions 1 to 6). Each test
//! runs over one bare state repository, `deneb` and `vega` its hosts.

mod common;

use boomdrive_control::{Answer, At, Carried, ControlPlane, Hello, Id, Launched, Outcome, Report};
use boomdrive_crew::read::LEASE;
use common::{act, file, hello, ids, launch, plan, pull, State, REQUESTS};

const CREW: &str = "afterthought/crew";
const PLACE: &str = "/places/change--a";

fn kept() -> Hello {
    hello(&[CREW], "4")
}

/// `madswan/main` holding `requests`, its plan holding `units`.
fn main_holding(state: &State, requests: &[String], units: &[&str]) {
    state.main(&[
        ("requests.rec", &file(REQUESTS, requests)),
        ("plan.rec", &plan(units, &[])),
    ]);
}

fn at(text: &str) -> At {
    At::parse(text).unwrap()
}

/// A launch answered `outcome` at `place` with `words`, its place's
/// status carried and listed or not.
fn answered(
    outcome: Outcome,
    place: Option<&str>,
    words: Option<&str>,
    listed: Option<bool>,
) -> Launched {
    Launched {
        lease: LEASE,
        answer: Answer {
            outcome,
            place_dir: place.map(str::to_string),
            words: words.map(str::to_string),
            at: at("2026-11-01T10:00:00Z"),
        },
        status: listed.map(|listed| Carried {
            text: format!(
                r#"{{"status":{{"places":[{{"place":"{}","phase":"live"}}],"host":{{}}}}}}"#,
                place.unwrap_or_default()
            ),
            at: at("2026-11-01T10:01:00Z"),
            listed,
        }),
        entry: None,
    }
}

/// The runner's report answering launch `n` as `launched` has it.
fn report_of(n: u64, launched: Launched) -> Report {
    let mut report = Report::default();
    report
        .launches
        .insert(Id::new(format!("madswan/{n}")), launched);
    report
}

/// Launch `n` held on its After, as the runner answers it.
fn held(n: u64) -> Report {
    report_of(
        n,
        answered(
            Outcome::Held,
            None,
            Some("held on its After madswan/11"),
            None,
        ),
    )
}

/// The crew-control-plane spec's "A drop naming another host": a drop
/// counts whatever host it names, so the launch it names is given no
/// offer, and the drop, naming another host, is given nothing.
#[test]
fn a_drop_naming_another_host() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(12, Some("deneb"), "unit/u", CREW, "u", &[]),
            act(14, "drop", Some("vega"), "unit/u", 12, &[]),
        ],
        &["u"],
    );
    let (mut deneb, _) = state.crew("deneb");
    let pulled = deneb.pull(&kept(), &held(12)).expect("a pull");
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
}

/// "A held launch dropped": the drop, naming this host, is given while
/// the plan holds the launch's unit, and the launch it names is not.
#[test]
fn a_held_launch_dropped() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(12, Some("deneb"), "unit/u", CREW, "u", &[]),
            act(14, "drop", Some("deneb"), "unit/u", 12, &[]),
        ],
        &["u"],
    );
    let (mut deneb, _) = state.crew("deneb");
    let pulled = deneb.pull(&kept(), &held(12)).expect("a pull");
    assert_eq!(ids(&pulled), ["madswan/14"]);
}

/// "A dropped launch the runner launched": it is given still, so its
/// place is followed to its end, and its drop with it.
#[test]
fn a_dropped_launch_the_runner_launched() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(12, Some("deneb"), "unit/u", CREW, "u", &[]),
            act(14, "drop", Some("deneb"), "unit/u", 12, &[]),
        ],
        &["u"],
    );
    let (mut deneb, _) = state.crew("deneb");
    let report = report_of(
        12,
        answered(Outcome::Launched, Some(PLACE), None, Some(true)),
    );
    let pulled = deneb.pull(&kept(), &report).expect("a pull");
    assert_eq!(ids(&pulled), ["madswan/12", "madswan/14"]);
}

/// "A kit's launch dropped before it is claimed": no runner answers it,
/// so it is claimed by no one, no branch is written, and neither it nor
/// its drop is given.
#[test]
fn a_kits_launch_dropped_before_it_is_claimed() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(20, None, "unit/u", CREW, "u", &[]),
            act(21, "drop", None, "unit/u", 20, &[]),
        ],
        &["u"],
    );
    let (mut deneb, _) = state.crew("deneb");
    let pulled = pull(&mut deneb, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert_eq!(state.tip("madswan/runner/deneb"), None);
}
