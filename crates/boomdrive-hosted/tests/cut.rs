//! What goes up is version 2's copy (the hosted-control-plane spec's
//! "What goes up is version 2's copy, checked before it goes", its
//! scenario "Flume's places cut"; design Decisions 9 and 14 of
//! the-runner-dials-the-hosted-control-plane, and Decisions 2 and 9 of
//! the-hosted-protocol-carries-every-end-of-a-place): each place of Flume
//! 0.21.0's and Flume 0.4.1's fixtures, each entry of sluice 0.61.1's,
//! each preview and release of sluice 0.74.3's (Decision 4 of
//! the-hosted-protocol-carries-land-and-preview) and Flume's host part,
//! cut by walking version 2's schema, passes its definition and holds
//! nothing version 2 leaves out. Each end of Flume
//! 0.21.0's goes up in a form version 2 names, and each of Flume 0.4.1's,
//! which has no key, is left out. The fixtures are the binary's, read
//! where they are kept.

mod common;

use boomdrive_control::HostPart;
use boomdrive_hosted::copy;
use common::{definition, errors, validator};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

/// The fields of a place's status no copy holds (the runner-protocol
/// spec's "A place's status is carried as its copy").
const LEFT_OUT: &[&str] = &["last_words", "screen", "printed", "readiness", "repository"];

/// The directory of one tool's fixtures, `flume-0.21.0`, `flume-0.4.1`,
/// `sluice-0.61.1` or `sluice-0.74.3`.
fn fixtures(tool: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../boomdrive/tests/fixtures")
        .join(tool)
}

/// Each fixture of `tool` whose name starts with `status`, by its name,
/// with its text as the tool printed it.
fn statuses(tool: &str) -> Vec<(String, String)> {
    let dir = fixtures(tool);
    let mut found: Vec<(String, String)> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            let name = path.file_name().unwrap().to_string_lossy();
            name.starts_with("status") && name.ends_with(".json")
        })
        .map(|path| {
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            (name, std::fs::read_to_string(&path).unwrap())
        })
        .collect();
    found.sort();
    found
}

/// Each place's status as `flume --json status <place>` gives it, of the
/// fixtures of `flume`, `flume-0.21.0` or `flume-0.4.1`, by the fixture
/// and the place's name: a fixture of one place as Flume printed it, and
/// each place of a fixture of many as the answer of one.
fn places(flume: &str) -> Vec<(String, String)> {
    let mut places = Vec::new();
    for (name, text) in statuses(flume) {
        let answer: Value = serde_json::from_str(&text).unwrap();
        let Some(each) = answer.pointer("/status/places").and_then(Value::as_array) else {
            continue;
        };
        if each.len() == 1 {
            places.push((name, text));
            continue;
        }
        for place in each {
            let one = json!({ "status": { "places": [place] } }).to_string();
            places.push((format!("{name}, {}", place["name"]), one));
        }
    }
    places
}

/// Each path of `value` at which a key of `names` is held.
fn held(value: &Value, names: &[&str], at: &str, found: &mut Vec<String>) {
    match value {
        Value::Object(fields) => {
            for (key, field) in fields {
                let path = format!("{at}/{key}");
                if names.contains(&key.as_str()) {
                    found.push(path.clone());
                }
                held(field, names, &path, found);
            }
        }
        Value::Array(items) => {
            for (n, item) in items.iter().enumerate() {
                held(item, names, &format!("{at}/{n}"), found);
            }
        }
        _ => {}
    }
}

