//! The facts the runner reads from Flume's answers (design Decisions 6,
//! 8, 12, 13 and 16), each guarded: a field absent, or of another type
//! than Flume 0.4.1 prints it, reads as none, so that no answer is misread
//! as another (crew `read.2`). The runner judges nothing by them but what
//! those decisions name, and carries every answer whole (core.1, core.11).
//! A place's entry and the host part's fields are sliced as Flume printed
//! them, and never re-serialised.

use serde::Deserialize;
use serde_json::value::RawValue;
use serde_json::Value;

/// The entry of every place an answer of `flume --json status` holds:
/// the listing's every place, or the one place asked for.
pub fn places(answer: &Value) -> &[Value] {
    answer
        .pointer("/status/places")
        .and_then(Value::as_array)
        .map_or(&[], Vec::as_slice)
}

/// The entry of the place at `location` in `answer`: the place standing
/// there, else the record Flume keeps of it once tidied while an end of
/// it is pending, `<location>@<at>`, the latest where it keeps several
/// (flume's 43, 62).
pub fn entry<'a>(answer: &'a Value, location: &str) -> Option<&'a Value> {
    let places = places(answer);
    if let Some(standing) = places
        .iter()
        .find(|entry| self::location(entry) == Some(location))
    {
        return Some(standing);
    }
    places
        .iter()
        .filter_map(|entry| Some((tidied_at(self::location(entry)?, location)?, entry)))
        .max_by_key(|(at, _)| *at)
        .map(|(_, entry)| entry)
}

/// The time of a tidied place's record, where `named` is `location`'s,
/// `<location>@<at>`.
fn tidied_at(named: &str, location: &str) -> Option<u64> {
    let at = named.strip_prefix(location)?.strip_prefix('@')?;
    if at.is_empty() || !at.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    at.parse().ok()
}

/// The entry of the place whose mark is `mark`.
pub fn marked<'a>(answer: &'a Value, mark: &str) -> Option<&'a Value> {
    places(answer)
        .iter()
        .find(|entry| self::mark(entry) == Some(mark))
}

/// The entry of the place a launch's answer names, `launched.place`.
pub fn launched(answer: &Value) -> Option<&Value> {
    answer
        .pointer("/launched/place")
        .filter(|entry| entry.is_object())
}

/// A place's location, its `place` as Flume printed it: a path, or
/// `<location>@<at>` for a tidied place's record.
pub fn location(entry: &Value) -> Option<&str> {
    entry.get("place")?.as_str()
}

/// A place's mark.
pub fn mark(entry: &Value) -> Option<&str> {
    entry.get("mark")?.as_str()
}

/// Whether a place's phase is live or making, so that its run stands
/// (crew's `role_run`).
pub fn live_or_making(entry: &Value) -> bool {
    match entry.get("phase") {
        Some(Value::String(phase)) => phase == "live",
        Some(Value::Object(phase)) => phase.len() == 1 && phase.contains_key("making"),
        _ => false,
    }
}

/// Whether a place has ended, so Flume holds it no more but as a record:
/// its phase `tidied` or `gone`, `{"tidied" | "gone": {…}}`. A place over,
/// whose worktree may stand until Flume tidies it, or one that waits on
/// its caller, has not ended (record 0021).
pub fn ended(entry: &Value) -> bool {
    let Some(phase) = entry.get("phase").and_then(Value::as_object) else {
        return false;
    };
    phase.len() == 1 && (phase.contains_key("tidied") || phase.contains_key("gone"))
}

/// Whether the place whose entry is `entry` is tidied and Flume keeps its
/// record while an end of it is pending for `name`: its phase `{"tidied":
/// {…, "pending_for": [<name>, …]}}` (flume's 43, 62; record 0030).
pub fn pending_for(entry: &Value, name: &str) -> bool {
    let Some(phase) = entry.get("phase").and_then(Value::as_object) else {
        return false;
    };
    if phase.len() != 1 {
        return false;
    }
    phase
        .get("tidied")
        .and_then(|tidied| tidied.get("pending_for"))
        .and_then(Value::as_array)
        .is_some_and(|pending| pending.iter().any(|each| each.as_str() == Some(name)))
}

/// Why a place's work is over, Flume's `over`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Over {
    /// Landed, at the commit Flume names where it names one.
    Landed(Option<String>),
    Dropped,
    Done,
}

