//! The cut of a tool's answer to its copy (design Decision 9; core.11,
//! core.27, core.42): a place's status, sluice's entry, sluice's answer
//! and listing of a preview and its answer to a release, and Flume's host
//! part, each cut by walking version 2's own definition of its copy, so
//! the schema is the one list of what goes up, then checked against that
//! definition before it goes. The cut keeps each value as the tool printed
//! it and writes none again. Because every object of version 2 is closed,
//! the check holds the line and the cut only makes a copy that passes.

use crate::check;
use crate::protocol::v2::{ADDRESS, FILES};
use boomdrive_control::HostPart;
use jsonschema::Validator;
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::sync::OnceLock;

/// The definition a place's status is cut by.
pub const PLACE_STATUS: &str = "place-status.schema.json#/$defs/place_status";

/// The definition sluice's entry is cut by.
pub const SUBMISSION_ENTRY: &str = "submission-entry.schema.json#/$defs/submission_entry";

/// The definition Flume's host part is cut by.
pub const HOST_PART: &str = "hello.schema.json#/$defs/host_part";

/// The definition sluice's answer to a preview's ask, and each row of its
/// listing, is cut by.
pub const PREVIEW: &str = "preview.schema.json#/$defs/preview";

/// The definition sluice's answer to a release is cut by.
pub const RELEASE: &str = "preview.schema.json#/$defs/release";

/// The definition an end of a place's status is checked against: one of
/// its named forms, or its delivery alone.
pub const END_RECORD: &str = "place-status.schema.json#/$defs/end_record";

/// The definition an end of a form version 2 does not name is cut by: its
/// delivery alone.
pub const END_DELIVERY: &str = "place-status.schema.json#/$defs/end_delivery";

/// A place's status as its copy: the copy, and Flume's ends it could not
/// carry whole, each by its index in Flume's `ends` (design Decision 2 of
/// the-hosted-protocol-carries-every-end-of-a-place).
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceCopy {
    /// The copy, which passes version 2's `place_status`.
    pub copy: Value,
    /// Each end carried by its delivery alone.
    pub by_delivery: Vec<usize>,
    /// Each end left out, which not even its delivery alone could carry.
    pub left_out: Vec<usize>,
}

impl PlaceCopy {
    /// Whether every end went up in a form version 2 names.
    pub fn whole(&self) -> bool {
        self.by_delivery.is_empty() && self.left_out.is_empty()
    }
}

/// How one of Flume's ends went into a copy.
#[derive(Clone, Debug, PartialEq)]
enum EndCopy {
    /// Cut by the form version 2 names for its `of`, and passing.
    Named(Value),
    /// Cut to its delivery alone, and passing.
    Delivery(Value),
    /// Neither passed.
    LeftOut,
}

/// The copy of Flume's status of one place, `text` as `flume --json
/// status <place>` printed it: its `status.places[0]`, cut, each of its
/// ends copied on its own, and checked whole; or why it is none, naming
/// a path and never a value.
///
/// Each end is cut by the form version 2 names for its `of` and checked;
/// where there is none, or the check fails, it is cut to its delivery
/// alone and checked; and where that fails too, it is left out. The
/// ends' copies stand in `ends` in Flume's order. A copy that fails with
/// its ends so copied fails outside them, and fails whole.
pub fn place_status(text: &str) -> Result<PlaceCopy, String> {
    let answer: Value =
        serde_json::from_str(text).map_err(|_| "Flume's status is no JSON".to_string())?;
    let place = answer
        .pointer("/status/places/0")
        .ok_or_else(|| "Flume's status holds no `/status/places/0`".to_string())?;
    let mut copy = cut(PLACE_STATUS, place);
    let (mut by_delivery, mut left_out) = (Vec::new(), Vec::new());
    if let (Some(ends), Value::Object(fields)) =
        (place.get("ends").and_then(Value::as_array), &mut copy)
    {
        let mut copies = Vec::new();
        for (n, end) in ends.iter().enumerate() {
            match end_copy(end) {
                EndCopy::Named(copy) => copies.push(copy),
                EndCopy::Delivery(copy) => {
                    copies.push(copy);
                    by_delivery.push(n);
                }
                EndCopy::LeftOut => left_out.push(n),
            }
        }
        fields.insert("ends".to_string(), Value::Array(copies));
    }
    checked(check::place_status(), copy).map(|copy| PlaceCopy {
        copy,
        by_delivery,
        left_out,
    })
}