/// The copy of the place `name`, `text` as Flume printed it, which
/// passes version 2's `place_status` and holds nothing version 2 leaves
/// out, with Flume's ends it carried by their delivery alone and left out.
fn cut_whole(name: &str, text: &str) -> copy::PlaceCopy {
    let validator = validator(&definition("place-status.schema.json", "place_status"));
    let placed = copy::place_status(text).unwrap_or_else(|words| panic!("{name}: {words}"));
    let copy = &placed.copy;
    let refused = errors(&validator, copy);
    assert!(refused.is_empty(), "{name}'s copy is refused: {refused:#?}");
    let mut found = Vec::new();
    held(copy, LEFT_OUT, "", &mut found);
    for task in copy["tasks"].as_array().into_iter().flatten() {
        if task.get("text").is_some() {
            found.push("/tasks/*/text".to_string());
        }
    }
    assert!(found.is_empty(), "{name}'s copy holds {found:?}");
    placed
}

/// Flume's ends of the place `text`, as it printed them.
fn flumes_ends(text: &str) -> Vec<Value> {
    let printed: Value = serde_json::from_str(text).unwrap();
    printed["status"]["places"][0]["ends"]
        .as_array()
        .cloned()
        .unwrap_or_default()
}

#[test]
fn flumes_places_cut() {
    let newest = places("flume-0.21.0");
    assert_eq!(
        newest.len(),
        2,
        "Flume 0.21.0's fixtures hold {}",
        newest.len()
    );
    let mut overs = Vec::new();
    for (name, text) in newest {
        let placed = cut_whole(&name, &text);
        assert!(
            placed.whole(),
            "{name}: every end in a form version 2 names"
        );
        let printed = flumes_ends(&text);
        let ends = placed.copy["ends"].as_array().expect("ends");
        assert_eq!(ends.len(), printed.len(), "{name}: every end goes up");
        for (end, flumes) in ends.iter().zip(&printed) {
            assert!(end.get("deliveries").is_some(), "{name}: a named form");
            assert_eq!(end["key"], flumes["key"], "{name}: the end's key");
            assert!(end["delivery"].is_string(), "{name}: the delivery's key");
            assert_eq!(
                end["delivery"], flumes["delivery"],
                "{name}: as Flume gave it"
            );
            if end["of"] == "over" {
                overs.push(end["over"].as_str().expect("its word").to_string());
            }
        }
    }
    overs.sort();
    assert_eq!(overs, ["done", "dropped"], "a drop's and a done's end");

    let oldest = places("flume-0.4.1");
    assert!(
        oldest.len() >= 15,
        "Flume 0.4.1's fixtures hold {}",
        oldest.len()
    );
    for (name, text) in oldest {
        let placed = cut_whole(&name, &text);
        let printed = flumes_ends(&text);
        assert!(placed.by_delivery.is_empty(), "{name}: {placed:?}");
        assert_eq!(
            placed.left_out,
            (0..printed.len()).collect::<Vec<_>>(),
            "{name}: each end, which has no key, left out"
        );
        assert_eq!(placed.copy["ends"], json!([]), "{name}");
    }
}

#[test]
fn a_status_cut_keeps_each_value_as_flume_printed_it() {
    let text = std::fs::read_to_string(fixtures("flume-0.4.1").join("status-code.json")).unwrap();
    let printed: Value = serde_json::from_str(&text).unwrap();
    let place = &printed["status"]["places"][0];
    let copy = copy::place_status(&text).expect("a copy").copy;
    for (key, value) in copy.as_object().unwrap() {
        if !value.is_object() && !value.is_array() {
            assert_eq!(value, &place[key], "{key}");
        }
    }
    assert_eq!(copy["phase"], place["phase"]);
    assert_eq!(copy["submission"], place["submission"]);
}

/// The fields of sluice's preview no copy holds (the runner-protocol
/// spec's "A preview's copy holds no output of its command").
const PREVIEW_LEFT_OUT: &[&str] = &["checkout", "names", "ports", "exit", "output"];

