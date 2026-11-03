//! Version 1 names no grant and no credential (the runner-protocol spec's
//! "Version 1 names no grant and no credential"; design Decision 14): a
//! connection's account, and what it may do, come from its enrolment,
//! never from a message (core.4; flywheel-cloud records 0047 and 0065).

mod common;

use common::files;
use serde_json::Value;
use std::collections::BTreeSet;

/// The words no name of version 1 holds.
const UNNAMED: [&str; 7] = [
    "grant",
    "role",
    "permission",
    "token",
    "credential",
    "secret",
    "password",
];

/// Every name in `value`, at any depth: each member's name, which takes
/// in each `properties` key and each `$defs` key, and each field a
/// `required` lists.
fn names(value: &Value, found: &mut BTreeSet<String>) {
    match value {
        Value::Object(members) => {
            for (key, member) in members {
                found.insert(key.clone());
                if let ("required", Value::Array(fields)) = (key.as_str(), member) {
                    found.extend(fields.iter().filter_map(Value::as_str).map(str::to_string));
                }
                names(member, found);
            }
        }
        Value::Array(elements) => elements.iter().for_each(|e| names(e, found)),
        _ => {}
    }
}

/// The unnamed word `name` holds, alone or as a word of it, in the
/// singular or the plural.
fn unnamed_word(name: &str) -> Option<&'static str> {
    let name = name.to_lowercase();
    name.split(['_', '-', '$']).find_map(|word| {
        let singular = word.strip_suffix('s').unwrap_or(word);
        UNNAMED
            .into_iter()
            .find(|unnamed| *unnamed == word || *unnamed == singular)
    })
}

#[test]
fn every_name_read() {
    for (file, value) in files() {
        let mut found = BTreeSet::new();
        names(&value, &mut found);
        for name in &found {
            if let Some(word) = unnamed_word(name) {
                panic!("{file} names {name:?}, a {word}'s name");
            }
        }
    }
    let mut all = BTreeSet::new();
    files().iter().for_each(|(_, value)| names(value, &mut all));
    for name in ["protocol_version", "hello", "share", "sealed", "lease"] {
        assert!(all.contains(name), "the walk misses {name}");
    }
    for name in ["forge_token", "roles", "grant", "Secret"] {
        assert!(unnamed_word(name).is_some(), "{name} is not caught");
    }
}