/// One of Flume's ends, `end`, cut by its named form, else by its delivery
/// alone, else left out, each cut checked against `end_record`.
fn end_copy(end: &Value) -> EndCopy {
    let passes = |copy: &Value| check::end_record().is_valid(copy);
    let named = end
        .get("of")
        .and_then(Value::as_str)
        .and_then(|of| named_forms().get(of));
    if let Some(form) = named {
        let copy = cut(form, end);
        if passes(&copy) {
            return EndCopy::Named(copy);
        }
    }
    let copy = cut(END_DELIVERY, end);
    match passes(&copy) {
        true => EndCopy::Delivery(copy),
        false => EndCopy::LeftOut,
    }
}

/// Each form of an end version 2 names, by its `of`, as a reference to
/// its branch of `end_record`.
fn named_forms() -> &'static BTreeMap<String, String> {
    static FORMS: OnceLock<BTreeMap<String, String>> = OnceLock::new();
    FORMS.get_or_init(|| {
        let Some((_, record)) = resolve(None, END_RECORD) else {
            return BTreeMap::new();
        };
        record
            .get("oneOf")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .enumerate()
            .filter_map(|(n, branch)| {
                let of = branch.pointer("/properties/of/const")?.as_str()?;
                Some((of.to_string(), format!("{END_RECORD}/oneOf/{n}")))
            })
            .collect()
    })
}

/// The copy of sluice's entry of a submission, `text` as `sluice status
/// <n> --json` printed it, cut and checked; or why it is none, naming a
/// path and never a value.
pub fn submission_entry(text: &str) -> Result<Value, String> {
    let entry: Value =
        serde_json::from_str(text).map_err(|_| "sluice's entry is no JSON".to_string())?;
    checked(check::submission_entry(), cut(SUBMISSION_ENTRY, &entry))
}

/// The copy of a preview, `text` as the runner carries sluice's answer to
/// a preview's ask or a row of its listing, cut and checked; or why it is
/// none, naming a path and never a value (design Decision 5 of
/// the-hosted-protocol-carries-land-and-preview).
pub fn preview(text: &str) -> Result<Value, String> {
    let preview: Value =
        serde_json::from_str(text).map_err(|_| "sluice's preview is no JSON".to_string())?;
    checked(check::preview(), cut(PREVIEW, &preview))
}

/// The copy of a release, `text` as the runner carries sluice's answer to
/// it, cut and checked; or why it is none, naming a path and never a
/// value.
pub fn release(text: &str) -> Result<Value, String> {
    let release: Value =
        serde_json::from_str(text).map_err(|_| "sluice's release is no JSON".to_string())?;
    checked(check::release(), cut(RELEASE, &release))
}

/// The copy of Flume's host part, each of `part`'s texts read as the JSON
/// value Flume printed, cut and checked; or why it is none, naming a path
/// and never a value.
pub fn host_part(part: &HostPart) -> Result<Value, String> {
    let mut values = Map::new();
    for (name, text) in [
        ("running", &part.running),
        ("bound", &part.bound),
        ("room", &part.room),
        ("streams", &part.streams),
    ] {
        let value: Value = serde_json::from_str(text)
            .map_err(|_| format!("Flume's host part's `{name}` is no JSON"))?;
        values.insert(name.to_string(), value);
    }
    checked(check::host_part(), cut(HOST_PART, &Value::Object(values)))
}

/// `copy`, where it passes `validator`; or why not, naming the paths and
/// never a value.
fn checked(validator: &Validator, copy: Value) -> Result<Value, String> {
    match check::failure(validator, &copy) {
        None => Ok(copy),
        Some(words) => Err(words),
    }
}

