//! The facts read from Flume's answers, over every one of Flume 0.4.1's
//! own answers in `tests/fixtures/flume-0.4.1` (Task notes 3.2). The
//! fixtures' README says what stood when each was read.

use boomdrive::facts::{self, Over, Pane, Room, StreamRoom};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/flume-0.4.1")
}

/// A fixture's text, as Flume printed it.
fn text(name: &str) -> String {
    fs::read_to_string(fixtures().join(name)).expect("the fixture")
}

/// A fixture, read as JSON.
fn answer(name: &str) -> Value {
    serde_json::from_str(&text(name)).expect("the fixture is JSON")
}

/// The scratch host's folder of places, as the launch's answer names it.
fn places_dir() -> String {
    let launch = answer("launch.json");
    let place = facts::launched(&launch)
        .and_then(facts::location)
        .expect("the launch names its place");
    place
        .strip_suffix("fix--parser")
        .expect("the place is fix--parser")
        .to_string()
}

/// The one place `name` holds.
fn only(name: &str) -> Value {
    let answer = answer(name);
    let places = facts::places(&answer);
    assert_eq!(places.len(), 1, "{name} holds {} places", places.len());
    places[0].clone()
}

/// Every fixture the tests here and in `flume.rs` read.
const READ: [&str; 20] = [
    "status.json",
    "status-bounded.json",
    "status-construct.json",
    "status-code.json",
    "status-answer.json",
    "status-on-hold.json",
    "status-delivered.json",
    "status-submitted.json",
    "status-landed-kept.json",
    "status-tidied.json",
    "status-gone.json",
    "status-live-agent.json",
    "launch.json",
    "launch-refused.json",
    "priority.json",
    "retry.json",
    "done.json",
    "drop.json",
    "retry-refused.json",
    "usage.stderr",
];

