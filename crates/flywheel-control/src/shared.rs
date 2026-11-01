//! The cases every implementation passes (the control-plane-interface
//! spec's "Every implementation passes the shared cases"), each a function
//! over a [`Harness`] that makes one implementation's control plane
//! holding given wants. The wants they give are those every
//! implementation can hold, as crew's requests can: ids the harness
//! gives, in order; leases numbered 1 with no expiry; launches of
//! [`REPOSITORY`] with no held stage and no agent setting; and acts on
//! those launches.

use crate::report::{Acted, Answer, At, Carried, Launched, Outcome, Report};
use crate::want::{Act, ActSort, Id, Launch, Lease, Offer, Want};
use crate::{Capacity, ControlPlane, Hello, HostPart, ToolVersion, INTERFACE_VERSION};
use std::collections::BTreeMap;
use std::fmt::Debug;

/// The repository every launch of the cases names, which the cases'
/// hello keeps.
pub const REPOSITORY: &str = "agentplot/flume";

/// What makes one implementation's control plane for the cases.
pub trait Harness {
    type Plane: ControlPlane;
    /// What of a control plane the harness can see: what it holds, or the
    /// commits its branches stand at.
    type Seen: PartialEq + Debug;

    /// The id the implementation gives the `n`th want it holds, counting
    /// from 1.
    fn id(&self, n: u32) -> Id;
    /// A control plane holding `wants`, in that order.
    fn plane(&mut self, wants: &[Want]) -> Self::Plane;
    /// What the harness can see of `plane` now.
    fn seen(&mut self, plane: &Self::Plane) -> Self::Seen;
}

/// A pull gives the wants in the control plane's order.
pub fn wants_in_order<H: Harness>(harness: &mut H) {
    let wants = vec![
        offer(harness, 1, "a", &[]),
        act(harness, 2, 1, ActSort::Priority(5)),
        offer(harness, 3, "b", &[1]),
        act(harness, 4, 3, ActSort::Drop),
    ];
    let mut plane = harness.plane(&wants);
    let report = Report::default();
    let pulled = plane.pull(&hello(&report), &report).expect("a pull");
    assert_eq!(pulled.wants, wants);
    assert!(pulled.terminals.is_empty());
    assert_eq!(pulled.report.unwrap_or_default(), Report::default());
}

/// A reported answer and carried status come back in the next pull's
/// report, though that pull is handed none, as after the runner's report
/// file is lost.
pub fn a_report_comes_back<H: Harness>(harness: &mut H) {
    let wants = vec![offer(harness, 1, "a", &[])];
    let mut plane = harness.plane(&wants);
    let none = Report::default();
    plane.pull(&hello(&none), &none).expect("a pull");
    let report = launched(harness.id(1));
    plane.report(&report).expect("a report");
    let pulled = plane.pull(&hello(&none), &none).expect("a pull");
    let back = pulled.report.expect("the report the control plane holds");
    let (sent, got) = (
        &report.launches[&harness.id(1)],
        &back.launches[&harness.id(1)],
    );
    assert_eq!(got.lease.number, sent.lease.number);
    assert_eq!(got.answer, sent.answer);
    let text_and_time = |launched: &Launched| launched.status.clone().map(|c| (c.text, c.at));
    assert_eq!(text_and_time(got), text_and_time(sent));
}

/// A second report of one report changes nothing the harness can see, and
/// nothing the next pull gives back.
pub fn a_report_twice<H: Harness>(harness: &mut H) {
    let wants = vec![offer(harness, 1, "a", &[])];
    let mut plane = harness.plane(&wants);
    let none = Report::default();
    plane.pull(&hello(&none), &none).expect("a pull");
    let report = launched(harness.id(1));
    let first = plane.report(&report).expect("a report");
    let seen = harness.seen(&plane);
    let given = plane.pull(&hello(&report), &report).expect("a pull").report;
    let second = plane.report(&report).expect("a report");
    assert_eq!(second, first);
    assert_eq!(harness.seen(&plane), seen);
    let again = plane.pull(&hello(&report), &report).expect("a pull").report;
    assert_eq!(again, given);
}