/// `value` cut by the definition `reference`, a file of version 2 and a
/// JSON pointer in it, `<file>#<pointer>`:
///
/// - an object keeps only the properties its definition names, each cut
///   by its own definition, where the definition is closed; an open one
///   keeps the rest whole;
/// - an array cuts each item;
/// - a `oneOf` cuts by the one branch whose form the value takes: a
///   constant by its value, an object by its keys and their constants,
///   `null` by its type;
/// - a `$ref` is followed within version 2's files;
/// - every other value, and one its definition carries whole, is kept as
///   the tool printed it.
///
/// A value that fits no branch is not cut: it is kept whole, so the check
/// that follows fails and names its path.
pub fn cut(reference: &str, value: &Value) -> Value {
    match resolve(None, reference) {
        Some((file, definition)) => walk(file, definition, value),
        None => value.clone(),
    }
}

/// Each of version 2's files by its name, read once.
fn files() -> &'static BTreeMap<&'static str, Value> {
    static FILES_READ: OnceLock<BTreeMap<&'static str, Value>> = OnceLock::new();
    FILES_READ.get_or_init(|| {
        FILES
            .iter()
            .map(|(name, text)| {
                // Each file is the crate's own, read by its tests.
                (*name, serde_json::from_str(text).unwrap_or(Value::Null))
            })
            .collect()
    })
}

/// What `reference` names, read from the file `from` where it names no
/// file of its own, with the name of the file it is in; none where it is
/// outside version 2's files.
fn resolve(from: Option<&'static str>, reference: &str) -> Option<(&'static str, &'static Value)> {
    let reference = reference.strip_prefix(ADDRESS).unwrap_or(reference);
    let (file, pointer) = reference.split_once('#').unwrap_or((reference, ""));
    let (name, document) = match file {
        "" => {
            let name = from?;
            (name, files().get(name)?)
        }
        file => files()
            .get_key_value(file)
            .map(|(name, value)| (*name, value))?,
    };
    Some((name, document.pointer(pointer)?))
}

/// `definition`, in the file `file`, with each `$ref` it is followed, and
/// the file it is then in.
fn followed(
    mut file: &'static str,
    mut definition: &'static Value,
) -> (&'static str, &'static Value) {
    // A reference that leads back to itself is followed no further than
    // the version's files are deep.
    for _ in 0..files().len() * 8 {
        let Some(reference) = definition.get("$ref").and_then(Value::as_str) else {
            break;
        };
        match resolve(Some(file), reference) {
            Some((next, target)) => (file, definition) = (next, target),
            None => break,
        }
    }
    (file, definition)
}

/// `value` cut by `definition`, in the file `file`.
fn walk(file: &'static str, definition: &'static Value, value: &Value) -> Value {
    let (file, definition) = followed(file, definition);
    if let Some(branches) = definition.get("oneOf").and_then(Value::as_array) {
        return match branches.iter().find(|branch| fits(file, branch, value)) {
            Some(branch) => walk(file, branch, value),
            None => value.clone(),
        };
    }
    match value {
        Value::Object(fields) => {
            let properties = definition.get("properties").and_then(Value::as_object);
            let rest = definition.get("additionalProperties");
            if properties.is_none() && rest.is_none() {
                return value.clone();
            }
            let mut copy = Map::new();
            for (name, field) in fields {
                let cut = match (properties.and_then(|named| named.get(name)), rest) {
                    (Some(named), _) => walk(file, named, field),
                    (None, Some(Value::Bool(false))) => continue,
                    (None, Some(rest @ Value::Object(_))) => walk(file, rest, field),
                    (None, _) => field.clone(),
                };
                copy.insert(name.clone(), cut);
            }
            Value::Object(copy)
        }
        Value::Array(items) => match definition.get("items") {
            Some(item) => Value::Array(items.iter().map(|each| walk(file, item, each)).collect()),
            None => value.clone(),
        },
        _ => value.clone(),
    }
}

/// Whether `value` takes the form of the `oneOf` branch `branch`, in the
/// file `file`: its constant, one of its values, its type, and for an
/// object, every key it requires present and every constant it names
/// held.
fn fits(file: &'static str, branch: &'static Value, value: &Value) -> bool {
    let (file, branch) = followed(file, branch);
    if let Some(constant) = branch.get("const") {
        return value == constant;
    }
    if let Some(values) = branch.get("enum").and_then(Value::as_array) {
        return values.contains(value);
    }
    if let Some(branches) = branch.get("oneOf").and_then(Value::as_array) {
        return branches.iter().any(|each| fits(file, each, value));
    }
    let typed = match branch.get("type") {
        Some(Value::String(name)) => is_type(name, value),
        Some(Value::Array(names)) => names
            .iter()
            .filter_map(Value::as_str)
            .any(|name| is_type(name, value)),
        _ => true,
    };
    let Value::Object(fields) = value else {
        return typed;
    };
    let required = branch
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .all(|name| fields.contains_key(name));
    let constants = branch
        .get("properties")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
        .filter_map(|(name, property)| Some((name, property.get("const")?)))
        .all(|(name, constant)| fields.get(name).is_none_or(|field| field == constant));
    typed && required && constants
}