/// The hosted-control-plane spec's "sluice's previews cut" (design
/// Decision 4 of the-hosted-protocol-carries-land-and-preview): sluice
/// 0.74.3's answer of a preview brought up and each row of its listing,
/// cut as a preview's copy, and its answers of a release, cut as a
/// release's, each pass their definition, keep sluice's `state` or
/// `released`, and hold no preview's checkout, names, ports, exit or
/// output (core.11, core.55).
#[test]
fn sluices_previews_cut() {
    let read = |name: &str| {
        std::fs::read_to_string(fixtures("sluice-0.74.3").join(name))
            .unwrap_or_else(|e| panic!("{name}: {e}"))
    };
    let preview = validator(&definition("preview.schema.json", "preview"));
    let mut previews = vec![(
        "preview-starting.json".to_string(),
        read("preview-starting.json"),
    )];
    let rows: Vec<Value> = serde_json::from_str(&read("previews.json")).unwrap();
    previews.extend(
        rows.iter()
            .enumerate()
            .map(|(n, row)| (format!("previews.json's row {n}"), row.to_string())),
    );
    for (name, text) in &previews {
        let printed: Value = serde_json::from_str(text).unwrap();
        let copy = copy::preview(text).unwrap_or_else(|words| panic!("{name}: {words}"));
        let refused = errors(&preview, &copy);
        assert!(refused.is_empty(), "{name}'s copy is refused: {refused:#?}");
        for field in PREVIEW_LEFT_OUT {
            assert!(copy.get(*field).is_none(), "{name}'s copy holds `{field}`");
        }
        assert_eq!(copy["state"], printed["state"], "{name}");
    }
    let release = validator(&definition("preview.schema.json", "release"));
    let releases = ["release.json", "release-nothing.json"];
    for name in releases {
        let text = read(name);
        let printed: Value = serde_json::from_str(&text).unwrap();
        let copy = copy::release(&text).unwrap_or_else(|words| panic!("{name}: {words}"));
        let refused = errors(&release, &copy);
        assert!(refused.is_empty(), "{name}'s copy is refused: {refused:#?}");
        assert_eq!(copy["released"], printed["released"], "{name}");
    }
    assert_eq!(
        (previews.len(), releases.len()),
        (3, 2),
        "sluice's previews and releases cut"
    );
}

#[test]
fn sluices_entries_cut() {
    let validator = validator(&definition(
        "submission-entry.schema.json",
        "submission_entry",
    ));
    let mut cut = 0;
    for (name, text) in statuses("sluice-0.61.1") {
        let entry: Value = serde_json::from_str(&text).unwrap();
        if entry.get("error").is_some() {
            continue;
        }
        let copy = copy::submission_entry(&text).unwrap_or_else(|words| panic!("{name}: {words}"));
        let refused = errors(&validator, &copy);
        assert!(refused.is_empty(), "{name}'s copy is refused: {refused:#?}");
        for row in copy["failed"].as_array().into_iter().flatten() {
            assert!(row.get("log").is_none(), "{name}'s copy holds a row's log");
        }
        assert_eq!(copy["state"], entry["state"], "{name}");
        cut += 1;
    }
    assert_eq!(cut, 3, "sluice's entries cut");
}

#[test]
fn flumes_host_part_cut() {
    let validator = validator(&definition("hello.schema.json", "host_part"));
    for name in ["status.json", "status-bounded.json"] {
        let text = std::fs::read_to_string(fixtures("flume-0.4.1").join(name)).unwrap();
        let host = &serde_json::from_str::<Value>(&text).unwrap()["status"]["host"];
        let part = HostPart {
            running: host["running"].to_string(),
            bound: host["bound"].to_string(),
            room: host["room"].to_string(),
            streams: host["streams"].to_string(),
        };
        let copy = copy::host_part(&part).unwrap_or_else(|words| panic!("{name}: {words}"));
        let refused = errors(&validator, &copy);
        assert!(refused.is_empty(), "{name}'s copy is refused: {refused:#?}");
        let mut queues = Vec::new();
        held(&copy, &["queue"], "", &mut queues);
        assert!(queues.is_empty(), "{name}'s copy holds {queues:?}");
        assert_eq!(copy["room"], host["room"], "{name}");
    }
}
