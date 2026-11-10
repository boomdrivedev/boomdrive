//! The cut of a tool's answer to its copy (design Decision 9; core.11,
//! core.27, core.42): a place's status, sluice's entry and Flume's host
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

/// The copy of Flume's status of one place, `text` as `flume --json
/// status <place>` printed it: its `status.places[0]`, cut and checked;
/// or why it is none, naming a path and never a value.
pub fn place_status(text: &str) -> Result<Value, String> {
    let answer: Value =
        serde_json::from_str(text).map_err(|_| "Flume's status is no JSON".to_string())?;
    let place = answer
        .pointer("/status/places/0")
        .ok_or_else(|| "Flume's status holds no `/status/places/0`".to_string())?;
    checked(check::place_status(), cut(PLACE_STATUS, place))
}

/// The copy of sluice's entry of a submission, `text` as `sluice status
/// <n> --json` printed it, cut and checked; or why it is none, naming a
/// path and never a value.
pub fn submission_entry(text: &str) -> Result<Value, String> {
    let entry: Value =
        serde_json::from_str(text).map_err(|_| "sluice's entry is no JSON".to_string())?;
    checked(check::submission_entry(), cut(SUBMISSION_ENTRY, &entry))
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