/// A want the report holds settled is still given: the runner, not the
/// control plane, answers it from its report.
pub fn a_settled_want_is_given<H: Harness>(harness: &mut H) {
    let wants = vec![
        offer(harness, 1, "a", &[]),
        act(harness, 2, 1, ActSort::Priority(5)),
    ];
    let mut plane = harness.plane(&wants);
    let none = Report::default();
    plane.pull(&hello(&none), &none).expect("a pull");
    let mut report = launched(harness.id(1));
    report.acts.insert(
        harness.id(2),
        Acted {
            launch: harness.id(1),
            lease: 1,
            outcome: Outcome::Done,
            words: None,
            at: at("2026-11-01T10:02:00Z"),
        },
    );
    plane.report(&report).expect("a report");
    let pulled = plane.pull(&hello(&report), &report).expect("a pull");
    assert_eq!(pulled.wants, wants);
}

/// The hello the cases hand every pull: a host keeping [`REPOSITORY`],
/// with room, holding `report`'s leases.
pub fn hello(report: &Report) -> Hello {
    Hello {
        interface_version: INTERFACE_VERSION,
        runner_version: "flywheel 0.0.0".to_string(),
        tool_versions: BTreeMap::from([(
            "flume".to_string(),
            ToolVersion::Printed("flume 0.4.1".to_string()),
        )]),
        repositories: vec![REPOSITORY.to_string()],
        capacity: Capacity::Printed(HostPart {
            running: "0".to_string(),
            bound: "4".to_string(),
            room: "4".to_string(),
            streams:
                r#"[{"name":"default","priority":0,"running":0,"bound":4,"room":4,"queue":[]}]"#
                    .to_string(),
        }),
        leases: report.leases(),
    }
}

/// The lease every want of the cases is given under.
fn lease() -> Lease {
    Lease {
        number: 1,
        expires: None,
    }
}

fn at(text: &str) -> At {
    At::parse(text).expect("a time")
}

/// The `n`th want: an offer of the launch `name`, after the wants
/// numbered `after`.
fn offer<H: Harness>(harness: &H, n: u32, name: &str, after: &[u32]) -> Want {
    Want::Offer(Offer {
        id: harness.id(n),
        lease: lease(),
        launch: Launch {
            repository: REPOSITORY.to_string(),
            kind: "change".to_string(),
            name: name.to_string(),
            base: Some("main".to_string()),
            brief: Some(format!("Build {name}.\nAs its design says.")),
            stream: Some("default".to_string()),
            priority: Some(2),
            intent: Some("build".to_string()),
            recipients: vec!["operator".to_string()],
            holds: vec![],
            hold_submission: false,
            after: after.iter().map(|n| harness.id(*n)).collect(),
            agent_settings: vec![],
        },
    })
}

/// The `n`th want: an act on the launch the `of`th want offers.
fn act<H: Harness>(harness: &H, n: u32, of: u32, what: ActSort) -> Want {
    Want::Act(Act {
        id: harness.id(n),
        launch: harness.id(of),
        lease: 1,
        what,
    })
}

/// A report answering launch `id` launched, with its place's status.
fn launched(id: Id) -> Report {
    let status = r#"{"status":{"places":[{"location":"/places/a","phase":"live"}]}}"#;
    let launched = Launched {
        lease: lease(),
        answer: Answer {
            outcome: Outcome::Launched,
            place_dir: Some("/places/a".to_string()),
            words: None,
            at: at("2026-11-01T10:00:00Z"),
        },
        status: Some(Carried {
            text: status.to_string(),
            at: at("2026-11-01T10:01:00Z"),
            listed: true,
        }),
        entry: None,
    };
    Report {
        launches: BTreeMap::from([(id, launched)]),
        ..Report::default()
    }
}