/// Why a place's work is over, where its phase is over, gone or tidied:
/// `{"over" | "gone" | "tidied": {"over": <over>, …}}`.
pub fn over(entry: &Value) -> Option<Over> {
    let phase = entry.get("phase")?.as_object()?;
    if phase.len() != 1 {
        return None;
    }
    let (word, held) = phase.iter().next()?;
    if !["over", "gone", "tidied"].contains(&word.as_str()) {
        return None;
    }
    match held.get("over")? {
        Value::String(over) if over == "dropped" => Some(Over::Dropped),
        Value::String(over) if over == "done" => Some(Over::Done),
        Value::Object(over) if over.len() == 1 => {
            let landed = over.get("landed")?.as_object()?;
            let commit = landed.get("commit").and_then(Value::as_str);
            Some(Over::Landed(commit.map(str::to_string)))
        }
        _ => None,
    }
}

/// Whether a place waits on its caller because its last stage delivered
/// and its kind does not land, `waiting_on.why` `delivered`.
pub fn waits_delivered(entry: &Value) -> bool {
    entry.pointer("/waiting_on/why").and_then(Value::as_str) == Some("delivered")
}

/// The id of the sluice entry a place's submission is,
/// `submission.read.found.submission.id`.
pub fn submission_id(entry: &Value) -> Option<&str> {
    entry
        .pointer("/submission/read/found/submission/id")?
        .as_str()
}

/// A herdr pane, as Flume's clues address one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pane {
    /// The pane's id in its session.
    pub id: String,
    /// The session's socket, `HERDR_SOCKET_PATH`.
    pub socket_file: String,
}

/// The pane a place's agent shows in, from `clues.shown_at.agent`,
/// `herdr:<pane>@<socket>` (flume's 18).
pub fn pane(entry: &Value) -> Option<Pane> {
    let shown = entry.pointer("/clues/shown_at/agent")?.as_str()?;
    let (id, socket) = shown.strip_prefix("herdr:")?.split_once('@')?;
    if id.is_empty() || socket.is_empty() {
        return None;
    }
    Some(Pane {
        id: id.to_string(),
        socket_file: socket.to_string(),
    })
}

/// Room, as Flume's host part gives it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Room {
    /// `null`: no bound.
    Unbounded,
    /// The places that may yet start.
    Count(i64),
}

/// The host's room, `status.host.room`.
pub fn host_room(answer: &Value) -> Option<Room> {
    room(answer.pointer("/status/host/room")?)
}

/// A room as Flume prints one: `null` or a whole number.
fn room(value: &Value) -> Option<Room> {
    match value {
        Value::Null => Some(Room::Unbounded),
        number => number.as_i64().map(Room::Count),
    }
}

/// One of the host's streams, as Flume's host part gives its room.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StreamRoom {
    pub name: String,
    pub room: Option<Room>,
    /// Whether no place waits in its queue.
    pub queue_empty: Option<bool>,
}

/// The host's streams, `status.host.streams`, in Flume's order. A stream
/// with no name is left out, since no launch can name it.
pub fn streams(answer: &Value) -> Vec<StreamRoom> {
    let Some(streams) = answer
        .pointer("/status/host/streams")
        .and_then(Value::as_array)
    else {
        return Vec::new();
    };
    streams
        .iter()
        .filter_map(|stream| {
            Some(StreamRoom {
                name: stream.get("name")?.as_str()?.to_string(),
                room: stream.get("room").and_then(room),
                queue_empty: stream
                    .get("queue")
                    .and_then(Value::as_array)
                    .map(Vec::is_empty),
            })
        })
        .collect()
}

/// The host part's fields the hello carries (design Decision 6), each the
/// text Flume printed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostPart {
    pub running: String,
    pub bound: String,
    pub room: String,
    pub streams: String,
}

#[derive(Deserialize)]
struct Listing<'a> {
    #[serde(borrow)]
    status: Status<'a>,
}

#[derive(Deserialize)]
struct Status<'a> {
    #[serde(borrow)]
    host: Host<'a>,
}

/// The four fields of the host part, and none of `pools`, `addresses`,
/// `bases`, `terminal_kept` or the rest, which are Flume's bookkeeping.
#[derive(Deserialize)]
struct Host<'a> {
    #[serde(borrow)]
    running: &'a RawValue,
    #[serde(borrow)]
    bound: &'a RawValue,
    #[serde(borrow)]
    room: &'a RawValue,
    #[serde(borrow)]
    streams: &'a RawValue,
}