/// Whether `value` is of JSON Schema's type `name`.
fn is_type(name: &str, value: &Value) -> bool {
    match name {
        "null" => value.is_null(),
        "boolean" => value.is_boolean(),
        "string" => value.is_string(),
        "object" => value.is_object(),
        "array" => value.is_array(),
        "number" => value.is_number(),
        "integer" => {
            value.is_i64() || value.is_u64() || value.as_f64().is_some_and(|n| n.fract() == 0.0)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_closed_object_keeps_only_what_it_names() {
        let stream = json!({ "name": "work", "from": "launch", "queue": ["a"] });
        assert_eq!(
            cut("place-status.schema.json#/$defs/stream", &stream),
            json!({ "name": "work", "from": "launch" })
        );
    }

    #[test]
    fn an_open_object_is_carried_whole() {
        let made = json!({ "name": "preview", "url": "https://preview.example.com" });
        assert_eq!(cut("place-status.schema.json#/$defs/made", &made), made);
    }

    #[test]
    fn a_branch_is_chosen_by_its_constant_its_keys_or_null() {
        let hold = json!({ "on": "submission", "since": 7, "words": "read first", "by": "x" });
        assert_eq!(
            cut("place-status.schema.json#/$defs/hold", &hold),
            json!({ "on": "submission", "since": 7, "words": "read first" })
        );
        let kept = json!({ "teardown": { "step": "sluice release", "words": "no sluice" } });
        assert_eq!(
            cut("place-status.schema.json#/$defs/kept", &kept),
            json!({ "teardown": { "step": "sluice release" } })
        );
        assert_eq!(
            cut("place-status.schema.json#/$defs/over", &json!("done")),
            json!("done")
        );
        let intent = json!({ "from": "base", "base": "main", "digest": "d", "path": "/p" });
        assert_eq!(
            cut("place-status.schema.json#/$defs/intent", &intent),
            json!({ "from": "base", "base": "main", "digest": "d" })
        );
    }

    #[test]
    fn a_value_that_fits_no_branch_is_kept_whole() {
        let phase = json!({ "paused": { "words": "kept whole" } });
        assert_eq!(cut("place-status.schema.json#/$defs/phase", &phase), phase);
    }

    #[test]
    fn a_copy_that_fails_names_a_path_and_no_value() {
        let words = submission_entry(r#"{"id":"n0t-a-number","state":"queued"}"#)
            .expect_err("an id that is no integer fails");
        assert!(words.starts_with("/id"), "{words}");
        assert!(!words.contains("n0t-a-number"), "{words}");
    }

    /// Version 2's example of a tidied place, as `flume --json status`
    /// would print it, its ends `ends`.
    fn status_with(ends: Value) -> String {
        let report: Value = serde_json::from_str(include_str!(
            "../protocol/v2/examples/from-runner/report-a-tidied-place.json"
        ))
        .expect("the example");
        let mut place = report["body"]["report"]["launches"]["offer-6"]["status"]["copy"].clone();
        place["ends"] = ends;
        json!({ "status": { "places": [place] } }).to_string()
    }

    /// A stage's end as Flume 0.21.0 prints it, beside what version 2
    /// leaves out of it: its agent's last words and its screen.
    fn stage_end(state: &str) -> Value {
        json!({
            "of": "stage", "stage": "code", "start": 1, "state": state,
            "end": {
                "head": null, "report": null, "words": null, "needs": null,
                "last_words": "n0t-sh0wn", "commits": [], "screen": null, "made": []
            },
            "read_at": 1793559675, "deliveries": [],
            "key": format!("stage/code/1/{state}"), "pending_for": ["boomdrive"],
            "delivery": format!("fix--gate/0a1b2c3d/stage/code/1/{state}"),
            "expires_at": 1794164475,
            "owed": [{ "recipient": "boomdrive", "expired": false, "known": true }],
            "confirmed": []
        })
    }

    #[test]
    fn each_end_is_copied_on_its_own() {
        let stall = json!({
            "of": "stall", "since": 1793559632, "waiting": "answer", "answerer": "example/planner",
            "read_at": 1793559640, "deliveries": [], "key": "stall/1793559632",
            "pending_for": [], "owed": [], "confirmed": []
        });
        let mut moved = stage_end("delivered");
        moved["of"] = json!("moved");
        let mut keyless = stage_end("delivered");
        keyless.as_object_mut().unwrap().remove("key");
        let ends = json!([
            stage_end("delivered"),
            moved,
            stage_end("dreaming"),
            keyless,
            stall
        ]);

        let placed = place_status(&status_with(ends)).expect("a copy");

        assert_eq!(placed.by_delivery, [1, 2]);
        assert_eq!(placed.left_out, [3]);
        assert!(!placed.whole());
        let copies = placed.copy["ends"].as_array().expect("ends");
        assert_eq!(copies.len(), 4, "{copies:#?}");
        assert_eq!(copies[0]["of"], "stage");
        assert_eq!(copies[0]["end"]["head"], Value::Null);
        assert!(copies[0]["end"].get("last_words").is_none());
        assert!(copies[0]["deliveries"].is_array(), "a named form whole");
        let delivery_alone = json!({
            "of": "moved", "key": "stage/code/1/delivered", "read_at": 1793559675,
            "pending_for": ["boomdrive"],
            "delivery": "fix--gate/0a1b2c3d/stage/code/1/delivered", "expires_at": 1794164475,
            "owed": [{ "recipient": "boomdrive", "expired": false, "known": true }],
            "confirmed": []
        });
        assert_eq!(copies[1], delivery_alone);
        assert_eq!(copies[2]["of"], "stage");
        assert_eq!(copies[2]["key"], "stage/code/1/dreaming");
        assert!(copies[2].get("state").is_none(), "by its delivery alone");
        assert_eq!(
            copies[3],
            json!({
                "of": "stall", "read_at": 1793559640, "deliveries": [], "key": "stall/1793559632",
                "pending_for": [], "owed": [], "confirmed": []
            }),
            "a stall's end holds none of its move or answerer"
        );
    }

    #[test]
    fn a_copy_whose_ends_all_pass_is_whole() {
        let placed = place_status(&status_with(json!([stage_end("delivered")]))).expect("a copy");
        assert!(placed.whole());
        assert_eq!(placed.copy["ends"].as_array().map(Vec::len), Some(1));
    }

    #[test]
    fn a_failure_outside_the_ends_fails_whole() {
        let mut status: Value =
            serde_json::from_str(&status_with(json!([stage_end("dreaming")]))).unwrap();
        status["status"]["places"][0]["priority"] = json!("n0t-a-number");
        let words = place_status(&status.to_string()).expect_err("a priority that is no integer");
        assert!(words.starts_with("/priority"), "{words}");
        assert!(!words.contains("n0t-a-number"), "{words}");
        let mut status: Value = serde_json::from_str(&status_with(json!([]))).unwrap();
        status["status"]["places"][0]["ends"] = json!("n0t-a-list");
        let words = place_status(&status.to_string()).expect_err("ends that are no list");
        assert!(words.starts_with("/ends"), "{words}");
        assert!(!words.contains("n0t-a-list"), "{words}");
    }

    #[test]
    fn the_host_part_drops_each_streams_queue() {
        let part = HostPart {
            running: "1".to_string(),
            bound: "null".to_string(),
            room: "null".to_string(),
            streams: r#"[{"name":"work","priority":0,"running":1,"bound":1,"room":0,"queue":["/places/x"]}]"#
                .to_string(),
        };
        assert_eq!(
            host_part(&part).expect("a copy"),
            json!({
                "running": 1,
                "bound": null,
                "room": null,
                "streams": [{ "name": "work", "priority": 0, "running": 1, "bound": 1, "room": 0 }]
            })
        );
    }
}