#[test]
fn every_fixture_is_read() {
    let held: BTreeSet<String> = fs::read_dir(fixtures())
        .expect("the fixtures")
        .map(|file| {
            file.expect("a fixture")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name != "README.md")
        .collect();
    let read: BTreeSet<String> = READ.iter().map(|name| name.to_string()).collect();
    assert_eq!(
        held, read,
        "a fixture no test reads, or one read and not held"
    );
    let readme = text("README.md");
    for name in READ {
        assert!(
            readme.contains(&format!("`{name}`")),
            "README says nothing of {name}"
        );
    }
}

#[test]
fn the_listing_of_every_place() {
    let listing = answer("status.json");
    let dir = places_dir();
    let held: Vec<&str> = facts::places(&listing)
        .iter()
        .map(|entry| facts::location(entry).expect("a location"))
        .collect();
    let named = |end: &str| format!("{dir}{end}");
    assert_eq!(
        held,
        [
            named("change--parse-words"),
            named("crew-planner--scratch"),
            named("fix--gate@1793559694"),
            named("fix--indent"),
            named("fix--lexer"),
            named("fix--parser@1793559507"),
            named("fix--tabs"),
        ]
    );
    let marks: Vec<&str> = facts::places(&listing)
        .iter()
        .map(|entry| facts::mark(entry).expect("a mark"))
        .collect();
    assert_eq!(
        marks,
        [
            "flywheel:scratch/3",
            "flywheel:scratch/planner/1",
            "flywheel:scratch/5",
            "flywheel:scratch/7",
            "flywheel:scratch/2",
            "flywheel:scratch/1",
            "flywheel:scratch/4",
        ]
    );
    let tabs = facts::marked(&listing, "flywheel:scratch/4").expect("fix--tabs by its mark");
    assert_eq!(facts::location(tabs), Some(named("fix--tabs").as_str()));
    assert_eq!(facts::marked(&listing, "flywheel:scratch/9"), None);
    assert_eq!(facts::marked(&listing, "scratch/4"), None);
}

#[test]
fn the_host_and_its_streams_room() {
    let listing = answer("status.json");
    assert_eq!(facts::host_room(&listing), Some(Room::Unbounded));
    assert_eq!(
        facts::streams(&listing),
        [
            StreamRoom {
                name: "wide".into(),
                room: Some(Room::Count(1)),
                queue_empty: Some(true)
            },
            StreamRoom {
                name: "default".into(),
                room: Some(Room::Unbounded),
                queue_empty: Some(true)
            },
            StreamRoom {
                name: "work".into(),
                room: Some(Room::Count(0)),
                queue_empty: Some(false)
            },
        ]
    );
    let bounded = answer("status-bounded.json");
    assert_eq!(facts::host_room(&bounded), Some(Room::Count(1)));
    let live = answer("status-live-agent.json");
    assert_eq!(facts::host_room(&live), Some(Room::Unbounded));
    assert_eq!(
        facts::streams(&live),
        [StreamRoom {
            name: "default".into(),
            room: Some(Room::Unbounded),
            queue_empty: Some(true)
        }]
    );
}

#[test]
fn the_host_part_as_flume_printed_it() {
    for name in [
        "status.json",
        "status-bounded.json",
        "status-construct.json",
        "status-code.json",
        "status-answer.json",
        "status-on-hold.json",
        "status-delivered.json",
        "status-submitted.json",
        "status-landed-kept.json",
        "status-tidied.json",
        "status-live-agent.json",
    ] {
        let printed = text(name);
        let part = facts::host_part(&printed).unwrap_or_else(|| panic!("{name}: no host part"));
        let host = &answer(name)["status"]["host"];
        for (field, sliced) in [
            ("running", &part.running),
            ("bound", &part.bound),
            ("room", &part.room),
            ("streams", &part.streams),
        ] {
            assert!(
                printed.contains(&format!("\"{field}\":{sliced}")),
                "{name}: {field} is not as Flume printed it: {sliced}"
            );
            let read: Value = serde_json::from_str(sliced).expect("JSON");
            assert_eq!(&read, &host[field], "{name}: {field}");
        }
        for word in [
            "pools",
            "accounts",
            "addresses",
            "bases",
            "terminal_kept",
            "stand-in",
        ] {
            for sliced in [&part.running, &part.bound, &part.room, &part.streams] {
                assert!(!sliced.contains(word), "{name}: {word} in {sliced}");
            }
        }
    }
    let part = facts::host_part(&text("status-bounded.json")).expect("the host part");
    assert_eq!(
        (
            part.running.as_str(),
            part.bound.as_str(),
            part.room.as_str()
        ),
        ("3", "4", "1")
    );
    assert!(part
        .streams
        .starts_with(r#"[{"name":"wide","priority":5,"running":2,"bound":3,"room":1,"queue":[]}"#));
    for name in [
        "status-gone.json",
        "launch.json",
        "launch-refused.json",
        "done.json",
    ] {
        assert_eq!(facts::host_part(&text(name)), None, "{name}");
    }
}

#[test]
fn a_place_at_each_phase() {
    let dir = places_dir();
    let construct = only("status-construct.json");
    let code = only("status-code.json");
    for (name, entry) in [("construct", &construct), ("code", &code)] {
        assert_eq!(
            facts::location(entry),
            Some(format!("{dir}change--parse-words").as_str()),
            "{name}"
        );
        assert_eq!(facts::mark(entry), Some("flywheel:scratch/3"), "{name}");
        assert!(facts::live_or_making(entry), "{name}");
        assert_eq!(facts::over(entry), None, "{name}");
        assert!(!facts::waits_delivered(entry), "{name}");
        assert_eq!(facts::submission_id(entry), None, "{name}");
        assert_eq!(
            facts::pane(entry),
            None,
            "{name}: the printed command shows no pane"
        );
    }
    assert_ne!(construct, code, "the place moved on");

    for name in ["status-answer.json", "status-on-hold.json"] {
        let entry = only(name);
        assert!(facts::live_or_making(&entry), "{name}");
        assert!(!facts::waits_delivered(&entry), "{name}");
        assert_eq!(facts::over(&entry), None, "{name}");
    }

    let delivered = only("status-delivered.json");
    assert_eq!(facts::mark(&delivered), Some("flywheel:scratch/planner/1"));
    assert!(facts::live_or_making(&delivered));
    assert!(facts::waits_delivered(&delivered));
    assert_eq!(facts::over(&delivered), None);

    let submitted = only("status-submitted.json");
    assert_eq!(facts::submission_id(&submitted), Some("3"));
    assert!(facts::live_or_making(&submitted));
    assert_eq!(facts::over(&submitted), None);

    let kept = only("status-landed-kept.json");
    assert_eq!(
        facts::over(&kept),
        Some(Over::Landed(Some(
            "fa2b0243a22184c264b5e5e4ed02e96f2c77b66b".into()
        )))
    );
    assert!(!facts::live_or_making(&kept));
    assert_eq!(facts::submission_id(&kept), Some("1"));
    assert_eq!(
        facts::location(&kept),
        Some(format!("{dir}fix--parser").as_str())
    );

    let tidied = only("status-tidied.json");
    assert_eq!(
        facts::over(&tidied),
        Some(Over::Landed(Some(
            "6723d38346b52a7642046f43cadce3eccdeb4bc0".into()
        )))
    );
    assert!(!facts::live_or_making(&tidied));
    assert_eq!(facts::submission_id(&tidied), None);
    assert_eq!(
        facts::location(&tidied),
        Some(format!("{dir}fix--gate@1793559694").as_str())
    );
}

#[test]
fn a_live_place_shows_its_agents_pane() {
    let entry = only("status-live-agent.json");
    assert!(facts::live_or_making(&entry));
    assert_eq!(facts::mark(&entry), None);
    assert_eq!(
        facts::pane(&entry),
        Some(Pane {
            id: "w1:p1".into(),
            socket_file: "/private/tmp/scratch/host/home/config/herdr/sessions/flume/herdr.sock"
                .into()
        })
    );
}

#[test]
fn a_places_entry_sliced_from_the_listing() {
    let dir = places_dir();
    let listing = answer("status.json");
    let bounded = answer("status-bounded.json");
    for name in [
        "status-answer.json",
        "status-delivered.json",
        "status-code.json",
        "status-tidied.json",
    ] {
        let one = answer(name);
        let location = facts::location(&facts::places(&one)[0])
            .expect("a location")
            .to_string();
        let asked = location.split('@').next().expect("a location").to_string();
        let carried =
            facts::entry(&one, &asked).unwrap_or_else(|| panic!("{name}: no entry for {asked}"));
        let listed = facts::entry(&listing, &asked)
            .unwrap_or_else(|| panic!("status.json: no entry for {asked}"));
        // A place unchanged since it was asked for is the same entry in
        // the listing; one that moved on is not (design Decision 12).
        assert_eq!(listed == carried, name != "status-code.json", "{name}");
    }
    // Two listings whose host part alone differs hold the same entries.
    assert_ne!(listing["status"]["host"], bounded["status"]["host"]);
    for entry in facts::places(&listing) {
        let location = facts::location(entry).expect("a location");
        assert_eq!(facts::entry(&bounded, location), Some(entry), "{location}");
    }
    // A tidied place is found by the location it was launched at.
    let tidied = facts::entry(&listing, &format!("{dir}fix--parser")).expect("the tidied record");
    assert_eq!(
        facts::location(tidied),
        Some(format!("{dir}fix--parser@1793559507").as_str())
    );
    assert_eq!(
        facts::entry(&listing, &format!("{dir}fix--slow-gate")),
        None
    );
    assert_eq!(facts::entry(&listing, &format!("{dir}fix")), None);
}

#[test]
fn the_place_a_launch_names() {
    let dir = places_dir();
    let launch = answer("launch.json");
    let place = facts::launched(&launch).expect("the place launched");
    assert_eq!(
        facts::location(place),
        Some(format!("{dir}fix--parser").as_str())
    );
    assert_eq!(facts::mark(place), Some("flywheel:scratch/1"));
    assert!(facts::live_or_making(place));
    assert_eq!(facts::places(&launch), &[] as &[Value]);
}

#[test]
fn a_refusal_and_an_acts_answer_hold_no_place() {
    for name in [
        "status-gone.json",
        "launch-refused.json",
        "retry-refused.json",
        "priority.json",
        "retry.json",
        "done.json",
        "drop.json",
    ] {
        let answer = answer(name);
        assert_eq!(facts::places(&answer), &[] as &[Value], "{name}");
        assert_eq!(facts::launched(&answer), None, "{name}");
        assert_eq!(facts::host_room(&answer), None, "{name}");
        assert_eq!(facts::streams(&answer), [], "{name}");
    }
}