/// The host part's `running`, `bound`, `room` and `streams`, sliced from
/// the text of a `flume --json status` as Flume printed each; none where
/// the text is not JSON or lacks one of them.
pub fn host_part(text: &str) -> Option<HostPart> {
    let listing: Listing = serde_json::from_str(text).ok()?;
    let host = listing.status.host;
    Some(HostPart {
        running: host.running.get().to_string(),
        bound: host.bound.get().to_string(),
        room: host.room.get().to_string(),
        streams: host.streams.get().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_phase_as_flume_writes_each() {
        let at = |phase: Value| json!({ "place": "/p", "phase": phase });
        assert!(live_or_making(&at(json!("live"))));
        assert!(live_or_making(&at(json!({ "making": { "by": "launch" } }))));
        for phase in [
            json!({ "over": { "over": "done", "kept": null } }),
            json!({ "tidied": { "over": "dropped", "at": 1, "pending_for": [] } }),
            json!("over"),
            json!(null),
            json!(7),
            json!({ "making": {}, "live": {} }),
        ] {
            assert!(!live_or_making(&at(phase.clone())), "{phase} read live");
        }
        assert_eq!(
            over(&at(json!({ "gone": { "over": "done" } }))),
            Some(Over::Done)
        );
        assert_eq!(
            over(&at(
                json!({ "tidied": { "over": "dropped", "at": 1, "pending_for": ["r"] } })
            )),
            Some(Over::Dropped)
        );
        assert_eq!(
            over(&at(
                json!({ "over": { "over": { "landed": {} }, "kept": null } })
            )),
            Some(Over::Landed(None))
        );
        assert_eq!(over(&at(json!("live"))), None);
    }

    #[test]
    fn a_place_that_has_ended() {
        let at = |phase: Value| json!({ "place": "/p", "phase": phase });
        for phase in [
            json!({ "tidied": { "over": "dropped", "at": 1, "pending_for": ["r"] } }),
            json!({ "tidied": { "over": { "landed": { "commit": "c" } }, "at": 2 } }),
            json!({ "gone": { "over": "done" } }),
        ] {
            assert!(ended(&at(phase.clone())), "{phase} read not ended");
        }
        let delivered = json!({
            "place": "/p",
            "phase": "live",
            "waiting_on": { "why": "delivered" },
        });
        assert!(waits_delivered(&delivered));
        assert!(!ended(&delivered), "a delivered wait read ended");
        for phase in [
            json!("live"),
            json!({ "making": { "by": "launch" } }),
            json!({ "over": { "over": "done", "kept": null } }),
            json!({ "over": { "over": { "landed": {} }, "kept": null } }),
            json!("tidied"),
            json!({ "tidied": {}, "gone": {} }),
            json!(null),
        ] {
            assert!(!ended(&at(phase.clone())), "{phase} read ended");
        }
        assert!(!ended(&json!({})));
    }

    #[test]
    fn a_record_pending_for_a_recipient() {
        let at = |phase: Value| json!({ "place": "/p@1", "phase": phase });
        let tidied = |pending: Value| {
            at(json!({ "tidied": { "over": "dropped", "at": 1, "pending_for": pending } }))
        };
        assert!(pending_for(&tidied(json!(["boomdrive"])), "boomdrive"));
        assert!(pending_for(
            &tidied(json!(["scratch/planner", "boomdrive"])),
            "boomdrive"
        ));
        for entry in [
            tidied(json!(["scratch/planner"])),
            tidied(json!([])),
            tidied(json!("boomdrive")),
            tidied(json!([7])),
            at(json!({ "tidied": { "over": "done", "at": 1 } })),
            at(json!({ "gone": { "over": "done", "pending_for": ["boomdrive"] } })),
            at(json!({ "over": { "over": "done", "kept": null } })),
            at(json!({ "tidied": { "pending_for": ["boomdrive"] }, "gone": {} })),
            at(json!("live")),
            json!({ "place": "/p" }),
        ] {
            assert!(!pending_for(&entry, "boomdrive"), "{entry} read pending");
        }
    }

    #[test]
    fn a_field_of_another_type_reads_as_none() {
        let entry = json!({
            "place": 7,
            "mark": ["boomdrive:a/1"],
            "phase": { "over": { "over": { "landed": { "commit": 5 } } }, "making": {} },
            "waiting_on": { "why": 3 },
            "submission": { "read": { "found": { "submission": { "id": 3 } } } },
            "clues": { "shown_at": { "agent": 1 } },
        });
        assert_eq!(location(&entry), None);
        assert_eq!(mark(&entry), None);
        assert_eq!(over(&entry), None);
        assert!(!live_or_making(&entry));
        assert!(!waits_delivered(&entry));
        assert_eq!(submission_id(&entry), None);
        assert_eq!(pane(&entry), None);
        let landed = json!({ "phase": { "over": { "over": { "landed": { "commit": 5 } } } } });
        assert_eq!(over(&landed), Some(Over::Landed(None)));
        assert_eq!(
            over(&json!({ "phase": { "over": { "over": "lost" } } })),
            None
        );
        assert_eq!(
            over(&json!({ "phase": { "landed": { "over": "done" } } })),
            None
        );
    }

    #[test]
    fn a_field_absent_reads_as_none() {
        let entry = json!({});
        assert_eq!(location(&entry), None);
        assert_eq!(mark(&entry), None);
        assert_eq!(over(&entry), None);
        assert!(!live_or_making(&entry));
        assert!(!waits_delivered(&entry));
        assert_eq!(submission_id(&entry), None);
        assert_eq!(pane(&entry), None);
        assert_eq!(
            places(&json!({ "status": { "places": {} } })),
            &[] as &[Value]
        );
        assert_eq!(launched(&json!({ "launched": { "place": "/p" } })), None);
        assert_eq!(host_room(&json!({ "status": { "host": {} } })), None);
        assert_eq!(
            streams(&json!({ "status": { "host": { "streams": {} } } })),
            []
        );
        assert_eq!(
            host_part(r#"{"status":{"host":{"running":0,"bound":null,"room":null}}}"#),
            None
        );
        assert_eq!(host_part("not json"), None);
    }

    #[test]
    fn a_pane_address() {
        let shown = |agent: Value| json!({ "clues": { "shown_at": { "agent": agent } } });
        assert_eq!(
            pane(&shown(json!("herdr:w1:p2@/s/herdr@1.sock"))),
            Some(Pane {
                id: "w1:p2".into(),
                socket_file: "/s/herdr@1.sock".into()
            })
        );
        for agent in [
            json!(null),
            json!("tmux:w1:p2@/s"),
            json!("herdr:w1:p2"),
            json!("herdr:@/s"),
            json!("herdr:w1@"),
        ] {
            assert_eq!(pane(&shown(agent.clone())), None, "{agent} read as a pane");
        }
    }

    #[test]
    fn rooms() {
        let host = |room: Value| json!({ "status": { "host": { "room": room } } });
        assert_eq!(host_room(&host(json!(null))), Some(Room::Unbounded));
        assert_eq!(host_room(&host(json!(0))), Some(Room::Count(0)));
        assert_eq!(host_room(&host(json!(2))), Some(Room::Count(2)));
        assert_eq!(host_room(&host(json!("2"))), None);
        assert_eq!(host_room(&host(json!(1.5))), None);
        let listed = json!({ "status": { "host": { "streams": [
            { "name": "a", "room": "1", "queue": {} },
            { "room": 1, "queue": [] },
            { "name": "b" },
        ] } } });
        assert_eq!(
            streams(&listed),
            [
                StreamRoom {
                    name: "a".into(),
                    room: None,
                    queue_empty: None
                },
                StreamRoom {
                    name: "b".into(),
                    room: None,
                    queue_empty: None
                },
            ]
        );
    }

    #[test]
    fn a_tidied_record_is_its_places_entry() {
        let answer = json!({ "status": { "places": [
            { "place": "/p/fix--a@20" },
            { "place": "/p/fix--a@100" },
            { "place": "/p/fix--a@x" },
            { "place": "/p/fix--ab@200" },
        ] } });
        assert_eq!(
            entry(&answer, "/p/fix--a").and_then(location),
            Some("/p/fix--a@100")
        );
        let standing = json!({ "status": { "places": [
            { "place": "/p/fix--a@100" },
            { "place": "/p/fix--a" },
        ] } });
        assert_eq!(
            entry(&standing, "/p/fix--a").and_then(location),
            Some("/p/fix--a")
        );
        assert_eq!(entry(&answer, "/p/fix"), None);
    }
}
