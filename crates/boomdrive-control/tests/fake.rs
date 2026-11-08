//! The fake control plane (design Decision 21): the control-plane-interface
//! spec's scenarios that are its own, and the shared cases run over it
//! ("The fake control plane passes").

use boomdrive_control::fake::{Fake, Sent};
use boomdrive_control::shared::{self, Harness};
use boomdrive_control::*;
use std::collections::BTreeMap;
use std::time::Duration;

/// The shared cases' harness over [`Fake`], seeing the report it holds.
struct Fakes;

impl Harness for Fakes {
    type Plane = Fake;
    type Seen = Option<Report>;

    fn id(&self, n: u32) -> Id {
        Id::new(format!("fake/{n}"))
    }

    fn plane(&mut self, wants: &[Want]) -> Fake {
        Fake::holding(wants.to_vec())
    }

    fn seen(&mut self, plane: &Fake) -> Option<Report> {
        plane.held().cloned()
    }
}

#[test]
fn shared_wants_in_order() {
    shared::wants_in_order(&mut Fakes);
}

#[test]
fn shared_a_report_comes_back() {
    shared::a_report_comes_back(&mut Fakes);
}

#[test]
fn shared_a_report_twice() {
    shared::a_report_twice(&mut Fakes);
}

#[test]
fn shared_a_settled_want_is_given() {
    shared::a_settled_want_is_given(&mut Fakes);
}

fn at(text: &str) -> At {
    At::parse(text).unwrap()
}

fn launch(name: &str) -> Launch {
    Launch {
        repository: "agentplot/flume".to_string(),
        kind: "change".to_string(),
        name: name.to_string(),
        base: None,
        brief: None,
        stream: None,
        priority: None,
        intent: None,
        recipients: vec![],
        holds: vec![],
        hold_submission: false,
        after: vec![],
        agent_settings: vec![],
    }
}

fn offer(id: &str, number: u64, launch: Launch) -> Want {
    Want::Offer(Offer {
        id: Id::new(id),
        lease: Lease {
            number,
            expires: Some(at("2026-11-01T11:00:00Z")),
        },
        launch,
    })
}

/// A report answering each of `ids` launched.
fn launched(ids: &[&str]) -> Report {
    let launched = |id: &str| Launched {
        lease: Lease {
            number: 1,
            expires: None,
        },
        repository: None,
        answer: Answer {
            outcome: Outcome::Launched,
            place_dir: Some(format!("/places/{id}")),
            words: None,
            at: at("2026-11-01T10:00:00Z"),
        },
        status: None,
        entry: None,
    };
    Report {
        launches: ids.iter().map(|id| (Id::new(*id), launched(id))).collect(),
        ..Report::default()
    }
}

fn pull(fake: &mut Fake) -> Result<Pulled, Unread> {
    let report = Report::default();
    fake.pull(&shared::hello(&report), &report)
}

#[test]
fn wants_in_order() {
    let wants = vec![
        offer("a", 1, launch("a")),
        Want::Act(Act {
            id: Id::new("p"),
            launch: Id::new("a"),
            lease: 1,
            what: ActSort::Priority(3),
        }),
        Want::Command(Command {
            id: Id::new("c"),
            lease: Lease {
                number: 1,
                expires: None,
            },
            what: CommandSort::Branch {
                repository: "agentplot/flume".to_string(),
                branch: "bolt/x".to_string(),
                from: "main".to_string(),
            },
        }),
    ];
    let mut fake = Fake::holding(wants.clone());
    assert_eq!(pull(&mut fake).unwrap().wants, wants);
    assert_eq!(pull(&mut fake).unwrap().wants, wants);
}

#[test]
fn a_pull_that_cannot_be_read() {
    let mut fake = Fake::holding(vec![offer("a", 1, launch("a"))]);
    fake.refuse_pulls("no network");
    assert_eq!(
        pull(&mut fake).unwrap_err(),
        Unread("no network".to_string())
    );
    assert!(fake.hellos().is_empty());
    fake.answer_pulls();
    assert_eq!(pull(&mut fake).unwrap().wants.len(), 1);
    assert_eq!(fake.hellos(), [shared::hello(&Report::default())]);
}

#[test]
fn an_offer_carries_its_id_and_lease() {
    let mut fake = Fake::holding(vec![offer("madswan/7", 3, launch("a"))]);
    let wants = pull(&mut fake).unwrap().wants;
    let Want::Offer(given) = &wants[0] else {
        panic!("an offer: {wants:?}");
    };
    assert_eq!(given.id, Id::new("madswan/7"));
    assert_eq!(
        given.lease,
        Lease {
            number: 3,
            expires: Some(at("2026-11-01T11:00:00Z")),
        }
    );
}

#[test]
fn an_offer_with_every_field() {
    let every = Launch {
        kind: "crew/crew-planner".to_string(),
        base: Some("bolt/x".to_string()),
        brief: Some("Plan.\n".to_string()),
        stream: Some("roles".to_string()),
        priority: Some(4),
        intent: Some("plan".to_string()),
        recipients: vec!["role/planner".to_string()],
        holds: vec![Hold {
            stage: "construct".to_string(),
            words: "wait".to_string(),
        }],
        hold_submission: true,
        after: vec![Id::new("madswan/1")],
        agent_settings: vec![AgentSetting {
            field: "model".to_string(),
            value: "opus".to_string(),
        }],
        ..launch("planner")
    };
    let want = offer("madswan/2", 1, every.clone());
    let mut fake = Fake::holding(vec![want.clone()]);
    let wants = pull(&mut fake).unwrap().wants;
    assert_eq!(wants, [want]);
    let Want::Offer(given) = &wants[0] else {
        panic!("an offer: {wants:?}");
    };
    assert_eq!(given.launch, every);
    assert_eq!(given.launch.kind, "crew/crew-planner");
    let report = launched(&["madswan/2"]);
    fake.report(&report).unwrap();
    let held = pull(&mut fake).unwrap().report.unwrap();
    let text = serde_json::to_string(&held).unwrap();
    assert_eq!(serde_json::from_str::<Report>(&text).unwrap(), report);
}

#[test]
fn a_report_sent_twice() {
    let mut fake = Fake::new();
    let report = launched(&["a"]);
    let first = fake.report(&report).unwrap();
    let held = fake.held().cloned();
    let second = fake.report(&report).unwrap();
    assert_eq!((second, fake.held().cloned()), (first, held));
    assert_eq!(fake.held(), Some(&report));
    assert_eq!(fake.reports(), [report.clone(), report]);
}

#[test]
fn a_report_not_sent() {
    let mut fake = Fake::new();
    fake.refuse_reports("the socket is closed");
    let report = launched(&["a"]);
    assert_eq!(
        fake.report(&report).unwrap_err(),
        Unsent("the socket is closed".to_string())
    );
    assert!(fake.reports().is_empty());
    assert_eq!(pull(&mut fake).unwrap().report, None);
    fake.take_reports();
    fake.report(&report).unwrap();
    assert_eq!(pull(&mut fake).unwrap().report, Some(report));
}

#[test]
fn a_launch_let_go() {
    let mut fake = Fake::holding(vec![offer("a", 1, launch("a")), offer("b", 1, launch("b"))]);
    fake.let_go(Id::new("a"));
    let reported = fake.report(&launched(&["a", "b"])).unwrap();
    assert_eq!(reported.let_go, [Id::new("a")]);
    assert!(reported.refused.is_empty());
    let pulled = pull(&mut fake).unwrap();
    assert_eq!(pulled.report, Some(launched(&["b"])));
    assert_eq!(pulled.wants, [offer("b", 1, launch("b"))]);
}

#[test]
fn a_launch_refused() {
    let mut fake = Fake::new();
    fake.refuse(Id::new("a"), "lease 1 is replaced by 2");
    fake.let_go(Id::new("c"));
    let reported = fake.report(&launched(&["a", "b"])).unwrap();
    assert_eq!(
        reported.refused,
        BTreeMap::from([(Id::new("a"), "lease 1 is replaced by 2".to_string())])
    );
    assert!(reported.let_go.is_empty(), "c is in no report");
    assert_eq!(fake.held(), Some(&launched(&["b"])));
    assert_eq!(fake.reports(), [launched(&["a", "b"])]);
}

#[test]
fn a_terminal_is_given_once_and_its_link_is_the_tests() {
    let mut fake = Fake::new();
    let far = fake.open_terminal(Id::new("t"), Id::new("a"), 3);
    let mut terminals = pull(&mut fake).unwrap().terminals;
    assert!(pull(&mut fake).unwrap().terminals.is_empty());
    assert_eq!(terminals.len(), 1);
    let Terminal {
        id,
        launch,
        lease,
        mut link,
    } = terminals.remove(0);
    assert_eq!((id, launch, lease), (Id::new("t"), Id::new("a"), 3));

    let wait = Duration::from_millis(10);
    assert_eq!(link.input(wait), None);
    far.input.send(Input::Text("ls\n".to_string())).unwrap();
    far.input
        .send(Input::Keys(vec!["Enter".to_string()]))
        .unwrap();
    assert_eq!(link.input(wait), Some(Input::Text("ls\n".to_string())));
    assert_eq!(
        link.input(wait),
        Some(Input::Keys(vec!["Enter".to_string()]))
    );
    assert!(link.screen("$ ls"));
    assert_eq!(far.sent.recv().unwrap(), Sent::Screen("$ ls".to_string()));

    drop(far.input);
    assert_eq!(link.input(wait), Some(Input::Close));
    link.close("the launch is let go");
    assert_eq!(
        far.sent.recv().unwrap(),
        Sent::Closed("the launch is let go".to_string())
    );
}

#[test]
fn a_terminals_far_end_stops_listening() {
    let mut fake = Fake::new();
    let far = fake.open_terminal(Id::new("t"), Id::new("a"), 1);
    let mut link = pull(&mut fake).unwrap().terminals.remove(0).link;
    drop(far.sent);
    assert!(!link.screen("$"));
    link.close("gone");
}
